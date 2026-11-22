#![allow(unused_imports, unused_variables, unused_mut, unused_assignments, non_snake_case, dead_code, clippy::all, unreachable_code, unused_parens)]

#[inline(never)]
pub unsafe extern "win64" fn fn_141621110(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415fab80; const P1: u64 = 0x141620b90; const P2: u64 = 0x141620d10; const P3: u64 = 0x141620d50; const P4: u64 = 0x141621340;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = (crate::simd_ops::ru64(rdx.wrapping_add(0x18u64))) as u64; r15 = r9; rbp = r8; r14 = rdx; rdi = rcx; rbx = (crate::simd_ops::ru32(rsi.wrapping_add(0x8u64))) as u32 as u64; fl_l = ((rbx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rsi.wrapping_add(0xcu64))) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 3; continue; } bb = 1; continue; }
            1 => { rcx = rsi; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 20; continue; } bb = 2; continue; }
            2 => { fl_l = ((rbx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rsi.wrapping_add(0xcu64))) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 20; continue; } bb = 3; continue; }
            3 => { rdx = (crate::simd_ops::ru32(rsi.wrapping_add(0x8u64))) as u32 as u64; rax = (crate::simd_ops::ru64(rsi)) as u64; rcx = ((rdx.wrapping_add(rdx.wrapping_mul(4))) as u64) as u64; rbx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; rax = ((rdx.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rsi.wrapping_add(0x8u64), ((rax as u32)) as u32); fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 20; continue; } bb = 4; continue; }
            4 => { rcx = rbx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 20; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x88u64))) as u64; fl_l = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 8; continue; } bb = 6; continue; }
            6 => { rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x88u64))) as u64; rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x18u64))) as u64; crate::simd_ops::wu64(rdx.wrapping_add(0x18u64), rax); fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 8; continue; }
            8 => { rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x88u64))) as u64; fl_l = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 20; continue; } bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru64(r15)) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x20u64), rax); fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { fl_l = (crate::simd_ops::lk_add_u32(rdi.wrapping_add(0x24u64), (1) as u32)) as u32 as u64; bb = 11; continue; }
            11 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x18u64), rdi); fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 12; continue; }
            12 => { rcx = ((rcx).wrapping_add(0x10u64)) as u64; rax = 0xffffffff; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rcx.wrapping_add(0x14u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 14; continue; } bb = 13; continue; }
            13 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 14; continue; }
            14 => { rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x88u64))) as u64; rsi = 0x1; rax = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; fl_l = (crate::simd_ops::ru32(rax.wrapping_add(0x8u64))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 16; continue; } bb = 15; continue; }
            15 => { rax = (crate::simd_ops::ru64(rax)) as u64; rcx = (crate::simd_ops::ru32(rax.wrapping_add(0x10u64))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x10u64), ((rcx as u32)) as u32); bb = 17; continue; }
            16 => { fl_l = (crate::simd_ops::lk_and_u8(rbx.wrapping_add(0x12u64), (0xfcu8) as u8)) as u8 as u64; fl_r = fl_l; crate::simd_ops::wu16(rbx.wrapping_add(0x10u64), ((rsi as u16)) as u16); bb = 17; continue; }
            17 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x20u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 19; continue; } bb = 18; continue; }
            18 => { rcx = rdi; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 19; continue; }
            19 => { rax = ((rsi as u32)) as u32 as u64; bb = 23; continue; }
            20 => { rcx = (crate::simd_ops::ru64(r14.wrapping_add(0x18u64))) as u64; crate::simd_ops::wu64(r14.wrapping_add(0x18u64), (0x0u64) as u64); fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 22; continue; } bb = 21; continue; }
            21 => { rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 22; continue; }
            22 => { rax = 0x34; bb = 23; continue; }
            23 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141621270(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415fa0e0; const P1: u64 = 0x8; const P2: u64 = 0x278; const P3: u64 = 0x141620d00; const P4: u64 = 0x141620d50; const P5: u64 = 0x110; const P6: u64 = 0x108; const P7: u64 = 0x10; const P8: u64 = 0x14183acc8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = rcx; r14 = r8; rcx = ((rcx).wrapping_add(0x108u64)) as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rbx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru32(rax.wrapping_add(P1))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 10; continue; } bb = 2; continue; }
            2 => { rdx = (crate::simd_ops::ru64(rdi)) as u64; rcx = rdi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rdx.wrapping_add(P2))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 7; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rsi); rsi = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rcx = rsi; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            5 => { rcx = (crate::simd_ops::ru64(r14)) as u64; crate::simd_ops::wu64(r14, rsi); rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 7; continue; }
            7 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru32(rdi.wrapping_add(P5))) as u32 as u64; rcx = (((rcx as u32)).wrapping_sub(1)) as u32 as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rdi.wrapping_add(P6)))) as u64; fl_l = rbx; fl_r = rcx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 9; continue; } bb = 8; continue; }
            8 => { rcx = ((rcx).wrapping_sub(rbx)) as u64; rdx = ((rbx.wrapping_add(P7)) as u64) as u64; rcx = ((((rcx as u32)) & (0xfffffff0u32))) as u32 as u64; r8 = ((rcx as u32)) as u32 as u64; rcx = rbx; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); bb = 9; continue; }
            9 => { fl_l = (crate::simd_ops::lk_sub_u32(rdi.wrapping_add(P5), (1) as u32)) as u32 as u64; bb = 11; continue; }
            10 => { fl_l = (((rax as u32)).wrapping_sub(1)) as u32 as u64; rax = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(P1), ((rax as u32)) as u32); bb = 11; continue; }
            11 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141621340(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141621850; const P1: u64 = 0x2; const P2: u64 = 0x8; const P3: u64 = 0x141620d00;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rcx = ((rcx).wrapping_add(0x108u64)) as u64;
    rdi = rdx;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rbx = rax;
    fl_l = rax; fl_r = rax;
    if !(((((fl_l as u64)) & ((fl_r as u64))) != 0)) { rax = ((rbx.wrapping_add(P1)) as u32) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    fl_l = (crate::simd_ops::ru32(rax.wrapping_add(P2))) as u32 as u64; fl_r = 0x0;
    if !((((fl_l as u32)) != ((fl_r as u32)))) { rcx = rdi; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); }
    crate::simd_ops::lk_add_u32(rbx.wrapping_add(P2), (1) as u32);
    rax = 0x1;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141621390(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415fa0e0; const P1: u64 = 0x8; const P2: u64 = 0x141620d50; const P3: u64 = 0x110; const P4: u64 = 0x108; const P5: u64 = 0x10; const P6: u64 = 0x14183acc8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = rcx; rcx = ((rcx).wrapping_add(0x108u64)) as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rbx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru32(rax.wrapping_add(P1))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 5; continue; } bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rdi.wrapping_add(P3))) as u32 as u64; rax = (((rax as u32)).wrapping_sub(1)) as u32 as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(rdi.wrapping_add(P4)))) as u64; fl_l = rbx; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = ((rax).wrapping_sub(rbx)) as u64; rdx = ((rbx.wrapping_add(P5)) as u64) as u64; rax = ((((rax as u32)) & (0xfffffff0u32))) as u32 as u64; rcx = rbx; r8 = ((rax as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::lk_sub_u32(rdi.wrapping_add(P3), (1) as u32)) as u32 as u64; bb = 6; continue; }
            5 => { fl_l = (((rax as u32)).wrapping_sub(1)) as u32 as u64; rax = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(P1), ((rax as u32)) as u32); bb = 6; continue; }
            6 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = 0x1; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141621420(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141524340; const P1: u64 = 0x141524330; const P2: u64 = 0x1415242e0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdx = (crate::simd_ops::ru32(rcx.wrapping_add(0xcu64))) as u32 as u64; rdi = rcx; rbp = ((rdx as u32)) as u32 as u64; rcx = 0x1; rbp = (((rbp as u32)).wrapping_shr(1u32)) as u32 as u64; rbp = (((rbp as u32)).wrapping_add((rdx as u32))) as u32 as u64; rax = (crate::simd_ops::ru64(rdi)) as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; rsi = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; rbp = (if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { (rcx as u32) } else { (rbp as u32) }) as u32 as u64; rcx = 0; fl_l = 0; fl_r = 0; rbp = (((rbp as u32)).wrapping_add((rdx as u32))) as u32 as u64; rdx = ((rbp as u32)) as u32 as u64; rdx = ((rdx).wrapping_shl(4u32)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { r8 = rdx; rdx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 12; continue; } bb = 2; continue; }
            2 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 13; continue; }
            3 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rdi)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 5; continue; }
            5 => { fl_l = rax; fl_r = rbx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 12; continue; } bb = 6; continue; }
            6 => { fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 7; continue; }
            7 => { rax = rbx; r8 = 0; fl_l = 0; fl_r = 0; bb = 8; continue; }
            8 => { fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { crate::simd_ops::wu32(rax.wrapping_add(0x8u64), ((r8 as u32)) as u32); bb = 10; continue; }
            10 => { rcx = (crate::simd_ops::ru64(rdi)) as u64; rdx = rax; rdx = ((rdx).wrapping_sub(rbx)) as u64; xmm0 = (crate::simd_ops::ru128(rdx.wrapping_add(rcx))) as u128; crate::simd_ops::wu128(rax, (xmm0) as u128); rax = ((rax).wrapping_add(0x10u64)) as u64; fl_l = rsi; fl_r = (0x1u64) as u64; rsi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 8; continue; } bb = 11; continue; }
            11 => { rdx = (crate::simd_ops::ru64(rdi)) as u64; rcx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 12; continue; }
            12 => { crate::simd_ops::wu64(rdi, rbx); rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); crate::simd_ops::wu32(rdi.wrapping_add(0xcu64), ((rbp as u32)) as u32); bb = 13; continue; }
            13 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416214f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x70; const P1: u64 = 0xd4;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rbx = rcx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0xa; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rcx = (rcx & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 4; continue; }
            3 => { rcx = (rcx & !0xffu64) | (((0x80u8) as u8 as u64)); bb = 4; continue; }
            4 => { rax = ((crate::simd_ops::ru8(rbx.wrapping_add(P1))) as u32) as u32 as u64; rax = (rax & !0xffu64) | ((((((rax as u8)) & (0x7fu8))) as u8 as u64)); rax = (rax & !0xffu64) | ((((((rax as u8)) | ((rcx as u8)))) as u8 as u64)); crate::simd_ops::wu8(rbx.wrapping_add(P1), ((rax as u8)) as u8); rax = 0x1; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141621530(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14158f5a0; const P1: u64 = 0x1415f9ee0; const P2: u64 = 0x143930fa8; const P3: u64 = 0x1415f8000; const P4: u64 = 0x1415fa0e0; const P5: u64 = 0x141620d00; const P6: u64 = 0x141620d50; const P7: u64 = 0x14183acc8; const P8: u64 = 0x1415c11f0; const P9: u64 = 0x1415f8290; const P10: u64 = 0x1415c2070; const P11: u64 = 0x14152ede0; const P12: u64 = 0x1415864b0;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 111>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rbp = rsp.wrapping_sub(0x260); rsp = ((rsp).wrapping_sub(0x360u64)) as u64; rax = (crate::simd_ops::ru32(rdx)) as u32 as u64; rdi = rdx; crate::simd_ops::wu32(rsp.wrapping_add(0x60u64), ((rax as u32)) as u32); rsi = rcx; rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; r14 = 0x2; crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), rax); rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x10u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), rax); rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x18u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), rax); rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x34u64))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_sub(0x6cu64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x28u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x78u64), rax); rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x30u64))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_sub(0x70u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x20u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::lk_add_u32(rax, (1) as u32)) as u32 as u64; bb = 2; continue; }
            2 => { xmm0 = (crate::simd_ops::ru128(rdx.wrapping_add(0x38u64))) as u128; crate::simd_ops::wu64(rbp.wrapping_sub(0x80u64), rax); rcx = rbp.wrapping_add(0x10); rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x80u64))) as u32 as u64; xmm1 = (crate::simd_ops::ru128(rdx.wrapping_add(0x48u64))) as u128; crate::simd_ops::wu32(rbp.wrapping_sub(0x20u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x84u64))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_sub(0x1cu64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x88u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x18u64), rax); rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x90u64))) as u32 as u64; crate::simd_ops::wu128(rbp.wrapping_sub(0x68u64), (xmm0) as u128); crate::simd_ops::wu32(rbp.wrapping_sub(0x10u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x94u64))) as u32 as u64; xmm0 = (crate::simd_ops::ru128(rdx.wrapping_add(0x58u64))) as u128; crate::simd_ops::wu32(rbp.wrapping_sub(0xcu64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(0xa0u64))) as u32 as u64; crate::simd_ops::wu128(rbp.wrapping_sub(0x48u64), (xmm0) as u128); crate::simd_ops::wu32(rbp, ((rax as u32)) as u32); xmm0 = ((crate::simd_ops::ru64(rdx.wrapping_add(0x78u64))) as u64) as u128; rax = (crate::simd_ops::ru64(rdx.wrapping_add(0xa8u64))) as u64; crate::simd_ops::wu128(rbp.wrapping_sub(0x58u64), (xmm1) as u128); crate::simd_ops::wu64(rbp.wrapping_add(0x8u64), rax); xmm1 = (crate::simd_ops::ru128(rdx.wrapping_add(0x68u64))) as u128; crate::simd_ops::wu64(rbp.wrapping_sub(0x28u64), ((xmm0 as u64)) as u64); xmm0 = ((crate::simd_ops::ru64(rdx.wrapping_add(0x98u64))) as u64) as u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x38u64), (xmm1) as u128); crate::simd_ops::wu64(rbp.wrapping_sub(0x8u64), ((xmm0 as u64)) as u64); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x2e0u64))) as u64; rdx = rdi.wrapping_add(0xb0); r12 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rbp.wrapping_add(0x240u64), rax); rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x2e8u64))) as u64; rcx = rbp.wrapping_add(0x10); crate::simd_ops::wu64(rbp.wrapping_add(0x248u64), rax); crate::simd_ops::wu64(rbp.wrapping_add(0xc8u64), r12); crate::simd_ops::wu64(rbp.wrapping_add(0xd0u64), r12); crate::simd_ops::wu64(rbp.wrapping_add(0xd8u64), r12); crate::simd_ops::wu64(rbp.wrapping_add(0xe0u64), r12); crate::simd_ops::wu32(rbp.wrapping_add(0xe8u64), ((r12 as u32)) as u32); crate::simd_ops::wu64(rbp.wrapping_add(0x230u64), r12); crate::simd_ops::wu64(rbp.wrapping_add(0x238u64), r12); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = ((crate::simd_ops::ru8(rbp.wrapping_add(0x258u64))) as u32) as u32 as u64; rdx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x18u64))) as u64; rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x2f0u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x250u64), rax); rax = (((rcx as u8)) as u32) as u32 as u64; rax = (rax & !0xffu64) | ((((((rax as u8)) ^ (crate::simd_ops::ru8(rdi.wrapping_add(0x2f8u64))))) as u8 as u64)); rax = (rax & !0xffu64) | ((((((rax as u8)) & (0xfu8))) as u8 as u64)); rcx = (rcx & !0xffu64) | ((((((rcx as u8)) ^ ((rax as u8)))) as u8 as u64)); crate::simd_ops::wu8(rbp.wrapping_add(0x258u64), ((rcx as u8)) as u8); rax = (crate::simd_ops::ru64(rdx)) as u64; rcx = (crate::simd_ops::ru64(P2)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); r8 = (crate::simd_ops::ru64(rdx.wrapping_add(0x10u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), r8); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), r12); rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x20u64))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x40u64), ((rax as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), r12); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), r12); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), r12); fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 3; continue; }
            3 => { fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(r8)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 5; continue; }
            5 => { rdx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x18u64))) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rcx = (crate::simd_ops::ru64(rax)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rcx); bb = 7; continue; }
            7 => { rdx = (crate::simd_ops::ru64(rdx.wrapping_add(0x18u64))) as u64; rcx = rsi.wrapping_add(0x108); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 18; continue; } bb = 8; continue; }
            8 => { rax = (crate::simd_ops::ru32(rax.wrapping_add(0x8u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 17; continue; } bb = 9; continue; }
            9 => { rdx = (crate::simd_ops::ru64(rsi)) as u64; rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rdx.wrapping_add(0x278u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 14; continue; } bb = 10; continue; }
            10 => { rdi = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 11; continue; }
            11 => { rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 12; continue; }
            12 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rdi); fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 13; continue; }
            13 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 14; continue; }
            14 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru32(rsi.wrapping_add(0x110u64))) as u32 as u64; rcx = (((rcx as u32)).wrapping_sub(1)) as u32 as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rsi.wrapping_add(0x108u64)))) as u64; fl_l = rbx; fl_r = rcx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 16; continue; } bb = 15; continue; }
            15 => { rcx = ((rcx).wrapping_sub(rbx)) as u64; rdx = rbx.wrapping_add(0x10); rcx = ((((rcx as u32)) & (0xfffffff0u32))) as u32 as u64; r8 = ((rcx as u32)) as u32 as u64; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 16; continue; }
            16 => { fl_l = (crate::simd_ops::lk_sub_u32(rsi.wrapping_add(0x110u64), (1) as u32)) as u32 as u64; bb = 18; continue; }
            17 => { fl_l = (((rax as u32)).wrapping_sub(1)) as u32 as u64; rax = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x8u64), ((rax as u32)) as u32); bb = 18; continue; }
            18 => { fl_l = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; fl_r = r12; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 20; continue; } bb = 19; continue; }
            19 => { rax = rsp.wrapping_add(0x20); rcx = rsi; rdx = rsp.wrapping_add(0x60); crate::simd_ops::wu64(rbp.wrapping_sub(0x18u64), rax); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r14 = ((rax as u32)) as u32 as u64; bb = 20; continue; }
            20 => { rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 22; continue; } bb = 21; continue; }
            21 => { rcx = (crate::simd_ops::ru64(P2)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 22; continue; }
            22 => { rcx = rsp.wrapping_add(0x48); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P10, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 24; continue; } bb = 23; continue; }
            23 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), r12); bb = 24; continue; }
            24 => { rcx = rbp.wrapping_add(0x10); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P11, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x80u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 26; continue; } bb = 25; continue; }
            25 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P12, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 26; continue; }
            26 => { r11 = rsp.wrapping_add(0x360); rax = ((r14 as u32)) as u32 as u64; rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; rdi = (crate::simd_ops::ru64(r11.wrapping_add(0x30u64))) as u64; r12 = (crate::simd_ops::ru64(r11.wrapping_add(0x38u64))) as u64; rsp = r11; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141621850(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141621420; const P1: u64 = 0x14183acc8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rcx; rdi = 0; fl_l = 0; fl_r = 0; rcx = (crate::simd_ops::ru32(rcx.wrapping_add(0x8u64))) as u32 as u64; rsi = rdx; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { r9 = (crate::simd_ops::ru64(rbx)) as u64; bb = 2; continue; }
            2 => { rdx = ((rcx as u32)) as u32 as u64; rdx = (((rdx as u32)).wrapping_shr(1u32)) as u32 as u64; rdx = (((rdx as u32)).wrapping_add((rdi as u32))) as u32 as u64; rax = ((((rdx as u32)) as i32 as i64) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rax = ((rax).wrapping_add(r9)) as u64; fl_l = (crate::simd_ops::ru64(rax)) as u64; fl_r = rsi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 22; continue; } bb = 3; continue; }
            3 => { rax = ((rdx.wrapping_add(0x1u64)) as u32) as u32 as u64; rax = (if (((fl_l as u64)) >= ((fl_r as u64))) { (rdi as u32) } else { (rax as u32) }) as u32 as u64; rdi = ((rax as u32)) as u32 as u64; rax = ((rcx.wrapping_sub(0x1u64)) as u32) as u32 as u64; rax = (if (((fl_l as u64)) >= ((fl_r as u64))) { (rcx as u32) } else { (rax as u32) }) as u32 as u64; rcx = ((rax as u32)) as u32 as u64; fl_l = (((rcx as u32)).wrapping_shr(1u32)) as u32 as u64; rcx = ((fl_l as u32)) as u32 as u64; if (((fl_l as u32)) != 0) { bb = 2; continue; } bb = 4; continue; }
            4 => { rax = ((((rdi as u32)) as i32 as i64) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; fl_l = ((rax).wrapping_add(crate::simd_ops::ru64(rbx))) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_add(0xcu64))) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rbp); rbp = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; if (((fl_l as u64)) == 0) { bb = 13; continue; } bb = 5; continue; }
            5 => { fl_l = ((rbp as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rcx = rbx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 20; continue; } bb = 7; continue; }
            7 => { fl_l = ((rbp as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rbx.wrapping_add(0xcu64))) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 20; continue; } bb = 8; continue; }
            8 => { rbp = (crate::simd_ops::ru64(rbx)) as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), r14); r14 = ((rdi as u32)) as u32 as u64; r14 = ((r14).wrapping_shl(4u32)) as u64; rbp = ((rbp).wrapping_add(r14)) as u64; fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 10; continue; } bb = 9; continue; }
            9 => { rax = (((rax as u32)).wrapping_sub((rdi as u32))) as u32 as u64; rcx = rbp.wrapping_add(0x10); rax = (((rax as u32)).wrapping_shl(4u32)) as u32 as u64; rdx = rbp; r8 = ((rax as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 10; continue; }
            10 => { fl_l = rbp; fl_r = rbp; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 11; continue; }
            11 => { crate::simd_ops::wu32(rbp.wrapping_add(0x8u64), (0x0u32) as u32); bb = 12; continue; }
            12 => { crate::simd_ops::lk_add_u32(rbx.wrapping_add(0x8u64), (1) as u32); rax = (crate::simd_ops::ru64(rbx)) as u64; fl_l = ((rax).wrapping_add(r14)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; bb = 18; continue; }
            13 => { fl_l = ((rbp as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 15; continue; } bb = 14; continue; }
            14 => { rcx = rbx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 20; continue; } bb = 15; continue; }
            15 => { fl_l = ((rbp as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rbx.wrapping_add(0xcu64))) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 20; continue; } bb = 16; continue; }
            16 => { rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; rax = ((rcx as u32)) as u32 as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rcx = ((rcx.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x8u64), ((rcx as u32)) as u32); fl_l = ((rax).wrapping_add(crate::simd_ops::ru64(rbx))) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; if (((fl_l as u64)) == 0) { bb = 18; continue; } bb = 17; continue; }
            17 => { crate::simd_ops::wu32(rax.wrapping_add(0x8u64), (0x0u32) as u32); bb = 18; continue; }
            18 => { fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 21; continue; } bb = 19; continue; }
            19 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; crate::simd_ops::wu64(rax, rsi); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            20 => { rax = 0; fl_l = 0; fl_r = 0; bb = 21; continue; }
            21 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; bb = 22; continue; }
            22 => { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416219c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141524340; const P1: u64 = 0x141524330; const P2: u64 = 0x1415242e0; const P3: u64 = 0x143930f44; const P4: u64 = 0x1415c2bf0;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 19>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), r8); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x60u64)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; r12 = rdx; rbx = rcx; r13 = 0x1; r9 = ((crate::simd_ops::ru16(rax)) as u32) as u32 as u64; rax = ((rax).wrapping_add(0x2u64)) as u64; crate::simd_ops::wu64(rcx, rax); r15 = ((r9 as u32)) as u32 as u64; fl_l = ((r9 as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 26; continue; } bb = 1; continue; }
            1 => { r14 = r8.wrapping_add(0x20); rbp = 0; fl_l = 0; fl_r = 0; fl_l = ((r15 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(r14.wrapping_add(0xcu64))) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 15; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(r14)) as u64; rdx = ((r9 as u32)) as u32 as u64; rsi = (crate::simd_ops::ru32(r14.wrapping_add(0x8u64))) as u32 as u64; rcx = 0; fl_l = 0; fl_r = 0; rdx = ((rdx).wrapping_shl(3u32)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 3; continue; }
            3 => { r8 = rdx; rdx = rax; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rdi = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 12; continue; } bb = 4; continue; }
            4 => { bb = 13; continue; }
            5 => { rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rdi = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(r14)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 7; continue; }
            7 => { fl_l = rax; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 12; continue; } bb = 8; continue; }
            8 => { rdx = rbp; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 9; continue; }
            9 => { bb = 10; continue; }
            10 => { rax = (crate::simd_ops::ru64(r14)) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(rdx.wrapping_mul(8)))) as u64; crate::simd_ops::wu64(rdi.wrapping_add(rdx.wrapping_mul(8)), rcx); rdx = ((rdx).wrapping_add(1)) as u64; fl_l = rdx; fl_r = rsi; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 10; continue; } bb = 11; continue; }
            11 => { rdx = (crate::simd_ops::ru64(r14)) as u64; rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 12; continue; }
            12 => { crate::simd_ops::wu64(r14, rdi); rax = (((r13 as u8)) as u32) as u32 as u64; crate::simd_ops::wu32(r14.wrapping_add(0xcu64), ((r15 as u32)) as u32); bb = 13; continue; }
            13 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; rcx = 0x34; rcx = (if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { (r13 as u32) } else { (rcx as u32) }) as u32 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 15; continue; } bb = 14; continue; }
            14 => { rax = ((rcx as u32)) as u32 as u64; bb = 27; continue; }
            15 => { fl_l = ((r15 as u32)) as u32 as u64; fl_r = ((r15 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 26; continue; } bb = 16; continue; }
            16 => { bb = 17; continue; }
            17 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rsi = (crate::simd_ops::ru32(rax)) as u32 as u64; rax = ((rax).wrapping_add(0x4u64)) as u64; crate::simd_ops::wu64(rbx, rax); r13 = ((crate::simd_ops::ru8(rax)) as u32) as u32 as u64; rax = ((rax).wrapping_add(1)) as u64; crate::simd_ops::wu64(rbx, rax); r11 = ((crate::simd_ops::ru8(rax)) as u32) as u32 as u64; rax = ((rax).wrapping_add(1)) as u64; crate::simd_ops::wu64(rbx, rax); rcx = ((crate::simd_ops::ru8(rax)) as u32) as u32 as u64; rax = ((rax).wrapping_add(1)) as u64; r9 = ((rcx as u32)) as u32 as u64; crate::simd_ops::wu64(rbx, rax); r9 = ((((r9 as u32)) & (0x7fu32))) as u32 as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; if (((((fl_l as u8)) & ((fl_r as u8))) as i8) >= 0) { bb = 20; continue; } bb = 18; continue; }
            18 => { bb = 19; continue; }
            19 => { r8 = ((crate::simd_ops::ru8(rax)) as u32) as u32 as u64; rcx = ((r9 as u32)) as u32 as u64; rcx = (((rcx as u32)).wrapping_shl(7u32)) as u32 as u64; rax = ((rax).wrapping_add(1)) as u64; rdx = ((r8 as u32)) as u32 as u64; crate::simd_ops::wu64(rbx, rax); rdx = ((((rdx as u32)) & (0x7fu32))) as u32 as u64; r9 = ((rdx as u32)) as u32 as u64; r9 = ((((r9 as u32)) | ((rcx as u32)))) as u32 as u64; fl_l = ((r8 as u8)) as u8 as u64; fl_r = ((r8 as u8)) as u8 as u64; if (((((fl_l as u8)) & ((fl_r as u8))) as i8) < 0) { bb = 19; continue; } bb = 20; continue; }
            20 => { r8 = (crate::simd_ops::ru32(rax)) as u32 as u64; rax = ((rax).wrapping_add(0x4u64)) as u64; crate::simd_ops::wu64(rbx, rax); rdx = rax.wrapping_add(0x1); r10 = ((crate::simd_ops::ru8(rax)) as u32) as u32 as u64; crate::simd_ops::wu64(rbx, rdx); rdi = ((crate::simd_ops::ru16(rdx)) as u32) as u32 as u64; rdx = ((rdx).wrapping_add(0x2u64)) as u64; crate::simd_ops::wu64(rbx, rdx); fl_l = ((r11 as u8)) as u8 as u64; fl_r = (0x6u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 22; continue; } bb = 21; continue; }
            21 => { fl_l = (crate::simd_ops::ru32(P3)) as u32 as u64; fl_r = 0x1; rax = 0x2; rcx = 0x5; rax = (if (((fl_l as u32)) == ((fl_r as u32))) { (rcx as u32) } else { (rax as u32) }) as u32 as u64; bb = 23; continue; }
            22 => { rax = (((r11 as u8)) as u32) as u32 as u64; bb = 23; continue; }
            23 => { rcx = rsp.wrapping_add(0xa0); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rcx); rcx = r14; crate::simd_ops::wu32(rsp.wrapping_add(0x50u64), ((rdi as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rdx); rdx = 0x2; crate::simd_ops::wu32(rsp.wrapping_add(0x40u64), ((r10 as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), ((r8 as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((r9 as u32)) as u32); r9 = ((rsi as u32)) as u32 as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x28u64), ((rax as u8)) as u8); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xb0u64))) as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x20u64), ((r13 as u8)) as u8); r8 = (crate::simd_ops::ru32(rax.wrapping_add(0x10u64))) as u32 as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 27; continue; } bb = 24; continue; }
            24 => { rax = ((((rdi as u32)) as i32).wrapping_mul((0xfffffff4u32) as i32) as u32) as u32 as u64; rcx = ((rdi.wrapping_add(rdi.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_shl(2u32)) as u64; rbp = (((rbp as u32)).wrapping_add(1)) as u32 as u64; crate::simd_ops::lk_add_u64(rbx, (rcx) as u64); crate::simd_ops::lk_add_u32(r12, ((rax as u32)) as u32); fl_l = ((rbp as u32)) as u32 as u64; fl_r = ((r15 as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 17; continue; } bb = 25; continue; }
            25 => { r13 = 0x1; bb = 26; continue; }
            26 => { rax = ((r13 as u32)) as u32 as u64; bb = 27; continue; }
            27 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0xa8u64))) as u64; rsp = ((rsp).wrapping_add(0x60u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141621be0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142690720; const P1: u64 = 0x1415cca50; const P2: u64 = 0x141524340; const P3: u64 = 0x141524330; const P4: u64 = 0x1415242e0; const P5: u64 = 0x141622c50;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x28u64)) as u64; r15 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu8(rcx.wrapping_add(0x20u64), (0xffu8) as u8); crate::simd_ops::wu64(rcx.wrapping_add(0x8u64), r15); rax = P0; crate::simd_ops::wu32(rcx.wrapping_add(0x10u64), ((r15 as u32)) as u32); r14 = rcx; crate::simd_ops::wu32(rcx.wrapping_add(0x18u64), ((r15 as u32)) as u32); crate::simd_ops::wu64(rcx.wrapping_add(0x30u64), r15); crate::simd_ops::wu64(rcx.wrapping_add(0x38u64), r15); crate::simd_ops::wu64(rcx.wrapping_add(0x40u64), r15); crate::simd_ops::wu8(rcx.wrapping_add(0x28u64), (0xffu8) as u8); crate::simd_ops::wu64(rcx.wrapping_add(0x50u64), r8); crate::simd_ops::wu64(rcx.wrapping_add(0x58u64), r15); crate::simd_ops::wu64(rcx.wrapping_add(0x60u64), r15); crate::simd_ops::wu64(rcx, rax); crate::simd_ops::wu64(rcx.wrapping_add(0x48u64), rdx); fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(0x40u64))) as u32 as u64; fl_r = ((r15 as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { xmm0 = (crate::simd_ops::ru128(rdx.wrapping_add(0x10u64))) as u128; xmm1 = (crate::simd_ops::ru128(rdx.wrapping_add(0x20u64))) as u128; xmm2 = (crate::simd_ops::ru128(rdx.wrapping_add(0x30u64))) as u128; crate::simd_ops::wu128(rcx.wrapping_add(0x8u64), (xmm0) as u128); crate::simd_ops::wu128(rcx.wrapping_add(0x18u64), (xmm1) as u128); crate::simd_ops::wu128(rcx.wrapping_add(0x28u64), (xmm2) as u128); bb = 2; continue; }
            2 => { crate::simd_ops::lk_add_u32(r8.wrapping_add(0x14u64), (1) as u32); rdx = (crate::simd_ops::ru64(rcx.wrapping_add(0x50u64))) as u64; fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0x4cu64))) as u8 as u64; fl_r = (0x8u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 5; continue; } bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru32(rdx.wrapping_add(0x48u64))) as u32 as u64; fl_r = ((r15 as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rdx = (crate::simd_ops::ru64(rdx.wrapping_add(0x48u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(r14.wrapping_add(0x50u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rsi); r13 = (crate::simd_ops::ru32(rax.wrapping_add(0x38u64))) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r12); fl_l = ((r13 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(r14.wrapping_add(0x64u64))) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 26; continue; } bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(r14.wrapping_add(0x58u64))) as u64; rdx = ((r13 as u32)) as u32 as u64; rbp = (crate::simd_ops::ru32(r14.wrapping_add(0x60u64))) as u32 as u64; rcx = 0; fl_l = 0; fl_r = 0; rdx = ((rdx).wrapping_shl(5u32)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 7; continue; }
            7 => { r8 = rdx; rdx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r15 = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 23; continue; } bb = 8; continue; }
            8 => { bb = 24; continue; }
            9 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r15 = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 24; continue; } bb = 10; continue; }
            10 => { rax = (crate::simd_ops::ru64(r14.wrapping_add(0x58u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 23; continue; } bb = 11; continue; }
            11 => { fl_l = rax; fl_r = r15; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 23; continue; } bb = 12; continue; }
            12 => { fl_l = rbp; fl_r = rbp; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 22; continue; } bb = 13; continue; }
            13 => { r12 = (0xffffffffffffffe4u64) as u64; rbx = r15.wrapping_add(0x1c); fl_l = r12; fl_r = r15; r12 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; bb = 14; continue; }
            14 => { rdx = rbx.wrapping_sub(0x1c); r8 = 0; fl_l = 0; fl_r = 0; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 16; continue; } bb = 15; continue; }
            15 => { crate::simd_ops::wu64(rdx, r8); crate::simd_ops::wu32(rbx.wrapping_sub(0x14u64), ((r8 as u32)) as u32); crate::simd_ops::wu64(rbx.wrapping_sub(0xcu64), r8); crate::simd_ops::wu64(rbx.wrapping_sub(0x4u64), r8); bb = 16; continue; }
            16 => { rax = (crate::simd_ops::ru64(r14.wrapping_add(0x58u64))) as u64; rcx = ((r12.wrapping_add(rbx)) as u64) as u64; xmm0 = (crate::simd_ops::ru128(rax.wrapping_add(rcx))) as u128; crate::simd_ops::wu128(rdx, (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(rax.wrapping_add(rcx).wrapping_add(0x10u64))) as u128; crate::simd_ops::wu128(rdx.wrapping_add(0x10u64), (xmm1) as u128); rsi = (crate::simd_ops::ru64(r14.wrapping_add(0x58u64))) as u64; rsi = ((rsi).wrapping_add(rcx)) as u64; rdx = (crate::simd_ops::ru64(rsi.wrapping_add(0x10u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 18; continue; } bb = 17; continue; }
            17 => { rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rsi.wrapping_add(0x18u64), ((r8 as u32)) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsi.wrapping_add(0x10u64), rax); crate::simd_ops::wu32(rsi.wrapping_add(0x1cu64), ((rax as u32)) as u32); bb = 18; continue; }
            18 => { rax = (crate::simd_ops::ru64(rsi)) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 21; continue; } bb = 19; continue; }
            19 => { rcx = ((rcx).wrapping_add(0x10u64)) as u64; rax = 0xffffffff; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rcx.wrapping_add(0x14u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 21; continue; } bb = 20; continue; }
            20 => { rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 21; continue; }
            21 => { rbx = ((rbx).wrapping_add(0x20u64)) as u64; fl_l = rbp; fl_r = (0x1u64) as u64; rbp = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 14; continue; } bb = 22; continue; }
            22 => { rdx = (crate::simd_ops::ru64(r14.wrapping_add(0x58u64))) as u64; rcx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 23; continue; }
            23 => { crate::simd_ops::wu64(r14.wrapping_add(0x58u64), r15); rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); crate::simd_ops::wu32(r14.wrapping_add(0x64u64), ((r13 as u32)) as u32); bb = 24; continue; }
            24 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 38; continue; } bb = 25; continue; }
            25 => { r15 = 0; fl_l = 0; fl_r = 0; bb = 26; continue; }
            26 => { rax = (crate::simd_ops::ru64(r14.wrapping_add(0x50u64))) as u64; rbx = (crate::simd_ops::ru64(rax.wrapping_add(0x30u64))) as u64; rcx = (crate::simd_ops::ru32(rax.wrapping_add(0x38u64))) as u32 as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rcx = ((rcx).wrapping_add(rbx)) as u64; fl_l = rbx; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 38; continue; } bb = 27; continue; }
            27 => { r12 = 0x1; bb = 28; continue; }
            28 => { rsi = (crate::simd_ops::ru32(r14.wrapping_add(0x60u64))) as u32 as u64; rcx = ((rsi as u32)) as u32 as u64; rax = (crate::simd_ops::ru32(r14.wrapping_add(0x64u64))) as u32 as u64; rbp = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 31; continue; } bb = 29; continue; }
            29 => { rdx = ((rax as u32)) as u32 as u64; rcx = r14.wrapping_add(0x58); rdx = (((rdx as u32)).wrapping_shr(1u32)) as u32 as u64; rdx = (((rdx as u32)).wrapping_add((rax as u32))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; rdx = (if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { (r12 as u32) } else { (rdx as u32) }) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 34; continue; } bb = 30; continue; }
            30 => { rcx = (crate::simd_ops::ru32(r14.wrapping_add(0x60u64))) as u32 as u64; bb = 31; continue; }
            31 => { fl_l = ((rsi as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(r14.wrapping_add(0x64u64))) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 34; continue; } bb = 32; continue; }
            32 => { rax = ((rcx as u32)) as u32 as u64; rcx = ((rcx.wrapping_add(0x1u64)) as u32) as u32 as u64; rax = ((rax).wrapping_shl(5u32)) as u64; crate::simd_ops::wu32(r14.wrapping_add(0x60u64), ((rcx as u32)) as u32); fl_l = ((rax).wrapping_add(crate::simd_ops::ru64(r14.wrapping_add(0x58u64)))) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; if (((fl_l as u64)) == 0) { bb = 35; continue; } bb = 33; continue; }
            33 => { crate::simd_ops::wu64(rax, r15); crate::simd_ops::wu32(rax.wrapping_add(0x8u64), ((r15 as u32)) as u32); crate::simd_ops::wu64(rax.wrapping_add(0x10u64), r15); crate::simd_ops::wu64(rax.wrapping_add(0x18u64), (0x0u64) as u64); bb = 35; continue; }
            34 => { rax = r15; bb = 35; continue; }
            35 => { crate::simd_ops::wu64(rax, rbp); rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x8u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 37; continue; } bb = 36; continue; }
            36 => { fl_l = (crate::simd_ops::lk_add_u32(rax.wrapping_add(0x24u64), (1) as u32)) as u32 as u64; bb = 37; continue; }
            37 => { rcx = (crate::simd_ops::ru64(r14.wrapping_add(0x50u64))) as u64; rbx = ((rbx).wrapping_add(0x10u64)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x38u64))) as u32 as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(rcx.wrapping_add(0x30u64)))) as u64; fl_l = rbx; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 28; continue; } bb = 38; continue; }
            38 => { r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rax = r14; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x28u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141621e80(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142690720; const P1: u64 = 0x1415ccc10; const P2: u64 = 0x1415aeaa0; const P3: u64 = 0x1415242e0; const P4: u64 = 0x1415cc8f0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(0x50u64))) as u64; rax = P0; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rsi); r15 = rcx; crate::simd_ops::wu64(rcx, rax); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), r12); fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0x4cu64))) as u8 as u64; fl_r = (0x8u8) as u8 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r14); if (((fl_l as u8)) == ((fl_r as u8))) { bb = 3; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru32(rdx.wrapping_add(0x48u64))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rdx = (crate::simd_ops::ru64(rdx.wrapping_add(0x48u64))) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 3; continue; }
            3 => { r14 = (crate::simd_ops::ru64(r15.wrapping_add(0x58u64))) as u64; r12 = 0; fl_l = 0; fl_r = 0; rcx = (crate::simd_ops::ru32(r15.wrapping_add(0x60u64))) as u32 as u64; rax = ((rcx as u32)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rbx); rax = ((rax).wrapping_shl(5u32)) as u64; rax = ((rax).wrapping_add(r14)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rdi); fl_l = r14; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 11; continue; } bb = 4; continue; }
            4 => { crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rbp); bb = 5; continue; }
            5 => { rdi = (crate::simd_ops::ru64(r14.wrapping_add(0x10u64))) as u64; rax = (crate::simd_ops::ru32(r14.wrapping_add(0x18u64))) as u32 as u64; rcx = ((rdi.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = rdi; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 9; continue; } bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rsi = (crate::simd_ops::ru64(r15.wrapping_add(0x50u64))) as u64; fl_l = rax; fl_r = rax; rbp = rax.wrapping_add(0x18); rbx = (crate::simd_ops::ru64(rsi.wrapping_add(0x20u64))) as u64; rbp = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { r12 } else { rbp }) as u64; rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x28u64))) as u32 as u64; rcx = ((rbx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 8; continue; } bb = 7; continue; }
            7 => { rdx = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbp; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru32(rsi.wrapping_add(0x28u64))) as u32 as u64; rbx = ((rbx).wrapping_add(0x8u64)) as u64; rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x20u64))) as u64; rcx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 7; continue; } bb = 8; continue; }
            8 => { rcx = (crate::simd_ops::ru32(r14.wrapping_add(0x18u64))) as u32 as u64; rdi = ((rdi).wrapping_add(0x8u64)) as u64; rax = (crate::simd_ops::ru64(r14.wrapping_add(0x10u64))) as u64; rcx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rdi; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 6; continue; } bb = 9; continue; }
            9 => { rcx = (crate::simd_ops::ru32(r15.wrapping_add(0x60u64))) as u32 as u64; r14 = ((r14).wrapping_add(0x20u64)) as u64; rdx = (crate::simd_ops::ru64(r15.wrapping_add(0x58u64))) as u64; rax = ((rcx as u32)) as u32 as u64; rax = ((rax).wrapping_shl(5u32)) as u64; rax = ((rax).wrapping_add(rdx)) as u64; fl_l = r14; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 5; continue; } bb = 10; continue; }
            10 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; r14 = rdx; bb = 11; continue; }
            11 => { rsi = 0xffffffff; fl_l = r14; fl_r = r14; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 21; continue; } bb = 12; continue; }
            12 => { rbx = ((rcx as u32)) as u32 as u64; rbx = ((rbx).wrapping_shl(5u32)) as u64; rbx = ((rbx).wrapping_add(r14)) as u64; fl_l = r14; fl_r = rbx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 20; continue; } bb = 13; continue; }
            13 => { rdi = r14.wrapping_add(0x10); bb = 14; continue; }
            14 => { rdx = (crate::simd_ops::ru64(rdi)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 16; continue; } bb = 15; continue; }
            15 => { rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rdi.wrapping_add(0x8u64), ((r12 as u32)) as u32); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rdi, r12); crate::simd_ops::wu32(rdi.wrapping_add(0xcu64), ((r12 as u32)) as u32); bb = 16; continue; }
            16 => { rax = (crate::simd_ops::ru64(rdi.wrapping_sub(0x10u64))) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 19; continue; } bb = 17; continue; }
            17 => { rcx = ((rcx).wrapping_add(0x10u64)) as u64; rax = ((rsi as u32)) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rcx.wrapping_add(0x14u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 19; continue; } bb = 18; continue; }
            18 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 19; continue; }
            19 => { rdi = ((rdi).wrapping_add(0x20u64)) as u64; rax = rdi.wrapping_sub(0x10); fl_l = rax; fl_r = rbx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 14; continue; } bb = 20; continue; }
            20 => { rdx = (crate::simd_ops::ru64(r15.wrapping_add(0x58u64))) as u64; rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(r15.wrapping_add(0x60u64), ((r12 as u32)) as u32); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(r15.wrapping_add(0x58u64), r12); crate::simd_ops::wu32(r15.wrapping_add(0x64u64), ((r12 as u32)) as u32); bb = 21; continue; }
            21 => { rcx = (crate::simd_ops::ru64(r15.wrapping_add(0x50u64))) as u64; let xa_src: u32 = (rsi as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rcx.wrapping_add(0x14u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rsi = (xa_old) as u32 as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; fl_l = ((rsi as u32)) as u32 as u64; fl_r = 0x1; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 23; continue; } bb = 22; continue; }
            22 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 23; continue; }
            23 => { rcx = r15; rsp = ((rsp).wrapping_add(0x30u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P4, rax, rcx, rdx, r8, r9, rsp); }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141622040(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142690730; const P1: u64 = 0x142690750; const P2: u64 = 0x1415bfa60; const P3: u64 = 0x14161f8a0; const P4: u64 = 0x1415242e0; const P5: u64 = 0x1415c2b30; const P6: u64 = 0x1415bd160; const P7: u64 = 0x141834d30;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rbx); rax = P0; crate::simd_ops::wu64(rcx, rax); rdi = rcx; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rbp); rax = P1; crate::simd_ops::wu64(rcx.wrapping_add(0x18u64), rax); rbp = ((rdx as u32)) as u32 as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x38u64))) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rsi); rsi = (crate::simd_ops::ru64(rcx.wrapping_add(0x30u64))) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r15); rax = ((rax).wrapping_add(rsi)) as u64; r15 = 0; fl_l = 0; fl_r = 0; fl_l = rsi; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 17; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), r14); bb = 2; continue; }
            2 => { rbx = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 3; continue; }
            3 => { rdx = rdi; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { r9 = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; r8 = rbx; crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), r15); rdx = rdi; rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x40u64))) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            5 => { r14 = (crate::simd_ops::ru64(rbx.wrapping_add(0x28u64))) as u64; crate::simd_ops::wu32(rbx, ((r15 as u32)) as u32); fl_l = r14; fl_r = r14; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 6; continue; }
            6 => { rdx = (crate::simd_ops::ru64(r14)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(r14.wrapping_add(0x8u64), ((r15 as u32)) as u32); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(r14, r15); crate::simd_ops::wu32(r14.wrapping_add(0xcu64), ((r15 as u32)) as u32); bb = 8; continue; }
            8 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x28u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { rcx = 0x2; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 10; continue; }
            10 => { crate::simd_ops::wu64(rbx.wrapping_add(0x28u64), r15); bb = 11; continue; }
            11 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 12; continue; }
            12 => { rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 13; continue; }
            13 => { crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), r15); crate::simd_ops::wu64(rbx.wrapping_add(0x18u64), r15); crate::simd_ops::wu32(rbx.wrapping_add(0x20u64), (0x1u32) as u32); rdx = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 15; continue; } bb = 14; continue; }
            14 => { rcx = 0x10; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 15; continue; }
            15 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x38u64))) as u32 as u64; rsi = ((rsi).wrapping_add(0x10u64)) as u64; rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x30u64))) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rax = ((rax).wrapping_add(rcx)) as u64; fl_l = rsi; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 16; continue; }
            16 => { r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rsi = rcx; bb = 17; continue; }
            17 => { fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 19; continue; } bb = 18; continue; }
            18 => { rdx = rsi; crate::simd_ops::wu32(rdi.wrapping_add(0x38u64), ((r15 as u32)) as u32); rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rdi.wrapping_add(0x30u64), r15); crate::simd_ops::wu32(rdi.wrapping_add(0x3cu64), ((r15 as u32)) as u32); bb = 19; continue; }
            19 => { rcx = rdi.wrapping_add(0x20); rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rcx = rdi.wrapping_add(0x20); rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rcx = rdi; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; fl_l = ((rbp as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 21; continue; } bb = 20; continue; }
            20 => { rdx = 0x50; rcx = rdi; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); bb = 21; continue; }
            21 => { rax = rdi; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416221c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14153e940; const P1: u64 = 0x1415ae570;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x28u64)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x58u64))) as u64; r11 = r8; r10 = (crate::simd_ops::ru32(rcx.wrapping_add(0x60u64))) as u32 as u64; r14 = rdx; r10 = ((r10).wrapping_shl(5u32)) as u64; rbp = rcx; r10 = ((r10).wrapping_add(rax)) as u64; fl_l = rax; fl_r = r10; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 16; continue; } bb = 1; continue; }
            1 => { bb = 2; continue; }
            2 => { r9 = (crate::simd_ops::ru64(rax)) as u64; r8 = (crate::simd_ops::ru64(r9.wrapping_add(0x8u64))) as u64; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { fl_l = r8; fl_r = r11; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 6; continue; } bb = 4; continue; }
            4 => { rax = ((rax).wrapping_add(0x20u64)) as u64; fl_l = rax; fl_r = r10; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 5; continue; }
            5 => { rsp = ((rsp).wrapping_add(0x28u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            6 => { crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rbx); rbx = rax.wrapping_add(0x10); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rdi = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; fl_l = ((rdi as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rbx.wrapping_add(0xcu64))) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 9; continue; } bb = 7; continue; }
            7 => { rcx = rbx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 15; continue; } bb = 8; continue; }
            8 => { fl_l = ((rdi as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rbx.wrapping_add(0xcu64))) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 15; continue; } bb = 9; continue; }
            9 => { rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; rax = ((rcx.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x8u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 15; continue; } bb = 10; continue; }
            10 => { rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rdx, r14); rcx = r14.wrapping_add(0x18); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rsi); rsi = rcx; rdx = rsp.wrapping_add(0x40); fl_l = r14; fl_r = r14; rsi = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rax } else { rsi }) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x30u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rdi = (crate::simd_ops::ru64(rbp.wrapping_add(0x50u64))) as u64; r14 = rax; rbx = (crate::simd_ops::ru64(rdi.wrapping_add(0x20u64))) as u64; rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x28u64))) as u32 as u64; rdx = ((rbx.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 14; continue; } bb = 11; continue; }
            11 => { rdx = (crate::simd_ops::ru64(rbx)) as u64; rax = (crate::simd_ops::ru64(r14)) as u64; rcx = (crate::simd_ops::ru32(rdx.wrapping_add(0x8u64))) as u32 as u64; fl_l = (((rax) >> (((rcx) as u32 & 63))) & 1) as u64; if ((fl_l as u64) == 0) { bb = 13; continue; } bb = 12; continue; }
            12 => { rcx = rsi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 13; continue; }
            13 => { rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x28u64))) as u32 as u64; rbx = ((rbx).wrapping_add(0x8u64)) as u64; rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x20u64))) as u64; rcx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 11; continue; } bb = 14; continue; }
            14 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; bb = 15; continue; }
            15 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; bb = 16; continue; }
            16 => { rsp = ((rsp).wrapping_add(0x28u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416222e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xffffffffffffffb2; const P1: u64 = 0x141524330; const P2: u64 = 0x1415bd140; const P3: u64 = 0x142690730; const P4: u64 = 0x4c; const P5: u64 = 0x142690750; const P6: u64 = 0x18; const P7: u64 = 0x20; const P8: u64 = 0x28; const P9: u64 = 0x30; const P10: u64 = 0x38; const P11: u64 = 0x40;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdx = 0x50;
    rcx = ((rdx.wrapping_add(P0)) as u32) as u32 as u64;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rbx = rax;
    fl_l = rax; fl_r = rax;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rcx = rax; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rax = P3; crate::simd_ops::wu8(rbx.wrapping_add(P4), (0x8u8) as u8); crate::simd_ops::wu64(rbx, rax); rax = P5; crate::simd_ops::wu64(rbx.wrapping_add(P6), rax); rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rbx.wrapping_add(P7), rax); crate::simd_ops::wu64(rbx.wrapping_add(P8), rax); crate::simd_ops::wu64(rbx.wrapping_add(P9), rax); crate::simd_ops::wu64(rbx.wrapping_add(P10), rax); crate::simd_ops::wu64(rbx.wrapping_add(P11), rax); crate::simd_ops::wu32(rbx.wrapping_add(0x48u64), ((rax as u32)) as u32); rax = rbx; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141622350(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x60; const P1: u64 = 0x58; const P2: u64 = 0x8; const P3: u64 = 0x20;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r8 = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; r8 = ((r8).wrapping_shl(5u32)) as u64; fl_l = ((r8).wrapping_add(rax)) as u64; fl_r = fl_l; r8 = ((fl_l as u64)) as u64; bb = 1; continue; }
            1 => { fl_l = rax; fl_r = r8; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 5; continue; } bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru64(rax)) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(P3))) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 6; continue; } bb = 4; continue; }
            4 => { fl_l = ((rax).wrapping_add(0x20u64)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; bb = 1; continue; }
            5 => { rax = 0; fl_l = 0; fl_r = 0; bb = 6; continue; }
            6 => { return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141622380(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14159e060; const P1: u64 = 0x143930fa0; const P2: u64 = 0x1415c3fb0; const P3: u64 = 0x141529ed0; const P4: u64 = 0x1434268d0; const P5: u64 = 0x141594710; const P6: u64 = 0x1415c0c30; const P7: u64 = 0x1415d8ec0; const P8: u64 = 0x143930f88; const P9: u64 = 0x1426452b0; const P10: u64 = 0x1426452a8; const P11: u64 = 0x142645560; const P12: u64 = 0x14159a6d0;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 25>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0xa0u64)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x10u64))) as u32 as u64; rsi = r9; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0xf0u64))) as u64; rbx = rdx; r9 = r8; crate::simd_ops::wu16(rsp.wrapping_add(0x3cu64), (0x1u16) as u16); rdx = rcx.wrapping_add(0x20); crate::simd_ops::wu32(rsp.wrapping_add(0x40u64), ((rax as u32)) as u32); rdi = rcx; r8 = rsp.wrapping_add(0x38); r12 = 0; fl_l = 0; fl_r = 0; rcx = rbp; crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), ((r12 as u32)) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r13 = ((r12.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x48u64))) as u32 as u64; fl_r = ((r12 as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 23; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x38u64))) as u32 as u64; rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x20u64))) as u32 as u64; rbx = (crate::simd_ops::ru64(rdi.wrapping_add(0x30u64))) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rax = ((rax).wrapping_add(rbx)) as u64; fl_l = rbx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 5; continue; } bb = 2; continue; }
            2 => { bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru32(rbx)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rbx = ((rbx).wrapping_add(0x10u64)) as u64; fl_l = rbx; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; fl_l = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; fl_r = r12; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 23; continue; } bb = 6; continue; }
            6 => { rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x48u64))) as u64; rax = rsp.wrapping_add(0x58); rcx = (crate::simd_ops::ru64(P1)) as u64; r9 = rsp.wrapping_add(0x30); r8 = rbp; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); crate::simd_ops::wu128(rsp.wrapping_add(0x90u64), (xmm6) as u128); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), r12); crate::simd_ops::wu32(rsp.wrapping_add(0x60u64), ((r12 as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x68u64), ((r12 as u32)) as u32); crate::simd_ops::wu8(rsp.wrapping_add(0x70u64), (0xffu8) as u8); crate::simd_ops::wu8(rsp.wrapping_add(0x78u64), (0xffu8) as u8); crate::simd_ops::wu64(rsp.wrapping_add(0x80u64), r12); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; fl_r = r12; rbx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rbp = (rbp & !0xffu64) | (((if (((fl_l as u64)) != ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; fl_r = r12; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 10; continue; } bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x20u64))) as u32 as u64; r8 = rsp.wrapping_add(0xd0); xmm1 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x30u64))) as u32) as u128; rcx = rbx.wrapping_add(0x10); crate::simd_ops::wu32(rsp.wrapping_add(0xd0u64), ((rax as u32)) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(P4)) as u32 as u64; rax = (crate::simd_ops::ru32(rsp.wrapping_add(0xd0u64))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x20u64), ((rax as u32)) as u32); xmm6 = (xmm0) as u128; if (f32::from_bits((fl_l as u32)) != f32::from_bits((fl_r as u32)) && !f32::from_bits((fl_l as u32)).is_nan() && !f32::from_bits((fl_r as u32)).is_nan()) { bb = 11; continue; } bb = 8; continue; }
            8 => { fl_l = ((rbp as u8)) as u8 as u64; fl_r = ((rbp as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 22; continue; } bb = 9; continue; }
            9 => { bb = 11; continue; }
            10 => { xmm6 = 0u128; bb = 11; continue; }
            11 => { rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x50u64))) as u32 as u64; rbx = (crate::simd_ops::ru32(rsi.wrapping_add(0x8u64))) as u32 as u64; rax = ((((rax as u32)) & (0xfffffffcu32))) as u32 as u64; r8 = (((rbp as u8)) as u32) as u32 as u64; r8 = ((((r8 as u32)) ^ ((r13 as u32)))) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rdi); r8 = ((((r8 as u32)) | ((rax as u32)))) as u32 as u64; rax = ((r8 as u32)) as u32 as u64; rcx = ((rbx.wrapping_add(rbx.wrapping_mul(2))) as u64) as u64; rax = ((((rax as u32)) & (0xffffff03u32))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x50u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rsi)) as u64; rcx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 15; continue; } bb = 12; continue; }
            12 => { fl_l = (crate::simd_ops::ru64(rax)) as u64; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 14; continue; } bb = 13; continue; }
            13 => { rdx = ((r8 as u32)) as u32 as u64; rdx = ((((rdx as u32)) ^ (crate::simd_ops::ru32(rax.wrapping_add(0x8u64))))) as u32 as u64; fl_l = ((r13 as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 15; continue; } bb = 14; continue; }
            14 => { rax = ((rax).wrapping_add(0x18u64)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 12; continue; } bb = 15; continue; }
            15 => { fl_l = rax; fl_r = rcx; rdx = rax.wrapping_add(0x10); rdx = (if (((fl_l as u64)) == ((fl_r as u64))) { r12 } else { rdx }) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 16; continue; }
            16 => { crate::simd_ops::wu32(rdx, ((xmm6 as u32)) as u32); bb = 22; continue; }
            17 => { rax = (crate::simd_ops::ru32(rsi.wrapping_add(0xcu64))) as u32 as u64; fl_l = rbx; fl_r = rax; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 19; continue; } bb = 18; continue; }
            18 => { rdx = ((rax as u32)) as u32 as u64; rcx = rsi; rdx = (((rdx as u32)).wrapping_shr(1u32)) as u32 as u64; rdx = (((rdx as u32)).wrapping_add((rax as u32))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; rdx = (if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { (r13 as u32) } else { (rdx as u32) }) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 22; continue; } bb = 19; continue; }
            19 => { rax = (crate::simd_ops::ru32(rsi.wrapping_add(0xcu64))) as u32 as u64; fl_l = rbx; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 22; continue; } bb = 20; continue; }
            20 => { rcx = (crate::simd_ops::ru32(rsi.wrapping_add(0x8u64))) as u32 as u64; rax = ((rcx.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rsi.wrapping_add(0x8u64), ((rax as u32)) as u32); rcx = ((rcx.wrapping_add(rcx.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru64(rsi)) as u64; rdx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 22; continue; } bb = 21; continue; }
            21 => { xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x48u64))) as u128; crate::simd_ops::wu32(rdx.wrapping_add(0x10u64), ((xmm6 as u32)) as u32); crate::simd_ops::wu128(rdx, (xmm0) as u128); bb = 22; continue; }
            22 => { xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x90u64))) as u128; bb = 23; continue; }
            23 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0xf8u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xd8u64), r14); fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 60; continue; } bb = 24; continue; }
            24 => { rsi = (crate::simd_ops::ru32(rbx.wrapping_add(0x148u64))) as u32 as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x14cu64))) as u32 as u64; fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 26; continue; } bb = 25; continue; }
            25 => { rdx = ((rax as u32)) as u32 as u64; rcx = rbx; rdx = (((rdx as u32)).wrapping_shr(1u32)) as u32 as u64; rdx = (((rdx as u32)).wrapping_add((rax as u32))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; rdx = (if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { (r13 as u32) } else { (rdx as u32) }) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 62; continue; } bb = 26; continue; }
            26 => { fl_l = ((rsi as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rbx.wrapping_add(0x14cu64))) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 62; continue; } bb = 27; continue; }
            27 => { rdx = (crate::simd_ops::ru32(rbx.wrapping_add(0x148u64))) as u32 as u64; rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x140u64))) as u64; rcx = ((rdx.wrapping_add(rdx.wrapping_mul(4))) as u64) as u64; r14 = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; rax = ((rdx.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x148u64), ((rax as u32)) as u32); fl_l = r14; fl_r = r14; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 62; continue; } bb = 28; continue; }
            28 => { crate::simd_ops::wu64(r14, r12); rax = rdi.wrapping_add(0x18); crate::simd_ops::wu64(r14.wrapping_add(0x8u64), r12); rcx = rdi.wrapping_add(0x20); crate::simd_ops::wu32(r14.wrapping_add(0x10u64), ((r12 as u32)) as u32); fl_l = rdi; fl_r = rdi; crate::simd_ops::wu64(r14.wrapping_add(0x18u64), r12); rdx = r14; crate::simd_ops::wu64(r14.wrapping_add(0x20u64), r12); rax = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { r12 } else { rax }) as u64; crate::simd_ops::wu64(r14.wrapping_add(0x8u64), rax); rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x40u64))) as u64; crate::simd_ops::wu64(r14, rax); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(0x4cu64))) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 60; continue; } bb = 29; continue; }
            29 => { rbp = (crate::simd_ops::ru32(rdi.wrapping_add(0x48u64))) as u32 as u64; rbx = (crate::simd_ops::ru64(P8)) as u64; bb = 30; continue; }
            30 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x268u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 41; continue; } bb = 31; continue; }
            31 => { rcx = ((rax.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; let cx_src: u32 = (rcx as u32); let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u32(rbx.wrapping_add(0x268u64), fl_l as u32, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u32)) as u32 as u64; } if (((fl_l as u32)) != ((fl_r as u32))) { bb = 30; continue; } bb = 32; continue; }
            32 => { rsi = rbx.wrapping_add(0x270); bb = 33; continue; }
            33 => { rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x2b0u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 43; continue; } bb = 34; continue; }
            34 => { rdx = 0; fl_l = 0; fl_r = 0; rax = ((rbp as u32)) as u32 as u64; let d = ((rcx as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x2a8u64))) as u64; rdi = (crate::simd_ops::ru64(rax.wrapping_add(rdx.wrapping_mul(8)))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 38; continue; } bb = 35; continue; }
            35 => { bb = 36; continue; }
            36 => { fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x10u64))) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 39; continue; } bb = 37; continue; }
            37 => { rdi = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 36; continue; } bb = 38; continue; }
            38 => { rdi = r12; bb = 39; continue; }
            39 => { fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 44; continue; } bb = 40; continue; }
            40 => { fl_l = (crate::simd_ops::lk_add_u32(rdi.wrapping_add(0x14u64), (1) as u32)) as u32 as u64; bb = 44; continue; }
            41 => { rsi = rbx.wrapping_add(0x270); rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P9), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x268u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 33; continue; } bb = 42; continue; }
            42 => { rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P10), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 30; continue; }
            43 => { rdi = r12; bb = 44; continue; }
            44 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x268u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 51; continue; } bb = 45; continue; }
            45 => { rcx = ((rax.wrapping_sub(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; let cx_src: u32 = (rcx as u32); let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u32(rbx.wrapping_add(0x268u64), fl_l as u32, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u32)) as u32 as u64; } if (((fl_l as u32)) == ((fl_r as u32))) { bb = 48; continue; } bb = 46; continue; }
            46 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x268u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) >= 0) { bb = 45; continue; } bb = 47; continue; }
            47 => { bb = 52; continue; }
            48 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 53; continue; } bb = 49; continue; }
            49 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x298u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 53; continue; } bb = 50; continue; }
            50 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x2a0u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P11), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 53; continue; }
            51 => { rsi = rbx.wrapping_add(0x270); bb = 52; continue; }
            52 => { rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P10), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 53; continue; }
            53 => { rbx = (crate::simd_ops::ru32(r14.wrapping_add(0x20u64))) as u32 as u64; rax = (crate::simd_ops::ru32(r14.wrapping_add(0x24u64))) as u32 as u64; fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 55; continue; } bb = 54; continue; }
            54 => { rdx = ((rax as u32)) as u32 as u64; rcx = r14.wrapping_add(0x18); rdx = (((rdx as u32)).wrapping_shr(1u32)) as u32 as u64; rdx = (((rdx as u32)).wrapping_add((rax as u32))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; rdx = (if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { (r13 as u32) } else { (rdx as u32) }) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P12, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 58; continue; } bb = 55; continue; }
            55 => { fl_l = ((rbx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(r14.wrapping_add(0x24u64))) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 58; continue; } bb = 56; continue; }
            56 => { rcx = (crate::simd_ops::ru32(r14.wrapping_add(0x20u64))) as u32 as u64; rax = ((rcx.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(r14.wrapping_add(0x20u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(r14.wrapping_add(0x18u64))) as u64; rdx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 58; continue; } bb = 57; continue; }
            57 => { crate::simd_ops::wu64(rdx, rdi); fl_l = (crate::simd_ops::lk_add_u32(rdi.wrapping_add(0x14u64), (1) as u32)) as u32 as u64; bb = 58; continue; }
            58 => { rax = 0xffffffff; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rdi.wrapping_add(0x14u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((r13 as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 60; continue; } bb = 59; continue; }
            59 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 60; continue; }
            60 => { rax = ((r13 as u32)) as u32 as u64; bb = 61; continue; }
            61 => { r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0xd8u64))) as u64; r11 = rsp.wrapping_add(0xa0); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x40u64))) as u64; rbp = (crate::simd_ops::ru64(r11.wrapping_add(0x48u64))) as u64; rsp = r11; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            62 => { rax = 0x34; bb = 61; continue; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416227b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415d90a0;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rax = ((r8 as u32)) as u32 as u64;
    rcx = ((rcx).wrapping_add(0x8u64)) as u64;
    r8 = rdx;
    rdx = ((rax as u32)) as u32 as u64;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rax = 0x1;
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416227d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14158f5a0; const P1: u64 = 0x1415864b0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; r14 = 0; fl_l = 0; fl_r = 0; rbx = rdx.wrapping_add(0xb0); crate::simd_ops::wu64(rdx.wrapping_add(0x8u64), r14); rbp = rcx; crate::simd_ops::wu64(rdx.wrapping_add(0x30u64), r14); rcx = rbx; crate::simd_ops::wu64(rdx.wrapping_add(0x28u64), r14); rdi = r8; crate::simd_ops::wu64(rdx.wrapping_add(0x20u64), r14); rsi = rdx; crate::simd_ops::wu32(rdx.wrapping_add(0x7cu64), ((r14 as u32)) as u32); crate::simd_ops::wu64(rdx.wrapping_add(0x90u64), r14); crate::simd_ops::wu16(rdx.wrapping_add(0x98u64), (0xff00u16) as u16); crate::simd_ops::wu64(rdx.wrapping_add(0xa8u64), r14); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rbx.wrapping_add(0xb8u64), r14); rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rbx.wrapping_add(0xc0u64), r14); xmm0 = 0u128; crate::simd_ops::wu64(rbx.wrapping_add(0x220u64), r14); xmm1 = 0u128; crate::simd_ops::wu64(rbx.wrapping_add(0x228u64), r14); crate::simd_ops::wu64(rbx.wrapping_add(0xccu64), r14); crate::simd_ops::wu64(rbx.wrapping_add(0xd4u64), r14); crate::simd_ops::wu128(rbx, (xmm0) as u128); crate::simd_ops::wu128(rbx.wrapping_add(0x10u64), (xmm0) as u128); crate::simd_ops::wu128(rbx.wrapping_add(0x20u64), (xmm0) as u128); crate::simd_ops::wu128(rbx.wrapping_add(0x30u64), (xmm0) as u128); crate::simd_ops::wu128(rbx.wrapping_add(0x40u64), (xmm0) as u128); crate::simd_ops::wu128(rbx.wrapping_add(0x50u64), (xmm0) as u128); crate::simd_ops::wu64(rbx.wrapping_add(0x60u64), rax); crate::simd_ops::wu32(rbx.wrapping_add(0x70u64), ((r14 as u32)) as u32); crate::simd_ops::wu64(rbx.wrapping_add(0xa8u64), r14); crate::simd_ops::wu32(rbx.wrapping_add(0xb0u64), ((r14 as u32)) as u32); rcx = ((crate::simd_ops::ru8(rbx.wrapping_add(0xb6u64))) as u32) as u32 as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x7cu64), r14); rcx = (rcx & !0xffu64) | ((((((rcx as u8)) & (0x31u8))) as u8 as u64)); crate::simd_ops::wu64(rbx.wrapping_add(0x84u64), r14); rcx = (rcx & !0xffu64) | ((((((rcx as u8)) & (0xcfu8))) as u8 as u64)); crate::simd_ops::lk_and_u8(rbx.wrapping_add(0xb7u64), (0xf8u8) as u8); rcx = (rcx & !0xffu64) | ((((((rcx as u8)) & (0xfeu8))) as u8 as u64)); crate::simd_ops::wu8(rbx.wrapping_add(0xb6u64), ((rcx as u8)) as u8); crate::simd_ops::wu32(rbx.wrapping_add(0x8cu64), ((r14 as u32)) as u32); crate::simd_ops::wu16(rbx.wrapping_add(0xb4u64), ((r14 as u16)) as u16); crate::simd_ops::wu32(rbx.wrapping_add(0x78u64), (0x3f800000u32) as u32); crate::simd_ops::wu128(rbx.wrapping_add(0xc8u64), (xmm1) as u128); crate::simd_ops::wu32(rbx.wrapping_add(0xc0u64), ((r14 as u32)) as u32); crate::simd_ops::wu32(rbx.wrapping_add(0xd8u64), ((xmm0 as u32)) as u32); crate::simd_ops::wu64(rbx.wrapping_add(0x230u64), r14); crate::simd_ops::wu64(rbx.wrapping_add(0x238u64), r14); crate::simd_ops::lk_and_u8(rsi.wrapping_add(0x2f4u64), (0xfeu8) as u8); crate::simd_ops::wu32(rsi.wrapping_add(0x2f0u64), ((r14 as u32)) as u32); rax = ((crate::simd_ops::ru8(rsi.wrapping_add(0x2f8u64))) as u32) as u32 as u64; crate::simd_ops::wu32(rsi.wrapping_add(0x38u64), ((r14 as u32)) as u32); rax = (rax & !0xffu64) | ((((((rax as u8)) & (0xf3u8))) as u8 as u64)); crate::simd_ops::wu32(rsi, ((r14 as u32)) as u32); rax = (rax & !0xffu64) | ((((((rax as u8)) | (0x3u8))) as u8 as u64)); crate::simd_ops::wu8(rsi.wrapping_add(0x2f8u64), ((rax as u8)) as u8); rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x50u64))) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x40u64))) as u64; fl_l = rcx; fl_r = rcx; rax = rcx.wrapping_add(0x10); rax = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { r14 } else { rax }) as u64; crate::simd_ops::wu64(rsi.wrapping_add(0x8u64), rax); rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x48u64))) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; crate::simd_ops::wu64(rsi.wrapping_add(0x10u64), rcx); crate::simd_ops::wu32(rdi.wrapping_add(0x4u64), (0x4u32) as u32); crate::simd_ops::wu32(rdi, ((r14 as u32)) as u32); crate::simd_ops::wu64(rsi.wrapping_add(0x18u64), rdi); rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x48u64))) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x54u64))) as u32 as u64; crate::simd_ops::wu32(rsi.wrapping_add(0x34u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x48u64))) as u64; crate::simd_ops::wu64(rsi.wrapping_add(0x28u64), rax); rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x50u64))) as u32 as u64; crate::simd_ops::wu32(rsi.wrapping_add(0x30u64), ((rax as u32)) as u32); rbx = (crate::simd_ops::ru64(rcx.wrapping_add(0x40u64))) as u64; rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x20u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 2; continue; }
            2 => { fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::lk_add_u32(rbx, (1) as u32)) as u32 as u64; bb = 4; continue; }
            4 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; crate::simd_ops::wu64(rsi.wrapping_add(0x20u64), rbx); rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x48u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rcx = (crate::simd_ops::ru32(rax.wrapping_add(0x58u64))) as u32 as u64; rax = rsi; crate::simd_ops::lk_or_u8(rsi.wrapping_add(0x2f8u64), (0x8u8) as u8); crate::simd_ops::wu32(rsi.wrapping_add(0x80u64), ((rcx as u32)) as u32); crate::simd_ops::wu32(rsi.wrapping_add(0x84u64), ((r14 as u32)) as u32); crate::simd_ops::wu64(rsi.wrapping_add(0x88u64), r14); crate::simd_ops::wu32(rsi.wrapping_add(0x90u64), ((r14 as u32)) as u32); rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416229d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xc; const P1: u64 = 0x8; const P2: u64 = 0x141524340; const P3: u64 = 0x141524330; const P4: u64 = 0x10; const P5: u64 = 0xfffffffffffffff0; const P6: u64 = 0x1415242e0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbp = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; rbx = rcx; rax = (crate::simd_ops::ru64(rcx)) as u64; rbp = (((rbp as u32)).wrapping_add((rdx as u32))) as u32 as u64; rsi = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64; rcx = 0; fl_l = 0; fl_r = 0; rdx = ((rbp as u32)) as u32 as u64; rdx = ((rdx).wrapping_shl(4u32)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { r8 = rdx; rdx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 2; continue; }
            2 => { bb = 11; continue; }
            3 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 12; continue; }
            5 => { rax = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 6; continue; }
            6 => { fl_l = rax; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 11; continue; } bb = 7; continue; }
            7 => { fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 8; continue; }
            8 => { rcx = 0; fl_l = 0; fl_r = 0; bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = ((rcx.wrapping_add(P4)) as u64) as u64; xmm0 = (crate::simd_ops::ru128(rcx.wrapping_add(rax).wrapping_add(P5))) as u128; crate::simd_ops::wu128(rcx.wrapping_add(rdi).wrapping_add(P5), (xmm0) as u128); fl_l = rsi; fl_r = (0x1u64) as u64; rsi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 9; continue; } bb = 10; continue; }
            10 => { rdx = (crate::simd_ops::ru64(rbx)) as u64; rcx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 11; continue; }
            11 => { crate::simd_ops::wu64(rbx, rdi); rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); crate::simd_ops::wu32(rbx.wrapping_add(P0), ((rbp as u32)) as u32); bb = 12; continue; }
            12 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141622a80(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141524340; const P1: u64 = 0x141524330; const P2: u64 = 0x1415242e0;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbp = (crate::simd_ops::ru32(rcx.wrapping_add(0xcu64))) as u32 as u64; rbx = rcx; rax = (crate::simd_ops::ru64(rcx)) as u64; rbp = (((rbp as u32)).wrapping_add((rdx as u32))) as u32 as u64; rsi = (crate::simd_ops::ru32(rcx.wrapping_add(0x8u64))) as u32 as u64; rcx = 0; fl_l = 0; fl_r = 0; rdx = ((rbp as u32)) as u32 as u64; rdx = ((rdx).wrapping_shl(3u32)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { r8 = rdx; rdx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 2; continue; }
            2 => { bb = 25; continue; }
            3 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            5 => { rax = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 25; continue; } bb = 6; continue; }
            6 => { fl_l = rax; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 25; continue; } bb = 7; continue; }
            7 => { r11 = 0; fl_l = 0; fl_r = 0; fl_l = rsi; fl_r = (0x4u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 19; continue; } bb = 8; continue; }
            8 => { crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), r12); r10 = rsi.wrapping_sub(0x4); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), r14); rdx = rdi.wrapping_add(0x8); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), r15); r14 = 0x10; r15 = 0x8; r10 = ((r10).wrapping_shr(2u32)) as u64; r12 = (0xfffffffffffffff8u64) as u64; r15 = ((r15).wrapping_sub(rdi)) as u64; r14 = ((r14).wrapping_sub(rdi)) as u64; r12 = ((r12).wrapping_sub(rdi)) as u64; fl_l = ((r10).wrapping_add(1)) as u64; r10 = ((fl_l as u64)) as u64; r11 = ((r10.wrapping_mul(4)) as u64) as u64; bb = 9; continue; }
            9 => { rcx = rdx.wrapping_sub(0x8); r8 = rdx; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { crate::simd_ops::wu32(rcx, (0xff7fffffu32) as u32); crate::simd_ops::wu32(rdx.wrapping_sub(0x4u64), (0x7f7fffffu32) as u32); bb = 11; continue; }
            11 => { rax = (crate::simd_ops::ru64(rbx)) as u64; r9 = ((r12.wrapping_add(rdx)) as u64) as u64; xmm0 = ((crate::simd_ops::ru64(r9.wrapping_add(rax))) as u64) as u128; crate::simd_ops::wu64(rcx, ((xmm0 as u64)) as u64); fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 12; continue; }
            12 => { crate::simd_ops::wu32(rdx, (0xff7fffffu32) as u32); crate::simd_ops::wu32(r8.wrapping_add(0x4u64), (0x7f7fffffu32) as u32); bb = 13; continue; }
            13 => { rax = (crate::simd_ops::ru64(rbx)) as u64; r8 = rdx.wrapping_add(0x8); xmm0 = ((crate::simd_ops::ru64(rax.wrapping_add(r9).wrapping_add(0x8u64))) as u64) as u128; crate::simd_ops::wu64(rdx, ((xmm0 as u64)) as u64); fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 15; continue; } bb = 14; continue; }
            14 => { crate::simd_ops::wu32(r8, (0xff7fffffu32) as u32); crate::simd_ops::wu32(rdx.wrapping_add(0xcu64), (0x7f7fffffu32) as u32); bb = 15; continue; }
            15 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = ((r15.wrapping_add(rdx)) as u64) as u64; xmm0 = ((crate::simd_ops::ru64(rcx.wrapping_add(rax))) as u64) as u128; crate::simd_ops::wu64(r8, ((xmm0 as u64)) as u64); r8 = rdx.wrapping_add(0x10); fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 16; continue; }
            16 => { crate::simd_ops::wu32(r8, (0xff7fffffu32) as u32); crate::simd_ops::wu32(rdx.wrapping_add(0x14u64), (0x7f7fffffu32) as u32); bb = 17; continue; }
            17 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = ((r14.wrapping_add(rdx)) as u64) as u64; rdx = ((rdx).wrapping_add(0x20u64)) as u64; xmm0 = ((crate::simd_ops::ru64(rcx.wrapping_add(rax))) as u64) as u128; crate::simd_ops::wu64(r8, ((xmm0 as u64)) as u64); fl_l = r10; fl_r = (0x1u64) as u64; r10 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 9; continue; } bb = 18; continue; }
            18 => { r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; bb = 19; continue; }
            19 => { fl_l = r11; fl_r = rsi; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 24; continue; } bb = 20; continue; }
            20 => { rcx = ((rdi.wrapping_add(r11.wrapping_mul(8))) as u64) as u64; fl_l = rsi; fl_r = r11; rsi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; bb = 21; continue; }
            21 => { fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 23; continue; } bb = 22; continue; }
            22 => { crate::simd_ops::wu32(rcx, (0xff7fffffu32) as u32); crate::simd_ops::wu32(rcx.wrapping_add(0x4u64), (0x7f7fffffu32) as u32); bb = 23; continue; }
            23 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rax = ((rax).wrapping_sub(rdi)) as u64; xmm0 = ((crate::simd_ops::ru64(rax.wrapping_add(rcx))) as u64) as u128; crate::simd_ops::wu64(rcx, ((xmm0 as u64)) as u64); rcx = ((rcx).wrapping_add(0x8u64)) as u64; fl_l = rsi; fl_r = (0x1u64) as u64; rsi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 21; continue; } bb = 24; continue; }
            24 => { rdx = (crate::simd_ops::ru64(rbx)) as u64; rcx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 25; continue; }
            25 => { crate::simd_ops::wu64(rbx, rdi); rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); crate::simd_ops::wu32(rbx.wrapping_add(0xcu64), ((rbp as u32)) as u32); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141622c50(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141524340; const P1: u64 = 0x141524330; const P2: u64 = 0x1415242e0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x28u64)) as u64; r12 = (crate::simd_ops::ru32(rcx.wrapping_add(0xcu64))) as u32 as u64; rsi = rcx; rax = (crate::simd_ops::ru64(rcx)) as u64; r12 = (((r12 as u32)).wrapping_add((rdx as u32))) as u32 as u64; rbp = (crate::simd_ops::ru32(rcx.wrapping_add(0x8u64))) as u32 as u64; rcx = 0; fl_l = 0; fl_r = 0; rdx = ((r12 as u32)) as u32 as u64; rdx = ((rdx).wrapping_shl(5u32)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { r8 = rdx; rdx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r14 = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 2; continue; }
            2 => { bb = 19; continue; }
            3 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r14 = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x28u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            5 => { rax = (crate::simd_ops::ru64(rsi)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 19; continue; } bb = 6; continue; }
            6 => { fl_l = rax; fl_r = r14; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 19; continue; } bb = 7; continue; }
            7 => { fl_l = rbp; fl_r = rbp; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 18; continue; } bb = 8; continue; }
            8 => { crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rbx); rbx = r14.wrapping_add(0x1c); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), r13); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r15); r15 = (0xffffffffffffffe4u64) as u64; r15 = ((r15).wrapping_sub(r14)) as u64; r13 = 0; fl_l = 0; fl_r = 0; bb = 9; continue; }
            9 => { rdx = rbx.wrapping_sub(0x1c); fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { crate::simd_ops::wu64(rdx, r13); crate::simd_ops::wu32(rbx.wrapping_sub(0x14u64), ((r13 as u32)) as u32); crate::simd_ops::wu64(rbx.wrapping_sub(0xcu64), r13); crate::simd_ops::wu64(rbx.wrapping_sub(0x4u64), r13); bb = 11; continue; }
            11 => { rax = (crate::simd_ops::ru64(rsi)) as u64; rcx = ((r15.wrapping_add(rbx)) as u64) as u64; xmm0 = (crate::simd_ops::ru128(rcx.wrapping_add(rax))) as u128; crate::simd_ops::wu128(rdx, (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(rcx.wrapping_add(rax).wrapping_add(0x10u64))) as u128; crate::simd_ops::wu128(rdx.wrapping_add(0x10u64), (xmm1) as u128); rdi = (crate::simd_ops::ru64(rsi)) as u64; rdi = ((rdi).wrapping_add(rcx)) as u64; rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 12; continue; }
            12 => { rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rdi.wrapping_add(0x18u64), ((r13 as u32)) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rdi.wrapping_add(0x10u64), r13); crate::simd_ops::wu32(rdi.wrapping_add(0x1cu64), ((r13 as u32)) as u32); bb = 13; continue; }
            13 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 16; continue; } bb = 14; continue; }
            14 => { rcx = ((rcx).wrapping_add(0x10u64)) as u64; rax = 0xffffffff; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rcx.wrapping_add(0x14u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 16; continue; } bb = 15; continue; }
            15 => { rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 16; continue; }
            16 => { rbx = ((rbx).wrapping_add(0x20u64)) as u64; fl_l = rbp; fl_r = (0x1u64) as u64; rbp = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 9; continue; } bb = 17; continue; }
            17 => { r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; bb = 18; continue; }
            18 => { rdx = (crate::simd_ops::ru64(rsi)) as u64; rcx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 19; continue; }
            19 => { crate::simd_ops::wu64(rsi, r14); rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); crate::simd_ops::wu32(rsi.wrapping_add(0xcu64), ((r12 as u32)) as u32); rsp = ((rsp).wrapping_add(0x28u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141622db0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x28; const P1: u64 = 0x8; const P2: u64 = 0x4;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru32(rax.wrapping_add(P1))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 8; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rax)) as u64; rdx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 7; continue; } bb = 3; continue; }
            3 => { bb = 4; continue; }
            4 => { fl_l = ((xmm1 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rax)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 6; continue; } bb = 5; continue; }
            5 => { fl_l = ((xmm1 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rax.wrapping_add(P2))) as u32 as u64; if !(f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 8; continue; } bb = 6; continue; }
            6 => { rax = ((rax).wrapping_add(0x8u64)) as u64; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } bb = 7; continue; }
            7 => { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); return rax; }
            8 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141622df0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1434268d0; const P1: u64 = 0x14161f890; const P2: u64 = 0x1416227d0; const P3: u64 = 0x1415945c0; const P4: u64 = 0x1415bfb60; const P5: u64 = 0x1415c0d60; const P6: u64 = 0x14152ede0; const P7: u64 = 0x1415864b0; const P8: u64 = 0x143930f08; const P9: u64 = 0x1415ab5b0; const P10: u64 = 0x141529ed0; const P11: u64 = 0x141594b70;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0; let mut xmm7: u128 = 0; let mut xmm8: u128 = 0; let mut xmm9: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 127>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r11 = rsp; rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rbp = r11.wrapping_sub(0x2f8); rsp = ((rsp).wrapping_sub(0x3d8u64)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x50u64))) as u64; rbx = 0; fl_l = 0; fl_r = 0; rdi = (crate::simd_ops::ru64(rcx.wrapping_add(0x58u64))) as u64; rdx = ((rbx as u32)) as u32 as u64; fl_l = (crate::simd_ops::ru64(r8)) as u64; fl_r = rdx; r12 = rcx; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rax); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x48u64))) as u32 as u64; rdx = (rdx & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); rax = ((((rax as u32)) & (0xffffff24u32))) as u32 as u64; crate::simd_ops::wu128(r11.wrapping_sub(0x68u64), (xmm8) as u128); rdx = ((((rdx as u32)) & (0xffffff27u32))) as u32 as u64; xmm8 = (xmm3) as u128; rdx = ((((rdx as u32)) | ((rax as u32)))) as u32 as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x60u64))) as u32 as u64; rax = ((rax).wrapping_shl(5u32)) as u64; rdx = ((((rdx as u32)) | (0x24u32))) as u32 as u64; rax = ((rax).wrapping_add(rdi)) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x48u64), ((rdx as u32)) as u32); fl_l = rdi; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 50; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(r11.wrapping_sub(0x28u64), r13); crate::simd_ops::wu128(r11.wrapping_sub(0x48u64), (xmm6) as u128); crate::simd_ops::wu128(r11.wrapping_sub(0x58u64), (xmm7) as u128); xmm7 = ((crate::simd_ops::ru32(rbp.wrapping_add(0x320u64))) as u32) as u128; crate::simd_ops::wu128(r11.wrapping_sub(0x78u64), (xmm9) as u128); xmm9 = ((crate::simd_ops::ru32(P0)) as u32) as u128; crate::simd_ops::wu64(r11.wrapping_add(0x10u64), rsi); crate::simd_ops::wu64(r11.wrapping_sub(0x30u64), r14); crate::simd_ops::wu64(r11.wrapping_sub(0x38u64), r15); bb = 2; continue; }
            2 => { r13 = (crate::simd_ops::ru64(rdi)) as u64; fl_l = (crate::simd_ops::ru64(r13.wrapping_add(0x8u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 48; continue; } bb = 3; continue; }
            3 => { r8 = (crate::simd_ops::ru64(r13.wrapping_add(0x28u64))) as u64; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru32(r8.wrapping_add(0x8u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 12; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(r8)) as u64; r9 = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rax; fl_r = r9; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 10; continue; } bb = 6; continue; }
            6 => { bb = 7; continue; }
            7 => { fl_l = ((xmm8 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rax)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 9; continue; } bb = 8; continue; }
            8 => { fl_l = ((xmm8 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rax.wrapping_add(0x4u64))) as u32 as u64; if !(f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 11; continue; } bb = 9; continue; }
            9 => { rax = ((rax).wrapping_add(0x8u64)) as u64; fl_l = rax; fl_r = r9; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 7; continue; } bb = 10; continue; }
            10 => { rcx = (crate::simd_ops::ru32(r8.wrapping_add(0x8u64))) as u32 as u64; r14 = (r14 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 14; continue; }
            11 => { rax = (crate::simd_ops::ru64(r8)) as u64; r14 = (r14 & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rcx = (crate::simd_ops::ru32(r8.wrapping_add(0x8u64))) as u32 as u64; rdx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; bb = 16; continue; }
            12 => { r14 = (r14 & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 20; continue; } bb = 13; continue; }
            13 => { rcx = (crate::simd_ops::ru32(r8.wrapping_add(0x8u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 20; continue; } bb = 14; continue; }
            14 => { rax = (crate::simd_ops::ru64(r8)) as u64; rdx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 19; continue; } bb = 15; continue; }
            15 => { bb = 16; continue; }
            16 => { fl_l = ((xmm7 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rax)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 18; continue; } bb = 17; continue; }
            17 => { fl_l = ((xmm7 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rax.wrapping_add(0x4u64))) as u32 as u64; if !(f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 20; continue; } bb = 18; continue; }
            18 => { rax = ((rax).wrapping_add(0x8u64)) as u64; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 16; continue; } bb = 19; continue; }
            19 => { rsi = (rsi & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 21; continue; }
            20 => { rsi = (rsi & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 21; continue; }
            21 => { rcx = (crate::simd_ops::ru64(r12.wrapping_add(0x48u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 37; continue; } bb = 22; continue; }
            22 => { fl_l = ((r14 as u8)) as u8 as u64; fl_r = ((rsi as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 37; continue; } bb = 23; continue; }
            23 => { fl_l = ((r14 as u8)) as u8 as u64; fl_r = ((r14 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 34; continue; } bb = 24; continue; }
            24 => { fl_l = ((rsi as u8)) as u8 as u64; fl_r = ((rsi as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 37; continue; } bb = 25; continue; }
            25 => { fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; fl_r = 0x1; crate::simd_ops::wu32(rdi.wrapping_add(0x8u64), ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32); if (((fl_l as u32)) != ((fl_r as u32))) { bb = 37; continue; } bb = 26; continue; }
            26 => { r8 = rsp.wrapping_add(0x38); crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), ((rbx as u32)) as u32); rdx = rbp.wrapping_sub(0x80); crate::simd_ops::wu32(rsp.wrapping_add(0x3cu64), (0x4u32) as u32); rcx = r12; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru8(rbp.wrapping_add(0x18u64))) as u8 as u64; fl_r = (0x90u8) as u8 as u64; rbx = (crate::simd_ops::ru64(r13.wrapping_add(0x8u64))) as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 30; continue; } bb = 27; continue; }
            27 => { fl_l = (crate::simd_ops::ru8(rbp.wrapping_add(0x1du64))) as u8 as u64; fl_r = ((rsi as u8)) as u8 as u64; if (((fl_l as u8)) <= ((fl_r as u8))) { bb = 30; continue; } bb = 28; continue; }
            28 => { rax = ((crate::simd_ops::ru8(rbp.wrapping_add(0x278u64))) as u32) as u32 as u64; rcx = rbp.wrapping_sub(0x80); rax = (rax & !0xffu64) | ((((((rax as u8)) & (0x1u8))) as u8 as u64)); crate::simd_ops::wu8(rbp.wrapping_add(0x300u64), ((rax as u8)) as u8); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = ((rax as u32)) as u32 as u64; r9 = rbp.wrapping_add(0x300); rax = (crate::simd_ops::ru32(rbp.wrapping_sub(0x4cu64))) as u32 as u64; rdx = rbp.wrapping_add(0x18); crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rax as u32)) as u32); rcx = rbx; rax = (crate::simd_ops::ru64(rbp.wrapping_sub(0x70u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 32; continue; } bb = 29; continue; }
            29 => { rdx = ((crate::simd_ops::ru8(rbp.wrapping_add(0x278u64))) as u32) as u32 as u64; rax = (((rdx as u8)) as u32) as u32 as u64; rax = (rax & !0xffu64) | ((((((rax as u8)) ^ (crate::simd_ops::ru8(rbp.wrapping_add(0x300u64))))) as u8 as u64)); rax = (rax & !0xffu64) | ((((((rax as u8)) & (0x1u8))) as u8 as u64)); fl_l = ((((rdx as u8)) ^ ((rax as u8)))) as u8 as u64; fl_r = fl_l; rdx = (rdx & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); crate::simd_ops::wu8(rbp.wrapping_add(0x278u64), ((rdx as u8)) as u8); bb = 30; continue; }
            30 => { rdx = rbp.wrapping_sub(0x80); rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 32; continue; } bb = 31; continue; }
            31 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = rbp.wrapping_sub(0x80); rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x250u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 32; continue; }
            32 => { rcx = rbp.wrapping_add(0x30); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x60u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 37; continue; } bb = 33; continue; }
            33 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 37; continue; }
            34 => { fl_l = ((rsi as u8)) as u8 as u64; fl_r = ((rsi as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 37; continue; } bb = 35; continue; }
            35 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; rax = (((rax as u32)).wrapping_add(1)) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x8u64), ((rax as u32)) as u32); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 37; continue; } bb = 36; continue; }
            36 => { rcx = (crate::simd_ops::ru64(r12.wrapping_add(0x48u64))) as u64; rdx = rsp.wrapping_add(0x50); crate::simd_ops::wu16(rsp.wrapping_add(0x6fu64), (0x0u16) as u16); crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), rbx); crate::simd_ops::wu8(rsp.wrapping_add(0x6cu64), (0x0u8) as u8); crate::simd_ops::wu8(rsp.wrapping_add(0x6eu64), (0x0u8) as u8); crate::simd_ops::wu32(rsp.wrapping_add(0x50u64), ((rbx as u32)) as u32); rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rax); rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x18u64))) as u32 as u64; rcx = (crate::simd_ops::ru64(r13.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x60u64), ((rax as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x68u64), (0x4u32) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x64u64), ((rbx as u32)) as u32); crate::simd_ops::wu8(rsp.wrapping_add(0x6du64), (0x0u8) as u8); rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x80u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = (crate::simd_ops::ru64(r12.wrapping_add(0x48u64))) as u64; rdx = (crate::simd_ops::ru64(r13.wrapping_add(0x8u64))) as u64; rcx = (crate::simd_ops::ru64(P8)) as u64; r9 = (crate::simd_ops::ru32(r8.wrapping_add(0x18u64))) as u32 as u64; r8 = (crate::simd_ops::ru64(r8.wrapping_add(0x10u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 37; continue; }
            37 => { fl_l = (crate::simd_ops::ru64(r13.wrapping_add(0x10u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 39; continue; } bb = 38; continue; }
            38 => { rax = (crate::simd_ops::ru32(r13.wrapping_add(0x20u64))) as u32 as u64; r8 = rsp.wrapping_add(0x30); xmm1 = (xmm7) as u128; crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((rax as u32)) as u32); rcx = r13.wrapping_add(0x10); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P10, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x30u64))) as u32 as u64; xmm6 = (xmm0) as u128; crate::simd_ops::wu32(r13.wrapping_add(0x20u64), ((rax as u32)) as u32); bb = 40; continue; }
            39 => { xmm6 = 0u128; bb = 40; continue; }
            40 => { fl_l = ((r14 as u8)) as u8 as u64; fl_r = ((rsi as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 44; continue; } bb = 41; continue; }
            41 => { fl_l = ((xmm6 as u32)) as u32 as u64; fl_r = ((xmm9 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) != f32::from_bits((fl_r as u32)) && !f32::from_bits((fl_l as u32)).is_nan() && !f32::from_bits((fl_r as u32)).is_nan()) { bb = 44; continue; } bb = 42; continue; }
            42 => { fl_l = (crate::simd_ops::ru64(r13.wrapping_add(0x10u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 44; continue; } bb = 43; continue; }
            43 => { rax = (crate::simd_ops::ru32(r13.wrapping_add(0x20u64))) as u32 as u64; r8 = rsp.wrapping_add(0x34); xmm1 = (xmm8) as u128; crate::simd_ops::wu32(rsp.wrapping_add(0x34u64), ((rax as u32)) as u32); rcx = r13.wrapping_add(0x10); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P10, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = ((xmm9 as u32)) as u32 as u64; rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x34u64))) as u32 as u64; crate::simd_ops::wu32(r13.wrapping_add(0x20u64), ((rax as u32)) as u32); if (f32::from_bits((fl_l as u32)) == f32::from_bits((fl_r as u32)) || f32::from_bits((fl_l as u32)).is_nan() || f32::from_bits((fl_r as u32)).is_nan()) { bb = 47; continue; } bb = 44; continue; }
            44 => { rbx = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x18u64))) as u32 as u64; rax = ((rbx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 47; continue; } bb = 45; continue; }
            45 => { bb = 46; continue; }
            46 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; r8 = rsp.wrapping_add(0x40); r9 = (r9 & !0xffu64) | (((0x1u8) as u8 as u64)); xmm1 = (xmm6) as u128; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P11, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x18u64))) as u32 as u64; rbx = ((rbx).wrapping_add(0x8u64)) as u64; rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; rax = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 46; continue; } bb = 47; continue; }
            47 => { rbx = 0; fl_l = 0; fl_r = 0; bb = 48; continue; }
            48 => { rax = (crate::simd_ops::ru32(r12.wrapping_add(0x60u64))) as u32 as u64; rdi = ((rdi).wrapping_add(0x20u64)) as u64; rax = ((rax).wrapping_shl(5u32)) as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(r12.wrapping_add(0x58u64)))) as u64; fl_l = rdi; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 49; continue; }
            49 => { xmm9 = (crate::simd_ops::ru128(rsp.wrapping_add(0x380u64))) as u128; xmm7 = (crate::simd_ops::ru128(rsp.wrapping_add(0x3a0u64))) as u128; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x3b0u64))) as u128; r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x3c0u64))) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x3c8u64))) as u64; r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x3d0u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x408u64))) as u64; bb = 50; continue; }
            50 => { xmm8 = (crate::simd_ops::ru128(rsp.wrapping_add(0x390u64))) as u128; rsp = ((rsp).wrapping_add(0x3d8u64)) as u64; r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416231d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x30; const P1: u64 = 0x38; const P2: u64 = 0x8; const P3: u64 = 0x70; const P4: u64 = 0xd0; const P5: u64 = 0x180;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rsi = (((r8 as u8)) as u32) as u32 as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64; rbp = (((rdx as u8)) as u32) as u32 as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rdi = rcx; rax = ((rax).wrapping_add(rbx)) as u64; fl_l = rbx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 6; continue; } bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(P3))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru16(rax.wrapping_add(P4))) as u16 as u64; fl_r = (0x0u16) as u16 as u64; if ((((fl_l as u16)) as i16) <= (((fl_r as u16)) as i16)) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rcx)) as u64; r8 = (((rsi as u8)) as u32) as u32 as u64; rdx = (((rbp as u8)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P5))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(P1))) as u32 as u64; rbx = ((rbx).wrapping_add(0x10u64)) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(rdi.wrapping_add(P0)))) as u64; fl_l = rbx; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 6; continue; }
            6 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141623260(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415bd1d0;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rdx = 0x5;
    return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141623270(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415aeaa0;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = (crate::simd_ops::ru64(rcx.wrapping_add(0x58u64))) as u64; r10 = r8; r9 = (crate::simd_ops::ru32(rcx.wrapping_add(0x60u64))) as u32 as u64; rbp = rdx; r9 = ((r9).wrapping_shl(5u32)) as u64; r9 = ((r9).wrapping_add(rbx)) as u64; fl_l = rbx; fl_r = r9; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 15; continue; } bb = 1; continue; }
            1 => { bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rbx)) as u64; r8 = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { fl_l = r8; fl_r = r10; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 6; continue; } bb = 4; continue; }
            4 => { rbx = ((rbx).wrapping_add(0x20u64)) as u64; fl_l = rbx; fl_r = r9; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 5; continue; }
            5 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            6 => { rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rsi); rsi = (crate::simd_ops::ru64(rcx.wrapping_add(0x50u64))) as u64; fl_l = rbp; fl_r = rbp; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), r14); r14 = rdx.wrapping_add(0x18); r14 = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rax } else { r14 }) as u64; rdi = (crate::simd_ops::ru64(rsi.wrapping_add(0x20u64))) as u64; rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x28u64))) as u32 as u64; rcx = ((rdi.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = rdi; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 8; continue; } bb = 7; continue; }
            7 => { rdx = (crate::simd_ops::ru64(rdi)) as u64; rcx = r14; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru32(rsi.wrapping_add(0x28u64))) as u32 as u64; rdi = ((rdi).wrapping_add(0x8u64)) as u64; rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x20u64))) as u64; rcx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rdi; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 7; continue; } bb = 8; continue; }
            8 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x18u64))) as u32 as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rcx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 15; continue; } bb = 9; continue; }
            9 => { fl_l = (crate::simd_ops::ru64(rax)) as u64; fl_r = rbp; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 12; continue; } bb = 10; continue; }
            10 => { rax = ((rax).wrapping_add(0x8u64)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 9; continue; } bb = 11; continue; }
            11 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            12 => { rcx = ((rcx).wrapping_add(0xfffffffffffffff8u64)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 14; continue; } bb = 13; continue; }
            13 => { rcx = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::wu64(rax, rcx); bb = 14; continue; }
            14 => { fl_l = (crate::simd_ops::lk_sub_u32(rbx.wrapping_add(0x18u64), (1) as u32)) as u32 as u64; bb = 15; continue; }
            15 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141623370(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141623520; const P1: u64 = 0x1416229d0; const P2: u64 = 0x141524330; const P3: u64 = 0x141623b90; const P4: u64 = 0x141541e90; const P5: u64 = 0x1415242e0;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x30u64))) as u64; r13 = rcx; rbp = (crate::simd_ops::ru32(rcx.wrapping_add(0x38u64))) as u32 as u64; r15 = ((r9 as u32)) as u32 as u64; rcx = ((rbp as u32)) as u32 as u64; r12 = r8; rcx = ((rcx).wrapping_shl(4u32)) as u64; rdi = ((rdx as u32)) as u32 as u64; rcx = ((rcx).wrapping_add(rax)) as u64; r10 = rax; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 12; continue; } bb = 1; continue; }
            1 => { bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru32(r10)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 8; continue; } bb = 3; continue; }
            3 => { r10 = ((r10).wrapping_add(0x10u64)) as u64; fl_l = r10; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 4; continue; }
            4 => { rbx = r13.wrapping_add(0x30); bb = 5; continue; }
            5 => { fl_l = (crate::simd_ops::ru32(rax)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 13; continue; } bb = 6; continue; }
            6 => { rax = ((rax).wrapping_add(0x10u64)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 5; continue; } bb = 7; continue; }
            7 => { bb = 13; continue; }
            8 => { rbx = (crate::simd_ops::ru64(r10.wrapping_add(0x8u64))) as u64; bb = 9; continue; }
            9 => { r8 = ((r15 as u32)) as u32 as u64; rdx = r12; rcx = rbx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rdi = ((rax as u32)) as u32 as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { r9 = (crate::simd_ops::ru64(rcx)) as u64; r8 = 0; fl_l = 0; fl_r = 0; rdx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(r9.wrapping_add(0x180u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 11; continue; }
            11 => { rax = ((rdi as u32)) as u32 as u64; bb = 29; continue; }
            12 => { rbx = r13.wrapping_add(0x30); bb = 13; continue; }
            13 => { rdx = 0; fl_l = 0; fl_r = 0; rsi = rax.wrapping_add(0x8); fl_l = rax; fl_r = rcx; rsi = (if (((fl_l as u64)) == ((fl_r as u64))) { rdx } else { rsi }) as u64; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 22; continue; } bb = 14; continue; }
            14 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0xcu64))) as u32 as u64; fl_l = rbp; fl_r = rax; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 19; continue; } bb = 15; continue; }
            15 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 17; continue; } bb = 16; continue; }
            16 => { rdx = ((rsi.wrapping_add(0x1u64)) as u32) as u32 as u64; bb = 18; continue; }
            17 => { rdx = ((rax as u32)) as u32 as u64; rdx = (((rdx as u32)).wrapping_shr(1u32)) as u32 as u64; fl_l = (((rdx as u32)).wrapping_add((rax as u32))) as u32 as u64; fl_r = fl_l; rdx = ((fl_l as u32)) as u32 as u64; bb = 18; continue; }
            18 => { rcx = rbx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 28; continue; } bb = 19; continue; }
            19 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0xcu64))) as u32 as u64; fl_l = rbp; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 28; continue; } bb = 20; continue; }
            20 => { rcx = (crate::simd_ops::ru32(r13.wrapping_add(0x38u64))) as u32 as u64; rbp = 0; fl_l = 0; fl_r = 0; rsi = ((rbp as u32)) as u32 as u64; rax = ((rcx.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(r13.wrapping_add(0x38u64), ((rax as u32)) as u32); rax = ((rcx as u32)) as u32 as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(rbx))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 23; continue; } bb = 21; continue; }
            21 => { crate::simd_ops::wu32(rax, ((rdi as u32)) as u32); rsi = rax.wrapping_add(0x8); bb = 23; continue; }
            22 => { rbp = 0; fl_l = 0; fl_r = 0; bb = 23; continue; }
            23 => { fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 28; continue; } bb = 24; continue; }
            24 => { rdx = 0x30; rcx = ((rdx.wrapping_sub(0x20u64)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rbx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 27; continue; } bb = 25; continue; }
            25 => { crate::simd_ops::wu32(rax, ((rbp as u32)) as u32); rdx = r13; crate::simd_ops::wu64(rax.wrapping_add(0x8u64), rbp); rcx = rax; crate::simd_ops::wu64(rax.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rax.wrapping_add(0x18u64), (0x0u64) as u64); crate::simd_ops::wu32(rax.wrapping_add(0x20u64), (0x1u32) as u32); crate::simd_ops::wu64(rax.wrapping_add(0x28u64), rbp); crate::simd_ops::wu64(rsi, rax); crate::simd_ops::wu32(rax, ((rdi as u32)) as u32); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rsi = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 9; continue; } bb = 26; continue; }
            26 => { rdx = ((rdi as u32)) as u32 as u64; rcx = r13.wrapping_add(0x30); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rdx = rbx; rcx = 0x10; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rax = ((rsi as u32)) as u32 as u64; bb = 29; continue; }
            27 => { crate::simd_ops::wu64(rsi, rbp); bb = 28; continue; }
            28 => { rdx = ((rdi as u32)) as u32 as u64; rcx = r13.wrapping_add(0x30); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rax = 0x34; bb = 29; continue; }
            29 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141623520(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415242e0; const P1: u64 = 0x141524330; const P2: u64 = 0x14152a3c0; const P3: u64 = 0x143427174; const P4: u64 = 0x141622a80;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0; let mut xmm7: u128 = 0; let mut xmm8: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 15>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x50u64)) as u64; r13 = 0; fl_l = 0; fl_r = 0; r12 = 0x1; rsi = ((r8 as u32)) as u32 as u64; r14 = rdx; rbp = rcx; rdi = ((r12 as u32)) as u32 as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 41; continue; } bb = 1; continue; }
            1 => { rdx = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rcx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 3; continue; }
            3 => { crate::simd_ops::wu64(rbp.wrapping_add(0x10u64), r13); crate::simd_ops::wu64(rbp.wrapping_add(0x18u64), r13); crate::simd_ops::wu32(rbp.wrapping_add(0x20u64), ((r12 as u32)) as u32); fl_l = r14; fl_r = r14; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 40; continue; } bb = 4; continue; }
            4 => { rax = ((rsi.wrapping_add(0x2u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x20u64), ((r12 as u32)) as u32); rdx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u32) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x18u64), ((rax as u32)) as u32); rdx = (((rdx as u32)).wrapping_shl(2u32)) as u32 as u64; rcx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rbp.wrapping_add(0x10u64), rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 39; continue; } bb = 5; continue; }
            5 => { r9 = 0; fl_l = 0; fl_r = 0; rcx = rbp.wrapping_add(0x10); r8 = ((rsi as u32)) as u32 as u64; rdx = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x28u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { crate::simd_ops::wu32(rax.wrapping_add(0x8u64), ((r13 as u32)) as u32); bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x18u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 38; continue; } bb = 8; continue; }
            8 => { rax = (((rax as u32)).wrapping_add(0xfffffffeu32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 38; continue; } bb = 9; continue; }
            9 => { fl_l = (crate::simd_ops::ru64(rbp.wrapping_add(0x28u64))) as u64; fl_r = r13; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 12; continue; } bb = 10; continue; }
            10 => { rdx = 0x10; rcx = ((rdx.wrapping_sub(0xeu64)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 20; continue; } bb = 11; continue; }
            11 => { crate::simd_ops::wu64(rax, r13); crate::simd_ops::wu64(rax.wrapping_add(0x8u64), r13); crate::simd_ops::wu64(rbp.wrapping_add(0x28u64), rax); bb = 12; continue; }
            12 => { crate::simd_ops::wu64(rsp.wrapping_add(0x80u64), r15); rsi = ((r13 as u32)) as u32 as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm6) as u128); r15 = (r15 & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm8) as u128); xmm6 = 0u128; xmm8 = ((crate::simd_ops::ru32(P3)) as u32) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm7) as u128); bb = 13; continue; }
            13 => { r8 = (crate::simd_ops::ru32(rbp.wrapping_add(0x18u64))) as u32 as u64; rdx = ((r8.wrapping_sub(0x2u64)) as u32) as u32 as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 15; continue; } bb = 14; continue; }
            14 => { rdx = 0xfffffffe; bb = 15; continue; }
            15 => { fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; rax = ((r8.wrapping_sub(0x2u64)) as u32) as u32 as u64; rax = (if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { (r13 as u32) } else { (rax as u32) }) as u32 as u64; fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 30; continue; } bb = 16; continue; }
            16 => { r9 = (crate::simd_ops::ru64(rbp.wrapping_add(0x10u64))) as u64; r11 = ((rsi.wrapping_add(0x2u64)) as u32) as u32 as u64; rax = ((r11.wrapping_sub(0x1u64)) as u32) as u32 as u64; r10 = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; xmm0 = ((crate::simd_ops::ru32(r9.wrapping_add(r10.wrapping_mul(4)).wrapping_add(0x4u64))) as u32) as u128; fl_l = ((r15 as u8)) as u8 as u64; fl_r = ((r15 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 21; continue; } bb = 17; continue; }
            17 => { fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = ((xmm6 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 19; continue; } bb = 18; continue; }
            18 => { fl_l = (crate::simd_ops::ru32(r9.wrapping_add(r10.wrapping_mul(4)).wrapping_add(0x8u64))) as u32 as u64; fl_r = 0x9; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 29; continue; } bb = 19; continue; }
            19 => { xmm8 = ((crate::simd_ops::ru32(r9.wrapping_add(r10.wrapping_mul(4)))) as u32) as u128; r15 = (((r12 as u8)) as u32) as u32 as u64; fl_l = (((rsi as u32)).wrapping_add(1)) as u32 as u64; rsi = ((fl_l as u32)) as u32 as u64; bb = 13; continue; }
            20 => { crate::simd_ops::wu64(rbp.wrapping_add(0x28u64), r13); rdi = 0x34; bb = 44; continue; }
            21 => { fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = ((xmm6 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 29; continue; } bb = 22; continue; }
            22 => { fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; rdx = (if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { (r13 as u32) } else { (rdx as u32) }) as u32 as u64; rdx = (((rdx as u32)).wrapping_sub(1)) as u32 as u64; fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 24; continue; } bb = 23; continue; }
            23 => { rcx = ((r11.wrapping_add(r11.wrapping_mul(2))) as u64) as u64; fl_l = ((xmm6 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(r9.wrapping_add(rcx.wrapping_mul(4)).wrapping_add(0x4u64))) as u32 as u64; if !(f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 29; continue; } bb = 24; continue; }
            24 => { rdi = (crate::simd_ops::ru64(rbp.wrapping_add(0x28u64))) as u64; r15 = (r15 & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; xmm7 = ((crate::simd_ops::ru32(r9.wrapping_add(r10.wrapping_mul(4)))) as u32) as u128; r14 = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; rax = (crate::simd_ops::ru32(rdi.wrapping_add(0xcu64))) as u32 as u64; fl_l = ((r14 as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 26; continue; } bb = 25; continue; }
            25 => { rdx = ((rax as u32)) as u32 as u64; rcx = rdi; rdx = (((rdx as u32)).wrapping_shr(1u32)) as u32 as u64; rdx = (((rdx as u32)).wrapping_add((rax as u32))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; rdx = (if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { (r12 as u32) } else { (rdx as u32) }) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 36; continue; } bb = 26; continue; }
            26 => { fl_l = ((r14 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rdi.wrapping_add(0xcu64))) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 36; continue; } bb = 27; continue; }
            27 => { rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; rax = (crate::simd_ops::ru64(rdi)) as u64; rdx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; rax = ((rcx.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x8u64), ((rax as u32)) as u32); fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 36; continue; } bb = 28; continue; }
            28 => { crate::simd_ops::wu32(rdx, (0xff7fffffu32) as u32); crate::simd_ops::wu32(rdx.wrapping_add(0x4u64), (0x7f7fffffu32) as u32); crate::simd_ops::wu32(rdx, ((xmm8 as u32)) as u32); crate::simd_ops::wu32(rdx.wrapping_add(0x4u64), ((xmm7 as u32)) as u32); bb = 29; continue; }
            29 => { fl_l = (((rsi as u32)).wrapping_add(1)) as u32 as u64; rsi = ((fl_l as u32)) as u32 as u64; bb = 13; continue; }
            30 => { fl_l = ((r15 as u8)) as u8 as u64; fl_r = ((r15 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 37; continue; } bb = 31; continue; }
            31 => { rbx = (crate::simd_ops::ru64(rbp.wrapping_add(0x28u64))) as u64; rax = ((r8.wrapping_sub(0x2u64)) as u32) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x10u64))) as u64; rdi = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; xmm6 = ((crate::simd_ops::ru32(rax.wrapping_add(rcx.wrapping_mul(4)))) as u32) as u128; rax = (crate::simd_ops::ru32(rbx.wrapping_add(0xcu64))) as u32 as u64; fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 33; continue; } bb = 32; continue; }
            32 => { rdx = ((rax as u32)) as u32 as u64; rcx = rbx; rdx = (((rdx as u32)).wrapping_shr(1u32)) as u32 as u64; rdx = (((rdx as u32)).wrapping_add((rax as u32))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; rdx = (if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { (r12 as u32) } else { (rdx as u32) }) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 36; continue; } bb = 33; continue; }
            33 => { fl_l = ((rdi as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rbx.wrapping_add(0xcu64))) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 36; continue; } bb = 34; continue; }
            34 => { rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; rax = ((rcx.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x8u64), ((rax as u32)) as u32); fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 36; continue; } bb = 35; continue; }
            35 => { crate::simd_ops::wu32(rdx, (0xff7fffffu32) as u32); crate::simd_ops::wu32(rdx.wrapping_add(0x4u64), (0x7f7fffffu32) as u32); crate::simd_ops::wu32(rdx, ((xmm8 as u32)) as u32); crate::simd_ops::wu32(rdx.wrapping_add(0x4u64), ((xmm6 as u32)) as u32); bb = 37; continue; }
            36 => { r12 = 0x34; bb = 37; continue; }
            37 => { xmm7 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x40u64))) as u128; r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; xmm8 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; bb = 38; continue; }
            38 => { rdi = ((r12 as u32)) as u32 as u64; bb = 44; continue; }
            39 => { crate::simd_ops::wu32(rbp.wrapping_add(0x18u64), ((r13 as u32)) as u32); rdi = 0x34; bb = 41; continue; }
            40 => { rdi = 0x1f; bb = 41; continue; }
            41 => { rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x10u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 43; continue; } bb = 42; continue; }
            42 => { rcx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rbp.wrapping_add(0x10u64), r13); bb = 43; continue; }
            43 => { crate::simd_ops::wu64(rbp.wrapping_add(0x10u64), r13); crate::simd_ops::wu64(rbp.wrapping_add(0x18u64), r13); crate::simd_ops::wu32(rbp.wrapping_add(0x20u64), ((r12 as u32)) as u32); bb = 44; continue; }
            44 => { r11 = rsp.wrapping_add(0x50); rax = ((rdi as u32)) as u32 as u64; rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x38u64))) as u64; rbp = (crate::simd_ops::ru64(r11.wrapping_add(0x40u64))) as u64; rsp = r11; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141623820(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1416219c0; const P1: u64 = 0x1416231d0; const P2: u64 = 0x1416229d0; const P3: u64 = 0x141623370;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; crate::simd_ops::lk_add_u64(rdx, (0x4u64) as u64); r14 = r8; rbx = rdx; r8 = rcx; r15 = rcx; rdx = r14; rcx = rbx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 13; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rsi); rdx = (crate::simd_ops::ru32(rax)) as u32 as u64; rax = ((rax).wrapping_add(0x4u64)) as u64; crate::simd_ops::wu64(rbx, rax); r8 = ((crate::simd_ops::ru8(rax)) as u32) as u32 as u64; rax = ((rax).wrapping_add(1)) as u64; crate::simd_ops::wu64(rbx, rax); fl_l = (crate::simd_ops::ru8(r15.wrapping_add(0x4cu64))) as u8 as u64; fl_r = ((r8 as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 3; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru32(r15.wrapping_add(0x48u64))) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu32(rsp.wrapping_add(0x48u64), ((rdx as u32)) as u32); rcx = r15; crate::simd_ops::wu8(rsp.wrapping_add(0x4cu64), ((r8 as u8)) as u8); rdx = 0; fl_l = 0; fl_r = 0; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; r8 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(r15.wrapping_add(0x48u64), rax); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rsi = 0x1; rbp = (crate::simd_ops::ru32(rax)) as u32 as u64; rax = ((rax).wrapping_add(0x4u64)) as u64; crate::simd_ops::wu64(rbx, rax); fl_l = ((rbp as u32)) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 16; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru32(r15.wrapping_add(0x3cu64))) as u32 as u64; rcx = r15.wrapping_add(0x30); fl_l = ((rbp as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rdx = ((rbp as u32)) as u32 as u64; rdx = (((rdx as u32)).wrapping_sub((rax as u32))) as u32 as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; rcx = 0x34; rcx = (if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { (rsi as u32) } else { (rcx as u32) }) as u32 as u64; rsi = ((rcx as u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 15; continue; } bb = 7; continue; }
            7 => { crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rdi); rdi = 0; fl_l = 0; fl_r = 0; fl_l = ((rbp as u32)) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 14; continue; } bb = 8; continue; }
            8 => { bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = r15; rdx = (crate::simd_ops::ru32(rax)) as u32 as u64; rax = ((rax).wrapping_add(0x4u64)) as u64; crate::simd_ops::wu64(rbx, rax); rsi = (crate::simd_ops::ru32(rax)) as u32 as u64; r8 = rax.wrapping_add(0x4); r9 = ((rsi as u32)) as u32 as u64; crate::simd_ops::wu64(rbx, r8); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 11; continue; } bb = 10; continue; }
            10 => { rcx = ((((rsi as u32)) as i32).wrapping_mul((0xfffffff4u32) as i32) as u32) as u32 as u64; rdx = ((rsi.wrapping_add(rsi.wrapping_mul(2))) as u64) as u64; rdx = ((rdx).wrapping_shl(2u32)) as u64; rdi = (((rdi as u32)).wrapping_add(1)) as u32 as u64; crate::simd_ops::lk_add_u64(rbx, (rdx) as u64); crate::simd_ops::lk_add_u32(r14, ((rcx as u32)) as u32); fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 9; continue; } bb = 11; continue; }
            11 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; bb = 12; continue; }
            12 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; bb = 13; continue; }
            13 => { rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            14 => { rax = ((rsi as u32)) as u32 as u64; bb = 11; continue; }
            15 => { rax = ((rcx as u32)) as u32 as u64; bb = 12; continue; }
            16 => { rax = ((rsi as u32)) as u32 as u64; bb = 12; continue; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141623960(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x40; const P1: u64 = 0x30; const P2: u64 = 0x38; const P3: u64 = 0x8; const P4: u64 = 0x1415bfa60; const P5: u64 = 0x14161f8a0; const P6: u64 = 0x141623b90;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; fl_r = (0x0u64) as u64; rdi = rcx; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbp); rbp = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 8; continue; } bb = 1; continue; }
            1 => { rbx = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(P2))) as u32 as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rax = ((rax).wrapping_add(rbx)) as u64; fl_l = rbx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 8; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), r14); r14 = 0; fl_l = 0; fl_r = 0; bb = 3; continue; }
            3 => { rsi = (crate::simd_ops::ru64(rbx.wrapping_add(P3))) as u64; rcx = (crate::simd_ops::ru64(rsi.wrapping_add(P3))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 4; continue; }
            4 => { rdx = rdi; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { r9 = (crate::simd_ops::ru64(rsi.wrapping_add(P3))) as u64; r8 = rsi; crate::simd_ops::wu64(rsi.wrapping_add(P3), r14); rdx = rdi; rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P0))) as u64; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(P2))) as u32 as u64; rbx = ((rbx).wrapping_add(0x10u64)) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(rdi.wrapping_add(P1)))) as u64; fl_l = rbx; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 7; continue; }
            7 => { r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; bb = 8; continue; }
            8 => { crate::simd_ops::wu64(rdi.wrapping_add(P0), rbp); fl_l = rbp; fl_r = rbp; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 9; continue; }
            9 => { rbx = (crate::simd_ops::ru64(rdi.wrapping_add(P1))) as u64; rax = (crate::simd_ops::ru32(rdi.wrapping_add(P2))) as u32 as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rax = ((rax).wrapping_add(rbx)) as u64; fl_l = rbx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 12; continue; } bb = 10; continue; }
            10 => { bb = 11; continue; }
            11 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P3))) as u64; rdx = rdi; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rdi.wrapping_add(P2))) as u32 as u64; rbx = ((rbx).wrapping_add(0x10u64)) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(rdi.wrapping_add(P1)))) as u64; fl_l = rbx; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 11; continue; } bb = 12; continue; }
            12 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141623a40(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x40; const P1: u64 = 0x20; const P2: u64 = 0x18; const P3: u64 = 0x143930f70; const P4: u64 = 0x1415d9620; const P5: u64 = 0x4c; const P6: u64 = 0x48; const P7: u64 = 0x1415d9820;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x50u64)) as u64;
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rdi = r8;
    crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax);
    r8 = ((rcx.wrapping_add(P1)) as u64) as u64;
    rax = ((rcx.wrapping_add(P2)) as u64) as u64;
    crate::simd_ops::wu32(rsp.wrapping_add(0x40u64), (0x0u32) as u32);
    rsi = rdx;
    crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rax);
    rbx = rcx;
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi);
    rcx = (crate::simd_ops::ru64(P3)) as u64;
    r9 = rdx;
    rdx = rsp.wrapping_add(0x30);
    rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp);
    fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(P5))) as u8 as u64; fl_r = (0x4u8) as u8 as u64;
    if !((((fl_l as u8)) != ((fl_r as u8)))) { r8 = (crate::simd_ops::ru32(rbx.wrapping_add(P6))) as u32 as u64; rdx = rsp.wrapping_add(0x30); rcx = (crate::simd_ops::ru64(P3)) as u64; r9 = rsi; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64;
    rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64;
    rsp = ((rsp).wrapping_add(0x50u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141623ad0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x30; const P1: u64 = 0x38; const P2: u64 = 0x8; const P3: u64 = 0x1415bfa60; const P4: u64 = 0x40; const P5: u64 = 0x14161f8a0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rbx = rcx; r8 = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64; r8 = ((r8).wrapping_shl(4u32)) as u64; r8 = ((r8).wrapping_add(rax)) as u64; fl_l = rax; fl_r = r8; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 10; continue; } bb = 1; continue; }
            1 => { bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru32(rax)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 5; continue; } bb = 3; continue; }
            3 => { rax = ((rax).wrapping_add(0x10u64)) as u64; fl_l = rax; fl_r = r8; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 4; continue; }
            4 => { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            5 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rdi); rdi = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P2))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 6; continue; }
            6 => { fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 7; continue; }
            7 => { rdx = rbx; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { r9 = (crate::simd_ops::ru64(rdi.wrapping_add(P2))) as u64; r8 = rdi; crate::simd_ops::wu64(rdi.wrapping_add(P2), (0x0u64) as u64); rdx = rbx; rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P4))) as u64; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); bb = 9; continue; }
            9 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; bb = 10; continue; }
            10 => { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141623b50(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x30; const P1: u64 = 0x38; const P2: u64 = 0x8; const P3: u64 = 0x141623b90;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r8 = rcx; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rax = (crate::simd_ops::ru32(r8.wrapping_add(P1))) as u32 as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rax = ((rax).wrapping_add(rcx)) as u64; fl_l = rcx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 5; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru32(rcx)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 4; continue; } bb = 2; continue; }
            2 => { rcx = ((rcx).wrapping_add(0x10u64)) as u64; fl_l = rcx; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 3; continue; }
            3 => { return rax; }
            4 => { rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64; rdx = r8; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            5 => { return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141623b90(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143930f88; const P1: u64 = 0x1426452b0; const P2: u64 = 0x1426452a8; const P3: u64 = 0x142645560; const P4: u64 = 0x1415bf3c0; const P5: u64 = 0x14161f8a0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; fl_r = (0x0u64) as u64; r13 = rdx; r12 = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = 0x1; rsp = ((rsp).wrapping_add(0x30u64)) as u64; r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            2 => { crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rbx); rbp = 0x1; rbx = (crate::simd_ops::ru64(P0)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), r14); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r15); r15 = (crate::simd_ops::ru32(rcx)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rsi); bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 13; continue; } bb = 4; continue; }
            4 => { rcx = ((rax.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; let cx_src: u32 = (rcx as u32); let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u32(rbx, fl_l as u32, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u32)) as u32 as u64; } if (((fl_l as u32)) != ((fl_r as u32))) { bb = 3; continue; } bb = 5; continue; }
            5 => { rsi = rbx.wrapping_add(0x8); bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x48u64))) as u32 as u64; r14 = 0; fl_l = 0; fl_r = 0; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 15; continue; } bb = 7; continue; }
            7 => { rdx = 0; fl_l = 0; fl_r = 0; rax = ((r15 as u32)) as u32 as u64; let d = ((rcx as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x40u64))) as u64; rdi = (crate::simd_ops::ru64(rax.wrapping_add(rdx.wrapping_mul(8)))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 8; continue; }
            8 => { fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x10u64))) as u32 as u64; fl_r = ((r15 as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 11; continue; } bb = 9; continue; }
            9 => { rdi = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 8; continue; } bb = 10; continue; }
            10 => { rdi = r14; bb = 11; continue; }
            11 => { fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 16; continue; } bb = 12; continue; }
            12 => { fl_l = (crate::simd_ops::lk_add_u32(rdi.wrapping_add(0x14u64), (1) as u32)) as u32 as u64; bb = 16; continue; }
            13 => { rsi = rbx.wrapping_add(0x8); rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 6; continue; } bb = 14; continue; }
            14 => { rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); bb = 3; continue; }
            15 => { rdi = r14; bb = 16; continue; }
            16 => { rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 24; continue; } bb = 17; continue; }
            17 => { bb = 18; continue; }
            18 => { rcx = ((rax.wrapping_sub(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; let cx_src: u32 = (rcx as u32); let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u32(rbx, fl_l as u32, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u32)) as u32 as u64; } if (((fl_l as u32)) == ((fl_r as u32))) { bb = 21; continue; } bb = 19; continue; }
            19 => { rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) >= 0) { bb = 18; continue; } bb = 20; continue; }
            20 => { bb = 25; continue; }
            21 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 26; continue; } bb = 22; continue; }
            22 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x30u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 26; continue; } bb = 23; continue; }
            23 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x38u64))) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); bb = 26; continue; }
            24 => { rsi = rbx.wrapping_add(0x8); bb = 25; continue; }
            25 => { rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); bb = 26; continue; }
            26 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rbx = rdi.wrapping_sub(0x10); fl_l = rdi; fl_r = rdi; rbx = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { r14 } else { rbx }) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 35; continue; } bb = 27; continue; }
            27 => { rax = (crate::simd_ops::ru64(r13.wrapping_add(0x40u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 33; continue; } bb = 28; continue; }
            28 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x78u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 33; continue; } bb = 29; continue; }
            29 => { fl_l = rcx; fl_r = rax; rbp = ((r14 as u32)) as u32 as u64; rbp = (rbp & !0xffu64) | (((if (((fl_l as u64)) != ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); rbp = (((rbp as u32)).wrapping_add(1)) as u32 as u64; fl_l = rcx; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 33; continue; } bb = 30; continue; }
            30 => { rdi = (crate::simd_ops::ru64(r12.wrapping_add(0x8u64))) as u64; rdx = r13; rcx = rbx; crate::simd_ops::wu64(r12.wrapping_add(0x8u64), rbx); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 32; continue; } bb = 31; continue; }
            31 => { rcx = (crate::simd_ops::ru64(r13.wrapping_add(0x40u64))) as u64; r9 = rdi; r8 = r12; rdx = r13; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); bb = 33; continue; }
            32 => { crate::simd_ops::wu64(r12.wrapping_add(0x8u64), r14); bb = 33; continue; }
            33 => { rcx = rbx.wrapping_add(0x10); rdx = 0xffffffff; let xa_src: u32 = (rdx as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rcx.wrapping_add(0x14u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rdx = (xa_old) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 35; continue; } bb = 34; continue; }
            34 => { rdx = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rdx.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 35; continue; }
            35 => { r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rax = ((rbp as u32)) as u32 as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141623d40(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x11; const P2: u64 = 0x14; const P3: u64 = 0x10; const P4: u64 = 0x141623d40;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x50u64)) as u64; rbx = (crate::simd_ops::ru64(rcx)) as u64; rsi = rdx; rax = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; rdi = rcx; rax = ((rax).wrapping_shl(4u32)) as u64; rax = ((rax).wrapping_add(rbx)) as u64; fl_l = rbx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 5; continue; } bb = 1; continue; }
            1 => { rax = ((crate::simd_ops::ru8(rdi.wrapping_add(P1))) as u32) as u32 as u64; fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(P0))) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; xmm0 = ((crate::simd_ops::ru32(rsi.wrapping_add(P2))) as u32) as u128; xmm3 = ((crate::simd_ops::ru32(rsi.wrapping_add(P3))) as u32) as u128; r8 = (crate::simd_ops::ru64(rsi.wrapping_add(P0))) as u64; rdx = (crate::simd_ops::ru64(rsi)) as u64; crate::simd_ops::wu8(rbx.wrapping_add(P0), ((rax as u8)) as u8); rax = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((xmm0 as u32)) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(P3))) as u8 as u64; fl_r = (0x3u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(P0))) as u32 as u64; rbx = ((rbx).wrapping_add(0x10u64)) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(rdi))) as u64; fl_l = rbx; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 5; continue; }
            5 => { rax = ((crate::simd_ops::ru8(rdi.wrapping_add(P3))) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 8; continue; } bb = 6; continue; }
            6 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x2u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { xmm0 = (crate::simd_ops::ru128(rsi)) as u128; rdx = rsp.wrapping_add(0x30); rax = (rax & !0xffu64) | ((((((rax as u8)) & (0xfeu8))) as u8 as u64)); xmm1 = ((crate::simd_ops::ru64(rsi.wrapping_add(P3))) as u64) as u128; rcx = rdi; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128); crate::simd_ops::wu8(rdi.wrapping_add(P3), ((rax as u8)) as u8); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), ((xmm1 as u64)) as u64); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 8; continue; }
            8 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rsp = ((rsp).wrapping_add(0x50u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141623df0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142690758; const P1: u64 = 0x1416196f0; const P2: u64 = 0x1415c2db0; const P3: u64 = 0x1415cca50;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x50u64)) as u64; r10 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu8(rcx.wrapping_add(0x20u64), (0xffu8) as u8); rdi = rcx; crate::simd_ops::wu64(rcx.wrapping_add(0x8u64), r10); crate::simd_ops::wu32(rcx.wrapping_add(0x10u64), ((r10 as u32)) as u32); rax = P0; crate::simd_ops::wu32(rcx.wrapping_add(0x18u64), ((r10 as u32)) as u32); crate::simd_ops::wu64(rcx.wrapping_add(0x30u64), r10); crate::simd_ops::wu64(rcx.wrapping_add(0x38u64), r10); crate::simd_ops::wu64(rcx.wrapping_add(0x40u64), r10); crate::simd_ops::wu64(rcx.wrapping_add(0x48u64), r10); crate::simd_ops::wu64(rcx.wrapping_add(0x50u64), r10); crate::simd_ops::wu16(rcx.wrapping_add(0x58u64), ((r10 as u16)) as u16); crate::simd_ops::wu8(rcx.wrapping_add(0x28u64), (0xffu8) as u8); crate::simd_ops::wu64(rcx.wrapping_add(0x68u64), rdx); crate::simd_ops::wu64(rcx.wrapping_add(0x70u64), r10); crate::simd_ops::wu64(rcx.wrapping_add(0x78u64), r10); crate::simd_ops::wu64(rcx.wrapping_add(0x80u64), r10); crate::simd_ops::wu64(rcx.wrapping_add(0x88u64), r10); crate::simd_ops::wu64(rcx.wrapping_add(0x90u64), r10); crate::simd_ops::wu64(rcx.wrapping_add(0x98u64), r10); crate::simd_ops::wu64(rcx, rax); rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x68u64))) as u64; crate::simd_ops::wu64(rcx.wrapping_add(0x60u64), r10); crate::simd_ops::wu64(rcx.wrapping_add(0xa0u64), r10); crate::simd_ops::wu64(rcx.wrapping_add(0xa8u64), r10); crate::simd_ops::wu32(rcx.wrapping_add(0xb0u64), ((r10 as u32)) as u32); crate::simd_ops::wu64(rcx.wrapping_add(0xb4u64), (0x7fffffffu64) as u64); crate::simd_ops::wu64(rcx.wrapping_add(0xbcu64), r10); rcx = ((rcx).wrapping_add(0xc4u64)) as u64; crate::simd_ops::wu64(rdi.wrapping_add(0xc8u64), r10); crate::simd_ops::wu64(rdi.wrapping_add(0xd0u64), r10); crate::simd_ops::wu32(rdi.wrapping_add(0xd8u64), (0x1000000u32) as u32); crate::simd_ops::wu32(rcx, ((r8 as u32)) as u32); crate::simd_ops::lk_add_u32(rax.wrapping_add(0x14u64), (1) as u32); xmm0 = (crate::simd_ops::ru128(r9)) as u128; rdx = rsp.wrapping_add(0x20); xmm1 = (crate::simd_ops::ru128(r9.wrapping_add(0x10u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); xmm0 = (crate::simd_ops::ru128(r9.wrapping_add(0x20u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm1) as u128); crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm0) as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x40u64))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; xmm1 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; crate::simd_ops::wu128(rdi.wrapping_add(0x8u64), (xmm0) as u128); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x40u64))) as u128; crate::simd_ops::wu128(rdi.wrapping_add(0x18u64), (xmm1) as u128); crate::simd_ops::wu128(rdi.wrapping_add(0x28u64), (xmm0) as u128); bb = 2; continue; }
            2 => { rsi = (crate::simd_ops::ru64(rdi.wrapping_add(0x68u64))) as u64; rbx = (crate::simd_ops::ru64(rsi.wrapping_add(0x48u64))) as u64; rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x50u64))) as u32 as u64; rcx = ((rbx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 7; continue; } bb = 3; continue; }
            3 => { bb = 4; continue; }
            4 => { rdx = (crate::simd_ops::ru64(rbx)) as u64; rcx = rdi.wrapping_add(0x90); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; rax = (crate::simd_ops::ru32(rcx)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x60u64), ((rax as u32)) as u32); rax = ((crate::simd_ops::ru8(rcx.wrapping_add(0xdu64))) as u32) as u32 as u64; rcx = rdi; crate::simd_ops::wu8(rsp.wrapping_add(0x64u64), ((rax as u8)) as u8); rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru32(rsi.wrapping_add(0x50u64))) as u32 as u64; rbx = ((rbx).wrapping_add(0x8u64)) as u64; rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x48u64))) as u64; rcx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } bb = 7; continue; }
            7 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rax = rdi; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rsp = ((rsp).wrapping_add(0x50u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141623f80(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142690758; const P1: u64 = 0x68; const P2: u64 = 0x14; const P3: u64 = 0x18; const P4: u64 = 0x90; const P5: u64 = 0x98; const P6: u64 = 0x1415242e0; const P7: u64 = 0x9c; const P8: u64 = 0x48; const P9: u64 = 0x50; const P10: u64 = 0x54; const P11: u64 = 0x1415cc8f0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = P0; rbx = rcx; crate::simd_ops::wu64(rcx, rax); rax = 0xffffffff; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rcx.wrapping_add(P2), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            2 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P4))) as u64; rdi = 0; fl_l = 0; fl_r = 0; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rbx.wrapping_add(P5), ((rdi as u32)) as u32); rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(P4), rdi); crate::simd_ops::wu32(rbx.wrapping_add(P7), ((rdi as u32)) as u32); bb = 4; continue; }
            4 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P8))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rbx.wrapping_add(P9), ((rdi as u32)) as u32); rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(P8), rdi); crate::simd_ops::wu32(rbx.wrapping_add(P10), ((rdi as u32)) as u32); bb = 6; continue; }
            6 => { rcx = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P11, rax, rcx, rdx, r8, r9, rsp); }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141624050(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141623f80; const P1: u64 = 0x141834d30;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = ((rdx as u32)) as u32 as u64;
    rdi = rcx;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    fl_l = ((rbx as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64;
    if !(((((fl_l as u8)) & ((fl_r as u8))) == 0)) { rdx = 0x140; rcx = rdi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rax = rdi;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141624090(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141623f80; const P1: u64 = 0x141834d30;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = ((rdx as u32)) as u32 as u64;
    rdi = rcx;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    fl_l = ((rbx as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64;
    if !(((((fl_l as u8)) & ((fl_r as u8))) == 0)) { rdx = 0xe0; rcx = rdi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rax = rdi;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416240d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x88; const P1: u64 = 0x80; const P2: u64 = 0xc; const P3: u64 = 0x14153e940; const P4: u64 = 0x8; const P5: u64 = 0x1; const P6: u64 = 0x24;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 1; continue; }
            1 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rsi); rbx = rdx; rsi = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rdi); rdi = ((rcx.wrapping_add(P1)) as u64) as u64; rax = (crate::simd_ops::ru64(rdi)) as u64; rcx = ((rax.wrapping_add(rsi.wrapping_mul(8))) as u64) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 4; continue; } bb = 2; continue; }
            2 => { fl_l = rbx; fl_r = (crate::simd_ops::ru64(rax)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 9; continue; } bb = 3; continue; }
            3 => { rax = ((rax).wrapping_add(0x8u64)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 4; continue; }
            4 => { fl_l = ((rsi as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rdi.wrapping_add(P2))) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 7; continue; } bb = 5; continue; }
            5 => { rcx = rdi; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 9; continue; } bb = 6; continue; }
            6 => { fl_l = ((rsi as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rdi.wrapping_add(P2))) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 9; continue; } bb = 7; continue; }
            7 => { rcx = (crate::simd_ops::ru32(rdi.wrapping_add(P4))) as u32 as u64; rax = ((rcx.wrapping_add(P5)) as u32) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(P4), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rdi)) as u64; rdx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { crate::simd_ops::wu64(rdx, rbx); fl_l = (crate::simd_ops::lk_add_u32(rbx.wrapping_add(P6), (1) as u32)) as u32 as u64; bb = 9; continue; }
            9 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); bb = 10; continue; }
            10 => { return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141624150(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0xd8; const P1: u64 = 0xfc; const P2: u64 = 0x108; const P3: u64 = 0x1438152b4; const P4: u64 = 0xd0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = (crate::simd_ops::ru16(rcx.wrapping_add(P0))) as u16 as u64; fl_r = (0x0u16) as u16 as u64;
    rdx = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64;
    rax = (crate::simd_ops::ru32(rcx.wrapping_add(P2))) as u32 as u64;
    if ((((fl_l as u16)) as i16) <= (((fl_r as u16)) as i16)) { crate::simd_ops::wu32(rcx.wrapping_add(P4), (0x0u32) as u32); return rax; }
    r8 = ((crate::simd_ops::ru16(P3)) as u32) as u32 as u64;
    fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64;
    rdx = (if (((fl_l as u32)) >= ((fl_r as u32))) { (rax as u32) } else { (rdx as u32) }) as u32 as u64;
    rax = ((r8.wrapping_add(r8)) as u32) as u32 as u64;
    fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64;
    if !((((fl_l as u32)) >= ((fl_r as u32)))) { crate::simd_ops::wu32(rcx.wrapping_add(P4), ((r8 as u32)) as u32); return rax; }
    crate::simd_ops::wu32(rcx.wrapping_add(P4), (0x0u32) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141624190(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0xd8; const P1: u64 = 0x1438152b4; const P2: u64 = 0x1438152ac; const P3: u64 = 0x14342668c; const P4: u64 = 0x108; const P5: u64 = 0xd0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    fl_l = (crate::simd_ops::ru16(rcx.wrapping_add(P0))) as u16 as u64; fl_r = (0x0u16) as u16 as u64;
    if ((((fl_l as u16)) as i16) <= (((fl_r as u16)) as i16)) { crate::simd_ops::wu32(rcx.wrapping_add(P5), (0x0u32) as u32); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
    rdx = ((crate::simd_ops::ru16(P1)) as u32) as u32 as u64;
    xmm1 = 0u128;
    rax = (crate::simd_ops::ru32(P2)) as u32 as u64;
    xmm0 = (((rdx as u32)) as u32) as u128;
    xmm1 = (xmm1 & !0xffff_ffffu128) | ((((rax) as i64) as f32).to_bits() as u128);
    xmm0 = crate::simd_ops::vcvtdq2ps(xmm0);
    xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) / f32::from_bits(xmm0 as u32)).to_bits() as u128);
    xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(crate::simd_ops::ru32(P3))).to_bits() as u128);
    fl_l = ((xmm1 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rcx.wrapping_add(P4))) as u32 as u64;
    if !((f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32)))) { crate::simd_ops::wu32(rcx.wrapping_add(P5), ((rdx as u32)) as u32); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
    crate::simd_ops::wu32(rcx.wrapping_add(P5), (0x0u32) as u32);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416241e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0xd8; const P1: u64 = 0x1438152b4; const P2: u64 = 0xd0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = (crate::simd_ops::ru16(rcx.wrapping_add(P0))) as u16 as u64; fl_r = (0x0u16) as u16 as u64;
    if !(((((fl_l as u16)) as i16) <= (((fl_r as u16)) as i16))) { rax = ((crate::simd_ops::ru16(P1)) as u32) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(P2), ((rax as u32)) as u32); return rax; }
    crate::simd_ops::wu32(rcx.wrapping_add(P2), (0x0u32) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141624210(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141524330; const P1: u64 = 0x141623df0; const P2: u64 = 0x142690830; const P3: u64 = 0x1426907e8; const P4: u64 = 0x1426907a0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = r9; rsi = ((r8 as u32)) as u32 as u64; rbp = rdx; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 8; continue; } bb = 1; continue; }
            1 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 6; continue; } bb = 2; continue; }
            2 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = 0; fl_l = 0; fl_r = 0; bb = 12; continue; }
            4 => { rdx = 0x110; rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rbx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 5; continue; }
            5 => { r9 = rdi; r8 = ((rsi as u32)) as u32 as u64; rdx = rbp; rcx = rax; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rax = P2; crate::simd_ops::wu8(rbx.wrapping_add(0x108u64), (0x0u8) as u8); crate::simd_ops::wu64(rbx, rax); bb = 11; continue; }
            6 => { rdx = 0x110; rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rbx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 7; continue; }
            7 => { r9 = rdi; r8 = ((rsi as u32)) as u32 as u64; rdx = rbp; rcx = rax; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rax = P3; crate::simd_ops::wu64(rbx, rax); bb = 11; continue; }
            8 => { rdx = 0x140; rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rbx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { r9 = rdi; r8 = ((rsi as u32)) as u32 as u64; rdx = rbp; rcx = rax; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rax = P4; crate::simd_ops::wu64(rbx, rax); rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rbx.wrapping_add(0x118u64), rax); crate::simd_ops::wu64(rbx.wrapping_add(0x120u64), rax); crate::simd_ops::wu64(rbx.wrapping_add(0x128u64), rax); crate::simd_ops::wu64(rbx.wrapping_add(0x130u64), rax); crate::simd_ops::wu8(rbx.wrapping_add(0x138u64), ((rax as u8)) as u8); bb = 11; continue; }
            10 => { rax = 0; fl_l = 0; fl_r = 0; rbx = ((rax as u32)) as u32 as u64; bb = 11; continue; }
            11 => { rax = rbx; bb = 12; continue; }
            12 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141624330(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x28; const P1: u64 = 0x68; const P2: u64 = 0x8; const P3: u64 = 0xb4; const P4: u64 = 0x38; const P5: u64 = 0x40;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    rdi = rcx;
    rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp);
    rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P1))) as u64;
    r8 = rdi;
    rdx = rax;
    rbx = rax;
    r9 = (crate::simd_ops::ru64(rcx)) as u64;
    rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(r9.wrapping_add(P0))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp);
    rax = (crate::simd_ops::ru32(rbx.wrapping_add(P2))) as u32 as u64;
    rcx = rdi;
    crate::simd_ops::wu32(rdi.wrapping_add(P3), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru64(rdi)) as u64;
    rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P4))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp);
    rax = (crate::simd_ops::ru64(rdi)) as u64;
    rcx = rdi;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P5))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141624380(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0xf8; const P1: u64 = 0xb0; const P2: u64 = 0xfc; const P3: u64 = 0x108; const P4: u64 = 0x104;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    xmm0 = 0u128;
    fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64;
    if !((f32::from_bits((fl_l as u32)) == f32::from_bits((fl_r as u32)) || f32::from_bits((fl_l as u32)).is_nan() || f32::from_bits((fl_r as u32)).is_nan())) { crate::simd_ops::wu32(rcx.wrapping_add(P1), (0x0u32) as u32); }
    fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(P2))) as u32 as u64; fl_r = 0x0;
    if (((fl_l as u32)) != ((fl_r as u32))) { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
    fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(P3))) as u32 as u64; fl_r = 0x0;
    crate::simd_ops::wu32(rcx.wrapping_add(P0), (0x3f800000u32) as u32);
    if !((((fl_l as u32)) != ((fl_r as u32)))) { rax = (crate::simd_ops::ru32(rcx.wrapping_add(P4))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(P0), ((rax as u32)) as u32); }
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416243c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xd0; const P1: u64 = 0xf8; const P2: u64 = 0x110; const P3: u64 = 0xc8; const P4: u64 = 0x141624720;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = ((rsp).wrapping_sub(0x48u64)) as u64;
    fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; fl_r = 0x0;
    rax = rcx;
    if !((((fl_l as u32)) != ((fl_r as u32)))) { xmm3 = ((crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32) as u128; r8 = rsp.wrapping_add(0x50); xmm0 = ((crate::simd_ops::ru32(rax.wrapping_add(P2))) as u32) as u128; rdx = ((rcx.wrapping_add(P3)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), r8); rcx = ((rcx).wrapping_add(0x118u64)) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((xmm0 as u32)) as u32); r8 = 0x1; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((xmm3 as u32)) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); }
    rsp = ((rsp).wrapping_add(0x48u64)) as u64;
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141624420(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xc0; const P1: u64 = 0x1415ccba0; const P2: u64 = 0x98;
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    crate::simd_ops::wu32(rcx.wrapping_add(P0), (0x3u32) as u32);
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    crate::simd_ops::wu32(rbx.wrapping_add(P2), (0x0u32) as u32);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141624450(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xa8; const P1: u64 = 0xc0; const P2: u64 = 0xc4; const P3: u64 = 0xcc; const P4: u64 = 0xda; const P5: u64 = 0xc8; const P6: u64 = 0xc; const P7: u64 = 0x8; const P8: u64 = 0x68; const P9: u64 = 0x10; const P10: u64 = 0x59; const P11: u64 = 0x58; const P12: u64 = 0x141623d40; const P13: u64 = 0x141618a40;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0; let mut xmm7: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = ((rsp).wrapping_sub(0x60u64)) as u64; rsi = rcx; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru32(rsi.wrapping_add(P1))) as u32 as u64; rax = (((rax as u32)).wrapping_sub(0x3u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 8; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru32(rsi.wrapping_add(P2))) as u32 as u64; fl_r = 0x1; if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { bb = 4; continue; } bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru32(rsi.wrapping_add(P3))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 8; continue; } bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::ru8(rsi.wrapping_add(P4))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x80u64), rdi); crate::simd_ops::wu128(rsp.wrapping_add(0x50u64), (xmm6) as u128); crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm7) as u128); if (((fl_l as u8)) == ((fl_r as u8))) { bb = 6; continue; } bb = 5; continue; }
            5 => { xmm7 = ((crate::simd_ops::ru32(rsi.wrapping_add(P5))) as u32) as u128; bb = 7; continue; }
            6 => { xmm7 = 0u128; bb = 7; continue; }
            7 => { xmm6 = ((crate::simd_ops::ru32(rcx.wrapping_add(P6))) as u32) as u128; rdi = ((rsi.wrapping_add(P7)) as u64) as u64; rax = (crate::simd_ops::ru64(rsi.wrapping_add(P8))) as u64; rdx = rsp.wrapping_add(0x20); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rdi); xmm1 = (xmm7) as u128; xmm1 = crate::simd_ops::vunpcklps(xmm1, xmm6); crate::simd_ops::wu8(rsp.wrapping_add(0x74u64), (0x4u8) as u8); rcx = (crate::simd_ops::ru32(rax.wrapping_add(P9))) as u32 as u64; crate::simd_ops::lk_add_u8(rsi.wrapping_add(P10), (1) as u8); crate::simd_ops::lk_add_u8(rsi.wrapping_add(P11), (0x4u8) as u8); crate::simd_ops::wu32(rsp.wrapping_add(0x70u64), ((rcx as u32)) as u32); rcx = rsi.wrapping_add(0x48); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), ((xmm1 as u64)) as u64); crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P12, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = ((crate::simd_ops::ru8(rsi.wrapping_add(0x58u64))) as u32) as u32 as u64; xmm3 = (xmm6) as u128; rcx = (((rax as u8)) as u32) as u32 as u64; xmm2 = (xmm7) as u128; rcx = (rcx & !0xffu64) | (((((rcx as u8)).wrapping_shr(2u32)) as u8 as u64)); rax = (rax & !0xffu64) | ((((((rax as u8)) & (0x3u8))) as u8 as u64)); rcx = (rcx & !0xffu64) | (((((rcx as u8)).wrapping_sub(1)) as u8 as u64)); rdx = rdi; rcx = (rcx & !0xffu64) | (((((rcx as u8)).wrapping_shl(2u32)) as u8 as u64)); rcx = (rcx & !0xffu64) | ((((((rcx as u8)) | ((rax as u8)))) as u8 as u64)); crate::simd_ops::wu8(rsi.wrapping_add(0x58u64), ((rcx as u8)) as u8); rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x68u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P13, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm7 = (crate::simd_ops::ru128(rsp.wrapping_add(0x40u64))) as u128; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x50u64))) as u128; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; bb = 8; continue; }
            8 => { rsp = ((rsp).wrapping_add(0x60u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141624560(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143426fd0; const P1: u64 = 0x141529ed0;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0; let mut xmm7: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 15>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x68u64)) as u64; rbx = (crate::simd_ops::ru64(rcx.wrapping_add(0x90u64))) as u64; r15 = rcx; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x98u64))) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rcx); xmm1 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x34u64))) as u32) as u128; crate::simd_ops::wu32(rsp.wrapping_add(0x24u64), ((xmm1 as u32)) as u32); rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((r15 as u32)) as u32); r9 = ((rbx.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = r9; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 18; continue; } bb = 1; continue; }
            1 => { r10 = ((crate::simd_ops::ru8(rsp.wrapping_add(0x8cu64))) as u32) as u32 as u64; r8 = ((r10 as u32)) as u32 as u64; rax = ((rdx as u32)) as u32 as u64; r8 = ((r8).wrapping_shl(32u32)) as u64; fl_l = (((r8) | (rax))) as u64; fl_r = fl_l; r8 = ((fl_l as u64)) as u64; bb = 2; continue; }
            2 => { rcx = ((crate::simd_ops::ru8(rbx.wrapping_add(0x4u64))) as u32) as u32 as u64; rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; rcx = ((rcx).wrapping_shl(32u32)) as u64; rcx = (((rcx) | (rax))) as u64; fl_l = rcx; fl_r = r8; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 5; continue; } bb = 3; continue; }
            3 => { rbx = ((rbx).wrapping_add(0x18u64)) as u64; fl_l = rbx; fl_r = r9; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 4; continue; }
            4 => { rsp = ((rsp).wrapping_add(0x68u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            5 => { crate::simd_ops::wu64(rsp.wrapping_add(0x80u64), rbp); rbp = r10; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), r14); r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x50u64), (xmm6) as u128); xmm6 = ((crate::simd_ops::ru32(rsp.wrapping_add(0xa0u64))) as u32) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm7) as u128); xmm7 = ((crate::simd_ops::ru32(P0)) as u32) as u128; rbp = ((rbp).wrapping_shl(32u32)) as u64; rax = ((rdx as u32)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x90u64), rsi); fl_l = (((rbp) | (rax))) as u64; fl_r = fl_l; rbp = ((fl_l as u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x98u64), rdi); bb = 6; continue; }
            6 => { rcx = ((crate::simd_ops::ru8(rbx.wrapping_add(0x4u64))) as u32) as u32 as u64; rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; rcx = ((rcx).wrapping_shl(32u32)) as u64; rcx = (((rcx) | (rax))) as u64; fl_l = rcx; fl_r = rbp; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 17; continue; } bb = 7; continue; }
            7 => { rdi = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; fl_l = ((xmm6 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rdi.wrapping_add(0x30u64))) as u32 as u64; if (f32::from_bits((fl_l as u32)) != f32::from_bits((fl_r as u32)) && !f32::from_bits((fl_l as u32)).is_nan() && !f32::from_bits((fl_r as u32)).is_nan()) { bb = 9; continue; } bb = 8; continue; }
            8 => { xmm0 = ((crate::simd_ops::ru32(rdi.wrapping_add(0x38u64))) as u32) as u128; bb = 11; continue; }
            9 => { fl_l = ((xmm6 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rdi.wrapping_add(0x34u64))) as u32 as u64; if (f32::from_bits((fl_l as u32)) != f32::from_bits((fl_r as u32)) && !f32::from_bits((fl_l as u32)).is_nan() && !f32::from_bits((fl_r as u32)).is_nan()) { bb = 12; continue; } bb = 10; continue; }
            10 => { xmm0 = ((crate::simd_ops::ru32(rdi.wrapping_add(0x3cu64))) as u32) as u128; bb = 11; continue; }
            11 => { fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = ((xmm7 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) != f32::from_bits((fl_r as u32)) && !f32::from_bits((fl_l as u32)).is_nan() && !f32::from_bits((fl_r as u32)).is_nan()) { bb = 16; continue; } bb = 12; continue; }
            12 => { fl_l = (crate::simd_ops::ru64(rdi.wrapping_add(0x18u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 14; continue; } bb = 13; continue; }
            13 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x28u64))) as u32 as u64; r8 = rsp.wrapping_add(0xa0); xmm1 = (xmm6) as u128; crate::simd_ops::wu32(rsp.wrapping_add(0xa0u64), ((rax as u32)) as u32); rcx = rdi.wrapping_add(0x18); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0xa0u64))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x28u64), ((rax as u32)) as u32); bb = 15; continue; }
            14 => { xmm0 = 0u128; bb = 15; continue; }
            15 => { rax = ((crate::simd_ops::ru8(rdi.wrapping_add(0x14u64))) as u32) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(rax.wrapping_mul(4)).wrapping_add(0x30u64), ((xmm6 as u32)) as u32); rax = ((crate::simd_ops::ru8(rdi.wrapping_add(0x14u64))) as u32) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(rax.wrapping_mul(4)).wrapping_add(0x38u64), ((xmm0 as u32)) as u32); fl_l = (crate::simd_ops::lk_xor_u8(rdi.wrapping_add(0x14u64), (0x1u8) as u8)) as u8 as u64; fl_r = fl_l; bb = 16; continue; }
            16 => { rax = (crate::simd_ops::ru64(r14)) as u64; xmm2 = (xmm0) as u128; rdx = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; rcx = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru32(r15.wrapping_add(0x98u64))) as u32 as u64; rbx = ((rbx).wrapping_add(0x18u64)) as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru64(r15.wrapping_add(0x90u64))) as u64; rcx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 6; continue; } bb = 17; continue; }
            17 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x98u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x50u64))) as u128; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; xmm7 = (crate::simd_ops::ru128(rsp.wrapping_add(0x40u64))) as u128; bb = 18; continue; }
            18 => { rsp = ((rsp).wrapping_add(0x68u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141624720(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143426454; const P1: u64 = 0x143426d88; const P2: u64 = 0x1434268d0; const P3: u64 = 0x140000000; const P4: u64 = 0x142676604; const P5: u64 = 0x142676614; const P6: u64 = 0x142676624; const P7: u64 = 0x142676640; const P8: u64 = 0x143426780; const P9: u64 = 0x143426c2c; const P10: u64 = 0x14342698c; const P11: u64 = 0x142674d70; const P12: u64 = 0x143427104; const P13: u64 = 0x143427080; const P14: u64 = 0x143428760; const P15: u64 = 0x143426444; const P16: u64 = 0x14183ad76;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut j_tgt, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm4: u128 = 0; let mut xmm5: u128 = 0; let mut xmm6: u128 = 0; let mut xmm7: u128 = 0; let mut xmm8: u128 = 0; let mut xmm9: u128 = 0; let mut xmm10: u128 = 0; let mut xmm11: u128 = 0; let mut xmm12: u128 = 0; let mut xmm13: u128 = 0; let mut xmm14: u128 = 0; let mut xmm15: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 31>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = (rsp) as u64; crate::simd_ops::wu64(rax.wrapping_add(0x18u64), (rbx) as u64); crate::simd_ops::wu64(rax.wrapping_add(0x20u64), (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rbp = ((rax.wrapping_sub(0x47u64)) as u64) as u64; rsp = ((rsp).wrapping_sub(0xf0u64)) as u64; xmm1 = (crate::simd_ops::ru128(rcx.wrapping_add(0x10u64))) as u128; r11 = ((r8 as u32)) as u32 as u64; rbx = (rcx) as u64; xmm0 = (crate::simd_ops::ru128(rcx)) as u128; xmm5 = ((crate::simd_ops::ru32(rbp.wrapping_add(0x6fu64))) as u32) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x60u64), (xmm14) as u128); r10 = ((rdx.wrapping_add(r11.wrapping_mul(4))) as u64) as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128); crate::simd_ops::wu64(rax.wrapping_add(0x8u64), (rsi) as u64); xmm0 = 0u128; crate::simd_ops::wu128(rax.wrapping_sub(0x18u64), (xmm6) as u128); crate::simd_ops::wu128(rax.wrapping_sub(0x28u64), (xmm7) as u128); xmm6 = (xmm1) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((((r11) as i64) as f32).to_bits() as u128); xmm7 = (xmm3) as u128; xmm7 = (xmm7 & !0xffff_ffffu128) | ((f32::from_bits(xmm7 as u32) - f32::from_bits(xmm5 as u32)).to_bits() as u128); crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm1) as u128); crate::simd_ops::wu128(rbp.wrapping_sub(0x71u64), (xmm1) as u128); xmm7 = (xmm7 & !0xffff_ffffu128) | ((f32::from_bits(xmm7 as u32) / f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm0 = 0u128; fl_l = ((xmm1 as u32)) as u32 as u64; fl_r = ((xmm0 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) != f32::from_bits((fl_r as u32)) && !f32::from_bits((fl_l as u32)).is_nan() && !f32::from_bits((fl_r as u32)).is_nan()) { bb = 2; continue; } bb = 1; continue; }
            1 => { rsi = (rsi & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 3; continue; }
            2 => { rsi = (rsi & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 3; continue; }
            3 => { xmm0 = ((crate::simd_ops::ru32(P0)) as u32) as u128; fl_l = ((xmm1 as u32)) as u32 as u64; fl_r = ((xmm0 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 5; continue; } bb = 4; continue; }
            4 => { xmm1 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x71u64))) as u128; xmm6 = (xmm0) as u128; xmm1 = (xmm1 & !0xffff_ffffu128) | (xmm0 & 0xffff_ffffu128); crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm1) as u128); crate::simd_ops::wu128(rbp.wrapping_sub(0x71u64), (xmm1) as u128); bb = 5; continue; }
            5 => { xmm3 = ((crate::simd_ops::ru32(P1)) as u32) as u128; xmm0 = (xmm1) as u128; xmm14 = ((crate::simd_ops::ru32(P2)) as u32) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0xc0u64), (xmm8) as u128); crate::simd_ops::wu128(rsp.wrapping_add(0xb0u64), (xmm9) as u128); crate::simd_ops::wu128(rsp.wrapping_add(0xa0u64), (xmm10) as u128); xmm0 = crate::simd_ops::vpsrldq(xmm0, 0x4u8); rdi = ((xmm0 as u32)) as u32 as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x90u64), (xmm11) as u128); crate::simd_ops::wu128(rsp.wrapping_add(0x80u64), (xmm12) as u128); fl_l = ((rdi as u32)) as u32 as u64; fl_r = (0x5u32) as u32 as u64; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 151; continue; } bb = 6; continue; }
            6 => { r8 = ((P3) as u64) as u64; rax = ((((rdi as u32)) as i32 as i64) as u64) as u64; rcx = (crate::simd_ops::ru32(r8.wrapping_add(rax.wrapping_mul(4)).wrapping_add(0x162535cu64))) as u32 as u64; fl_l = ((rcx).wrapping_add(r8)) as u64 as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; j_tgt = (rcx) as u64; bb = 0xffff_fffe; continue; }
            7 => { fl_l = (rdx) as u64 as u64; fl_r = (r10) as u64 as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 151; continue; } bb = 8; continue; }
            8 => { xmm8 = ((crate::simd_ops::ru32(P4)) as u32) as u128; xmm9 = ((crate::simd_ops::ru32(P5)) as u32) as u128; xmm10 = ((crate::simd_ops::ru32(P6)) as u32) as u128; xmm11 = ((crate::simd_ops::ru32(P7)) as u32) as u128; xmm12 = ((crate::simd_ops::ru32(P8)) as u32) as u128; xmm4 = ((crate::simd_ops::ru32(rbp.wrapping_sub(0x75u64))) as u32) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x70u64), (xmm13) as u128); xmm13 = ((crate::simd_ops::ru32(P9)) as u32) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x50u64), (xmm15) as u128); xmm15 = (xmm14) as u128; xmm15 = (xmm15 & !0xffff_ffffu128) | ((f32::from_bits(xmm15 as u32) / f32::from_bits(xmm6 as u32)).to_bits() as u128); bb = 9; continue; }
            9 => { xmm1 = ((crate::simd_ops::ru32(P10)) as u32) as u128; xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) - f32::from_bits(xmm4 as u32)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm15 as u32)).to_bits() as u128); rcx = (crate::simd_ops::cvt_f32_i32((f32::from_bits(xmm1 as u32)))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = (0x80000000u32) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 12; continue; } bb = 10; continue; }
            10 => { xmm0 = (((rcx as u32)) as u32) as u128; xmm0 = crate::simd_ops::vcvtdq2ps(xmm0); fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = ((xmm1 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) == f32::from_bits((fl_r as u32)) || f32::from_bits((fl_l as u32)).is_nan() || f32::from_bits((fl_r as u32)).is_nan()) { bb = 12; continue; } bb = 11; continue; }
            11 => { xmm1 = crate::simd_ops::vunpcklps(xmm1, xmm1); rax = (crate::simd_ops::vmovmskps(xmm1) as u32) as u32 as u64; rax = ((((rax as u32)) & (0x1u32))) as u32 as u64; rax = ((((rax as u32)) ^ (0x1u32))) as u32 as u64; fl_l = (((rcx as u32)).wrapping_add((rax as u32))) as u32 as u64; fl_r = fl_l; rcx = ((fl_l as u32)) as u32 as u64; xmm1 = (((rcx as u32)) as u32) as u128; xmm1 = crate::simd_ops::vcvtdq2ps(xmm1); bb = 12; continue; }
            12 => { rax = (crate::simd_ops::cvt_f32_i32((f32::from_bits(xmm1 as u32)))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) >= 0) { bb = 14; continue; } bb = 13; continue; }
            13 => { rax = (0xffffffffu32) as u32 as u64; bb = 15; continue; }
            14 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = ((r11 as u32)) as u32 as u64; rax = (if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { (r11 as u32) } else { (rax as u32) }) as u32 as u64; bb = 15; continue; }
            15 => { rax = (rax as u32 as i32 as i64) as u64; rcx = ((rdx.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = (r10) as u64; fl_l = (rcx) as u64 as u64; fl_r = (r10) as u64 as u64; rax = (if (((fl_l as u64)) < ((fl_r as u64))) { rcx } else { rax }) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 18; continue; } bb = 16; continue; }
            16 => { bb = 17; continue; }
            17 => { xmm4 = (xmm4 & !0xffff_ffffu128) | ((f32::from_bits(xmm4 as u32) + f32::from_bits(xmm6 as u32)).to_bits() as u128); xmm2 = (xmm9) as u128; xmm5 = (xmm5 & !0xffff_ffffu128) | ((f32::from_bits(xmm5 as u32) + f32::from_bits(xmm7 as u32)).to_bits() as u128); xmm1 = (xmm4) as u128; crate::simd_ops::wu32(rbp.wrapping_sub(0x75u64), ((xmm4 as u32)) as u32); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm4 as u32)).to_bits() as u128); xmm0 = (xmm1) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm8 as u32)).to_bits() as u128); xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) - f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) * f32::from_bits(xmm1 as u32)).to_bits() as u128); xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) - f32::from_bits(xmm10 as u32)).to_bits() as u128); xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) * f32::from_bits(xmm1 as u32)).to_bits() as u128); xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) + f32::from_bits(xmm11 as u32)).to_bits() as u128); xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) * f32::from_bits(xmm4 as u32)).to_bits() as u128); xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) + f32::from_bits(xmm12 as u32)).to_bits() as u128); xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) * f32::from_bits(xmm5 as u32)).to_bits() as u128); crate::simd_ops::wu32(rdx, ((xmm2 as u32)) as u32); rdx = ((rdx).wrapping_add(0x4u64)) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 17; continue; } bb = 18; continue; }
            18 => { xmm1 = (xmm13) as u128; xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) - f32::from_bits(xmm4 as u32)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm15 as u32)).to_bits() as u128); rcx = (crate::simd_ops::cvt_f32_i32((f32::from_bits(xmm1 as u32)))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = (0x80000000u32) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 21; continue; } bb = 19; continue; }
            19 => { xmm0 = (((rcx as u32)) as u32) as u128; xmm0 = crate::simd_ops::vcvtdq2ps(xmm0); fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = ((xmm1 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) == f32::from_bits((fl_r as u32)) || f32::from_bits((fl_l as u32)).is_nan() || f32::from_bits((fl_r as u32)).is_nan()) { bb = 21; continue; } bb = 20; continue; }
            20 => { xmm1 = crate::simd_ops::vunpcklps(xmm1, xmm1); rax = (crate::simd_ops::vmovmskps(xmm1) as u32) as u32 as u64; rax = ((((rax as u32)) & (0x1u32))) as u32 as u64; rax = ((((rax as u32)) ^ (0x1u32))) as u32 as u64; fl_l = (((rcx as u32)).wrapping_add((rax as u32))) as u32 as u64; fl_r = fl_l; rcx = ((fl_l as u32)) as u32 as u64; xmm1 = (((rcx as u32)) as u32) as u128; xmm1 = crate::simd_ops::vcvtdq2ps(xmm1); bb = 21; continue; }
            21 => { rax = (crate::simd_ops::cvt_f32_i32((f32::from_bits(xmm1 as u32)))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) >= 0) { bb = 23; continue; } bb = 22; continue; }
            22 => { rax = (0xffffffffu32) as u32 as u64; bb = 24; continue; }
            23 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = ((r11 as u32)) as u32 as u64; rax = (if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { (r11 as u32) } else { (rax as u32) }) as u32 as u64; bb = 24; continue; }
            24 => { rax = (rax as u32 as i32 as i64) as u64; rcx = ((rdx.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = (r10) as u64; fl_l = (rcx) as u64 as u64; fl_r = (r10) as u64 as u64; rax = (if (((fl_l as u64)) < ((fl_r as u64))) { rcx } else { rax }) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 28; continue; } bb = 25; continue; }
            25 => { bb = 26; continue; }
            26 => { xmm4 = (xmm4 & !0xffff_ffffu128) | ((f32::from_bits(xmm4 as u32) + f32::from_bits(xmm6 as u32)).to_bits() as u128); xmm3 = (xmm13) as u128; xmm2 = (xmm9) as u128; xmm5 = (xmm5 & !0xffff_ffffu128) | ((f32::from_bits(xmm5 as u32) + f32::from_bits(xmm7 as u32)).to_bits() as u128); xmm3 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) - f32::from_bits(xmm4 as u32)).to_bits() as u128); crate::simd_ops::wu32(rbp.wrapping_sub(0x75u64), ((xmm4 as u32)) as u32); xmm1 = (xmm3) as u128; xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm3 as u32)).to_bits() as u128); xmm0 = (xmm1) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm8 as u32)).to_bits() as u128); xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) - f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) * f32::from_bits(xmm1 as u32)).to_bits() as u128); xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) - f32::from_bits(xmm10 as u32)).to_bits() as u128); xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) * f32::from_bits(xmm1 as u32)).to_bits() as u128); xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) + f32::from_bits(xmm11 as u32)).to_bits() as u128); xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) * f32::from_bits(xmm3 as u32)).to_bits() as u128); xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) + f32::from_bits(xmm12 as u32)).to_bits() as u128); xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) * f32::from_bits(xmm5 as u32)).to_bits() as u128); crate::simd_ops::wu32(rdx, ((xmm2 as u32)) as u32); rdx = ((rdx).wrapping_add(0x4u64)) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 26; continue; } bb = 27; continue; }
            27 => { xmm3 = ((crate::simd_ops::ru32(P1)) as u32) as u128; bb = 28; continue; }
            28 => { xmm1 = ((crate::simd_ops::ru32(P11)) as u32) as u128; xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) - f32::from_bits(xmm4 as u32)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm15 as u32)).to_bits() as u128); rcx = (crate::simd_ops::cvt_f32_i32((f32::from_bits(xmm1 as u32)))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = (0x80000000u32) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 31; continue; } bb = 29; continue; }
            29 => { xmm0 = (((rcx as u32)) as u32) as u128; xmm0 = crate::simd_ops::vcvtdq2ps(xmm0); fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = ((xmm1 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) == f32::from_bits((fl_r as u32)) || f32::from_bits((fl_l as u32)).is_nan() || f32::from_bits((fl_r as u32)).is_nan()) { bb = 31; continue; } bb = 30; continue; }
            30 => { xmm1 = crate::simd_ops::vunpcklps(xmm1, xmm1); rax = (crate::simd_ops::vmovmskps(xmm1) as u32) as u32 as u64; rax = ((((rax as u32)) & (0x1u32))) as u32 as u64; rax = ((((rax as u32)) ^ (0x1u32))) as u32 as u64; fl_l = (((rcx as u32)).wrapping_add((rax as u32))) as u32 as u64; fl_r = fl_l; rcx = ((fl_l as u32)) as u32 as u64; xmm1 = (((rcx as u32)) as u32) as u128; xmm1 = crate::simd_ops::vcvtdq2ps(xmm1); bb = 31; continue; }
            31 => { rax = (crate::simd_ops::cvt_f32_i32((f32::from_bits(xmm1 as u32)))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) >= 0) { bb = 33; continue; } bb = 32; continue; }
            32 => { rax = (0xffffffffu32) as u32 as u64; bb = 34; continue; }
            33 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = ((r11 as u32)) as u32 as u64; rax = (if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { (r11 as u32) } else { (rax as u32) }) as u32 as u64; bb = 34; continue; }
            34 => { rax = (rax as u32 as i32 as i64) as u64; rcx = ((rdx.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = (r10) as u64; fl_l = (rcx) as u64 as u64; fl_r = (r10) as u64 as u64; rax = (if (((fl_l as u64)) < ((fl_r as u64))) { rcx } else { rax }) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 38; continue; } bb = 35; continue; }
            35 => { bb = 36; continue; }
            36 => { xmm4 = (xmm4 & !0xffff_ffffu128) | ((f32::from_bits(xmm4 as u32) + f32::from_bits(xmm6 as u32)).to_bits() as u128); xmm2 = (xmm9) as u128; xmm5 = (xmm5 & !0xffff_ffffu128) | ((f32::from_bits(xmm5 as u32) + f32::from_bits(xmm7 as u32)).to_bits() as u128); xmm3 = (xmm4) as u128; crate::simd_ops::wu32(rbp.wrapping_sub(0x75u64), ((xmm4 as u32)) as u32); xmm3 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) - f32::from_bits(xmm13 as u32)).to_bits() as u128); xmm1 = (xmm3) as u128; xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm3 as u32)).to_bits() as u128); xmm0 = (xmm1) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm8 as u32)).to_bits() as u128); xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) - f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm0 = (xmm12) as u128; xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) * f32::from_bits(xmm1 as u32)).to_bits() as u128); xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) - f32::from_bits(xmm10 as u32)).to_bits() as u128); xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) * f32::from_bits(xmm1 as u32)).to_bits() as u128); xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) + f32::from_bits(xmm11 as u32)).to_bits() as u128); xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) * f32::from_bits(xmm3 as u32)).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) - f32::from_bits(xmm2 as u32)).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm5 as u32)).to_bits() as u128); crate::simd_ops::wu32(rdx, ((xmm0 as u32)) as u32); rdx = ((rdx).wrapping_add(0x4u64)) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 36; continue; } bb = 37; continue; }
            37 => { xmm3 = ((crate::simd_ops::ru32(P1)) as u32) as u128; bb = 38; continue; }
            38 => { xmm1 = (xmm3) as u128; xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) - f32::from_bits(xmm4 as u32)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm15 as u32)).to_bits() as u128); rcx = (crate::simd_ops::cvt_f32_i32((f32::from_bits(xmm1 as u32)))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = (0x80000000u32) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 41; continue; } bb = 39; continue; }
            39 => { xmm0 = (((rcx as u32)) as u32) as u128; xmm0 = crate::simd_ops::vcvtdq2ps(xmm0); fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = ((xmm1 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) == f32::from_bits((fl_r as u32)) || f32::from_bits((fl_l as u32)).is_nan() || f32::from_bits((fl_r as u32)).is_nan()) { bb = 41; continue; } bb = 40; continue; }
            40 => { xmm1 = crate::simd_ops::vunpcklps(xmm1, xmm1); rax = (crate::simd_ops::vmovmskps(xmm1) as u32) as u32 as u64; rax = ((((rax as u32)) & (0x1u32))) as u32 as u64; rax = ((((rax as u32)) ^ (0x1u32))) as u32 as u64; fl_l = (((rcx as u32)).wrapping_add((rax as u32))) as u32 as u64; fl_r = fl_l; rcx = ((fl_l as u32)) as u32 as u64; xmm1 = (((rcx as u32)) as u32) as u128; xmm1 = crate::simd_ops::vcvtdq2ps(xmm1); bb = 41; continue; }
            41 => { rax = (crate::simd_ops::cvt_f32_i32((f32::from_bits(xmm1 as u32)))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) >= 0) { bb = 43; continue; } bb = 42; continue; }
            42 => { rax = (0xffffffffu32) as u32 as u64; bb = 44; continue; }
            43 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = ((r11 as u32)) as u32 as u64; rax = (if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { (r11 as u32) } else { (rax as u32) }) as u32 as u64; bb = 44; continue; }
            44 => { rax = (rax as u32 as i32 as i64) as u64; rcx = ((rdx.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = (r10) as u64; fl_l = (rcx) as u64 as u64; fl_r = (r10) as u64 as u64; rax = (if (((fl_l as u64)) < ((fl_r as u64))) { rcx } else { rax }) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 47; continue; } bb = 45; continue; }
            45 => { bb = 46; continue; }
            46 => { xmm4 = (xmm4 & !0xffff_ffffu128) | ((f32::from_bits(xmm4 as u32) + f32::from_bits(xmm6 as u32)).to_bits() as u128); xmm2 = (xmm9) as u128; xmm5 = (xmm5 & !0xffff_ffffu128) | ((f32::from_bits(xmm5 as u32) + f32::from_bits(xmm7 as u32)).to_bits() as u128); xmm3 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) - f32::from_bits(xmm4 as u32)).to_bits() as u128); crate::simd_ops::wu32(rbp.wrapping_sub(0x75u64), ((xmm4 as u32)) as u32); xmm1 = (xmm3) as u128; xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm3 as u32)).to_bits() as u128); xmm0 = (xmm1) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm8 as u32)).to_bits() as u128); xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) - f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm0 = (xmm12) as u128; xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) * f32::from_bits(xmm1 as u32)).to_bits() as u128); xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) - f32::from_bits(xmm10 as u32)).to_bits() as u128); xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) * f32::from_bits(xmm1 as u32)).to_bits() as u128); xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) + f32::from_bits(xmm11 as u32)).to_bits() as u128); xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) * f32::from_bits(xmm3 as u32)).to_bits() as u128); xmm3 = ((crate::simd_ops::ru32(P1)) as u32) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) - f32::from_bits(xmm2 as u32)).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm5 as u32)).to_bits() as u128); crate::simd_ops::wu32(rdx, ((xmm0 as u32)) as u32); rdx = ((rdx).wrapping_add(0x4u64)) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 46; continue; } bb = 47; continue; }
            47 => { fl_l = ((xmm4 as u32)) as u32 as u64; fl_r = ((xmm3 as u32)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 49; continue; } bb = 48; continue; }
            48 => { xmm4 = (xmm4 & !0xffff_ffffu128) | ((f32::from_bits(xmm4 as u32) + f32::from_bits(core::ptr::read_unaligned((P12) as *const u32))).to_bits() as u128); crate::simd_ops::wu32(rbp.wrapping_sub(0x75u64), ((xmm4 as u32)) as u32); bb = 49; continue; }
            49 => { fl_l = (rdx) as u64 as u64; fl_r = (r10) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 9; continue; } bb = 50; continue; }
            50 => { xmm1 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; xmm15 = (crate::simd_ops::ru128(rsp.wrapping_add(0x50u64))) as u128; xmm13 = (crate::simd_ops::ru128(rsp.wrapping_add(0x70u64))) as u128; bb = 152; continue; }
            51 => { fl_l = (rdx) as u64 as u64; fl_r = (r10) as u64 as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 151; continue; } bb = 52; continue; }
            52 => { xmm3 = ((crate::simd_ops::ru32(rbp.wrapping_sub(0x79u64))) as u32) as u128; xmm9 = (xmm14) as u128; xmm8 = ((crate::simd_ops::ru32(rbp.wrapping_sub(0x7du64))) as u32) as u128; xmm11 = ((crate::simd_ops::ru32(P8)) as u32) as u128; xmm10 = ((crate::simd_ops::ru32(P13)) as u32) as u128; xmm4 = ((crate::simd_ops::ru32(rbp.wrapping_sub(0x75u64))) as u32) as u128; xmm2 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x30u64))) as u32) as u128; xmm9 = (xmm9 & !0xffff_ffffu128) | ((f32::from_bits(xmm9 as u32) / f32::from_bits(xmm6 as u32)).to_bits() as u128); bb = 53; continue; }
            53 => { xmm1 = (xmm11) as u128; xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) - f32::from_bits(xmm4 as u32)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm9 as u32)).to_bits() as u128); rcx = (crate::simd_ops::cvt_f32_i32((f32::from_bits(xmm1 as u32)))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = (0x80000000u32) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 56; continue; } bb = 54; continue; }
            54 => { xmm0 = (((rcx as u32)) as u32) as u128; xmm0 = crate::simd_ops::vcvtdq2ps(xmm0); fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = ((xmm1 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) == f32::from_bits((fl_r as u32)) || f32::from_bits((fl_l as u32)).is_nan() || f32::from_bits((fl_r as u32)).is_nan()) { bb = 56; continue; } bb = 55; continue; }
            55 => { xmm1 = crate::simd_ops::vunpcklps(xmm1, xmm1); rax = (crate::simd_ops::vmovmskps(xmm1) as u32) as u32 as u64; rax = ((((rax as u32)) & (0x1u32))) as u32 as u64; rax = ((((rax as u32)) ^ (0x1u32))) as u32 as u64; fl_l = (((rcx as u32)).wrapping_add((rax as u32))) as u32 as u64; fl_r = fl_l; rcx = ((fl_l as u32)) as u32 as u64; xmm1 = (((rcx as u32)) as u32) as u128; xmm1 = crate::simd_ops::vcvtdq2ps(xmm1); bb = 56; continue; }
            56 => { rax = (crate::simd_ops::cvt_f32_i32((f32::from_bits(xmm1 as u32)))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) >= 0) { bb = 58; continue; } bb = 57; continue; }
            57 => { rax = (0xffffffffu32) as u32 as u64; bb = 59; continue; }
            58 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = ((r11 as u32)) as u32 as u64; rax = (if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { (r11 as u32) } else { (rax as u32) }) as u32 as u64; bb = 59; continue; }
            59 => { rax = (rax as u32 as i32 as i64) as u64; rcx = ((rdx.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = (r10) as u64; fl_l = (rcx) as u64 as u64; fl_r = (r10) as u64 as u64; rax = (if (((fl_l as u64)) < ((fl_r as u64))) { rcx } else { rax }) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 63; continue; } bb = 60; continue; }
            60 => { bb = 61; continue; }
            61 => { xmm0 = (xmm2) as u128; xmm4 = (xmm4 & !0xffff_ffffu128) | ((f32::from_bits(xmm4 as u32) + f32::from_bits(xmm6 as u32)).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm3 as u32)).to_bits() as u128); xmm5 = (xmm5 & !0xffff_ffffu128) | ((f32::from_bits(xmm5 as u32) + f32::from_bits(xmm7 as u32)).to_bits() as u128); xmm1 = (xmm4) as u128; crate::simd_ops::wu32(rbp.wrapping_sub(0x75u64), ((xmm4 as u32)) as u32); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) + f32::from_bits(xmm4 as u32)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm8 as u32)).to_bits() as u128); xmm2 = (xmm1) as u128; xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) - f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm0 = (xmm2) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm5 as u32)).to_bits() as u128); crate::simd_ops::wu32(rdx, ((xmm0 as u32)) as u32); rdx = ((rdx).wrapping_add(0x4u64)) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 61; continue; } bb = 62; continue; }
            62 => { crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((xmm2 as u32)) as u32); bb = 63; continue; }
            63 => { xmm1 = (xmm14) as u128; xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) - f32::from_bits(xmm4 as u32)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm9 as u32)).to_bits() as u128); rcx = (crate::simd_ops::cvt_f32_i32((f32::from_bits(xmm1 as u32)))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = (0x80000000u32) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 66; continue; } bb = 64; continue; }
            64 => { xmm0 = (((rcx as u32)) as u32) as u128; xmm0 = crate::simd_ops::vcvtdq2ps(xmm0); fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = ((xmm1 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) == f32::from_bits((fl_r as u32)) || f32::from_bits((fl_l as u32)).is_nan() || f32::from_bits((fl_r as u32)).is_nan()) { bb = 66; continue; } bb = 65; continue; }
            65 => { xmm1 = crate::simd_ops::vunpcklps(xmm1, xmm1); rax = (crate::simd_ops::vmovmskps(xmm1) as u32) as u32 as u64; rax = ((((rax as u32)) & (0x1u32))) as u32 as u64; rax = ((((rax as u32)) ^ (0x1u32))) as u32 as u64; fl_l = (((rcx as u32)).wrapping_add((rax as u32))) as u32 as u64; fl_r = fl_l; rcx = ((fl_l as u32)) as u32 as u64; xmm1 = (((rcx as u32)) as u32) as u128; xmm1 = crate::simd_ops::vcvtdq2ps(xmm1); bb = 66; continue; }
            66 => { rax = (crate::simd_ops::cvt_f32_i32((f32::from_bits(xmm1 as u32)))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) >= 0) { bb = 68; continue; } bb = 67; continue; }
            67 => { rax = (0xffffffffu32) as u32 as u64; bb = 69; continue; }
            68 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = ((r11 as u32)) as u32 as u64; rax = (if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { (r11 as u32) } else { (rax as u32) }) as u32 as u64; bb = 69; continue; }
            69 => { rax = (rax as u32 as i32 as i64) as u64; rcx = ((rdx.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = (r10) as u64; fl_l = (rcx) as u64 as u64; fl_r = (r10) as u64 as u64; rax = (if (((fl_l as u64)) < ((fl_r as u64))) { rcx } else { rax }) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 73; continue; } bb = 70; continue; }
            70 => { bb = 71; continue; }
            71 => { xmm0 = (xmm2) as u128; xmm4 = (xmm4 & !0xffff_ffffu128) | ((f32::from_bits(xmm4 as u32) + f32::from_bits(xmm6 as u32)).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm3 as u32)).to_bits() as u128); xmm5 = (xmm5 & !0xffff_ffffu128) | ((f32::from_bits(xmm5 as u32) + f32::from_bits(xmm7 as u32)).to_bits() as u128); xmm1 = (xmm14) as u128; xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) - f32::from_bits(xmm4 as u32)).to_bits() as u128); crate::simd_ops::wu32(rbp.wrapping_sub(0x75u64), ((xmm4 as u32)) as u32); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) + f32::from_bits(xmm1 as u32)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm8 as u32)).to_bits() as u128); xmm2 = (xmm1) as u128; xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) - f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm0 = (xmm2) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm5 as u32)).to_bits() as u128); crate::simd_ops::wu32(rdx, ((xmm0 as u32)) as u32); rdx = ((rdx).wrapping_add(0x4u64)) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 71; continue; } bb = 72; continue; }
            72 => { crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((xmm2 as u32)) as u32); bb = 73; continue; }
            73 => { fl_l = ((xmm4 as u32)) as u32 as u64; fl_r = ((xmm14 as u32)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 75; continue; } bb = 74; continue; }
            74 => { xmm4 = (xmm4 & !0xffff_ffffu128) | ((f32::from_bits(xmm4 as u32) + f32::from_bits(xmm10 as u32)).to_bits() as u128); crate::simd_ops::wu32(rbp.wrapping_sub(0x75u64), ((xmm4 as u32)) as u32); bb = 75; continue; }
            75 => { fl_l = (rdx) as u64 as u64; fl_r = (r10) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 53; continue; } bb = 76; continue; }
            76 => { xmm1 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; xmm3 = ((crate::simd_ops::ru32(P1)) as u32) as u128; bb = 152; continue; }
            77 => { fl_l = (rdx) as u64 as u64; fl_r = (r10) as u64 as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 151; continue; } bb = 78; continue; }
            78 => { xmm12 = ((crate::simd_ops::ru32(rbp.wrapping_add(0x77u64))) as u32) as u128; xmm10 = (xmm14) as u128; xmm3 = ((crate::simd_ops::ru32(rbp.wrapping_sub(0x79u64))) as u32) as u128; xmm9 = ((crate::simd_ops::ru32(rbp.wrapping_sub(0x7du64))) as u32) as u128; xmm8 = ((crate::simd_ops::ru32(P14)) as u32) as u128; xmm11 = ((crate::simd_ops::ru32(P13)) as u32) as u128; xmm4 = ((crate::simd_ops::ru32(rbp.wrapping_sub(0x75u64))) as u32) as u128; xmm2 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x30u64))) as u32) as u128; xmm10 = (xmm10 & !0xffff_ffffu128) | ((f32::from_bits(xmm10 as u32) / f32::from_bits(xmm6 as u32)).to_bits() as u128); bb = 79; continue; }
            79 => { xmm1 = (xmm12) as u128; xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) - f32::from_bits(xmm4 as u32)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm10 as u32)).to_bits() as u128); rcx = (crate::simd_ops::cvt_f32_i32((f32::from_bits(xmm1 as u32)))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = (0x80000000u32) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 82; continue; } bb = 80; continue; }
            80 => { xmm0 = (((rcx as u32)) as u32) as u128; xmm0 = crate::simd_ops::vcvtdq2ps(xmm0); fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = ((xmm1 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) == f32::from_bits((fl_r as u32)) || f32::from_bits((fl_l as u32)).is_nan() || f32::from_bits((fl_r as u32)).is_nan()) { bb = 82; continue; } bb = 81; continue; }
            81 => { xmm1 = crate::simd_ops::vunpcklps(xmm1, xmm1); rax = (crate::simd_ops::vmovmskps(xmm1) as u32) as u32 as u64; rax = ((((rax as u32)) & (0x1u32))) as u32 as u64; rax = ((((rax as u32)) ^ (0x1u32))) as u32 as u64; fl_l = (((rcx as u32)).wrapping_add((rax as u32))) as u32 as u64; fl_r = fl_l; rcx = ((fl_l as u32)) as u32 as u64; xmm1 = (((rcx as u32)) as u32) as u128; xmm1 = crate::simd_ops::vcvtdq2ps(xmm1); bb = 82; continue; }
            82 => { rax = (crate::simd_ops::cvt_f32_i32((f32::from_bits(xmm1 as u32)))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) >= 0) { bb = 84; continue; } bb = 83; continue; }
            83 => { rax = (0xffffffffu32) as u32 as u64; bb = 85; continue; }
            84 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = ((r11 as u32)) as u32 as u64; rax = (if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { (r11 as u32) } else { (rax as u32) }) as u32 as u64; bb = 85; continue; }
            85 => { rax = (rax as u32 as i32 as i64) as u64; rcx = ((rdx.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = (r10) as u64; fl_l = (rcx) as u64 as u64; fl_r = (r10) as u64 as u64; rax = (if (((fl_l as u64)) < ((fl_r as u64))) { rcx } else { rax }) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 89; continue; } bb = 86; continue; }
            86 => { bb = 87; continue; }
            87 => { xmm0 = (xmm2) as u128; xmm5 = (xmm5 & !0xffff_ffffu128) | ((f32::from_bits(xmm5 as u32) + f32::from_bits(xmm7 as u32)).to_bits() as u128); xmm1 = (xmm9) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm3 as u32)).to_bits() as u128); xmm4 = (xmm4 & !0xffff_ffffu128) | ((f32::from_bits(xmm4 as u32) + f32::from_bits(xmm6 as u32)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm5 as u32)).to_bits() as u128); crate::simd_ops::wu32(rbp.wrapping_sub(0x75u64), ((xmm4 as u32)) as u32); xmm2 = (xmm1) as u128; xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) - f32::from_bits(xmm0 as u32)).to_bits() as u128); crate::simd_ops::wu32(rdx, ((xmm2 as u32)) as u32); rdx = ((rdx).wrapping_add(0x4u64)) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 87; continue; } bb = 88; continue; }
            88 => { crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((xmm2 as u32)) as u32); bb = 89; continue; }
            89 => { xmm1 = (xmm14) as u128; xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) - f32::from_bits(xmm4 as u32)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm10 as u32)).to_bits() as u128); rcx = (crate::simd_ops::cvt_f32_i32((f32::from_bits(xmm1 as u32)))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = (0x80000000u32) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 92; continue; } bb = 90; continue; }
            90 => { xmm0 = (((rcx as u32)) as u32) as u128; xmm0 = crate::simd_ops::vcvtdq2ps(xmm0); fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = ((xmm1 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) == f32::from_bits((fl_r as u32)) || f32::from_bits((fl_l as u32)).is_nan() || f32::from_bits((fl_r as u32)).is_nan()) { bb = 92; continue; } bb = 91; continue; }
            91 => { xmm1 = crate::simd_ops::vunpcklps(xmm1, xmm1); rax = (crate::simd_ops::vmovmskps(xmm1) as u32) as u32 as u64; rax = ((((rax as u32)) & (0x1u32))) as u32 as u64; rax = ((((rax as u32)) ^ (0x1u32))) as u32 as u64; fl_l = (((rcx as u32)).wrapping_add((rax as u32))) as u32 as u64; fl_r = fl_l; rcx = ((fl_l as u32)) as u32 as u64; xmm1 = (((rcx as u32)) as u32) as u128; xmm1 = crate::simd_ops::vcvtdq2ps(xmm1); bb = 92; continue; }
            92 => { rax = (crate::simd_ops::cvt_f32_i32((f32::from_bits(xmm1 as u32)))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) >= 0) { bb = 94; continue; } bb = 93; continue; }
            93 => { rax = (0xffffffffu32) as u32 as u64; bb = 95; continue; }
            94 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = ((r11 as u32)) as u32 as u64; rax = (if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { (r11 as u32) } else { (rax as u32) }) as u32 as u64; bb = 95; continue; }
            95 => { rax = (rax as u32 as i32 as i64) as u64; rcx = ((rdx.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = (r10) as u64; fl_l = (rcx) as u64 as u64; fl_r = (r10) as u64 as u64; rax = (if (((fl_l as u64)) < ((fl_r as u64))) { rcx } else { rax }) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 99; continue; } bb = 96; continue; }
            96 => { bb = 97; continue; }
            97 => { xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) * f32::from_bits(xmm3 as u32)).to_bits() as u128); xmm4 = (xmm4 & !0xffff_ffffu128) | ((f32::from_bits(xmm4 as u32) + f32::from_bits(xmm6 as u32)).to_bits() as u128); xmm5 = (xmm5 & !0xffff_ffffu128) | ((f32::from_bits(xmm5 as u32) + f32::from_bits(xmm7 as u32)).to_bits() as u128); xmm2 = (xmm2) ^ (xmm8); crate::simd_ops::wu32(rdx, ((xmm2 as u32)) as u32); rdx = ((rdx).wrapping_add(0x4u64)) as u64; crate::simd_ops::wu32(rbp.wrapping_sub(0x75u64), ((xmm4 as u32)) as u32); fl_l = (rdx) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 97; continue; } bb = 98; continue; }
            98 => { crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((xmm2 as u32)) as u32); bb = 99; continue; }
            99 => { fl_l = ((xmm4 as u32)) as u32 as u64; fl_r = ((xmm14 as u32)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 101; continue; } bb = 100; continue; }
            100 => { xmm4 = (xmm4 & !0xffff_ffffu128) | ((f32::from_bits(xmm4 as u32) + f32::from_bits(xmm11 as u32)).to_bits() as u128); crate::simd_ops::wu32(rbp.wrapping_sub(0x75u64), ((xmm4 as u32)) as u32); bb = 101; continue; }
            101 => { fl_l = (rdx) as u64 as u64; fl_r = (r10) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 79; continue; } bb = 102; continue; }
            102 => { xmm1 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; xmm3 = ((crate::simd_ops::ru32(P1)) as u32) as u128; bb = 152; continue; }
            103 => { fl_l = (rdx) as u64 as u64; fl_r = (r10) as u64 as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 151; continue; } bb = 104; continue; }
            104 => { xmm3 = ((crate::simd_ops::ru32(rbp.wrapping_sub(0x79u64))) as u32) as u128; xmm9 = (xmm14) as u128; xmm8 = ((crate::simd_ops::ru32(rbp.wrapping_sub(0x7du64))) as u32) as u128; xmm10 = ((crate::simd_ops::ru32(P13)) as u32) as u128; xmm4 = ((crate::simd_ops::ru32(rbp.wrapping_sub(0x75u64))) as u32) as u128; xmm2 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x30u64))) as u32) as u128; xmm9 = (xmm9 & !0xffff_ffffu128) | ((f32::from_bits(xmm9 as u32) / f32::from_bits(xmm6 as u32)).to_bits() as u128); bb = 105; continue; }
            105 => { xmm1 = (xmm14) as u128; xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) - f32::from_bits(xmm4 as u32)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm9 as u32)).to_bits() as u128); rcx = (crate::simd_ops::cvt_f32_i32((f32::from_bits(xmm1 as u32)))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = (0x80000000u32) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 108; continue; } bb = 106; continue; }
            106 => { xmm0 = (((rcx as u32)) as u32) as u128; xmm0 = crate::simd_ops::vcvtdq2ps(xmm0); fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = ((xmm1 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) == f32::from_bits((fl_r as u32)) || f32::from_bits((fl_l as u32)).is_nan() || f32::from_bits((fl_r as u32)).is_nan()) { bb = 108; continue; } bb = 107; continue; }
            107 => { xmm1 = crate::simd_ops::vunpcklps(xmm1, xmm1); rax = (crate::simd_ops::vmovmskps(xmm1) as u32) as u32 as u64; rax = ((((rax as u32)) & (0x1u32))) as u32 as u64; rax = ((((rax as u32)) ^ (0x1u32))) as u32 as u64; fl_l = (((rcx as u32)).wrapping_add((rax as u32))) as u32 as u64; fl_r = fl_l; rcx = ((fl_l as u32)) as u32 as u64; xmm1 = (((rcx as u32)) as u32) as u128; xmm1 = crate::simd_ops::vcvtdq2ps(xmm1); bb = 108; continue; }
            108 => { rax = (crate::simd_ops::cvt_f32_i32((f32::from_bits(xmm1 as u32)))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) >= 0) { bb = 110; continue; } bb = 109; continue; }
            109 => { rax = (0xffffffffu32) as u32 as u64; bb = 111; continue; }
            110 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = ((r11 as u32)) as u32 as u64; rax = (if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { (r11 as u32) } else { (rax as u32) }) as u32 as u64; bb = 111; continue; }
            111 => { rax = (rax as u32 as i32 as i64) as u64; rcx = ((rdx.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = (r10) as u64; fl_l = (rcx) as u64 as u64; fl_r = (r10) as u64 as u64; rax = (if (((fl_l as u64)) < ((fl_r as u64))) { rcx } else { rax }) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 115; continue; } bb = 112; continue; }
            112 => { bb = 113; continue; }
            113 => { xmm0 = (xmm2) as u128; xmm4 = (xmm4 & !0xffff_ffffu128) | ((f32::from_bits(xmm4 as u32) + f32::from_bits(xmm6 as u32)).to_bits() as u128); xmm5 = (xmm5 & !0xffff_ffffu128) | ((f32::from_bits(xmm5 as u32) + f32::from_bits(xmm7 as u32)).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm3 as u32)).to_bits() as u128); xmm1 = (xmm4) as u128; crate::simd_ops::wu32(rbp.wrapping_sub(0x75u64), ((xmm4 as u32)) as u32); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm5 as u32)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm8 as u32)).to_bits() as u128); xmm2 = (xmm1) as u128; xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) - f32::from_bits(xmm0 as u32)).to_bits() as u128); crate::simd_ops::wu32(rdx, ((xmm2 as u32)) as u32); rdx = ((rdx).wrapping_add(0x4u64)) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 113; continue; } bb = 114; continue; }
            114 => { crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((xmm2 as u32)) as u32); bb = 115; continue; }
            115 => { fl_l = ((xmm4 as u32)) as u32 as u64; fl_r = ((xmm14 as u32)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 117; continue; } bb = 116; continue; }
            116 => { xmm4 = (xmm4 & !0xffff_ffffu128) | ((f32::from_bits(xmm4 as u32) + f32::from_bits(xmm10 as u32)).to_bits() as u128); crate::simd_ops::wu32(rbp.wrapping_sub(0x75u64), ((xmm4 as u32)) as u32); bb = 117; continue; }
            117 => { fl_l = (rdx) as u64 as u64; fl_r = (r10) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 105; continue; } bb = 118; continue; }
            118 => { xmm1 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; xmm3 = ((crate::simd_ops::ru32(P1)) as u32) as u128; bb = 152; continue; }
            119 => { fl_l = (rdx) as u64 as u64; fl_r = (r10) as u64 as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 151; continue; } bb = 120; continue; }
            120 => { xmm3 = ((crate::simd_ops::ru32(rbp.wrapping_sub(0x79u64))) as u32) as u128; xmm9 = (xmm14) as u128; xmm8 = ((crate::simd_ops::ru32(rbp.wrapping_sub(0x7du64))) as u32) as u128; xmm10 = ((crate::simd_ops::ru32(P13)) as u32) as u128; xmm4 = ((crate::simd_ops::ru32(rbp.wrapping_sub(0x75u64))) as u32) as u128; xmm2 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x30u64))) as u32) as u128; xmm9 = (xmm9 & !0xffff_ffffu128) | ((f32::from_bits(xmm9 as u32) / f32::from_bits(xmm6 as u32)).to_bits() as u128); bb = 121; continue; }
            121 => { xmm1 = (xmm14) as u128; xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) - f32::from_bits(xmm4 as u32)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm9 as u32)).to_bits() as u128); rcx = (crate::simd_ops::cvt_f32_i32((f32::from_bits(xmm1 as u32)))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = (0x80000000u32) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 124; continue; } bb = 122; continue; }
            122 => { xmm0 = (((rcx as u32)) as u32) as u128; xmm0 = crate::simd_ops::vcvtdq2ps(xmm0); fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = ((xmm1 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) == f32::from_bits((fl_r as u32)) || f32::from_bits((fl_l as u32)).is_nan() || f32::from_bits((fl_r as u32)).is_nan()) { bb = 124; continue; } bb = 123; continue; }
            123 => { xmm1 = crate::simd_ops::vunpcklps(xmm1, xmm1); rax = (crate::simd_ops::vmovmskps(xmm1) as u32) as u32 as u64; rax = ((((rax as u32)) & (0x1u32))) as u32 as u64; rax = ((((rax as u32)) ^ (0x1u32))) as u32 as u64; fl_l = (((rcx as u32)).wrapping_add((rax as u32))) as u32 as u64; fl_r = fl_l; rcx = ((fl_l as u32)) as u32 as u64; xmm1 = (((rcx as u32)) as u32) as u128; xmm1 = crate::simd_ops::vcvtdq2ps(xmm1); bb = 124; continue; }
            124 => { rax = (crate::simd_ops::cvt_f32_i32((f32::from_bits(xmm1 as u32)))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) >= 0) { bb = 126; continue; } bb = 125; continue; }
            125 => { rax = (0xffffffffu32) as u32 as u64; bb = 127; continue; }
            126 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = ((r11 as u32)) as u32 as u64; rax = (if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { (r11 as u32) } else { (rax as u32) }) as u32 as u64; bb = 127; continue; }
            127 => { rax = (rax as u32 as i32 as i64) as u64; rcx = ((rdx.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = (r10) as u64; fl_l = (rcx) as u64 as u64; fl_r = (r10) as u64 as u64; rax = (if (((fl_l as u64)) < ((fl_r as u64))) { rcx } else { rax }) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 131; continue; } bb = 128; continue; }
            128 => { bb = 129; continue; }
            129 => { xmm0 = (xmm2) as u128; xmm4 = (xmm4 & !0xffff_ffffu128) | ((f32::from_bits(xmm4 as u32) + f32::from_bits(xmm6 as u32)).to_bits() as u128); xmm1 = (xmm14) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm3 as u32)).to_bits() as u128); xmm5 = (xmm5 & !0xffff_ffffu128) | ((f32::from_bits(xmm5 as u32) + f32::from_bits(xmm7 as u32)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) - f32::from_bits(xmm4 as u32)).to_bits() as u128); crate::simd_ops::wu32(rbp.wrapping_sub(0x75u64), ((xmm4 as u32)) as u32); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm5 as u32)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm8 as u32)).to_bits() as u128); xmm2 = (xmm1) as u128; xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) - f32::from_bits(xmm0 as u32)).to_bits() as u128); crate::simd_ops::wu32(rdx, ((xmm2 as u32)) as u32); rdx = ((rdx).wrapping_add(0x4u64)) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 129; continue; } bb = 130; continue; }
            130 => { crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((xmm2 as u32)) as u32); bb = 131; continue; }
            131 => { fl_l = ((xmm4 as u32)) as u32 as u64; fl_r = ((xmm14 as u32)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 133; continue; } bb = 132; continue; }
            132 => { xmm4 = (xmm4 & !0xffff_ffffu128) | ((f32::from_bits(xmm4 as u32) + f32::from_bits(xmm10 as u32)).to_bits() as u128); crate::simd_ops::wu32(rbp.wrapping_sub(0x75u64), ((xmm4 as u32)) as u32); bb = 133; continue; }
            133 => { fl_l = (rdx) as u64 as u64; fl_r = (r10) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 121; continue; } bb = 134; continue; }
            134 => { xmm1 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; xmm3 = ((crate::simd_ops::ru32(P1)) as u32) as u128; bb = 152; continue; }
            135 => { fl_l = (rdx) as u64 as u64; fl_r = (r10) as u64 as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 151; continue; } bb = 136; continue; }
            136 => { xmm3 = ((crate::simd_ops::ru32(rbp.wrapping_sub(0x79u64))) as u32) as u128; xmm10 = (xmm14) as u128; xmm8 = ((crate::simd_ops::ru32(rbp.wrapping_sub(0x7du64))) as u32) as u128; xmm11 = ((crate::simd_ops::ru32(P13)) as u32) as u128; xmm9 = ((crate::simd_ops::ru32(P15)) as u32) as u128; r9 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x69u64))) as u64; xmm4 = ((crate::simd_ops::ru32(rbp.wrapping_sub(0x75u64))) as u32) as u128; xmm2 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x30u64))) as u32) as u128; crate::simd_ops::wu64(rsp.wrapping_add(0x108u64), (r14) as u64); r14 = (0x5851f42d4c957f2du64) as u64; xmm10 = (xmm10 & !0xffff_ffffu128) | ((f32::from_bits(xmm10 as u32) / f32::from_bits(xmm6 as u32)).to_bits() as u128); bb = 137; continue; }
            137 => { xmm1 = (xmm14) as u128; xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) - f32::from_bits(xmm4 as u32)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm10 as u32)).to_bits() as u128); rcx = (crate::simd_ops::cvt_f32_i32((f32::from_bits(xmm1 as u32)))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = (0x80000000u32) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 140; continue; } bb = 138; continue; }
            138 => { xmm0 = (((rcx as u32)) as u32) as u128; xmm0 = crate::simd_ops::vcvtdq2ps(xmm0); fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = ((xmm1 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) == f32::from_bits((fl_r as u32)) || f32::from_bits((fl_l as u32)).is_nan() || f32::from_bits((fl_r as u32)).is_nan()) { bb = 140; continue; } bb = 139; continue; }
            139 => { xmm1 = crate::simd_ops::vunpcklps(xmm1, xmm1); rax = (crate::simd_ops::vmovmskps(xmm1) as u32) as u32 as u64; rax = ((((rax as u32)) & (0x1u32))) as u32 as u64; rax = ((((rax as u32)) ^ (0x1u32))) as u32 as u64; fl_l = (((rcx as u32)).wrapping_add((rax as u32))) as u32 as u64; fl_r = fl_l; rcx = ((fl_l as u32)) as u32 as u64; xmm1 = (((rcx as u32)) as u32) as u128; xmm1 = crate::simd_ops::vcvtdq2ps(xmm1); bb = 140; continue; }
            140 => { rax = (crate::simd_ops::cvt_f32_i32((f32::from_bits(xmm1 as u32)))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) >= 0) { bb = 142; continue; } bb = 141; continue; }
            141 => { rax = (0xffffffffu32) as u32 as u64; bb = 143; continue; }
            142 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = ((r11 as u32)) as u32 as u64; rax = (if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { (r11 as u32) } else { (rax as u32) }) as u32 as u64; bb = 143; continue; }
            143 => { rax = (rax as u32 as i32 as i64) as u64; r8 = (r10) as u64; rcx = ((rdx.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; fl_l = (rcx) as u64 as u64; fl_r = (r10) as u64 as u64; r8 = (if (((fl_l as u64)) < ((fl_r as u64))) { rcx } else { r8 }) as u64; fl_l = (rdx) as u64 as u64; fl_r = (r8) as u64 as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 147; continue; } bb = 144; continue; }
            144 => { rcx = (r9) as u64; fl_l = ((rcx).wrapping_shr(33u32)) as u64 as u64; rcx = ((fl_l as u64)) as u64; bb = 145; continue; }
            145 => { xmm1 = 0u128; rax = ((rcx as u32)) as u32 as u64; xmm0 = (xmm2) as u128; xmm5 = (xmm5 & !0xffff_ffffu128) | ((f32::from_bits(xmm5 as u32) + f32::from_bits(xmm7 as u32)).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm3 as u32)).to_bits() as u128); xmm4 = (xmm4 & !0xffff_ffffu128) | ((f32::from_bits(xmm4 as u32) + f32::from_bits(xmm6 as u32)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((((rax) as i64) as f32).to_bits() as u128); crate::simd_ops::wu32(rbp.wrapping_sub(0x75u64), ((xmm4 as u32)) as u32); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm9 as u32)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm5 as u32)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm8 as u32)).to_bits() as u128); xmm2 = (xmm1) as u128; xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) - f32::from_bits(xmm0 as u32)).to_bits() as u128); crate::simd_ops::wu32(rdx, ((xmm2 as u32)) as u32); rdx = ((rdx).wrapping_add(0x4u64)) as u64; fl_l = (rdx) as u64 as u64; fl_r = (r8) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 145; continue; } bb = 146; continue; }
            146 => { crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((xmm2 as u32)) as u32); bb = 147; continue; }
            147 => { fl_l = ((xmm4 as u32)) as u32 as u64; fl_r = ((xmm14 as u32)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 149; continue; } bb = 148; continue; }
            148 => { r9 = (((r9) as i64).wrapping_mul((r14) as i64) as u64) as u64; xmm4 = (xmm4 & !0xffff_ffffu128) | ((f32::from_bits(xmm4 as u32) + f32::from_bits(xmm11 as u32)).to_bits() as u128); fl_l = ((r9).wrapping_add(1)) as u64 as u64; r9 = ((fl_l as u64)) as u64; crate::simd_ops::wu32(rbp.wrapping_sub(0x75u64), ((xmm4 as u32)) as u32); crate::simd_ops::wu64(rbp.wrapping_sub(0x69u64), (r9) as u64); bb = 149; continue; }
            149 => { fl_l = (rdx) as u64 as u64; fl_r = (r10) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 137; continue; } bb = 150; continue; }
            150 => { xmm1 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x71u64))) as u128; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x108u64))) as u64; xmm3 = ((crate::simd_ops::ru32(P1)) as u32) as u128; bb = 152; continue; }
            151 => { xmm4 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; xmm4 = crate::simd_ops::vshufps(xmm4, xmm4, 0xffu8); bb = 152; continue; }
            152 => { xmm12 = (crate::simd_ops::ru128(rsp.wrapping_add(0x80u64))) as u128; fl_l = ((rsi as u8)) as u8 as u64; fl_r = ((rsi as u8)) as u8 as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x100u64))) as u64; xmm11 = (crate::simd_ops::ru128(rsp.wrapping_add(0x90u64))) as u128; xmm10 = (crate::simd_ops::ru128(rsp.wrapping_add(0xa0u64))) as u128; xmm9 = (crate::simd_ops::ru128(rsp.wrapping_add(0xb0u64))) as u128; xmm8 = (crate::simd_ops::ru128(rsp.wrapping_add(0xc0u64))) as u128; xmm7 = (crate::simd_ops::ru128(rsp.wrapping_add(0xd0u64))) as u128; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0xe0u64))) as u128; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 158; continue; } bb = 153; continue; }
            153 => { xmm2 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; crate::simd_ops::wu128(rbx, (xmm2) as u128); crate::simd_ops::wu128(rbx.wrapping_add(0x10u64), (xmm1) as u128); fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 155; continue; } bb = 154; continue; }
            154 => { xmm1 = (xmm3) as u128; bb = 157; continue; }
            155 => { fl_l = ((rdi as u32)) as u32 as u64; fl_r = (0x5u32) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 158; continue; } bb = 156; continue; }
            156 => { xmm1 = (xmm14) as u128; bb = 157; continue; }
            157 => { xmm0 = (xmm4) as u128; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P16) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu32(rbx.wrapping_add(0xcu64), ((xmm0 as u32)) as u32); bb = 158; continue; }
            158 => { xmm14 = (crate::simd_ops::ru128(rsp.wrapping_add(0x60u64))) as u128; r11 = ((rsp.wrapping_add(0xf0u64)) as u64) as u64; rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rdi = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; rsp = (r11) as u64; rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            0xffff_fffe => { match j_tgt.wrapping_sub(0x141624720u64) { 0x6b => { bb = 1; continue; } 0x70 => { bb = 2; continue; } 0x73 => { bb = 3; continue; } 0x80 => { bb = 4; continue; } 0x94 => { bb = 5; continue; } 0xe8 => { bb = 6; continue; } 0xff => { bb = 7; continue; } 0x108 => { bb = 8; continue; } 0x160 => { bb = 9; continue; } 0x17d => { bb = 10; continue; } 0x189 => { bb = 11; continue; } 0x19e => { bb = 12; continue; } 0x1a6 => { bb = 13; continue; } 0x1ad => { bb = 14; continue; } 0x1b4 => { bb = 15; continue; } 0x1c9 => { bb = 16; continue; } 0x1d0 => { bb = 17; continue; } 0x220 => { bb = 18; continue; } 0x239 => { bb = 19; continue; } 0x245 => { bb = 20; continue; } 0x25a => { bb = 21; continue; } 0x262 => { bb = 22; continue; } 0x269 => { bb = 23; continue; } 0x270 => { bb = 24; continue; } 0x285 => { bb = 25; continue; } 0x290 => { bb = 26; continue; } 0x2e8 => { bb = 27; continue; } 0x2f0 => { bb = 28; continue; } 0x30d => { bb = 29; continue; } 0x319 => { bb = 30; continue; } 0x32e => { bb = 31; continue; } 0x336 => { bb = 32; continue; } 0x33d => { bb = 33; continue; } 0x344 => { bb = 34; continue; } 0x359 => { bb = 35; continue; } 0x360 => { bb = 36; continue; } 0x3bb => { bb = 37; continue; } 0x3c3 => { bb = 38; continue; } 0x3db => { bb = 39; continue; } 0x3e7 => { bb = 40; continue; } 0x3fc => { bb = 41; continue; } 0x404 => { bb = 42; continue; } 0x40b => { bb = 43; continue; } 0x412 => { bb = 44; continue; } 0x427 => { bb = 45; continue; } 0x430 => { bb = 46; continue; } 0x48f => { bb = 47; continue; } 0x494 => { bb = 48; continue; } 0x4a1 => { bb = 49; continue; } 0x4aa => { bb = 50; continue; } 0x4c0 => { bb = 51; continue; } 0x4c9 => { bb = 52; continue; } 0x500 => { bb = 53; continue; } 0x519 => { bb = 54; continue; } 0x525 => { bb = 55; continue; } 0x53a => { bb = 56; continue; } 0x542 => { bb = 57; continue; } 0x549 => { bb = 58; continue; } 0x550 => { bb = 59; continue; } 0x565 => { bb = 60; continue; } 0x570 => { bb = 61; continue; } 0x5ab => { bb = 62; continue; } 0x5b1 => { bb = 63; continue; } 0x5ca => { bb = 64; continue; } 0x5d6 => { bb = 65; continue; } 0x5eb => { bb = 66; continue; } 0x5f3 => { bb = 67; continue; } 0x5fa => { bb = 68; continue; } 0x601 => { bb = 69; continue; } 0x616 => { bb = 70; continue; } 0x620 => { bb = 71; continue; } 0x660 => { bb = 72; continue; } 0x666 => { bb = 73; continue; } 0x66c => { bb = 74; continue; } 0x676 => { bb = 75; continue; } 0x67f => { bb = 76; continue; } 0x691 => { bb = 77; continue; } 0x69a => { bb = 78; continue; } 0x6d1 => { bb = 79; continue; } 0x6ea => { bb = 80; continue; } 0x6f6 => { bb = 81; continue; } 0x70b => { bb = 82; continue; } 0x713 => { bb = 83; continue; } 0x71a => { bb = 84; continue; } 0x721 => { bb = 85; continue; } 0x736 => { bb = 86; continue; } 0x740 => { bb = 87; continue; } 0x770 => { bb = 88; continue; } 0x776 => { bb = 89; continue; } 0x78f => { bb = 90; continue; } 0x79b => { bb = 91; continue; } 0x7b0 => { bb = 92; continue; } 0x7b8 => { bb = 93; continue; } 0x7bf => { bb = 94; continue; } 0x7c6 => { bb = 95; continue; } 0x7db => { bb = 96; continue; } 0x7e0 => { bb = 97; continue; } 0x802 => { bb = 98; continue; } 0x808 => { bb = 99; continue; } 0x80e => { bb = 100; continue; } 0x818 => { bb = 101; continue; } 0x821 => { bb = 102; continue; } 0x833 => { bb = 103; continue; } 0x83c => { bb = 104; continue; } 0x870 => { bb = 105; continue; } 0x889 => { bb = 106; continue; } 0x895 => { bb = 107; continue; } 0x8aa => { bb = 108; continue; } 0x8b2 => { bb = 109; continue; } 0x8b9 => { bb = 110; continue; } 0x8c0 => { bb = 111; continue; } 0x8d5 => { bb = 112; continue; } 0x8e0 => { bb = 113; continue; } 0x914 => { bb = 114; continue; } 0x91a => { bb = 115; continue; } 0x920 => { bb = 116; continue; } 0x92a => { bb = 117; continue; } 0x933 => { bb = 118; continue; } 0x945 => { bb = 119; continue; } 0x94e => { bb = 120; continue; } 0x980 => { bb = 121; continue; } 0x999 => { bb = 122; continue; } 0x9a5 => { bb = 123; continue; } 0x9ba => { bb = 124; continue; } 0x9c2 => { bb = 125; continue; } 0x9c9 => { bb = 126; continue; } 0x9d0 => { bb = 127; continue; } 0x9e5 => { bb = 128; continue; } 0x9f0 => { bb = 129; continue; } 0xa29 => { bb = 130; continue; } 0xa2f => { bb = 131; continue; } 0xa35 => { bb = 132; continue; } 0xa3f => { bb = 133; continue; } 0xa48 => { bb = 134; continue; } 0xa5a => { bb = 135; continue; } 0xa63 => { bb = 136; continue; } 0xab0 => { bb = 137; continue; } 0xac9 => { bb = 138; continue; } 0xad5 => { bb = 139; continue; } 0xaea => { bb = 140; continue; } 0xaf2 => { bb = 141; continue; } 0xaf9 => { bb = 142; continue; } 0xb00 => { bb = 143; continue; } 0xb15 => { bb = 144; continue; } 0xb20 => { bb = 145; continue; } 0xb60 => { bb = 146; continue; } 0xb66 => { bb = 147; continue; } 0xb6c => { bb = 148; continue; } 0xb81 => { bb = 149; continue; } 0xb8a => { bb = 150; continue; } 0xba0 => { bb = 151; continue; } 0xba9 => { bb = 152; continue; } 0xbf3 => { bb = 153; continue; } 0xc03 => { bb = 154; continue; } 0xc08 => { bb = 155; continue; } 0xc0d => { bb = 156; continue; } 0xc11 => { bb = 157; continue; } 0xc1e => { bb = 158; continue; } _ => { let s5 = crate::simd_ops::read_s5_tail(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x40) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(j_tgt, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::set_xmm_ret(c_x0); return c_rax; } } }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141625380(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1416195b0; const P1: u64 = 0x1415970c0; const P2: u64 = 0x141625720; const P3: u64 = 0x14160d400;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rbx); fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(0xc0u64))) as u32 as u64; fl_r = 0x3; rdi = rcx; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 14; continue; } bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x68u64))) as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 14; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(0xc4u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 6; continue; } bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(0xdbu64))) as u8 as u64; fl_r = ((rsi as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 14; continue; } bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x30u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 5; continue; }
            5 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((rsi as u32)) as u32); rdx = rsp.wrapping_add(0x30); crate::simd_ops::wu32(rsp.wrapping_add(0x34u64), (0x4u32) as u32); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 14; continue; }
            6 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 12; continue; } bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x78u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 14; continue; } bb = 8; continue; }
            8 => { rbx = (crate::simd_ops::ru64(rdi.wrapping_add(0x70u64))) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((rsi as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x34u64), (0x4u32) as u32); rcx = ((rbx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 14; continue; } bb = 9; continue; }
            9 => { bb = 10; continue; }
            10 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; rdx = rsp.wrapping_add(0x30); r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x78u64))) as u32 as u64; rbx = ((rbx).wrapping_add(0x8u64)) as u64; rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x70u64))) as u64; rcx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 10; continue; } bb = 11; continue; }
            11 => { bb = 14; continue; }
            12 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 14; continue; } bb = 13; continue; }
            13 => { rcx = rdi; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 14; continue; }
            14 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x88u64))) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 18; continue; } bb = 15; continue; }
            15 => { rdx = (crate::simd_ops::ru32(rdi.wrapping_add(0x88u64))) as u32 as u64; rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x80u64))) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(rdx.wrapping_mul(8)).wrapping_sub(0x8u64))) as u64; rax = ((rdx.wrapping_sub(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x88u64), ((rax as u32)) as u32); rcx = ((rcx).wrapping_add(0x10u64)) as u64; rax = 0xffffffff; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rcx.wrapping_add(0x14u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 17; continue; } bb = 16; continue; }
            16 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 17; continue; }
            17 => { fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x88u64))) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 15; continue; } bb = 18; continue; }
            18 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0xa0u64))) as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x88u64), ((rsi as u32)) as u32); fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 20; continue; } bb = 19; continue; }
            19 => { crate::simd_ops::wu64(rdi.wrapping_add(0xa0u64), rsi); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 20; continue; }
            20 => { crate::simd_ops::wu64(rdi.wrapping_add(0xa8u64), rsi); rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416254d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x140000000; const P1: u64 = 0x1415c1600; const P2: u64 = 0x141624150; const P3: u64 = 0x1434264e4;
    let (mut rax, mut fl_l, mut fl_r, mut j_tgt) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x28u64)) as u64; rdx = (((rdx as u32)).wrapping_add(0xfffffff8u32)) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x5; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 10; continue; } bb = 1; continue; }
            1 => { rax = ((((rdx as u32)) as i32 as i64) as u64) as u64; r8 = P0; rdx = (crate::simd_ops::ru32(r8.wrapping_add(rax.wrapping_mul(4)).wrapping_add(0x16255c0u64))) as u32 as u64; fl_l = ((rdx).wrapping_add(r8)) as u64; fl_r = fl_l; rdx = ((fl_l as u64)) as u64; j_tgt = rdx; bb = 0xffff_fffe; continue; }
            2 => { xmm0 = 0u128; xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | (((f32::from_bits(xmm2 as u32)) as f64).to_bits() as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(rcx.wrapping_add(0xfcu64), ((rax as u32)) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); rsp = ((rsp).wrapping_add(0x28u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            3 => { xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) * f32::from_bits(crate::simd_ops::ru32(P3))).to_bits() as u128); rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); crate::simd_ops::wu32(rcx.wrapping_add(0x100u64), ((xmm2 as u32)) as u32); rsp = ((rsp).wrapping_add(0x28u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            4 => { xmm0 = 0u128; xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | (((f32::from_bits(xmm2 as u32)) as f64).to_bits() as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(rcx.wrapping_add(0x108u64), ((rax as u32)) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); rsp = ((rsp).wrapping_add(0x28u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            5 => { xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) * f32::from_bits(crate::simd_ops::ru32(P3))).to_bits() as u128); rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); crate::simd_ops::wu32(rcx.wrapping_add(0x104u64), ((xmm2 as u32)) as u32); rsp = ((rsp).wrapping_add(0x28u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            6 => { xmm0 = 0u128; fl_l = ((xmm2 as u32)) as u32 as u64; fl_r = ((xmm0 as u32)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 8; continue; } bb = 7; continue; }
            7 => { xmm0 = 0u128; xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | (((f32::from_bits(xmm2 as u32)) as f64).to_bits() as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (((rax as u32)).wrapping_add(crate::simd_ops::ru32(rcx.wrapping_add(0x108u64)))) as u32 as u64; rax = (((rax as u32)).wrapping_add(crate::simd_ops::ru32(rcx.wrapping_add(0xfcu64)))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(0xb4u64), ((rax as u32)) as u32); rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); rsp = ((rsp).wrapping_add(0x28u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            8 => { rax = 0x7fffffff; crate::simd_ops::wu32(rcx.wrapping_add(0xb4u64), ((rax as u32)) as u32); rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); rsp = ((rsp).wrapping_add(0x28u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            9 => { xmm0 = 0u128; xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | (((f32::from_bits(xmm2 as u32)) as f64).to_bits() as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(rcx.wrapping_add(0x10cu64), ((rax as u32)) as u32); rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); rsp = ((rsp).wrapping_add(0x28u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            10 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x28u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            0xffff_fffe => { match j_tgt.wrapping_sub(0x1416254d0u64) { 0x10 => { bb = 1; continue; } 0x27 => { bb = 2; continue; } 0x45 => { bb = 3; continue; } 0x5c => { bb = 4; continue; } 0x7a => { bb = 5; continue; } 0x91 => { bb = 6; continue; } 0x99 => { bb = 7; continue; } 0xbe => { bb = 8; continue; } 0xd0 => { bb = 9; continue; } 0xe9 => { bb = 10; continue; } _ => { return crate::simd_ops::tail_stk_xmm(j_tgt, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); } } }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416255e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x140000000; const P1: u64 = 0x1434264e4; const P2: u64 = 0x1415c1600; const P3: u64 = 0x1438152b4; const P4: u64 = 0x1438152ac; const P5: u64 = 0x14342668c;
    let (mut rax, mut fl_l, mut fl_r, mut j_tgt) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x28u64)) as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = (0x5u32) as u32 as u64; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 11; continue; } bb = 1; continue; }
            1 => { rax = ((((rdx as u32)) as i32 as i64) as u64) as u64; r8 = ((P0) as u64) as u64; rdx = (crate::simd_ops::ru32(r8.wrapping_add(rax.wrapping_mul(4)).wrapping_add(0x16256e8u64))) as u32 as u64; fl_l = ((rdx).wrapping_add(r8)) as u64 as u64; fl_r = fl_l; rdx = ((fl_l as u64)) as u64; j_tgt = (rdx) as u64; bb = 0xffff_fffe; continue; }
            2 => { xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) * f32::from_bits(core::ptr::read_unaligned((P1) as *const u32))).to_bits() as u128); rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); crate::simd_ops::wu32(rcx.wrapping_add(0xf8u64), ((xmm2 as u32)) as u32); rsp = ((rsp).wrapping_add(0x28u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            3 => { xmm0 = 0u128; xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | (((f32::from_bits(xmm2 as u32)) as f64).to_bits() as u128); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu32(rcx.wrapping_add(0xfcu64), ((rax as u32)) as u32); rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); rsp = ((rsp).wrapping_add(0x28u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            4 => { fl_l = (crate::simd_ops::ru16(rcx.wrapping_add(0xd8u64))) as u16 as u64; fl_r = (0x0u16) as u16 as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x108u64), ((xmm2 as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x138u64), (0x1u8) as u8); if ((((fl_l as u16)) as i16) <= (((fl_r as u16)) as i16)) { bb = 6; continue; } bb = 5; continue; }
            5 => { rdx = ((crate::simd_ops::ru16(P3)) as u32) as u32 as u64; xmm1 = 0u128; rax = (crate::simd_ops::ru32(P4)) as u32 as u64; xmm0 = (((rdx as u32)) as u32) as u128; xmm1 = (xmm1 & !0xffff_ffffu128) | ((((rax) as i64) as f32).to_bits() as u128); xmm0 = crate::simd_ops::vcvtdq2ps(xmm0); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) / f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(core::ptr::read_unaligned((P5) as *const u32))).to_bits() as u128); fl_l = ((xmm2 as u32)) as u32 as u64; fl_r = ((xmm1 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 7; continue; }
            7 => { crate::simd_ops::wu32(rcx.wrapping_add(0xd0u64), ((rdx as u32)) as u32); rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); rsp = ((rsp).wrapping_add(0x28u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            8 => { rax = (crate::simd_ops::cvt_f32_i64((f32::from_bits(xmm2 as u32)))) as u64; crate::simd_ops::wu8(rcx.wrapping_add(0x138u64), (0x1u8) as u8); crate::simd_ops::wu32(rcx.wrapping_add(0x104u64), ((rax as u32)) as u32); rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); rsp = ((rsp).wrapping_add(0x28u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            9 => { xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) * f32::from_bits(core::ptr::read_unaligned((P1) as *const u32))).to_bits() as u128); rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); crate::simd_ops::wu8(rcx.wrapping_add(0x138u64), (0x1u8) as u8); crate::simd_ops::wu32(rcx.wrapping_add(0x10cu64), ((xmm2 as u32)) as u32); rsp = ((rsp).wrapping_add(0x28u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            10 => { xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) * f32::from_bits(core::ptr::read_unaligned((P1) as *const u32))).to_bits() as u128); rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); crate::simd_ops::wu8(rcx.wrapping_add(0x138u64), (0x1u8) as u8); crate::simd_ops::wu32(rcx.wrapping_add(0x110u64), ((xmm2 as u32)) as u32); rsp = ((rsp).wrapping_add(0x28u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            11 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x28u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            0xffff_fffe => { match j_tgt.wrapping_sub(0x1416255e0u64) { 0xd => { bb = 1; continue; } 0x24 => { bb = 2; continue; } 0x3b => { bb = 3; continue; } 0x54 => { bb = 4; continue; } 0x6d => { bb = 5; continue; } 0x9a => { bb = 6; continue; } 0x9c => { bb = 7; continue; } 0xa9 => { bb = 8; continue; } 0xc2 => { bb = 9; continue; } 0xe0 => { bb = 10; continue; } 0xfe => { bb = 11; continue; } _ => { let s5 = crate::simd_ops::read_s5_tail(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x40) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(j_tgt, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::set_xmm_ret(c_x0); return c_rax; } } }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141625700(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x100;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0xe; rdx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 3; continue; } bb = 1; continue; }
            1 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 4; continue; } bb = 2; continue; }
            2 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; return rax; }
            3 => { crate::simd_ops::wu32(rcx.wrapping_add(P0), ((xmm2 as u32)) as u32); bb = 4; continue; }
            4 => { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141625720(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415c2850; const P1: u64 = 0x1415970c0; const P2: u64 = 0x141611720; const P3: u64 = 0x143930f08; const P4: u64 = 0x1415a5db0; const P5: u64 = 0x1415ab5b0; const P6: u64 = 0x1415a34b0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x88u64))) as u32 as u64; rsi = rcx; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 17; continue; } bb = 1; continue; }
            1 => { rbx = (crate::simd_ops::ru64(rcx.wrapping_add(0xa0u64))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 2; continue; }
            2 => { rbx = (crate::simd_ops::ru64(rbx.wrapping_add(0x48u64))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 3; continue; }
            3 => { rdi = 0; fl_l = 0; fl_r = 0; bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x90u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 5; continue; }
            5 => { rdx = rsi; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rdx = rsp.wrapping_add(0x50); r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); crate::simd_ops::wu32(rsp.wrapping_add(0x50u64), ((rdi as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x54u64), (0x4u32) as u32); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 7; continue; }
            7 => { rbx = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 4; continue; } bb = 8; continue; }
            8 => { rdi = (crate::simd_ops::ru64(rsi.wrapping_add(0xa0u64))) as u64; rdi = (crate::simd_ops::ru64(rdi.wrapping_add(0x70u64))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 21; continue; } bb = 9; continue; }
            9 => { bb = 10; continue; }
            10 => { rbx = (crate::simd_ops::ru64(rsi.wrapping_add(0x80u64))) as u64; rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x88u64))) as u32 as u64; rcx = ((rbx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 15; continue; } bb = 11; continue; }
            11 => { bb = 12; continue; }
            12 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; rdx = rsi; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 14; continue; } bb = 13; continue; }
            13 => { r8 = (crate::simd_ops::ru64(rbx)) as u64; r9 = 0x3; rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; rcx = (crate::simd_ops::ru64(P3)) as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 14; continue; }
            14 => { rcx = (crate::simd_ops::ru32(rsi.wrapping_add(0x88u64))) as u32 as u64; rbx = ((rbx).wrapping_add(0x8u64)) as u64; rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x80u64))) as u64; rcx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 12; continue; } bb = 15; continue; }
            15 => { rdi = (crate::simd_ops::ru64(rdi)) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 10; continue; } bb = 16; continue; }
            16 => { rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            17 => { rbx = (crate::simd_ops::ru64(rcx.wrapping_add(0x80u64))) as u64; rcx = ((rbx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 21; continue; } bb = 18; continue; }
            18 => { crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), r14); bb = 19; continue; }
            19 => { r14 = (crate::simd_ops::ru64(rbx)) as u64; rbp = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; rdx = r14; rdi = (crate::simd_ops::ru32(rsi.wrapping_add(0x10u64))) as u32 as u64; r8 = rbp; rcx = (crate::simd_ops::ru64(P3)) as u64; r9 = ((rdi as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); r9 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x4u32) as u32); r8 = ((rdi as u32)) as u32 as u64; rdx = rbp; rcx = r14; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru32(rsi.wrapping_add(0x88u64))) as u32 as u64; rbx = ((rbx).wrapping_add(0x8u64)) as u64; rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x80u64))) as u64; rcx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 19; continue; } bb = 20; continue; }
            20 => { r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; bb = 21; continue; }
            21 => { rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416258c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xbc; const P1: u64 = 0x68; const P2: u64 = 0x141618ad0; const P3: u64 = 0x141625380; const P4: u64 = 0x1415ccba0; const P5: u64 = 0x98; const P6: u64 = 0x80; const P7: u64 = 0x88; const P8: u64 = 0x1415242e0; const P9: u64 = 0x8c; const P10: u64 = 0x70; const P11: u64 = 0x78;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; fl_r = 0x0; rbx = rcx; if ((((fl_l as u32)) as i32) <= (((fl_r as u32)) as i32)) { bb = 2; continue; } bb = 1; continue; }
            1 => { rdx = rcx; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            2 => { rcx = rbx; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rdi = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rbx.wrapping_add(P5), ((rdi as u32)) as u32); rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P6))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rbx.wrapping_add(P7), ((rdi as u32)) as u32); rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(P6), rdi); crate::simd_ops::wu32(rbx.wrapping_add(P9), ((rdi as u32)) as u32); bb = 4; continue; }
            4 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P10))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rbx.wrapping_add(P11), ((rdi as u32)) as u32); rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(0x70u64), rdi); crate::simd_ops::wu32(rbx.wrapping_add(0x7cu64), ((rdi as u32)) as u32); bb = 6; continue; }
            6 => { crate::simd_ops::wu32(rbx.wrapping_add(0xc0u64), (0x4u32) as u32); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141625950(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0xc4; const P1: u64 = 0xcc; const P2: u64 = 0xd4; const P3: u64 = 0xa8; const P4: u64 = 0x8; const P5: u64 = 0xc0; const P6: u64 = 0xc; const P7: u64 = 0xc8; const P8: u64 = 0xda; const P9: u64 = 0xb8;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; fl_r = 0x3; r8 = ((rdx as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64; fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(P2))) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { bb = 7; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(P3))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 3; continue; }
            3 => { rdx = (crate::simd_ops::ru32(rax.wrapping_add(P4))) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0xffffffff; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 5; continue; } bb = 4; continue; }
            4 => { crate::simd_ops::wu32(rcx.wrapping_add(P5), ((rdx as u32)) as u32); bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru32(rax.wrapping_add(P6))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(P7), ((rax as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(P8), (0x1u8) as u8); bb = 6; continue; }
            6 => { fl_l = (crate::simd_ops::lk_add_u32(rcx.wrapping_add(P9), ((r8 as u32)) as u32)) as u32 as u64; fl_r = fl_l; bb = 7; continue; }
            7 => { return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416259a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x104; const P1: u64 = 0x108; const P2: u64 = 0x100; const P3: u64 = 0x143426780; const P4: u64 = 0xc4; const P5: u64 = 0xcc; const P6: u64 = 0xd4; const P7: u64 = 0xa8; const P8: u64 = 0x8; const P9: u64 = 0xc0; const P10: u64 = 0xc; const P11: u64 = 0xc8;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; r8 = 0; fl_l = 0; fl_r = 0; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (((rax as u32)).wrapping_sub((rdx as u32))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(P0), ((rax as u32)) as u32); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) > 0) { bb = 4; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(P1))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; xmm0 = 0u128; rax = ((rdx as u32)) as u32 as u64; xmm0 = (xmm0 & !0xffff_ffffu128) | ((((rax) as i64) as f32).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(crate::simd_ops::ru32(rcx.wrapping_add(P2)))).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(crate::simd_ops::ru32(P3))).to_bits() as u128); r8 = (crate::simd_ops::cvt_f32_i64((f32::from_bits(xmm0 as u32)))) as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 4; continue; } bb = 3; continue; }
            3 => { r8 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu8(rcx.wrapping_add(P1), (0x1u8) as u8); bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(P4))) as u32 as u64; fl_r = 0x3; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru32(rcx.wrapping_add(P5))) as u32 as u64; fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(P6))) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { bb = 11; continue; } bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(P7))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 7; continue; }
            7 => { rdx = (crate::simd_ops::ru32(rax.wrapping_add(P8))) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0xffffffff; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 9; continue; } bb = 8; continue; }
            8 => { crate::simd_ops::wu32(rcx.wrapping_add(P9), ((rdx as u32)) as u32); bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru32(rax.wrapping_add(P10))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(P11), ((rax as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0xdau64), (0x1u8) as u8); bb = 10; continue; }
            10 => { fl_l = (crate::simd_ops::lk_add_u32(rcx.wrapping_add(0xb8u64), ((r8 as u32)) as u32)) as u32 as u64; fl_r = fl_l; bb = 11; continue; }
            11 => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141625a40(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141625380; const P1: u64 = 0x14153e940; const P2: u64 = 0x1416240d0; const P3: u64 = 0x14160d040; const P4: u64 = 0x1415242e0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(0xa0u64))) as u64; rsi = r8; rdi = rdx; rbx = rcx; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { fl_l = rax; fl_r = (crate::simd_ops::ru64(rdx.wrapping_add(0x18u64))) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 3; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu32(rcx.wrapping_add(0xc0u64), (0x3u32) as u32); rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); bb = 3; continue; }
            3 => { rbp = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rbx.wrapping_add(0xc0u64), (0x1u32) as u32); crate::simd_ops::wu32(rbx.wrapping_add(0xb8u64), ((rbp as u32)) as u32); rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x28u64))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0xb0u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0xc4u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 10; continue; } bb = 4; continue; }
            4 => { rdi = (crate::simd_ops::ru64(rdi.wrapping_add(0x30u64))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 18; continue; } bb = 5; continue; }
            5 => { rsi = (crate::simd_ops::ru32(rbx.wrapping_add(0x78u64))) as u32 as u64; fl_l = ((rsi as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rbx.wrapping_add(0x7cu64))) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 8; continue; } bb = 6; continue; }
            6 => { rcx = rbx.wrapping_add(0x70); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 18; continue; } bb = 7; continue; }
            7 => { fl_l = ((rsi as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rbx.wrapping_add(0x7cu64))) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 18; continue; } bb = 8; continue; }
            8 => { rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x78u64))) as u32 as u64; rax = ((rcx.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x78u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x70u64))) as u64; rdx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 18; continue; } bb = 9; continue; }
            9 => { crate::simd_ops::wu64(rdx, rdi); bb = 18; continue; }
            10 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 12; continue; } bb = 11; continue; }
            11 => { rdx = rsi; rcx = rbx; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 18; continue; }
            12 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 16; continue; } bb = 13; continue; }
            13 => { rdx = rsi; rcx = rbx; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(0xa0u64))) as u64; fl_r = rbp; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 18; continue; } bb = 14; continue; }
            14 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x18u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 18; continue; } bb = 15; continue; }
            15 => { crate::simd_ops::wu64(rbx.wrapping_add(0xa0u64), rcx); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 18; continue; }
            16 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x70u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 18; continue; } bb = 17; continue; }
            17 => { rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rbx.wrapping_add(0x78u64), ((rbp as u32)) as u32); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(0x70u64), rbp); crate::simd_ops::wu32(rbx.wrapping_add(0x7cu64), ((rbp as u32)) as u32); bb = 18; continue; }
            18 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = 0x1; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141625b60(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0xc0; const P1: u64 = 0xb4; const P2: u64 = 0xb8;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64;
    r8 = rcx;
    rax = (((rax as u32)).wrapping_sub(0x3u32)) as u32 as u64;
    fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1;
    if (((fl_l as u32)) <= ((fl_r as u32))) { return rax; }
    fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64; fl_r = 0x7fffffff;
    if !((((fl_l as u32)) != ((fl_r as u32)))) { rcx = (crate::simd_ops::ru32(rcx.wrapping_add(P2))) as u32 as u64; fl_l = (((rcx as u32)).wrapping_add((rdx as u32))) as u32 as u64; fl_r = fl_l; rcx = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(P0), (0x2u32) as u32); crate::simd_ops::wu32(r8.wrapping_add(P1), ((rcx as u32)) as u32); }
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141625ba0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x11; const P2: u64 = 0xc; const P3: u64 = 0x10; const P4: u64 = 0x141625ba0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x50u64)) as u64; rbx = (crate::simd_ops::ru64(rcx)) as u64; rsi = rdx; rax = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; rdi = rcx; rax = ((rax).wrapping_shl(4u32)) as u64; rax = ((rax).wrapping_add(rbx)) as u64; fl_l = rbx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 5; continue; } bb = 1; continue; }
            1 => { rax = ((crate::simd_ops::ru8(rdi.wrapping_add(P1))) as u32) as u32 as u64; fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(P0))) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; xmm0 = ((crate::simd_ops::ru32(rsi.wrapping_add(P2))) as u32) as u128; rdx = (crate::simd_ops::ru64(rsi)) as u64; xmm3 = ((crate::simd_ops::ru32(rsi.wrapping_add(P0))) as u32) as u128; r8 = (crate::simd_ops::ru64(rsi.wrapping_add(P3))) as u64; crate::simd_ops::wu8(rbx.wrapping_add(P0), ((rax as u8)) as u8); rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = (crate::simd_ops::ru64(rdx.wrapping_add(P2))) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((xmm0 as u32)) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(P3))) as u8 as u64; fl_r = (0x3u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(P0))) as u32 as u64; rbx = ((rbx).wrapping_add(0x10u64)) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(rdi))) as u64; fl_l = rbx; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 5; continue; }
            5 => { rax = ((crate::simd_ops::ru8(rdi.wrapping_add(P3))) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 8; continue; } bb = 6; continue; }
            6 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x2u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { xmm0 = (crate::simd_ops::ru128(rsi)) as u128; rdx = rsp.wrapping_add(0x30); rax = (rax & !0xffu64) | ((((((rax as u8)) & (0xfeu8))) as u8 as u64)); xmm1 = ((crate::simd_ops::ru64(rsi.wrapping_add(P3))) as u64) as u128; rcx = rdi; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128); crate::simd_ops::wu8(rdi.wrapping_add(P3), ((rax as u8)) as u8); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), ((xmm1 as u64)) as u64); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 8; continue; }
            8 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rsp = ((rsp).wrapping_add(0x50u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141625c50(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141628f50; const P1: u64 = 0x141625c50;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r11 = rsp; rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x58u64)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x8u64))) as u32 as u64; r14 = rdx; rax = ((rax).wrapping_shl(4u32)) as u64; rdi = rcx; crate::simd_ops::wu64(r11.wrapping_add(0x8u64), rbx); rbx = (crate::simd_ops::ru64(rcx)) as u64; rax = ((rax).wrapping_add(rbx)) as u64; fl_l = rbx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 13; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(r11.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(r11.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(r11.wrapping_sub(0x18u64), r15); crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm6) as u128); bb = 2; continue; }
            2 => { rax = ((crate::simd_ops::ru8(rdi.wrapping_add(0x11u64))) as u32) as u32 as u64; fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0x8u64))) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 10; continue; } bb = 3; continue; }
            3 => { r15 = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu8(rbx.wrapping_add(0x8u64), ((rax as u8)) as u8); rcx = r15; rax = (crate::simd_ops::ru64(r15)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(r14)) as u64; rbp = rax; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rdx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm6 = (xmm0) as u128; bb = 6; continue; }
            5 => { rax = (crate::simd_ops::ru64(r14.wrapping_add(0x8u64))) as u64; xmm6 = ((crate::simd_ops::ru32(rax.wrapping_add(0x8u64))) as u32) as u128; bb = 6; continue; }
            6 => { rsi = (crate::simd_ops::ru64(r14.wrapping_add(0x8u64))) as u64; rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x28u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { rdx = rbp; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 9; continue; }
            8 => { xmm0 = ((crate::simd_ops::ru32(rsi.wrapping_add(0x8u64))) as u32) as u128; bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru64(r15)) as u64; xmm3 = (xmm6) as u128; rdx = (crate::simd_ops::ru64(rsi.wrapping_add(0xcu64))) as u64; r8 = rbp; rcx = r15; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((xmm0 as u32)) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 10; continue; }
            10 => { fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(0x10u64))) as u8 as u64; fl_r = (0x3u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 12; continue; } bb = 11; continue; }
            11 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; rbx = ((rbx).wrapping_add(0x10u64)) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(rdi))) as u64; fl_l = rbx; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 12; continue; }
            12 => { r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x40u64))) as u128; bb = 13; continue; }
            13 => { rax = ((crate::simd_ops::ru8(rdi.wrapping_add(0x10u64))) as u32) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 16; continue; } bb = 14; continue; }
            14 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x2u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 16; continue; } bb = 15; continue; }
            15 => { xmm0 = (crate::simd_ops::ru128(r14)) as u128; rax = (rax & !0xffu64) | ((((((rax as u8)) & (0xfeu8))) as u8 as u64)); rdx = rsp.wrapping_add(0x30); rcx = rdi; crate::simd_ops::wu8(rdi.wrapping_add(0x10u64), ((rax as u8)) as u8); crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 16; continue; }
            16 => { rsp = ((rsp).wrapping_add(0x58u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141625d70(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1416285e0; const P1: u64 = 0x1415fa0e0; const P2: u64 = 0x14162ae30; const P3: u64 = 0x1415fc1a0; const P4: u64 = 0x141537600; const P5: u64 = 0x14162a710; const P6: u64 = 0x14162b200;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rdx; r12 = r8; rdx = ((crate::simd_ops::ru8(r8)) as u32) as u32 as u64; r15 = rcx; fl_l = ((rdx as u8)) as u8 as u64; fl_r = (0xffu8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 2; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru64(r8.wrapping_add(0x8u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 14; continue; } bb = 2; continue; }
            2 => { rcx = ((rcx).wrapping_add(0x8u64)) as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rbp = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 3; continue; }
            3 => { rdx = (crate::simd_ops::ru64(r12.wrapping_add(0x8u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 4; continue; }
            4 => { rcx = rax.wrapping_add(0x8); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 5; continue; }
            5 => { rsi = ((crate::simd_ops::ru16(rax.wrapping_add(0x8u64))) as u32) as u32 as u64; rax = (crate::simd_ops::ru64(rbx)) as u64; r8 = ((rsi.wrapping_add(rsi.wrapping_mul(4))) as u64) as u64; rdi = ((rax.wrapping_add(r8.wrapping_mul(8))) as u64) as u64; rcx = rdi; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rax = ((crate::simd_ops::ru16(rbx.wrapping_add(0x10u64))) as u32) as u32 as u64; rcx = rbp.wrapping_add(0x8); crate::simd_ops::wu16(rdi.wrapping_add(0x20u64), ((rax as u16)) as u16); crate::simd_ops::wu16(rbx.wrapping_add(0x10u64), ((rsi as u16)) as u16); rdx = (crate::simd_ops::ru64(r12.wrapping_add(0x8u64))) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru32(rbp.wrapping_add(0x10u64))) as u32 as u64; fl_r = 0x0; rbx = (((rax as u8)) as u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 10; continue; } bb = 6; continue; }
            6 => { rcx = rbp.wrapping_add(0x8); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 10; continue; }
            7 => { rax = ((rax).wrapping_add(0x2u64)) as u64; rcx = 0; fl_l = 0; fl_r = 0; fl_l = (crate::simd_ops::ru8(rbp.wrapping_add(0x4u64))) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; rax = (if (((fl_l as u8)) == ((fl_r as u8))) { rcx } else { rax }) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { rsi = ((crate::simd_ops::ru16(rax)) as u32) as u32 as u64; rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = ((rsi.wrapping_add(rsi.wrapping_mul(4))) as u64) as u64; rdi = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; rcx = rdi; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rax = ((crate::simd_ops::ru16(rbx.wrapping_add(0x10u64))) as u32) as u32 as u64; crate::simd_ops::wu16(rdi.wrapping_add(0x20u64), ((rax as u16)) as u16); crate::simd_ops::wu16(rbx.wrapping_add(0x10u64), ((rsi as u16)) as u16); rbx = ((crate::simd_ops::ru8(rbp.wrapping_add(0x4u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rbp.wrapping_add(0x4u64), (0x0u8) as u8); bb = 10; continue; }
            9 => { rbx = (rbx & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 10; continue; }
            10 => { fl_l = (crate::simd_ops::ru8(rbp.wrapping_add(0x4u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 13; continue; } bb = 11; continue; }
            11 => { fl_l = (crate::simd_ops::ru32(rbp.wrapping_add(0x10u64))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 13; continue; } bb = 12; continue; }
            12 => { rcx = rbp; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rdx = ((crate::simd_ops::ru8(r12)) as u32) as u32 as u64; rcx = r15.wrapping_add(0x8); rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); bb = 13; continue; }
            13 => { rax = (((rbx as u8)) as u32) as u32 as u64; bb = 16; continue; }
            14 => { rcx = 0; fl_l = 0; fl_r = 0; rax = r15.wrapping_add(0x2); fl_l = (crate::simd_ops::ru8(r15.wrapping_add(0x4u64))) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; rax = (if (((fl_l as u8)) == ((fl_r as u8))) { rcx } else { rax }) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 16; continue; } bb = 15; continue; }
            15 => { rsi = ((crate::simd_ops::ru16(rax)) as u32) as u32 as u64; rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = ((rsi.wrapping_add(rsi.wrapping_mul(4))) as u64) as u64; rdi = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; rcx = rdi; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rax = ((crate::simd_ops::ru16(rbx.wrapping_add(0x10u64))) as u32) as u32 as u64; crate::simd_ops::wu16(rdi.wrapping_add(0x20u64), ((rax as u16)) as u16); crate::simd_ops::wu16(rbx.wrapping_add(0x10u64), ((rsi as u16)) as u16); rax = ((crate::simd_ops::ru8(r15.wrapping_add(0x4u64))) as u32) as u32 as u64; crate::simd_ops::wu8(r15.wrapping_add(0x4u64), (0x0u8) as u8); bb = 16; continue; }
            16 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141625ef0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141628690; const P1: u64 = 0x1416285e0; const P2: u64 = 0x141625d70; const P3: u64 = 0x14162a870; const P4: u64 = 0x14162b200; const P5: u64 = 0x14162ae30; const P6: u64 = 0x14162a750; const P7: u64 = 0x14162b270;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rdx; r15 = r8; rdx = (crate::simd_ops::ru32(r8)) as u32 as u64; r14 = rcx; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru8(r8.wrapping_add(0x8u64))) as u8 as u64; fl_r = (0xffu8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 4; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru8(r8.wrapping_add(0x10u64))) as u8 as u64; fl_r = (0xffu8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 4; continue; } bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru64(r8.wrapping_add(0x18u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 20; continue; } bb = 4; continue; }
            4 => { r13 = rcx.wrapping_add(0x8); rcx = r13; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rbp = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 20; continue; } bb = 5; continue; }
            5 => { rdx = ((crate::simd_ops::ru8(r15.wrapping_add(0x8u64))) as u32) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), r12); fl_l = ((rdx as u8)) as u8 as u64; fl_r = (0xffu8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 8; continue; } bb = 6; continue; }
            6 => { fl_l = (crate::simd_ops::ru8(r15.wrapping_add(0x10u64))) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 8; continue; } bb = 7; continue; }
            7 => { fl_l = (crate::simd_ops::ru64(r15.wrapping_add(0x18u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 13; continue; } bb = 8; continue; }
            8 => { rcx = rax.wrapping_add(0x8); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rsi = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 9; continue; }
            9 => { r8 = r15.wrapping_add(0x10); rdx = rbx; rcx = rax; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru8(rsi.wrapping_add(0x4u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; rbx = (((rax as u8)) as u32) as u32 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 12; continue; } bb = 10; continue; }
            10 => { fl_l = (crate::simd_ops::ru32(rsi.wrapping_add(0x10u64))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 12; continue; } bb = 11; continue; }
            11 => { rcx = rsi; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rdx = ((crate::simd_ops::ru8(r15.wrapping_add(0x8u64))) as u32) as u32 as u64; rcx = rbp.wrapping_add(0x8); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 12; continue; }
            12 => { r14 = rbp.wrapping_add(0x6); bb = 16; continue; }
            13 => { rcx = 0; fl_l = 0; fl_r = 0; r14 = rbp.wrapping_add(0x6); fl_l = (crate::simd_ops::ru8(r14)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; rax = rbp.wrapping_add(0x4); rax = (if (((fl_l as u8)) == ((fl_r as u8))) { rcx } else { rax }) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 15; continue; } bb = 14; continue; }
            14 => { rsi = ((crate::simd_ops::ru16(rax)) as u32) as u32 as u64; rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = ((rsi.wrapping_add(rsi.wrapping_mul(4))) as u64) as u64; rdi = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; rcx = rdi; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rax = ((crate::simd_ops::ru16(rbx.wrapping_add(0x10u64))) as u32) as u32 as u64; crate::simd_ops::wu16(rdi.wrapping_add(0x20u64), ((rax as u16)) as u16); crate::simd_ops::wu16(rbx.wrapping_add(0x10u64), ((rsi as u16)) as u16); rbx = ((crate::simd_ops::ru8(r14)) as u32) as u32 as u64; crate::simd_ops::wu8(r14, (0x0u8) as u8); bb = 16; continue; }
            15 => { rbx = (rbx & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 16; continue; }
            16 => { fl_l = (crate::simd_ops::ru8(r14)) as u8 as u64; fl_r = (0x0u8) as u8 as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 19; continue; } bb = 17; continue; }
            17 => { fl_l = (crate::simd_ops::ru32(rbp.wrapping_add(0x10u64))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 19; continue; } bb = 18; continue; }
            18 => { rcx = rbp; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru32(r15)) as u32 as u64; rcx = r13; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); bb = 19; continue; }
            19 => { rax = (((rbx as u8)) as u32) as u32 as u64; bb = 22; continue; }
            20 => { rcx = 0; fl_l = 0; fl_r = 0; rax = r14.wrapping_add(0x4); fl_l = (crate::simd_ops::ru8(r14.wrapping_add(0x6u64))) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; rax = (if (((fl_l as u8)) == ((fl_r as u8))) { rcx } else { rax }) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 22; continue; } bb = 21; continue; }
            21 => { rsi = ((crate::simd_ops::ru16(rax)) as u32) as u32 as u64; rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = ((rsi.wrapping_add(rsi.wrapping_mul(4))) as u64) as u64; rdi = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; rcx = rdi; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rax = ((crate::simd_ops::ru16(rbx.wrapping_add(0x10u64))) as u32) as u32 as u64; crate::simd_ops::wu16(rdi.wrapping_add(0x20u64), ((rax as u16)) as u16); crate::simd_ops::wu16(rbx.wrapping_add(0x10u64), ((rsi as u16)) as u16); rax = ((crate::simd_ops::ru8(r14.wrapping_add(0x6u64))) as u32) as u32 as u64; crate::simd_ops::wu8(r14.wrapping_add(0x6u64), (0x0u8) as u8); bb = 22; continue; }
            22 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141626080(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141628740; const P1: u64 = 0x141628690; const P2: u64 = 0x141625ef0; const P3: u64 = 0x14162ab40; const P4: u64 = 0x14162b270; const P5: u64 = 0x14162ae30; const P6: u64 = 0x14162a920; const P7: u64 = 0x14162b2e0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rdx; r15 = r8; rdx = (crate::simd_ops::ru64(r8)) as u64; r14 = rcx; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 6; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru32(r8.wrapping_add(0x8u64))) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 6; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru32(r8.wrapping_add(0x10u64))) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 6; continue; } bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru8(r8.wrapping_add(0x18u64))) as u8 as u64; fl_r = (0xffu8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 6; continue; } bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::ru8(r8.wrapping_add(0x20u64))) as u8 as u64; fl_r = (0xffu8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 6; continue; } bb = 5; continue; }
            5 => { fl_l = (crate::simd_ops::ru64(r8.wrapping_add(0x28u64))) as u64; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 24; continue; } bb = 6; continue; }
            6 => { r13 = rcx.wrapping_add(0x8); rcx = r13; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rbp = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 24; continue; } bb = 7; continue; }
            7 => { rdx = (crate::simd_ops::ru32(r15.wrapping_add(0x8u64))) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), r12); fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 12; continue; } bb = 8; continue; }
            8 => { fl_l = (crate::simd_ops::ru32(r15.wrapping_add(0x10u64))) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 12; continue; } bb = 9; continue; }
            9 => { fl_l = (crate::simd_ops::ru8(r15.wrapping_add(0x18u64))) as u8 as u64; fl_r = (0xffu8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 12; continue; } bb = 10; continue; }
            10 => { fl_l = (crate::simd_ops::ru8(r15.wrapping_add(0x20u64))) as u8 as u64; fl_r = (0xffu8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 12; continue; } bb = 11; continue; }
            11 => { fl_l = (crate::simd_ops::ru64(r15.wrapping_add(0x28u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 17; continue; } bb = 12; continue; }
            12 => { rcx = rax.wrapping_add(0x10); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); r14 = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 13; continue; }
            13 => { r8 = r15.wrapping_add(0x10); rdx = rbx; rcx = rax; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru8(r14.wrapping_add(0x6u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; rbx = (((rax as u8)) as u32) as u32 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 16; continue; } bb = 14; continue; }
            14 => { fl_l = (crate::simd_ops::ru32(r14.wrapping_add(0x10u64))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 16; continue; } bb = 15; continue; }
            15 => { rcx = r14; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru32(r15.wrapping_add(0x8u64))) as u32 as u64; rcx = rbp.wrapping_add(0x10); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 16; continue; }
            16 => { r14 = rbp.wrapping_add(0xa); bb = 20; continue; }
            17 => { rcx = 0; fl_l = 0; fl_r = 0; r14 = rbp.wrapping_add(0xa); fl_l = (crate::simd_ops::ru8(r14)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; rax = rbp.wrapping_add(0x8); rax = (if (((fl_l as u8)) == ((fl_r as u8))) { rcx } else { rax }) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 19; continue; } bb = 18; continue; }
            18 => { rsi = ((crate::simd_ops::ru16(rax)) as u32) as u32 as u64; rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = ((rsi.wrapping_add(rsi.wrapping_mul(4))) as u64) as u64; rdi = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; rcx = rdi; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rax = ((crate::simd_ops::ru16(rbx.wrapping_add(0x10u64))) as u32) as u32 as u64; crate::simd_ops::wu16(rdi.wrapping_add(0x20u64), ((rax as u16)) as u16); crate::simd_ops::wu16(rbx.wrapping_add(0x10u64), ((rsi as u16)) as u16); rbx = ((crate::simd_ops::ru8(r14)) as u32) as u32 as u64; crate::simd_ops::wu8(r14, (0x0u8) as u8); bb = 20; continue; }
            19 => { rbx = (rbx & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 20; continue; }
            20 => { fl_l = (crate::simd_ops::ru8(r14)) as u8 as u64; fl_r = (0x0u8) as u8 as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 23; continue; } bb = 21; continue; }
            21 => { fl_l = (crate::simd_ops::ru32(rbp.wrapping_add(0x18u64))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 23; continue; } bb = 22; continue; }
            22 => { rcx = rbp; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru64(r15)) as u64; rcx = r13; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); bb = 23; continue; }
            23 => { rax = (((rbx as u8)) as u32) as u32 as u64; bb = 26; continue; }
            24 => { rcx = 0; fl_l = 0; fl_r = 0; rax = r14.wrapping_add(0x2); fl_l = (crate::simd_ops::ru8(r14.wrapping_add(0x4u64))) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; rax = (if (((fl_l as u8)) == ((fl_r as u8))) { rcx } else { rax }) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 26; continue; } bb = 25; continue; }
            25 => { rsi = ((crate::simd_ops::ru16(rax)) as u32) as u32 as u64; rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = ((rsi.wrapping_add(rsi.wrapping_mul(4))) as u64) as u64; rdi = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; rcx = rdi; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rax = ((crate::simd_ops::ru16(rbx.wrapping_add(0x10u64))) as u32) as u32 as u64; crate::simd_ops::wu16(rdi.wrapping_add(0x20u64), ((rax as u16)) as u16); crate::simd_ops::wu16(rbx.wrapping_add(0x10u64), ((rsi as u16)) as u16); rax = ((crate::simd_ops::ru8(r14.wrapping_add(0x4u64))) as u32) as u32 as u64; crate::simd_ops::wu8(r14.wrapping_add(0x4u64), (0x0u8) as u8); bb = 26; continue; }
            26 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141626220(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141627d70; const P1: u64 = 0x141628f50; const P2: u64 = 0x14162a1e0;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 23>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x90u64)) as u64; rbp = 0; fl_l = 0; fl_r = 0; r12 = r9; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0x6u64))) as u8 as u64; fl_r = ((rbp as u8)) as u8 as u64; r9 = rdx; rdx = rcx.wrapping_add(0x4); r14 = r8; rdx = (if (((fl_l as u8)) == ((fl_r as u8))) { rbp } else { rdx }) as u64; rbx = rcx; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = r9; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0xc8u64))) as u64; bb = 2; continue; }
            2 => { rsi = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x10u64))) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rsi); rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = ((rsi.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rsi; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 27; continue; } bb = 3; continue; }
            3 => { xmm0 = (crate::simd_ops::ru128(r9)) as u128; crate::simd_ops::wu64(rsp.wrapping_add(0xd0u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0x88u64), r13); r13 = rsi.wrapping_add(0x10); crate::simd_ops::wu128(rsp.wrapping_add(0x60u64), (xmm0) as u128); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), r13); crate::simd_ops::wu64(rsp.wrapping_add(0x80u64), r15); bb = 4; continue; }
            4 => { rax = ((crate::simd_ops::ru8(rsi)) as u32) as u32 as u64; crate::simd_ops::wu8(r12, ((rax as u8)) as u8); fl_l = (crate::simd_ops::ru8(r13.wrapping_sub(0xcu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 9; continue; } bb = 5; continue; }
            5 => { rax = ((crate::simd_ops::ru16(r13.wrapping_sub(0xeu64))) as u32) as u32 as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rdx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; r8 = (crate::simd_ops::ru64(rax)) as u64; rax = (crate::simd_ops::ru64(rdi)) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x28u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { rdx = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 8; continue; }
            7 => { xmm0 = ((crate::simd_ops::ru32(r8.wrapping_add(rdx.wrapping_mul(8)).wrapping_add(0x20u64))) as u32) as u128; bb = 8; continue; }
            8 => { rcx = (crate::simd_ops::ru64(rdi)) as u64; r9 = rsp.wrapping_add(0xd8); crate::simd_ops::wu8(rsp.wrapping_add(0x30u64), (0x0u8) as u8); xmm2 = (xmm0) as u128; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rbp as u32)) as u32); rdx = r14; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbp); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0xc8u64))) as u64; bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru32(r13)) as u32 as u64; rdi = (crate::simd_ops::ru64(r13.wrapping_sub(0x8u64))) as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = ((rdi.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rdi; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 25; continue; } bb = 10; continue; }
            10 => { xmm0 = (crate::simd_ops::ru128(r9)) as u128; rbp = rdi.wrapping_add(0x10); r10 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu128(rsp.wrapping_add(0x70u64), (xmm0) as u128); r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; bb = 11; continue; }
            11 => { rax = ((crate::simd_ops::ru8(rdi)) as u32) as u32 as u64; crate::simd_ops::wu8(r12.wrapping_add(0x8u64), ((rax as u8)) as u8); fl_l = (crate::simd_ops::ru8(rbp.wrapping_sub(0xcu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 16; continue; } bb = 12; continue; }
            12 => { rax = ((crate::simd_ops::ru16(rbp.wrapping_sub(0xeu64))) as u32) as u32 as u64; r8 = (crate::simd_ops::ru64(rsi)) as u64; rdx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = (crate::simd_ops::ru64(r15)) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x28u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 13; continue; }
            13 => { rdx = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r10 = 0; fl_l = 0; fl_r = 0; bb = 15; continue; }
            14 => { xmm0 = ((crate::simd_ops::ru32(r8.wrapping_add(rdx.wrapping_mul(8)).wrapping_add(0x20u64))) as u32) as u128; bb = 15; continue; }
            15 => { rcx = (crate::simd_ops::ru64(r15)) as u64; r9 = rsp.wrapping_add(0x40); crate::simd_ops::wu8(rsp.wrapping_add(0x30u64), (0x0u8) as u8); xmm2 = (xmm0) as u128; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((r10 as u32)) as u32); rdx = r14; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r10); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 16; continue; }
            16 => { rbx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x8u64))) as u64; rax = (crate::simd_ops::ru32(rbp)) as u32 as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rax = ((rax).wrapping_add(rbx)) as u64; fl_l = rbx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 23; continue; } bb = 17; continue; }
            17 => { r13 = 0; fl_l = 0; fl_r = 0; bb = 18; continue; }
            18 => { rax = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu64(r12.wrapping_add(0x10u64), rax); rax = ((crate::simd_ops::ru16(rbx.wrapping_add(0x8u64))) as u32) as u32 as u64; r8 = (crate::simd_ops::ru64(rsi)) as u64; rdx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = (crate::simd_ops::ru64(r15)) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x28u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 20; continue; } bb = 19; continue; }
            19 => { rdx = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 21; continue; }
            20 => { xmm0 = ((crate::simd_ops::ru32(r8.wrapping_add(rdx.wrapping_mul(8)).wrapping_add(0x20u64))) as u32) as u128; bb = 21; continue; }
            21 => { rcx = (crate::simd_ops::ru64(r15)) as u64; r9 = rsp.wrapping_add(0x41); crate::simd_ops::wu8(rsp.wrapping_add(0x30u64), ((r13 as u8)) as u8); xmm2 = (xmm0) as u128; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((r13 as u32)) as u32); rdx = r14; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r13); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru32(rbp)) as u32 as u64; rbx = ((rbx).wrapping_add(0x10u64)) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(rbp.wrapping_sub(0x8u64)))) as u64; fl_l = rbx; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 18; continue; } bb = 22; continue; }
            22 => { r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; bb = 23; continue; }
            23 => { r10 = 0; fl_l = 0; fl_r = 0; rdi = ((rdi).wrapping_add(0x18u64)) as u64; crate::simd_ops::wu64(r12.wrapping_add(0x10u64), r10); rbp = ((rbp).wrapping_add(0x18u64)) as u64; rax = (crate::simd_ops::ru32(r13)) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru64(r13.wrapping_sub(0x8u64))) as u64; rcx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rdi; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 11; continue; } bb = 24; continue; }
            24 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rbp = 0; fl_l = 0; fl_r = 0; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0xc0u64))) as u64; bb = 25; continue; }
            25 => { r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0xc8u64))) as u64; rsi = ((rsi).wrapping_add(0x18u64)) as u64; crate::simd_ops::wu8(r12.wrapping_add(0x8u64), (0xffu8) as u8); r13 = ((r13).wrapping_add(0x18u64)) as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x10u64))) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), r13); rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rcx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rsi; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } bb = 26; continue; }
            26 => { r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x88u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0xd0u64))) as u64; bb = 27; continue; }
            27 => { crate::simd_ops::wu8(r12, (0xffu8) as u8); rsp = ((rsp).wrapping_add(0x90u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416264d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141627d70; const P1: u64 = 0x141628f50; const P2: u64 = 0x14162a1e0; const P3: u64 = 0x141626220;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 23>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x88u64)) as u64; r12 = 0; fl_l = 0; fl_r = 0; rbx = rdx; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0x4u64))) as u8 as u64; fl_r = ((r12 as u8)) as u8 as u64; rdx = rcx.wrapping_add(0x2); r14 = r9; r15 = r8; rdx = (if (((fl_l as u8)) == ((fl_r as u8))) { r12 } else { rdx }) as u64; rdi = rcx; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 2; continue; }
            2 => { rsi = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x10u64))) as u32 as u64; rax = ((rax).wrapping_shl(5u32)) as u64; rax = ((rax).wrapping_add(rsi)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rsi); fl_l = rsi; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 24; continue; } bb = 3; continue; }
            3 => { xmm0 = (crate::simd_ops::ru128(rbx)) as u128; crate::simd_ops::wu64(rsp.wrapping_add(0xd0u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x80u64), r13); r13 = rsi.wrapping_add(0x18); crate::simd_ops::wu128(rsp.wrapping_add(0x50u64), (xmm0) as u128); bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rsi)) as u64; crate::simd_ops::wu64(r14, rax); fl_l = (crate::simd_ops::ru8(r13.wrapping_sub(0xeu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 9; continue; } bb = 5; continue; }
            5 => { rax = ((crate::simd_ops::ru16(r13.wrapping_sub(0x10u64))) as u32) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rdx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; r8 = (crate::simd_ops::ru64(rax)) as u64; rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x28u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { rdx = r15; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 8; continue; }
            7 => { xmm0 = ((crate::simd_ops::ru32(r8.wrapping_add(rdx.wrapping_mul(8)).wrapping_add(0x20u64))) as u32) as u128; bb = 8; continue; }
            8 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; r9 = rsp.wrapping_add(0xd8); crate::simd_ops::wu8(rsp.wrapping_add(0x30u64), (0x0u8) as u8); xmm2 = (xmm0) as u128; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((r12 as u32)) as u32); rdx = r15; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r12); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru32(r13)) as u32 as u64; rdi = (crate::simd_ops::ru64(r13.wrapping_sub(0x8u64))) as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = ((rdi.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rdi; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 22; continue; } bb = 10; continue; }
            10 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0xc8u64))) as u64; rbp = rdi.wrapping_add(0x10); xmm0 = (crate::simd_ops::ru128(rsi)) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x60u64), (xmm0) as u128); bb = 11; continue; }
            11 => { rax = (crate::simd_ops::ru32(rdi)) as u32 as u64; crate::simd_ops::wu32(r14.wrapping_add(0x8u64), ((rax as u32)) as u32); fl_l = (crate::simd_ops::ru8(rbp.wrapping_sub(0xau64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 16; continue; } bb = 12; continue; }
            12 => { rax = ((crate::simd_ops::ru16(rbp.wrapping_sub(0xcu64))) as u32) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rdx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; r8 = (crate::simd_ops::ru64(rax)) as u64; rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x28u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 13; continue; }
            13 => { rdx = r15; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 15; continue; }
            14 => { xmm0 = ((crate::simd_ops::ru32(r8.wrapping_add(rdx.wrapping_mul(8)).wrapping_add(0x20u64))) as u32) as u128; bb = 15; continue; }
            15 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; r9 = rsp.wrapping_add(0x40); crate::simd_ops::wu8(rsp.wrapping_add(0x30u64), (0x0u8) as u8); xmm2 = (xmm0) as u128; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((r12 as u32)) as u32); rdx = r15; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r12); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 16; continue; }
            16 => { rax = (crate::simd_ops::ru32(rbp)) as u32 as u64; rbx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x8u64))) as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = ((rbx.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 20; continue; } bb = 17; continue; }
            17 => { bb = 18; continue; }
            18 => { xmm0 = (crate::simd_ops::ru128(rsi)) as u128; rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; r9 = r14.wrapping_add(0x18); r8 = r15; crate::simd_ops::wu32(r14.wrapping_add(0x10u64), ((rax as u32)) as u32); rdx = rsp.wrapping_add(0x70); crate::simd_ops::wu128(rsp.wrapping_add(0x70u64), (xmm0) as u128); rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru32(rbp)) as u32 as u64; rbx = ((rbx).wrapping_add(0x18u64)) as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru64(rbp.wrapping_sub(0x8u64))) as u64; rcx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 18; continue; } bb = 19; continue; }
            19 => { r12 = 0; fl_l = 0; fl_r = 0; bb = 20; continue; }
            20 => { crate::simd_ops::wu32(r14.wrapping_add(0x10u64), ((r12 as u32)) as u32); rdi = ((rdi).wrapping_add(0x18u64)) as u64; rax = (crate::simd_ops::ru32(r13)) as u32 as u64; rbp = ((rbp).wrapping_add(0x18u64)) as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru64(r13.wrapping_sub(0x8u64))) as u64; rcx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rdi; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 11; continue; } bb = 21; continue; }
            21 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; bb = 22; continue; }
            22 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0xc0u64))) as u64; rsi = ((rsi).wrapping_add(0x20u64)) as u64; crate::simd_ops::wu32(r14.wrapping_add(0x8u64), ((r12 as u32)) as u32); r13 = ((r13).wrapping_add(0x20u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rsi); rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x10u64))) as u32 as u64; rax = ((rax).wrapping_shl(5u32)) as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(rdi.wrapping_add(0x8u64)))) as u64; fl_l = rsi; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } bb = 23; continue; }
            23 => { r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0xd0u64))) as u64; bb = 24; continue; }
            24 => { crate::simd_ops::wu64(r14, r12); rsp = ((rsp).wrapping_add(0x88u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141626720(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14162ae30; const P1: u64 = 0x14183acc8; const P2: u64 = 0x1415242e0; const P3: u64 = 0x1415fa0e0; const P4: u64 = 0x1415fc1a0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = rsp; crate::simd_ops::wu64(rax.wrapping_add(0x20u64), r9); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; crate::simd_ops::wu64(rax.wrapping_add(0x8u64), rbx); rdi = rcx; crate::simd_ops::wu64(rax.wrapping_add(0x10u64), rbp); rbx = rdx; crate::simd_ops::wu64(rax.wrapping_add(0x18u64), rsi); rbp = r9; crate::simd_ops::wu64(rax.wrapping_sub(0x10u64), r12); r12 = 0; fl_l = 0; fl_r = 0; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0x4u64))) as u8 as u64; fl_r = ((r12 as u8)) as u8 as u64; crate::simd_ops::wu64(rax.wrapping_sub(0x18u64), r13); crate::simd_ops::wu64(rax.wrapping_sub(0x28u64), r15); rax = rcx.wrapping_add(0x2); rax = (if (((fl_l as u8)) == ((fl_r as u8))) { r12 } else { rax }) as u64; r15 = r8; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru64(r9)) as u64; fl_r = r12; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rbp = ((crate::simd_ops::ru16(rax)) as u32) as u32 as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; rcx = ((rbp.wrapping_mul(4)) as u64) as u64; rcx = ((rcx).wrapping_add(rbp)) as u64; rsi = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; rcx = rsi; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rax = ((crate::simd_ops::ru16(rbx.wrapping_add(0x10u64))) as u32) as u32 as u64; crate::simd_ops::wu16(rsi.wrapping_add(0x20u64), ((rax as u16)) as u16); crate::simd_ops::wu16(rbx.wrapping_add(0x10u64), ((rbp as u16)) as u16); rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; crate::simd_ops::wu8(rdi.wrapping_add(0x4u64), ((r12 as u8)) as u8); bb = 3; continue; }
            3 => { rsi = (crate::simd_ops::ru64(r15)) as u64; r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), r14); fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 15; continue; } bb = 4; continue; }
            4 => { r14 = (crate::simd_ops::ru32(rdi.wrapping_add(0x10u64))) as u32 as u64; fl_l = ((r14 as u32)) as u32 as u64; fl_r = 0x1; r14 = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((((fl_l as u32)).wrapping_sub((fl_r as u32))) as i32) < 0) { bb = 12; continue; } bb = 5; continue; }
            5 => { r15 = ((((r14 as u32)) as i32 as i64) as u64) as u64; r12 = ((r14.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = ((r15).wrapping_shl(4u32)) as u64; r15 = ((fl_l as u64)) as u64; bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(r15))) as u64; crate::simd_ops::wu64(r13, rax); fl_l = (crate::simd_ops::ru64(rbp)) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 10; continue; } bb = 7; continue; }
            7 => { rbp = ((crate::simd_ops::ru16(rcx.wrapping_add(r15).wrapping_add(0x8u64))) as u32) as u32 as u64; rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = ((rbp.wrapping_mul(4)) as u64) as u64; rcx = ((rcx).wrapping_add(rbp)) as u64; rsi = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; rcx = rsi; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rax = ((crate::simd_ops::ru16(rbx.wrapping_add(0x10u64))) as u32) as u32 as u64; crate::simd_ops::wu16(rsi.wrapping_add(0x20u64), ((rax as u16)) as u16); crate::simd_ops::wu16(rbx.wrapping_add(0x10u64), ((rbp as u16)) as u16); rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x10u64))) as u32 as u64; rcx = ((r14 as u32)) as u32 as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rdi.wrapping_add(0x8u64)))) as u64; fl_l = ((r12 as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 9; continue; } bb = 8; continue; }
            8 => { rax = (((rax as u32)).wrapping_sub((r14 as u32))) as u32 as u64; rdx = rcx.wrapping_add(0x10); rax = (((rax as u32)).wrapping_shl(4u32)) as u32 as u64; r8 = ((rax.wrapping_sub(0x10u64)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 9; continue; }
            9 => { fl_l = (crate::simd_ops::lk_sub_u32(rdi.wrapping_add(0x10u64), (1) as u32)) as u32 as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; bb = 10; continue; }
            10 => { r12 = (((r12 as u32)).wrapping_sub(1)) as u32 as u64; r15 = ((r15).wrapping_sub(0x10u64)) as u64; fl_l = ((r14 as u32)) as u32 as u64; fl_r = 0x1; r14 = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((((fl_l as u32)).wrapping_sub((fl_r as u32))) as i32) >= 0) { bb = 6; continue; } bb = 11; continue; }
            11 => { r12 = 0; fl_l = 0; fl_r = 0; bb = 12; continue; }
            12 => { fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x10u64))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 18; continue; } bb = 13; continue; }
            13 => { rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 18; continue; } bb = 14; continue; }
            14 => { rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rdi.wrapping_add(0x10u64), ((r12 as u32)) as u32); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rdi.wrapping_add(0x8u64), r12); crate::simd_ops::wu32(rdi.wrapping_add(0x14u64), ((r12 as u32)) as u32); bb = 18; continue; }
            15 => { rdx = rsi; rcx = rdi.wrapping_add(0x8); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 18; continue; } bb = 16; continue; }
            16 => { crate::simd_ops::wu64(r13, rsi); fl_l = (crate::simd_ops::ru64(rbp)) as u64; fl_r = r12; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 18; continue; } bb = 17; continue; }
            17 => { rbp = ((crate::simd_ops::ru16(rax.wrapping_add(0x8u64))) as u32) as u32 as u64; rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = ((rbp.wrapping_mul(4)) as u64) as u64; rdx = ((rdx).wrapping_add(rbp)) as u64; rsi = ((rax.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64; rcx = rsi; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rax = ((crate::simd_ops::ru16(rbx.wrapping_add(0x10u64))) as u32) as u32 as u64; rcx = rdi.wrapping_add(0x8); crate::simd_ops::wu16(rsi.wrapping_add(0x20u64), ((rax as u16)) as u16); crate::simd_ops::wu16(rbx.wrapping_add(0x10u64), ((rbp as u16)) as u16); rdx = (crate::simd_ops::ru64(r15)) as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 18; continue; }
            18 => { r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; crate::simd_ops::wu64(r13, r12); fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(0x4u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 21; continue; } bb = 19; continue; }
            19 => { fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x10u64))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 21; continue; } bb = 20; continue; }
            20 => { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            21 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141626910(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141626c10; const P1: u64 = 0x1415242e0; const P2: u64 = 0x14183acc8; const P3: u64 = 0x14162ae30; const P4: u64 = 0x1416285e0; const P5: u64 = 0x14162a870; const P6: u64 = 0x14162b200;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 17>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = rsp; crate::simd_ops::wu64(rax.wrapping_add(0x20u64), r9); crate::simd_ops::wu64(rax.wrapping_add(0x18u64), r8); crate::simd_ops::wu64(rax.wrapping_add(0x10u64), rdx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = ((rsp).wrapping_sub(0x80u64)) as u64; crate::simd_ops::wu64(rax.wrapping_sub(0x10u64), rbx); rbp = rcx; crate::simd_ops::wu64(rax.wrapping_sub(0x18u64), rsi); rbx = rdx; crate::simd_ops::wu64(rax.wrapping_sub(0x20u64), rdi); rdi = ((crate::simd_ops::ru8(r8)) as u32) as u32 as u64; crate::simd_ops::wu64(rax.wrapping_sub(0x28u64), r12); crate::simd_ops::wu64(rax.wrapping_sub(0x30u64), r13); crate::simd_ops::wu64(rax.wrapping_sub(0x38u64), r14); r14 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rax.wrapping_sub(0x40u64), r15); r15 = r8; fl_l = ((rdi as u8)) as u8 as u64; fl_r = (0xffu8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 30; continue; } bb = 1; continue; }
            1 => { r13 = (crate::simd_ops::ru32(rcx.wrapping_add(0x10u64))) as u32 as u64; fl_l = ((r13 as u32)) as u32 as u64; fl_r = 0x1; r13 = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((((fl_l as u32)).wrapping_sub((fl_r as u32))) as i32) < 0) { bb = 33; continue; } bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0xb0u64))) as u64; r14 = ((r13.wrapping_add(0x1u64)) as u32) as u32 as u64; r8 = ((r8).wrapping_add(0x8u64)) as u64; rax = ((((r13 as u32)) as i32 as i64) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), r8); crate::simd_ops::wu32(rsp.wrapping_add(0x90u64), ((r14 as u32)) as u32); rdx = rcx.wrapping_add(0x8); rdi = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; fl_l = ((rdi).wrapping_shl(3u32)) as u64; rdi = ((fl_l as u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rdi); bb = 3; continue; }
            3 => { r12 = (crate::simd_ops::ru64(rbp.wrapping_add(0x8u64))) as u64; r12 = ((r12).wrapping_add(rdi)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdx); rdx = rbx; rax = ((crate::simd_ops::ru8(r12)) as u32) as u32 as u64; crate::simd_ops::wu8(rcx, ((rax as u8)) as u8); rcx = r12; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 15; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru32(r12.wrapping_add(0x10u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; rsi = ((((rax as u32)) as i32 as i64) as u64) as u64; if (((((fl_l as u32)).wrapping_sub((fl_r as u32))) as i32) < 0) { bb = 10; continue; } bb = 5; continue; }
            5 => { r14 = ((rsi.wrapping_add(rsi.wrapping_mul(2))) as u64) as u64; r14 = ((r14).wrapping_shl(3u32)) as u64; rbx = 0; fl_l = 0; fl_r = 0; bb = 6; continue; }
            6 => { rdi = (crate::simd_ops::ru64(r12.wrapping_add(0x8u64))) as u64; rdi = ((rdi).wrapping_add(r14)) as u64; crate::simd_ops::wu8(rdi.wrapping_add(0x4u64), ((rbx as u8)) as u8); rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rdi.wrapping_add(0x10u64), ((rbx as u32)) as u32); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rdi.wrapping_add(0x8u64), rbx); crate::simd_ops::wu32(rdi.wrapping_add(0x14u64), ((rbx as u32)) as u32); bb = 8; continue; }
            8 => { r14 = ((r14).wrapping_sub(0x18u64)) as u64; fl_l = rsi; fl_r = (0x1u64) as u64; rsi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((((fl_l as u64)).wrapping_sub((fl_r as u64))) as i64) >= 0) { bb = 6; continue; } bb = 9; continue; }
            9 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x98u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; r14 = (crate::simd_ops::ru32(rsp.wrapping_add(0x90u64))) as u32 as u64; bb = 10; continue; }
            10 => { rdx = (crate::simd_ops::ru64(r12.wrapping_add(0x8u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 11; continue; }
            11 => { rsi = 0; fl_l = 0; fl_r = 0; rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(r12.wrapping_add(0x10u64), ((rsi as u32)) as u32); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(r12.wrapping_add(0x8u64), rsi); crate::simd_ops::wu32(r12.wrapping_add(0x14u64), ((rsi as u32)) as u32); bb = 12; continue; }
            12 => { crate::simd_ops::wu8(r12.wrapping_add(0x4u64), (0x0u8) as u8); rcx = ((r13.wrapping_mul(2)) as u64) as u64; rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x8u64))) as u64; rcx = ((rcx).wrapping_add(r13)) as u64; rcx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x10u64))) as u32 as u64; fl_l = ((r14 as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 14; continue; } bb = 13; continue; }
            13 => { rax = (((rax as u32)).wrapping_sub((r13 as u32))) as u32 as u64; rdx = rcx.wrapping_add(0x18); rax = ((rax.wrapping_add(rax.wrapping_mul(2))) as u32) as u32 as u64; r8 = ((rax.wrapping_mul(8).wrapping_sub(0x18u64)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 14; continue; }
            14 => { fl_l = (crate::simd_ops::lk_sub_u32(rbp.wrapping_add(0x10u64), (1) as u32)) as u32 as u64; bb = 15; continue; }
            15 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0xb0u64))) as u64; r14 = (((r14 as u32)).wrapping_sub(1)) as u32 as u64; r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0xa8u64))) as u64; rdi = ((rdi).wrapping_sub(0x18u64)) as u64; fl_l = ((r13 as u32)) as u32 as u64; fl_r = 0x1; r13 = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; r8 = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x90u64), ((r14 as u32)) as u32); rdx = rcx.wrapping_add(0x8); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rdi); if (((((fl_l as u32)).wrapping_sub((fl_r as u32))) as i32) >= 0) { bb = 3; continue; } bb = 16; continue; }
            16 => { r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0xa0u64))) as u64; r12 = rcx; r14 = 0; fl_l = 0; fl_r = 0; bb = 17; continue; }
            17 => { r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0xa8u64))) as u64; bb = 18; continue; }
            18 => { r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; crate::simd_ops::wu8(r12, (0xffu8) as u8); fl_l = (crate::simd_ops::ru8(r15)) as u8 as u64; fl_r = (0xffu8) as u8 as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 24; continue; } bb = 19; continue; }
            19 => { fl_l = (crate::simd_ops::ru8(r15.wrapping_add(0x8u64))) as u8 as u64; fl_r = (0xffu8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 24; continue; } bb = 20; continue; }
            20 => { fl_l = (crate::simd_ops::ru64(r15.wrapping_add(0x10u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 24; continue; } bb = 21; continue; }
            21 => { fl_l = (crate::simd_ops::ru8(rbp.wrapping_add(0x6u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; rax = rbp.wrapping_add(0x4); rax = (if (((fl_l as u8)) == ((fl_r as u8))) { r14 } else { rax }) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 24; continue; } bb = 22; continue; }
            22 => { fl_l = (crate::simd_ops::ru64(r9)) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 24; continue; } bb = 23; continue; }
            23 => { rsi = ((crate::simd_ops::ru16(rax)) as u32) as u32 as u64; rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = ((rsi.wrapping_add(rsi.wrapping_mul(4))) as u64) as u64; rdi = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; rcx = rdi; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rax = ((crate::simd_ops::ru16(rbx.wrapping_add(0x10u64))) as u32) as u32 as u64; crate::simd_ops::wu16(rdi.wrapping_add(0x20u64), ((rax as u16)) as u16); crate::simd_ops::wu16(rbx.wrapping_add(0x10u64), ((rsi as u16)) as u16); crate::simd_ops::wu8(rbp.wrapping_add(0x6u64), (0x0u8) as u8); bb = 24; continue; }
            24 => { fl_l = (crate::simd_ops::ru32(rbp.wrapping_add(0x10u64))) as u32 as u64; fl_r = 0x0; r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 27; continue; } bb = 25; continue; }
            25 => { rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x8u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 27; continue; } bb = 26; continue; }
            26 => { rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rbp.wrapping_add(0x10u64), ((r14 as u32)) as u32); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbp.wrapping_add(0x8u64), r14); crate::simd_ops::wu32(rbp.wrapping_add(0x14u64), ((r14 as u32)) as u32); bb = 27; continue; }
            27 => { fl_l = (crate::simd_ops::ru8(rbp.wrapping_add(0x6u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 34; continue; } bb = 28; continue; }
            28 => { fl_l = (crate::simd_ops::ru32(rbp.wrapping_add(0x10u64))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 34; continue; } bb = 29; continue; }
            29 => { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); rsp = ((rsp).wrapping_add(0x80u64)) as u64; rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            30 => { rdx = (((rdi as u8)) as u32) as u32 as u64; rcx = ((rcx).wrapping_add(0x8u64)) as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0xb0u64))) as u64; rsi = rax; r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0xa8u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 18; continue; } bb = 31; continue; }
            31 => { rdx = r12.wrapping_add(0x8); crate::simd_ops::wu8(r12, ((rdi as u8)) as u8); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdx); r8 = r15.wrapping_add(0x8); rdx = rbx; rcx = rax; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 17; continue; } bb = 32; continue; }
            32 => { rcx = rsi; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rdx = ((crate::simd_ops::ru8(r15)) as u32) as u32 as u64; rcx = rbp.wrapping_add(0x8); rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); bb = 17; continue; }
            33 => { r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0xb0u64))) as u64; bb = 18; continue; }
            34 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x80u64)) as u64; rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141626c10(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14162ae30; const P1: u64 = 0x14183acc8; const P2: u64 = 0x1415242e0; const P3: u64 = 0x1415fa0e0; const P4: u64 = 0x1415fc1a0; const P5: u64 = 0x1416285e0; const P6: u64 = 0x141626720; const P7: u64 = 0x14162a710; const P8: u64 = 0x14162b200;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 15>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = rsp; crate::simd_ops::wu64(rax.wrapping_add(0x20u64), r9); crate::simd_ops::wu64(rax.wrapping_add(0x18u64), r8); crate::simd_ops::wu64(rax.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x70u64)) as u64; crate::simd_ops::wu64(rax.wrapping_add(0x10u64), rbx); r14 = rcx; crate::simd_ops::wu64(rax.wrapping_sub(0x10u64), rbp); rbx = rdx; crate::simd_ops::wu64(rax.wrapping_sub(0x18u64), rsi); crate::simd_ops::wu64(rax.wrapping_sub(0x20u64), rdi); rdi = ((crate::simd_ops::ru8(r8)) as u32) as u32 as u64; crate::simd_ops::wu64(rax.wrapping_sub(0x28u64), r12); r12 = r8; r8 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rax.wrapping_sub(0x30u64), r13); crate::simd_ops::wu64(rax.wrapping_sub(0x38u64), r15); fl_l = ((rdi as u8)) as u8 as u64; fl_r = (0xffu8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 34; continue; } bb = 1; continue; }
            1 => { r13 = (crate::simd_ops::ru32(rcx.wrapping_add(0x10u64))) as u32 as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0xa0u64))) as u64; fl_l = ((r13 as u32)) as u32 as u64; fl_r = 0x1; r13 = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((r13 as u32)) as u32); if (((((fl_l as u32)).wrapping_sub((fl_r as u32))) as i32) < 0) { bb = 39; continue; } bb = 2; continue; }
            2 => { rax = ((((r13 as u32)) as i32 as i64) as u64) as u64; r12 = ((r13.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x34u64), ((r12 as u32)) as u32); rsi = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; fl_l = ((rsi).wrapping_shl(3u32)) as u64; rsi = ((fl_l as u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rsi); bb = 3; continue; }
            3 => { rdi = (crate::simd_ops::ru64(r14.wrapping_add(0x8u64))) as u64; rax = ((crate::simd_ops::ru8(rsi.wrapping_add(rdi))) as u32) as u32 as u64; crate::simd_ops::wu8(rbp, ((rax as u8)) as u8); fl_l = (crate::simd_ops::ru8(rsi.wrapping_add(rdi).wrapping_add(0x4u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 6; continue; } bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::ru64(r9)) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rbp = ((crate::simd_ops::ru16(rsi.wrapping_add(rdi).wrapping_add(0x2u64))) as u32) as u32 as u64; rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = ((rbp.wrapping_mul(4)) as u64) as u64; rcx = ((rcx).wrapping_add(rbp)) as u64; rsi = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; rcx = rsi; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rax = ((crate::simd_ops::ru16(rbx.wrapping_add(0x10u64))) as u32) as u32 as u64; r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0x98u64))) as u64; crate::simd_ops::wu16(rsi.wrapping_add(0x20u64), ((rax as u16)) as u16); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; crate::simd_ops::wu16(rbx.wrapping_add(0x10u64), ((rbp as u16)) as u16); rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0xa0u64))) as u64; crate::simd_ops::wu8(rax.wrapping_add(rdi).wrapping_add(0x4u64), (0x0u8) as u8); bb = 7; continue; }
            6 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; bb = 7; continue; }
            7 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; rsi = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 21; continue; } bb = 8; continue; }
            8 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; r14 = (crate::simd_ops::ru32(rsi.wrapping_add(rdi).wrapping_add(0x10u64))) as u32 as u64; r15 = rsi.wrapping_add(0x8); fl_l = ((r14 as u32)) as u32 as u64; fl_r = 0x1; r14 = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((((fl_l as u32)).wrapping_sub((fl_r as u32))) as i32) < 0) { bb = 16; continue; } bb = 9; continue; }
            9 => { r12 = ((((r14 as u32)) as i32 as i64) as u64) as u64; r13 = ((r14.wrapping_add(0x1u64)) as u32) as u32 as u64; r15 = ((r15).wrapping_add(rdi)) as u64; fl_l = ((r12).wrapping_shl(4u32)) as u64; r12 = ((fl_l as u64)) as u64; bb = 10; continue; }
            10 => { rcx = (crate::simd_ops::ru64(r15)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(r12))) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x8u64), rax); fl_l = (crate::simd_ops::ru64(r9)) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 14; continue; } bb = 11; continue; }
            11 => { rbp = ((crate::simd_ops::ru16(rcx.wrapping_add(r12).wrapping_add(0x8u64))) as u32) as u32 as u64; rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = ((rbp.wrapping_mul(4)) as u64) as u64; rcx = ((rcx).wrapping_add(rbp)) as u64; rsi = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; rcx = rsi; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rax = ((crate::simd_ops::ru16(rbx.wrapping_add(0x10u64))) as u32) as u32 as u64; crate::simd_ops::wu16(rsi.wrapping_add(0x20u64), ((rax as u16)) as u16); rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; crate::simd_ops::wu16(rbx.wrapping_add(0x10u64), ((rbp as u16)) as u16); rcx = ((r14 as u32)) as u32 as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(r15))) as u64; rax = (crate::simd_ops::ru32(rsi.wrapping_add(rdi).wrapping_add(0x10u64))) as u32 as u64; fl_l = ((r13 as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 13; continue; } bb = 12; continue; }
            12 => { rax = (((rax as u32)).wrapping_sub((r14 as u32))) as u32 as u64; rdx = rcx.wrapping_add(0x10); rax = (((rax as u32)).wrapping_shl(4u32)) as u32 as u64; r8 = ((rax.wrapping_sub(0x10u64)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 13; continue; }
            13 => { fl_l = (crate::simd_ops::lk_sub_u32(rsi.wrapping_add(rdi).wrapping_add(0x10u64), (1) as u32)) as u32 as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0xa0u64))) as u64; r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0x98u64))) as u64; bb = 14; continue; }
            14 => { r13 = (((r13 as u32)).wrapping_sub(1)) as u32 as u64; r12 = ((r12).wrapping_sub(0x10u64)) as u64; fl_l = ((r14 as u32)) as u32 as u64; fl_r = 0x1; r14 = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((((fl_l as u32)).wrapping_sub((fl_r as u32))) as i32) >= 0) { bb = 10; continue; } bb = 15; continue; }
            15 => { r13 = (crate::simd_ops::ru32(rsp.wrapping_add(0x30u64))) as u32 as u64; r12 = (crate::simd_ops::ru32(rsp.wrapping_add(0x34u64))) as u32 as u64; bb = 17; continue; }
            16 => { fl_l = ((r15).wrapping_add(rdi)) as u64; fl_r = fl_l; r15 = ((fl_l as u64)) as u64; bb = 17; continue; }
            17 => { fl_l = (crate::simd_ops::ru32(rsi.wrapping_add(rdi).wrapping_add(0x10u64))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 20; continue; } bb = 18; continue; }
            18 => { rdx = (crate::simd_ops::ru64(r15)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 20; continue; } bb = 19; continue; }
            19 => { r14 = 0; fl_l = 0; fl_r = 0; rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(r15.wrapping_add(0x8u64), ((r14 as u32)) as u32); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(r15, r14); crate::simd_ops::wu32(r15.wrapping_add(0xcu64), ((r14 as u32)) as u32); bb = 20; continue; }
            20 => { r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; bb = 25; continue; }
            21 => { r15 = rax.wrapping_add(0x8); rdx = rsi; r15 = ((r15).wrapping_add(rdi)) as u64; rcx = r15; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 24; continue; } bb = 22; continue; }
            22 => { r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0x98u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x8u64), rsi); fl_l = (crate::simd_ops::ru64(r9)) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 24; continue; } bb = 23; continue; }
            23 => { rbp = ((crate::simd_ops::ru16(rax.wrapping_add(0x8u64))) as u32) as u32 as u64; rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = ((rbp.wrapping_mul(4)) as u64) as u64; rcx = ((rcx).wrapping_add(rbp)) as u64; rsi = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; rcx = rsi; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rax = ((crate::simd_ops::ru16(rbx.wrapping_add(0x10u64))) as u32) as u32 as u64; rcx = r15; crate::simd_ops::wu16(rsi.wrapping_add(0x20u64), ((rax as u16)) as u16); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; crate::simd_ops::wu16(rbx.wrapping_add(0x10u64), ((rbp as u16)) as u16); rdx = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0xa0u64))) as u64; bb = 24; continue; }
            24 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; bb = 25; continue; }
            25 => { r8 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rbp.wrapping_add(0x8u64), r8); fl_l = (crate::simd_ops::ru8(rsi.wrapping_add(rdi).wrapping_add(0x4u64))) as u8 as u64; fl_r = ((r8 as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 32; continue; } bb = 26; continue; }
            26 => { fl_l = (crate::simd_ops::ru32(rsi.wrapping_add(rdi).wrapping_add(0x10u64))) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 32; continue; } bb = 27; continue; }
            27 => { crate::simd_ops::wu8(rsi.wrapping_add(rdi).wrapping_add(0x4u64), ((r8 as u8)) as u8); rdx = (crate::simd_ops::ru64(r15)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 29; continue; } bb = 28; continue; }
            28 => { rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(r15.wrapping_add(0x8u64), ((r8 as u32)) as u32); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); r8 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(r15, r8); crate::simd_ops::wu32(r15.wrapping_add(0xcu64), ((r8 as u32)) as u32); bb = 29; continue; }
            29 => { rax = ((r13 as u32)) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru64(r14.wrapping_add(0x8u64))) as u64; rcx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; rax = (crate::simd_ops::ru32(r14.wrapping_add(0x10u64))) as u32 as u64; fl_l = ((r12 as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 31; continue; } bb = 30; continue; }
            30 => { rax = (((rax as u32)).wrapping_sub((r13 as u32))) as u32 as u64; rdx = rcx.wrapping_add(0x18); rax = ((rax.wrapping_add(rax.wrapping_mul(2))) as u32) as u32 as u64; r8 = ((rax.wrapping_mul(8).wrapping_sub(0x18u64)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 31; continue; }
            31 => { crate::simd_ops::lk_sub_u32(r14.wrapping_add(0x10u64), (1) as u32); r8 = 0; fl_l = 0; fl_r = 0; bb = 32; continue; }
            32 => { r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0x98u64))) as u64; r13 = (((r13 as u32)).wrapping_sub(1)) as u32 as u64; r12 = (((r12 as u32)).wrapping_sub(1)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((r13 as u32)) as u32); rsi = ((rsi).wrapping_sub(0x18u64)) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x34u64), ((r12 as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rsi); fl_l = ((r13 as u32)) as u32 as u64; fl_r = ((r13 as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) >= 0) { bb = 3; continue; } bb = 33; continue; }
            33 => { r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; bb = 38; continue; }
            34 => { rdx = (((rdi as u8)) as u32) as u32 as u64; rcx = ((rcx).wrapping_add(0x8u64)) as u64; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0xa0u64))) as u64; rsi = rax; r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0x98u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 50; continue; } bb = 35; continue; }
            35 => { rdx = rbp.wrapping_add(0x8); crate::simd_ops::wu8(rbp, ((rdi as u8)) as u8); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdx); r8 = r12.wrapping_add(0x8); rdx = rbx; rcx = rax; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 37; continue; } bb = 36; continue; }
            36 => { rcx = rsi; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rdx = ((crate::simd_ops::ru8(r12)) as u32) as u32 as u64; rcx = r14.wrapping_add(0x8); rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); bb = 37; continue; }
            37 => { r8 = 0; fl_l = 0; fl_r = 0; bb = 38; continue; }
            38 => { r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0x98u64))) as u64; bb = 39; continue; }
            39 => { r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; crate::simd_ops::wu8(rbp, (0xffu8) as u8); fl_l = (crate::simd_ops::ru8(r12)) as u8 as u64; fl_r = (0xffu8) as u8 as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 44; continue; } bb = 40; continue; }
            40 => { fl_l = (crate::simd_ops::ru64(r12.wrapping_add(0x8u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 44; continue; } bb = 41; continue; }
            41 => { fl_l = (crate::simd_ops::ru8(r14.wrapping_add(0x4u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; rax = r14.wrapping_add(0x2); rax = (if (((fl_l as u8)) == ((fl_r as u8))) { r8 } else { rax }) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 44; continue; } bb = 42; continue; }
            42 => { fl_l = (crate::simd_ops::ru64(r9)) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 44; continue; } bb = 43; continue; }
            43 => { rsi = ((crate::simd_ops::ru16(rax)) as u32) as u32 as u64; rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = ((rsi.wrapping_add(rsi.wrapping_mul(4))) as u64) as u64; rdi = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; rcx = rdi; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rax = ((crate::simd_ops::ru16(rbx.wrapping_add(0x10u64))) as u32) as u32 as u64; crate::simd_ops::wu16(rdi.wrapping_add(0x20u64), ((rax as u16)) as u16); crate::simd_ops::wu16(rbx.wrapping_add(0x10u64), ((rsi as u16)) as u16); crate::simd_ops::wu8(r14.wrapping_add(0x4u64), (0x0u8) as u8); bb = 44; continue; }
            44 => { fl_l = (crate::simd_ops::ru32(r14.wrapping_add(0x10u64))) as u32 as u64; fl_r = 0x0; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 47; continue; } bb = 45; continue; }
            45 => { rdx = (crate::simd_ops::ru64(r14.wrapping_add(0x8u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 47; continue; } bb = 46; continue; }
            46 => { rbx = 0; fl_l = 0; fl_r = 0; rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(r14.wrapping_add(0x10u64), ((rbx as u32)) as u32); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(r14.wrapping_add(0x8u64), rbx); crate::simd_ops::wu32(r14.wrapping_add(0x14u64), ((rbx as u32)) as u32); bb = 47; continue; }
            47 => { fl_l = (crate::simd_ops::ru8(r14.wrapping_add(0x4u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x88u64))) as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 51; continue; } bb = 48; continue; }
            48 => { fl_l = (crate::simd_ops::ru32(r14.wrapping_add(0x10u64))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 51; continue; } bb = 49; continue; }
            49 => { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); rsp = ((rsp).wrapping_add(0x70u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            50 => { r8 = 0; fl_l = 0; fl_r = 0; bb = 39; continue; }
            51 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x70u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141627040(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141627430; const P1: u64 = 0x1415242e0; const P2: u64 = 0x14183acc8; const P3: u64 = 0x141628690; const P4: u64 = 0x14162ab40; const P5: u64 = 0x14162b270; const P6: u64 = 0x14162ae30;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 25>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = rsp; crate::simd_ops::wu64(rax.wrapping_add(0x20u64), r9); crate::simd_ops::wu64(rax.wrapping_add(0x18u64), r8); crate::simd_ops::wu64(rax.wrapping_add(0x10u64), rdx); crate::simd_ops::wu64(rax.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = ((rsp).wrapping_sub(0xc0u64)) as u64; crate::simd_ops::wu64(rax.wrapping_sub(0x10u64), rbx); rbp = rcx; crate::simd_ops::wu64(rax.wrapping_sub(0x18u64), rsi); rbx = rdx; crate::simd_ops::wu64(rax.wrapping_sub(0x20u64), rdi); rdi = (crate::simd_ops::ru32(r8)) as u32 as u64; crate::simd_ops::wu64(rax.wrapping_sub(0x28u64), r12); r12 = r9; crate::simd_ops::wu64(rax.wrapping_sub(0x30u64), r13); r13 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rax.wrapping_sub(0x38u64), r14); r14 = r8; crate::simd_ops::wu64(rax.wrapping_sub(0x40u64), r15); fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 29; continue; } bb = 1; continue; }
            1 => { rdi = (crate::simd_ops::ru32(rcx.wrapping_add(0x18u64))) as u32 as u64; fl_l = ((rdi as u32)) as u32 as u64; fl_r = 0x1; rdi = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((rdi as u32)) as u32); if (((((fl_l as u32)).wrapping_sub((fl_r as u32))) as i32) < 0) { bb = 33; continue; } bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0xf0u64))) as u64; r13 = ((rdi.wrapping_add(0x1u64)) as u32) as u32 as u64; r8 = ((r8).wrapping_add(0x8u64)) as u64; rax = ((((rdi as u32)) as i32 as i64) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), r8); crate::simd_ops::wu32(rsp.wrapping_add(0x34u64), ((r13 as u32)) as u32); rdx = rcx.wrapping_add(0x8); r15 = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; fl_l = ((r15).wrapping_shl(3u32)) as u64; r15 = ((fl_l as u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), r15); bb = 3; continue; }
            3 => { r14 = (crate::simd_ops::ru64(rbp.wrapping_add(0x10u64))) as u64; r9 = r12; r14 = ((r14).wrapping_add(r15)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdx); rdx = rbx; crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), r14); rax = (crate::simd_ops::ru32(r14)) as u32 as u64; crate::simd_ops::wu32(rcx, ((rax as u32)) as u32); rcx = r14; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 27; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru32(r14.wrapping_add(0x10u64))) as u32 as u64; r12 = r14.wrapping_add(0x8); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), r12); rsi = ((((rax as u32)) as i32 as i64) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rsi); if (((((fl_l as u32)).wrapping_sub((fl_r as u32))) as i32) < 0) { bb = 22; continue; } bb = 5; continue; }
            5 => { r14 = ((rsi.wrapping_add(rsi.wrapping_mul(2))) as u64) as u64; r14 = ((r14).wrapping_shl(3u32)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), r14); rbx = 0; fl_l = 0; fl_r = 0; bb = 6; continue; }
            6 => { rbp = (crate::simd_ops::ru64(r12)) as u64; rbp = ((rbp).wrapping_add(r14)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rbp); rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x10u64))) as u32 as u64; rdi = rbp.wrapping_add(0x8); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rdi); r15 = ((((rax as u32)) as i32 as i64) as u64) as u64; if (((((fl_l as u32)).wrapping_sub((fl_r as u32))) as i32) < 0) { bb = 18; continue; } bb = 7; continue; }
            7 => { r12 = ((r15.wrapping_add(r15.wrapping_mul(2))) as u64) as u64; fl_l = ((r12).wrapping_shl(3u32)) as u64; r12 = ((fl_l as u64)) as u64; bb = 8; continue; }
            8 => { r13 = (crate::simd_ops::ru64(rdi)) as u64; r13 = ((r13).wrapping_add(r12)) as u64; rax = (crate::simd_ops::ru32(r13.wrapping_add(0x10u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; rsi = ((((rax as u32)) as i32 as i64) as u64) as u64; if (((((fl_l as u32)).wrapping_sub((fl_r as u32))) as i32) < 0) { bb = 14; continue; } bb = 9; continue; }
            9 => { rbp = ((rsi.wrapping_add(rsi.wrapping_mul(2))) as u64) as u64; fl_l = ((rbp).wrapping_shl(3u32)) as u64; rbp = ((fl_l as u64)) as u64; bb = 10; continue; }
            10 => { rdi = (crate::simd_ops::ru64(r13.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu8(rdi.wrapping_add(rbp).wrapping_add(0x4u64), ((rbx as u8)) as u8); rdx = (crate::simd_ops::ru64(rdi.wrapping_add(rbp).wrapping_add(0x8u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 11; continue; }
            11 => { rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rdi.wrapping_add(rbp).wrapping_add(0x10u64), ((rbx as u32)) as u32); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rdi.wrapping_add(rbp).wrapping_add(0x8u64), rbx); crate::simd_ops::wu32(rdi.wrapping_add(rbp).wrapping_add(0x14u64), ((rbx as u32)) as u32); bb = 12; continue; }
            12 => { rbp = ((rbp).wrapping_sub(0x18u64)) as u64; fl_l = rsi; fl_r = (0x1u64) as u64; rsi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((((fl_l as u64)).wrapping_sub((fl_r as u64))) as i64) >= 0) { bb = 10; continue; } bb = 13; continue; }
            13 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; bb = 14; continue; }
            14 => { rdx = (crate::simd_ops::ru64(r13.wrapping_add(0x8u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 16; continue; } bb = 15; continue; }
            15 => { rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(r13.wrapping_add(0x10u64), ((rbx as u32)) as u32); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(r13.wrapping_add(0x8u64), rbx); crate::simd_ops::wu32(r13.wrapping_add(0x14u64), ((rbx as u32)) as u32); bb = 16; continue; }
            16 => { r12 = ((r12).wrapping_sub(0x18u64)) as u64; crate::simd_ops::wu8(r13.wrapping_add(0x4u64), ((rbx as u8)) as u8); fl_l = r15; fl_r = (0x1u64) as u64; r15 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((((fl_l as u64)).wrapping_sub((fl_r as u64))) as i64) >= 0) { bb = 8; continue; } bb = 17; continue; }
            17 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; bb = 18; continue; }
            18 => { rdx = (crate::simd_ops::ru64(rdi)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 20; continue; } bb = 19; continue; }
            19 => { rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rdi.wrapping_add(0x8u64), ((rbx as u32)) as u32); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rdi, rbx); crate::simd_ops::wu32(rdi.wrapping_add(0xcu64), ((rbx as u32)) as u32); bb = 20; continue; }
            20 => { rsi = ((rsi).wrapping_sub(1)) as u64; crate::simd_ops::wu8(rbp.wrapping_add(0x6u64), ((rbx as u8)) as u8); r14 = ((r14).wrapping_sub(0x18u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), r14); fl_l = rsi; fl_r = rsi; if (((((fl_l as u64)) & ((fl_r as u64))) as i64) >= 0) { bb = 6; continue; } bb = 21; continue; }
            21 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0xd8u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0xd0u64))) as u64; rdi = (crate::simd_ops::ru32(rsp.wrapping_add(0x30u64))) as u32 as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; r13 = (crate::simd_ops::ru32(rsp.wrapping_add(0x34u64))) as u32 as u64; bb = 22; continue; }
            22 => { rdx = (crate::simd_ops::ru64(r12)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 24; continue; } bb = 23; continue; }
            23 => { rsi = 0; fl_l = 0; fl_r = 0; rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(r12.wrapping_add(0x8u64), ((rsi as u32)) as u32); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(r12, rsi); crate::simd_ops::wu32(r12.wrapping_add(0xcu64), ((rsi as u32)) as u32); bb = 24; continue; }
            24 => { rax = ((rdi as u32)) as u32 as u64; crate::simd_ops::wu8(r14.wrapping_add(0x6u64), (0x0u8) as u8); rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x10u64))) as u64; rcx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x18u64))) as u32 as u64; fl_l = ((r13 as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 26; continue; } bb = 25; continue; }
            25 => { rax = (((rax as u32)).wrapping_sub((rdi as u32))) as u32 as u64; rdx = rcx.wrapping_add(0x18); rax = ((rax.wrapping_add(rax.wrapping_mul(2))) as u32) as u32 as u64; r8 = ((rax.wrapping_mul(8).wrapping_sub(0x18u64)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 26; continue; }
            26 => { fl_l = (crate::simd_ops::lk_sub_u32(rbp.wrapping_add(0x18u64), (1) as u32)) as u32 as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0xe8u64))) as u64; bb = 27; continue; }
            27 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0xf0u64))) as u64; rdi = (((rdi as u32)).wrapping_sub(1)) as u32 as u64; r8 = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; r13 = (((r13 as u32)).wrapping_sub(1)) as u32 as u64; r15 = ((r15).wrapping_sub(0x18u64)) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((rdi as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x34u64), ((r13 as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), r15); rdx = rcx.wrapping_add(0x8); fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) >= 0) { bb = 3; continue; } bb = 28; continue; }
            28 => { r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0xe0u64))) as u64; r15 = rcx; r13 = 0; fl_l = 0; fl_r = 0; bb = 34; continue; }
            29 => { rdx = ((rdi as u32)) as u32 as u64; rcx = ((rcx).wrapping_add(0x10u64)) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0xf0u64))) as u64; rsi = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 32; continue; } bb = 30; continue; }
            30 => { rdx = r15.wrapping_add(0x8); crate::simd_ops::wu32(r15, ((rdi as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdx); r8 = r14.wrapping_add(0x8); rdx = rbx; r9 = r12; rcx = rax; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 32; continue; } bb = 31; continue; }
            31 => { rcx = rsi; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru32(r14)) as u32 as u64; rcx = rbp.wrapping_add(0x10); rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); bb = 32; continue; }
            32 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0xd0u64))) as u64; bb = 34; continue; }
            33 => { r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0xf0u64))) as u64; bb = 34; continue; }
            34 => { crate::simd_ops::wu32(r15, ((r13 as u32)) as u32); fl_l = (crate::simd_ops::ru32(r14)) as u32 as u64; fl_r = 0x0; r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x88u64))) as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 42; continue; } bb = 35; continue; }
            35 => { fl_l = (crate::simd_ops::ru32(r14.wrapping_add(0x8u64))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 42; continue; } bb = 36; continue; }
            36 => { fl_l = (crate::simd_ops::ru8(r14.wrapping_add(0x10u64))) as u8 as u64; fl_r = (0xffu8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 42; continue; } bb = 37; continue; }
            37 => { fl_l = (crate::simd_ops::ru8(r14.wrapping_add(0x18u64))) as u8 as u64; fl_r = (0xffu8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 42; continue; } bb = 38; continue; }
            38 => { fl_l = (crate::simd_ops::ru64(r14.wrapping_add(0x20u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 42; continue; } bb = 39; continue; }
            39 => { fl_l = (crate::simd_ops::ru8(rbp.wrapping_add(0xau64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; rax = rbp.wrapping_add(0x8); rax = (if (((fl_l as u8)) == ((fl_r as u8))) { r13 } else { rax }) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 42; continue; } bb = 40; continue; }
            40 => { fl_l = (crate::simd_ops::ru64(r12)) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 42; continue; } bb = 41; continue; }
            41 => { rsi = ((crate::simd_ops::ru16(rax)) as u32) as u32 as u64; rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = ((rsi.wrapping_add(rsi.wrapping_mul(4))) as u64) as u64; rdi = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; rcx = rdi; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rax = ((crate::simd_ops::ru16(rbx.wrapping_add(0x10u64))) as u32) as u32 as u64; crate::simd_ops::wu16(rdi.wrapping_add(0x20u64), ((rax as u16)) as u16); crate::simd_ops::wu16(rbx.wrapping_add(0x10u64), ((rsi as u16)) as u16); crate::simd_ops::wu8(rbp.wrapping_add(0xau64), (0x0u8) as u8); bb = 42; continue; }
            42 => { fl_l = (crate::simd_ops::ru32(rbp.wrapping_add(0x18u64))) as u32 as u64; fl_r = 0x0; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0xa0u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0xa8u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0xb0u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0xb8u64))) as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 45; continue; } bb = 43; continue; }
            43 => { rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x10u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 45; continue; } bb = 44; continue; }
            44 => { rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rbp.wrapping_add(0x18u64), ((r13 as u32)) as u32); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbp.wrapping_add(0x10u64), r13); crate::simd_ops::wu32(rbp.wrapping_add(0x1cu64), ((r13 as u32)) as u32); bb = 45; continue; }
            45 => { fl_l = (crate::simd_ops::ru8(rbp.wrapping_add(0xau64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x98u64))) as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 48; continue; } bb = 46; continue; }
            46 => { fl_l = (crate::simd_ops::ru32(rbp.wrapping_add(0x18u64))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 48; continue; } bb = 47; continue; }
            47 => { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); rsp = ((rsp).wrapping_add(0xc0u64)) as u64; rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            48 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0xc0u64)) as u64; rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141627430(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141626910; const P1: u64 = 0x1415242e0; const P2: u64 = 0x14183acc8; const P3: u64 = 0x141628690; const P4: u64 = 0x14162a750; const P5: u64 = 0x14162b270; const P6: u64 = 0x14162ae30;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 21>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = rsp; crate::simd_ops::wu64(rax.wrapping_add(0x20u64), r9); crate::simd_ops::wu64(rax.wrapping_add(0x18u64), r8); crate::simd_ops::wu64(rax.wrapping_add(0x10u64), rdx); crate::simd_ops::wu64(rax.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = ((rsp).wrapping_sub(0xa0u64)) as u64; crate::simd_ops::wu64(rax.wrapping_sub(0x10u64), rbx); rbp = rcx; crate::simd_ops::wu64(rax.wrapping_sub(0x18u64), rsi); rbx = rdx; crate::simd_ops::wu64(rax.wrapping_sub(0x20u64), rdi); rdi = (crate::simd_ops::ru32(r8)) as u32 as u64; crate::simd_ops::wu64(rax.wrapping_sub(0x28u64), r12); r12 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rax.wrapping_sub(0x38u64), r14); r14 = r8; crate::simd_ops::wu64(rax.wrapping_sub(0x40u64), r15); fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 23; continue; } bb = 1; continue; }
            1 => { rsi = (crate::simd_ops::ru32(rcx.wrapping_add(0x10u64))) as u32 as u64; fl_l = ((rsi as u32)) as u32 as u64; fl_r = 0x1; rsi = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((rsi as u32)) as u32); if (((((fl_l as u32)).wrapping_sub((fl_r as u32))) as i32) < 0) { bb = 28; continue; } bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0xd0u64))) as u64; r8 = ((r8).wrapping_add(0x8u64)) as u64; crate::simd_ops::wu64(rax.wrapping_sub(0x30u64), r13); r13 = ((rsi.wrapping_add(0x1u64)) as u32) as u32 as u64; rax = ((((rsi as u32)) as i32 as i64) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), r8); rdx = rcx.wrapping_add(0x8); crate::simd_ops::wu32(rsp.wrapping_add(0x34u64), ((r13 as u32)) as u32); r12 = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; fl_l = ((r12).wrapping_shl(3u32)) as u64; r12 = ((fl_l as u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), r12); bb = 3; continue; }
            3 => { r14 = (crate::simd_ops::ru64(rbp.wrapping_add(0x8u64))) as u64; r14 = ((r14).wrapping_add(r12)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdx); rdx = rbx; rax = (crate::simd_ops::ru32(r14)) as u32 as u64; crate::simd_ops::wu32(rcx, ((rax as u32)) as u32); rcx = r14; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 21; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru32(r14.wrapping_add(0x10u64))) as u32 as u64; rdi = r14.wrapping_add(0x8); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rdi); r15 = ((((rax as u32)) as i32 as i64) as u64) as u64; if (((((fl_l as u32)).wrapping_sub((fl_r as u32))) as i32) < 0) { bb = 16; continue; } bb = 5; continue; }
            5 => { r12 = ((r15.wrapping_add(r15.wrapping_mul(2))) as u64) as u64; r12 = ((r12).wrapping_shl(3u32)) as u64; rbx = 0; fl_l = 0; fl_r = 0; bb = 6; continue; }
            6 => { r13 = (crate::simd_ops::ru64(rdi)) as u64; r13 = ((r13).wrapping_add(r12)) as u64; rax = (crate::simd_ops::ru32(r13.wrapping_add(0x10u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; rsi = ((((rax as u32)) as i32 as i64) as u64) as u64; if (((((fl_l as u32)).wrapping_sub((fl_r as u32))) as i32) < 0) { bb = 12; continue; } bb = 7; continue; }
            7 => { rbp = ((rsi.wrapping_add(rsi.wrapping_mul(2))) as u64) as u64; fl_l = ((rbp).wrapping_shl(3u32)) as u64; rbp = ((fl_l as u64)) as u64; bb = 8; continue; }
            8 => { rdi = (crate::simd_ops::ru64(r13.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu8(rdi.wrapping_add(rbp).wrapping_add(0x4u64), ((rbx as u8)) as u8); rdx = (crate::simd_ops::ru64(rdi.wrapping_add(rbp).wrapping_add(0x8u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rdi.wrapping_add(rbp).wrapping_add(0x10u64), ((rbx as u32)) as u32); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rdi.wrapping_add(rbp).wrapping_add(0x8u64), rbx); crate::simd_ops::wu32(rdi.wrapping_add(rbp).wrapping_add(0x14u64), ((rbx as u32)) as u32); bb = 10; continue; }
            10 => { rbp = ((rbp).wrapping_sub(0x18u64)) as u64; fl_l = rsi; fl_r = (0x1u64) as u64; rsi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((((fl_l as u64)).wrapping_sub((fl_r as u64))) as i64) >= 0) { bb = 8; continue; } bb = 11; continue; }
            11 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; bb = 12; continue; }
            12 => { rdx = (crate::simd_ops::ru64(r13.wrapping_add(0x8u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 13; continue; }
            13 => { rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(r13.wrapping_add(0x10u64), ((rbx as u32)) as u32); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(r13.wrapping_add(0x8u64), rbx); crate::simd_ops::wu32(r13.wrapping_add(0x14u64), ((rbx as u32)) as u32); bb = 14; continue; }
            14 => { r12 = ((r12).wrapping_sub(0x18u64)) as u64; crate::simd_ops::wu8(r13.wrapping_add(0x4u64), ((rbx as u8)) as u8); fl_l = r15; fl_r = (0x1u64) as u64; r15 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((((fl_l as u64)).wrapping_sub((fl_r as u64))) as i64) >= 0) { bb = 6; continue; } bb = 15; continue; }
            15 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0xb8u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0xb0u64))) as u64; rsi = (crate::simd_ops::ru32(rsp.wrapping_add(0x30u64))) as u32 as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; r13 = (crate::simd_ops::ru32(rsp.wrapping_add(0x34u64))) as u32 as u64; bb = 16; continue; }
            16 => { rdx = (crate::simd_ops::ru64(rdi)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 18; continue; } bb = 17; continue; }
            17 => { r15 = 0; fl_l = 0; fl_r = 0; rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rdi.wrapping_add(0x8u64), ((r15 as u32)) as u32); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rdi, r15); crate::simd_ops::wu32(rdi.wrapping_add(0xcu64), ((r15 as u32)) as u32); bb = 18; continue; }
            18 => { rax = ((rsi as u32)) as u32 as u64; crate::simd_ops::wu8(r14.wrapping_add(0x6u64), (0x0u8) as u8); rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x8u64))) as u64; rcx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x10u64))) as u32 as u64; fl_l = ((r13 as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 20; continue; } bb = 19; continue; }
            19 => { rax = (((rax as u32)).wrapping_sub((rsi as u32))) as u32 as u64; rdx = rcx.wrapping_add(0x18); rax = ((rax.wrapping_add(rax.wrapping_mul(2))) as u32) as u32 as u64; r8 = ((rax.wrapping_mul(8).wrapping_sub(0x18u64)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 20; continue; }
            20 => { fl_l = (crate::simd_ops::lk_sub_u32(rbp.wrapping_add(0x10u64), (1) as u32)) as u32 as u64; bb = 21; continue; }
            21 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0xd0u64))) as u64; rsi = (((rsi as u32)).wrapping_sub(1)) as u32 as u64; r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0xc8u64))) as u64; r13 = (((r13 as u32)).wrapping_sub(1)) as u32 as u64; r8 = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; r12 = ((r12).wrapping_sub(0x18u64)) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((rsi as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x34u64), ((r13 as u32)) as u32); rdx = rcx.wrapping_add(0x8); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), r12); fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) >= 0) { bb = 3; continue; } bb = 22; continue; }
            22 => { r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0xc0u64))) as u64; r15 = rcx; r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; r12 = 0; fl_l = 0; fl_r = 0; bb = 29; continue; }
            23 => { rdx = ((rdi as u32)) as u32 as u64; rcx = ((rcx).wrapping_add(0x8u64)) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0xd0u64))) as u64; rsi = rax; r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0xc8u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 27; continue; } bb = 24; continue; }
            24 => { rdx = r15.wrapping_add(0x8); crate::simd_ops::wu32(r15, ((rdi as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdx); r8 = r14.wrapping_add(0x8); rdx = rbx; rcx = rax; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 26; continue; } bb = 25; continue; }
            25 => { rcx = rsi; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru32(r14)) as u32 as u64; rcx = rbp.wrapping_add(0x8); rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); bb = 26; continue; }
            26 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0xb0u64))) as u64; r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0xc8u64))) as u64; bb = 29; continue; }
            27 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0xb0u64))) as u64; bb = 29; continue; }
            28 => { r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0xd0u64))) as u64; bb = 29; continue; }
            29 => { crate::simd_ops::wu32(r15, ((r12 as u32)) as u32); fl_l = (crate::simd_ops::ru32(r14)) as u32 as u64; fl_r = 0x0; r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 36; continue; } bb = 30; continue; }
            30 => { fl_l = (crate::simd_ops::ru8(r14.wrapping_add(0x8u64))) as u8 as u64; fl_r = (0xffu8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 36; continue; } bb = 31; continue; }
            31 => { fl_l = (crate::simd_ops::ru8(r14.wrapping_add(0x10u64))) as u8 as u64; fl_r = (0xffu8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 36; continue; } bb = 32; continue; }
            32 => { fl_l = (crate::simd_ops::ru64(r14.wrapping_add(0x18u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 36; continue; } bb = 33; continue; }
            33 => { fl_l = (crate::simd_ops::ru8(rbp.wrapping_add(0x6u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; rax = rbp.wrapping_add(0x4); rax = (if (((fl_l as u8)) == ((fl_r as u8))) { r12 } else { rax }) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 36; continue; } bb = 34; continue; }
            34 => { fl_l = (crate::simd_ops::ru64(r9)) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 36; continue; } bb = 35; continue; }
            35 => { rsi = ((crate::simd_ops::ru16(rax)) as u32) as u32 as u64; rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = ((rsi.wrapping_add(rsi.wrapping_mul(4))) as u64) as u64; rdi = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; rcx = rdi; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rax = ((crate::simd_ops::ru16(rbx.wrapping_add(0x10u64))) as u32) as u32 as u64; crate::simd_ops::wu16(rdi.wrapping_add(0x20u64), ((rax as u16)) as u16); crate::simd_ops::wu16(rbx.wrapping_add(0x10u64), ((rsi as u16)) as u16); crate::simd_ops::wu8(rbp.wrapping_add(0x6u64), (0x0u8) as u8); bb = 36; continue; }
            36 => { fl_l = (crate::simd_ops::ru32(rbp.wrapping_add(0x10u64))) as u32 as u64; fl_r = 0x0; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x88u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x98u64))) as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 39; continue; } bb = 37; continue; }
            37 => { rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x8u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 39; continue; } bb = 38; continue; }
            38 => { rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rbp.wrapping_add(0x10u64), ((r12 as u32)) as u32); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbp.wrapping_add(0x8u64), r12); crate::simd_ops::wu32(rbp.wrapping_add(0x14u64), ((r12 as u32)) as u32); bb = 39; continue; }
            39 => { fl_l = (crate::simd_ops::ru8(rbp.wrapping_add(0x6u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 42; continue; } bb = 40; continue; }
            40 => { fl_l = (crate::simd_ops::ru32(rbp.wrapping_add(0x10u64))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 42; continue; } bb = 41; continue; }
            41 => { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); rsp = ((rsp).wrapping_add(0xa0u64)) as u64; rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            42 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0xa0u64)) as u64; rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141627790(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141627040; const P1: u64 = 0x14162a920; const P2: u64 = 0x14183acc8; const P3: u64 = 0x141628740; const P4: u64 = 0x14162b2e0; const P5: u64 = 0x14162ae30; const P6: u64 = 0x1415242e0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = rsp; crate::simd_ops::wu64(rax.wrapping_add(0x20u64), r9); crate::simd_ops::wu64(rax.wrapping_add(0x18u64), r8); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = ((rsp).wrapping_sub(0x60u64)) as u64; crate::simd_ops::wu64(rax.wrapping_add(0x8u64), rbx); rbp = rcx; crate::simd_ops::wu64(rax.wrapping_sub(0x10u64), rsi); rbx = rdx; crate::simd_ops::wu64(rax.wrapping_sub(0x18u64), rdi); rsi = r8; rdi = (crate::simd_ops::ru64(r8)) as u64; crate::simd_ops::wu64(rax.wrapping_sub(0x20u64), r12); r12 = r9; crate::simd_ops::wu64(rax.wrapping_sub(0x28u64), r13); r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; crate::simd_ops::wu64(rax.wrapping_sub(0x30u64), r14); crate::simd_ops::wu64(rax.wrapping_sub(0x38u64), r15); fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 9; continue; } bb = 1; continue; }
            1 => { rdi = (crate::simd_ops::ru32(rcx.wrapping_add(0x10u64))) as u32 as u64; fl_l = ((rdi as u32)) as u32 as u64; fl_r = 0x1; rdi = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((((fl_l as u32)).wrapping_sub((fl_r as u32))) as i32) < 0) { bb = 12; continue; } bb = 2; continue; }
            2 => { r14 = ((((rdi as u32)) as i32 as i64) as u64) as u64; rdx = r8.wrapping_add(0x8); crate::simd_ops::wu64(rsp.wrapping_add(0x90u64), rdx); rcx = r13.wrapping_add(0x8); fl_l = ((r14).wrapping_shl(5u32)) as u64; r14 = ((fl_l as u64)) as u64; r15 = ((rdi.wrapping_add(0x1u64)) as u32) as u32 as u64; rsi = r9; bb = 3; continue; }
            3 => { r12 = (crate::simd_ops::ru64(rbp.wrapping_add(0x8u64))) as u64; r8 = rdx; r12 = ((r12).wrapping_add(r14)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rcx); r9 = rsi; rdx = rbx; rcx = r12; rax = (crate::simd_ops::ru64(r12)) as u64; crate::simd_ops::wu64(r13, rax); rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 7; continue; } bb = 4; continue; }
            4 => { rcx = r12; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x10u64))) as u32 as u64; rcx = ((rdi as u32)) as u32 as u64; rcx = ((rcx).wrapping_shl(5u32)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rbp.wrapping_add(0x8u64)))) as u64; fl_l = ((r15 as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = (((rax as u32)).wrapping_sub((rdi as u32))) as u32 as u64; rdx = rcx.wrapping_add(0x20); rax = (((rax as u32)).wrapping_shl(5u32)) as u32 as u64; r8 = ((rax.wrapping_sub(0x20u64)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            6 => { fl_l = (crate::simd_ops::lk_sub_u32(rbp.wrapping_add(0x10u64), (1) as u32)) as u32 as u64; bb = 7; continue; }
            7 => { rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; rcx = r13.wrapping_add(0x8); r15 = (((r15 as u32)).wrapping_sub(1)) as u32 as u64; r14 = ((r14).wrapping_sub(0x20u64)) as u64; fl_l = ((rdi as u32)) as u32 as u64; fl_r = 0x1; rdi = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((((fl_l as u32)).wrapping_sub((fl_r as u32))) as i32) >= 0) { bb = 3; continue; } bb = 8; continue; }
            8 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x88u64))) as u64; bb = 12; continue; }
            9 => { rdx = rdi; rcx = ((rcx).wrapping_add(0x8u64)) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); r14 = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 10; continue; }
            10 => { rdx = r13.wrapping_add(0x8); crate::simd_ops::wu64(r13, rdi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdx); r8 = rsi.wrapping_add(0x8); rdx = rbx; r9 = r12; rcx = rax; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 12; continue; } bb = 11; continue; }
            11 => { rcx = r14; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru64(rsi)) as u64; rcx = rbp.wrapping_add(0x8); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 12; continue; }
            12 => { r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; r14 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(r13, r14); r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; fl_l = (crate::simd_ops::ru64(rsi)) as u64; fl_r = r14; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 21; continue; } bb = 13; continue; }
            13 => { fl_l = (crate::simd_ops::ru32(rsi.wrapping_add(0x8u64))) as u32 as u64; fl_r = ((r14 as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 21; continue; } bb = 14; continue; }
            14 => { fl_l = (crate::simd_ops::ru32(rsi.wrapping_add(0x10u64))) as u32 as u64; fl_r = ((r14 as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 21; continue; } bb = 15; continue; }
            15 => { fl_l = (crate::simd_ops::ru8(rsi.wrapping_add(0x18u64))) as u8 as u64; fl_r = (0xffu8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 21; continue; } bb = 16; continue; }
            16 => { fl_l = (crate::simd_ops::ru8(rsi.wrapping_add(0x20u64))) as u8 as u64; fl_r = (0xffu8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 21; continue; } bb = 17; continue; }
            17 => { fl_l = (crate::simd_ops::ru64(rsi.wrapping_add(0x28u64))) as u64; fl_r = r14; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 21; continue; } bb = 18; continue; }
            18 => { fl_l = (crate::simd_ops::ru8(rbp.wrapping_add(0x4u64))) as u8 as u64; fl_r = ((r14 as u8)) as u8 as u64; rax = rbp.wrapping_add(0x2); rax = (if (((fl_l as u8)) == ((fl_r as u8))) { r14 } else { rax }) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 21; continue; } bb = 19; continue; }
            19 => { fl_l = (crate::simd_ops::ru64(r12)) as u64; fl_r = r14; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 21; continue; } bb = 20; continue; }
            20 => { rsi = ((crate::simd_ops::ru16(rax)) as u32) as u32 as u64; rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = ((rsi.wrapping_add(rsi.wrapping_mul(4))) as u64) as u64; rdi = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; rcx = rdi; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rax = ((crate::simd_ops::ru16(rbx.wrapping_add(0x10u64))) as u32) as u32 as u64; crate::simd_ops::wu16(rdi.wrapping_add(0x20u64), ((rax as u16)) as u16); crate::simd_ops::wu16(rbx.wrapping_add(0x10u64), ((rsi as u16)) as u16); crate::simd_ops::wu8(rbp.wrapping_add(0x4u64), ((r14 as u8)) as u8); bb = 21; continue; }
            21 => { r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; fl_l = (crate::simd_ops::ru32(rbp.wrapping_add(0x10u64))) as u32 as u64; fl_r = ((r14 as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 24; continue; } bb = 22; continue; }
            22 => { rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x8u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 24; continue; } bb = 23; continue; }
            23 => { rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rbp.wrapping_add(0x10u64), ((r14 as u32)) as u32); rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbp.wrapping_add(0x8u64), r14); crate::simd_ops::wu32(rbp.wrapping_add(0x14u64), ((r14 as u32)) as u32); bb = 24; continue; }
            24 => { fl_l = (crate::simd_ops::ru8(rbp.wrapping_add(0x4u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 27; continue; } bb = 25; continue; }
            25 => { fl_l = (crate::simd_ops::ru32(rbp.wrapping_add(0x10u64))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 27; continue; } bb = 26; continue; }
            26 => { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); rsp = ((rsp).wrapping_add(0x60u64)) as u64; rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            27 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x60u64)) as u64; rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416279b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0xc; const P1: u64 = 0x8; const P2: u64 = 0x40; const P3: u64 = 0x14; const P4: u64 = 0x1c; const P5: u64 = 0x28; const P6: u64 = 0x30; const P7: u64 = 0x38; const P8: u64 = 0x48; const P9: u64 = 0x4c; const P10: u64 = 0x50; const P11: u64 = 0x58;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    crate::simd_ops::wu64(rcx.wrapping_add(P0), rdx);
    r8 = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rcx, r8);
    rax = 0xffff;
    crate::simd_ops::wu32(rcx.wrapping_add(P1), ((r8 as u32)) as u32);
    xmm0 = 0u128;
    crate::simd_ops::wu16(rcx.wrapping_add(P2), ((rax as u16)) as u16);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rcx.wrapping_add(P3), r8);
    crate::simd_ops::wu64(rcx.wrapping_add(P4), r8);
    crate::simd_ops::wu64(rcx.wrapping_add(P5), r8);
    crate::simd_ops::wu64(rcx.wrapping_add(P6), r8);
    crate::simd_ops::wu64(rcx.wrapping_add(P7), r8);
    crate::simd_ops::wu128(rcx.wrapping_add(P8), (xmm0) as u128);
    crate::simd_ops::wu8(rcx.wrapping_add(P9), ((rax as u8)) as u8);
    crate::simd_ops::wu64(rcx.wrapping_add(P10), r8);
    crate::simd_ops::wu64(rcx.wrapping_add(P11), r8);
    crate::simd_ops::wu16(rcx.wrapping_add(0x70u64), ((rax as u16)) as u16);
    crate::simd_ops::wu16(rcx.wrapping_add(0x88u64), ((rax as u16)) as u16);
    rax = rcx;
    crate::simd_ops::wu64(rcx.wrapping_add(0x60u64), r8);
    crate::simd_ops::wu64(rcx.wrapping_add(0x68u64), r8);
    crate::simd_ops::wu64(rcx.wrapping_add(0x78u64), r8);
    crate::simd_ops::wu64(rcx.wrapping_add(0x80u64), r8);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141627a20(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x28; const P1: u64 = 0x143930fa0; const P2: u64 = 0x1415c5200; const P3: u64 = 0x8; const P4: u64 = 0x1415242e0; const P5: u64 = 0x10; const P6: u64 = 0x18; const P7: u64 = 0x30; const P8: u64 = 0x14162a6a0; const P9: u64 = 0x78; const P10: u64 = 0x80; const P11: u64 = 0x84; const P12: u64 = 0x14162acc0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rsi = 0; fl_l = 0; fl_r = 0; rbx = rcx; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { rdx = (crate::simd_ops::ru64(rdi.wrapping_add(P0))) as u64; r8 = rdi; rcx = (crate::simd_ops::ru64(P1)) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru64(rdi.wrapping_add(P3))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 3; continue; }
            3 => { crate::simd_ops::wu64(rdi.wrapping_add(P3), rsi); rdx = rdi; crate::simd_ops::wu64(rdi.wrapping_add(P5), rsi); rcx = 0x10; crate::simd_ops::wu32(rdi.wrapping_add(P6), (0x1u32) as u32); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { rcx = ((rbx.wrapping_add(P7)) as u64) as u64; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P9))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rbx.wrapping_add(P10), ((rsi as u32)) as u32); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(P9), rsi); crate::simd_ops::wu32(rbx.wrapping_add(P11), ((rsi as u32)) as u32); bb = 6; continue; }
            6 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x60u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rbx.wrapping_add(0x68u64), ((rsi as u32)) as u32); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(0x60u64), rsi); crate::simd_ops::wu32(rbx.wrapping_add(0x6cu64), ((rsi as u32)) as u32); bb = 8; continue; }
            8 => { rcx = rbx.wrapping_add(0x30); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P12, rax, rcx, rdx, r8, r9, rsp); }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141627ae0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x2; const P1: u64 = 0x8; const P2: u64 = 0x10; const P3: u64 = 0x1415242e0; const P4: u64 = 0x14;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = ((crate::simd_ops::ru8(rdx)) as u32) as u32 as u64; rbx = rdx; crate::simd_ops::wu8(rcx, ((rax as u8)) as u8); rbp = 0; fl_l = 0; fl_r = 0; rax = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64; rdi = rcx; crate::simd_ops::wu32(rcx.wrapping_add(P0), ((rax as u32)) as u32); rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu32(rcx.wrapping_add(P2), ((rbp as u32)) as u32); rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rdi.wrapping_add(P1), rbp); crate::simd_ops::wu32(rdi.wrapping_add(P4), ((rbp as u32)) as u32); bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; crate::simd_ops::wu64(rdi.wrapping_add(P1), rax); rax = (crate::simd_ops::ru32(rbx.wrapping_add(P2))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(P2), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_add(P4))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(P4), ((rax as u32)) as u32); crate::simd_ops::wu64(rbx.wrapping_add(P1), rbp); crate::simd_ops::wu64(rbx.wrapping_add(P2), rbp); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141627b60(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x4; const P1: u64 = 0x8; const P2: u64 = 0x10; const P3: u64 = 0x1415242e0; const P4: u64 = 0x14;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru32(rdx)) as u32 as u64; rdi = rdx; crate::simd_ops::wu32(rcx, ((rax as u32)) as u32); rbp = 0; fl_l = 0; fl_r = 0; rax = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64; rbx = rcx; crate::simd_ops::wu32(rcx.wrapping_add(P0), ((rax as u32)) as u32); rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu32(rcx.wrapping_add(P2), ((rbp as u32)) as u32); rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(P1), rbp); crate::simd_ops::wu32(rbx.wrapping_add(P4), ((rbp as u32)) as u32); bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(P1))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(P1), rax); rax = (crate::simd_ops::ru32(rdi.wrapping_add(P2))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(P2), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdi.wrapping_add(P4))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(P4), ((rax as u32)) as u32); rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu64(rdi.wrapping_add(P1), rbp); crate::simd_ops::wu64(rdi.wrapping_add(P2), rbp); rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141627be0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x2; const P1: u64 = 0x8; const P2: u64 = 0x10; const P3: u64 = 0x1415242e0; const P4: u64 = 0x14;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = ((crate::simd_ops::ru8(rdx)) as u32) as u32 as u64; rdi = rdx; crate::simd_ops::wu8(rcx, ((rax as u8)) as u8); rbp = 0; fl_l = 0; fl_r = 0; rax = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64; rbx = rcx; crate::simd_ops::wu32(rcx.wrapping_add(P0), ((rax as u32)) as u32); rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu32(rcx.wrapping_add(P2), ((rbp as u32)) as u32); rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(P1), rbp); crate::simd_ops::wu32(rbx.wrapping_add(P4), ((rbp as u32)) as u32); bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(P1))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(P1), rax); rax = (crate::simd_ops::ru32(rdi.wrapping_add(P2))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(P2), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdi.wrapping_add(P4))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(P4), ((rax as u32)) as u32); rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu64(rdi.wrapping_add(P1), rbp); crate::simd_ops::wu64(rdi.wrapping_add(P2), rbp); rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141627c60(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x18; const P3: u64 = 0x1415242e0; const P4: u64 = 0x1c;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; rdi = rdx; crate::simd_ops::wu64(rcx, rax); rbp = 0; fl_l = 0; fl_r = 0; rax = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64; rbx = rcx; crate::simd_ops::wu32(rcx.wrapping_add(P0), ((rax as u32)) as u32); rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu32(rcx.wrapping_add(P2), ((rbp as u32)) as u32); rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(P1), rbp); crate::simd_ops::wu32(rbx.wrapping_add(P4), ((rbp as u32)) as u32); bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(P1))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(P1), rax); rax = (crate::simd_ops::ru32(rdi.wrapping_add(P2))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(P2), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdi.wrapping_add(P4))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(P4), ((rax as u32)) as u32); rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu64(rdi.wrapping_add(P1), rbp); crate::simd_ops::wu64(rdi.wrapping_add(P2), rbp); rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141627ce0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415e6080; const P1: u64 = 0x18; const P2: u64 = 0x19; const P3: u64 = 0x20;
    let (mut rax, mut rbx, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; rsi = rdx; r8 = (crate::simd_ops::ru64(rcx)) as u64; r14 = rcx; crate::simd_ops::wu64(rcx, rax); rcx = ((rcx).wrapping_add(0x8u64)) as u64; crate::simd_ops::wu64(rdx, r8); rdx = ((rdx).wrapping_add(0x8u64)) as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rax = ((crate::simd_ops::ru8(r14.wrapping_add(P1))) as u32) as u32 as u64; rax = (rax & !0xffu64) | ((((((rax as u8)) ^ (crate::simd_ops::ru8(rsi.wrapping_add(P1))))) as u8 as u64)); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = (rax & !0xffu64) | ((((((rax as u8)) & (0x1u8))) as u8 as u64)); rax = (rax & !0xffu64) | ((((((rax as u8)) ^ (crate::simd_ops::ru8(r14.wrapping_add(P1))))) as u8 as u64)); rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rcx = (((rax as u8)) as u32) as u32 as u64; crate::simd_ops::wu8(r14.wrapping_add(P1), ((rax as u8)) as u8); rcx = (rcx & !0xffu64) | ((((((rcx as u8)) ^ (crate::simd_ops::ru8(rsi.wrapping_add(P1))))) as u8 as u64)); rcx = (rcx & !0xffu64) | ((((((rcx as u8)) & (0x2u8))) as u8 as u64)); rcx = (rcx & !0xffu64) | ((((((rcx as u8)) ^ ((rax as u8)))) as u8 as u64)); crate::simd_ops::wu8(r14.wrapping_add(P1), ((rcx as u8)) as u8); rcx = (rcx & !0xffu64) | ((((((rcx as u8)) ^ (crate::simd_ops::ru8(rsi.wrapping_add(P1))))) as u8 as u64)); rcx = (rcx & !0xffu64) | ((((((rcx as u8)) & (0x3u8))) as u8 as u64)); rcx = (rcx & !0xffu64) | ((((((rcx as u8)) ^ (crate::simd_ops::ru8(rsi.wrapping_add(P1))))) as u8 as u64)); crate::simd_ops::wu8(r14.wrapping_add(P1), ((rcx as u8)) as u8); rax = ((crate::simd_ops::ru8(rsi.wrapping_add(P2))) as u32) as u32 as u64; crate::simd_ops::wu8(r14.wrapping_add(P2), ((rax as u8)) as u8); rax = (crate::simd_ops::ru32(rsi.wrapping_add(P3))) as u32 as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; crate::simd_ops::wu32(r14.wrapping_add(P3), ((rax as u32)) as u32); rax = r14; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141627d70(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x28; const P2: u64 = 0x141628f50; const P3: u64 = 0x20; const P4: u64 = 0x14162a1e0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x40u64)) as u64;
    rax = ((crate::simd_ops::ru16(rdx)) as u32) as u32 as u64;
    rbx = r8;
    rdi = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rdx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64;
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    r8 = (crate::simd_ops::ru64(rax)) as u64;
    rax = (crate::simd_ops::ru64(rdi)) as u64;
    rcx = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64;
    fl_l = rcx; fl_r = rcx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rdx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); } else { xmm0 = ((crate::simd_ops::ru32(r8.wrapping_add(rdx.wrapping_mul(8)).wrapping_add(P3))) as u32) as u128; }
    rcx = (crate::simd_ops::ru64(rdi)) as u64;
    r9 = rsp.wrapping_add(0x50);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu8(rsp.wrapping_add(0x30u64), (0x0u8) as u8);
    crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rax as u32)) as u32);
    xmm2 = (xmm0) as u128;
    rdx = rbx;
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax);
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64;
    rsp = ((rsp).wrapping_add(0x40u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141627de0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x142690878; const P2: u64 = 0x143930f80; const P3: u64 = 0x1415adef0; const P4: u64 = 0x28; const P5: u64 = 0x8; const P6: u64 = 0x1415242e0; const P7: u64 = 0x14; const P8: u64 = 0x141834d30;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = (crate::simd_ops::ru64(rcx)) as u64; rbp = 0; fl_l = 0; fl_r = 0; rsi = ((rdx as u32)) as u32 as u64; rbx = rcx; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { rdx = (crate::simd_ops::ru64(rdi.wrapping_add(P0))) as u64; rax = P1; crate::simd_ops::wu64(rdi, rax); fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru64(P2)) as u64; r8 = rdi; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rdi.wrapping_add(P0), rbp); bb = 3; continue; }
            3 => { rdx = rdi; crate::simd_ops::wu8(rdi.wrapping_add(P4), ((rbp as u8)) as u8); rcx = 0x10; crate::simd_ops::wu64(rdi.wrapping_add(P5), rbp); rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx, rbp); bb = 4; continue; }
            4 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P5))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rbx.wrapping_add(P0), ((rbp as u32)) as u32); rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(P5), rbp); crate::simd_ops::wu32(rbx.wrapping_add(P7), ((rbp as u32)) as u32); bb = 6; continue; }
            6 => { fl_l = ((rsi as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { rdx = 0x28; rcx = rbx; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); bb = 8; continue; }
            8 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141627e90(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x30; const P1: u64 = 0x10; const P2: u64 = 0x20; const P3: u64 = 0x141628950; const P4: u64 = 0x8; const P5: u64 = 0x60; const P6: u64 = 0x141616050;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x60u64)) as u64;
    rax = (crate::simd_ops::ru64(rdx)) as u64;
    rdi = rcx;
    rcx = rdx;
    rbx = rdx;
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rcx = ((rdi.wrapping_add(P0)) as u64) as u64;
    rdx = rsp.wrapping_add(0x30);
    xmm0 = (crate::simd_ops::ru128(rax)) as u128;
    xmm1 = (crate::simd_ops::ru128(rax.wrapping_add(P1))) as u128;
    crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128);
    xmm0 = (crate::simd_ops::ru128(rax.wrapping_add(P2))) as u128;
    crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm1) as u128);
    crate::simd_ops::wu128(rsp.wrapping_add(0x50u64), (xmm0) as u128);
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbx);
    rdx = rsp.wrapping_add(0x20);
    crate::simd_ops::wu8(rsp.wrapping_add(0x28u64), (0x0u8) as u8);
    rcx = ((rax.wrapping_add(P4)) as u64) as u64;
    fl_l = rax; fl_r = rax;
    if !(((((fl_l as u64)) & ((fl_r as u64))) != 0)) { rcx = ((rdi.wrapping_add(P5)) as u64) as u64; }
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64;
    rsp = ((rsp).wrapping_add(0x60u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141627f00(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xffffffffffffffff;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdi); r9 = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; r10 = 0; fl_l = 0; fl_r = 0; r11 = (((rdx as u8)) as u32) as u32 as u64; rbx = rcx; fl_l = ((r9 as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { rdi = (crate::simd_ops::ru64(rcx)) as u64; bb = 2; continue; }
            2 => { rax = ((r9 as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shr(1u32)) as u32 as u64; rax = (((rax as u32)).wrapping_add((r10 as u32))) as u32 as u64; rcx = ((((rax as u32)) as i32 as i64) as u64) as u64; rdx = ((rcx.wrapping_add(rcx.wrapping_mul(2))) as u64) as u64; rdx = ((rdx.wrapping_mul(8)) as u64) as u64; rcx = ((crate::simd_ops::ru8(rdx.wrapping_add(rdi))) as u32) as u32 as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((r11 as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 5; continue; } bb = 3; continue; }
            3 => { rax = (((rax as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((r11 as u8)) as u8 as u64; rax = (if (((fl_l as u8)) >= ((fl_r as u8))) { (r10 as u32) } else { (rax as u32) }) as u32 as u64; r10 = ((rax as u32)) as u32 as u64; rax = ((r9.wrapping_add(P1)) as u32) as u32 as u64; rax = (if (((fl_l as u8)) >= ((fl_r as u8))) { (r9 as u32) } else { (rax as u32) }) as u32 as u64; r9 = ((rax as u32)) as u32 as u64; fl_l = (((r9 as u32)).wrapping_shr(1u32)) as u32 as u64; r9 = ((fl_l as u32)) as u32 as u64; if (((fl_l as u32)) != 0) { bb = 2; continue; } bb = 4; continue; }
            4 => { rax = ((((r10 as u32)) as i32 as i64) as u64) as u64; crate::simd_ops::wu8(r8, (0x0u8) as u8); rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru64(rbx)) as u64; rax = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x8u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x10u64))) as u64; return rax; }
            5 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x10u64))) as u64; crate::simd_ops::wu8(r8, (0x1u8) as u8); rax = (crate::simd_ops::ru64(rbx)) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x8u64))) as u64; fl_l = ((rax).wrapping_add(rdx)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141627f90(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xffffffffffffffff;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdi); r9 = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; r10 = 0; fl_l = 0; fl_r = 0; r11 = ((rdx as u32)) as u32 as u64; rbx = rcx; fl_l = ((r9 as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { rdi = (crate::simd_ops::ru64(rcx)) as u64; bb = 2; continue; }
            2 => { rax = ((r9 as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shr(1u32)) as u32 as u64; rax = (((rax as u32)).wrapping_add((r10 as u32))) as u32 as u64; rcx = ((((rax as u32)) as i32 as i64) as u64) as u64; rdx = ((rcx.wrapping_add(rcx.wrapping_mul(2))) as u64) as u64; rdx = ((rdx.wrapping_mul(8)) as u64) as u64; rcx = (crate::simd_ops::ru32(rdx.wrapping_add(rdi))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((r11 as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 5; continue; } bb = 3; continue; }
            3 => { rax = (((rax as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((r11 as u32)) as u32 as u64; rax = (if (((fl_l as u32)) >= ((fl_r as u32))) { (r10 as u32) } else { (rax as u32) }) as u32 as u64; r10 = ((rax as u32)) as u32 as u64; rax = ((r9.wrapping_add(P1)) as u32) as u32 as u64; rax = (if (((fl_l as u32)) >= ((fl_r as u32))) { (r9 as u32) } else { (rax as u32) }) as u32 as u64; r9 = ((rax as u32)) as u32 as u64; fl_l = (((r9 as u32)).wrapping_shr(1u32)) as u32 as u64; r9 = ((fl_l as u32)) as u32 as u64; if (((fl_l as u32)) != 0) { bb = 2; continue; } bb = 4; continue; }
            4 => { rax = ((((r10 as u32)) as i32 as i64) as u64) as u64; crate::simd_ops::wu8(r8, (0x0u8) as u8); rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru64(rbx)) as u64; rax = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x8u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x10u64))) as u64; return rax; }
            5 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x10u64))) as u64; crate::simd_ops::wu8(r8, (0x1u8) as u8); rax = (crate::simd_ops::ru64(rbx)) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x8u64))) as u64; fl_l = ((rax).wrapping_add(rdx)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141628020(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xffffffffffffffff;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdi); r9 = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; r10 = 0; fl_l = 0; fl_r = 0; r11 = rdx; rbx = rcx; fl_l = ((r9 as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { rdi = (crate::simd_ops::ru64(rcx)) as u64; bb = 2; continue; }
            2 => { rax = ((r9 as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shr(1u32)) as u32 as u64; rax = (((rax as u32)).wrapping_add((r10 as u32))) as u32 as u64; rcx = ((((rax as u32)) as i32 as i64) as u64) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(rdi))) as u64; fl_l = rdx; fl_r = r11; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 5; continue; } bb = 3; continue; }
            3 => { rax = (((rax as u32)).wrapping_add(1)) as u32 as u64; fl_l = rdx; fl_r = r11; rax = (if (((fl_l as u64)) >= ((fl_r as u64))) { (r10 as u32) } else { (rax as u32) }) as u32 as u64; r10 = ((rax as u32)) as u32 as u64; rax = ((r9.wrapping_add(P1)) as u32) as u32 as u64; rax = (if (((fl_l as u64)) >= ((fl_r as u64))) { (r9 as u32) } else { (rax as u32) }) as u32 as u64; r9 = ((rax as u32)) as u32 as u64; fl_l = (((r9 as u32)).wrapping_shr(1u32)) as u32 as u64; r9 = ((fl_l as u32)) as u32 as u64; if (((fl_l as u32)) != 0) { bb = 2; continue; } bb = 4; continue; }
            4 => { rax = ((((r10 as u32)) as i32 as i64) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; crate::simd_ops::wu8(r8, (0x0u8) as u8); fl_l = ((rax).wrapping_add(crate::simd_ops::ru64(rbx))) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x8u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x10u64))) as u64; return rax; }
            5 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x10u64))) as u64; crate::simd_ops::wu8(r8, (0x1u8) as u8); rax = (crate::simd_ops::ru64(rbx)) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x8u64))) as u64; fl_l = ((rax).wrapping_add(rcx)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416280b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xffffffffffffffff;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdi); r9 = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; r10 = 0; fl_l = 0; fl_r = 0; r11 = rdx; rbx = rcx; fl_l = ((r9 as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { rdi = (crate::simd_ops::ru64(rcx)) as u64; bb = 2; continue; }
            2 => { rax = ((r9 as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shr(1u32)) as u32 as u64; rax = (((rax as u32)).wrapping_add((r10 as u32))) as u32 as u64; rcx = ((((rax as u32)) as i32 as i64) as u64) as u64; rcx = ((rcx).wrapping_shl(5u32)) as u64; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(rdi))) as u64; fl_l = rdx; fl_r = r11; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 5; continue; } bb = 3; continue; }
            3 => { rax = (((rax as u32)).wrapping_add(1)) as u32 as u64; fl_l = rdx; fl_r = r11; rax = (if (((fl_l as u64)) >= ((fl_r as u64))) { (r10 as u32) } else { (rax as u32) }) as u32 as u64; r10 = ((rax as u32)) as u32 as u64; rax = ((r9.wrapping_add(P1)) as u32) as u32 as u64; rax = (if (((fl_l as u64)) >= ((fl_r as u64))) { (r9 as u32) } else { (rax as u32) }) as u32 as u64; r9 = ((rax as u32)) as u32 as u64; fl_l = (((r9 as u32)).wrapping_shr(1u32)) as u32 as u64; r9 = ((fl_l as u32)) as u32 as u64; if (((fl_l as u32)) != 0) { bb = 2; continue; } bb = 4; continue; }
            4 => { rax = ((((r10 as u32)) as i32 as i64) as u64) as u64; rax = ((rax).wrapping_shl(5u32)) as u64; crate::simd_ops::wu8(r8, (0x0u8) as u8); fl_l = ((rax).wrapping_add(crate::simd_ops::ru64(rbx))) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x8u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x10u64))) as u64; return rax; }
            5 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x10u64))) as u64; crate::simd_ops::wu8(r8, (0x1u8) as u8); rax = (crate::simd_ops::ru64(rbx)) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x8u64))) as u64; fl_l = ((rax).wrapping_add(rcx)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141628140(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143930fa0; const P1: u64 = 0x1415c3f50; const P2: u64 = 0x141529ed0; const P3: u64 = 0x1416264d0; const P4: u64 = 0x141625c50; const P5: u64 = 0x141628590;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 17>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = rsp; crate::simd_ops::wu64(rax.wrapping_add(0x18u64), rbx); crate::simd_ops::wu64(rax.wrapping_add(0x20u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x80u64)) as u64; rsi = (crate::simd_ops::ru64(rcx.wrapping_add(0x28u64))) as u64; rbx = rdx; rdi = rcx; fl_l = rdx; fl_r = rsi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 9; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rcx.wrapping_add(0x28u64), rdx); fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 2; continue; }
            2 => { rdx = (crate::simd_ops::ru64(rdx.wrapping_add(0x28u64))) as u64; r8 = rax.wrapping_add(0x8); rcx = (crate::simd_ops::ru64(P0)) as u64; crate::simd_ops::wu128(rax.wrapping_sub(0x18u64), (xmm6) as u128); xmm6 = 0u128; crate::simd_ops::wu32(rax.wrapping_add(0x8u64), ((rsi as u32)) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; fl_r = rsi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x18u64))) as u32 as u64; r8 = rsp.wrapping_add(0x98); xmm1 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x90u64))) as u32) as u128; rcx = rbx.wrapping_add(0x8); crate::simd_ops::wu32(rsp.wrapping_add(0x98u64), ((rax as u32)) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x98u64))) as u32 as u64; xmm6 = (xmm0) as u128; crate::simd_ops::wu32(rbx.wrapping_add(0x18u64), ((rax as u32)) as u32); bb = 4; continue; }
            4 => { crate::simd_ops::wu32(rdi.wrapping_add(0x8u64), ((xmm6 as u32)) as u32); xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x70u64))) as u128; bb = 5; continue; }
            5 => { rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x90u64), rdi); rcx = rdi.wrapping_add(0x30); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rax); crate::simd_ops::wu32(rsp.wrapping_add(0x48u64), ((rax as u32)) as u32); r9 = rsp.wrapping_add(0x40); crate::simd_ops::wu32(rsp.wrapping_add(0x50u64), ((rax as u32)) as u32); r8 = rsp.wrapping_add(0x40); crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), rax); rdx = rsp.wrapping_add(0x20); rax = rsp.wrapping_add(0x90); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rcx); rcx = ((rcx).wrapping_add(0x18u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); crate::simd_ops::wu8(rsp.wrapping_add(0x58u64), (0xffu8) as u8); crate::simd_ops::wu8(rsp.wrapping_add(0x60u64), (0xffu8) as u8); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x68u64))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 7; continue; } bb = 6; continue; }
            6 => { crate::simd_ops::lk_add_u8(rdi.wrapping_add(0x71u64), (1) as u8); rdx = rsp.wrapping_add(0x30); crate::simd_ops::lk_add_u8(rdi.wrapping_add(0x70u64), (0x4u8) as u8); rcx = rdi.wrapping_add(0x60); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rdi); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = ((crate::simd_ops::ru8(rdi.wrapping_add(0x70u64))) as u32) as u32 as u64; rdx = (((rax as u8)) as u32) as u32 as u64; rax = (rax & !0xffu64) | ((((((rax as u8)) & (0x3u8))) as u8 as u64)); rdx = (rdx & !0xffu64) | (((((rdx as u8)).wrapping_shr(2u32)) as u8 as u64)); rdx = (rdx & !0xffu64) | (((((rdx as u8)).wrapping_sub(1)) as u8 as u64)); rdx = (rdx & !0xffu64) | (((((rdx as u8)).wrapping_shl(2u32)) as u8 as u64)); fl_l = ((((rdx as u8)) | ((rax as u8)))) as u8 as u64; fl_r = fl_l; rdx = (rdx & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); crate::simd_ops::wu8(rdi.wrapping_add(0x70u64), ((rdx as u8)) as u8); bb = 7; continue; }
            7 => { fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 9; continue; }
            9 => { r11 = rsp.wrapping_add(0x80); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; rsp = r11; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141628280(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141524330; const P1: u64 = 0x142690880; const P2: u64 = 0x1415242e0; const P3: u64 = 0x14152a3c0; const P4: u64 = 0x143930fa0; const P5: u64 = 0x1415c4750; const P6: u64 = 0x1415c5200;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; r15 = rdx; rbx = rcx; rdx = 0x30; rbp = ((r9 as u32)) as u32 as u64; r14 = r8; rcx = ((rdx.wrapping_sub(0x20u64)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rsi = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu8(rsi.wrapping_add(0x2cu64), (0x8u8) as u8); rax = P1; crate::simd_ops::wu64(rsi, rax); crate::simd_ops::wu64(rsi.wrapping_add(0x20u64), r15); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), r12); r12 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsi.wrapping_add(0x8u64), r12); crate::simd_ops::wu64(rsi.wrapping_add(0x10u64), r12); crate::simd_ops::wu32(rsi.wrapping_add(0x18u64), (0x1u32) as u32); crate::simd_ops::wu64(rsi.wrapping_add(0x28u64), rbx); rdx = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 3; continue; }
            3 => { crate::simd_ops::wu64(rsi.wrapping_add(0x8u64), r12); crate::simd_ops::wu64(rsi.wrapping_add(0x10u64), r12); crate::simd_ops::wu32(rsi.wrapping_add(0x18u64), (0x1u32) as u32); fl_l = r14; fl_r = r14; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 4; continue; }
            4 => { fl_l = ((rbp as u32)) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 11; continue; } bb = 5; continue; }
            5 => { rax = ((rbp.wrapping_add(0x2u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rsi.wrapping_add(0x18u64), (0x1u32) as u32); rdx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u32) as u32 as u64; crate::simd_ops::wu32(rsi.wrapping_add(0x10u64), ((rax as u32)) as u32); rdx = (((rdx as u32)).wrapping_shl(2u32)) as u32 as u64; rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rsi.wrapping_add(0x8u64), rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 6; continue; }
            6 => { r9 = 0; fl_l = 0; fl_r = 0; rcx = rsi.wrapping_add(0x8); r8 = ((rbp as u32)) as u32 as u64; rdx = r14; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(P4)) as u64; r8 = rsi; rdx = rbx; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 11; continue; } bb = 7; continue; }
            7 => { rax = rsi; bb = 8; continue; }
            8 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; bb = 9; continue; }
            9 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            10 => { crate::simd_ops::wu32(rsi.wrapping_add(0x10u64), ((r12 as u32)) as u32); bb = 11; continue; }
            11 => { rdx = (crate::simd_ops::ru64(rsi.wrapping_add(0x28u64))) as u64; r8 = rsi; rcx = (crate::simd_ops::ru64(P4)) as u64; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 12; continue; }
            12 => { rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 13; continue; }
            13 => { crate::simd_ops::wu64(rsi.wrapping_add(0x8u64), r12); rdx = rsi; crate::simd_ops::wu64(rsi.wrapping_add(0x10u64), r12); rcx = 0x10; crate::simd_ops::wu32(rsi.wrapping_add(0x18u64), (0x1u32) as u32); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rax = 0; fl_l = 0; fl_r = 0; bb = 8; continue; }
            14 => { bb = 9; continue; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416283d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143426f34; const P1: u64 = 0x143930f80; const P2: u64 = 0x1415ada30; const P3: u64 = 0x1415ad8b0;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 17>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x80u64)) as u64; fl_l = (crate::simd_ops::ru32(rsp.wrapping_add(0xb8u64))) as u32 as u64; fl_r = 0x2; rsi = r8; crate::simd_ops::wu128(rsp.wrapping_add(0x70u64), (xmm6) as u128); rdi = rcx; xmm6 = ((crate::simd_ops::ru32(r8)) as u32) as u128; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 5; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { xmm0 = ((crate::simd_ops::ru32(rax.wrapping_add(0xcu64))) as u32) as u128; bb = 4; continue; }
            3 => { xmm0 = (xmm1) as u128; bb = 4; continue; }
            4 => { xmm6 = (xmm6 & !0xffff_ffffu128) | ((f32::from_bits(xmm6 as u32) + f32::from_bits(xmm0 as u32)).to_bits() as u128); bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xb0u64))) as u64; r10 = rsp.wrapping_add(0x40); r11 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rsp.wrapping_add(0x44u64), (0x4u32) as u32); fl_l = r9; fl_r = r9; crate::simd_ops::wu32(rsp.wrapping_add(0x40u64), ((r11 as u32)) as u32); rcx = ((r11 as u32)) as u32 as u64; r10 = (if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { r9 } else { r10 }) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 19; continue; } bb = 6; continue; }
            6 => { rdx = (crate::simd_ops::ru32(rax)) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 19; continue; } bb = 7; continue; }
            7 => { fl_l = ((xmm1 as u32)) as u32 as u64; fl_r = ((xmm6 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) == f32::from_bits((fl_r as u32)) || f32::from_bits((fl_l as u32)).is_nan() || f32::from_bits((fl_r as u32)).is_nan()) { bb = 19; continue; } bb = 8; continue; }
            8 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 14; continue; } bb = 9; continue; }
            9 => { fl_l = ((xmm1 as u32)) as u32 as u64; fl_r = ((xmm6 as u32)) as u32 as u64; xmm0 = 0u128; if (f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 12; continue; } bb = 10; continue; }
            10 => { xmm2 = ((crate::simd_ops::ru32(rax.wrapping_add(0x4u64))) as u32) as u128; fl_l = ((xmm2 as u32)) as u32 as u64; fl_r = ((xmm0 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) == f32::from_bits((fl_r as u32)) || f32::from_bits((fl_l as u32)).is_nan() || f32::from_bits((fl_r as u32)).is_nan()) { bb = 12; continue; } bb = 11; continue; }
            11 => { xmm0 = (xmm6) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) - f32::from_bits(xmm1 as u32)).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) / f32::from_bits(xmm2 as u32)).to_bits() as u128); bb = 18; continue; }
            12 => { xmm2 = ((crate::simd_ops::ru32(rax.wrapping_add(0x8u64))) as u32) as u128; fl_l = ((xmm2 as u32)) as u32 as u64; fl_r = ((xmm0 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) == f32::from_bits((fl_r as u32)) || f32::from_bits((fl_l as u32)).is_nan() || f32::from_bits((fl_r as u32)).is_nan()) { bb = 19; continue; } bb = 13; continue; }
            13 => { xmm0 = (xmm1) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) - f32::from_bits(xmm6 as u32)).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) / f32::from_bits(xmm2 as u32)).to_bits() as u128); bb = 18; continue; }
            14 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 19; continue; } bb = 15; continue; }
            15 => { fl_l = ((xmm1 as u32)) as u32 as u64; fl_r = ((xmm6 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 17; continue; } bb = 16; continue; }
            16 => { xmm0 = ((crate::simd_ops::ru32(rax.wrapping_add(0x4u64))) as u32) as u128; bb = 18; continue; }
            17 => { xmm0 = ((crate::simd_ops::ru32(rax.wrapping_add(0x8u64))) as u32) as u128; bb = 18; continue; }
            18 => { xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(crate::simd_ops::ru32(P0))).to_bits() as u128); rcx = (crate::simd_ops::cvt_f32_i32((f32::from_bits(xmm0 as u32)))) as u32 as u64; bb = 19; continue; }
            19 => { r8 = (crate::simd_ops::ru32(r10)) as u32 as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; r8 = (if ((((fl_l as u32)) as i32) <= (((fl_r as u32)) as i32)) { (rcx as u32) } else { (r8 as u32) }) as u32 as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 30; continue; } bb = 20; continue; }
            20 => { rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 22; continue; } bb = 21; continue; }
            21 => { rax = (crate::simd_ops::ru32(r10.wrapping_add(0x4u64))) as u32 as u64; xmm3 = (xmm6) as u128; rcx = (crate::simd_ops::ru64(P1)) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((r11 as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rax as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((r8 as u32)) as u32); r8 = 0x10000000; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 28; continue; }
            22 => { fl_l = ((xmm6 as u32)) as u32 as u64; fl_r = ((xmm1 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) == f32::from_bits((fl_r as u32)) || f32::from_bits((fl_l as u32)).is_nan() || f32::from_bits((fl_r as u32)).is_nan()) { bb = 28; continue; } bb = 23; continue; }
            23 => { fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 26; continue; } bb = 24; continue; }
            24 => { fl_l = (crate::simd_ops::ru32(rax)) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 26; continue; } bb = 25; continue; }
            25 => { rcx = (rcx & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 27; continue; }
            26 => { rcx = (rcx & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 27; continue; }
            27 => { rax = (crate::simd_ops::ru32(r10.wrapping_add(0x4u64))) as u32 as u64; rdx = rsp.wrapping_add(0x48); crate::simd_ops::wu32(rsp.wrapping_add(0x60u64), ((r8 as u32)) as u32); r8 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu8(rsp.wrapping_add(0x6eu64), ((rcx as u8)) as u8); rcx = (crate::simd_ops::ru64(P1)) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x58u64), ((xmm1 as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x5cu64), ((xmm6 as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), (0x10000000u64) as u64); crate::simd_ops::wu32(rsp.wrapping_add(0x64u64), ((rax as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x68u64), (0x9u32) as u32); crate::simd_ops::wu16(rsp.wrapping_add(0x6cu64), ((r11 as u16)) as u16); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rdi.wrapping_add(0x10u64), rax); bb = 28; continue; }
            28 => { fl_l = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 30; continue; } bb = 29; continue; }
            29 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0xc0u64))) as u64; rax = ((crate::simd_ops::ru8(rcx)) as u32) as u32 as u64; crate::simd_ops::wu8(rdi.wrapping_add(0x28u64), ((rax as u8)) as u8); rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); crate::simd_ops::wu32(rsi, ((xmm6 as u32)) as u32); crate::simd_ops::wu8(rcx, (0x0u8) as u8); bb = 31; continue; }
            30 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 31; continue; }
            31 => { xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x70u64))) as u128; r11 = rsp.wrapping_add(0x80); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x10u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x18u64))) as u64; rsp = r11; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141628590(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x28; const P1: u64 = 0x143930fa0; const P2: u64 = 0x1415c5200; const P3: u64 = 0x8; const P4: u64 = 0x1415242e0; const P5: u64 = 0x10; const P6: u64 = 0x18;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rbx = rcx;
    r8 = rcx;
    rcx = (crate::simd_ops::ru64(P1)) as u64;
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P3))) as u64;
    fl_l = rdx; fl_r = rdx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); }
    rax = 0;
    fl_l = 0; fl_r = 0;
    rdx = rbx;
    crate::simd_ops::wu64(rbx.wrapping_add(P3), rax);
    crate::simd_ops::wu64(rbx.wrapping_add(P5), rax);
    crate::simd_ops::wu32(rbx.wrapping_add(P6), (0x1u32) as u32);
    rcx = ((rax.wrapping_add(P5)) as u32) as u32 as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(P4, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416285e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xffffffffffffffff;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdi); r8 = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; r11 = 0; fl_l = 0; fl_r = 0; r10 = (((rdx as u8)) as u32) as u32 as u64; rdi = rcx; r9 = ((r11 as u32)) as u32 as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { rbx = (crate::simd_ops::ru64(rcx)) as u64; bb = 2; continue; }
            2 => { rax = ((r8 as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shr(1u32)) as u32 as u64; rax = (((rax as u32)).wrapping_add((r9 as u32))) as u32 as u64; rcx = ((((rax as u32)) as i32 as i64) as u64) as u64; rdx = ((rcx.wrapping_add(rcx.wrapping_mul(2))) as u64) as u64; rcx = ((rbx.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64; rdx = ((crate::simd_ops::ru8(rbx.wrapping_add(rdx.wrapping_mul(8)))) as u32) as u32 as u64; fl_l = ((rdx as u8)) as u8 as u64; fl_r = ((r10 as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 5; continue; } bb = 3; continue; }
            3 => { rax = (((rax as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((rdx as u8)) as u8 as u64; fl_r = ((r10 as u8)) as u8 as u64; rax = (if (((fl_l as u8)) >= ((fl_r as u8))) { (r9 as u32) } else { (rax as u32) }) as u32 as u64; r9 = ((rax as u32)) as u32 as u64; rax = ((r8.wrapping_add(P1)) as u32) as u32 as u64; rax = (if (((fl_l as u8)) >= ((fl_r as u8))) { (r8 as u32) } else { (rax as u32) }) as u32 as u64; r8 = ((rax as u32)) as u32 as u64; fl_l = (((r8 as u32)).wrapping_shr(1u32)) as u32 as u64; r8 = ((fl_l as u32)) as u32 as u64; if (((fl_l as u32)) != 0) { bb = 2; continue; } bb = 4; continue; }
            4 => { rcx = ((((r9 as u32)) as i32 as i64) as u64) as u64; rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; rdx = ((rcx.wrapping_add(rcx.wrapping_mul(2))) as u64) as u64; rcx = (crate::simd_ops::ru64(rdi)) as u64; rcx = ((rcx.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64; r11 = (if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { rcx } else { r11 }) as u64; rax = r11; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x8u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x10u64))) as u64; return rax; }
            5 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x8u64))) as u64; rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x10u64))) as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; r11 = (if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { rcx } else { r11 }) as u64; rax = r11; return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141628690(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xffffffffffffffff;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdi); r8 = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; r11 = 0; fl_l = 0; fl_r = 0; r10 = ((rdx as u32)) as u32 as u64; rdi = rcx; r9 = ((r11 as u32)) as u32 as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { rbx = (crate::simd_ops::ru64(rcx)) as u64; bb = 2; continue; }
            2 => { rax = ((r8 as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shr(1u32)) as u32 as u64; rax = (((rax as u32)).wrapping_add((r9 as u32))) as u32 as u64; rcx = ((((rax as u32)) as i32 as i64) as u64) as u64; rdx = ((rcx.wrapping_add(rcx.wrapping_mul(2))) as u64) as u64; rcx = ((rbx.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64; rdx = (crate::simd_ops::ru32(rbx.wrapping_add(rdx.wrapping_mul(8)))) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((r10 as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 5; continue; } bb = 3; continue; }
            3 => { rax = (((rax as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((r10 as u32)) as u32 as u64; rax = (if (((fl_l as u32)) >= ((fl_r as u32))) { (r9 as u32) } else { (rax as u32) }) as u32 as u64; r9 = ((rax as u32)) as u32 as u64; rax = ((r8.wrapping_add(P1)) as u32) as u32 as u64; rax = (if (((fl_l as u32)) >= ((fl_r as u32))) { (r8 as u32) } else { (rax as u32) }) as u32 as u64; r8 = ((rax as u32)) as u32 as u64; fl_l = (((r8 as u32)).wrapping_shr(1u32)) as u32 as u64; r8 = ((fl_l as u32)) as u32 as u64; if (((fl_l as u32)) != 0) { bb = 2; continue; } bb = 4; continue; }
            4 => { rcx = ((((r9 as u32)) as i32 as i64) as u64) as u64; rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; rdx = ((rcx.wrapping_add(rcx.wrapping_mul(2))) as u64) as u64; rcx = (crate::simd_ops::ru64(rdi)) as u64; rcx = ((rcx.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64; r11 = (if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { rcx } else { r11 }) as u64; rax = r11; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x8u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x10u64))) as u64; return rax; }
            5 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x8u64))) as u64; rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x10u64))) as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; r11 = (if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { rcx } else { r11 }) as u64; rax = r11; return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141628740(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xffffffffffffffff;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdi); r8 = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; r11 = 0; fl_l = 0; fl_r = 0; r10 = rdx; rdi = rcx; r9 = ((r11 as u32)) as u32 as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { rbx = (crate::simd_ops::ru64(rcx)) as u64; bb = 2; continue; }
            2 => { rax = ((r8 as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shr(1u32)) as u32 as u64; rax = (((rax as u32)).wrapping_add((r9 as u32))) as u32 as u64; rdx = ((((rax as u32)) as i32 as i64) as u64) as u64; rdx = ((rdx).wrapping_shl(5u32)) as u64; rdx = ((rdx).wrapping_add(rbx)) as u64; rcx = (crate::simd_ops::ru64(rdx)) as u64; fl_l = rcx; fl_r = r10; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 5; continue; } bb = 3; continue; }
            3 => { rax = (((rax as u32)).wrapping_add(1)) as u32 as u64; fl_l = rcx; fl_r = r10; rax = (if (((fl_l as u64)) >= ((fl_r as u64))) { (r9 as u32) } else { (rax as u32) }) as u32 as u64; r9 = ((rax as u32)) as u32 as u64; rax = ((r8.wrapping_add(P1)) as u32) as u32 as u64; rax = (if (((fl_l as u64)) >= ((fl_r as u64))) { (r8 as u32) } else { (rax as u32) }) as u32 as u64; r8 = ((rax as u32)) as u32 as u64; fl_l = (((r8 as u32)).wrapping_shr(1u32)) as u32 as u64; r8 = ((fl_l as u32)) as u32 as u64; if (((fl_l as u32)) != 0) { bb = 2; continue; } bb = 4; continue; }
            4 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rdx = ((((r9 as u32)) as i32 as i64) as u64) as u64; rdx = ((rdx).wrapping_shl(5u32)) as u64; rdx = ((rdx).wrapping_add(crate::simd_ops::ru64(rdi))) as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; r11 = (if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { rdx } else { r11 }) as u64; rax = r11; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x8u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x10u64))) as u64; return rax; }
            5 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x8u64))) as u64; rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x10u64))) as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; r11 = (if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { rdx } else { r11 }) as u64; rax = r11; return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416287e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14162a420; const P1: u64 = 0x141625ba0; const P2: u64 = 0x141618910;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 19>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x80u64)) as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x70u64), (xmm6) as u128); rdi = r9; rax = r9.wrapping_add(0x20); rbp = r8; r14 = rdx; rsi = rcx; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { xmm6 = ((crate::simd_ops::ru32(r8.wrapping_add(0x20u64))) as u32) as u128; fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 6; continue; } bb = 2; continue; }
            2 => { rax = rcx.wrapping_add(0x8); bb = 6; continue; }
            3 => { fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = rcx.wrapping_add(0x8); bb = 5; continue; }
            5 => { xmm6 = ((crate::simd_ops::ru32(rax)) as u32) as u128; bb = 6; continue; }
            6 => { xmm0 = ((crate::simd_ops::ru32(rax)) as u32) as u128; rbx = (rbx & !0xffu64) | (((0x1u8) as u8 as u64)); fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = ((xmm6 as u32)) as u32 as u64; crate::simd_ops::wu8(rsp.wrapping_add(0xb0u64), ((rbx as u8)) as u8); crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((xmm0 as u32)) as u32); if (f32::from_bits((fl_l as u32)) == f32::from_bits((fl_r as u32)) || f32::from_bits((fl_l as u32)).is_nan() || f32::from_bits((fl_r as u32)).is_nan()) { bb = 8; continue; } bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xc0u64))) as u64; r9 = rsp.wrapping_add(0x50); crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rax); r8 = rsp.wrapping_add(0x30); rax = rsp.wrapping_add(0xb8); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); xmm1 = (xmm6) as u128; rax = rsp.wrapping_add(0xb0); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), r14); rcx = rbp; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); crate::simd_ops::wu32(rsp.wrapping_add(0x68u64), (0x1u32) as u32); crate::simd_ops::wu8(rsp.wrapping_add(0x6cu64), ((rbx as u8)) as u8); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm1 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x30u64))) as u32) as u128; rdx = rsp.wrapping_add(0x50); crate::simd_ops::lk_add_u8(rsi.wrapping_add(0x89u64), (1) as u8); rcx = rsi.wrapping_add(0x78); crate::simd_ops::lk_add_u8(rsi.wrapping_add(0x88u64), (0x4u8) as u8); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rsi); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x38u64))) as u128; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), r14); xmm0 = crate::simd_ops::vshufps(xmm0, xmm0, 0xd2u8); xmm0 = (xmm0 & !0xffff_ffffu128) | (xmm6 & 0xffff_ffffu128); xmm0 = crate::simd_ops::vshufps(xmm0, xmm0, 0x27u8); xmm0 = (xmm0 & !0xffff_ffffu128) | (xmm1 & 0xffff_ffffu128); xmm0 = crate::simd_ops::vshufps(xmm0, xmm0, 0x39u8); crate::simd_ops::wu128(rsp.wrapping_add(0x50u64), (xmm0) as u128); xmm0 = ((crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64) as u128; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), ((xmm0 as u64)) as u64); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = ((crate::simd_ops::ru8(rsi.wrapping_add(0x88u64))) as u32) as u32 as u64; rbx = ((crate::simd_ops::ru8(rsp.wrapping_add(0xb0u64))) as u32) as u32 as u64; rcx = (((rax as u8)) as u32) as u32 as u64; rcx = (rcx & !0xffu64) | (((((rcx as u8)).wrapping_shr(2u32)) as u8 as u64)); rax = (rax & !0xffu64) | ((((((rax as u8)) & (0x3u8))) as u8 as u64)); rcx = (rcx & !0xffu64) | (((((rcx as u8)).wrapping_sub(1)) as u8 as u64)); rcx = (rcx & !0xffu64) | (((((rcx as u8)).wrapping_shl(2u32)) as u8 as u64)); rcx = (rcx & !0xffu64) | ((((((rcx as u8)) | ((rax as u8)))) as u8 as u64)); crate::simd_ops::wu8(rsi.wrapping_add(0x88u64), ((rcx as u8)) as u8); fl_l = ((rbx as u8)) as u8 as u64; fl_r = ((rbx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { fl_l = rdi; fl_r = rdi; rax = rsi.wrapping_add(0x60); rdx = rdi.wrapping_add(0x8); rdx = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rax } else { rdx }) as u64; rcx = rbp.wrapping_add(0x8); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 9; continue; }
            9 => { xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x70u64))) as u128; r11 = rsp.wrapping_add(0x80); rbp = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; rax = (((rbx as u8)) as u32) as u32 as u64; rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rsp = r11; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141628a50(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1416285e0; const P1: u64 = 0x1415fa0e0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rdi = ((crate::simd_ops::ru8(rdx)) as u32) as u32 as u64; r15 = 0; fl_l = 0; fl_r = 0; r14 = rdx; r13 = rcx; fl_l = ((rdi as u8)) as u8 as u64; fl_r = (0xffu8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 3; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0x8u64))) as u8 as u64; fl_r = ((rdi as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 3; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru64(rdx.wrapping_add(0x10u64))) as u64; fl_r = r15; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 20; continue; } bb = 3; continue; }
            3 => { rdx = (((rdi as u8)) as u32) as u32 as u64; rcx = ((rcx).wrapping_add(0x8u64)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 6; continue; } bb = 4; continue; }
            4 => { fl_l = ((rdi as u8)) as u8 as u64; fl_r = (0xffu8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 20; continue; } bb = 5; continue; }
            5 => { rdx = (rdx & !0xffu64) | (((0xffu8) as u8 as u64)); crate::simd_ops::wu8(r14, (0xffu8) as u8); rcx = r13.wrapping_add(0x8); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 6; continue; }
            6 => { rbp = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 20; continue; } bb = 7; continue; }
            7 => { r15 = ((crate::simd_ops::ru8(r14.wrapping_add(0x8u64))) as u32) as u32 as u64; fl_l = ((r15 as u8)) as u8 as u64; fl_r = (0xffu8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 9; continue; } bb = 8; continue; }
            8 => { fl_l = (crate::simd_ops::ru64(r14.wrapping_add(0x10u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 18; continue; } bb = 9; continue; }
            9 => { rdx = (((r15 as u8)) as u32) as u32 as u64; rcx = rax.wrapping_add(0x8); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rsi = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 12; continue; } bb = 10; continue; }
            10 => { rax = rbp; fl_l = ((r15 as u8)) as u8 as u64; fl_r = (0xffu8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 18; continue; } bb = 11; continue; }
            11 => { rdx = (rdx & !0xffu64) | (((0xffu8) as u8 as u64)); crate::simd_ops::wu8(r14.wrapping_add(0x8u64), (0xffu8) as u8); rcx = rbp.wrapping_add(0x8); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rsi = rax; bb = 12; continue; }
            12 => { rax = rbp; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 18; continue; } bb = 13; continue; }
            13 => { rdx = (crate::simd_ops::ru64(r14.wrapping_add(0x10u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 16; continue; } bb = 14; continue; }
            14 => { rcx = rsi.wrapping_add(0x8); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 16; continue; } bb = 15; continue; }
            15 => { rcx = rax.wrapping_add(0x8); bb = 17; continue; }
            16 => { rax = 0; fl_l = 0; fl_r = 0; rcx = rsi.wrapping_add(0x2); crate::simd_ops::wu64(r14.wrapping_add(0x10u64), rax); fl_l = (crate::simd_ops::ru8(rsi.wrapping_add(0x4u64))) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; rcx = (if (((fl_l as u8)) == ((fl_r as u8))) { rax } else { rcx }) as u64; bb = 17; continue; }
            17 => { rax = rbp; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 19; continue; } bb = 18; continue; }
            18 => { r15 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu8(rsp.wrapping_add(0x20u64), (0xffu8) as u8); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), r15); rcx = rax.wrapping_add(0x2); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; crate::simd_ops::wu128(r14.wrapping_add(0x8u64), (xmm0) as u128); fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x4u64))) as u8 as u64; fl_r = ((r15 as u8)) as u8 as u64; rcx = (if (((fl_l as u8)) == ((fl_r as u8))) { r15 } else { rcx }) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 20; continue; } bb = 19; continue; }
            19 => { rax = rcx; bb = 21; continue; }
            20 => { crate::simd_ops::wu8(rsp.wrapping_add(0x20u64), (0xffu8) as u8); rax = r13.wrapping_add(0x4); crate::simd_ops::wu8(rsp.wrapping_add(0x28u64), (0xffu8) as u8); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), r15); xmm1 = ((crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64) as u128; crate::simd_ops::wu128(r14, (xmm0) as u128); crate::simd_ops::wu64(r14.wrapping_add(0x10u64), ((xmm1 as u64)) as u64); fl_l = (crate::simd_ops::ru8(r13.wrapping_add(0x6u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; rax = (if (((fl_l as u8)) == ((fl_r as u8))) { r15 } else { rax }) as u64; bb = 21; continue; }
            21 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141628bd0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141628690; const P1: u64 = 0x141628a50;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x50u64)) as u64; rsi = (crate::simd_ops::ru32(rdx)) as u32 as u64; r12 = 0; fl_l = 0; fl_r = 0; rdi = rdx; r13 = rcx; fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 5; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru32(rdx.wrapping_add(0x8u64))) as u32 as u64; fl_r = ((r12 as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 5; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0x10u64))) as u8 as u64; fl_r = (0xffu8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 5; continue; } bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0x18u64))) as u8 as u64; fl_r = (0xffu8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 5; continue; } bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::ru64(rdx.wrapping_add(0x20u64))) as u64; fl_r = r12; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 18; continue; } bb = 5; continue; }
            5 => { rdx = ((rsi as u32)) as u32 as u64; rcx = ((rcx).wrapping_add(0x10u64)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 8; continue; } bb = 6; continue; }
            6 => { fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 18; continue; } bb = 7; continue; }
            7 => { rdx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rdi, ((r12 as u32)) as u32); rcx = r13.wrapping_add(0x10); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 18; continue; } bb = 8; continue; }
            8 => { fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; fl_r = ((r12 as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 12; continue; } bb = 9; continue; }
            9 => { fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(0x10u64))) as u8 as u64; fl_r = (0xffu8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 12; continue; } bb = 10; continue; }
            10 => { fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(0x18u64))) as u8 as u64; fl_r = (0xffu8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 12; continue; } bb = 11; continue; }
            11 => { fl_l = (crate::simd_ops::ru64(rdi.wrapping_add(0x20u64))) as u64; fl_r = r12; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 16; continue; } bb = 12; continue; }
            12 => { rdx = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; rcx = rax.wrapping_add(0x8); rbp = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 15; continue; } bb = 13; continue; }
            13 => { rax = rbp; fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; fl_r = ((r12 as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 16; continue; } bb = 14; continue; }
            14 => { rdx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rdi.wrapping_add(0x8u64), ((r12 as u32)) as u32); rcx = rbp.wrapping_add(0x8); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rax; rax = rbp; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 16; continue; } bb = 15; continue; }
            15 => { rdx = rdi.wrapping_add(0x10); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rax; rax = rbp; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 17; continue; } bb = 16; continue; }
            16 => { crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((r12 as u32)) as u32); rcx = rax.wrapping_add(0x4); crate::simd_ops::wu8(rsp.wrapping_add(0x28u64), (0xffu8) as u8); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; crate::simd_ops::wu8(rsp.wrapping_add(0x30u64), (0xffu8) as u8); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), r12); xmm1 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; crate::simd_ops::wu128(rdi.wrapping_add(0x8u64), (xmm0) as u128); crate::simd_ops::wu128(rdi.wrapping_add(0x18u64), (xmm1) as u128); fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x6u64))) as u8 as u64; fl_r = ((r12 as u8)) as u8 as u64; rcx = (if (((fl_l as u8)) == ((fl_r as u8))) { r12 } else { rcx }) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 18; continue; } bb = 17; continue; }
            17 => { rax = rcx; bb = 19; continue; }
            18 => { crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((r12 as u32)) as u32); rax = r13.wrapping_add(0x8); crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((r12 as u32)) as u32); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; crate::simd_ops::wu8(rsp.wrapping_add(0x30u64), (0xffu8) as u8); crate::simd_ops::wu8(rsp.wrapping_add(0x38u64), (0xffu8) as u8); xmm1 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), r12); crate::simd_ops::wu128(rdi, (xmm0) as u128); xmm0 = ((crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64) as u128; crate::simd_ops::wu128(rdi.wrapping_add(0x10u64), (xmm1) as u128); crate::simd_ops::wu64(rdi.wrapping_add(0x20u64), ((xmm0 as u64)) as u64); fl_l = (crate::simd_ops::ru8(r13.wrapping_add(0xau64))) as u8 as u64; fl_r = ((r12 as u8)) as u8 as u64; rax = (if (((fl_l as u8)) == ((fl_r as u8))) { r12 } else { rax }) as u64; bb = 19; continue; }
            19 => { r11 = rsp.wrapping_add(0x50); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rbp = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x30u64))) as u64; rdi = (crate::simd_ops::ru64(r11.wrapping_add(0x38u64))) as u64; rsp = r11; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141628d50(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141628740; const P1: u64 = 0x141628690; const P2: u64 = 0x1416285e0; const P3: u64 = 0x1415fa0e0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; rdi = rdx.wrapping_add(0x8); rbx = 0; fl_l = 0; fl_r = 0; r15 = r8; r14 = rcx; rsi = ((rbx as u32)) as u32 as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 7; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru32(rdi)) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 7; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 7; continue; } bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(0x10u64))) as u8 as u64; fl_r = (0xffu8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 7; continue; } bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(0x18u64))) as u8 as u64; fl_r = (0xffu8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 7; continue; } bb = 5; continue; }
            5 => { fl_l = (crate::simd_ops::ru64(rdi.wrapping_add(0x20u64))) as u64; fl_r = rbx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rcx = ((rcx).wrapping_add(0x1au64)) as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0x2u64))) as u8 as u64; fl_r = ((rbx as u8)) as u8 as u64; rcx = (if (((fl_l as u8)) == ((fl_r as u8))) { rbx } else { rcx }) as u64; bb = 44; continue; }
            7 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0x1cu64))) as u8 as u64; fl_r = ((rbx as u8)) as u8 as u64; rsi = rcx.wrapping_add(0x1a); rdx = rax; rsi = (if (((fl_l as u8)) == ((fl_r as u8))) { rbx } else { rsi }) as u64; rcx = ((rcx).wrapping_add(0x20u64)) as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 46; continue; } bb = 8; continue; }
            8 => { rdx = (crate::simd_ops::ru32(rdi)) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 14; continue; } bb = 9; continue; }
            9 => { fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 14; continue; } bb = 10; continue; }
            10 => { fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(0x10u64))) as u8 as u64; fl_r = (0xffu8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 14; continue; } bb = 11; continue; }
            11 => { fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(0x18u64))) as u8 as u64; fl_r = (0xffu8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 14; continue; } bb = 12; continue; }
            12 => { fl_l = (crate::simd_ops::ru64(rdi.wrapping_add(0x20u64))) as u64; fl_r = rbx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 14; continue; } bb = 13; continue; }
            13 => { fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0xau64))) as u8 as u64; fl_r = ((rbx as u8)) as u8 as u64; rcx = rax.wrapping_add(0x8); rcx = (if (((fl_l as u8)) == ((fl_r as u8))) { rbx } else { rcx }) as u64; bb = 44; continue; }
            14 => { fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0xau64))) as u8 as u64; fl_r = ((rbx as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 16; continue; } bb = 15; continue; }
            15 => { rsi = rax.wrapping_add(0x8); bb = 16; continue; }
            16 => { rcx = rax.wrapping_add(0x10); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 43; continue; } bb = 17; continue; }
            17 => { rdx = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 22; continue; } bb = 18; continue; }
            18 => { fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(0x10u64))) as u8 as u64; fl_r = (0xffu8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 22; continue; } bb = 19; continue; }
            19 => { fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(0x18u64))) as u8 as u64; fl_r = (0xffu8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 22; continue; } bb = 20; continue; }
            20 => { fl_l = (crate::simd_ops::ru64(rdi.wrapping_add(0x20u64))) as u64; fl_r = rbx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 22; continue; } bb = 21; continue; }
            21 => { fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x6u64))) as u8 as u64; fl_r = ((rbx as u8)) as u8 as u64; rcx = rax.wrapping_add(0x4); rcx = (if (((fl_l as u8)) == ((fl_r as u8))) { rbx } else { rcx }) as u64; bb = 44; continue; }
            22 => { fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x6u64))) as u8 as u64; fl_r = ((rbx as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 24; continue; } bb = 23; continue; }
            23 => { rsi = rax.wrapping_add(0x4); bb = 24; continue; }
            24 => { rcx = rax.wrapping_add(0x8); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 43; continue; } bb = 25; continue; }
            25 => { rdx = ((crate::simd_ops::ru8(rdi.wrapping_add(0x10u64))) as u32) as u32 as u64; fl_l = ((rdx as u8)) as u8 as u64; fl_r = (0xffu8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 29; continue; } bb = 26; continue; }
            26 => { fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(0x18u64))) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 29; continue; } bb = 27; continue; }
            27 => { fl_l = (crate::simd_ops::ru64(rdi.wrapping_add(0x20u64))) as u64; fl_r = rbx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 29; continue; } bb = 28; continue; }
            28 => { fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x6u64))) as u8 as u64; fl_r = ((rbx as u8)) as u8 as u64; rcx = rax.wrapping_add(0x4); rcx = (if (((fl_l as u8)) == ((fl_r as u8))) { rbx } else { rcx }) as u64; bb = 44; continue; }
            29 => { fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x6u64))) as u8 as u64; fl_r = ((rbx as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 31; continue; } bb = 30; continue; }
            30 => { rsi = rax.wrapping_add(0x4); bb = 31; continue; }
            31 => { rcx = rax.wrapping_add(0x8); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 43; continue; } bb = 32; continue; }
            32 => { rdx = ((crate::simd_ops::ru8(rdi.wrapping_add(0x18u64))) as u32) as u32 as u64; fl_l = ((rdx as u8)) as u8 as u64; fl_r = (0xffu8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 35; continue; } bb = 33; continue; }
            33 => { fl_l = (crate::simd_ops::ru64(rdi.wrapping_add(0x20u64))) as u64; fl_r = rbx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 35; continue; } bb = 34; continue; }
            34 => { fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x4u64))) as u8 as u64; fl_r = ((rbx as u8)) as u8 as u64; rcx = rax.wrapping_add(0x2); rcx = (if (((fl_l as u8)) == ((fl_r as u8))) { rbx } else { rcx }) as u64; bb = 44; continue; }
            35 => { fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x4u64))) as u8 as u64; fl_r = ((rbx as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 37; continue; } bb = 36; continue; }
            36 => { rsi = rax.wrapping_add(0x2); bb = 37; continue; }
            37 => { rcx = rax.wrapping_add(0x8); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 43; continue; } bb = 38; continue; }
            38 => { rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x20u64))) as u64; r8 = ((crate::simd_ops::ru8(rax.wrapping_add(0x4u64))) as u32) as u32 as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 42; continue; } bb = 39; continue; }
            39 => { fl_l = ((r8 as u8)) as u8 as u64; fl_r = ((r8 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 41; continue; } bb = 40; continue; }
            40 => { rsi = rax.wrapping_add(0x2); bb = 41; continue; }
            41 => { rcx = rax.wrapping_add(0x8); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; rcx = rax.wrapping_add(0x8); rcx = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rbx } else { rcx }) as u64; bb = 44; continue; }
            42 => { fl_l = ((r8 as u8)) as u8 as u64; fl_r = ((r8 as u8)) as u8 as u64; rcx = rax.wrapping_add(0x2); rcx = (if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { rbx } else { rcx }) as u64; bb = 44; continue; }
            43 => { rcx = rbx; bb = 44; continue; }
            44 => { fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 46; continue; } bb = 45; continue; }
            45 => { rax = ((crate::simd_ops::ru16(rcx)) as u32) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = (crate::simd_ops::ru64(r14)) as u64; rbx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; bb = 46; continue; }
            46 => { rax = rbx; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 48; continue; } bb = 47; continue; }
            47 => { rcx = ((crate::simd_ops::ru16(rsi)) as u32) as u32 as u64; rdx = ((rcx.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64; rcx = (crate::simd_ops::ru64(r14)) as u64; rdx = ((rcx.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64; crate::simd_ops::wu64(r15, rdx); bb = 48; continue; }
            48 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141628f50(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x20; const P2: u64 = 0x28; const P3: u64 = 0x143930fa0; const P4: u64 = 0x1415c3fb0; const P5: u64 = 0x8; const P6: u64 = 0x18; const P7: u64 = 0x141529ed0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x60u64)) as u64;
    xmm0 = (crate::simd_ops::ru128(rdx)) as u128;
    rbx = rcx;
    r9 = rsp.wrapping_add(0x78);
    xmm1 = (crate::simd_ops::ru128(rdx.wrapping_add(P0))) as u128;
    r8 = rsp.wrapping_add(0x30);
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (0x0u64) as u64);
    crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128);
    xmm0 = (crate::simd_ops::ru128(rdx.wrapping_add(P1))) as u128;
    rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64;
    rcx = (crate::simd_ops::ru64(P3)) as u64;
    crate::simd_ops::wu128(rsp.wrapping_add(0x50u64), (xmm0) as u128);
    crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm1) as u128);
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(P5))) as u64; fl_r = (0x0u64) as u64;
    if !((((fl_l as u64)) == ((fl_r as u64)))) { rax = (crate::simd_ops::ru32(rbx.wrapping_add(P6))) as u32 as u64; r8 = rsp.wrapping_add(0x70); xmm1 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x78u64))) as u32) as u128; rcx = ((rbx.wrapping_add(P5)) as u64) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x70u64), ((rax as u32)) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x70u64))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(P6), ((rax as u32)) as u32); rsp = ((rsp).wrapping_add(0x60u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
    xmm0 = 0u128;
    rsp = ((rsp).wrapping_add(0x60u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141628fd0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141628950; const P1: u64 = 0x20; const P2: u64 = 0x8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdi = rcx;
    rbx = r8;
    rcx = ((rcx).wrapping_add(0x30u64)) as u64;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    fl_l = rax; fl_r = rax;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rax = (crate::simd_ops::ru32(rax.wrapping_add(P1))) as u32 as u64; crate::simd_ops::wu32(rbx, ((rax as u32)) as u32); rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    rax = (crate::simd_ops::ru32(rdi.wrapping_add(P2))) as u32 as u64;
    crate::simd_ops::wu32(rbx, ((rax as u32)) as u32);
    rax = (rax & !0xffu64) | (((0u8) as u8 as u64));
    fl_l = 0; fl_r = 0;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141629020(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141524340; const P1: u64 = 0x141524330; const P2: u64 = 0x1415242e0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdx = (crate::simd_ops::ru32(rcx.wrapping_add(0xcu64))) as u32 as u64; rdi = rcx; rbp = ((rdx as u32)) as u32 as u64; rcx = 0x1; rbp = (((rbp as u32)).wrapping_shr(1u32)) as u32 as u64; rbp = (((rbp as u32)).wrapping_add((rdx as u32))) as u32 as u64; rax = (crate::simd_ops::ru64(rdi)) as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; rsi = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; rbp = (if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { (rcx as u32) } else { (rbp as u32) }) as u32 as u64; rcx = 0; fl_l = 0; fl_r = 0; rbp = (((rbp as u32)).wrapping_add((rdx as u32))) as u32 as u64; rdx = ((rbp as u32)) as u32 as u64; rdx = ((rdx).wrapping_shl(4u32)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { r8 = rdx; rdx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 12; continue; } bb = 2; continue; }
            2 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 13; continue; }
            3 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rdi)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 5; continue; }
            5 => { fl_l = rax; fl_r = rbx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 12; continue; } bb = 6; continue; }
            6 => { fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 7; continue; }
            7 => { rax = rbx; r8 = 0; fl_l = 0; fl_r = 0; bb = 8; continue; }
            8 => { fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { crate::simd_ops::wu64(rax, r8); bb = 10; continue; }
            10 => { rcx = (crate::simd_ops::ru64(rdi)) as u64; rdx = rax; rdx = ((rdx).wrapping_sub(rbx)) as u64; xmm0 = (crate::simd_ops::ru128(rdx.wrapping_add(rcx))) as u128; crate::simd_ops::wu128(rax, (xmm0) as u128); rax = ((rax).wrapping_add(0x10u64)) as u64; fl_l = rsi; fl_r = (0x1u64) as u64; rsi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 8; continue; } bb = 11; continue; }
            11 => { rdx = (crate::simd_ops::ru64(rdi)) as u64; rcx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 12; continue; }
            12 => { crate::simd_ops::wu64(rdi, rbx); rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); crate::simd_ops::wu32(rdi.wrapping_add(0xcu64), ((rbp as u32)) as u32); bb = 13; continue; }
            13 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141629200(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141524340; const P1: u64 = 0x141524330; const P2: u64 = 0x141627b60; const P3: u64 = 0x1415242e0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdx = (crate::simd_ops::ru32(rcx.wrapping_add(0xcu64))) as u32 as u64; rdi = rcx; r15 = ((rdx as u32)) as u32 as u64; rcx = 0x1; r15 = (((r15 as u32)).wrapping_shr(1u32)) as u32 as u64; r15 = (((r15 as u32)).wrapping_add((rdx as u32))) as u32 as u64; rsi = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; r15 = (if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { (rcx as u32) } else { (r15 as u32) }) as u32 as u64; rcx = (crate::simd_ops::ru64(rdi)) as u64; r15 = (((r15 as u32)).wrapping_add((rdx as u32))) as u32 as u64; rdx = ((r15.wrapping_add(r15.wrapping_mul(2))) as u64) as u64; rdx = ((rdx).wrapping_shl(3u32)) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { r8 = rdx; rdx = rcx; rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rbp = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 13; continue; } bb = 2; continue; }
            2 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            3 => { rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rbp = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rdi)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 5; continue; }
            5 => { fl_l = rax; fl_r = rbp; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 13; continue; } bb = 6; continue; }
            6 => { fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 7; continue; }
            7 => { crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rbx); rbx = rbp.wrapping_add(0x14); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), r12); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), r14); r14 = (0xffffffffffffffecu64) as u64; r14 = ((r14).wrapping_sub(rbp)) as u64; r12 = 0; fl_l = 0; fl_r = 0; bb = 8; continue; }
            8 => { rcx = rbx.wrapping_sub(0x14); fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { crate::simd_ops::wu32(rcx, ((r12 as u32)) as u32); crate::simd_ops::wu8(rbx.wrapping_sub(0xeu64), ((r12 as u8)) as u8); crate::simd_ops::wu64(rbx.wrapping_sub(0xcu64), r12); crate::simd_ops::wu64(rbx.wrapping_sub(0x4u64), r12); bb = 10; continue; }
            10 => { rdx = ((r14.wrapping_add(rbx)) as u64) as u64; rdx = ((rdx).wrapping_add(crate::simd_ops::ru64(rdi))) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rbx = ((rbx).wrapping_add(0x18u64)) as u64; fl_l = rsi; fl_r = (0x1u64) as u64; rsi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 8; continue; } bb = 11; continue; }
            11 => { r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; bb = 12; continue; }
            12 => { rdx = (crate::simd_ops::ru64(rdi)) as u64; rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 13; continue; }
            13 => { crate::simd_ops::wu64(rdi, rbp); rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; crate::simd_ops::wu32(rdi.wrapping_add(0xcu64), ((r15 as u32)) as u32); rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141629420(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141524340; const P1: u64 = 0x141524330; const P2: u64 = 0x141627c60; const P3: u64 = 0x1415242e0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdx = (crate::simd_ops::ru32(rcx.wrapping_add(0xcu64))) as u32 as u64; rdi = rcx; r15 = ((rdx as u32)) as u32 as u64; rcx = 0x1; r15 = (((r15 as u32)).wrapping_shr(1u32)) as u32 as u64; r15 = (((r15 as u32)).wrapping_add((rdx as u32))) as u32 as u64; rax = (crate::simd_ops::ru64(rdi)) as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; rbp = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; r15 = (if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { (rcx as u32) } else { (r15 as u32) }) as u32 as u64; rcx = 0; fl_l = 0; fl_r = 0; r15 = (((r15 as u32)).wrapping_add((rdx as u32))) as u32 as u64; rdx = ((r15 as u32)) as u32 as u64; rdx = ((rdx).wrapping_shl(5u32)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { r8 = rdx; rdx = rax; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rsi = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 13; continue; } bb = 2; continue; }
            2 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            3 => { rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rsi = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rdi)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 5; continue; }
            5 => { fl_l = rax; fl_r = rsi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 13; continue; } bb = 6; continue; }
            6 => { fl_l = rbp; fl_r = rbp; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 7; continue; }
            7 => { crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rbx); rbx = rsi.wrapping_add(0x1c); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), r12); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), r14); r14 = (0xffffffffffffffe4u64) as u64; r14 = ((r14).wrapping_sub(rsi)) as u64; r12 = 0; fl_l = 0; fl_r = 0; bb = 8; continue; }
            8 => { rcx = rbx.wrapping_sub(0x1c); fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { crate::simd_ops::wu64(rcx, r12); crate::simd_ops::wu8(rbx.wrapping_sub(0x12u64), ((r12 as u8)) as u8); crate::simd_ops::wu64(rbx.wrapping_sub(0xcu64), r12); crate::simd_ops::wu64(rbx.wrapping_sub(0x4u64), r12); bb = 10; continue; }
            10 => { rdx = ((r14.wrapping_add(rbx)) as u64) as u64; rdx = ((rdx).wrapping_add(crate::simd_ops::ru64(rdi))) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rbx = ((rbx).wrapping_add(0x20u64)) as u64; fl_l = rbp; fl_r = (0x1u64) as u64; rbp = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 8; continue; } bb = 11; continue; }
            11 => { r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; bb = 12; continue; }
            12 => { rdx = (crate::simd_ops::ru64(rdi)) as u64; rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 13; continue; }
            13 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); crate::simd_ops::wu64(rdi, rsi); crate::simd_ops::wu32(rdi.wrapping_add(0xcu64), ((r15 as u32)) as u32); rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141629530(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141524340; const P1: u64 = 0x141524330; const P2: u64 = 0x141627ce0; const P3: u64 = 0x141627de0; const P4: u64 = 0x1415242e0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x28u64)) as u64; rdx = (crate::simd_ops::ru32(rcx.wrapping_add(0xcu64))) as u32 as u64; rsi = rcx; r12 = ((rdx as u32)) as u32 as u64; rcx = 0x1; r12 = (((r12 as u32)).wrapping_shr(1u32)) as u32 as u64; r12 = (((r12 as u32)).wrapping_add((rdx as u32))) as u32 as u64; r14 = (crate::simd_ops::ru32(rsi.wrapping_add(0x8u64))) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; r12 = (if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { (rcx as u32) } else { (r12 as u32) }) as u32 as u64; rcx = (crate::simd_ops::ru64(rsi)) as u64; r12 = (((r12 as u32)).wrapping_add((rdx as u32))) as u32 as u64; rdx = ((r12.wrapping_add(r12.wrapping_mul(4))) as u64) as u64; rdx = ((rdx).wrapping_shl(3u32)) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { r8 = rdx; rdx = rcx; rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rbp = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 13; continue; } bb = 2; continue; }
            2 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x28u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            3 => { rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rbp = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rsi)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 5; continue; }
            5 => { fl_l = rax; fl_r = rbp; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 13; continue; } bb = 6; continue; }
            6 => { fl_l = r14; fl_r = r14; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 7; continue; }
            7 => { crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rdi); rdi = rbp.wrapping_add(0x14); crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), r13); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r15); r15 = (0xffffffffffffffecu64) as u64; r15 = ((r15).wrapping_sub(rbp)) as u64; r13 = 0; fl_l = 0; fl_r = 0; bb = 8; continue; }
            8 => { rcx = rdi.wrapping_sub(0x14); fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { crate::simd_ops::wu64(rcx, r13); crate::simd_ops::wu64(rdi.wrapping_sub(0xcu64), r13); crate::simd_ops::wu64(rdi.wrapping_sub(0x4u64), r13); crate::simd_ops::wu16(rdi.wrapping_add(0x4u64), ((r13 as u16)) as u16); crate::simd_ops::wu32(rdi.wrapping_add(0xcu64), ((r13 as u32)) as u32); bb = 10; continue; }
            10 => { rdx = (crate::simd_ops::ru64(rsi)) as u64; rbx = ((r15.wrapping_add(rdi)) as u64) as u64; rdx = ((rdx).wrapping_add(rbx)) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rsi)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = ((rcx).wrapping_add(rbx)) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rdi = ((rdi).wrapping_add(0x28u64)) as u64; fl_l = r14; fl_r = (0x1u64) as u64; r14 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 8; continue; } bb = 11; continue; }
            11 => { r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; bb = 12; continue; }
            12 => { rdx = (crate::simd_ops::ru64(rsi)) as u64; rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 13; continue; }
            13 => { crate::simd_ops::wu64(rsi, rbp); rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); crate::simd_ops::wu32(rsi.wrapping_add(0xcu64), ((r12 as u32)) as u32); rsp = ((rsp).wrapping_add(0x28u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141629660(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x18; const P2: u64 = 0x141529ed0; const P3: u64 = 0x20; const P4: u64 = 0x14162a1e0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x50u64)) as u64;
    fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; fl_r = (0x0u64) as u64;
    rsi = r8;
    rdi = rcx;
    if !((((fl_l as u64)) == ((fl_r as u64)))) { rax = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64; r8 = rsp.wrapping_add(0x40); xmm1 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x80u64))) as u32) as u128; rcx = ((rcx).wrapping_add(0x8u64)) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x40u64), ((rax as u32)) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x40u64))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(P1), ((rax as u32)) as u32); } else { xmm0 = 0u128; }
    rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P3))) as u64;
    r9 = rsp.wrapping_add(0x60);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu8(rsp.wrapping_add(0x30u64), (0x0u8) as u8);
    crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rax as u32)) as u32);
    xmm2 = (xmm0) as u128;
    rdx = rsi;
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax);
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64;
    rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64;
    rsp = ((rsp).wrapping_add(0x50u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416296e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x30; const P1: u64 = 0x10; const P2: u64 = 0x20; const P3: u64 = 0x141628950; const P4: u64 = 0x8; const P5: u64 = 0x60; const P6: u64 = 0x1415c4b70;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x50u64)) as u64;
    rax = (crate::simd_ops::ru64(rdx)) as u64;
    rdi = rcx;
    rcx = rdx;
    rbx = rdx;
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rcx = ((rdi.wrapping_add(P0)) as u64) as u64;
    rdx = rsp.wrapping_add(0x20);
    xmm0 = (crate::simd_ops::ru128(rax)) as u128;
    xmm1 = (crate::simd_ops::ru128(rax.wrapping_add(P1))) as u128;
    crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128);
    xmm0 = (crate::simd_ops::ru128(rax.wrapping_add(P2))) as u128;
    crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm1) as u128);
    crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm0) as u128);
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rdx = rbx;
    rcx = ((rax.wrapping_add(P4)) as u64) as u64;
    fl_l = rax; fl_r = rax;
    if !(((((fl_l as u64)) & ((fl_r as u64))) != 0)) { rcx = ((rdi.wrapping_add(P5)) as u64) as u64; }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64;
    rsp = ((rsp).wrapping_add(0x50u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141629740(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141629ba0; const P1: u64 = 0x141629530; const P2: u64 = 0x141628740; const P3: u64 = 0x14162b060; const P4: u64 = 0x14162a920; const P5: u64 = 0x14162b2e0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rbx); r12 = r9; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rbp); r9 = rsp.wrapping_add(0x50); rdi = rcx; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), r14); rsi = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r15); rcx = ((rcx).wrapping_add(0x18u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rsi); r15 = rdx; rbx = r8; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); r14 = rax; fl_l = (crate::simd_ops::ru8(rbx)) as u8 as u64; fl_r = ((rsi as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = ((crate::simd_ops::ru16(rax)) as u32) as u32 as u64; bb = 22; continue; }
            2 => { fl_l = r14; fl_r = r14; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 23; continue; } bb = 3; continue; }
            3 => { r8 = ((crate::simd_ops::ru16(rdi.wrapping_add(0x10u64))) as u32) as u32 as u64; rbp = 0xffff; fl_l = ((r8 as u16)) as u16 as u64; fl_r = ((rbp as u16)) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 20; continue; } bb = 4; continue; }
            4 => { rbx = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; fl_l = ((rbx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rdi.wrapping_add(0xcu64))) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 7; continue; } bb = 5; continue; }
            5 => { rcx = rdi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 9; continue; } bb = 6; continue; }
            6 => { fl_l = ((rbx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rdi.wrapping_add(0xcu64))) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 9; continue; } bb = 7; continue; }
            7 => { rdx = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; rax = (crate::simd_ops::ru64(rdi)) as u64; rcx = ((rdx.wrapping_add(rdx.wrapping_mul(4))) as u64) as u64; r8 = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; rax = ((rdx.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x8u64), ((rax as u32)) as u32); fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { crate::simd_ops::wu64(r8, rsi); crate::simd_ops::wu64(r8.wrapping_add(0x8u64), rsi); crate::simd_ops::wu64(r8.wrapping_add(0x10u64), rsi); crate::simd_ops::wu16(r8.wrapping_add(0x18u64), ((rsi as u16)) as u16); crate::simd_ops::wu32(r8.wrapping_add(0x20u64), ((rsi as u32)) as u32); r8 = ((crate::simd_ops::ru16(rdi.wrapping_add(0x8u64))) as u32) as u32 as u64; r8 = (r8 & !0xffffu64) | (((((r8 as u16)).wrapping_sub(1)) as u16 as u64)); fl_l = ((r8 as u16)) as u16 as u64; fl_r = ((rbp as u16)) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 21; continue; } bb = 9; continue; }
            9 => { rdx = (crate::simd_ops::ru64(r15)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 15; continue; } bb = 10; continue; }
            10 => { fl_l = (crate::simd_ops::ru32(r15.wrapping_add(0x8u64))) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 15; continue; } bb = 11; continue; }
            11 => { fl_l = (crate::simd_ops::ru32(r15.wrapping_add(0x10u64))) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 15; continue; } bb = 12; continue; }
            12 => { fl_l = (crate::simd_ops::ru8(r15.wrapping_add(0x18u64))) as u8 as u64; fl_r = (0xffu8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 15; continue; } bb = 13; continue; }
            13 => { fl_l = (crate::simd_ops::ru8(r15.wrapping_add(0x20u64))) as u8 as u64; fl_r = (0xffu8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 15; continue; } bb = 14; continue; }
            14 => { fl_l = (crate::simd_ops::ru64(r15.wrapping_add(0x28u64))) as u64; fl_r = rsi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 19; continue; } bb = 15; continue; }
            15 => { rcx = rdi.wrapping_add(0x20); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rbp = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 19; continue; } bb = 16; continue; }
            16 => { rdx = r15.wrapping_add(0x8); rcx = rax; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru8(rbp.wrapping_add(0xau64))) as u8 as u64; fl_r = ((rsi as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 23; continue; } bb = 17; continue; }
            17 => { fl_l = (crate::simd_ops::ru32(rbp.wrapping_add(0x18u64))) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 23; continue; } bb = 18; continue; }
            18 => { rcx = rbp; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru64(r15)) as u64; rcx = rdi.wrapping_add(0x20); rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); bb = 23; continue; }
            19 => { crate::simd_ops::wu8(rdi.wrapping_add(0x1cu64), ((rsi as u8)) as u8); bb = 23; continue; }
            20 => { rcx = (crate::simd_ops::ru64(rdi)) as u64; rdx = ((r8.wrapping_add(r8.wrapping_mul(4))) as u64) as u64; rax = ((crate::simd_ops::ru16(rcx.wrapping_add(rdx.wrapping_mul(8)).wrapping_add(0x20u64))) as u32) as u32 as u64; crate::simd_ops::wu16(rdi.wrapping_add(0x10u64), ((rax as u16)) as u16); crate::simd_ops::wu16(rcx.wrapping_add(rdx.wrapping_mul(8)).wrapping_add(0x20u64), ((rbp as u16)) as u16); bb = 21; continue; }
            21 => { crate::simd_ops::wu16(r14, ((r8 as u16)) as u16); rax = (((r8 as u16)) as u32) as u32 as u64; bb = 22; continue; }
            22 => { rcx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = (crate::simd_ops::ru64(rdi)) as u64; rsi = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; bb = 23; continue; }
            23 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 25; continue; } bb = 24; continue; }
            24 => { rcx = ((crate::simd_ops::ru16(rax)) as u32) as u32 as u64; rdx = ((rcx.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64; rcx = (crate::simd_ops::ru64(rdi)) as u64; rdx = ((rcx.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64; crate::simd_ops::wu64(r12, rdx); bb = 25; continue; }
            25 => { rax = rsi; rsp = ((rsp).wrapping_add(0x30u64)) as u64; r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416298e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141628020; const P1: u64 = 0x141629020; const P2: u64 = 0x14183acc8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x38u64)) as u64; fl_l = (crate::simd_ops::ru64(rdx)) as u64; fl_r = (0x0u64) as u64; r14 = r8; rdi = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 24; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), r12); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), r13); fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0x4u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = rcx.wrapping_add(0x2); crate::simd_ops::wu64(r9, rax); bb = 4; continue; }
            4 => { r12 = (crate::simd_ops::ru64(rdx)) as u64; rcx = ((rcx).wrapping_add(0x8u64)) as u64; rdx = r12; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); r13 = 0; fl_l = 0; fl_r = 0; rbx = rax; fl_l = (crate::simd_ops::ru8(r14)) as u8 as u64; fl_r = ((r13 as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 23; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x14u64))) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rbp); rbp = (crate::simd_ops::ru32(rdi.wrapping_add(0x10u64))) as u32 as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 6; continue; }
            6 => { rbx = ((rbx).wrapping_sub(crate::simd_ops::ru64(rdi.wrapping_add(0x8u64)))) as u64; rbx = ((((rbx) as i64).wrapping_shr(4u32) as u64)) as u64; fl_l = ((rbp as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 8; continue; } bb = 7; continue; }
            7 => { rcx = rdi.wrapping_add(0x8); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 21; continue; } bb = 8; continue; }
            8 => { fl_l = ((rbp as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rdi.wrapping_add(0x14u64))) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 21; continue; } bb = 9; continue; }
            9 => { r14 = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x10u64))) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r15); r15 = ((rbx as u32)) as u32 as u64; r15 = ((r15).wrapping_shl(4u32)) as u64; r14 = ((r14).wrapping_add(r15)) as u64; fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 11; continue; } bb = 10; continue; }
            10 => { rax = (((rax as u32)).wrapping_sub((rbx as u32))) as u32 as u64; rcx = r14.wrapping_add(0x10); rax = (((rax as u32)).wrapping_shl(4u32)) as u32 as u64; rdx = r14; r8 = ((rax as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 11; continue; }
            11 => { fl_l = r14; fl_r = r14; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 12; continue; }
            12 => { crate::simd_ops::wu64(r14, r13); bb = 13; continue; }
            13 => { crate::simd_ops::lk_add_u32(rdi.wrapping_add(0x10u64), (1) as u32); rbx = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; fl_l = ((rbx).wrapping_add(r15)) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; bb = 19; continue; }
            14 => { fl_l = ((rbp as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 16; continue; } bb = 15; continue; }
            15 => { rcx = rdi.wrapping_add(0x8); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 21; continue; } bb = 16; continue; }
            16 => { fl_l = ((rbp as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rdi.wrapping_add(0x14u64))) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 21; continue; } bb = 17; continue; }
            17 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x10u64))) as u32 as u64; rbx = ((rax as u32)) as u32 as u64; rbx = ((rbx).wrapping_shl(4u32)) as u64; rax = ((rax.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x10u64), ((rax as u32)) as u32); fl_l = ((rbx).wrapping_add(crate::simd_ops::ru64(rdi.wrapping_add(0x8u64)))) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; if (((fl_l as u64)) == 0) { bb = 19; continue; } bb = 18; continue; }
            18 => { crate::simd_ops::wu64(rbx, r13); bb = 19; continue; }
            19 => { fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 22; continue; } bb = 20; continue; }
            20 => { crate::simd_ops::wu64(rbx, r12); bb = 22; continue; }
            21 => { rbx = r13; bb = 22; continue; }
            22 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; bb = 23; continue; }
            23 => { r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = rbx.wrapping_add(0x8); rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; fl_l = rbx; fl_r = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rax = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { r13 } else { rax }) as u64; r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rsp = ((rsp).wrapping_add(0x38u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            24 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0x4u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; rax = rcx.wrapping_add(0x2); if (((fl_l as u8)) != ((fl_r as u8))) { bb = 26; continue; } bb = 25; continue; }
            25 => { rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu16(rax, ((rcx as u16)) as u16); bb = 26; continue; }
            26 => { rcx = ((crate::simd_ops::ru8(rax.wrapping_add(0x2u64))) as u32) as u32 as u64; crate::simd_ops::wu8(r8, ((rcx as u8)) as u8); crate::simd_ops::wu8(rax.wrapping_add(0x2u64), (0x1u8) as u8); rsp = ((rsp).wrapping_add(0x38u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141629a40(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141629f80; const P1: u64 = 0x141629e30; const P2: u64 = 0x141629ce0; const P3: u64 = 0x1416298e0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; fl_l = (crate::simd_ops::ru32(rdx)) as u32 as u64; fl_r = 0x0; rbx = rdx.wrapping_add(0x8); rsi = r9; r14 = r8; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 7; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru8(rbx)) as u8 as u64; fl_r = (0xffu8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 7; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0x8u64))) as u8 as u64; fl_r = (0xffu8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 7; continue; } bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 7; continue; } bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0x6u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; rax = rcx.wrapping_add(0x4); if (((fl_l as u8)) != ((fl_r as u8))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu16(rax, ((rcx as u16)) as u16); bb = 6; continue; }
            6 => { rcx = ((crate::simd_ops::ru8(rax.wrapping_add(0x2u64))) as u32) as u32 as u64; crate::simd_ops::wu8(r8, ((rcx as u8)) as u8); crate::simd_ops::wu8(rax.wrapping_add(0x2u64), (0x1u8) as u8); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            7 => { fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 8; continue; }
            8 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0x6u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 10; continue; } bb = 9; continue; }
            9 => { rax = rcx.wrapping_add(0x4); crate::simd_ops::wu64(r9, rax); bb = 10; continue; }
            10 => { rdx = (crate::simd_ops::ru32(rdx)) as u32 as u64; rcx = ((rcx).wrapping_add(0x8u64)) as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rcx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 12; continue; } bb = 11; continue; }
            11 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            12 => { fl_l = (crate::simd_ops::ru8(rbx)) as u8 as u64; fl_r = (0xffu8) as u8 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rdi); if (((fl_l as u8)) != ((fl_r as u8))) { bb = 16; continue; } bb = 13; continue; }
            13 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0x8u64))) as u8 as u64; fl_r = (0xffu8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 16; continue; } bb = 14; continue; }
            14 => { fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 16; continue; } bb = 15; continue; }
            15 => { fl_l = ((rcx).wrapping_add(0x4u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; bb = 23; continue; }
            16 => { fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 19; continue; } bb = 17; continue; }
            17 => { fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x6u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 19; continue; } bb = 18; continue; }
            18 => { fl_l = ((rax).wrapping_add(0x4u64)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; crate::simd_ops::wu64(rsi, rax); bb = 19; continue; }
            19 => { rdx = ((crate::simd_ops::ru8(rbx)) as u32) as u32 as u64; rcx = ((rcx).wrapping_add(0x8u64)) as u64; r8 = r14; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 30; continue; } bb = 20; continue; }
            20 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0x8u64))) as u8 as u64; fl_r = (0xffu8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 26; continue; } bb = 21; continue; }
            21 => { fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 26; continue; } bb = 22; continue; }
            22 => { fl_l = ((rcx).wrapping_add(0x2u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; bb = 23; continue; }
            23 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0x2u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 25; continue; } bb = 24; continue; }
            24 => { rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu16(rcx, ((rax as u16)) as u16); bb = 25; continue; }
            25 => { rax = ((crate::simd_ops::ru8(rcx.wrapping_add(0x2u64))) as u32) as u32 as u64; crate::simd_ops::wu8(r14, ((rax as u8)) as u8); crate::simd_ops::wu8(rcx.wrapping_add(0x2u64), (0x1u8) as u8); bb = 32; continue; }
            26 => { fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 29; continue; } bb = 27; continue; }
            27 => { fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x4u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 29; continue; } bb = 28; continue; }
            28 => { fl_l = ((rax).wrapping_add(0x2u64)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; crate::simd_ops::wu64(rsi, rax); bb = 29; continue; }
            29 => { rdx = ((crate::simd_ops::ru8(rbx.wrapping_add(0x8u64))) as u32) as u32 as u64; rcx = ((rcx).wrapping_add(0x8u64)) as u64; r8 = r14; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 31; continue; } bb = 30; continue; }
            30 => { rcx = 0; fl_l = 0; fl_r = 0; bb = 32; continue; }
            31 => { r9 = rsi; rdx = rbx.wrapping_add(0x10); r8 = r14; rcx = rax; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = rax; bb = 32; continue; }
            32 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = rcx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141629ba0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14162a0c0; const P1: u64 = 0x141629f80; const P2: u64 = 0x141629a40;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; fl_l = (crate::simd_ops::ru64(rdx)) as u64; fl_r = (0x0u64) as u64; rbx = rdx.wrapping_add(0x8); rsi = r9; r14 = r8; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 9; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru32(rbx)) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 9; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 9; continue; } bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0x10u64))) as u8 as u64; fl_r = (0xffu8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 9; continue; } bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0x18u64))) as u8 as u64; fl_r = (0xffu8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 9; continue; } bb = 5; continue; }
            5 => { fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(0x20u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 9; continue; } bb = 6; continue; }
            6 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0x4u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; rax = rcx.wrapping_add(0x2); if (((fl_l as u8)) != ((fl_r as u8))) { bb = 8; continue; } bb = 7; continue; }
            7 => { rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu16(rax, ((rcx as u16)) as u16); bb = 8; continue; }
            8 => { rcx = ((crate::simd_ops::ru8(rax.wrapping_add(0x2u64))) as u32) as u32 as u64; crate::simd_ops::wu8(r8, ((rcx as u8)) as u8); crate::simd_ops::wu8(rax.wrapping_add(0x2u64), (0x1u8) as u8); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            9 => { fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 10; continue; }
            10 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0x4u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 12; continue; } bb = 11; continue; }
            11 => { rax = rcx.wrapping_add(0x2); crate::simd_ops::wu64(r9, rax); bb = 12; continue; }
            12 => { rdx = (crate::simd_ops::ru64(rdx)) as u64; rcx = ((rcx).wrapping_add(0x8u64)) as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rcx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 14; continue; } bb = 13; continue; }
            13 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            14 => { fl_l = (crate::simd_ops::ru32(rbx)) as u32 as u64; fl_r = 0x0; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rdi); if (((fl_l as u32)) != ((fl_r as u32))) { bb = 22; continue; } bb = 15; continue; }
            15 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 22; continue; } bb = 16; continue; }
            16 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0x10u64))) as u8 as u64; fl_r = (0xffu8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 22; continue; } bb = 17; continue; }
            17 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0x18u64))) as u8 as u64; fl_r = (0xffu8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 22; continue; } bb = 18; continue; }
            18 => { fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(0x20u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 22; continue; } bb = 19; continue; }
            19 => { rcx = ((rcx).wrapping_add(0x8u64)) as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0x2u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 21; continue; } bb = 20; continue; }
            20 => { rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu16(rcx, ((rax as u16)) as u16); bb = 21; continue; }
            21 => { rax = ((crate::simd_ops::ru8(rcx.wrapping_add(0x2u64))) as u32) as u32 as u64; crate::simd_ops::wu8(r14, ((rax as u8)) as u8); crate::simd_ops::wu8(rcx.wrapping_add(0x2u64), (0x1u8) as u8); bb = 28; continue; }
            22 => { fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 25; continue; } bb = 23; continue; }
            23 => { fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0xau64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 25; continue; } bb = 24; continue; }
            24 => { fl_l = ((rax).wrapping_add(0x8u64)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; crate::simd_ops::wu64(rsi, rax); bb = 25; continue; }
            25 => { rdx = (crate::simd_ops::ru32(rbx)) as u32 as u64; rcx = ((rcx).wrapping_add(0x10u64)) as u64; r8 = r14; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 27; continue; } bb = 26; continue; }
            26 => { rcx = 0; fl_l = 0; fl_r = 0; bb = 28; continue; }
            27 => { r9 = rsi; rdx = rbx.wrapping_add(0x8); r8 = r14; rcx = rax; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = rax; bb = 28; continue; }
            28 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = rcx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141629f80(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141627f90; const P1: u64 = 0x141629200; const P2: u64 = 0x14183acc8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = r8; rbp = ((rdx as u32)) as u32 as u64; rdi = rcx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru8(rbx)) as u8 as u64; fl_r = (0x0u8) as u8 as u64; rcx = rax; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 20; continue; } bb = 1; continue; }
            1 => { rbx = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; r8 = (crate::simd_ops::ru32(rdi.wrapping_add(0xcu64))) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rsi); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 2; continue; }
            2 => { rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(rdi))) as u64; rax = (0x2aaaaaaaaaaaaaabu64) as u64; let prod = (rax as i64 as i128).wrapping_mul((rcx) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rsi = rdx; rsi = ((((rsi) as i64).wrapping_shr(2u32) as u64)) as u64; rax = rsi; rax = ((rax).wrapping_shr(63u32)) as u64; rsi = ((rsi).wrapping_add(rax)) as u64; fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rcx = rdi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 18; continue; } bb = 4; continue; }
            4 => { fl_l = ((rbx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rdi.wrapping_add(0xcu64))) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 18; continue; } bb = 5; continue; }
            5 => { rbx = (crate::simd_ops::ru64(rdi)) as u64; rax = ((rsi as u32)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), r14); rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; r14 = ((rcx.wrapping_mul(8)) as u64) as u64; rbx = ((rbx).wrapping_add(r14)) as u64; fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rax = (((rax as u32)).wrapping_sub((rsi as u32))) as u32 as u64; rcx = rbx.wrapping_add(0x18); rdx = rbx; r8 = ((rax.wrapping_add(rax.wrapping_mul(2))) as u32) as u32 as u64; r8 = (((r8 as u32)).wrapping_shl(3u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 7; continue; }
            7 => { fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rbx, ((rax as u32)) as u32); crate::simd_ops::wu8(rbx.wrapping_add(0x6u64), ((rax as u8)) as u8); crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), rax); crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), rax); bb = 9; continue; }
            9 => { crate::simd_ops::lk_add_u32(rdi.wrapping_add(0x8u64), (1) as u32); rcx = (crate::simd_ops::ru64(rdi)) as u64; fl_l = ((rcx).wrapping_add(r14)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; bb = 15; continue; }
            10 => { fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 12; continue; } bb = 11; continue; }
            11 => { rcx = rdi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 18; continue; } bb = 12; continue; }
            12 => { fl_l = ((rbx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rdi.wrapping_add(0xcu64))) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 18; continue; } bb = 13; continue; }
            13 => { rdx = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; rax = (crate::simd_ops::ru64(rdi)) as u64; rcx = ((rdx.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64; rcx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; rax = ((rdx.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x8u64), ((rax as u32)) as u32); fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 14; continue; }
            14 => { rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rcx, ((rax as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x6u64), ((rax as u8)) as u8); crate::simd_ops::wu64(rcx.wrapping_add(0x8u64), rax); crate::simd_ops::wu64(rcx.wrapping_add(0x10u64), rax); bb = 15; continue; }
            15 => { fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 16; continue; }
            16 => { crate::simd_ops::wu32(rcx, ((rbp as u32)) as u32); bb = 17; continue; }
            17 => { rax = rcx; bb = 19; continue; }
            18 => { rax = 0; fl_l = 0; fl_r = 0; bb = 19; continue; }
            19 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; bb = 20; continue; }
            20 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14162a0c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1416280b0; const P1: u64 = 0x141629420; const P2: u64 = 0x14183acc8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = r8; rbp = rdx; rdi = rcx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru8(rbx)) as u8 as u64; fl_r = (0x0u8) as u8 as u64; rsi = rax; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 18; continue; } bb = 1; continue; }
            1 => { rbx = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; rax = (crate::simd_ops::ru32(rdi.wrapping_add(0xcu64))) as u32 as u64; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 2; continue; }
            2 => { rsi = ((rsi).wrapping_sub(crate::simd_ops::ru64(rdi))) as u64; rsi = ((((rsi) as i64).wrapping_shr(5u32) as u64)) as u64; fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rcx = rdi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 17; continue; } bb = 4; continue; }
            4 => { fl_l = ((rbx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rdi.wrapping_add(0xcu64))) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 17; continue; } bb = 5; continue; }
            5 => { rbx = (crate::simd_ops::ru64(rdi)) as u64; rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), r14); r14 = ((rsi as u32)) as u32 as u64; r14 = ((r14).wrapping_shl(5u32)) as u64; rbx = ((rbx).wrapping_add(r14)) as u64; fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rax = (((rax as u32)).wrapping_sub((rsi as u32))) as u32 as u64; rcx = rbx.wrapping_add(0x20); rax = (((rax as u32)).wrapping_shl(5u32)) as u32 as u64; rdx = rbx; r8 = ((rax as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 7; continue; }
            7 => { fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rbx, rax); crate::simd_ops::wu8(rbx.wrapping_add(0xau64), ((rax as u8)) as u8); crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), rax); crate::simd_ops::wu64(rbx.wrapping_add(0x18u64), rax); bb = 9; continue; }
            9 => { crate::simd_ops::lk_add_u32(rdi.wrapping_add(0x8u64), (1) as u32); rcx = (crate::simd_ops::ru64(rdi)) as u64; fl_l = ((rcx).wrapping_add(r14)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; bb = 15; continue; }
            10 => { fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 12; continue; } bb = 11; continue; }
            11 => { rcx = rdi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 17; continue; } bb = 12; continue; }
            12 => { fl_l = ((rbx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rdi.wrapping_add(0xcu64))) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 17; continue; } bb = 13; continue; }
            13 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; rcx = ((rax as u32)) as u32 as u64; rcx = ((rcx).wrapping_shl(5u32)) as u64; rax = ((rax.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x8u64), ((rax as u32)) as u32); fl_l = ((rcx).wrapping_add(crate::simd_ops::ru64(rdi))) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; if (((fl_l as u64)) == 0) { bb = 15; continue; } bb = 14; continue; }
            14 => { rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rcx, rax); crate::simd_ops::wu8(rcx.wrapping_add(0xau64), ((rax as u8)) as u8); crate::simd_ops::wu64(rcx.wrapping_add(0x10u64), rax); crate::simd_ops::wu64(rcx.wrapping_add(0x18u64), rax); bb = 15; continue; }
            15 => { rax = rcx; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 18; continue; } bb = 16; continue; }
            16 => { crate::simd_ops::wu64(rcx, rbp); bb = 18; continue; }
            17 => { rax = 0; fl_l = 0; fl_r = 0; bb = 18; continue; }
            18 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14162a1e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141629740; const P1: u64 = 0x141614f90; const P2: u64 = 0x14162a420; const P3: u64 = 0x141625ba0; const P4: u64 = 0x1416287e0; const P5: u64 = 0x14162b330; const P6: u64 = 0x14160bf70; const P7: u64 = 0x141628d50; const P8: u64 = 0x141626080; const P9: u64 = 0x14162a6a0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 25>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu32(rsp.wrapping_add(0x18u64), ((xmm2 as u32)) as u32); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rbp = rsp.wrapping_sub(0xf); rsp = ((rsp).wrapping_sub(0x90u64)) as u64; r12 = r9; rdi = rcx; rbx = 0; fl_l = 0; fl_r = 0; r9 = rbp.wrapping_add(0x4f); r8 = r12; crate::simd_ops::wu64(rbp.wrapping_add(0x4fu64), rbx); rcx = ((rcx).wrapping_add(0x30u64)) as u64; rsi = rdx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r15 = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 23; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0xd8u64), r14); r14 = (crate::simd_ops::ru64(rbp.wrapping_add(0x4fu64))) as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x80u64), (xmm6) as u128); fl_l = (crate::simd_ops::ru8(r12)) as u8 as u64; fl_r = ((rbx as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 5; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu32(rax.wrapping_add(0x20u64), ((rbx as u32)) as u32); rcx = r14.wrapping_add(0x8); fl_l = r14; fl_r = r14; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rcx = rdi.wrapping_add(0x60); bb = 4; continue; }
            4 => { xmm0 = (crate::simd_ops::ru128(rsi)) as u128; rdx = rax.wrapping_add(0x8); xmm1 = (crate::simd_ops::ru128(rsi.wrapping_add(0x10u64))) as u128; r8 = rbp.wrapping_sub(0x31); crate::simd_ops::wu128(rbp.wrapping_sub(0x31u64), (xmm0) as u128); xmm0 = (crate::simd_ops::ru128(rsi.wrapping_add(0x20u64))) as u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x21u64), (xmm1) as u128); crate::simd_ops::wu128(rbp.wrapping_sub(0x11u64), (xmm0) as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 5; continue; }
            5 => { rcx = ((crate::simd_ops::ru8(r12)) as u32) as u32 as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { xmm6 = ((crate::simd_ops::ru32(r15.wrapping_add(0x20u64))) as u32) as u128; bb = 10; continue; }
            7 => { fl_l = r14; fl_r = r14; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { xmm6 = ((crate::simd_ops::ru32(r14.wrapping_add(0x20u64))) as u32) as u128; bb = 10; continue; }
            9 => { xmm6 = ((crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32) as u128; bb = 10; continue; }
            10 => { rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x77u64))) as u32 as u64; r9 = rbp.wrapping_sub(0x31); crate::simd_ops::wu32(rbp.wrapping_sub(0x19u64), ((rax as u32)) as u32); r8 = rbp.wrapping_add(0x5f); rax = rbp.wrapping_add(0x4f); crate::simd_ops::wu8(rbp.wrapping_add(0x4fu64), ((rbx as u8)) as u8); rbx = (crate::simd_ops::ru64(rbp.wrapping_add(0x6fu64))) as u64; xmm1 = (xmm6) as u128; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); rax = rbp.wrapping_add(0x7f); crate::simd_ops::wu8(rbp.wrapping_sub(0x15u64), ((rcx as u8)) as u8); rcx = r15; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); crate::simd_ops::wu64(rbp.wrapping_sub(0x31u64), rdi); crate::simd_ops::wu64(rbp.wrapping_sub(0x29u64), rsi); crate::simd_ops::wu64(rbp.wrapping_sub(0x21u64), rbx); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru8(rbp.wrapping_add(0x4fu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 12; continue; } bb = 11; continue; }
            11 => { xmm1 = ((crate::simd_ops::ru32(rbp.wrapping_add(0x5fu64))) as u32) as u128; rdx = rbp.wrapping_sub(0x31); crate::simd_ops::lk_add_u8(rdi.wrapping_add(0x89u64), (1) as u8); rcx = rdi.wrapping_add(0x78); crate::simd_ops::lk_add_u8(rdi.wrapping_add(0x88u64), (0x4u8) as u8); crate::simd_ops::wu64(rbp.wrapping_sub(0x51u64), rdi); xmm0 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x51u64))) as u128; crate::simd_ops::wu64(rbp.wrapping_sub(0x41u64), rsi); xmm0 = crate::simd_ops::vshufps(xmm0, xmm0, 0xd2u8); xmm0 = (xmm0 & !0xffff_ffffu128) | (xmm6 & 0xffff_ffffu128); xmm0 = crate::simd_ops::vshufps(xmm0, xmm0, 0x27u8); xmm0 = (xmm0 & !0xffff_ffffu128) | (xmm1 & 0xffff_ffffu128); xmm0 = crate::simd_ops::vshufps(xmm0, xmm0, 0x39u8); crate::simd_ops::wu128(rbp.wrapping_sub(0x31u64), (xmm0) as u128); xmm0 = ((crate::simd_ops::ru64(rbp.wrapping_sub(0x41u64))) as u64) as u128; crate::simd_ops::wu64(rbp.wrapping_sub(0x21u64), ((xmm0 as u64)) as u64); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = ((crate::simd_ops::ru8(rdi.wrapping_add(0x88u64))) as u32) as u32 as u64; rcx = (((rax as u8)) as u32) as u32 as u64; rax = (rax & !0xffu64) | ((((((rax as u8)) & (0x3u8))) as u8 as u64)); rcx = (rcx & !0xffu64) | (((((rcx as u8)).wrapping_shr(2u32)) as u8 as u64)); rcx = (rcx & !0xffu64) | (((((rcx as u8)).wrapping_sub(1)) as u8 as u64)); rcx = (rcx & !0xffu64) | (((((rcx as u8)).wrapping_shl(2u32)) as u8 as u64)); fl_l = ((((rcx as u8)) | ((rax as u8)))) as u8 as u64; fl_r = fl_l; rcx = (rcx & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); crate::simd_ops::wu8(rdi.wrapping_add(0x88u64), ((rcx as u8)) as u8); bb = 12; continue; }
            12 => { fl_l = (crate::simd_ops::ru8(rbp.wrapping_add(0x7fu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x80u64))) as u128; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 15; continue; } bb = 13; continue; }
            13 => { r9 = r14; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbx); r8 = r15; rdx = rsi; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 15; continue; } bb = 14; continue; }
            14 => { rdx = rsi; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 15; continue; }
            15 => { fl_l = (crate::simd_ops::ru8(r12)) as u8 as u64; fl_r = (0x0u8) as u8 as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0xd8u64))) as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 25; continue; } bb = 16; continue; }
            16 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0xcu64))) as u64; rdx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 25; continue; } bb = 17; continue; }
            17 => { rbx = 0; fl_l = 0; fl_r = 0; r8 = rbp.wrapping_add(0x4f); rdx = rsi; crate::simd_ops::wu64(rbp.wrapping_add(0x4fu64), rbx); rcx = rdi.wrapping_add(0x30); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 23; continue; } bb = 18; continue; }
            18 => { r9 = (crate::simd_ops::ru64(rbp.wrapping_add(0x4fu64))) as u64; r8 = rax; rdx = rsi; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbx); rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 23; continue; } bb = 19; continue; }
            19 => { rcx = rdi.wrapping_add(0x48); r8 = rsi; rdx = rdi.wrapping_add(0x30); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 23; continue; } bb = 20; continue; }
            20 => { fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(0x4cu64))) as u8 as u64; fl_r = ((rbx as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 23; continue; } bb = 21; continue; }
            21 => { fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x58u64))) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 23; continue; } bb = 22; continue; }
            22 => { rcx = rdi.wrapping_add(0x30); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 23; continue; }
            23 => { rax = 0x34; bb = 24; continue; }
            24 => { rsp = ((rsp).wrapping_add(0x90u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            25 => { rax = 0x1; bb = 24; continue; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14162a420(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141524330; const P1: u64 = 0x142690878; const P2: u64 = 0x1416283d0; const P3: u64 = 0x14162a660; const P4: u64 = 0x141623d40;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 21>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x80u64)) as u64; rax = (crate::simd_ops::ru32(r9.wrapping_add(0x18u64))) as u32 as u64; rsi = r9; r15 = (crate::simd_ops::ru64(r9.wrapping_add(0x10u64))) as u64; rbp = r8; rdx = (crate::simd_ops::ru64(r9)) as u64; rdi = rcx; r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0xd0u64))) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0xc8u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(r9.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x70u64), (xmm6) as u128); xmm6 = (xmm1) as u128; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rdx); fl_l = r15; fl_r = r15; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x14u64))) as u32 as u64; r14 = rdx.wrapping_add(0x14); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; r12 = (r12 & !0xffu64) | (((if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { 1u8 } else { 0u8 }) as u8 as u64)); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 11; continue; } bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru64(rcx)) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 24; continue; } bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0x29u64))) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 24; continue; } bb = 4; continue; }
            4 => { bb = 22; continue; }
            5 => { fl_l = (crate::simd_ops::ru8(r15.wrapping_add(0x8u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 9; continue; } bb = 6; continue; }
            6 => { fl_l = (crate::simd_ops::ru8(r9.wrapping_add(0x1cu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 8; continue; } bb = 7; continue; }
            7 => { fl_l = (crate::simd_ops::ru8(r15.wrapping_add(0x9u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 9; continue; } bb = 8; continue; }
            8 => { r12 = (r12 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 10; continue; }
            9 => { r12 = (r12 & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 10; continue; }
            10 => { r14 = rdx.wrapping_add(0x14); bb = 11; continue; }
            11 => { rdx = (crate::simd_ops::ru64(rcx)) as u64; rbx = 0; fl_l = 0; fl_r = 0; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 16; continue; } bb = 12; continue; }
            12 => { rdx = ((rbx.wrapping_add(0x30u64)) as u32) as u32 as u64; rcx = ((rbx.wrapping_add(0x10u64)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 13; continue; }
            13 => { rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; rax = P1; crate::simd_ops::wu64(rdx, rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; crate::simd_ops::wu64(rdx.wrapping_add(0x18u64), rbx); crate::simd_ops::wu32(rdx.wrapping_add(0x20u64), ((rbx as u32)) as u32); crate::simd_ops::wu64(rdx.wrapping_add(0x8u64), rax); crate::simd_ops::wu64(rdx.wrapping_add(0x10u64), rbx); crate::simd_ops::wu16(rdx.wrapping_add(0x28u64), ((rbx as u16)) as u16); rax = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::wu64(rdx.wrapping_add(0x18u64), rax); rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x8u64))) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_add(0x20u64), ((rax as u32)) as u32); bb = 15; continue; }
            14 => { rdx = rbx; bb = 15; continue; }
            15 => { crate::simd_ops::wu64(rdi, rdx); fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 23; continue; } bb = 16; continue; }
            16 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0x29u64))) as u8 as u64; fl_r = ((rbx as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 23; continue; } bb = 17; continue; }
            17 => { fl_l = ((r12 as u8)) as u8 as u64; fl_r = ((r12 as u8)) as u8 as u64; r14 = (if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { rbx } else { r14 }) as u64; fl_l = r15; fl_r = r15; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 19; continue; } bb = 18; continue; }
            18 => { rbx = (crate::simd_ops::ru64(r15)) as u64; bb = 19; continue; }
            19 => { rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x18u64))) as u32 as u64; r9 = rbx; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), r13); r8 = rbp; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rax as u32)) as u32); xmm1 = (xmm6) as u128; rcx = rdx; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r14); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 21; continue; } bb = 20; continue; }
            20 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xd8u64))) as u64; crate::simd_ops::wu8(rax, (0x0u8) as u8); crate::simd_ops::wu32(rbp, ((xmm6 as u32)) as u32); crate::simd_ops::wu32(rdi.wrapping_add(0x20u64), ((xmm6 as u32)) as u32); bb = 34; continue; }
            21 => { rcx = (crate::simd_ops::ru64(rdi)) as u64; bb = 22; continue; }
            22 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 23; continue; }
            23 => { rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; bb = 24; continue; }
            24 => { fl_l = (crate::simd_ops::ru32(rsp.wrapping_add(0xc8u64))) as u32 as u64; fl_r = 0x2; xmm0 = ((crate::simd_ops::ru32(rbp)) as u32) as u128; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 26; continue; } bb = 25; continue; }
            25 => { xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(xmm6 as u32)).to_bits() as u128); bb = 26; continue; }
            26 => { fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = ((xmm6 as u32)) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x20u64), ((xmm0 as u32)) as u32); if (f32::from_bits((fl_l as u32)) == f32::from_bits((fl_r as u32)) || f32::from_bits((fl_l as u32)).is_nan() || f32::from_bits((fl_r as u32)).is_nan()) { bb = 28; continue; } bb = 27; continue; }
            27 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xd8u64))) as u64; rcx = rdi.wrapping_add(0x8); xmm1 = (xmm6) as u128; crate::simd_ops::wu8(rax, (0x1u8) as u8); rax = (crate::simd_ops::ru64(rdx.wrapping_add(0xcu64))) as u64; rdx = rsp.wrapping_add(0x50); xmm2 = ((crate::simd_ops::ru32(rdi.wrapping_add(0x20u64))) as u32) as u128; crate::simd_ops::lk_add_u8(rdi.wrapping_add(0x19u64), (1) as u8); crate::simd_ops::lk_add_u8(rdi.wrapping_add(0x18u64), (0x4u8) as u8); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rax); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x50u64))) as u128; xmm1 = crate::simd_ops::vunpcklps(xmm1, xmm2); crate::simd_ops::wu128(rsp.wrapping_add(0x50u64), (xmm0) as u128); crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), ((xmm1 as u64)) as u64); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = ((crate::simd_ops::ru8(rdi.wrapping_add(0x18u64))) as u32) as u32 as u64; rcx = (((rax as u8)) as u32) as u32 as u64; rax = (rax & !0xffu64) | ((((((rax as u8)) & (0x3u8))) as u8 as u64)); rcx = (rcx & !0xffu64) | (((((rcx as u8)).wrapping_shr(2u32)) as u8 as u64)); rcx = (rcx & !0xffu64) | (((((rcx as u8)).wrapping_sub(1)) as u8 as u64)); rcx = (rcx & !0xffu64) | (((((rcx as u8)).wrapping_shl(2u32)) as u8 as u64)); fl_l = ((((rcx as u8)) | ((rax as u8)))) as u8 as u64; fl_r = fl_l; rcx = (rcx & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); crate::simd_ops::wu8(rdi.wrapping_add(0x18u64), ((rcx as u8)) as u8); bb = 28; continue; }
            28 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x20u64))) as u32 as u64; crate::simd_ops::wu32(rbp, ((rax as u32)) as u32); rcx = ((crate::simd_ops::ru8(r13)) as u32) as u32 as u64; rax = ((crate::simd_ops::ru8(rdi.wrapping_add(0x18u64))) as u32) as u32 as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 31; continue; } bb = 29; continue; }
            29 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = (0xfcu8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 33; continue; } bb = 30; continue; }
            30 => { rax = (rax & !0xffu64) | ((((((rax as u8)) | (0x2u8))) as u8 as u64)); crate::simd_ops::wu8(rdi.wrapping_add(0x18u64), ((rax as u8)) as u8); rcx = (rcx & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 33; continue; }
            31 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x2u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 33; continue; } bb = 32; continue; }
            32 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = (0xfcu8) as u8 as u64; rcx = (rcx & !0xffu64) | (((if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { 1u8 } else { 0u8 }) as u8 as u64)); bb = 33; continue; }
            33 => { crate::simd_ops::wu8(r13, ((rcx as u8)) as u8); bb = 34; continue; }
            34 => { xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x70u64))) as u128; r11 = rsp.wrapping_add(0x80); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(r11.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x40u64))) as u64; rsp = r11; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14162a660(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x143930f80; const P2: u64 = 0x1415adef0; const P3: u64 = 0x28;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rbx = rcx;
    fl_l = rdx; fl_r = rdx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { r8 = rcx; rcx = (crate::simd_ops::ru64(P1)) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(P0), (0x0u64) as u64); crate::simd_ops::wu8(rbx.wrapping_add(P3), (0x0u8) as u8); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    crate::simd_ops::wu8(rcx.wrapping_add(P3), (0x0u8) as u8);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14162a6a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x28; const P1: u64 = 0x20; const P2: u64 = 0x14162a920; const P3: u64 = 0x1415242e0; const P4: u64 = 0x2c; const P5: u64 = 0x1c; const P6: u64 = 0x14162acc0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; rbx = rcx; fl_l = ((rdi as u32)) as u32 as u64; fl_r = 0x1; rdi = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((((fl_l as u32)).wrapping_sub((fl_r as u32))) as i32) < 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { bb = 2; continue; }
            2 => { rcx = ((rdi as u32)) as u32 as u64; rcx = ((rcx).wrapping_shl(5u32)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rbx.wrapping_add(P1)))) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rdi as u32)) as u32 as u64; fl_r = 0x1; rdi = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((((fl_l as u32)).wrapping_sub((fl_r as u32))) as i32) >= 0) { bb = 2; continue; } bb = 3; continue; }
            3 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rdi = 0; fl_l = 0; fl_r = 0; rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rbx.wrapping_add(P0), ((rdi as u32)) as u32); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(P1), rdi); crate::simd_ops::wu32(rbx.wrapping_add(P4), ((rdi as u32)) as u32); bb = 5; continue; }
            5 => { rcx = rbx; crate::simd_ops::wu8(rbx.wrapping_add(P5), (0x0u8) as u8); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P6, rax, rcx, rdx, r8, r9, rsp); }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14162a710(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x4; const P1: u64 = 0x8; const P2: u64 = 0x10; const P3: u64 = 0x1415242e0; const P4: u64 = 0x14;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    crate::simd_ops::wu8(rcx.wrapping_add(P0), (0x0u8) as u8);
    rbx = rcx;
    rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64;
    fl_l = rdx; fl_r = rdx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { crate::simd_ops::wu32(rcx.wrapping_add(P2), (0x0u32) as u32); rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(P1), (0x0u64) as u64); crate::simd_ops::wu32(rbx.wrapping_add(P4), (0x0u32) as u32); }
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14162a750(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415242e0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = ((rsp).wrapping_sub(0x38u64)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x10u64))) as u32 as u64; r12 = 0; fl_l = 0; fl_r = 0; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), r14); r14 = ((((rax as u32)) as i32 as i64) as u64) as u64; r13 = rcx; if (((((fl_l as u32)).wrapping_sub((fl_r as u32))) as i32) < 0) { bb = 11; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r15); r15 = ((r14.wrapping_add(r14.wrapping_mul(2))) as u64) as u64; fl_l = ((r15).wrapping_shl(3u32)) as u64; r15 = ((fl_l as u64)) as u64; bb = 2; continue; }
            2 => { rbp = (crate::simd_ops::ru64(r13.wrapping_add(0x8u64))) as u64; rbp = ((rbp).wrapping_add(r15)) as u64; rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x10u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; rbx = ((((rax as u32)) as i32 as i64) as u64) as u64; if (((((fl_l as u32)).wrapping_sub((fl_r as u32))) as i32) < 0) { bb = 7; continue; } bb = 3; continue; }
            3 => { rsi = ((rbx.wrapping_add(rbx.wrapping_mul(2))) as u64) as u64; fl_l = ((rsi).wrapping_shl(3u32)) as u64; rsi = ((fl_l as u64)) as u64; bb = 4; continue; }
            4 => { rdi = (crate::simd_ops::ru64(rbp.wrapping_add(0x8u64))) as u64; rdi = ((rdi).wrapping_add(rsi)) as u64; crate::simd_ops::wu8(rdi.wrapping_add(0x4u64), ((r12 as u8)) as u8); rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rdi.wrapping_add(0x10u64), ((r12 as u32)) as u32); rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rdi.wrapping_add(0x8u64), r12); crate::simd_ops::wu32(rdi.wrapping_add(0x14u64), ((r12 as u32)) as u32); bb = 6; continue; }
            6 => { rsi = ((rsi).wrapping_sub(0x18u64)) as u64; fl_l = rbx; fl_r = (0x1u64) as u64; rbx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((((fl_l as u64)).wrapping_sub((fl_r as u64))) as i64) >= 0) { bb = 4; continue; } bb = 7; continue; }
            7 => { rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x8u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rbp.wrapping_add(0x10u64), ((r12 as u32)) as u32); rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbp.wrapping_add(0x8u64), r12); crate::simd_ops::wu32(rbp.wrapping_add(0x14u64), ((r12 as u32)) as u32); bb = 9; continue; }
            9 => { r15 = ((r15).wrapping_sub(0x18u64)) as u64; crate::simd_ops::wu8(rbp.wrapping_add(0x4u64), ((r12 as u8)) as u8); fl_l = r14; fl_r = (0x1u64) as u64; r14 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((((fl_l as u64)).wrapping_sub((fl_r as u64))) as i64) >= 0) { bb = 2; continue; } bb = 10; continue; }
            10 => { r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; bb = 11; continue; }
            11 => { rdx = (crate::simd_ops::ru64(r13.wrapping_add(0x8u64))) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 12; continue; }
            12 => { rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(r13.wrapping_add(0x10u64), ((r12 as u32)) as u32); rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(r13.wrapping_add(0x8u64), r12); crate::simd_ops::wu32(r13.wrapping_add(0x14u64), ((r12 as u32)) as u32); bb = 13; continue; }
            13 => { crate::simd_ops::wu8(r13.wrapping_add(0x6u64), ((r12 as u8)) as u8); rsp = ((rsp).wrapping_add(0x38u64)) as u64; r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14162a870(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x4; const P2: u64 = 0x8; const P3: u64 = 0x1415242e0; const P4: u64 = 0x14;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; r14 = 0; fl_l = 0; fl_r = 0; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx); rbx = ((((rax as u32)) as i32 as i64) as u64) as u64; rbp = rcx; if (((((fl_l as u32)).wrapping_sub((fl_r as u32))) as i32) < 0) { bb = 6; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rsi); rsi = ((rbx.wrapping_add(rbx.wrapping_mul(2))) as u64) as u64; fl_l = ((rsi).wrapping_shl(3u32)) as u64; rsi = ((fl_l as u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rdi); bb = 2; continue; }
            2 => { rdi = (crate::simd_ops::ru64(rbp.wrapping_add(0x8u64))) as u64; rdi = ((rdi).wrapping_add(rsi)) as u64; crate::simd_ops::wu8(rdi.wrapping_add(P1), ((r14 as u8)) as u8); rdx = (crate::simd_ops::ru64(rdi.wrapping_add(P2))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rdi.wrapping_add(P0), ((r14 as u32)) as u32); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rdi.wrapping_add(P2), r14); crate::simd_ops::wu32(rdi.wrapping_add(P4), ((r14 as u32)) as u32); bb = 4; continue; }
            4 => { rsi = ((rsi).wrapping_sub(0x18u64)) as u64; fl_l = rbx; fl_r = (0x1u64) as u64; rbx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((((fl_l as u64)).wrapping_sub((fl_r as u64))) as i64) >= 0) { bb = 2; continue; } bb = 5; continue; }
            5 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; bb = 6; continue; }
            6 => { rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x8u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rbp.wrapping_add(0x10u64), ((r14 as u32)) as u32); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbp.wrapping_add(0x8u64), r14); crate::simd_ops::wu32(rbp.wrapping_add(0x14u64), ((r14 as u32)) as u32); bb = 8; continue; }
            8 => { crate::simd_ops::wu8(rbp.wrapping_add(0x4u64), ((r14 as u8)) as u8); rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14162a920(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415242e0;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 15>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r11 = rsp; crate::simd_ops::wu64(r11.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = ((rsp).wrapping_sub(0x70u64)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x18u64))) as u32 as u64; rbp = rcx; rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(r11.wrapping_sub(0x10u64), rbx); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; rbx = ((((rax as u32)) as i32 as i64) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x90u64), rbx); if (((((fl_l as u32)).wrapping_sub((fl_r as u32))) as i32) < 0) { bb = 23; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(r11.wrapping_sub(0x18u64), rsi); rsi = ((rbx.wrapping_add(rbx.wrapping_mul(2))) as u64) as u64; crate::simd_ops::wu64(r11.wrapping_sub(0x20u64), rdi); crate::simd_ops::wu64(r11.wrapping_sub(0x28u64), r12); crate::simd_ops::wu64(r11.wrapping_sub(0x30u64), r13); fl_l = ((rsi).wrapping_shl(3u32)) as u64; rsi = ((fl_l as u64)) as u64; crate::simd_ops::wu64(r11.wrapping_sub(0x38u64), r14); crate::simd_ops::wu64(r11.wrapping_sub(0x40u64), r15); crate::simd_ops::wu64(rsp.wrapping_add(0x98u64), rsi); bb = 2; continue; }
            2 => { rdi = (crate::simd_ops::ru64(rbp.wrapping_add(0x10u64))) as u64; rdi = ((rdi).wrapping_add(rsi)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x10u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; r12 = ((((rax as u32)) as i32 as i64) as u64) as u64; if (((((fl_l as u32)).wrapping_sub((fl_r as u32))) as i32) < 0) { bb = 19; continue; } bb = 3; continue; }
            3 => { rbx = ((r12.wrapping_add(r12.wrapping_mul(2))) as u64) as u64; fl_l = ((rbx).wrapping_shl(3u32)) as u64; rbx = ((fl_l as u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x88u64), rbx); bb = 4; continue; }
            4 => { r13 = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; r13 = ((r13).wrapping_add(rbx)) as u64; rax = (crate::simd_ops::ru32(r13.wrapping_add(0x10u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; r14 = ((((rax as u32)) as i32 as i64) as u64) as u64; if (((((fl_l as u32)).wrapping_sub((fl_r as u32))) as i32) < 0) { bb = 15; continue; } bb = 5; continue; }
            5 => { r15 = ((r14.wrapping_add(r14.wrapping_mul(2))) as u64) as u64; fl_l = ((r15).wrapping_shl(3u32)) as u64; r15 = ((fl_l as u64)) as u64; bb = 6; continue; }
            6 => { rbp = (crate::simd_ops::ru64(r13.wrapping_add(0x8u64))) as u64; rbp = ((rbp).wrapping_add(r15)) as u64; rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x10u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; rbx = ((((rax as u32)) as i32 as i64) as u64) as u64; if (((((fl_l as u32)).wrapping_sub((fl_r as u32))) as i32) < 0) { bb = 11; continue; } bb = 7; continue; }
            7 => { rsi = ((rbx.wrapping_add(rbx.wrapping_mul(2))) as u64) as u64; fl_l = ((rsi).wrapping_shl(3u32)) as u64; rsi = ((fl_l as u64)) as u64; bb = 8; continue; }
            8 => { rdi = (crate::simd_ops::ru64(rbp.wrapping_add(0x8u64))) as u64; rdi = ((rdi).wrapping_add(rsi)) as u64; crate::simd_ops::wu8(rdi.wrapping_add(0x4u64), (0x0u8) as u8); rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { crate::simd_ops::wu32(rdi.wrapping_add(0x10u64), ((rcx as u32)) as u32); rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rdi.wrapping_add(0x8u64), rcx); crate::simd_ops::wu32(rdi.wrapping_add(0x14u64), ((rcx as u32)) as u32); bb = 10; continue; }
            10 => { rsi = ((rsi).wrapping_sub(0x18u64)) as u64; fl_l = rbx; fl_r = (0x1u64) as u64; rbx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((((fl_l as u64)).wrapping_sub((fl_r as u64))) as i64) >= 0) { bb = 8; continue; } bb = 11; continue; }
            11 => { rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x8u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 12; continue; }
            12 => { crate::simd_ops::wu32(rbp.wrapping_add(0x10u64), ((rcx as u32)) as u32); rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rbp.wrapping_add(0x8u64), rcx); crate::simd_ops::wu32(rbp.wrapping_add(0x14u64), ((rcx as u32)) as u32); bb = 13; continue; }
            13 => { r15 = ((r15).wrapping_sub(0x18u64)) as u64; crate::simd_ops::wu8(rbp.wrapping_add(0x4u64), (0x0u8) as u8); fl_l = r14; fl_r = (0x1u64) as u64; r14 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((((fl_l as u64)).wrapping_sub((fl_r as u64))) as i64) >= 0) { bb = 6; continue; } bb = 14; continue; }
            14 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x88u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; bb = 15; continue; }
            15 => { rdx = (crate::simd_ops::ru64(r13.wrapping_add(0x8u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 16; continue; }
            16 => { crate::simd_ops::wu32(r13.wrapping_add(0x10u64), ((rcx as u32)) as u32); rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(r13.wrapping_add(0x8u64), rcx); crate::simd_ops::wu32(r13.wrapping_add(0x14u64), ((rcx as u32)) as u32); bb = 17; continue; }
            17 => { rbx = ((rbx).wrapping_sub(0x18u64)) as u64; crate::simd_ops::wu8(r13.wrapping_add(0x6u64), (0x0u8) as u8); fl_l = r12; fl_r = (0x1u64) as u64; r12 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x88u64), rbx); if (((((fl_l as u64)).wrapping_sub((fl_r as u64))) as i64) >= 0) { bb = 4; continue; } bb = 18; continue; }
            18 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x98u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; bb = 19; continue; }
            19 => { rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 21; continue; } bb = 20; continue; }
            20 => { crate::simd_ops::wu32(rdi.wrapping_add(0x10u64), ((rcx as u32)) as u32); rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rdi.wrapping_add(0x8u64), rcx); crate::simd_ops::wu32(rdi.wrapping_add(0x14u64), ((rcx as u32)) as u32); bb = 21; continue; }
            21 => { rbx = ((rbx).wrapping_sub(1)) as u64; crate::simd_ops::wu8(rdi.wrapping_add(0x6u64), (0x0u8) as u8); rsi = ((rsi).wrapping_sub(0x18u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x90u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x98u64), rsi); fl_l = rbx; fl_r = rbx; if (((((fl_l as u64)) & ((fl_r as u64))) as i64) >= 0) { bb = 2; continue; } bb = 22; continue; }
            22 => { r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; bb = 23; continue; }
            23 => { rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x10u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 25; continue; } bb = 24; continue; }
            24 => { crate::simd_ops::wu32(rbp.wrapping_add(0x18u64), ((rcx as u32)) as u32); rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rbp.wrapping_add(0x10u64), rax); crate::simd_ops::wu32(rbp.wrapping_add(0x1cu64), ((rax as u32)) as u32); crate::simd_ops::wu8(rbp.wrapping_add(0xau64), ((rax as u8)) as u8); rsp = ((rsp).wrapping_add(0x70u64)) as u64; rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            25 => { crate::simd_ops::wu8(rbp.wrapping_add(0xau64), (0x0u8) as u8); rsp = ((rsp).wrapping_add(0x70u64)) as u64; rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14162ab40(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415242e0;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r11 = rsp; crate::simd_ops::wu64(r11.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x50u64)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x10u64))) as u32 as u64; rdi = rcx; rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(r11.wrapping_sub(0x20u64), r12); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; r12 = ((((rax as u32)) as i32 as i64) as u64) as u64; if (((((fl_l as u32)).wrapping_sub((fl_r as u32))) as i32) < 0) { bb = 17; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(r11.wrapping_add(0x18u64), rbx); rbx = ((r12.wrapping_add(r12.wrapping_mul(2))) as u64) as u64; crate::simd_ops::wu64(r11.wrapping_sub(0x10u64), rbp); crate::simd_ops::wu64(r11.wrapping_sub(0x18u64), rsi); crate::simd_ops::wu64(r11.wrapping_sub(0x28u64), r13); fl_l = ((rbx).wrapping_shl(3u32)) as u64; rbx = ((fl_l as u64)) as u64; crate::simd_ops::wu64(r11.wrapping_sub(0x30u64), r14); crate::simd_ops::wu64(r11.wrapping_sub(0x38u64), r15); crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), rbx); bb = 2; continue; }
            2 => { r13 = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; r13 = ((r13).wrapping_add(rbx)) as u64; rax = (crate::simd_ops::ru32(r13.wrapping_add(0x10u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; r14 = ((((rax as u32)) as i32 as i64) as u64) as u64; if (((((fl_l as u32)).wrapping_sub((fl_r as u32))) as i32) < 0) { bb = 13; continue; } bb = 3; continue; }
            3 => { r15 = ((r14.wrapping_add(r14.wrapping_mul(2))) as u64) as u64; fl_l = ((r15).wrapping_shl(3u32)) as u64; r15 = ((fl_l as u64)) as u64; bb = 4; continue; }
            4 => { rbp = (crate::simd_ops::ru64(r13.wrapping_add(0x8u64))) as u64; rbp = ((rbp).wrapping_add(r15)) as u64; rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x10u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; rbx = ((((rax as u32)) as i32 as i64) as u64) as u64; if (((((fl_l as u32)).wrapping_sub((fl_r as u32))) as i32) < 0) { bb = 9; continue; } bb = 5; continue; }
            5 => { rsi = ((rbx.wrapping_add(rbx.wrapping_mul(2))) as u64) as u64; fl_l = ((rsi).wrapping_shl(3u32)) as u64; rsi = ((fl_l as u64)) as u64; bb = 6; continue; }
            6 => { rdi = (crate::simd_ops::ru64(rbp.wrapping_add(0x8u64))) as u64; rdi = ((rdi).wrapping_add(rsi)) as u64; crate::simd_ops::wu8(rdi.wrapping_add(0x4u64), (0x0u8) as u8); rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { crate::simd_ops::wu32(rdi.wrapping_add(0x10u64), ((rcx as u32)) as u32); rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rdi.wrapping_add(0x8u64), rcx); crate::simd_ops::wu32(rdi.wrapping_add(0x14u64), ((rcx as u32)) as u32); bb = 8; continue; }
            8 => { rsi = ((rsi).wrapping_sub(0x18u64)) as u64; fl_l = rbx; fl_r = (0x1u64) as u64; rbx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((((fl_l as u64)).wrapping_sub((fl_r as u64))) as i64) >= 0) { bb = 6; continue; } bb = 9; continue; }
            9 => { rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x8u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { crate::simd_ops::wu32(rbp.wrapping_add(0x10u64), ((rcx as u32)) as u32); rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rbp.wrapping_add(0x8u64), rcx); crate::simd_ops::wu32(rbp.wrapping_add(0x14u64), ((rcx as u32)) as u32); bb = 11; continue; }
            11 => { r15 = ((r15).wrapping_sub(0x18u64)) as u64; crate::simd_ops::wu8(rbp.wrapping_add(0x4u64), (0x0u8) as u8); fl_l = r14; fl_r = (0x1u64) as u64; r14 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((((fl_l as u64)).wrapping_sub((fl_r as u64))) as i64) >= 0) { bb = 4; continue; } bb = 12; continue; }
            12 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; bb = 13; continue; }
            13 => { rdx = (crate::simd_ops::ru64(r13.wrapping_add(0x8u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 15; continue; } bb = 14; continue; }
            14 => { crate::simd_ops::wu32(r13.wrapping_add(0x10u64), ((rcx as u32)) as u32); rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(r13.wrapping_add(0x8u64), rcx); crate::simd_ops::wu32(r13.wrapping_add(0x14u64), ((rcx as u32)) as u32); bb = 15; continue; }
            15 => { rbx = ((rbx).wrapping_sub(0x18u64)) as u64; crate::simd_ops::wu8(r13.wrapping_add(0x6u64), (0x0u8) as u8); fl_l = r12; fl_r = (0x1u64) as u64; r12 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), rbx); if (((((fl_l as u64)).wrapping_sub((fl_r as u64))) as i64) >= 0) { bb = 2; continue; } bb = 16; continue; }
            16 => { r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; bb = 17; continue; }
            17 => { rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 19; continue; } bb = 18; continue; }
            18 => { crate::simd_ops::wu32(rdi.wrapping_add(0x10u64), ((rcx as u32)) as u32); rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rdi.wrapping_add(0x8u64), rax); crate::simd_ops::wu32(rdi.wrapping_add(0x14u64), ((rax as u32)) as u32); crate::simd_ops::wu8(rdi.wrapping_add(0x6u64), ((rax as u8)) as u8); rsp = ((rsp).wrapping_add(0x50u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            19 => { crate::simd_ops::wu8(rdi.wrapping_add(0x6u64), (0x0u8) as u8); rsp = ((rsp).wrapping_add(0x50u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14162acc0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142690878; const P1: u64 = 0x143930f80; const P2: u64 = 0x1415adef0; const P3: u64 = 0x1415242e0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = (crate::simd_ops::ru64(rcx)) as u64; r15 = P0; r14 = rcx; rbp = 0; fl_l = 0; fl_r = 0; rcx = (crate::simd_ops::ru32(rcx.wrapping_add(0x8u64))) as u32 as u64; rax = ((rcx.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64; rdx = ((rdi.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = rdi; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 7; continue; } bb = 1; continue; }
            1 => { rbx = (crate::simd_ops::ru64(rdi)) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 2; continue; }
            2 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; crate::simd_ops::wu64(rbx, r15); fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(P1)) as u64; r8 = rbx; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), rbp); bb = 4; continue; }
            4 => { rdx = rbx; crate::simd_ops::wu8(rbx.wrapping_add(0x28u64), ((rbp as u8)) as u8); rcx = 0x10; crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), rbp); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rdi, rbp); bb = 5; continue; }
            5 => { rcx = ((crate::simd_ops::ru8(rdi.wrapping_add(0x18u64))) as u32) as u32 as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = (0xfcu8) as u8 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x10u64), ((rbp as u32)) as u32); rax = (rax & !0xffu64) | (((if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { 1u8 } else { 0u8 }) as u8 as u64)); rcx = (rcx & !0xffu64) | ((((((rcx as u8)) & (0xfeu8))) as u8 as u64)); rax = (rax & !0xffu64) | ((((((rax as u8)) | ((rcx as u8)))) as u8 as u64)); crate::simd_ops::wu8(rdi.wrapping_add(0x18u64), ((rax as u8)) as u8); rdi = ((rdi).wrapping_add(0x28u64)) as u64; rdx = (crate::simd_ops::ru32(r14.wrapping_add(0x8u64))) as u32 as u64; r8 = (crate::simd_ops::ru64(r14)) as u64; rax = ((rdx.wrapping_add(rdx.wrapping_mul(4))) as u64) as u64; rcx = ((r8.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = rdi; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 6; continue; }
            6 => { rdi = r8; rcx = ((rdx as u32)) as u32 as u64; bb = 7; continue; }
            7 => { fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 19; continue; } bb = 8; continue; }
            8 => { rax = ((rcx as u32)) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rbx = ((rdi.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rdi; fl_r = rbx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 18; continue; } bb = 9; continue; }
            9 => { crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rsi); fl_l = ((rdi).wrapping_add(0x8u64)) as u64; fl_r = fl_l; rdi = ((fl_l as u64)) as u64; bb = 10; continue; }
            10 => { rsi = (crate::simd_ops::ru64(rdi.wrapping_sub(0x8u64))) as u64; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 11; continue; }
            11 => { rdx = (crate::simd_ops::ru64(rsi.wrapping_add(0x10u64))) as u64; crate::simd_ops::wu64(rsi, r15); fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 12; continue; }
            12 => { rcx = (crate::simd_ops::ru64(P1)) as u64; r8 = rsi; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rsi.wrapping_add(0x10u64), rbp); bb = 13; continue; }
            13 => { rdx = rsi; crate::simd_ops::wu8(rsi.wrapping_add(0x28u64), ((rbp as u8)) as u8); rcx = 0x10; crate::simd_ops::wu64(rsi.wrapping_add(0x8u64), rbp); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rdi.wrapping_sub(0x8u64), rbp); bb = 14; continue; }
            14 => { rdx = (crate::simd_ops::ru64(rdi)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 16; continue; } bb = 15; continue; }
            15 => { rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rdi.wrapping_add(0x8u64), ((rbp as u32)) as u32); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rdi, rbp); crate::simd_ops::wu32(rdi.wrapping_add(0xcu64), ((rbp as u32)) as u32); bb = 16; continue; }
            16 => { rdi = ((rdi).wrapping_add(0x28u64)) as u64; rax = rdi.wrapping_sub(0x8); fl_l = rax; fl_r = rbx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 10; continue; } bb = 17; continue; }
            17 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; bb = 18; continue; }
            18 => { rdx = (crate::simd_ops::ru64(r14)) as u64; rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(r14.wrapping_add(0x8u64), ((rbp as u32)) as u32); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(r14, rbp); crate::simd_ops::wu32(r14.wrapping_add(0xcu64), ((rbp as u32)) as u32); bb = 19; continue; }
            19 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rax = 0xffff; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; crate::simd_ops::wu16(r14.wrapping_add(0x10u64), ((rax as u16)) as u16); rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14162ae30(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x142690878; const P2: u64 = 0x143930f80; const P3: u64 = 0x1415adef0; const P4: u64 = 0x28; const P5: u64 = 0x8; const P6: u64 = 0x1415242e0; const P7: u64 = 0x18;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = (crate::simd_ops::ru64(rcx)) as u64; rsi = 0; fl_l = 0; fl_r = 0; rdi = rcx; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64; rax = P1; crate::simd_ops::wu64(rbx, rax); fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru64(P2)) as u64; r8 = rbx; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(P0), rsi); bb = 3; continue; }
            3 => { rdx = rbx; crate::simd_ops::wu8(rbx.wrapping_add(P4), ((rsi as u8)) as u8); rcx = 0x10; crate::simd_ops::wu64(rbx.wrapping_add(P5), rsi); rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rdi, rsi); bb = 4; continue; }
            4 => { rcx = ((crate::simd_ops::ru8(rdi.wrapping_add(P7))) as u32) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = (0xfcu8) as u8 as u64; crate::simd_ops::wu32(rdi.wrapping_add(P0), ((rsi as u32)) as u32); rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rax = (rax & !0xffu64) | (((if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { 1u8 } else { 0u8 }) as u8 as u64)); rcx = (rcx & !0xffu64) | ((((((rcx as u8)) & (0xfeu8))) as u8 as u64)); rax = (rax & !0xffu64) | ((((((rax as u8)) | ((rcx as u8)))) as u8 as u64)); crate::simd_ops::wu8(rdi.wrapping_add(P7), ((rax as u8)) as u8); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14162aeb0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x28; const P1: u64 = 0x10; const P2: u64 = 0x29; const P3: u64 = 0x18; const P4: u64 = 0x20; const P5: u64 = 0x8; const P6: u64 = 0x14162a1e0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 15>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x70u64)) as u64; rbx = rcx; rdi = (rdi & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rcx = 0; fl_l = 0; fl_r = 0; fl_l = ((r9 as u8)) as u8 as u64; fl_r = ((r9 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rdi = ((crate::simd_ops::ru8(rbx.wrapping_add(P0))) as u32) as u32 as u64; crate::simd_ops::wu64(rbx.wrapping_add(P1), rcx); bb = 2; continue; }
            2 => { crate::simd_ops::wu8(rbx.wrapping_add(P2), (0x1u8) as u8); r9 = rsp.wrapping_add(0x98); rax = (crate::simd_ops::ru64(rbx.wrapping_add(P3))) as u64; rdx = rsp.wrapping_add(0x40); crate::simd_ops::wu8(rsp.wrapping_add(0x30u64), ((rdi as u8)) as u8); crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rcx as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rax); rax = (crate::simd_ops::ru32(rbx.wrapping_add(P4))) as u32 as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x98u64), ((rcx as u8)) as u8); crate::simd_ops::wu32(rsp.wrapping_add(0x50u64), ((rcx as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), rcx); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rcx); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P5))) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x48u64), ((rax as u32)) as u32); crate::simd_ops::wu8(rsp.wrapping_add(0x58u64), (0xffu8) as u8); crate::simd_ops::wu8(rsp.wrapping_add(0x60u64), (0xffu8) as u8); rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rdi as u8)) as u8 as u64; fl_r = ((rdi as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu8(rbx.wrapping_add(P2), ((rdi as u8)) as u8); bb = 4; continue; }
            4 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; rsp = ((rsp).wrapping_add(0x70u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14162af40(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1416285e0; const P1: u64 = 0x1415fa0e0; const P2: u64 = 0x14183acc8; const P3: u64 = 0x1415242e0; const P4: u64 = 0x14162b200;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; r14 = rdx; rsi = rcx; rdx = ((crate::simd_ops::ru8(rdx)) as u32) as u32 as u64; fl_l = ((rdx as u8)) as u8 as u64; fl_r = (0xffu8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 2; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru64(r14.wrapping_add(0x8u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 19; continue; } bb = 2; continue; }
            2 => { rbp = rcx.wrapping_add(0x8); rcx = rbp; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rdi = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 19; continue; } bb = 3; continue; }
            3 => { rdx = (crate::simd_ops::ru64(r14.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), r15); r15 = 0; fl_l = 0; fl_r = 0; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 4; continue; }
            4 => { rcx = rax.wrapping_add(0x8); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 5; continue; }
            5 => { rdx = (crate::simd_ops::ru32(rdi.wrapping_add(0x10u64))) as u32 as u64; rdx = (((rdx as u32)).wrapping_sub(1)) as u32 as u64; rdx = ((rdx).wrapping_shl(4u32)) as u64; rdx = ((rdx).wrapping_add(crate::simd_ops::ru64(rdi.wrapping_add(0x8u64)))) as u64; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rdx = ((rdx).wrapping_sub(rax)) as u64; rcx = rax; rdx = ((((rdx as u32)) & (0xfffffff0u32))) as u32 as u64; r8 = ((rdx as u32)) as u32 as u64; rdx = rax.wrapping_add(0x10); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 7; continue; }
            7 => { fl_l = (crate::simd_ops::lk_sub_u32(rdi.wrapping_add(0x10u64), (1) as u32)) as u32 as u64; rsi = (rsi & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 9; continue; }
            8 => { rsi = (rsi & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 9; continue; }
            9 => { fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x10u64))) as u32 as u64; fl_r = ((r15 as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 13; continue; } bb = 10; continue; }
            10 => { rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 11; continue; }
            11 => { rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rdi.wrapping_add(0x10u64), ((r15 as u32)) as u32); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rdi.wrapping_add(0x8u64), r15); crate::simd_ops::wu32(rdi.wrapping_add(0x14u64), ((r15 as u32)) as u32); bb = 13; continue; }
            12 => { rsi = ((crate::simd_ops::ru8(rax.wrapping_add(0x4u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rax.wrapping_add(0x4u64), ((r15 as u8)) as u8); bb = 13; continue; }
            13 => { fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(0x4u64))) as u8 as u64; fl_r = ((r15 as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 18; continue; } bb = 14; continue; }
            14 => { fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x10u64))) as u32 as u64; fl_r = ((r15 as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 18; continue; } bb = 15; continue; }
            15 => { crate::simd_ops::wu8(rdi.wrapping_add(0x4u64), ((r15 as u8)) as u8); rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 16; continue; }
            16 => { rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rdi.wrapping_add(0x10u64), ((r15 as u32)) as u32); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rdi.wrapping_add(0x8u64), r15); crate::simd_ops::wu32(rdi.wrapping_add(0x14u64), ((r15 as u32)) as u32); bb = 17; continue; }
            17 => { rdx = ((crate::simd_ops::ru8(r14)) as u32) as u32 as u64; rcx = rbp; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 18; continue; }
            18 => { r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rax = (((rsi as u8)) as u32) as u32 as u64; bb = 20; continue; }
            19 => { rax = ((crate::simd_ops::ru8(rsi.wrapping_add(0x4u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rsi.wrapping_add(0x4u64), (0x0u8) as u8); bb = 20; continue; }
            20 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14162b060(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141628690; const P1: u64 = 0x1416285e0; const P2: u64 = 0x14162af40; const P3: u64 = 0x14162a870; const P4: u64 = 0x14162b200; const P5: u64 = 0x14162a750; const P6: u64 = 0x14162b270; const P7: u64 = 0x14162ab40;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; r12 = rdx; rdi = rcx; rdx = (crate::simd_ops::ru32(rdx)) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 5; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru32(r12.wrapping_add(0x8u64))) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 5; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru8(r12.wrapping_add(0x10u64))) as u8 as u64; fl_r = (0xffu8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 5; continue; } bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru8(r12.wrapping_add(0x18u64))) as u8 as u64; fl_r = (0xffu8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 5; continue; } bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::ru64(r12.wrapping_add(0x20u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 28; continue; } bb = 5; continue; }
            5 => { r15 = rcx.wrapping_add(0x10); rcx = r15; crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), r15); rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rsi = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 28; continue; } bb = 6; continue; }
            6 => { rdx = (crate::simd_ops::ru32(r12.wrapping_add(0x8u64))) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), r14); fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 10; continue; } bb = 7; continue; }
            7 => { fl_l = (crate::simd_ops::ru8(r12.wrapping_add(0x10u64))) as u8 as u64; fl_r = (0xffu8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 10; continue; } bb = 8; continue; }
            8 => { fl_l = (crate::simd_ops::ru8(r12.wrapping_add(0x18u64))) as u8 as u64; fl_r = (0xffu8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 10; continue; } bb = 9; continue; }
            9 => { fl_l = (crate::simd_ops::ru64(r12.wrapping_add(0x20u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 23; continue; } bb = 10; continue; }
            10 => { rcx = rax.wrapping_add(0x8); rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rbp = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 23; continue; } bb = 11; continue; }
            11 => { rdx = ((crate::simd_ops::ru8(r12.wrapping_add(0x10u64))) as u32) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), r13); fl_l = ((rdx as u8)) as u8 as u64; fl_r = (0xffu8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 14; continue; } bb = 12; continue; }
            12 => { fl_l = (crate::simd_ops::ru8(r12.wrapping_add(0x18u64))) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 14; continue; } bb = 13; continue; }
            13 => { fl_l = (crate::simd_ops::ru64(r12.wrapping_add(0x20u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 18; continue; } bb = 14; continue; }
            14 => { rcx = rax.wrapping_add(0x8); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); r15 = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 18; continue; } bb = 15; continue; }
            15 => { rdx = r12.wrapping_add(0x18); rcx = rax; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru8(r15.wrapping_add(0x4u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; r14 = (((rax as u8)) as u32) as u32 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 19; continue; } bb = 16; continue; }
            16 => { fl_l = (crate::simd_ops::ru32(r15.wrapping_add(0x10u64))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 19; continue; } bb = 17; continue; }
            17 => { rcx = r15; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rdx = ((crate::simd_ops::ru8(r12.wrapping_add(0x10u64))) as u32) as u32 as u64; rcx = rbp.wrapping_add(0x8); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 19; continue; }
            18 => { r14 = ((crate::simd_ops::ru8(rbp.wrapping_add(0x6u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rbp.wrapping_add(0x6u64), (0x0u8) as u8); bb = 19; continue; }
            19 => { fl_l = (crate::simd_ops::ru8(rbp.wrapping_add(0x6u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 22; continue; } bb = 20; continue; }
            20 => { fl_l = (crate::simd_ops::ru32(rbp.wrapping_add(0x10u64))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 22; continue; } bb = 21; continue; }
            21 => { rcx = rbp; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru32(r12.wrapping_add(0x8u64))) as u32 as u64; rcx = rsi.wrapping_add(0x8); rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); bb = 22; continue; }
            22 => { r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; bb = 24; continue; }
            23 => { r14 = ((crate::simd_ops::ru8(rsi.wrapping_add(0x6u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rsi.wrapping_add(0x6u64), (0x0u8) as u8); bb = 24; continue; }
            24 => { fl_l = (crate::simd_ops::ru8(rsi.wrapping_add(0x6u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 27; continue; } bb = 25; continue; }
            25 => { fl_l = (crate::simd_ops::ru32(rsi.wrapping_add(0x10u64))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 27; continue; } bb = 26; continue; }
            26 => { rcx = rsi; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru32(r12)) as u32 as u64; rcx = r15; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); bb = 27; continue; }
            27 => { rax = (((r14 as u8)) as u32) as u32 as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            28 => { rax = ((crate::simd_ops::ru8(rdi.wrapping_add(0xau64))) as u32) as u32 as u64; crate::simd_ops::wu8(rdi.wrapping_add(0xau64), (0x0u8) as u8); rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14162b2e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141628740; const P1: u64 = 0x8; const P2: u64 = 0x20; const P3: u64 = 0x14183acc8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    fl_l = rax; fl_r = rax;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { r8 = (crate::simd_ops::ru32(rbx.wrapping_add(P1))) as u32 as u64; r8 = (((r8 as u32)).wrapping_sub(1)) as u32 as u64; r8 = ((r8).wrapping_shl(5u32)) as u64; r8 = ((r8).wrapping_add(crate::simd_ops::ru64(rbx))) as u64; fl_l = rax; fl_r = r8; if !((((fl_l as u64)) >= ((fl_r as u64)))) { r8 = ((r8).wrapping_sub(rax)) as u64; rdx = ((rax.wrapping_add(P2)) as u64) as u64; r8 = ((((r8 as u32)) & (0xffffffe0u32))) as u32 as u64; rcx = rax; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); } crate::simd_ops::lk_sub_u32(rbx.wrapping_add(P1), (1) as u32); rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14162b330(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x30; const P1: u64 = 0x141626080; const P2: u64 = 0x4c; const P3: u64 = 0x58; const P4: u64 = 0x14162a6a0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    r8 = rdx;
    rcx = ((rcx).wrapping_add(0x48u64)) as u64;
    rdx = ((rbx.wrapping_add(P0)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64;
    if !(((((fl_l as u8)) & ((fl_r as u8))) == 0)) { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(P2))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { rax = 0x1; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; } fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(P3))) as u32 as u64; fl_r = 0x0; if !((((fl_l as u32)) != ((fl_r as u32)))) { rcx = ((rbx.wrapping_add(P0)) as u64) as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); } rax = 0x1; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rax = 0x2;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14162b390(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141628d50; const P1: u64 = 0x1416287e0; const P2: u64 = 0x48; const P3: u64 = 0x30; const P4: u64 = 0x141626080; const P5: u64 = 0x4c; const P6: u64 = 0x58; const P7: u64 = 0x14162a6a0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rbp = r8; rsi = rdx; rbx = rcx; fl_l = ((r9 as u8)) as u8 as u64; fl_r = ((r9 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { r8 = rsp.wrapping_add(0x30); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (0x0u64) as u64); rcx = ((rcx).wrapping_add(0x30u64)) as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 2; continue; }
            2 => { r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; r8 = rax; rdx = rsi; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbp); rcx = rbx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 7; continue; } bb = 3; continue; }
            3 => { rcx = ((rbx.wrapping_add(P2)) as u64) as u64; r8 = rsi; rdx = ((rbx.wrapping_add(P3)) as u64) as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 8; continue; } bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(P5))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 7; continue; } bb = 5; continue; }
            5 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(P6))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rcx = ((rbx.wrapping_add(P3)) as u64) as u64; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); bb = 7; continue; }
            7 => { rax = 0x1; bb = 9; continue; }
            8 => { rax = 0x2; bb = 9; continue; }
            9 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14162b440(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x30; const P1: u64 = 0x141627790; const P2: u64 = 0x4c; const P3: u64 = 0x58; const P4: u64 = 0x14162a6a0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x60u64)) as u64;
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu8(rsp.wrapping_add(0x48u64), (0xffu8) as u8);
    rbx = rcx;
    crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax);
    crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), ((rax as u32)) as u32);
    r9 = rsp.wrapping_add(0x30);
    crate::simd_ops::wu32(rsp.wrapping_add(0x40u64), ((rax as u32)) as u32);
    r8 = rdx;
    crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rax);
    rcx = ((rcx).wrapping_add(0x48u64)) as u64;
    rax = rsp.wrapping_add(0x30);
    crate::simd_ops::wu8(rsp.wrapping_add(0x50u64), (0xffu8) as u8);
    rdx = ((rbx.wrapping_add(P0)) as u64) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax);
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(P2))) as u8 as u64; fl_r = (0x0u8) as u8 as u64;
    if (((fl_l as u8)) != ((fl_r as u8))) { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rsp = ((rsp).wrapping_add(0x60u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(P3))) as u32 as u64; fl_r = 0x0;
    if !((((fl_l as u32)) != ((fl_r as u32)))) { rcx = ((rbx.wrapping_add(P0)) as u64) as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64;
    rsp = ((rsp).wrapping_add(0x60u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14162b4b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x142690888; const P1: u64 = 0x8; const P2: u64 = 0x10; const P3: u64 = 0x18; const P4: u64 = 0x20; const P5: u64 = 0x28; const P6: u64 = 0x30; const P7: u64 = 0x38; const P8: u64 = 0x40; const P9: u64 = 0x4a; const P10: u64 = 0x50; const P11: u64 = 0x5c;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = P0;
    crate::simd_ops::wu64(rcx, rax);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rcx.wrapping_add(P1), rax);
    crate::simd_ops::wu64(rcx.wrapping_add(P2), rax);
    crate::simd_ops::wu64(rcx.wrapping_add(P3), rax);
    crate::simd_ops::wu8(rcx.wrapping_add(P4), ((rax as u8)) as u8);
    crate::simd_ops::wu64(rcx.wrapping_add(P5), rax);
    crate::simd_ops::wu64(rcx.wrapping_add(P6), rax);
    crate::simd_ops::wu64(rcx.wrapping_add(P7), rax);
    crate::simd_ops::wu32(rcx.wrapping_add(P8), ((rax as u32)) as u32);
    crate::simd_ops::wu8(rcx.wrapping_add(P9), ((rax as u8)) as u8);
    crate::simd_ops::wu32(rcx.wrapping_add(P10), ((rax as u32)) as u32);
    crate::simd_ops::wu64(rcx.wrapping_add(P11), rax);
    crate::simd_ops::wu64(rcx.wrapping_add(0x64u64), rax);
    rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x6cu64))) as u32 as u64;
    rax = ((((rax as u32)) & (0xfffffffcu32))) as u32 as u64;
    fl_l = ((((rax as u32)) | (0x4u32))) as u32 as u64; fl_r = fl_l;
    rax = ((fl_l as u32)) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(0x6cu64), ((rax as u32)) as u32);
    rax = rcx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14162b540(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut r10, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = 0; fl_l = 0; fl_r = 0; r10 = rdx; r9 = ((rax as u32)) as u32 as u64; bb = 1; continue; }
            1 => { r9 = (((r9 as u32)).wrapping_shl(7u32)) as u32 as u64; crate::simd_ops::wu32(r10, ((r9 as u32)) as u32); rdx = (crate::simd_ops::ru64(rcx)) as u64; r8 = ((crate::simd_ops::ru8(rdx)) as u32) as u32 as u64; rdx = ((rdx).wrapping_add(1)) as u64; crate::simd_ops::wu64(rcx, rdx); r9 = ((r8 as u32)) as u32 as u64; r9 = ((((r9 as u32)) & (0x7fu32))) as u32 as u64; r9 = ((((r9 as u32)) | (crate::simd_ops::ru32(r10)))) as u32 as u64; crate::simd_ops::wu32(r10, ((r9 as u32)) as u32); fl_l = ((r8 as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 5; continue; } bb = 2; continue; }
            2 => { rax = (((rax as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x4; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 1; continue; } bb = 3; continue; }
            3 => { if (((fl_l as u32)) != ((fl_r as u32))) { bb = 5; continue; } bb = 4; continue; }
            4 => { crate::simd_ops::wu32(r10, (0xffffffffu32) as u32); bb = 5; continue; }
            5 => { return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14162b590(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x64; const P1: u64 = 0x8; const P2: u64 = 0x68; const P3: u64 = 0xc; const P4: u64 = 0x5c; const P5: u64 = 0x60; const P6: u64 = 0x4;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64;
    crate::simd_ops::wu32(rdx.wrapping_add(P1), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rcx.wrapping_add(P2))) as u32 as u64;
    crate::simd_ops::wu32(rdx.wrapping_add(P3), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rcx.wrapping_add(P4))) as u32 as u64;
    crate::simd_ops::wu32(rdx, ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rcx.wrapping_add(P5))) as u32 as u64;
    crate::simd_ops::wu32(rdx.wrapping_add(P6), ((rax as u32)) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14162b5b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14162b9f0; const P1: u64 = 0x14162b8e0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; r15 = r8; rdi = rcx.wrapping_add(0x38); r14 = rdx; rbx = rcx; r12 = 0x80; bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0x6cu64))) as u8 as u64; fl_r = (0x2u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 8; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru64(rdi)) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 8; continue; } bb = 3; continue; }
            3 => { rax = ((crate::simd_ops::ru8(rbx.wrapping_add(0x44u64))) as u32) as u32 as u64; rcx = (((rax as u8)) as u32) as u32 as u64; rcx = (rcx & !0xffu64) | ((((((rcx as u8)) & (0xf0u8))) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if (((((fl_l as u8)) & ((fl_r as u8))) as i8) >= 0) { bb = 8; continue; } bb = 4; continue; }
            4 => { fl_l = ((rcx as u8)) as u8 as u64; fl_r = (0xf0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 8; continue; } bb = 5; continue; }
            5 => { crate::simd_ops::wu8(r14, ((rcx as u8)) as u8); rax = ((crate::simd_ops::ru8(rbx.wrapping_add(0x44u64))) as u32) as u32 as u64; rax = (rax & !0xffu64) | ((((((rax as u8)) & (0xfu8))) as u8 as u64)); crate::simd_ops::wu8(r14.wrapping_add(0x1u64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(rbx.wrapping_add(0x45u64))) as u32) as u32 as u64; crate::simd_ops::wu8(r14.wrapping_add(0x4u64), ((rax as u8)) as u8); rdx = ((crate::simd_ops::ru8(rbx.wrapping_add(0x46u64))) as u32) as u32 as u64; crate::simd_ops::wu8(r14.wrapping_add(0x5u64), ((rdx as u8)) as u8); fl_l = ((rcx as u8)) as u8 as u64; fl_r = (0x90u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 7; continue; } bb = 6; continue; }
            6 => { fl_l = ((rdx as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; rax = (((rcx as u8)) as u32) as u32 as u64; rax = (if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { (r12 as u32) } else { (rax as u32) }) as u32 as u64; crate::simd_ops::wu8(r14, ((rax as u8)) as u8); bb = 7; continue; }
            7 => { rsi = (rsi & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 9; continue; }
            8 => { rsi = (rsi & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x5cu64))) as u32 as u64; crate::simd_ops::wu32(r15, ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x6cu64))) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x2u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rdi = rbx.wrapping_add(0x38); bb = 18; continue; }
            11 => { fl_l = (crate::simd_ops::ru64(rdi)) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 14; continue; } bb = 12; continue; }
            12 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 14; continue; } bb = 13; continue; }
            13 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 18; continue; }
            14 => { rcx = rbx.wrapping_add(0x20); rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru64(rdi)) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 17; continue; } bb = 15; continue; }
            15 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0x6cu64))) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 17; continue; } bb = 16; continue; }
            16 => { rdx = 0x1; rcx = rbx.wrapping_add(0x20); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 17; continue; }
            17 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x40u64))) as u32 as u64; crate::simd_ops::lk_add_u32(rbx.wrapping_add(0x60u64), ((rax as u32)) as u32); rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); fl_l = (crate::simd_ops::lk_add_u32(rbx.wrapping_add(0x5cu64), (1) as u32)) as u32 as u64; bb = 18; continue; }
            18 => { fl_l = ((rsi as u8)) as u8 as u64; fl_r = ((rsi as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 21; continue; } bb = 19; continue; }
            19 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 1; continue; } bb = 20; continue; }
            20 => { bb = 22; continue; }
            21 => { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 22; continue; }
            22 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}
