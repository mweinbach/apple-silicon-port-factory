#![allow(unused_imports, unused_variables, unused_mut, unused_assignments, non_snake_case, dead_code, clippy::all, unreachable_code, unused_parens)]

#[inline(never)]
pub unsafe extern "win64" fn fn_1416747b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x34; const P1: u64 = 0x1415242e0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(P0))) as u8 as u64; fl_r = (0x1u8) as u8 as u64;
    rbx = rcx;
    if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { rdx = rbx; rcx = 0x7; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P1, rax, rcx, rdx, r8, r9, rsp); }
    rdx = (crate::simd_ops::ru64(rcx)) as u64;
    fl_l = rdx; fl_r = rdx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rcx = 0x7; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); }
    rdx = rbx;
    rcx = 0x7;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(P1, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416747f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143932040; const P1: u64 = 0x143932048; const P2: u64 = 0x143932038; const P3: u64 = 0x28; const P4: u64 = 0x1415242e0; const P5: u64 = 0x14393204c; const P6: u64 = 0x143932050; const P7: u64 = 0x143932058; const P8: u64 = 0x14393205c;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; r8 = (crate::simd_ops::ru64(P0)) as u64; rsi = 0; fl_l = 0; fl_r = 0; r9 = (crate::simd_ops::ru32(P1)) as u32 as u64; rdi = rcx; crate::simd_ops::wu64(P2, rsi); rbx = r8; rdx = ((r8.wrapping_add(r9.wrapping_mul(8))) as u64) as u64; fl_l = r8; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 4; continue; } bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); r9 = (crate::simd_ops::ru32(P1)) as u32 as u64; r8 = (crate::simd_ops::ru64(P0)) as u64; bb = 3; continue; }
            3 => { rax = ((r9 as u32)) as u32 as u64; rbx = ((rbx).wrapping_add(0x8u64)) as u64; rcx = ((r8.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 4; continue; }
            4 => { fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rdx = r8; crate::simd_ops::wu32(P1, ((rsi as u32)) as u32); rcx = 0x7; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(P0, rsi); crate::simd_ops::wu32(P5, ((rsi as u32)) as u32); bb = 6; continue; }
            6 => { rdx = (crate::simd_ops::ru64(P6)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { rcx = 0x7; crate::simd_ops::wu32(P7, ((rsi as u32)) as u32); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(P6, rsi); crate::simd_ops::wu32(P8, ((rsi as u32)) as u32); bb = 8; continue; }
            8 => { fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rdi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rdx = rdi; rcx = 0x7; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 10; continue; }
            10 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416748d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143932048; const P1: u64 = 0x143932040; const P2: u64 = 0x28;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    fl_l = ((rcx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(P0)) as u32 as u64;
    if (((fl_l as u32)) >= ((fl_r as u32))) { rax = 0x1f; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    rax = ((rcx as u32)) as u32 as u64;
    rbx = ((rax.wrapping_mul(8)) as u64) as u64;
    rax = (crate::simd_ops::ru64(P1)) as u64;
    rcx = (crate::simd_ops::ru64(rbx.wrapping_add(rax))) as u64;
    fl_l = rcx; fl_r = rcx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(P1)) as u64; crate::simd_ops::wu64(rbx.wrapping_add(rax), (0x0u64) as u64); rax = 0x1; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    rax = 0x1f;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141674930(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143932038; const P1: u64 = 0x14183acc2;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedStack::<1>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rbp = rsp.wrapping_add(0x40); rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x34u64))) as u32 as u64; rdi = rcx.wrapping_add(0x8); fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x2u8) as u8 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rdx); crate::simd_ops::wu64(rbp.wrapping_add(0x28u64), rbx); rsi = rdx; rbx = rcx; crate::simd_ops::wu8(rbp.wrapping_add(0x20u64), (0x1u8) as u8); rcx = 0x0; rdx = rbp.wrapping_add(0x20); rdi = (if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { rcx } else { rdi }) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdx); rcx = (crate::simd_ops::ru64(P0)) as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; r8 = (crate::simd_ops::ru32(rbx.wrapping_add(0x30u64))) as u32 as u64; r9 = rdi; rax = (crate::simd_ops::ru64(rcx)) as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rdx = (crate::simd_ops::ru64(rbx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 3; continue; }
            2 => { rdx = (crate::simd_ops::ru32(rbx)) as u32 as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 3; continue; }
            3 => { crate::simd_ops::wu32(rbp.wrapping_add(0x30u64), ((rax as u32)) as u32); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 7; continue; } bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::ru64(rsi)) as u64; fl_r = (0x0u64) as u64; if ((((fl_l as u64)) as i64) > (((fl_r as u64)) as i64)) { bb = 6; continue; } bb = 5; continue; }
            5 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x30u64))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 7; continue; } bb = 6; continue; }
            6 => { fl_l = (crate::simd_ops::ru8(rbp.wrapping_add(0x20u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 11; continue; } bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru32(rsp)) as u32 as u64; rax = 0x210; rsp = ((rsp).wrapping_sub(rax)) as u64; rsi = rsp.wrapping_add(0x40); rax = (crate::simd_ops::ru32(rsi)) as u32 as u64; rdx = 0; fl_l = 0; fl_r = 0; r8 = 0x104; rcx = rsi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0x34u64))) as u8 as u64; fl_r = (0x1u8) as u8 as u64; rdx = rbp.wrapping_add(0x30); rcx = (crate::simd_ops::ru64(P0)) as u64; r9 = rdi; rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x30u64))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), (0x104u32) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rsi); r10 = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rax as u32)) as u32); if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { r8 = (crate::simd_ops::ru64(rbx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(r10.wrapping_add(0x20u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 10; continue; }
            9 => { r8 = (crate::simd_ops::ru32(rbx)) as u32 as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(r10.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 10; continue; }
            10 => { crate::simd_ops::wu32(rbp.wrapping_add(0x30u64), (0x2u32) as u32); rax = 0x2; bb = 11; continue; }
            11 => { rbx = (crate::simd_ops::ru64(rbp.wrapping_add(0x28u64))) as u64; rsp = rbp; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141674a20(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143932048; const P1: u64 = 0x143932040; const P2: u64 = 0x1416768e0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdx = (crate::simd_ops::ru32(P0)) as u32 as u64; rbx = 0; fl_l = 0; fl_r = 0; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 5; continue; } bb = 1; continue; }
            1 => { bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(P1)) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(rbx.wrapping_mul(8)))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru32(P0)) as u32 as u64; bb = 4; continue; }
            4 => { rbx = (((rbx as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 2; continue; } bb = 5; continue; }
            5 => { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141674a70(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143932048; const P1: u64 = 0x143932040; const P2: u64 = 0x141676920;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; r9 = (crate::simd_ops::ru32(P0)) as u32 as u64; rbx = 0; fl_l = 0; fl_r = 0; rdi = (((rdx as u8)) as u32) as u32 as u64; rsi = rcx; fl_l = ((r9 as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 5; continue; } bb = 1; continue; }
            1 => { bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(P1)) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(rbx.wrapping_mul(8)))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { fl_l = rsi; fl_r = rcx; r8 = (((rdi as u8)) as u32) as u32 as u64; rdx = (rdx & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); r9 = (crate::simd_ops::ru32(P0)) as u32 as u64; bb = 4; continue; }
            4 => { rbx = (((rbx as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 2; continue; } bb = 5; continue; }
            5 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141674ae0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x18; const P2: u64 = 0x10; const P3: u64 = 0x1415242e0; const P4: u64 = 0x14;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rdi = rcx;
    fl_l = rbx; fl_r = rbx;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64;
    rbx = ((rbx).wrapping_add(0x10u64)) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rsi);
    rsi = 0;
    fl_l = 0; fl_r = 0;
    fl_l = rdx; fl_r = rdx;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rdx = (crate::simd_ops::ru64(rdi.wrapping_add(P0))) as u64; rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rdi.wrapping_add(P0), rsi); rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    crate::simd_ops::wu32(rbx.wrapping_add(P2), ((rsi as u32)) as u32);
    fl_l = rbx; fl_r = rdx;
    if !((((fl_l as u64)) == ((fl_r as u64)))) { rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); }
    crate::simd_ops::wu64(rbx.wrapping_add(P0), rsi);
    crate::simd_ops::wu32(rbx.wrapping_add(P4), ((rsi as u32)) as u32);
    rdx = (crate::simd_ops::ru64(rdi.wrapping_add(P0))) as u64;
    rcx = 0;
    fl_l = 0; fl_r = 0;
    rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp);
    crate::simd_ops::wu64(rdi.wrapping_add(P0), rsi);
    rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141674b40(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143426ea4;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x48u64)) as u64; rdi = 0; fl_l = 0; fl_r = 0; r15 = r9; crate::simd_ops::wu32(r8, ((rdi as u32)) as u32); r12 = r8; crate::simd_ops::wu8(r9, ((rdi as u8)) as u8); rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; rcx = (crate::simd_ops::ru32(rcx.wrapping_add(0x10u64))) as u32 as u64; rbp = ((rdi.wrapping_add(0x2u64)) as u32) as u32 as u64; r10 = ((rcx.wrapping_add(rcx.wrapping_mul(2))) as u64) as u64; rcx = ((rax.wrapping_add(r10.wrapping_mul(8))) as u64) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 3; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru32(rax)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rax = ((rax).wrapping_add(0x18u64)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 3; continue; }
            3 => { fl_l = rax; fl_r = rcx; rdx = rax.wrapping_add(0x8); rdx = (if (((fl_l as u64)) == ((fl_r as u64))) { rdi } else { rdx }) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 15; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; rdx = rsp.wrapping_add(0x70); crate::simd_ops::wu64(rsp.wrapping_add(0x80u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), r14); rcx = (crate::simd_ops::ru64(rax)) as u64; fl_l = rcx; fl_r = rcx; crate::simd_ops::wu32(rsp.wrapping_add(0x70u64), ((rdi as u32)) as u32); rsi = rcx.wrapping_sub(0x80); rsi = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rdi } else { rsi }) as u64; rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x80u64))) as u64; rcx = rsi.wrapping_add(0x80); (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x50u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbp = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 14; continue; } bb = 5; continue; }
            5 => { rdx = (crate::simd_ops::ru64(rsi)) as u64; rcx = rsi; crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), rbx); (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rdx.wrapping_add(0x40u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 13; continue; } bb = 6; continue; }
            6 => { r8 = (crate::simd_ops::ru64(rsi.wrapping_add(0x80u64))) as u64; rdx = rsp.wrapping_add(0x20); rcx = rsi.wrapping_add(0x80); (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(r8.wrapping_add(0x10u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { fl_l = rbx; fl_r = rax; rbx = (if (((fl_l as u64)) >= ((fl_r as u64))) { (rax as u32) } else { (rbx as u32) }) as u32 as u64; bb = 8; continue; }
            8 => { rdx = (crate::simd_ops::ru32(rsp.wrapping_add(0x70u64))) as u32 as u64; xmm1 = 0u128; rax = ((rbx as u32)) as u32 as u64; xmm0 = 0u128; xmm1 = (xmm1 & !0xffff_ffffu128) | ((((rdx) as i64) as f32).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((((rax) as i64) as f32).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) / f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(crate::simd_ops::ru32(P0))).to_bits() as u128); crate::simd_ops::wu32(r12, ((xmm1 as u32)) as u32); fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 11; continue; } bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x60u64))) as u64; rbx = (((rbx as u32)).wrapping_sub((rdx as u32))) as u32 as u64; rcx = (crate::simd_ops::ru32(rax.wrapping_add(0x43cu64))) as u32 as u64; rcx = (((rcx as u32)).wrapping_sub(crate::simd_ops::ru32(rax.wrapping_add(0x440u64)))) as u32 as u64; fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 11; continue; } bb = 10; continue; }
            10 => { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 12; continue; }
            11 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 12; continue; }
            12 => { crate::simd_ops::wu8(r15, ((rax as u8)) as u8); bb = 13; continue; }
            13 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; bb = 14; continue; }
            14 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; bb = 15; continue; }
            15 => { fl_l = ((rbp as u32)) as u32 as u64; fl_r = 0x2; rdi = (rdi & !0xffu64) | (((if (((fl_l as u32)) == ((fl_r as u32))) { 1u8 } else { 0u8 }) as u8 as u64)); rax = ((rdi.wrapping_add(0x1u64)) as u32) as u32 as u64; rsp = ((rsp).wrapping_add(0x48u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141674ca0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x30; const P2: u64 = 0xc; const P3: u64 = 0x14; const P4: u64 = 0x18; const P5: u64 = 0x24; const P6: u64 = 0x1c; const P7: u64 = 0x28; const P8: u64 = 0x2c; const P9: u64 = 0x34;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu32(rcx.wrapping_add(P0), (0x200000u32) as u32);
    crate::simd_ops::wu64(rcx, rax);
    crate::simd_ops::wu8(rcx.wrapping_add(P1), ((rax as u8)) as u8);
    crate::simd_ops::wu64(rcx.wrapping_add(P2), (0x1000u64) as u64);
    crate::simd_ops::wu32(rcx.wrapping_add(P3), (0x4000u32) as u32);
    crate::simd_ops::wu32(rcx.wrapping_add(P4), (0x1u32) as u32);
    crate::simd_ops::wu32(rcx.wrapping_add(P5), (0x20000u32) as u32);
    crate::simd_ops::wu64(rcx.wrapping_add(P6), (0x1u64) as u64);
    crate::simd_ops::wu32(rcx.wrapping_add(P7), (0x43be0000u32) as u32);
    crate::simd_ops::wu32(rcx.wrapping_add(P8), (0x8u32) as u32);
    crate::simd_ops::wu32(rcx.wrapping_add(P9), (0xffffffffu32) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141674d00(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141524350; const P1: u64 = 0x141524320; const P2: u64 = 0x14183acbc; const P3: u64 = 0x1415242e0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdx = (crate::simd_ops::ru32(rcx.wrapping_add(0x14u64))) as u32 as u64; r8 = 0x1; rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; rbp = ((rdx as u32)) as u32 as u64; rsi = (crate::simd_ops::ru32(rcx.wrapping_add(0x10u64))) as u32 as u64; rdi = rcx; rbp = (((rbp as u32)).wrapping_shr(1u32)) as u32 as u64; rbp = (((rbp as u32)).wrapping_add((rdx as u32))) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; rbp = (if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { (r8 as u32) } else { (rbp as u32) }) as u32 as u64; rbp = (((rbp as u32)).wrapping_add((rdx as u32))) as u32 as u64; rdx = ((rbp as u32)) as u32 as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 1; continue; }
            1 => { fl_l = rdx; fl_r = r8; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rbx = rcx; bb = 5; continue; }
            3 => { rcx = 0; fl_l = 0; fl_r = 0; fl_l = rdi; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 7; continue; } bb = 4; continue; }
            4 => { r9 = ((r8 as u32)) as u32 as u64; r8 = rdx; rdx = rax; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rbx = rax; bb = 5; continue; }
            5 => { fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 19; continue; } bb = 6; continue; }
            6 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 20; continue; }
            7 => { rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rbx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 8; continue; }
            8 => { r8 = rsi; rdx = rdi; rcx = rax; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 19; continue; }
            9 => { fl_l = rdx; fl_r = r8; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 11; continue; } bb = 10; continue; }
            10 => { rbx = rdi; bb = 12; continue; }
            11 => { rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rbx = rax; bb = 12; continue; }
            12 => { fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 13; continue; }
            13 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 19; continue; } bb = 14; continue; }
            14 => { fl_l = rax; fl_r = rbx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 19; continue; } bb = 15; continue; }
            15 => { rax = 0; fl_l = 0; fl_r = 0; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 16; continue; }
            16 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; rdx = ((crate::simd_ops::ru8(rax.wrapping_add(rcx))) as u32) as u32 as u64; crate::simd_ops::wu8(rax.wrapping_add(rbx), ((rdx as u8)) as u8); rax = ((rax).wrapping_add(1)) as u64; fl_l = rax; fl_r = rsi; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 16; continue; } bb = 17; continue; }
            17 => { rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; fl_l = rdi; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 19; continue; } bb = 18; continue; }
            18 => { rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 19; continue; }
            19 => { crate::simd_ops::wu64(rdi.wrapping_add(0x8u64), rbx); rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); crate::simd_ops::wu32(rdi.wrapping_add(0x14u64), ((rbp as u32)) as u32); bb = 20; continue; }
            20 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141674e00(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xc; const P1: u64 = 0x8; const P2: u64 = 0x141524340; const P3: u64 = 0x141524330; const P4: u64 = 0x1415242e0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdx = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; rdi = rcx; rbp = ((rdx as u32)) as u32 as u64; rcx = 0x1; rbp = (((rbp as u32)).wrapping_shr(1u32)) as u32 as u64; rbp = (((rbp as u32)).wrapping_add((rdx as u32))) as u32 as u64; rax = (crate::simd_ops::ru64(rdi)) as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; rsi = (crate::simd_ops::ru32(rdi.wrapping_add(P1))) as u32 as u64; rbp = (if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { (rcx as u32) } else { (rbp as u32) }) as u32 as u64; rcx = 0x7; rbp = (((rbp as u32)).wrapping_add((rdx as u32))) as u32 as u64; rdx = ((rbp as u32)) as u32 as u64; rdx = ((rdx).wrapping_shl(3u32)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { r8 = rdx; rdx = rax; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rbx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 10; continue; } bb = 2; continue; }
            2 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 11; continue; }
            3 => { rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rbx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rdi)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 5; continue; }
            5 => { fl_l = rax; fl_r = rbx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 10; continue; } bb = 6; continue; }
            6 => { rax = 0; fl_l = 0; fl_r = 0; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 7; continue; }
            7 => { bb = 8; continue; }
            8 => { rcx = (crate::simd_ops::ru64(rdi)) as u64; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(rax.wrapping_mul(8)))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(rax.wrapping_mul(8)), rdx); rax = ((rax).wrapping_add(1)) as u64; fl_l = rax; fl_r = rsi; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 8; continue; } bb = 9; continue; }
            9 => { rdx = (crate::simd_ops::ru64(rdi)) as u64; rcx = 0x7; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 10; continue; }
            10 => { crate::simd_ops::wu64(rdi, rbx); rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); crate::simd_ops::wu32(rdi.wrapping_add(P0), ((rbp as u32)) as u32); bb = 11; continue; }
            11 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141674ec0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141524340; const P1: u64 = 0x141524330; const P2: u64 = 0x1415242e0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdx = (crate::simd_ops::ru32(rcx.wrapping_add(0xcu64))) as u32 as u64; rdi = rcx; rbp = ((rdx as u32)) as u32 as u64; rcx = 0x1; rbp = (((rbp as u32)).wrapping_shr(1u32)) as u32 as u64; rbp = (((rbp as u32)).wrapping_add((rdx as u32))) as u32 as u64; rsi = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; rbp = (if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { (rcx as u32) } else { (rbp as u32) }) as u32 as u64; rcx = (crate::simd_ops::ru64(rdi)) as u64; rbp = (((rbp as u32)).wrapping_add((rdx as u32))) as u32 as u64; rdx = ((rbp.wrapping_mul(2)) as u64) as u64; rdx = ((rdx).wrapping_add(rbp)) as u64; rdx = ((rdx).wrapping_shl(3u32)) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { r8 = rdx; rdx = rcx; rcx = 0x7; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 12; continue; } bb = 2; continue; }
            2 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 13; continue; }
            3 => { rcx = 0x7; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rdi)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 5; continue; }
            5 => { fl_l = rax; fl_r = rbx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 12; continue; } bb = 6; continue; }
            6 => { fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 7; continue; }
            7 => { rax = rbx; r8 = 0; fl_l = 0; fl_r = 0; bb = 8; continue; }
            8 => { fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { crate::simd_ops::wu64(rax.wrapping_add(0x10u64), r8); bb = 10; continue; }
            10 => { rcx = (crate::simd_ops::ru64(rdi)) as u64; rdx = rax; rdx = ((rdx).wrapping_sub(rbx)) as u64; xmm0 = (crate::simd_ops::ru128(rdx.wrapping_add(rcx))) as u128; crate::simd_ops::wu128(rax, (xmm0) as u128); xmm1 = ((crate::simd_ops::ru64(rdx.wrapping_add(rcx).wrapping_add(0x10u64))) as u64) as u128; crate::simd_ops::wu64(rax.wrapping_add(0x10u64), ((xmm1 as u64)) as u64); rax = ((rax).wrapping_add(0x18u64)) as u64; fl_l = rsi; fl_r = (0x1u64) as u64; rsi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 8; continue; } bb = 11; continue; }
            11 => { rdx = (crate::simd_ops::ru64(rdi)) as u64; rcx = 0x7; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 12; continue; }
            12 => { crate::simd_ops::wu64(rdi, rbx); rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); crate::simd_ops::wu32(rdi.wrapping_add(0xcu64), ((rbp as u32)) as u32); bb = 13; continue; }
            13 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141674fb0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xc; const P1: u64 = 0x8; const P2: u64 = 0x141524340; const P3: u64 = 0x141524330; const P4: u64 = 0x10; const P5: u64 = 0xfffffffffffffff0; const P6: u64 = 0x1415242e0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdx = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; rdi = rcx; rbp = ((rdx as u32)) as u32 as u64; rcx = 0x1; rbp = (((rbp as u32)).wrapping_shr(1u32)) as u32 as u64; rbp = (((rbp as u32)).wrapping_add((rdx as u32))) as u32 as u64; rax = (crate::simd_ops::ru64(rdi)) as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; rsi = (crate::simd_ops::ru32(rdi.wrapping_add(P1))) as u32 as u64; rbp = (if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { (rcx as u32) } else { (rbp as u32) }) as u32 as u64; rcx = 0x7; rbp = (((rbp as u32)).wrapping_add((rdx as u32))) as u32 as u64; rdx = ((rbp as u32)) as u32 as u64; rdx = ((rdx).wrapping_shl(4u32)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { r8 = rdx; rdx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 10; continue; } bb = 2; continue; }
            2 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 11; continue; }
            3 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rdi)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 5; continue; }
            5 => { fl_l = rax; fl_r = rbx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 10; continue; } bb = 6; continue; }
            6 => { fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 7; continue; }
            7 => { rax = 0; fl_l = 0; fl_r = 0; bb = 8; continue; }
            8 => { rcx = (crate::simd_ops::ru64(rdi)) as u64; rax = ((rax.wrapping_add(P4)) as u64) as u64; xmm0 = (crate::simd_ops::ru128(rax.wrapping_add(rcx).wrapping_add(P5))) as u128; crate::simd_ops::wu128(rax.wrapping_add(rbx).wrapping_add(P5), (xmm0) as u128); fl_l = rsi; fl_r = (0x1u64) as u64; rsi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 8; continue; } bb = 9; continue; }
            9 => { rdx = (crate::simd_ops::ru64(rdi)) as u64; rcx = 0x7; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 10; continue; }
            10 => { crate::simd_ops::wu64(rdi, rbx); rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); crate::simd_ops::wu32(rdi.wrapping_add(P0), ((rbp as u32)) as u32); bb = 11; continue; }
            11 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141675080(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141673950; const P1: u64 = 0x141678900; const P2: u64 = 0x141674ec0; const P3: u64 = 0x141524330; const P4: u64 = 0x142693b08; const P5: u64 = 0x1416756f0; const P6: u64 = 0x141674ae0; const P7: u64 = 0x1416755a0;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 15>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x50u64)) as u64; r10 = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; rbx = ((rdx as u32)) as u32 as u64; r15 = (crate::simd_ops::ru32(rcx.wrapping_add(0x10u64))) as u32 as u64; r12 = rcx; rbp = (((r9 as u8)) as u32) as u32 as u64; r13 = r8; rsi = 0x2; rax = r10; rdx = ((r15.wrapping_add(r15.wrapping_mul(2))) as u64) as u64; rcx = ((r10.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64; fl_l = r10; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 3; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru32(rax)) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rax = ((rax).wrapping_add(0x18u64)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 3; continue; }
            3 => { r8 = 0; fl_l = 0; fl_r = 0; rdx = rax.wrapping_add(0x8); fl_l = rax; fl_r = rcx; rdx = (if (((fl_l as u64)) == ((fl_r as u64))) { r8 } else { rdx }) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 4; continue; }
            4 => { rbx = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; rdx = (((rbp as u8)) as u32) as u32 as u64; rcx = rbx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 25; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rbx)) as u64; r10 = (crate::simd_ops::ru32(rbx.wrapping_add(0x20u64))) as u32 as u64; fl_l = rax; fl_r = rax; r11 = rax.wrapping_sub(0x80); rax = 0x0; r11 = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rax } else { r11 }) as u64; rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; r10 = ((r10).wrapping_add(rax)) as u64; r9 = (r9 & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; fl_l = rax; fl_r = r10; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 8; continue; } bb = 6; continue; }
            6 => { bb = 7; continue; }
            7 => { r8 = ((crate::simd_ops::ru8(rax)) as u32) as u32 as u64; fl_l = ((r8 as u8)) as u8 as u64; fl_r = ((r9 as u8)) as u8 as u64; rdx = (((r9 as u8)) as u32) as u32 as u64; r8 = (if ((((fl_l as u8)) as i8) <= (((fl_r as u8)) as i8)) { (rdx as u32) } else { (r8 as u32) }) as u32 as u64; rax = ((rax).wrapping_add(1)) as u64; r9 = (((r8 as u8)) as u32) as u32 as u64; fl_l = rax; fl_r = r10; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 7; continue; } bb = 8; continue; }
            8 => { rcx = (crate::simd_ops::ru64(r11.wrapping_add(0x60u64))) as u64; r8 = (((r9 as u8)) as u32) as u32 as u64; rdx = r11; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rsi = 0x1; bb = 25; continue; }
            9 => { crate::simd_ops::wu64(rsp.wrapping_add(0x80u64), rdi); fl_l = r10; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 12; continue; } bb = 10; continue; }
            10 => { fl_l = (crate::simd_ops::ru32(r10)) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 12; continue; } bb = 11; continue; }
            11 => { r10 = ((r10).wrapping_add(0x18u64)) as u64; fl_l = r10; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 10; continue; } bb = 12; continue; }
            12 => { fl_l = r10; fl_r = rcx; rdi = r10.wrapping_add(0x8); rdi = (if (((fl_l as u64)) == ((fl_r as u64))) { r8 } else { rdi }) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 19; continue; } bb = 13; continue; }
            13 => { rax = (crate::simd_ops::ru32(r12.wrapping_add(0x14u64))) as u32 as u64; fl_l = r15; fl_r = rax; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 16; continue; } bb = 14; continue; }
            14 => { rcx = r12.wrapping_add(0x8); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 24; continue; } bb = 15; continue; }
            15 => { r8 = 0; fl_l = 0; fl_r = 0; bb = 16; continue; }
            16 => { rax = (crate::simd_ops::ru32(r12.wrapping_add(0x14u64))) as u32 as u64; fl_l = r15; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 24; continue; } bb = 17; continue; }
            17 => { rdx = (crate::simd_ops::ru32(r12.wrapping_add(0x10u64))) as u32 as u64; rax = (crate::simd_ops::ru64(r12.wrapping_add(0x8u64))) as u64; rcx = ((rdx.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64; rdi = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; rax = ((rdx.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(r12.wrapping_add(0x10u64), ((rax as u32)) as u32); fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 24; continue; } bb = 18; continue; }
            18 => { crate::simd_ops::wu64(rdi.wrapping_add(0x10u64), r8); crate::simd_ops::wu32(rdi, ((rbx as u32)) as u32); rdi = ((rdi).wrapping_add(0x8u64)) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 24; continue; } bb = 19; continue; }
            19 => { rdx = 0x28; rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rdi.wrapping_add(0x8u64), rax); rdx = ((rbx as u32)) as u32 as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 23; continue; } bb = 20; continue; }
            20 => { rcx = 0; fl_l = 0; fl_r = 0; r9 = (((rbp as u8)) as u32) as u32 as u64; crate::simd_ops::wu64(rax, rcx); r8 = r13; crate::simd_ops::wu32(rax.wrapping_add(0x8u64), ((rcx as u32)) as u32); crate::simd_ops::wu64(rax.wrapping_add(0x18u64), rcx); crate::simd_ops::wu64(rax.wrapping_add(0x20u64), rcx); rcx = r12; r15 = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; rax = (crate::simd_ops::ru64(r12)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r15); rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x68u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 22; continue; } bb = 21; continue; }
            21 => { rdx = (((rbp as u8)) as u32) as u32 as u64; rcx = r15; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); r9 = ((rbx as u32)) as u32 as u64; r8 = P4; rdx = 0xb; rcx = rsp.wrapping_add(0x30); rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(r15)) as u64; rdx = rsp.wrapping_add(0x30); rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x40u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(r15)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x60u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rsi = 0x1; bb = 24; continue; }
            22 => { rcx = rdi; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rdx = ((rbx as u32)) as u32 as u64; bb = 23; continue; }
            23 => { rcx = r12.wrapping_add(0x8); rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); bb = 24; continue; }
            24 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; bb = 25; continue; }
            25 => { r11 = rsp.wrapping_add(0x50); rax = ((rsi as u32)) as u32 as u64; rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x38u64))) as u64; rbp = (crate::simd_ops::ru64(r11.wrapping_add(0x40u64))) as u64; rsp = r11; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416752c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143932050; const P1: u64 = 0x143932058; const P2: u64 = 0x8; const P3: u64 = 0xffffffffffffffff; const P4: u64 = 0x10; const P5: u64 = 0x14183acc8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; r8 = (crate::simd_ops::ru64(P0)) as u64; rdi = rcx; rdx = (crate::simd_ops::ru32(P1)) as u32 as u64; rbx = r8; rax = ((rdx as u32)) as u32 as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rax = ((rax).wrapping_add(r8)) as u64; fl_l = r8; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 8; continue; } bb = 1; continue; }
            1 => { bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 6; continue; } bb = 3; continue; }
            3 => { rax = ((rdx.wrapping_add(P3)) as u32) as u32 as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rax = ((rax).wrapping_add(r8)) as u64; fl_l = rbx; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = ((rax).wrapping_sub(rbx)) as u64; rdx = ((rbx.wrapping_add(P4)) as u64) as u64; rax = ((((rax as u32)) & (0xfffffff0u32))) as u32 as u64; rcx = rbx; r8 = ((rax as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru32(P1)) as u32 as u64; r8 = (crate::simd_ops::ru64(P0)) as u64; bb = 5; continue; }
            5 => { fl_l = (((rdx as u32)).wrapping_sub(1)) as u32 as u64; rdx = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(P1, ((rdx as u32)) as u32); bb = 7; continue; }
            6 => { fl_l = ((rbx).wrapping_add(0x10u64)) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; bb = 7; continue; }
            7 => { rax = ((rdx as u32)) as u32 as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rax = ((rax).wrapping_add(r8)) as u64; fl_l = rbx; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 8; continue; }
            8 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141675350(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143932060; const P1: u64 = 0x1426469c8; const P2: u64 = 0x143932058; const P3: u64 = 0x143932050;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x28u64)) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = ((rcx.wrapping_add(0x2u64)) as u32) as u32 as u64; rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
            2 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx); rbx = (0xffffffffffffffffu64) as u64; rax = rbx; bb = 3; continue; }
            3 => { rax = ((rax).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru16(rcx.wrapping_add(rax.wrapping_mul(2)))) as u16 as u64; fl_r = (0x0u16) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 3; continue; } bb = 4; continue; }
            4 => { fl_l = rax; fl_r = (0x20u64) as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 19; continue; } bb = 5; continue; }
            5 => { fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 6; continue; }
            6 => { rdx = ((crate::simd_ops::ru16(rcx.wrapping_add(rax.wrapping_mul(2)).wrapping_sub(0x2u64))) as u32) as u32 as u64; fl_l = ((rdx as u16)) as u16 as u64; fl_r = (0x2fu16) as u16 as u64; if (((fl_l as u16)) == ((fl_r as u16))) { bb = 19; continue; } bb = 7; continue; }
            7 => { fl_l = ((rdx as u16)) as u16 as u64; fl_r = (0x5cu16) as u16 as u64; if (((fl_l as u16)) == ((fl_r as u16))) { bb = 19; continue; } bb = 8; continue; }
            8 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rax = rbx; bb = 9; continue; }
            9 => { rax = ((rax).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru16(rcx.wrapping_add(rax.wrapping_mul(2)))) as u16 as u64; fl_r = (0x0u16) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 9; continue; } bb = 10; continue; }
            10 => { rax = ((rax).wrapping_add(1)) as u64; fl_l = rax; fl_r = (0x1fu64) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 13; continue; } bb = 11; continue; }
            11 => { rbx = 0x1f; bb = 15; continue; }
            12 => { bb = 13; continue; }
            13 => { rbx = ((rbx).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru16(rcx.wrapping_add(rbx.wrapping_mul(2)))) as u16 as u64; fl_r = (0x0u16) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 13; continue; } bb = 14; continue; }
            14 => { fl_l = ((rbx).wrapping_add(1)) as u64; rbx = ((fl_l as u64)) as u64; bb = 15; continue; }
            15 => { r8 = rcx; rdi = P0; rcx = rdi; r9 = rbx; rdx = 0x20; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu16(rdi.wrapping_add(rbx.wrapping_mul(2)), ((rax as u16)) as u16); rbx = (crate::simd_ops::ru32(P2)) as u32 as u64; fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 18; continue; } bb = 16; continue; }
            16 => { bb = 17; continue; }
            17 => { rbx = (((rbx as u32)).wrapping_sub(1)) as u32 as u64; rcx = rdi; rax = ((rbx as u32)) as u32 as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(P3))) as u64; rdx = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 17; continue; } bb = 18; continue; }
            18 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rax = 0x1; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
            19 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = 0x2; rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141675460(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141674ae0; const P1: u64 = 0x1416755a0; const P2: u64 = 0x141678900;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; r14 = rcx.wrapping_add(0x8); rcx = (crate::simd_ops::ru32(rcx.wrapping_add(0x10u64))) as u32 as u64; rdi = ((rdx as u32)) as u32 as u64; r9 = ((rcx.wrapping_add(rcx.wrapping_mul(2))) as u64) as u64; rcx = ((rax.wrapping_add(r9.wrapping_mul(8))) as u64) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 4; continue; } bb = 1; continue; }
            1 => { bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru32(rax)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = ((rax).wrapping_add(0x18u64)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 4; continue; }
            4 => { rbp = 0; fl_l = 0; fl_r = 0; rsi = rax.wrapping_add(0x8); fl_l = rax; fl_r = rcx; rsi = (if (((fl_l as u64)) == ((fl_r as u64))) { rbp } else { rsi }) as u64; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 23; continue; } bb = 5; continue; }
            5 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx); rbx = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; rdx = (crate::simd_ops::ru32(rbx.wrapping_add(0x20u64))) as u32 as u64; rdx = ((rdx).wrapping_add(rax)) as u64; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 13; continue; } bb = 6; continue; }
            6 => { bb = 7; continue; }
            7 => { fl_l = (crate::simd_ops::ru8(rax)) as u8 as u64; fl_r = ((r8 as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 10; continue; } bb = 8; continue; }
            8 => { rax = ((rax).wrapping_add(1)) as u64; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 7; continue; } bb = 9; continue; }
            9 => { bb = 13; continue; }
            10 => { rdx = ((rdx).wrapping_sub(1)) as u64; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 12; continue; } bb = 11; continue; }
            11 => { rcx = ((crate::simd_ops::ru8(rdx)) as u32) as u32 as u64; crate::simd_ops::wu8(rax, ((rcx as u8)) as u8); bb = 12; continue; }
            12 => { fl_l = (crate::simd_ops::lk_sub_u32(rbx.wrapping_add(0x20u64), (1) as u32)) as u32 as u64; bb = 13; continue; }
            13 => { fl_l = (crate::simd_ops::lk_add_u32(rbx.wrapping_add(0x8u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 19; continue; } bb = 14; continue; }
            14 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 16; continue; } bb = 15; continue; }
            15 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx, rbp); bb = 16; continue; }
            16 => { rcx = rsi; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rdx = ((rdi as u32)) as u32 as u64; rcx = r14; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rax = 0x1; bb = 17; continue; }
            17 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; bb = 18; continue; }
            18 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            19 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; fl_l = rax; fl_r = rax; r10 = (crate::simd_ops::ru32(rbx.wrapping_add(0x20u64))) as u32 as u64; rcx = rax.wrapping_sub(0x80); rcx = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rbp } else { rcx }) as u64; r9 = (r9 & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; r10 = ((r10).wrapping_add(rdx)) as u64; fl_l = rdx; fl_r = r10; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 22; continue; } bb = 20; continue; }
            20 => { bb = 21; continue; }
            21 => { r8 = ((crate::simd_ops::ru8(rdx)) as u32) as u32 as u64; fl_l = ((r8 as u8)) as u8 as u64; fl_r = ((r9 as u8)) as u8 as u64; rax = (((r9 as u8)) as u32) as u32 as u64; r8 = (if ((((fl_l as u8)) as i8) <= (((fl_r as u8)) as i8)) { (rax as u32) } else { (r8 as u32) }) as u32 as u64; rdx = ((rdx).wrapping_add(1)) as u64; r9 = (((r8 as u8)) as u32) as u32 as u64; fl_l = rdx; fl_r = r10; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 21; continue; } bb = 22; continue; }
            22 => { rdx = rcx; r8 = (((r9 as u8)) as u32) as u32 as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x60u64))) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rax = 0x2; bb = 17; continue; }
            23 => { rax = 0x2; bb = 18; continue; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416755a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xffffffffffffffff; const P2: u64 = 0x18; const P3: u64 = 0x14183acc8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; r8 = (crate::simd_ops::ru64(rcx)) as u64; rbx = rcx; rcx = r8; r9 = (crate::simd_ops::ru32(rbx.wrapping_add(P0))) as u32 as u64; rax = ((r9.wrapping_add(r9.wrapping_mul(2))) as u64) as u64; rax = ((r8.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = r8; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 7; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru32(rcx)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 4; continue; } bb = 2; continue; }
            2 => { rcx = ((rcx).wrapping_add(0x18u64)) as u64; fl_l = rcx; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 3; continue; }
            3 => { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            4 => { rax = ((r9.wrapping_add(P1)) as u32) as u32 as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rdx = ((r8.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = rcx; fl_r = rdx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rdx = ((rdx).wrapping_sub(rcx)) as u64; rax = (0x2aaaaaaaaaaaaaabu64) as u64; let prod = (rax as i64 as i128).wrapping_mul((rdx) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rdx = ((((rdx) as i64).wrapping_shr(2u32) as u64)) as u64; rax = rdx; rax = ((rax).wrapping_shr(63u32)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; r8 = ((rdx.wrapping_add(rdx.wrapping_mul(2))) as u32) as u32 as u64; r8 = (((r8 as u32)).wrapping_shl(3u32)) as u32 as u64; rdx = ((rcx.wrapping_add(P2)) as u64) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            6 => { fl_l = (crate::simd_ops::lk_sub_u32(rbx.wrapping_add(P0), (1) as u32)) as u32 as u64; bb = 7; continue; }
            7 => { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141675620(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x18; const P3: u64 = 0x20; const P4: u64 = 0xffffffffffffff80; const P5: u64 = 0x141678900;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; r11 = (((r8 as u8)) as u32) as u32 as u64; rcx = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64; r10 = ((rcx.wrapping_add(rcx.wrapping_mul(2))) as u64) as u64; rcx = ((rax.wrapping_add(r10.wrapping_mul(8))) as u64) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 4; continue; } bb = 1; continue; }
            1 => { bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru32(rax)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = ((rax).wrapping_add(0x18u64)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 4; continue; }
            4 => { rbx = 0; fl_l = 0; fl_r = 0; rdx = ((rax.wrapping_add(P0)) as u64) as u64; fl_l = rax; fl_r = rcx; rdx = (if (((fl_l as u64)) == ((fl_r as u64))) { rbx } else { rdx }) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 5; continue; }
            5 => { r10 = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; rcx = (crate::simd_ops::ru64(r10.wrapping_add(P2))) as u64; r8 = (crate::simd_ops::ru32(r10.wrapping_add(P3))) as u32 as u64; r8 = ((r8).wrapping_add(rcx)) as u64; fl_l = rcx; fl_r = r8; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 8; continue; } bb = 6; continue; }
            6 => { fl_l = (crate::simd_ops::ru8(rcx)) as u8 as u64; fl_r = ((r9 as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 9; continue; } bb = 7; continue; }
            7 => { rcx = ((rcx).wrapping_add(1)) as u64; fl_l = rcx; fl_r = r8; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 6; continue; } bb = 8; continue; }
            8 => { rax = 0x2; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            9 => { crate::simd_ops::wu8(rcx, ((r11 as u8)) as u8); rax = (crate::simd_ops::ru64(r10)) as u64; fl_l = rax; fl_r = rax; r11 = ((rax.wrapping_add(P4)) as u64) as u64; rax = (crate::simd_ops::ru64(r10.wrapping_add(P2))) as u64; r10 = (crate::simd_ops::ru32(r10.wrapping_add(P3))) as u32 as u64; r11 = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rbx } else { r11 }) as u64; r10 = ((r10).wrapping_add(rax)) as u64; r9 = (r9 & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; fl_l = rax; fl_r = r10; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 12; continue; } bb = 10; continue; }
            10 => { bb = 11; continue; }
            11 => { r8 = ((crate::simd_ops::ru8(rax)) as u32) as u32 as u64; fl_l = ((r8 as u8)) as u8 as u64; fl_r = ((r9 as u8)) as u8 as u64; rdx = (((r9 as u8)) as u32) as u32 as u64; r8 = (if ((((fl_l as u8)) as i8) <= (((fl_r as u8)) as i8)) { (rdx as u32) } else { (r8 as u32) }) as u32 as u64; rax = ((rax).wrapping_add(1)) as u64; r9 = (((r8 as u8)) as u32) as u32 as u64; fl_l = rax; fl_r = r10; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 11; continue; } bb = 12; continue; }
            12 => { rcx = (crate::simd_ops::ru64(r11.wrapping_add(0x60u64))) as u64; r8 = (((r9 as u8)) as u32) as u32 as u64; rdx = r11; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rax = 0x1; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141675750(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14167c230; const P1: u64 = 0x142693bf8; const P2: u64 = 0x30; const P3: u64 = 0x20; const P4: u64 = 0x28; const P5: u64 = 0x1426452b8; const P6: u64 = 0x142693d38; const P7: u64 = 0x68; const P8: u64 = 0x142693dc8; const P9: u64 = 0x80; const P10: u64 = 0x74; const P11: u64 = 0x77;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rcx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rdi = 0; fl_l = 0; fl_r = 0; rax = P1; rcx = ((rbx.wrapping_add(P2)) as u64) as u64; crate::simd_ops::wu64(rbx, rax); crate::simd_ops::wu64(rbx.wrapping_add(P3), rdi); crate::simd_ops::wu64(rbx.wrapping_add(P4), rdi); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); rax = P6; crate::simd_ops::wu64(rbx.wrapping_add(P7), rdi); crate::simd_ops::wu64(rbx, rax); rax = P8; crate::simd_ops::wu64(rbx.wrapping_add(P9), rax); rax = rbx; crate::simd_ops::wu16(rbx.wrapping_add(P10), (0x132u16) as u16); crate::simd_ops::wu32(rbx.wrapping_add(P11), ((rdi as u32)) as u32); crate::simd_ops::wu16(rbx.wrapping_add(0x7bu64), ((rdi as u16)) as u16); crate::simd_ops::wu32(rbx.wrapping_add(0x90u64), (0xffffffffu32) as u32); crate::simd_ops::wu32(rbx.wrapping_add(0x98u64), ((rdi as u32)) as u32); crate::simd_ops::wu32(rbx.wrapping_add(0xa8u64), ((rdi as u32)) as u32); crate::simd_ops::wu32(rbx.wrapping_add(0xb8u64), ((rdi as u32)) as u32); crate::simd_ops::wu64(rbx.wrapping_add(0xc0u64), rdi); crate::simd_ops::wu64(rbx.wrapping_add(0xc8u64), rdi); crate::simd_ops::wu8(rbx.wrapping_add(0xd0u64), ((rdi as u8)) as u8); crate::simd_ops::wu16(rbx.wrapping_add(0xd2u64), ((rdi as u16)) as u16); crate::simd_ops::wu8(rbx.wrapping_add(0xd4u64), ((rdi as u8)) as u8); crate::simd_ops::wu8(rbx.wrapping_add(0x76u64), ((rdi as u8)) as u8); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141675800(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x142693e58; const P1: u64 = 0x18; const P2: u64 = 0x30; const P3: u64 = 0x38; const P4: u64 = 0x40; const P5: u64 = 0x1c; const P6: u64 = 0x20; const P7: u64 = 0x28;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = P0;
    crate::simd_ops::wu32(rcx.wrapping_add(P1), (0x1u32) as u32);
    crate::simd_ops::wu64(rcx, rax);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rcx.wrapping_add(P2), rax);
    crate::simd_ops::wu16(rcx.wrapping_add(P3), ((rax as u16)) as u16);
    crate::simd_ops::wu64(rcx.wrapping_add(P4), rax);
    rax = rcx;
    crate::simd_ops::wu32(rcx.wrapping_add(P5), (0x1u32) as u32);
    crate::simd_ops::wu64(rcx.wrapping_add(P6), rdx);
    crate::simd_ops::wu64(rcx.wrapping_add(P7), r8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141675840(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14167c250; const P1: u64 = 0x142693ba0; const P2: u64 = 0x98; const P3: u64 = 0x80; const P4: u64 = 0x90; const P5: u64 = 0x1426452b8; const P6: u64 = 0xc0; const P7: u64 = 0x14167cbb0; const P8: u64 = 0x428; const P9: u64 = 0x43c; const P10: u64 = 0x440;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rdx;
    rdi = rcx;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rax = P1;
    crate::simd_ops::wu64(rdi, rax);
    rcx = ((rdi.wrapping_add(P2)) as u64) as u64;
    crate::simd_ops::wu64(rdi.wrapping_add(P3), (0x0u64) as u64);
    crate::simd_ops::wu64(rdi.wrapping_add(P4), (0x0u64) as u64);
    rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp);
    rcx = ((rdi.wrapping_add(P6)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp);
    crate::simd_ops::wu64(rdi.wrapping_add(P8), rbx);
    rax = rdi;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    crate::simd_ops::wu32(rdi.wrapping_add(P9), (0xffffffffu32) as u32);
    crate::simd_ops::wu32(rdi.wrapping_add(P10), (0x0u32) as u32);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416758c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14167c230; const P1: u64 = 0x142693bf8; const P2: u64 = 0x30; const P3: u64 = 0x20; const P4: u64 = 0x28; const P5: u64 = 0x1426452b8; const P6: u64 = 0x142693c60; const P7: u64 = 0x68; const P8: u64 = 0x142693cc8; const P9: u64 = 0x80; const P10: u64 = 0x74; const P11: u64 = 0x77;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rcx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rdi = 0; fl_l = 0; fl_r = 0; rax = P1; rcx = ((rbx.wrapping_add(P2)) as u64) as u64; crate::simd_ops::wu64(rbx, rax); crate::simd_ops::wu64(rbx.wrapping_add(P3), rdi); crate::simd_ops::wu64(rbx.wrapping_add(P4), rdi); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); rax = P6; crate::simd_ops::wu64(rbx.wrapping_add(P7), rdi); crate::simd_ops::wu64(rbx, rax); rax = P8; crate::simd_ops::wu64(rbx.wrapping_add(P9), rax); rax = rbx; crate::simd_ops::wu16(rbx.wrapping_add(P10), (0x32u16) as u16); crate::simd_ops::wu32(rbx.wrapping_add(P11), ((rdi as u32)) as u32); crate::simd_ops::wu16(rbx.wrapping_add(0x7bu64), ((rdi as u16)) as u16); crate::simd_ops::wu64(rbx.wrapping_add(0x88u64), rdi); crate::simd_ops::wu64(rbx.wrapping_add(0x90u64), rdi); crate::simd_ops::wu64(rbx.wrapping_add(0x98u64), rdi); crate::simd_ops::wu64(rbx.wrapping_add(0xa0u64), rdi); crate::simd_ops::wu32(rbx.wrapping_add(0xa8u64), (0xffffffffu32) as u32); crate::simd_ops::wu16(rbx.wrapping_add(0xacu64), ((rdi as u16)) as u16); crate::simd_ops::wu32(rbx.wrapping_add(0xb0u64), ((rdi as u32)) as u32); crate::simd_ops::wu32(rbx.wrapping_add(0xb8u64), ((rdi as u32)) as u32); crate::simd_ops::wu8(rbx.wrapping_add(0xbcu64), (0x1u8) as u8); crate::simd_ops::wu8(rbx.wrapping_add(0x76u64), ((rdi as u8)) as u8); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416759c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142693ba0; const P1: u64 = 0x14167cc00; const P2: u64 = 0x98; const P3: u64 = 0x1426452a0; const P4: u64 = 0x14167c2d0;
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rax = P0;
    rbx = rcx;
    crate::simd_ops::wu64(rcx, rax);
    rcx = ((rcx).wrapping_add(0xc0u64)) as u64;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rcx = ((rbx.wrapping_add(P2)) as u64) as u64;
    rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp);
    rcx = rbx;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(P4, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141675a50(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x79; const P1: u64 = 0x142693bf8; const P2: u64 = 0x60; const P3: u64 = 0x28; const P4: u64 = 0x428; const P5: u64 = 0x8; const P6: u64 = 0x20; const P7: u64 = 0x1416747b0; const P8: u64 = 0x68; const P9: u64 = 0x1415242e0; const P10: u64 = 0x30; const P11: u64 = 0x1426452a0; const P12: u64 = 0x14167c2a0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(P0))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; rax = P1; crate::simd_ops::wu64(rcx, rax); rbx = rcx; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64; rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P3))) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(P4))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P5))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P6))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(P6), (0x0u64) as u64); bb = 4; continue; }
            4 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P8))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rcx = 0x7; rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            6 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P3))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { rcx = 0x7; rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); bb = 8; continue; }
            8 => { rcx = ((rbx.wrapping_add(P10)) as u64) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P11), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P12, rax, rcx, rdx, r8, r9, rsp); }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141675b00(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7a; const P1: u64 = 0x142693d38; const P2: u64 = 0x142693dc8; const P3: u64 = 0x80; const P4: u64 = 0x60; const P5: u64 = 0x14167c3a0; const P6: u64 = 0x142693b10; const P7: u64 = 0x141675a50; const P8: u64 = 0x141834d30;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(P0))) as u8 as u64; fl_r = (0x0u8) as u8 as u64;
    rax = P1;
    crate::simd_ops::wu64(rcx, rax);
    rdi = ((rdx as u32)) as u32 as u64;
    rax = P2;
    rbx = rcx;
    crate::simd_ops::wu64(rcx.wrapping_add(P3), rax);
    if !((((fl_l as u8)) == ((fl_r as u8)))) { rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P4))) as u64; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); }
    rax = P6;
    rcx = rbx;
    crate::simd_ops::wu64(rbx.wrapping_add(P3), rax);
    rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp);
    fl_l = ((rdi as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64;
    if !(((((fl_l as u8)) & ((fl_r as u8))) == 0)) { rdx = 0xd8; rcx = rbx; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); }
    rax = rbx;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141675b70(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142693e58; const P1: u64 = 0x40; const P2: u64 = 0x1415242e0; const P3: u64 = 0x142693b10; const P4: u64 = 0x141834d30;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdi = ((rdx as u32)) as u32 as u64;
    rax = P0;
    rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64;
    rbx = rcx;
    crate::simd_ops::wu64(rcx, rax);
    fl_l = rdx; fl_r = rdx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rcx = 0x7; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(P1), (0x0u64) as u64); }
    rax = P3;
    crate::simd_ops::wu64(rbx, rax);
    fl_l = ((rdi as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64;
    if !(((((fl_l as u8)) & ((fl_r as u8))) == 0)) { rdx = 0x48; rcx = rbx; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); }
    rax = rbx;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141675bd0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142693ba0; const P1: u64 = 0x14167cc00; const P2: u64 = 0x98; const P3: u64 = 0x1426452a0; const P4: u64 = 0x14167c2d0; const P5: u64 = 0x141834d30;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rax = P0;
    rdi = rcx;
    crate::simd_ops::wu64(rcx, rax);
    rbx = ((rdx as u32)) as u32 as u64;
    rcx = ((rcx).wrapping_add(0xc0u64)) as u64;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rcx = ((rdi.wrapping_add(P2)) as u64) as u64;
    rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp);
    rcx = rdi;
    rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp);
    fl_l = ((rbx as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64;
    if !(((((fl_l as u8)) & ((fl_r as u8))) == 0)) { rdx = 0x448; rcx = rdi; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rax = rdi;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141675c30(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7a; const P1: u64 = 0x142693c60; const P2: u64 = 0x142693cc8; const P3: u64 = 0x80; const P4: u64 = 0x60; const P5: u64 = 0x14167c7e0; const P6: u64 = 0x14268dac0; const P7: u64 = 0x141675a50; const P8: u64 = 0x141834d30;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(P0))) as u8 as u64; fl_r = (0x0u8) as u8 as u64;
    rax = P1;
    crate::simd_ops::wu64(rcx, rax);
    rdi = ((rdx as u32)) as u32 as u64;
    rax = P2;
    rbx = rcx;
    crate::simd_ops::wu64(rcx.wrapping_add(P3), rax);
    if !((((fl_l as u8)) == ((fl_r as u8)))) { rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P4))) as u64; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); }
    rax = P6;
    rcx = rbx;
    crate::simd_ops::wu64(rbx.wrapping_add(P3), rax);
    rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp);
    fl_l = ((rdi as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64;
    if !(((((fl_l as u8)) & ((fl_r as u8))) == 0)) { rdx = 0xc0; rcx = rbx; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); }
    rax = rbx;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141675d10(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x78; const P1: u64 = 0xd3; const P2: u64 = 0x14; const P3: u64 = 0xc8; const P4: u64 = 0xb8; const P5: u64 = 0xc0; const P6: u64 = 0x60; const P7: u64 = 0x18; const P8: u64 = 0x1426452b0; const P9: u64 = 0x1416762e0; const P10: u64 = 0x1426452a8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = rdx; rdi = rcx; fl_l = ((r8 as u8)) as u8 as u64; fl_r = ((r8 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 6; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(P0))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 6; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(P1))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 6; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru32(rdx.wrapping_add(P2))) as u32 as u64; rax = ((((rax as u32)) & (0xfffffffbu32))) as u32 as u64; crate::simd_ops::wu64(rdx, (0x0u64) as u64); rax = ((((rax as u32)) | (0x3u32))) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_add(P2), ((rax as u32)) as u32); fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(P3))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 5; continue; } bb = 4; continue; }
            4 => { crate::simd_ops::lk_add_u32(rcx.wrapping_add(P4), (1) as u32); crate::simd_ops::wu64(rcx.wrapping_add(P3), rdx); crate::simd_ops::wu64(rcx.wrapping_add(P5), rdx); rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            5 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(P5))) as u64; crate::simd_ops::wu64(rax, rsi); crate::simd_ops::lk_add_u32(rcx.wrapping_add(P4), (1) as u32); crate::simd_ops::wu64(rcx.wrapping_add(P5), rsi); rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            6 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx); rbx = (crate::simd_ops::ru64(rcx.wrapping_add(P6))) as u64; rcx = ((rbx.wrapping_add(P7)) as u64) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P8), rax), rax, rcx, rdx, r8, r9, rsp); rdx = rsi; rcx = rdi; rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); rcx = ((rbx.wrapping_add(P7)) as u64) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P10), rax), rax, rcx, rdx, r8, r9, rsp); }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141675dd0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x78; const P1: u64 = 0xb8; const P2: u64 = 0x77; const P3: u64 = 0xbc; const P4: u64 = 0x8; const P5: u64 = 0x18; const P6: u64 = 0x60; const P7: u64 = 0x10; const P8: u64 = 0x430; const P9: u64 = 0x76; const P10: u64 = 0x28; const P11: u64 = 0xa0; const P12: u64 = 0x1426452b0; const P13: u64 = 0x14167d050; const P14: u64 = 0x1415242e0; const P15: u64 = 0x1426452a8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = rdx; rbx = rcx; fl_l = ((r8 as u8)) as u8 as u64; fl_r = ((r8 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 8; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(P0))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 8; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64; fl_r = 0x4; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 8; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu8(rcx.wrapping_add(P2), (0x0u8) as u8); crate::simd_ops::wu8(rcx.wrapping_add(P3), (0x1u8) as u8); rax = (crate::simd_ops::ru64(rdx.wrapping_add(P4))) as u64; rdx = (crate::simd_ops::ru32(rax.wrapping_add(P5))) as u32 as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P6))) as u64; rdx = (((rdx as u32)).wrapping_sub(crate::simd_ops::ru32(rdi.wrapping_add(P7)))) as u32 as u64; rcx = (crate::simd_ops::ru32(rax.wrapping_add(P8))) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rdx = ((rcx as u32)) as u32 as u64; crate::simd_ops::wu8(rbx.wrapping_add(P3), (0x0u8) as u8); bb = 5; continue; }
            5 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(P9))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 8; continue; } bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(P7))) as u32 as u64; r8 = (crate::simd_ops::ru64(rbx.wrapping_add(P10))) as u64; rcx = (crate::simd_ops::ru64(r8)) as u64; rcx = ((rcx).wrapping_sub(rax)) as u64; rax = (crate::simd_ops::ru64(rdi.wrapping_add(P4))) as u64; rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(rax))) as u64; rax = ((rdx as u32)) as u32 as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 8; continue; } bb = 7; continue; }
            7 => { crate::simd_ops::wu8(rbx.wrapping_add(P2), (0x1u8) as u8); crate::simd_ops::wu8(rbx.wrapping_add(P3), (0x1u8) as u8); rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P4))) as u64; rax = (crate::simd_ops::ru32(r8)) as u32 as u64; rax = (((rax as u32)).wrapping_sub(crate::simd_ops::ru32(rcx.wrapping_add(P5)))) as u32 as u64; rax = (((rax as u32)).wrapping_sub(crate::simd_ops::ru32(rcx))) as u32 as u64; fl_l = (crate::simd_ops::lk_add_u32(rbx.wrapping_add(P11), ((rax as u32)) as u32)) as u32 as u64; fl_r = fl_l; bb = 8; continue; }
            8 => { rsi = (crate::simd_ops::ru64(rbx.wrapping_add(0x60u64))) as u64; rcx = rsi.wrapping_add(0x18); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P12), rax), rax, rcx, rdx, r8, r9, rsp); r8 = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x60u64))) as u64; crate::simd_ops::wu64(rdi.wrapping_add(0x8u64), (0x0u64) as u64); fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { rdx = rbx.wrapping_add(0x88); rcx = ((rcx).wrapping_add(0xc0u64)) as u64; rax = crate::simd_ops::call_stk(P13, rax, rcx, rdx, r8, r9, rsp); bb = 10; continue; }
            10 => { rdx = rdi; rcx = 0x7; rax = crate::simd_ops::call_stk(P14, rax, rcx, rdx, r8, r9, rsp); rcx = rsi.wrapping_add(0x18); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P15), rax), rax, rcx, rdx, r8, r9, rsp); }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141675ec0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x78; const P1: u64 = 0x141675f10; const P2: u64 = 0x88;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdx = ((rcx.wrapping_add(P0)) as u64) as u64;
    rdi = rcx;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rdx = ((rdi.wrapping_add(P2)) as u64) as u64;
    rcx = rdi;
    rbx = (((rax as u8)) as u32) as u32 as u64;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    fl_l = ((rbx as u8)) as u8 as u64; fl_r = ((rbx as u8)) as u8 as u64;
    if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64;
    if !(((((fl_l as u8)) & ((fl_r as u8))) == 0)) { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rax = (rax & !0xffu64) | (((0u8) as u8 as u64));
    fl_l = 0; fl_r = 0;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141675f10(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x78; const P2: u64 = 0x18; const P3: u64 = 0x1415242e0; const P4: u64 = 0x10;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx); rbp = rdx; rbx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rdi); rdi = 0; fl_l = 0; fl_r = 0; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), r14); bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(P1))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; r14 = rbx; rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 9; continue; } bb = 3; continue; }
            3 => { rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 8; continue; } bb = 4; continue; }
            4 => { fl_l = rbx; fl_r = (crate::simd_ops::ru64(rbp.wrapping_add(0x8u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { crate::simd_ops::wu64(rbp.wrapping_add(0x8u64), rcx); bb = 7; continue; }
            6 => { crate::simd_ops::wu64(rdi.wrapping_add(P2), rcx); bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru64(r14)) as u64; rbx = rcx; rdx = 0; fl_l = 0; fl_r = 0; rcx = r14; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rdx = r14; rcx = 0x7; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 10; continue; }
            8 => { rdi = rbx; rbx = rcx; bb = 10; continue; }
            9 => { rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P4))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rdi = rbx; rbx = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64; bb = 10; continue; }
            10 => { fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 11; continue; }
            11 => { r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; bb = 12; continue; }
            12 => { fl_l = (crate::simd_ops::ru64(rbp.wrapping_add(0x8u64))) as u64; fl_r = (0x0u64) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 14; continue; } bb = 13; continue; }
            13 => { crate::simd_ops::wu64(rbp.wrapping_add(0x8u64), (0x0u64) as u64); rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            14 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141675fd0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x40; const P1: u64 = 0x98; const P2: u64 = 0x1426452b0; const P3: u64 = 0x80; const P4: u64 = 0x18; const P5: u64 = 0x1426452a8;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x30u64)) as u64;
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64;
    rsi = rcx;
    r10 = (crate::simd_ops::ru64(rcx)) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax);
    rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(r10.wrapping_add(P0))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp);
    rdi = rax;
    fl_l = rax; fl_r = rax;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    rcx = ((rsi.wrapping_add(P1)) as u64) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rbx);
    rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp);
    rax = (crate::simd_ops::ru64(rsi.wrapping_add(P3))) as u64;
    fl_l = rax; fl_r = rax;
    if !(((((fl_l as u64)) & ((fl_r as u64))) != 0)) { crate::simd_ops::wu64(rsi.wrapping_add(P3), rdi); crate::simd_ops::wu64(rdi.wrapping_add(P4), rax); } else { crate::simd_ops::wu64(rdi.wrapping_add(P4), rax); crate::simd_ops::wu64(rsi.wrapping_add(P3), rdi); }
    rcx = ((rsi.wrapping_add(P1)) as u64) as u64;
    rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64;
    rax = rdi;
    rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64;
    rsp = ((rsp).wrapping_add(0x30u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141676060(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x40; const P1: u64 = 0x7b; const P2: u64 = 0x38; const P3: u64 = 0x98; const P4: u64 = 0x1426452b0; const P5: u64 = 0x90; const P6: u64 = 0x18; const P7: u64 = 0x1426452a8;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r11 = rsp; crate::simd_ops::wu64(r11.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(r11.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(r11.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rax = ((crate::simd_ops::ru8(rsp.wrapping_add(0x70u64))) as u32) as u32 as u64; xmm0 = 0u128; r10 = (crate::simd_ops::ru64(rcx)) as u64; rbp = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((xmm0 as u32)) as u32); rbx = ((r9 as u32)) as u32 as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x3cu64), (0x0u8) as u8); r9 = r11.wrapping_sub(0x18); crate::simd_ops::wu64(r11.wrapping_sub(0x14u64), rbp); rsi = rcx; crate::simd_ops::wu8(rsp.wrapping_add(0x3du64), ((rax as u8)) as u8); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; crate::simd_ops::wu64(r11.wrapping_sub(0x20u64), rax); crate::simd_ops::wu64(r11.wrapping_sub(0x28u64), rbp); (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(r10.wrapping_add(P0))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 1; continue; }
            1 => { r8 = (crate::simd_ops::ru64(rax)) as u64; rdx = ((rbx as u32)) as u32 as u64; rcx = rax; crate::simd_ops::wu8(rax.wrapping_add(P1), (0x1u8) as u8); (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(r8.wrapping_add(P2))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = ((rsi.wrapping_add(P3)) as u64) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rsi.wrapping_add(P5))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rsi.wrapping_add(P5), rdi); crate::simd_ops::wu64(rdi.wrapping_add(P6), rbp); bb = 4; continue; }
            3 => { crate::simd_ops::wu64(rdi.wrapping_add(P6), rax); crate::simd_ops::wu64(rsi.wrapping_add(P5), rdi); bb = 4; continue; }
            4 => { rcx = ((rsi.wrapping_add(P3)) as u64) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P7), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = rdi; bb = 5; continue; }
            5 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141676120(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x38; const P1: u64 = 0x98; const P2: u64 = 0x1426452b0; const P3: u64 = 0x80; const P4: u64 = 0x18; const P5: u64 = 0x1426452a8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    rsi = rcx;
    rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp);
    rdi = rax;
    fl_l = rax; fl_r = rax;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    rcx = ((rsi.wrapping_add(P1)) as u64) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx);
    rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp);
    rax = (crate::simd_ops::ru64(rsi.wrapping_add(P3))) as u64;
    fl_l = rax; fl_r = rax;
    if !(((((fl_l as u64)) & ((fl_r as u64))) != 0)) { crate::simd_ops::wu64(rsi.wrapping_add(P3), rdi); crate::simd_ops::wu64(rdi.wrapping_add(P4), rax); } else { crate::simd_ops::wu64(rdi.wrapping_add(P4), rax); crate::simd_ops::wu64(rsi.wrapping_add(P3), rdi); }
    rcx = ((rsi.wrapping_add(P1)) as u64) as u64;
    rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rax = rdi;
    rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416761a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1426452b0; const P1: u64 = 0x48; const P2: u64 = 0xfffffffffffffff8; const P3: u64 = 0xfffffffffffffffc; const P4: u64 = 0x50; const P5: u64 = 0x28; const P6: u64 = 0x8; const P7: u64 = 0x10; const P8: u64 = 0x20; const P9: u64 = 0x18; const P10: u64 = 0xffffffffffffff80; const P11: u64 = 0x141676780; const P12: u64 = 0x1426452a8;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = rcx; rcx = ((rcx).wrapping_add(0xffffffffffffffb0u64)) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rdi.wrapping_add(P1))) as u64; crate::simd_ops::wu8(rdi.wrapping_add(P2), (0x1u8) as u8); crate::simd_ops::wu8(rdi.wrapping_add(P3), (0x0u8) as u8); r9 = ((crate::simd_ops::ru8(rdi.wrapping_add(P4))) as u32) as u32 as u64; fl_l = ((r9 as u8)) as u8 as u64; fl_r = ((r9 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 8; continue; } bb = 1; continue; }
            1 => { r11 = (crate::simd_ops::ru32(rdi.wrapping_add(P5))) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rsi); bb = 2; continue; }
            2 => { rdx = (crate::simd_ops::ru64(rax.wrapping_add(P6))) as u64; rsi = (crate::simd_ops::ru32(rax.wrapping_add(P7))) as u32 as u64; rcx = (crate::simd_ops::ru32(rdi.wrapping_add(P8))) as u32 as u64; r10 = (crate::simd_ops::ru64(rdx)) as u64; r8 = ((rdx.wrapping_add(P9)) as u64) as u64; rdx = ((r10.wrapping_add(rsi)) as u64) as u64; fl_l = rdx; fl_r = rcx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 5; continue; } bb = 3; continue; }
            3 => { rdx = (crate::simd_ops::ru32(r8)) as u32 as u64; rdx = ((rdx).wrapping_add(r10)) as u64; fl_l = rdx; fl_r = rcx; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rcx = (((rcx as u32)).wrapping_sub((rsi as u32))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((r10 as u32)) as u32 as u64; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 6; continue; }
            5 => { rcx = (crate::simd_ops::ru32(r8)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 6; continue; }
            6 => { r11 = (((r11 as u32)).wrapping_add((rcx as u32))) as u32 as u64; fl_l = (((r9 as u8)).wrapping_add(0xffu8)) as u8 as u64; fl_r = fl_l; r9 = (r9 & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); crate::simd_ops::wu32(rdi.wrapping_add(P5), ((r11 as u32)) as u32); crate::simd_ops::wu8(rdi.wrapping_add(P4), ((r9 as u8)) as u8); rax = (crate::simd_ops::ru64(rax)) as u64; if (((fl_l as u8)) != 0) { bb = 2; continue; } bb = 7; continue; }
            7 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; bb = 8; continue; }
            8 => { rcx = ((rdi.wrapping_add(P10)) as u64) as u64; rax = crate::simd_ops::call_stk(P11, rax, rcx, rdx, r8, r9, rsp); rax = 0; fl_l = 0; fl_r = 0; rcx = rdi.wrapping_sub(0x50); crate::simd_ops::wu64(rdi.wrapping_add(0x48u64), rax); crate::simd_ops::wu64(rdi.wrapping_add(0x40u64), rax); crate::simd_ops::wu32(rdi.wrapping_add(0x38u64), ((rax as u32)) as u32); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P12), rax), rax, rcx, rdx, r8, r9, rsp); }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141676260(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415242e0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    fl_l = rcx; fl_r = rcx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rbx = rcx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rdx = rbx; rcx = 0x7; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); }
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141676290(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14167c860; const P1: u64 = 0xc0; const P2: u64 = 0x14167de10; const P3: u64 = 0x1415242e0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rcx = ((rbx.wrapping_add(P1)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    fl_l = rbx; fl_r = rbx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rdx = rbx; rcx = 0x7; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P3, rax, rcx, rdx, r8, r9, rsp); }
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416762e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xa0; const P1: u64 = 0x10; const P2: u64 = 0x8; const P3: u64 = 0x18; const P4: u64 = 0xa8; const P5: u64 = 0x60; const P6: u64 = 0x14167d920; const P7: u64 = 0x1415242e0;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; r8 = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; rbx = rdx; rdx = (crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32 as u64; r9 = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64; r10 = (crate::simd_ops::ru64(r9)) as u64; rax = ((r10.wrapping_add(rdx)) as u64) as u64; fl_l = rax; fl_r = r8; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 3; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru32(r9.wrapping_add(P3))) as u32 as u64; rax = ((rax).wrapping_add(r10)) as u64; fl_l = rax; fl_r = r8; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 3; continue; } bb = 2; continue; }
            2 => { r8 = (((r8 as u32)).wrapping_sub((rdx as u32))) as u32 as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((r10 as u32)) as u32 as u64; r8 = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 4; continue; }
            3 => { r8 = (crate::simd_ops::ru32(r9.wrapping_add(P3))) as u32 as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; r8 = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 4; continue; }
            4 => { crate::simd_ops::lk_sub_u32(rcx.wrapping_add(P4), ((r8 as u32)) as u32); rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P5))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(P2), (0x0u64) as u64); fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rcx = ((rcx).wrapping_add(0xc0u64)) as u64; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            6 => { rdx = rbx; rcx = 0x7; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P7, rax, rcx, rdx, r8, r9, rsp); }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141676360(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0xa8; const P1: u64 = 0x94;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rax = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64;
    xmm0 = 0u128;
    xmm0 = (xmm0 & !0xffff_ffffu128) | ((((rax) as i64) as f32).to_bits() as u128);
    xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) / f32::from_bits(crate::simd_ops::ru32(rcx.wrapping_add(P1)))).to_bits() as u128);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141676380(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x60; const P1: u64 = 0xa0; const P2: u64 = 0xb0; const P3: u64 = 0xb4; const P4: u64 = 0x430; const P5: u64 = 0x70; const P6: u64 = 0x58; const P7: u64 = 0x1439318d4;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    r9 = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    xmm1 = 0u128;
    rax = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64;
    rdx = 0;
    fl_l = 0; fl_r = 0;
    rax = (((rax as u32)).wrapping_sub(crate::simd_ops::ru32(rcx.wrapping_add(P2)))) as u32 as u64;
    xmm0 = ((crate::simd_ops::ru32(rcx.wrapping_add(P3))) as u32) as u128;
    rax = (((rax as u32)).wrapping_sub(1)) as u32 as u64;
    r8 = (crate::simd_ops::ru32(r9.wrapping_add(P4))) as u32 as u64;
    rax = (((rax as u32)).wrapping_add((r8 as u32))) as u32 as u64;
    let d = ((r8 as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64);
    if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; }
    r8 = ((rax as u32)) as u32 as u64;
    rax = (crate::simd_ops::ru64(r9.wrapping_add(P5))) as u64;
    fl_l = rax; fl_r = (crate::simd_ops::ru64(rcx.wrapping_add(P6))) as u64;
    rax = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64;
    xmm1 = (xmm1 & !0xffff_ffffu128) | ((((r8) as i64) as f32).to_bits() as u128);
    xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) / f32::from_bits(xmm1 as u32)).to_bits() as u128);
    xmm1 = 0u128;
    xmm1 = (xmm1 & !0xffff_ffffu128) | ((((rax) as i64) as f32).to_bits() as u128);
    xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) / f32::from_bits(crate::simd_ops::ru32(P7))).to_bits() as u128);
    xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) - f32::from_bits(xmm1 as u32)).to_bits() as u128);
    xmm1 = 0u128;
    xmm0 = (xmm0 & !0xffff_ffffu128) | ((if f32::from_bits(xmm0 as u32) > f32::from_bits(xmm1 as u32) { f32::from_bits(xmm0 as u32) } else { f32::from_bits(xmm1 as u32) }).to_bits() as u128);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416763e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x20; const P1: u64 = 0x78; const P2: u64 = 0x28; const P3: u64 = 0x141674930; const P4: u64 = 0x18; const P5: u64 = 0x1416747b0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    fl_l = rcx; fl_r = rcx;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rax = 0x1; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(P1))) as u8 as u64; fl_r = (0x0u8) as u8 as u64;
    if !((((fl_l as u8)) != ((fl_r as u8)))) { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rdi); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rdi = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if !((((fl_l as u32)) != ((fl_r as u32)))) { rdx = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rdx.wrapping_add(P4))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); } rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64; fl_l = rcx; fl_r = rcx; if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(P0), (0x0u64) as u64); } rax = ((rdi as u32)) as u32 as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    rax = 0x1;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141676450(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x18; const P1: u64 = 0x1426452b0; const P2: u64 = 0x20; const P3: u64 = 0x1; const P4: u64 = 0x28; const P5: u64 = 0x1426452a8;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x30u64)) as u64;
    rdi = ((rcx.wrapping_add(P0)) as u64) as u64;
    rbx = rdx;
    rcx = rdi;
    rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp);
    rax = (crate::simd_ops::ru64(rbx)) as u64;
    r9 = rsp.wrapping_add(0x40);
    r8 = rsp.wrapping_add(0x48);
    crate::simd_ops::wu8(rsp.wrapping_add(0x20u64), (0x1u8) as u8);
    rdx = rsp.wrapping_add(0x50);
    rcx = rbx;
    rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp);
    r10 = (crate::simd_ops::ru64(rbx)) as u64;
    r9 = 0;
    fl_l = 0; fl_r = 0;
    rdx = rax;
    rcx = rbx;
    r8 = ((r9.wrapping_add(P3)) as u32) as u32 as u64;
    rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(r10.wrapping_add(P4))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp);
    rcx = rdi;
    rbx = (((rax as u8)) as u32) as u32 as u64;
    rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp);
    rax = (((rbx as u8)) as u32) as u32 as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64;
    rsp = ((rsp).wrapping_add(0x30u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416764c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142693ef8; const P1: u64 = 0x142693ee8; const P2: u64 = 0x1415144a0; const P3: u64 = 0x14067b660; const P4: u64 = 0x1426452b0; const P5: u64 = 0x14167c820; const P6: u64 = 0x14167c7e0; const P7: u64 = 0x1426452a8; const P8: u64 = 0x1426456f8; const P9: u64 = 0x14167c780; const P10: u64 = 0x14167c920; const P11: u64 = 0x142693f10;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 21>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0xa0u64)) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0xe8u64))) as u64; rbp = 0; fl_l = 0; fl_r = 0; rax = ((crate::simd_ops::ru8(rsp.wrapping_add(0xd8u64))) as u32) as u32 as u64; rdi = rcx; xmm1 = ((crate::simd_ops::ru32(rsp.wrapping_add(0xe0u64))) as u32) as u128; rsi = ((r9 as u32)) as u32 as u64; crate::simd_ops::wu32(r14, ((rbp as u32)) as u32); crate::simd_ops::wu32(rcx.wrapping_add(0xb4u64), ((xmm1 as u32)) as u32); crate::simd_ops::wu32(rcx.wrapping_add(0xb0u64), ((rbp as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0xbcu64), ((rbp as u8)) as u8); crate::simd_ops::wu8(rcx.wrapping_add(0x76u64), ((rdx as u8)) as u8); crate::simd_ops::wu8(rcx.wrapping_add(0x74u64), ((rax as u8)) as u8); crate::simd_ops::wu64(rcx.wrapping_add(0x90u64), r8); crate::simd_ops::wu32(rcx.wrapping_add(0xa0u64), ((rsi as u32)) as u32); fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { r8 = P0; bb = 2; continue; }
            2 => { r9 = P1; rdx = 0x64; rcx = rsp.wrapping_add(0x30); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r9 = (0xffffffffffffffffu64) as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x28u64), ((rbp as u8)) as u8); r8 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rbp as u32)) as u32); rcx = rsp.wrapping_add(0x30); rdx = ((r9.wrapping_add(0x3u64)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = 0x1f; bb = 23; continue; }
            3 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x64u8) as u8 as u64; if (((fl_l as u8)) > ((fl_r as u8))) { bb = 38; continue; } bb = 4; continue; }
            4 => { xmm0 = 0u128; fl_l = ((xmm1 as u32)) as u32 as u64; fl_r = ((xmm0 as u32)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 38; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru32(rcx.wrapping_add(0xb8u64))) as u32 as u64; rax = (((rax as u32)).wrapping_sub(0x2u32)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xb0u64), rbx); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0xfffffffd; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 37; continue; } bb = 6; continue; }
            6 => { fl_l = ((rdx as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 14; continue; } bb = 7; continue; }
            7 => { rdx = 0; fl_l = 0; fl_r = 0; rax = ((rsi as u32)) as u32 as u64; let d = (crate::simd_ops::ru32(rcx.wrapping_add(0x70u64))) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 37; continue; } bb = 8; continue; }
            8 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0x79u64))) as u8 as u64; fl_r = ((rbp as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 14; continue; } bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x28u64))) as u64; rbx = (crate::simd_ops::ru64(rcx.wrapping_add(0x88u64))) as u64; rcx = (crate::simd_ops::ru64(rax)) as u64; rax = ((rbx.wrapping_add(rsi)) as u64) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 11; continue; } bb = 10; continue; }
            10 => { rax = ((rcx as u32)) as u32 as u64; rsi = ((rbp as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_sub((rbx as u32))) as u32 as u64; fl_l = rbx; fl_r = rcx; rsi = (if (((fl_l as u64)) < ((fl_r as u64))) { (rax as u32) } else { (rsi as u32) }) as u32 as u64; bb = 11; continue; }
            11 => { fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(0x7bu64))) as u8 as u64; fl_r = ((rbp as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 14; continue; } bb = 12; continue; }
            12 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x40u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = ((rsi as u32)) as u32 as u64; rdx = ((rdx).wrapping_add(rbx)) as u64; rcx = ((rax as u32)) as u32 as u64; fl_l = rdx; fl_r = rcx; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 14; continue; } bb = 13; continue; }
            13 => { rsi = ((rax as u32)) as u32 as u64; fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; rsi = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 14; continue; }
            14 => { fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 24; continue; } bb = 15; continue; }
            15 => { rcx = rdi.wrapping_add(0x30); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(rdi.wrapping_add(0xb8u64), (0x1u32) as u32); fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(0x78u64))) as u8 as u64; fl_r = ((rbp as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 19; continue; } bb = 16; continue; }
            16 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 19; continue; } bb = 17; continue; }
            17 => { fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(0x7au64))) as u8 as u64; fl_r = ((rbp as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 21; continue; } bb = 18; continue; }
            18 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x60u64))) as u64; crate::simd_ops::wu8(rdi.wrapping_add(0x7au64), (0x1u8) as u8); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 21; continue; }
            19 => { crate::simd_ops::wu8(rdi.wrapping_add(0x7cu64), ((rbp as u8)) as u8); fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(0x7au64))) as u8 as u64; fl_r = ((rbp as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 21; continue; } bb = 20; continue; }
            20 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x60u64))) as u64; crate::simd_ops::wu8(rdi.wrapping_add(0x7au64), ((rbp as u8)) as u8); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 21; continue; }
            21 => { rcx = rdi.wrapping_add(0x30); crate::simd_ops::wu32(r14, ((rbp as u32)) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P7), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = 0x1; bb = 22; continue; }
            22 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0xb0u64))) as u64; bb = 23; continue; }
            23 => { r11 = rsp.wrapping_add(0xa0); rbp = (crate::simd_ops::ru64(r11.wrapping_add(0x18u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rdi = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; rsp = r11; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            24 => { rcx = rdi.wrapping_add(0x58); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P8), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rdi.wrapping_add(0x30); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru8(rsp.wrapping_add(0xd0u64))) as u8 as u64; fl_r = ((rbp as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 31; continue; } bb = 25; continue; }
            25 => { rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(rdi.wrapping_add(0xb8u64), (0x2u32) as u32); fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(0x78u64))) as u8 as u64; fl_r = ((rbp as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 27; continue; } bb = 26; continue; }
            26 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 28; continue; } bb = 27; continue; }
            27 => { crate::simd_ops::wu8(rdi.wrapping_add(0x7cu64), (0x1u8) as u8); bb = 28; continue; }
            28 => { fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(0x7au64))) as u8 as u64; fl_r = ((rbp as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 30; continue; } bb = 29; continue; }
            29 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x60u64))) as u64; crate::simd_ops::wu8(rdi.wrapping_add(0x7au64), (0x1u8) as u8); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 30; continue; }
            30 => { rcx = rdi.wrapping_add(0x30); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P7), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x60u64))) as u64; rdx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P10, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0xb8u64))) as u32 as u64; fl_r = 0x1; crate::simd_ops::wu32(r14, ((rsi as u32)) as u32); rbp = (rbp & !0xffu64) | (((if (((fl_l as u32)) != ((fl_r as u32))) { 1u8 } else { 0u8 }) as u8 as u64)); fl_l = (((rbp as u32)).wrapping_add(1)) as u32 as u64; rbp = ((fl_l as u32)) as u32 as u64; rax = ((rbp as u32)) as u32 as u64; bb = 22; continue; }
            31 => { crate::simd_ops::wu32(rdi.wrapping_add(0xb8u64), (0x2u32) as u32); fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(0x78u64))) as u8 as u64; fl_r = ((rbp as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 33; continue; } bb = 32; continue; }
            32 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 34; continue; } bb = 33; continue; }
            33 => { crate::simd_ops::wu8(rdi.wrapping_add(0x7cu64), (0x1u8) as u8); bb = 34; continue; }
            34 => { fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(0x7au64))) as u8 as u64; fl_r = ((rbp as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 36; continue; } bb = 35; continue; }
            35 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x60u64))) as u64; crate::simd_ops::wu8(rdi.wrapping_add(0x7au64), (0x1u8) as u8); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 36; continue; }
            36 => { rcx = rdi.wrapping_add(0x30); rbp = 0x1; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P7), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = ((rbp as u32)) as u32 as u64; crate::simd_ops::wu32(r14, ((rsi as u32)) as u32); bb = 22; continue; }
            37 => { rax = 0x2; bb = 22; continue; }
            38 => { r8 = P11; bb = 2; continue; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141676780(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1426452b0; const P1: u64 = 0x14167d920; const P2: u64 = 0x1415242e0; const P3: u64 = 0x1426452a8; const P4: u64 = 0x141678970;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rdi = rcx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x70u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rax = ((crate::simd_ops::ru8(rdi.wrapping_add(0xd0u64))) as u32) as u32 as u64; fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0xb8u64))) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 20; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rbx); rbx = (crate::simd_ops::ru64(rdi.wrapping_add(0xc8u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), r14); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r15); r15 = 0; fl_l = 0; fl_r = 0; r14 = ((r15 as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 4; continue; } bb = 2; continue; }
            2 => { bb = 3; continue; }
            3 => { r14 = rbx; rbx = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rax; fl_r = (0x1u64) as u64; rax = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 4; continue; }
            4 => { rbp = (crate::simd_ops::ru64(rdi.wrapping_add(0x60u64))) as u64; rcx = rbp.wrapping_add(0x18); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 19; continue; } bb = 5; continue; }
            5 => { crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rsi); bb = 6; continue; }
            6 => { rsi = rbx; rax = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rbx; fl_r = (crate::simd_ops::ru64(rdi.wrapping_add(0xc8u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 8; continue; } bb = 7; continue; }
            7 => { crate::simd_ops::wu64(rdi.wrapping_add(0xc8u64), rax); bb = 9; continue; }
            8 => { crate::simd_ops::wu64(r14, rax); bb = 9; continue; }
            9 => { fl_l = rbx; fl_r = (crate::simd_ops::ru64(rdi.wrapping_add(0xc0u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 11; continue; } bb = 10; continue; }
            10 => { crate::simd_ops::wu64(rdi.wrapping_add(0xc0u64), r14); bb = 11; continue; }
            11 => { crate::simd_ops::lk_sub_u32(rdi.wrapping_add(0xb8u64), (1) as u32); rbx = rax; r8 = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; r9 = (crate::simd_ops::ru32(rsi.wrapping_add(0x10u64))) as u32 as u64; rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0xa0u64))) as u32 as u64; rdx = (crate::simd_ops::ru64(r8)) as u64; rax = ((rdx.wrapping_add(r9)) as u64) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 14; continue; } bb = 12; continue; }
            12 => { rax = (crate::simd_ops::ru32(r8.wrapping_add(0x18u64))) as u32 as u64; rax = ((rax).wrapping_add(rdx)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 14; continue; } bb = 13; continue; }
            13 => { rcx = (((rcx as u32)).wrapping_sub((r9 as u32))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 15; continue; }
            14 => { rcx = (crate::simd_ops::ru32(r8.wrapping_add(0x18u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 15; continue; }
            15 => { crate::simd_ops::lk_sub_u32(rdi.wrapping_add(0xa8u64), ((rcx as u32)) as u32); rdx = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x60u64))) as u64; crate::simd_ops::wu64(rsi.wrapping_add(0x8u64), r15); fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 16; continue; }
            16 => { rcx = ((rcx).wrapping_add(0xc0u64)) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 17; continue; }
            17 => { rdx = rsi; rcx = 0x7; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 6; continue; } bb = 18; continue; }
            18 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; bb = 19; continue; }
            19 => { rcx = rbp.wrapping_add(0x18); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; bb = 20; continue; }
            20 => { rcx = rdi; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P4, rax, rcx, rdx, r8, r9, rsp); }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416768e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1426452b0; const P1: u64 = 0xc0; const P2: u64 = 0x14167d070; const P3: u64 = 0x18; const P4: u64 = 0x1426452a8;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    rcx = ((rcx).wrapping_add(0x18u64)) as u64;
    rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp);
    rcx = ((rbx.wrapping_add(P1)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    rcx = ((rbx.wrapping_add(P3)) as u64) as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141676920(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x88; const P1: u64 = 0x141676980; const P2: u64 = 0x78;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; r9 = ((rcx.wrapping_add(P0)) as u64) as u64; rsi = (((r8 as u8)) as u32) as u32 as u64; rbx = (((rdx as u8)) as u32) as u32 as u64; rdi = rcx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rbx as u8)) as u8 as u64; fl_r = ((rbx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rdx = (rdx & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 4; continue; }
            3 => { rdx = 0; fl_l = 0; fl_r = 0; bb = 4; continue; }
            4 => { r9 = ((rdi.wrapping_add(P2)) as u64) as u64; r8 = (((rsi as u8)) as u32) as u32 as u64; rcx = rdi; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P1, rax, rcx, rdx, r8, r9, rsp); }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141676980(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1426452b0; const P1: u64 = 0x1415242e0; const P2: u64 = 0x1426452a8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu8(rsp.wrapping_add(0x10u64), ((rdx as u8)) as u8); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x28u64)) as u64; rdi = rcx.wrapping_add(0x98); r13 = r9; rcx = rdi; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rdi); r12 = (((r8 as u8)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); rsi = (crate::simd_ops::ru64(r13.wrapping_add(0x8u64))) as u64; r14 = 0; fl_l = 0; fl_r = 0; rbx = ((r14 as u32)) as u32 as u64; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 19; continue; } bb = 1; continue; }
            1 => { rdi = ((crate::simd_ops::ru8(rsp.wrapping_add(0x68u64))) as u32) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r15); bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru8(rsi.wrapping_add(0x78u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 9; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rsi)) as u64; rcx = rsi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x18u64))) as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 8; continue; } bb = 4; continue; }
            4 => { fl_l = rsi; fl_r = (crate::simd_ops::ru64(r13.wrapping_add(0x8u64))) as u64; r15 = rsi; rsi = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { crate::simd_ops::wu64(r13.wrapping_add(0x8u64), rcx); bb = 7; continue; }
            6 => { crate::simd_ops::wu64(rbx.wrapping_add(0x18u64), rcx); bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru64(r15)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = r15; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rdx = r15; rcx = 0x7; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 16; continue; }
            8 => { rbx = rsi; rsi = rcx; bb = 16; continue; }
            9 => { fl_l = ((rdi as u8)) as u8 as u64; fl_r = ((rdi as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 15; continue; } bb = 10; continue; }
            10 => { fl_l = r14; fl_r = r14; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 11; continue; }
            11 => { rax = ((crate::simd_ops::ru8(r14.wrapping_add(0x74u64))) as u32) as u32 as u64; fl_l = (crate::simd_ops::ru8(rsi.wrapping_add(0x74u64))) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) as i8) >= (((fl_r as u8)) as i8)) { bb = 15; continue; } bb = 12; continue; }
            12 => { fl_l = (crate::simd_ops::ru8(rsi.wrapping_add(0x74u64))) as u8 as u64; fl_r = ((r12 as u8)) as u8 as u64; if ((((fl_l as u8)) as i8) >= (((fl_r as u8)) as i8)) { bb = 15; continue; } bb = 13; continue; }
            13 => { rcx = rsi.wrapping_add(0x30); r15 = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); rbp = ((crate::simd_ops::ru8(rsi.wrapping_add(0x7cu64))) as u32) as u32 as u64; rcx = rsi.wrapping_add(0x30); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = ((rbp as u8)) as u8 as u64; fl_r = ((rbp as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 15; continue; } bb = 14; continue; }
            14 => { r14 = rsi; bb = 15; continue; }
            15 => { rbx = rsi; rsi = (crate::simd_ops::ru64(rsi.wrapping_add(0x18u64))) as u64; bb = 16; continue; }
            16 => { fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 17; continue; }
            17 => { r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; fl_l = r14; fl_r = r14; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 19; continue; } bb = 18; continue; }
            18 => { rax = (crate::simd_ops::ru64(r14)) as u64; rcx = r14; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rbx = (rbx & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 20; continue; }
            19 => { rbx = (rbx & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 20; continue; }
            20 => { rcx = rdi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); rax = (((rbx as u8)) as u32) as u32 as u64; rsp = ((rsp).wrapping_add(0x28u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141676ac0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1426452b0; const P1: u64 = 0x141676dd0; const P2: u64 = 0x14167c780; const P3: u64 = 0x1426452a8; const P4: u64 = 0x14167c920;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = 0; fl_l = 0; fl_r = 0; rbx = rcx; crate::simd_ops::wu64(rdx, rax); rcx = ((rcx).wrapping_add(0xffffffffffffffb0u64)) as u64; crate::simd_ops::wu32(r8, ((rax as u32)) as u32); r15 = (((r9 as u8)) as u32) as u32 as u64; r14 = r8; rsi = rdx; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); rdx = r14; rcx = rbx.wrapping_sub(0x80); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rsi, rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 13; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0x53u64))) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 13; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_sub(0x7u64))) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 7; continue; } bb = 3; continue; }
            3 => { rdx = r14; rcx = rbx.wrapping_sub(0x80); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rsi, rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 13; continue; } bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_sub(0x20u64))) as u64; rdx = rbx.wrapping_sub(0x80); rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x50u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rdx = r14; rcx = rbx.wrapping_sub(0x80); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rsi, rax); bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(rsi)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 13; continue; } bb = 7; continue; }
            7 => { fl_l = ((r15 as u8)) as u8 as u64; fl_r = ((r15 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 13; continue; } bb = 8; continue; }
            8 => { fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 13; continue; } bb = 9; continue; }
            9 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0x53u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 13; continue; } bb = 10; continue; }
            10 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_sub(0x6u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 12; continue; } bb = 11; continue; }
            11 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x28u64))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 13; continue; } bb = 12; continue; }
            12 => { rcx = rbx.wrapping_sub(0x80); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = rbx.wrapping_sub(0x50); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rbx.wrapping_sub(0x20u64))) as u64; rdx = rbx.wrapping_sub(0x80); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rcx = rbx.wrapping_sub(0x50); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); rdx = r14; rcx = rbx.wrapping_sub(0x80); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rsi, rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 13; continue; }
            13 => { rcx = rbx.wrapping_sub(0x50); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0x53u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 15; continue; } bb = 14; continue; }
            14 => { rax = 0x2; bb = 23; continue; }
            15 => { fl_l = (crate::simd_ops::ru64(rsi)) as u64; fl_r = (0x0u64) as u64; rax = ((crate::simd_ops::ru8(rbx.wrapping_sub(0x9u64))) as u32) as u32 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 20; continue; } bb = 16; continue; }
            16 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 19; continue; } bb = 17; continue; }
            17 => { rax = (crate::simd_ops::ru64(rbx.wrapping_sub(0x58u64))) as u64; rcx = (crate::simd_ops::ru64(rax)) as u64; fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; fl_r = rcx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 19; continue; } bb = 18; continue; }
            18 => { rax = 0x11; bb = 23; continue; }
            19 => { rax = 0x2e; bb = 23; continue; }
            20 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 22; continue; } bb = 21; continue; }
            21 => { rax = (crate::simd_ops::ru64(rbx.wrapping_sub(0x58u64))) as u64; rcx = (crate::simd_ops::ru64(rax)) as u64; rax = 0x11; fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; fl_r = rcx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 23; continue; } bb = 22; continue; }
            22 => { rax = 0x2d; bb = 23; continue; }
            23 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141676c30(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x20; const P1: u64 = 0x30; const P2: u64 = 0x28; const P3: u64 = 0x38;
    let (mut rax, mut fl_l) = (crate::simd_ops::get_rax_in(), 0u64);
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rax = ((rax).wrapping_add(crate::simd_ops::ru64(rcx.wrapping_add(P1)))) as u64;
    crate::simd_ops::wu64(rdx, rax);
    rax = (crate::simd_ops::ru32(rcx.wrapping_add(P2))) as u32 as u64;
    rax = (((rax as u32)).wrapping_sub(crate::simd_ops::ru32(rcx.wrapping_add(P1)))) as u32 as u64;
    crate::simd_ops::wu32(r8, ((rax as u32)) as u32);
    rax = 0x11;
    fl_l = (crate::simd_ops::lk_add_u16(rcx.wrapping_add(P3), (1) as u16)) as u16 as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141676c50(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x20; const P1: u64 = 0x10;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64;
    crate::simd_ops::wu32(rdx, ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141676c70(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x14; const P1: u64 = 0x1c; const P2: u64 = 0x4; const P3: u64 = 0x20; const P4: u64 = 0x8; const P5: u64 = 0x51; const P6: u64 = 0xc; const P7: u64 = 0xfffffffffffffff4; const P8: u64 = 0xd;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64;
    crate::simd_ops::wu32(rdx, ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64;
    crate::simd_ops::wu32(rdx.wrapping_add(P2), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rcx.wrapping_add(P3))) as u32 as u64;
    crate::simd_ops::wu32(rdx.wrapping_add(P4), ((rax as u32)) as u32);
    rax = ((crate::simd_ops::ru8(rcx.wrapping_add(P5))) as u32) as u32 as u64;
    crate::simd_ops::wu8(rdx.wrapping_add(P6), ((rax as u8)) as u8);
    rax = ((crate::simd_ops::ru8(rcx.wrapping_add(P7))) as u32) as u32 as u64;
    crate::simd_ops::wu8(rdx.wrapping_add(P8), ((rax as u8)) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141676c90(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0xffffffffffffffe0; const P1: u64 = 0x438; const P2: u64 = 0xffffffffffffffe8; const P3: u64 = 0x8; const P4: u64 = 0xffffffffffffffa8; const P5: u64 = 0x10; const P6: u64 = 0xfffffffffffffff9; const P7: u64 = 0x18;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    r8 = (crate::simd_ops::ru32(rax.wrapping_add(P1))) as u32 as u64;
    crate::simd_ops::wu32(rdx, ((r8 as u32)) as u32);
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64;
    crate::simd_ops::wu64(rdx.wrapping_add(P3), rax);
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P4))) as u64;
    r8 = (crate::simd_ops::ru64(rax)) as u64;
    crate::simd_ops::wu64(rdx.wrapping_add(P5), r8);
    rax = ((crate::simd_ops::ru8(rcx.wrapping_add(P6))) as u32) as u32 as u64;
    crate::simd_ops::wu8(rdx.wrapping_add(P7), ((rax as u8)) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141676cc0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x18; const P1: u64 = 0x40; const P2: u64 = 0x8; const P3: u64 = 0x28; const P4: u64 = 0x10;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    crate::simd_ops::wu8(rdx.wrapping_add(P0), (0x1u8) as u8);
    crate::simd_ops::wu32(rdx, (0xffffffffu32) as u32);
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64;
    crate::simd_ops::wu64(rdx.wrapping_add(P2), rax);
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P3))) as u64;
    crate::simd_ops::wu64(rdx.wrapping_add(P4), rax);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141676ce0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x7b; const P1: u64 = 0x94; const P2: u64 = 0x60; const P3: u64 = 0x434; const P4: u64 = 0x98;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(P0))) as u8 as u64; fl_r = (0x0u8) as u8 as u64;
    if !((((fl_l as u8)) == ((fl_r as u8)))) { rax = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64;
    xmm0 = ((crate::simd_ops::ru32(rax.wrapping_add(P3))) as u32) as u128;
    xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(crate::simd_ops::ru32(rcx.wrapping_add(P1)))).to_bits() as u128);
    rcx = (crate::simd_ops::ru32(rcx.wrapping_add(P4))) as u32 as u64;
    rax = (crate::simd_ops::cvt_f32_i64((f32::from_bits(xmm0 as u32)))) as u64;
    fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64;
    rax = (if (((fl_l as u32)) > ((fl_r as u32))) { (rcx as u32) } else { (rax as u32) }) as u32 as u64;
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141676d20(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x28; const P1: u64 = 0x30;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64;
    fl_l = ((rax as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64;
    rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141676d30(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1426452b0; const P1: u64 = 0x48; const P2: u64 = 0x10; const P3: u64 = 0x8; const P4: u64 = 0xffffffffffffffa8; const P5: u64 = 0xffffffffffffffb0; const P6: u64 = 0x1426452a8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi);
    crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rdi);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, r14);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rsi = rcx;
    r14 = rdx;
    rcx = ((rcx).wrapping_add(0xffffffffffffffb0u64)) as u64;
    rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp);
    rax = (crate::simd_ops::ru64(rsi.wrapping_add(P1))) as u64;
    fl_l = rax; fl_r = rax;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rdi = (crate::simd_ops::ru32(rax.wrapping_add(P2))) as u32 as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64; fl_l = ((rdi).wrapping_add(crate::simd_ops::ru64(rax))) as u64; fl_r = fl_l; rdi = ((fl_l as u64)) as u64; } else { rdi = (crate::simd_ops::ru64(rsi.wrapping_add(P3))) as u64; }
    fl_l = r14; fl_r = r14;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rcx = (crate::simd_ops::ru64(rsi.wrapping_add(P4))) as u64; fl_l = rdi; fl_r = (crate::simd_ops::ru64(rcx)) as u64; rcx = (rcx & !0xffu64) | (((if (((fl_l as u64)) >= ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); crate::simd_ops::wu8(r14, ((rcx as u8)) as u8); }
    rcx = ((rsi.wrapping_add(P5)) as u64) as u64;
    rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rax = rdi;
    rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64;
    rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    r14 = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141676dc0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0xfffffffffffffff7; const P1: u64 = 0x8;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = rdx; fl_r = rdx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rax = ((crate::simd_ops::ru8(rcx.wrapping_add(P0))) as u32) as u32 as u64; crate::simd_ops::wu8(rdx, ((rax as u8)) as u8); }
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141676dd0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141676780; const P1: u64 = 0x141678970;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; r8 = ((crate::simd_ops::ru8(rcx.wrapping_add(0xd0u64))) as u32) as u32 as u64; rsi = rdx; rdi = rcx; fl_l = ((r8 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rcx.wrapping_add(0xb8u64))) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 11; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx); rbx = (crate::simd_ops::ru64(rcx.wrapping_add(0xc8u64))) as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 4; continue; } bb = 2; continue; }
            2 => { rax = ((r8 as u32)) as u32 as u64; bb = 3; continue; }
            3 => { rbx = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rax; fl_r = (0x1u64) as u64; rax = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x10u64))) as u32 as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rax))) as u64; fl_l = rcx; fl_r = (crate::simd_ops::ru64(rdi.wrapping_add(0x88u64))) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rdi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x88u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = rdi; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rsi, (0x0u32) as u32); rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            6 => { r8 = (r8 & !0xffu64) | (((((r8 as u8)).wrapping_add(1)) as u8 as u64)); crate::simd_ops::wu8(rdi.wrapping_add(0xd0u64), ((r8 as u8)) as u8); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rcx = (crate::simd_ops::ru32(rax.wrapping_add(0x18u64))) as u32 as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rax))) as u64; crate::simd_ops::wu64(rdi.wrapping_add(0x88u64), rcx); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rcx = (crate::simd_ops::ru32(rax.wrapping_add(0x18u64))) as u32 as u64; rcx = (((rcx as u32)).wrapping_sub(crate::simd_ops::ru32(rbx.wrapping_add(0x10u64)))) as u32 as u64; crate::simd_ops::wu32(rdx, ((rcx as u32)) as u32); r8 = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rdx = (crate::simd_ops::ru32(rbx.wrapping_add(0x10u64))) as u32 as u64; rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0xa0u64))) as u32 as u64; r9 = (crate::simd_ops::ru64(r8)) as u64; rax = ((r9.wrapping_add(rdx)) as u64) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 9; continue; } bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru32(r8.wrapping_add(0x18u64))) as u32 as u64; rax = ((rax).wrapping_add(r9)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 9; continue; } bb = 8; continue; }
            8 => { rcx = (((rcx as u32)).wrapping_sub((rdx as u32))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 10; continue; }
            9 => { rcx = (crate::simd_ops::ru32(r8.wrapping_add(0x18u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 10; continue; }
            10 => { crate::simd_ops::lk_sub_u32(rdi.wrapping_add(0xa8u64), ((rcx as u32)) as u32); rcx = rdi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x10u64))) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(rcx.wrapping_add(0x8u64)))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            11 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rdx, (0x0u32) as u32); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141676f10(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1434268d0; const P1: u64 = 0x141677d90;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; crate::simd_ops::wu64(rcx.wrapping_add(0x60u64), rdx); rdi = r8; fl_l = (crate::simd_ops::ru64(r8)) as u64; fl_r = (0x0u64) as u64; rbx = rcx; if ((((fl_l as u64)) as i64) >= (((fl_r as u64)) as i64)) { bb = 2; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu8(rcx.wrapping_add(0x78u64), (0x1u8) as u8); rax = 0x1f; crate::simd_ops::wu8(rcx.wrapping_add(0x7cu64), (0x0u8) as u8); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            2 => { crate::simd_ops::wu32(rcx.wrapping_add(0x90u64), ((r9 as u32)) as u32); rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x428u64))) as u64; rdx = rdi; rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r9 = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 9; continue; } bb = 3; continue; }
            3 => { r10 = (crate::simd_ops::ru32(rsp.wrapping_add(0x60u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((r10 as u32)) as u32 as u64; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 9; continue; } bb = 4; continue; }
            4 => { rdx = 0; fl_l = 0; fl_r = 0; rax = ((r10 as u32)) as u32 as u64; let d = ((r9 as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 9; continue; } bb = 5; continue; }
            5 => { r8 = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rdx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rbx.wrapping_add(0x70u64), ((r9 as u32)) as u32); crate::simd_ops::wu32(rbx.wrapping_add(0xb0u64), ((r9 as u32)) as u32); xmm0 = ((crate::simd_ops::ru32(r8)) as u32) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((if f32::from_bits(xmm0 as u32) > f32::from_bits(crate::simd_ops::ru32(P0)) { f32::from_bits(xmm0 as u32) } else { f32::from_bits(crate::simd_ops::ru32(P0)) }).to_bits() as u128); crate::simd_ops::wu32(rbx.wrapping_add(0x94u64), ((xmm0 as u32)) as u32); rcx = (crate::simd_ops::ru32(r8.wrapping_add(0x4u64))) as u32 as u64; rax = ((rcx as u32)) as u32 as u64; let d = ((r9 as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } rcx = (((rcx as u32)).wrapping_sub((rdx as u32))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x9cu64), ((rcx as u32)) as u32); rax = (crate::simd_ops::ru64(rdi)) as u64; rcx = (crate::simd_ops::ru32(r8.wrapping_add(0x8u64))) as u32 as u64; fl_l = rcx; fl_r = rax; rcx = (if ((((fl_l as u64)) as i64) > (((fl_r as u64)) as i64)) { (rax as u32) } else { (rcx as u32) }) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0xa0u64), ((rcx as u32)) as u32); rcx = 0x1; rax = ((crate::simd_ops::ru8(r8.wrapping_add(0xcu64))) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; rcx = (if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { (rax as u32) } else { (rcx as u32) }) as u32 as u64; crate::simd_ops::wu8(rbx.wrapping_add(0xd1u64), ((rcx as u8)) as u8); rax = ((crate::simd_ops::ru8(r8.wrapping_add(0xdu64))) as u32) as u32 as u64; crate::simd_ops::wu8(rbx.wrapping_add(0x74u64), ((rax as u8)) as u8); crate::simd_ops::wu64(rbx.wrapping_add(0x88u64), (0x0u64) as u64); fl_l = (crate::simd_ops::ru64(rdi)) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 7; continue; } bb = 6; continue; }
            6 => { crate::simd_ops::wu8(rbx.wrapping_add(0x77u64), (0x1u8) as u8); crate::simd_ops::wu8(rbx.wrapping_add(0x7cu64), (0x0u8) as u8); bb = 7; continue; }
            7 => { rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; r8 = ((r10 as u32)) as u32 as u64; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 10; continue; } bb = 8; continue; }
            8 => { crate::simd_ops::wu8(rbx.wrapping_add(0x7cu64), (0x0u8) as u8); crate::simd_ops::wu8(rbx.wrapping_add(0x78u64), (0x1u8) as u8); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            9 => { rax = 0x2; crate::simd_ops::wu8(rbx.wrapping_add(0x7cu64), (0x0u8) as u8); crate::simd_ops::wu8(rbx.wrapping_add(0x78u64), (0x1u8) as u8); bb = 10; continue; }
            10 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141677050(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14; const P1: u64 = 0x8; const P2: u64 = 0x28; const P3: u64 = 0x18; const P4: u64 = 0x2c; const P5: u64 = 0x430; const P6: u64 = 0xc0; const P7: u64 = 0x434; const P8: u64 = 0x438; const P9: u64 = 0x34; const P10: u64 = 0x43c; const P11: u64 = 0x14167d780; const P12: u64 = 0x14167c600;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = rcx; rbx = rdx; rcx = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 8; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 3; continue; } bb = 2; continue; }
            2 => { xmm0 = 0u128; fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rdx.wrapping_add(P2))) as u32 as u64; if (f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 8; continue; } bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(P3))) as u8 as u64; fl_r = (0x2u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 6; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru32(rdx.wrapping_add(P4))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 8; continue; } bb = 5; continue; }
            5 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x400; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 8; continue; } bb = 6; continue; }
            6 => { crate::simd_ops::wu32(rdi.wrapping_add(P5), ((rcx as u32)) as u32); rcx = ((rdi.wrapping_add(P6)) as u64) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(P2))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(P7), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(P4))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(P8), ((r8 as u32)) as u32); r8 = rdi; crate::simd_ops::wu32(rdi.wrapping_add(P1), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(P9))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(P10), ((rax as u32)) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P11, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 9; continue; } bb = 7; continue; }
            7 => { rdx = rbx.wrapping_add(0x1c); rcx = rdi; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk_xmm(P12, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); }
            8 => { rax = 0x1f; bb = 9; continue; }
            9 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416770f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x60; const P1: u64 = 0x78; const P2: u64 = 0x7c; const P3: u64 = 0x428; const P4: u64 = 0x10; const P5: u64 = 0x430; const P6: u64 = 0x70; const P7: u64 = 0x1;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; crate::simd_ops::wu64(rcx.wrapping_add(P0), rdx); rdi = rdx; fl_l = (crate::simd_ops::ru64(r8)) as u64; fl_r = (0x0u64) as u64; rbx = rcx; if ((((fl_l as u64)) as i64) >= (((fl_r as u64)) as i64)) { bb = 2; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu8(rcx.wrapping_add(P1), (0x1u8) as u8); rax = 0x1f; crate::simd_ops::wu8(rcx.wrapping_add(P2), (0x0u8) as u8); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            2 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P3))) as u64; rdx = r8; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P4))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 6; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(P5))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 6; continue; } bb = 4; continue; }
            4 => { rdx = 0; fl_l = 0; fl_r = 0; let d = ((rcx as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { crate::simd_ops::wu32(rbx.wrapping_add(P6), ((rcx as u32)) as u32); rax = ((rdx.wrapping_add(P7)) as u32) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            6 => { crate::simd_ops::wu8(rbx.wrapping_add(P1), (0x1u8) as u8); rax = 0x2; crate::simd_ops::wu8(rbx.wrapping_add(P2), (0x0u8) as u8); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141677180(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1426452b0; const P1: u64 = 0x80; const P2: u64 = 0xd3; const P3: u64 = 0x68; const P4: u64 = 0x141678970; const P5: u64 = 0x10; const P6: u64 = 0x60; const P7: u64 = 0x14167c7d0; const P8: u64 = 0x30; const P9: u64 = 0x1426452a8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdi = rcx;
    rcx = ((rcx).wrapping_add(0x30u64)) as u64;
    rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp);
    rcx = ((rdi.wrapping_add(P1)) as u64) as u64;
    crate::simd_ops::wu8(rdi.wrapping_add(P2), (0x1u8) as u8);
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp);
    rcx = rdi;
    rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp);
    fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(P5))) as u8 as u64; fl_r = (0x0u8) as u8 as u64;
    if !((((fl_l as u8)) == ((fl_r as u8)))) { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P6))) as u64; rdx = rdi; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); }
    rcx = ((rdi.wrapping_add(P8)) as u64) as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P9), rax), rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416771e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1426452b0; const P1: u64 = 0x78; const P2: u64 = 0xb8; const P3: u64 = 0x8; const P4: u64 = 0x7a; const P5: u64 = 0x60; const P6: u64 = 0x14167c820; const P7: u64 = 0x7c; const P8: u64 = 0x14167c7e0; const P9: u64 = 0x10; const P10: u64 = 0x14167c7d0; const P11: u64 = 0x30; const P12: u64 = 0x1426452a8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = rcx; rcx = ((rcx).wrapping_add(0x30u64)) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(P1))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; crate::simd_ops::wu32(rdi.wrapping_add(P2), (0x4u32) as u32); if (((fl_l as u8)) == ((fl_r as u8))) { bb = 4; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rcx = rdi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 4; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(P4))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 6; continue; } bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P5))) as u64; crate::simd_ops::wu8(rdi.wrapping_add(P4), (0x1u8) as u8); rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            4 => { fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(P4))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; crate::simd_ops::wu8(rdi.wrapping_add(P7), (0x0u8) as u8); if (((fl_l as u8)) == ((fl_r as u8))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P5))) as u64; crate::simd_ops::wu8(rdi.wrapping_add(P4), (0x0u8) as u8); rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            6 => { fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(P9))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 9; continue; } bb = 7; continue; }
            7 => { fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(P2))) as u32 as u64; fl_r = 0xfffffffd; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P5))) as u64; rdx = rdi; rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp); bb = 9; continue; }
            9 => { rcx = ((rdi.wrapping_add(P11)) as u64) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P12), rax), rax, rcx, rdx, r8, r9, rsp); }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141677280(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x28; const P1: u64 = 0xa0; const P2: u64 = 0x79; const P3: u64 = 0x80; const P4: u64 = 0x20;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x30u64)) as u64;
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rdi = rcx;
    rdx = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64;
    crate::simd_ops::wu8(rcx.wrapping_add(P2), (0x1u8) as u8);
    fl_l = rdx; fl_r = (crate::simd_ops::ru64(rax)) as u64;
    if !(((((fl_l as u64)) as i64) <= (((fl_r as u64)) as i64))) { rax = (crate::simd_ops::ru64(rcx.wrapping_add(P3))) as u64; rdx = rsp.wrapping_add(0x20); rcx = ((rcx).wrapping_sub(0xffffffffffffff80u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rbx); rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P4))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rdi.wrapping_add(P0))) as u64; rcx = ((rdi.wrapping_add(P3)) as u64) as u64; rdx = (crate::simd_ops::ru32(rax)) as u32 as u64; rax = (crate::simd_ops::ru64(rdi.wrapping_add(P3))) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rdx as u32)) as u32); rdx = rsp.wrapping_add(0x20); rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; }
    rsp = ((rsp).wrapping_add(0x30u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416772f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1426456f8;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = ((rcx).wrapping_add(0x70u64)) as u64; fl_r = fl_l;
    rcx = ((fl_l as u64)) as u64;
    return crate::simd_ops::call_fwd(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141677300(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1426452b0; const P1: u64 = 0x1426452a8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x28u64)) as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0x53u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; rdi = rdx; rsi = rcx; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = 0x2; rsp = ((rsp).wrapping_add(0x28u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            2 => { crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rbx); rcx = ((rcx).wrapping_add(0xffffffffffffffb0u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), r14); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru8(rsi.wrapping_sub(0x7u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; r14 = 0x2e; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 15; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), r12); r12 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r15); bb = 4; continue; }
            4 => { crate::simd_ops::wu32(rdi, ((r12 as u32)) as u32); rbp = ((r14 as u32)) as u32 as u64; rcx = ((crate::simd_ops::ru8(rsi.wrapping_add(0x50u64))) as u32) as u32 as u64; rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x48u64))) as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rax)) as u64; fl_l = rcx; fl_r = (0x1u64) as u64; rcx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 5; continue; } bb = 6; continue; }
            6 => { fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 7; continue; }
            7 => { rbp = 0x2d; bb = 8; continue; }
            8 => { rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; rdx = (crate::simd_ops::ru32(rcx.wrapping_add(0x18u64))) as u32 as u64; rdx = (((rdx as u32)).wrapping_sub(crate::simd_ops::ru32(rax.wrapping_add(0x10u64)))) as u32 as u64; crate::simd_ops::lk_add_u32(rdi, ((rdx as u32)) as u32); rax = (crate::simd_ops::ru64(rax)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 8; continue; } bb = 9; continue; }
            9 => { fl_l = (crate::simd_ops::ru8(rsi.wrapping_sub(0x6u64))) as u8 as u64; fl_r = ((r12 as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 11; continue; } bb = 10; continue; }
            10 => { rax = (crate::simd_ops::ru32(rdi)) as u32 as u64; fl_l = (crate::simd_ops::ru32(rsi.wrapping_add(0x28u64))) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 13; continue; } bb = 11; continue; }
            11 => { rcx = (crate::simd_ops::ru64(rsi.wrapping_sub(0x20u64))) as u64; rdx = rsi.wrapping_sub(0x80); rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x50u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 4; continue; } bb = 12; continue; }
            12 => { rax = (crate::simd_ops::ru64(rsi.wrapping_sub(0x20u64))) as u64; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x68u64))) as u8 as u64; fl_r = ((r12 as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 14; continue; } bb = 13; continue; }
            13 => { rbp = 0x11; bb = 14; continue; }
            14 => { r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; r14 = ((rbp as u32)) as u32 as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; bb = 15; continue; }
            15 => { rcx = rsi.wrapping_sub(0x50); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rax = ((r14 as u32)) as u32 as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x28u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141677410(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x28; const P1: u64 = 0x30;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64;
    fl_l = ((rax as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64;
    rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64;
    crate::simd_ops::wu32(rdx, ((rax as u32)) as u32);
    rax = 0x11;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141677420(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1416764c0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = ((rsp).wrapping_sub(0x48u64)) as u64;
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64;
    rcx = ((rcx).wrapping_add(0xffffffffffffff80u64)) as u64;
    xmm0 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x78u64))) as u32) as u128;
    crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rax);
    rax = ((crate::simd_ops::ru8(rsp.wrapping_add(0x70u64))) as u32) as u32 as u64;
    crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((xmm0 as u32)) as u32);
    crate::simd_ops::wu8(rsp.wrapping_add(0x28u64), ((rax as u8)) as u8);
    crate::simd_ops::wu8(rsp.wrapping_add(0x20u64), ((r9 as u8)) as u8);
    r9 = ((r8 as u32)) as u32 as u64;
    r8 = rdx;
    rdx = 0;
    fl_l = 0; fl_r = 0;
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rsp = ((rsp).wrapping_add(0x48u64)) as u64;
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141677470(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xffffffffffffffb0; const P1: u64 = 0x1426452b0; const P2: u64 = 0x50; const P3: u64 = 0xffffffffffffffe0; const P4: u64 = 0x48; const P5: u64 = 0x40; const P6: u64 = 0x38; const P7: u64 = 0x8; const P8: u64 = 0x14167d920; const P9: u64 = 0x1415242e0; const P10: u64 = 0x1426452a8; const P11: u64 = 0xffffffffffffff80; const P12: u64 = 0x141678970;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = ((rcx.wrapping_add(P0)) as u64) as u64; rdi = rcx; rcx = rbx; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(P2))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) <= ((fl_r as u8))) { bb = 8; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbp); rbp = (crate::simd_ops::ru64(rdi.wrapping_add(P3))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rsi); rsi = (crate::simd_ops::ru64(rdi.wrapping_add(P4))) as u64; rcx = rbp.wrapping_add(0x18); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rdi.wrapping_add(P4))) as u64; r8 = 0; fl_l = 0; fl_r = 0; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rax)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu64(rdi.wrapping_add(P5), r8); rax = ((r8 as u32)) as u32 as u64; bb = 4; continue; }
            4 => { crate::simd_ops::wu64(rdi.wrapping_add(P4), rax); fl_l = (crate::simd_ops::lk_sub_u32(rdi.wrapping_add(P6), (1) as u32)) as u32 as u64; bb = 5; continue; }
            5 => { rdx = (crate::simd_ops::ru64(rsi.wrapping_add(P7))) as u64; rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P3))) as u64; crate::simd_ops::wu64(rsi.wrapping_add(P7), r8); fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { rcx = ((rcx).wrapping_add(0xc0u64)) as u64; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); bb = 7; continue; }
            7 => { rdx = rsi; rcx = 0x7; rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); rcx = rbp.wrapping_add(0x18); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P10), rax), rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::lk_sub_u8(rdi.wrapping_add(P2), (1) as u8); rcx = ((rdi.wrapping_add(P11)) as u64) as u64; rax = crate::simd_ops::call_stk(P12, rax, rcx, rdx, r8, r9, rsp); rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rdi = 0x1; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; bb = 9; continue; }
            8 => { rdi = 0x2; bb = 9; continue; }
            9 => { rcx = rbx; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P10), rax), rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rax = ((rdi as u32)) as u32 as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141677540(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x38;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = ((crate::simd_ops::ru16(rcx.wrapping_add(P0))) as u32) as u32 as u64;
    fl_l = ((rax as u16)) as u16 as u64; fl_r = ((rax as u16)) as u16 as u64;
    if !(((((fl_l as u16)) & ((fl_r as u16))) != 0)) { rax = 0x2; return rax; }
    fl_l = (((rax as u16)).wrapping_sub(1)) as u16 as u64;
    rax = (rax & !0xffffu64) | ((((fl_l as u16)) as u16 as u64));
    crate::simd_ops::wu16(rcx.wrapping_add(P0), ((rax as u16)) as u16);
    rax = 0x1;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141677560(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1426452b0; const P1: u64 = 0x14167d920; const P2: u64 = 0x1415242e0; const P3: u64 = 0x1426452a8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; r15 = 0; fl_l = 0; fl_r = 0; rbp = ((rdx as u32)) as u32 as u64; rbx = rcx; rsi = ((r15 as u32)) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 24; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(0xb8u64))) as u32 as u64; fl_r = ((r15 as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 24; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), r14); r14 = (crate::simd_ops::ru64(rcx.wrapping_add(0x60u64))) as u64; rcx = r14.wrapping_add(0x18); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); rdi = (crate::simd_ops::ru64(rbx.wrapping_add(0xc0u64))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 23; continue; } bb = 3; continue; }
            3 => { fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 23; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; r8 = (crate::simd_ops::ru64(rbx.wrapping_add(0xc8u64))) as u64; rcx = (crate::simd_ops::ru32(rax.wrapping_add(0x18u64))) as u32 as u64; rax = r8; rcx = (((rcx as u32)).wrapping_sub(crate::simd_ops::ru32(rdi.wrapping_add(0x10u64)))) as u32 as u64; rsi = (((rsi as u32)).wrapping_add((rcx as u32))) as u32 as u64; rcx = r15; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 5; continue; }
            5 => { bb = 6; continue; }
            6 => { fl_l = rax; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 9; continue; } bb = 7; continue; }
            7 => { rcx = rax; rax = (crate::simd_ops::ru64(rax)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 6; continue; } bb = 8; continue; }
            8 => { bb = 16; continue; }
            9 => { fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 16; continue; } bb = 10; continue; }
            10 => { rdx = (crate::simd_ops::ru64(rax)) as u64; fl_l = rax; fl_r = r8; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 12; continue; } bb = 11; continue; }
            11 => { crate::simd_ops::wu64(rbx.wrapping_add(0xc8u64), rdx); bb = 13; continue; }
            12 => { crate::simd_ops::wu64(rcx, rdx); bb = 13; continue; }
            13 => { fl_l = rax; fl_r = (crate::simd_ops::ru64(rbx.wrapping_add(0xc0u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 15; continue; } bb = 14; continue; }
            14 => { crate::simd_ops::wu64(rbx.wrapping_add(0xc0u64), rcx); bb = 15; continue; }
            15 => { fl_l = (crate::simd_ops::lk_sub_u32(rbx.wrapping_add(0xb8u64), (1) as u32)) as u32 as u64; bb = 16; continue; }
            16 => { rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; r9 = (crate::simd_ops::ru32(rdi.wrapping_add(0x10u64))) as u32 as u64; rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0xa0u64))) as u32 as u64; r8 = (crate::simd_ops::ru64(rdx)) as u64; rax = ((r8.wrapping_add(r9)) as u64) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 19; continue; } bb = 17; continue; }
            17 => { rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x18u64))) as u32 as u64; rax = ((rax).wrapping_add(r8)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 19; continue; } bb = 18; continue; }
            18 => { rcx = (((rcx as u32)).wrapping_sub((r9 as u32))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 20; continue; }
            19 => { rcx = (crate::simd_ops::ru32(rdx.wrapping_add(0x18u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 20; continue; }
            20 => { crate::simd_ops::lk_sub_u32(rbx.wrapping_add(0xa8u64), ((rcx as u32)) as u32); rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x60u64))) as u64; crate::simd_ops::wu64(rdi.wrapping_add(0x8u64), r15); fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 22; continue; } bb = 21; continue; }
            21 => { rcx = ((rcx).wrapping_add(0xc0u64)) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 22; continue; }
            22 => { rdx = rdi; rcx = 0x7; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rdi = (crate::simd_ops::ru64(rbx.wrapping_add(0xc0u64))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } bb = 23; continue; }
            23 => { rcx = r14.wrapping_add(0x18); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; bb = 24; continue; }
            24 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rax = ((rsi as u32)) as u32 as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416776d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415242e0; const P1: u64 = 0x1426452b0; const P2: u64 = 0x1426452a8; const P3: u64 = 0x1434268d0; const P4: u64 = 0x1439318d4;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0; let mut xmm7: u128 = 0; let mut xmm8: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x50u64)) as u64; rsi = (crate::simd_ops::ru64(rcx.wrapping_add(0x80u64))) as u64; rdi = 0; fl_l = 0; fl_r = 0; r15 = rdx; rbp = rcx; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 1; continue; }
            1 => { bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru8(rsi.wrapping_add(0x78u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 9; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rsi)) as u64; rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x18u64))) as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 8; continue; } bb = 4; continue; }
            4 => { fl_l = rsi; fl_r = (crate::simd_ops::ru64(rbp.wrapping_add(0x80u64))) as u64; rbx = rsi; rsi = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { crate::simd_ops::wu64(rbp.wrapping_add(0x80u64), rcx); bb = 7; continue; }
            6 => { crate::simd_ops::wu64(rdi.wrapping_add(0x18u64), rcx); bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rbx; rcx = 0x7; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 12; continue; }
            8 => { rdi = rsi; rsi = rcx; bb = 12; continue; }
            9 => { fl_l = (crate::simd_ops::ru8(rsi.wrapping_add(0x75u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 11; continue; } bb = 10; continue; }
            10 => { rcx = rsi.wrapping_add(0x30); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = ((crate::simd_ops::ru8(rsi.wrapping_add(0x7cu64))) as u32) as u32 as u64; rcx = rsi.wrapping_add(0x30); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rdi as u8)) as u8 as u64; fl_r = ((rdi as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 15; continue; } bb = 11; continue; }
            11 => { rdi = rsi; rsi = (crate::simd_ops::ru64(rsi.wrapping_add(0x18u64))) as u64; bb = 12; continue; }
            12 => { fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 13; continue; }
            13 => { rax = 0; fl_l = 0; fl_r = 0; bb = 14; continue; }
            14 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; rsp = ((rsp).wrapping_add(0x50u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            15 => { rax = (crate::simd_ops::ru64(rsi)) as u64; rcx = rsi; crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), r14); crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm6) as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x30u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r14 = (crate::simd_ops::ru64(rsi.wrapping_add(0x18u64))) as u64; rbx = rsi; xmm6 = (xmm0) as u128; fl_l = r14; fl_r = r14; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 37; continue; } bb = 16; continue; }
            16 => { crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm7) as u128); xmm7 = 0u128; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm8) as u128); xmm8 = ((crate::simd_ops::ru32(P3)) as u32) as u128; bb = 17; continue; }
            17 => { fl_l = (crate::simd_ops::ru8(r14.wrapping_add(0x78u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 24; continue; } bb = 18; continue; }
            18 => { rax = (crate::simd_ops::ru64(r14)) as u64; rcx = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(r14.wrapping_add(0x18u64))) as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 23; continue; } bb = 19; continue; }
            19 => { fl_l = r14; fl_r = (crate::simd_ops::ru64(rbp.wrapping_add(0x80u64))) as u64; rdi = r14; r14 = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 21; continue; } bb = 20; continue; }
            20 => { crate::simd_ops::wu64(rbp.wrapping_add(0x80u64), rcx); bb = 22; continue; }
            21 => { crate::simd_ops::wu64(rbx.wrapping_add(0x18u64), rcx); bb = 22; continue; }
            22 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rdi; rcx = 0x7; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 35; continue; }
            23 => { rbx = r14; r14 = rcx; bb = 35; continue; }
            24 => { fl_l = (crate::simd_ops::ru8(r14.wrapping_add(0x75u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 34; continue; } bb = 25; continue; }
            25 => { rcx = r14.wrapping_add(0x30); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = ((crate::simd_ops::ru8(r14.wrapping_add(0x7cu64))) as u32) as u32 as u64; rcx = r14.wrapping_add(0x30); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rdi as u8)) as u8 as u64; fl_r = ((rdi as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 34; continue; } bb = 26; continue; }
            26 => { rax = (crate::simd_ops::ru64(r14)) as u64; rcx = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x30u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm3 = (xmm0) as u128; fl_l = ((xmm3 as u32)) as u32 as u64; fl_r = ((xmm7 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) != f32::from_bits((fl_r as u32)) && !f32::from_bits((fl_l as u32)).is_nan() && !f32::from_bits((fl_r as u32)).is_nan()) { bb = 32; continue; } bb = 27; continue; }
            27 => { rax = ((crate::simd_ops::ru8(r14.wrapping_add(0x74u64))) as u32) as u32 as u64; rcx = ((crate::simd_ops::ru8(rsi.wrapping_add(0x74u64))) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; if ((((fl_l as u8)) as i8) > (((fl_r as u8)) as i8)) { bb = 33; continue; } bb = 28; continue; }
            28 => { fl_l = ((xmm6 as u32)) as u32 as u64; fl_r = ((xmm7 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 33; continue; } bb = 29; continue; }
            29 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 34; continue; } bb = 30; continue; }
            30 => { rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x70u64))) as u64; xmm1 = 0u128; rax = rcx; xmm0 = 0u128; rax = ((rax).wrapping_sub(crate::simd_ops::ru64(r14.wrapping_add(0x58u64)))) as u64; xmm2 = (xmm8) as u128; xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) / f32::from_bits(crate::simd_ops::ru32(P4))).to_bits() as u128); rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(rsi.wrapping_add(0x58u64)))) as u64; xmm1 = (xmm1 & !0xffff_ffffu128) | ((((rax) as i64) as f32).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((((rcx) as i64) as f32).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm2 as u32)).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm2 as u32)).to_bits() as u128); fl_l = ((xmm1 as u32)) as u32 as u64; fl_r = ((xmm0 as u32)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 34; continue; } bb = 31; continue; }
            31 => { bb = 33; continue; }
            32 => { fl_l = ((xmm3 as u32)) as u32 as u64; fl_r = ((xmm6 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 34; continue; } bb = 33; continue; }
            33 => { xmm6 = (xmm3) as u128; rsi = r14; bb = 34; continue; }
            34 => { rbx = r14; r14 = (crate::simd_ops::ru64(r14.wrapping_add(0x18u64))) as u64; bb = 35; continue; }
            35 => { fl_l = r14; fl_r = r14; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 17; continue; } bb = 36; continue; }
            36 => { xmm8 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; xmm7 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; bb = 37; continue; }
            37 => { r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rax = rsi; crate::simd_ops::wu32(r15, ((xmm6 as u32)) as u32); xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x40u64))) as u128; bb = 14; continue; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141677910(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1426452b0; const P1: u64 = 0x1415242e0; const P2: u64 = 0x1426452a8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; r15 = rcx; r14 = 0; fl_l = 0; fl_r = 0; rcx = ((rcx).wrapping_add(0x98u64)) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); rdi = (crate::simd_ops::ru64(r15.wrapping_add(0x90u64))) as u64; r12 = 0; fl_l = 0; fl_r = 0; rsi = (rsi & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rbp = 0; fl_l = 0; fl_r = 0; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(0x78u64))) as u8 as u64; fl_r = ((r14 as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 7; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rcx = rdi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 7; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x18u64))) as u64; rsi = rdi; fl_l = rdi; fl_r = (crate::simd_ops::ru64(r15.wrapping_add(0x90u64))) as u64; rdi = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 5; continue; } bb = 4; continue; }
            4 => { crate::simd_ops::wu64(r15.wrapping_add(0x90u64), rax); bb = 6; continue; }
            5 => { crate::simd_ops::wu64(rbp.wrapping_add(0x18u64), rax); bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(rsi)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rsi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rdx = rsi; rcx = 0x7; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rsi = (rsi & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 8; continue; }
            7 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rcx = rdi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x48u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rbp = rdi; rdi = (crate::simd_ops::ru64(rdi.wrapping_add(0x18u64))) as u64; fl_l = (((r12 as u32)).wrapping_add((rax as u32))) as u32 as u64; fl_r = fl_l; r12 = ((fl_l as u32)) as u32 as u64; bb = 8; continue; }
            8 => { fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 1; continue; } bb = 9; continue; }
            9 => { rdi = (crate::simd_ops::ru64(r15.wrapping_add(0x90u64))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 33; continue; } bb = 10; continue; }
            10 => { bb = 11; continue; }
            11 => { fl_l = ((rsi as u8)) as u8 as u64; fl_r = ((rsi as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 13; continue; } bb = 12; continue; }
            12 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rcx = rdi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x50u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 13; continue; }
            13 => { fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(0x7au64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 17; continue; } bb = 14; continue; }
            14 => { fl_l = r14; fl_r = r14; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 16; continue; } bb = 15; continue; }
            15 => { rax = ((crate::simd_ops::ru8(r14.wrapping_add(0x74u64))) as u32) as u32 as u64; fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(0x74u64))) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) as i8) <= (((fl_r as u8)) as i8)) { bb = 17; continue; } bb = 16; continue; }
            16 => { r14 = rdi; bb = 17; continue; }
            17 => { rdi = (crate::simd_ops::ru64(rdi.wrapping_add(0x18u64))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 11; continue; } bb = 18; continue; }
            18 => { fl_l = r14; fl_r = r14; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 33; continue; } bb = 19; continue; }
            19 => { rax = (crate::simd_ops::ru64(r14)) as u64; rcx = r14; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x48u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru64(r14)) as u64; rcx = r14; rdi = ((rax as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rdx.wrapping_add(0x40u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rbp = ((rax as u32)) as u32 as u64; rbp = (((rbp as u32)).wrapping_sub((rdi as u32))) as u32 as u64; rcx = ((r12.wrapping_add(rbp)) as u32) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(r15.wrapping_add(0x43cu64))) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 33; continue; } bb = 20; continue; }
            20 => { bb = 21; continue; }
            21 => { rdi = (crate::simd_ops::ru64(r15.wrapping_add(0x90u64))) as u64; rsi = 0; fl_l = 0; fl_r = 0; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 31; continue; } bb = 22; continue; }
            22 => { bb = 23; continue; }
            23 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rcx = rdi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x48u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 28; continue; } bb = 24; continue; }
            24 => { rax = ((crate::simd_ops::ru8(rdi.wrapping_add(0x74u64))) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (crate::simd_ops::ru8(r14.wrapping_add(0x74u64))) as u8 as u64; if ((((fl_l as u8)) as i8) >= (((fl_r as u8)) as i8)) { bb = 28; continue; } bb = 25; continue; }
            25 => { fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 27; continue; } bb = 26; continue; }
            26 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = (crate::simd_ops::ru8(rsi.wrapping_add(0x74u64))) as u8 as u64; if ((((fl_l as u8)) as i8) >= (((fl_r as u8)) as i8)) { bb = 28; continue; } bb = 27; continue; }
            27 => { rsi = rdi; bb = 28; continue; }
            28 => { rdi = (crate::simd_ops::ru64(rdi.wrapping_add(0x18u64))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 23; continue; } bb = 29; continue; }
            29 => { fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 31; continue; } bb = 30; continue; }
            30 => { rax = (crate::simd_ops::ru64(rsi)) as u64; rdx = ((rbp as u32)) as u32 as u64; rcx = rsi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x58u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); r12 = (((r12 as u32)).wrapping_sub((rax as u32))) as u32 as u64; rax = ((r12.wrapping_add(rbp)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(r15.wrapping_add(0x43cu64))) as u32 as u64; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 21; continue; } bb = 31; continue; }
            31 => { rax = ((r12.wrapping_add(rbp)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(r15.wrapping_add(0x43cu64))) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 33; continue; } bb = 32; continue; }
            32 => { rax = (crate::simd_ops::ru64(r14)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = r14; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x58u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); r14 = 0; fl_l = 0; fl_r = 0; bb = 33; continue; }
            33 => { rcx = r15.wrapping_add(0x98); crate::simd_ops::wu32(r15.wrapping_add(0x440u64), ((r12 as u32)) as u32); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rax = r14; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141677ae0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1426452b0; const P1: u64 = 0x1426456f8; const P2: u64 = 0x1416776d0; const P3: u64 = 0x1415242e0; const P4: u64 = 0x1426452a8; const P5: u64 = 0x1434268d0; const P6: u64 = 0x1439318d4; const P7: u64 = 0x141677910;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0; let mut xmm7: u128 = 0; let mut xmm8: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 15>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x50u64)) as u64; rbp = rcx; rbx = rdx; rcx = ((rcx).wrapping_add(0x98u64)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rbp.wrapping_add(0x70); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru8(rbp.wrapping_add(0x68u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rdx = rbx; rcx = rbp; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r14 = rax; bb = 43; continue; }
            2 => { r14 = (crate::simd_ops::ru64(rbp.wrapping_add(0x80u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x80u64), rsi); rsi = 0; fl_l = 0; fl_r = 0; fl_l = r14; fl_r = r14; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 41; continue; } bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru8(r14.wrapping_add(0x78u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 10; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(r14)) as u64; rcx = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(r14.wrapping_add(0x18u64))) as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 9; continue; } bb = 5; continue; }
            5 => { fl_l = r14; fl_r = (crate::simd_ops::ru64(rbp.wrapping_add(0x80u64))) as u64; rbx = r14; r14 = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 7; continue; } bb = 6; continue; }
            6 => { crate::simd_ops::wu64(rbp.wrapping_add(0x80u64), rcx); bb = 8; continue; }
            7 => { crate::simd_ops::wu64(rsi.wrapping_add(0x18u64), rcx); bb = 8; continue; }
            8 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rbx; rcx = 0x7; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 12; continue; }
            9 => { rsi = r14; r14 = rcx; bb = 12; continue; }
            10 => { rcx = r14.wrapping_add(0x30); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rsi = ((crate::simd_ops::ru8(r14.wrapping_add(0x7cu64))) as u32) as u32 as u64; rcx = r14.wrapping_add(0x30); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rsi as u8)) as u8 as u64; fl_r = ((rsi as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 14; continue; } bb = 11; continue; }
            11 => { rsi = r14; r14 = (crate::simd_ops::ru64(r14.wrapping_add(0x18u64))) as u64; bb = 12; continue; }
            12 => { fl_l = r14; fl_r = r14; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } bb = 13; continue; }
            13 => { bb = 41; continue; }
            14 => { rax = (crate::simd_ops::ru64(r14)) as u64; rcx = r14; crate::simd_ops::wu64(rsp.wrapping_add(0x90u64), r12); crate::simd_ops::wu64(rsp.wrapping_add(0x98u64), r15); crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm6) as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x30u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r15 = (crate::simd_ops::ru64(r14.wrapping_add(0x18u64))) as u64; rbx = r14; r12 = ((crate::simd_ops::ru8(r14.wrapping_add(0x7au64))) as u32) as u32 as u64; xmm6 = (xmm0) as u128; fl_l = r15; fl_r = r15; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 40; continue; } bb = 15; continue; }
            15 => { crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm7) as u128); xmm7 = 0u128; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm8) as u128); xmm8 = ((crate::simd_ops::ru32(P5)) as u32) as u128; bb = 16; continue; }
            16 => { fl_l = (crate::simd_ops::ru8(r15.wrapping_add(0x78u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 23; continue; } bb = 17; continue; }
            17 => { rax = (crate::simd_ops::ru64(r15)) as u64; rcx = r15; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(r15.wrapping_add(0x18u64))) as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 22; continue; } bb = 18; continue; }
            18 => { fl_l = r15; fl_r = (crate::simd_ops::ru64(rbp.wrapping_add(0x80u64))) as u64; rsi = r15; r15 = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 20; continue; } bb = 19; continue; }
            19 => { crate::simd_ops::wu64(rbp.wrapping_add(0x80u64), rcx); bb = 21; continue; }
            20 => { crate::simd_ops::wu64(rbx.wrapping_add(0x18u64), rcx); bb = 21; continue; }
            21 => { rax = (crate::simd_ops::ru64(rsi)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rsi; rcx = 0x7; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 38; continue; }
            22 => { rbx = r15; r15 = rcx; bb = 38; continue; }
            23 => { rcx = r15.wrapping_add(0x30); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rsi = ((crate::simd_ops::ru8(r15.wrapping_add(0x7cu64))) as u32) as u32 as u64; rcx = r15.wrapping_add(0x30); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rsi as u8)) as u8 as u64; fl_r = ((rsi as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 37; continue; } bb = 24; continue; }
            24 => { rax = (crate::simd_ops::ru64(r15)) as u64; rcx = r15; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x30u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = ((crate::simd_ops::ru8(r15.wrapping_add(0x7au64))) as u32) as u32 as u64; xmm3 = (xmm0) as u128; rdx = (((rax as u8)) as u32) as u32 as u64; fl_l = ((r12 as u8)) as u8 as u64; fl_r = ((r12 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 27; continue; } bb = 25; continue; }
            25 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 28; continue; } bb = 26; continue; }
            26 => { r12 = (r12 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 36; continue; }
            27 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 37; continue; } bb = 28; continue; }
            28 => { fl_l = ((xmm3 as u32)) as u32 as u64; fl_r = ((xmm7 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) != f32::from_bits((fl_r as u32)) && !f32::from_bits((fl_l as u32)).is_nan() && !f32::from_bits((fl_r as u32)).is_nan()) { bb = 34; continue; } bb = 29; continue; }
            29 => { rax = ((crate::simd_ops::ru8(r15.wrapping_add(0x74u64))) as u32) as u32 as u64; rcx = ((crate::simd_ops::ru8(r14.wrapping_add(0x74u64))) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; if ((((fl_l as u8)) as i8) > (((fl_r as u8)) as i8)) { bb = 35; continue; } bb = 30; continue; }
            30 => { fl_l = ((xmm6 as u32)) as u32 as u64; fl_r = ((xmm7 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 35; continue; } bb = 31; continue; }
            31 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 37; continue; } bb = 32; continue; }
            32 => { rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x70u64))) as u64; xmm1 = 0u128; rax = rcx; xmm0 = 0u128; rax = ((rax).wrapping_sub(crate::simd_ops::ru64(r15.wrapping_add(0x58u64)))) as u64; xmm2 = (xmm8) as u128; xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) / f32::from_bits(crate::simd_ops::ru32(P6))).to_bits() as u128); rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(r14.wrapping_add(0x58u64)))) as u64; xmm1 = (xmm1 & !0xffff_ffffu128) | ((((rax) as i64) as f32).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((((rcx) as i64) as f32).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm2 as u32)).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm2 as u32)).to_bits() as u128); fl_l = ((xmm1 as u32)) as u32 as u64; fl_r = ((xmm0 as u32)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 37; continue; } bb = 33; continue; }
            33 => { bb = 35; continue; }
            34 => { fl_l = ((xmm3 as u32)) as u32 as u64; fl_r = ((xmm6 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 37; continue; } bb = 35; continue; }
            35 => { r12 = (((rdx as u8)) as u32) as u32 as u64; bb = 36; continue; }
            36 => { r14 = r15; xmm6 = (xmm3) as u128; bb = 37; continue; }
            37 => { rbx = r15; r15 = (crate::simd_ops::ru64(r15.wrapping_add(0x18u64))) as u64; bb = 38; continue; }
            38 => { fl_l = r15; fl_r = r15; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 16; continue; } bb = 39; continue; }
            39 => { xmm8 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; xmm7 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; bb = 40; continue; }
            40 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x88u64))) as u64; fl_l = ((r12 as u8)) as u8 as u64; fl_r = ((r12 as u8)) as u8 as u64; r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x98u64))) as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; crate::simd_ops::wu32(rax, ((xmm6 as u32)) as u32); xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x40u64))) as u128; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 42; continue; } bb = 41; continue; }
            41 => { rcx = rbp; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r14 = rax; bb = 42; continue; }
            42 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; bb = 43; continue; }
            43 => { rcx = rbp.wrapping_add(0x98); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = r14; rsp = ((rsp).wrapping_add(0x50u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141677d90(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 1>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); r11 = ((r8 as u32)) as u32 as u64; rbx = rdx; r10 = rcx; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 25; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rdi); rdi = (crate::simd_ops::ru32(rdx.wrapping_add(0x8u64))) as u32 as u64; fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 17; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rax = ((rdi as u32)) as u32 as u64; rsi = (crate::simd_ops::ru32(rcx.wrapping_add(0x70u64))) as u32 as u64; fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; r8 = ((rsi as u32)) as u32 as u64; r8 = (if (((fl_l as u32)) >= ((fl_r as u32))) { (rdi as u32) } else { (r8 as u32) }) as u32 as u64; rax = (if (((fl_l as u32)) >= ((fl_r as u32))) { (rsi as u32) } else { (rax as u32) }) as u32 as u64; rdx = 0; fl_l = 0; fl_r = 0; let d = ((r8 as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 16; continue; } bb = 3; continue; }
            3 => { rax = ((r8 as u32)) as u32 as u64; rcx = 0; fl_l = 0; fl_r = 0; rax = ((((rax as u32)) | ((rdx as u32)))) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 6; continue; } bb = 4; continue; }
            4 => { bb = 5; continue; }
            5 => { r8 = (((r8 as u32)).wrapping_shr(1u32)) as u32 as u64; rcx = (((rcx as u32)).wrapping_add(1)) as u32 as u64; rdx = (((rdx as u32)).wrapping_shr(1u32)) as u32 as u64; rax = ((r8 as u32)) as u32 as u64; rax = ((((rax as u32)) | ((rdx as u32)))) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 5; continue; } bb = 6; continue; }
            6 => { fl_l = ((r8 as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { r8 = (((r8 as u32)).wrapping_shr(1u32)) as u32 as u64; fl_l = ((r8 as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 7; continue; } bb = 8; continue; }
            8 => { fl_l = ((rdx as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { rdx = (((rdx as u32)).wrapping_shr(1u32)) as u32 as u64; fl_l = ((rdx as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 9; continue; } bb = 10; continue; }
            10 => { fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 15; continue; } bb = 11; continue; }
            11 => { fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; r9 = ((r8 as u32)) as u32 as u64; r9 = (if (((fl_l as u32)) >= ((fl_r as u32))) { (rdx as u32) } else { (r9 as u32) }) as u32 as u64; rdx = (if (((fl_l as u32)) >= ((fl_r as u32))) { (r8 as u32) } else { (rdx as u32) }) as u32 as u64; r8 = ((rdx as u32)) as u32 as u64; r8 = (((r8 as u32)).wrapping_sub((r9 as u32))) as u32 as u64; r8 = (((r8 as u32)).wrapping_shr(1u32)) as u32 as u64; fl_l = ((r8 as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 14; continue; } bb = 12; continue; }
            12 => { bb = 13; continue; }
            13 => { r8 = (((r8 as u32)).wrapping_shr(1u32)) as u32 as u64; fl_l = ((r8 as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 13; continue; } bb = 14; continue; }
            14 => { rdx = ((r9 as u32)) as u32 as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 11; continue; } bb = 15; continue; }
            15 => { fl_l = (((r8 as u32)).wrapping_shl(((rcx as u32) & 31))) as u32 as u64; r8 = ((fl_l as u32)) as u32 as u64; bb = 16; continue; }
            16 => { rdx = 0; fl_l = 0; fl_r = 0; rax = ((rdi as u32)) as u32 as u64; let d = ((r8 as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } rax = ((((rax as u32)) as i32).wrapping_mul(((rsi as u32)) as i32) as u32) as u32 as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x10u64))) as u64; crate::simd_ops::wu32(r10.wrapping_add(0xb0u64), ((rax as u32)) as u32); bb = 17; continue; }
            17 => { r8 = (crate::simd_ops::ru32(r10.wrapping_add(0xb0u64))) as u32 as u64; rdx = 0; fl_l = 0; fl_r = 0; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x18u64))) as u64; rax = ((r11 as u32)) as u32 as u64; let d = ((r8 as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } r9 = ((r11 as u32)) as u32 as u64; crate::simd_ops::wu32(r10.wrapping_add(0xacu64), ((r8 as u32)) as u32); r9 = (((r9 as u32)).wrapping_sub((rdx as u32))) as u32 as u64; crate::simd_ops::wu32(r10.wrapping_add(0xa4u64), ((r9 as u32)) as u32); rcx = (crate::simd_ops::ru32(rbx)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 21; continue; } bb = 18; continue; }
            18 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((r11 as u32)) as u32 as u64; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 24; continue; } bb = 19; continue; }
            19 => { rdx = 0; fl_l = 0; fl_r = 0; rax = ((rcx as u32)) as u32 as u64; let d = ((r8 as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 24; continue; } bb = 20; continue; }
            20 => { crate::simd_ops::wu32(r10.wrapping_add(0xa4u64), ((rcx as u32)) as u32); rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; crate::simd_ops::wu32(r10.wrapping_add(0xacu64), ((rax as u32)) as u32); rax = 0x1; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            21 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x4u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 26; continue; } bb = 22; continue; }
            22 => { fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 26; continue; } bb = 23; continue; }
            23 => { rcx = ((rax.wrapping_sub(0x1u64)) as u32) as u32 as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = (((rcx as u32)).wrapping_add((r8 as u32))) as u32 as u64; rax = ((rcx as u32)) as u32 as u64; let d = ((r8 as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } rcx = (((rcx as u32)).wrapping_sub((rdx as u32))) as u32 as u64; crate::simd_ops::wu32(r10.wrapping_add(0xacu64), ((rcx as u32)) as u32); fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 26; continue; } bb = 24; continue; }
            24 => { rax = 0x2; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            25 => { rcx = (crate::simd_ops::ru32(rcx.wrapping_add(0x70u64))) as u32 as u64; rdx = 0; fl_l = 0; fl_r = 0; rax = ((r11 as u32)) as u32 as u64; crate::simd_ops::wu32(r10.wrapping_add(0xacu64), ((rcx as u32)) as u32); let d = ((rcx as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } fl_l = ((r11 as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; r11 = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; crate::simd_ops::wu32(r10.wrapping_add(0xa4u64), ((r11 as u32)) as u32); bb = 26; continue; }
            26 => { rax = 0x1; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141677f00(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0xb0; const P1: u64 = 0xffffffffffffffff; const P2: u64 = 0x94;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    r8 = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64;
    r9 = ((rdx.wrapping_add(P1)) as u32) as u32 as u64;
    rdx = 0;
    fl_l = 0; fl_r = 0;
    rax = ((r9 as u32)) as u32 as u64;
    let d = ((r8 as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64);
    if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; }
    r8 = (((r8 as u32)).wrapping_sub((rdx as u32))) as u32 as u64;
    rdx = 0x800;
    rax = ((r8.wrapping_add(r9)) as u32) as u32 as u64;
    fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64;
    rax = (if (((fl_l as u32)) < ((fl_r as u32))) { (rdx as u32) } else { (rax as u32) }) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(P2), ((rax as u32)) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141677f30(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x28; const P1: u64 = 0x79; const P2: u64 = 0x1416739b0; const P3: u64 = 0x20;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rax = ((r8 as u32)) as u32 as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(P0), rdx);
    r8 = (crate::simd_ops::ru32(rsp.wrapping_add(0x50u64))) as u32 as u64;
    rbx = rcx;
    crate::simd_ops::wu8(rcx.wrapping_add(P1), (0x0u8) as u8);
    rdx = r9;
    rcx = ((rax as u32)) as u32 as u64;
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    crate::simd_ops::wu64(rbx.wrapping_add(P3), rax);
    fl_l = 0; fl_r = rax;
    rax = (0u64.wrapping_sub(fl_r as u64)) as u64;
    rax = (if (((fl_l as u64)) < ((fl_r as u64))) { 0xffffffffu32 } else { 0u32 }) as u32 as u64;
    rax = (((rax as u32)).wrapping_add(0x2u32)) as u32 as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141677f70(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x28; const P1: u64 = 0x79; const P2: u64 = 0x141673a30; const P3: u64 = 0x20;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rax = r8;
    crate::simd_ops::wu64(rcx.wrapping_add(P0), rdx);
    r8 = (crate::simd_ops::ru32(rsp.wrapping_add(0x50u64))) as u32 as u64;
    rbx = rcx;
    crate::simd_ops::wu8(rcx.wrapping_add(P1), (0x0u8) as u8);
    rdx = r9;
    rcx = rax;
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    crate::simd_ops::wu64(rbx.wrapping_add(P3), rax);
    fl_l = 0; fl_r = rax;
    rax = (0u64.wrapping_sub(fl_r as u64)) as u64;
    rax = (if (((fl_l as u64)) < ((fl_r as u64))) { 0xffffffffu32 } else { 0u32 }) as u32 as u64;
    rax = (((rax as u32)).wrapping_add(0x2u32)) as u32 as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141677fb0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1434268d0; const P1: u64 = 0x1426452b0; const P2: u64 = 0x141678970; const P3: u64 = 0x1426452a8; const P4: u64 = 0x14167d920; const P5: u64 = 0x1415242e0;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm0: u128 = 0; let mut xmm1: u128 = 0; let mut xmm2: u128 = 0; let mut xmm3: u128 = 0; let mut xmm6: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x48u64)) as u64; rax = ((crate::simd_ops::ru8(rdx.wrapping_add(0xdu64))) as u32) as u32 as u64; rsi = rdx; rdi = rcx; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x64u8) as u8 as u64; if (((fl_l as u8)) > ((fl_r as u8))) { bb = 43; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu8(rcx.wrapping_sub(0xcu64), ((rax as u8)) as u8); rax = (crate::simd_ops::ru64(rcx.wrapping_sub(0x58u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), rbp); crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm6) as u128); xmm6 = ((crate::simd_ops::ru32(rdx)) as u32) as u128; rbp = (crate::simd_ops::ru64(rax)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x8u64))) as u32 as u64; xmm6 = (xmm6 & !0xffff_ffffu128) | ((if f32::from_bits(xmm6 as u32) > f32::from_bits(crate::simd_ops::ru32(P0)) { f32::from_bits(xmm6 as u32) } else { f32::from_bits(crate::simd_ops::ru32(P0)) }).to_bits() as u128); fl_l = rax; fl_r = rbp; if ((((fl_l as u64)) as i64) <= (((fl_r as u64)) as i64)) { bb = 3; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_sub(0x7u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rbp = ((rax as u32)) as u32 as u64; bb = 4; continue; }
            4 => { crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rbx); fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(0x20u64))) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 9; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32 as u64; fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(0x1cu64))) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 9; continue; } bb = 6; continue; }
            6 => { rax = ((crate::simd_ops::ru8(rdx.wrapping_add(0xcu64))) as u32) as u32 as u64; rsi = 0x1; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; rsi = (if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { (rax as u32) } else { (rsi as u32) }) as u32 as u64; fl_l = ((xmm6 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rcx.wrapping_add(0x14u64))) as u32 as u64; if (f32::from_bits((fl_l as u32)) != f32::from_bits((fl_r as u32)) && !f32::from_bits((fl_l as u32)).is_nan() && !f32::from_bits((fl_r as u32)).is_nan()) { bb = 8; continue; } bb = 7; continue; }
            7 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0x51u64))) as u8 as u64; fl_r = ((rsi as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 42; continue; } bb = 8; continue; }
            8 => { rcx = ((rcx).wrapping_add(0xffffffffffffffb0u64)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rdi.wrapping_sub(0x80); crate::simd_ops::wu32(rdi.wrapping_add(0x14u64), ((xmm6 as u32)) as u32); crate::simd_ops::wu8(rdi.wrapping_add(0x51u64), ((rsi as u8)) as u8); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rdi.wrapping_sub(0x50); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 42; continue; }
            9 => { rbx = rcx.wrapping_sub(0x50); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), r13); rcx = rbx; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), r14); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), r15); crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), rbx); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(rdi.wrapping_add(0x14u64), ((xmm6 as u32)) as u32); rax = 0x1; rcx = ((crate::simd_ops::ru8(rsi.wrapping_add(0xcu64))) as u32) as u32 as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; rax = (if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { (rcx as u32) } else { (rax as u32) }) as u32 as u64; rdx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu8(rdi.wrapping_add(0x51u64), ((rax as u8)) as u8); rcx = rdi.wrapping_sub(0x80); r15 = (crate::simd_ops::ru32(rsi.wrapping_add(0x4u64))) as u32 as u64; rax = ((r15 as u32)) as u32 as u64; let d = (crate::simd_ops::ru32(rdi.wrapping_sub(0x10u64))) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } rax = (crate::simd_ops::ru64(rdi.wrapping_sub(0x80u64))) as u64; r15 = (((r15 as u32)).wrapping_sub((rdx as u32))) as u32 as u64; rdx = ((rbp as u32)) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x1cu64), ((r15 as u32)) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x88u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = ((crate::simd_ops::ru8(rdi.wrapping_add(0x50u64))) as u32) as u32 as u64; r11 = 0; fl_l = 0; fl_r = 0; rsi = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; rdx = ((r11 as u32)) as u32 as u64; rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x48u64))) as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 12; continue; } bb = 10; continue; }
            10 => { r8 = ((rax as u32)) as u32 as u64; bb = 11; continue; }
            11 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; rdx = rcx; rcx = (crate::simd_ops::ru64(rcx)) as u64; rsi = (crate::simd_ops::ru32(rax.wrapping_add(0x18u64))) as u32 as u64; rsi = ((rsi).wrapping_add(crate::simd_ops::ru64(rax))) as u64; fl_l = r8; fl_r = (0x1u64) as u64; r8 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 11; continue; } bb = 12; continue; }
            12 => { fl_l = ((rbp as u32)) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 15; continue; } bb = 13; continue; }
            13 => { rax = ((rbp as u32)) as u32 as u64; fl_l = rsi; fl_r = rax; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 15; continue; } bb = 14; continue; }
            14 => { rsi = ((r15 as u32)) as u32 as u64; bb = 15; continue; }
            15 => { r14 = r11; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 41; continue; } bb = 16; continue; }
            16 => { r8 = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; r10 = rcx; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x10u64))) as u32 as u64; r9 = (crate::simd_ops::ru64(r8)) as u64; rax = ((rax).wrapping_add(r9)) as u64; fl_l = rax; fl_r = rsi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 23; continue; } bb = 17; continue; }
            17 => { rax = (crate::simd_ops::ru64(rcx)) as u64; fl_l = rcx; fl_r = (crate::simd_ops::ru64(rdi.wrapping_add(0x48u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 19; continue; } bb = 18; continue; }
            18 => { crate::simd_ops::wu64(rdi.wrapping_add(0x48u64), rax); bb = 20; continue; }
            19 => { crate::simd_ops::wu64(rdx, rax); bb = 20; continue; }
            20 => { fl_l = rcx; fl_r = (crate::simd_ops::ru64(rdi.wrapping_add(0x40u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 22; continue; } bb = 21; continue; }
            21 => { crate::simd_ops::wu64(rdi.wrapping_add(0x40u64), rdx); bb = 22; continue; }
            22 => { crate::simd_ops::lk_sub_u32(rdi.wrapping_add(0x38u64), (1) as u32); rcx = rax; fl_l = r14; fl_r = r14; r14 = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { r11 } else { r14 }) as u64; crate::simd_ops::wu64(r10, r14); r14 = r10; bb = 27; continue; }
            23 => { rsi = (crate::simd_ops::ru32(r8.wrapping_add(0x18u64))) as u32 as u64; rsi = ((rsi).wrapping_add(r9)) as u64; fl_l = ((rbp as u32)) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 26; continue; } bb = 24; continue; }
            24 => { rax = ((rbp as u32)) as u32 as u64; fl_l = rsi; fl_r = rax; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 26; continue; } bb = 25; continue; }
            25 => { rsi = ((r15 as u32)) as u32 as u64; bb = 26; continue; }
            26 => { rdx = rcx; rcx = (crate::simd_ops::ru64(rcx)) as u64; bb = 27; continue; }
            27 => { fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 16; continue; } bb = 28; continue; }
            28 => { fl_l = r14; fl_r = r14; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 41; continue; } bb = 29; continue; }
            29 => { crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), r12); r12 = (crate::simd_ops::ru64(rdi.wrapping_sub(0x20u64))) as u64; rcx = r12.wrapping_add(0x18); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = 0; fl_l = 0; fl_r = 0; r15 = r14; bb = 30; continue; }
            30 => { rax = (crate::simd_ops::ru64(r15)) as u64; rbp = r15; fl_l = r15; fl_r = r14; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 32; continue; } bb = 31; continue; }
            31 => { r14 = rax; bb = 33; continue; }
            32 => { crate::simd_ops::wu64(rbx, rax); bb = 33; continue; }
            33 => { r9 = (crate::simd_ops::ru32(rbp.wrapping_add(0x10u64))) as u32 as u64; r15 = rax; rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x8u64))) as u64; rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x20u64))) as u32 as u64; r8 = (crate::simd_ops::ru64(rax)) as u64; rdx = rax.wrapping_add(0x18); rax = ((r8.wrapping_add(r9)) as u64) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 36; continue; } bb = 34; continue; }
            34 => { rax = (crate::simd_ops::ru32(rdx)) as u32 as u64; rax = ((rax).wrapping_add(r8)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 36; continue; } bb = 35; continue; }
            35 => { rcx = (((rcx as u32)).wrapping_sub((r9 as u32))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 37; continue; }
            36 => { rcx = (crate::simd_ops::ru32(rdx)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 37; continue; }
            37 => { crate::simd_ops::lk_sub_u32(rdi.wrapping_add(0x28u64), ((rcx as u32)) as u32); rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x8u64))) as u64; rcx = (crate::simd_ops::ru64(rdi.wrapping_sub(0x20u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x8u64), rbx); fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 39; continue; } bb = 38; continue; }
            38 => { rcx = ((rcx).wrapping_add(0xc0u64)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 39; continue; }
            39 => { rdx = rbp; rcx = 0x7; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = r15; fl_r = r15; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 30; continue; } bb = 40; continue; }
            40 => { rcx = r12.wrapping_add(0x18); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; bb = 41; continue; }
            41 => { r8 = (crate::simd_ops::ru64(rdi.wrapping_sub(0x80u64))) as u64; rcx = rdi.wrapping_sub(0x80); rdx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(r8.wrapping_add(0x78u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rdi.wrapping_sub(0x80); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; bb = 42; continue; }
            42 => { xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; rax = 0x1; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsp = ((rsp).wrapping_add(0x48u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            43 => { rax = 0x1f; rsp = ((rsp).wrapping_add(0x48u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141678280(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    xmm0 = (crate::simd_ops::ru128(rdx)) as u128;
    rax = 0x1;
    crate::simd_ops::wu128(rcx.wrapping_add(P0), (xmm0) as u128);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416782b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1426452b0; const P1: u64 = 0x141678970; const P2: u64 = 0x14167c7d0; const P3: u64 = 0x1426452a8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; r15 = (crate::simd_ops::ru32(rcx.wrapping_add(0x2cu64))) as u32 as u64; r14 = rcx; rcx = ((rcx).wrapping_add(0xffffffffffffffb0u64)) as u64; rbp = ((rdx as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(r14.wrapping_sub(0x20u64))) as u64; rdx = 0; fl_l = 0; fl_r = 0; r8 = (crate::simd_ops::ru32(r14.wrapping_add(0x30u64))) as u32 as u64; rcx = ((r8 as u32)) as u32 as u64; r9 = (crate::simd_ops::ru32(rax.wrapping_add(0x430u64))) as u32 as u64; rax = ((r9 as u32)) as u32 as u64; let d = ((r8 as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } crate::simd_ops::wu32(r14.wrapping_add(0x2cu64), ((r8 as u32)) as u32); r9 = (((r9 as u32)).wrapping_sub((rdx as u32))) as u32 as u64; crate::simd_ops::wu32(r14.wrapping_add(0x24u64), ((r9 as u32)) as u32); fl_l = ((rbp as u32)) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 5; continue; } bb = 1; continue; }
            1 => { fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 5; continue; } bb = 2; continue; }
            2 => { rdx = 0; fl_l = 0; fl_r = 0; rcx = ((rbp.wrapping_sub(0x1u64)) as u32) as u32 as u64; rcx = (((rcx as u32)).wrapping_add((r8 as u32))) as u32 as u64; rax = ((rcx as u32)) as u32 as u64; let d = ((r8 as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } rcx = (((rcx as u32)).wrapping_sub((rdx as u32))) as u32 as u64; crate::simd_ops::wu32(r14.wrapping_add(0x2cu64), ((rcx as u32)) as u32); fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 5; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(r14)) as u64; rcx = r14; rsi = 0x2; crate::simd_ops::wu8(r14.wrapping_add(0x53u64), (0x1u8) as u8); rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x68u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = r14.wrapping_sub(0x80); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru8(r14.wrapping_sub(0x70u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 7; continue; } bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru64(r14.wrapping_sub(0x20u64))) as u64; rdx = r14.wrapping_sub(0x80); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 7; continue; }
            5 => { rsi = 0x1; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((r15 as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 7; continue; } bb = 6; continue; }
            6 => { r8 = (crate::simd_ops::ru64(r14.wrapping_sub(0x80u64))) as u64; rcx = r14.wrapping_sub(0x80); rdx = ((rbp as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(r8.wrapping_add(0x80u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = r14.wrapping_sub(0x80); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 7; continue; }
            7 => { rcx = r14.wrapping_sub(0x50); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rax = ((rsi as u32)) as u32 as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416783a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1426452b0; const P1: u64 = 0x141676780; const P2: u64 = 0x141678970; const P3: u64 = 0x1426452a8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; r15 = r9; rbx = ((r8 as u32)) as u32 as u64; r14 = rdx; rdi = rcx; fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(r9, (0x0u64) as u64); bb = 2; continue; }
            2 => { fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 7; continue; } bb = 3; continue; }
            3 => { fl_l = ((rbx as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x70u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = ((r14).wrapping_add(rax)) as u64; fl_r = fl_l; r14 = ((fl_l as u64)) as u64; bb = 7; continue; }
            5 => { fl_l = ((rbx as u32)) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 23; continue; } bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(rcx.wrapping_sub(0x58u64))) as u64; fl_l = ((r14).wrapping_add(crate::simd_ops::ru64(rax))) as u64; fl_r = fl_l; r14 = ((fl_l as u64)) as u64; bb = 7; continue; }
            7 => { fl_l = r14; fl_r = r14; if (((((fl_l as u64)) & ((fl_r as u64))) as i64) < 0) { bb = 23; continue; } bb = 8; continue; }
            8 => { rcx = (crate::simd_ops::ru32(rdi.wrapping_sub(0x10u64))) as u32 as u64; rax = r14; rdx = if (rax as i64) < 0 { 0xffffffffffffffffu64 } else { 0u64 }; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rsi); let d = (rcx) as i64 as i128; let n = (((rdx as u128) << 64) | (rax as u128)) as i128; if d != 0 { rax = n.wrapping_div(d) as i64 as u64; rdx = n.wrapping_rem(d) as i64 as u64; } rsi = r14; rsi = ((rsi).wrapping_sub(rdx)) as u64; fl_l = rdx; fl_r = rdx; rsi = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { r14 } else { rsi }) as u64; fl_l = r15; fl_r = r15; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 15; continue; } bb = 9; continue; }
            9 => { fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 14; continue; } bb = 10; continue; }
            10 => { fl_l = ((rbx as u32)) as u32 as u64; fl_r = 0x1; rbx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 13; continue; } bb = 11; continue; }
            11 => { fl_l = ((rbx as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 15; continue; } bb = 12; continue; }
            12 => { rax = (crate::simd_ops::ru64(rdi.wrapping_sub(0x58u64))) as u64; rcx = rsi; fl_l = rcx; fl_r = (crate::simd_ops::ru64(rax)) as u64; rcx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; crate::simd_ops::wu64(r15, rcx); bb = 15; continue; }
            13 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rdi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x70u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = rsi; fl_l = rcx; fl_r = rax; rcx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; crate::simd_ops::wu64(r15, rcx); bb = 15; continue; }
            14 => { crate::simd_ops::wu64(r15, rsi); bb = 15; continue; }
            15 => { rdi = ((rdi).wrapping_add(0xffffffffffffff80u64)) as u64; rcx = rdi.wrapping_add(0x30); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); rcx = ((crate::simd_ops::ru8(rdi.wrapping_add(0xd0u64))) as u32) as u32 as u64; crate::simd_ops::wu64(rdi.wrapping_add(0x88u64), rsi); fl_l = ((rcx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rdi.wrapping_add(0xb8u64))) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 20; continue; } bb = 16; continue; }
            16 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(0xc8u64))) as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 18; continue; } bb = 17; continue; }
            17 => { rax = (crate::simd_ops::ru64(rax)) as u64; fl_l = rcx; fl_r = (0x1u64) as u64; rcx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 17; continue; } bb = 18; continue; }
            18 => { rdx = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; rcx = rdi; r8 = (crate::simd_ops::ru32(rax.wrapping_add(0x10u64))) as u32 as u64; r8 = ((r8).wrapping_add(crate::simd_ops::ru64(rdx))) as u64; fl_l = r8; fl_r = rsi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 21; continue; } bb = 19; continue; }
            19 => { rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 22; continue; }
            20 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rdx = rsi; rcx = rdi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x78u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = rdi; bb = 21; continue; }
            21 => { rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 22; continue; }
            22 => { rcx = rdi.wrapping_add(0x30); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = 0x1; bb = 24; continue; }
            23 => { rax = 0x1f; bb = 24; continue; }
            24 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416784f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x30; const P1: u64 = 0x28;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = rdx; bb = 7; continue; }
            2 => { fl_l = ((r8 as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; bb = 6; continue; }
            4 => { fl_l = ((r8 as u32)) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 11; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; bb = 6; continue; }
            6 => { fl_l = ((rax).wrapping_add(rdx)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; bb = 7; continue; }
            7 => { fl_l = rax; fl_r = rax; if (((((fl_l as u64)) & ((fl_r as u64))) as i64) < 0) { bb = 11; continue; } bb = 8; continue; }
            8 => { fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { crate::simd_ops::wu64(r9, rdx); bb = 10; continue; }
            10 => { crate::simd_ops::wu64(rcx.wrapping_add(P0), rax); rax = 0x1; return rax; }
            11 => { rax = 0x1f; return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141678530(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x38; const P1: u64 = 0x8; const P2: u64 = 0xffffffffffffffa8; const P3: u64 = 0xfffffffffffffff0; const P4: u64 = 0xfffffffffffffff7;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rbx = r9; r11 = rdx; r10 = rcx; fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(r9, (0x0u64) as u64); bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 18; continue; } bb = 3; continue; }
            3 => { fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 8; continue; } bb = 4; continue; }
            4 => { fl_l = ((r8 as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 6; continue; } bb = 5; continue; }
            5 => { fl_l = ((r11).wrapping_add(crate::simd_ops::ru64(rcx.wrapping_add(P1)))) as u64; fl_r = fl_l; r11 = ((fl_l as u64)) as u64; bb = 8; continue; }
            6 => { fl_l = ((r8 as u32)) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 17; continue; } bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64; fl_l = ((r11).wrapping_add(crate::simd_ops::ru64(rax))) as u64; fl_r = fl_l; r11 = ((fl_l as u64)) as u64; bb = 8; continue; }
            8 => { fl_l = r11; fl_r = r11; if (((((fl_l as u64)) & ((fl_r as u64))) as i64) < 0) { bb = 17; continue; } bb = 9; continue; }
            9 => { rcx = (crate::simd_ops::ru32(rcx.wrapping_add(P3))) as u32 as u64; rax = r11; rdx = if (rax as i64) < 0 { 0xffffffffffffffffu64 } else { 0u64 }; r9 = r11; let d = (rcx) as i64 as i128; let n = (((rdx as u128) << 64) | (rax as u128)) as i128; if d != 0 { rax = n.wrapping_div(d) as i64 as u64; rdx = n.wrapping_rem(d) as i64 as u64; } r9 = ((r9).wrapping_sub(rdx)) as u64; fl_l = rdx; fl_r = rdx; r9 = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { r11 } else { r9 }) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 16; continue; } bb = 10; continue; }
            10 => { fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 15; continue; } bb = 11; continue; }
            11 => { fl_l = ((r8 as u32)) as u32 as u64; fl_r = 0x1; r8 = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 14; continue; } bb = 12; continue; }
            12 => { fl_l = ((r8 as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 18; continue; } bb = 13; continue; }
            13 => { rax = (crate::simd_ops::ru64(r10.wrapping_add(P2))) as u64; rcx = r9; fl_l = rcx; fl_r = (crate::simd_ops::ru64(rax)) as u64; rcx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; crate::simd_ops::wu64(rbx, rcx); bb = 16; continue; }
            14 => { rax = r9; fl_l = rax; fl_r = (crate::simd_ops::ru64(r10.wrapping_add(P1))) as u64; rax = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; crate::simd_ops::wu64(rbx, rax); bb = 16; continue; }
            15 => { crate::simd_ops::wu64(rbx, r9); bb = 16; continue; }
            16 => { rax = (crate::simd_ops::ru64(r10.wrapping_add(P2))) as u64; crate::simd_ops::wu64(r10.wrapping_add(P1), r9); fl_l = r9; fl_r = (crate::simd_ops::ru64(rax)) as u64; rax = (rax & !0xffu64) | (((if ((((fl_l as u64)) as i64) >= (((fl_r as u64)) as i64)) { 1u8 } else { 0u8 }) as u8 as u64)); crate::simd_ops::wu8(r10.wrapping_add(P4), ((rax as u8)) as u8); rax = 0x1; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x8u64))) as u64; return rax; }
            17 => { rax = 0x1f; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x8u64))) as u64; return rax; }
            18 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x8u64))) as u64; rax = 0x2; return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141678600(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x78; const P1: u64 = 0xb8; const P2: u64 = 0x8; const P3: u64 = 0x7a; const P4: u64 = 0x7c; const P5: u64 = 0x60; const P6: u64 = 0x14167c820; const P7: u64 = 0x14167c7e0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut fl_k: u32 = 0;
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(P0))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; rdi = ((rdx as u32)) as u32 as u64; rbx = (rcx) as u64; crate::simd_ops::wu32(rcx.wrapping_add(P1), ((rdx as u32)) as u32); if (((fl_l as u8)) == ((fl_r as u8))) { fl_k = 0; bb = 3; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rcx)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { fl_k = 1; bb = 3; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(P3))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; fl_k = 0; bb = 5; continue; }
            3 => { rax = ((crate::simd_ops::ru8(rbx.wrapping_add(P3))) as u32) as u32 as u64; fl_l = ((rdi as u32)) as u32 as u64; fl_r = (0x2u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 7; continue; } bb = 4; continue; }
            4 => { crate::simd_ops::wu8(rbx.wrapping_add(P4), (0x1u8) as u8); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; fl_k = 1; bb = 5; continue; }
            5 => { if (match fl_k { 0 => (((fl_l as u8)) != ((fl_r as u8))), 1 => ((((fl_l as u8)) & ((fl_r as u8))) != 0), _ => false }) { bb = 9; continue; } bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P5))) as u64; crate::simd_ops::wu8(rbx.wrapping_add(P3), (0x1u8) as u8); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); let s5 = crate::simd_ops::read_s5_tail(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x40) as *const u64); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); }
            7 => { crate::simd_ops::wu8(rbx.wrapping_add(P4), (0x0u8) as u8); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P5))) as u64; crate::simd_ops::wu8(rbx.wrapping_add(P3), (0x0u8) as u8); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 9; continue; }
            9 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416787a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x52; const P1: u64 = 0x1426452b0; const P2: u64 = 0xfffffffffffffff7; const P3: u64 = 0xffffffffffffff80; const P4: u64 = 0xfffffffffffffff8; const P5: u64 = 0xfffffffffffffffc; const P6: u64 = 0x141678970; const P7: u64 = 0xffffffffffffffe0; const P8: u64 = 0x54; const P9: u64 = 0x70; const P10: u64 = 0xffffffffffffffd8; const P11: u64 = 0xffffffffffffffb0; const P12: u64 = 0x1426452a8; const P13: u64 = 0x14167c740;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(P0))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; rdi = rcx; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 6; continue; } bb = 1; continue; }
            1 => { rcx = ((rcx).wrapping_add(0xffffffffffffffb0u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(P2))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; rcx = ((rdi.wrapping_add(P3)) as u64) as u64; crate::simd_ops::wu8(rdi.wrapping_add(P0), (0x1u8) as u8); if (((fl_l as u8)) != ((fl_r as u8))) { bb = 4; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(P4))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 5; continue; }
            4 => { rax = 0; fl_l = 0; fl_r = 0; bb = 5; continue; }
            5 => { crate::simd_ops::wu8(rdi.wrapping_add(P5), ((rax as u8)) as u8); rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rdi.wrapping_add(P7))) as u64; crate::simd_ops::wu8(rdi.wrapping_add(P8), (0x1u8) as u8); rcx = (crate::simd_ops::ru64(rax.wrapping_add(P9))) as u64; crate::simd_ops::wu64(rdi.wrapping_add(P10), rcx); rcx = ((rdi.wrapping_add(P11)) as u64) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P12), rax), rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rdi.wrapping_sub(0x20u64))) as u64; rcx = rbx.wrapping_add(0x18); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rdi.wrapping_sub(0x20u64))) as u64; rax = crate::simd_ops::call_stk(P13, rax, rcx, rdx, r8, r9, rsp); rcx = rbx.wrapping_add(0x18); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P12), rax), rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; bb = 6; continue; }
            6 => { rax = 0; fl_l = 0; fl_r = 0; fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(0x53u64))) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; rax = (rax & !0xffu64) | (((if (((fl_l as u8)) != ((fl_r as u8))) { 1u8 } else { 0u8 }) as u8 as u64)); rax = (((rax as u32)).wrapping_add(1)) as u32 as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141678830(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xd4; const P1: u64 = 0x60;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(P0))) as u8 as u64; fl_r = (0x0u8) as u8 as u64;
    if !((((fl_l as u8)) == ((fl_r as u8)))) { fl_l = rcx; fl_r = (0xffffffffffffff80u64) as u64; rcx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; return crate::simd_ops::call_fwd(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141678850(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1426452b0; const P1: u64 = 0xffffffffffffff80; const P2: u64 = 0x52; const P3: u64 = 0xfffffffffffffffc; const P4: u64 = 0x141676780; const P5: u64 = 0xffffffffffffffb0; const P6: u64 = 0x1426452a8;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdi = rcx;
    rcx = ((rcx).wrapping_add(0xffffffffffffffb0u64)) as u64;
    rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp);
    rcx = ((rdi.wrapping_add(P1)) as u64) as u64;
    crate::simd_ops::wu8(rdi.wrapping_add(P2), (0x0u8) as u8);
    crate::simd_ops::wu8(rdi.wrapping_add(P3), (0x0u8) as u8);
    rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp);
    rcx = ((rdi.wrapping_add(P5)) as u64) as u64;
    rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rax = 0x1;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416788a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1426452b0; const P1: u64 = 0x60; const P2: u64 = 0xd2; const P3: u64 = 0x7c; const P4: u64 = 0x141678970; const P5: u64 = 0x30; const P6: u64 = 0x1426452a8;
    let (mut rax, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rsi = rcx;
    rdi = ((rdx as u32)) as u32 as u64;
    rcx = ((rcx).wrapping_add(0x30u64)) as u64;
    rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp);
    rax = (crate::simd_ops::ru64(rsi)) as u64;
    rdx = ((rdi as u32)) as u32 as u64;
    rcx = rsi;
    rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp);
    rcx = rsi;
    crate::simd_ops::wu8(rsi.wrapping_add(P2), (0x0u8) as u8);
    rdi = ((rax as u32)) as u32 as u64;
    crate::simd_ops::wu8(rsi.wrapping_add(P3), (0x0u8) as u8);
    rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp);
    rcx = ((rsi.wrapping_add(P5)) as u64) as u64;
    rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rax = ((rdi as u32)) as u32 as u64;
    rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141678900(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x74; const P1: u64 = 0x98; const P2: u64 = 0x1426452b0; const P3: u64 = 0x90; const P4: u64 = 0x50; const P5: u64 = 0x18; const P6: u64 = 0x1426452a8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdi = rcx;
    fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(P0))) as u8 as u64; fl_r = ((r8 as u8)) as u8 as u64;
    if (((fl_l as u8)) == ((fl_r as u8))) { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx);
    fl_l = ((r8 as u8)) as u8 as u64; fl_r = (0x64u8) as u8 as u64;
    if !((((fl_l as u8)) > ((fl_r as u8)))) { crate::simd_ops::wu8(rdx.wrapping_add(P0), ((r8 as u8)) as u8); }
    rbx = ((rcx.wrapping_add(P1)) as u64) as u64;
    rcx = rbx;
    rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp);
    rdi = (crate::simd_ops::ru64(rdi.wrapping_add(P3))) as u64;
    fl_l = rdi; fl_r = rdi;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rcx = rbx; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    loop { rax = (crate::simd_ops::ru64(rdi)) as u64; rcx = rdi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P4))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rdi = (crate::simd_ops::ru64(rdi.wrapping_add(P5))) as u64; fl_l = rdi; fl_r = rdi; if !(((((fl_l as u64)) & ((fl_r as u64))) != 0)) { break; } }
    rcx = rbx;
    rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141678970(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1426452b0; const P1: u64 = 0x1426452a8; const P2: u64 = 0x14167c3e0; const P3: u64 = 0x14167c3a0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(0xa0u64))) as u32 as u64; fl_r = 0x0; rsi = rcx; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rdi); if (((fl_l as u32)) != ((fl_r as u32))) { bb = 12; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x28u64))) as u64; rbx = (crate::simd_ops::ru64(rax)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x68u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rbx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 12; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru8(rsi.wrapping_add(0x79u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 12; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu8(rsi.wrapping_add(0x77u64), (0x1u8) as u8); bb = 4; continue; }
            4 => { rbp = rsi.wrapping_add(0x78); bb = 5; continue; }
            5 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 6; continue; }
            6 => { rcx = rsi.wrapping_add(0x30); crate::simd_ops::wu8(rsi.wrapping_add(0x7cu64), ((rax as u8)) as u8); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); rdi = ((crate::simd_ops::ru8(rsi.wrapping_add(0x7cu64))) as u32) as u32 as u64; rcx = rsi.wrapping_add(0x30); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = ((rdi as u8)) as u8 as u64; fl_r = ((rdi as u8)) as u8 as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru64(rsi)) as u64; rcx = rsi; rbx = (crate::simd_ops::ru32(rsi.wrapping_add(0xa8u64))) as u32 as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x40u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 10; continue; } bb = 8; continue; }
            8 => { fl_l = (crate::simd_ops::ru8(rbp)) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 15; continue; } bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru64(rsi)) as u64; rcx = rsi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 15; continue; } bb = 10; continue; }
            10 => { fl_l = (crate::simd_ops::ru8(rsi.wrapping_add(0x7au64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 17; continue; } bb = 11; continue; }
            11 => { rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x60u64))) as u64; crate::simd_ops::wu8(rsi.wrapping_add(0x7au64), (0x1u8) as u8); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P2, rax, rcx, rdx, r8, r9, rsp); }
            12 => { fl_l = (crate::simd_ops::ru8(rsi.wrapping_add(0xd2u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; crate::simd_ops::wu8(rsi.wrapping_add(0x77u64), (0x0u8) as u8); if (((fl_l as u8)) == ((fl_r as u8))) { bb = 4; continue; } bb = 13; continue; }
            13 => { fl_l = (crate::simd_ops::ru8(rsi.wrapping_add(0x78u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; rbp = rsi.wrapping_add(0x78); if (((fl_l as u8)) != ((fl_r as u8))) { bb = 5; continue; } bb = 14; continue; }
            14 => { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 6; continue; }
            15 => { fl_l = (crate::simd_ops::ru8(rsi.wrapping_add(0x7au64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 17; continue; } bb = 16; continue; }
            16 => { rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x60u64))) as u64; crate::simd_ops::wu8(rsi.wrapping_add(0x7au64), (0x0u8) as u8); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 17; continue; }
            17 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141678a70(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xd3; const P1: u64 = 0x68; const P2: u64 = 0x141678970; const P3: u64 = 0x10; const P4: u64 = 0x60; const P5: u64 = 0x14167c7d0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x2;
    if !((((fl_l as u32)) != ((fl_r as u32)))) { crate::simd_ops::wu8(rcx.wrapping_add(P0), (0x1u8) as u8); rcx = ((rcx).wrapping_sub(0xffffffffffffff80u64)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); }
    rcx = rbx;
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(P3))) as u8 as u64; fl_r = (0x0u8) as u8 as u64;
    if !((((fl_l as u8)) == ((fl_r as u8)))) { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P4))) as u64; rdx = rbx; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P5, rax, rcx, rdx, r8, r9, rsp); }
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141678ac0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xb8; const P1: u64 = 0xbc; const P2: u64 = 0xa0; const P3: u64 = 0x88; const P4: u64 = 0x78; const P5: u64 = 0x8; const P6: u64 = 0x7a; const P7: u64 = 0x60; const P8: u64 = 0x14167c820; const P9: u64 = 0x7c; const P10: u64 = 0x14167c7e0; const P11: u64 = 0x10; const P12: u64 = 0x14167c7d0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rcx; fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu32(rcx.wrapping_add(P0), (0x4u32) as u32); bb = 4; continue; }
            2 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(P1))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 10; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru32(rcx.wrapping_add(P2))) as u32 as u64; fl_l = (crate::simd_ops::lk_add_u64(rcx.wrapping_add(P3), (rax) as u64)) as u64; fl_r = fl_l; crate::simd_ops::wu32(rcx.wrapping_add(P0), (0x1u32) as u32); bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(P4))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 8; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P5))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 8; continue; } bb = 6; continue; }
            6 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(P6))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 10; continue; } bb = 7; continue; }
            7 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P7))) as u64; crate::simd_ops::wu8(rbx.wrapping_add(P6), (0x1u8) as u8); rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); bb = 10; continue; }
            8 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(P6))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; crate::simd_ops::wu8(rbx.wrapping_add(P9), (0x0u8) as u8); if (((fl_l as u8)) == ((fl_r as u8))) { bb = 10; continue; } bb = 9; continue; }
            9 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P7))) as u64; crate::simd_ops::wu8(rbx.wrapping_add(P6), (0x0u8) as u8); rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp); bb = 10; continue; }
            10 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(P11))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 13; continue; } bb = 11; continue; }
            11 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0xb8u64))) as u32 as u64; fl_r = 0xfffffffd; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 13; continue; } bb = 12; continue; }
            12 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x60u64))) as u64; rdx = rbx; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P12, rax, rcx, rdx, r8, r9, rsp); }
            13 => { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141678b60(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1426452b0; const P1: u64 = 0x60; const P2: u64 = 0xffffffffffffff80; const P3: u64 = 0x14167c780; const P4: u64 = 0xffffffffffffffb0; const P5: u64 = 0x1426452a8; const P6: u64 = 0xffffffffffffffe0; const P7: u64 = 0x14167c920;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdi = rcx;
    rcx = ((rcx).wrapping_add(0xffffffffffffffb0u64)) as u64;
    rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp);
    rax = (crate::simd_ops::ru64(rdi)) as u64;
    rcx = rdi;
    rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp);
    fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x2;
    if !((((fl_l as u32)) != ((fl_r as u32)))) { rcx = ((rdi.wrapping_add(P2)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = ((rdi.wrapping_add(P4)) as u64) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P6))) as u64; rdx = ((rdi.wrapping_add(P2)) as u64) as u64; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; } else { rcx = ((rdi.wrapping_add(P4)) as u64) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); }
    rax = (crate::simd_ops::ru64(rdi)) as u64;
    rcx = rdi;
    rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141678bd0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1416764c0;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = ((rsp).wrapping_sub(0x48u64)) as u64;
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64;
    rcx = ((rcx).wrapping_add(0xffffffffffffff80u64)) as u64;
    xmm0 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x78u64))) as u32) as u128;
    crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rax);
    rax = ((crate::simd_ops::ru8(rsp.wrapping_add(0x70u64))) as u32) as u32 as u64;
    crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((xmm0 as u32)) as u32);
    crate::simd_ops::wu8(rsp.wrapping_add(0x28u64), ((rax as u8)) as u8);
    crate::simd_ops::wu8(rsp.wrapping_add(0x20u64), ((r9 as u8)) as u8);
    r9 = ((r8 as u32)) as u32 as u64;
    r8 = rdx;
    rdx = (rdx & !0xffu64) | (((0x1u8) as u8 as u64));
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rsp = ((rsp).wrapping_add(0x48u64)) as u64;
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141678c60(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142694060; const P1: u64 = 0x1426940f0; const P2: u64 = 0x80; const P3: u64 = 0x141675970; const P4: u64 = 0x141834d30;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rax = P0;
    rbx = ((rdx as u32)) as u32 as u64;
    crate::simd_ops::wu64(rcx, rax);
    rdi = rcx;
    rax = P1;
    crate::simd_ops::wu64(rcx.wrapping_add(P2), rax);
    rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp);
    fl_l = ((rbx as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64;
    if !(((((fl_l as u8)) & ((fl_r as u8))) == 0)) { rdx = 0xe8; rcx = rdi; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rax = rdi;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141678cb0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142693f28; const P1: u64 = 0x1416759c0; const P2: u64 = 0x141834d30;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rax = P0;
    rbx = ((rdx as u32)) as u32 as u64;
    crate::simd_ops::wu64(rcx, rax);
    rdi = rcx;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    fl_l = ((rbx as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64;
    if !(((((fl_l as u8)) & ((fl_r as u8))) == 0)) { rdx = 0x460; rcx = rdi; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rax = rdi;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141678cf0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142693f88; const P1: u64 = 0x142693ff0; const P2: u64 = 0x80; const P3: u64 = 0x141675a00; const P4: u64 = 0x141834d30;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rax = P0;
    rbx = ((rdx as u32)) as u32 as u64;
    crate::simd_ops::wu64(rcx, rax);
    rdi = rcx;
    rax = P1;
    crate::simd_ops::wu64(rcx.wrapping_add(P2), rax);
    rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp);
    fl_l = ((rbx as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64;
    if !(((((fl_l as u8)) & ((fl_r as u8))) == 0)) { rdx = 0xd0; rcx = rdi; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rax = rdi;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141678de0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1426452b0; const P1: u64 = 0xffffffffffffff80; const P2: u64 = 0x141678600; const P3: u64 = 0x40; const P4: u64 = 0x48; const P5: u64 = 0x14167c780; const P6: u64 = 0xffffffffffffffb0; const P7: u64 = 0x1426452a8; const P8: u64 = 0xffffffffffffffe0; const P9: u64 = 0x14167c920;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    rcx = ((rcx).wrapping_add(0xffffffffffffffb0u64)) as u64;
    rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp);
    rdx = 0x3;
    rcx = ((rbx.wrapping_add(P1)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(P3))) as u64; fl_r = (0x0u64) as u64;
    if !((((fl_l as u64)) == ((fl_r as u64)))) { rcx = ((rbx.wrapping_add(P1)) as u64) as u64; crate::simd_ops::wu8(rbx.wrapping_add(P4), (0x1u8) as u8); rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rcx = ((rbx.wrapping_add(P6)) as u64) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P7), rax), rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P8))) as u64; rdx = ((rbx.wrapping_add(P1)) as u64) as u64; rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); rcx = ((rbx.wrapping_add(P6)) as u64) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu8(rbx.wrapping_add(P4), (0x0u8) as u8); }
    rcx = ((rbx.wrapping_add(P6)) as u64) as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P7), rax), rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141678e70(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0xd8; const P1: u64 = 0x14; const P2: u64 = 0xa0; const P3: u64 = 0x8; const P4: u64 = 0x10; const P5: u64 = 0x18; const P6: u64 = 0xa8; const P7: u64 = 0xe0;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; r8 = rcx; rax = (crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32 as u64; rax = ((((rax as u32)) & (0xfffffffau32))) as u32 as u64; rax = ((((rax as u32)) | (0x2u32))) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_add(P1), ((rax as u32)) as u32); rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rcx = (crate::simd_ops::ru32(rcx.wrapping_add(P2))) as u32 as u64; r10 = (crate::simd_ops::ru64(rdx.wrapping_add(P3))) as u64; r9 = (crate::simd_ops::ru32(rdx.wrapping_add(P4))) as u32 as u64; r11 = (crate::simd_ops::ru64(r10)) as u64; rax = ((r11.wrapping_add(r9)) as u64) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 3; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru32(r10.wrapping_add(P5))) as u32 as u64; rax = ((rax).wrapping_add(r11)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rcx = (((rcx as u32)).wrapping_sub((r9 as u32))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((r11 as u32)) as u32 as u64; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 4; continue; }
            3 => { rcx = (crate::simd_ops::ru32(r10.wrapping_add(P5))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::ru32(r8.wrapping_add(P6))) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(P6), ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32); rax = (crate::simd_ops::ru64(rdx.wrapping_add(P3))) as u64; rcx = (crate::simd_ops::ru32(rax.wrapping_add(P5))) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_add(P4), ((rcx as u32)) as u32); crate::simd_ops::wu64(r8.wrapping_add(P0), (0x0u64) as u64); crate::simd_ops::wu8(r8.wrapping_add(P7), (0x1u8) as u8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141678ef0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xd8; const P1: u64 = 0x10; const P2: u64 = 0x8; const P3: u64 = 0x141678e70;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { r8 = (crate::simd_ops::ru32(rax.wrapping_add(P1))) as u32 as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; r8 = ((r8).wrapping_add(crate::simd_ops::ru64(rax))) as u64; fl_l = (r8) as u64 as u64; fl_r = (rdx) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P3) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); } bb = 2; continue; }
            2 => { return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141678f20(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1426452b0; const P1: u64 = 0x14167d130; const P2: u64 = 0x14167d920; const P3: u64 = 0x14167d330; const P4: u64 = 0x14167db20; const P5: u64 = 0x1415242e0; const P6: u64 = 0x1426452a8;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 15>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r9); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xc8u64))) as u64; r14 = rcx; rdi = 0; fl_l = 0; fl_r = 0; rcx = ((rcx).wrapping_add(0x18u64)) as u64; rbx = r9; r15 = ((r8 as u32)) as u32 as u64; crate::simd_ops::wu64(rax, rdi); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0xc0u64))) as u64; rdx = ((rdi as u32)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x80u64), rdx); fl_l = (crate::simd_ops::ru8(r14.wrapping_add(0x418u64))) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 3; continue; } bb = 1; continue; }
            1 => { fl_l = ((r15 as u32)) as u32 as u64; fl_r = 0xffffffff; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 3; continue; } bb = 2; continue; }
            2 => { r9 = (crate::simd_ops::ru32(rsp.wrapping_add(0xa0u64))) as u32 as u64; rax = rsp.wrapping_add(0x80); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rax); rsi = r14.wrapping_add(0xc0); rax = ((crate::simd_ops::ru8(rsp.wrapping_add(0xb0u64))) as u32) as u32 as u64; r8 = rbx; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), r12); rdx = ((r15 as u32)) as u32 as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x28u64), ((rax as u8)) as u8); rcx = rsi; rax = (crate::simd_ops::ru32(rsp.wrapping_add(0xa8u64))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rax as u32)) as u32); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; rbp = ((rax as u32)) as u32 as u64; bb = 4; continue; }
            3 => { rbp = ((rdi as u32)) as u32 as u64; rsi = r14.wrapping_add(0xc0); bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::ru8(rsp.wrapping_add(0xb8u64))) as u8 as u64; fl_r = ((rdi as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 8; continue; } bb = 5; continue; }
            5 => { fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 19; continue; } bb = 6; continue; }
            6 => { fl_l = (crate::simd_ops::ru64(rdx.wrapping_add(0x10u64))) as u64; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 8; continue; } bb = 7; continue; }
            7 => { rcx = rsi; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 19; continue; }
            8 => { rax = (crate::simd_ops::ru64(r14)) as u64; rcx = r14; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x48u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rbx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 16; continue; } bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 12; continue; } bb = 10; continue; }
            10 => { r8 = (crate::simd_ops::ru32(rsp.wrapping_add(0xa8u64))) as u32 as u64; r9 = rsp.wrapping_add(0x80); rdx = (crate::simd_ops::ru32(r12)) as u32 as u64; rcx = rsi; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 11; continue; }
            11 => { r9 = (crate::simd_ops::ru32(rdx.wrapping_add(0x1cu64))) as u32 as u64; r8 = r14.wrapping_add(0x448); r10 = (crate::simd_ops::ru32(r12)) as u32 as u64; r11 = (crate::simd_ops::ru64(rsp.wrapping_add(0x98u64))) as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x88u64))) as u64; rcx = (crate::simd_ops::ru32(rax.wrapping_add(0x70u64))) as u32 as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(0x28u64))) as u64; rcx = (((rcx) as i64).wrapping_mul((crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as i64) as u64) as u64; rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(r8.wrapping_add(0x10u64), rax); rcx = ((rcx).wrapping_add(r11)) as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xc8u64))) as u64; crate::simd_ops::wu64(r8, rcx); rcx = rsi; crate::simd_ops::wu32(r8.wrapping_add(0x8u64), ((r9 as u32)) as u32); r9 = ((r15 as u32)) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(0xcu64), ((r10 as u32)) as u32); crate::simd_ops::wu64(rax, r8); rax = (crate::simd_ops::ru32(r12)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rax as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r11); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; bb = 12; continue; }
            12 => { fl_l = (crate::simd_ops::lk_and_u32(rbx.wrapping_add(0x14u64), (0xfffffff8u32) as u32)) as u32 as u64; fl_r = fl_l; crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), rax); crate::simd_ops::wu32(rbx.wrapping_add(0x10u64), ((rbp as u32)) as u32); bb = 18; continue; }
            13 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), rdi); fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 15; continue; } bb = 14; continue; }
            14 => { rcx = rsi; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 15; continue; }
            15 => { rdx = rbx; rcx = 0x7; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); bb = 19; continue; }
            16 => { rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 18; continue; } bb = 17; continue; }
            17 => { rcx = rsi; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rsp.wrapping_add(0x80u64), rdi); bb = 18; continue; }
            18 => { rdi = rbx; bb = 19; continue; }
            19 => { rcx = r14.wrapping_add(0x18); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; rax = rdi; rsp = ((rsp).wrapping_add(0x40u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141679140(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1426452b0; const P1: u64 = 0xfffffffffffffff8; const P2: u64 = 0xffffffffffffff80; const P3: u64 = 0xfffffffffffffffc; const P4: u64 = 0x141678600; const P5: u64 = 0x40; const P6: u64 = 0x48; const P7: u64 = 0x14167c780; const P8: u64 = 0xffffffffffffffb0; const P9: u64 = 0x1426452a8; const P10: u64 = 0xffffffffffffffe0; const P11: u64 = 0x14167c920;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    rcx = ((rcx).wrapping_add(0xffffffffffffffb0u64)) as u64;
    rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp);
    rdx = 0x3;
    crate::simd_ops::wu8(rbx.wrapping_add(P1), (0x1u8) as u8);
    rcx = ((rbx.wrapping_add(P2)) as u64) as u64;
    crate::simd_ops::wu8(rbx.wrapping_add(P3), (0x0u8) as u8);
    rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp);
    fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(P5))) as u64; fl_r = (0x0u64) as u64;
    if !((((fl_l as u64)) == ((fl_r as u64)))) { rcx = ((rbx.wrapping_add(P2)) as u64) as u64; crate::simd_ops::wu8(rbx.wrapping_add(P6), (0x1u8) as u8); rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rcx = ((rbx.wrapping_add(P8)) as u64) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P9), rax), rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P10))) as u64; rdx = ((rbx.wrapping_add(P2)) as u64) as u64; rax = crate::simd_ops::call_stk(P11, rax, rcx, rdx, r8, r9, rsp); rcx = rbx.wrapping_sub(0x50); crate::simd_ops::wu8(rbx.wrapping_add(0x48u64), (0x0u8) as u8); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); }
    rcx = rbx.wrapping_sub(0x50);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P9), rax), rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416791d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1416763e0; const P1: u64 = 0x1426452b0; const P2: u64 = 0x14167df40; const P3: u64 = 0x1426452a8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 15>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x60u64)) as u64; rbp = rcx; crate::simd_ops::wu128(rsp.wrapping_add(0x50u64), (xmm6) as u128); rcx = rdx; xmm6 = (xmm2) as u128; rbx = rdx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = ((rax as u32)) as u32 as u64; rcx = rbx; rax = (crate::simd_ops::ru64(rbx)) as u64; fl_l = ((rdi as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { r9 = 0; fl_l = 0; fl_r = 0; rdx = 0; fl_l = 0; fl_r = 0; r8 = ((r9.wrapping_add(0x2u64)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x28u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x50u64))) as u128; rsp = ((rsp).wrapping_add(0x60u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            2 => { r9 = rsp.wrapping_add(0x98); crate::simd_ops::wu64(rsp.wrapping_add(0x88u64), r14); r8 = rsp.wrapping_add(0x40); crate::simd_ops::wu8(rsp.wrapping_add(0x20u64), (0x0u8) as u8); rdx = rsp.wrapping_add(0x38); (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x20u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r14 = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { r9 = 0; fl_l = 0; fl_r = 0; r8 = ((rax.wrapping_add(0x35u64)) as u32) as u32 as u64; rdx = 0; fl_l = 0; fl_r = 0; bb = 13; continue; }
            4 => { rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 5; continue; }
            5 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0x76u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; r8 = rsp.wrapping_add(0x30); rax = ((crate::simd_ops::ru8(rbx.wrapping_add(0x74u64))) as u32) as u32 as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x428u64))) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((xmm6 as u32)) as u32); crate::simd_ops::wu8(rsp.wrapping_add(0x34u64), ((rax as u8)) as u8); r9 = (crate::simd_ops::ru64(rdx.wrapping_add(0x10u64))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdx); rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x80u64), rsi); if (((fl_l as u8)) != ((fl_r as u8))) { bb = 7; continue; } bb = 6; continue; }
            6 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x28u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 8; continue; }
            7 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x30u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 8; continue; }
            8 => { rcx = rbp.wrapping_add(0x18); rdi = ((rax as u32)) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = (crate::simd_ops::ru64(r14.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rdx.wrapping_add(0x10u64), (0x0u64) as u64); fl_l = ((rdi as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 11; continue; } bb = 9; continue; }
            9 => { fl_l = (crate::simd_ops::ru32(rdx.wrapping_add(0x20u64))) as u32 as u64; fl_r = 0xffffffff; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 11; continue; } bb = 10; continue; }
            10 => { rcx = rbp.wrapping_add(0xc0); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 11; continue; }
            11 => { rcx = rbp.wrapping_add(0x18); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; fl_l = rdx; fl_r = rdx; bb = 12; continue; }
            12 => { r9 = (r9 & !0xffu64) | (((if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { 1u8 } else { 0u8 }) as u8 as u64)); r8 = ((rdi as u32)) as u32 as u64; rdx = r14; bb = 13; continue; }
            13 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x28u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x88u64))) as u64; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x50u64))) as u128; rsp = ((rsp).wrapping_add(0x60u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141679310(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1426452b0; const P1: u64 = 0x14167d920; const P2: u64 = 0x1415242e0; const P3: u64 = 0x1426452a8; const P4: u64 = 0x141678e70;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rax = ((crate::simd_ops::ru8(rcx.wrapping_add(0xd0u64))) as u32) as u32 as u64; r14 = (r14 & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; r15 = ((rdx as u32)) as u32 as u64; rdi = rcx; fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(0xb8u64))) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 24; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rbx); rbx = (crate::simd_ops::ru64(rcx.wrapping_add(0xc8u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), r12); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r13); r13 = 0; fl_l = 0; fl_r = 0; rsi = ((r13 as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 4; continue; } bb = 2; continue; }
            2 => { bb = 3; continue; }
            3 => { rsi = rbx; rbx = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rax; fl_r = (0x1u64) as u64; rax = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 4; continue; }
            4 => { r12 = (crate::simd_ops::ru64(rcx.wrapping_add(0x60u64))) as u64; rcx = r12.wrapping_add(0x18); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 23; continue; } bb = 5; continue; }
            5 => { crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rbp); bb = 6; continue; }
            6 => { fl_l = ((r14 as u8)) as u8 as u64; fl_r = ((r14 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 9; continue; } bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rcx = (crate::simd_ops::ru32(rax.wrapping_add(0x18u64))) as u32 as u64; rcx = (((rcx as u32)).wrapping_sub(crate::simd_ops::ru32(rbx.wrapping_add(0x10u64)))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((r15 as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 9; continue; } bb = 8; continue; }
            8 => { rsi = rbx; rbx = (crate::simd_ops::ru64(rbx)) as u64; bb = 21; continue; }
            9 => { r14 = (r14 & !0xffu64) | (((0x1u8) as u8 as u64)); rax = (crate::simd_ops::ru64(rbx)) as u64; rbp = rbx; fl_l = rbx; fl_r = (crate::simd_ops::ru64(rdi.wrapping_add(0xc8u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 11; continue; } bb = 10; continue; }
            10 => { crate::simd_ops::wu64(rdi.wrapping_add(0xc8u64), rax); bb = 12; continue; }
            11 => { crate::simd_ops::wu64(rsi, rax); bb = 12; continue; }
            12 => { fl_l = rbx; fl_r = (crate::simd_ops::ru64(rdi.wrapping_add(0xc0u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 14; continue; } bb = 13; continue; }
            13 => { crate::simd_ops::wu64(rdi.wrapping_add(0xc0u64), rsi); bb = 14; continue; }
            14 => { crate::simd_ops::lk_sub_u32(rdi.wrapping_add(0xb8u64), (1) as u32); rbx = rax; rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x8u64))) as u64; r9 = (crate::simd_ops::ru32(rbp.wrapping_add(0x10u64))) as u32 as u64; rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0xa0u64))) as u32 as u64; r8 = (crate::simd_ops::ru64(rax)) as u64; rdx = rax.wrapping_add(0x18); rax = ((r8.wrapping_add(r9)) as u64) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 17; continue; } bb = 15; continue; }
            15 => { rax = (crate::simd_ops::ru32(rdx)) as u32 as u64; rax = ((rax).wrapping_add(r8)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 17; continue; } bb = 16; continue; }
            16 => { rcx = (((rcx as u32)).wrapping_sub((r9 as u32))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 18; continue; }
            17 => { rcx = (crate::simd_ops::ru32(rdx)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 18; continue; }
            18 => { crate::simd_ops::lk_sub_u32(rdi.wrapping_add(0xa8u64), ((rcx as u32)) as u32); rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x8u64))) as u64; rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x60u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x8u64), r13); fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 20; continue; } bb = 19; continue; }
            19 => { rcx = ((rcx).wrapping_add(0xc0u64)) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 20; continue; }
            20 => { rdx = rbp; rcx = 0x7; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 21; continue; }
            21 => { fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 6; continue; } bb = 22; continue; }
            22 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; bb = 23; continue; }
            23 => { rcx = r12.wrapping_add(0x18); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; bb = 24; continue; }
            24 => { rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0xd8u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 28; continue; } bb = 25; continue; }
            25 => { fl_l = ((r14 as u8)) as u8 as u64; fl_r = ((r14 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 27; continue; } bb = 26; continue; }
            26 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; rcx = (crate::simd_ops::ru32(rax.wrapping_add(0x18u64))) as u32 as u64; rcx = (((rcx as u32)).wrapping_sub(crate::simd_ops::ru32(rdx.wrapping_add(0x10u64)))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((r15 as u32)) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 28; continue; } bb = 27; continue; }
            27 => { rcx = rdi; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 28; continue; }
            28 => { rsp = ((rsp).wrapping_add(0x30u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416794b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0xd8; const P1: u64 = 0xd0; const P2: u64 = 0xb8; const P3: u64 = 0xc0; const P4: u64 = 0x8; const P5: u64 = 0x18; const P6: u64 = 0x88;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { rax = ((crate::simd_ops::ru8(rcx.wrapping_add(P1))) as u32) as u32 as u64; fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(P2))) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 4; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(P3))) as u64; bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(rax.wrapping_add(P4))) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(P5))) as u32 as u64; fl_l = ((rax).wrapping_add(crate::simd_ops::ru64(rcx))) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; return rax; }
            4 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(P6))) as u64; return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141679500(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xffffffffffffffef; const P1: u64 = 0x141524330; const P2: u64 = 0x8; const P3: u64 = 0x10; const P4: u64 = 0x14;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rdx = 0x18;
    rcx = ((rdx.wrapping_add(P0)) as u32) as u32 as u64;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rcx = 0;
    fl_l = 0; fl_r = 0;
    fl_l = rax; fl_r = rax;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { crate::simd_ops::wu64(rax, rcx); crate::simd_ops::wu64(rax.wrapping_add(P2), rcx); crate::simd_ops::wu32(rax.wrapping_add(P3), ((rcx as u32)) as u32); rcx = (crate::simd_ops::ru32(rax.wrapping_add(P4))) as u32 as u64; rcx = ((((rcx as u32)) & (0xfffffffbu32))) as u32 as u64; rcx = ((((rcx as u32)) | (0xbu32))) as u32 as u64; crate::simd_ops::wu32(rax.wrapping_add(P4), ((rcx as u32)) as u32); rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
    rax = rcx;
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141679540(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141677ae0; const P1: u64 = 0x1416763e0; const P2: u64 = 0x1426452b0; const P3: u64 = 0x14167df40; const P4: u64 = 0x1426452a8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm0: u128 = 0; let mut xmm1: u128 = 0; let mut xmm2: u128 = 0; let mut xmm3: u128 = 0; let mut xmm6: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 15>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = ((rsp).wrapping_sub(0x68u64)) as u64; rdx = rsp.wrapping_add(0x90); rsi = rcx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 18; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), r15); r15 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm6) as u128); crate::simd_ops::wu64(rsp.wrapping_add(0x80u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), r14); bb = 2; continue; }
            2 => { xmm6 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x90u64))) as u32) as u128; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = ((rax as u32)) as u32 as u64; rcx = rbx; rax = (crate::simd_ops::ru64(rbx)) as u64; fl_l = ((rdi as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { r9 = 0; fl_l = 0; fl_r = 0; rdx = 0; fl_l = 0; fl_r = 0; r8 = ((r9.wrapping_add(0x2u64)) as u32) as u32 as u64; bb = 16; continue; }
            4 => { r9 = rsp.wrapping_add(0x88); crate::simd_ops::wu8(rsp.wrapping_add(0x20u64), ((r15 as u8)) as u8); r8 = rsp.wrapping_add(0x38); rdx = rsp.wrapping_add(0x30); (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x20u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r14 = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { r9 = 0; fl_l = 0; fl_r = 0; r8 = ((rax.wrapping_add(0x35u64)) as u32) as u32 as u64; rdx = 0; fl_l = 0; fl_r = 0; bb = 15; continue; }
            6 => { rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 7; continue; }
            7 => { r8 = rsp.wrapping_add(0x98); rax = ((crate::simd_ops::ru8(rbx.wrapping_add(0x74u64))) as u32) as u32 as u64; rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x428u64))) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x98u64), ((xmm6 as u32)) as u32); crate::simd_ops::wu8(rsp.wrapping_add(0x9cu64), ((rax as u8)) as u8); r9 = (crate::simd_ops::ru64(rdx.wrapping_add(0x10u64))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdx); rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0x76u64))) as u8 as u64; fl_r = ((r15 as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 9; continue; } bb = 8; continue; }
            8 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x28u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 10; continue; }
            9 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x30u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 10; continue; }
            10 => { rcx = rsi.wrapping_add(0x18); rdi = ((rax as u32)) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = (crate::simd_ops::ru64(r14.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rdx.wrapping_add(0x10u64), r15); fl_l = ((rdi as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 13; continue; } bb = 11; continue; }
            11 => { fl_l = (crate::simd_ops::ru32(rdx.wrapping_add(0x20u64))) as u32 as u64; fl_r = 0xffffffff; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 13; continue; } bb = 12; continue; }
            12 => { rcx = rsi.wrapping_add(0xc0); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 13; continue; }
            13 => { rcx = rsi.wrapping_add(0x18); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; fl_l = rdx; fl_r = rdx; bb = 14; continue; }
            14 => { r9 = (r9 & !0xffu64) | (((if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { 1u8 } else { 0u8 }) as u8 as u64)); r8 = ((rdi as u32)) as u32 as u64; rdx = r14; bb = 15; continue; }
            15 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; bb = 16; continue; }
            16 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x28u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rsp.wrapping_add(0x90); rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 17; continue; }
            17 => { xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x40u64))) as u128; r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; bb = 18; continue; }
            18 => { rsp = ((rsp).wrapping_add(0x68u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416796c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141677ae0; const P1: u64 = 0x1416791d0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdx = rsp.wrapping_add(0x38);
    rbx = rcx;
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    fl_l = rax; fl_r = rax;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { xmm2 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x38u64))) as u32) as u128; rdx = rax; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); }
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416796f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1426452b0; const P1: u64 = 0x1426452a8; const P2: u64 = 0x141678f20; const P3: u64 = 0x141678970;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x50u64)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x28u64))) as u64; rsi = rcx; crate::simd_ops::wu64(rdx, rax); rcx = ((rcx).wrapping_add(0x30u64)) as u64; crate::simd_ops::wu64(r8, (0x0u64) as u64); r15 = r8; crate::simd_ops::wu8(r9, (0x0u8) as u8); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru8(rsi.wrapping_add(0x78u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 19; continue; } bb = 1; continue; }
            1 => { rcx = rsi.wrapping_add(0x30); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); rdi = ((crate::simd_ops::ru8(rsi.wrapping_add(0x7cu64))) as u32) as u32 as u64; rcx = rsi.wrapping_add(0x30); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = ((rdi as u8)) as u8 as u64; fl_r = ((rdi as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 19; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru64(rsi.wrapping_add(0xd8u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 19; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rsi)) as u64; rcx = rsi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x68u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rdi = rax; rax = (crate::simd_ops::ru32(rsi.wrapping_add(0xa0u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 6; continue; } bb = 4; continue; }
            4 => { fl_l = rdi; fl_r = rax; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rdi = (crate::simd_ops::ru32(rsi.wrapping_add(0x9cu64))) as u32 as u64; bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x28u64))) as u64; rbp = (crate::simd_ops::ru32(rsi.wrapping_add(0xa4u64))) as u32 as u64; rcx = (crate::simd_ops::ru64(rax)) as u64; rax = ((rdi.wrapping_add(rbp)) as u64) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 8; continue; } bb = 7; continue; }
            7 => { r14 = (r14 & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 9; continue; }
            8 => { rbp = 0; fl_l = 0; fl_r = 0; rax = ((rcx as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_sub((rdi as u32))) as u32 as u64; r14 = (r14 & !0xffu64) | (((0x1u8) as u8 as u64)); fl_l = rdi; fl_r = rcx; rbp = (if (((fl_l as u64)) < ((fl_r as u64))) { (rax as u32) } else { (rbp as u32) }) as u32 as u64; bb = 9; continue; }
            9 => { fl_l = (crate::simd_ops::ru8(rsi.wrapping_add(0x7bu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 12; continue; } bb = 10; continue; }
            10 => { rax = (crate::simd_ops::ru64(rsi)) as u64; rcx = rsi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x40u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rdx = ((rbp as u32)) as u32 as u64; rdx = ((rdx).wrapping_add(rdi)) as u64; rcx = ((rax as u32)) as u32 as u64; fl_l = rdx; fl_r = rcx; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 12; continue; } bb = 11; continue; }
            11 => { rbp = ((rax as u32)) as u32 as u64; fl_l = ((rbp as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; rbp = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 12; continue; }
            12 => { crate::simd_ops::wu32(rsp.wrapping_add(0x70u64), ((rbp as u32)) as u32); fl_l = ((rbp as u32)) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 19; continue; } bb = 13; continue; }
            13 => { rcx = (crate::simd_ops::ru32(rsi.wrapping_add(0xacu64))) as u32 as u64; rax = rsp.wrapping_add(0x70); r8 = (crate::simd_ops::ru32(rsi.wrapping_add(0x90u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), r15); r9 = rdi; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rax); rcx = (if (((fl_l as u32)) >= ((fl_r as u32))) { (rbp as u32) } else { (rcx as u32) }) as u32 as u64; rax = ((crate::simd_ops::ru8(rsp.wrapping_add(0x90u64))) as u32) as u32 as u64; rdx = rsi; crate::simd_ops::wu8(rsp.wrapping_add(0x38u64), ((rax as u8)) as u8); rax = (crate::simd_ops::ru32(rsi.wrapping_add(0xb0u64))) as u32 as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x30u64), ((r14 as u8)) as u8); crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rax as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rcx as u32)) as u32); rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x60u64))) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rbp = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 19; continue; } bb = 14; continue; }
            14 => { rdx = (crate::simd_ops::ru32(rsi.wrapping_add(0xa0u64))) as u32 as u64; crate::simd_ops::wu64(rsi.wrapping_add(0xd8u64), rax); fl_l = rdi; fl_r = rdx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 17; continue; } bb = 15; continue; }
            15 => { rcx = (crate::simd_ops::ru32(rsp.wrapping_add(0x70u64))) as u32 as u64; rax = ((rdi.wrapping_add(rcx)) as u64) as u64; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 18; continue; } bb = 16; continue; }
            16 => { rcx = ((rdx as u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x70u64), ((rcx as u32)) as u32); bb = 18; continue; }
            17 => { rcx = (crate::simd_ops::ru32(rsp.wrapping_add(0x70u64))) as u32 as u64; bb = 18; continue; }
            18 => { crate::simd_ops::lk_add_u32(rsi.wrapping_add(0xa8u64), ((rcx as u32)) as u32); rcx = rsi; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x60u64))) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x70u64))) as u64; crate::simd_ops::wu64(rsi.wrapping_add(0x58u64), rcx); bb = 20; continue; }
            19 => { rbp = 0; fl_l = 0; fl_r = 0; bb = 20; continue; }
            20 => { rcx = rsi.wrapping_add(0x30); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); r11 = rsp.wrapping_add(0x50); rax = rbp; rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; rbp = (crate::simd_ops::ru64(r11.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x38u64))) as u64; rsp = r11; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416798b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1426452b0; const P1: u64 = 0x1426452a8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), r8); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x38u64)) as u64; r14 = 0; fl_l = 0; fl_r = 0; rsi = rcx; crate::simd_ops::wu64(r8, r14); rcx = ((rcx).wrapping_add(0x30u64)) as u64; crate::simd_ops::wu8(r9, ((r14 as u8)) as u8); r15 = rdx; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru8(rsi.wrapping_add(0x78u64))) as u8 as u64; fl_r = ((r14 as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 12; continue; } bb = 1; continue; }
            1 => { rcx = rsi.wrapping_add(0x30); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rdi); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); rdi = ((crate::simd_ops::ru8(rsi.wrapping_add(0x7cu64))) as u32) as u32 as u64; rcx = rsi.wrapping_add(0x30); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = ((rdi as u8)) as u8 as u64; fl_r = ((rdi as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 11; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x28u64))) as u64; crate::simd_ops::wu64(r15, rax); rax = (crate::simd_ops::ru32(rsi.wrapping_add(0xb0u64))) as u32 as u64; rcx = (crate::simd_ops::ru32(rsi.wrapping_add(0xa0u64))) as u32 as u64; r15 = ((rax as u32)) as u32 as u64; r15 = ((r15).wrapping_add(crate::simd_ops::ru64(rsi.wrapping_add(0x88u64)))) as u64; rcx = (((rcx as u32)).wrapping_sub((rax as u32))) as u32 as u64; rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x60u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), r12); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r13); r12 = (crate::simd_ops::ru32(rax.wrapping_add(0x430u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((r12 as u32)) as u32 as u64; r12 = (if (((fl_l as u32)) <= ((fl_r as u32))) { (rcx as u32) } else { (r12 as u32) }) as u32 as u64; rbp = ((r12 as u32)) as u32 as u64; fl_l = (crate::simd_ops::ru8(rsi.wrapping_add(0x76u64))) as u8 as u64; fl_r = ((r14 as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 8; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x28u64))) as u64; rcx = (crate::simd_ops::ru64(rax)) as u64; rax = ((r12 as u32)) as u32 as u64; rax = ((rax).wrapping_add(r15)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = ((rcx as u32)) as u32 as u64; rbp = ((r14 as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_sub((r15 as u32))) as u32 as u64; fl_l = r15; fl_r = rcx; rbp = (if (((fl_l as u64)) < ((fl_r as u64))) { (rax as u32) } else { (rbp as u32) }) as u32 as u64; bb = 5; continue; }
            5 => { fl_l = (crate::simd_ops::ru8(rsi.wrapping_add(0x7bu64))) as u8 as u64; fl_r = ((r14 as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 8; continue; } bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(rsi)) as u64; rcx = rsi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x40u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rdx = ((rbp as u32)) as u32 as u64; rdx = ((rdx).wrapping_add(r15)) as u64; rcx = ((rax as u32)) as u32 as u64; fl_l = rdx; fl_r = rcx; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 8; continue; } bb = 7; continue; }
            7 => { rbp = ((rax as u32)) as u32 as u64; fl_l = ((rbp as u32)) as u32 as u64; fl_r = ((r15 as u32)) as u32 as u64; rbp = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 8; continue; }
            8 => { rax = (crate::simd_ops::ru32(rsi.wrapping_add(0xb0u64))) as u32 as u64; r13 = (crate::simd_ops::ru64(rsi.wrapping_add(0x60u64))) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x60u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rcx = r13.wrapping_add(0x18); crate::simd_ops::wu64(rax, r14); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(r13)) as u64; rcx = r13; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x48u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = r13.wrapping_add(0x18); r14 = rax; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = r14; fl_r = r14; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x28u64))) as u64; rcx = r13.wrapping_add(0x448); rdx = (crate::simd_ops::ru32(rsi.wrapping_add(0x70u64))) as u32 as u64; r9 = rsi.wrapping_add(0x88); r8 = (crate::simd_ops::ru32(rsp.wrapping_add(0x60u64))) as u32 as u64; rdx = (((rdx) as i64).wrapping_mul((crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as i64) as u64) as u64; rax = ((r8 as u32)) as u32 as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(r9.wrapping_add(0x8u64)))) as u64; rdx = ((rdx).wrapping_add(r15)) as u64; crate::simd_ops::wu64(rcx.wrapping_add(0x10u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; crate::simd_ops::wu64(rcx, rdx); crate::simd_ops::wu32(rcx.wrapping_add(0x8u64), ((r12 as u32)) as u32); crate::simd_ops::wu32(rcx.wrapping_add(0xcu64), ((rbp as u32)) as u32); crate::simd_ops::wu64(rax, rcx); crate::simd_ops::wu64(r9.wrapping_add(0x10u64), rcx); crate::simd_ops::lk_and_u32(r14.wrapping_add(0x14u64), (0xfffffff8u32) as u32); crate::simd_ops::wu64(r14.wrapping_add(0x8u64), r9); crate::simd_ops::wu32(r14.wrapping_add(0x10u64), ((r8 as u32)) as u32); rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x60u64))) as u64; fl_l = (crate::simd_ops::lk_add_u32(rsi.wrapping_add(0xb0u64), ((rbp as u32)) as u32)) as u32 as u64; fl_r = fl_l; crate::simd_ops::wu64(rsi.wrapping_add(0xc0u64), r14); rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x70u64))) as u64; crate::simd_ops::wu64(rsi.wrapping_add(0x58u64), rcx); bb = 10; continue; }
            10 => { r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; bb = 11; continue; }
            11 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; bb = 12; continue; }
            12 => { rcx = rsi.wrapping_add(0x30); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); rax = r14; rsp = ((rsp).wrapping_add(0x38u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141679a70(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xd8; const P1: u64 = 0x8; const P2: u64 = 0x18; const P3: u64 = 0x10; const P4: u64 = 0x141678e70; const P5: u64 = 0x141677560;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = 0;
    fl_l = 0; fl_r = 0;
    rdi = ((rdx as u32)) as u32 as u64;
    rsi = rcx;
    fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64;
    if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { rdi = (((rdi as u32)).wrapping_sub((rbx as u32))) as u32 as u64; rcx = rsi; rdx = ((rdi as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rax = (((rax as u32)).wrapping_add((rbx as u32))) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    fl_l = rcx; fl_r = rcx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rax = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; rbx = (crate::simd_ops::ru32(rax.wrapping_add(P2))) as u32 as u64; rbx = (((rbx as u32)).wrapping_sub(crate::simd_ops::ru32(rcx.wrapping_add(P3)))) as u32 as u64; rcx = rsi; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); }
    rdi = (((rdi as u32)).wrapping_sub((rbx as u32))) as u32 as u64;
    rcx = rsi;
    rdx = ((rdi as u32)) as u32 as u64;
    rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp);
    rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    rax = (((rax as u32)).wrapping_add((rbx as u32))) as u32 as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141679ad0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 1>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rax = (crate::simd_ops::ru64(rcx.wrapping_add(0xc8u64))) as u64; r9 = 0; fl_l = 0; fl_r = 0; r8 = ((crate::simd_ops::ru8(rcx.wrapping_add(0xd0u64))) as u32) as u32 as u64; rdi = rcx; r11 = ((rdx as u32)) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(0xa0u64), ((r11 as u32)) as u32); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 1; continue; }
            1 => { fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 4; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rax)) as u64; r8 = (((r8 as u32)).wrapping_sub(1)) as u32 as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 1; continue; } bb = 3; continue; }
            3 => { bb = 11; continue; }
            4 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); rbx = r11; bb = 5; continue; }
            5 => { rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; r10 = (crate::simd_ops::ru32(rax.wrapping_add(0x10u64))) as u32 as u64; r8 = (crate::simd_ops::ru64(rcx)) as u64; rdx = rcx.wrapping_add(0x18); rcx = ((r8.wrapping_add(r10)) as u64) as u64; fl_l = rcx; fl_r = rbx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 8; continue; } bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru32(rdx)) as u32 as u64; rcx = ((rcx).wrapping_add(r8)) as u64; fl_l = rcx; fl_r = rbx; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 8; continue; } bb = 7; continue; }
            7 => { rcx = ((r11 as u32)) as u32 as u64; rcx = (((rcx as u32)).wrapping_sub((r10 as u32))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 9; continue; }
            8 => { rcx = (crate::simd_ops::ru32(rdx)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((r10 as u32)) as u32 as u64; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru64(rax)) as u64; r9 = (((r9 as u32)).wrapping_add((rcx as u32))) as u32 as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 10; continue; }
            10 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x10u64))) as u64; bb = 11; continue; }
            11 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(0xd8u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 16; continue; } bb = 12; continue; }
            12 => { rdx = (crate::simd_ops::ru32(rax.wrapping_add(0x10u64))) as u32 as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; r8 = (crate::simd_ops::ru64(rax)) as u64; rcx = rax.wrapping_add(0x18); rax = ((r8.wrapping_add(rdx)) as u64) as u64; fl_l = rax; fl_r = r11; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 15; continue; } bb = 13; continue; }
            13 => { rax = (crate::simd_ops::ru32(rcx)) as u32 as u64; rax = ((rax).wrapping_add(r8)) as u64; fl_l = rax; fl_r = r11; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 15; continue; } bb = 14; continue; }
            14 => { r11 = (((r11 as u32)).wrapping_sub((rdx as u32))) as u32 as u64; r11 = (((r11 as u32)).wrapping_sub((r8 as u32))) as u32 as u64; fl_l = (((r9 as u32)).wrapping_add((r11 as u32))) as u32 as u64; fl_r = fl_l; r9 = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0xa8u64), ((r9 as u32)) as u32); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            15 => { r11 = (crate::simd_ops::ru32(rcx)) as u32 as u64; r11 = (((r11 as u32)).wrapping_sub((rdx as u32))) as u32 as u64; fl_l = (((r9 as u32)).wrapping_add((r11 as u32))) as u32 as u64; fl_r = fl_l; r9 = ((fl_l as u32)) as u32 as u64; bb = 16; continue; }
            16 => { crate::simd_ops::wu32(rdi.wrapping_add(0xa8u64), ((r9 as u32)) as u32); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141679bb0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1426452b0; const P1: u64 = 0xd8; const P2: u64 = 0x14; const P3: u64 = 0x141675d10; const P4: u64 = 0xe0; const P5: u64 = 0x141678a70; const P6: u64 = 0x30; const P7: u64 = 0x1426452a8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = rcx; r15 = (((r9 as u8)) as u32) as u32 as u64; rcx = ((rcx).wrapping_add(0x30u64)) as u64; r14 = ((r8 as u32)) as u32 as u64; rdi = rdx; rbp = (rbp & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 1; continue; }
            1 => { fl_l = ((r14 as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 5; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru64(rsi.wrapping_add(P1))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 5; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(P2))) as u32 as u64; rax = (rax & !0xffu64) | ((((((rax as u8)) & (0x7u8))) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x2u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 5; continue; } bb = 4; continue; }
            4 => { r8 = (((r14 as u8)) as u32) as u32 as u64; bb = 6; continue; }
            5 => { r8 = (r8 & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 6; continue; }
            6 => { fl_l = ((r15 as u8)) as u8 as u64; fl_r = ((r15 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 9; continue; } bb = 7; continue; }
            7 => { fl_l = ((r8 as u8)) as u8 as u64; fl_r = ((r8 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { crate::simd_ops::lk_and_u32(rdi.wrapping_add(P2), (0xfffffffbu32) as u32); fl_l = (crate::simd_ops::lk_or_u32(rdi.wrapping_add(P2), (0x3u32) as u32)) as u32 as u64; fl_r = fl_l; bb = 9; continue; }
            9 => { rdx = rdi; rcx = rsi; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu8(rsi.wrapping_add(P4), ((rbp as u8)) as u8); rbp = (rbp & !0xffu64) | (((0x1u8) as u8 as u64)); crate::simd_ops::wu64(rsi.wrapping_add(P1), (0x0u64) as u64); bb = 10; continue; }
            10 => { rdx = ((r14 as u32)) as u32 as u64; rcx = rsi; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rcx = ((rsi.wrapping_add(P6)) as u64) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P7), rax), rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rax = (((rbp as u8)) as u32) as u32 as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141679c70(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1426452b0; const P1: u64 = 0x14; const P2: u64 = 0x141675dd0; const P3: u64 = 0xc0; const P4: u64 = 0x141678ac0; const P5: u64 = 0x30; const P6: u64 = 0x1426452a8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = rcx; rbp = ((r8 as u32)) as u32 as u64; rcx = ((rcx).wrapping_add(0x30u64)) as u64; rsi = rdx; r14 = (r14 & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 1; continue; }
            1 => { fl_l = ((rbp as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 4; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru32(rsi.wrapping_add(P1))) as u32 as u64; rax = (rax & !0xffu64) | ((((((rax as u8)) & (0x7u8))) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x2u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 4; continue; } bb = 3; continue; }
            3 => { r8 = (((rbp as u8)) as u32) as u32 as u64; bb = 5; continue; }
            4 => { r8 = (r8 & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 5; continue; }
            5 => { rdx = rsi; rcx = rdi; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rdi.wrapping_add(P3), (0x0u64) as u64); r14 = (r14 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 6; continue; }
            6 => { rdx = ((rbp as u32)) as u32 as u64; rcx = rdi; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rcx = ((rdi.wrapping_add(P5)) as u64) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = (((r14 as u8)) as u32) as u32 as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141679d10(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141524330; const P1: u64 = 0x141674a70; const P2: u64 = 0x141675750; const P3: u64 = 0x142694060; const P4: u64 = 0x1426940f0; const P5: u64 = 0x141676f10; const P6: u64 = 0x1415242e0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x88u64))) as u64; r12 = 0; fl_l = 0; fl_r = 0; r15 = rdx; rsi = rcx; rdx = 0xe8; rbp = r9; r14 = ((r8 as u32)) as u32 as u64; rcx = ((r12.wrapping_add(0x7u64)) as u32) as u32 as u64; crate::simd_ops::wu64(rdi, r12); rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rbx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rdx = ((crate::simd_ops::ru8(rbp.wrapping_add(0xdu64))) as u32) as u32 as u64; rcx = rsi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rdx = 0xe8; rcx = ((rbx.wrapping_add(0x7u64)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rbx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 2; continue; }
            2 => { rcx = rbx; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rax = P3; r9 = ((r14 as u32)) as u32 as u64; crate::simd_ops::wu64(rbx, rax); r8 = r15; crate::simd_ops::wu8(rbx.wrapping_add(0xe0u64), ((r12 as u8)) as u8); rax = P4; crate::simd_ops::wu64(rbx.wrapping_add(0xd8u64), r12); rdx = rsi; crate::simd_ops::wu64(rbx.wrapping_add(0x80u64), rax); rcx = rbx; rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x430u64))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbp); rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { fl_l = rbx; fl_r = rbx; rcx = rbx.wrapping_add(0x80); rax = rbx; rcx = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { r12 } else { rcx }) as u64; crate::simd_ops::wu64(rdi, rcx); bb = 6; continue; }
            4 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rdx = rbx; rcx = 0x7; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            5 => { crate::simd_ops::wu64(rdi, r12); rax = 0; fl_l = 0; fl_r = 0; bb = 6; continue; }
            6 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141679e40(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141524330; const P1: u64 = 0x141674a70; const P2: u64 = 0x1416758c0; const P3: u64 = 0x142693f88; const P4: u64 = 0x142693ff0; const P5: u64 = 0x1416770f0; const P6: u64 = 0x1415242e0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; r15 = 0; fl_l = 0; fl_r = 0; r14 = rdx; rsi = rcx; crate::simd_ops::wu64(r9, r15); rdx = 0xd0; rdi = r9; rbp = ((r8 as u32)) as u32 as u64; rcx = ((r15.wrapping_add(0x7u64)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rbx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rdx = (rdx & !0xffu64) | (((0x64u8) as u8 as u64)); rcx = rsi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rdx = 0xd0; rcx = ((rbx.wrapping_add(0x7u64)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rbx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 2; continue; }
            2 => { rcx = rbx; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rax = P3; r9 = ((rbp as u32)) as u32 as u64; crate::simd_ops::wu64(rbx, rax); r8 = r14; crate::simd_ops::wu8(rbx.wrapping_add(0xc8u64), ((r15 as u8)) as u8); rax = P4; crate::simd_ops::wu64(rbx.wrapping_add(0xc0u64), r15); rdx = rsi; rcx = rbx; crate::simd_ops::wu64(rbx.wrapping_add(0x80u64), rax); rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { fl_l = rbx; fl_r = rbx; rcx = rbx.wrapping_add(0x80); rax = rbx; rcx = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { r15 } else { rcx }) as u64; crate::simd_ops::wu64(rdi, rcx); bb = 6; continue; }
            4 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rdx = rbx; rcx = 0x7; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            5 => { rax = 0; fl_l = 0; fl_r = 0; bb = 6; continue; }
            6 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141679f30(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141675840; const P1: u64 = 0x1426941d8; const P2: u64 = 0x448; const P3: u64 = 0x450; const P4: u64 = 0x458; const P5: u64 = 0x460;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rax = P1;
    crate::simd_ops::wu64(rbx, rax);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu32(rbx.wrapping_add(P2), ((rax as u32)) as u32);
    crate::simd_ops::wu64(rbx.wrapping_add(P3), rax);
    crate::simd_ops::wu64(rbx.wrapping_add(P4), rax);
    crate::simd_ops::wu64(rbx.wrapping_add(P5), rax);
    rax = rbx;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141679f90(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1426943e8; const P1: u64 = 0x142694478; const P2: u64 = 0x80; const P3: u64 = 0xe0; const P4: u64 = 0xd8; const P5: u64 = 0xf0; const P6: u64 = 0x141675970; const P7: u64 = 0x141834d30;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rax = P0;
    rbx = ((rdx as u32)) as u32 as u64;
    crate::simd_ops::wu64(rcx, rax);
    rdi = rcx;
    rax = P1;
    crate::simd_ops::wu64(rcx.wrapping_add(P2), rax);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rcx.wrapping_add(P3), rax);
    crate::simd_ops::wu64(rcx.wrapping_add(P4), rax);
    crate::simd_ops::wu64(rcx.wrapping_add(P5), rax);
    rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp);
    fl_l = ((rbx as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64;
    if !(((((fl_l as u8)) & ((fl_r as u8))) == 0)) { rdx = 0xf8; rcx = rdi; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rax = rdi;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14167a000(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142694238; const P1: u64 = 0x1426942a0; const P2: u64 = 0x80; const P3: u64 = 0xc8; const P4: u64 = 0xc0; const P5: u64 = 0xd8; const P6: u64 = 0x141675a00; const P7: u64 = 0x141834d30;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rax = P0;
    rbx = ((rdx as u32)) as u32 as u64;
    crate::simd_ops::wu64(rcx, rax);
    rdi = rcx;
    rax = P1;
    crate::simd_ops::wu64(rcx.wrapping_add(P2), rax);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rcx.wrapping_add(P3), rax);
    crate::simd_ops::wu64(rcx.wrapping_add(P4), rax);
    crate::simd_ops::wu64(rcx.wrapping_add(P5), rax);
    rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp);
    fl_l = ((rbx as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64;
    if !(((((fl_l as u8)) & ((fl_r as u8))) == 0)) { rdx = 0xe0; rcx = rdi; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rax = rdi;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14167a070(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142694180; const P1: u64 = 0x1416759c0; const P2: u64 = 0x141834d30;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rax = P0;
    rbx = ((rdx as u32)) as u32 as u64;
    crate::simd_ops::wu64(rcx, rax);
    rdi = rcx;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    fl_l = ((rbx as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64;
    if !(((((fl_l as u8)) & ((fl_r as u8))) == 0)) { rdx = 0x468; rcx = rdi; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rax = rdi;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14167a0b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142694180; const P1: u64 = 0x1416759c0; const P2: u64 = 0x141834d30;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rax = P0;
    rbx = ((rdx as u32)) as u32 as u64;
    crate::simd_ops::wu64(rcx, rax);
    rdi = rcx;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    fl_l = ((rbx as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64;
    if !(((((fl_l as u8)) & ((fl_r as u8))) == 0)) { rdx = 0x448; rcx = rdi; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rax = rdi;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14167a170(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1426452b0; const P1: u64 = 0x48; const P2: u64 = 0x58; const P3: u64 = 0xffffffffffffff80; const P4: u64 = 0x141678600; const P5: u64 = 0xffffffffffffffb0; const P6: u64 = 0x1426452a8; const P7: u64 = 0x14167c780; const P8: u64 = 0x14167bef0; const P9: u64 = 0xffffffffffffffe0; const P10: u64 = 0x14167c920;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = rcx; rcx = ((rcx).wrapping_add(0xffffffffffffffb0u64)) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru64(rdi.wrapping_add(P1))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru64(rdi.wrapping_add(P2))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rcx = ((rdi.wrapping_add(P3)) as u64) as u64; rdx = 0x3; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rcx = ((rdi.wrapping_add(P5)) as u64) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp); }
            3 => { rdx = 0x3; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx); rcx = ((rdi.wrapping_add(P3)) as u64) as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rcx = ((rdi.wrapping_add(P3)) as u64) as u64; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rcx = ((rdi.wrapping_add(P3)) as u64) as u64; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); rcx = ((rdi.wrapping_add(P5)) as u64) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P9))) as u64; rdx = ((rdi.wrapping_add(P3)) as u64) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P10, rax, rcx, rdx, r8, r9, rsp); }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14167a210(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x60; const P1: u64 = 0xd8; const P2: u64 = 0x418; const P3: u64 = 0x14167a410;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rdx = ((rcx.wrapping_add(P1)) as u64) as u64;
    fl_l = (crate::simd_ops::ru8(rax.wrapping_add(P2))) as u8 as u64; fl_r = (0x0u8) as u8 as u64;
    r8 = (r8 & !0xffu64) | (((if (((fl_l as u8)) == ((fl_r as u8))) { 1u8 } else { 0u8 }) as u8 as u64));
    return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14167a230(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141675d10; const P1: u64 = 0x14167c430; const P2: u64 = 0x14167bd30; const P3: u64 = 0x14167e020;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; r14 = 0; fl_l = 0; fl_r = 0; rdi = rcx; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0xe0u64))) as u64; rsi = ((r14 as u32)) as u32 as u64; r8 = ((r14 as u32)) as u32 as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 1; continue; }
            1 => { bb = 2; continue; }
            2 => { r9 = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; r11 = rcx; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x10u64))) as u32 as u64; r10 = (crate::simd_ops::ru64(r9)) as u64; rax = ((rax).wrapping_add(r10)) as u64; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 9; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rcx)) as u64; fl_l = rcx; fl_r = (crate::simd_ops::ru64(rdi.wrapping_add(0xe0u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 5; continue; } bb = 4; continue; }
            4 => { crate::simd_ops::wu64(rdi.wrapping_add(0xe0u64), rax); bb = 6; continue; }
            5 => { crate::simd_ops::wu64(r8, rax); bb = 6; continue; }
            6 => { fl_l = rcx; fl_r = (crate::simd_ops::ru64(rdi.wrapping_add(0xd8u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 8; continue; } bb = 7; continue; }
            7 => { crate::simd_ops::wu64(rdi.wrapping_add(0xd8u64), r8); bb = 8; continue; }
            8 => { rcx = rax; fl_l = rsi; fl_r = rsi; rax = r14; rax = (if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { rsi } else { rax }) as u64; rsi = r11; crate::simd_ops::wu64(r11, rax); bb = 13; continue; }
            9 => { rdx = (crate::simd_ops::ru32(r9.wrapping_add(0x18u64))) as u32 as u64; rax = (crate::simd_ops::ru32(rdi.wrapping_add(0xa0u64))) as u32 as u64; rdx = ((rdx).wrapping_add(r10)) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 12; continue; } bb = 10; continue; }
            10 => { fl_l = rdx; fl_r = rax; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 12; continue; } bb = 11; continue; }
            11 => { rdx = (crate::simd_ops::ru32(rdi.wrapping_add(0x9cu64))) as u32 as u64; bb = 12; continue; }
            12 => { r8 = rcx; rcx = (crate::simd_ops::ru64(rcx)) as u64; bb = 13; continue; }
            13 => { fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 14; continue; }
            14 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x60u64))) as u64; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x418u64))) as u8 as u64; fl_r = ((r14 as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 17; continue; } bb = 15; continue; }
            15 => { fl_l = (crate::simd_ops::ru64(rdi.wrapping_add(0xe0u64))) as u64; fl_r = r14; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 17; continue; } bb = 16; continue; }
            16 => { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 18; continue; }
            17 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 18; continue; }
            18 => { crate::simd_ops::wu8(rsp.wrapping_add(0x30u64), ((rax as u8)) as u8); rbx = rsi; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 32; continue; } bb = 19; continue; }
            19 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = rbx; fl_l = rbx; fl_r = rsi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 21; continue; } bb = 20; continue; }
            20 => { rsi = rax; bb = 22; continue; }
            21 => { crate::simd_ops::wu64(r14, rax); bb = 22; continue; }
            22 => { rbx = rax; rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x14u64))) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x7u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 30; continue; } bb = 23; continue; }
            23 => { r10 = (crate::simd_ops::ru32(rdx.wrapping_add(0x10u64))) as u32 as u64; rax = ((((rax as u32)) & (0xfffffffau32))) as u32 as u64; rax = ((((rax as u32)) | (0x2u32))) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_add(0x14u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0xa0u64))) as u32 as u64; r9 = (crate::simd_ops::ru64(rax)) as u64; r8 = rax.wrapping_add(0x18); rax = ((r9.wrapping_add(r10)) as u64) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 26; continue; } bb = 24; continue; }
            24 => { rax = (crate::simd_ops::ru32(r8)) as u32 as u64; rax = ((rax).wrapping_add(r9)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 26; continue; } bb = 25; continue; }
            25 => { rcx = (((rcx as u32)).wrapping_sub((r10 as u32))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 27; continue; }
            26 => { rcx = (crate::simd_ops::ru32(r8)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((r10 as u32)) as u32 as u64; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 27; continue; }
            27 => { crate::simd_ops::lk_sub_u32(rdi.wrapping_add(0xa8u64), ((rcx as u32)) as u32); rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; rcx = (crate::simd_ops::ru32(rax.wrapping_add(0x18u64))) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_add(0x10u64), ((rcx as u32)) as u32); rax = (crate::simd_ops::ru64(rdi.wrapping_add(0xf0u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 29; continue; } bb = 28; continue; }
            28 => { crate::simd_ops::wu64(rdi.wrapping_add(0xf0u64), rdx); crate::simd_ops::wu64(rdx, r14); bb = 31; continue; }
            29 => { crate::simd_ops::wu64(rdx, rax); crate::simd_ops::wu64(rdi.wrapping_add(0xf0u64), rdx); bb = 31; continue; }
            30 => { r8 = 0; fl_l = 0; fl_r = 0; rcx = rdi; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x60u64))) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 31; continue; }
            31 => { fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 19; continue; } bb = 32; continue; }
            32 => { rcx = rdi; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rdi.wrapping_add(0xf0u64))) as u64; r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 35; continue; } bb = 33; continue; }
            33 => { bb = 34; continue; }
            34 => { rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x60u64))) as u64; r9 = rsp.wrapping_add(0x30); rcx = rbx; rbx = (crate::simd_ops::ru64(rbx)) as u64; rdx = (crate::simd_ops::ru64(rdx.wrapping_add(0x428u64))) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru8(rsp.wrapping_add(0x30u64))) as u8 as u64; fl_r = ((r14 as u8)) as u8 as u64; r8 = (r8 & !0xffu64) | (((if (((fl_l as u8)) == ((fl_r as u8))) { 1u8 } else { 0u8 }) as u8 as u64)); fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 34; continue; } bb = 35; continue; }
            35 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14167a410(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141675d10; const P1: u64 = 0x14167c430; const P2: u64 = 0x14167bd30; const P3: u64 = 0x14167e020;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); crate::simd_ops::wu8(rsp.wrapping_add(0x18u64), ((r8 as u8)) as u8); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; r14 = 0; fl_l = 0; fl_r = 0; rsi = rdx; rdi = rcx; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 16; continue; } bb = 1; continue; }
            1 => { rdx = rbx; rax = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rbx; fl_r = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rsi.wrapping_add(0x8u64), rax); bb = 4; continue; }
            3 => { crate::simd_ops::wu64(r14, rax); bb = 4; continue; }
            4 => { fl_l = rbx; fl_r = (crate::simd_ops::ru64(rsi)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { crate::simd_ops::wu64(rsi, r14); bb = 6; continue; }
            6 => { rbx = rax; rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x14u64))) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x7u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 14; continue; } bb = 7; continue; }
            7 => { r10 = (crate::simd_ops::ru32(rdx.wrapping_add(0x10u64))) as u32 as u64; rax = ((((rax as u32)) & (0xfffffffau32))) as u32 as u64; rax = ((((rax as u32)) | (0x2u32))) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_add(0x14u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0xa0u64))) as u32 as u64; r9 = (crate::simd_ops::ru64(rax)) as u64; r8 = rax.wrapping_add(0x18); rax = ((r9.wrapping_add(r10)) as u64) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 10; continue; } bb = 8; continue; }
            8 => { rax = (crate::simd_ops::ru32(r8)) as u32 as u64; rax = ((rax).wrapping_add(r9)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 10; continue; } bb = 9; continue; }
            9 => { rcx = (((rcx as u32)).wrapping_sub((r10 as u32))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 11; continue; }
            10 => { rcx = (crate::simd_ops::ru32(r8)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((r10 as u32)) as u32 as u64; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 11; continue; }
            11 => { crate::simd_ops::lk_sub_u32(rdi.wrapping_add(0xa8u64), ((rcx as u32)) as u32); rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; rcx = (crate::simd_ops::ru32(rax.wrapping_add(0x18u64))) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_add(0x10u64), ((rcx as u32)) as u32); rax = (crate::simd_ops::ru64(rdi.wrapping_add(0xf0u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 13; continue; } bb = 12; continue; }
            12 => { crate::simd_ops::wu64(rdi.wrapping_add(0xf0u64), rdx); crate::simd_ops::wu64(rdx, r14); bb = 15; continue; }
            13 => { crate::simd_ops::wu64(rdx, rax); crate::simd_ops::wu64(rdi.wrapping_add(0xf0u64), rdx); bb = 15; continue; }
            14 => { r8 = 0; fl_l = 0; fl_r = 0; rcx = rdi; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x60u64))) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 15; continue; }
            15 => { fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 1; continue; } bb = 16; continue; }
            16 => { rcx = rdi; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rdi.wrapping_add(0xf0u64))) as u64; r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 19; continue; } bb = 17; continue; }
            17 => { bb = 18; continue; }
            18 => { rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x60u64))) as u64; r9 = rsp.wrapping_add(0x40); rcx = rbx; rbx = (crate::simd_ops::ru64(rbx)) as u64; rdx = (crate::simd_ops::ru64(rdx.wrapping_add(0x428u64))) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru8(rsp.wrapping_add(0x40u64))) as u8 as u64; fl_r = ((r14 as u8)) as u8 as u64; r8 = (r8 & !0xffu64) | (((if (((fl_l as u8)) == ((fl_r as u8))) { 1u8 } else { 0u8 }) as u8 as u64)); fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 18; continue; } bb = 19; continue; }
            19 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14167a550(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x458; const P1: u64 = 0x450; const P2: u64 = 0x448; const P3: u64 = 0x28; const P4: u64 = 0x70; const P5: u64 = 0x8; const P6: u64 = 0x50; const P7: u64 = 0x14167e260; const P8: u64 = 0x20; const P9: u64 = 0x10; const P10: u64 = 0x14; const P11: u64 = 0x18;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    r10 = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    r11 = 0;
    fl_l = 0; fl_r = 0;
    fl_l = r10; fl_r = r10;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rax = (crate::simd_ops::ru64(rdx.wrapping_add(P3))) as u64; rcx = (crate::simd_ops::ru32(rdx.wrapping_add(P4))) as u32 as u64; rcx = (((rcx) as i64).wrapping_mul((crate::simd_ops::ru64(rax.wrapping_add(P5))) as i64) as u64) as u64; crate::simd_ops::lk_and_u32(r10.wrapping_add(P6), (0xfffffffcu32) as u32); rax = P7; fl_l = ((rcx).wrapping_add(r9)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; crate::simd_ops::wu64(r10.wrapping_add(P8), rax); crate::simd_ops::wu64(r10.wrapping_add(P5), rcx); rax = r10; rcx = (crate::simd_ops::ru32(rsp.wrapping_add(0x28u64))) as u32 as u64; crate::simd_ops::wu32(r10.wrapping_add(P9), ((rcx as u32)) as u32); rcx = (crate::simd_ops::ru32(rsp.wrapping_add(0x30u64))) as u32 as u64; crate::simd_ops::wu32(r10.wrapping_add(P10), ((rcx as u32)) as u32); crate::simd_ops::wu64(r10.wrapping_add(P11), r8); crate::simd_ops::wu64(r10.wrapping_add(0x28u64), r10); crate::simd_ops::wu64(r10.wrapping_add(0x30u64), r11); crate::simd_ops::wu64(r10.wrapping_add(0x48u64), rdx); return rax; }
    rax = (crate::simd_ops::ru64(r10)) as u64;
    fl_l = rax; fl_r = rax;
    if !(((((fl_l as u64)) & ((fl_r as u64))) != 0)) { crate::simd_ops::wu64(rcx.wrapping_add(P1), r11); rax = ((r11 as u32)) as u32 as u64; }
    crate::simd_ops::wu64(rcx.wrapping_add(P0), rax);
    crate::simd_ops::lk_sub_u32(rcx.wrapping_add(P2), (1) as u32);
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P3))) as u64;
    rcx = (crate::simd_ops::ru32(rdx.wrapping_add(P4))) as u32 as u64;
    rcx = (((rcx) as i64).wrapping_mul((crate::simd_ops::ru64(rax.wrapping_add(P5))) as i64) as u64) as u64;
    crate::simd_ops::lk_and_u32(r10.wrapping_add(P6), (0xfffffffcu32) as u32);
    rax = P7;
    fl_l = ((rcx).wrapping_add(r9)) as u64; fl_r = fl_l;
    rcx = ((fl_l as u64)) as u64;
    crate::simd_ops::wu64(r10.wrapping_add(P8), rax);
    crate::simd_ops::wu64(r10.wrapping_add(P5), rcx);
    rax = r10;
    rcx = (crate::simd_ops::ru32(rsp.wrapping_add(0x28u64))) as u32 as u64;
    crate::simd_ops::wu32(r10.wrapping_add(P9), ((rcx as u32)) as u32);
    rcx = (crate::simd_ops::ru32(rsp.wrapping_add(0x30u64))) as u32 as u64;
    crate::simd_ops::wu32(r10.wrapping_add(P10), ((rcx as u32)) as u32);
    crate::simd_ops::wu64(r10.wrapping_add(P11), r8);
    crate::simd_ops::wu64(r10.wrapping_add(0x28u64), r10);
    crate::simd_ops::wu64(r10.wrapping_add(0x30u64), r11);
    crate::simd_ops::wu64(r10.wrapping_add(0x48u64), rdx);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14167a5d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1426452b0; const P1: u64 = 0x14167d130; const P2: u64 = 0x14167d920; const P3: u64 = 0x14167d330; const P4: u64 = 0x14167a550; const P5: u64 = 0x14167db20; const P6: u64 = 0x1415242e0; const P7: u64 = 0x1426452a8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 15>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r9); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xc8u64))) as u64; rdi = 0; fl_l = 0; fl_r = 0; r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0xd0u64))) as u64; r14 = rcx; rbx = r9; r12 = ((r8 as u32)) as u32 as u64; rsi = rdx; crate::simd_ops::wu64(rax, rdi); rax = rcx.wrapping_add(0x18); rcx = rax; crate::simd_ops::wu8(r13, ((rdi as u8)) as u8); crate::simd_ops::wu64(rsp.wrapping_add(0x80u64), rax); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); rdx = ((rdi as u32)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xd0u64), rdx); fl_l = (crate::simd_ops::ru8(r14.wrapping_add(0x418u64))) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 3; continue; } bb = 1; continue; }
            1 => { fl_l = ((r12 as u32)) as u32 as u64; fl_r = 0xffffffff; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 3; continue; } bb = 2; continue; }
            2 => { r9 = (crate::simd_ops::ru32(rsp.wrapping_add(0xa0u64))) as u32 as u64; rax = rsp.wrapping_add(0xd0); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rax); rbp = r14.wrapping_add(0xc0); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xc0u64))) as u64; r8 = rbx; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); rdx = ((r12 as u32)) as u32 as u64; rax = ((crate::simd_ops::ru8(rsp.wrapping_add(0xb0u64))) as u32) as u32 as u64; rcx = rbp; crate::simd_ops::wu8(rsp.wrapping_add(0x28u64), ((rax as u8)) as u8); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0xa8u64))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rax as u32)) as u32); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0xd0u64))) as u64; r15 = ((rax as u32)) as u32 as u64; bb = 4; continue; }
            3 => { r15 = ((rdi as u32)) as u32 as u64; rbp = r14.wrapping_add(0xc0); bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::ru8(rsp.wrapping_add(0xb8u64))) as u8 as u64; fl_r = ((rdi as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 8; continue; } bb = 5; continue; }
            5 => { fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 27; continue; } bb = 6; continue; }
            6 => { fl_l = (crate::simd_ops::ru64(rdx.wrapping_add(0x10u64))) as u64; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 8; continue; } bb = 7; continue; }
            7 => { rcx = rbp; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 27; continue; }
            8 => { rax = (crate::simd_ops::ru64(r14)) as u64; rcx = r14; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x48u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rbx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 26; continue; } bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xd0u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 17; continue; } bb = 10; continue; }
            10 => { r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0xc0u64))) as u64; r9 = rsp.wrapping_add(0xd0); r8 = (crate::simd_ops::ru32(rsp.wrapping_add(0xa8u64))) as u32 as u64; rcx = rbp; rdx = (crate::simd_ops::ru32(r15)) as u32 as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); r8 = (crate::simd_ops::ru64(rsp.wrapping_add(0xd0u64))) as u64; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 11; continue; }
            11 => { rax = (crate::simd_ops::ru32(r15)) as u32 as u64; rdx = rsi; r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x98u64))) as u64; rcx = r14; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rax as u32)) as u32); r9 = r13; rax = (crate::simd_ops::ru32(r8.wrapping_add(0x1cu64))) as u32 as u64; r8 = (crate::simd_ops::ru64(r8.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rax as u32)) as u32); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0xc8u64))) as u64; r9 = ((r12 as u32)) as u32 as u64; rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0xd0u64))) as u64; r8 = rax; crate::simd_ops::wu64(r14, rax); rcx = (crate::simd_ops::ru32(r15)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rcx as u32)) as u32); rcx = rbp; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r13); rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xd0u64))) as u64; crate::simd_ops::lk_and_u32(rbx.wrapping_add(0x14u64), (0xfffffff8u32) as u32); crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), rax); crate::simd_ops::wu32(rbx.wrapping_add(0x10u64), ((rdi as u32)) as u32); rax = (crate::simd_ops::ru64(r14)) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x40u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 13; continue; } bb = 12; continue; }
            12 => { crate::simd_ops::wu64(rax.wrapping_add(0x40u64), rbx); crate::simd_ops::wu64(rbx.wrapping_add(0x18u64), rdi); bb = 22; continue; }
            13 => { crate::simd_ops::wu64(rbx.wrapping_add(0x18u64), rcx); crate::simd_ops::wu64(rax.wrapping_add(0x40u64), rbx); bb = 22; continue; }
            14 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), rdi); fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 16; continue; } bb = 15; continue; }
            15 => { rcx = rbp; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 16; continue; }
            16 => { rdx = rbx; rcx = 0x7; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); bb = 27; continue; }
            17 => { crate::simd_ops::lk_and_u32(rbx.wrapping_add(0x14u64), (0xfffffff8u32) as u32); crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), rax); crate::simd_ops::wu32(rbx.wrapping_add(0x10u64), ((r15 as u32)) as u32); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xd0u64))) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 22; continue; } bb = 18; continue; }
            18 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x40u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 20; continue; } bb = 19; continue; }
            19 => { crate::simd_ops::wu64(rcx.wrapping_add(0x40u64), rbx); crate::simd_ops::wu64(rbx.wrapping_add(0x18u64), rdi); bb = 21; continue; }
            20 => { crate::simd_ops::wu64(rbx.wrapping_add(0x18u64), rax); crate::simd_ops::wu64(rcx.wrapping_add(0x40u64), rbx); bb = 21; continue; }
            21 => { crate::simd_ops::wu8(r13, (0x1u8) as u8); bb = 22; continue; }
            22 => { crate::simd_ops::wu64(rbx, rdi); fl_l = (crate::simd_ops::ru64(rsi.wrapping_add(0xe0u64))) as u64; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 24; continue; } bb = 23; continue; }
            23 => { crate::simd_ops::wu64(rsi.wrapping_add(0xe0u64), rbx); bb = 25; continue; }
            24 => { rax = (crate::simd_ops::ru64(rsi.wrapping_add(0xd8u64))) as u64; crate::simd_ops::wu64(rax, rbx); bb = 25; continue; }
            25 => { crate::simd_ops::wu64(rsi.wrapping_add(0xd8u64), rbx); crate::simd_ops::wu64(rbx.wrapping_add(0x20u64), rsi); bb = 26; continue; }
            26 => { rdi = rbx; bb = 27; continue; }
            27 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P7), rax), rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x88u64))) as u64; rax = rdi; rsp = ((rsp).wrapping_add(0x40u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14167a870(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14167c860; const P1: u64 = 0x448; const P2: u64 = 0x460; const P3: u64 = 0x10; const P4: u64 = 0x7; const P5: u64 = 0x8; const P6: u64 = 0x1415242e0; const P7: u64 = 0x141676290;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdi = rcx;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rsi = 0;
    fl_l = 0; fl_r = 0;
    rbx = ((rdi.wrapping_add(P1)) as u64) as u64;
    fl_l = (crate::simd_ops::ru64(rdi.wrapping_add(P2))) as u64; fl_r = rsi;
    if !((((fl_l as u64)) == ((fl_r as u64)))) { crate::simd_ops::wu64(rbx.wrapping_add(P3), rsi); rcx = ((rsi.wrapping_add(P4)) as u32) as u32 as u64; crate::simd_ops::wu64(rbx.wrapping_add(P5), rsi); crate::simd_ops::wu32(rbx, ((rsi as u32)) as u32); rdx = (crate::simd_ops::ru64(rdi.wrapping_add(P2))) as u64; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); }
    rcx = rdi;
    crate::simd_ops::wu64(rbx.wrapping_add(P3), rsi);
    crate::simd_ops::wu64(rbx.wrapping_add(P5), rsi);
    crate::simd_ops::wu32(rbx, ((rsi as u32)) as u32);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(P7, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14167a8e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1426452b0; const P1: u64 = 0xfffffffffffffff8; const P2: u64 = 0xffffffffffffff80; const P3: u64 = 0xfffffffffffffffc; const P4: u64 = 0x141678600; const P5: u64 = 0x48; const P6: u64 = 0x58; const P7: u64 = 0xffffffffffffffb0; const P8: u64 = 0x1426452a8; const P9: u64 = 0x14167c780; const P10: u64 = 0x14167bef0; const P11: u64 = 0xffffffffffffffe0; const P12: u64 = 0x14167c920;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rcx; rcx = ((rcx).wrapping_add(0xffffffffffffffb0u64)) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); rdx = 0x3; crate::simd_ops::wu8(rbx.wrapping_add(P1), (0x1u8) as u8); rcx = ((rbx.wrapping_add(P2)) as u64) as u64; crate::simd_ops::wu8(rbx.wrapping_add(P3), (0x0u8) as u8); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(P5))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(P6))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rcx = ((rbx.wrapping_add(P7)) as u64) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P8), rax), rax, rcx, rdx, r8, r9, rsp); }
            3 => { rcx = ((rbx.wrapping_add(P2)) as u64) as u64; rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); rcx = ((rbx.wrapping_add(P2)) as u64) as u64; rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp); rcx = ((rbx.wrapping_add(P7)) as u64) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P8), rax), rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P11))) as u64; rdx = rbx.wrapping_sub(0x80); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P12, rax, rcx, rdx, r8, r9, rsp); }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14167a980(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x14167d920; const P2: u64 = 0x1415242e0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdi = rdx;
    rbx = 0;
    fl_l = 0; fl_r = 0;
    rdx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    crate::simd_ops::wu64(rdi.wrapping_add(P0), rbx);
    fl_l = rdx; fl_r = rdx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rcx = ((rcx).wrapping_add(0xc0u64)) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rbx = ((rax as u32)) as u32 as u64; }
    rdx = rdi;
    rcx = 0x7;
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    rax = ((rbx as u32)) as u32 as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14167a9d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1426452b0; const P1: u64 = 0x14167d920; const P2: u64 = 0x1415242e0; const P3: u64 = 0x1426452a8; const P4: u64 = 0x141675d10; const P5: u64 = 0x14167c430; const P6: u64 = 0x14167e020;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x38u64)) as u64; rax = ((crate::simd_ops::ru8(rcx.wrapping_add(0xd0u64))) as u32) as u32 as u64; rdi = rcx; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), r13); r13 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), r14); r14 = (r14 & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r15); r15 = ((rdx as u32)) as u32 as u64; fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(0xb8u64))) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 26; continue; } bb = 1; continue; }
            1 => { rbx = (crate::simd_ops::ru64(rcx.wrapping_add(0xc8u64))) as u64; rsi = ((r13 as u32)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), r12); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 4; continue; } bb = 2; continue; }
            2 => { bb = 3; continue; }
            3 => { rsi = rbx; rbx = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rax; fl_r = (0x1u64) as u64; rax = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 4; continue; }
            4 => { r12 = (crate::simd_ops::ru64(rcx.wrapping_add(0x60u64))) as u64; rcx = r12.wrapping_add(0x18); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 23; continue; } bb = 5; continue; }
            5 => { crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rbp); bb = 6; continue; }
            6 => { fl_l = ((r14 as u8)) as u8 as u64; fl_r = ((r14 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 9; continue; } bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rcx = (crate::simd_ops::ru32(rax.wrapping_add(0x18u64))) as u32 as u64; rcx = (((rcx as u32)).wrapping_sub(crate::simd_ops::ru32(rbx.wrapping_add(0x10u64)))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((r15 as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 9; continue; } bb = 8; continue; }
            8 => { rsi = rbx; rbx = (crate::simd_ops::ru64(rbx)) as u64; bb = 21; continue; }
            9 => { r14 = (r14 & !0xffu64) | (((0x1u8) as u8 as u64)); rax = (crate::simd_ops::ru64(rbx)) as u64; rbp = rbx; fl_l = rbx; fl_r = (crate::simd_ops::ru64(rdi.wrapping_add(0xc8u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 11; continue; } bb = 10; continue; }
            10 => { crate::simd_ops::wu64(rdi.wrapping_add(0xc8u64), rax); bb = 12; continue; }
            11 => { crate::simd_ops::wu64(rsi, rax); bb = 12; continue; }
            12 => { fl_l = rbx; fl_r = (crate::simd_ops::ru64(rdi.wrapping_add(0xc0u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 14; continue; } bb = 13; continue; }
            13 => { crate::simd_ops::wu64(rdi.wrapping_add(0xc0u64), rsi); bb = 14; continue; }
            14 => { crate::simd_ops::lk_sub_u32(rdi.wrapping_add(0xb8u64), (1) as u32); rbx = rax; rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x8u64))) as u64; r9 = (crate::simd_ops::ru32(rbp.wrapping_add(0x10u64))) as u32 as u64; rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0xa0u64))) as u32 as u64; r8 = (crate::simd_ops::ru64(rax)) as u64; rdx = rax.wrapping_add(0x18); rax = ((r8.wrapping_add(r9)) as u64) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 17; continue; } bb = 15; continue; }
            15 => { rax = (crate::simd_ops::ru32(rdx)) as u32 as u64; rax = ((rax).wrapping_add(r8)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 17; continue; } bb = 16; continue; }
            16 => { rcx = (((rcx as u32)).wrapping_sub((r9 as u32))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 18; continue; }
            17 => { rcx = (crate::simd_ops::ru32(rdx)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 18; continue; }
            18 => { crate::simd_ops::lk_sub_u32(rdi.wrapping_add(0xa8u64), ((rcx as u32)) as u32); rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x8u64))) as u64; rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x60u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x8u64), r13); fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 20; continue; } bb = 19; continue; }
            19 => { rcx = ((rcx).wrapping_add(0xc0u64)) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 20; continue; }
            20 => { rdx = rbp; rcx = 0x7; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 21; continue; }
            21 => { fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 6; continue; } bb = 22; continue; }
            22 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; bb = 23; continue; }
            23 => { rcx = r12.wrapping_add(0x18); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; fl_l = ((r14 as u8)) as u8 as u64; fl_r = ((r14 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 26; continue; } bb = 24; continue; }
            24 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x60u64))) as u64; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x418u64))) as u8 as u64; fl_r = ((r13 as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 26; continue; } bb = 25; continue; }
            25 => { rbx = (crate::simd_ops::ru64(rdi.wrapping_add(0xe0u64))) as u64; rsi = r13; crate::simd_ops::wu8(rsp.wrapping_add(0x50u64), (0x1u8) as u8); bb = 31; continue; }
            26 => { rbx = (crate::simd_ops::ru64(rdi.wrapping_add(0xe0u64))) as u64; rsi = r13; crate::simd_ops::wu8(rsp.wrapping_add(0x50u64), ((r13 as u8)) as u8); fl_l = ((r14 as u8)) as u8 as u64; fl_r = ((r14 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 31; continue; } bb = 27; continue; }
            27 => { fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 48; continue; } bb = 28; continue; }
            28 => { bb = 29; continue; }
            29 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rcx = (crate::simd_ops::ru32(rax.wrapping_add(0x18u64))) as u32 as u64; rcx = (((rcx as u32)).wrapping_sub(crate::simd_ops::ru32(rbx.wrapping_add(0x10u64)))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((r15 as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 33; continue; } bb = 30; continue; }
            30 => { rsi = rbx; rbx = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 29; continue; } bb = 31; continue; }
            31 => { fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 48; continue; } bb = 32; continue; }
            32 => { bb = 33; continue; }
            33 => { rdx = rbx; rax = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rbx; fl_r = (crate::simd_ops::ru64(rdi.wrapping_add(0xe0u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 35; continue; } bb = 34; continue; }
            34 => { crate::simd_ops::wu64(rdi.wrapping_add(0xe0u64), rax); bb = 36; continue; }
            35 => { crate::simd_ops::wu64(rsi, rax); bb = 36; continue; }
            36 => { fl_l = rbx; fl_r = (crate::simd_ops::ru64(rdi.wrapping_add(0xd8u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 38; continue; } bb = 37; continue; }
            37 => { crate::simd_ops::wu64(rdi.wrapping_add(0xd8u64), rsi); bb = 38; continue; }
            38 => { rbx = rax; rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x14u64))) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x7u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 46; continue; } bb = 39; continue; }
            39 => { r10 = (crate::simd_ops::ru32(rdx.wrapping_add(0x10u64))) as u32 as u64; rax = ((((rax as u32)) & (0xfffffffau32))) as u32 as u64; rax = ((((rax as u32)) | (0x2u32))) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_add(0x14u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0xa0u64))) as u32 as u64; r9 = (crate::simd_ops::ru64(rax)) as u64; r8 = rax.wrapping_add(0x18); rax = ((r9.wrapping_add(r10)) as u64) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 42; continue; } bb = 40; continue; }
            40 => { rax = (crate::simd_ops::ru32(r8)) as u32 as u64; rax = ((rax).wrapping_add(r9)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 42; continue; } bb = 41; continue; }
            41 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 43; continue; }
            42 => { rcx = (crate::simd_ops::ru32(r8)) as u32 as u64; bb = 43; continue; }
            43 => { rcx = (((rcx as u32)).wrapping_sub((r10 as u32))) as u32 as u64; crate::simd_ops::lk_sub_u32(rdi.wrapping_add(0xa8u64), ((rcx as u32)) as u32); rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; rcx = (crate::simd_ops::ru32(rax.wrapping_add(0x18u64))) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_add(0x10u64), ((rcx as u32)) as u32); rax = (crate::simd_ops::ru64(rdi.wrapping_add(0xf0u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 45; continue; } bb = 44; continue; }
            44 => { crate::simd_ops::wu64(rdi.wrapping_add(0xf0u64), rdx); crate::simd_ops::wu64(rdx, r13); bb = 47; continue; }
            45 => { crate::simd_ops::wu64(rdx, rax); crate::simd_ops::wu64(rdi.wrapping_add(0xf0u64), rdx); bb = 47; continue; }
            46 => { r8 = 0; fl_l = 0; fl_r = 0; rcx = rdi; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x60u64))) as u64; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); bb = 47; continue; }
            47 => { fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 33; continue; } bb = 48; continue; }
            48 => { rbx = (crate::simd_ops::ru64(rdi.wrapping_add(0xf0u64))) as u64; r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 51; continue; } bb = 49; continue; }
            49 => { bb = 50; continue; }
            50 => { rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x60u64))) as u64; r9 = rsp.wrapping_add(0x50); rcx = rbx; rbx = (crate::simd_ops::ru64(rbx)) as u64; rdx = (crate::simd_ops::ru64(rdx.wrapping_add(0x428u64))) as u64; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru8(rsp.wrapping_add(0x50u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; r8 = (r8 & !0xffu64) | (((if (((fl_l as u8)) == ((fl_r as u8))) { 1u8 } else { 0u8 }) as u8 as u64)); fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 50; continue; } bb = 51; continue; }
            51 => { rsp = ((rsp).wrapping_add(0x38u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14167ad00(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xffffffffffffffb0; const P1: u64 = 0x1426452b0; const P2: u64 = 0x48; const P3: u64 = 0x38; const P4: u64 = 0x1426452a8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = ((rcx.wrapping_add(P0)) as u64) as u64;
    rdi = rcx;
    rcx = rbx;
    rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp);
    fl_l = (crate::simd_ops::ru64(rdi.wrapping_add(P2))) as u64; fl_r = (0x0u64) as u64;
    if !((((fl_l as u64)) == ((fl_r as u64)))) { rdi = 0x2; } else { rdi = (crate::simd_ops::ru32(rdi.wrapping_add(P3))) as u32 as u64; }
    rcx = rbx;
    rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rax = ((rdi as u32)) as u32 as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14167ad50(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x2c; const P1: u64 = 0x141677050; const P2: u64 = 0x6; const P3: u64 = 0x141524330; const P4: u64 = 0x460; const P5: u64 = 0x1; const P6: u64 = 0x40; const P7: u64 = 0x458; const P8: u64 = 0x450; const P9: u64 = 0x448;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; fl_l = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64; fl_r = 0x0; rdi = rdx; rbx = rcx; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = 0x1f; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            2 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rsi); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rsi = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 12; continue; } bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru32(rdi.wrapping_add(P0))) as u32 as u64; rdx = (((rcx) as i64).wrapping_mul((0x58u64) as i64) as u64) as u64; rcx = ((rax.wrapping_add(P2)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(P4), rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = ((rsi.wrapping_add(P5)) as u32) as u32 as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            5 => { rcx = (crate::simd_ops::ru32(rdi.wrapping_add(P0))) as u32 as u64; rdx = (((rcx) as i64).wrapping_mul((0x58u64) as i64) as u64) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; r8 = 0; fl_l = 0; fl_r = 0; bb = 6; continue; }
            6 => { fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { crate::simd_ops::wu64(rax, r8); crate::simd_ops::wu64(rax.wrapping_add(P6), r8); bb = 8; continue; }
            8 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P7))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { crate::simd_ops::wu64(rbx.wrapping_add(P7), rax); crate::simd_ops::wu64(rbx.wrapping_add(P8), rax); crate::simd_ops::wu64(rax, r8); bb = 11; continue; }
            10 => { crate::simd_ops::wu64(rax, rcx); crate::simd_ops::wu64(rbx.wrapping_add(P7), rax); bb = 11; continue; }
            11 => { crate::simd_ops::lk_add_u32(rbx.wrapping_add(P9), (1) as u32); rax = ((rax).wrapping_add(0x58u64)) as u64; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 6; continue; } bb = 12; continue; }
            12 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rax = ((rsi as u32)) as u32 as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14167ae20(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xffffffffffffffdf; const P1: u64 = 0x141524330; const P2: u64 = 0x14; const P3: u64 = 0x8; const P4: u64 = 0x10; const P5: u64 = 0x18; const P6: u64 = 0x20;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rdx = 0x28;
    rcx = ((rdx.wrapping_add(P0)) as u32) as u32 as u64;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rdx = 0;
    fl_l = 0; fl_r = 0;
    fl_l = rax; fl_r = rax;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rcx = (crate::simd_ops::ru32(rax.wrapping_add(P2))) as u32 as u64; rcx = ((((rcx as u32)) & (0xfffffffbu32))) as u32 as u64; crate::simd_ops::wu64(rax, rdx); rcx = ((((rcx as u32)) | (0xbu32))) as u32 as u64; crate::simd_ops::wu64(rax.wrapping_add(P3), rdx); crate::simd_ops::wu32(rax.wrapping_add(P2), ((rcx as u32)) as u32); crate::simd_ops::wu32(rax.wrapping_add(P4), ((rdx as u32)) as u32); crate::simd_ops::wu64(rax.wrapping_add(P5), rdx); crate::simd_ops::wu64(rax.wrapping_add(P6), rdx); rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
    rax = rdx;
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14167ae80(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141677ae0; const P1: u64 = 0x1416763e0; const P2: u64 = 0x14167e110;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedStack::<1>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x58u64)) as u64; rbp = rsp.wrapping_add(0x30); rdi = (crate::simd_ops::ru32(rcx.wrapping_add(0x8u64))) as u32 as u64; r12 = 0; fl_l = 0; fl_r = 0; rdi = (((rdi as u32)).wrapping_sub(crate::simd_ops::ru32(rcx.wrapping_add(0x6cu64)))) as u32 as u64; r14 = rcx; rsi = ((r12 as u32)) as u32 as u64; crate::simd_ops::wu64(rbp, r12); r15 = (0xffffffffffffff0u64) as u64; rdx = ((rdi.wrapping_add(rdi.wrapping_mul(2))) as u64) as u64; rdx = ((rdx).wrapping_shl(3u32)) as u64; rax = rdx.wrapping_add(0xf); fl_l = rax; fl_r = rdx; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = r15; bb = 2; continue; }
            2 => { rax = (((rax) & (0xfffffffffffffff0u64))) as u64; rsp = ((rsp).wrapping_sub(rax)) as u64; rax = rsp.wrapping_add(0x30); crate::simd_ops::wu64(rbp.wrapping_add(0x8u64), rax); rax = rdx.wrapping_add(0xf); fl_l = rax; fl_r = rdx; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = r15; bb = 4; continue; }
            4 => { rax = (((rax) & (0xfffffffffffffff0u64))) as u64; rsp = ((rsp).wrapping_sub(rax)) as u64; xmm0 = 0u128; crate::simd_ops::wu32(rbp.wrapping_add(0x78u64), ((xmm0 as u32)) as u32); rax = rsp.wrapping_add(0x30); crate::simd_ops::wu64(rbp.wrapping_add(0x10u64), rax); fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 40; continue; } bb = 5; continue; }
            5 => { bb = 6; continue; }
            6 => { rdx = rbp.wrapping_add(0x78); rcx = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 7; continue; }
            7 => { rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r10 = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 9; continue; } bb = 8; continue; }
            8 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(r10.wrapping_add(0x10u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 16; continue; }
            9 => { r9 = rbp.wrapping_add(0x70); crate::simd_ops::wu8(rsp.wrapping_add(0x20u64), ((r12 as u8)) as u8); r8 = rbp.wrapping_add(0x80); rdx = rbp.wrapping_add(0x88); (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(r10.wrapping_add(0x20u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { rax = (crate::simd_ops::ru64(rbx)) as u64; r8 = ((rdx.wrapping_add(0x35u64)) as u32) as u32 as u64; r9 = 0; fl_l = 0; fl_r = 0; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x28u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 16; continue; }
            11 => { r9 = (crate::simd_ops::ru64(rbp.wrapping_add(0x80u64))) as u64; fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 14; continue; } bb = 12; continue; }
            12 => { fl_l = (crate::simd_ops::ru8(rbp.wrapping_add(0x70u64))) as u8 as u64; fl_r = ((r12 as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 16; continue; } bb = 13; continue; }
            13 => { rax = (crate::simd_ops::ru64(rbx)) as u64; r8 = ((r9.wrapping_add(0x1u64)) as u32) as u32 as u64; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x28u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 16; continue; }
            14 => { rcx = (crate::simd_ops::ru32(r9.wrapping_add(0x50u64))) as u32 as u64; rax = ((rcx as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shr(1u32)) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 16; continue; } bb = 15; continue; }
            15 => { rcx = ((((rcx as u32)) | (0x2u32))) as u32 as u64; crate::simd_ops::wu32(r9.wrapping_add(0x50u64), ((rcx as u32)) as u32); r8 = ((crate::simd_ops::ru8(rbx.wrapping_add(0x76u64))) as u32) as u32 as u64; rax = (((r8 as u8)) as u32) as u32 as u64; fl_l = 0; fl_r = ((rax as u8)) as u8 as u64; rax = (rax & !0xffu64) | (((0u8.wrapping_sub(fl_r as u8)) as u8 as u64)); rcx = (if (((fl_l as u8)) < ((fl_r as u8))) { 0xffffffffffffffffu64 } else { 0u64 }) as u64; rcx = ((((rcx as u32)) & (0x4u32))) as u32 as u64; fl_l = 0; fl_r = ((r8 as u8)) as u8 as u64; r8 = (r8 & !0xffu64) | (((0u8.wrapping_sub(fl_r as u8)) as u8 as u64)); rdx = (crate::simd_ops::ru32(rbp.wrapping_add(rcx))) as u32 as u64; r8 = ((rdx.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64; rax = ((rdx.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(rcx), ((rax as u32)) as u32); rcx = (if (((fl_l as u8)) < ((fl_r as u8))) { 0xffffffffffffffffu64 } else { 0u64 }) as u64; rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x88u64))) as u64; rcx = ((((rcx as u32)) & (0x8u32))) as u32 as u64; fl_l = (((rsi as u32)).wrapping_add(1)) as u32 as u64; rsi = ((fl_l as u32)) as u32 as u64; rdx = (crate::simd_ops::ru64(rbp.wrapping_add(rcx).wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rdx.wrapping_add(r8.wrapping_mul(8)), rax); rax = ((crate::simd_ops::ru8(rbx.wrapping_add(0x74u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rdx.wrapping_add(r8.wrapping_mul(8)).wrapping_add(0xcu64), ((rax as u8)) as u8); rax = r9.wrapping_add(0x8); xmm0 = ((crate::simd_ops::ru32(rbp.wrapping_add(0x78u64))) as u32) as u128; crate::simd_ops::wu32(rdx.wrapping_add(r8.wrapping_mul(8)).wrapping_add(0x8u64), ((xmm0 as u32)) as u32); crate::simd_ops::wu64(rdx.wrapping_add(r8.wrapping_mul(8)).wrapping_add(0x10u64), rax); bb = 16; continue; }
            16 => { fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 6; continue; } bb = 17; continue; }
            17 => { fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 40; continue; } bb = 18; continue; }
            18 => { rcx = (crate::simd_ops::ru64(r14.wrapping_add(0x428u64))) as u64; rdi = rbp.wrapping_add(0x8); r14 = rbp; crate::simd_ops::wu64(rbp.wrapping_add(0x70u64), rcx); crate::simd_ops::wu64(rbp.wrapping_add(0x80u64), r14); crate::simd_ops::wu64(rbp.wrapping_add(0x78u64), rdi); bb = 19; continue; }
            19 => { r14 = (crate::simd_ops::ru32(r14)) as u32 as u64; fl_l = ((r14 as u32)) as u32 as u64; fl_r = ((r14 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 39; continue; } bb = 20; continue; }
            20 => { r15 = ((r14.wrapping_mul(4)) as u64) as u64; rsi = ((r14 as u32)) as u32 as u64; rax = r15.wrapping_add(0xf); fl_l = rax; fl_r = r15; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 22; continue; } bb = 21; continue; }
            21 => { rax = (0xffffffffffffff0u64) as u64; bb = 22; continue; }
            22 => { rax = (((rax) & (0xfffffffffffffff0u64))) as u64; r13 = (crate::simd_ops::ru64(rdi)) as u64; r9 = P2; rsp = ((rsp).wrapping_sub(rax)) as u64; r8 = r13; rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = ((r14 as u32)) as u32 as u64; rbx = rsp.wrapping_add(0x30); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbx); fl_l = ((r12 as u32)) as u32 as u64; fl_r = ((r12 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 24; continue; } bb = 23; continue; }
            23 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x28u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 25; continue; }
            24 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x30u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 25; continue; }
            25 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 38; continue; } bb = 26; continue; }
            26 => { rax = r15.wrapping_add(0xf); rdi = 0; fl_l = 0; fl_r = 0; fl_l = rax; fl_r = r15; r15 = (0xffffffffffffff0u64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 28; continue; } bb = 27; continue; }
            27 => { rax = r15; bb = 28; continue; }
            28 => { rax = (((rax) & (0xfffffffffffffff0u64))) as u64; rsp = ((rsp).wrapping_sub(rax)) as u64; rax = ((r14.wrapping_mul(8)) as u64) as u64; rcx = rax.wrapping_add(0xf); r8 = rsp.wrapping_add(0x30); fl_l = rcx; fl_r = rax; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 30; continue; } bb = 29; continue; }
            29 => { rcx = r15; bb = 30; continue; }
            30 => { rcx = (((rcx) & (0xfffffffffffffff0u64))) as u64; rax = rcx; rsp = ((rsp).wrapping_sub(rcx)) as u64; rdx = rsp.wrapping_add(0x30); fl_l = ((r14 as u32)) as u32 as u64; fl_r = ((r14 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 37; continue; } bb = 31; continue; }
            31 => { r9 = r13.wrapping_add(0x10); bb = 32; continue; }
            32 => { fl_l = (crate::simd_ops::ru32(rbx)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 34; continue; } bb = 33; continue; }
            33 => { rax = (crate::simd_ops::ru64(r9)) as u64; crate::simd_ops::wu64(rdx.wrapping_add(rdi.wrapping_mul(8)), rax); rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(rdi.wrapping_mul(4)), ((rax as u32)) as u32); fl_l = (((rdi as u32)).wrapping_add(1)) as u32 as u64; rdi = ((fl_l as u32)) as u32 as u64; bb = 34; continue; }
            34 => { r9 = ((r9).wrapping_add(0x18u64)) as u64; rbx = ((rbx).wrapping_add(0x4u64)) as u64; fl_l = rsi; fl_r = (0x1u64) as u64; rsi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 32; continue; } bb = 35; continue; }
            35 => { fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 37; continue; } bb = 36; continue; }
            36 => { rcx = ((rdi as u32)) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 37; continue; }
            37 => { rdi = (crate::simd_ops::ru64(rbp.wrapping_add(0x78u64))) as u64; bb = 38; continue; }
            38 => { rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x70u64))) as u64; bb = 39; continue; }
            39 => { r14 = (crate::simd_ops::ru64(rbp.wrapping_add(0x80u64))) as u64; rdi = ((rdi).wrapping_add(0x8u64)) as u64; r14 = ((r14).wrapping_add(0x4u64)) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x78u64), rdi); r12 = (((r12 as u32)).wrapping_add(1)) as u32 as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x80u64), r14); fl_l = ((r12 as u32)) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 19; continue; } bb = 40; continue; }
            40 => { rsp = rbp.wrapping_add(0x28); r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14167b310(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1426452b0; const P1: u64 = 0x14167c5b0; const P2: u64 = 0x1426452a8; const P3: u64 = 0x14167a5d0; const P4: u64 = 0x141678970;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 17>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x60u64)) as u64; crate::simd_ops::wu64(r8, (0x0u64) as u64); rsi = rcx; crate::simd_ops::wu8(r9, (0x0u8) as u8); r12 = r9; rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x28u64))) as u64; r15 = r8; rcx = ((rcx).wrapping_add(0x30u64)) as u64; crate::simd_ops::wu64(rdx, rax); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x60u64))) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru8(rsi.wrapping_add(0x78u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 18; continue; } bb = 1; continue; }
            1 => { rcx = rsi.wrapping_add(0x30); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); rdi = ((crate::simd_ops::ru8(rsi.wrapping_add(0x7cu64))) as u32) as u32 as u64; rcx = rsi.wrapping_add(0x30); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = ((rdi as u8)) as u8 as u64; fl_r = ((rdi as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 18; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rsi)) as u64; rcx = rsi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x68u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rdi = rax; rax = (crate::simd_ops::ru32(rsi.wrapping_add(0xa0u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 5; continue; } bb = 3; continue; }
            3 => { fl_l = rdi; fl_r = rax; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rdi = (crate::simd_ops::ru32(rsi.wrapping_add(0x9cu64))) as u32 as u64; bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x28u64))) as u64; rbp = (crate::simd_ops::ru32(rsi.wrapping_add(0xa4u64))) as u32 as u64; rcx = (crate::simd_ops::ru64(rax)) as u64; rax = ((rdi.wrapping_add(rbp)) as u64) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 7; continue; } bb = 6; continue; }
            6 => { r14 = (r14 & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 8; continue; }
            7 => { rbp = 0; fl_l = 0; fl_r = 0; rax = ((rcx as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_sub((rdi as u32))) as u32 as u64; r14 = (r14 & !0xffu64) | (((0x1u8) as u8 as u64)); fl_l = rdi; fl_r = rcx; rbp = (if (((fl_l as u64)) < ((fl_r as u64))) { (rax as u32) } else { (rbp as u32) }) as u32 as u64; bb = 8; continue; }
            8 => { fl_l = (crate::simd_ops::ru8(rsi.wrapping_add(0x7bu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 11; continue; } bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru64(rsi)) as u64; rcx = rsi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x40u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rdx = ((rbp as u32)) as u32 as u64; rdx = ((rdx).wrapping_add(rdi)) as u64; rcx = ((rax as u32)) as u32 as u64; fl_l = rdx; fl_r = rcx; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 11; continue; } bb = 10; continue; }
            10 => { rbp = ((rax as u32)) as u32 as u64; fl_l = ((rbp as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; rbp = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 11; continue; }
            11 => { crate::simd_ops::wu32(rsp.wrapping_add(0x90u64), ((rbp as u32)) as u32); fl_l = ((rbp as u32)) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 18; continue; } bb = 12; continue; }
            12 => { rcx = (crate::simd_ops::ru32(rsi.wrapping_add(0xacu64))) as u32 as u64; rax = rsp.wrapping_add(0x98); r8 = (crate::simd_ops::ru32(rsi.wrapping_add(0x90u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), r12); r9 = rdi; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rax); rcx = (if (((fl_l as u32)) >= ((fl_r as u32))) { (rbp as u32) } else { (rcx as u32) }) as u32 as u64; rax = rsp.wrapping_add(0x90); rdx = rsi; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rax); rax = ((crate::simd_ops::ru8(rsp.wrapping_add(0xb0u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x38u64), ((rax as u8)) as u8); rax = (crate::simd_ops::ru32(rsi.wrapping_add(0xb0u64))) as u32 as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x30u64), ((r14 as u8)) as u8); crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rax as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rcx as u32)) as u32); rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x60u64))) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rbp = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 18; continue; } bb = 13; continue; }
            13 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x98u64))) as u64; crate::simd_ops::wu64(r15, rax); rdx = (crate::simd_ops::ru32(rsi.wrapping_add(0xa0u64))) as u32 as u64; fl_l = rdi; fl_r = rdx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 16; continue; } bb = 14; continue; }
            14 => { rcx = (crate::simd_ops::ru32(rsp.wrapping_add(0x90u64))) as u32 as u64; rax = ((rdi.wrapping_add(rcx)) as u64) as u64; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 17; continue; } bb = 15; continue; }
            15 => { rcx = ((rdx as u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x90u64), ((rcx as u32)) as u32); bb = 17; continue; }
            16 => { rcx = (crate::simd_ops::ru32(rsp.wrapping_add(0x90u64))) as u32 as u64; bb = 17; continue; }
            17 => { crate::simd_ops::lk_add_u32(rsi.wrapping_add(0xa8u64), ((rcx as u32)) as u32); rcx = rsi; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x60u64))) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x70u64))) as u64; crate::simd_ops::wu64(rsi.wrapping_add(0x58u64), rcx); bb = 19; continue; }
            18 => { rbp = 0; fl_l = 0; fl_r = 0; bb = 19; continue; }
            19 => { rcx = rsi.wrapping_add(0x30); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); r11 = rsp.wrapping_add(0x60); rax = rbp; rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x40u64))) as u64; rbp = (crate::simd_ops::ru64(r11.wrapping_add(0x48u64))) as u64; rsp = r11; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14167b4e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1426452b0; const P1: u64 = 0x14167c5b0; const P2: u64 = 0x1426452a8; const P3: u64 = 0x14167cc40; const P4: u64 = 0x14167a980; const P5: u64 = 0x14167a550; const P6: u64 = 0x141678600;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 15>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), r8); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rsi = rcx; crate::simd_ops::wu64(r8, (0x0u64) as u64); rcx = ((rcx).wrapping_add(0x30u64)) as u64; crate::simd_ops::wu8(r9, (0x0u8) as u8); r14 = rdx; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x60u64))) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru8(rsi.wrapping_add(0x78u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 14; continue; } bb = 1; continue; }
            1 => { rcx = rsi.wrapping_add(0x30); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); rdi = ((crate::simd_ops::ru8(rsi.wrapping_add(0x7cu64))) as u32) as u32 as u64; rcx = rsi.wrapping_add(0x30); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = ((rdi as u8)) as u8 as u64; fl_r = ((rdi as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 14; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x28u64))) as u64; r13 = rsi.wrapping_add(0x88); crate::simd_ops::wu64(r14, rax); rax = (crate::simd_ops::ru32(rsi.wrapping_add(0xb0u64))) as u32 as u64; rcx = (crate::simd_ops::ru32(rsi.wrapping_add(0xa0u64))) as u32 as u64; r14 = ((rax as u32)) as u32 as u64; r14 = ((r14).wrapping_add(crate::simd_ops::ru64(r13))) as u64; rcx = (((rcx as u32)).wrapping_sub((rax as u32))) as u32 as u64; rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x60u64))) as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x80u64), (0x0u8) as u8); rdx = (crate::simd_ops::ru32(rax.wrapping_add(0x430u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; rdx = (if (((fl_l as u32)) <= ((fl_r as u32))) { (rcx as u32) } else { (rdx as u32) }) as u32 as u64; fl_l = (crate::simd_ops::ru8(rsi.wrapping_add(0x76u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((rdx as u32)) as u32); rbp = ((rdx as u32)) as u32 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 9; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x28u64))) as u64; rcx = (crate::simd_ops::ru64(rax)) as u64; rax = ((rdx as u32)) as u32 as u64; rax = ((rax).wrapping_add(r14)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 5; continue; } bb = 4; continue; }
            4 => { crate::simd_ops::wu8(rsp.wrapping_add(0x80u64), (0x0u8) as u8); bb = 6; continue; }
            5 => { rbp = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu8(rsp.wrapping_add(0x80u64), (0x1u8) as u8); rax = ((rcx as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_sub((r14 as u32))) as u32 as u64; fl_l = r14; fl_r = rcx; rbp = (if (((fl_l as u64)) < ((fl_r as u64))) { (rax as u32) } else { (rbp as u32) }) as u32 as u64; bb = 6; continue; }
            6 => { fl_l = (crate::simd_ops::ru8(rsi.wrapping_add(0x7bu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 9; continue; } bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru64(rsi)) as u64; rcx = rsi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x40u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rdx = ((rbp as u32)) as u32 as u64; rdx = ((rdx).wrapping_add(r14)) as u64; rcx = ((rax as u32)) as u32 as u64; fl_l = rdx; fl_r = rcx; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 9; continue; } bb = 8; continue; }
            8 => { rbp = ((rax as u32)) as u32 as u64; fl_l = ((rbp as u32)) as u32 as u64; fl_r = ((r14 as u32)) as u32 as u64; rbp = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 9; continue; }
            9 => { r15 = (crate::simd_ops::ru64(rsi.wrapping_add(0x60u64))) as u64; rax = (crate::simd_ops::ru32(rsi.wrapping_add(0xb0u64))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x98u64), ((rax as u32)) as u32); rcx = r15.wrapping_add(0x18); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(r15)) as u64; rcx = r15; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x48u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rdi = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 10; continue; }
            10 => { fl_l = (crate::simd_ops::ru64(r13.wrapping_add(0x10u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 16; continue; } bb = 11; continue; }
            11 => { rcx = r15.wrapping_add(0xc0); rdx = r13; r8 = rsp.wrapping_add(0x38); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; fl_l = r13; fl_r = r13; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 17; continue; } bb = 12; continue; }
            12 => { rdx = rdi; rcx = r15; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 13; continue; }
            13 => { rcx = r15.wrapping_add(0x18); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); bb = 14; continue; }
            14 => { rdi = 0; fl_l = 0; fl_r = 0; bb = 15; continue; }
            15 => { rcx = rsi.wrapping_add(0x30); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x88u64))) as u64; rax = rdi; rsp = ((rsp).wrapping_add(0x40u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            16 => { crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), r13); bb = 17; continue; }
            17 => { rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x30u64))) as u32 as u64; r9 = r14; r8 = (crate::simd_ops::ru32(rsp.wrapping_add(0x98u64))) as u32 as u64; rdx = rsi; r8 = ((r8).wrapping_add(crate::simd_ops::ru64(r13.wrapping_add(0x8u64)))) as u64; rcx = r15; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rbp as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rax as u32)) as u32); rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; r14 = rax; crate::simd_ops::wu64(rcx.wrapping_add(0x10u64), rax); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x98u64))) as u32 as u64; crate::simd_ops::lk_and_u32(rdi.wrapping_add(0x14u64), (0xfffffff8u32) as u32); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; crate::simd_ops::wu64(rdi.wrapping_add(0x8u64), rcx); crate::simd_ops::wu32(rdi.wrapping_add(0x10u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(r14.wrapping_add(0x40u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 19; continue; } bb = 18; continue; }
            18 => { crate::simd_ops::wu64(r14.wrapping_add(0x40u64), rdi); crate::simd_ops::wu64(rdi.wrapping_add(0x18u64), rax); bb = 20; continue; }
            19 => { crate::simd_ops::wu64(rdi.wrapping_add(0x18u64), rax); crate::simd_ops::wu64(r14.wrapping_add(0x40u64), rdi); bb = 20; continue; }
            20 => { crate::simd_ops::wu64(rdi, (0x0u64) as u64); fl_l = (crate::simd_ops::ru64(rsi.wrapping_add(0xc8u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 22; continue; } bb = 21; continue; }
            21 => { crate::simd_ops::wu64(rsi.wrapping_add(0xc8u64), rdi); bb = 23; continue; }
            22 => { rax = (crate::simd_ops::ru64(rsi.wrapping_add(0xc0u64))) as u64; crate::simd_ops::wu64(rax, rdi); bb = 23; continue; }
            23 => { crate::simd_ops::wu64(rsi.wrapping_add(0xc0u64), rdi); rcx = r15.wrapping_add(0x18); crate::simd_ops::wu64(rdi.wrapping_add(0x20u64), rsi); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; crate::simd_ops::wu64(rax, r14); rcx = (crate::simd_ops::ru32(rsi.wrapping_add(0xb0u64))) as u32 as u64; rcx = (((rcx as u32)).wrapping_add((rbp as u32))) as u32 as u64; fl_l = (crate::simd_ops::ru8(rsp.wrapping_add(0x80u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; crate::simd_ops::wu32(rsi.wrapping_add(0xb0u64), ((rcx as u32)) as u32); if (((fl_l as u8)) != ((fl_r as u8))) { bb = 25; continue; } bb = 24; continue; }
            24 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rsi.wrapping_add(0xa0u64))) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 26; continue; } bb = 25; continue; }
            25 => { rdx = 0; fl_l = 0; fl_r = 0; rcx = rsi; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); bb = 26; continue; }
            26 => { rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x60u64))) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x70u64))) as u64; crate::simd_ops::wu64(rsi.wrapping_add(0x58u64), rcx); bb = 15; continue; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14167b760(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141675d10; const P1: u64 = 0x14167c430; const P2: u64 = 0x14167bd30; const P3: u64 = 0x141677560; const P4: u64 = 0x14167e020;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbp = 0; fl_l = 0; fl_r = 0; rbx = ((rdx as u32)) as u32 as u64; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(0xd8u64))) as u64; rdi = rcx; rsi = ((rbp as u32)) as u32 as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 23; continue; } bb = 1; continue; }
            1 => { bb = 2; continue; }
            2 => { fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 23; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; r9 = (crate::simd_ops::ru64(rdi.wrapping_add(0xe0u64))) as u64; rcx = (crate::simd_ops::ru32(rax.wrapping_add(0x18u64))) as u32 as u64; rax = r9; rcx = (((rcx as u32)).wrapping_sub(crate::simd_ops::ru32(rdx.wrapping_add(0x10u64)))) as u32 as u64; rsi = (((rsi as u32)).wrapping_add((rcx as u32))) as u32 as u64; rcx = rbp; fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 4; continue; }
            4 => { fl_l = rax; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 7; continue; } bb = 5; continue; }
            5 => { rcx = rax; rax = (crate::simd_ops::ru64(rax)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 4; continue; } bb = 6; continue; }
            6 => { bb = 13; continue; }
            7 => { fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 8; continue; }
            8 => { r8 = (crate::simd_ops::ru64(rax)) as u64; fl_l = rax; fl_r = r9; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 10; continue; } bb = 9; continue; }
            9 => { crate::simd_ops::wu64(rdi.wrapping_add(0xe0u64), r8); bb = 11; continue; }
            10 => { crate::simd_ops::wu64(rcx, r8); bb = 11; continue; }
            11 => { fl_l = rax; fl_r = (crate::simd_ops::ru64(rdi.wrapping_add(0xd8u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 13; continue; } bb = 12; continue; }
            12 => { crate::simd_ops::wu64(rdi.wrapping_add(0xd8u64), rcx); bb = 13; continue; }
            13 => { rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x14u64))) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x7u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 21; continue; } bb = 14; continue; }
            14 => { r10 = (crate::simd_ops::ru32(rdx.wrapping_add(0x10u64))) as u32 as u64; rax = ((((rax as u32)) & (0xfffffffau32))) as u32 as u64; rax = ((((rax as u32)) | (0x2u32))) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_add(0x14u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0xa0u64))) as u32 as u64; r9 = (crate::simd_ops::ru64(rax)) as u64; r8 = rax.wrapping_add(0x18); rax = ((r9.wrapping_add(r10)) as u64) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 17; continue; } bb = 15; continue; }
            15 => { rax = (crate::simd_ops::ru32(r8)) as u32 as u64; rax = ((rax).wrapping_add(r9)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 17; continue; } bb = 16; continue; }
            16 => { rcx = (((rcx as u32)).wrapping_sub((r10 as u32))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 18; continue; }
            17 => { rcx = (crate::simd_ops::ru32(r8)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((r10 as u32)) as u32 as u64; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 18; continue; }
            18 => { crate::simd_ops::lk_sub_u32(rdi.wrapping_add(0xa8u64), ((rcx as u32)) as u32); rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; rcx = (crate::simd_ops::ru32(rax.wrapping_add(0x18u64))) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_add(0x10u64), ((rcx as u32)) as u32); rax = (crate::simd_ops::ru64(rdi.wrapping_add(0xf0u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 20; continue; } bb = 19; continue; }
            19 => { crate::simd_ops::wu64(rdi.wrapping_add(0xf0u64), rdx); crate::simd_ops::wu64(rdx, rbp); bb = 22; continue; }
            20 => { crate::simd_ops::wu64(rdx, rax); crate::simd_ops::wu64(rdi.wrapping_add(0xf0u64), rdx); bb = 22; continue; }
            21 => { r8 = 0; fl_l = 0; fl_r = 0; rcx = rdi; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x60u64))) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 22; continue; }
            22 => { rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0xd8u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 23; continue; }
            23 => { rcx = rdi; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rbx = (((rbx as u32)).wrapping_sub((rsi as u32))) as u32 as u64; rcx = rdi; rdx = ((rbx as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rdi.wrapping_add(0xf0u64))) as u64; rsi = (((rsi as u32)).wrapping_add((rax as u32))) as u32 as u64; fl_l = (crate::simd_ops::ru64(rdi.wrapping_add(0xe0u64))) as u64; fl_r = rbp; r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); crate::simd_ops::wu8(rsp.wrapping_add(0x38u64), (if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8); fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 25; continue; } bb = 24; continue; }
            24 => { rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x60u64))) as u64; r9 = rsp.wrapping_add(0x38); rcx = rbx; rbx = (crate::simd_ops::ru64(rbx)) as u64; rdx = (crate::simd_ops::ru64(rdx.wrapping_add(0x428u64))) as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru8(rsp.wrapping_add(0x38u64))) as u8 as u64; fl_r = ((rbp as u8)) as u8 as u64; r8 = (r8 & !0xffu64) | (((if (((fl_l as u8)) == ((fl_r as u8))) { 1u8 } else { 0u8 }) as u8 as u64)); fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 24; continue; } bb = 25; continue; }
            25 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = ((rsi as u32)) as u32 as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14167b910(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdi); rax = (crate::simd_ops::ru64(rcx.wrapping_add(0xc8u64))) as u64; r9 = 0; fl_l = 0; fl_r = 0; r8 = ((crate::simd_ops::ru8(rcx.wrapping_add(0xd0u64))) as u32) as u32 as u64; rdi = rcx; rbx = ((rdx as u32)) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(0xa0u64), ((rbx as u32)) as u32); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 1; continue; }
            1 => { bb = 2; continue; }
            2 => { fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 6; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rax)) as u64; r8 = (((r8 as u32)).wrapping_sub(1)) as u32 as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 4; continue; }
            4 => { bb = 11; continue; }
            5 => { bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; r10 = (crate::simd_ops::ru32(rax.wrapping_add(0x10u64))) as u32 as u64; r8 = (crate::simd_ops::ru64(rcx)) as u64; rdx = rcx.wrapping_add(0x18); rcx = ((r8.wrapping_add(r10)) as u64) as u64; fl_l = rcx; fl_r = rbx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 9; continue; } bb = 7; continue; }
            7 => { rcx = (crate::simd_ops::ru32(rdx)) as u32 as u64; rcx = ((rcx).wrapping_add(r8)) as u64; fl_l = rcx; fl_r = rbx; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 9; continue; } bb = 8; continue; }
            8 => { rcx = ((rbx as u32)) as u32 as u64; rcx = (((rcx as u32)).wrapping_sub((r10 as u32))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 10; continue; }
            9 => { rcx = (crate::simd_ops::ru32(rdx)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((r10 as u32)) as u32 as u64; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 10; continue; }
            10 => { rax = (crate::simd_ops::ru64(rax)) as u64; r9 = (((r9 as u32)).wrapping_add((rcx as u32))) as u32 as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 6; continue; } bb = 11; continue; }
            11 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(0xe0u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 18; continue; } bb = 12; continue; }
            12 => { bb = 13; continue; }
            13 => { rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; r10 = (crate::simd_ops::ru32(rax.wrapping_add(0x10u64))) as u32 as u64; r8 = (crate::simd_ops::ru64(rcx)) as u64; rdx = rcx.wrapping_add(0x18); rcx = ((r8.wrapping_add(r10)) as u64) as u64; fl_l = rcx; fl_r = rbx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 16; continue; } bb = 14; continue; }
            14 => { rcx = (crate::simd_ops::ru32(rdx)) as u32 as u64; rcx = ((rcx).wrapping_add(r8)) as u64; fl_l = rcx; fl_r = rbx; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 16; continue; } bb = 15; continue; }
            15 => { rcx = ((rbx as u32)) as u32 as u64; rcx = (((rcx as u32)).wrapping_sub((r10 as u32))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 17; continue; }
            16 => { rcx = (crate::simd_ops::ru32(rdx)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((r10 as u32)) as u32 as u64; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 17; continue; }
            17 => { rax = (crate::simd_ops::ru64(rax)) as u64; r9 = (((r9 as u32)).wrapping_add((rcx as u32))) as u32 as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 13; continue; } bb = 18; continue; }
            18 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu32(rdi.wrapping_add(0xa8u64), ((r9 as u32)) as u32); rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x10u64))) as u64; return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14167ba00(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1426452b0; const P1: u64 = 0x14167b170; const P2: u64 = 0x141675d10; const P3: u64 = 0x14167bd30; const P4: u64 = 0x141678a70; const P5: u64 = 0x14167c430; const P6: u64 = 0x1426452a8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = rcx; r12 = (((r9 as u8)) as u32) as u32 as u64; rcx = ((rcx).wrapping_add(0x30u64)) as u64; r15 = ((r8 as u32)) as u32 as u64; rdi = rdx; r14 = (r14 & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 1; continue; }
            1 => { fl_l = ((r15 as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 5; continue; } bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x14u64))) as u32 as u64; rax = ((rcx as u32)) as u32 as u64; rax = (rax & !0xffu64) | ((((((rax as u8)) & (0x7u8))) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x2u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 5; continue; } bb = 3; continue; }
            3 => { rbp = (((r15 as u8)) as u32) as u32 as u64; fl_l = (crate::simd_ops::ru64(rsi.wrapping_add(0xe0u64))) as u64; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 6; continue; } bb = 4; continue; }
            4 => { rax = ((rcx as u32)) as u32 as u64; rcx = ((((rcx as u32)) & (0xfffffffbu32))) as u32 as u64; rax = ((((rax as u32)) & (0xfffffff9u32))) as u32 as u64; rcx = ((((rcx as u32)) | (0x3u32))) as u32 as u64; rax = ((((rax as u32)) | ((r15 as u32)))) as u32 as u64; fl_l = ((r12 as u8)) as u8 as u64; fl_r = ((r12 as u8)) as u8 as u64; rcx = (if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { (rax as u32) } else { (rcx as u32) }) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x14u64), ((rcx as u32)) as u32); bb = 12; continue; }
            5 => { rbp = (rbp & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 6; continue; }
            6 => { fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 7; continue; }
            7 => { r8 = (((rbp as u8)) as u32) as u32 as u64; rdx = rdi; rcx = rsi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); fl_l = ((r12 as u8)) as u8 as u64; fl_r = ((r12 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 10; continue; } bb = 8; continue; }
            8 => { rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x14u64))) as u32 as u64; rax = ((rcx as u32)) as u32 as u64; rax = (rax & !0xffu64) | ((((((rax as u8)) & (0x7u8))) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x2u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 10; continue; } bb = 9; continue; }
            9 => { rcx = ((((rcx as u32)) & (0xfffffffbu32))) as u32 as u64; fl_l = ((((rcx as u32)) | (0x3u32))) as u32 as u64; fl_r = fl_l; rcx = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x14u64), ((rcx as u32)) as u32); bb = 10; continue; }
            10 => { r8 = (((rbp as u8)) as u32) as u32 as u64; rdx = rdi; rcx = rsi; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = rsi; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); r14 = (r14 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 11; continue; }
            11 => { rdx = ((r15 as u32)) as u32 as u64; rcx = rsi; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x60u64))) as u64; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); bb = 12; continue; }
            12 => { rcx = rsi.wrapping_add(0x30); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rax = (((r14 as u8)) as u32) as u32 as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14167bb00(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1426452b0; const P1: u64 = 0x14167b240; const P2: u64 = 0x141675dd0; const P3: u64 = 0x14167be10; const P4: u64 = 0x141678ac0; const P5: u64 = 0x14167c430; const P6: u64 = 0x1426452a8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = rcx; r12 = (((r9 as u8)) as u32) as u32 as u64; rcx = ((rcx).wrapping_add(0x30u64)) as u64; r15 = ((r8 as u32)) as u32 as u64; rdi = rdx; r14 = (r14 & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 1; continue; }
            1 => { fl_l = ((r15 as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 5; continue; } bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x14u64))) as u32 as u64; rax = ((rcx as u32)) as u32 as u64; rax = (rax & !0xffu64) | ((((((rax as u8)) & (0x7u8))) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x2u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 5; continue; } bb = 3; continue; }
            3 => { rbp = (((r15 as u8)) as u32) as u32 as u64; fl_l = (crate::simd_ops::ru64(rsi.wrapping_add(0xc8u64))) as u64; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 6; continue; } bb = 4; continue; }
            4 => { rax = ((rcx as u32)) as u32 as u64; rcx = ((((rcx as u32)) & (0xfffffffbu32))) as u32 as u64; rax = ((((rax as u32)) & (0xfffffff9u32))) as u32 as u64; rcx = ((((rcx as u32)) | (0x3u32))) as u32 as u64; rax = ((((rax as u32)) | ((r15 as u32)))) as u32 as u64; fl_l = ((r12 as u8)) as u8 as u64; fl_r = ((r12 as u8)) as u8 as u64; rcx = (if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { (rax as u32) } else { (rcx as u32) }) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x14u64), ((rcx as u32)) as u32); bb = 12; continue; }
            5 => { rbp = (rbp & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 6; continue; }
            6 => { fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 7; continue; }
            7 => { r8 = (((rbp as u8)) as u32) as u32 as u64; rdx = rdi; rcx = rsi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); fl_l = ((r12 as u8)) as u8 as u64; fl_r = ((r12 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 10; continue; } bb = 8; continue; }
            8 => { rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x14u64))) as u32 as u64; rax = ((rcx as u32)) as u32 as u64; rax = (rax & !0xffu64) | ((((((rax as u8)) & (0x7u8))) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x2u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 10; continue; } bb = 9; continue; }
            9 => { rcx = ((((rcx as u32)) & (0xfffffffbu32))) as u32 as u64; fl_l = ((((rcx as u32)) | (0x3u32))) as u32 as u64; fl_r = fl_l; rcx = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x14u64), ((rcx as u32)) as u32); bb = 10; continue; }
            10 => { r8 = (((rbp as u8)) as u32) as u32 as u64; rdx = rdi; rcx = rsi; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = rsi; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); r14 = (r14 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 11; continue; }
            11 => { rdx = ((r15 as u32)) as u32 as u64; rcx = rsi; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x60u64))) as u64; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); bb = 12; continue; }
            12 => { rcx = rsi.wrapping_add(0x30); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rax = (((r14 as u8)) as u32) as u32 as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14167bc00(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1426452b0; const P1: u64 = 0x14167bef0; const P2: u64 = 0x14167b240; const P3: u64 = 0x141675dd0; const P4: u64 = 0x14167be10; const P5: u64 = 0x141678ac0; const P6: u64 = 0x14167c430; const P7: u64 = 0x1426452a8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = rcx; r12 = (((r9 as u8)) as u32) as u32 as u64; rcx = ((rcx).wrapping_add(0x30u64)) as u64; r14 = ((r8 as u32)) as u32 as u64; rdi = rdx; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = ((r14 as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = rsi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            2 => { rcx = rsi.wrapping_add(0x30); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rbp); r15 = (r15 & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 3; continue; }
            3 => { fl_l = ((r14 as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 7; continue; } bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x14u64))) as u32 as u64; rax = ((rcx as u32)) as u32 as u64; rax = (rax & !0xffu64) | ((((((rax as u8)) & (0x7u8))) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x2u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 7; continue; } bb = 5; continue; }
            5 => { rbp = (((r14 as u8)) as u32) as u32 as u64; fl_l = (crate::simd_ops::ru64(rsi.wrapping_add(0xc8u64))) as u64; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 8; continue; } bb = 6; continue; }
            6 => { rax = ((rcx as u32)) as u32 as u64; rcx = ((((rcx as u32)) & (0xfffffffbu32))) as u32 as u64; rax = ((((rax as u32)) & (0xfffffff9u32))) as u32 as u64; rcx = ((((rcx as u32)) | (0x3u32))) as u32 as u64; rax = ((((rax as u32)) | ((r14 as u32)))) as u32 as u64; fl_l = ((r12 as u8)) as u8 as u64; fl_r = ((r12 as u8)) as u8 as u64; rcx = (if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { (rax as u32) } else { (rcx as u32) }) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x14u64), ((rcx as u32)) as u32); bb = 14; continue; }
            7 => { rbp = (rbp & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 8; continue; }
            8 => { fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 9; continue; }
            9 => { r8 = (((rbp as u8)) as u32) as u32 as u64; rdx = rdi; rcx = rsi; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); fl_l = ((r12 as u8)) as u8 as u64; fl_r = ((r12 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 12; continue; } bb = 10; continue; }
            10 => { rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x14u64))) as u32 as u64; rax = ((rcx as u32)) as u32 as u64; rax = (rax & !0xffu64) | ((((((rax as u8)) & (0x7u8))) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x2u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 12; continue; } bb = 11; continue; }
            11 => { rcx = ((((rcx as u32)) & (0xfffffffbu32))) as u32 as u64; fl_l = ((((rcx as u32)) | (0x3u32))) as u32 as u64; fl_r = fl_l; rcx = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x14u64), ((rcx as u32)) as u32); bb = 12; continue; }
            12 => { r8 = (((rbp as u8)) as u32) as u32 as u64; rdx = rdi; rcx = rsi; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = rsi; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); r15 = (r15 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 13; continue; }
            13 => { rdx = ((r14 as u32)) as u32 as u64; rcx = rsi; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x60u64))) as u64; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); bb = 14; continue; }
            14 => { rcx = rsi.wrapping_add(0x30); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P7), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rsi.wrapping_add(0x30); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P7), rax), rax, rcx, rdx, r8, r9, rsp); rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rax = (((r15 as u8)) as u32) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14167bef0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141675dd0; const P1: u64 = 0x14167c430; const P2: u64 = 0x14167be10; const P3: u64 = 0x14167e020;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = (crate::simd_ops::ru64(rcx.wrapping_add(0xc8u64))) as u64; rsi = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu8(rsp.wrapping_add(0x30u64), (0x1u8) as u8); rdi = rcx; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 1; continue; }
            1 => { bb = 2; continue; }
            2 => { rdx = rbx; rax = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rbx; fl_r = (crate::simd_ops::ru64(rdi.wrapping_add(0xc8u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu64(rdi.wrapping_add(0xc8u64), rax); bb = 5; continue; }
            4 => { crate::simd_ops::wu64(rsi, rax); bb = 5; continue; }
            5 => { fl_l = rbx; fl_r = (crate::simd_ops::ru64(rdi.wrapping_add(0xc0u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 7; continue; } bb = 6; continue; }
            6 => { crate::simd_ops::wu64(rdi.wrapping_add(0xc0u64), rsi); bb = 7; continue; }
            7 => { rbx = rax; rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x14u64))) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x7u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 11; continue; } bb = 8; continue; }
            8 => { rax = ((((rax as u32)) & (0xfffffffau32))) as u32 as u64; rax = ((((rax as u32)) | (0x2u32))) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_add(0x14u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rdi.wrapping_add(0xd8u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { crate::simd_ops::wu64(rdi.wrapping_add(0xd8u64), rdx); crate::simd_ops::wu64(rdx, rsi); bb = 12; continue; }
            10 => { crate::simd_ops::wu64(rdx, rax); crate::simd_ops::wu64(rdi.wrapping_add(0xd8u64), rdx); bb = 12; continue; }
            11 => { r8 = 0; fl_l = 0; fl_r = 0; rcx = rdi; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x60u64))) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 12; continue; }
            12 => { fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 13; continue; }
            13 => { rcx = rdi; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rdi.wrapping_add(0xd8u64))) as u64; r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 16; continue; } bb = 14; continue; }
            14 => { bb = 15; continue; }
            15 => { rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x60u64))) as u64; r9 = rsp.wrapping_add(0x30); rcx = rbx; rbx = (crate::simd_ops::ru64(rbx)) as u64; rdx = (crate::simd_ops::ru64(rdx.wrapping_add(0x428u64))) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru8(rsp.wrapping_add(0x30u64))) as u8 as u64; fl_r = ((rsi as u8)) as u8 as u64; r8 = (r8 & !0xffu64) | (((if (((fl_l as u8)) == ((fl_r as u8))) { 1u8 } else { 0u8 }) as u8 as u64)); fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 15; continue; } bb = 16; continue; }
            16 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14167c000(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141524330; const P1: u64 = 0x141674a70; const P2: u64 = 0x141675750; const P3: u64 = 0x142694508; const P4: u64 = 0x142694478; const P5: u64 = 0x141676f10; const P6: u64 = 0x1415242e0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x88u64))) as u64; r12 = 0; fl_l = 0; fl_r = 0; r15 = rdx; rsi = rcx; rdx = 0xf8; rbp = r9; r14 = ((r8 as u32)) as u32 as u64; rcx = ((r12.wrapping_add(0x7u64)) as u32) as u32 as u64; crate::simd_ops::wu64(rdi, r12); rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rbx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rdx = ((crate::simd_ops::ru8(rbp.wrapping_add(0xdu64))) as u32) as u32 as u64; rcx = rsi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rdx = 0xf8; rcx = ((rbx.wrapping_add(0x7u64)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rbx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 2; continue; }
            2 => { rcx = rbx; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rax = P3; r9 = ((r14 as u32)) as u32 as u64; crate::simd_ops::wu64(rbx, rax); r8 = r15; rax = P4; rdx = rsi; crate::simd_ops::wu64(rbx.wrapping_add(0x80u64), rax); rcx = rbx; crate::simd_ops::wu64(rbx.wrapping_add(0xf0u64), r12); crate::simd_ops::wu64(rbx.wrapping_add(0xe0u64), r12); crate::simd_ops::wu64(rbx.wrapping_add(0xd8u64), r12); rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x430u64))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbp); rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { fl_l = rbx; fl_r = rbx; rcx = rbx.wrapping_add(0x80); rax = rbx; rcx = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { r12 } else { rcx }) as u64; crate::simd_ops::wu64(rdi, rcx); bb = 6; continue; }
            4 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rdx = rbx; rcx = 0x7; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            5 => { crate::simd_ops::wu64(rdi, r12); rax = 0; fl_l = 0; fl_r = 0; bb = 6; continue; }
            6 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14167c130(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141524330; const P1: u64 = 0x141674a70; const P2: u64 = 0x1416758c0; const P3: u64 = 0x142694310; const P4: u64 = 0x142694378; const P5: u64 = 0x1416770f0; const P6: u64 = 0x1415242e0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; r15 = 0; fl_l = 0; fl_r = 0; r14 = rdx; rsi = rcx; crate::simd_ops::wu64(r9, r15); rdx = 0xe0; rdi = r9; rbp = ((r8 as u32)) as u32 as u64; rcx = ((r15.wrapping_add(0x7u64)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rbx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rdx = (rdx & !0xffu64) | (((0x64u8) as u8 as u64)); rcx = rsi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rdx = 0xe0; rcx = ((rbx.wrapping_add(0x7u64)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rbx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 2; continue; }
            2 => { rcx = rbx; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rax = P3; r9 = ((rbp as u32)) as u32 as u64; crate::simd_ops::wu64(rbx, rax); r8 = r14; rax = P4; rdx = rsi; crate::simd_ops::wu64(rbx.wrapping_add(0x80u64), rax); rcx = rbx; crate::simd_ops::wu64(rbx.wrapping_add(0xd8u64), r15); crate::simd_ops::wu64(rbx.wrapping_add(0xc8u64), r15); crate::simd_ops::wu64(rbx.wrapping_add(0xc0u64), r15); rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { fl_l = rbx; fl_r = rbx; rcx = rbx.wrapping_add(0x80); rax = rbx; rcx = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { r15 } else { rcx }) as u64; crate::simd_ops::wu64(rdi, rcx); bb = 6; continue; }
            4 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rdx = rbx; rcx = 0x7; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            5 => { rax = 0; fl_l = 0; fl_r = 0; bb = 6; continue; }
            6 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14167c230(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x142694598; const P1: u64 = 0x8; const P2: u64 = 0x10;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = P0;
    crate::simd_ops::wu64(rcx.wrapping_add(P1), (0x0u64) as u64);
    crate::simd_ops::wu64(rcx, rax);
    rax = rcx;
    crate::simd_ops::wu8(rcx.wrapping_add(P2), (0x0u8) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14167c250(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1426945a0; const P1: u64 = 0x10; const P2: u64 = 0x1426452b8; const P3: u64 = 0x40; const P4: u64 = 0x48; const P5: u64 = 0x50; const P6: u64 = 0x58; const P7: u64 = 0x60; const P8: u64 = 0x68; const P9: u64 = 0x6c;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rax = P0;
    rdi = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rcx, rax);
    rbx = rcx;
    crate::simd_ops::wu64(rcx.wrapping_add(P1), rdi);
    rcx = ((rcx).wrapping_add(0x18u64)) as u64;
    rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp);
    rax = rbx;
    crate::simd_ops::wu64(rbx.wrapping_add(P3), rdi);
    crate::simd_ops::wu64(rbx.wrapping_add(P4), rdi);
    crate::simd_ops::wu64(rbx.wrapping_add(P5), rdi);
    crate::simd_ops::wu64(rbx.wrapping_add(P6), rdi);
    crate::simd_ops::wu64(rbx.wrapping_add(P7), rdi);
    crate::simd_ops::wu8(rbx.wrapping_add(P8), ((rdi as u8)) as u8);
    crate::simd_ops::wu32(rbx.wrapping_add(P9), ((rdi as u32)) as u32);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14167c2a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142694598; const P1: u64 = 0x8; const P2: u64 = 0x1426455b0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rax = P0;
    rbx = rcx;
    crate::simd_ops::wu64(rcx, rax);
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64;
    fl_l = rcx; fl_r = rcx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(P1), (0x0u64) as u64); }
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14167c2d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1426945a0; const P1: u64 = 0x1426452a0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = P0;
    crate::simd_ops::wu64(rcx, rax);
    fl_l = ((rcx).wrapping_add(0x18u64)) as u64; fl_r = fl_l;
    rcx = ((fl_l as u64)) as u64;
    return crate::simd_ops::call_fwd(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14167c2f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142694598; const P1: u64 = 0x8; const P2: u64 = 0x1426455b0; const P3: u64 = 0x141834d30;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rax = P0;
    rbx = rcx;
    crate::simd_ops::wu64(rcx, rax);
    rdi = ((rdx as u32)) as u32 as u64;
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64;
    fl_l = rcx; fl_r = rcx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(P1), (0x0u64) as u64); }
    fl_l = ((rdi as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64;
    if !(((((fl_l as u8)) & ((fl_r as u8))) == 0)) { rdx = 0x18; rcx = rbx; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); }
    rax = rbx;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14167c350(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1426945a0; const P1: u64 = 0x1426452a0; const P2: u64 = 0x141834d30;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rax = P0;
    rdi = rcx;
    crate::simd_ops::wu64(rcx, rax);
    rbx = ((rdx as u32)) as u32 as u64;
    rcx = ((rcx).wrapping_add(0x18u64)) as u64;
    rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp);
    fl_l = ((rbx as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64;
    if !(((((fl_l as u8)) & ((fl_r as u8))) == 0)) { rdx = 0x70; rcx = rdi; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rax = rdi;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14167c3e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1426452b0; const P1: u64 = 0x64; const P2: u64 = 0x68; const P3: u64 = 0x58; const P4: u64 = 0x142645560; const P5: u64 = 0x18; const P6: u64 = 0x1426452a8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdi = rcx;
    rcx = ((rcx).wrapping_add(0x18u64)) as u64;
    rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp);
    crate::simd_ops::lk_add_u32(rdi.wrapping_add(P1), (1) as u32);
    fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(P1))) as u32 as u64; fl_r = 0x1;
    if (((fl_l as u32)) != ((fl_r as u32))) { rcx = ((rdi.wrapping_add(P5)) as u64) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp); }
    fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(P2))) as u8 as u64; fl_r = (0x0u8) as u8 as u64;
    if !((((fl_l as u8)) != ((fl_r as u8)))) { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P3))) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); }
    rcx = ((rdi.wrapping_add(P5)) as u64) as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14167c430(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1426452b0; const P1: u64 = 0x8; const P2: u64 = 0x6c; const P3: u64 = 0x48; const P4: u64 = 0x142645560; const P5: u64 = 0x18; const P6: u64 = 0x1426452a8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdi = rcx;
    rcx = ((rcx).wrapping_add(0x18u64)) as u64;
    rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp);
    rax = (crate::simd_ops::ru32(rdi.wrapping_add(P1))) as u32 as u64;
    crate::simd_ops::lk_sub_u32(rdi.wrapping_add(P2), (1) as u32);
    rax = (((rax as u32)).wrapping_sub(1)) as u32 as u64;
    fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(P2))) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64;
    if !((((fl_l as u32)) != ((fl_r as u32)))) { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P3))) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); }
    rcx = ((rdi.wrapping_add(P5)) as u64) as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp);
}
