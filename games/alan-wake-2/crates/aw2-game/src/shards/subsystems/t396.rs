#![allow(unused_imports, unused_variables, unused_mut, unused_assignments, non_snake_case, dead_code, clippy::all, unreachable_code, unused_parens)]

#[inline(never)]
pub unsafe extern "win64" fn fn_14253e140(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r8 = 0; fl_l = 0; fl_r = 0; fl_l = rcx; fl_r = rdx; rcx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(rdx))) as u64; fl_l = (crate::simd_ops::ru64(rdx)) as u64; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 4; continue; } bb = 2; continue; }
            2 => { r8 = ((r8).wrapping_add(1)) as u64; rdx = ((rdx).wrapping_add(0x8u64)) as u64; fl_l = r8; fl_r = (0x8u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 1; continue; } bb = 3; continue; }
            3 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; return rax; }
            4 => { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14253e170(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r9 = 0; fl_l = 0; fl_r = 0; fl_l = rcx; fl_r = rdx; rcx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(rdx))) as u64; rax = (((rax) & (crate::simd_ops::ru64(rdx)))) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(rdx)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } bb = 2; continue; }
            2 => { r9 = ((r9).wrapping_add(1)) as u64; rdx = ((rdx).wrapping_add(0x8u64)) as u64; fl_l = r9; fl_r = (0x8u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 1; continue; } bb = 3; continue; }
            3 => { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); return rax; }
            4 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14253e240(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = 0; fl_l = 0; fl_r = 0; bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(rax.wrapping_mul(8)))) as u64; fl_r = (0xffffffffffffffffu64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } bb = 2; continue; }
            2 => { rax = ((rax).wrapping_add(1)) as u64; fl_l = rax; fl_r = (0x8u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 1; continue; } bb = 3; continue; }
            3 => { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); return rax; }
            4 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14253e260(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = 0; fl_l = 0; fl_r = 0; bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(rax.wrapping_mul(8)))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } bb = 2; continue; }
            2 => { rax = ((rax).wrapping_add(1)) as u64; fl_l = rax; fl_r = (0x8u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 1; continue; } bb = 3; continue; }
            3 => { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); return rax; }
            4 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14253e280(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x18; const P3: u64 = 0x20; const P4: u64 = 0x28; const P5: u64 = 0x30; const P6: u64 = 0x38;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    crate::simd_ops::wu64(rcx, (0xffffffffffffffffu64) as u64);
    crate::simd_ops::wu64(rcx.wrapping_add(P0), (0xffffffffffffffffu64) as u64);
    crate::simd_ops::wu64(rcx.wrapping_add(P1), (0xffffffffffffffffu64) as u64);
    crate::simd_ops::wu64(rcx.wrapping_add(P2), (0xffffffffffffffffu64) as u64);
    crate::simd_ops::wu64(rcx.wrapping_add(P3), (0xffffffffffffffffu64) as u64);
    crate::simd_ops::wu64(rcx.wrapping_add(P4), (0xffffffffffffffffu64) as u64);
    crate::simd_ops::wu64(rcx.wrapping_add(P5), (0xffffffffffffffffu64) as u64);
    crate::simd_ops::wu64(rcx.wrapping_add(P6), (0xffffffffffffffffu64) as u64);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14253e2d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x18; const P3: u64 = 0x20; const P4: u64 = 0x28; const P5: u64 = 0x30; const P6: u64 = 0x38;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rcx, rax);
    crate::simd_ops::wu64(rcx.wrapping_add(P0), rax);
    crate::simd_ops::wu64(rcx.wrapping_add(P1), rax);
    crate::simd_ops::wu64(rcx.wrapping_add(P2), rax);
    crate::simd_ops::wu64(rcx.wrapping_add(P3), rax);
    crate::simd_ops::wu64(rcx.wrapping_add(P4), rax);
    crate::simd_ops::wu64(rcx.wrapping_add(P5), rax);
    crate::simd_ops::wu64(rcx.wrapping_add(P6), rax);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14253e300(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0xc; const P1: u64 = 0x18; const P2: u64 = 0x28;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = ((crate::simd_ops::ru16(rcx.wrapping_add(P0))) as u32) as u32 as u64;
    fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64;
    if (((fl_l as u32)) < ((fl_r as u32))) { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); return rax; }
    fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64; fl_r = (0xfffffffffffffffcu64) as u64;
    if !(((((fl_l as u64)) & ((fl_r as u64))) != 0)) { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; return rax; }
    rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64));
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14253e480(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x1438430b0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = (((rcx) ^ (crate::simd_ops::ru64(P0)))) as u64; fl_r = fl_l;
    rcx = ((fl_l as u64)) as u64;
    rax = rcx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14253e4a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0xffff;
    let (mut rax, mut fl_l) = (crate::simd_ops::get_rax_in(), 0u64);
    rax = ((rcx.wrapping_add(P0)) as u64) as u64;
    fl_l = ((rax).wrapping_shr(16u32)) as u64;
    rax = ((fl_l as u64)) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14253e4c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = (((rcx) & (0xffffffffffff0000u64))) as u64; fl_r = fl_l;
    rcx = ((fl_l as u64)) as u64;
    rax = rcx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14253e590(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l) = (crate::simd_ops::get_rax_in(), 0u64);
    rdx = (0u64.wrapping_sub(rdx)) as u64;
    let xa_src: u64 = rdx; let xa_old: u64 = crate::simd_ops::atomic_xadd_u64(rcx, xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64;
    rdx = (xa_old) as u64;
    rax = rdx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14253e5c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xffffffffffffffff; const P1: u64 = 0x14253b6b0; const P2: u64 = 0xd0; const P3: u64 = 0x14253b840; const P4: u64 = 0xc0; const P5: u64 = 0x14253b2a0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = ((rcx.wrapping_add(P0)) as u64) as u64;
    rdx = r8;
    rbx = (((rbx) & (0xfffffffffe000000u64))) as u64;
    rdi = r8;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rax = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64;
    fl_l = rax; fl_r = rax;
    if !(((((fl_l as u64)) & ((fl_r as u64))) != 0)) { r8 = rdi; rcx = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P3, rax, rcx, rdx, r8, r9, rsp); }
    fl_l = rax; fl_r = (crate::simd_ops::ru64(rbx.wrapping_add(P4))) as u64;
    if !((((fl_l as u64)) != ((fl_r as u64)))) { rdx = rdi; rcx = rbx; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14253e630(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xb28; const P1: u64 = 0x14253a840; const P2: u64 = 0x14253a960;
    let (mut rax, mut r10, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x38u64)) as u64; r10 = r8; rax = rdx; fl_l = r8; fl_r = (0x1000000u64) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rdx = 0x2000000; fl_l = r8; fl_r = rdx; rdx = (if (((fl_l as u64)) >= ((fl_r as u64))) { r8 } else { rdx }) as u64; r8 = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; rcx = rax; rsp = ((rsp).wrapping_add(0x38u64)) as u64; return crate::simd_ops::tail_stk(P1, rax, rcx, rdx, r8, r9, rsp); }
            2 => { fl_l = rax; fl_r = (0x4000u64) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 6; continue; } bb = 3; continue; }
            3 => { fl_l = rax; fl_r = (0x20000u64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 5; continue; } bb = 4; continue; }
            4 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r9); r8 = 0x80000; r9 = rax; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rsp = ((rsp).wrapping_add(0x38u64)) as u64; return rax; }
            5 => { fl_l = rax; fl_r = (0x1000000u64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 7; continue; } bb = 6; continue; }
            6 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r9); r8 = rax; r9 = rax; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rsp = ((rsp).wrapping_add(0x38u64)) as u64; return rax; }
            7 => { r8 = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; rdx = r10; rcx = rax; rsp = ((rsp).wrapping_add(0x38u64)) as u64; return crate::simd_ops::tail_stk(P1, rax, rcx, rdx, r8, r9, rsp); }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14253e6d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xffffffffffffffff; const P1: u64 = 0x143a095c0; const P2: u64 = 0xc0; const P3: u64 = 0x380; const P4: u64 = 0x143a09840; const P5: u64 = 0x18; const P6: u64 = 0x10; const P7: u64 = 0xd0; const P8: u64 = 0x14253b2a0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r8 = ((rcx.wrapping_add(P0)) as u64) as u64; r9 = rdx; r8 = (((r8) & (0xfffffffffe000000u64))) as u64; rax = P1; crate::simd_ops::lk_add_u64(r8.wrapping_add(P2), (1) as u64); rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P3))) as u64; rcx = ((rcx).wrapping_add(0x100u64)) as u64; fl_l = rcx; fl_r = rax; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 6; continue; } bb = 1; continue; }
            1 => { rax = P4; fl_l = rcx; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 6; continue; } bb = 2; continue; }
            2 => { rdx = 0x1; let xa_src: u64 = rdx; let xa_old: u64 = crate::simd_ops::atomic_xadd_u64(rcx.wrapping_add(P5), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rdx = (xa_old) as u64; fl_l = ((rdx).wrapping_add(1)) as u64; rdx = ((fl_l as u64)) as u64; bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(P6))) as u64; fl_l = rax; fl_r = rdx; if ((((fl_l as u64)) as i64) >= (((fl_r as u64)) as i64)) { bb = 5; continue; } bb = 4; continue; }
            4 => { fl_l = rax; let cx_src: u64 = rdx; let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u64(rcx.wrapping_add(P6), fl_l as u64, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u64)) as u64; } if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 5; continue; }
            5 => { fl_l = (crate::simd_ops::lk_add_u64(rcx, (1) as u64)) as u64; bb = 9; continue; }
            6 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(P5))) as u64; rax = ((rax).wrapping_add(1)) as u64; crate::simd_ops::wu64(rcx.wrapping_add(P5), rax); fl_l = rax; fl_r = (crate::simd_ops::ru64(rcx.wrapping_add(P6))) as u64; if ((((fl_l as u64)) as i64) <= (((fl_r as u64)) as i64)) { bb = 8; continue; } bb = 7; continue; }
            7 => { crate::simd_ops::wu64(rcx.wrapping_add(P6), rax); bb = 8; continue; }
            8 => { fl_l = (crate::simd_ops::lk_add_u64(rcx, (1) as u64)) as u64; bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru64(r8.wrapping_add(P2))) as u64; fl_l = (crate::simd_ops::ru64(r8.wrapping_add(P7))) as u64; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 11; continue; } bb = 10; continue; }
            10 => { rdx = r9; rcx = r8; return crate::simd_ops::call_fwd(P8, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            11 => { return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14253e800(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x18; const P1: u64 = 0xffffffffffffffff; const P2: u64 = 0x108; const P3: u64 = 0x4; const P4: u64 = 0xe; const P5: u64 = 0x1c; const P6: u64 = 0x142532ff0; const P7: u64 = 0xfffffffffffffff8; const P8: u64 = 0x8; const P9: u64 = 0x143a095c0; const P10: u64 = 0x1425391a0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(P0))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; rbx = r8; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 8; continue; } bb = 1; continue; }
            1 => { fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 2; continue; }
            2 => { rcx = ((r8.wrapping_add(P1)) as u64) as u64; rax = rbx; rcx = (((rcx) & (0xfffffffffe000000u64))) as u64; rax = ((rax).wrapping_sub(rcx)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rdx = ((rcx.wrapping_add(P2)) as u64) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(P3))) as u32 as u64; rdx = ((rdx).wrapping_sub(rax)) as u64; fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(P4))) as u8 as u64; fl_r = (0x2u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 5; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru32(rdx.wrapping_add(P5))) as u32 as u64; rcx = 0x80000000; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 6; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru32(rdx)) as u32 as u64; fl_l = ((rax).wrapping_shl(16u32)) as u64; rax = ((fl_l as u64)) as u64; bb = 6; continue; }
            5 => { rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            6 => { fl_l = rax; fl_r = (0x8u64) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 8; continue; } bb = 7; continue; }
            7 => { rdx = ((rax.wrapping_add(P7)) as u64) as u64; rcx = ((rbx.wrapping_add(P8)) as u64) as u64; r8 = P9; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P10, rax, rcx, rdx, r8, r9, rsp); }
            8 => { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14253e890(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143bbd998; const P1: u64 = 0x1425412f0; const P2: u64 = 0x143bbdfa8; const P3: u64 = 0x1426456c8; const P4: u64 = 0x1426452e0; const P5: u64 = 0x143bf60f8; const P6: u64 = 0x143426098; const P7: u64 = 0x142645308; const P8: u64 = 0x1434260a8; const P9: u64 = 0x142645758;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 23>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x80u64)) as u64; rbx = (crate::simd_ops::ru64(P0)) as u64; r14 = r9; rbp = r8; r12 = rdx; r15 = rcx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = rax; rdi = ((rdi).wrapping_sub(crate::simd_ops::ru64(P2))) as u64; rdi = ((rdi).wrapping_sub(rbx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rax; r9 = rsp.wrapping_add(0xc8); rax = rsp.wrapping_add(0xc0); r8 = rsp.wrapping_add(0xd0); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); rdx = rsp.wrapping_add(0xd8); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0xc0u64))) as u32 as u64; r9 = (0x346dc5d63886594bu64) as u64; rcx = (crate::simd_ops::ru32(rsp.wrapping_add(0xc4u64))) as u32 as u64; r8 = (crate::simd_ops::ru32(rsp.wrapping_add(0xccu64))) as u32 as u64; rcx = ((rcx).wrapping_shl(32u32)) as u64; rcx = (((rcx) | (rax))) as u64; r8 = ((r8).wrapping_shl(32u32)) as u64; rax = r9; let prod = (rax as u128).wrapping_mul((rcx) as u128); rax = prod as u64; rdx = (prod >> 64) as u64; fl_l = (rdx != 0) as u64; rcx = (crate::simd_ops::ru32(rsp.wrapping_add(0xc8u64))) as u32 as u64; rax = r9; r8 = (((r8) | (rcx))) as u64; rsi = rdx; let prod = (rax as u128).wrapping_mul((r8) as u128); rax = prod as u64; rdx = (prod >> 64) as u64; fl_l = (rdx != 0) as u64; rax = (crate::simd_ops::ru64(P5)) as u64; rbx = rdx; rsi = ((rsi).wrapping_shr(11u32)) as u64; rbx = ((rbx).wrapping_shr(11u32)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { rcx = P6; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P7), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rdx = P8; rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P9), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(P5, rax); bb = 4; continue; }
            3 => { rax = (crate::simd_ops::ru64(P5)) as u64; bb = 4; continue; }
            4 => { rdx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), rdx); xmm0 = 0u128; ymm0_hi = 0; let y_addr: u64 = rsp.wrapping_add(0x30u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); let y_addr: u64 = rsp.wrapping_add(0x50u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = 0x48; rdx = rsp.wrapping_add(0x30); rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; bb = 6; continue; }
            6 => { r8 = (crate::simd_ops::ru32(rsp.wrapping_add(0x34u64))) as u32 as u64; rax = 0; fl_l = 0; fl_r = 0; r9 = (0x7fffffffffffffffu64) as u64; fl_l = r15; fl_r = r15; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 7; continue; }
            7 => { fl_l = rdi; fl_r = rdi; if (((((fl_l as u64)) & ((fl_r as u64))) as i64) >= 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { rcx = ((rax as u32)) as u32 as u64; bb = 10; continue; }
            9 => { fl_l = rdi; fl_r = r9; rcx = r9; rcx = (if ((((fl_l as u64)) as i64) < (((fl_r as u64)) as i64)) { rdi } else { rcx }) as u64; bb = 10; continue; }
            10 => { crate::simd_ops::wu64(r15, rcx); bb = 11; continue; }
            11 => { fl_l = r12; fl_r = r12; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 16; continue; } bb = 12; continue; }
            12 => { fl_l = rsi; fl_r = rsi; if (((((fl_l as u64)) & ((fl_r as u64))) as i64) >= 0) { bb = 14; continue; } bb = 13; continue; }
            13 => { rcx = rax; bb = 15; continue; }
            14 => { fl_l = rsi; fl_r = r9; rcx = r9; rcx = (if ((((fl_l as u64)) as i64) < (((fl_r as u64)) as i64)) { rsi } else { rcx }) as u64; bb = 15; continue; }
            15 => { crate::simd_ops::wu64(r12, rcx); bb = 16; continue; }
            16 => { fl_l = rbp; fl_r = rbp; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 20; continue; } bb = 17; continue; }
            17 => { fl_l = rbx; fl_r = rbx; if (((((fl_l as u64)) & ((fl_r as u64))) as i64) < 0) { bb = 19; continue; } bb = 18; continue; }
            18 => { fl_l = rbx; fl_r = r9; rax = r9; rax = (if ((((fl_l as u64)) as i64) < (((fl_r as u64)) as i64)) { rbx } else { rax }) as u64; bb = 19; continue; }
            19 => { crate::simd_ops::wu64(rbp, rax); bb = 20; continue; }
            20 => { fl_l = r14; fl_r = r14; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 22; continue; } bb = 21; continue; }
            21 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; crate::simd_ops::wu64(r14, rax); bb = 22; continue; }
            22 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0xe0u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 24; continue; } bb = 23; continue; }
            23 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; crate::simd_ops::wu64(rcx, rax); bb = 24; continue; }
            24 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0xe8u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 26; continue; } bb = 25; continue; }
            25 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; crate::simd_ops::wu64(rcx, rax); bb = 26; continue; }
            26 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xf0u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 28; continue; } bb = 27; continue; }
            27 => { crate::simd_ops::wu64(rax, rdx); bb = 28; continue; }
            28 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xf8u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 30; continue; } bb = 29; continue; }
            29 => { crate::simd_ops::wu64(rax, r8); bb = 30; continue; }
            30 => { rsp = ((rsp).wrapping_add(0x80u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14253eb30(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1425462a0;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rax = (crate::simd_ops::ru64(rax)) as u64;
    rax = ((rax).wrapping_add(0x3c0u64)) as u64;
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14253eb50(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143425db0; const P1: u64 = 0x143425d60; const P2: u64 = 0x143425dc0; const P3: u64 = 0x143425d70; const P4: u64 = 0x143425e08; const P5: u64 = 0x14253fa60; const P6: u64 = 0x143425e18; const P7: u64 = 0x143425e28; const P8: u64 = 0x143425e38; const P9: u64 = 0x142537d10; const P10: u64 = 0x143425de8; const P11: u64 = 0x143425d28; const P12: u64 = 0x143a09610; const P13: u64 = 0x1425400a0; const P14: u64 = 0x143a09600; const P15: u64 = 0x143a09608; const P16: u64 = 0x143a09618; const P17: u64 = 0x1433dc2d4; const P18: u64 = 0x143425d20; const P19: u64 = 0x143425d88; const P20: u64 = 0x14341086c; const P21: u64 = 0x1426cdcb0; const P22: u64 = 0x143231690; const P23: u64 = 0x143425df8; const P24: u64 = 0x143a09630; const P25: u64 = 0x143a09620; const P26: u64 = 0x143a09628; const P27: u64 = 0x143a09638; const P28: u64 = 0x143425e6c; const P29: u64 = 0x143a09650; const P30: u64 = 0x143425e74; const P31: u64 = 0x143a09670; const P32: u64 = 0x143425e80; const P33: u64 = 0x143a09690; const P34: u64 = 0x143a09680; const P35: u64 = 0x143a09688; const P36: u64 = 0x143a09698; const P37: u64 = 0x143425d78; const P38: u64 = 0x143425e88; const P39: u64 = 0x143a095d0; const P40: u64 = 0x143a095c0; const P41: u64 = 0x143a095c8; const P42: u64 = 0x143a095d8; const P43: u64 = 0x143425d90; const P44: u64 = 0x143425d98; const P45: u64 = 0x143425e40; const P46: u64 = 0x143a096b0; const P47: u64 = 0x143a096a0; const P48: u64 = 0x143a096a8; const P49: u64 = 0x143a096b8; const P50: u64 = 0x143425e50; const P51: u64 = 0x143a09790; const P52: u64 = 0x143a09780; const P53: u64 = 0x143a09788; const P54: u64 = 0x143a09798; const P55: u64 = 0x143425e58; const P56: u64 = 0x143a095f0; const P57: u64 = 0x143a095e0; const P58: u64 = 0x143a095e8; const P59: u64 = 0x143a095f8; const P60: u64 = 0x143a096d0; const P61: u64 = 0x143a096c0; const P62: u64 = 0x143a096c8; const P63: u64 = 0x143a096d8; const P64: u64 = 0x143425e60; const P65: u64 = 0x143a097a0; const P66: u64 = 0x143425ec8; const P67: u64 = 0x143a097f0; const P68: u64 = 0x143425ed4; const P69: u64 = 0x143a097b0; const P70: u64 = 0x143425ee0; const P71: u64 = 0x143a097c0; const P72: u64 = 0x143425ee8; const P73: u64 = 0x143a097d0; const P74: u64 = 0x143425e94; const P75: u64 = 0x143a097e0; const P76: u64 = 0x143425ea0; const P77: u64 = 0x143a096f0; const P78: u64 = 0x143a096e0; const P79: u64 = 0x143a096e8; const P80: u64 = 0x143a096f8; const P81: u64 = 0x143a09808; const P82: u64 = 0x143a09800; const P83: u64 = 0x143425ea8; const P84: u64 = 0x143425d48; const P85: u64 = 0x143d01c60; const P86: u64 = 0x142538840; const P87: u64 = 0x143425eb8; const P88: u64 = 0x143425f48; const P89: u64 = 0x143bbd998; const P90: u64 = 0x1425412f0; const P91: u64 = 0x143bbdfa8; const P92: u64 = 0x1426456c8; const P93: u64 = 0x1426452e0; const P94: u64 = 0x143bf60f8; const P95: u64 = 0x143426098; const P96: u64 = 0x142645308; const P97: u64 = 0x1434260a8; const P98: u64 = 0x142645758; const P99: u64 = 0x143425f58; const P100: u64 = 0x143425f60; const P101: u64 = 0x143425f78; const P102: u64 = 0x143425ef0; const P103: u64 = 0x1431edc44; const P104: u64 = 0x143425f38;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 67>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (r8) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rdx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rcx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r12) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r13) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r15) as u64); rbp = ((rsp.wrapping_sub(0xd8u64)) as u64) as u64; rsp = ((rsp).wrapping_sub(0x1d8u64)) as u64; rdi = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), (0xffu64) as u64); rax = ((rbp.wrapping_sub(0x30u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), (rdi) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), (rax) as u64); r9 = ((P0) as u64) as u64; rax = ((P1) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), (rdi) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), (rax) as u64); r8 = ((P2) as u64) as u64; rax = ((P3) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), (rdi) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), (rax) as u64); rdx = ((rsp.wrapping_add(0x50u64)) as u64) as u64; rax = ((P4) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (rax) as u64); rcx = ((P5) as u64) as u64; rax = ((P6) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (rax) as u64); rax = ((P7) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (rax) as u64); rax = ((P8) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rax) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r9 = ((P10) as u64) as u64; r8 = ((P11) as u64) as u64; rdx = ((rsp.wrapping_add(0x50u64)) as u64) as u64; rcx = ((P5) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (crate::simd_ops::ru64(P12)) as u64; r9 = ((rsp.wrapping_add(0x50u64)) as u64) as u64; rdx = ((rdi.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rdi) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P13) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (crate::simd_ops::ru64(P14)) as u64; r9 = ((rsp.wrapping_add(0x50u64)) as u64) as u64; rdx = ((rdi.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rdi) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P13) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (crate::simd_ops::ru64(P15)) as u64; r9 = ((rsp.wrapping_add(0x50u64)) as u64) as u64; rdx = ((rdi.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rdi) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P13) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (crate::simd_ops::ru64(P16)) as u64; r9 = ((rsp.wrapping_add(0x50u64)) as u64) as u64; rdx = ((rdi.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rdi) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P13) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = (0x1u32) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rdi) as u64); rcx = ((rdx as u32)) as u32 as u64; r9 = ((rsp.wrapping_add(0x50u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P13) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r9 = ((P17) as u64) as u64; r8 = ((P18) as u64) as u64; rdx = ((rsp.wrapping_add(0x50u64)) as u64) as u64; rcx = ((P5) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = (crate::simd_ops::ru64(P15)) as u64; rsi = ((P19) as u64) as u64; fl_l = (crate::simd_ops::ru64(P14)) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) as i64) <= (((fl_r as u64)) as i64)) { bb = 2; continue; } bb = 1; continue; }
            1 => { r8 = ((P20) as u64) as u64; rdx = ((rsp.wrapping_add(0x50u64)) as u64) as u64; rcx = ((P5) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r8 = ((P21) as u64) as u64; rdx = ((rsp.wrapping_add(0x50u64)) as u64) as u64; rcx = ((P5) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r8 = ((P22) as u64) as u64; bb = 3; continue; }
            2 => { r8 = (rsi) as u64; bb = 3; continue; }
            3 => { rdx = ((rsp.wrapping_add(0x50u64)) as u64) as u64; rcx = ((P5) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r9 = ((P23) as u64) as u64; r8 = ((P11) as u64) as u64; rdx = ((rsp.wrapping_add(0x50u64)) as u64) as u64; rcx = ((P5) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (crate::simd_ops::ru64(P24)) as u64; r9 = ((rsp.wrapping_add(0x50u64)) as u64) as u64; rdx = (0x1u32) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rdi) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P13) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (crate::simd_ops::ru64(P25)) as u64; r9 = ((rsp.wrapping_add(0x50u64)) as u64) as u64; rdx = (0x1u32) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rdi) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P13) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (crate::simd_ops::ru64(P26)) as u64; r9 = ((rsp.wrapping_add(0x50u64)) as u64) as u64; rdx = (0x1u32) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rdi) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P13) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (crate::simd_ops::ru64(P27)) as u64; r9 = ((rsp.wrapping_add(0x50u64)) as u64) as u64; rdx = (0x1u32) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rdi) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P13) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = (0x1u32) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rdi) as u64); rcx = ((rdx as u32)) as u32 as u64; r9 = ((rsp.wrapping_add(0x50u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P13) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r9 = ((P17) as u64) as u64; r8 = ((P18) as u64) as u64; rdx = ((rsp.wrapping_add(0x50u64)) as u64) as u64; rcx = ((P5) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = (crate::simd_ops::ru64(P26)) as u64; fl_l = (crate::simd_ops::ru64(P25)) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) as i64) <= (((fl_r as u64)) as i64)) { bb = 5; continue; } bb = 4; continue; }
            4 => { r8 = ((P20) as u64) as u64; rdx = ((rsp.wrapping_add(0x50u64)) as u64) as u64; rcx = ((P5) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r8 = ((P21) as u64) as u64; rdx = ((rsp.wrapping_add(0x50u64)) as u64) as u64; rcx = ((P5) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r8 = ((P22) as u64) as u64; bb = 6; continue; }
            5 => { r8 = (rsi) as u64; bb = 6; continue; }
            6 => { rdx = ((rsp.wrapping_add(0x50u64)) as u64) as u64; rcx = ((P5) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r9 = ((P28) as u64) as u64; r8 = ((P11) as u64) as u64; rdx = ((rsp.wrapping_add(0x50u64)) as u64) as u64; rcx = ((P5) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (crate::simd_ops::ru64(P29)) as u64; r9 = ((rsp.wrapping_add(0x50u64)) as u64) as u64; rdx = (0x1u32) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rdi) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P13) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r8 = ((P22) as u64) as u64; rdx = ((rsp.wrapping_add(0x50u64)) as u64) as u64; rcx = ((P5) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r9 = ((P30) as u64) as u64; r8 = ((P11) as u64) as u64; rdx = ((rsp.wrapping_add(0x50u64)) as u64) as u64; rcx = ((P5) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (crate::simd_ops::ru64(P31)) as u64; r9 = ((rsp.wrapping_add(0x50u64)) as u64) as u64; rdx = (0x1u32) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rdi) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P13) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r8 = ((P22) as u64) as u64; rdx = ((rsp.wrapping_add(0x50u64)) as u64) as u64; rcx = ((P5) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r9 = ((P32) as u64) as u64; r8 = ((P11) as u64) as u64; rdx = ((rsp.wrapping_add(0x50u64)) as u64) as u64; rcx = ((P5) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (crate::simd_ops::ru64(P33)) as u64; r9 = ((rsp.wrapping_add(0x50u64)) as u64) as u64; rdx = (0x1u32) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rdi) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P13) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (crate::simd_ops::ru64(P34)) as u64; r9 = ((rsp.wrapping_add(0x50u64)) as u64) as u64; rdx = (0x1u32) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rdi) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P13) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (crate::simd_ops::ru64(P35)) as u64; r9 = ((rsp.wrapping_add(0x50u64)) as u64) as u64; rdx = (0x1u32) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rdi) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P13) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (crate::simd_ops::ru64(P36)) as u64; r9 = ((rsp.wrapping_add(0x50u64)) as u64) as u64; rdx = (0x1u32) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rdi) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P13) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = (0x1u32) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rdi) as u64); rcx = ((rdx as u32)) as u32 as u64; r9 = ((rsp.wrapping_add(0x50u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P13) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r9 = ((P17) as u64) as u64; r8 = ((P18) as u64) as u64; rdx = ((rsp.wrapping_add(0x50u64)) as u64) as u64; rcx = ((P5) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = (crate::simd_ops::ru64(P35)) as u64; fl_l = (crate::simd_ops::ru64(P34)) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) as i64) <= (((fl_r as u64)) as i64)) { bb = 8; continue; } bb = 7; continue; }
            7 => { r8 = ((P20) as u64) as u64; rdx = ((rsp.wrapping_add(0x50u64)) as u64) as u64; rcx = ((P5) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r8 = ((P37) as u64) as u64; rdx = ((rsp.wrapping_add(0x50u64)) as u64) as u64; rcx = ((P5) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r8 = ((P22) as u64) as u64; bb = 9; continue; }
            8 => { r8 = (rsi) as u64; bb = 9; continue; }
            9 => { rdx = ((rsp.wrapping_add(0x50u64)) as u64) as u64; rcx = ((P5) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r9 = ((P38) as u64) as u64; r8 = ((P11) as u64) as u64; rdx = ((rsp.wrapping_add(0x50u64)) as u64) as u64; rcx = ((P5) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (crate::simd_ops::ru64(P39)) as u64; r9 = ((rsp.wrapping_add(0x50u64)) as u64) as u64; rdx = (0xffffffffffffffffu64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rdi) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P13) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (crate::simd_ops::ru64(P40)) as u64; r9 = ((rsp.wrapping_add(0x50u64)) as u64) as u64; rdx = (0xffffffffffffffffu64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rdi) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P13) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (crate::simd_ops::ru64(P41)) as u64; r9 = ((rsp.wrapping_add(0x50u64)) as u64) as u64; rdx = (0xffffffffffffffffu64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rdi) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P13) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (crate::simd_ops::ru64(P42)) as u64; r9 = ((rsp.wrapping_add(0x50u64)) as u64) as u64; rdx = (0xffffffffffffffffu64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rdi) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P13) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r9 = ((P21) as u64) as u64; r8 = ((P43) as u64) as u64; rdx = ((rsp.wrapping_add(0x50u64)) as u64) as u64; rcx = ((P5) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = (crate::simd_ops::ru64(P40)) as u64; rbx = ((P44) as u64) as u64; fl_l = (rax) as u64 as u64; fl_r = (crate::simd_ops::ru64(P41)) as u64 as u64; rdx = ((rsp.wrapping_add(0x50u64)) as u64) as u64; r8 = (rbx) as u64; rcx = ((P5) as u64) as u64; r8 = (if ((((fl_l as u64)) as i64) <= (((fl_r as u64)) as i64)) { rsi } else { r8 }) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r9 = ((P45) as u64) as u64; r8 = ((P11) as u64) as u64; rdx = ((rsp.wrapping_add(0x50u64)) as u64) as u64; rcx = ((P5) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (crate::simd_ops::ru64(P46)) as u64; r9 = ((rsp.wrapping_add(0x50u64)) as u64) as u64; rdx = (0xffffffffffffffffu64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rdi) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P13) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (crate::simd_ops::ru64(P47)) as u64; r9 = ((rsp.wrapping_add(0x50u64)) as u64) as u64; rdx = (0xffffffffffffffffu64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rdi) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P13) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (crate::simd_ops::ru64(P48)) as u64; r9 = ((rsp.wrapping_add(0x50u64)) as u64) as u64; rdx = (0xffffffffffffffffu64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rdi) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P13) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (crate::simd_ops::ru64(P49)) as u64; r9 = ((rsp.wrapping_add(0x50u64)) as u64) as u64; rdx = (0xffffffffffffffffu64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rdi) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P13) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r9 = ((P21) as u64) as u64; r8 = ((P43) as u64) as u64; rdx = ((rsp.wrapping_add(0x50u64)) as u64) as u64; rcx = ((P5) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = (crate::simd_ops::ru64(P47)) as u64; rdx = ((rsp.wrapping_add(0x50u64)) as u64) as u64; fl_l = (rax) as u64 as u64; fl_r = (crate::simd_ops::ru64(P48)) as u64 as u64; rcx = ((P5) as u64) as u64; r8 = (rbx) as u64; r8 = (if ((((fl_l as u64)) as i64) <= (((fl_r as u64)) as i64)) { rsi } else { r8 }) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r9 = ((P50) as u64) as u64; r8 = ((P11) as u64) as u64; rdx = ((rsp.wrapping_add(0x50u64)) as u64) as u64; rcx = ((P5) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (crate::simd_ops::ru64(P51)) as u64; r9 = ((rsp.wrapping_add(0x50u64)) as u64) as u64; rdx = (0xffffffffffffffffu64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rdi) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P13) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (crate::simd_ops::ru64(P52)) as u64; r9 = ((rsp.wrapping_add(0x50u64)) as u64) as u64; rdx = (0xffffffffffffffffu64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rdi) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P13) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (crate::simd_ops::ru64(P53)) as u64; r9 = ((rsp.wrapping_add(0x50u64)) as u64) as u64; rdx = (0xffffffffffffffffu64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rdi) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P13) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (crate::simd_ops::ru64(P54)) as u64; r9 = ((rsp.wrapping_add(0x50u64)) as u64) as u64; rdx = (0xffffffffffffffffu64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rdi) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P13) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r9 = ((P21) as u64) as u64; r8 = ((P43) as u64) as u64; rdx = ((rsp.wrapping_add(0x50u64)) as u64) as u64; rcx = ((P5) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = (crate::simd_ops::ru64(P52)) as u64; rdx = ((rsp.wrapping_add(0x50u64)) as u64) as u64; fl_l = (rax) as u64 as u64; fl_r = (crate::simd_ops::ru64(P53)) as u64 as u64; rcx = ((P5) as u64) as u64; r8 = (rbx) as u64; r8 = (if ((((fl_l as u64)) as i64) <= (((fl_r as u64)) as i64)) { rsi } else { r8 }) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r9 = ((P55) as u64) as u64; r8 = ((P11) as u64) as u64; rdx = ((rsp.wrapping_add(0x50u64)) as u64) as u64; rcx = ((P5) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (crate::simd_ops::ru64(P56)) as u64; r9 = ((rsp.wrapping_add(0x50u64)) as u64) as u64; rdx = (0xffffffffffffffffu64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rdi) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P13) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (crate::simd_ops::ru64(P57)) as u64; r9 = ((rsp.wrapping_add(0x50u64)) as u64) as u64; rdx = (0xffffffffffffffffu64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rdi) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P13) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (crate::simd_ops::ru64(P58)) as u64; r9 = ((rsp.wrapping_add(0x50u64)) as u64) as u64; rdx = (0xffffffffffffffffu64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rdi) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P13) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (crate::simd_ops::ru64(P59)) as u64; r9 = ((rsp.wrapping_add(0x50u64)) as u64) as u64; rdx = (0xffffffffffffffffu64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rdi) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P13) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r9 = ((P21) as u64) as u64; r8 = ((P43) as u64) as u64; rdx = ((rsp.wrapping_add(0x50u64)) as u64) as u64; rcx = ((P5) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = (crate::simd_ops::ru64(P57)) as u64; rdx = ((rsp.wrapping_add(0x50u64)) as u64) as u64; fl_l = (rax) as u64 as u64; fl_r = (crate::simd_ops::ru64(P58)) as u64 as u64; rcx = ((P5) as u64) as u64; r8 = (rbx) as u64; r8 = (if ((((fl_l as u64)) as i64) <= (((fl_r as u64)) as i64)) { rsi } else { r8 }) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r9 = ((P45) as u64) as u64; r8 = ((P11) as u64) as u64; rdx = ((rsp.wrapping_add(0x50u64)) as u64) as u64; rcx = ((P5) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (crate::simd_ops::ru64(P60)) as u64; r9 = ((rsp.wrapping_add(0x50u64)) as u64) as u64; rdx = (0xffffffffffffffffu64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rdi) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P13) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (crate::simd_ops::ru64(P61)) as u64; r9 = ((rsp.wrapping_add(0x50u64)) as u64) as u64; rdx = (0xffffffffffffffffu64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rdi) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P13) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (crate::simd_ops::ru64(P62)) as u64; r9 = ((rsp.wrapping_add(0x50u64)) as u64) as u64; rdx = (0xffffffffffffffffu64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rdi) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P13) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (crate::simd_ops::ru64(P63)) as u64; r9 = ((rsp.wrapping_add(0x50u64)) as u64) as u64; rdx = (0xffffffffffffffffu64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rdi) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P13) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r9 = ((P21) as u64) as u64; r8 = ((P43) as u64) as u64; rdx = ((rsp.wrapping_add(0x50u64)) as u64) as u64; rcx = ((P5) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = (crate::simd_ops::ru64(P61)) as u64; rdx = ((rsp.wrapping_add(0x50u64)) as u64) as u64; fl_l = (rax) as u64 as u64; fl_r = (crate::simd_ops::ru64(P62)) as u64 as u64; rcx = ((P5) as u64) as u64; r8 = (rbx) as u64; r8 = (if ((((fl_l as u64)) as i64) <= (((fl_r as u64)) as i64)) { rsi } else { r8 }) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r9 = ((P64) as u64) as u64; r8 = ((P11) as u64) as u64; rdx = ((rsp.wrapping_add(0x50u64)) as u64) as u64; rcx = ((P5) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (crate::simd_ops::ru64(P65)) as u64; r9 = ((rsp.wrapping_add(0x50u64)) as u64) as u64; rdx = (0xffffffffffffffffu64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rdi) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P13) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r8 = ((P22) as u64) as u64; rdx = ((rsp.wrapping_add(0x50u64)) as u64) as u64; rcx = ((P5) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r9 = ((P66) as u64) as u64; r8 = ((P11) as u64) as u64; rdx = ((rsp.wrapping_add(0x50u64)) as u64) as u64; rcx = ((P5) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (crate::simd_ops::ru64(P67)) as u64; r9 = ((rsp.wrapping_add(0x50u64)) as u64) as u64; rdx = (0xffffffffffffffffu64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rdi) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P13) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r8 = ((P22) as u64) as u64; rdx = ((rsp.wrapping_add(0x50u64)) as u64) as u64; rcx = ((P5) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r9 = ((P68) as u64) as u64; r8 = ((P11) as u64) as u64; rdx = ((rsp.wrapping_add(0x50u64)) as u64) as u64; rcx = ((P5) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (crate::simd_ops::ru64(P69)) as u64; r9 = ((rsp.wrapping_add(0x50u64)) as u64) as u64; rdx = (0xffffffffffffffffu64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rdi) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P13) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r8 = ((P22) as u64) as u64; rdx = ((rsp.wrapping_add(0x50u64)) as u64) as u64; rcx = ((P5) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r9 = ((P70) as u64) as u64; r8 = ((P11) as u64) as u64; rdx = ((rsp.wrapping_add(0x50u64)) as u64) as u64; rcx = ((P5) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (crate::simd_ops::ru64(P71)) as u64; r9 = ((rsp.wrapping_add(0x50u64)) as u64) as u64; rdx = (0xffffffffffffffffu64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rdi) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P13) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r8 = ((P22) as u64) as u64; rdx = ((rsp.wrapping_add(0x50u64)) as u64) as u64; rcx = ((P5) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r9 = ((P72) as u64) as u64; r8 = ((P11) as u64) as u64; rdx = ((rsp.wrapping_add(0x50u64)) as u64) as u64; rcx = ((P5) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (crate::simd_ops::ru64(P73)) as u64; r9 = ((rsp.wrapping_add(0x50u64)) as u64) as u64; rdx = (0xffffffffffffffffu64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rdi) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P13) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r8 = ((P22) as u64) as u64; rdx = ((rsp.wrapping_add(0x50u64)) as u64) as u64; rcx = ((P5) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r9 = ((P74) as u64) as u64; r8 = ((P11) as u64) as u64; rdx = ((rsp.wrapping_add(0x50u64)) as u64) as u64; rcx = ((P5) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (crate::simd_ops::ru64(P75)) as u64; r9 = ((rsp.wrapping_add(0x50u64)) as u64) as u64; rdx = (0xffffffffffffffffu64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rdi) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P13) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r8 = ((P22) as u64) as u64; rdx = ((rsp.wrapping_add(0x50u64)) as u64) as u64; rcx = ((P5) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r9 = ((P76) as u64) as u64; r8 = ((P11) as u64) as u64; rdx = ((rsp.wrapping_add(0x50u64)) as u64) as u64; rcx = ((P5) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (crate::simd_ops::ru64(P77)) as u64; r9 = ((rsp.wrapping_add(0x50u64)) as u64) as u64; rdx = (0xffffffffffffffffu64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rdi) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P13) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (crate::simd_ops::ru64(P78)) as u64; r9 = ((rsp.wrapping_add(0x50u64)) as u64) as u64; rdx = (0xffffffffffffffffu64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rdi) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P13) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (crate::simd_ops::ru64(P79)) as u64; r9 = ((rsp.wrapping_add(0x50u64)) as u64) as u64; rdx = (0xffffffffffffffffu64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rdi) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P13) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (crate::simd_ops::ru64(P80)) as u64; r9 = ((rsp.wrapping_add(0x50u64)) as u64) as u64; rdx = (0xffffffffffffffffu64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rdi) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P13) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r9 = ((P21) as u64) as u64; r8 = ((P43) as u64) as u64; rdx = ((rsp.wrapping_add(0x50u64)) as u64) as u64; rcx = ((P5) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = (crate::simd_ops::ru64(P78)) as u64; rdx = ((rsp.wrapping_add(0x50u64)) as u64) as u64; fl_l = (rax) as u64 as u64; fl_r = (crate::simd_ops::ru64(P79)) as u64 as u64; rcx = ((P5) as u64) as u64; rbx = (if ((((fl_l as u64)) as i64) <= (((fl_r as u64)) as i64)) { rsi } else { rbx }) as u64; r8 = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (crate::simd_ops::ru64(P81)) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { r8 = (rdi) as u64; bb = 12; continue; }
            11 => { rax = (crate::simd_ops::ru64(P82)) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_add(rax)) as u64; rdx = if (rax as i64) < 0 { 0xffffffffffffffffu64 } else { 0u64 }; let d = (rcx) as i64 as i128; let n = (((rdx as u128) << 64) | (rax as u128)) as i128; if d != 0 { rax = n.wrapping_div(d) as i64 as u64; rdx = n.wrapping_rem(d) as i64 as u64; } r8 = (rax) as u64; bb = 12; continue; }
            12 => { rax = (0x6666666666666667u64) as u64; r9 = ((P83) as u64) as u64; let prod = (rax as i64 as i128).wrapping_mul((r8) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rdx = ((((rdx) as i64).wrapping_shr(2u32) as u64)) as u64; rcx = (rdx) as u64; rcx = ((rcx).wrapping_shr(63u32)) as u64; rdx = ((rdx).wrapping_add(rcx)) as u64; rcx = ((rdx.wrapping_add(rdx.wrapping_mul(4))) as u32) as u32 as u64; rcx = (((rcx as u32)).wrapping_add((rcx as u32))) as u32 as u64; r8 = (((r8 as u32)).wrapping_sub((rcx as u32))) as u32 as u64; rcx = ((P5) as u64) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((r8 as u32)) as u32); r8 = ((P84) as u64) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rdx as u32)) as u32); rdx = ((rsp.wrapping_add(0x50u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = (crate::simd_ops::ru64(P85)) as u64; fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 14; continue; } bb = 13; continue; }
            13 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P86) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 14; continue; }
            14 => { r9 = ((P87) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rax) as u64); r8 = ((P88) as u64) as u64; rdx = ((rsp.wrapping_add(0x50u64)) as u64) as u64; rcx = ((P5) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rbx = (crate::simd_ops::ru64(P89)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P90) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rsi = (rax) as u64; rsi = ((rsi).wrapping_sub(rbx)) as u64; rsi = ((rsi).wrapping_sub(crate::simd_ops::ru64(P91))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P92)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (rax) as u64; r9 = ((rbp.wrapping_add(0x128u64)) as u64) as u64; rax = ((rbp.wrapping_add(0x130u64)) as u64) as u64; r8 = ((rbp.wrapping_add(0x120u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rax) as u64); rdx = ((rbp.wrapping_add(0x138u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P93)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x130u64))) as u32 as u64; r8 = (0x346dc5d63886594bu64) as u64; rcx = (crate::simd_ops::ru32(rbp.wrapping_add(0x134u64))) as u32 as u64; rcx = ((rcx).wrapping_shl(32u32)) as u64; rcx = (((rcx) | (rax))) as u64; rax = (r8) as u64; let prod = (rax as u128).wrapping_mul((rcx) as u128); rax = prod as u64; rdx = (prod >> 64) as u64; fl_l = (rdx != 0) as u64; rcx = (crate::simd_ops::ru32(rbp.wrapping_add(0x128u64))) as u32 as u64; rax = (r8) as u64; r14 = (rdx) as u64; rdx = (crate::simd_ops::ru32(rbp.wrapping_add(0x12cu64))) as u32 as u64; rdx = ((rdx).wrapping_shl(32u32)) as u64; rdx = (((rdx) | (rcx))) as u64; r14 = ((r14).wrapping_shr(11u32)) as u64; let prod = (rax as u128).wrapping_mul((rdx) as u128); rax = prod as u64; rdx = (prod >> 64) as u64; fl_l = (rdx != 0) as u64; rax = (crate::simd_ops::ru64(P94)) as u64; r15 = (rdx) as u64; r15 = ((r15).wrapping_shr(11u32)) as u64; fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 18; continue; } bb = 15; continue; }
            15 => { rcx = ((P95) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P96)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 16; continue; }
            16 => { rdx = ((P97) as u64) as u64; rcx = (rax) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P98)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu64(P94, (rax) as u64); bb = 18; continue; }
            17 => { rax = (crate::simd_ops::ru64(P94)) as u64; bb = 18; continue; }
            18 => { r12 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rbp.wrapping_sub(0x40u64), (r12) as u64); xmm0 = 0u128; ymm0_hi = 0; let y_addr: u64 = rbp.wrapping_sub(0x80u64); core::ptr::write_unaligned(y_addr as *mut u128, xmm0); core::ptr::write_unaligned(y_addr.wrapping_add(16) as *mut u128, ymm0_hi); let y_addr: u64 = rbp.wrapping_sub(0x60u64); core::ptr::write_unaligned(y_addr as *mut u128, xmm0); core::ptr::write_unaligned(y_addr.wrapping_add(16) as *mut u128, ymm0_hi); fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 20; continue; } bb = 19; continue; }
            19 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P92)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (rax) as u64; r8 = ((r12.wrapping_add(0x48u64)) as u32) as u32 as u64; rdx = ((rbp.wrapping_sub(0x80u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P94)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r12 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x40u64))) as u64; bb = 20; continue; }
            20 => { r13 = (crate::simd_ops::ru32(rbp.wrapping_sub(0x7cu64))) as u32 as u64; rax = (0x7fffffffffffffffu64) as u64; fl_l = (rsi) as u64 as u64; fl_r = (rsi) as u64 as u64; if (((((fl_l as u64)) & ((fl_r as u64))) as i64) >= 0) { bb = 22; continue; } bb = 21; continue; }
            21 => { r8 = (rdi) as u64; bb = 23; continue; }
            22 => { fl_l = (rsi) as u64 as u64; fl_r = (rax) as u64 as u64; r8 = (rax) as u64; r8 = (if ((((fl_l as u64)) as i64) < (((fl_r as u64)) as i64)) { rsi } else { r8 }) as u64; bb = 23; continue; }
            23 => { fl_l = (r14) as u64 as u64; fl_r = (r14) as u64 as u64; if (((((fl_l as u64)) & ((fl_r as u64))) as i64) >= 0) { bb = 25; continue; } bb = 24; continue; }
            24 => { rbx = (rdi) as u64; bb = 26; continue; }
            25 => { fl_l = (r14) as u64 as u64; fl_r = (rax) as u64 as u64; rbx = (rax) as u64; rbx = (if ((((fl_l as u64)) as i64) < (((fl_r as u64)) as i64)) { r14 } else { rbx }) as u64; bb = 26; continue; }
            26 => { fl_l = (r15) as u64 as u64; fl_r = (r15) as u64 as u64; if (((((fl_l as u64)) & ((fl_r as u64))) as i64) < 0) { bb = 28; continue; } bb = 27; continue; }
            27 => { fl_l = (r15) as u64 as u64; fl_r = (rax) as u64 as u64; rdi = (rax) as u64; rdi = (if ((((fl_l as u64)) as i64) < (((fl_r as u64)) as i64)) { r15 } else { rdi }) as u64; bb = 28; continue; }
            28 => { rcx = (r8) as u64; r9 = ((P99) as u64) as u64; rsi = (0x624dd2f1a9fbe77u64) as u64; rax = (rsi) as u64; let prod = (rax as u128).wrapping_mul((r8) as u128); rax = prod as u64; rdx = (prod >> 64) as u64; fl_l = (rdx != 0) as u64; rcx = ((rcx).wrapping_sub(rdx)) as u64; rcx = ((rcx).wrapping_shr(1u32)) as u64; rcx = ((rcx).wrapping_add(rdx)) as u64; rdx = ((rsp.wrapping_add(0x50u64)) as u64) as u64; rcx = ((rcx).wrapping_shr(9u32)) as u64; rax = (((rcx) as i64).wrapping_mul((0x3e8u64) as i64) as u64) as u64; r8 = ((r8).wrapping_sub(rax)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (r8) as u64); r8 = ((P100) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rcx) as u64); rcx = ((P5) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu32(rsp.wrapping_add(0x40u64), ((r13 as u32)) as u32); r9 = ((P101) as u64) as u64; r8 = (rdi) as u64; rcx = (rbx) as u64; rax = (rsi) as u64; let prod = (rax as u128).wrapping_mul((rdi) as u128); rax = prod as u64; rdx = (prod >> 64) as u64; fl_l = (rdx != 0) as u64; rax = (rsi) as u64; r8 = ((r8).wrapping_sub(rdx)) as u64; r8 = ((r8).wrapping_shr(1u32)) as u64; r8 = ((r8).wrapping_add(rdx)) as u64; let prod = (rax as u128).wrapping_mul((rbx) as u128); rax = prod as u64; rdx = (prod >> 64) as u64; fl_l = (rdx != 0) as u64; r8 = ((r8).wrapping_shr(9u32)) as u64; rcx = ((rcx).wrapping_sub(rdx)) as u64; rax = (((r8) as i64).wrapping_mul((0x3e8u64) as i64) as u64) as u64; rcx = ((rcx).wrapping_shr(1u32)) as u64; rcx = ((rcx).wrapping_add(rdx)) as u64; rdi = ((rdi).wrapping_sub(rax)) as u64; rcx = ((rcx).wrapping_shr(9u32)) as u64; rdx = ((rsp.wrapping_add(0x50u64)) as u64) as u64; rax = (((rcx) as i64).wrapping_mul((0x3e8u64) as i64) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (rdi) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (r8) as u64); rbx = ((rbx).wrapping_sub(rax)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (rbx) as u64); r8 = ((P102) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rcx) as u64); rcx = ((P5) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x78u64))) as u64; rbx = ((P103) as u64) as u64; r9 = ((rsp.wrapping_add(0x50u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rbx) as u64); rdx = (0x1u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P13) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = (r12) as u64 as u64; fl_r = (r12) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 30; continue; } bb = 29; continue; }
            29 => { r8 = ((P104) as u64) as u64; rdx = ((rsp.wrapping_add(0x50u64)) as u64) as u64; rcx = ((P5) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r9 = ((rsp.wrapping_add(0x50u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rbx) as u64); rdx = (0x1u32) as u32 as u64; rcx = (r12) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P13) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 30; continue; }
            30 => { r8 = ((P22) as u64) as u64; rdx = ((rsp.wrapping_add(0x50u64)) as u64) as u64; rcx = ((P5) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rsp = ((rsp).wrapping_add(0x1d8u64)) as u64; r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r13 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r12 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rsi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14253fa60(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x20; const P1: u64 = 0x18; const P2: u64 = 0x14253faf0; const P3: u64 = 0x10; const P4: u64 = 0x1;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rdx; rsi = rcx; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rdi); rdi = ((crate::simd_ops::ru8(rcx)) as u32) as u32 as u64; fl_l = ((rdi as u8)) as u8 as u64; fl_r = ((rdi as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 9; continue; } bb = 3; continue; }
            3 => { bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64; fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64; fl_r = rax; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rcx = rbx; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64; rax = (crate::simd_ops::ru64(rbx.wrapping_add(P3))) as u64; crate::simd_ops::wu8(rcx.wrapping_add(rax), ((rdi as u8)) as u8); crate::simd_ops::lk_add_u64(rbx.wrapping_add(P1), (1) as u64); fl_l = ((rdi as u8)) as u8 as u64; fl_r = (0xau8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 8; continue; } bb = 7; continue; }
            7 => { rcx = rbx; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 8; continue; }
            8 => { rdi = ((crate::simd_ops::ru8(rsi.wrapping_add(P4))) as u32) as u32 as u64; rsi = ((rsi).wrapping_add(1)) as u64; fl_l = ((rdi as u8)) as u8 as u64; fl_r = ((rdi as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 4; continue; } bb = 9; continue; }
            9 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; bb = 10; continue; }
            10 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); bb = 11; continue; }
            11 => { return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14253faf0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x18; const P2: u64 = 0x8; const P3: u64 = 0x142646870; const P4: u64 = 0x142537f50; const P5: u64 = 0x143a632e8; const P6: u64 = 0x142538090; const P7: u64 = 0x142537f30;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rbx = rcx; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; crate::simd_ops::wu8(rdx.wrapping_add(rax), (0x0u8) as u8); rdi = (crate::simd_ops::ru64(rcx)) as u64; rsi = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rbp = (crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { rcx = 0x1; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = rdi; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 4; continue; } bb = 2; continue; }
            2 => { rcx = 0x2; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = rdi; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rdx = rbp; rcx = rsi; rax = crate::simd_ops::call_stk(rdi, rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            4 => { rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(P5)) as u64; rcx = P6; fl_l = rax; fl_r = rax; rax = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rcx } else { rax }) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rsi; rax = crate::simd_ops::call_stk(rax, rax, rcx, rdx, r8, r9, rsp); rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            6 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(P1), (0x0u64) as u64); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14253fba0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143425d60; const P1: u64 = 0x143425db0; const P2: u64 = 0x143425d70; const P3: u64 = 0x143425dc0; const P4: u64 = 0x143425e08; const P5: u64 = 0x14253fa60; const P6: u64 = 0x143425e18; const P7: u64 = 0x143425e28; const P8: u64 = 0x143425e38; const P9: u64 = 0x142537d10;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x58u64)) as u64;
    rax = P0;
    crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rax);
    r9 = P1;
    rax = P2;
    crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rax);
    r8 = P3;
    rax = P4;
    crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rax);
    rcx = P5;
    rax = P6;
    crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax);
    rax = P7;
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax);
    rax = P8;
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax);
    rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x58u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14253fc20(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x143425ea8; const P2: u64 = 0x14253fa60; const P3: u64 = 0x143425d48; const P4: u64 = 0x142537d10;
    let (mut rax, mut r10, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x38u64)) as u64;
    r8 = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    r10 = r9;
    fl_l = r8; fl_r = r8;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_add(rax)) as u64; rdx = if (rax as i64) < 0 { 0xffffffffffffffffu64 } else { 0u64 }; let d = (r8) as i64 as i128; let n = (((rdx as u128) << 64) | (rax as u128)) as i128; if d != 0 { rax = n.wrapping_div(d) as i64 as u64; rdx = n.wrapping_rem(d) as i64 as u64; } r8 = rax; }
    rax = (0x6666666666666667u64) as u64;
    r9 = P1;
    let prod = (rax as i64 as i128).wrapping_mul((r8) as i64 as i128) as u128;
    rax = prod as u64; rdx = (prod >> 64) as u64;
    rdx = ((((rdx) as i64).wrapping_shr(2u32) as u64)) as u64;
    rcx = rdx;
    rcx = ((rcx).wrapping_shr(63u32)) as u64;
    rdx = ((rdx).wrapping_add(rcx)) as u64;
    rcx = ((rdx.wrapping_add(rdx.wrapping_mul(4))) as u32) as u32 as u64;
    rcx = (((rcx as u32)).wrapping_add((rcx as u32))) as u32 as u64;
    r8 = (((r8 as u32)).wrapping_sub((rcx as u32))) as u32 as u64;
    rcx = P2;
    crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((r8 as u32)) as u32);
    r8 = P3;
    crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rdx as u32)) as u32);
    rdx = r10;
    rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x38u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14253fca0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143425d28; const P1: u64 = 0x14253fa60; const P2: u64 = 0x142537d10; const P3: u64 = 0x1425400a0; const P4: u64 = 0x143231690;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x30u64)) as u64;
    rdi = r9;
    r8 = P0;
    r9 = rdx;
    rbx = rcx;
    rdx = rdi;
    rcx = P1;
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    rcx = (crate::simd_ops::ru64(rbx)) as u64;
    r9 = rdi;
    rdx = (0xffffffffffffffffu64) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (0x0u64) as u64);
    rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp);
    r8 = P4;
    rdx = rdi;
    rcx = P1;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64;
    rsp = ((rsp).wrapping_add(0x30u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(P2, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14253fd10(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143425d28; const P1: u64 = 0x14253fa60; const P2: u64 = 0x142537d10; const P3: u64 = 0x10; const P4: u64 = 0x1425400a0; const P5: u64 = 0x143231690;
    let (mut rax, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x30u64)) as u64;
    rdi = rcx;
    r8 = P0;
    r9 = rdx;
    rcx = P1;
    rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64;
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64;
    rdx = 0x1;
    rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P3))) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (0x0u64) as u64);
    rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp);
    rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64;
    r8 = P5;
    rcx = P1;
    rsp = ((rsp).wrapping_add(0x30u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(P2, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14253fd80(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14253fdb0;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x38u64)) as u64;
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (0x0u64) as u64);
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax);
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x38u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14253fdb0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143425d28; const P1: u64 = 0x14253fa60; const P2: u64 = 0x142537d10; const P3: u64 = 0x1425400a0; const P4: u64 = 0x1433dc2d4; const P5: u64 = 0x143425d20; const P6: u64 = 0x14341086c; const P7: u64 = 0x143425d78; const P8: u64 = 0x143425d88; const P9: u64 = 0x1426cdcb0; const P10: u64 = 0x143425d90; const P11: u64 = 0x143425d98; const P12: u64 = 0x143425d40; const P13: u64 = 0x143231690;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rbp = r8; rdi = rcx; r8 = P0; r9 = rdx; rcx = P1; rdx = rbx; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; r9 = rbx; fl_l = rbp; fl_r = rbp; if (((((fl_l as u64)) & ((fl_r as u64))) as i64) <= 0) { bb = 7; continue; } bb = 1; continue; }
            1 => { rsi = 0; fl_l = 0; fl_r = 0; rdx = rbp; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rdi)) as u64; r9 = rbx; rdx = rbp; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; r9 = rbx; rdx = rbp; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x18u64))) as u64; r9 = rbx; rdx = rbp; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); r9 = rbx; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); rdx = ((rsi.wrapping_add(0x1u64)) as u32) as u32 as u64; rcx = rbp; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); fl_l = rbp; fl_r = (0x1u64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 2; continue; }
            2 => { r9 = P4; rdx = rbx; r8 = P5; rcx = P1; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            3 => { rcx = (crate::simd_ops::ru64(rdi)) as u64; r9 = rbx; rdx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; fl_l = (crate::simd_ops::ru64(rdi)) as u64; fl_r = rax; if ((((fl_l as u64)) as i64) <= (((fl_r as u64)) as i64)) { bb = 6; continue; } bb = 5; continue; }
            5 => { r8 = P6; rdx = rbx; rcx = P1; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); r8 = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rax = P7; fl_l = r8; fl_r = r8; rcx = P1; rdx = rbx; r8 = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rax } else { r8 }) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 13; continue; }
            6 => { r8 = P8; bb = 14; continue; }
            7 => { if (((((fl_l as u64)) & ((fl_r as u64))) as i64) >= 0) { bb = 12; continue; } bb = 8; continue; }
            8 => { rsi = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); rdx = rsi.wrapping_sub(0x1); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rdi)) as u64; rdx = rsi.wrapping_sub(0x1); r9 = rbx; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; rdx = rsi.wrapping_sub(0x1); r9 = rbx; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x18u64))) as u64; rdx = rsi.wrapping_sub(0x1); r9 = rbx; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); fl_l = rbp; fl_r = (0xffffffffffffffffu64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 10; continue; } bb = 9; continue; }
            9 => { r9 = P9; rdx = rbx; r8 = P10; rcx = P1; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 11; continue; }
            10 => { rcx = rbp; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); rcx = (0u64.wrapping_sub(rcx)) as u64; r9 = rbx; rdx = 0x1; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rdi)) as u64; r9 = rbx; rdx = if (rax as i64) < 0 { 0xffffffffffffffffu64 } else { 0u64 }; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); let d = (rbp) as i64 as i128; let n = (((rdx as u128) << 64) | (rax as u128)) as i128; if d != 0 { rax = n.wrapping_div(d) as i64 as u64; rdx = n.wrapping_rem(d) as i64 as u64; } rdx = 0; fl_l = 0; fl_r = 0; rax = (0u64.wrapping_sub(rax)) as u64; rcx = rax; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 11; continue; }
            11 => { rcx = (crate::simd_ops::ru64(rdi)) as u64; rax = P8; fl_l = rcx; fl_r = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; r8 = P11; r8 = (if ((((fl_l as u64)) as i64) <= (((fl_r as u64)) as i64)) { rax } else { r8 }) as u64; bb = 14; continue; }
            12 => { rsi = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); rdx = ((rsi.wrapping_add(0x1u64)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rdi)) as u64; rdx = ((rsi.wrapping_add(0x1u64)) as u32) as u32 as u64; r9 = rbx; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); r9 = P4; rdx = rbx; r8 = P12; rcx = P1; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x18u64))) as u64; rdx = ((rsi.wrapping_add(0x1u64)) as u32) as u32 as u64; r9 = rbx; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 13; continue; }
            13 => { r8 = P13; bb = 14; continue; }
            14 => { rdx = rbx; rcx = P1; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P2, rax, rcx, rdx, r8, r9, rsp); }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142540030(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1433dc2d4; const P1: u64 = 0x143425d20; const P2: u64 = 0x14253fa60; const P3: u64 = 0x142537d10; const P4: u64 = 0x1425400a0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x38u64)) as u64;
    rax = r9;
    fl_l = rdx; fl_r = (0x1u64) as u64;
    if !((((fl_l as u64)) != ((fl_r as u64)))) { r9 = P0; rdx = rax; r8 = P1; rcx = P2; rsp = ((rsp).wrapping_add(0x38u64)) as u64; return crate::simd_ops::tail_stk(P3, rax, rcx, rdx, r8, r9, rsp); }
    rdx = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (0x0u64) as u64);
    rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x38u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142540080(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1425400a0;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x38u64)) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (0x0u64) as u64);
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x38u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425400a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1433dc2d4; const P1: u64 = 0x1433d9e0c; const P2: u64 = 0x1426cdcb0; const P3: u64 = 0x143425d30; const P4: u64 = 0x1405eba60; const P5: u64 = 0x143425d3c; const P6: u64 = 0x143425d08; const P7: u64 = 0x1433dd120; const P8: u64 = 0x14341284c; const P9: u64 = 0x14336ec7c; const P10: u64 = 0x143425d10; const P11: u64 = 0x143425d20; const P12: u64 = 0x14253fa60; const P13: u64 = 0x142537d10;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), r8); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x50u64)) as u64; fl_l = rdx; fl_r = rdx; crate::simd_ops::wu8(rsp.wrapping_add(0x30u64), (0x0u8) as u8); rax = P0; r10 = rcx; r11 = P1; rcx = 0x3e8; r11 = (if (((((fl_l as u64)) & ((fl_r as u64))) as i64) <= 0) { rax } else { r11 }) as u64; rsi = r9; rax = 0x400; rdi = rdx; rax = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { (rcx as u32) } else { (rax as u32) }) as u32 as u64; if (((((fl_l as u64)) & ((fl_r as u64))) as i64) <= 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { r10 = (((r10) as i64).wrapping_mul((rdi) as i64) as u64) as u64; bb = 2; continue; }
            2 => { rbx = r10; fl_l = 0; fl_r = rbx; rbx = (0u64.wrapping_sub(fl_r as u64)) as u64; rbx = (if (((((fl_l as u64)).wrapping_sub((fl_r as u64))) as i64) < 0) { r10 } else { rbx }) as u64; fl_l = rbx; fl_r = rax; if ((((fl_l as u64)) as i64) >= (((fl_r as u64)) as i64)) { bb = 6; continue; } bb = 3; continue; }
            3 => { fl_l = r10; fl_r = (0x1u64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 5; continue; } bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::ru8(r11)) as u8 as u64; fl_r = (0x42u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 7; continue; } bb = 5; continue; }
            5 => { fl_l = r10; fl_r = r10; rax = P2; r9 = ((r10 as u32)) as u32 as u64; r8 = P3; r11 = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rax } else { r11 }) as u64; rcx = rsp.wrapping_add(0x30); rdx = 0x20; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r11); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 7; continue; }
            6 => { rcx = rax; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), r11); rcx = (((rcx) as i64).wrapping_mul((rax) as i64) as u64) as u64; r9 = P5; fl_l = rbx; fl_r = rcx; r8 = rcx; r8 = (if ((((fl_l as u64)) as i64) < (((fl_r as u64)) as i64)) { rax } else { r8 }) as u64; rdx = r8; rdx = (((rdx) as i64).wrapping_mul((rax) as i64) as u64) as u64; fl_l = rbx; fl_r = rcx; rax = P6; rax = (if ((((fl_l as u64)) as i64) < (((fl_r as u64)) as i64)) { r9 } else { rax }) as u64; fl_l = rbx; fl_r = rdx; r9 = P7; r9 = (if ((((fl_l as u64)) as i64) < (((fl_r as u64)) as i64)) { rax } else { r9 }) as u64; rdx = (if ((((fl_l as u64)) as i64) < (((fl_r as u64)) as i64)) { r8 } else { rdx }) as u64; rax = (0xcccccccccccccccdu64) as u64; r8 = P8; let prod = (rax as u128).wrapping_mul((rdx) as u128); rax = prod as u64; rdx = (prod >> 64) as u64; fl_l = (rdx != 0) as u64; rax = r10; rcx = rdx; rdx = if (rax as i64) < 0 { 0xffffffffffffffffu64 } else { 0u64 }; rcx = ((rcx).wrapping_shr(3u32)) as u64; let d = (rcx) as i64 as i128; let n = (((rdx as u128) << 64) | (rax as u128)) as i128; if d != 0 { rax = n.wrapping_div(d) as i64 as u64; rdx = n.wrapping_rem(d) as i64 as u64; } rcx = P9; fl_l = rdi; fl_r = rdi; rbx = rax; rdx = 0x8; rax = P2; rax = (if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { rcx } else { rax }) as u64; rcx = rsp.wrapping_add(0x70); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rcx = rsp.wrapping_add(0x70); rax = (0x6666666666666667u64) as u64; let prod = (rax as i64 as i128).wrapping_mul((rbx) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rcx); r8 = P10; rdx = ((((rdx) as i64).wrapping_shr(2u32) as u64)) as u64; rcx = rsp.wrapping_add(0x30); rax = rdx; rax = ((rax).wrapping_shr(63u32)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; r9 = ((rdx as u32)) as u32 as u64; rax = ((rdx.wrapping_add(rdx.wrapping_mul(4))) as u32) as u32 as u64; rdx = 0x20; rax = (((rax as u32)).wrapping_add((rax as u32))) as u32 as u64; rbx = (((rbx as u32)).wrapping_sub((rax as u32))) as u32 as u64; rax = ((rbx as u32)) as u32 as u64; fl_l = 0; fl_r = ((rax as u32)) as u32 as u64; rax = (0u32.wrapping_sub(fl_r as u32)) as u32 as u64; rax = (if (((((fl_l as u32)).wrapping_sub((fl_r as u32))) as i32) < 0) { (rbx as u32) } else { (rax as u32) }) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rax as u32)) as u32); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 7; continue; }
            7 => { r8 = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; rax = P11; fl_l = r8; fl_r = r8; r9 = rsp.wrapping_add(0x30); rdx = rsi; rcx = P12; r8 = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rax } else { r8 }) as u64; rax = crate::simd_ops::call_stk(P13, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rsp = ((rsp).wrapping_add(0x50u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142540260(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x143a095c0; const P1: u64 = 0x143a095d8; const P2: u64 = 0x143a095c8; const P3: u64 = 0x143a095d0; const P4: u64 = 0x143a095e0; const P5: u64 = 0x143a095f8; const P6: u64 = 0x143a095e8; const P7: u64 = 0x143a095f0; const P8: u64 = 0x143a09600; const P9: u64 = 0x143a09618; const P10: u64 = 0x143a09608; const P11: u64 = 0x143a09610; const P12: u64 = 0x143a09620; const P13: u64 = 0x143a09638; const P14: u64 = 0x143a09628; const P15: u64 = 0x143a09630; const P16: u64 = 0x143a09640; const P17: u64 = 0x143a09658; const P18: u64 = 0x143a09648; const P19: u64 = 0x143a09650; const P20: u64 = 0x143a09660; const P21: u64 = 0x143a09678; const P22: u64 = 0x143a09668; const P23: u64 = 0x143a09670; const P24: u64 = 0x143a09680; const P25: u64 = 0x143a09698; const P26: u64 = 0x143a09688; const P27: u64 = 0x143a09690; const P28: u64 = 0x143a096c0; const P29: u64 = 0x143a096d8; const P30: u64 = 0x143a096c8; const P31: u64 = 0x143a096d0; const P32: u64 = 0x143a096a0; const P33: u64 = 0x143a096b8; const P34: u64 = 0x143a096a8; const P35: u64 = 0x143a096b0; const P36: u64 = 0x143a096e0; const P37: u64 = 0x143a096f8; const P38: u64 = 0x143a096e8; const P39: u64 = 0x143a096f0; const P40: u64 = 0x143a09760; const P41: u64 = 0x143a09778; const P42: u64 = 0x143a09768; const P43: u64 = 0x143a09770; const P44: u64 = 0x143a09780; const P45: u64 = 0x143a09798; const P46: u64 = 0x143a09788; const P47: u64 = 0x143a09790; const P48: u64 = 0x143a09700; const P49: u64 = 0x143a09718; const P50: u64 = 0x143a09708; const P51: u64 = 0x143a09710; const P52: u64 = 0x143a09720; const P53: u64 = 0x143a09738; const P54: u64 = 0x143a09728; const P55: u64 = 0x143a09730; const P56: u64 = 0x143a09740; const P57: u64 = 0x143a09758; const P58: u64 = 0x143a09748; const P59: u64 = 0x143a09750; const P60: u64 = 0x143a097a0; const P61: u64 = 0x143a097a8; const P62: u64 = 0x143a097b0; const P63: u64 = 0x143a097b8; const P64: u64 = 0x143a097c0; const P65: u64 = 0x143a097c8; const P66: u64 = 0x143a097d0; const P67: u64 = 0x143a097d8; const P68: u64 = 0x143a097e0; const P69: u64 = 0x143a097e8; const P70: u64 = 0x143a097f0; const P71: u64 = 0x143a097f8; const P72: u64 = 0x143a09800; const P73: u64 = 0x143a09808; const P74: u64 = 0x143a09810; const P75: u64 = 0x143a09818; const P76: u64 = 0x143a09820; const P77: u64 = 0x143a09828; const P78: u64 = 0x143a09830; const P79: u64 = 0x143a09838;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = P0; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 80; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rdx)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::lk_add_u64(P0, (rax) as u64); rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x18u64))) as u64; crate::simd_ops::lk_add_u64(P1, (rax) as u64); rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; crate::simd_ops::lk_add_u64(P2, (rax) as u64); rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x10u64))) as u64; fl_l = (crate::simd_ops::lk_add_u64(P3, (rax) as u64)) as u64; fl_r = fl_l; bb = 4; continue; }
            4 => { rcx = rdx.wrapping_add(0x20); rax = P4; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 8; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rcx)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 8; continue; } bb = 7; continue; }
            7 => { crate::simd_ops::lk_add_u64(P4, (rax) as u64); rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x18u64))) as u64; crate::simd_ops::lk_add_u64(P5, (rax) as u64); rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; crate::simd_ops::lk_add_u64(P6, (rax) as u64); rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; fl_l = (crate::simd_ops::lk_add_u64(P7, (rax) as u64)) as u64; fl_r = fl_l; bb = 8; continue; }
            8 => { rcx = rdx.wrapping_add(0x40); rax = P8; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 12; continue; } bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru64(rcx)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 12; continue; } bb = 11; continue; }
            11 => { crate::simd_ops::lk_add_u64(P8, (rax) as u64); rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x18u64))) as u64; crate::simd_ops::lk_add_u64(P9, (rax) as u64); rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; crate::simd_ops::lk_add_u64(P10, (rax) as u64); rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; fl_l = (crate::simd_ops::lk_add_u64(P11, (rax) as u64)) as u64; fl_r = fl_l; bb = 12; continue; }
            12 => { rcx = rdx.wrapping_add(0x60); rax = P12; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 16; continue; } bb = 13; continue; }
            13 => { rax = (crate::simd_ops::ru64(rcx)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 15; continue; } bb = 14; continue; }
            14 => { fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 16; continue; } bb = 15; continue; }
            15 => { crate::simd_ops::lk_add_u64(P12, (rax) as u64); rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x18u64))) as u64; crate::simd_ops::lk_add_u64(P13, (rax) as u64); rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; crate::simd_ops::lk_add_u64(P14, (rax) as u64); rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; fl_l = (crate::simd_ops::lk_add_u64(P15, (rax) as u64)) as u64; fl_r = fl_l; bb = 16; continue; }
            16 => { rcx = rdx.wrapping_add(0x80); rax = P16; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 20; continue; } bb = 17; continue; }
            17 => { rax = (crate::simd_ops::ru64(rcx)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 19; continue; } bb = 18; continue; }
            18 => { fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 20; continue; } bb = 19; continue; }
            19 => { crate::simd_ops::lk_add_u64(P16, (rax) as u64); rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x18u64))) as u64; crate::simd_ops::lk_add_u64(P17, (rax) as u64); rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; crate::simd_ops::lk_add_u64(P18, (rax) as u64); rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; fl_l = (crate::simd_ops::lk_add_u64(P19, (rax) as u64)) as u64; fl_r = fl_l; bb = 20; continue; }
            20 => { rcx = rdx.wrapping_add(0xa0); rax = P20; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 24; continue; } bb = 21; continue; }
            21 => { rax = (crate::simd_ops::ru64(rcx)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 23; continue; } bb = 22; continue; }
            22 => { fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 24; continue; } bb = 23; continue; }
            23 => { crate::simd_ops::lk_add_u64(P20, (rax) as u64); rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x18u64))) as u64; crate::simd_ops::lk_add_u64(P21, (rax) as u64); rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; crate::simd_ops::lk_add_u64(P22, (rax) as u64); rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; fl_l = (crate::simd_ops::lk_add_u64(P23, (rax) as u64)) as u64; fl_r = fl_l; bb = 24; continue; }
            24 => { rcx = rdx.wrapping_add(0xc0); rax = P24; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 28; continue; } bb = 25; continue; }
            25 => { rax = (crate::simd_ops::ru64(rcx)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 27; continue; } bb = 26; continue; }
            26 => { fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 28; continue; } bb = 27; continue; }
            27 => { crate::simd_ops::lk_add_u64(P24, (rax) as u64); rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x18u64))) as u64; crate::simd_ops::lk_add_u64(P25, (rax) as u64); rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; crate::simd_ops::lk_add_u64(P26, (rax) as u64); rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; fl_l = (crate::simd_ops::lk_add_u64(P27, (rax) as u64)) as u64; fl_r = fl_l; bb = 28; continue; }
            28 => { rcx = rdx.wrapping_add(0x100); rax = P28; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 32; continue; } bb = 29; continue; }
            29 => { rax = (crate::simd_ops::ru64(rcx)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 31; continue; } bb = 30; continue; }
            30 => { fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 32; continue; } bb = 31; continue; }
            31 => { crate::simd_ops::lk_add_u64(P28, (rax) as u64); rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x18u64))) as u64; crate::simd_ops::lk_add_u64(P29, (rax) as u64); rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; crate::simd_ops::lk_add_u64(P30, (rax) as u64); rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; fl_l = (crate::simd_ops::lk_add_u64(P31, (rax) as u64)) as u64; fl_r = fl_l; bb = 32; continue; }
            32 => { rcx = rdx.wrapping_add(0xe0); rax = P32; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 36; continue; } bb = 33; continue; }
            33 => { rax = (crate::simd_ops::ru64(rcx)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 35; continue; } bb = 34; continue; }
            34 => { fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 36; continue; } bb = 35; continue; }
            35 => { crate::simd_ops::lk_add_u64(P32, (rax) as u64); rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x18u64))) as u64; crate::simd_ops::lk_add_u64(P33, (rax) as u64); rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; crate::simd_ops::lk_add_u64(P34, (rax) as u64); rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; fl_l = (crate::simd_ops::lk_add_u64(P35, (rax) as u64)) as u64; fl_r = fl_l; bb = 36; continue; }
            36 => { rcx = rdx.wrapping_add(0x120); rax = P36; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 40; continue; } bb = 37; continue; }
            37 => { rax = (crate::simd_ops::ru64(rcx)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 39; continue; } bb = 38; continue; }
            38 => { fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 40; continue; } bb = 39; continue; }
            39 => { crate::simd_ops::lk_add_u64(P36, (rax) as u64); rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x18u64))) as u64; crate::simd_ops::lk_add_u64(P37, (rax) as u64); rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; crate::simd_ops::lk_add_u64(P38, (rax) as u64); rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; fl_l = (crate::simd_ops::lk_add_u64(P39, (rax) as u64)) as u64; fl_r = fl_l; bb = 40; continue; }
            40 => { rcx = rdx.wrapping_add(0x1a0); rax = P40; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 44; continue; } bb = 41; continue; }
            41 => { rax = (crate::simd_ops::ru64(rcx)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 43; continue; } bb = 42; continue; }
            42 => { fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 44; continue; } bb = 43; continue; }
            43 => { crate::simd_ops::lk_add_u64(P40, (rax) as u64); rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x18u64))) as u64; crate::simd_ops::lk_add_u64(P41, (rax) as u64); rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; crate::simd_ops::lk_add_u64(P42, (rax) as u64); rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; fl_l = (crate::simd_ops::lk_add_u64(P43, (rax) as u64)) as u64; fl_r = fl_l; bb = 44; continue; }
            44 => { rcx = rdx.wrapping_add(0x1c0); rax = P44; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 48; continue; } bb = 45; continue; }
            45 => { rax = (crate::simd_ops::ru64(rcx)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 47; continue; } bb = 46; continue; }
            46 => { fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 48; continue; } bb = 47; continue; }
            47 => { crate::simd_ops::lk_add_u64(P44, (rax) as u64); rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x18u64))) as u64; crate::simd_ops::lk_add_u64(P45, (rax) as u64); rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; crate::simd_ops::lk_add_u64(P46, (rax) as u64); rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; fl_l = (crate::simd_ops::lk_add_u64(P47, (rax) as u64)) as u64; fl_r = fl_l; bb = 48; continue; }
            48 => { rcx = rdx.wrapping_add(0x140); rax = P48; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 52; continue; } bb = 49; continue; }
            49 => { rax = (crate::simd_ops::ru64(rcx)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 51; continue; } bb = 50; continue; }
            50 => { fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 52; continue; } bb = 51; continue; }
            51 => { crate::simd_ops::lk_add_u64(P48, (rax) as u64); rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x18u64))) as u64; crate::simd_ops::lk_add_u64(P49, (rax) as u64); rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; crate::simd_ops::lk_add_u64(P50, (rax) as u64); rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; fl_l = (crate::simd_ops::lk_add_u64(P51, (rax) as u64)) as u64; fl_r = fl_l; bb = 52; continue; }
            52 => { rcx = rdx.wrapping_add(0x160); rax = P52; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 56; continue; } bb = 53; continue; }
            53 => { rax = (crate::simd_ops::ru64(rcx)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 55; continue; } bb = 54; continue; }
            54 => { fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 56; continue; } bb = 55; continue; }
            55 => { crate::simd_ops::lk_add_u64(P52, (rax) as u64); rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x18u64))) as u64; crate::simd_ops::lk_add_u64(P53, (rax) as u64); rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; crate::simd_ops::lk_add_u64(P54, (rax) as u64); rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; fl_l = (crate::simd_ops::lk_add_u64(P55, (rax) as u64)) as u64; fl_r = fl_l; bb = 56; continue; }
            56 => { rcx = rdx.wrapping_add(0x180); rax = P56; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 60; continue; } bb = 57; continue; }
            57 => { rax = (crate::simd_ops::ru64(rcx)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 59; continue; } bb = 58; continue; }
            58 => { fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 60; continue; } bb = 59; continue; }
            59 => { crate::simd_ops::lk_add_u64(P56, (rax) as u64); rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x18u64))) as u64; crate::simd_ops::lk_add_u64(P57, (rax) as u64); rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; crate::simd_ops::lk_add_u64(P58, (rax) as u64); rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; fl_l = (crate::simd_ops::lk_add_u64(P59, (rax) as u64)) as u64; fl_r = fl_l; bb = 60; continue; }
            60 => { rcx = rdx.wrapping_add(0x1e0); rax = P60; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 62; continue; } bb = 61; continue; }
            61 => { rax = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::lk_add_u64(P60, (rax) as u64); rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; fl_l = (crate::simd_ops::lk_add_u64(P61, (rax) as u64)) as u64; fl_r = fl_l; bb = 62; continue; }
            62 => { rcx = rdx.wrapping_add(0x1f0); rax = P62; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 64; continue; } bb = 63; continue; }
            63 => { rax = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::lk_add_u64(P62, (rax) as u64); rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; fl_l = (crate::simd_ops::lk_add_u64(P63, (rax) as u64)) as u64; fl_r = fl_l; bb = 64; continue; }
            64 => { rcx = rdx.wrapping_add(0x200); rax = P64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 66; continue; } bb = 65; continue; }
            65 => { rax = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::lk_add_u64(P64, (rax) as u64); rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; fl_l = (crate::simd_ops::lk_add_u64(P65, (rax) as u64)) as u64; fl_r = fl_l; bb = 66; continue; }
            66 => { rcx = rdx.wrapping_add(0x210); rax = P66; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 68; continue; } bb = 67; continue; }
            67 => { rax = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::lk_add_u64(P66, (rax) as u64); rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; fl_l = (crate::simd_ops::lk_add_u64(P67, (rax) as u64)) as u64; fl_r = fl_l; bb = 68; continue; }
            68 => { rcx = rdx.wrapping_add(0x220); rax = P68; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 70; continue; } bb = 69; continue; }
            69 => { rax = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::lk_add_u64(P68, (rax) as u64); rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; fl_l = (crate::simd_ops::lk_add_u64(P69, (rax) as u64)) as u64; fl_r = fl_l; bb = 70; continue; }
            70 => { rcx = rdx.wrapping_add(0x230); rax = P70; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 72; continue; } bb = 71; continue; }
            71 => { rax = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::lk_add_u64(P70, (rax) as u64); rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; fl_l = (crate::simd_ops::lk_add_u64(P71, (rax) as u64)) as u64; fl_r = fl_l; bb = 72; continue; }
            72 => { rcx = rdx.wrapping_add(0x240); rax = P72; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 74; continue; } bb = 73; continue; }
            73 => { rax = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::lk_add_u64(P72, (rax) as u64); rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; fl_l = (crate::simd_ops::lk_add_u64(P73, (rax) as u64)) as u64; fl_r = fl_l; bb = 74; continue; }
            74 => { rcx = rdx.wrapping_add(0x250); rax = P74; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 76; continue; } bb = 75; continue; }
            75 => { rax = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::lk_add_u64(P74, (rax) as u64); rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; fl_l = (crate::simd_ops::lk_add_u64(P75, (rax) as u64)) as u64; fl_r = fl_l; bb = 76; continue; }
            76 => { rcx = rdx.wrapping_add(0x260); rax = P76; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 78; continue; } bb = 77; continue; }
            77 => { rax = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::lk_add_u64(P76, (rax) as u64); rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; fl_l = (crate::simd_ops::lk_add_u64(P77, (rax) as u64)) as u64; fl_r = fl_l; bb = 78; continue; }
            78 => { rdx = ((rdx).wrapping_add(0x270u64)) as u64; rax = P78; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 80; continue; } bb = 79; continue; }
            79 => { rax = (crate::simd_ops::ru64(rdx)) as u64; crate::simd_ops::lk_add_u64(P78, (rax) as u64); rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; fl_l = (crate::simd_ops::lk_add_u64(P79, (rax) as u64)) as u64; fl_r = fl_l; bb = 80; continue; }
            80 => { return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142540890(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = rcx; fl_r = rdx;
    if !((((fl_l as u64)) == ((fl_r as u64)))) { rax = (crate::simd_ops::ru64(rdx)) as u64; crate::simd_ops::lk_add_u64(rcx, (rax) as u64); rax = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_l = (crate::simd_ops::lk_add_u64(rcx.wrapping_add(P0), (rax) as u64)) as u64; fl_r = fl_l; }
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425408b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x18; const P2: u64 = 0x10;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = rcx; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 4; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rdx)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::lk_add_u64(rcx, (rax) as u64); rax = (crate::simd_ops::ru64(rdx.wrapping_add(P1))) as u64; crate::simd_ops::lk_add_u64(rcx.wrapping_add(P1), (rax) as u64); rax = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; crate::simd_ops::lk_add_u64(rcx.wrapping_add(P0), (rax) as u64); rax = (crate::simd_ops::ru64(rdx.wrapping_add(P2))) as u64; fl_l = (crate::simd_ops::lk_add_u64(rcx.wrapping_add(P2), (rax) as u64)) as u64; fl_r = fl_l; bb = 4; continue; }
            4 => { return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142540980(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x143a095c0; const P1: u64 = 0x143a09840;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = P0;
    fl_l = rcx; fl_r = rax;
    if (((fl_l as u64)) < ((fl_r as u64))) { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; return rax; }
    rax = P1;
    fl_l = rcx; fl_r = rax;
    if !((((fl_l as u64)) >= ((fl_r as u64)))) { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); return rax; }
    rax = (rax & !0xffu64) | (((0u8) as u8 as u64));
    fl_l = 0; fl_r = 0;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425409e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rcx)) as u64; fl_l = rax; fl_r = rdx; if ((((fl_l as u64)) as i64) >= (((fl_r as u64)) as i64)) { bb = 3; continue; } bb = 2; continue; }
            2 => { fl_l = rax; let cx_src: u64 = rdx; let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u64(rcx, fl_l as u64, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u64)) as u64; } if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 3; continue; }
            3 => { return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142540a50(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143bbdfa8; const P1: u64 = 0x1425412f0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    xmm1 = 0u128;
    xmm1 = (xmm1 & !0xffff_ffff_ffff_ffffu128) | ((((crate::simd_ops::ru64(P0)) as i64) as f64).to_bits() as u128);
    xmm0 = 0u128;
    fl_l = ((xmm1 as u64)) as u64; fl_r = ((xmm0 as u64)) as u64;
    if (f64::from_bits(fl_l).is_nan() || f64::from_bits(fl_r).is_nan()) { rsp = ((rsp).wrapping_add(0x28u64)) as u64; return crate::simd_ops::tail_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); }
    if !((f64::from_bits(fl_l) != f64::from_bits(fl_r) && !f64::from_bits(fl_l).is_nan() && !f64::from_bits(fl_r).is_nan())) { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbx); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rax; fl_r = rbx; rax = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(P0, rax); }
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return crate::simd_ops::tail_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142540aa0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1425412f0; const P1: u64 = 0x143bbdfa8;
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rax = ((rax).wrapping_sub(crate::simd_ops::ru64(P1))) as u64;
    rax = ((rax).wrapping_sub(rbx)) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142540ad0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x143a095c0; const P1: u64 = 0x143a09840; const P2: u64 = 0x8;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = P0;
    fl_l = rcx; fl_r = rax;
    if (((fl_l as u64)) < ((fl_r as u64))) { crate::simd_ops::lk_add_u64(rcx.wrapping_add(P2), (1) as u64); fl_l = (crate::simd_ops::lk_add_u64(rcx, (1) as u64)) as u64; return rax; }
    rax = P1;
    fl_l = rcx; fl_r = rax;
    if !((((fl_l as u64)) >= ((fl_r as u64)))) { crate::simd_ops::lk_add_u64(rcx.wrapping_add(P2), (1) as u64); fl_l = (crate::simd_ops::lk_add_u64(rcx, (1) as u64)) as u64; return rax; }
    crate::simd_ops::lk_add_u64(rcx.wrapping_add(P2), (1) as u64);
    fl_l = (crate::simd_ops::lk_add_u64(rcx, (1) as u64)) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142540b50(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1425408f0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = 0; fl_r = rdx;
    rdx = (0u64.wrapping_sub(fl_r as u64)) as u64;
    return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142540bb0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14253eb30; const P1: u64 = 0x143a095c0; const P2: u64 = 0x14183acc2; const P3: u64 = 0x143bbd998; const P4: u64 = 0x143bbdfa8; const P5: u64 = 0x1425412f0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rbx = P1;
    fl_l = rax; fl_r = rbx;
    if !((((fl_l as u64)) == ((fl_r as u64)))) { rdx = 0; fl_l = 0; fl_r = 0; r8 = 0x280; rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); }
    rdx = 0;
    fl_l = 0; fl_r = 0;
    r8 = 0x280;
    rcx = rbx;
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    fl_l = (crate::simd_ops::ru64(P3)) as u64; fl_r = (0x0u64) as u64;
    if (((fl_l as u64)) != ((fl_r as u64))) { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
    xmm1 = 0u128;
    xmm1 = (xmm1 & !0xffff_ffff_ffff_ffffu128) | ((((crate::simd_ops::ru64(P4)) as i64) as f64).to_bits() as u128);
    xmm0 = 0u128;
    fl_l = ((xmm1 as u64)) as u64; fl_r = ((xmm0 as u64)) as u64;
    if (f64::from_bits(fl_l).is_nan() || f64::from_bits(fl_r).is_nan()) { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(P3, rax); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
    if !((f64::from_bits(fl_l) != f64::from_bits(fl_r) && !f64::from_bits(fl_l).is_nan() && !f64::from_bits(fl_r).is_nan())) { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rax; fl_r = rbx; rax = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; crate::simd_ops::wu64(P4, rax); }
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    crate::simd_ops::wu64(P3, rax);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142540c70(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143c06158; const P1: u64 = 0x1434260c0; const P2: u64 = 0x142645308; const P3: u64 = 0x1434260d0; const P4: u64 = 0x142645758; const P5: u64 = 0x1e;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rax = (crate::simd_ops::ru64(P0)) as u64;
    rbx = rcx;
    fl_l = rax; fl_r = rax;
    if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { r9 = 0x2; rdx = rbx; rcx = 0; fl_l = 0; fl_r = 0; r8 = ((r9.wrapping_add(P5)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(rax, rax, rcx, rdx, r8, r9, rsp); rax = (((rax as u32)).wrapping_shr(31u32)) as u32 as u64; rax = (rax & !0xffu64) | ((((((rax as u8)) ^ (0x1u8))) as u8 as u64)); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    rcx = P1;
    rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp);
    fl_l = rax; fl_r = rax;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rdx = P3; rcx = rax; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(P0, rax); } else { rax = (crate::simd_ops::ru64(P0)) as u64; }
    fl_l = rax; fl_r = rax;
    if !(((((fl_l as u64)) & ((fl_r as u64))) != 0)) { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    r9 = 0x2;
    rdx = rbx;
    rcx = 0;
    fl_l = 0; fl_r = 0;
    r8 = ((r9.wrapping_add(P5)) as u32) as u32 as u64;
    rax = crate::simd_ops::call_stk(rax, rax, rcx, rdx, r8, r9, rsp);
    rax = (((rax as u32)).wrapping_shr(31u32)) as u32 as u64;
    rax = (rax & !0xffu64) | ((((((rax as u8)) ^ (0x1u8))) as u8 as u64));
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142540cf0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x4;
    let (mut rax, mut fl_l) = (crate::simd_ops::get_rax_in(), 0u64);
    rax = (crate::simd_ops::ru32(rcx)) as u32 as u64;
    rdx = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64;
    rdx = ((rdx).wrapping_shl(32u32)) as u64;
    rdx = (((rdx) | (rax))) as u64;
    rax = (0x346dc5d63886594bu64) as u64;
    let prod = (rax as u128).wrapping_mul((rdx) as u128);
    rax = prod as u64; rdx = (prod >> 64) as u64; fl_l = (rdx != 0) as u64;
    fl_l = ((rdx).wrapping_shr(11u32)) as u64;
    rdx = ((fl_l as u64)) as u64;
    rax = rdx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142540d20(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143bd6038; const P1: u64 = 0x142645700;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    r8 = (crate::simd_ops::ru64(P0)) as u64;
    rbx = rcx;
    fl_l = r8; fl_r = r8;
    if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { rax = rbx; rdx = if (rax as i64) < 0 { 0xffffffffffffffffu64 } else { 0u64 }; let d = (r8) as i64 as i128; let n = (((rdx as u128) << 64) | (rax as u128)) as i128; if d != 0 { rax = n.wrapping_div(d) as i64 as u64; rdx = n.wrapping_rem(d) as i64 as u64; } rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    rcx = rsp.wrapping_add(0x30);
    rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp);
    rax = (0x20c49ba5e353f7cfu64) as u64;
    let prod = (rax as i64 as i128).wrapping_mul((crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as i64 as i128) as u128;
    rax = prod as u64; rdx = (prod >> 64) as u64;
    r8 = rdx;
    r8 = ((((r8) as i64).wrapping_shr(7u32) as u64)) as u64;
    rax = r8;
    rax = ((rax).wrapping_shr(63u32)) as u64;
    fl_l = ((r8).wrapping_add(rax)) as u64; fl_r = fl_l;
    r8 = ((fl_l as u64)) as u64;
    crate::simd_ops::wu64(P0, r8);
    if !((((fl_l as u64)) != 0)) { r8 = 0x1; crate::simd_ops::wu64(P0, r8); }
    rax = rbx;
    rdx = if (rax as i64) < 0 { 0xffffffffffffffffu64 } else { 0u64 };
    let d = (r8) as i64 as i128; let n = (((rdx as u128) << 64) | (rax as u128)) as i128;
    if d != 0 { rax = n.wrapping_div(d) as i64 as u64; rdx = n.wrapping_rem(d) as i64 as u64; }
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142540d90(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142541180; const P1: u64 = 0x143bfe128; const P2: u64 = 0x143844165; const P3: u64 = 0x1426456c8; const P4: u64 = 0x1434261f0; const P5: u64 = 0x142537a30; const P6: u64 = 0x143bc5fd8; const P7: u64 = 0x142645730;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 15>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x70u64)) as u64; rsi = rcx; rdi = ((r8 as u32)) as u32 as u64; rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rax = 0; fl_l = 0; fl_r = 0; fl_l = (crate::simd_ops::ru64(P1)) as u64; fl_r = rax; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), rax); if (((fl_l as u64)) == ((fl_r as u64))) { bb = 7; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru8(P2)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 7; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rsp.wrapping_add(0x80u64), rbx); crate::simd_ops::wu8(rsp.wrapping_add(0x40u64), (0x5u8) as u8); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), (0x10u64) as u64); fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu8(rsp.wrapping_add(0x50u64), (0x2u8) as u8); crate::simd_ops::wu32(rsp.wrapping_add(0x58u64), ((rdi as u32)) as u32); bb = 4; continue; }
            4 => { rbx = ((rdi as u32)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x98u64), (0x40000000u64) as u64); rbx = (((((rbx as u32)) as i32).wrapping_shr(31u32) as u32)) as u32 as u64; rbx = (((rbx as u32)).wrapping_add(0x2u32)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x88u64), rsi); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((rbx as u32)) as u32); r8 = rsp.wrapping_add(0x98); rcx = rax; rdx = rsp.wrapping_add(0x88); rax = rsp.wrapping_add(0x40); r9 = 0x20003000; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x4u32) as u32); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; rdx = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x88u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 11; continue; } bb = 6; continue; }
            6 => { rcx = P4; crate::simd_ops::wu8(P2, (0x0u8) as u8); rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); bb = 7; continue; }
            7 => { fl_l = (crate::simd_ops::ru64(P6)) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 10; continue; } bb = 8; continue; }
            8 => { fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { crate::simd_ops::wu8(rsp.wrapping_add(0x40u64), (0x2u8) as u8); crate::simd_ops::wu32(rsp.wrapping_add(0x48u64), ((rdi as u32)) as u32); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), (0x1u32) as u32); r9 = 0x20003000; rcx = rax; r8 = 0x40000000; rax = rsp.wrapping_add(0x40); rdx = rsi; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x4u32) as u32); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp); rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; rsp = ((rsp).wrapping_add(0x70u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            10 => { rdx = 0x40000000; r9 = 0x4; r8 = 0x20003000; rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P7), rax), rax, rcx, rdx, r8, r9, rsp); bb = 11; continue; }
            11 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; rsp = ((rsp).wrapping_add(0x70u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142540f20(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143872f38; const P1: u64 = 0x143872cb4; const P2: u64 = 0x143872cb0; const P3: u64 = 0x1425375a0; const P4: u64 = 0x143c0e188; const P5: u64 = 0x142541030;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rcx = (crate::simd_ops::ru64(P0)) as u64; rbx = ((r9 as u32)) as u32 as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rdi = r8; rsi = rdx; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 16; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru32(P1)) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rcx = P2; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(P0)) as u64; bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru32(P2)) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 16; continue; } bb = 4; continue; }
            4 => { rdx = 0; fl_l = 0; fl_r = 0; rax = rsi; let d = (rcx) as u128; let n = ((rdx as u128) << 64) | (rax as u128); if d != 0 { rax = (n / d) as u64; rdx = (n % d) as u64; } fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 7; continue; } bb = 5; continue; }
            5 => { rax = rdi; let d = (rcx) as u128; let n = ((rdx as u128) << 64) | (rax as u128); if d != 0 { rax = (n / d) as u64; rdx = (n % d) as u64; } fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 8; continue; }
            7 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 8; continue; }
            8 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 16; continue; } bb = 9; continue; }
            9 => { fl_l = (crate::simd_ops::ru8(rsp.wrapping_add(0x58u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 16; continue; } bb = 10; continue; }
            10 => { fl_l = ((((rbx as u32)) >> (12u32)) & 1) as u64; if ((fl_l as u32) == 0) { bb = 16; continue; } bb = 11; continue; }
            11 => { fl_l = ((((rbx as u32)) >> (13u32)) & 1) as u64; if ((fl_l as u32) == 0) { bb = 16; continue; } bb = 12; continue; }
            12 => { rax = (crate::simd_ops::ru64(P4)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 13; continue; }
            13 => { rcx = rax.wrapping_sub(0x1); fl_l = rax; let cx_src: u64 = rcx; let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u64(P4, fl_l as u64, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u64)) as u64; } bb = 16; continue; }
            14 => { r9 = ((rbx as u32)) as u32 as u64; crate::simd_ops::wu8(r14, (0x1u8) as u8); fl_l = ((((r9 as u32)) >> (29u32)) & 1) as u64; r9 = (((r9 as u32)) | ((1u32) << (29u32))) as u32 as u64; r8 = rdi; rdx = rsi; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 17; continue; } bb = 15; continue; }
            15 => { crate::simd_ops::wu64(P4, (0xau64) as u64); bb = 16; continue; }
            16 => { rax = ((rbx as u32)) as u32 as u64; r9 = ((rbx as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shr(29u32)) as u32 as u64; r8 = rdi; rax = (rax & !0xffu64) | ((((((rax as u8)) & (0x1u8))) as u8 as u64)); rdx = rsi; crate::simd_ops::wu8(r14, ((rax as u8)) as u8); rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); bb = 17; continue; }
            17 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142541030(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142539f60; const P1: u64 = 0x142645730; const P2: u64 = 0x142645708; const P3: u64 = 0x143426100; const P4: u64 = 0x142538300; const P5: u64 = 0x143872f30; const P6: u64 = 0x143bc5fd8; const P7: u64 = 0x1426456c8; const P8: u64 = 0x143426180; const P9: u64 = 0x142537a30;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 15>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x70u64)) as u64; rcx = r8; rsi = ((r9 as u32)) as u32 as u64; rbx = r8; rdi = rdx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbp = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { r9 = 0x4; r8 = ((rsi as u32)) as u32 as u64; rdx = rdi; rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 9; continue; } bb = 2; continue; }
            2 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rsi as u32)) as u32); rcx = P3; r8 = ((rax as u32)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbx); r9 = rbp; rdx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 3; continue; }
            3 => { fl_l = rbx; fl_r = (0x1u64) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 8; continue; } bb = 4; continue; }
            4 => { rdx = 0; fl_l = 0; fl_r = 0; rax = rbx; let d = (crate::simd_ops::ru64(P5)) as u128; let n = ((rdx as u128) << 64) | (rax as u128); if d != 0 { rax = (n / d) as u64; rdx = (n % d) as u64; } fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 8; continue; } bb = 5; continue; }
            5 => { fl_l = (crate::simd_ops::ru64(P6)) as u64; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 8; continue; } bb = 6; continue; }
            6 => { xmm0 = 0u128; rax = rsp.wrapping_add(0x50); crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rbx); crate::simd_ops::wu128(rsp.wrapping_add(0x50u64), (xmm0) as u128); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), (0x1u64) as u64); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P7), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), (0x1u32) as u32); r9 = ((rsi as u32)) as u32 as u64; rcx = rax; r8 = rdi; rax = rsp.wrapping_add(0x40); rdx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x4u32) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 9; continue; } bb = 7; continue; }
            7 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rsi as u32)) as u32); rcx = P8; r8 = ((rax as u32)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbx); r9 = 0; fl_l = 0; fl_r = 0; rdx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 8; continue; }
            8 => { r9 = 0x4; r8 = ((rsi as u32)) as u32 as u64; rdx = rdi; rcx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 9; continue; }
            9 => { r11 = rsp.wrapping_add(0x70); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x10u64))) as u64; rbp = (crate::simd_ops::ru64(r11.wrapping_add(0x18u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rsp = r11; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142541180(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143a28fa7; const P1: u64 = 0x143872f38; const P2: u64 = 0x143872f30; const P3: u64 = 0x1426456c8; const P4: u64 = 0x142645018; const P5: u64 = 0x143425f80; const P6: u64 = 0x142645020; const P7: u64 = 0x142645028; const P8: u64 = 0x142645708; const P9: u64 = 0x142645328; const P10: u64 = 0x1426455b0; const P11: u64 = 0x143425f98; const P12: u64 = 0x142537a30;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; fl_l = (crate::simd_ops::ru8(P0)) as u8 as u64; fl_r = (0x0u8) as u8 as u64; rsi = rcx; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(P1)) as u64; rcx = (crate::simd_ops::ru64(P2)) as u64; fl_l = rax; fl_r = rax; rcx = (if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { rax } else { rcx }) as u64; fl_l = rcx; fl_r = rcx; rax = (rax & !0xffu64) | (((if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { 1u8 } else { 0u8 }) as u8 as u64)); rsp = ((rsp).wrapping_add(0x40u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            2 => { crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rdi); rdi = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rdi); crate::simd_ops::wu8(P0, (0x1u8) as u8); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rax; r8 = rsp.wrapping_add(0x58); rdx = ((rdi.wrapping_add(0x28u64)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); rbx = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 9; continue; } bb = 3; continue; }
            3 => { r8 = rsp.wrapping_add(0x34); rcx = 0; fl_l = 0; fl_r = 0; rdx = P5; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp); rbx = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 8; continue; } bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; r8 = rsp.wrapping_add(0x30); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rdi); r9 = 0; fl_l = 0; fl_r = 0; rdx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), (0x1u32) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x3cu64), (0x2u32) as u32); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P7), rax), rax, rcx, rdx, r8, r9, rsp); rbx = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 8; continue; } bb = 5; continue; }
            5 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P8), rax), rax, rcx, rdx, r8, r9, rsp); rbx = 0; fl_l = 0; fl_r = 0; rdi = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; rbx = (rbx & !0xffu64) | (((if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { 1u8 } else { 0u8 }) as u8 as u64)); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 8; continue; } bb = 6; continue; }
            6 => { fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P9), rax), rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rsi, rax); bb = 8; continue; }
            8 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P10), rax), rax, rcx, rdx, r8, r9, rsp); bb = 9; continue; }
            9 => { fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 13; continue; } bb = 10; continue; }
            10 => { fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 12; continue; } bb = 11; continue; }
            11 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P8), rax), rax, rcx, rdx, r8, r9, rsp); rdi = ((rax as u32)) as u32 as u64; bb = 12; continue; }
            12 => { rdx = ((rdi as u32)) as u32 as u64; rcx = P11; rax = crate::simd_ops::call_stk(P12, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; bb = 13; continue; }
            13 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rax = (rax & !0xffu64) | (((if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { 1u8 } else { 0u8 }) as u8 as u64)); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425412a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x143c0e188;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    crate::simd_ops::wu64(P0, (0xau64) as u64);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425412d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x143c0e188;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru64(rdx)) as u64;
    fl_l = rax; let cx_src: u64 = r8;
    let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u64(P0, fl_l as u64, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u64)) as u64; }
    if !((((fl_l as u64)) != ((fl_r as u64)))) { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); return rax; }
    crate::simd_ops::wu64(rdx, rax);
    rax = (rax & !0xffu64) | (((0u8) as u8 as u64));
    fl_l = 0; fl_r = 0;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142541370(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1426456c8; const P1: u64 = 0x1426452e0; const P2: u64 = 0x143bf60f8; const P3: u64 = 0x143426098; const P4: u64 = 0x142645308; const P5: u64 = 0x1434260a8; const P6: u64 = 0x142645758;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 17>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x80u64)) as u64; rbx = rcx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rax; r9 = rsp.wrapping_add(0x98); rax = rsp.wrapping_add(0x90); r8 = rsp.wrapping_add(0xa0); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); rdx = rsp.wrapping_add(0xa8); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x90u64))) as u32 as u64; r8 = (0x346dc5d63886594bu64) as u64; rdx = (crate::simd_ops::ru32(rsp.wrapping_add(0x94u64))) as u32 as u64; rcx = (crate::simd_ops::ru32(rsp.wrapping_add(0x98u64))) as u32 as u64; rdx = ((rdx).wrapping_shl(32u32)) as u64; rdx = (((rdx) | (rax))) as u64; rax = r8; let prod = (rax as u128).wrapping_mul((rdx) as u128); rax = prod as u64; rdx = (prod >> 64) as u64; fl_l = (rdx != 0) as u64; rax = r8; rdx = ((rdx).wrapping_shr(11u32)) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), rdx); rdx = (crate::simd_ops::ru32(rsp.wrapping_add(0x9cu64))) as u32 as u64; rdx = ((rdx).wrapping_shl(32u32)) as u64; rdx = (((rdx) | (rcx))) as u64; let prod = (rax as u128).wrapping_mul((rdx) as u128); rax = prod as u64; rdx = (prod >> 64) as u64; fl_l = (rdx != 0) as u64; rax = (crate::simd_ops::ru64(P2)) as u64; rdx = ((rdx).wrapping_shr(11u32)) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), rdx); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { rcx = P3; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rdx = P5; rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(P2, rax); bb = 4; continue; }
            3 => { rax = (crate::simd_ops::ru64(P2)) as u64; bb = 4; continue; }
            4 => { rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), rcx); xmm0 = 0u128; ymm0_hi = 0; let y_addr: u64 = rsp.wrapping_add(0x30u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); let y_addr: u64 = rsp.wrapping_add(0x50u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = 0x48; rdx = rsp.wrapping_add(0x30); rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x18u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x20u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x28u64), rax); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x34u64))) as u32 as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x38u64), rax); crate::simd_ops::wu64(rbx.wrapping_add(0x30u64), rcx); rsp = ((rsp).wrapping_add(0x80u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425414c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142645738; const P1: u64 = 0x142645708; const P2: u64 = 0x1426455e0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x50u64)) as u64; rdx = 0; fl_l = 0; fl_r = 0; r8 = 0x8000; rdi = rcx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 9; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rbx); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1e7; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 6; continue; } bb = 2; continue; }
            2 => { xmm0 = 0u128; ymm0_hi = 0; let y_addr: u64 = rsp.wrapping_add(0x20u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm0) as u128); r8 = 0x30; rdx = rsp.wrapping_add(0x20); rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; fl_l = rcx; fl_r = rdi; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 8; continue; } bb = 3; continue; }
            3 => { rdi = ((rdi).wrapping_sub(rcx)) as u64; fl_l = rdi; fl_r = (0x2000000u64) as u64; if ((((fl_l as u64)) as i64) >= (((fl_r as u64)) as i64)) { bb = 8; continue; } bb = 4; continue; }
            4 => { rdx = 0; fl_l = 0; fl_r = 0; r8 = 0x8000; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 7; continue; } bb = 5; continue; }
            5 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 6; continue; }
            6 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsp = ((rsp).wrapping_add(0x50u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            7 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rax = 0; fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x50u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            8 => { rax = ((rbx as u32)) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsp = ((rsp).wrapping_add(0x50u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            9 => { rax = 0; fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x50u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142541580(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142645340; const P1: u64 = 0x143bee0c8; const P2: u64 = 0x142645310;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x38u64)) as u64; rcx = rsp.wrapping_add(0x40); crate::simd_ops::wu32(rsp.wrapping_add(0x40u64), (0x0u32) as u32); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x40u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 11; continue; } bb = 1; continue; }
            1 => { bb = 2; continue; }
            2 => { r8 = (crate::simd_ops::ru64(P1)) as u64; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 3; continue; }
            3 => { rdx = rsp.wrapping_add(0x20); rcx = (((rax as u16)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(r8, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 8; continue; } bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; fl_r = (0x0u64) as u64; bb = 7; continue; }
            5 => { rdx = rsp.wrapping_add(0x48); rcx = (((rax as u8)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 8; continue; } bb = 6; continue; }
            6 => { fl_l = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; fl_r = (0x0u64) as u64; bb = 7; continue; }
            7 => { if (((fl_l as u64)) != ((fl_r as u64))) { bb = 10; continue; } bb = 8; continue; }
            8 => { rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x40u64))) as u32 as u64; fl_l = (((rax as u32)).wrapping_add(0xffffffffu32)) as u32 as u64; fl_r = fl_l; rax = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x40u64), ((rax as u32)) as u32); if (((fl_l as u32)) != 0) { bb = 2; continue; } bb = 9; continue; }
            9 => { rax = ((rax).wrapping_add(1)) as u64; rsp = ((rsp).wrapping_add(0x38u64)) as u64; return rax; }
            10 => { rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x40u64))) as u32 as u64; bb = 11; continue; }
            11 => { rax = ((rax).wrapping_add(1)) as u64; rsp = ((rsp).wrapping_add(0x38u64)) as u64; return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142541610(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142541180; const P1: u64 = 0x143bfe128; const P2: u64 = 0x143844165; const P3: u64 = 0x1426456c8; const P4: u64 = 0x1434261f0; const P5: u64 = 0x142537a30; const P6: u64 = 0x143bc5fd8; const P7: u64 = 0x142645730; const P8: u64 = 0x142645708;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 17>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = ((rsp).wrapping_sub(0x80u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), rsi); rsi = rcx; rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), rdi); rdi = ((r8 as u32)) as u32 as u64; crate::simd_ops::wu8(r9, (0x1u8) as u8); rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rbp = 0; fl_l = 0; fl_r = 0; fl_l = (crate::simd_ops::ru64(P1)) as u64; fl_r = rbp; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), rbp); if (((fl_l as u64)) == ((fl_r as u64))) { bb = 7; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru8(P2)) as u8 as u64; fl_r = ((rbp as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 7; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rsp.wrapping_add(0x90u64), rbx); crate::simd_ops::wu8(rsp.wrapping_add(0x40u64), (0x5u8) as u8); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), (0x10u64) as u64); fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu8(rsp.wrapping_add(0x50u64), (0x2u8) as u8); crate::simd_ops::wu32(rsp.wrapping_add(0x58u64), ((rdi as u32)) as u32); bb = 4; continue; }
            4 => { rbx = ((rdi as u32)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xa8u64), (0x40000000u64) as u64); rbx = (((((rbx as u32)) as i32).wrapping_shr(31u32) as u32)) as u32 as u64; rbx = (((rbx as u32)).wrapping_add(0x2u32)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x98u64), rsi); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((rbx as u32)) as u32); r8 = rsp.wrapping_add(0xa8); rcx = rax; rdx = rsp.wrapping_add(0x98); rax = rsp.wrapping_add(0x40); r9 = 0x20003000; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x4u32) as u32); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; rdx = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x98u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 11; continue; } bb = 6; continue; }
            6 => { rcx = P4; crate::simd_ops::wu8(P2, ((rbp as u8)) as u8); rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); bb = 7; continue; }
            7 => { fl_l = (crate::simd_ops::ru64(P6)) as u64; fl_r = rbp; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 10; continue; } bb = 8; continue; }
            8 => { fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { crate::simd_ops::wu8(rsp.wrapping_add(0x40u64), (0x2u8) as u8); crate::simd_ops::wu32(rsp.wrapping_add(0x48u64), ((rdi as u32)) as u32); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), (0x1u32) as u32); r9 = 0x20003000; rcx = rax; r8 = 0x40000000; rax = rsp.wrapping_add(0x40); rdx = rsi; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x4u32) as u32); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp); bb = 11; continue; }
            10 => { rdx = 0x40000000; r9 = 0x4; r8 = 0x20003000; rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P7), rax), rax, rcx, rdx, r8, r9, rsp); bb = 11; continue; }
            11 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0xb0u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; crate::simd_ops::wu64(rcx, rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 12; continue; }
            12 => { rax = ((rbp as u32)) as u32 as u64; rsp = ((rsp).wrapping_add(0x80u64)) as u64; rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            13 => { rsp = ((rsp).wrapping_add(0x80u64)) as u64; rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P8), rax), rax, rcx, rdx, r8, r9, rsp); }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425417c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142645338; const P1: u64 = 0x142645708;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rax = (((r8 as u8)) as u32) as u32 as u64;
    crate::simd_ops::wu32(rsp.wrapping_add(0x40u64), (0x0u32) as u32);
    fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64;
    r8 = 0x4;
    r9 = 0x1;
    r8 = (if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { (r9 as u32) } else { (r8 as u32) }) as u32 as u64;
    r9 = rsp.wrapping_add(0x40);
    rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp);
    fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64;
    if !(((((fl_l as u32)) & ((fl_r as u32))) == 0)) { rax = 0; fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
    rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142541810(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142645730; const P1: u64 = 0x142645708;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    crate::simd_ops::wu8(r8, (0x0u8) as u8);
    r9 = 0x4;
    r8 = 0x1000;
    rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp);
    fl_l = rax; fl_r = rax;
    if !(((((fl_l as u64)) & ((fl_r as u64))) != 0)) { rsp = ((rsp).wrapping_add(0x28u64)) as u64; return crate::simd_ops::tail_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); }
    rax = 0;
    fl_l = 0; fl_r = 0;
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142541850(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142645738; const P1: u64 = 0x142645708;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = r8;
    r8 = 0x4000;
    rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp);
    crate::simd_ops::wu8(rbx, (0x1u8) as u8);
    fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64;
    if !(((((fl_l as u32)) & ((fl_r as u32))) == 0)) { rax = 0; fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142541890(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143bce008; const P1: u64 = 0x143be6098; const P2: u64 = 0x143bde068; const P3: u64 = 0x142645300;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru64(P0)) as u64; rbx = 0; fl_l = 0; fl_r = 0; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru64(P1)) as u64; fl_r = rbx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 4; continue; } bb = 2; continue; }
            2 => { rcx = rsp.wrapping_add(0x38); rax = crate::simd_ops::call_stk(rax, rax, rcx, rdx, r8, r9, rsp); rdx = rsp.wrapping_add(0x30); crate::simd_ops::wu16(rsp.wrapping_add(0x30u64), ((rbx as u16)) as u16); rcx = rsp.wrapping_add(0x38); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 7; continue; } bb = 3; continue; }
            3 => { rbx = ((crate::simd_ops::ru16(rsp.wrapping_add(0x30u64))) as u32) as u32 as u64; rax = (((rbx as u16)) as u32) as u32 as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            4 => { fl_l = (crate::simd_ops::ru64(P2)) as u64; fl_r = rbx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 7; continue; } bb = 5; continue; }
            5 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); rdx = rsp.wrapping_add(0x30); crate::simd_ops::wu8(rsp.wrapping_add(0x30u64), ((rbx as u8)) as u8); rcx = (((rax as u8)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { rbx = ((crate::simd_ops::ru8(rsp.wrapping_add(0x30u64))) as u32) as u32 as u64; bb = 7; continue; }
            7 => { rax = (((rbx as u16)) as u32) as u32 as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142541ac0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142645730; const P1: u64 = 0x142645708;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    r9 = 0x4;
    r8 = 0x80000;
    rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp);
    fl_l = rax; fl_r = rax;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rax = 0; fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return crate::simd_ops::tail_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142541b00(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143872f38; const P1: u64 = 0x143872cb4; const P2: u64 = 0x143872cb0; const P3: u64 = 0x1425375a0; const P4: u64 = 0x143c0e188; const P5: u64 = 0x142541030; const P6: u64 = 0x142645708;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; fl_l = ((r8 as u8)) as u8 as u64; fl_r = ((r8 as u8)) as u8 as u64; r8 = (crate::simd_ops::ru64(P0)) as u64; rbx = 0x3000; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rbp = (((r9 as u8)) as u32) as u32 as u64; rdi = rdx; rsi = rcx; crate::simd_ops::wu8(rax, (0x1u8) as u8); rax = 0x2000; rbx = (if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { (rax as u32) } else { (rbx as u32) }) as u32 as u64; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 15; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru32(P1)) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rcx = P2; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); r8 = (crate::simd_ops::ru64(P0)) as u64; bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru32(P2)) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 15; continue; } bb = 4; continue; }
            4 => { rdx = 0; fl_l = 0; fl_r = 0; rax = rsi; let d = (r8) as u128; let n = ((rdx as u128) << 64) | (rax as u128); if d != 0 { rax = (n / d) as u64; rdx = (n % d) as u64; } fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 7; continue; } bb = 5; continue; }
            5 => { rax = rdi; let d = (r8) as u128; let n = ((rdx as u128) << 64) | (rax as u128); if d != 0 { rax = (n / d) as u64; rdx = (n % d) as u64; } fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 8; continue; }
            7 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 8; continue; }
            8 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 15; continue; } bb = 9; continue; }
            9 => { fl_l = ((rbp as u8)) as u8 as u64; fl_r = ((rbp as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 15; continue; } bb = 10; continue; }
            10 => { fl_l = ((((rbx as u32)) >> (12u32)) & 1) as u64; if ((fl_l as u32) == 0) { bb = 15; continue; } bb = 11; continue; }
            11 => { rax = (crate::simd_ops::ru64(P4)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 12; continue; }
            12 => { rcx = rax.wrapping_sub(0x1); fl_l = rax; let cx_src: u64 = rcx; let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u64(P4, fl_l as u64, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u64)) as u64; } bb = 15; continue; }
            13 => { r9 = ((rbx as u32)) as u32 as u64; crate::simd_ops::wu8(r14, (0x1u8) as u8); fl_l = ((((r9 as u32)) >> (29u32)) & 1) as u64; r9 = (((r9 as u32)) | ((1u32) << (29u32))) as u32 as u64; r8 = rdi; rdx = rsi; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rcx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 16; continue; } bb = 14; continue; }
            14 => { crate::simd_ops::wu64(P4, (0xau64) as u64); bb = 15; continue; }
            15 => { r9 = ((rbx as u32)) as u32 as u64; crate::simd_ops::wu8(r14, (0x0u8) as u8); r8 = rdi; rdx = rsi; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rcx = rax; bb = 16; continue; }
            16 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; crate::simd_ops::wu64(rax, rcx); fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 18; continue; } bb = 17; continue; }
            17 => { rax = 0; fl_l = 0; fl_r = 0; bb = 19; continue; }
            18 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp); bb = 19; continue; }
            19 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142541c40(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142645630;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    r8 = 0x41;
    crate::simd_ops::wu8(rdx, (0x0u8) as u8);
    rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp);
    rax = (((rax as u32)).wrapping_sub(1)) as u32 as u64;
    fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x3f;
    rax = (rax & !0xffu64) | (((if (((fl_l as u32)) <= ((fl_r as u32))) { 1u8 } else { 0u8 }) as u8 as u64));
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142541c70(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142534300; const P1: u64 = 0x1438673c8; const P2: u64 = 0x1426457c8; const P3: u64 = 0x1426457b0; const P4: u64 = 0x143a28fa6; const P5: u64 = 0x142645330; const P6: u64 = 0x1426457d0; const P7: u64 = 0x142646870; const P8: u64 = 0x142646848;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x50u64)) as u64; rbx = rcx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 16; continue; } bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru64(P1)) as u64; fl_l = rcx; fl_r = (0xffffffffffffffffu64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 6; continue; } bb = 2; continue; }
            2 => { rcx = 0xfffffff4; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(P1, rax); rcx = rax; fl_l = rax; fl_r = (0xffffffffffffffffu64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 5; continue; } bb = 3; continue; }
            3 => { rdx = rsp.wrapping_add(0x30); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(P1)) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rdx = (rdx & !0xffu64) | (((0x1u8) as u8 as u64)); crate::simd_ops::wu8(P4, ((rdx as u8)) as u8); bb = 7; continue; }
            5 => { rdx = (rdx & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; crate::simd_ops::wu8(P4, ((rdx as u8)) as u8); bb = 7; continue; }
            6 => { rdx = ((crate::simd_ops::ru8(P4)) as u32) as u32 as u64; bb = 7; continue; }
            7 => { fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 16; continue; } bb = 8; continue; }
            8 => { r10 = 0; fl_l = 0; fl_r = 0; r8 = ((r10 as u32)) as u32 as u64; fl_l = (crate::simd_ops::ru8(rbx)) as u8 as u64; fl_r = ((r8 as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 10; continue; } bb = 9; continue; }
            9 => { r8 = ((r8).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(r8))) as u8 as u64; fl_r = ((r10 as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 9; continue; } bb = 10; continue; }
            10 => { rax = r8.wrapping_sub(0x1); r9 = 0xfffffffd; fl_l = rax; fl_r = r9; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 16; continue; } bb = 11; continue; }
            11 => { crate::simd_ops::wu32(rsp.wrapping_add(0x68u64), ((r10 as u32)) as u32); fl_l = ((rdx as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 13; continue; } bb = 12; continue; }
            12 => { r9 = rsp.wrapping_add(0x68); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r10); rdx = rbx; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); rsp = ((rsp).wrapping_add(0x50u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            13 => { fl_l = rcx; fl_r = (0xffffffffffffffffu64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 15; continue; } bb = 14; continue; }
            14 => { r9 = rsp.wrapping_add(0x68); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r10); rdx = rbx; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp); rsp = ((rsp).wrapping_add(0x50u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            15 => { rcx = 0x2; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P7), rax), rax, rcx, rdx, r8, r9, rsp); rdx = rax; rcx = rbx; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P8), rax), rax, rcx, rdx, r8, r9, rsp); bb = 16; continue; }
            16 => { rsp = ((rsp).wrapping_add(0x50u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142541d70(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142540c40; const P1: u64 = 0x1426452e8; const P2: u64 = 0x143857d08;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rcx = P0;
    rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp);
    crate::simd_ops::wu32(P2, ((rax as u32)) as u32);
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142541da0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143857d08; const P1: u64 = 0x1426452f0;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rcx = (crate::simd_ops::ru32(P0)) as u32 as u64;
    return crate::simd_ops::call_fwd(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142541dc0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143857d08; const P1: u64 = 0x142645318;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rdx = rcx;
    rcx = (crate::simd_ops::ru32(P0)) as u32 as u64;
    return crate::simd_ops::call_fwd(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142541dd0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143d01c60; const P1: u64 = 0x142538840; const P2: u64 = 0x142538a60; const P3: u64 = 0x143426380; const P4: u64 = 0x142538300; const P5: u64 = 0x143c16458; const P6: u64 = 0x14253a550; const P7: u64 = 0x143c7d580; const P8: u64 = 0x143a09580; const P9: u64 = 0x14183acc2; const P10: u64 = 0x143c7f580; const P11: u64 = 0x143c7d200; const P12: u64 = 0x142539cc0; const P13: u64 = 0x143426350; const P14: u64 = 0x142537a30;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 27>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rbp = rsp.wrapping_sub(0x3f); rsp = ((rsp).wrapping_sub(0xc0u64)) as u64; rbx = r8; rdi = rcx; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = 0; fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0xc0u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            2 => { crate::simd_ops::wu64(rsp.wrapping_add(0xf0u64), r12); r12 = (0xffffffffffffffffu64) as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((r12 as u32)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xb8u64), r14); r14 = ((r12 as u32)) as u32 as u64; r14 = (if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { (rdx as u32) } else { (r14 as u32) }) as u32 as u64; fl_l = ((r14 as u32)) as u32 as u64; fl_r = ((r14 as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 6; continue; } bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(P0)) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rax; bb = 5; continue; }
            5 => { rax = ((((r14 as u32)) as i32 as i64) as u64) as u64; rdx = 0; fl_l = 0; fl_r = 0; let d = (rcx) as u128; let n = ((rdx as u128) << 64) | (rax as u128); if d != 0 { rax = (n / d) as u64; rdx = (n % d) as u64; } r14 = rdx; bb = 6; continue; }
            6 => { rax = 0; fl_l = 0; fl_r = 0; r9 = rbp.wrapping_sub(0x51); crate::simd_ops::wu64(rbp.wrapping_add(0x7fu64), rax); r8 = rbx; crate::simd_ops::wu64(rbp.wrapping_sub(0x51u64), rax); rdx = ((r14 as u32)) as u32 as u64; rax = rbp.wrapping_sub(0x31); rcx = rdi; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); rax = rbp.wrapping_add(0x7f); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rbp.wrapping_add(0x5fu64), rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 37; continue; } bb = 7; continue; }
            7 => { r8 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x51u64))) as u64; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 37; continue; } bb = 8; continue; }
            8 => { crate::simd_ops::wu64(rsp.wrapping_add(0xf8u64), r13); rcx = P3; crate::simd_ops::wu64(rsp.wrapping_add(0xe8u64), rsi); r9 = rdi; rdx = ((r14 as u32)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xb0u64), r15); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r13 = (crate::simd_ops::ru64(rbp.wrapping_add(0x7fu64))) as u64; fl_l = r13; fl_r = (0x2000000u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 35; continue; } bb = 9; continue; }
            9 => { r13 = ((r13).wrapping_shr(25u32)) as u64; rax = 0x10; xmm0 = 0u128; r15 = 0x20; crate::simd_ops::wu128(rbp.wrapping_sub(0x11u64), (xmm0) as u128); rsi = r13.wrapping_add(0x3f); rsi = ((rsi).wrapping_shr(6u32)) as u64; fl_l = (crate::simd_ops::ru8(rbp.wrapping_sub(0x21u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; crate::simd_ops::wu128(rbp.wrapping_sub(0x49u64), (xmm0) as u128); r15 = (if (((fl_l as u8)) != ((fl_r as u8))) { (rax as u32) } else { (r15 as u32) }) as u32 as u64; rax = 0; fl_l = 0; fl_r = 0; r15 = (((r15) as i64).wrapping_mul((rsi) as i64) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x1u64), rax); xmm0 = ((crate::simd_ops::ru64(rbp.wrapping_sub(0x1u64))) as u64) as u128; r15 = ((r15).wrapping_add(0x90u64)) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x39u64), ((xmm0 as u64)) as u64); fl_l = r15; fl_r = (0x2000u64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 14; continue; } bb = 10; continue; }
            10 => { rcx = (crate::simd_ops::ru64(P5)) as u64; rcx = ((rcx).wrapping_add(r15)) as u64; fl_l = rcx; fl_r = (0x2000u64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 14; continue; } bb = 11; continue; }
            11 => { rax = r15.wrapping_add(0xffffff); fl_l = rax; fl_r = (0x2000u64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 14; continue; } bb = 12; continue; }
            12 => { rbx = rax; let xa_src: u64 = rbx; let xa_old: u64 = crate::simd_ops::atomic_xadd_u64(P5, xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rbx = (xa_old) as u64; rax = ((rax).wrapping_add(rbx)) as u64; fl_l = rax; fl_r = (0x2000u64) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 18; continue; } bb = 13; continue; }
            13 => { fl_l = rax; let cx_src: u64 = rbx; let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u64(P5, fl_l as u64, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u64)) as u64; } bb = 14; continue; }
            14 => { rdx = rbp.wrapping_sub(0x49); rcx = r15; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = rax; rcx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 35; continue; } bb = 15; continue; }
            15 => { xmm3 = ((crate::simd_ops::ru64(rbp.wrapping_sub(0x39u64))) as u64) as u128; xmm2 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x49u64))) as u128; bb = 16; continue; }
            16 => { xmm0 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x31u64))) as u128; xmm1 = ((crate::simd_ops::ru64(rbp.wrapping_sub(0x21u64))) as u64) as u128; rdx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rbx.wrapping_add(0x38u64), r15); r15 = (crate::simd_ops::ru64(rbp.wrapping_add(0x5fu64))) as u64; rax = rsi.wrapping_add(0x11); crate::simd_ops::wu128(rbx.wrapping_add(0x8u64), (xmm0) as u128); crate::simd_ops::wu64(rbx.wrapping_add(0x18u64), ((xmm1 as u64)) as u64); rax = ((rbx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; crate::simd_ops::wu32(rbx, ((rdx as u32)) as u32); crate::simd_ops::wu128(rbx.wrapping_add(0x40u64), (xmm2) as u128); crate::simd_ops::wu16(rbx.wrapping_add(0x5cu64), (0x100u16) as u16); crate::simd_ops::wu64(rbx.wrapping_add(0x50u64), ((xmm3 as u64)) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x28u64), r13); crate::simd_ops::wu64(rbx.wrapping_add(0x30u64), rsi); crate::simd_ops::wu64(rbx.wrapping_add(0x20u64), r15); crate::simd_ops::wu32(rbx.wrapping_add(0x58u64), ((r14 as u32)) as u32); crate::simd_ops::wu64(rbx.wrapping_add(0x68u64), rdx); crate::simd_ops::wu64(rbx.wrapping_add(0x60u64), rdx); crate::simd_ops::wu64(rbx.wrapping_add(0x70u64), rax); fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0x18u64))) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 23; continue; } bb = 17; continue; }
            17 => { crate::simd_ops::wu64(rcx.wrapping_add(0x78u64), rdx); crate::simd_ops::wu64(rbx.wrapping_add(0x80u64), rdx); bb = 26; continue; }
            18 => { rax = 0; fl_l = 0; fl_r = 0; rbx = ((rbx).wrapping_add(0xffffffu64)) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x1fu64), rax); rbx = (((rbx) & (0xffffffffff000000u64))) as u64; xmm2 = 0u128; rax = P7; crate::simd_ops::wu32(rbp.wrapping_add(0x23u64), (0x2u32) as u32); xmm3 = ((crate::simd_ops::ru64(rbp.wrapping_add(0x1fu64))) as u64) as u128; rbx = ((rbx).wrapping_add(rax)) as u64; fl_l = (crate::simd_ops::ru8(P8)) as u8 as u64; fl_r = (0x0u8) as u8 as u64; crate::simd_ops::wu128(rbp.wrapping_add(0xfu64), (xmm2) as u128); crate::simd_ops::wu128(rbp.wrapping_sub(0x49u64), (xmm2) as u128); crate::simd_ops::wu64(rbp.wrapping_sub(0x39u64), ((xmm3 as u64)) as u64); if (((fl_l as u8)) == ((fl_r as u8))) { bb = 20; continue; } bb = 19; continue; }
            19 => { rdi = rbx; rax = 0; fl_l = 0; fl_r = 0; rcx = r15; while rcx != 0 { crate::simd_ops::wu8(rdi, rax as u8); rdi = rdi.wrapping_add(1); rcx = rcx.wrapping_sub(1); } bb = 21; continue; }
            20 => { r8 = r15; rdx = 0; fl_l = 0; fl_r = 0; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm3 = ((crate::simd_ops::ru64(rbp.wrapping_add(0x1fu64))) as u64) as u128; xmm2 = 0u128; bb = 21; continue; }
            21 => { fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 22; continue; }
            22 => { rcx = rbx; bb = 16; continue; }
            23 => { rdi = rsi; rdx = ((rsi.wrapping_mul(2).wrapping_add(0x11u64)) as u64) as u64; rdi = ((rdi).wrapping_shl(4u32)) as u64; rdx = ((rdx).wrapping_add(rsi)) as u64; rdi = ((rdi).wrapping_add(0x88u64)) as u64; rdi = ((rdi).wrapping_add(rbx)) as u64; crate::simd_ops::wu64(rcx.wrapping_add(0x78u64), rdi); rdx = ((rbx.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x80u64), rdx); fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 26; continue; } bb = 24; continue; }
            24 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0x19u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 26; continue; } bb = 25; continue; }
            25 => { r8 = ((rsi.wrapping_mul(8)) as u64) as u64; rdx = 0xff; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 26; continue; }
            26 => { rsi = ((rsi).wrapping_shl(6u32)) as u64; rcx = 0x1; rax = rsi; rax = ((rax).wrapping_sub(r13)) as u64; fl_l = rax; fl_r = rax; if (((((fl_l as u64)) & ((fl_r as u64))) as i64) <= 0) { bb = 30; continue; } bb = 27; continue; }
            27 => { rsi = ((rsi).wrapping_sub(rax)) as u64; rdx = rsi; rdx = ((rdx).wrapping_shr(6u32)) as u64; fl_l = rax; fl_r = (0x40u64) as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 29; continue; } bb = 28; continue; }
            28 => { rax = ((rcx).wrapping_shl((((rax) as u32) & 63))) as u64; rax = ((rax).wrapping_sub(1)) as u64; fl_l = ((((rsi as u32)) & (0x3fu32))) as u32 as u64; fl_r = fl_l; rsi = ((fl_l as u32)) as u32 as u64; r12 = ((rax).wrapping_shl((((rsi) as u32) & 63))) as u64; bb = 29; continue; }
            29 => { fl_l = (crate::simd_ops::lk_or_u64(rbx.wrapping_add(rdx.wrapping_mul(8)).wrapping_add(0x88u64), (r12) as u64)) as u64; fl_r = fl_l; bb = 30; continue; }
            30 => { let xa_src: u64 = rcx; let xa_old: u64 = crate::simd_ops::atomic_xadd_u64(P10, xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rcx = (xa_old) as u64; fl_l = rcx; fl_r = (0x70u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 32; continue; } bb = 31; continue; }
            31 => { fl_l = (crate::simd_ops::lk_sub_u64(P10, (1) as u64)) as u64; bb = 36; continue; }
            32 => { rax = ((rcx.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rbx, ((rax as u32)) as u32); rax = P11; crate::simd_ops::wu64(rax.wrapping_add(rcx.wrapping_mul(8)), rbx); rax = 0; fl_l = 0; fl_r = 0; bb = 33; continue; }
            33 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0xe8u64))) as u64; r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0xb0u64))) as u64; r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0xf8u64))) as u64; bb = 34; continue; }
            34 => { r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0xf0u64))) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0xb8u64))) as u64; rsp = ((rsp).wrapping_add(0xc0u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            35 => { r15 = (crate::simd_ops::ru64(rbp.wrapping_add(0x5fu64))) as u64; bb = 36; continue; }
            36 => { xmm0 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x31u64))) as u128; xmm1 = ((crate::simd_ops::ru64(rbp.wrapping_sub(0x21u64))) as u64) as u128; rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x7fu64))) as u64; r9 = rbp.wrapping_sub(0x11); r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); rcx = r15; crate::simd_ops::wu128(rbp.wrapping_sub(0x11u64), (xmm0) as u128); crate::simd_ops::wu64(rbp.wrapping_sub(0x1u64), ((xmm1 as u64)) as u64); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P12, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = 0xc; bb = 33; continue; }
            37 => { rdx = rdi; rcx = P13; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P14, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = 0xc; bb = 34; continue; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425421a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1425397a0; const P1: u64 = 0x1425422f0; const P2: u64 = 0x142539cc0; const P3: u64 = 0x143426330; const P4: u64 = 0x142538300; const P5: u64 = 0x1434263c8; const P6: u64 = 0x1426cdcb0; const P7: u64 = 0x1434263e0;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 17>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x80u64)) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0xb0u64))) as u64; r14 = (((rdx as u8)) as u32) as u32 as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu32(rbx, (0x0u32) as u32); bb = 2; continue; }
            2 => { rdi = rcx.wrapping_add(0x1ffffff); r9 = (((r8 as u8)) as u32) as u32 as u64; rax = rsp.wrapping_add(0x40); rdi = (((rdi) & (0xfffffffffe000000u64))) as u64; rcx = rdi; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); r8 = (((r14 as u8)) as u32) as u32 as u64; rdx = 0x2000000; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbp = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = ((rbp.wrapping_add(0xcu64)) as u32) as u32 as u64; bb = 8; continue; }
            4 => { xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x40u64))) as u128; xmm1 = ((crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64) as u128; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx); rax = rsp.wrapping_add(0x60); rbx = ((crate::simd_ops::ru8(rsp.wrapping_add(0x50u64))) as u32) as u32 as u64; r9 = 0xffffffff; crate::simd_ops::wu64(rsp.wrapping_add(0x90u64), rsi); r8 = (((rbx as u8)) as u32) as u32 as u64; rsi = rdi; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); rdx = rdi; rsi = ((rsi).wrapping_shr(10u32)) as u64; rcx = rbp; crate::simd_ops::wu128(rsp.wrapping_add(0x60u64), (xmm0) as u128); crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), ((xmm1 as u64)) as u64); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x40u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x60u64), (xmm0) as u128); xmm0 = ((crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64) as u128; r9 = rsp.wrapping_add(0x60); r8 = (((r14 as u8)) as u32) as u32 as u64; rdx = rdi; rcx = rbp; crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), ((xmm0 as u64)) as u64); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rsi; rcx = P3; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = 0xc; bb = 7; continue; }
            6 => { rax = P5; fl_l = ((rbx as u8)) as u8 as u64; fl_r = ((rbx as u8)) as u8 as u64; r8 = P6; rdx = rsi; r8 = (if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { rax } else { r8 }) as u64; rcx = P7; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = 0; fl_l = 0; fl_r = 0; bb = 7; continue; }
            7 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; bb = 8; continue; }
            8 => { r11 = rsp.wrapping_add(0x80); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x18u64))) as u64; rbp = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rdi = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; rsp = r11; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425422f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143c16458; const P1: u64 = 0x14253a550; const P2: u64 = 0x143c7d580; const P3: u64 = 0x143a09580; const P4: u64 = 0x14183acc2; const P5: u64 = 0x143c7f580; const P6: u64 = 0x143c7d200;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 21>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu8(rsp.wrapping_add(0x18u64), ((r8 as u8)) as u8); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x88u64)) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0xe0u64))) as u64; r13 = ((r9 as u32)) as u32 as u64; rdi = (((r8 as u8)) as u32) as u32 as u64; r15 = rdx; fl_l = r14; fl_r = r14; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu32(r14, (0x0u32) as u32); bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rsp.wrapping_add(0xb8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x80u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), r12); fl_l = r15; fl_r = (0x2000000u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 29; continue; } bb = 3; continue; }
            3 => { r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0xd8u64))) as u64; rax = 0x10; r15 = ((r15).wrapping_shr(25u32)) as u64; rbp = 0x20; xmm0 = 0u128; crate::simd_ops::wu128(rsp.wrapping_add(0x50u64), (xmm0) as u128); crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); rsi = r15.wrapping_add(0x3f); rsi = ((rsi).wrapping_shr(6u32)) as u64; fl_l = (crate::simd_ops::ru8(r12.wrapping_add(0x10u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; rbp = (if (((fl_l as u8)) != ((fl_r as u8))) { (rax as u32) } else { (rbp as u32) }) as u32 as u64; rax = 0; fl_l = 0; fl_r = 0; rbp = (((rbp) as i64).wrapping_mul((rsi) as i64) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rax); xmm0 = ((crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64) as u128; rbp = ((rbp).wrapping_add(0x90u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), ((xmm0 as u64)) as u64); fl_l = rbp; fl_r = (0x2000u64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 8; continue; } bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru64(P0)) as u64; rcx = ((rcx).wrapping_add(rbp)) as u64; fl_l = rcx; fl_r = (0x2000u64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 8; continue; } bb = 5; continue; }
            5 => { rax = rbp.wrapping_add(0xffffff); fl_l = rax; fl_r = (0x2000u64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 8; continue; } bb = 6; continue; }
            6 => { rbx = rax; let xa_src: u64 = rbx; let xa_old: u64 = crate::simd_ops::atomic_xadd_u64(P0, xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rbx = (xa_old) as u64; rax = ((rax).wrapping_add(rbx)) as u64; fl_l = rax; fl_r = (0x2000u64) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 12; continue; } bb = 7; continue; }
            7 => { fl_l = rax; let cx_src: u64 = rbx; let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u64(P0, fl_l as u64, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u64)) as u64; } bb = 8; continue; }
            8 => { rdx = rsp.wrapping_add(0x20); rcx = rbp; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = rax; rdx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 29; continue; } bb = 9; continue; }
            9 => { xmm3 = ((crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64) as u128; xmm2 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; bb = 10; continue; }
            10 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xb0u64))) as u64; rcx = 0; fl_l = 0; fl_r = 0; xmm0 = (crate::simd_ops::ru128(r12)) as u128; xmm1 = ((crate::simd_ops::ru64(r12.wrapping_add(0x10u64))) as u64) as u128; crate::simd_ops::wu64(rbx.wrapping_add(0x20u64), rax); rax = rsi.wrapping_add(0x11); crate::simd_ops::wu128(rbx.wrapping_add(0x8u64), (xmm0) as u128); rax = ((rbx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; crate::simd_ops::wu32(rbx, ((rcx as u32)) as u32); crate::simd_ops::wu64(rbx.wrapping_add(0x18u64), ((xmm1 as u64)) as u64); crate::simd_ops::wu128(rbx.wrapping_add(0x40u64), (xmm2) as u128); crate::simd_ops::wu64(rbx.wrapping_add(0x70u64), rax); crate::simd_ops::wu8(rbx.wrapping_add(0x5cu64), ((rcx as u8)) as u8); crate::simd_ops::wu64(rbx.wrapping_add(0x38u64), rbp); crate::simd_ops::wu64(rbx.wrapping_add(0x50u64), ((xmm3 as u64)) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x28u64), r15); crate::simd_ops::wu64(rbx.wrapping_add(0x30u64), rsi); crate::simd_ops::wu32(rbx.wrapping_add(0x58u64), ((r13 as u32)) as u32); crate::simd_ops::wu8(rbx.wrapping_add(0x5du64), ((rdi as u8)) as u8); crate::simd_ops::wu64(rbx.wrapping_add(0x68u64), rcx); crate::simd_ops::wu64(rbx.wrapping_add(0x60u64), rcx); fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0x18u64))) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 17; continue; } bb = 11; continue; }
            11 => { crate::simd_ops::wu64(rdx.wrapping_add(0x78u64), rcx); crate::simd_ops::wu64(rbx.wrapping_add(0x80u64), rcx); bb = 20; continue; }
            12 => { rax = 0; fl_l = 0; fl_r = 0; rbx = ((rbx).wrapping_add(0xffffffu64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rax); rbx = (((rbx) & (0xffffffffff000000u64))) as u64; xmm2 = 0u128; rax = P2; crate::simd_ops::wu32(rsp.wrapping_add(0x4cu64), (0x2u32) as u32); xmm3 = ((crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64) as u128; rbx = ((rbx).wrapping_add(rax)) as u64; fl_l = (crate::simd_ops::ru8(P3)) as u8 as u64; fl_r = (0x0u8) as u8 as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x38u64), (xmm2) as u128); crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm2) as u128); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), ((xmm3 as u64)) as u64); if (((fl_l as u8)) == ((fl_r as u8))) { bb = 14; continue; } bb = 13; continue; }
            13 => { rdi = rbx; rax = 0; fl_l = 0; fl_r = 0; rcx = rbp; while rcx != 0 { crate::simd_ops::wu8(rdi, rax as u8); rdi = rdi.wrapping_add(1); rcx = rcx.wrapping_sub(1); } rdi = ((crate::simd_ops::ru8(rsp.wrapping_add(0xc0u64))) as u32) as u32 as u64; bb = 15; continue; }
            14 => { r8 = rbp; rdx = 0; fl_l = 0; fl_r = 0; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm3 = ((crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64) as u128; xmm2 = 0u128; bb = 15; continue; }
            15 => { fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 16; continue; }
            16 => { rdx = rbx; bb = 10; continue; }
            17 => { rcx = rsi; rcx = ((rcx).wrapping_shl(4u32)) as u64; rcx = ((rcx).wrapping_add(0x88u64)) as u64; rcx = ((rcx).wrapping_add(rbx)) as u64; crate::simd_ops::wu64(rdx.wrapping_add(0x78u64), rcx); rdx = ((rsi.wrapping_mul(2).wrapping_add(0x11u64)) as u64) as u64; rdx = ((rdx).wrapping_add(rsi)) as u64; rdx = ((rbx.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x80u64), rdx); fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 20; continue; } bb = 18; continue; }
            18 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0x19u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 20; continue; } bb = 19; continue; }
            19 => { r8 = ((rsi.wrapping_mul(8)) as u64) as u64; rdx = 0xff; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 20; continue; }
            20 => { rsi = ((rsi).wrapping_shl(6u32)) as u64; rcx = 0x1; rax = rsi; rax = ((rax).wrapping_sub(r15)) as u64; fl_l = rax; fl_r = rax; if (((((fl_l as u64)) & ((fl_r as u64))) as i64) <= 0) { bb = 25; continue; } bb = 21; continue; }
            21 => { rsi = ((rsi).wrapping_sub(rax)) as u64; rdx = rsi; rdx = ((rdx).wrapping_shr(6u32)) as u64; fl_l = rax; fl_r = (0x40u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 23; continue; } bb = 22; continue; }
            22 => { rax = rcx.wrapping_sub(0x2); bb = 24; continue; }
            23 => { rax = ((rcx).wrapping_shl((((rax) as u32) & 63))) as u64; rax = ((rax).wrapping_sub(1)) as u64; fl_l = ((((rsi as u32)) & (0x3fu32))) as u32 as u64; fl_r = fl_l; rsi = ((fl_l as u32)) as u32 as u64; rax = ((rax).wrapping_shl((((rsi) as u32) & 63))) as u64; bb = 24; continue; }
            24 => { fl_l = (crate::simd_ops::lk_or_u64(rbx.wrapping_add(rdx.wrapping_mul(8)).wrapping_add(0x88u64), (rax) as u64)) as u64; fl_r = fl_l; bb = 25; continue; }
            25 => { fl_l = r14; fl_r = r14; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 27; continue; } bb = 26; continue; }
            26 => { crate::simd_ops::wu32(r14, (0xffffffffu32) as u32); bb = 27; continue; }
            27 => { let xa_src: u64 = rcx; let xa_old: u64 = crate::simd_ops::atomic_xadd_u64(P5, xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rcx = (xa_old) as u64; fl_l = rcx; fl_r = (0x70u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 31; continue; } bb = 28; continue; }
            28 => { fl_l = (crate::simd_ops::lk_sub_u64(P5, (1) as u64)) as u64; bb = 29; continue; }
            29 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 30; continue; }
            30 => { r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0xb8u64))) as u64; rsp = ((rsp).wrapping_add(0x88u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            31 => { rax = ((rcx.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rbx, ((rax as u32)) as u32); rax = P6; crate::simd_ops::wu64(rax.wrapping_add(rcx.wrapping_mul(8)), rbx); fl_l = r14; fl_r = r14; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 33; continue; } bb = 32; continue; }
            32 => { rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; crate::simd_ops::wu32(r14, ((rax as u32)) as u32); bb = 33; continue; }
            33 => { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 30; continue; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142542620(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x143c7f580; const P1: u64 = 0x1; const P2: u64 = 0x143c7d200;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = rdx; fl_r = rdx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { crate::simd_ops::wu32(rdx, (0xffffffffu32) as u32); }
    r8 = 0x1;
    let xa_src: u64 = r8; let xa_old: u64 = crate::simd_ops::atomic_xadd_u64(P0, xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64;
    r8 = (xa_old) as u64;
    fl_l = r8; fl_r = (0x70u64) as u64;
    if !((((fl_l as u64)) < ((fl_r as u64)))) { crate::simd_ops::lk_sub_u64(P0, (1) as u64); rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; return rax; }
    rax = ((r8.wrapping_add(P1)) as u32) as u32 as u64;
    crate::simd_ops::wu32(rcx, ((rax as u32)) as u32);
    rax = P2;
    crate::simd_ops::wu64(rax.wrapping_add(r8.wrapping_mul(8)), rcx);
    fl_l = rdx; fl_r = rdx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rax = (crate::simd_ops::ru32(rcx)) as u32 as u64; crate::simd_ops::wu32(rdx, ((rax as u32)) as u32); }
    rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64));
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142542670(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1425427b0;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    r8 = rdx;
    rdx = (rdx & !0xffu64) | (((0x1u8) as u8 as u64));
    rcx = (((rdx as u8)) as u32) as u32 as u64;
    return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142542690(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143c7f580; const P1: u64 = 0x143c7d200; const P2: u64 = 0x142539cc0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 21>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = rsp; crate::simd_ops::wu64(rax.wrapping_add(0x18u64), rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x90u64)) as u64; rsi = (crate::simd_ops::ru64(P0)) as u64; r14 = 0; fl_l = 0; fl_r = 0; rbp = ((r14 as u32)) as u32 as u64; rdi = ((r14 as u32)) as u32 as u64; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rax.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rax.wrapping_add(0x10u64), r15); r15 = P1; bb = 2; continue; }
            2 => { rbx = (crate::simd_ops::ru64(r15.wrapping_add(rdi.wrapping_mul(8)))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(0x20u64))) as u64; fl_r = r14; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 6; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x1cu64))) as u32 as u64; rax = (((rax as u32)).wrapping_add(0xfffffffdu32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 6; continue; } bb = 5; continue; }
            5 => { crate::simd_ops::wu64(r15.wrapping_add(rdi.wrapping_mul(8)), r14); r9 = rsp.wrapping_add(0x30); rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x28u64))) as u64; r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); xmm0 = (crate::simd_ops::ru128(rbx.wrapping_add(0x8u64))) as u128; xmm1 = ((crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64) as u128; rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x20u64))) as u64; rdx = ((rdx).wrapping_shl(25u32)) as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), ((xmm1 as u64)) as u64); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 7; continue; }
            6 => { rbp = rdi; bb = 7; continue; }
            7 => { xmm1 = ((crate::simd_ops::ru64(rbx.wrapping_add(0x50u64))) as u64) as u128; xmm0 = (crate::simd_ops::ru128(rbx.wrapping_add(0x40u64))) as u128; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), ((xmm1 as u64)) as u64); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rax = ((rax).wrapping_shr(32u32)) as u64; rax = (((rax as u32)).wrapping_add(0xfffffffdu32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 9; continue; } bb = 8; continue; }
            8 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x38u64))) as u64; r9 = rsp.wrapping_add(0x70); r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); rcx = rbx; crate::simd_ops::wu128(rsp.wrapping_add(0x70u64), (xmm0) as u128); crate::simd_ops::wu64(rsp.wrapping_add(0x80u64), ((xmm1 as u64)) as u64); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 9; continue; }
            9 => { rdi = ((rdi).wrapping_add(1)) as u64; fl_l = rdi; fl_r = rsi; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 2; continue; } bb = 10; continue; }
            10 => { r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0xb8u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0xb0u64))) as u64; bb = 11; continue; }
            11 => { rax = rsi; fl_l = rax; let cx_src: u64 = rbp; let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u64(P0, fl_l as u64, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u64)) as u64; } rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0xc0u64))) as u64; rsp = ((rsp).wrapping_add(0x90u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425427b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142534300; const P1: u64 = 0x143872dd4; const P2: u64 = 0x143872dd0; const P3: u64 = 0x1425375a0; const P4: u64 = 0x143872ef4; const P5: u64 = 0x143872ef0; const P6: u64 = 0x143c7f580; const P7: u64 = 0x143c164e8; const P8: u64 = 0x1425412f0; const P9: u64 = 0x143c7d200; const P10: u64 = 0x1425428d0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; r15 = r8; rbp = (((rdx as u8)) as u32) as u32 as u64; r12 = (((rcx as u8)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 19; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru32(P1)) as u32 as u64; fl_r = 0x0; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rbx); if (((fl_l as u32)) != ((fl_r as u32))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rcx = P2; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru32(P4)) as u32 as u64; fl_r = 0x0; rbx = (crate::simd_ops::ru32(P2)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rcx = P5; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru32(P5)) as u32 as u64; rax = ((((rax as u32)) as i32).wrapping_mul(((rbx as u32)) as i32) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 18; continue; } bb = 6; continue; }
            6 => { crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rsi); rsi = (crate::simd_ops::ru64(P6)) as u64; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 7; continue; }
            7 => { crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rdi); rax = 0; fl_l = 0; fl_r = 0; rdi = 0x1; fl_l = rax; let cx_src: u64 = rdi; let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u64(P7, fl_l as u64, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u64)) as u64; } fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 16; continue; } bb = 8; continue; }
            8 => { crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), r14); rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rbp as u8)) as u8 as u64; fl_r = ((rbp as u8)) as u8 as u64; r14 = rax; rdi = (if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { rsi } else { rdi }) as u64; rbx = 0; fl_l = 0; fl_r = 0; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 15; continue; } bb = 9; continue; }
            9 => { rbp = P9; bb = 10; continue; }
            10 => { rcx = (crate::simd_ops::ru64(rbp.wrapping_add(rbx.wrapping_mul(8)))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 11; continue; }
            11 => { r9 = r15; r8 = (((r12 as u8)) as u32) as u32 as u64; rdx = r14; rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 14; continue; } bb = 12; continue; }
            12 => { fl_l = rdi; fl_r = (0x1u64) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 15; continue; } bb = 13; continue; }
            13 => { fl_l = ((rdi).wrapping_sub(1)) as u64; rdi = ((fl_l as u64)) as u64; bb = 14; continue; }
            14 => { rbx = ((rbx).wrapping_add(1)) as u64; fl_l = rbx; fl_r = rsi; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 10; continue; } bb = 15; continue; }
            15 => { r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; crate::simd_ops::wu64(P7, (0x0u64) as u64); bb = 16; continue; }
            16 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; bb = 17; continue; }
            17 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; bb = 18; continue; }
            18 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; bb = 19; continue; }
            19 => { rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425428d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142542dc0; const P1: u64 = 0x142547120; const P2: u64 = 0x142542e80; const P3: u64 = 0x1425412f0;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 15>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r9); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = ((rsp).wrapping_sub(0x70u64)) as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0x18u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; r10 = r9; r13 = rcx; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 43; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(0x80u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 43; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x68u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 43; continue; } bb = 3; continue; }
            3 => { fl_l = ((r8 as u8)) as u8 as u64; fl_r = ((r8 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { fl_l = rax; fl_r = rdx; if ((((fl_l as u64)) as i64) > (((fl_r as u64)) as i64)) { bb = 43; continue; } bb = 5; continue; }
            5 => { crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rsi); rdx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rdi); fl_l = rax; let cx_src: u64 = rdx; let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u64(rcx.wrapping_add(0x68u64), fl_l as u64, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u64)) as u64; } rdi = (rdi & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rdx); rsi = (rsi & !0xffu64) | (((0x1u8) as u8 as u64)); r11 = ((rdx as u32)) as u32 as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x80u64), ((rsi as u8)) as u8); fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(0x30u64))) as u64; fl_r = rdx; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 42; continue; } bb = 6; continue; }
            6 => { crate::simd_ops::wu64(rsp.wrapping_add(0x88u64), rbx); rcx = ((rdx.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), rbp); rbx = ((rdx as u32)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), r12); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), r14); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), r15); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdx); bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru64(r13.wrapping_add(0x80u64))) as u64; r15 = (crate::simd_ops::ru64(rax.wrapping_add(r11.wrapping_mul(8)))) as u64; fl_l = r15; fl_r = r15; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 38; continue; } bb = 8; continue; }
            8 => { r12 = rdx; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rdx); bb = 9; continue; }
            9 => { r14 = rdx; rax = r12; bb = 10; continue; }
            10 => { fl_l = (((r15) >> (((rax) as u32 & 63))) & 1) as u64; if ((fl_l as u64) == 0) { bb = 12; continue; } bb = 11; continue; }
            11 => { r14 = ((r14).wrapping_add(1)) as u64; rax = ((rax).wrapping_add(1)) as u64; fl_l = rax; fl_r = (0x40u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 10; continue; } bb = 12; continue; }
            12 => { rax = ((rbx.wrapping_add(r12)) as u64) as u64; fl_l = r14; fl_r = r14; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 37; continue; } bb = 13; continue; }
            13 => { r9 = rax; r9 = ((((r9 as u32)) & (0x3fu32))) as u32 as u64; fl_l = ((rax).wrapping_shr(6u32)) as u64; rax = ((fl_l as u64)) as u64; r8 = ((rax.wrapping_mul(8)) as u64) as u64; bb = 14; continue; }
            14 => { fl_l = r14; fl_r = (0x40u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 16; continue; } bb = 15; continue; }
            15 => { rdx = (0xffffffffffffffffu64) as u64; bb = 17; continue; }
            16 => { rax = ((rcx).wrapping_shl((((r14) as u32) & 63))) as u64; fl_l = ((rax).wrapping_sub(1)) as u64; rax = ((fl_l as u64)) as u64; rdx = ((rax).wrapping_shl((((r9) as u32) & 63))) as u64; bb = 17; continue; }
            17 => { rax = (crate::simd_ops::ru64(r8.wrapping_add(r13).wrapping_add(0x88u64))) as u64; bb = 18; continue; }
            18 => { fl_l = rdx; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 31; continue; } bb = 19; continue; }
            19 => { rcx = rax; rcx = (((rcx) | (rdx))) as u64; fl_l = rax; let cx_src: u64 = rcx; let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u64(r8.wrapping_add(r13).wrapping_add(0x88u64), fl_l as u64, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u64)) as u64; } if (((fl_l as u64)) != ((fl_r as u64))) { bb = 18; continue; } bb = 20; continue; }
            20 => { fl_l = r14; fl_r = r14; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 35; continue; } bb = 21; continue; }
            21 => { rax = (crate::simd_ops::ru64(r13.wrapping_add(0x80u64))) as u64; rdi = ((r14.wrapping_add(r12)) as u64) as u64; rbp = (rbp & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rsi = r12; r15 = (crate::simd_ops::ru64(rax.wrapping_add(r11.wrapping_mul(8)))) as u64; fl_l = r12; fl_r = rdi; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 30; continue; } bb = 22; continue; }
            22 => { r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; bb = 23; continue; }
            23 => { rbx = 0; fl_l = 0; fl_r = 0; rax = rsi; bb = 24; continue; }
            24 => { fl_l = (((r15) >> (((rax) as u32 & 63))) & 1) as u64; if ((fl_l as u64) == 0) { bb = 26; continue; } bb = 25; continue; }
            25 => { rbx = ((rbx).wrapping_add(1)) as u64; rax = ((rax).wrapping_add(1)) as u64; fl_l = rax; fl_r = rdi; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 24; continue; } bb = 26; continue; }
            26 => { fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 28; continue; } bb = 27; continue; }
            27 => { rdx = ((r12.wrapping_add(rsi)) as u64) as u64; r9 = r10; r8 = rbx; rcx = r13; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); r10 = (crate::simd_ops::ru64(rsp.wrapping_add(0x98u64))) as u64; fl_l = rbx; fl_r = r14; rbp = (((rbp as u8)) as u32) as u32 as u64; rax = 0x1; rbp = (if (((fl_l as u64)) == ((fl_r as u64))) { (rax as u32) } else { (rbp as u32) }) as u32 as u64; bb = 28; continue; }
            28 => { rsi = ((rsi).wrapping_add(1)) as u64; rsi = ((rsi).wrapping_add(rbx)) as u64; fl_l = rsi; fl_r = rdi; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 23; continue; } bb = 29; continue; }
            29 => { r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; fl_l = ((rbp as u8)) as u8 as u64; fl_r = ((rbp as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 33; continue; } bb = 30; continue; }
            30 => { rsi = (rsi & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; crate::simd_ops::wu8(rsp.wrapping_add(0x80u64), ((rsi as u8)) as u8); bb = 34; continue; }
            31 => { rcx = 0x1; fl_l = r14; fl_r = (0x1u64) as u64; r14 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 36; continue; } bb = 32; continue; }
            32 => { bb = 14; continue; }
            33 => { rsi = ((crate::simd_ops::ru8(rsp.wrapping_add(0x80u64))) as u32) as u32 as u64; bb = 34; continue; }
            34 => { rcx = r13.wrapping_add(0x88); r8 = r14; r9 = ((rbx.wrapping_add(r12)) as u64) as u64; rdi = (rdi & !0xffu64) | (((0x1u8) as u8 as u64)); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); r10 = (crate::simd_ops::ru64(rsp.wrapping_add(0x98u64))) as u64; r11 = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; bb = 35; continue; }
            35 => { rcx = 0x1; bb = 36; continue; }
            36 => { rdx = 0; fl_l = 0; fl_r = 0; bb = 37; continue; }
            37 => { r12 = ((r12).wrapping_add(1)) as u64; r12 = ((r12).wrapping_add(r14)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), r12); fl_l = r12; fl_r = (0x40u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 9; continue; } bb = 38; continue; }
            38 => { r11 = ((r11).wrapping_add(1)) as u64; rbx = ((rbx).wrapping_add(0x40u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), r11); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbx); fl_l = r11; fl_r = (crate::simd_ops::ru64(r13.wrapping_add(0x30u64))) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 7; continue; } bb = 39; continue; }
            39 => { r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; fl_l = ((rsi as u8)) as u8 as u64; fl_r = ((rsi as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 41; continue; } bb = 40; continue; }
            40 => { rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rbx = ((((rax as u32)) as i32 as i64) as u64) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rdx = ((rax.wrapping_add(rbx)) as u64) as u64; rax = 0; fl_l = 0; fl_r = 0; fl_l = rax; let cx_src: u64 = rdx; let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u64(r13.wrapping_add(0x68u64), fl_l as u64, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u64)) as u64; } bb = 41; continue; }
            41 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x88u64))) as u64; bb = 42; continue; }
            42 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rax = (((rdi as u8)) as u32) as u32 as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x70u64)) as u64; r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            43 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x70u64)) as u64; r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142542b70(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142546890; const P1: u64 = 0x1425390a0; const P2: u64 = 0x1425408f0; const P3: u64 = 0x142546f80;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = rsp; crate::simd_ops::wu64(rax.wrapping_add(0x20u64), r9); crate::simd_ops::wu64(rax.wrapping_add(0x10u64), rdx); crate::simd_ops::wu64(rax.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x50u64)) as u64; rdi = ((r8.wrapping_add(r9)) as u64) as u64; rbp = (rbp & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rsi = r8; fl_l = r8; fl_r = rdi; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 14; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rax.wrapping_add(0x18u64), rbx); crate::simd_ops::wu64(rax.wrapping_sub(0x38u64), r15); r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; crate::simd_ops::wu64(rax.wrapping_sub(0x20u64), r12); crate::simd_ops::wu64(rax.wrapping_sub(0x28u64), r13); crate::simd_ops::wu64(rax.wrapping_sub(0x30u64), r14); bb = 2; continue; }
            2 => { rbx = 0; fl_l = 0; fl_r = 0; rax = rsi; bb = 3; continue; }
            3 => { fl_l = (((r15) >> (((rax) as u32 & 63))) & 1) as u64; if ((fl_l as u64) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rbx = ((rbx).wrapping_add(1)) as u64; rax = ((rax).wrapping_add(1)) as u64; fl_l = rax; fl_r = rdi; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 3; continue; } bb = 5; continue; }
            5 => { fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 6; continue; }
            6 => { r14 = rdx; r12 = rbx; r14 = ((r14).wrapping_shl(6u32)) as u64; r8 = rbx; r14 = ((r14).wrapping_add(rsi)) as u64; r12 = ((r12).wrapping_shl(25u32)) as u64; r13 = r14; r9 = r14; r13 = ((r13).wrapping_shl(25u32)) as u64; r13 = ((r13).wrapping_add(crate::simd_ops::ru64(rcx.wrapping_add(0x20u64)))) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x78u64))) as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0x98u64))) as u64; rdx = r12; rcx = r13; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); r13 = (((rax as u8)) as u32) as u32 as u64; bb = 9; continue; }
            8 => { r8 = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x98u64))) as u64; rdx = r12; rcx = ((rcx).wrapping_add(0x60u64)) as u64; r13 = (((rax as u8)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 9; continue; }
            9 => { r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; r9 = r14; r8 = rbx; rcx = (crate::simd_ops::ru64(r12.wrapping_add(0x80u64))) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); fl_l = ((r13 as u8)) as u8 as u64; fl_r = ((r13 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { rcx = (crate::simd_ops::ru64(r12.wrapping_add(0x78u64))) as u64; r9 = r14; r8 = rbx; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 11; continue; }
            11 => { fl_l = rbx; fl_r = (crate::simd_ops::ru64(rsp.wrapping_add(0x88u64))) as u64; rax = 0x1; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rbp = (((rbp as u8)) as u32) as u32 as u64; rbp = (if (((fl_l as u64)) == ((fl_r as u64))) { (rax as u32) } else { (rbp as u32) }) as u32 as u64; bb = 12; continue; }
            12 => { rsi = ((rsi).wrapping_add(1)) as u64; rsi = ((rsi).wrapping_add(rbx)) as u64; fl_l = rsi; fl_r = rdi; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 2; continue; } bb = 13; continue; }
            13 => { r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; bb = 14; continue; }
            14 => { rax = (((rbp as u8)) as u32) as u32 as u64; rsp = ((rsp).wrapping_add(0x50u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142542cd0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143872dd4; const P1: u64 = 0x143872dd0; const P2: u64 = 0x1425375a0; const P3: u64 = 0x143872ef4; const P4: u64 = 0x143872ef0; const P5: u64 = 0x142534300; const P6: u64 = 0x68; const P7: u64 = 0x1425412f0; const P8: u64 = 0x80; const P9: u64 = 0x1425474d0; const P10: u64 = 0x142542dc0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; fl_l = (crate::simd_ops::ru32(P0)) as u32 as u64; fl_r = 0x0; r15 = r9; rsi = r8; rbp = rdx; rdi = rcx; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = P1; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru32(P3)) as u32 as u64; fl_r = 0x0; r14 = (crate::simd_ops::ru32(P1)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rcx = P4; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { rbx = (crate::simd_ops::ru32(P4)) as u32 as u64; rbx = ((((rbx as u32)) as i32).wrapping_mul(((r14 as u32)) as i32) as u32) as u32 as u64; fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 12; continue; } bb = 5; continue; }
            5 => { rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 11; continue; } bb = 6; continue; }
            6 => { fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 11; continue; } bb = 7; continue; }
            7 => { fl_l = (crate::simd_ops::ru64(rdi.wrapping_add(P6))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 9; continue; } bb = 8; continue; }
            8 => { rax = 0x66666667; let prod = (rax as u32 as i32 as i64).wrapping_mul(((rbx as u32)) as i32 as i64) as u64; rax = prod as u32 as u64; rdx = (prod >> 32) as u32 as u64; rdx = (((((rdx as u32)) as i32).wrapping_shr(2u32) as u32)) as u32 as u64; rax = ((rdx as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shr(31u32)) as u32 as u64; rax = (((rax as u32)).wrapping_add((rdx as u32))) as u32 as u64; rax = (rax as u32 as i32 as i64) as u64; fl_l = (crate::simd_ops::lk_add_u64(rdi.wrapping_add(P6), (rax) as u64)) as u64; fl_r = fl_l; bb = 10; continue; }
            9 => { rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rcx = ((((rbx as u32)) as i32 as i64) as u64) as u64; fl_l = ((rax).wrapping_add(rcx)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; crate::simd_ops::wu64(rdi.wrapping_add(P6), rax); bb = 10; continue; }
            10 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P8))) as u64; r9 = rbp; r8 = rsi; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (0x0u64) as u64); rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); bb = 12; continue; }
            11 => { r9 = r15; r8 = rsi; rdx = rbp; rcx = rdi; rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp); bb = 12; continue; }
            12 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142542dc0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x20; const P1: u64 = 0x78; const P2: u64 = 0x142546890; const P3: u64 = 0x1425390a0; const P4: u64 = 0x60; const P5: u64 = 0x1425408f0; const P6: u64 = 0x80; const P7: u64 = 0x142546f80;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; r14 = rdx; r15 = r9; rsi = rcx; r14 = ((r14).wrapping_shl(25u32)) as u64; r14 = ((r14).wrapping_add(crate::simd_ops::ru64(rcx.wrapping_add(P0)))) as u64; rbp = r8; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; r9 = rdx; rbx = r8; rbp = ((rbp).wrapping_shl(25u32)) as u64; rdi = rdx; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); r9 = r15; rdx = rbp; rcx = r14; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); r14 = (((rax as u8)) as u32) as u32 as u64; bb = 3; continue; }
            2 => { r8 = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = ((r15.wrapping_add(P4)) as u64) as u64; rdx = rbp; r14 = (((rax as u8)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(rsi.wrapping_add(P6))) as u64; r9 = rdi; r8 = rbx; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); fl_l = ((r14 as u8)) as u8 as u64; fl_r = ((r14 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru64(rsi.wrapping_add(P1))) as u64; r9 = rdi; r8 = rbx; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            5 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142542e80(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143872dd4; const P1: u64 = 0x143872dd0; const P2: u64 = 0x1425375a0; const P3: u64 = 0x143872ef4; const P4: u64 = 0x143872ef0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    fl_l = (crate::simd_ops::ru32(P0)) as u32 as u64; fl_r = 0x0;
    if !((((fl_l as u32)) != ((fl_r as u32)))) { rcx = P1; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); }
    fl_l = (crate::simd_ops::ru32(P3)) as u32 as u64; fl_r = 0x0;
    rbx = (crate::simd_ops::ru32(P1)) as u32 as u64;
    if !((((fl_l as u32)) != ((fl_r as u32)))) { rcx = P4; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); }
    rax = (crate::simd_ops::ru32(P4)) as u32 as u64;
    rax = ((((rax as u32)) as i32).wrapping_mul(((rbx as u32)) as i32) as u32) as u32 as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142542ed0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142534300; const P1: u64 = 0x143c7f580; const P2: u64 = 0x143872ed4; const P3: u64 = 0x143872ed0; const P4: u64 = 0x1425375a0; const P5: u64 = 0x143872f4a; const P6: u64 = 0x143872c74; const P7: u64 = 0x143872c70; const P8: u64 = 0x143872f48; const P9: u64 = 0x1425421a0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rbp = r9; r14 = (((rdx as u8)) as u32) as u32 as u64; rsi = rcx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 20; continue; } bb = 1; continue; }
            1 => { fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 20; continue; } bb = 2; continue; }
            2 => { rbx = (crate::simd_ops::ru64(P1)) as u64; fl_l = rbx; fl_r = (0x6cu64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 20; continue; } bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru32(P2)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rcx = P3; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            5 => { rax = (((crate::simd_ops::ru32(P3)) as i32 as i64) as u64) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 20; continue; } bb = 6; continue; }
            6 => { rax = ((rax).wrapping_shl(10u32)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 20; continue; } bb = 7; continue; }
            7 => { fl_l = (crate::simd_ops::ru8(P5)) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 9; continue; } bb = 8; continue; }
            8 => { fl_l = ((rax).wrapping_shr(2u32)) as u64; rax = ((fl_l as u64)) as u64; bb = 9; continue; }
            9 => { rdi = rax.wrapping_add(0x1ffffff); rdi = (((rdi) & (0xfffffffffe000000u64))) as u64; fl_l = rbx; fl_r = (0x8u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 11; continue; } bb = 10; continue; }
            10 => { fl_l = ((rbx).wrapping_shr(3u32)) as u64; rbx = ((fl_l as u64)) as u64; rdi = ((rdi).wrapping_shl((((rbx) as u32) & 63))) as u64; bb = 11; continue; }
            11 => { fl_l = rdi; fl_r = rsi; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 20; continue; } bb = 12; continue; }
            12 => { rax = (crate::simd_ops::ru64(P6)) as u64; rbx = (rbx & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 14; continue; } bb = 13; continue; }
            13 => { rcx = P7; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(P6)) as u64; bb = 14; continue; }
            14 => { rcx = (crate::simd_ops::ru32(P7)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 16; continue; } bb = 15; continue; }
            15 => { rdx = ((crate::simd_ops::ru8(P8)) as u32) as u32 as u64; bb = 19; continue; }
            16 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 18; continue; } bb = 17; continue; }
            17 => { rcx = P7; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru32(P7)) as u32 as u64; bb = 18; continue; }
            18 => { rax = 0x1; rdx = (((rbx as u8)) as u32) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; rdx = (if (((fl_l as u32)) == ((fl_r as u32))) { (rax as u32) } else { (rdx as u32) }) as u32 as u64; bb = 19; continue; }
            19 => { r8 = (((r14 as u8)) as u32) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbp); rcx = rdi; rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; rax = (rax & !0xffu64) | (((if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { 1u8 } else { 0u8 }) as u8 as u64)); bb = 21; continue; }
            20 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 21; continue; }
            21 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142543010(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143c7f580; const P1: u64 = 0x143c7d200; const P2: u64 = 0x1425432a0;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu8(rsp.wrapping_add(0x20u64), ((r9 as u8)) as u8); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rsi = (crate::simd_ops::ru64(P0)) as u64; r13 = rdx; r10 = (crate::simd_ops::ru32(rsp.wrapping_add(0x88u64))) as u32 as u64; rbp = ((rcx as u32)) as u32 as u64; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 1; continue; }
            1 => { r12 = 0x70; fl_l = ((r10 as u32)) as u32 as u64; fl_r = ((r10 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 10; continue; } bb = 2; continue; }
            2 => { rcx = ((r10.wrapping_sub(0x1u64)) as u32) as u32 as u64; rax = ((rcx as u32)) as u32 as u64; rax = (if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { (r12 as u32) } else { (rax as u32) }) as u32 as u64; rax = (rax as u32 as i32 as i64) as u64; fl_l = rax; fl_r = rsi; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 8; continue; } bb = 3; continue; }
            3 => { r8 = rdx.wrapping_add(0x1ffffff); r8 = ((r8).wrapping_shr(25u32)) as u64; fl_l = ((r10 as u32)) as u32 as u64; fl_r = ((r10 as u32)) as u32 as u64; rcx = (if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { (r12 as u32) } else { (rcx as u32) }) as u32 as u64; rax = ((((rcx as u32)) as i32 as i64) as u64) as u64; rcx = P1; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(rax.wrapping_mul(8)))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::ru8(rsp.wrapping_add(0x80u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 6; continue; } bb = 5; continue; }
            5 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0x5du64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 8; continue; } bb = 6; continue; }
            6 => { fl_l = (crate::simd_ops::ru32(rcx)) as u32 as u64; fl_r = ((r10 as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 8; continue; } bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { rax = 0; fl_l = 0; fl_r = 0; bb = 9; continue; }
            9 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            10 => { r15 = ((crate::simd_ops::ru8(rsp.wrapping_add(0x80u64))) as u32) as u32 as u64; rcx = P1; rdi = 0; fl_l = 0; fl_r = 0; rbx = ((rdi as u32)) as u32 as u64; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 23; continue; } bb = 11; continue; }
            11 => { r14 = rdx.wrapping_add(0x1ffffff); fl_l = ((r14).wrapping_shr(25u32)) as u64; r14 = ((fl_l as u64)) as u64; bb = 12; continue; }
            12 => { fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; rax = ((rbx as u32)) as u32 as u64; rax = (if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { (r12 as u32) } else { (rax as u32) }) as u32 as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(rax.wrapping_mul(8)))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 22; continue; } bb = 13; continue; }
            13 => { fl_l = ((r15 as u8)) as u8 as u64; fl_r = ((r15 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 15; continue; } bb = 14; continue; }
            14 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0x5du64))) as u8 as u64; fl_r = ((rdi as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 22; continue; } bb = 15; continue; }
            15 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0x5cu64))) as u8 as u64; fl_r = ((rdi as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 17; continue; } bb = 16; continue; }
            16 => { fl_l = (crate::simd_ops::ru32(rcx)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 22; continue; } bb = 17; continue; }
            17 => { fl_l = ((rbp as u32)) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 20; continue; } bb = 18; continue; }
            18 => { rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x58u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 20; continue; } bb = 19; continue; }
            19 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 22; continue; } bb = 20; continue; }
            20 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; r8 = r14; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 9; continue; } bb = 21; continue; }
            21 => { r9 = ((crate::simd_ops::ru8(rsp.wrapping_add(0x78u64))) as u32) as u32 as u64; bb = 22; continue; }
            22 => { rbx = ((rbx).wrapping_add(1)) as u64; rcx = P1; fl_l = rbx; fl_r = rsi; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 12; continue; } bb = 23; continue; }
            23 => { fl_l = ((rbp as u32)) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 8; continue; } bb = 24; continue; }
            24 => { fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 25; continue; }
            25 => { rbx = r13.wrapping_add(0x1ffffff); fl_l = ((rbx).wrapping_shr(25u32)) as u64; rbx = ((fl_l as u64)) as u64; bb = 26; continue; }
            26 => { fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; rax = ((rdi as u32)) as u32 as u64; rax = (if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { (r12 as u32) } else { (rax as u32) }) as u32 as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(rax.wrapping_mul(8)))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 34; continue; } bb = 27; continue; }
            27 => { fl_l = ((r15 as u8)) as u8 as u64; fl_r = ((r15 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 29; continue; } bb = 28; continue; }
            28 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0x5du64))) as u8 as u64; fl_r = ((r15 as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 34; continue; } bb = 29; continue; }
            29 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0x5cu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 31; continue; } bb = 30; continue; }
            30 => { fl_l = (crate::simd_ops::ru32(rcx)) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 34; continue; } bb = 31; continue; }
            31 => { rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x58u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 34; continue; } bb = 32; continue; }
            32 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 34; continue; } bb = 33; continue; }
            33 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; r8 = rbx; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 9; continue; } bb = 34; continue; }
            34 => { rdi = ((rdi).wrapping_add(1)) as u64; fl_l = rdi; fl_r = rsi; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 8; continue; } bb = 35; continue; }
            35 => { r9 = ((crate::simd_ops::ru8(rsp.wrapping_add(0x78u64))) as u32) as u32 as u64; rcx = P1; bb = 26; continue; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142543210(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1ffffff; const P1: u64 = 0xffffffffffffffff; const P2: u64 = 0x143c7d200; const P3: u64 = 0x5d; const P4: u64 = 0x5c; const P5: u64 = 0x58; const P6: u64 = 0x1425432a0;
    let (mut rax, mut r10, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r10 = ((r9.wrapping_add(P0)) as u64) as u64; rax = 0x70; r10 = ((r10).wrapping_shr(25u32)) as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = ((rcx.wrapping_add(P1)) as u32) as u32 as u64; bb = 2; continue; }
            2 => { rax = (rax as u32 as i32 as i64) as u64; rcx = P2; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(rax.wrapping_mul(8)))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 16; continue; } bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru8(rsp.wrapping_add(0x38u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 5; continue; } bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(P3))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 16; continue; } bb = 5; continue; }
            5 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(P4))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x40u64))) as u32 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 7; continue; } bb = 6; continue; }
            6 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 9; continue; } bb = 7; continue; }
            7 => { fl_l = (crate::simd_ops::ru32(rcx)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 16; continue; } bb = 8; continue; }
            8 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 18; continue; } bb = 9; continue; }
            9 => { fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 13; continue; } bb = 10; continue; }
            10 => { rax = (crate::simd_ops::ru32(rcx.wrapping_add(P5))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 13; continue; } bb = 11; continue; }
            11 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 13; continue; } bb = 12; continue; }
            12 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 14; continue; }
            13 => { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 14; continue; }
            14 => { fl_l = ((rdx as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 17; continue; } bb = 15; continue; }
            15 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 18; continue; } bb = 16; continue; }
            16 => { rax = 0; fl_l = 0; fl_r = 0; return rax; }
            17 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 16; continue; } bb = 18; continue; }
            18 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; r8 = r10; r9 = ((crate::simd_ops::ru8(rsp.wrapping_add(0x30u64))) as u32) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); return crate::simd_ops::tail_stk(P6, rax, rcx, rdx, r8, r9, rsp); }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425432a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142547250; const P1: u64 = 0x142546f80; const P2: u64 = 0x1425474d0; const P3: u64 = 0x142539460; const P4: u64 = 0x142546890;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 15>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x60u64)) as u64; rsi = rcx; rax = rsp.wrapping_add(0x88); r15 = (((r9 as u8)) as u32) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); rcx = ((rcx).wrapping_add(0x88u64)) as u64; r9 = r8; rbx = r8; rdx = (crate::simd_ops::ru64(rsi.wrapping_add(0x30u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 16; continue; } bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x80u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x80u64), rbp); rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x88u64))) as u64; xmm0 = 0u128; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128); crate::simd_ops::wu64(rsp.wrapping_add(0x90u64), rdi); rax = rbp; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0xa0u64))) as u64; rax = ((rax).wrapping_shr(6u32)) as u64; crate::simd_ops::wu64(rsi.wrapping_add(0x60u64), rax); rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rax); rax = (crate::simd_ops::ru32(rsi)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), ((rax as u32)) as u32); rax = ((crate::simd_ops::ru8(rsi.wrapping_add(0x5cu64))) as u32) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x98u64), r14); r14 = rbp; crate::simd_ops::wu8(rsp.wrapping_add(0x3cu64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(rsi.wrapping_add(0x18u64))) as u32) as u32 as u64; r14 = ((r14).wrapping_shl(25u32)) as u64; r14 = ((r14).wrapping_add(crate::simd_ops::ru64(rsi.wrapping_add(0x20u64)))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbp); crate::simd_ops::wu32(rsp.wrapping_add(0x5cu64), (0x6u32) as u32); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; xmm1 = ((crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64) as u128; crate::simd_ops::wu128(rdi, (xmm0) as u128); crate::simd_ops::wu64(rdi.wrapping_add(0x10u64), ((xmm1 as u64)) as u64); crate::simd_ops::wu8(rdi.wrapping_add(0x10u64), ((rax as u8)) as u8); fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { r9 = rbp; r8 = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru8(rsi.wrapping_add(0x1au64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 6; continue; } bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x70u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { r9 = rbp; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (0x0u64) as u64); r8 = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu8(rdi.wrapping_add(0x12u64), ((rax as u8)) as u8); bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x78u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { crate::simd_ops::wu8(rdi.wrapping_add(0x11u64), (0x1u8) as u8); bb = 15; continue; }
            8 => { r9 = rbp; r8 = rbx; fl_l = ((r15 as u8)) as u8 as u64; fl_r = ((r15 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 14; continue; } bb = 9; continue; }
            9 => { rax = rsp.wrapping_add(0x88); crate::simd_ops::wu8(rdi.wrapping_add(0x11u64), (0x1u8) as u8); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru8(rsp.wrapping_add(0x88u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 15; continue; } bb = 10; continue; }
            10 => { rbx = ((rbx).wrapping_shl(25u32)) as u64; r8 = rsp.wrapping_add(0x88); rdx = rbx; crate::simd_ops::wu8(rsp.wrapping_add(0x88u64), (0x0u8) as u8); rcx = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 12; continue; } bb = 11; continue; }
            11 => { crate::simd_ops::wu8(rdi.wrapping_add(0x11u64), ((rax as u8)) as u8); bb = 15; continue; }
            12 => { fl_l = (crate::simd_ops::ru8(rsp.wrapping_add(0x88u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 15; continue; } bb = 13; continue; }
            13 => { crate::simd_ops::wu8(rdi.wrapping_add(0x12u64), (0x1u8) as u8); bb = 15; continue; }
            14 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu8(rdi.wrapping_add(0x11u64), ((rax as u8)) as u8); bb = 15; continue; }
            15 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; rax = r14; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x98u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; rsp = ((rsp).wrapping_add(0x60u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            16 => { rax = 0; fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x60u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142543460(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x30; const P1: u64 = 0x142547250; const P2: u64 = 0x60;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x30u64)) as u64;
    rbx = rcx;
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r8);
    r9 = rdx;
    rcx = ((rcx).wrapping_add(0x88u64)) as u64;
    rdi = r8;
    rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64;
    if !(((((fl_l as u8)) & ((fl_r as u8))) == 0)) { rax = (crate::simd_ops::ru64(rdi)) as u64; rax = ((rax).wrapping_shr(6u32)) as u64; crate::simd_ops::wu64(rbx.wrapping_add(P2), rax); rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64;
    rax = (rax & !0xffu64) | (((0u8) as u8 as u64));
    fl_l = 0; fl_r = 0;
    rsp = ((rsp).wrapping_add(0x30u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425434c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x20;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rdx = ((rdx).wrapping_shl(25u32)) as u64;
    fl_l = ((rdx).wrapping_add(crate::simd_ops::ru64(rcx.wrapping_add(P0)))) as u64; fl_r = fl_l;
    rdx = ((fl_l as u64)) as u64;
    rax = rdx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425434e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14; const P1: u64 = 0x10; const P2: u64 = 0x142539cc0;
    let (mut rax, mut r10, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = ((rsp).wrapping_sub(0x58u64)) as u64;
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64;
    r10 = r8;
    rax = (((rax as u32)).wrapping_add(0xfffffffdu32)) as u32 as u64;
    fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x2;
    if !((((fl_l as u32)) > ((fl_r as u32)))) { xmm0 = (crate::simd_ops::ru128(rdx)) as u128; xmm1 = ((crate::simd_ops::ru64(rdx.wrapping_add(P1))) as u64) as u128; rdx = r10; r9 = rsp.wrapping_add(0x30); r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), ((xmm1 as u64)) as u64); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); }
    rsp = ((rsp).wrapping_add(0x58u64)) as u64;
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142543530(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143c16458; const P1: u64 = 0x14253a550; const P2: u64 = 0x143c7d580; const P3: u64 = 0x143a09580; const P4: u64 = 0x14183acc2;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rax = 0; fl_l = 0; fl_r = 0; rbp = rdx; xmm0 = 0u128; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); rsi = rcx; crate::simd_ops::wu128(rdx, (xmm0) as u128); rax = rcx.wrapping_sub(0x1); crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); xmm0 = ((crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64) as u128; crate::simd_ops::wu64(rdx.wrapping_add(0x10u64), ((xmm0 as u64)) as u64); fl_l = rax; fl_r = (0x1fffu64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 5; continue; } bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru64(P0)) as u64; rcx = ((rcx).wrapping_add(rsi)) as u64; fl_l = rcx; fl_r = (0x2000u64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 5; continue; } bb = 2; continue; }
            2 => { rax = rsi.wrapping_add(0xffffff); fl_l = rax; fl_r = (0x2000u64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 5; continue; } bb = 3; continue; }
            3 => { rbx = rax; let xa_src: u64 = rbx; let xa_old: u64 = crate::simd_ops::atomic_xadd_u64(P0, xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rbx = (xa_old) as u64; rax = ((rax).wrapping_add(rbx)) as u64; fl_l = rax; fl_r = (0x2000u64) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 6; continue; } bb = 4; continue; }
            4 => { fl_l = rax; let cx_src: u64 = rbx; let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u64(P0, fl_l as u64, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u64)) as u64; } bb = 5; continue; }
            5 => { rdx = rbp; rcx = rsi; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); }
            6 => { rax = 0; fl_l = 0; fl_r = 0; rbx = ((rbx).wrapping_add(0xffffffu64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); rbx = (((rbx) & (0xffffffffff000000u64))) as u64; xmm0 = 0u128; crate::simd_ops::wu128(rdx, (xmm0) as u128); rax = P2; crate::simd_ops::wu32(rsp.wrapping_add(0x34u64), (0x2u32) as u32); xmm0 = ((crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64) as u128; rbx = ((rbx).wrapping_add(rax)) as u64; fl_l = (crate::simd_ops::ru8(P3)) as u8 as u64; fl_r = (0x0u8) as u8 as u64; crate::simd_ops::wu64(rdx.wrapping_add(0x10u64), ((xmm0 as u64)) as u64); if (((fl_l as u8)) == ((fl_r as u8))) { bb = 8; continue; } bb = 7; continue; }
            7 => { crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rdi); rax = 0; fl_l = 0; fl_r = 0; rdi = rbx; rcx = rsi; while rcx != 0 { crate::simd_ops::wu8(rdi, rax as u8); rdi = rdi.wrapping_add(1); rcx = rcx.wrapping_sub(1); } rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; bb = 9; continue; }
            8 => { r8 = rsi; rdx = 0; fl_l = 0; fl_r = 0; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 9; continue; }
            9 => { fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 10; continue; }
            10 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142543660(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143c16458; const P1: u64 = 0x143c7d580; const P2: u64 = 0x143a09580; const P3: u64 = 0x14183acc2;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rdi = r8; rax = 0; fl_l = 0; fl_r = 0; xmm0 = 0u128; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); r8 = rcx; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); crate::simd_ops::wu128(rdi, (xmm0) as u128); xmm0 = ((crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64) as u128; rax = rcx.wrapping_sub(0x1); crate::simd_ops::wu64(rdi.wrapping_add(0x10u64), ((xmm0 as u64)) as u64); fl_l = rax; fl_r = (0x1fffu64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 8; continue; } bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru64(P0)) as u64; rcx = ((rcx).wrapping_add(r8)) as u64; fl_l = rcx; fl_r = (0x2000u64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 8; continue; } bb = 2; continue; }
            2 => { rax = r8.wrapping_add(0xffffff); fl_l = rax; fl_r = (0x2000u64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 8; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rbx); rbx = rax; let xa_src: u64 = rbx; let xa_old: u64 = crate::simd_ops::atomic_xadd_u64(P0, xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rbx = (xa_old) as u64; rax = ((rax).wrapping_add(rbx)) as u64; fl_l = rax; fl_r = (0x2000u64) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 5; continue; } bb = 4; continue; }
            4 => { fl_l = rax; let cx_src: u64 = rbx; let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u64(P0, fl_l as u64, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u64)) as u64; } rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rax = 0; fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            5 => { rax = 0; fl_l = 0; fl_r = 0; rbx = ((rbx).wrapping_add(0xffffffu64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); rbx = (((rbx) & (0xffffffffff000000u64))) as u64; xmm0 = 0u128; crate::simd_ops::wu128(rdi, (xmm0) as u128); rax = P1; crate::simd_ops::wu32(rsp.wrapping_add(0x34u64), (0x2u32) as u32); xmm0 = ((crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64) as u128; rbx = ((rbx).wrapping_add(rax)) as u64; fl_l = (crate::simd_ops::ru8(P2)) as u8 as u64; fl_r = (0x0u8) as u8 as u64; crate::simd_ops::wu64(rdi.wrapping_add(0x10u64), ((xmm0 as u64)) as u64); if (((fl_l as u8)) == ((fl_r as u8))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rdi = rbx; rax = 0; fl_l = 0; fl_r = 0; rcx = r8; while rcx != 0 { crate::simd_ops::wu8(rdi, rax as u8); rdi = rdi.wrapping_add(1); rcx = rcx.wrapping_sub(1); } rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            7 => { rdx = 0; fl_l = 0; fl_r = 0; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            8 => { rax = 0; fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142543780(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xc;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64;
    fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64;
    if !((((((fl_l as u32)) & ((fl_r as u32))) as i32) > 0)) { rax = 0x70; rax = (rax as u32 as i32 as i64) as u64; crate::simd_ops::wu64(rdx, rax); rax = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::wu64(r8, rax); rax = ((crate::simd_ops::ru8(rcx.wrapping_add(P1))) as u32) as u32 as u64; return rax; }
    fl_l = (((rax as u32)).wrapping_sub(1)) as u32 as u64;
    rax = ((fl_l as u32)) as u32 as u64;
    rax = (rax as u32 as i32 as i64) as u64;
    crate::simd_ops::wu64(rdx, rax);
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    crate::simd_ops::wu64(r8, rax);
    rax = ((crate::simd_ops::ru8(rcx.wrapping_add(P1))) as u32) as u32 as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425437c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x8; const P2: u64 = 0xc;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rax);
    rax = rcx;
    xmm0 = 0u128;
    crate::simd_ops::wu128(rcx, (xmm0) as u128);
    crate::simd_ops::wu32(rsp.wrapping_add(0x14u64), (0x6u32) as u32);
    xmm0 = ((crate::simd_ops::ru64(rsp.wrapping_add(0x10u64))) as u64) as u128;
    crate::simd_ops::wu64(rcx.wrapping_add(P0), ((xmm0 as u64)) as u64);
    crate::simd_ops::wu32(rcx.wrapping_add(P1), ((rdx as u32)) as u32);
    crate::simd_ops::wu64(rcx, r9);
    crate::simd_ops::wu8(rcx.wrapping_add(P2), ((r8 as u8)) as u8);
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142543800(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x28;
    let (mut rax, mut fl_l) = (crate::simd_ops::get_rax_in(), 0u64);
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    fl_l = ((rax).wrapping_shl(25u32)) as u64;
    rax = ((fl_l as u64)) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142543810(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l) = (crate::simd_ops::get_rax_in(), 0u64);
    fl_l = ((rcx).wrapping_shl(25u32)) as u64;
    rcx = ((fl_l as u64)) as u64;
    rax = rcx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142543820(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x1ffffff;
    let (mut rax, mut fl_l) = (crate::simd_ops::get_rax_in(), 0u64);
    rax = ((rcx.wrapping_add(P0)) as u64) as u64;
    fl_l = ((rax).wrapping_shr(25u32)) as u64;
    rax = ((fl_l as u64)) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142543840(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = ((rdx as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 4; continue; } bb = 2; continue; }
            2 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; return rax; }
            4 => { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142543870(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x1;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = ((rcx.wrapping_add(P0)) as u32) as u32 as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142543880(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0xffffffffffffffff;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = 0x70;
    fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64;
    if !((((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0)) { rax = ((rcx.wrapping_add(P0)) as u32) as u32 as u64; }
    rax = (rax as u32 as i32 as i64) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142543930(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143d01c60; const P1: u64 = 0x142538840; const P2: u64 = 0x143bce008; const P3: u64 = 0x143be6098; const P4: u64 = 0x143bde068; const P5: u64 = 0x142645300;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = (crate::simd_ops::ru64(P0)) as u64; fl_l = rdi; fl_r = (0x1u64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = 0; fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            2 => { crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rbx); fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rdi = rax; bb = 4; continue; }
            4 => { rbx = 0; fl_l = 0; fl_r = 0; fl_l = rdi; fl_r = (0x1u64) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 14; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(P2)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 6; continue; }
            6 => { fl_l = (crate::simd_ops::ru64(P3)) as u64; fl_r = rbx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 9; continue; } bb = 7; continue; }
            7 => { rcx = rsp.wrapping_add(0x40); rax = crate::simd_ops::call_stk(rax, rax, rcx, rdx, r8, r9, rsp); rdx = rsp.wrapping_add(0x38); crate::simd_ops::wu16(rsp.wrapping_add(0x38u64), ((rbx as u16)) as u16); rcx = rsp.wrapping_add(0x40); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 12; continue; } bb = 8; continue; }
            8 => { rbx = ((crate::simd_ops::ru16(rsp.wrapping_add(0x38u64))) as u32) as u32 as u64; bb = 12; continue; }
            9 => { fl_l = (crate::simd_ops::ru64(P4)) as u64; fl_r = rbx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 12; continue; } bb = 10; continue; }
            10 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); rdx = rsp.wrapping_add(0x30); crate::simd_ops::wu8(rsp.wrapping_add(0x30u64), ((rbx as u8)) as u8); rcx = (((rax as u8)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 12; continue; } bb = 11; continue; }
            11 => { rbx = ((crate::simd_ops::ru8(rsp.wrapping_add(0x30u64))) as u32) as u32 as u64; bb = 12; continue; }
            12 => { rbx = (((rbx as u16)) as u32) as u32 as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 14; continue; } bb = 13; continue; }
            13 => { rax = ((rbx as u32)) as u32 as u64; rdx = 0; fl_l = 0; fl_r = 0; let d = (rdi) as u128; let n = ((rdx as u128) << 64) | (rax as u128); if d != 0 { rax = (n / d) as u64; rdx = (n % d) as u64; } rbx = rdx; bb = 14; continue; }
            14 => { rax = ((rbx as u32)) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142543a50(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0xffffffffffffffff;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = ((rdx.wrapping_add(P0)) as u64) as u64;
    r8 = rdx;
    rax = ((rax).wrapping_add(rcx)) as u64;
    rdx = 0;
    fl_l = 0; fl_r = 0;
    let d = (r8) as u128; let n = ((rdx as u128) << 64) | (rax as u128);
    if d != 0 { rax = (n / d) as u64; rdx = (n % d) as u64; }
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142543b90(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143d01c60; const P1: u64 = 0x142538840; const P2: u64 = 0x143bce008; const P3: u64 = 0x143be6098; const P4: u64 = 0x143bde068; const P5: u64 = 0x142645300; const P6: u64 = 0x142543010; const P7: u64 = 0x142534300; const P8: u64 = 0x143c7f580; const P9: u64 = 0x143872ed4; const P10: u64 = 0x143872ed0; const P11: u64 = 0x1425375a0; const P12: u64 = 0x143872f4a; const P13: u64 = 0x143872c74; const P14: u64 = 0x143872c70; const P15: u64 = 0x143872f48; const P16: u64 = 0x1425421a0; const P17: u64 = 0x143c7d200; const P18: u64 = 0x1425432a0; const P19: u64 = 0x143872e14; const P20: u64 = 0x143872e10; const P21: u64 = 0x1425397a0; const P22: u64 = 0x143872f30; const P23: u64 = 0x142539350; const P24: u64 = 0x1426467c0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 19>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu8(rsp.wrapping_add(0x20u64), ((r9 as u8)) as u8); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), r8); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x60u64)) as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0xd0u64))) as u64; rbp = rcx; rbx = (crate::simd_ops::ru64(P0)) as u64; rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rcx); rax = r8; r13 = rdx; xmm0 = 0u128; crate::simd_ops::wu128(r12, (xmm0) as u128); xmm0 = ((crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64) as u128; crate::simd_ops::wu64(r12.wrapping_add(0x10u64), ((xmm0 as u64)) as u64); fl_l = rbx; fl_r = (0x1u64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rdi = 0; fl_l = 0; fl_r = 0; rsi = ((rdi as u32)) as u32 as u64; bb = 16; continue; }
            2 => { fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = rax; bb = 4; continue; }
            4 => { rdi = 0; fl_l = 0; fl_r = 0; fl_l = rbx; fl_r = (0x1u64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rsi = ((rdi as u32)) as u32 as u64; bb = 15; continue; }
            6 => { rax = (crate::simd_ops::ru64(P2)) as u64; rsi = (((rdi as u16)) as u32) as u32 as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 7; continue; }
            7 => { fl_l = (crate::simd_ops::ru64(P3)) as u64; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 10; continue; } bb = 8; continue; }
            8 => { rcx = rsp.wrapping_add(0x40); (rax, xmm0) = crate::simd_ops::call_stk_xmm(rax, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rsp.wrapping_add(0xd8); crate::simd_ops::wu16(rsp.wrapping_add(0xd8u64), ((rdi as u16)) as u16); rcx = rsp.wrapping_add(0x40); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 13; continue; } bb = 9; continue; }
            9 => { rsi = ((crate::simd_ops::ru16(rsp.wrapping_add(0xd8u64))) as u32) as u32 as u64; bb = 13; continue; }
            10 => { fl_l = (crate::simd_ops::ru64(P4)) as u64; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 13; continue; } bb = 11; continue; }
            11 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rsp.wrapping_add(0xd0); crate::simd_ops::wu8(rsp.wrapping_add(0xd0u64), ((rsi as u8)) as u8); rcx = (((rax as u8)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 13; continue; } bb = 12; continue; }
            12 => { rsi = ((crate::simd_ops::ru8(rsp.wrapping_add(0xd0u64))) as u32) as u32 as u64; bb = 13; continue; }
            13 => { rsi = (((rsi as u16)) as u32) as u32 as u64; fl_l = rsi; fl_r = rbx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 15; continue; } bb = 14; continue; }
            14 => { rax = ((rsi as u32)) as u32 as u64; rdx = 0; fl_l = 0; fl_r = 0; let d = (rbx) as u128; let n = ((rdx as u128) << 64) | (rax as u128); if d != 0 { rax = (n / d) as u64; rdx = (n % d) as u64; } rsi = rdx; bb = 15; continue; }
            15 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xb0u64))) as u64; r9 = ((crate::simd_ops::ru8(rsp.wrapping_add(0xb8u64))) as u32) as u32 as u64; bb = 16; continue; }
            16 => { r15 = (crate::simd_ops::ru32(rsp.wrapping_add(0xc8u64))) as u32 as u64; r14 = ((crate::simd_ops::ru8(rsp.wrapping_add(0xc0u64))) as u32) as u32 as u64; fl_l = rbp; fl_r = (0x1000000u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 54; continue; } bb = 17; continue; }
            17 => { fl_l = r13; fl_r = (0x2000000u64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 54; continue; } bb = 18; continue; }
            18 => { fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 54; continue; } bb = 19; continue; }
            19 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), r12); rdx = rbp; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((r15 as u32)) as u32); rcx = ((rsi as u32)) as u32 as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x20u64), ((r14 as u8)) as u8); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 71; continue; } bb = 20; continue; }
            20 => { fl_l = ((r15 as u32)) as u32 as u64; fl_r = ((r15 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 54; continue; } bb = 21; continue; }
            21 => { crate::simd_ops::wu32(rsp.wrapping_add(0xd0u64), ((rdi as u32)) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 54; continue; } bb = 22; continue; }
            22 => { rbx = (crate::simd_ops::ru64(P8)) as u64; fl_l = rbx; fl_r = (0x6cu64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 54; continue; } bb = 23; continue; }
            23 => { fl_l = (crate::simd_ops::ru32(P9)) as u32 as u64; fl_r = ((r15 as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 25; continue; } bb = 24; continue; }
            24 => { rcx = P10; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P11, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 25; continue; }
            25 => { rax = (((crate::simd_ops::ru32(P10)) as i32 as i64) as u64) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 54; continue; } bb = 26; continue; }
            26 => { rax = ((rax).wrapping_shl(10u32)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 54; continue; } bb = 27; continue; }
            27 => { fl_l = (crate::simd_ops::ru8(P12)) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 29; continue; } bb = 28; continue; }
            28 => { fl_l = ((rax).wrapping_shr(2u32)) as u64; rax = ((fl_l as u64)) as u64; bb = 29; continue; }
            29 => { rbp = rax.wrapping_add(0x1ffffff); rbp = (((rbp) & (0xfffffffffe000000u64))) as u64; fl_l = rbx; fl_r = (0x8u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 31; continue; } bb = 30; continue; }
            30 => { fl_l = ((rbx).wrapping_shr(3u32)) as u64; rbx = ((fl_l as u64)) as u64; rbp = ((rbp).wrapping_shl((((rbx) as u32) & 63))) as u64; bb = 31; continue; }
            31 => { fl_l = rbp; fl_r = (crate::simd_ops::ru64(rsp.wrapping_add(0xa0u64))) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 53; continue; } bb = 32; continue; }
            32 => { rax = (crate::simd_ops::ru64(P13)) as u64; rbx = (rbx & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 34; continue; } bb = 33; continue; }
            33 => { rcx = P14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P11, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(P13)) as u64; bb = 34; continue; }
            34 => { rcx = (crate::simd_ops::ru32(P14)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 36; continue; } bb = 35; continue; }
            35 => { rdx = ((crate::simd_ops::ru8(P15)) as u32) as u32 as u64; bb = 39; continue; }
            36 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 38; continue; } bb = 37; continue; }
            37 => { rcx = P14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P11, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru32(P14)) as u32 as u64; bb = 38; continue; }
            38 => { rax = 0x1; rdx = (((rbx as u8)) as u32) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; rdx = (if (((fl_l as u32)) == ((fl_r as u32))) { (rax as u32) } else { (rdx as u32) }) as u32 as u64; bb = 39; continue; }
            39 => { rax = rsp.wrapping_add(0xd0); r8 = (((r14 as u8)) as u32) as u32 as u64; rcx = rbp; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P16, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0xa0u64))) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 54; continue; } bb = 40; continue; }
            40 => { rax = (crate::simd_ops::ru32(rsp.wrapping_add(0xd0u64))) as u32 as u64; r8 = rbp.wrapping_add(0x1ffffff); r8 = ((r8).wrapping_shr(25u32)) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) > 0) { bb = 42; continue; } bb = 41; continue; }
            41 => { rax = 0x70; bb = 43; continue; }
            42 => { fl_l = (((rax as u32)).wrapping_sub(1)) as u32 as u64; rax = ((fl_l as u32)) as u32 as u64; bb = 43; continue; }
            43 => { rax = (rax as u32 as i32 as i64) as u64; rcx = P17; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(rax.wrapping_mul(8)))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 54; continue; } bb = 44; continue; }
            44 => { fl_l = ((r14 as u8)) as u8 as u64; fl_r = ((r14 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 46; continue; } bb = 45; continue; }
            45 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0x5du64))) as u8 as u64; fl_r = ((rbx as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 54; continue; } bb = 46; continue; }
            46 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0x5cu64))) as u8 as u64; fl_r = ((rbx as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 48; continue; } bb = 47; continue; }
            47 => { fl_l = (crate::simd_ops::ru32(rcx)) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 54; continue; } bb = 48; continue; }
            48 => { fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 51; continue; } bb = 49; continue; }
            49 => { rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x58u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 51; continue; } bb = 50; continue; }
            50 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 54; continue; } bb = 51; continue; }
            51 => { rsi = ((crate::simd_ops::ru8(rsp.wrapping_add(0xb8u64))) as u32) as u32 as u64; r9 = (((rsi as u8)) as u32) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r12); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P18, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 55; continue; } bb = 52; continue; }
            52 => { bb = 71; continue; }
            53 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0xa0u64))) as u64; bb = 54; continue; }
            54 => { rsi = ((crate::simd_ops::ru8(rsp.wrapping_add(0xb8u64))) as u32) as u32 as u64; bb = 55; continue; }
            55 => { fl_l = (crate::simd_ops::ru32(P19)) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 57; continue; } bb = 56; continue; }
            56 => { rcx = P20; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P11, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 57; continue; }
            57 => { fl_l = (crate::simd_ops::ru32(P20)) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 70; continue; } bb = 58; continue; }
            58 => { fl_l = ((r15 as u32)) as u32 as u64; fl_r = ((r15 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 70; continue; } bb = 59; continue; }
            59 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0xb0u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 69; continue; } bb = 60; continue; }
            60 => { xmm0 = 0u128; rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rax); crate::simd_ops::wu128(r12, (xmm0) as u128); xmm0 = ((crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64) as u128; crate::simd_ops::wu64(r12.wrapping_add(0x10u64), ((xmm0 as u64)) as u64); fl_l = rcx; fl_r = (0x2000000u64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 68; continue; } bb = 61; continue; }
            61 => { rax = r13.wrapping_sub(0x1); rbx = ((rax.wrapping_add(rcx)) as u64) as u64; fl_l = r13; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 63; continue; } bb = 62; continue; }
            62 => { fl_l = ((!(rax)) & (rbx)) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; bb = 64; continue; }
            63 => { rdx = 0; fl_l = 0; fl_r = 0; rax = rbx; let d = (r13) as u128; let n = ((rdx as u128) << 64) | (rax as u128); if d != 0 { rax = (n / d) as u64; rdx = (n % d) as u64; } fl_l = rbx; fl_r = rdx; rbx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; bb = 64; continue; }
            64 => { rbx = ((rbx).wrapping_sub(rcx)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r12); r9 = (((r14 as u8)) as u32) as u32 as u64; r8 = (((rsi as u8)) as u32) as u32 as u64; rdx = r13; rcx = ((rbx.wrapping_add(rbp)) as u64) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P21, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 68; continue; } bb = 65; continue; }
            65 => { rdi = ((rax.wrapping_add(rbx)) as u64) as u64; fl_l = ((rsi as u8)) as u8 as u64; fl_r = ((rsi as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 68; continue; } bb = 66; continue; }
            66 => { fl_l = rbx; fl_r = (crate::simd_ops::ru64(P22)) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 68; continue; } bb = 67; continue; }
            67 => { r8 = rsp.wrapping_add(0xa0); rdx = rbx; rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P23, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 68; continue; }
            68 => { rax = rdi; bb = 71; continue; }
            69 => { r9 = (((r14 as u8)) as u32) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r12); r8 = (((rsi as u8)) as u32) as u32 as u64; rdx = r13; rcx = rbp; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P21, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 71; continue; }
            70 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P24), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(rax, (0xcu32) as u32); rax = 0; fl_l = 0; fl_r = 0; bb = 71; continue; }
            71 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0xa8u64))) as u64; rsp = ((rsp).wrapping_add(0x60u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142543fc0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1425408f0; const P1: u64 = 0x142539cc0; const P2: u64 = 0x1425427b0; const P3: u64 = 0x143c7d200; const P4: u64 = 0x143426250; const P5: u64 = 0x1425386a0; const P6: u64 = 0x1434262a0; const P7: u64 = 0x142546f80; const P8: u64 = 0x143872dd4; const P9: u64 = 0x143872dd0; const P10: u64 = 0x1425375a0; const P11: u64 = 0x143872ef4; const P12: u64 = 0x143872ef0; const P13: u64 = 0x142534300; const P14: u64 = 0x1425412f0; const P15: u64 = 0x1425474d0; const P16: u64 = 0x142542dc0; const P17: u64 = 0x1434262f0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 15>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 1; continue; }
            1 => { rax = rsp; rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x60u64)) as u64; rbx = r8; rsi = rdx; r15 = rcx; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 2; continue; }
            2 => { rdx = (crate::simd_ops::ru32(r9.wrapping_add(0x14u64))) as u32 as u64; crate::simd_ops::wu64(rax.wrapping_sub(0x20u64), r13); r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0xa0u64))) as u64; crate::simd_ops::wu64(rax.wrapping_add(0x8u64), rbp); crate::simd_ops::wu64(rax.wrapping_add(0x10u64), rdi); crate::simd_ops::wu64(rax.wrapping_sub(0x28u64), r14); rax = ((rdx.wrapping_sub(0x3u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 11; continue; } bb = 3; continue; }
            3 => { fl_l = rbx; fl_r = rsi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 6; continue; } bb = 4; continue; }
            4 => { fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rbx = (0u64.wrapping_sub(rbx)) as u64; rcx = r13.wrapping_add(0x60); rdx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 6; continue; }
            6 => { xmm0 = (crate::simd_ops::ru128(r9)) as u128; xmm1 = ((crate::simd_ops::ru64(r9.wrapping_add(0x10u64))) as u64) as u128; r9 = rsp.wrapping_add(0x30); r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); rdx = rsi; rcx = r15; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), ((xmm1 as u64)) as u64); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 7; continue; }
            7 => { r8 = r13; rdx = 0; fl_l = 0; fl_r = 0; rcx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 8; continue; }
            8 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x88u64))) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; bb = 9; continue; }
            9 => { rsp = ((rsp).wrapping_add(0x60u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); bb = 10; continue; }
            10 => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            11 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x6; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 7; continue; } bb = 12; continue; }
            12 => { xmm1 = (crate::simd_ops::ru128(r9)) as u128; xmm0 = ((crate::simd_ops::ru64(r9.wrapping_add(0x10u64))) as u64) as u128; rax = (((xmm1 >> 64) as u32) as u32) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), ((xmm0 as u64)) as u64); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) > 0) { bb = 14; continue; } bb = 13; continue; }
            13 => { rax = ((rdx.wrapping_add(0x6au64)) as u32) as u32 as u64; bb = 15; continue; }
            14 => { fl_l = (((rax as u32)).wrapping_sub(1)) as u32 as u64; rax = ((fl_l as u32)) as u32 as u64; bb = 15; continue; }
            15 => { rax = (rax as u32 as i32 as i64) as u64; rdi = P3; rbp = rsi.wrapping_add(0x1ffffff); rbp = ((rbp).wrapping_shr(25u32)) as u64; rdi = (crate::simd_ops::ru64(rdi.wrapping_add(rax.wrapping_mul(8)))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 17; continue; } bb = 16; continue; }
            16 => { crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm1) as u128); xmm1 = ((crate::simd_ops::ru64(r9.wrapping_add(0x10u64))) as u64) as u128; rax = rsp.wrapping_add(0x30); r9 = rsi; r8 = r15; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); rdx = P4; rcx = ((rdi.wrapping_add(0x16u64)) as u32) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), ((xmm1 as u64)) as u64); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 8; continue; }
            17 => { r14 = ((xmm1 as u64)) as u64; rax = r14; rax = ((rax).wrapping_shr(6u32)) as u64; fl_l = (crate::simd_ops::ru64(rdi.wrapping_add(0x30u64))) as u64; fl_r = rax; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 19; continue; } bb = 18; continue; }
            18 => { crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm1) as u128); xmm1 = ((crate::simd_ops::ru64(r9.wrapping_add(0x10u64))) as u64) as u128; rax = rsp.wrapping_add(0x30); r9 = rsi; r8 = r15; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); rdx = P6; rcx = 0x16; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), ((xmm1 as u64)) as u64); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 8; continue; }
            19 => { fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(0x18u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 36; continue; } bb = 20; continue; }
            20 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x78u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 36; continue; } bb = 21; continue; }
            21 => { crate::simd_ops::wu64(rsp.wrapping_add(0x90u64), r12); fl_l = rbx; fl_r = rsi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 24; continue; } bb = 22; continue; }
            22 => { r9 = r14; r8 = rbp; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 24; continue; } bb = 23; continue; }
            23 => { rbx = (0u64.wrapping_sub(rbx)) as u64; rcx = r13.wrapping_add(0x60); rdx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 24; continue; }
            24 => { fl_l = (crate::simd_ops::ru32(P8)) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 26; continue; } bb = 25; continue; }
            25 => { rcx = P9; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P10, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 26; continue; }
            26 => { fl_l = (crate::simd_ops::ru32(P11)) as u32 as u64; fl_r = 0x0; r12 = (crate::simd_ops::ru32(P9)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 28; continue; } bb = 27; continue; }
            27 => { rcx = P12; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P10, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 28; continue; }
            28 => { rbx = (crate::simd_ops::ru32(P12)) as u32 as u64; rbx = ((((rbx as u32)) as i32).wrapping_mul(((r12 as u32)) as i32) as u32) as u32 as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 36; continue; } bb = 29; continue; }
            29 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P13, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 35; continue; } bb = 30; continue; }
            30 => { fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 35; continue; } bb = 31; continue; }
            31 => { fl_l = (crate::simd_ops::ru64(rdi.wrapping_add(0x68u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 33; continue; } bb = 32; continue; }
            32 => { rax = 0x66666667; let prod = (rax as u32 as i32 as i64).wrapping_mul(((rbx as u32)) as i32 as i64) as u64; rax = prod as u32 as u64; rdx = (prod >> 32) as u32 as u64; rdx = (((((rdx as u32)) as i32).wrapping_shr(2u32) as u32)) as u32 as u64; rax = ((rdx as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shr(31u32)) as u32 as u64; rax = (((rax as u32)).wrapping_add((rdx as u32))) as u32 as u64; rax = (rax as u32 as i32 as i64) as u64; fl_l = (crate::simd_ops::lk_add_u64(rdi.wrapping_add(0x68u64), (rax) as u64)) as u64; fl_r = fl_l; bb = 34; continue; }
            33 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P14, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = ((((rbx as u32)) as i32 as i64) as u64) as u64; fl_l = ((rax).wrapping_add(rcx)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; crate::simd_ops::wu64(rdi.wrapping_add(0x68u64), rax); bb = 34; continue; }
            34 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x80u64))) as u64; r9 = r14; r8 = rbp; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (0x0u64) as u64); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P15, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 36; continue; }
            35 => { r9 = r13; r8 = rbp; rdx = r14; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P16, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 36; continue; }
            36 => { rcx = rdi.wrapping_add(0x88); r9 = r14; r8 = rbp; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 7; continue; } bb = 37; continue; }
            37 => { r9 = rsi; rdx = P17; r8 = r15; rcx = 0xb; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 8; continue; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142544280(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x14; const P1: u64 = 0xc; const P2: u64 = 0x8;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; fl_r = 0x6; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 6; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(P1))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 3; continue; } bb = 2; continue; }
            2 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 5; continue; } bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(P2))) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; return rax; }
            5 => { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); return rax; }
            6 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; rax = (rax & !0xffu64) | (((if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { 1u8 } else { 0u8 }) as u8 as u64)); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425442b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1425421a0;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x38u64)) as u64;
    r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64));
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (0x0u64) as u64);
    rdx = (((r8 as u8)) as u32) as u32 as u64;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x38u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142544300(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143c7f580; const P1: u64 = 0x143c7d200; const P2: u64 = 0x142539cc0; const P3: u64 = 0x1425427b0;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 21>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = rsp; crate::simd_ops::wu64(rax.wrapping_add(0x18u64), rbp); crate::simd_ops::wu64(rax.wrapping_add(0x20u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x90u64)) as u64; rsi = (crate::simd_ops::ru64(P0)) as u64; r15 = 0; fl_l = 0; fl_r = 0; r14 = rcx; rbp = ((r15 as u32)) as u32 as u64; rdi = ((r15 as u32)) as u32 as u64; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rax.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rax.wrapping_add(0x10u64), r12); r12 = P1; bb = 2; continue; }
            2 => { rbx = (crate::simd_ops::ru64(r12.wrapping_add(rdi.wrapping_mul(8)))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(0x20u64))) as u64; fl_r = r15; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 6; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x1cu64))) as u32 as u64; rax = (((rax as u32)).wrapping_add(0xfffffffdu32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 6; continue; } bb = 5; continue; }
            5 => { crate::simd_ops::wu64(r12.wrapping_add(rdi.wrapping_mul(8)), r15); r9 = rsp.wrapping_add(0x30); rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x28u64))) as u64; r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); xmm0 = (crate::simd_ops::ru128(rbx.wrapping_add(0x8u64))) as u128; xmm1 = ((crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64) as u128; rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x20u64))) as u64; rdx = ((rdx).wrapping_shl(25u32)) as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), ((xmm1 as u64)) as u64); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 7; continue; }
            6 => { rbp = rdi; bb = 7; continue; }
            7 => { xmm1 = ((crate::simd_ops::ru64(rbx.wrapping_add(0x50u64))) as u64) as u128; xmm0 = (crate::simd_ops::ru128(rbx.wrapping_add(0x40u64))) as u128; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), ((xmm1 as u64)) as u64); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rax = ((rax).wrapping_shr(32u32)) as u64; rax = (((rax as u32)).wrapping_add(0xfffffffdu32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 9; continue; } bb = 8; continue; }
            8 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x38u64))) as u64; r9 = rsp.wrapping_add(0x70); r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); rcx = rbx; crate::simd_ops::wu128(rsp.wrapping_add(0x70u64), (xmm0) as u128); crate::simd_ops::wu64(rsp.wrapping_add(0x80u64), ((xmm1 as u64)) as u64); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 9; continue; }
            9 => { rdi = ((rdi).wrapping_add(1)) as u64; fl_l = rdi; fl_r = rsi; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 2; continue; } bb = 10; continue; }
            10 => { r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0xb8u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0xb0u64))) as u64; bb = 11; continue; }
            11 => { rax = rsi; fl_l = rax; let cx_src: u64 = rbp; let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u64(P0, fl_l as u64, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u64)) as u64; } rdx = (rdx & !0xffu64) | (((0x1u8) as u8 as u64)); r8 = r14; rcx = (((rdx as u8)) as u32) as u32 as u64; r11 = rsp.wrapping_add(0x90); rbp = (crate::simd_ops::ru64(r11.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x38u64))) as u64; rsp = r11; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142544430(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143d01c60; const P1: u64 = 0x142538840; const P2: u64 = 0x142541dd0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rdi = r8; rbx = rcx; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 1; continue; }
            1 => { rsi = (crate::simd_ops::ru64(P0)) as u64; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rsi = rax; rax = 0x1; fl_l = rsi; fl_r = rsi; rsi = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rax } else { rsi }) as u64; bb = 3; continue; }
            3 => { rdx = 0; fl_l = 0; fl_r = 0; rax = rbx; let d = (rsi) as u128; let n = ((rdx as u128) << 64) | (rax as u128); if d != 0 { rax = (n / d) as u64; rdx = (n % d) as u64; } r12 = rdx; r15 = rax; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { r14 = 0; fl_l = 0; fl_r = 0; bb = 6; continue; }
            5 => { rdx = 0; fl_l = 0; fl_r = 0; rax = rdi; let d = (rsi) as u128; let n = ((rdx as u128) << 64) | (rax as u128); if d != 0 { rax = (n / d) as u64; rdx = (n % d) as u64; } rdi = 0; fl_l = 0; fl_r = 0; r14 = rax.wrapping_add(0x32); fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 6; continue; }
            6 => { fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 7; continue; }
            7 => { fl_l = rdi; fl_r = r12; rbp = r15.wrapping_add(0x1); r8 = r14; rdx = ((rdi as u32)) as u32 as u64; rbp = (if (((fl_l as u64)) >= ((fl_r as u64))) { r15 } else { rbp }) as u64; rcx = rbp; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 10; continue; } bb = 8; continue; }
            8 => { rax = rbx; rcx = rbx; rax = ((rax).wrapping_sub(rbp)) as u64; rbx = 0; fl_l = 0; fl_r = 0; rdi = ((rdi).wrapping_add(1)) as u64; fl_l = rcx; fl_r = rbp; rbx = (if (((fl_l as u64)) >= ((fl_r as u64))) { rax } else { rbx }) as u64; fl_l = rdi; fl_r = rsi; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 6; continue; } bb = 9; continue; }
            9 => { rax = 0; fl_l = 0; fl_r = 0; bb = 10; continue; }
            10 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142544510(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143c06158; const P1: u64 = 0x1434260c0; const P2: u64 = 0x142645308; const P3: u64 = 0x1434260d0; const P4: u64 = 0x142645758; const P5: u64 = 0x143426400; const P6: u64 = 0x142537a30; const P7: u64 = 0x1425448b0; const P8: u64 = 0x1425449e0;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rbx = rcx; fl_l = ((rdx as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 9; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(P0)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 6; continue; } bb = 2; continue; }
            2 => { rcx = P1; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rdx = P3; rcx = rax; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(P0, rax); bb = 5; continue; }
            4 => { rax = (crate::simd_ops::ru64(P0)) as u64; bb = 5; continue; }
            5 => { fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 6; continue; }
            6 => { r9 = 0x2; rdx = rsp.wrapping_add(0x20); rcx = 0; fl_l = 0; fl_r = 0; r8 = ((r9.wrapping_add(0x1eu64)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(rax, rax, rcx, rdx, r8, r9, rsp); rax = (((rax as u32)).wrapping_shr(31u32)) as u32 as u64; fl_l = ((((rax as u8)) ^ (0x1u8))) as u8 as u64; fl_r = fl_l; rax = (rax & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 21; continue; }
            8 => { rcx = P5; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); bb = 9; continue; }
            9 => { rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rcx = rax; rdx = (0x302b8631721c51beu64) as u64; r8 = (0xbf58476d1ce4e5b9u64) as u64; r9 = (0x94d049bb133111ebu64) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 15; continue; } bb = 10; continue; }
            10 => { rax = rdx; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x721c51beu32) as u32); rax = ((rax).wrapping_shr(30u32)) as u64; rax = (((rax) ^ (rdx))) as u64; rax = (((rax) as i64).wrapping_mul((r8) as i64) as u64) as u64; rcx = rax; rcx = ((rcx).wrapping_shr(27u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) as i64).wrapping_mul((r9) as i64) as u64) as u64; r10 = rcx; r10 = ((r10).wrapping_shr(31u32)) as u64; fl_l = (((r10) ^ (rcx))) as u64; fl_r = fl_l; r10 = ((fl_l as u64)) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x24u64), ((r10 as u32)) as u32); bb = 11; continue; }
            11 => { rax = r10; rax = ((rax).wrapping_shr(30u32)) as u64; rax = (((rax) ^ (r10))) as u64; rax = (((rax) as i64).wrapping_mul((r8) as i64) as u64) as u64; rcx = rax; rcx = ((rcx).wrapping_shr(27u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) as i64).wrapping_mul((r9) as i64) as u64) as u64; r10 = rcx; r10 = ((r10).wrapping_shr(31u32)) as u64; fl_l = (((r10) ^ (rcx))) as u64; fl_r = fl_l; r10 = ((fl_l as u64)) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((r10 as u32)) as u32); if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 17; continue; } bb = 12; continue; }
            12 => { rax = rdx; crate::simd_ops::wu32(rsp.wrapping_add(0x2cu64), (0x721c51beu32) as u32); rax = ((rax).wrapping_shr(30u32)) as u64; rax = (((rax) ^ (rdx))) as u64; rax = (((rax) as i64).wrapping_mul((r8) as i64) as u64) as u64; rcx = rax; rcx = ((rcx).wrapping_shr(27u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) as i64).wrapping_mul((r9) as i64) as u64) as u64; r10 = rcx; r10 = ((r10).wrapping_shr(31u32)) as u64; fl_l = (((r10) ^ (rcx))) as u64; fl_r = fl_l; r10 = ((fl_l as u64)) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((r10 as u32)) as u32); bb = 13; continue; }
            13 => { rax = r10; rax = ((rax).wrapping_shr(30u32)) as u64; rax = (((rax) ^ (r10))) as u64; rax = (((rax) as i64).wrapping_mul((r8) as i64) as u64) as u64; rcx = rax; rcx = ((rcx).wrapping_shr(27u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) as i64).wrapping_mul((r9) as i64) as u64) as u64; r10 = rcx; r10 = ((r10).wrapping_shr(31u32)) as u64; fl_l = (((r10) ^ (rcx))) as u64; fl_r = fl_l; r10 = ((fl_l as u64)) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x34u64), ((r10 as u32)) as u32); if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 19; continue; } bb = 14; continue; }
            14 => { crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), (0x721c51beu32) as u32); bb = 20; continue; }
            15 => { rax = ((rax).wrapping_shr(30u32)) as u64; rax = (((rax) ^ (rcx))) as u64; rax = (((rax) as i64).wrapping_mul((r8) as i64) as u64) as u64; rcx = rax; rcx = ((rcx).wrapping_shr(27u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) as i64).wrapping_mul((r9) as i64) as u64) as u64; r10 = rcx; r10 = ((r10).wrapping_shr(31u32)) as u64; fl_l = (((r10) ^ (rcx))) as u64; fl_r = fl_l; r10 = ((fl_l as u64)) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((r10 as u32)) as u32); if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 22; continue; } bb = 16; continue; }
            16 => { rax = rdx; crate::simd_ops::wu32(rsp.wrapping_add(0x24u64), (0x721c51beu32) as u32); rax = ((rax).wrapping_shr(30u32)) as u64; rax = (((rax) ^ (rdx))) as u64; rax = (((rax) as i64).wrapping_mul((r8) as i64) as u64) as u64; rcx = rax; rcx = ((rcx).wrapping_shr(27u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) as i64).wrapping_mul((r9) as i64) as u64) as u64; r10 = rcx; r10 = ((r10).wrapping_shr(31u32)) as u64; fl_l = (((r10) ^ (rcx))) as u64; fl_r = fl_l; r10 = ((fl_l as u64)) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((r10 as u32)) as u32); bb = 17; continue; }
            17 => { rax = r10; rax = ((rax).wrapping_shr(30u32)) as u64; rax = (((rax) ^ (r10))) as u64; rax = (((rax) as i64).wrapping_mul((r8) as i64) as u64) as u64; rcx = rax; rcx = ((rcx).wrapping_shr(27u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) as i64).wrapping_mul((r9) as i64) as u64) as u64; r10 = rcx; r10 = ((r10).wrapping_shr(31u32)) as u64; fl_l = (((r10) ^ (rcx))) as u64; fl_r = fl_l; r10 = ((fl_l as u64)) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x2cu64), ((r10 as u32)) as u32); if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 24; continue; } bb = 18; continue; }
            18 => { rax = rdx; crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), (0x721c51beu32) as u32); rax = ((rax).wrapping_shr(30u32)) as u64; rax = (((rax) ^ (rdx))) as u64; rax = (((rax) as i64).wrapping_mul((r8) as i64) as u64) as u64; rcx = rax; rcx = ((rcx).wrapping_shr(27u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) as i64).wrapping_mul((r9) as i64) as u64) as u64; r10 = rcx; r10 = ((r10).wrapping_shr(31u32)) as u64; fl_l = (((r10) ^ (rcx))) as u64; fl_r = fl_l; r10 = ((fl_l as u64)) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x34u64), ((r10 as u32)) as u32); bb = 19; continue; }
            19 => { rax = r10; rax = ((rax).wrapping_shr(30u32)) as u64; rax = (((rax) ^ (r10))) as u64; rax = (((rax) as i64).wrapping_mul((r8) as i64) as u64) as u64; rcx = rax; rcx = ((rcx).wrapping_shr(27u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; rax = 0x11; rcx = (((rcx) as i64).wrapping_mul((r9) as i64) as u64) as u64; rdx = rcx; rdx = ((rdx).wrapping_shr(31u32)) as u64; fl_l = (((rdx) ^ (rcx))) as u64; fl_r = fl_l; rdx = ((fl_l as u64)) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), ((rdx as u32)) as u32); rdx = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rax } else { rdx }) as u64; bb = 20; continue; }
            20 => { rax = rdx; rax = ((rax).wrapping_shr(30u32)) as u64; rax = (((rax) ^ (rdx))) as u64; rax = (((rax) as i64).wrapping_mul((r8) as i64) as u64) as u64; rcx = rax; rcx = ((rcx).wrapping_shr(27u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) as i64).wrapping_mul((r9) as i64) as u64) as u64; rax = rcx; rax = ((rax).wrapping_shr(31u32)) as u64; fl_l = (((rax) ^ (rcx))) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x3cu64), ((rax as u32)) as u32); rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 21; continue; }
            21 => { r8 = rbx; crate::simd_ops::wu8(rbx.wrapping_add(0x84u64), ((rax as u8)) as u8); rdx = rsp.wrapping_add(0x20); rcx = rbx; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); rsp = ((rsp).wrapping_add(0x40u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            22 => { rax = r10; rax = ((rax).wrapping_shr(30u32)) as u64; rax = (((rax) ^ (r10))) as u64; rax = (((rax) as i64).wrapping_mul((r8) as i64) as u64) as u64; rcx = rax; rcx = ((rcx).wrapping_shr(27u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) as i64).wrapping_mul((r9) as i64) as u64) as u64; r10 = rcx; r10 = ((r10).wrapping_shr(31u32)) as u64; fl_l = (((r10) ^ (rcx))) as u64; fl_r = fl_l; r10 = ((fl_l as u64)) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x24u64), ((r10 as u32)) as u32); if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 11; continue; } bb = 23; continue; }
            23 => { rax = rdx; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0x721c51beu32) as u32); rax = ((rax).wrapping_shr(30u32)) as u64; rax = (((rax) ^ (rdx))) as u64; rax = (((rax) as i64).wrapping_mul((r8) as i64) as u64) as u64; rcx = rax; rcx = ((rcx).wrapping_shr(27u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) as i64).wrapping_mul((r9) as i64) as u64) as u64; r10 = rcx; r10 = ((r10).wrapping_shr(31u32)) as u64; fl_l = (((r10) ^ (rcx))) as u64; fl_r = fl_l; r10 = ((fl_l as u64)) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x2cu64), ((r10 as u32)) as u32); bb = 24; continue; }
            24 => { rax = r10; rax = ((rax).wrapping_shr(30u32)) as u64; rax = (((rax) ^ (r10))) as u64; rax = (((rax) as i64).wrapping_mul((r8) as i64) as u64) as u64; rcx = rax; rcx = ((rcx).wrapping_shr(27u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) as i64).wrapping_mul((r9) as i64) as u64) as u64; r10 = rcx; r10 = ((r10).wrapping_shr(31u32)) as u64; fl_l = (((r10) ^ (rcx))) as u64; fl_r = fl_l; r10 = ((fl_l as u64)) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((r10 as u32)) as u32); if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 13; continue; } bb = 25; continue; }
            25 => { rax = rdx; crate::simd_ops::wu32(rsp.wrapping_add(0x34u64), (0x721c51beu32) as u32); rax = ((rax).wrapping_shr(30u32)) as u64; rax = (((rax) ^ (rdx))) as u64; rax = (((rax) as i64).wrapping_mul((r8) as i64) as u64) as u64; rcx = rax; rcx = ((rcx).wrapping_shr(27u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) as i64).wrapping_mul((r9) as i64) as u64) as u64; rdx = rcx; rdx = ((rdx).wrapping_shr(31u32)) as u64; fl_l = (((rdx) ^ (rcx))) as u64; fl_r = fl_l; rdx = ((fl_l as u64)) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), ((rdx as u32)) as u32); bb = 20; continue; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425448b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1425412f0; const P1: u64 = 0x1425448b0;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rcx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rcx = P1; rax = (((rax) ^ (rcx))) as u64; rax = (((rax) ^ (rbx))) as u64; r8 = rax; r8 = ((r8).wrapping_shr(17u32)) as u64; r8 = (((r8) ^ (rax))) as u64; r8 = ((((r8 as u32)) & (0xfu32))) as u32 as u64; fl_l = ((r8).wrapping_add(0x1u64)) as u64; fl_r = fl_l; r8 = ((fl_l as u64)) as u64; if (((fl_l as u64)) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { r9 = 0x11; r10 = (0xbf58476d1ce4e5b9u64) as u64; r11 = (0x94d049bb133111ebu64) as u64; bb = 2; continue; }
            2 => { fl_l = rax; fl_r = rax; rax = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { r9 } else { rax }) as u64; rcx = rax; rcx = ((rcx).wrapping_shr(30u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) as i64).wrapping_mul((r10) as i64) as u64) as u64; rdx = rcx; rdx = ((rdx).wrapping_shr(27u32)) as u64; rdx = (((rdx) ^ (rcx))) as u64; rdx = (((rdx) as i64).wrapping_mul((r11) as i64) as u64) as u64; rax = rdx; rax = ((rax).wrapping_shr(31u32)) as u64; rax = (((rax) ^ (rdx))) as u64; fl_l = r8; fl_r = (0x1u64) as u64; r8 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 3; continue; }
            3 => { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142544940(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x80; const P1: u64 = 0x142544c00; const P2: u64 = 0x40;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; rbx = rcx; rdi = 0x10; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) > 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rax = ((rdi as u32)) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(P0), ((rdi as u32)) as u32); bb = 2; continue; }
            2 => { rcx = 0x20; rcx = ((rcx).wrapping_sub(rax)) as u64; rsi = (crate::simd_ops::ru32(rbx.wrapping_add(rcx.wrapping_mul(4)))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(rcx.wrapping_mul(4)), (0x0u32) as u32); crate::simd_ops::lk_sub_u32(rbx.wrapping_add(P0), (1) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_add(P0))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) > 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rcx = rbx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rax = ((rdi as u32)) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(P0), ((rdi as u32)) as u32); bb = 4; continue; }
            4 => { rdi = (((rdi as u32)).wrapping_sub((rax as u32))) as u32 as u64; rax = ((((rdi as u32)) as i32 as i64) as u64) as u64; rcx = (crate::simd_ops::ru32(rbx.wrapping_add(rax.wrapping_mul(4)).wrapping_add(P2))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(rax.wrapping_mul(4)).wrapping_add(P2), (0x0u32) as u32); rax = rsi; crate::simd_ops::lk_sub_u32(rbx.wrapping_add(P0), (1) as u32); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rax = ((rax).wrapping_shl(32u32)) as u64; rax = (((rax) | (rcx))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425449e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut r10, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r10 = rcx; r9 = rdx; rax = 0; fl_l = 0; fl_r = 0; xmm0 = 0u128; ymm0_hi = 0; let y_addr: u64 = rcx.wrapping_add(0x10u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); let y_addr: u64 = rcx.wrapping_add(0x30u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); let y_addr: u64 = rcx.wrapping_add(0x50u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); crate::simd_ops::wu128(rcx.wrapping_add(0x70u64), (xmm0) as u128); crate::simd_ops::wu64(rcx.wrapping_add(0x80u64), rax); crate::simd_ops::wu32(rcx, (0x61707865u32) as u32); crate::simd_ops::wu32(rcx.wrapping_add(0x4u64), (0x3320646eu32) as u32); crate::simd_ops::wu32(rcx.wrapping_add(0x8u64), (0x79622d32u32) as u32); crate::simd_ops::wu32(rcx.wrapping_add(0xcu64), (0x6b206574u32) as u32); rdx = ((crate::simd_ops::ru8(rdx.wrapping_add(0x3u64))) as u32) as u32 as u64; rax = ((crate::simd_ops::ru8(r9.wrapping_add(0x2u64))) as u32) as u32 as u64; rdx = (((rdx as u32)).wrapping_shl(8u32)) as u32 as u64; rdx = ((((rdx as u32)) | ((rax as u32)))) as u32 as u64; rax = ((crate::simd_ops::ru8(r9.wrapping_add(0x1u64))) as u32) as u32 as u64; rdx = (((rdx as u32)).wrapping_shl(8u32)) as u32 as u64; rdx = ((((rdx as u32)) | ((rax as u32)))) as u32 as u64; rax = ((crate::simd_ops::ru8(r9)) as u32) as u32 as u64; rdx = (((rdx as u32)).wrapping_shl(8u32)) as u32 as u64; rdx = ((((rdx as u32)) | ((rax as u32)))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x10u64), ((rdx as u32)) as u32); rax = ((crate::simd_ops::ru8(r9.wrapping_add(0x6u64))) as u32) as u32 as u64; rcx = ((crate::simd_ops::ru8(r9.wrapping_add(0x7u64))) as u32) as u32 as u64; rcx = (((rcx as u32)).wrapping_shl(8u32)) as u32 as u64; rcx = ((((rcx as u32)) | ((rax as u32)))) as u32 as u64; rax = ((crate::simd_ops::ru8(r9.wrapping_add(0x5u64))) as u32) as u32 as u64; rcx = (((rcx as u32)).wrapping_shl(8u32)) as u32 as u64; rcx = ((((rcx as u32)) | ((rax as u32)))) as u32 as u64; rax = ((crate::simd_ops::ru8(r9.wrapping_add(0x4u64))) as u32) as u32 as u64; rcx = (((rcx as u32)).wrapping_shl(8u32)) as u32 as u64; rcx = ((((rcx as u32)) | ((rax as u32)))) as u32 as u64; crate::simd_ops::wu32(r10.wrapping_add(0x14u64), ((rcx as u32)) as u32); rax = ((crate::simd_ops::ru8(r9.wrapping_add(0xau64))) as u32) as u32 as u64; rcx = ((crate::simd_ops::ru8(r9.wrapping_add(0xbu64))) as u32) as u32 as u64; rcx = (((rcx as u32)).wrapping_shl(8u32)) as u32 as u64; rcx = ((((rcx as u32)) | ((rax as u32)))) as u32 as u64; rax = ((crate::simd_ops::ru8(r9.wrapping_add(0x9u64))) as u32) as u32 as u64; rcx = (((rcx as u32)).wrapping_shl(8u32)) as u32 as u64; rcx = ((((rcx as u32)) | ((rax as u32)))) as u32 as u64; rax = ((crate::simd_ops::ru8(r9.wrapping_add(0x8u64))) as u32) as u32 as u64; rcx = (((rcx as u32)).wrapping_shl(8u32)) as u32 as u64; rcx = ((((rcx as u32)) | ((rax as u32)))) as u32 as u64; crate::simd_ops::wu32(r10.wrapping_add(0x18u64), ((rcx as u32)) as u32); rax = ((crate::simd_ops::ru8(r9.wrapping_add(0xeu64))) as u32) as u32 as u64; rcx = ((crate::simd_ops::ru8(r9.wrapping_add(0xfu64))) as u32) as u32 as u64; rcx = (((rcx as u32)).wrapping_shl(8u32)) as u32 as u64; rcx = ((((rcx as u32)) | ((rax as u32)))) as u32 as u64; rax = ((crate::simd_ops::ru8(r9.wrapping_add(0xdu64))) as u32) as u32 as u64; rcx = (((rcx as u32)).wrapping_shl(8u32)) as u32 as u64; rcx = ((((rcx as u32)) | ((rax as u32)))) as u32 as u64; rax = ((crate::simd_ops::ru8(r9.wrapping_add(0xcu64))) as u32) as u32 as u64; rcx = (((rcx as u32)).wrapping_shl(8u32)) as u32 as u64; rcx = ((((rcx as u32)) | ((rax as u32)))) as u32 as u64; crate::simd_ops::wu32(r10.wrapping_add(0x1cu64), ((rcx as u32)) as u32); rax = ((crate::simd_ops::ru8(r9.wrapping_add(0x12u64))) as u32) as u32 as u64; rcx = ((crate::simd_ops::ru8(r9.wrapping_add(0x13u64))) as u32) as u32 as u64; rcx = (((rcx as u32)).wrapping_shl(8u32)) as u32 as u64; rcx = ((((rcx as u32)) | ((rax as u32)))) as u32 as u64; rax = ((crate::simd_ops::ru8(r9.wrapping_add(0x11u64))) as u32) as u32 as u64; rcx = (((rcx as u32)).wrapping_shl(8u32)) as u32 as u64; rcx = ((((rcx as u32)) | ((rax as u32)))) as u32 as u64; rax = ((crate::simd_ops::ru8(r9.wrapping_add(0x10u64))) as u32) as u32 as u64; rcx = (((rcx as u32)).wrapping_shl(8u32)) as u32 as u64; rcx = ((((rcx as u32)) | ((rax as u32)))) as u32 as u64; crate::simd_ops::wu32(r10.wrapping_add(0x20u64), ((rcx as u32)) as u32); rax = ((crate::simd_ops::ru8(r9.wrapping_add(0x16u64))) as u32) as u32 as u64; rcx = ((crate::simd_ops::ru8(r9.wrapping_add(0x17u64))) as u32) as u32 as u64; rcx = (((rcx as u32)).wrapping_shl(8u32)) as u32 as u64; rcx = ((((rcx as u32)) | ((rax as u32)))) as u32 as u64; rax = ((crate::simd_ops::ru8(r9.wrapping_add(0x15u64))) as u32) as u32 as u64; rcx = (((rcx as u32)).wrapping_shl(8u32)) as u32 as u64; rcx = ((((rcx as u32)) | ((rax as u32)))) as u32 as u64; rax = ((crate::simd_ops::ru8(r9.wrapping_add(0x14u64))) as u32) as u32 as u64; rcx = (((rcx as u32)).wrapping_shl(8u32)) as u32 as u64; rcx = ((((rcx as u32)) | ((rax as u32)))) as u32 as u64; crate::simd_ops::wu32(r10.wrapping_add(0x24u64), ((rcx as u32)) as u32); rcx = ((crate::simd_ops::ru8(r9.wrapping_add(0x1bu64))) as u32) as u32 as u64; rcx = (((rcx as u32)).wrapping_shl(8u32)) as u32 as u64; rax = ((crate::simd_ops::ru8(r9.wrapping_add(0x1au64))) as u32) as u32 as u64; rcx = ((((rcx as u32)) | ((rax as u32)))) as u32 as u64; rax = ((crate::simd_ops::ru8(r9.wrapping_add(0x19u64))) as u32) as u32 as u64; rcx = (((rcx as u32)).wrapping_shl(8u32)) as u32 as u64; rcx = ((((rcx as u32)) | ((rax as u32)))) as u32 as u64; rax = ((crate::simd_ops::ru8(r9.wrapping_add(0x18u64))) as u32) as u32 as u64; rcx = (((rcx as u32)).wrapping_shl(8u32)) as u32 as u64; rcx = ((((rcx as u32)) | ((rax as u32)))) as u32 as u64; crate::simd_ops::wu32(r10.wrapping_add(0x28u64), ((rcx as u32)) as u32); rax = ((crate::simd_ops::ru8(r9.wrapping_add(0x1eu64))) as u32) as u32 as u64; rcx = ((crate::simd_ops::ru8(r9.wrapping_add(0x1fu64))) as u32) as u32 as u64; rcx = (((rcx as u32)).wrapping_shl(8u32)) as u32 as u64; rcx = ((((rcx as u32)) | ((rax as u32)))) as u32 as u64; rax = ((crate::simd_ops::ru8(r9.wrapping_add(0x1du64))) as u32) as u32 as u64; rcx = (((rcx as u32)).wrapping_shl(8u32)) as u32 as u64; rcx = ((((rcx as u32)) | ((rax as u32)))) as u32 as u64; rax = ((crate::simd_ops::ru8(r9.wrapping_add(0x1cu64))) as u32) as u32 as u64; rcx = (((rcx as u32)).wrapping_shl(8u32)) as u32 as u64; rcx = ((((rcx as u32)) | ((rax as u32)))) as u32 as u64; rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(r10.wrapping_add(0x2cu64), ((rcx as u32)) as u32); crate::simd_ops::wu32(r10.wrapping_add(0x38u64), ((r8 as u32)) as u32); fl_l = ((r8).wrapping_shr(32u32)) as u64; r8 = ((fl_l as u64)) as u64; crate::simd_ops::wu32(r10.wrapping_add(0x3cu64), ((r8 as u32)) as u32); crate::simd_ops::wu64(r10.wrapping_add(0x30u64), rax); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142544b80(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x3; const P1: u64 = 0x2; const P2: u64 = 0x1;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = ((crate::simd_ops::ru8(rcx.wrapping_add(rdx.wrapping_mul(4)).wrapping_add(P0))) as u32) as u32 as u64;
    r8 = ((rcx.wrapping_add(rdx.wrapping_mul(4))) as u64) as u64;
    rcx = ((crate::simd_ops::ru8(rcx.wrapping_add(rdx.wrapping_mul(4)).wrapping_add(P1))) as u32) as u32 as u64;
    rax = (((rax as u32)).wrapping_shl(8u32)) as u32 as u64;
    rax = ((((rax as u32)) | ((rcx as u32)))) as u32 as u64;
    rcx = ((crate::simd_ops::ru8(r8.wrapping_add(P2))) as u32) as u32 as u64;
    rax = (((rax as u32)).wrapping_shl(8u32)) as u32 as u64;
    rax = ((((rax as u32)) | ((rcx as u32)))) as u32 as u64;
    rcx = ((crate::simd_ops::ru8(r8)) as u32) as u32 as u64;
    rax = (((rax as u32)).wrapping_shl(8u32)) as u32 as u64;
    fl_l = ((((rax as u32)) | ((rcx as u32)))) as u32 as u64; fl_r = fl_l;
    rax = ((fl_l as u32)) as u32 as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142544bb0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x80; const P1: u64 = 0x142544c00;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rax = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64;
    rbx = rcx;
    fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64;
    if !((((((fl_l as u32)) & ((fl_r as u32))) as i32) > 0)) { rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rax = 0x10; crate::simd_ops::wu32(rbx.wrapping_add(P0), ((rax as u32)) as u32); }
    rcx = 0x20;
    rcx = ((rcx).wrapping_sub(rax)) as u64;
    rax = (crate::simd_ops::ru32(rbx.wrapping_add(rcx.wrapping_mul(4)))) as u32 as u64;
    crate::simd_ops::wu32(rbx.wrapping_add(rcx.wrapping_mul(4)), (0x0u32) as u32);
    crate::simd_ops::lk_sub_u32(rbx.wrapping_add(P0), (1) as u32);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142545060(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut r11, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rbx = ((rcx.wrapping_add(r8.wrapping_mul(4))) as u64) as u64; r8 = (crate::simd_ops::ru32(rcx.wrapping_add(rdx.wrapping_mul(4)))) as u32 as u64; r11 = ((rcx.wrapping_add(rdx.wrapping_mul(4))) as u64) as u64; r8 = (((r8 as u32)).wrapping_add(crate::simd_ops::ru32(rbx))) as u32 as u64; rdx = ((rcx.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; crate::simd_ops::wu32(r11, ((r8 as u32)) as u32); r10 = rcx; r8 = ((((r8 as u32)) ^ (crate::simd_ops::ru32(rdx)))) as u32 as u64; rcx = ((r8 as u32)) as u32 as u64; r8 = (((r8 as u32)).wrapping_shl(16u32)) as u32 as u64; rcx = (((rcx as u32)).wrapping_shr(16u32)) as u32 as u64; rcx = ((((rcx as u32)) | ((r8 as u32)))) as u32 as u64; crate::simd_ops::wu32(rdx, ((rcx as u32)) as u32); rcx = (((rcx as u32)).wrapping_add(crate::simd_ops::ru32(r10.wrapping_add(r9.wrapping_mul(4))))) as u32 as u64; crate::simd_ops::wu32(r10.wrapping_add(r9.wrapping_mul(4)), ((rcx as u32)) as u32); rcx = ((((rcx as u32)) ^ (crate::simd_ops::ru32(rbx)))) as u32 as u64; rax = ((rcx as u32)) as u32 as u64; rcx = (((rcx as u32)).wrapping_shl(12u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shr(20u32)) as u32 as u64; rax = ((((rax as u32)) | ((rcx as u32)))) as u32 as u64; crate::simd_ops::wu32(rbx, ((rax as u32)) as u32); crate::simd_ops::lk_add_u32(r11, ((rax as u32)) as u32); rcx = (crate::simd_ops::ru32(r11)) as u32 as u64; rcx = ((((rcx as u32)) ^ (crate::simd_ops::ru32(rdx)))) as u32 as u64; rax = ((rcx as u32)) as u32 as u64; rcx = (((rcx as u32)).wrapping_shl(8u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shr(24u32)) as u32 as u64; rax = ((((rax as u32)) | ((rcx as u32)))) as u32 as u64; crate::simd_ops::wu32(rdx, ((rax as u32)) as u32); crate::simd_ops::lk_add_u32(r10.wrapping_add(r9.wrapping_mul(4)), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(r10.wrapping_add(r9.wrapping_mul(4)))) as u32 as u64; rax = ((((rax as u32)) ^ (crate::simd_ops::ru32(rbx)))) as u32 as u64; rax = (((rax as u32)).rotate_right(25u32)) as u32 as u64; crate::simd_ops::wu32(rbx, ((rax as u32)) as u32); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x8u64))) as u64; return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425450e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = ((rcx as u32)) as u32 as u64;
    rcx = (((rdx as u8)) as u32) as u32 as u64;
    rax = (((rax as u32)).rotate_left(((rcx as u32) & 31))) as u32 as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142545160(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x84; const P1: u64 = 0x142544510;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(P0))) as u8 as u64; fl_r = (0x0u8) as u8 as u64;
    if !((((fl_l as u8)) == ((fl_r as u8)))) { rdx = 0; fl_l = 0; fl_r = 0; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142545180(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x20; const P2: u64 = 0x8; const P3: u64 = 0x1425452f0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x30u64)) as u64;
    rax = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    rbx = r8;
    r9 = (crate::simd_ops::ru64(rdx.wrapping_add(P1))) as u64;
    r8 = 0;
    fl_l = 0; fl_r = 0;
    rdi = rdx;
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax);
    rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp);
    fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64;
    if !(((((fl_l as u8)) & ((fl_r as u8))) != 0)) { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    fl_l = (crate::simd_ops::ru8(rbx)) as u8 as u64; fl_r = (0x0u8) as u8 as u64;
    if !((((fl_l as u8)) == ((fl_r as u8)))) { r8 = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64; rcx = rdi; rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P3, rax, rcx, rdx, r8, r9, rsp); }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64;
    rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64));
    rsp = ((rsp).wrapping_add(0x30u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425451f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1c; const P1: u64 = 0xc; const P2: u64 = 0xffffffffffffffff; const P3: u64 = 0xa; const P4: u64 = 0x18;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x60u64)) as u64; r10 = (crate::simd_ops::ru32(r8.wrapping_add(P0))) as u32 as u64; rax = 0x80000000; rdi = r9; rsi = rcx; fl_l = r10; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { r11 = ((r10 as u32)) as u32 as u64; r9 = ((r10 as u32)) as u32 as u64; bb = 3; continue; }
            2 => { r9 = (crate::simd_ops::ru32(r8)) as u32 as u64; r11 = (crate::simd_ops::ru32(r8)) as u32 as u64; r9 = ((r9).wrapping_shl(16u32)) as u64; fl_l = ((r11).wrapping_shl(16u32)) as u64; r11 = ((fl_l as u64)) as u64; bb = 3; continue; }
            3 => { rax = ((crate::simd_ops::ru16(r8.wrapping_add(P1))) as u32) as u32 as u64; rbx = ((r8.wrapping_add(P2)) as u64) as u64; rax = (((rax) as i64).wrapping_mul((r9) as i64) as u64) as u64; rbx = (((rbx) & (0xfffffffffe000000u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), r8); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); rcx = r8; rax = ((crate::simd_ops::ru16(r8.wrapping_add(P3))) as u32) as u32 as u64; rcx = ((rcx).wrapping_sub(rbx)) as u64; rax = (((rax) as i64).wrapping_mul((r9) as i64) as u64) as u64; rcx = ((rcx).wrapping_sub(0x108u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); rax = (0x6666666666666667u64) as u64; let prod = (rax as i64 as i128).wrapping_mul((rcx) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rdx = ((((rdx) as i64).wrapping_shr(5u32) as u64)) as u64; rax = rdx; rax = ((rax).wrapping_shr(63u32)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; rax = 0; fl_l = 0; fl_r = 0; fl_l = r10; fl_r = (0x8u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 7; continue; } bb = 4; continue; }
            4 => { fl_l = r10; fl_r = (0x40u64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = ((r10.wrapping_add(r10.wrapping_mul(2))) as u64) as u64; bb = 7; continue; }
            6 => { fl_l = r10; fl_r = (0x200u64) as u64; rax = (if (((fl_l as u64)) <= ((fl_r as u64))) { r10 } else { rax }) as u64; bb = 7; continue; }
            7 => { rdx = ((rdx).wrapping_shl(16u32)) as u64; rcx = rsi; rdx = ((rdx).wrapping_add(rax)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), r11); rax = (crate::simd_ops::ru32(r8.wrapping_add(P4))) as u32 as u64; rdx = ((rdx).wrapping_add(rbx)) as u64; r8 = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdx); rdx = rsp.wrapping_add(0x20); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), r9); rax = crate::simd_ops::call_stk(rdi, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rsp = ((rsp).wrapping_add(0x60u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425452f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142537450; const P1: u64 = 0x14183acc2;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedStack::<1>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rax = 0x2030; rsp = ((rsp).wrapping_sub(rax)) as u64; r13 = r8; r12 = rdx; r15 = rcx; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 23; continue; } bb = 1; continue; }
            1 => { r14 = (crate::simd_ops::ru64(rcx.wrapping_add(0x30u64))) as u64; fl_l = r14; fl_r = r14; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 23; continue; } bb = 2; continue; }
            2 => { rdx = (rdx & !0xffu64) | (((0x1u8) as u8 as u64)); rcx = r14; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru32(r14.wrapping_add(0x18u64))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 23; continue; } bb = 3; continue; }
            3 => { rsi = (crate::simd_ops::ru32(r14.wrapping_add(0x1cu64))) as u32 as u64; r8 = r14.wrapping_sub(0x1); r8 = (((r8) & (0xfffffffffe000000u64))) as u64; rax = 0x80000000; rcx = r14; rcx = ((rcx).wrapping_sub(r8)) as u64; rcx = ((rcx).wrapping_sub(0x108u64)) as u64; fl_l = rsi; fl_r = rax; rax = (0x6666666666666667u64) as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 9; continue; } bb = 4; continue; }
            4 => { let prod = (rax as i64 as i128).wrapping_mul((rcx) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rdi = 0; fl_l = 0; fl_r = 0; rbp = ((rsi as u32)) as u32 as u64; rbx = rdx; rbx = ((((rbx) as i64).wrapping_shr(5u32) as u64)) as u64; rax = rbx; rax = ((rax).wrapping_shr(63u32)) as u64; rbx = ((rbx).wrapping_add(rax)) as u64; rax = ((rdi as u32)) as u32 as u64; fl_l = rsi; fl_r = (0x8u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 10; continue; } bb = 5; continue; }
            5 => { rcx = ((rsi as u32)) as u32 as u64; fl_l = rsi; fl_r = (0x40u64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rax = ((rsi.wrapping_add(rsi.wrapping_mul(2))) as u64) as u64; bb = 10; continue; }
            7 => { fl_l = rcx; fl_r = (0x200u64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 10; continue; } bb = 8; continue; }
            8 => { rax = rcx; bb = 10; continue; }
            9 => { rbp = (crate::simd_ops::ru32(r14)) as u32 as u64; let prod = (rax as i64 as i128).wrapping_mul((rcx) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rsi = ((rbp as u32)) as u32 as u64; rbp = ((rbp).wrapping_shl(16u32)) as u64; rbx = rdx; rsi = ((rsi).wrapping_shl(16u32)) as u64; rbx = ((((rbx) as i64).wrapping_shr(5u32) as u64)) as u64; rax = rbx; rax = ((rax).wrapping_shr(63u32)) as u64; rbx = ((rbx).wrapping_add(rax)) as u64; rdi = 0; fl_l = 0; fl_r = 0; rax = ((rdi as u32)) as u32 as u64; bb = 10; continue; }
            10 => { rbx = ((rbx).wrapping_shl(16u32)) as u64; rbx = ((rbx).wrapping_add(rax)) as u64; rbx = ((rbx).wrapping_add(r8)) as u64; fl_l = (crate::simd_ops::ru16(r14.wrapping_add(0xau64))) as u16 as u64; fl_r = (0x1u16) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 12; continue; } bb = 11; continue; }
            11 => { rcx = (crate::simd_ops::ru64(r14.wrapping_add(0x30u64))) as u64; r9 = rbp; r8 = rbx; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r13); rdx = r15; rax = crate::simd_ops::call_stk(r12, rax, rcx, rdx, r8, r9, rsp); bb = 24; continue; }
            12 => { rdx = 0; fl_l = 0; fl_r = 0; rcx = rsp.wrapping_add(0x30); r8 = 0x2000; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); r8 = (crate::simd_ops::ru64(r14.wrapping_add(0x10u64))) as u64; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 15; continue; } bb = 13; continue; }
            13 => { bb = 14; continue; }
            14 => { rdx = 0; fl_l = 0; fl_r = 0; rax = r8; r8 = (crate::simd_ops::ru64(r8)) as u64; rax = ((rax).wrapping_sub(rbx)) as u64; let d = (rsi) as u128; let n = ((rdx as u128) << 64) | (rax as u128); if d != 0 { rax = (n / d) as u64; rdx = (n % d) as u64; } rdx = rsp.wrapping_add(0x30); rcx = rax; rcx = ((rcx).wrapping_shr(3u32)) as u64; rcx = ((rcx.wrapping_mul(8)) as u64) as u64; rdx = ((rdx).wrapping_add(rcx)) as u64; rax = ((rax).wrapping_sub(rcx)) as u64; rcx = (crate::simd_ops::ru64(rdx)) as u64; fl_l = (((rcx) >> (((rax) as u32 & 63))) & 1) as u64; rcx = ((rcx) | ((1u64) << (((rax) as u32 & 63)))) as u64; crate::simd_ops::wu64(rdx, rcx); fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 14; continue; } bb = 15; continue; }
            15 => { fl_l = ((rdi as u16)) as u16 as u64; fl_r = (crate::simd_ops::ru16(r14.wrapping_add(0xau64))) as u16 as u64; if (((fl_l as u16)) >= ((fl_r as u16))) { bb = 23; continue; } bb = 16; continue; }
            16 => { r12 = (((rsi) as i64).wrapping_mul((0x7u64) as i64) as u64) as u64; bb = 17; continue; }
            17 => { rax = rdi; rcx = rdi; rax = (((rax) & (0xfffffffffffffff8u64))) as u64; rdx = (crate::simd_ops::ru64(rsp.wrapping_add(rax).wrapping_add(0x30u64))) as u64; fl_l = rcx; fl_r = rax; rcx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 20; continue; } bb = 18; continue; }
            18 => { fl_l = rdx; fl_r = (0xffffffffffffffffu64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 20; continue; } bb = 19; continue; }
            19 => { rdi = ((rdi).wrapping_add(0x7u64)) as u64; fl_l = ((rbx).wrapping_add(r12)) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; bb = 22; continue; }
            20 => { fl_l = (((rdx) >> (((rcx) as u32 & 63))) & 1) as u64; if ((fl_l as u64) != 0) { bb = 22; continue; } bb = 21; continue; }
            21 => { rcx = (crate::simd_ops::ru64(r14.wrapping_add(0x30u64))) as u64; r9 = rbp; r8 = rbx; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r13); rdx = r15; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rsp.wrapping_add(0x2068u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 25; continue; } bb = 22; continue; }
            22 => { rax = ((crate::simd_ops::ru16(r14.wrapping_add(0xau64))) as u32) as u32 as u64; rdi = ((rdi).wrapping_add(1)) as u64; rbx = ((rbx).wrapping_add(rsi)) as u64; fl_l = rdi; fl_r = rax; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 17; continue; } bb = 23; continue; }
            23 => { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 24; continue; }
            24 => { r11 = rsp.wrapping_add(0x2030); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(r11.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x48u64))) as u64; rsp = r11; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            25 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 24; continue; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142545510(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1c; const P1: u64 = 0xffffffffffffffff; const P2: u64 = 0xa;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r10 = (crate::simd_ops::ru32(r8.wrapping_add(P0))) as u32 as u64; r11 = ((r8.wrapping_add(P1)) as u64) as u64; r11 = (((r11) & (0xfffffffffe000000u64))) as u64; rcx = r8; rcx = ((rcx).wrapping_sub(r11)) as u64; rax = (0x6666666666666667u64) as u64; rcx = ((rcx).wrapping_sub(0x108u64)) as u64; let prod = (rax as i64 as i128).wrapping_mul((rcx) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rdx = ((((rdx) as i64).wrapping_shr(5u32) as u64)) as u64; rax = rdx; rax = ((rax).wrapping_shr(63u32)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; fl_l = r10; fl_r = (0x8u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 5; continue; } bb = 1; continue; }
            1 => { fl_l = r10; fl_r = (0x40u64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rdx = ((rdx).wrapping_shl(16u32)) as u64; rax = ((r10.wrapping_add(r10.wrapping_mul(2))) as u64) as u64; rdx = ((rdx).wrapping_add(r11)) as u64; fl_l = ((rdx).wrapping_add(rax)) as u64; fl_r = fl_l; rdx = ((fl_l as u64)) as u64; bb = 7; continue; }
            3 => { fl_l = r10; fl_r = (0x200u64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rdx = ((rdx).wrapping_shl(16u32)) as u64; rdx = ((rdx).wrapping_add(r11)) as u64; fl_l = ((rdx).wrapping_add(r10)) as u64; fl_r = fl_l; rdx = ((fl_l as u64)) as u64; bb = 7; continue; }
            5 => { rdx = ((rdx).wrapping_shl(16u32)) as u64; rax = 0x80000000; rdx = ((rdx).wrapping_add(r11)) as u64; fl_l = r10; fl_r = rax; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 7; continue; } bb = 6; continue; }
            6 => { r10 = (crate::simd_ops::ru32(r8)) as u32 as u64; fl_l = ((r10).wrapping_shl(16u32)) as u64; r10 = ((fl_l as u64)) as u64; bb = 7; continue; }
            7 => { fl_l = r9; fl_r = rdx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 10; continue; } bb = 8; continue; }
            8 => { rax = ((crate::simd_ops::ru16(r8.wrapping_add(P2))) as u32) as u32 as u64; rax = (((rax) as i64).wrapping_mul((r10) as i64) as u64) as u64; rax = ((rax).wrapping_add(rdx)) as u64; fl_l = r9; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 10; continue; } bb = 9; continue; }
            9 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); crate::simd_ops::wu8(rcx, ((rax as u8)) as u8); rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; return rax; }
            10 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; crate::simd_ops::wu8(rcx, ((rax as u8)) as u8); fl_l = ((((rax as u8)) ^ (0x1u8))) as u8 as u64; fl_r = fl_l; rax = (rax & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425455d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142536370; const P1: u64 = 0x142536aa0; const P2: u64 = 0x142536420; const P3: u64 = 0x142545ac0;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 20; continue; } bb = 1; continue; }
            1 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x28u64)) as u64; fl_l = (crate::simd_ops::ru64(rdx.wrapping_add(0xbd0u64))) as u64; fl_r = (0x0u64) as u64; rbp = rdx; rdi = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 19; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rbx); rcx = rdx; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), r14); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r15); rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); r15 = rdi; rsi = rbp.wrapping_add(0x410); fl_l = r15; fl_r = rbp; r15 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; r14 = 0x4b; bb = 3; continue; }
            3 => { r9 = (crate::simd_ops::ru64(rsi)) as u64; r10 = ((r15.wrapping_add(rsi)) as u64) as u64; rbx = 0; fl_l = 0; fl_r = 0; fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 4; continue; }
            4 => { bb = 5; continue; }
            5 => { crate::simd_ops::wu64(r9.wrapping_add(0x30u64), rdi); bb = 6; continue; }
            6 => { rcx = 0; fl_l = 0; fl_r = 0; bb = 7; continue; }
            7 => { rdx = (crate::simd_ops::ru64(r9.wrapping_add(0x28u64))) as u64; r8 = rdx; rax = ((rdx as u32)) as u32 as u64; r8 = (((r8) & (0xfffffffffffffffcu64))) as u64; rax = ((((rax as u32)) & (0x3u32))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 10; continue; } bb = 8; continue; }
            8 => { fl_l = rcx; fl_r = (0x4u64) as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 6; continue; } bb = 9; continue; }
            9 => { fl_l = ((rcx).wrapping_add(1)) as u64; rcx = ((fl_l as u64)) as u64; bb = 7; continue; }
            10 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 13; continue; } bb = 11; continue; }
            11 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x3; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 13; continue; } bb = 12; continue; }
            12 => { rax = rdx; fl_l = rax; let cx_src: u64 = r8; let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u64(r9.wrapping_add(0x28u64), fl_l as u64, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u64)) as u64; } if (((fl_l as u64)) != ((fl_r as u64))) { bb = 7; continue; } bb = 13; continue; }
            13 => { r9 = (crate::simd_ops::ru64(r9.wrapping_add(0x38u64))) as u64; rbx = ((rbx).wrapping_add(1)) as u64; fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 14; continue; }
            14 => { rcx = (crate::simd_ops::ru64(r10.wrapping_add(0x8u64))) as u64; rax = (crate::simd_ops::ru64(rsi)) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 16; continue; } bb = 15; continue; }
            15 => { crate::simd_ops::wu64(r10, rax); rdx = r10; rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; rcx = rdi; crate::simd_ops::wu64(r10.wrapping_add(0x8u64), rax); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 17; continue; }
            16 => { crate::simd_ops::wu64(rcx.wrapping_add(0x38u64), rax); rax = (crate::simd_ops::ru64(r10.wrapping_add(0x8u64))) as u64; rcx = (crate::simd_ops::ru64(rsi)) as u64; crate::simd_ops::wu64(rcx.wrapping_add(0x40u64), rax); rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(r10.wrapping_add(0x8u64), rax); bb = 17; continue; }
            17 => { crate::simd_ops::lk_add_u64(rdi.wrapping_add(0xbd0u64), (rbx) as u64); rsi = ((rsi).wrapping_add(0x18u64)) as u64; crate::simd_ops::lk_sub_u64(rbp.wrapping_add(0xbd0u64), (rbx) as u64); fl_l = r14; fl_r = (0x1u64) as u64; r14 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 18; continue; }
            18 => { r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rcx = rbp; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = rbp; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 19; continue; }
            19 => { rsp = ((rsp).wrapping_add(0x28u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); bb = 20; continue; }
            20 => { return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142545730(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143073d80; const P1: u64 = 0xbf0; const P2: u64 = 0x142546510; const P3: u64 = 0x1425457f0; const P4: u64 = 0x10; const P5: u64 = 0x142534310; const P6: u64 = 0x18; const P7: u64 = 0xbe8; const P8: u64 = 0x1425338c0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 15; continue; } bb = 1; continue; }
            1 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = P0; rbx = rcx; fl_l = rcx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 14; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(P1))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P2, rax, rcx, rdx, r8, r9, rsp); }
            4 => { rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(P4))) as u64; fl_l = rcx; fl_r = rbx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 14; continue; } bb = 5; continue; }
            5 => { rax = ((crate::exe::tls_pointer() as u64)) as u64; r8 = 0x6348; rdx = (crate::simd_ops::ru64(rax)) as u64; fl_l = rbx; fl_r = (crate::simd_ops::ru64(r8.wrapping_add(rdx))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); bb = 7; continue; }
            7 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P6))) as u64; fl_l = rax; fl_r = rbx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 12; continue; } bb = 8; continue; }
            8 => { bb = 9; continue; }
            9 => { fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 10; continue; }
            10 => { rcx = rax; rax = (crate::simd_ops::ru64(rax.wrapping_add(P7))) as u64; fl_l = rax; fl_r = rbx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 9; continue; } bb = 11; continue; }
            11 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(P7))) as u64; crate::simd_ops::wu64(rcx.wrapping_add(P7), rax); rcx = rbx; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P8, rax, rcx, rdx, r8, r9, rsp); }
            12 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(P7))) as u64; crate::simd_ops::wu64(rcx.wrapping_add(P6), rax); bb = 13; continue; }
            13 => { rcx = rbx; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); bb = 14; continue; }
            14 => { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); bb = 15; continue; }
            15 => { return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425457f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14253b6b0; const P1: u64 = 0x14253b840; const P2: u64 = 0x14253b2a0; const P3: u64 = 0x142545ac0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; r14 = rcx; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 19; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(0xbd0u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 19; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rbp); rbp = 0x4b; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), r12); r12 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r15); r15 = rcx.wrapping_add(0x410); bb = 3; continue; }
            3 => { r9 = (crate::simd_ops::ru64(r15)) as u64; fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 4; continue; }
            4 => { bb = 5; continue; }
            5 => { rsi = (crate::simd_ops::ru64(r9.wrapping_add(0x38u64))) as u64; bb = 6; continue; }
            6 => { rcx = r12; bb = 7; continue; }
            7 => { rdx = (crate::simd_ops::ru64(r9.wrapping_add(0x28u64))) as u64; r8 = rdx; rax = ((rdx as u32)) as u32 as u64; r8 = (((r8) | (0x3u64))) as u64; rax = ((((rax as u32)) & (0x3u32))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 10; continue; } bb = 8; continue; }
            8 => { fl_l = rcx; fl_r = (0x4u64) as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 6; continue; } bb = 9; continue; }
            9 => { fl_l = ((rcx).wrapping_add(1)) as u64; rcx = ((fl_l as u64)) as u64; bb = 7; continue; }
            10 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x3; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 12; continue; } bb = 11; continue; }
            11 => { rax = rdx; fl_l = rax; let cx_src: u64 = r8; let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u64(r9.wrapping_add(0x28u64), fl_l as u64, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u64)) as u64; } if (((fl_l as u64)) != ((fl_r as u64))) { bb = 7; continue; } bb = 12; continue; }
            12 => { crate::simd_ops::wu32(r9.wrapping_add(0x18u64), ((r12 as u32)) as u32); rbx = r9.wrapping_sub(0x1); crate::simd_ops::wu64(r9.wrapping_add(0x38u64), r12); rcx = r9; crate::simd_ops::wu64(r9.wrapping_add(0x40u64), r12); rbx = (((rbx) & (0xfffffffffe000000u64))) as u64; rdi = (crate::simd_ops::ru64(r14)) as u64; rdi = ((rdi).wrapping_add(0x20u64)) as u64; rdx = rdi; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0xd0u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 14; continue; } bb = 13; continue; }
            13 => { r8 = rdi; rcx = rbx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 16; continue; }
            14 => { fl_l = rax; fl_r = (crate::simd_ops::ru64(rbx.wrapping_add(0xc0u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 16; continue; } bb = 15; continue; }
            15 => { rdx = rdi; rcx = rbx; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 16; continue; }
            16 => { r9 = rsi; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 17; continue; }
            17 => { r15 = ((r15).wrapping_add(0x18u64)) as u64; fl_l = rbp; fl_r = (0x1u64) as u64; rbp = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 18; continue; }
            18 => { r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; bb = 19; continue; }
            19 => { rcx = r14; rsp = ((rsp).wrapping_add(0x40u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P3, rax, rcx, rdx, r8, r9, rsp); }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142545950(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x28; const P1: u64 = 0x18; const P2: u64 = 0xffffffffffffffff; const P3: u64 = 0x38; const P4: u64 = 0x40; const P5: u64 = 0x14253b6b0; const P6: u64 = 0xd0; const P7: u64 = 0x14253b840; const P8: u64 = 0xc0; const P9: u64 = 0x14253b2a0;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; r11 = 0; fl_l = 0; fl_r = 0; bb = 1; continue; }
            1 => { rdx = r11; bb = 2; continue; }
            2 => { r9 = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64; r10 = r9; rax = ((r9 as u32)) as u32 as u64; r10 = (((r10) | (0x3u64))) as u64; rax = ((((rax as u32)) & (0x3u32))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 5; continue; } bb = 3; continue; }
            3 => { fl_l = rdx; fl_r = (0x4u64) as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 1; continue; } bb = 4; continue; }
            4 => { fl_l = ((rdx).wrapping_add(1)) as u64; rdx = ((fl_l as u64)) as u64; bb = 2; continue; }
            5 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x3; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rax = r9; fl_l = rax; let cx_src: u64 = r10; let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u64(r8.wrapping_add(P0), fl_l as u64, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u64)) as u64; } if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 7; continue; }
            7 => { crate::simd_ops::wu32(r8.wrapping_add(P1), ((r11 as u32)) as u32); rbx = ((r8.wrapping_add(P2)) as u64) as u64; crate::simd_ops::wu64(r8.wrapping_add(P3), r11); rbx = (((rbx) & (0xfffffffffe000000u64))) as u64; crate::simd_ops::wu64(r8.wrapping_add(P4), r11); rdi = (crate::simd_ops::ru64(rcx)) as u64; rcx = r8; rdi = ((rdi).wrapping_add(0x20u64)) as u64; rdx = rdi; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rbx.wrapping_add(P6))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { r8 = rdi; rcx = rbx; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            9 => { fl_l = rax; fl_r = (crate::simd_ops::ru64(rbx.wrapping_add(P8))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 11; continue; } bb = 10; continue; }
            10 => { rdx = rdi; rcx = rbx; rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); bb = 11; continue; }
            11 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142545a20(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143073d80; const P1: u64 = 0x10; const P2: u64 = 0x142534310; const P3: u64 = 0x18; const P4: u64 = 0xbe8; const P5: u64 = 0x1425338c0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 1; continue; }
            1 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = P0; rbx = rcx; fl_l = rcx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 11; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; fl_l = rcx; fl_r = rbx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 11; continue; } bb = 3; continue; }
            3 => { rax = ((crate::exe::tls_pointer() as u64)) as u64; r8 = 0x6348; rdx = (crate::simd_ops::ru64(rax)) as u64; fl_l = rbx; fl_r = (crate::simd_ops::ru64(r8.wrapping_add(rdx))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            5 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P3))) as u64; fl_l = rax; fl_r = rbx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 9; continue; } bb = 6; continue; }
            6 => { fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 7; continue; }
            7 => { rcx = rax; rax = (crate::simd_ops::ru64(rax.wrapping_add(P4))) as u64; fl_l = rax; fl_r = rbx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 6; continue; } bb = 8; continue; }
            8 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(P4))) as u64; crate::simd_ops::wu64(rcx.wrapping_add(P4), rax); rcx = rbx; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P5, rax, rcx, rdx, r8, r9, rsp); }
            9 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(P4))) as u64; crate::simd_ops::wu64(rcx.wrapping_add(P3), rax); bb = 10; continue; }
            10 => { rcx = rbx; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); bb = 11; continue; }
            11 => { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); bb = 12; continue; }
            12 => { return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142545ac0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14183acc2; const P1: u64 = 0x143a09580; const P2: u64 = 0x410; const P3: u64 = 0x143074190; const P4: u64 = 0x14183acbc; const P5: u64 = 0xbd0; const P6: u64 = 0xb18;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    rdx = 0;
    fl_l = 0; fl_r = 0;
    rcx = ((rcx).wrapping_add(0x8u64)) as u64;
    r8 = 0x408;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    fl_l = (crate::simd_ops::ru8(P1)) as u8 as u64; fl_r = (0x0u8) as u8 as u64;
    rcx = ((rbx.wrapping_add(P2)) as u64) as u64;
    if !((((fl_l as u8)) == ((fl_r as u8)))) { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rsi); rsi = P3; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rdi); rdi = rcx; rcx = 0x708; while rcx != 0 { crate::simd_ops::wu8(rdi, crate::simd_ops::ru8(rsi)); rdi = rdi.wrapping_add(1); rsi = rsi.wrapping_add(1); rcx = rcx.wrapping_sub(1); } rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; } else { rdx = P3; r8 = 0x708; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); }
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rbx.wrapping_add(P5), rax);
    crate::simd_ops::wu64(rbx.wrapping_add(P6), rax);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142545b40(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1425462a0; const P1: u64 = 0x10;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rax = (crate::simd_ops::ru64(rax)) as u64;
    rax = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64;
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142545b60(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = ((crate::exe::tls_pointer() as u64)) as u64;
    r8 = 0x6348;
    rdx = (crate::simd_ops::ru64(rax)) as u64;
    fl_l = rcx; fl_r = (crate::simd_ops::ru64(r8.wrapping_add(rdx))) as u64;
    rax = (rax & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64));
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142545b80(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143073d80; const P1: u64 = 0x142535600; const P2: u64 = 0x1438430a0; const P3: u64 = 0x14253b430; const P4: u64 = 0x14253add0; const P5: u64 = 0x142545e40; const P6: u64 = 0x142545f20; const P7: u64 = 0x142536420; const P8: u64 = 0x142535f70; const P9: u64 = 0x142537450; const P10: u64 = 0x142536070; const P11: u64 = 0x142536150; const P12: u64 = 0x14253b550; const P13: u64 = 0x14253b0b0; const P14: u64 = 0x14253c770; const P15: u64 = 0x143c7d0c0; const P16: u64 = 0x143c7d140; const P17: u64 = 0x143c4cfb0; const P18: u64 = 0x142539cc0; const P19: u64 = 0x1425427b0;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 45; continue; } bb = 1; continue; }
            1 => { r11 = rsp; rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x58u64)) as u64; rax = P0; rbp = ((rdx as u32)) as u32 as u64; r14 = rcx; fl_l = rcx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 44; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu64(r11.wrapping_add(0x8u64), rbx); fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x1; crate::simd_ops::wu64(r11.wrapping_add(0x10u64), rsi); crate::simd_ops::wu64(r11.wrapping_add(0x18u64), rdi); crate::simd_ops::wu64(r11.wrapping_sub(0x18u64), r15); r15 = (r15 & !0xffu64) | (((if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { 1u8 } else { 0u8 }) as u8 as u64)); rdx = (((r15 as u8)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rbp as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 11; continue; } bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru64(P2)) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = (crate::simd_ops::ru64(P2)) as u64; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 13; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(r14)) as u64; fl_l = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; fl_r = r14; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 13; continue; } bb = 6; continue; }
            6 => { fl_l = (crate::simd_ops::ru8(r14.wrapping_add(0xbf0u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 13; continue; } bb = 7; continue; }
            7 => { rbx = rax.wrapping_add(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 8; continue; }
            8 => { bb = 9; continue; }
            9 => { r9 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbx); r8 = 0; fl_l = 0; fl_r = 0; rdx = r14; rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 9; continue; } bb = 10; continue; }
            10 => { bb = 13; continue; }
            11 => { fl_l = ((rbp as u32)) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 13; continue; } bb = 12; continue; }
            12 => { r8 = 0; fl_l = 0; fl_r = 0; rdx = P5; rcx = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 13; continue; }
            13 => { rcx = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = (((r15 as u8)) as u32) as u32 as u64; rcx = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru64(r14.wrapping_add(0xbd0u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 24; continue; } bb = 14; continue; }
            14 => { crate::simd_ops::wu64(rsp.wrapping_add(0x88u64), r12); rsi = r14.wrapping_add(0x410); r12 = 0x4b; bb = 15; continue; }
            15 => { rbx = (crate::simd_ops::ru64(rsi)) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 22; continue; } bb = 16; continue; }
            16 => { bb = 17; continue; }
            17 => { rdi = (crate::simd_ops::ru64(rbx.wrapping_add(0x38u64))) as u64; rdx = (((r15 as u8)) as u32) as u32 as u64; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x18u64))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 19; continue; } bb = 18; continue; }
            18 => { rdx = rsi; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P10, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 21; continue; }
            19 => { fl_l = ((rbp as u32)) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 21; continue; } bb = 20; continue; }
            20 => { rdx = rsi; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P11, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 21; continue; }
            21 => { rbx = rdi; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 17; continue; } bb = 22; continue; }
            22 => { rsi = ((rsi).wrapping_add(0x18u64)) as u64; fl_l = r12; fl_r = (0x1u64) as u64; r12 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 15; continue; } bb = 23; continue; }
            23 => { r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x88u64))) as u64; bb = 24; continue; }
            24 => { rsi = (crate::simd_ops::ru64(r14)) as u64; rdi = 0x400; rsi = ((rsi).wrapping_add(0x20u64)) as u64; rax = 0x4000; fl_l = ((rbp as u32)) as u32 as u64; fl_r = 0x1; r15 = (r15 & !0xffu64) | (((if (((fl_l as u32)) == ((fl_r as u32))) { 1u8 } else { 0u8 }) as u8 as u64)); rdi = (if (((fl_l as u32)) == ((fl_r as u32))) { (rax as u32) } else { (rdi as u32) }) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 26; continue; } bb = 25; continue; }
            25 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P12, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 26; continue; }
            26 => { rdi = (((rdi as u32)).wrapping_sub(1)) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 33; continue; } bb = 27; continue; }
            27 => { r9 = rsi; r8 = 0; fl_l = 0; fl_r = 0; rdx = 0; fl_l = 0; fl_r = 0; rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P13, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(0xd0u64))) as u64; fl_r = (0x0u64) as u64; rcx = rbx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 29; continue; } bb = 28; continue; }
            28 => { r9 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); r8 = 0; fl_l = 0; fl_r = 0; rdx = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 32; continue; }
            29 => { r8 = (crate::simd_ops::ru64(rsi.wrapping_add(0x380u64))) as u64; rdx = (((r15 as u8)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P14, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(P15)) as u64; bb = 30; continue; }
            30 => { crate::simd_ops::wu64(rbx.wrapping_add(0xb0u64), rax); fl_l = rax; let cx_src: u64 = rbx; let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u64(P15, fl_l as u64, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u64)) as u64; } if (((fl_l as u64)) != ((fl_r as u64))) { bb = 30; continue; } bb = 31; continue; }
            31 => { fl_l = (crate::simd_ops::lk_add_u64(P16, (1) as u64)) as u64; bb = 32; continue; }
            32 => { fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) > 0) { bb = 26; continue; } bb = 33; continue; }
            33 => { r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; fl_l = ((rbp as u32)) as u32 as u64; fl_r = 0x1; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 43; continue; } bb = 34; continue; }
            34 => { fl_l = (crate::simd_ops::ru64(P2)) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 36; continue; } bb = 35; continue; }
            35 => { rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = (crate::simd_ops::ru64(P2)) as u64; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 43; continue; } bb = 36; continue; }
            36 => { rax = (crate::simd_ops::ru64(r14)) as u64; fl_l = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; fl_r = r14; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 43; continue; } bb = 37; continue; }
            37 => { rdi = P17; rsi = 0x10; rbx = rdi; bb = 38; continue; }
            38 => { fl_l = (crate::simd_ops::ru64(rdi)) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 41; continue; } bb = 39; continue; }
            39 => { rcx = 0; fl_l = 0; fl_r = 0; let tmp_xchg: u64 = crate::simd_ops::atomic_xchg_u64(rbx, rcx); rcx = (tmp_xchg) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 41; continue; } bb = 40; continue; }
            40 => { xmm0 = (crate::simd_ops::ru128(rcx.wrapping_add(0x1238u64))) as u128; xmm1 = ((crate::simd_ops::ru64(rcx.wrapping_add(0x1248u64))) as u64) as u128; r9 = rsp.wrapping_add(0x30); r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); rdx = 0x1250; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), ((xmm1 as u64)) as u64); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P18, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 41; continue; }
            41 => { rbx = ((rbx).wrapping_add(0x8u64)) as u64; rdi = ((rdi).wrapping_add(0x8u64)) as u64; fl_l = rsi; fl_r = (0x1u64) as u64; rsi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 38; continue; } bb = 42; continue; }
            42 => { r8 = (crate::simd_ops::ru64(r14)) as u64; rdx = (rdx & !0xffu64) | (((0x1u8) as u8 as u64)); r8 = ((r8).wrapping_add(0x3c0u64)) as u64; rcx = (((rdx as u8)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P19, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 43; continue; }
            43 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; bb = 44; continue; }
            44 => { rsp = ((rsp).wrapping_add(0x58u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); bb = 45; continue; }
            45 => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142545e40(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x28;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { bb = 1; continue; }
            1 => { rcx = 0; fl_l = 0; fl_r = 0; bb = 2; continue; }
            2 => { rdx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64; r9 = rdx; rax = ((rdx as u32)) as u32 as u64; r9 = (((r9) | (0x3u64))) as u64; rax = ((((rax as u32)) & (0x3u32))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 5; continue; } bb = 3; continue; }
            3 => { fl_l = rcx; fl_r = (0x4u64) as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 1; continue; } bb = 4; continue; }
            4 => { fl_l = ((rcx).wrapping_add(1)) as u64; rcx = ((fl_l as u64)) as u64; bb = 2; continue; }
            5 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x3; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rax = rdx; fl_l = rax; let cx_src: u64 = r9; let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u64(r8.wrapping_add(P0), fl_l as u64, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u64)) as u64; } if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 7; continue; }
            7 => { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142545ea0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142537450; const P1: u64 = 0x18; const P2: u64 = 0x142536070; const P3: u64 = 0x142536150;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = (crate::simd_ops::ru32(r9)) as u32 as u64; rdi = rdx; fl_l = ((rsi as u32)) as u32 as u64; fl_r = 0x1; rcx = r8; rbx = r8; rdx = (rdx & !0xffu64) | (((if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { 1u8 } else { 0u8 }) as u8 as u64)); rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(P1))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rdx = rdi; rcx = rbx; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            2 => { fl_l = ((rsi as u32)) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rdx = rdi; rcx = rbx; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142545f20(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xbd0; const P1: u64 = 0x410; const P2: u64 = 0x38;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; r14 = r8; r15 = rdx; rbp = rcx; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 9; continue; } bb = 2; continue; }
            2 => { r12 = 0; fl_l = 0; fl_r = 0; rdi = ((rcx.wrapping_add(P1)) as u64) as u64; rsi = ((r12 as u32)) as u32 as u64; bb = 3; continue; }
            3 => { r8 = (crate::simd_ops::ru64(rdi)) as u64; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 4; continue; }
            4 => { bb = 5; continue; }
            5 => { rbx = (crate::simd_ops::ru64(r8.wrapping_add(P2))) as u64; r9 = r14; rdx = rdi; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r12); rcx = rbp; rax = crate::simd_ops::call_stk(r15, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 9; continue; } bb = 6; continue; }
            6 => { r8 = rbx; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 7; continue; }
            7 => { rsi = ((rsi).wrapping_add(1)) as u64; rdi = ((rdi).wrapping_add(0x18u64)) as u64; fl_l = rsi; fl_r = (0x4au64) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 3; continue; } bb = 8; continue; }
            8 => { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 10; continue; }
            9 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 10; continue; }
            10 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425461f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x10;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    fl_l = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64; fl_r = rcx;
    rax = (rax & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64));
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142546220(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0xb28;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x28u64)) as u64; xmm0 = ((crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64) as u128; xmm1 = (crate::simd_ops::ru128(rdx)) as u128; rdx = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x10u64))) as u64; rax = ((rax).wrapping_shr(32u32)) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x6; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 6; continue; } bb = 1; continue; }
            1 => { rax = (((xmm1 >> 64) as u64)) as u64; rax = ((rax).wrapping_shr(32u32)) as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 5; continue; } bb = 3; continue; }
            3 => { rax = (((xmm1 >> 64) as u32) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x28u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            5 => { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); rsp = ((rsp).wrapping_add(0x28u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            6 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; rax = (rax & !0xffu64) | (((if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { 1u8 } else { 0u8 }) as u8 as u64)); rsp = ((rsp).wrapping_add(0x28u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425462a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142533dd0; const P1: u64 = 0x142534780; const P2: u64 = 0x143a096f8; const P3: u64 = 0x143a096f0; const P4: u64 = 0x143a096e0; const P5: u64 = 0x143867148;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x28u64)) as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 5; continue; } bb = 1; continue; }
            1 => { rcx = 0x1; let xa_src: u64 = rcx; let xa_old: u64 = crate::simd_ops::atomic_xadd_u64(P2, xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rcx = (xa_old) as u64; fl_l = ((rcx).wrapping_add(1)) as u64; rcx = ((fl_l as u64)) as u64; bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(P3)) as u64; fl_l = rax; fl_r = rcx; if ((((fl_l as u64)) as i64) >= (((fl_r as u64)) as i64)) { bb = 4; continue; } bb = 3; continue; }
            3 => { fl_l = rax; let cx_src: u64 = rcx; let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u64(P3, fl_l as u64, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u64)) as u64; } if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 4; continue; }
            4 => { crate::simd_ops::lk_add_u64(P4, (1) as u64); fl_l = (crate::simd_ops::lk_add_u64(P5, (1) as u64)) as u64; bb = 5; continue; }
            5 => { rcx = ((crate::exe::tls_pointer() as u64)) as u64; rdx = 0x6348; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = (crate::simd_ops::ru64(rdx.wrapping_add(rax))) as u64; rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142546310(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143073d80; const P1: u64 = 0x142535600; const P2: u64 = 0x142545e40; const P3: u64 = 0x142545f20; const P4: u64 = 0x142536420; const P5: u64 = 0x142535f70; const P6: u64 = 0x142537450; const P7: u64 = 0x142536070; const P8: u64 = 0x142536150; const P9: u64 = 0x14253b430; const P10: u64 = 0x14253b0b0; const P11: u64 = 0x14253add0; const P12: u64 = 0x14253c770; const P13: u64 = 0x143c7d0c0; const P14: u64 = 0x143c7d140; const P15: u64 = 0x1438430a0; const P16: u64 = 0x143c4cfb0; const P17: u64 = 0x142539cc0; const P18: u64 = 0x1425427b0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 31; continue; } bb = 1; continue; }
            1 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x50u64)) as u64; rax = P0; r14 = rcx; fl_l = rcx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 30; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rbx); rdx = (rdx & !0xffu64) | (((0x1u8) as u8 as u64)); crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), rdi); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = 0; fl_l = 0; fl_r = 0; rdx = P2; rcx = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = (rdx & !0xffu64) | (((0x1u8) as u8 as u64)); rcx = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru64(r14.wrapping_add(0xbd0u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 12; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), rbp); rsi = r14.wrapping_add(0x410); rbp = 0x4b; bb = 4; continue; }
            4 => { rbx = (crate::simd_ops::ru64(rsi)) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 5; continue; }
            5 => { bb = 6; continue; }
            6 => { rdi = (crate::simd_ops::ru64(rbx.wrapping_add(0x38u64))) as u64; rdx = (rdx & !0xffu64) | (((0x1u8) as u8 as u64)); rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x18u64))) as u32 as u64; fl_r = 0x0; rdx = rsi; rcx = rbx; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 8; continue; } bb = 7; continue; }
            7 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 9; continue; }
            8 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 9; continue; }
            9 => { rbx = rdi; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 6; continue; } bb = 10; continue; }
            10 => { rsi = ((rsi).wrapping_add(0x18u64)) as u64; fl_l = rbp; fl_r = (0x1u64) as u64; rbp = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } bb = 11; continue; }
            11 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; bb = 12; continue; }
            12 => { rsi = (crate::simd_ops::ru64(r14)) as u64; rdi = 0x400; fl_l = ((rsi).wrapping_add(0x20u64)) as u64; fl_r = fl_l; rsi = ((fl_l as u64)) as u64; bb = 13; continue; }
            13 => { rdi = (((rdi as u32)).wrapping_sub(1)) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 20; continue; } bb = 14; continue; }
            14 => { r9 = rsi; r8 = 0; fl_l = 0; fl_r = 0; rdx = 0; fl_l = 0; fl_r = 0; rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P10, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(0xd0u64))) as u64; fl_r = (0x0u64) as u64; rcx = rbx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 16; continue; } bb = 15; continue; }
            15 => { r9 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); r8 = 0; fl_l = 0; fl_r = 0; rdx = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P11, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 19; continue; }
            16 => { r8 = (crate::simd_ops::ru64(rsi.wrapping_add(0x380u64))) as u64; rdx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P12, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(P13)) as u64; bb = 17; continue; }
            17 => { crate::simd_ops::wu64(rbx.wrapping_add(0xb0u64), rax); fl_l = rax; let cx_src: u64 = rbx; let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u64(P13, fl_l as u64, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u64)) as u64; } if (((fl_l as u64)) != ((fl_r as u64))) { bb = 17; continue; } bb = 18; continue; }
            18 => { fl_l = (crate::simd_ops::lk_add_u64(P14, (1) as u64)) as u64; bb = 19; continue; }
            19 => { fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) > 0) { bb = 13; continue; } bb = 20; continue; }
            20 => { fl_l = (crate::simd_ops::ru64(P15)) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 22; continue; } bb = 21; continue; }
            21 => { rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = (crate::simd_ops::ru64(P15)) as u64; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 29; continue; } bb = 22; continue; }
            22 => { rax = (crate::simd_ops::ru64(r14)) as u64; fl_l = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; fl_r = r14; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 29; continue; } bb = 23; continue; }
            23 => { rdi = P16; rsi = 0x10; rbx = rdi; bb = 24; continue; }
            24 => { fl_l = (crate::simd_ops::ru64(rdi)) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 27; continue; } bb = 25; continue; }
            25 => { rcx = 0; fl_l = 0; fl_r = 0; let tmp_xchg: u64 = crate::simd_ops::atomic_xchg_u64(rbx, rcx); rcx = (tmp_xchg) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 27; continue; } bb = 26; continue; }
            26 => { xmm0 = (crate::simd_ops::ru128(rcx.wrapping_add(0x1238u64))) as u128; xmm1 = ((crate::simd_ops::ru64(rcx.wrapping_add(0x1248u64))) as u64) as u128; r9 = rsp.wrapping_add(0x30); r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); rdx = 0x1250; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), ((xmm1 as u64)) as u64); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P17, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 27; continue; }
            27 => { rbx = ((rbx).wrapping_add(0x8u64)) as u64; rdi = ((rdi).wrapping_add(0x8u64)) as u64; fl_l = rsi; fl_r = (0x1u64) as u64; rsi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 24; continue; } bb = 28; continue; }
            28 => { r8 = (crate::simd_ops::ru64(r14)) as u64; rdx = (rdx & !0xffu64) | (((0x1u8) as u8 as u64)); r8 = ((r8).wrapping_add(0x3c0u64)) as u64; rcx = (((rdx as u8)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P18, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 29; continue; }
            29 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; bb = 30; continue; }
            30 => { rsp = ((rsp).wrapping_add(0x50u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); bb = 31; continue; }
            31 => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142546510(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143073d80; const P1: u64 = 0x142536370; const P2: u64 = 0x142536aa0; const P3: u64 = 0x142536420; const P4: u64 = 0x142545ac0; const P5: u64 = 0x142545a20; const P6: u64 = 0x142546310;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 24; continue; } bb = 1; continue; }
            1 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rax = P0; r14 = rcx; fl_l = rcx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 23; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rdi); rdi = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; fl_l = rdi; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 21; continue; } bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(0xbd0u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 22; continue; } bb = 4; continue; }
            4 => { crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r15); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); r15 = rdi; rsi = r14.wrapping_add(0x410); fl_l = r15; fl_r = r14; r15 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; rbp = 0x4b; bb = 5; continue; }
            5 => { r9 = (crate::simd_ops::ru64(rsi)) as u64; r10 = ((r15.wrapping_add(rsi)) as u64) as u64; rbx = 0; fl_l = 0; fl_r = 0; fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 19; continue; } bb = 6; continue; }
            6 => { bb = 7; continue; }
            7 => { crate::simd_ops::wu64(r9.wrapping_add(0x30u64), rdi); bb = 8; continue; }
            8 => { rcx = 0; fl_l = 0; fl_r = 0; bb = 9; continue; }
            9 => { rdx = (crate::simd_ops::ru64(r9.wrapping_add(0x28u64))) as u64; r8 = rdx; rax = ((rdx as u32)) as u32 as u64; r8 = (((r8) & (0xfffffffffffffffcu64))) as u64; rax = ((((rax as u32)) & (0x3u32))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 12; continue; } bb = 10; continue; }
            10 => { fl_l = rcx; fl_r = (0x4u64) as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 8; continue; } bb = 11; continue; }
            11 => { fl_l = ((rcx).wrapping_add(1)) as u64; rcx = ((fl_l as u64)) as u64; bb = 9; continue; }
            12 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 15; continue; } bb = 13; continue; }
            13 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x3; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 15; continue; } bb = 14; continue; }
            14 => { rax = rdx; fl_l = rax; let cx_src: u64 = r8; let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u64(r9.wrapping_add(0x28u64), fl_l as u64, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u64)) as u64; } if (((fl_l as u64)) != ((fl_r as u64))) { bb = 9; continue; } bb = 15; continue; }
            15 => { r9 = (crate::simd_ops::ru64(r9.wrapping_add(0x38u64))) as u64; rbx = ((rbx).wrapping_add(1)) as u64; fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 7; continue; } bb = 16; continue; }
            16 => { rcx = (crate::simd_ops::ru64(r10.wrapping_add(0x8u64))) as u64; rax = (crate::simd_ops::ru64(rsi)) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 18; continue; } bb = 17; continue; }
            17 => { crate::simd_ops::wu64(r10, rax); rdx = r10; rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; rcx = rdi; crate::simd_ops::wu64(r10.wrapping_add(0x8u64), rax); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 19; continue; }
            18 => { crate::simd_ops::wu64(rcx.wrapping_add(0x38u64), rax); rax = (crate::simd_ops::ru64(r10.wrapping_add(0x8u64))) as u64; rcx = (crate::simd_ops::ru64(rsi)) as u64; crate::simd_ops::wu64(rcx.wrapping_add(0x40u64), rax); rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(r10.wrapping_add(0x8u64), rax); bb = 19; continue; }
            19 => { crate::simd_ops::lk_add_u64(rdi.wrapping_add(0xbd0u64), (rbx) as u64); rsi = ((rsi).wrapping_add(0x18u64)) as u64; crate::simd_ops::lk_sub_u64(r14.wrapping_add(0xbd0u64), (rbx) as u64); fl_l = rbp; fl_r = (0x1u64) as u64; rbp = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 5; continue; } bb = 20; continue; }
            20 => { r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rcx = r14; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = r14; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rcx = r14; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P5, rax, rcx, rdx, r8, r9, rsp); }
            21 => { rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); bb = 22; continue; }
            22 => { rcx = r14; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; bb = 23; continue; }
            23 => { rsp = ((rsp).wrapping_add(0x30u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); bb = 24; continue; }
            24 => { return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425466b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142545b80;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = ((crate::exe::tls_pointer() as u64)) as u64;
    rdx = 0x1;
    r8 = 0x6348;
    rcx = (crate::simd_ops::ru64(rax)) as u64;
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(rcx))) as u64;
    return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425466e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xbc8; const P1: u64 = 0xb48; const P2: u64 = 0x142544c00; const P3: u64 = 0x80; const P4: u64 = 0x40;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; rbx = ((rcx.wrapping_add(P1)) as u64) as u64; rdi = 0x10; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) > 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = rbx; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rax = ((rdi as u32)) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(P3), ((rdi as u32)) as u32); bb = 2; continue; }
            2 => { rcx = 0x20; rcx = ((rcx).wrapping_sub(rax)) as u64; rsi = (crate::simd_ops::ru32(rbx.wrapping_add(rcx.wrapping_mul(4)))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(rcx.wrapping_mul(4)), (0x0u32) as u32); crate::simd_ops::lk_sub_u32(rbx.wrapping_add(P3), (1) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_add(P3))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) > 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rcx = rbx; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rax = ((rdi as u32)) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(P3), ((rdi as u32)) as u32); bb = 4; continue; }
            4 => { rdi = (((rdi as u32)).wrapping_sub((rax as u32))) as u32 as u64; rax = ((((rdi as u32)) as i32 as i64) as u64) as u64; rcx = (crate::simd_ops::ru32(rbx.wrapping_add(rax.wrapping_mul(4)).wrapping_add(P4))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(rax.wrapping_mul(4)).wrapping_add(P4), (0x0u32) as u32); rax = rsi; crate::simd_ops::lk_sub_u32(rbx.wrapping_add(P3), (1) as u32); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rax = ((rax).wrapping_shl(32u32)) as u64; rax = (((rax) | (rcx))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142546780(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1425462a0; const P1: u64 = 0x10; const P2: u64 = 0x18; const P3: u64 = 0x143073d80; const P4: u64 = 0xbf0; const P5: u64 = 0xbe8; const P6: u64 = 0x1425457f0; const P7: u64 = 0x142534310; const P8: u64 = 0x1425338c0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rax)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; rcx = (crate::simd_ops::ru64(rax)) as u64; rbx = (crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 16; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rsi); rsi = P3; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rdi); rbp = 0x6348; bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(P4))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; rdi = (crate::simd_ops::ru64(rbx.wrapping_add(P5))) as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 13; continue; } bb = 3; continue; }
            3 => { fl_l = rbx; fl_r = rsi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 14; continue; } bb = 4; continue; }
            4 => { rcx = rbx; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rbx)) as u64; r8 = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; fl_l = r8; fl_r = rbx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 14; continue; } bb = 5; continue; }
            5 => { rax = ((crate::exe::tls_pointer() as u64)) as u64; rcx = (crate::simd_ops::ru64(rax)) as u64; fl_l = rbx; fl_r = (crate::simd_ops::ru64(rbp.wrapping_add(rcx))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rcx = r8; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); bb = 7; continue; }
            7 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64; fl_l = rax; fl_r = rbx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 11; continue; } bb = 8; continue; }
            8 => { fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 9; continue; }
            9 => { rcx = rax; rax = (crate::simd_ops::ru64(rax.wrapping_add(P5))) as u64; fl_l = rax; fl_r = rbx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 8; continue; } bb = 10; continue; }
            10 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(P5))) as u64; crate::simd_ops::wu64(rcx.wrapping_add(P5), rax); rcx = rbx; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); bb = 14; continue; }
            11 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(P5))) as u64; crate::simd_ops::wu64(rcx.wrapping_add(P2), rax); bb = 12; continue; }
            12 => { rcx = rbx; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); bb = 14; continue; }
            13 => { rcx = rbx; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); bb = 14; continue; }
            14 => { rbx = rdi; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 15; continue; }
            15 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; bb = 16; continue; }
            16 => { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142546890(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rdi); rdi = r9; r11 = r8; r9 = ((((r9 as u32)) & (0x3fu32))) as u32 as u64; rdi = ((rdi).wrapping_shr(6u32)) as u64; rsi = rcx; rax = ((r9.wrapping_add(r8)) as u64) as u64; fl_l = rax; fl_r = (0x40u64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 6; continue; } bb = 1; continue; }
            1 => { fl_l = r8; fl_r = (0x40u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(rdi.wrapping_mul(8)))) as u64; r8 = (0xffffffffffffffffu64) as u64; rax = (((rax) & (r8))) as u64; fl_l = rax; fl_r = r8; rax = (rax & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x8u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x10u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x18u64))) as u64; return rax; }
            3 => { fl_l = r11; fl_r = r11; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(rdi.wrapping_mul(8)))) as u64; r10 = 0; fl_l = 0; fl_r = 0; rax = (((rax) & (r10))) as u64; fl_l = rax; fl_r = r10; rax = (rax & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x8u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x10u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x18u64))) as u64; return rax; }
            5 => { rbx = 0x1; rax = ((rbx).wrapping_shl((((r11) as u32) & 63))) as u64; rax = ((rax).wrapping_sub(1)) as u64; r8 = ((rax).wrapping_shl((((r9) as u32) & 63))) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(rdi.wrapping_mul(8)))) as u64; rax = (((rax) & (r8))) as u64; fl_l = rax; fl_r = r8; rax = (rax & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x8u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x10u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x18u64))) as u64; return rax; }
            6 => { r10 = 0; fl_l = 0; fl_r = 0; rcx = 0x40; rcx = ((rcx).wrapping_sub(r9)) as u64; rbx = ((r10.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = rcx; fl_r = (0x40u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 8; continue; } bb = 7; continue; }
            7 => { r8 = r10.wrapping_sub(0x1); bb = 11; continue; }
            8 => { fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { r8 = r10; bb = 11; continue; }
            10 => { rax = ((rbx).wrapping_shl((((rcx) as u32) & 63))) as u64; fl_l = ((rax).wrapping_sub(1)) as u64; rax = ((fl_l as u64)) as u64; r8 = ((rax).wrapping_shl((((r9) as u32) & 63))) as u64; bb = 11; continue; }
            11 => { r11 = ((r11).wrapping_sub(rcx)) as u64; rdx = r11; rdx = ((rdx).wrapping_shr(6u32)) as u64; fl_l = ((((r11 as u32)) & (0x3fu32))) as u32 as u64; fl_r = fl_l; r11 = ((fl_l as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 13; continue; } bb = 12; continue; }
            12 => { r11 = r10; bb = 14; continue; }
            13 => { r11 = ((rbx).wrapping_shl((((r11) as u32) & 63))) as u64; fl_l = ((r11).wrapping_sub(1)) as u64; r11 = ((fl_l as u64)) as u64; bb = 14; continue; }
            14 => { rcx = (crate::simd_ops::ru64(rsi.wrapping_add(rdi.wrapping_mul(8)))) as u64; r9 = ((rsi.wrapping_add(rdi.wrapping_mul(8))) as u64) as u64; rax = r8; r9 = ((r9).wrapping_add(0x8u64)) as u64; rax = (((rax) & (rcx))) as u64; fl_l = rax; fl_r = r8; r8 = (r8 & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 15; continue; }
            15 => { rax = (((r8 as u8)) as u32) as u32 as u64; bb = 16; continue; }
            16 => { rcx = (crate::simd_ops::ru64(r9)) as u64; r8 = ((r10 as u32)) as u32 as u64; r9 = ((r9).wrapping_add(0x8u64)) as u64; rax = (((rax as u8)) as u32) as u32 as u64; fl_l = rcx; fl_r = (0xffffffffffffffffu64) as u64; r8 = (if (((fl_l as u64)) == ((fl_r as u64))) { (rax as u32) } else { (r8 as u32) }) as u32 as u64; rax = (((r8 as u8)) as u32) as u32 as u64; fl_l = rdx; fl_r = rbx; rdx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 16; continue; } bb = 17; continue; }
            17 => { fl_l = r11; fl_r = r11; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 19; continue; } bb = 18; continue; }
            18 => { rax = (((r8 as u8)) as u32) as u32 as u64; rdx = r11; rdx = (((rdx) & (crate::simd_ops::ru64(r9)))) as u64; fl_l = rdx; fl_r = r11; r10 = (if (((fl_l as u64)) == ((fl_r as u64))) { (rax as u32) } else { (r10 as u32) }) as u32 as u64; fl_l = rdx; fl_r = rdx; rcx = (((r10 as u8)) as u32) as u32 as u64; rax = ((rcx as u32)) as u32 as u64; rax = (if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { (rcx as u32) } else { (rax as u32) }) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x8u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x10u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x18u64))) as u64; return rax; }
            19 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x8u64))) as u64; rax = (((r8 as u8)) as u32) as u32 as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x10u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x18u64))) as u64; return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142546a20(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xffffffffffffffff; const P1: u64 = 0x2;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); r10 = rcx; rdx = 0; fl_l = 0; fl_r = 0; r10 = ((((r10 as u32)) & (0x3fu32))) as u32 as u64; rbx = r9; rax = ((r10.wrapping_add(r8)) as u64) as u64; fl_l = rax; fl_r = (0x40u64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 7; continue; } bb = 1; continue; }
            1 => { fl_l = r8; fl_r = (0x40u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rcx = ((rdx.wrapping_add(P0)) as u64) as u64; bb = 6; continue; }
            3 => { fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rcx = rdx; bb = 6; continue; }
            5 => { r11 = 0x1; rax = ((r11).wrapping_shl((((r8) as u32) & 63))) as u64; fl_l = ((rax).wrapping_sub(1)) as u64; rax = ((fl_l as u64)) as u64; rcx = ((rax).wrapping_shl((((r10) as u32) & 63))) as u64; bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; crate::simd_ops::wu64(r9, rcx); crate::simd_ops::wu64(rax, rdx); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu64(rax, rdx); rax = 0; fl_l = 0; fl_r = 0; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x8u64))) as u64; return rax; }
            7 => { r9 = 0x40; rcx = (0xffffffffffffffffu64) as u64; r9 = ((r9).wrapping_sub(r10)) as u64; r11 = ((rcx.wrapping_add(P1)) as u32) as u32 as u64; fl_l = r9; fl_r = (0x40u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 9; continue; } bb = 8; continue; }
            8 => { rax = rcx; bb = 12; continue; }
            9 => { fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { rax = rdx; bb = 12; continue; }
            11 => { rax = ((r11).wrapping_shl((((r9) as u32) & 63))) as u64; fl_l = ((rax).wrapping_sub(1)) as u64; rax = ((fl_l as u64)) as u64; rax = ((rax).wrapping_shl((((r10) as u32) & 63))) as u64; bb = 12; continue; }
            12 => { r8 = ((r8).wrapping_sub(r9)) as u64; crate::simd_ops::wu64(rbx, rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; r9 = r8; r9 = ((r9).wrapping_shr(6u32)) as u64; crate::simd_ops::wu64(rax, rcx); fl_l = ((((r8 as u32)) & (0x3fu32))) as u32 as u64; fl_r = fl_l; r8 = ((fl_l as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 14; continue; } bb = 13; continue; }
            13 => { rdx = ((r11).wrapping_shl((((r8) as u32) & 63))) as u64; fl_l = ((rdx).wrapping_sub(1)) as u64; rdx = ((fl_l as u64)) as u64; bb = 14; continue; }
            14 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = r9; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rcx, rdx); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142546af0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141484f90;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; rdi = ((rcx.wrapping_add(r8.wrapping_mul(8))) as u64) as u64; rbx = r9; r14 = r8; r15 = rdx; r12 = rcx; r13 = 0; fl_l = 0; fl_r = 0; bb = 1; continue; }
            1 => { r11 = (crate::simd_ops::ru64(rdi)) as u64; rsi = 0x1; r9 = rdi; rdx = rsi.wrapping_sub(0x2); fl_l = r11; fl_r = r11; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { r10 = ((rsi.wrapping_add(0x3fu64)) as u32) as u32 as u64; bb = 4; continue; }
            3 => { let bit_in: u64 = r11; if bit_in != 0 { rcx = (((63u32.wrapping_sub(bit_in.leading_zeros())) as u64)) as u64; } fl_l = bit_in as u64; r10 = 0x3f; crate::simd_ops::wu32(rsp.wrapping_add(0x60u64), ((rcx as u32)) as u32); fl_l = ((r10 as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; r10 = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 40; continue; } bb = 4; continue; }
            4 => { fl_l = r10; fl_r = rbx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 39; continue; } bb = 5; continue; }
            5 => { rcx = rbx; rax = r15; rcx = ((rcx).wrapping_sub(r10)) as u64; rax = ((rax).wrapping_sub(r14)) as u64; rcx = ((rcx).wrapping_add(0x3fu64)) as u64; rcx = ((rcx).wrapping_shr(6u32)) as u64; fl_l = rcx; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 40; continue; } bb = 6; continue; }
            6 => { rcx = r10; bb = 9; continue; }
            7 => { bb = 8; continue; }
            8 => { rdx = (0xffffffffffffffffu64) as u64; bb = 9; continue; }
            9 => { r8 = (crate::simd_ops::ru64(r9.wrapping_add(0x8u64))) as u64; rax = rcx.wrapping_add(0x40); r9 = ((r9).wrapping_add(0x8u64)) as u64; fl_l = rax; fl_r = rbx; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 11; continue; } bb = 10; continue; }
            10 => { rax = 0x40; bb = 15; continue; }
            11 => { rax = rbx; rax = ((rax).wrapping_sub(rcx)) as u64; fl_l = rax; fl_r = (0x40u64) as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 15; continue; } bb = 12; continue; }
            12 => { fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 14; continue; } bb = 13; continue; }
            13 => { rdx = r13; bb = 15; continue; }
            14 => { rdx = ((rsi).wrapping_shl((((rax) as u32) & 63))) as u64; fl_l = ((rdx).wrapping_sub(1)) as u64; rdx = ((fl_l as u64)) as u64; bb = 15; continue; }
            15 => { fl_l = r8; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 40; continue; } bb = 16; continue; }
            16 => { rcx = ((rcx).wrapping_add(rax)) as u64; fl_l = rcx; fl_r = rbx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 8; continue; } bb = 17; continue; }
            17 => { rsi = 0x40; rsi = ((rsi).wrapping_sub(r10)) as u64; fl_l = r10; fl_r = (0x40u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 19; continue; } bb = 18; continue; }
            18 => { r10 = (0xffffffffffffffffu64) as u64; bb = 20; continue; }
            19 => { rax = 0x1; rax = ((rax).wrapping_shl((((r10) as u32) & 63))) as u64; fl_l = ((rax).wrapping_sub(1)) as u64; rax = ((fl_l as u64)) as u64; r10 = ((rax).wrapping_shl((((rsi) as u32) & 63))) as u64; bb = 20; continue; }
            20 => { rcx = rdi; bb = 21; continue; }
            21 => { r8 = r10; r8 = (((r8) | (r11))) as u64; fl_l = r11; fl_r = r10; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 30; continue; } bb = 22; continue; }
            22 => { rax = r11; fl_l = rax; let cx_src: u64 = r8; let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u64(rdi, fl_l as u64, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u64)) as u64; } r11 = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 21; continue; } bb = 23; continue; }
            23 => { rcx = rdi.wrapping_add(0x8); fl_l = rcx; fl_r = r9; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 27; continue; } bb = 24; continue; }
            24 => { r8 = (0xffffffffffffffffu64) as u64; bb = 25; continue; }
            25 => { rax = 0; fl_l = 0; fl_r = 0; fl_l = rax; let cx_src: u64 = r8; let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u64(rcx, fl_l as u64, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u64)) as u64; } if (((fl_l as u64)) != ((fl_r as u64))) { bb = 30; continue; } bb = 26; continue; }
            26 => { rcx = ((rcx).wrapping_add(0x8u64)) as u64; fl_l = rcx; fl_r = r9; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 25; continue; } bb = 27; continue; }
            27 => { rax = (crate::simd_ops::ru64(rcx)) as u64; r8 = rdx; r8 = (((r8) | (rax))) as u64; fl_l = rax; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 30; continue; } bb = 28; continue; }
            28 => { fl_l = rax; let cx_src: u64 = r8; let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u64(rcx, fl_l as u64, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u64)) as u64; } if (((fl_l as u64)) == ((fl_r as u64))) { bb = 38; continue; } bb = 29; continue; }
            29 => { r8 = rax; r8 = (((r8) | (rdx))) as u64; fl_l = rdx; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 28; continue; } bb = 30; continue; }
            30 => { rcx = ((rcx).wrapping_sub(0x8u64)) as u64; fl_l = rcx; fl_r = rdi; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 33; continue; } bb = 31; continue; }
            31 => { bb = 32; continue; }
            32 => { crate::simd_ops::wu64(rcx, r13); rcx = ((rcx).wrapping_sub(0x8u64)) as u64; fl_l = rcx; fl_r = rdi; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 32; continue; } bb = 33; continue; }
            33 => { if (((fl_l as u64)) != ((fl_r as u64))) { bb = 36; continue; } bb = 34; continue; }
            34 => { rax = (crate::simd_ops::ru64(rcx)) as u64; r10 = (!(r10)) as u64; bb = 35; continue; }
            35 => { rdx = r10; rdx = (((rdx) & (rax))) as u64; fl_l = rax; let cx_src: u64 = rdx; let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u64(rcx, fl_l as u64, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u64)) as u64; } if (((fl_l as u64)) != ((fl_r as u64))) { bb = 35; continue; } bb = 36; continue; }
            36 => { fl_l = rbp; fl_r = (0x2u64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 40; continue; } bb = 37; continue; }
            37 => { fl_l = ((rbp).wrapping_add(1)) as u64; rbp = ((fl_l as u64)) as u64; bb = 1; continue; }
            38 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x88u64))) as u64; r14 = ((r14).wrapping_shl(6u32)) as u64; fl_l = ((r14).wrapping_add(rsi)) as u64; fl_r = fl_l; r14 = ((fl_l as u64)) as u64; crate::simd_ops::wu64(rax, r14); rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 41; continue; }
            39 => { r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0x88u64))) as u64; r8 = rbx; rdx = r14; rcx = r12; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); bb = 41; continue; }
            40 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 41; continue; }
            41 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142546d00(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 3>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), r8); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rax = 0; fl_l = 0; fl_r = 0; rdi = 0; fl_l = 0; fl_r = 0; r10 = r9; rsi = rdx; r14 = rcx; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 21; continue; } bb = 1; continue; }
            1 => { rbp = ((rax.wrapping_add(0x1u64)) as u32) as u32 as u64; bb = 2; continue; }
            2 => { rbx = 0; fl_l = 0; fl_r = 0; fl_l = rax; fl_r = rsi; rbx = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { rbx }) as u64; r9 = (crate::simd_ops::ru64(r14.wrapping_add(rbx.wrapping_mul(8)))) as u64; fl_l = r9; fl_r = (0xffffffffffffffffu64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 20; continue; } bb = 3; continue; }
            3 => { fl_l = r10; fl_r = (0x40u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rcx = (0xffffffffffffffffu64) as u64; bb = 8; continue; }
            5 => { fl_l = r10; fl_r = r10; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { rcx = 0; fl_l = 0; fl_r = 0; bb = 8; continue; }
            7 => { rcx = ((rbp).wrapping_shl((((r10) as u32) & 63))) as u64; fl_l = ((rcx).wrapping_sub(1)) as u64; rcx = ((fl_l as u64)) as u64; bb = 8; continue; }
            8 => { r11 = 0x40; rax = r9; r11 = ((r11).wrapping_sub(r10)) as u64; rax = (!(rax)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { rdx = ((rax.wrapping_add(0x40u64)) as u32) as u32 as u64; bb = 11; continue; }
            10 => { let bit_in: u64 = rax; if bit_in != 0 { rax = ((bit_in.trailing_zeros() as u64)) as u64; } fl_l = bit_in as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((rax as u32)) as u32); rdx = ((rax as u32)) as u32 as u64; bb = 11; continue; }
            11 => { r8 = ((rcx).wrapping_shl((((rdx) as u32) & 63))) as u64; fl_l = rdx; fl_r = r11; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 20; continue; } bb = 12; continue; }
            12 => { rcx = r8; fl_l = (((rcx) & (r9))) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 15; continue; } bb = 13; continue; }
            13 => { rcx = r8; rax = r9; rcx = (((rcx) | (r9))) as u64; fl_l = rax; let cx_src: u64 = rcx; let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u64(r14.wrapping_add(rbx.wrapping_mul(8)), fl_l as u64, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u64)) as u64; } r9 = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 19; continue; } bb = 14; continue; }
            14 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbx = ((rbx).wrapping_shl(6u32)) as u64; fl_l = ((rbx).wrapping_add(rdx)) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; crate::simd_ops::wu64(rax, rbx); rax = (((rbp as u8)) as u32) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            15 => { fl_l = r10; fl_r = rbp; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 17; continue; } bb = 16; continue; }
            16 => { rax = rbp; bb = 18; continue; }
            17 => { let bit_in: u64 = rcx; if bit_in != 0 { rcx = (((63u32.wrapping_sub(bit_in.leading_zeros())) as u64)) as u64; } fl_l = bit_in as u64; rax = 0x3f; rax = (((rax as u32)).wrapping_sub((rcx as u32))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rcx as u32)) as u32); rcx = ((rax as u32)) as u32 as u64; rax = 0x40; rax = ((rax).wrapping_sub(rcx)) as u64; fl_l = rax; fl_r = rdx; rax = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; bb = 18; continue; }
            18 => { fl_l = ((rdx).wrapping_add(rax)) as u64; fl_r = fl_l; rdx = ((fl_l as u64)) as u64; r8 = ((r8).wrapping_shl((((rax) as u32) & 63))) as u64; bb = 19; continue; }
            19 => { fl_l = rdx; fl_r = r11; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 12; continue; } bb = 20; continue; }
            20 => { rdi = ((rdi).wrapping_add(1)) as u64; rax = rbx.wrapping_add(0x1); fl_l = rdi; fl_r = rsi; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 2; continue; } bb = 21; continue; }
            21 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142546e30(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = rcx; fl_r = (0x40u64) as u64;
    if !((((fl_l as u64)) < ((fl_r as u64)))) { rax = (0xffffffffffffffffu64) as u64; return rax; }
    fl_l = rcx; fl_r = rcx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) != 0)) { rax = 0; fl_l = 0; fl_r = 0; return rax; }
    rax = 0x1;
    rax = ((rax).wrapping_shl((((rcx) as u32) & 63))) as u64;
    fl_l = ((rax).wrapping_sub(1)) as u64;
    rax = ((fl_l as u64)) as u64;
    rax = ((rax).wrapping_shl((((rdx) as u32) & 63))) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142546e60(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = ((((rcx as u32)) & (0x3fu32))) as u32 as u64; fl_r = fl_l;
    rcx = ((fl_l as u32)) as u32 as u64;
    rax = ((rcx as u32)) as u32 as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142546e90(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = 0x40;
    fl_l = rcx; fl_r = rcx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { let bit_in: u64 = rcx; if bit_in != 0 { rax = ((bit_in.trailing_zeros() as u64)) as u64; } fl_l = bit_in as u64; }
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142546ed0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x3f;
    let (mut rax, mut fl_l) = (crate::simd_ops::get_rax_in(), 0u64);
    rax = ((rcx.wrapping_add(P0)) as u64) as u64;
    fl_l = ((rax).wrapping_shr(6u32)) as u64;
    rax = ((fl_l as u64)) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142546f20(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    loop { r8 = rax; r8 = (((r8) | (rdx))) as u64; fl_l = rax; let cx_src: u64 = r8; let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u64(rcx, fl_l as u64, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u64)) as u64; } if !((((fl_l as u64)) != ((fl_r as u64)))) { break; } }
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142546f50(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    loop { r8 = rax; r8 = (((r8) & (rdx))) as u64; fl_l = rax; let cx_src: u64 = r8; let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u64(rcx, fl_l as u64, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u64)) as u64; } if !((((fl_l as u64)) != ((fl_r as u64)))) { break; } }
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142546f80(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 1>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); r11 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsi = r9; r9 = ((((r9 as u32)) & (0x3fu32))) as u32 as u64; rsi = ((rsi).wrapping_shr(6u32)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r14); r14 = rcx; rax = ((r9.wrapping_add(r8)) as u64) as u64; fl_l = rax; fl_r = (0x40u64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 6; continue; } bb = 1; continue; }
            1 => { fl_l = r8; fl_r = (0x40u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 3; continue; } bb = 2; continue; }
            2 => { r9 = r11.wrapping_sub(0x1); rbx = ((r11 as u32)) as u32 as u64; rdi = ((r11 as u32)) as u32 as u64; r10 = ((r11 as u32)) as u32 as u64; bb = 14; continue; }
            3 => { fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { r9 = r11; rbx = r11; rdi = r11; r10 = r11; bb = 14; continue; }
            5 => { rdx = 0x1; rbx = r11; rax = ((rdx).wrapping_shl((((r8) as u32) & 63))) as u64; fl_l = ((rax).wrapping_sub(1)) as u64; rax = ((fl_l as u64)) as u64; rdi = r11; r9 = ((rax).wrapping_shl((((r9) as u32) & 63))) as u64; r10 = r11; bb = 14; continue; }
            6 => { rcx = 0x40; rdx = 0x1; rcx = ((rcx).wrapping_sub(r9)) as u64; rbx = rdx.wrapping_sub(0x2); fl_l = rcx; fl_r = (0x40u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 8; continue; } bb = 7; continue; }
            7 => { r9 = rbx; bb = 11; continue; }
            8 => { fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { r9 = r11; bb = 11; continue; }
            10 => { rax = ((rdx).wrapping_shl((((rcx) as u32) & 63))) as u64; fl_l = ((rax).wrapping_sub(1)) as u64; rax = ((fl_l as u64)) as u64; r9 = ((rax).wrapping_shl((((r9) as u32) & 63))) as u64; bb = 11; continue; }
            11 => { r8 = ((r8).wrapping_sub(rcx)) as u64; r10 = r8; r10 = ((r10).wrapping_shr(6u32)) as u64; fl_l = ((((r8 as u32)) & (0x3fu32))) as u32 as u64; fl_r = fl_l; r8 = ((fl_l as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 13; continue; } bb = 12; continue; }
            12 => { rdi = r11; bb = 14; continue; }
            13 => { rdi = ((rdx).wrapping_shl((((r8) as u32) & 63))) as u64; fl_l = ((rdi).wrapping_sub(1)) as u64; rdi = ((fl_l as u64)) as u64; bb = 14; continue; }
            14 => { r8 = r9; rcx = ((r14.wrapping_add(rsi.wrapping_mul(8))) as u64) as u64; r8 = (!(r8)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; bb = 15; continue; }
            15 => { rdx = rax; rdx = (((rdx) & (r8))) as u64; fl_l = rax; let cx_src: u64 = rdx; let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u64(rcx, fl_l as u64, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u64)) as u64; } if (((fl_l as u64)) != ((fl_r as u64))) { bb = 15; continue; } bb = 16; continue; }
            16 => { rax = (((rax) & (r9))) as u64; rcx = ((rcx).wrapping_add(0x8u64)) as u64; fl_l = rax; fl_r = r9; r9 = (r9 & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); fl_l = r10; fl_r = r10; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 21; continue; } bb = 17; continue; }
            17 => { r8 = rbx; rsi = (((r9 as u8)) as u32) as u32 as u64; r8 = (!(r8)) as u64; bb = 18; continue; }
            18 => { fl_l = ((r10).wrapping_sub(1)) as u64; r10 = ((fl_l as u64)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; bb = 19; continue; }
            19 => { rdx = rax; rdx = (((rdx) & (r8))) as u64; fl_l = rax; let cx_src: u64 = rdx; let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u64(rcx, fl_l as u64, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u64)) as u64; } if (((fl_l as u64)) != ((fl_r as u64))) { bb = 19; continue; } bb = 20; continue; }
            20 => { rcx = ((rcx).wrapping_add(0x8u64)) as u64; rdx = (((rsi as u8)) as u32) as u32 as u64; rax = (((rax) & (rbx))) as u64; r9 = ((r11 as u32)) as u32 as u64; fl_l = rax; fl_r = rbx; r9 = (if (((fl_l as u64)) == ((fl_r as u64))) { (rdx as u32) } else { (r9 as u32) }) as u32 as u64; rsi = (((r9 as u8)) as u32) as u32 as u64; fl_l = r10; fl_r = r10; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 18; continue; } bb = 21; continue; }
            21 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x18u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x10u64))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 25; continue; } bb = 22; continue; }
            22 => { r8 = rdi; r8 = (!(r8)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; bb = 23; continue; }
            23 => { rdx = rax; rdx = (((rdx) & (r8))) as u64; fl_l = rax; let cx_src: u64 = rdx; let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u64(rcx, fl_l as u64, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u64)) as u64; } if (((fl_l as u64)) != ((fl_r as u64))) { bb = 23; continue; } bb = 24; continue; }
            24 => { rax = (((rax) & (rdi))) as u64; rcx = (((r9 as u8)) as u32) as u32 as u64; fl_l = rax; fl_r = rdi; rcx = (if (((fl_l as u64)) != ((fl_r as u64))) { (r11 as u32) } else { (rcx as u32) }) as u32 as u64; rax = (((rcx as u8)) as u32) as u32 as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            25 => { rax = (((r9 as u8)) as u32) as u32 as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142547120(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r10 = r9; r11 = rcx; r10 = ((r10).wrapping_shr(6u32)) as u64; fl_l = r8; fl_r = (0x40u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { r8 = (0xffffffffffffffffu64) as u64; bb = 4; continue; }
            2 => { fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = 0x1; rax = ((rax).wrapping_shl((((r8) as u32) & 63))) as u64; rax = ((rax).wrapping_sub(1)) as u64; fl_l = ((((r9 as u32)) & (0x3fu32))) as u32 as u64; fl_r = fl_l; r9 = ((fl_l as u32)) as u32 as u64; r8 = ((rax).wrapping_shl((((r9) as u32) & 63))) as u64; bb = 4; continue; }
            4 => { rdx = r8; rdx = (!(rdx)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(r10.wrapping_mul(8)))) as u64; bb = 5; continue; }
            5 => { rcx = rax; rcx = (((rcx) & (rdx))) as u64; fl_l = rax; let cx_src: u64 = rcx; let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u64(r11.wrapping_add(r10.wrapping_mul(8)), fl_l as u64, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u64)) as u64; } if (((fl_l as u64)) != ((fl_r as u64))) { bb = 5; continue; } bb = 6; continue; }
            6 => { rax = (((rax) & (r8))) as u64; fl_l = rax; fl_r = r8; rax = (rax & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142547190(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r10 = r9; r11 = rcx; r10 = ((r10).wrapping_shr(6u32)) as u64; fl_l = r8; fl_r = (0x40u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rdx = (0xffffffffffffffffu64) as u64; bb = 5; continue; }
            2 => { fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rdx = 0; fl_l = 0; fl_r = 0; bb = 5; continue; }
            4 => { rax = 0x1; rax = ((rax).wrapping_shl((((r8) as u32) & 63))) as u64; rax = ((rax).wrapping_sub(1)) as u64; fl_l = ((((r9 as u32)) & (0x3fu32))) as u32 as u64; fl_r = fl_l; r9 = ((fl_l as u32)) as u32 as u64; rdx = ((rax).wrapping_shl((((r9) as u32) & 63))) as u64; bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(r10.wrapping_mul(8)))) as u64; bb = 6; continue; }
            6 => { rcx = rax; rcx = (((rcx) | (rdx))) as u64; fl_l = rax; let cx_src: u64 = rcx; let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u64(r11.wrapping_add(r10.wrapping_mul(8)), fl_l as u64, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u64)) as u64; } if (((fl_l as u64)) != ((fl_r as u64))) { bb = 6; continue; } bb = 7; continue; }
            7 => { fl_l = rdx; fl_r = rax; rax = (rax & !0xffu64) | (((if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { 1u8 } else { 0u8 }) as u8 as u64)); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425471f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r10 = r9; r11 = rcx; r10 = ((r10).wrapping_shr(6u32)) as u64; fl_l = r8; fl_r = (0x40u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rdx = (0xffffffffffffffffu64) as u64; bb = 5; continue; }
            2 => { fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rdx = 0; fl_l = 0; fl_r = 0; bb = 5; continue; }
            4 => { rax = 0x1; rax = ((rax).wrapping_shl((((r8) as u32) & 63))) as u64; rax = ((rax).wrapping_sub(1)) as u64; fl_l = ((((r9 as u32)) & (0x3fu32))) as u32 as u64; fl_r = fl_l; r9 = ((fl_l as u32)) as u32 as u64; rdx = ((rax).wrapping_shl((((r9) as u32) & 63))) as u64; bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(r10.wrapping_mul(8)))) as u64; bb = 6; continue; }
            6 => { fl_l = rdx; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 9; continue; } bb = 7; continue; }
            7 => { rcx = rax; rcx = (((rcx) | (rdx))) as u64; fl_l = rax; let cx_src: u64 = rcx; let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u64(r11.wrapping_add(r10.wrapping_mul(8)), fl_l as u64, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u64)) as u64; } if (((fl_l as u64)) != ((fl_r as u64))) { bb = 6; continue; } bb = 8; continue; }
            8 => { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); return rax; }
            9 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142547250(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142546af0;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), r8); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rax = 0; fl_l = 0; fl_r = 0; rdi = r9; rbp = rdx; r13 = rcx; fl_l = r9; fl_r = (0x2u64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 22; continue; } bb = 1; continue; }
            1 => { rbx = 0; fl_l = 0; fl_r = 0; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 21; continue; } bb = 2; continue; }
            2 => { r15 = ((rax.wrapping_add(0x1u64)) as u32) as u32 as u64; bb = 3; continue; }
            3 => { r11 = 0; fl_l = 0; fl_r = 0; fl_l = rax; fl_r = rbp; r11 = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { r11 }) as u64; r9 = (crate::simd_ops::ru64(r13.wrapping_add(r11.wrapping_mul(8)))) as u64; fl_l = r9; fl_r = (0xffffffffffffffffu64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 20; continue; } bb = 4; continue; }
            4 => { fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rcx = 0; fl_l = 0; fl_r = 0; bb = 7; continue; }
            6 => { rcx = ((r15).wrapping_shl((((rdi) as u32) & 63))) as u64; fl_l = ((rcx).wrapping_sub(1)) as u64; rcx = ((fl_l as u64)) as u64; bb = 7; continue; }
            7 => { r10 = 0x40; rax = r9; r10 = ((r10).wrapping_sub(rdi)) as u64; rax = (!(rax)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { rdx = ((rax.wrapping_add(0x40u64)) as u32) as u32 as u64; bb = 10; continue; }
            9 => { let bit_in: u64 = rax; if bit_in != 0 { rax = ((bit_in.trailing_zeros() as u64)) as u64; } fl_l = bit_in as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x80u64), ((rax as u32)) as u32); rdx = ((rax as u32)) as u32 as u64; bb = 10; continue; }
            10 => { r8 = ((rcx).wrapping_shl((((rdx) as u32) & 63))) as u64; fl_l = rdx; fl_r = r10; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 20; continue; } bb = 11; continue; }
            11 => { bb = 12; continue; }
            12 => { rcx = r8; fl_l = (((rcx) & (r9))) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 15; continue; } bb = 13; continue; }
            13 => { rcx = r8; rax = r9; rcx = (((rcx) | (r9))) as u64; fl_l = rax; let cx_src: u64 = rcx; let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u64(r13.wrapping_add(r11.wrapping_mul(8)), fl_l as u64, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u64)) as u64; } r9 = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 19; continue; } bb = 14; continue; }
            14 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; r11 = ((r11).wrapping_shl(6u32)) as u64; r11 = ((r11).wrapping_add(rdx)) as u64; crate::simd_ops::wu64(rax, r11); rax = (((r15 as u8)) as u32) as u32 as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            15 => { fl_l = rdi; fl_r = r15; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 17; continue; } bb = 16; continue; }
            16 => { rax = r15; bb = 18; continue; }
            17 => { let bit_in: u64 = rcx; if bit_in != 0 { rcx = (((63u32.wrapping_sub(bit_in.leading_zeros())) as u64)) as u64; } fl_l = bit_in as u64; rax = 0x3f; rax = (((rax as u32)).wrapping_sub((rcx as u32))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x88u64), ((rcx as u32)) as u32); rcx = ((rax as u32)) as u32 as u64; rax = 0x40; rax = ((rax).wrapping_sub(rcx)) as u64; fl_l = rax; fl_r = rdx; rax = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; bb = 18; continue; }
            18 => { fl_l = ((rdx).wrapping_add(rax)) as u64; fl_r = fl_l; rdx = ((fl_l as u64)) as u64; r8 = ((r8).wrapping_shl((((rax) as u32) & 63))) as u64; bb = 19; continue; }
            19 => { fl_l = rdx; fl_r = r10; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 12; continue; } bb = 20; continue; }
            20 => { rbx = ((rbx).wrapping_add(1)) as u64; rax = r11.wrapping_add(0x1); fl_l = rbx; fl_r = rbp; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 3; continue; } bb = 21; continue; }
            21 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x40u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            22 => { crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), rsi); rsi = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), r14); fl_l = rbp; fl_r = rbp; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 41; continue; } bb = 23; continue; }
            23 => { r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; r15 = ((rsi.wrapping_add(0x1u64)) as u32) as u32 as u64; bb = 24; continue; }
            24 => { rbx = 0; fl_l = 0; fl_r = 0; fl_l = rax; fl_r = rbp; rbx = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { rbx }) as u64; fl_l = rdi; fl_r = (0x40u64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 39; continue; } bb = 25; continue; }
            25 => { r8 = (crate::simd_ops::ru64(r13.wrapping_add(rbx.wrapping_mul(8)))) as u64; fl_l = r8; fl_r = (0xffffffffffffffffu64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 39; continue; } bb = 26; continue; }
            26 => { fl_l = rdi; fl_r = (0x40u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 28; continue; } bb = 27; continue; }
            27 => { rcx = (0xffffffffffffffffu64) as u64; bb = 29; continue; }
            28 => { rcx = ((r15).wrapping_shl((((rdi) as u32) & 63))) as u64; fl_l = ((rcx).wrapping_sub(1)) as u64; rcx = ((fl_l as u64)) as u64; bb = 29; continue; }
            29 => { r11 = 0x40; rax = r8; r11 = ((r11).wrapping_sub(rdi)) as u64; rax = (!(rax)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 31; continue; } bb = 30; continue; }
            30 => { r10 = ((rax.wrapping_add(0x40u64)) as u32) as u32 as u64; bb = 32; continue; }
            31 => { let bit_in: u64 = rax; if bit_in != 0 { rax = ((bit_in.trailing_zeros() as u64)) as u64; } fl_l = bit_in as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((rax as u32)) as u32); r10 = ((rax as u32)) as u32 as u64; bb = 32; continue; }
            32 => { r9 = ((rcx).wrapping_shl((((r10) as u32) & 63))) as u64; fl_l = r10; fl_r = r11; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 39; continue; } bb = 33; continue; }
            33 => { rcx = r9; fl_l = (((rcx) & (r8))) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 37; continue; } bb = 34; continue; }
            34 => { rcx = r9; rax = r8; rcx = (((rcx) | (r8))) as u64; fl_l = rax; let cx_src: u64 = rcx; let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u64(r13.wrapping_add(rbx.wrapping_mul(8)), fl_l as u64, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u64)) as u64; } r8 = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 38; continue; } bb = 35; continue; }
            35 => { rbx = ((rbx).wrapping_shl(6u32)) as u64; fl_l = ((rbx).wrapping_add(r10)) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; crate::simd_ops::wu64(r14, rbx); bb = 36; continue; }
            36 => { rax = (((r15 as u8)) as u32) as u32 as u64; bb = 42; continue; }
            37 => { let bit_in: u64 = rcx; if bit_in != 0 { rcx = (((63u32.wrapping_sub(bit_in.leading_zeros())) as u64)) as u64; } fl_l = bit_in as u64; rax = 0x3f; rdx = 0x40; rax = (((rax as u32)).wrapping_sub((rcx as u32))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x34u64), ((rcx as u32)) as u32); rcx = ((rax as u32)) as u32 as u64; rdx = ((rdx).wrapping_sub(rcx)) as u64; rax = rdx; fl_l = rax; fl_r = r10; rax = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; r10 = rdx; r9 = ((r9).wrapping_shl((((rax) as u32) & 63))) as u64; bb = 38; continue; }
            38 => { fl_l = r10; fl_r = r11; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 33; continue; } bb = 39; continue; }
            39 => { crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), r14); r9 = rdi; r8 = rbx; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (0x0u64) as u64); rdx = rbp; rcx = r13; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 36; continue; } bb = 40; continue; }
            40 => { rsi = ((rsi).wrapping_add(1)) as u64; rax = rbx.wrapping_add(0x1); fl_l = rsi; fl_r = rbp; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 24; continue; } bb = 41; continue; }
            41 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 42; continue; }
            42 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425474d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 1>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x8u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); rdx = r9; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); r9 = ((((r9 as u32)) & (0x3fu32))) as u32 as u64; rbx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rdi); rdx = ((rdx).wrapping_shr(6u32)) as u64; rdi = r8; crate::simd_ops::wu64(rsp, r14); r14 = rcx; rax = ((r9.wrapping_add(r8)) as u64) as u64; rsi = ((rbx.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = rax; fl_r = (0x40u64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 6; continue; } bb = 1; continue; }
            1 => { fl_l = r8; fl_r = (0x40u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 3; continue; } bb = 2; continue; }
            2 => { r10 = rbx.wrapping_sub(0x1); r8 = ((rbx as u32)) as u32 as u64; rdi = ((rbx as u32)) as u32 as u64; r11 = ((rbx as u32)) as u32 as u64; bb = 14; continue; }
            3 => { fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { r10 = rbx; r8 = rbx; rdi = rbx; r11 = rbx; bb = 14; continue; }
            5 => { rax = ((rsi).wrapping_shl((((rdi) as u32) & 63))) as u64; fl_l = ((rax).wrapping_sub(1)) as u64; rax = ((fl_l as u64)) as u64; r8 = rbx; r10 = ((rax).wrapping_shl((((r9) as u32) & 63))) as u64; rdi = rbx; r11 = rbx; bb = 14; continue; }
            6 => { rcx = 0x40; r8 = (0xffffffffffffffffu64) as u64; rcx = ((rcx).wrapping_sub(r9)) as u64; fl_l = rcx; fl_r = (0x40u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 8; continue; } bb = 7; continue; }
            7 => { r10 = r8; bb = 11; continue; }
            8 => { fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { r10 = rbx; bb = 11; continue; }
            10 => { rax = ((rsi).wrapping_shl((((rcx) as u32) & 63))) as u64; fl_l = ((rax).wrapping_sub(1)) as u64; rax = ((fl_l as u64)) as u64; r10 = ((rax).wrapping_shl((((r9) as u32) & 63))) as u64; bb = 11; continue; }
            11 => { rdi = ((rdi).wrapping_sub(rcx)) as u64; r11 = rdi; r11 = ((r11).wrapping_shr(6u32)) as u64; fl_l = ((((rdi as u32)) & (0x3fu32))) as u32 as u64; fl_r = fl_l; rdi = ((fl_l as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 13; continue; } bb = 12; continue; }
            12 => { rdi = rbx; bb = 14; continue; }
            13 => { rdi = ((rsi).wrapping_shl((((rdi) as u32) & 63))) as u64; fl_l = ((rdi).wrapping_sub(1)) as u64; rdi = ((fl_l as u64)) as u64; bb = 14; continue; }
            14 => { rcx = ((r14.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; r14 = (crate::simd_ops::ru64(rsp)) as u64; bb = 15; continue; }
            15 => { rdx = rax; rdx = (((rdx) | (r10))) as u64; fl_l = rax; let cx_src: u64 = rdx; let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u64(rcx, fl_l as u64, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u64)) as u64; } if (((fl_l as u64)) != ((fl_r as u64))) { bb = 15; continue; } bb = 16; continue; }
            16 => { rcx = ((rcx).wrapping_add(0x8u64)) as u64; fl_l = (((rax) & (r10))) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; r9 = (r9 & !0xffu64) | (((if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { 1u8 } else { 0u8 }) as u8 as u64)); fl_l = rax; fl_r = r10; r10 = (r10 & !0xffu64) | (((if (((fl_l as u64)) != ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); fl_l = r11; fl_r = r11; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 22; continue; } bb = 17; continue; }
            17 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); rbp = (((r9 as u8)) as u32) as u32 as u64; bb = 18; continue; }
            18 => { fl_l = ((r11).wrapping_sub(1)) as u64; r11 = ((fl_l as u64)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; bb = 19; continue; }
            19 => { rdx = rax; rdx = (((rdx) | (r8))) as u64; fl_l = rax; let cx_src: u64 = rdx; let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u64(rcx, fl_l as u64, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u64)) as u64; } if (((fl_l as u64)) != ((fl_r as u64))) { bb = 19; continue; } bb = 20; continue; }
            20 => { rcx = ((rcx).wrapping_add(0x8u64)) as u64; rdx = (((rbp as u8)) as u32) as u32 as u64; fl_l = (((rax) & (r8))) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; r10 = (((r10 as u8)) as u32) as u32 as u64; r9 = ((rbx as u32)) as u32 as u64; r9 = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { (rdx as u32) } else { (r9 as u32) }) as u32 as u64; fl_l = rax; fl_r = r8; rbp = (((r9 as u8)) as u32) as u32 as u64; r10 = (if (((fl_l as u64)) != ((fl_r as u64))) { (rsi as u32) } else { (r10 as u32) }) as u32 as u64; fl_l = r11; fl_r = r11; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 18; continue; } bb = 21; continue; }
            21 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x18u64))) as u64; bb = 22; continue; }
            22 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 27; continue; } bb = 23; continue; }
            23 => { rax = (crate::simd_ops::ru64(rcx)) as u64; bb = 24; continue; }
            24 => { rdx = rax; rdx = (((rdx) | (rdi))) as u64; fl_l = rax; let cx_src: u64 = rdx; let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u64(rcx, fl_l as u64, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u64)) as u64; } if (((fl_l as u64)) != ((fl_r as u64))) { bb = 24; continue; } bb = 25; continue; }
            25 => { fl_l = (((rax) & (rdi))) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; rcx = (((r9 as u8)) as u32) as u32 as u64; rbx = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { (rcx as u32) } else { (rbx as u32) }) as u32 as u64; r9 = (((rbx as u8)) as u32) as u32 as u64; fl_l = rax; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 27; continue; } bb = 26; continue; }
            26 => { r10 = (r10 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 27; continue; }
            27 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x10u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 29; continue; } bb = 28; continue; }
            28 => { crate::simd_ops::wu8(rax, ((r10 as u8)) as u8); bb = 29; continue; }
            29 => { rax = (((r9 as u8)) as u32) as u32 as u64; rsp = ((rsp).wrapping_add(0x8u64)) as u64; return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142547700(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { return crate::simd_ops::tail_fwd_xmm(rax, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8, [xmm0, xmm1, xmm2, xmm3]); }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142547c60(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1401de3c0;
    let (mut rax, mut fl_l, mut fl_r, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbp);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbp = rdx;
    rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x20u64))) as u32 as u64;
    rax = ((((rax as u32)) & (0x2u32))) as u32 as u64;
    fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64;
    if !(((((fl_l as u32)) & ((fl_r as u32))) == 0)) { crate::simd_ops::lk_and_u32(rbp.wrapping_add(0x20u64), (0xfffffffdu32) as u32); rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x70u64))) as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); }
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbp = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142547cb0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142442de0; const P1: u64 = 0x14183acce;
    let (mut rax, mut fl_l, mut fl_r, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rdx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rbp = (rdx) as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x58u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rcx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142547cd0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142442de0; const P1: u64 = 0x14183acce;
    let (mut rax, mut fl_l, mut fl_r, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rdx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rbp = (rdx) as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x38u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rcx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142547cf0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142442de0; const P1: u64 = 0x14183acce;
    let (mut rax, mut fl_l, mut fl_r, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rdx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rbp = (rdx) as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x88u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rcx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142547d40(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142442de0; const P1: u64 = 0x14183acce;
    let (mut rax, mut fl_l, mut fl_r, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rdx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rbp = (rdx) as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x68u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rcx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142547d60(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142442de0; const P1: u64 = 0x14183acce;
    let (mut rax, mut fl_l, mut fl_r, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rdx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rbp = (rdx) as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x78u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rcx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142547da0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x30; const P1: u64 = 0x1401e4680; const P2: u64 = 0x38; const P3: u64 = 0x1401e5420; const P4: u64 = 0x1401e5380; const P5: u64 = 0x70; const P6: u64 = 0x1401e46b0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_l = ((rcx).wrapping_add(0x58u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            1 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P2))) as u64; fl_l = ((rcx).wrapping_add(0x8u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            2 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P2))) as u64; fl_l = ((rcx).wrapping_add(0x18u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            3 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P5))) as u64; return crate::simd_ops::call_fwd(P6, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142547df0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x80; const P1: u64 = 0x1401de400; const P2: u64 = 0x30; const P3: u64 = 0x1411ee710;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_l = ((rcx).wrapping_add(0x10u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            1 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_l = ((rcx).wrapping_add(0x20u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            2 => { rcx = ((rdx.wrapping_add(P2)) as u64) as u64; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142548290(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142442de0; const P1: u64 = 0x14183acce;
    let (mut rax, mut fl_l, mut fl_r, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rdx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbp = (rdx) as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x58u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rcx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142548860(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 1>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbp);
    rbp = rdx;
    rax = 0x208;
    r8 = ((crate::exe::tls_pointer() as u64)) as u64;
    r8 = (crate::simd_ops::ru64(r8)) as u64;
    r8 = ((r8).wrapping_add(rax)) as u64;
    r8 = (crate::simd_ops::ru32(r8)) as u32 as u64;
    r8 = ((((r8 as u32)) & (0xfffffffeu32))) as u32 as u64;
    rdx = ((crate::exe::tls_pointer() as u64)) as u64;
    rax = 0x208;
    rcx = ((rax as u32)) as u32 as u64;
    rax = (crate::simd_ops::ru64(rdx)) as u64;
    fl_l = ((rax).wrapping_add(rcx)) as u64; fl_r = fl_l;
    rax = ((fl_l as u64)) as u64;
    crate::simd_ops::wu32(rax, ((r8 as u32)) as u32);
    rbp = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425488a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 1>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbp);
    rbp = rdx;
    rax = 0x20c;
    r8 = ((crate::exe::tls_pointer() as u64)) as u64;
    r8 = (crate::simd_ops::ru64(r8)) as u64;
    r8 = ((r8).wrapping_add(rax)) as u64;
    r8 = (crate::simd_ops::ru32(r8)) as u32 as u64;
    r8 = ((((r8 as u32)) & (0xfffffffeu32))) as u32 as u64;
    rdx = ((crate::exe::tls_pointer() as u64)) as u64;
    rax = 0x20c;
    rcx = ((rax as u32)) as u32 as u64;
    rax = (crate::simd_ops::ru64(rdx)) as u64;
    fl_l = ((rax).wrapping_add(rcx)) as u64; fl_r = fl_l;
    rax = ((fl_l as u64)) as u64;
    crate::simd_ops::wu32(rax, ((r8 as u32)) as u32);
    rbp = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142548910(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1401de3c0;
    let (mut rax, mut fl_l, mut fl_r, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbp);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbp = rdx;
    rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x128u64))) as u32 as u64;
    rax = ((((rax as u32)) & (0x1u32))) as u32 as u64;
    fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64;
    if !(((((fl_l as u32)) & ((fl_r as u32))) == 0)) { crate::simd_ops::lk_and_u32(rbp.wrapping_add(0x128u64), (0xfffffffeu32) as u32); rcx = rbp.wrapping_add(0x50); rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); }
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbp = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142548980(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 1>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbp);
    rbp = rdx;
    rax = 0x204;
    r8 = ((crate::exe::tls_pointer() as u64)) as u64;
    r8 = (crate::simd_ops::ru64(r8)) as u64;
    r8 = ((r8).wrapping_add(rax)) as u64;
    r8 = (crate::simd_ops::ru32(r8)) as u32 as u64;
    r8 = ((((r8 as u32)) & (0xfffffffeu32))) as u32 as u64;
    rdx = ((crate::exe::tls_pointer() as u64)) as u64;
    rax = 0x204;
    rcx = ((rax as u32)) as u32 as u64;
    rax = (crate::simd_ops::ru64(rdx)) as u64;
    fl_l = ((rax).wrapping_add(rcx)) as u64; fl_r = fl_l;
    rax = ((fl_l as u64)) as u64;
    crate::simd_ops::wu32(rax, ((r8 as u32)) as u32);
    rbp = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425489c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 1>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbp);
    rbp = rdx;
    rax = 0x21c;
    r8 = ((crate::exe::tls_pointer() as u64)) as u64;
    r8 = (crate::simd_ops::ru64(r8)) as u64;
    r8 = ((r8).wrapping_add(rax)) as u64;
    r8 = (crate::simd_ops::ru32(r8)) as u32 as u64;
    r8 = ((((r8 as u32)) & (0xfffffffeu32))) as u32 as u64;
    rdx = ((crate::exe::tls_pointer() as u64)) as u64;
    rax = 0x21c;
    rcx = ((rax as u32)) as u32 as u64;
    rax = (crate::simd_ops::ru64(rdx)) as u64;
    fl_l = ((rax).wrapping_add(rcx)) as u64; fl_r = fl_l;
    rax = ((fl_l as u64)) as u64;
    crate::simd_ops::wu32(rax, ((r8 as u32)) as u32);
    rbp = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142548a30(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x18; const P2: u64 = 0x14240aa00; const P3: u64 = 0x20; const P4: u64 = 0x1401eb150; const P5: u64 = 0x14240ab10;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x38u64)) as u64; rbp = rdx; rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x140u64))) as u64; rbx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64; rdi = (((rax) as i64).wrapping_mul((0x108u64) as i64) as u64) as u64; rdi = ((rdi).wrapping_add(rbx)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 3; continue; } bb = 1; continue; }
            1 => { rsi = (crate::simd_ops::ru64(rbp.wrapping_add(0x180u64))) as u64; bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rbp.wrapping_add(0x30u64), (0x20000020u64) as u64); crate::simd_ops::wu8(rbp.wrapping_add(0x38u64), (0x0u8) as u8); rdx = rbx; rcx = rbp.wrapping_add(0x30); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rdx = rbp.wrapping_add(0x30); rcx = ((rsi.wrapping_add(P3)) as u64) as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rcx = rbp.wrapping_add(0x30); rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rbx = ((rbx).wrapping_add(0x108u64)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 3; continue; }
            3 => { rax = (0x0u64) as u64; rsp = ((rsp).wrapping_add(0x38u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142548ac0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 1>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbp);
    rbp = rdx;
    rax = 0x1ec;
    r8 = ((crate::exe::tls_pointer() as u64)) as u64;
    r8 = (crate::simd_ops::ru64(r8)) as u64;
    r8 = ((r8).wrapping_add(rax)) as u64;
    r8 = (crate::simd_ops::ru32(r8)) as u32 as u64;
    r8 = ((((r8 as u32)) & (0xfffffffeu32))) as u32 as u64;
    rdx = ((crate::exe::tls_pointer() as u64)) as u64;
    rax = 0x1ec;
    rcx = ((rax as u32)) as u32 as u64;
    rax = (crate::simd_ops::ru64(rdx)) as u64;
    fl_l = ((rax).wrapping_add(rcx)) as u64; fl_r = fl_l;
    rax = ((fl_l as u64)) as u64;
    crate::simd_ops::wu32(rax, ((r8 as u32)) as u32);
    rbp = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142548b00(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 1>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbp);
    rbp = rdx;
    rax = 0x200;
    r8 = ((crate::exe::tls_pointer() as u64)) as u64;
    r8 = (crate::simd_ops::ru64(r8)) as u64;
    r8 = ((r8).wrapping_add(rax)) as u64;
    r8 = (crate::simd_ops::ru32(r8)) as u32 as u64;
    r8 = ((((r8 as u32)) & (0xfffffffeu32))) as u32 as u64;
    rdx = ((crate::exe::tls_pointer() as u64)) as u64;
    rax = 0x200;
    rcx = ((rax as u32)) as u32 as u64;
    rax = (crate::simd_ops::ru64(rdx)) as u64;
    fl_l = ((rax).wrapping_add(rcx)) as u64; fl_r = fl_l;
    rax = ((fl_l as u64)) as u64;
    crate::simd_ops::wu32(rax, ((r8 as u32)) as u32);
    rbp = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142548b40(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 1>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbp);
    rbp = rdx;
    rax = 0x1e8;
    r8 = ((crate::exe::tls_pointer() as u64)) as u64;
    r8 = (crate::simd_ops::ru64(r8)) as u64;
    r8 = ((r8).wrapping_add(rax)) as u64;
    r8 = (crate::simd_ops::ru32(r8)) as u32 as u64;
    r8 = ((((r8 as u32)) & (0xfffffffeu32))) as u32 as u64;
    rdx = ((crate::exe::tls_pointer() as u64)) as u64;
    rax = 0x1e8;
    rcx = ((rax as u32)) as u32 as u64;
    rax = (crate::simd_ops::ru64(rdx)) as u64;
    fl_l = ((rax).wrapping_add(rcx)) as u64; fl_r = fl_l;
    rax = ((fl_l as u64)) as u64;
    crate::simd_ops::wu32(rax, ((r8 as u32)) as u32);
    rbp = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142548b80(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 1>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbp);
    rbp = rdx;
    rax = 0x218;
    r8 = ((crate::exe::tls_pointer() as u64)) as u64;
    r8 = (crate::simd_ops::ru64(r8)) as u64;
    r8 = ((r8).wrapping_add(rax)) as u64;
    r8 = (crate::simd_ops::ru32(r8)) as u32 as u64;
    r8 = ((((r8 as u32)) & (0xfffffffeu32))) as u32 as u64;
    rdx = ((crate::exe::tls_pointer() as u64)) as u64;
    rax = 0x218;
    rcx = ((rax as u32)) as u32 as u64;
    rax = (crate::simd_ops::ru64(rdx)) as u64;
    fl_l = ((rax).wrapping_add(rcx)) as u64; fl_r = fl_l;
    rax = ((fl_l as u64)) as u64;
    crate::simd_ops::wu32(rax, ((r8 as u32)) as u32);
    rbp = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142548bc0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 1>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbp);
    rbp = rdx;
    rax = 0x210;
    r8 = ((crate::exe::tls_pointer() as u64)) as u64;
    r8 = (crate::simd_ops::ru64(r8)) as u64;
    r8 = ((r8).wrapping_add(rax)) as u64;
    r8 = (crate::simd_ops::ru32(r8)) as u32 as u64;
    r8 = ((((r8 as u32)) & (0xfffffffeu32))) as u32 as u64;
    rdx = ((crate::exe::tls_pointer() as u64)) as u64;
    rax = 0x210;
    rcx = ((rax as u32)) as u32 as u64;
    rax = (crate::simd_ops::ru64(rdx)) as u64;
    fl_l = ((rax).wrapping_add(rcx)) as u64; fl_r = fl_l;
    rax = ((fl_l as u64)) as u64;
    crate::simd_ops::wu32(rax, ((r8 as u32)) as u32);
    rbp = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142548c00(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 1>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbp);
    rbp = rdx;
    rax = 0x214;
    r8 = ((crate::exe::tls_pointer() as u64)) as u64;
    r8 = (crate::simd_ops::ru64(r8)) as u64;
    r8 = ((r8).wrapping_add(rax)) as u64;
    r8 = (crate::simd_ops::ru32(r8)) as u32 as u64;
    r8 = ((((r8 as u32)) & (0xfffffffeu32))) as u32 as u64;
    rdx = ((crate::exe::tls_pointer() as u64)) as u64;
    rax = 0x214;
    rcx = ((rax as u32)) as u32 as u64;
    rax = (crate::simd_ops::ru64(rdx)) as u64;
    fl_l = ((rax).wrapping_add(rcx)) as u64; fl_r = fl_l;
    rax = ((fl_l as u64)) as u64;
    crate::simd_ops::wu32(rax, ((r8 as u32)) as u32);
    rbp = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142548c60(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x30; const P1: u64 = 0x1401de3c0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_l = ((rcx).wrapping_add(0x28u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            1 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_l = ((rcx).wrapping_add(0x50u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            2 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_l = ((rcx).wrapping_add(0x78u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            3 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_l = ((rcx).wrapping_add(0xa0u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142549640(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142442de0; const P1: u64 = 0x14183acce;
    let (mut rax, mut fl_l, mut fl_r, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rdx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbp = (rdx) as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x30u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rcx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142549d40(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x30; const P1: u64 = 0x1401e5030; const P2: u64 = 0x143843870; const P3: u64 = 0x1401de420;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_l = ((rcx).wrapping_add(0x20u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            1 => { rcx = P2; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14254a240(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142442de0; const P1: u64 = 0x14183acce;
    let (mut rax, mut fl_l, mut fl_r, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut xmm0: u128 = 0; let mut xmm1: u128 = 0; let mut xmm2: u128 = 0; let mut xmm3: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rdx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbp = (rdx) as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x40u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rcx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14254a290(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142442c80; const P1: u64 = 0x14183acce;
    let (mut rax, mut fl_l, mut fl_r, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut xmm0: u128 = 0; let mut xmm1: u128 = 0; let mut xmm2: u128 = 0; let mut xmm3: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rdx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbp = (rdx) as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x60u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rcx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14254a4d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142442de0; const P1: u64 = 0x14183acce;
    let (mut rax, mut fl_l, mut fl_r, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rdx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rbp = (rdx) as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x68u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rcx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14254aa20(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142442de0; const P1: u64 = 0x14183acce;
    let (mut rax, mut fl_l, mut fl_r, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rdx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rbp = (rdx) as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x70u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rcx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14254abf0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x18; const P2: u64 = 0x14240aa00; const P3: u64 = 0x1401eb150; const P4: u64 = 0x14240ab10;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x28u64)) as u64; rbp = rdx; rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x38u64))) as u64; rbx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64; rdi = (((rax) as i64).wrapping_mul((0x108u64) as i64) as u64) as u64; rdi = ((rdi).wrapping_add(rbx)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 3; continue; } bb = 1; continue; }
            1 => { rsi = (crate::simd_ops::ru64(rbp.wrapping_add(0x3b8u64))) as u64; bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rbp.wrapping_add(0x40u64), (0x20000020u64) as u64); crate::simd_ops::wu8(rbp.wrapping_add(0x48u64), (0x0u8) as u8); rdx = rbx; rcx = rbp.wrapping_add(0x40); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rdx = rbp.wrapping_add(0x40); rcx = rsi; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = rbp.wrapping_add(0x40); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rbx = ((rbx).wrapping_add(0x108u64)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 3; continue; }
            3 => { rax = (0x0u64) as u64; rsp = ((rsp).wrapping_add(0x28u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14254ad40(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143c48300; const P1: u64 = 0x140202b60; const P2: u64 = 0x30; const P3: u64 = 0x140202af0; const P4: u64 = 0x28; const P5: u64 = 0x140203b40;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rcx = P0; fl_l = ((rcx).wrapping_add(0x18u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            1 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P2))) as u64; fl_l = ((rcx).wrapping_add(0x8u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            2 => { rcx = ((rdx.wrapping_add(P4)) as u64) as u64; return crate::simd_ops::call_fwd(P5, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14254ae00(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x140205240; const P1: u64 = 0x140203920; const P2: u64 = 0x14183acce;
    let (mut rax, mut fl_l, mut fl_r, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rdx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbp = (rdx) as u64; rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x20u64))) as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x70u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); r8 = (crate::simd_ops::ru64(rbp.wrapping_add(0x78u64))) as u64; rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x28u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rcx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14254af20(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142442de0; const P1: u64 = 0x14183acce;
    let (mut rax, mut fl_l, mut fl_r, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rdx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbp = (rdx) as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x48u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rcx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14254b150(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1401de3c0; const P1: u64 = 0x141834ee8;
    let (mut rax, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbp);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbp = rdx;
    r9 = P0;
    r8 = 0xc;
    rdx = 0x8c;
    rcx = rbp.wrapping_add(0x60);
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbp = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14254c530(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x100; const P1: u64 = 0x140251770; const P2: u64 = 0x140251760;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_l = ((rcx).wrapping_add(0x30u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            1 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_l = ((rcx).wrapping_add(0x88u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            _ => { return rax; }
        }
    }
}
