#![allow(unused_imports, unused_variables, unused_mut, unused_assignments, non_snake_case, dead_code, clippy::all, unreachable_code, unused_parens)]

#[inline(never)]
pub unsafe extern "win64" fn fn_141863120(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143080988; const P1: u64 = 0x78; const P2: u64 = 0x70; const P3: u64 = 0x68; const P4: u64 = 0x60; const P5: u64 = 0x140203870; const P6: u64 = 0x58; const P7: u64 = 0x1402028d0; const P8: u64 = 0x8; const P9: u64 = 0x142645830; const P10: u64 = 0x142670ea0; const P11: u64 = 0x141834d30;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = P0; rbx = rcx; crate::simd_ops::wu64(rcx, rax); rdi = ((rdx as u32)) as u32 as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64; rcx = ((rcx).wrapping_sub(1)) as u64; rax = (crate::simd_ops::ru64(rbx.wrapping_add(P3))) as u64; rdx = ((rdx).wrapping_add(rcx)) as u64; rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P4))) as u64; rax = ((rax).wrapping_sub(1)) as u64; rdx = (((rdx) & (rax))) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(rdx.wrapping_mul(8)))) as u64; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64; fl_r = (0x1u64) as u64; crate::simd_ops::wu64(rbx.wrapping_add(P1), ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64); if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rbx.wrapping_add(P2), (0x0u64) as u64); bb = 3; continue; }
            3 => { rcx = ((rbx.wrapping_add(P6)) as u64) as u64; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rcx = ((rbx.wrapping_add(P8)) as u64) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P9), rax), rax, rcx, rdx, r8, r9, rsp); rax = P10; crate::simd_ops::wu64(rbx, rax); fl_l = ((rdi as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rdx = 0x80; rcx = rbx; rax = crate::simd_ops::call_stk(P11, rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            5 => { rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1418631c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142645828; const P1: u64 = 0x142645840; const P2: u64 = 0x140205b90; const P3: u64 = 0x141834d64; const P4: u64 = 0x142645a78;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = r8; r14 = rdx; rdi = rcx; rcx = ((rcx).wrapping_add(0x8u64)) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = 0x5; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x1418631fa, rcx, rsp); return rax; }
            2 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x54u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x7fffffff; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = (((rax as u32)).wrapping_sub(1)) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x54u64), ((rax as u32)) as u32); rcx = 0x6; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x141863215, rcx, rsp); return rax; }
            4 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x78u64))) as u64; rax = ((rax).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru64(rdi.wrapping_add(0x68u64))) as u64; fl_r = rax; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rcx = rdi.wrapping_add(0x58); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            6 => { rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x68u64))) as u64; rdx = ((rdx).wrapping_sub(1)) as u64; rcx = rdx; rcx = (((rcx) & (crate::simd_ops::ru64(rdi.wrapping_add(0x70u64))))) as u64; crate::simd_ops::wu64(rdi.wrapping_add(0x70u64), rcx); rbp = (crate::simd_ops::ru64(rdi.wrapping_add(0x78u64))) as u64; rbp = ((rbp).wrapping_add(rcx)) as u64; rdx = (((rdx) & (rbp))) as u64; r15 = ((rdx.wrapping_mul(8)) as u64) as u64; rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x60u64))) as u64; fl_l = (crate::simd_ops::ru64(r15.wrapping_add(rax))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 8; continue; } bb = 7; continue; }
            7 => { rcx = 0x10; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x60u64))) as u64; crate::simd_ops::wu64(r15.wrapping_add(rcx), rax); bb = 8; continue; }
            8 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x68u64))) as u64; rcx = ((rcx).wrapping_sub(1)) as u64; rcx = (((rcx) & (rbp))) as u64; rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x60u64))) as u64; rdx = (crate::simd_ops::ru64(rax.wrapping_add(rcx.wrapping_mul(8)))) as u64; rbp = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rdx, rbp); crate::simd_ops::wu64(rdx.wrapping_add(0x8u64), rbp); rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { fl_l = (crate::simd_ops::lk_add_u32(rax.wrapping_add(0x8u64), (1) as u32)) as u32 as u64; bb = 10; continue; }
            10 => { rax = (crate::simd_ops::ru64(rsi)) as u64; crate::simd_ops::wu64(rdx, rax); rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rdx.wrapping_add(0x8u64), rax); crate::simd_ops::lk_add_u64(rdi.wrapping_add(0x78u64), (1) as u64); rcx = 0x10; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = rax; crate::simd_ops::wu64(rax, rbp); crate::simd_ops::wu64(rax.wrapping_add(0x8u64), rbp); rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 11; continue; }
            11 => { fl_l = (crate::simd_ops::lk_add_u32(rax.wrapping_add(0x8u64), (1) as u32)) as u32 as u64; bb = 12; continue; }
            12 => { rax = (crate::simd_ops::ru64(rsi)) as u64; crate::simd_ops::wu64(rcx, rax); rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rcx.wrapping_add(0x8u64), rax); crate::simd_ops::wu64(r14, rcx); rcx = rdi.wrapping_add(0x8); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); rax = r14; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141863310(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x78; const P1: u64 = 0x142645828; const P2: u64 = 0x142645840; const P3: u64 = 0x54; const P4: u64 = 0x70; const P5: u64 = 0x68; const P6: u64 = 0x60; const P7: u64 = 0x140203870; const P8: u64 = 0x8; const P9: u64 = 0x142645a78;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = rcx; fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            2 => { rcx = ((rcx).wrapping_add(0x8u64)) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rcx = 0x5; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x14186334a, rcx, rsp); return rax; }
            4 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(P3))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x7fffffff; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = (((rax as u32)).wrapping_sub(1)) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(P3), ((rax as u32)) as u32); rcx = 0x6; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x141863365, rcx, rsp); return rax; }
            6 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P4))) as u64; rdx = (crate::simd_ops::ru64(rdi.wrapping_add(P0))) as u64; rcx = ((rcx).wrapping_sub(1)) as u64; rdx = ((rdx).wrapping_add(rcx)) as u64; rax = (crate::simd_ops::ru64(rdi.wrapping_add(P5))) as u64; rax = ((rax).wrapping_sub(1)) as u64; rdx = (((rdx) & (rax))) as u64; rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P6))) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(rdx.wrapping_mul(8)))) as u64; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru64(rdi.wrapping_add(P0))) as u64; fl_r = (0x1u64) as u64; crate::simd_ops::wu64(rdi.wrapping_add(P0), ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64); if (((fl_l as u64)) != ((fl_r as u64))) { bb = 8; continue; } bb = 7; continue; }
            7 => { crate::simd_ops::wu64(rdi.wrapping_add(P4), (0x0u64) as u64); bb = 8; continue; }
            8 => { rcx = ((rdi.wrapping_add(P8)) as u64) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P9), rax), rax, rcx, rdx, r8, r9, rsp); rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1418633c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142645828; const P1: u64 = 0x142645840; const P2: u64 = 0x54; const P3: u64 = 0x78; const P4: u64 = 0x70; const P5: u64 = 0x68; const P6: u64 = 0x60; const P7: u64 = 0x8; const P8: u64 = 0x142645a78;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = rdx; rsi = rcx; rcx = ((rcx).wrapping_add(0x8u64)) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = 0x5; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x1418633ee, rcx, rsp); return rax; }
            2 => { rax = (crate::simd_ops::ru32(rsi.wrapping_add(P2))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x7fffffff; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = (((rax as u32)).wrapping_sub(1)) as u32 as u64; crate::simd_ops::wu32(rsi.wrapping_add(P2), ((rax as u32)) as u32); rcx = 0x6; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x141863409, rcx, rsp); return rax; }
            4 => { rcx = (crate::simd_ops::ru64(rsi.wrapping_add(P3))) as u64; rdx = (crate::simd_ops::ru64(rsi.wrapping_add(P4))) as u64; rcx = ((rcx).wrapping_sub(1)) as u64; rdx = ((rdx).wrapping_add(rcx)) as u64; rax = (crate::simd_ops::ru64(rsi.wrapping_add(P5))) as u64; rax = ((rax).wrapping_sub(1)) as u64; rdx = (((rdx) & (rax))) as u64; rax = (crate::simd_ops::ru64(rsi.wrapping_add(P6))) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(rdx.wrapping_mul(8)))) as u64; rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rdi, rax); crate::simd_ops::wu64(rdi.wrapping_add(P7), rax); rax = (crate::simd_ops::ru64(rcx.wrapping_add(P7))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { fl_l = (crate::simd_ops::lk_add_u32(rax.wrapping_add(P7), (1) as u32)) as u32 as u64; bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::wu64(rdi, rax); rax = (crate::simd_ops::ru64(rcx.wrapping_add(P7))) as u64; crate::simd_ops::wu64(rdi.wrapping_add(P7), rax); rcx = ((rsi.wrapping_add(P7)) as u64) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P8), rax), rax, rcx, rdx, r8, r9, rsp); rax = rdi; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141863480(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143080988; const P1: u64 = 0x78; const P2: u64 = 0x70; const P3: u64 = 0x68; const P4: u64 = 0x60; const P5: u64 = 0x140203870; const P6: u64 = 0x58; const P7: u64 = 0x1402028d0; const P8: u64 = 0x8; const P9: u64 = 0x142645830; const P10: u64 = 0x142670ea0;
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
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rcx = ((rbx.wrapping_add(P6)) as u64) as u64; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rcx = ((rbx.wrapping_add(P8)) as u64) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P9), rax), rax, rcx, rdx, r8, r9, rsp); rax = P10; crate::simd_ops::wu64(rbx, rax); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64;
    rcx = ((rcx).wrapping_sub(1)) as u64;
    rax = (crate::simd_ops::ru64(rbx.wrapping_add(P3))) as u64;
    rdx = ((rdx).wrapping_add(rcx)) as u64;
    rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P4))) as u64;
    rax = ((rax).wrapping_sub(1)) as u64;
    rdx = (((rdx) & (rax))) as u64;
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(rdx.wrapping_mul(8)))) as u64;
    rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp);
    fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64; fl_r = (0x1u64) as u64;
    crate::simd_ops::wu64(rbx.wrapping_add(P1), ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64);
    if !((((fl_l as u64)) != ((fl_r as u64)))) { crate::simd_ops::wu64(rbx.wrapping_add(P2), (0x0u64) as u64); }
    rcx = ((rbx.wrapping_add(P6)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp);
    rcx = ((rbx.wrapping_add(P8)) as u64) as u64;
    rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P9), rax), rax, rcx, rdx, r8, r9, rsp);
    rax = P10;
    crate::simd_ops::wu64(rbx, rax);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141863500(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143080988; const P1: u64 = 0x142645820; const P2: u64 = 0x141834d64; const P3: u64 = 0x140205b90;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; r14 = rcx; rax = P0; crate::simd_ops::wu64(rcx, rax); rcx = ((rcx).wrapping_add(0x8u64)) as u64; rbp = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rcx, ((rbp as u32)) as u32); crate::simd_ops::wu64(rcx.wrapping_add(0x8u64), rbp); crate::simd_ops::wu64(rcx.wrapping_add(0x10u64), rbp); xmm0 = 0u128; ymm0_hi = 0; let y_addr: u64 = rcx.wrapping_add(0x18u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); crate::simd_ops::wu128(rcx.wrapping_add(0x38u64), (xmm0) as u128); crate::simd_ops::wu64(rcx.wrapping_add(0x48u64), rbp); rdx = ((rbp.wrapping_add(0x2u64)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = r14.wrapping_add(0x58); crate::simd_ops::wu64(rbx, rbp); crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), rbp); crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rbx.wrapping_add(0x18u64), rbp); crate::simd_ops::wu64(rbx.wrapping_add(0x20u64), rbp); rcx = ((rbp.wrapping_add(0x10u64)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rax.wrapping_add(0x8u64), rbp); crate::simd_ops::wu64(rbx, rax); crate::simd_ops::wu64(rax, rbx); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x20u64))) as u64; rax = ((rax).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; fl_r = rax; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 2; continue; }
            2 => { r8 = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; r8 = ((r8).wrapping_sub(1)) as u64; crate::simd_ops::lk_and_u64(rbx.wrapping_add(0x18u64), (r8) as u64); rdi = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; rdi = ((rdi).wrapping_add(crate::simd_ops::ru64(rbx.wrapping_add(0x20u64)))) as u64; r8 = (((r8) & (rdi))) as u64; rsi = ((r8.wrapping_mul(8)) as u64) as u64; rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; fl_l = (crate::simd_ops::ru64(rsi.wrapping_add(rax))) as u64; fl_r = rbp; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rcx = 0x10; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rsi.wrapping_add(rcx), rax); bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rcx = ((rcx).wrapping_sub(1)) as u64; rcx = (((rcx) & (rdi))) as u64; rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(rcx.wrapping_mul(8)))) as u64; crate::simd_ops::wu64(rcx, rbp); crate::simd_ops::wu64(rcx.wrapping_add(0x8u64), rbp); crate::simd_ops::lk_add_u64(rbx.wrapping_add(0x20u64), (1) as u64); rax = r14; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141863610(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x140204b40; const P1: u64 = 0x141863720; const P2: u64 = 0x140203210; const P3: u64 = 0x140206830; const P4: u64 = 0x1402032e0; const P5: u64 = 0x141863716;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x141863610, funcinfo_rva: 0x36f9488, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x58, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rbp) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rcx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rbx = (rdx) as u64; rdi = (rcx) as u64; rax = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rax as u32)) as u32); crate::simd_ops::wu64(rcx, (rax) as u64); crate::simd_ops::wu64(rcx.wrapping_add(0x8u64), (rax) as u64); crate::simd_ops::wu64(rcx.wrapping_add(0x10u64), (rax) as u64); crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x1u32) as u32); rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(rdx))) as u64; rax = (0x4924924924924925u64) as u64; let prod = (rax as i64 as i128).wrapping_mul((rcx) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rdx = ((((rdx) as i64).wrapping_shr(4u32) as u64)) as u64; rax = (rdx) as u64; rax = ((rax).wrapping_shr(63u32)) as u64; fl_l = ((rdx).wrapping_add(rax)) as u64 as u64; fl_r = fl_l; rdx = ((fl_l as u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), (rdx) as u64); if (((fl_l as u64)) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { rax = (0x492492492492492u64) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 10; continue; } bb = 2; continue; }
            2 => { rdx = ((rsp.wrapping_add(0x68u64)) as u64) as u64; rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14186368a); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 3; continue; }
            3 => { r14 = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rbx = (crate::simd_ops::ru64(rbx)) as u64; fl_l = (rbx) as u64 as u64; fl_r = (r14) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 9; continue; } bb = 4; continue; }
            4 => { bb = 5; continue; }
            5 => { rdx = ((rbx.wrapping_add(0x20u64)) as u64) as u64; rcx = ((rsp.wrapping_add(0x28u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1418636a9); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rbp = (rax) as u64; rsi = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; fl_l = (rsi) as u64 as u64; fl_r = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rdx = (rbx) as u64; rcx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1418636c1); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; xmm0 = (crate::simd_ops::ru128(rbp)) as u128; crate::simd_ops::wu128(rsi.wrapping_add(0x20u64), (xmm0) as u128); xmm1 = ((crate::simd_ops::ru64(rbp.wrapping_add(0x10u64))) as u64) as u128; crate::simd_ops::wu64(rsi.wrapping_add(0x30u64), ((xmm1 as u64)) as u64); fl_l = (crate::simd_ops::lk_add_u64(rdi.wrapping_add(0x8u64), (0x38u64) as u64)) as u64; fl_r = fl_l; bb = 8; continue; }
            7 => { r9 = (rbp) as u64; r8 = (rbx) as u64; rdx = (rsi) as u64; rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1418636ed); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 8; continue; }
            8 => { rbx = ((rbx).wrapping_add(0x38u64)) as u64; fl_l = (rbx) as u64 as u64; fl_r = (r14) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 5; continue; } bb = 9; continue; }
            9 => { rax = (rdi) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rsi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            10 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141863711); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::lift_ops::trap(0x80000003, P5, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141863720(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x140000000; const P1: u64 = 0x140208a00;
    let (mut rax, mut fl_l, mut fl_r, mut j_tgt) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x48u64)) as u64; xmm0 = ((crate::simd_ops::ru64(rdx.wrapping_add(0x10u64))) as u64) as u128; xmm1 = (crate::simd_ops::ru128(rdx)) as u128; rdx = P0; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), ((xmm0 as u64)) as u64); rax = (((crate::simd_ops::ru8(rsp.wrapping_add(0x30u64))) as i8 as i64) as u64) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(rax.wrapping_mul(4)).wrapping_add(0x18637ccu64))) as u32 as u64; fl_l = ((rax).wrapping_add(rdx)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; j_tgt = rax; bb = 0xffff_fffe; continue; }
            1 => { rax = ((xmm1 as u32)) as u32 as u64; crate::simd_ops::wu8(rcx, ((rax as u8)) as u8); rax = rcx; crate::simd_ops::wu8(rcx.wrapping_add(0x10u64), (0x0u8) as u8); rsp = ((rsp).wrapping_add(0x48u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            2 => { crate::simd_ops::wu32(rcx, ((xmm1 as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x10u64), (0x1u8) as u8); rax = rcx; rsp = ((rsp).wrapping_add(0x48u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            3 => { crate::simd_ops::wu64(rcx, ((xmm1 as u64)) as u64); crate::simd_ops::wu8(rcx.wrapping_add(0x10u64), (0x2u8) as u8); rax = rcx; rsp = ((rsp).wrapping_add(0x48u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            4 => { crate::simd_ops::wu32(rcx, ((xmm1 as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x10u64), (0x3u8) as u8); rax = rcx; rsp = ((rsp).wrapping_add(0x48u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            5 => { crate::simd_ops::wu64(rcx, ((xmm1 as u64)) as u64); crate::simd_ops::wu8(rcx.wrapping_add(0x10u64), (0x4u8) as u8); rax = rcx; rsp = ((rsp).wrapping_add(0x48u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            6 => { crate::simd_ops::wu128(rcx, (xmm1) as u128); crate::simd_ops::wu8(rcx.wrapping_add(0x10u64), (0x6u8) as u8); rax = rcx; rsp = ((rsp).wrapping_add(0x48u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            7 => { crate::simd_ops::wu64(rcx, ((xmm1 as u64)) as u64); crate::simd_ops::wu8(rcx.wrapping_add(0x10u64), (0xdu8) as u8); rax = rcx; rsp = ((rsp).wrapping_add(0x48u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            8 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, 0x1418637c5, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            0xffff_fffe => { match j_tgt.wrapping_sub(0x141863720u64) { 0x2c => { bb = 1; continue; } 0x3e => { bb = 2; continue; } 0x4e => { bb = 3; continue; } 0x5f => { bb = 4; continue; } 0x6f => { bb = 5; continue; } 0x7f => { bb = 6; continue; } 0x8f => { bb = 7; continue; } 0xa0 => { bb = 8; continue; } _ => { return crate::simd_ops::tail_stk_xmm(j_tgt, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); } } }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1418637f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x140202630; const P1: u64 = 0x140202e00; const P2: u64 = 0x140202480; const P3: u64 = 0x1426cdcb0; const P4: u64 = 0x1430806d8;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 17>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x80u64)) as u64; rdi = rdx; rsi = rcx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rax; rcx = rsp.wrapping_add(0x90); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::lk_add_u32(rcx.wrapping_add(0x8u64), (1) as u32)) as u32 as u64; bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru64(rax)) as u64; rdx = rsp.wrapping_add(0x50); rbx = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; r9 = rsp.wrapping_add(0x60); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; r8 = rsp.wrapping_add(0x70); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdx); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (0x0u64) as u64); rdx = rsi; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), (0x5u64) as u64); crate::simd_ops::wu8(rax, (0x0u8) as u8); rax = P3; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); rax = P4; xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rax); xmm1 = (crate::simd_ops::ru128(rsp.wrapping_add(0x40u64))) as u128; rax = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x50u64), (xmm0) as u128); xmm0 = (crate::simd_ops::ru128(rdi)) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x60u64), (xmm1) as u128); crate::simd_ops::wu128(rsp.wrapping_add(0x70u64), (xmm0) as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 3; continue; }
            3 => { rdi = 0xffffffff; rax = ((rdi as u32)) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(0x8u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 6; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); let xa_src: u32 = (rdi as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(0xcu64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rdi = (xa_old) as u32 as u64; fl_l = ((rdi as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 6; continue; }
            6 => { r11 = rsp.wrapping_add(0x80); rax = rsi; rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x18u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rsp = r11; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141863900(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x140202a90; const P1: u64 = 0x38; const P2: u64 = 0x140202af0; const P3: u64 = 0x18; const P4: u64 = 0x140202b60; const P5: u64 = 0x140202bd0;
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    rcx = ((rcx).wrapping_add(0x48u64)) as u64;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rcx = ((rbx.wrapping_add(P1)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    rcx = ((rbx.wrapping_add(P3)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp);
    rcx = rbx;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(P5, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141863940(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143c48300; const P1: u64 = 0x143c48310; const P2: u64 = 0x143c48318; const P3: u64 = 0x143c48320; const P4: u64 = 0x143c48330; const P5: u64 = 0x143c48338; const P6: u64 = 0x143c48340; const P7: u64 = 0x48; const P8: u64 = 0x141834d64; const P9: u64 = 0x8; const P10: u64 = 0x143c48348; const P11: u64 = 0x143c48350; const P12: u64 = 0x143c48360; const P13: u64 = 0x143c48368; const P14: u64 = 0x1434268d0; const P15: u64 = 0x140203960;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x141863940, funcinfo_rva: 0x36f94d4, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x28, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rcx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = ((P0) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (rdi) as u64); xmm0 = 0u128; crate::simd_ops::wu128(P0, (xmm0) as u128); rbx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(P1, (rbx) as u64); crate::simd_ops::wu64(P2, (rbx) as u64); crate::simd_ops::wu128(P3, (xmm0) as u128); rax = ((P4) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (rax) as u64); crate::simd_ops::wu32(P4, ((xmm0 as u32)) as u32); crate::simd_ops::wu64(P5, (rbx) as u64); crate::simd_ops::wu64(P6, (rbx) as u64); rcx = ((rbx.wrapping_add(P7)) as u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1418639a4); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu64(rax, (rax) as u64); crate::simd_ops::wu64(rax.wrapping_add(P9), (rax) as u64); crate::simd_ops::wu64(P5, (rax) as u64); crate::simd_ops::wu64(P10, (rbx) as u64); xmm0 = 0u128; crate::simd_ops::wu128(P11, (xmm0) as u128); crate::simd_ops::wu64(P12, (0x7u64) as u64); crate::simd_ops::wu64(P13, (0x8u64) as u64); xmm0 = ((crate::simd_ops::ru32(P14)) as u32) as u128; crate::simd_ops::wu32(P4, ((xmm0 as u32)) as u32); r8 = (rax) as u64; rcx = ((P10) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1418639fa); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P15) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = (rdi) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141863a20(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141834d64; const P1: u64 = 0x142670ec8;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rcx = 0x8;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rcx = P1;
    crate::simd_ops::wu64(rax, rcx);
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141863a70(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143080b18; const P1: u64 = 0x8; const P2: u64 = 0x20; const P3: u64 = 0x28; const P4: u64 = 0x140209650; const P5: u64 = 0x38; const P6: u64 = 0x40;
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rax = P0;
    rbx = rcx;
    crate::simd_ops::wu64(rcx, rax);
    let y_addr: u64 = rdx; xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16));
    let y_addr: u64 = rcx.wrapping_add(P1); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi);
    xmm1 = (crate::simd_ops::ru128(rdx.wrapping_add(P2))) as u128;
    crate::simd_ops::wu128(rcx.wrapping_add(P3), (xmm1) as u128);
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    crate::simd_ops::wu64(rbx.wrapping_add(P5), rax);
    rax = rbx;
    crate::simd_ops::wu16(rbx.wrapping_add(P6), (0x0u16) as u16);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141863b50(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1401f4c10; const P1: u64 = 0x141834d30;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdi = rcx;
    rbx = ((rdx as u32)) as u32 as u64;
    rcx = ((rcx).wrapping_add(0x38u64)) as u64;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    fl_l = ((rbx as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64;
    if !(((((fl_l as u8)) & ((fl_r as u8))) == 0)) { rdx = 0x48; rcx = rdi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rax = rdi;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141863c50(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141864fe0; const P1: u64 = 0x1426455a8; const P2: u64 = 0x142645598; const P3: u64 = 0x1426455c0; const P4: u64 = 0x142442de0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rbx = rcx; rcx = ((rcx).wrapping_add(0x8u64)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu8(rbx.wrapping_add(0x41u64), (0x1u8) as u8); rbp = (crate::simd_ops::ru64(rbx.wrapping_add(0x38u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); crate::simd_ops::wu8(rsp.wrapping_add(0x28u64), (0x1u8) as u8); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; crate::simd_ops::wu128(rbp, (xmm0) as u128); rsi = 0x1; let tmp_xchg: u64 = crate::simd_ops::atomic_xchg_u64(rbp.wrapping_add(0x18u64), rsi); rsi = (tmp_xchg) as u64; rax = (((rsi as u8)) as u32) as u32 as u64; rax = (rax & !0xffu64) | (((((rax as u8)).wrapping_shr(1u32)) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = rbp.wrapping_add(0x28); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::lk_add_u32(rbp.wrapping_add(0x30u64), (1) as u32); rcx = rbp.wrapping_add(0x28); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rbp.wrapping_add(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 2; continue; }
            2 => { fl_l = (((rsi) & (0xfffffffffffffffcu64))) as u64; fl_r = fl_l; rsi = ((fl_l as u64)) as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::lk_add_u32(rbp.wrapping_add(0x10u64), (1) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rsi); rax = (crate::simd_ops::ru64(rsi)) as u64; rdx = rbp; rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = 0xffffffff; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbp.wrapping_add(0x10u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 7; continue; } bb = 4; continue; }
            4 => { rbx = (crate::simd_ops::ru64(rbp.wrapping_add(0x18u64))) as u64; fl_l = (((rbx) & (0xfffffffffffffffcu64))) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 6; continue; }
            6 => { rcx = rbp; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru64(rsi)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 8; continue; }
            8 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141863da0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x1426460e0; const P2: u64 = 0x10; const P3: u64 = 0x1401f3ee0;
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp);
    rcx = ((rbx.wrapping_add(P2)) as u64) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(P3, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141863dd0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x1426460f0;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rdx = 0xfffffff0;
    rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp);
    rax = (((rax as u32)).wrapping_shr(30u32)) as u32 as u64;
    rax = (rax & !0xffu64) | (((!((rax as u8))) as u8 as u64));
    rax = (rax & !0xffu64) | ((((((rax as u8)) & (0x1u8))) as u8 as u64));
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141863e00(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142442de0; const P1: u64 = 0x1402090f0; const P2: u64 = 0x1425371a0; const P3: u64 = 0x142533200;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = rdx; rdi = rcx; rbx = (crate::simd_ops::ru64(rcx.wrapping_add(0x58u64))) as u64; crate::simd_ops::wu64(rcx.wrapping_add(0x58u64), (0x0u64) as u64); fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x38u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            2 => { fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            4 => { r9 = rdi.wrapping_add(0x8); r8 = rsp.wrapping_add(0x38); rcx = rsp.wrapping_add(0x30); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rax)) as u64; crate::simd_ops::wu64(rax, (0x0u64) as u64); rbx = (crate::simd_ops::ru64(rdi.wrapping_add(0x58u64))) as u64; crate::simd_ops::wu64(rdi.wrapping_add(0x58u64), rcx); fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x38u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); r10 = rbx.wrapping_sub(0x1); r10 = (((r10) & (0xfffffffffe000000u64))) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(r10.wrapping_add(0x100u64))) as u64; rdx = (rdx & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); rax = rbx; rax = ((rax).wrapping_sub(r10)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rcx = r10.wrapping_add(0x108); rcx = ((rcx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x4u64))) as u32 as u64; rcx = ((rcx).wrapping_sub(rax)) as u64; fl_l = ((rdx as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 10; continue; } bb = 6; continue; }
            6 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0xeu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 9; continue; } bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(rbx, rax); crate::simd_ops::wu64(rcx.wrapping_add(0x20u64), rbx); fl_l = (crate::simd_ops::lk_add_u32(rcx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 12; continue; } bb = 8; continue; }
            8 => { rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 12; continue; }
            9 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 11; continue; }
            10 => { r8 = 0; fl_l = 0; fl_r = 0; bb = 11; continue; }
            11 => { rdx = rcx; r9 = rbx; rcx = r10; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 12; continue; }
            12 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 13; continue; }
            13 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x38u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); bb = 14; continue; }
            14 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x58u64))) as u64; rax = ((crate::simd_ops::ru8(rax.wrapping_add(0x18u64))) as u32) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141863f50(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14186a670; const P1: u64 = 0x142646230; const P2: u64 = 0x20; const P3: u64 = 0x30; const P4: u64 = 0x100; const P5: u64 = 0x108; const P6: u64 = 0x4; const P7: u64 = 0xe; const P8: u64 = 0x18; const P9: u64 = 0x1425371a0; const P10: u64 = 0x142533200;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rbx = rdx; rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); r9 = rsp.wrapping_add(0x20); r8 = P0; rdx = 0; fl_l = 0; fl_r = 0; rcx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r10 = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; let y_addr: u64 = r10; xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rbx; crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); xmm1 = (crate::simd_ops::ru128(r10.wrapping_add(P2))) as u128; crate::simd_ops::wu128(rbx.wrapping_add(P2), (xmm1) as u128); crate::simd_ops::wu8(rbx.wrapping_add(P3), (0x1u8) as u8); fl_l = r10; fl_r = r10; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 1; continue; }
            1 => { r10 = ((r10).wrapping_sub(1)) as u64; r10 = (((r10) & (0xfffffffffe000000u64))) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(r10.wrapping_add(P4))) as u64; rcx = (rcx & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rax = r9; rax = ((rax).wrapping_sub(r10)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rdx = ((r10.wrapping_add(P5)) as u64) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(P6))) as u32 as u64; rdx = ((rdx).wrapping_sub(rax)) as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 6; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(P7))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 5; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(P2))) as u64; crate::simd_ops::wu64(r9, rax); crate::simd_ops::wu64(rdx.wrapping_add(P2), r9); fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(P8), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 8; continue; } bb = 4; continue; }
            4 => { rcx = rdx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 8; continue; }
            5 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 7; continue; }
            6 => { r8 = 0; fl_l = 0; fl_r = 0; bb = 7; continue; }
            7 => { rcx = r10; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P10, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 8; continue; }
            8 => { rax = rbx; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141864040(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14186aad0; const P1: u64 = 0x14186a670; const P2: u64 = 0x142646230; const P3: u64 = 0x1425371a0; const P4: u64 = 0x142533200;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut ymm1_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 21>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0xa0u64)) as u64; rbx = rdx; rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0xf8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rax; rcx = rsp.wrapping_add(0x60); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x30u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 2; continue; } bb = 1; continue; }
            1 => { let y_addr: u64 = rax; xmm1 = crate::simd_ops::ru128(y_addr); ymm1_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); xmm2 = (crate::simd_ops::ru128(rax.wrapping_add(0x20u64))) as u128; bb = 11; continue; }
            2 => { rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); r9 = rsp.wrapping_add(0x20); r8 = P1; rdx = 0; fl_l = 0; fl_r = 0; rcx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r10 = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; let y_addr: u64 = r10; xmm1 = crate::simd_ops::ru128(y_addr); ymm1_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rsp.wrapping_add(0x40u64); crate::simd_ops::wu128(y_addr, xmm1); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm1_hi); xmm2 = (crate::simd_ops::ru128(r10.wrapping_add(0x20u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm2) as u128); fl_l = r10; fl_r = r10; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 3; continue; }
            3 => { r10 = ((r10).wrapping_sub(1)) as u64; r10 = (((r10) & (0xfffffffffe000000u64))) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(r10.wrapping_add(0x100u64))) as u64; rcx = (rcx & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rax = r9; rax = ((rax).wrapping_sub(r10)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rdx = r10.wrapping_add(0x108); rdx = ((rdx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32 as u64; rdx = ((rdx).wrapping_sub(rax)) as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 8; continue; } bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0xeu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 7; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(r9, rax); crate::simd_ops::wu64(rdx.wrapping_add(0x20u64), r9); fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 11; continue; } bb = 6; continue; }
            6 => { rcx = rdx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 10; continue; }
            7 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 9; continue; }
            8 => { r8 = 0; fl_l = 0; fl_r = 0; bb = 9; continue; }
            9 => { rcx = r10; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 10; continue; }
            10 => { let y_addr: u64 = rsp.wrapping_add(0x40u64); xmm1 = crate::simd_ops::ru128(y_addr); ymm1_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); xmm2 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; bb = 11; continue; }
            11 => { let y_addr: u64 = rbx; crate::simd_ops::wu128(y_addr, xmm1); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm1_hi); crate::simd_ops::wu128(rbx.wrapping_add(0x20u64), (xmm2) as u128); rax = rbx; rsp = ((rsp).wrapping_add(0xa0u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141864170(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x1426460d0; const P2: u64 = 0x14186a450; const P3: u64 = 0x142646230;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x68u64)) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rdx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { r9 = rsp.wrapping_add(0x20); crate::simd_ops::wu8(rsp.wrapping_add(0x50u64), (0x0u8) as u8); r8 = P2; rdx = 0; fl_l = 0; fl_r = 0; rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; bb = 3; continue; }
            3 => { rsp = ((rsp).wrapping_add(0x68u64)) as u64; return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1418641c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142646070; const P1: u64 = 0x8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp);
    fl_l = rax; fl_r = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64;
    rax = (rax & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64));
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141864210(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x142646090;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = ((rsp).wrapping_sub(0x58u64)) as u64;
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rdx = rsp.wrapping_add(0x20);
    xmm0 = 0u128; ymm0_hi = 0;
    let y_addr: u64 = rsp.wrapping_add(0x24u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi);
    crate::simd_ops::wu64(rsp.wrapping_add(0x44u64), (0x0u64) as u64);
    crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x2cu32) as u32);
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    fl_l = (crate::simd_ops::ru32(rsp.wrapping_add(0x28u64))) as u32 as u64; fl_r = 0x3;
    rax = (rax & !0xffu64) | (((if (((fl_l as u32)) == ((fl_r as u32))) { 1u8 } else { 0u8 }) as u8 as u64));
    rsp = ((rsp).wrapping_add(0x58u64)) as u64;
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141864260(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x44;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    r8 = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64;
    rax = ((r8 as u32)) as u32 as u64;
    rax = (((rax as u32)).wrapping_shr(9u32)) as u32 as u64;
    rax = (rax & !0xffu64) | ((((((rax as u8)) & (0x1u8))) as u8 as u64));
    fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64;
    if (((fl_l as u8)) == ((fl_r as u8))) { return rax; }
    fl_l = ((rdx as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64;
    if !(((((fl_l as u8)) & ((fl_r as u8))) == 0)) { fl_l = ((((r8 as u32)) >> (9u32)) & 1) as u64; r8 = (((r8 as u32)) | ((1u32) << (9u32))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(P0), ((r8 as u32)) as u32); return rax; }
    fl_l = ((((r8 as u32)) >> (9u32)) & 1) as u64;
    r8 = (((r8 as u32)) & !((1u32) << (9u32))) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(P0), ((r8 as u32)) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1418642a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x44;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    r8 = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64;
    rax = ((r8 as u32)) as u32 as u64;
    rax = (((rax as u32)).wrapping_shr(6u32)) as u32 as u64;
    rax = (rax & !0xffu64) | ((((((rax as u8)) & (0x1u8))) as u8 as u64));
    fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64;
    if (((fl_l as u8)) == ((fl_r as u8))) { return rax; }
    fl_l = ((rdx as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64;
    if !(((((fl_l as u8)) & ((fl_r as u8))) == 0)) { fl_l = ((((r8 as u32)) | (0x40u32))) as u32 as u64; fl_r = fl_l; r8 = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(P0), ((r8 as u32)) as u32); return rax; }
    fl_l = ((((r8 as u32)) & (0xffffffbfu32))) as u32 as u64; fl_r = fl_l;
    r8 = ((fl_l as u32)) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(P0), ((r8 as u32)) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1418642d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x44; const P1: u64 = 0x8; const P2: u64 = 0x1426460d8;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x48u64)) as u64;
    r8 = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64;
    rax = ((r8 as u32)) as u32 as u64;
    rax = (((rax as u32)).wrapping_shr(5u32)) as u32 as u64;
    rax = (rax & !0xffu64) | ((((((rax as u8)) & (0x1u8))) as u8 as u64));
    fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64;
    if (((fl_l as u8)) == ((fl_r as u8))) { rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; rax = 0; fl_l = 0; fl_r = 0; rdx = (((rdx as u8)) as u32) as u32 as u64; r9 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), (0x43u32) as u32); rdx = ((rdx).wrapping_sub(0x2u64)) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rax as u32)) as u32); r8 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rax as u32)) as u32); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); rsp = ((rsp).wrapping_add(0x48u64)) as u64; return rax; }
    fl_l = ((rdx as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64;
    if !(((((fl_l as u8)) & ((fl_r as u8))) == 0)) { fl_l = ((((r8 as u32)) | (0x20u32))) as u32 as u64; fl_r = fl_l; r8 = ((fl_l as u32)) as u32 as u64; } else { fl_l = ((((r8 as u32)) & (0xffffffdfu32))) as u32 as u64; fl_r = fl_l; r8 = ((fl_l as u32)) as u32 as u64; }
    crate::simd_ops::wu32(rcx.wrapping_add(P0), ((r8 as u32)) as u32);
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64;
    rax = 0;
    fl_l = 0; fl_r = 0;
    rdx = (((rdx as u8)) as u32) as u32 as u64;
    r9 = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), (0x43u32) as u32);
    rdx = ((rdx).wrapping_sub(0x2u64)) as u64;
    crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rax as u32)) as u32);
    r8 = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rax as u32)) as u32);
    rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x48u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141864330(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x44; const P1: u64 = 0x141864d30;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    r8 = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64;
    rax = ((r8 as u32)) as u32 as u64;
    rax = (((rax as u32)).wrapping_shr(4u32)) as u32 as u64;
    rax = (rax & !0xffu64) | ((((((rax as u8)) & (0x1u8))) as u8 as u64));
    fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64;
    if !((((fl_l as u8)) == ((fl_r as u8)))) { rax = ((r8 as u32)) as u32 as u64; r8 = ((((r8 as u32)) | (0x10u32))) as u32 as u64; rax = ((((rax as u32)) & (0xffffffefu32))) as u32 as u64; fl_l = ((rdx as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; r8 = (if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { (rax as u32) } else { (r8 as u32) }) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(P0), ((r8 as u32)) as u32); return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141864360(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x44; const P1: u64 = 0x141864d30;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    r8 = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64;
    rax = ((r8 as u32)) as u32 as u64;
    rax = (((rax as u32)).wrapping_shr(3u32)) as u32 as u64;
    rax = (rax & !0xffu64) | ((((((rax as u8)) & (0x1u8))) as u8 as u64));
    fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64;
    if !((((fl_l as u8)) == ((fl_r as u8)))) { rax = ((r8 as u32)) as u32 as u64; r8 = ((((r8 as u32)) | (0x8u32))) as u32 as u64; rax = ((((rax as u32)) & (0xfffffff7u32))) as u32 as u64; fl_l = ((rdx as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; r8 = (if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { (rax as u32) } else { (r8 as u32) }) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(P0), ((r8 as u32)) as u32); return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141864390(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x44; const P1: u64 = 0x141864d30;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    r8 = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64;
    rax = ((r8 as u32)) as u32 as u64;
    rax = (((rax as u32)).wrapping_shr(2u32)) as u32 as u64;
    rax = (rax & !0xffu64) | ((((((rax as u8)) & (0x1u8))) as u8 as u64));
    fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64;
    if !((((fl_l as u8)) == ((fl_r as u8)))) { rax = ((r8 as u32)) as u32 as u64; r8 = ((((r8 as u32)) | (0x4u32))) as u32 as u64; rax = ((((rax as u32)) & (0xfffffffbu32))) as u32 as u64; fl_l = ((rdx as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; r8 = (if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { (rax as u32) } else { (r8 as u32) }) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(P0), ((r8 as u32)) as u32); return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1418643c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x44; const P1: u64 = 0x8; const P2: u64 = 0x142646088;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    r8 = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64;
    rax = ((r8 as u32)) as u32 as u64;
    rax = (((rax as u32)).wrapping_shr(1u32)) as u32 as u64;
    rax = (rax & !0xffu64) | ((((((rax as u8)) & (0x1u8))) as u8 as u64));
    fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64;
    if !((((fl_l as u8)) == ((fl_r as u8)))) { rax = ((r8 as u32)) as u32 as u64; r8 = ((((r8 as u32)) | (0x2u32))) as u32 as u64; rax = ((((rax as u32)) & (0xfffffffdu32))) as u32 as u64; fl_l = ((rdx as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; rdx = (((rdx as u8)) as u32) as u32 as u64; r8 = (if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { (rax as u32) } else { (r8 as u32) }) as u32 as u64; rdx = ((((rdx as u32)) ^ (0x1u32))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(P0), ((r8 as u32)) as u32); rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; fl_l = (((rdx as u32)).wrapping_shl(2u32)) as u32 as u64; rdx = ((fl_l as u32)) as u32 as u64; return crate::simd_ops::call_fwd(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141864400(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1426460f0; const P1: u64 = 0x141864d30; const P2: u64 = 0x1426460d8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 15>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x70u64)) as u64; rdi = (((rdx as u8)) as u32) as u32 as u64; rbx = rcx; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; rdx = 0xfffffff0; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); rax = (((rax as u32)).wrapping_shr(30u32)) as u32 as u64; rax = (rax & !0xffu64) | (((!((rax as u8))) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x88u64))) as u64; rsp = ((rsp).wrapping_add(0x70u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            2 => { rax = ((crate::simd_ops::ru8(rbx.wrapping_add(0x44u64))) as u32) as u32 as u64; rax = (rax & !0xffu64) | ((((((rax as u8)) & (0x1u8))) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rdi as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 7; continue; } bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x44u64))) as u32 as u64; rax = ((rcx as u32)) as u32 as u64; rcx = ((((rcx as u32)) | (0x1u32))) as u32 as u64; rax = ((((rax as u32)) & (0xfffffffeu32))) as u32 as u64; fl_l = ((rdi as u8)) as u8 as u64; fl_r = ((rdi as u8)) as u8 as u64; rcx = (if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { (rax as u32) } else { (rcx as u32) }) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x44u64), ((rcx as u32)) as u32); rcx = rbx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = rsp.wrapping_add(0x40); rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x100u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rdi as u8)) as u8 as u64; fl_r = ((rdi as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x4cu64))) as u32 as u64; r9 = (crate::simd_ops::ru32(rsp.wrapping_add(0x44u64))) as u32 as u64; r8 = (crate::simd_ops::ru32(rsp.wrapping_add(0x40u64))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), (0x20u32) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x48u64))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rax as u32)) as u32); bb = 6; continue; }
            5 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = rsp.wrapping_add(0x80); rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0xa0u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x4cu64))) as u32 as u64; rax = (((rax as u32)).wrapping_sub(crate::simd_ops::ru32(rsp.wrapping_add(0x84u64)))) as u32 as u64; rdx = (if (rax as u32 as i32) < 0 { 0xffffffffu32 } else { 0u32 }) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), (0x21u32) as u32); rax = (((rax as u32)).wrapping_sub((rdx as u32))) as u32 as u64; rax = (((((rax as u32)) as i32).wrapping_shr(1u32) as u32)) as u32 as u64; r9 = ((rax as u32)) as u32 as u64; rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x48u64))) as u32 as u64; rax = (((rax as u32)).wrapping_sub(crate::simd_ops::ru32(rsp.wrapping_add(0x80u64)))) as u32 as u64; r9 = (((r9 as u32)).wrapping_add(crate::simd_ops::ru32(rsp.wrapping_add(0x44u64)))) as u32 as u64; rdx = (if (rax as u32 as i32) < 0 { 0xffffffffu32 } else { 0u32 }) as u64; rax = (((rax as u32)).wrapping_sub((rdx as u32))) as u32 as u64; rax = (((((rax as u32)) as i32).wrapping_shr(1u32) as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_add(crate::simd_ops::ru32(rsp.wrapping_add(0x40u64)))) as u32 as u64; rcx = 0; fl_l = 0; fl_r = 0; r8 = ((rax as u32)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rcx as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rcx as u32)) as u32); bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rdx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); bb = 7; continue; }
            7 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x88u64))) as u64; rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); rsp = ((rsp).wrapping_add(0x70u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141864520(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1426460d0; const P1: u64 = 0x1426460e8; const P2: u64 = 0x142646078;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 17>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x80u64)) as u64; rbx = rdx; rdi = rcx; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; rdx = 0; fl_l = 0; fl_r = 0; r14 = r8; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rsi = rax; crate::simd_ops::wu8(r14, (0x0u8) as u8); fl_l = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rdi.wrapping_add(0x10u64), rax); crate::simd_ops::wu8(r14, (0x1u8) as u8); bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0x20u64))) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 8; continue; } bb = 3; continue; }
            3 => { r9 = (crate::simd_ops::ru32(rdi.wrapping_add(0x1cu64))) as u32 as u64; rcx = rsp.wrapping_add(0x20); rdx = (crate::simd_ops::ru32(rdi.wrapping_add(0x18u64))) as u32 as u64; r8 = 0; fl_l = 0; fl_r = 0; xmm0 = 0u128; ymm0_hi = 0; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = (crate::simd_ops::ru32(rsp.wrapping_add(0x28u64))) as u32 as u64; r8 = (((r8 as u32)).wrapping_sub(crate::simd_ops::ru32(rsp.wrapping_add(0x20u64)))) as u32 as u64; rdx = (crate::simd_ops::ru32(rsp.wrapping_add(0x2cu64))) as u32 as u64; rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x18u64))) as u32 as u64; rdx = (((rdx as u32)).wrapping_sub(crate::simd_ops::ru32(rsp.wrapping_add(0x24u64)))) as u32 as u64; rcx = (((rcx as u32)).wrapping_sub((r8 as u32))) as u32 as u64; rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x20u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 5; continue; } bb = 4; continue; }
            4 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rdi.wrapping_add(0x28u64))) as u32 as u64; rcx = (if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { crate::simd_ops::ru32(rdi.wrapping_add(0x28u64)) } else { (rcx as u32) }) as u32 as u64; rax = ((rcx as u32)) as u32 as u64; bb = 5; continue; }
            5 => { rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x1cu64))) as u32 as u64; rax = (((rax as u32)).wrapping_add((r8 as u32))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x18u64), ((rax as u32)) as u32); rcx = (((rcx as u32)).wrapping_sub((rdx as u32))) as u32 as u64; rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x24u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 7; continue; } bb = 6; continue; }
            6 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rdi.wrapping_add(0x2cu64))) as u32 as u64; rcx = (if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { crate::simd_ops::ru32(rdi.wrapping_add(0x2cu64)) } else { (rcx as u32) }) as u32 as u64; rax = ((rcx as u32)) as u32 as u64; bb = 7; continue; }
            7 => { fl_l = (((rax as u32)).wrapping_add((rdx as u32))) as u32 as u64; fl_r = fl_l; rax = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x1cu64), ((rax as u32)) as u32); bb = 8; continue; }
            8 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0x20u64))) as u8 as u64; fl_r = (0x2u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 12; continue; } bb = 9; continue; }
            9 => { fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(0x44u64))) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 12; continue; } bb = 10; continue; }
            10 => { xmm0 = 0u128; ymm0_hi = 0; let y_addr: u64 = rsp.wrapping_add(0x30u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), (0x48u32) as u32); let y_addr: u64 = rsp.wrapping_add(0x50u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), rax); rdx = rsp.wrapping_add(0x30); rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 12; continue; } bb = 11; continue; }
            11 => { rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x34u64))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x10u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x38u64))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x14u64), ((rax as u32)) as u32); bb = 12; continue; }
            12 => { r11 = rsp.wrapping_add(0x80); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x10u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x18u64))) as u64; rdi = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rsp = r11; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141864640(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x44; const P1: u64 = 0x8; const P2: u64 = 0x142646068;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64;
    rax = (((rax as u32)).wrapping_shr(9u32)) as u32 as u64;
    fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64;
    if !(((((fl_l as u8)) & ((fl_r as u8))) == 0)) { rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; return crate::simd_ops::call_fwd(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141864660(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x30; const P1: u64 = 0x34; const P2: u64 = 0x3c; const P3: u64 = 0x38; const P4: u64 = 0x40; const P5: u64 = 0x8;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(P0))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 4; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64; fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(P2))) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru32(rcx.wrapping_add(P3))) as u32 as u64; fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(P4))) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64; crate::simd_ops::wu64(rdx, rax); rax = rdx; crate::simd_ops::wu8(rdx.wrapping_add(P5), (0x1u8) as u8); crate::simd_ops::wu8(rcx.wrapping_add(P0), (0x0u8) as u8); return rax; }
            4 => { crate::simd_ops::wu8(rdx.wrapping_add(P5), (0x0u8) as u8); rax = rdx; crate::simd_ops::wu8(rcx.wrapping_add(P0), (0x0u8) as u8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1418646a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x44; const P1: u64 = 0x3c;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(P0))) as u8 as u64; fl_r = (0xcu8) as u8 as u64;
    if !(((((fl_l as u8)) & ((fl_r as u8))) == 0)) { crate::simd_ops::wu64(rcx.wrapping_add(P1), rdx); }
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1418646c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x30; const P1: u64 = 0xa0; const P2: u64 = 0x34; const P3: u64 = 0x3c;
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    rdx = rsp.wrapping_add(0x30);
    rbx = rcx;
    crate::simd_ops::wu8(rcx.wrapping_add(P0), (0x1u8) as u8);
    rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp);
    rdx = (crate::simd_ops::ru64(rax)) as u64;
    crate::simd_ops::wu64(rbx.wrapping_add(P2), rdx);
    crate::simd_ops::wu64(rbx.wrapping_add(P3), rdx);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141864700(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x142646088;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rdx = 0x3;
    return crate::simd_ops::call_fwd(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141864720(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x142646088;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rdx = 0x6;
    return crate::simd_ops::call_fwd(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141864740(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x1426461b0; const P2: u64 = 0x142646068;
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp);
    rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141864770(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x142646080; const P2: u64 = 0xa0; const P3: u64 = 0x142646208;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    rdx = rsp.wrapping_add(0x30);
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax);
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rax = (crate::simd_ops::ru64(rbx)) as u64;
    rdx = rsp.wrapping_add(0x40);
    rcx = rbx;
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    xmm0 = 0u128;
    xmm0 = (xmm0 & !0xffff_ffffu128) | ((((crate::simd_ops::ru32(rsp.wrapping_add(0x44u64))) as i32) as f32).to_bits() as u128);
    xmm2 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(crate::simd_ops::ru32(rsp.wrapping_add(0x3cu64)))).to_bits() as u128);
    xmm1 = 0u128;
    xmm1 = (xmm1 & !0xffff_ffffu128) | ((((crate::simd_ops::ru32(rsp.wrapping_add(0x34u64))) as i32) as f32).to_bits() as u128);
    xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) + f32::from_bits(xmm1 as u32)).to_bits() as u128);
    xmm0 = 0u128;
    xmm0 = (xmm0 & !0xffff_ffffu128) | ((((crate::simd_ops::ru32(rsp.wrapping_add(0x40u64))) as i32) as f32).to_bits() as u128);
    xmm3 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(crate::simd_ops::ru32(rsp.wrapping_add(0x38u64)))).to_bits() as u128);
    xmm1 = 0u128;
    xmm1 = (xmm1 & !0xffff_ffffu128) | ((((crate::simd_ops::ru32(rsp.wrapping_add(0x30u64))) as i32) as f32).to_bits() as u128);
    rdx = (crate::simd_ops::cvt_f32_i32((f32::from_bits(xmm2 as u32)))) as u32 as u64;
    xmm2 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) + f32::from_bits(xmm1 as u32)).to_bits() as u128);
    rcx = (crate::simd_ops::cvt_f32_i32((f32::from_bits(xmm2 as u32)))) as u32 as u64;
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141864800(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x50; const P1: u64 = 0x14397d820; const P2: u64 = 0x1418682d0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; fl_r = rdx;
    if !((((fl_l as u64)) == ((fl_r as u64)))) { crate::simd_ops::wu64(rcx.wrapping_add(P0), rdx); rcx = (crate::simd_ops::ru64(P1)) as u64; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141864830(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x142646120; const P2: u64 = 0x4;
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x30u64)) as u64;
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rbx = rdx;
    rdx = rsp.wrapping_add(0x20);
    rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp);
    rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x28u64))) as u32 as u64;
    rax = (((rax as u32)).wrapping_sub(crate::simd_ops::ru32(rsp.wrapping_add(0x20u64)))) as u32 as u64;
    crate::simd_ops::wu32(rbx, ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x2cu64))) as u32 as u64;
    rax = (((rax as u32)).wrapping_sub(crate::simd_ops::ru32(rsp.wrapping_add(0x24u64)))) as u32 as u64;
    crate::simd_ops::wu32(rbx.wrapping_add(P2), ((rax as u32)) as u32);
    rax = rbx;
    rsp = ((rsp).wrapping_add(0x30u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141864870(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x142646080; const P2: u64 = 0x4;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rbx = rdx;
    rax = 0;
    fl_l = 0; fl_r = 0;
    rdx = rsp.wrapping_add(0x30);
    crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax);
    rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp);
    rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x30u64))) as u32 as u64;
    crate::simd_ops::wu32(rbx, ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x34u64))) as u32 as u64;
    crate::simd_ops::wu32(rbx.wrapping_add(P2), ((rax as u32)) as u32);
    rax = rbx;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1418648b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14183acc2; const P1: u64 = 0x8; const P2: u64 = 0x142646238; const P3: u64 = 0x3; const P4: u64 = 0x4; const P5: u64 = 0x1423fe420;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<512, 265>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x840u64)) as u64; rdi = rdx; rbx = rcx; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x0u32) as u32); rdx = 0; fl_l = 0; fl_r = 0; r8 = 0x800; rcx = rsp.wrapping_add(0x40); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = 0x400; rdx = rsp.wrapping_add(0x40); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru32(rdi)) as u32 as u64; rax = ((((rax as u32)) & (0xff000010u32))) as u32 as u64; fl_l = ((((rax as u32)) | (0x10u32))) as u32 as u64; fl_r = fl_l; rax = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(rdi, ((rax as u32)) as u32); crate::simd_ops::wu8(rdi.wrapping_add(P3), (0x10u8) as u8); crate::simd_ops::wu32(rdi.wrapping_add(P4), (0x0u32) as u32); crate::simd_ops::wu8(rdi.wrapping_add(P1), (0x0u8) as u8); crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x1u32) as u32); rax = rsp.wrapping_add(0x40); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); rcx = rsp.wrapping_add(0x40); rax = (0xffffffffffffffffu64) as u64; bb = 1; continue; }
            1 => { rax = ((rax).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru16(rcx.wrapping_add(rax.wrapping_mul(2)))) as u16 as u64; fl_r = (0x0u16) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 1; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rax); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128); rdx = rsp.wrapping_add(0x30); rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = rdi; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x850u64))) as u64; rsp = ((rsp).wrapping_add(0x840u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141864980(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x20;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = (crate::simd_ops::ru64(rdx)) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(P0), rax);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141864990(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1426460f0; const P1: u64 = 0x142646088; const P2: u64 = 0x1426460e8; const P3: u64 = 0x1426460d8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = ((rsp).wrapping_sub(0x60u64)) as u64; rsi = rdx; rbx = rcx; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; rdx = 0xfffffff0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (((rax as u32)).wrapping_shr(30u32)) as u32 as u64; rax = (rax & !0xffu64) | (((!((rax as u8))) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; rsp = ((rsp).wrapping_add(0x60u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            2 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = rsp.wrapping_add(0x70); rcx = rbx; crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), rdi); (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0xa0u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru32(rsi)) as u32 as u64; fl_l = (crate::simd_ops::ru32(rax)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 5; continue; } bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru32(rsi.wrapping_add(0x4u64))) as u32 as u64; fl_l = (crate::simd_ops::ru32(rax.wrapping_add(0x4u64))) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; rsp = ((rsp).wrapping_add(0x60u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            5 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0xe0u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rdx = 0x9; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 7; continue; }
            7 => { r9 = (crate::simd_ops::ru32(rbx.wrapping_add(0x1cu64))) as u32 as u64; rcx = rsp.wrapping_add(0x40); rdx = (crate::simd_ops::ru32(rbx.wrapping_add(0x18u64))) as u32 as u64; rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rax); r8 = 0; fl_l = 0; fl_r = 0; rax = (crate::simd_ops::ru32(rsi)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x48u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x4u64))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x4cu64), ((rax as u32)) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm0 = (crate::simd_ops::ru128(rbx.wrapping_add(0x20u64))) as u128; rax = (crate::simd_ops::ru64(rsi)) as u64; r9 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu128(rsp.wrapping_add(0x50u64), (xmm0) as u128); crate::simd_ops::wu64(rbx.wrapping_add(0x20u64), rax); r8 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rbx.wrapping_add(0x28u64), rax); rdx = 0; fl_l = 0; fl_r = 0; rcx = (crate::simd_ops::ru32(rsp.wrapping_add(0x4cu64))) as u32 as u64; rcx = (((rcx as u32)).wrapping_sub(crate::simd_ops::ru32(rsp.wrapping_add(0x44u64)))) as u32 as u64; rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x48u64))) as u32 as u64; rax = (((rax as u32)).wrapping_sub(crate::simd_ops::ru32(rsp.wrapping_add(0x40u64)))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), (0x6u32) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rcx as u32)) as u32); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rax as u32)) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x50u64))) as u128; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; crate::simd_ops::wu128(rbx.wrapping_add(0x20u64), (xmm0) as u128); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; rax = (rax & !0xffu64) | (((if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { 1u8 } else { 0u8 }) as u8 as u64)); rsp = ((rsp).wrapping_add(0x60u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141864ac0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x1426460f0; const P2: u64 = 0x1c; const P3: u64 = 0x18; const P4: u64 = 0x140209310; const P5: u64 = 0x1426460d8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 19>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x90u64)) as u64; rdi = rdx; rbx = rcx; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rdx = 0xfffffff0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (((rax as u32)).wrapping_shr(30u32)) as u32 as u64; rax = (rax & !0xffu64) | (((!((rax as u8))) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0xa8u64))) as u64; rsp = ((rsp).wrapping_add(0x90u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            2 => { rcx = (crate::simd_ops::ru32(rbx.wrapping_add(P2))) as u32 as u64; rdx = rsp.wrapping_add(0x40); rax = (crate::simd_ops::ru32(rbx.wrapping_add(P3))) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rdx); rdx = rsp.wrapping_add(0x40); crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rdx); rdx = rsp.wrapping_add(0x40); crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), rdx); rdx = rdi; xmm0 = 0u128; crate::simd_ops::wu32(rsp.wrapping_add(0x5cu64), ((rcx as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x74u64), ((rcx as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x84u64), ((rcx as u32)) as u32); rcx = rsp.wrapping_add(0x50); crate::simd_ops::wu64(rsp.wrapping_add(0xa0u64), rsi); rsi = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm0) as u128); crate::simd_ops::wu32(rsp.wrapping_add(0x58u64), ((rax as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), rsi); crate::simd_ops::wu32(rsp.wrapping_add(0x70u64), ((rax as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x80u64), ((rax as u32)) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64; r9 = r8; crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), (0x5u32) as u32); r9 = ((r9).wrapping_shr(32u32)) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rsi as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rsi as u32)) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0xa0u64))) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0xa8u64))) as u64; rax = (rax & !0xffu64) | (((if (((fl_l as u32)) == ((fl_r as u32))) { 1u8 } else { 0u8 }) as u8 as u64)); rsp = ((rsp).wrapping_add(0x90u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141864bc0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1423fe080; const P1: u64 = 0x142646098; const P2: u64 = 0x1425371a0; const P3: u64 = 0x142533200;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 75>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r11 = rsp; crate::simd_ops::wu64(r11.wrapping_add(0x10u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x250u64)) as u64; rbx = rcx; rdi = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(r11.wrapping_add(0x8u64), ((rdi as u32)) as u32); rax = rsp.wrapping_add(0x50); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), (0x100u32) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x3cu64), (0x100u64) as u64); rax = rsp.wrapping_add(0x50); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rax); crate::simd_ops::wu16(rsp.wrapping_add(0x50u64), ((rdi as u16)) as u16); crate::simd_ops::wu32(r11.wrapping_add(0x8u64), (0x1u32) as u32); xmm0 = (crate::simd_ops::ru128(rdx)) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); rdx = rsp.wrapping_add(0x20); rcx = rsp.wrapping_add(0x30); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r10 = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; fl_l = r10; fl_r = r10; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 1; continue; }
            1 => { fl_l = r10; fl_r = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 9; continue; } bb = 2; continue; }
            2 => { r10 = ((r10).wrapping_sub(1)) as u64; r10 = (((r10) & (0xfffffffffe000000u64))) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(r10.wrapping_add(0x100u64))) as u64; rcx = (rcx & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = r9; rax = ((rax).wrapping_sub(r10)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rdx = r10.wrapping_add(0x108); rdx = ((rdx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32 as u64; rdx = ((rdx).wrapping_sub(rax)) as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 7; continue; } bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0xeu64))) as u8 as u64; fl_r = ((rdi as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 6; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(r9, rax); crate::simd_ops::wu64(rdx.wrapping_add(0x20u64), r9); fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 9; continue; } bb = 5; continue; }
            5 => { rcx = rdx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 9; continue; }
            6 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 8; continue; }
            7 => { r8 = 0; fl_l = 0; fl_r = 0; bb = 8; continue; }
            8 => { rcx = r10; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { crate::simd_ops::wu16(rax, ((rdi as u16)) as u16); bb = 11; continue; }
            11 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x268u64))) as u64; rsp = ((rsp).wrapping_add(0x250u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141864cf0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x44;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    r9 = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64;
    fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64;
    rax = (rax & !0xffu64) | (((if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { 1u8 } else { 0u8 }) as u8 as u64));
    fl_l = ((rax as u8)) as u8 as u64; fl_r = ((r8 as u8)) as u8 as u64;
    if !((((fl_l as u8)) != ((fl_r as u8)))) { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; return rax; }
    fl_l = ((r8 as u8)) as u8 as u64; fl_r = ((r8 as u8)) as u8 as u64;
    if !(((((fl_l as u8)) & ((fl_r as u8))) == 0)) { fl_l = ((((r9 as u32)) | ((rdx as u32)))) as u32 as u64; fl_r = fl_l; r9 = ((fl_l as u32)) as u32 as u64; rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); crate::simd_ops::wu32(rcx.wrapping_add(P0), ((r9 as u32)) as u32); return rax; }
    fl_l = ((!((rdx as u32))) & ((r9 as u32))) as u32 as u64; fl_r = fl_l;
    r9 = ((fl_l as u32)) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(P0), ((r9 as u32)) as u32);
    rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64));
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141864d30(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x44; const P1: u64 = 0x18; const P2: u64 = 0x1c; const P3: u64 = 0x8; const P4: u64 = 0x142646160;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rcx; rax = 0x80000000; rcx = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; rdx = 0xca0000; fl_l = ((rcx as u8)) as u8 as u64; fl_r = (0x81u8) as u8 as u64; r9 = 0x40000; rdx = (if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { (rax as u32) } else { (rdx as u32) }) as u32 as u64; rax = ((rcx as u32)) as u32 as u64; rax = (rax & !0xffu64) | ((((((rax as u8)) & (0x5u8))) as u8 as u64)); crate::simd_ops::wu32(rbx.wrapping_add(P1), ((rdx as u32)) as u32); r8 = ((rdx as u32)) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 2; continue; } bb = 1; continue; }
            1 => { fl_l = ((((rdx as u32)) | ((r9 as u32)))) as u32 as u64; fl_r = fl_l; rdx = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(P1), ((rdx as u32)) as u32); r8 = ((rdx as u32)) as u32 as u64; bb = 2; continue; }
            2 => { rax = ((rcx as u32)) as u32 as u64; rax = (rax & !0xffu64) | ((((((rax as u8)) & (0x11u8))) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x10u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 4; continue; } bb = 3; continue; }
            3 => { fl_l = ((((rdx as u32)) >> (16u32)) & 1) as u64; rdx = (((rdx as u32)) | ((1u32) << (16u32))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(P1), ((rdx as u32)) as u32); r8 = ((rdx as u32)) as u32 as u64; bb = 4; continue; }
            4 => { rax = ((rcx as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shr(1u32)) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { r8 = ((rdx as u32)) as u32 as u64; fl_l = ((((r8 as u32)) >> (28u32)) & 1) as u64; r8 = (((r8 as u32)) | ((1u32) << (28u32))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(P1), ((r8 as u32)) as u32); bb = 6; continue; }
            6 => { rax = ((rcx as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shr(8u32)) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; rax = 0x80; rax = (if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { (r9 as u32) } else { (rax as u32) }) as u32 as u64; rcx = (((rcx as u32)).wrapping_shr(5u32)) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(P2), ((rax as u32)) as u32); fl_l = ((rcx as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { fl_l = ((((rax as u32)) | (0x8u32))) as u32 as u64; fl_r = fl_l; rax = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(P2), ((rax as u32)) as u32); bb = 8; continue; }
            8 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P3))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { rdx = 0xfffffff0; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); r8 = (crate::simd_ops::ru32(rbx.wrapping_add(P2))) as u32 as u64; rdx = 0xffffffec; rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P3))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); }
            10 => { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141864e10(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143080a00; const P1: u64 = 0x142442de0; const P2: u64 = 0x14397d820; const P3: u64 = 0x140209590; const P4: u64 = 0x14186a310; const P5: u64 = 0x1401e5660; const P6: u64 = 0x1401e5470; const P7: u64 = 0x1425371a0; const P8: u64 = 0x142533200;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rdi = rcx; rax = P0; crate::simd_ops::wu64(rcx, rax); rbx = (crate::simd_ops::ru64(rcx.wrapping_add(0x58u64))) as u64; crate::simd_ops::wu64(rcx.wrapping_add(0x58u64), (0x0u64) as u64); fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x38u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rax); rbx = (crate::simd_ops::ru64(P2)) as u64; r8 = rsp.wrapping_add(0x58); rcx = rsp.wrapping_add(0x50); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; crate::simd_ops::wu8(rax.wrapping_add(0x18u64), (0x1u8) as u8); rdx = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdx); crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0x0u32) as u32); crate::simd_ops::lk_add_u32(rdx, (1) as u32); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rax); rdx = rsp.wrapping_add(0x60); rcx = rbx.wrapping_add(0x1f0); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rcx = rsp.wrapping_add(0x20); rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 4; continue; }
            4 => { rax = 0xffffffff; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx, xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 9; continue; } bb = 5; continue; }
            5 => { fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 6; continue; }
            6 => { rsi = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; fl_l = (((rsi) & (0xfffffffffffffffcu64))) as u64; fl_r = fl_l; rsi = ((fl_l as u64)) as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru64(rsi)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rsi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = rsi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 8; continue; }
            8 => { rcx = rbx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 9; continue; }
            9 => { crate::simd_ops::wu64(rdi.wrapping_add(0x8u64), (0x0u64) as u64); bb = 10; continue; }
            10 => { rbx = (crate::simd_ops::ru64(rdi.wrapping_add(0x58u64))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 18; continue; } bb = 11; continue; }
            11 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x38u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); r10 = rbx.wrapping_sub(0x1); r10 = (((r10) & (0xfffffffffe000000u64))) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(r10.wrapping_add(0x100u64))) as u64; rdx = (rdx & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); rax = rbx; rax = ((rax).wrapping_sub(r10)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rcx = r10.wrapping_add(0x108); rcx = ((rcx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x4u64))) as u32 as u64; rcx = ((rcx).wrapping_sub(rax)) as u64; fl_l = ((rdx as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 16; continue; } bb = 12; continue; }
            12 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0xeu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 15; continue; } bb = 13; continue; }
            13 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(rbx, rax); crate::simd_ops::wu64(rcx.wrapping_add(0x20u64), rbx); fl_l = (crate::simd_ops::lk_add_u32(rcx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 18; continue; } bb = 14; continue; }
            14 => { rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); bb = 18; continue; }
            15 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 17; continue; }
            16 => { r8 = 0; fl_l = 0; fl_r = 0; bb = 17; continue; }
            17 => { rdx = rcx; r9 = rbx; rcx = r10; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); bb = 18; continue; }
            18 => { rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141864fb0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x18; const P2: u64 = 0x28;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64;
    crate::simd_ops::wu64(rcx, rdx);
    xmm0 = (crate::simd_ops::ru128(r8)) as u128;
    crate::simd_ops::wu128(rcx.wrapping_add(P0), (xmm0) as u128);
    xmm0 = (crate::simd_ops::ru128(r9)) as u128;
    crate::simd_ops::wu128(rcx.wrapping_add(P1), (xmm0) as u128);
    rdx = (crate::simd_ops::ru64(rax)) as u64;
    rax = rcx;
    crate::simd_ops::wu64(rcx.wrapping_add(P2), rdx);
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141864fe0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14397d820; const P1: u64 = 0x1423fe080; const P2: u64 = 0x1426460a0; const P3: u64 = 0x1425371a0; const P4: u64 = 0x142533200;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 91>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r11 = rsp; crate::simd_ops::wu64(r11.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x298u64)) as u64; rdx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(r11.wrapping_add(0x10u64), ((rdx as u32)) as u32); r14 = (crate::simd_ops::ru64(P0)) as u64; r13 = (crate::simd_ops::ru64(rcx)) as u64; rbp = (crate::simd_ops::ru64(r14.wrapping_add(0x58u64))) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x28u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x2f8u64), rax); rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x1cu64))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x2f0u64), ((rax as u32)) as u32); r15 = (crate::simd_ops::ru32(rcx.wrapping_add(0x24u64))) as u32 as u64; r15 = (((r15 as u32)).wrapping_sub((rax as u32))) as u32 as u64; r12 = (crate::simd_ops::ru32(rcx.wrapping_add(0x18u64))) as u32 as u64; rsi = (crate::simd_ops::ru32(rcx.wrapping_add(0x20u64))) as u32 as u64; rsi = (((rsi as u32)).wrapping_sub((r12 as u32))) as u32 as u64; rdi = (crate::simd_ops::ru32(r13.wrapping_add(0x18u64))) as u32 as u64; rax = r11.wrapping_sub(0x248); crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), rax); crate::simd_ops::wu32(rsp.wrapping_add(0x78u64), (0x100u32) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x7cu64), (0x100u64) as u64); rax = r11.wrapping_sub(0x248); crate::simd_ops::wu64(r11.wrapping_sub(0x250u64), rax); crate::simd_ops::wu16(rsp.wrapping_add(0x90u64), ((rdx as u16)) as u16); crate::simd_ops::wu32(r11.wrapping_add(0x10u64), (0x1u32) as u32); xmm0 = (crate::simd_ops::ru128(rcx.wrapping_add(0x8u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x60u64), (xmm0) as u128); rdx = rsp.wrapping_add(0x60); rcx = rsp.wrapping_add(0x70); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rax = (crate::simd_ops::ru64(r14)) as u64; rcx = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x30u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = (((rax as u16)) as u32) as u32 as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x2e0u64))) as u64; rax = (crate::simd_ops::ru64(rax)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), r13); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rbp); rbp = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rbp); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x2f8u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rcx); crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), ((r15 as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((rsi as u32)) as u32); rcx = (crate::simd_ops::ru32(rsp.wrapping_add(0x2f0u64))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rcx as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((r12 as u32)) as u32); r9 = ((rdi as u32)) as u32 as u64; r8 = rbx; rcx = (crate::simd_ops::ru32(rax.wrapping_add(0x1cu64))) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = rax; r10 = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; fl_l = r10; fl_r = r10; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 1; continue; }
            1 => { fl_l = r10; fl_r = (crate::simd_ops::ru64(rsp.wrapping_add(0x88u64))) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 9; continue; } bb = 2; continue; }
            2 => { r10 = ((r10).wrapping_sub(1)) as u64; r10 = (((r10) & (0xfffffffffe000000u64))) as u64; rcx = ((crate::exe::teb_self() as u64)) as u64; fl_l = rcx; fl_r = (crate::simd_ops::ru64(r10.wrapping_add(0x100u64))) as u64; rcx = (rcx & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rax = r9; rax = ((rax).wrapping_sub(r10)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rdx = r10.wrapping_add(0x108); rdx = ((rdx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32 as u64; rdx = ((rdx).wrapping_sub(rax)) as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 7; continue; } bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0xeu64))) as u8 as u64; fl_r = ((rbp as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 6; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(r9, rax); crate::simd_ops::wu64(rdx.wrapping_add(0x20u64), r9); fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 9; continue; } bb = 5; continue; }
            5 => { rcx = rdx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 9; continue; }
            6 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 8; continue; }
            7 => { r8 = 0; fl_l = 0; fl_r = 0; bb = 8; continue; }
            8 => { rcx = r10; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x88u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { crate::simd_ops::wu16(rax, ((rbp as u16)) as u16); bb = 11; continue; }
            11 => { rax = rbx; rsp = ((rsp).wrapping_add(0x298u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1418651c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143080a00; const P1: u64 = 0x141864d30; const P2: u64 = 0x14397d820; const P3: u64 = 0x140209310; const P4: u64 = 0x1402094c0; const P5: u64 = 0x14186a310; const P6: u64 = 0x1426455a8; const P7: u64 = 0x1426455b8; const P8: u64 = 0x142645598; const P9: u64 = 0x1401f3450; const P10: u64 = 0x142442de0; const P11: u64 = 0x1426460d0; const P12: u64 = 0x142646de0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 31>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1418651c0, funcinfo_rva: 0x36f9834, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0xf8, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rcx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r12) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r13) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r15) as u64); rbp = ((rsp.wrapping_sub(0x17u64)) as u64) as u64; rsp = ((rsp).wrapping_sub(0xc0u64)) as u64; rsi = (r9) as u64; r14 = (r8) as u64; r15 = (rdx) as u64; r12 = (rcx) as u64; rbx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rbp.wrapping_add(0x5fu64), ((rbx as u32)) as u32); rax = ((P0) as u64) as u64; crate::simd_ops::wu64(rcx, (rax) as u64); r13 = ((rcx.wrapping_add(0x8u64)) as u64) as u64; crate::simd_ops::wu64(r13, (rbx) as u64); crate::simd_ops::wu64(rcx.wrapping_add(0x10u64), (rbx) as u64); crate::simd_ops::wu64(rcx.wrapping_add(0x18u64), (rbx) as u64); crate::simd_ops::wu64(rcx.wrapping_add(0x20u64), (rbx) as u64); crate::simd_ops::wu32(rcx.wrapping_add(0x28u64), (0x7fffffffu32) as u32); crate::simd_ops::wu32(rcx.wrapping_add(0x2cu64), (0x7fffffffu32) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x30u64), ((rbx as u8)) as u8); crate::simd_ops::wu64(rcx.wrapping_add(0x34u64), (rbx) as u64); crate::simd_ops::wu64(rcx.wrapping_add(0x3cu64), (rbx) as u64); rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x7fu64))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x44u64), ((rax as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x48u64), ((rbx as u8)) as u8); crate::simd_ops::wu64(rcx.wrapping_add(0x50u64), (rbx) as u64); crate::simd_ops::wu64(rcx.wrapping_add(0x58u64), (rbx) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14186523a); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdi = (crate::simd_ops::ru64(P2)) as u64; rdx = (crate::simd_ops::ru32(r12.wrapping_add(0x1cu64))) as u32 as u64; rcx = (crate::simd_ops::ru32(r12.wrapping_add(0x18u64))) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rbx) as u64); rbx = (crate::simd_ops::ru64(rbp.wrapping_add(0x77u64))) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rbx as u32)) as u32); rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x7bu64))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_sub(0x7du64), ((rax as u32)) as u32); rax = ((rsp.wrapping_add(0x20u64)) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x69u64), (rax) as u64); crate::simd_ops::wu32(rbp.wrapping_sub(0x61u64), ((rcx as u32)) as u32); crate::simd_ops::wu32(rbp.wrapping_sub(0x5du64), ((rdx as u32)) as u32); rax = ((rsp.wrapping_add(0x20u64)) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x59u64), (rax) as u64); crate::simd_ops::wu64(rbp.wrapping_sub(0x51u64), (rbx) as u64); crate::simd_ops::wu32(rbp.wrapping_sub(0x49u64), ((rcx as u32)) as u32); crate::simd_ops::wu32(rbp.wrapping_sub(0x45u64), ((rdx as u32)) as u32); rax = ((rsp.wrapping_add(0x20u64)) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x41u64), (rax) as u64); crate::simd_ops::wu32(rbp.wrapping_sub(0x39u64), ((rcx as u32)) as u32); crate::simd_ops::wu32(rbp.wrapping_sub(0x35u64), ((rdx as u32)) as u32); rdx = (rsi) as u64; rcx = ((rbp.wrapping_sub(0x69u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14186529b); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; xmm1 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; xmm0 = (crate::simd_ops::ru128(r12.wrapping_add(0x20u64))) as u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x1u64), (xmm0) as u128); crate::simd_ops::wu64(r12.wrapping_add(0x20u64), (rbx) as u64); crate::simd_ops::wu64(r12.wrapping_add(0x28u64), (rbx) as u64); crate::simd_ops::wu64(rbp.wrapping_sub(0x31u64), (r12) as u64); xmm0 = (crate::simd_ops::ru128(r14)) as u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x29u64), (xmm0) as u128); rax = ((xmm1 as u64)) as u64; crate::simd_ops::wu32(rbp.wrapping_sub(0x19u64), ((rax as u32)) as u32); rax = ((rax).wrapping_shr(32u32)) as u64; crate::simd_ops::wu32(rbp.wrapping_sub(0x15u64), ((rax as u32)) as u32); rax = (((xmm1 >> 64) as u64)) as u64; crate::simd_ops::wu32(rbp.wrapping_sub(0x11u64), ((rax as u32)) as u32); rax = ((rax).wrapping_shr(32u32)) as u64; crate::simd_ops::wu32(rbp.wrapping_sub(0xdu64), ((rax as u32)) as u32); crate::simd_ops::wu64(rbp.wrapping_sub(0x9u64), (r15) as u64); r8 = ((rbp.wrapping_sub(0x31u64)) as u64) as u64; rcx = ((rbp.wrapping_add(0x77u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1418652f5); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x77u64))) as u64; crate::simd_ops::wu8(rax.wrapping_add(0x40u64), (0x1u8) as u8); rdx = (crate::simd_ops::ru64(rax.wrapping_add(0x38u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x79u64), (rdx) as u64); rsi = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rbp.wrapping_sub(0x71u64), ((rsi as u32)) as u32); crate::simd_ops::lk_add_u32(rdx.wrapping_add(0x10u64), (1) as u32); crate::simd_ops::lk_or_u32(rbp.wrapping_add(0x5fu64), (0x2u32) as u32); rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x77u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x77u64), (rsi) as u64); crate::simd_ops::wu64(rbp.wrapping_add(0x67u64), (rax) as u64); rdx = ((rbp.wrapping_add(0x67u64)) as u64) as u64; rcx = ((rdi.wrapping_add(0x1f0u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14186532f); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdi = (crate::simd_ops::ru64(rbp.wrapping_sub(0x79u64))) as u64; rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x18u64))) as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 11; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(0x38u64))) as u8 as u64; fl_r = ((rsi as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 7; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rdi.wrapping_add(0x30u64), (rsi) as u64); crate::simd_ops::wu64(rdi.wrapping_add(0x20u64), (rsi) as u64); crate::simd_ops::wu64(rdi.wrapping_add(0x28u64), (rsi) as u64); crate::simd_ops::wu32(rdi.wrapping_add(0x30u64), ((rsi as u32)) as u32); crate::simd_ops::wu8(rdi.wrapping_add(0x38u64), (0x1u8) as u8); bb = 3; continue; }
            3 => { rcx = (rax) as u64; rcx = (((rcx) | (0x2u64))) as u64; fl_l = (rax) as u64 as u64; let cx_src: u64 = rcx; let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u64(rdi.wrapping_add(0x18u64), fl_l as u64, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u64)) as u64; } if (((fl_l as u64)) == ((fl_r as u64))) { bb = 6; continue; } bb = 4; continue; }
            4 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 3; continue; } bb = 5; continue; }
            5 => { r13 = ((r12.wrapping_add(0x8u64)) as u64) as u64; bb = 11; continue; }
            6 => { r13 = ((r12.wrapping_add(0x8u64)) as u64) as u64; bb = 7; continue; }
            7 => { rcx = ((rdi.wrapping_add(0x28u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141865383); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P6)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x30u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) > 0) { bb = 10; continue; } bb = 8; continue; }
            8 => { r9 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; r8 = (0xffffffffu32) as u32 as u64; rdx = ((rdi.wrapping_add(0x28u64)) as u64) as u64; rcx = ((rdi.wrapping_add(0x20u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1418653a1); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P7)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x30u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 8; continue; } bb = 9; continue; }
            9 => { r13 = ((r12.wrapping_add(0x8u64)) as u64) as u64; bb = 10; continue; }
            10 => { rax = (((rax as u32)).wrapping_sub(1)) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x30u64), ((rax as u32)) as u32); rcx = ((rdi.wrapping_add(0x28u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1418653bc); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P8)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 11; continue; }
            11 => { rbx = (crate::simd_ops::ru64(rdi)) as u64; rcx = ((rbp.wrapping_sub(0x79u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1418653c9); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu64(r12.wrapping_add(0x8u64), (rbx) as u64); rbx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x79u64))) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rbx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 12; continue; }
            12 => { rax = (0xffffffffu32) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(0x10u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 17; continue; } bb = 13; continue; }
            13 => { fl_l = (rbx) as u64 as u64; fl_r = (rbx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 14; continue; }
            14 => { rdi = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; fl_l = (((rdi) & (0xfffffffffffffffcu64))) as u64 as u64; fl_r = fl_l; rdi = ((fl_l as u64)) as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 16; continue; } bb = 15; continue; }
            15 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141865402); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141865408); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P10) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 16; continue; }
            16 => { rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141865411); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P10) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 17; continue; }
            17 => { rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rcx = (crate::simd_ops::ru64(r13)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14186541d); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P11)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu64(r12.wrapping_add(0x10u64), (rax) as u64); xmm0 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x1u64))) as u128; crate::simd_ops::wu128(r12.wrapping_add(0x20u64), (xmm0) as u128); rax = (crate::simd_ops::ru32(r12.wrapping_add(0x44u64))) as u32 as u64; rax = (((rax as u32)).wrapping_shr(10u32)) as u32 as u64; rax = ((((rax as u32)) & (0x1u32))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x7fu64), ((rax as u32)) as u32); r9 = (0x4u32) as u32 as u64; r8 = ((rbp.wrapping_add(0x7fu64)) as u64) as u64; rdx = ((r9.wrapping_add(0x1du64)) as u32) as u32 as u64; rcx = (crate::simd_ops::ru64(r13)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141865454); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P12)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = (r12) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x118u64))) as u64; rsp = ((rsp).wrapping_add(0xc0u64)) as u64; r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r13 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r12 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rsi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1418654a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xc; const P1: u64 = 0x8; const P2: u64 = 0x1426460e8; const P3: u64 = 0x14186aad0; const P4: u64 = 0x14186a450; const P5: u64 = 0x142646230; const P6: u64 = 0x4;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x60u64)) as u64; r9 = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; rbx = rdx; rdx = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64; rdi = rcx; rcx = (crate::simd_ops::ru64(rcx)) as u64; r8 = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru64(rbx)) as u64; rcx = rsp.wrapping_add(0x20); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru8(rsp.wrapping_add(0x50u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 2; continue; } bb = 1; continue; }
            1 => { r9 = rsp.wrapping_add(0x20); crate::simd_ops::wu8(rsp.wrapping_add(0x50u64), (0x0u8) as u8); r8 = P4; rdx = 0; fl_l = 0; fl_r = 0; rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            2 => { r10 = (crate::simd_ops::ru64(rdi)) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; r11 = rsi; r11 = ((r11).wrapping_shr(32u32)) as u64; rbx = rcx; rax = (crate::simd_ops::ru32(r10)) as u32 as u64; r9 = (crate::simd_ops::ru32(r10.wrapping_add(P1))) as u32 as u64; rax = (((rax as u32)).wrapping_sub((r9 as u32))) as u32 as u64; rax = (((rax as u32)).wrapping_add((rcx as u32))) as u32 as u64; rbx = ((rbx).wrapping_shr(32u32)) as u64; rdx = (if (rax as u32 as i32) < 0 { 0xffffffffu32 } else { 0u32 }) as u64; rax = (((rax as u32)).wrapping_sub((rdx as u32))) as u32 as u64; rax = (((((rax as u32)) as i32).wrapping_shr(1u32) as u32)) as u32 as u64; rcx = ((rax.wrapping_add(rsi)) as u32) as u32 as u64; rax = (crate::simd_ops::ru32(r10.wrapping_add(P6))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; rcx = (if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { (rsi as u32) } else { (rcx as u32) }) as u32 as u64; rax = (((rax as u32)).wrapping_sub(crate::simd_ops::ru32(r10.wrapping_add(P0)))) as u32 as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rax = (((rax as u32)).wrapping_add((rbx as u32))) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rdx = (if (rax as u32 as i32) < 0 { 0xffffffffu32 } else { 0u32 }) as u64; rax = (((rax as u32)).wrapping_sub((rdx as u32))) as u32 as u64; rax = (((((rax as u32)) as i32).wrapping_shr(1u32) as u32)) as u32 as u64; rdx = ((rax.wrapping_add(r11)) as u32) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((r11 as u32)) as u32 as u64; rdx = (if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { (r11 as u32) } else { (rdx as u32) }) as u32 as u64; r9 = (((r9 as u32)).wrapping_sub(crate::simd_ops::ru32(r10))) as u32 as u64; r9 = (((r9 as u32)).wrapping_add((rcx as u32))) as u32 as u64; crate::simd_ops::wu32(r10.wrapping_add(P1), ((r9 as u32)) as u32); rax = (crate::simd_ops::ru64(rdi)) as u64; crate::simd_ops::wu32(rax, ((rcx as u32)) as u32); rax = ((rdx as u32)) as u32 as u64; rcx = (crate::simd_ops::ru64(rdi)) as u64; rax = (((rax as u32)).wrapping_sub(crate::simd_ops::ru32(rcx.wrapping_add(P6)))) as u32 as u64; crate::simd_ops::lk_add_u32(rcx.wrapping_add(P0), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rdi)) as u64; crate::simd_ops::wu32(rax.wrapping_add(P6), ((rdx as u32)) as u32); rsp = ((rsp).wrapping_add(0x60u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141865580(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x14;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rcx, rdx);
    rax = (crate::simd_ops::ru64(r8)) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(P0), rax);
    rax = (crate::simd_ops::ru32(r9)) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(P1), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64;
    rdx = (crate::simd_ops::ru32(rax)) as u32 as u64;
    rax = rcx;
    crate::simd_ops::wu32(rcx.wrapping_add(P2), ((rdx as u32)) as u32);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1418655b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14; const P1: u64 = 0x10; const P2: u64 = 0x1426460e8; const P3: u64 = 0x14186aad0; const P4: u64 = 0x14186a450; const P5: u64 = 0x142646230; const P6: u64 = 0x8; const P7: u64 = 0x4; const P8: u64 = 0xc;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x60u64)) as u64; r9 = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; rbx = rdx; rdx = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64; rdi = rcx; rcx = (crate::simd_ops::ru64(rcx)) as u64; r8 = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru64(rbx)) as u64; rcx = rsp.wrapping_add(0x20); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru8(rsp.wrapping_add(0x50u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 2; continue; } bb = 1; continue; }
            1 => { r9 = rsp.wrapping_add(0x20); crate::simd_ops::wu8(rsp.wrapping_add(0x50u64), (0x0u8) as u8); r8 = P4; rdx = 0; fl_l = 0; fl_r = 0; rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru64(rdi)) as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; crate::simd_ops::wu32(rcx, ((rax as u32)) as u32); rcx = (crate::simd_ops::ru32(rdi.wrapping_add(P6))) as u32 as u64; rdx = (crate::simd_ops::ru64(rdi)) as u64; rax = ((rax).wrapping_shr(32u32)) as u64; rcx = (((rcx as u32)).wrapping_add(crate::simd_ops::ru32(rdx))) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_add(P6), ((rcx as u32)) as u32); rcx = (crate::simd_ops::ru64(rdi)) as u64; crate::simd_ops::wu32(rcx.wrapping_add(P7), ((rax as u32)) as u32); rcx = (crate::simd_ops::ru64(rdi)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(P7))) as u32 as u64; rax = (((rax as u32)).wrapping_add(crate::simd_ops::ru32(rdi.wrapping_add(P8)))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(P8), ((rax as u32)) as u32); rsp = ((rsp).wrapping_add(0x60u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141865660(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x4; const P2: u64 = 0xc; const P3: u64 = 0x1426460e8;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru32(rdx)) as u32 as u64;
    r9 = rdx;
    r8 = (crate::simd_ops::ru64(rcx)) as u64;
    crate::simd_ops::lk_add_u32(r8, ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx)) as u32 as u64;
    r8 = (crate::simd_ops::ru64(rcx)) as u64;
    crate::simd_ops::lk_add_u32(r8.wrapping_add(P0), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32 as u64;
    r8 = (crate::simd_ops::ru64(rcx)) as u64;
    crate::simd_ops::lk_add_u32(r8.wrapping_add(P1), ((rax as u32)) as u32);
    r8 = 0;
    fl_l = 0; fl_r = 0;
    rdx = (crate::simd_ops::ru64(rcx)) as u64;
    rax = (crate::simd_ops::ru32(r9.wrapping_add(P1))) as u32 as u64;
    fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(P2), ((rax as u32)) as u32)) as u32 as u64; fl_r = fl_l;
    r9 = (crate::simd_ops::ru32(rcx.wrapping_add(P2))) as u32 as u64;
    rdx = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64;
    rcx = (crate::simd_ops::ru64(rcx)) as u64;
    return crate::simd_ops::call_fwd(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1418656b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x4; const P2: u64 = 0xc; const P3: u64 = 0x140209310;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    r11 = rsp;
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x60u64)) as u64;
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(r11.wrapping_sub(0x48u64), rcx);
    crate::simd_ops::wu64(rcx, rax);
    r10 = ((r9 as u32)) as u32 as u64;
    rax = (crate::simd_ops::ru32(r8)) as u32 as u64;
    rbx = rcx;
    crate::simd_ops::wu32(rcx.wrapping_add(P0), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(r8.wrapping_add(P1))) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(P2), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru64(r8)) as u64;
    crate::simd_ops::wu32(r11.wrapping_sub(0x40u64), ((r9 as u32)) as u32);
    r9 = (crate::simd_ops::ru32(rsp.wrapping_add(0x90u64))) as u32 as u64;
    crate::simd_ops::wu64(r11.wrapping_sub(0x38u64), rcx);
    crate::simd_ops::wu64(r11.wrapping_sub(0x20u64), rcx);
    rcx = r11.wrapping_sub(0x48);
    crate::simd_ops::wu64(r11.wrapping_sub(0x30u64), rax);
    crate::simd_ops::wu32(r11.wrapping_sub(0x3cu64), ((r9 as u32)) as u32);
    crate::simd_ops::wu32(r11.wrapping_sub(0x28u64), ((r10 as u32)) as u32);
    crate::simd_ops::wu32(r11.wrapping_sub(0x24u64), ((r9 as u32)) as u32);
    crate::simd_ops::wu32(r11.wrapping_sub(0x18u64), ((r10 as u32)) as u32);
    crate::simd_ops::wu32(r11.wrapping_sub(0x14u64), ((r9 as u32)) as u32);
    rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp);
    rax = rbx;
    rsp = ((rsp).wrapping_add(0x60u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1418657c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143080bc0; const P1: u64 = 0x8; const P2: u64 = 0x1401f3d90; const P3: u64 = 0x10; const P4: u64 = 0x18;
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rax = P0;
    rbx = rcx;
    crate::simd_ops::wu64(rcx, rax);
    rax = ((crate::simd_ops::ru8(rdx)) as u32) as u32 as u64;
    crate::simd_ops::wu8(rcx.wrapping_add(P1), ((rax as u8)) as u8);
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    crate::simd_ops::wu64(rbx.wrapping_add(P3), rax);
    rax = rbx;
    crate::simd_ops::wu16(rbx.wrapping_add(P4), (0x0u16) as u16);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141865920(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142646108; const P1: u64 = 0x8; const P2: u64 = 0x142646170; const P3: u64 = 0x338; const P4: u64 = 0x140209990; const P5: u64 = 0x350; const P6: u64 = 0x390; const P7: u64 = 0x378; const P8: u64 = 0x388; const P9: u64 = 0x50; const P10: u64 = 0x380; const P11: u64 = 0x142646128; const P12: u64 = 0x1401f3ee0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rdi = rcx; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), (0x0u64) as u64); rcx = rsp.wrapping_add(0x40); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 10; continue; } bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rbx); rbx = (crate::simd_ops::ru64(rdi.wrapping_add(P1))) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); r8 = rsp.wrapping_add(0x48); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rax); rdx = rsp.wrapping_add(0x20); rcx = ((rbx.wrapping_add(P3)) as u64) as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rbx.wrapping_add(P5))) as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(rbx.wrapping_add(P3)))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; fl_l = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 10; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rdx = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P1))) as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(P6))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 6; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru32(rcx.wrapping_add(P7))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) >= 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P8))) as u64; bb = 9; continue; }
            6 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(P9))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { rcx = rax; bb = 9; continue; }
            8 => { rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P10))) as u64; bb = 9; continue; }
            9 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P11), rax), rax, rcx, rdx, r8, r9, rsp); bb = 10; continue; }
            10 => { rcx = rdi.wrapping_add(0x10); rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P12, rax, rcx, rdx, r8, r9, rsp); }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141865a50(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x142646110; const P2: u64 = 0x10; const P3: u64 = 0x1401f3ee0;
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    rcx = ((crate::simd_ops::ru8(rcx.wrapping_add(P0))) as u32) as u32 as u64;
    rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp);
    rcx = ((rbx.wrapping_add(P2)) as u64) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(P3, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141865ac0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x390; const P1: u64 = 0x378; const P2: u64 = 0x388; const P3: u64 = 0x142646128; const P4: u64 = 0x50; const P5: u64 = 0x380;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(P0))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 3; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) >= 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64; return crate::simd_ops::call_fwd(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            3 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(P4))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rcx = rax; return crate::simd_ops::call_fwd(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            5 => { rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P5))) as u64; return crate::simd_ops::call_fwd(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141865b10(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x338; const P1: u64 = 0x140209990; const P2: u64 = 0x18; const P3: u64 = 0x8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x30u64)) as u64;
    rbx = ((rcx.wrapping_add(P0)) as u64) as u64;
    rcx = rbx;
    r8 = rsp.wrapping_add(0x48);
    rdx = rsp.wrapping_add(0x20);
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rax = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64;
    rax = ((rax).wrapping_add(crate::simd_ops::ru64(rbx))) as u64;
    fl_l = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; fl_r = rax;
    if !((((fl_l as u64)) != ((fl_r as u64)))) { rax = 0; fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64;
    rax = (crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64;
    rsp = ((rsp).wrapping_add(0x30u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141865b60(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x338; const P1: u64 = 0x140209990; const P2: u64 = 0x18; const P3: u64 = 0x8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x30u64)) as u64;
    rbx = ((rcx.wrapping_add(P0)) as u64) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rdx);
    rcx = rbx;
    r8 = rsp.wrapping_add(0x40);
    rdx = rsp.wrapping_add(0x20);
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rax = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64;
    rax = ((rax).wrapping_add(crate::simd_ops::ru64(rbx))) as u64;
    fl_l = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; fl_r = rax;
    if !((((fl_l as u64)) != ((fl_r as u64)))) { rax = 0; fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64;
    rax = (crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64;
    rsp = ((rsp).wrapping_add(0x30u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141865bb0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1426455a8; const P1: u64 = 0x140209af0; const P2: u64 = 0x1401deb40; const P3: u64 = 0x142645598;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 15>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x50u64)) as u64; r15 = rcx; rdi = (crate::simd_ops::ru64(rcx.wrapping_add(0x340u64))) as u64; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(0x338u64))) as u64; rbx = rdx; fl_l = (crate::simd_ops::ru8(rdx)) as u8 as u64; fl_r = (0xffu8) as u8 as u64; if ((((fl_l as u8)) as i8) >= (((fl_r as u8)) as i8)) { bb = 3; continue; } bb = 1; continue; }
            1 => { xmm1 = crate::simd_ops::vpcmpeqd(xmm1, xmm1); bb = 2; continue; }
            2 => { xmm0 = crate::simd_ops::vpcmpgtb(xmm1, crate::simd_ops::ru128(rbx)); rax = (crate::simd_ops::vpmovmskb(xmm0) as u32) as u32 as u64; rax = (((rax as u32)).wrapping_add(1)) as u32 as u64; let bit_in: u32 = (rax as u32); if bit_in != 0 { rcx = ((bit_in.trailing_zeros() as u32)) as u32 as u64; } fl_l = bit_in as u64; rbx = ((rbx).wrapping_add(rcx)) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rdi = ((rdi).wrapping_add(rcx)) as u64; fl_l = (crate::simd_ops::ru8(rbx)) as u8 as u64; fl_r = (0xffu8) as u8 as u64; if ((((fl_l as u8)) as i8) < (((fl_r as u8)) as i8)) { bb = 2; continue; } bb = 3; continue; }
            3 => { rbp = (crate::simd_ops::ru64(r15.wrapping_add(0x350u64))) as u64; rbp = ((rbp).wrapping_add(rdx)) as u64; fl_l = rbx; fl_r = rbp; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 14; continue; } bb = 4; continue; }
            4 => { bb = 5; continue; }
            5 => { rsi = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(rsi)) as u64; rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0xb0u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 10; continue; } bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru64(rsi)) as u64; rdx = rsp.wrapping_add(0x88); rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0xa0u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r14 = rax; rcx = (crate::simd_ops::ru64(rsi)) as u64; r8 = (crate::simd_ops::ru64(rcx.wrapping_add(0x100u64))) as u64; rdx = rsp.wrapping_add(0x20); rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(r8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru32(r14)) as u32 as u64; fl_l = (crate::simd_ops::ru32(rax.wrapping_add(0x8u64))) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 9; continue; } bb = 8; continue; }
            8 => { rcx = (crate::simd_ops::ru32(r14.wrapping_add(0x4u64))) as u32 as u64; fl_l = (crate::simd_ops::ru32(rax.wrapping_add(0xcu64))) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 10; continue; } bb = 9; continue; }
            9 => { crate::simd_ops::wu8(rsi.wrapping_add(0x48u64), (0x1u8) as u8); bb = 10; continue; }
            10 => { rbx = ((rbx).wrapping_add(1)) as u64; rdi = ((rdi).wrapping_add(0x10u64)) as u64; fl_l = (crate::simd_ops::ru8(rbx)) as u8 as u64; fl_r = (0xffu8) as u8 as u64; if ((((fl_l as u8)) as i8) >= (((fl_r as u8)) as i8)) { bb = 13; continue; } bb = 11; continue; }
            11 => { xmm1 = crate::simd_ops::vpcmpeqd(xmm1, xmm1); bb = 12; continue; }
            12 => { xmm0 = crate::simd_ops::vpcmpgtb(xmm1, crate::simd_ops::ru128(rbx)); rax = (crate::simd_ops::vpmovmskb(xmm0) as u32) as u32 as u64; rax = (((rax as u32)).wrapping_add(1)) as u32 as u64; let bit_in: u32 = (rax as u32); if bit_in != 0 { rcx = ((bit_in.trailing_zeros() as u32)) as u32 as u64; } fl_l = bit_in as u64; rbx = ((rbx).wrapping_add(rcx)) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rdi = ((rdi).wrapping_add(rcx)) as u64; fl_l = (crate::simd_ops::ru8(rbx)) as u8 as u64; fl_r = (0xffu8) as u8 as u64; if ((((fl_l as u8)) as i8) < (((fl_r as u8)) as i8)) { bb = 12; continue; } bb = 13; continue; }
            13 => { fl_l = rbx; fl_r = rbp; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 5; continue; } bb = 14; continue; }
            14 => { rcx = r15.wrapping_add(0x160); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbp = (crate::simd_ops::ru32(r15.wrapping_add(0x170u64))) as u32 as u64; rbp = (((rbp as u32)).wrapping_add(0x3u32)) as u32 as u64; rbp = ((((rbp as u32)) & (0xfffffffcu32))) as u32 as u64; rdx = rbp.wrapping_add(0x4); rcx = r15.wrapping_add(0x168); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(r15.wrapping_add(0x168u64))) as u64; crate::simd_ops::wu32(rbp.wrapping_add(rax), (0x12u32) as u32); rdi = r15.wrapping_add(0x178); rdx = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; r8 = (crate::simd_ops::ru32(rdi.wrapping_add(0xcu64))) as u32 as u64; rcx = ((r8.wrapping_mul(4)) as u64) as u64; rax = ((rdx.wrapping_mul(4)) as u64) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 16; continue; } bb = 15; continue; }
            15 => { rax = rdx.wrapping_add(0x1); rdx = ((r8 as u32)) as u32 as u64; rdx = ((rdx).wrapping_shr(1u32)) as u64; rdx = ((rdx).wrapping_add(r8)) as u64; fl_l = rdx; fl_r = rax; rdx = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { rdx }) as u64; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 16; continue; }
            16 => { rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; rax = (crate::simd_ops::ru64(rdi)) as u64; crate::simd_ops::wu32(rax.wrapping_add(rcx.wrapping_mul(4)), ((rbp as u32)) as u32); fl_l = (crate::simd_ops::lk_add_u32(rdi.wrapping_add(0x8u64), (1) as u32)) as u32 as u64; rcx = r15.wrapping_add(0x160); r11 = rsp.wrapping_add(0x50); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x40u64))) as u64; rbp = (crate::simd_ops::ru64(r11.wrapping_add(0x48u64))) as u64; rsp = r11; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141865d70(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x160; const P1: u64 = 0x1426455a8; const P2: u64 = 0x4; const P3: u64 = 0x140209af0; const P4: u64 = 0x1; const P5: u64 = 0x1401deb40; const P6: u64 = 0x142645598;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbp = rcx; rbx = ((rcx.wrapping_add(P0)) as u64) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); r14 = (crate::simd_ops::ru32(rbp.wrapping_add(0x170u64))) as u32 as u64; r14 = (((r14 as u32)).wrapping_add(0x3u32)) as u32 as u64; r14 = ((((r14 as u32)) & (0xfffffffcu32))) as u32 as u64; rdx = ((r14.wrapping_add(P2)) as u64) as u64; rcx = rbp.wrapping_add(0x168); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x168u64))) as u64; crate::simd_ops::wu32(r14.wrapping_add(rax), (0x11u32) as u32); rbp = ((rbp).wrapping_add(0x178u64)) as u64; rdx = (crate::simd_ops::ru32(rbp.wrapping_add(0x8u64))) as u32 as u64; r8 = (crate::simd_ops::ru32(rbp.wrapping_add(0xcu64))) as u32 as u64; rcx = ((r8.wrapping_mul(4)) as u64) as u64; rax = ((rdx.wrapping_mul(4)) as u64) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = ((rdx.wrapping_add(P4)) as u64) as u64; rdx = ((r8 as u32)) as u32 as u64; rdx = ((rdx).wrapping_shr(1u32)) as u64; rdx = ((rdx).wrapping_add(r8)) as u64; fl_l = rdx; fl_r = rax; rdx = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { rdx }) as u64; rcx = rbp; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru32(rbp.wrapping_add(0x8u64))) as u32 as u64; rax = (crate::simd_ops::ru64(rbp)) as u64; crate::simd_ops::wu32(rax.wrapping_add(rcx.wrapping_mul(4)), ((r14 as u32)) as u32); crate::simd_ops::lk_add_u32(rbp.wrapping_add(0x8u64), (1) as u32); rcx = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp); }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141865e50(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x18; const P3: u64 = 0x20; const P4: u64 = 0xffffffffffffffd8; const P5: u64 = 0xffffffffffffffe0; const P6: u64 = 0xffffffffffffffe8; const P7: u64 = 0x160; const P8: u64 = 0x1426455a8; const P9: u64 = 0x140209af0; const P10: u64 = 0x1; const P11: u64 = 0x1401deb40; const P12: u64 = 0x142645598;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = rsp; crate::simd_ops::wu64(rax.wrapping_add(P0), rbx); crate::simd_ops::wu64(rax.wrapping_add(P1), rbp); crate::simd_ops::wu64(rax.wrapping_add(P2), rsi); crate::simd_ops::wu64(rax.wrapping_add(P3), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rbp = rcx; crate::simd_ops::wu32(rax.wrapping_add(P4), (0x2u32) as u32); crate::simd_ops::wu64(rax.wrapping_add(P5), rdx); fl_l = r8; fl_r = (0xf020u64) as u64; crate::simd_ops::wu8(rax.wrapping_add(P6), (if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8); rbx = ((rcx.wrapping_add(P7)) as u64) as u64; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P8), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r14 = (crate::simd_ops::ru32(rbp.wrapping_add(0x170u64))) as u32 as u64; r14 = (((r14 as u32)).wrapping_add(0x7u32)) as u32 as u64; r14 = ((((r14 as u32)) & (0xfffffff8u32))) as u32 as u64; rdx = ((r14.wrapping_add(P2)) as u64) as u64; rcx = rbp.wrapping_add(0x168); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x168u64))) as u64; xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; crate::simd_ops::wu128(rax.wrapping_add(r14), (xmm0) as u128); xmm1 = ((crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64) as u128; crate::simd_ops::wu64(rax.wrapping_add(r14).wrapping_add(P1), ((xmm1 as u64)) as u64); rbp = ((rbp).wrapping_add(0x178u64)) as u64; rdx = (crate::simd_ops::ru32(rbp.wrapping_add(0x8u64))) as u32 as u64; r8 = (crate::simd_ops::ru32(rbp.wrapping_add(0xcu64))) as u32 as u64; rcx = ((r8.wrapping_mul(4)) as u64) as u64; rax = ((rdx.wrapping_mul(4)) as u64) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = ((rdx.wrapping_add(P10)) as u64) as u64; rdx = ((r8 as u32)) as u32 as u64; rdx = ((rdx).wrapping_shr(1u32)) as u64; rdx = ((rdx).wrapping_add(r8)) as u64; fl_l = rdx; fl_r = rax; rdx = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { rdx }) as u64; rcx = rbp; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P11, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru32(rbp.wrapping_add(0x8u64))) as u32 as u64; rax = (crate::simd_ops::ru64(rbp)) as u64; crate::simd_ops::wu32(rax.wrapping_add(rcx.wrapping_mul(4)), ((r14 as u32)) as u32); crate::simd_ops::lk_add_u32(rbp.wrapping_add(0x8u64), (1) as u32); rcx = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P12), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141865f50(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x18; const P3: u64 = 0x20; const P4: u64 = 0xffffffffffffffe8; const P5: u64 = 0xffffffffffffffc8; const P6: u64 = 0xffffffffffffffd0; const P7: u64 = 0x143426ea0; const P8: u64 = 0x160; const P9: u64 = 0x1426455a8; const P10: u64 = 0x140209af0; const P11: u64 = 0x1; const P12: u64 = 0x1401deb40; const P13: u64 = 0x142645598;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = rsp; crate::simd_ops::wu64(rax.wrapping_add(P0), rbx); crate::simd_ops::wu64(rax.wrapping_add(P1), rbp); crate::simd_ops::wu64(rax.wrapping_add(P2), rsi); crate::simd_ops::wu64(rax.wrapping_add(P3), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x50u64)) as u64; crate::simd_ops::wu128(rax.wrapping_add(P4), (xmm6) as u128); rbp = rcx; crate::simd_ops::wu32(rax.wrapping_add(P5), (0x5u32) as u32); crate::simd_ops::wu64(rax.wrapping_add(P6), rdx); r8 = ((r8).wrapping_shr(16u32)) as u64; rax = (((r8 as u16)) as u32) as u32 as u64; xmm0 = 0u128; xmm0 = (xmm0 & !0xffff_ffffu128) | (((((rax as u32)) as i32) as f32).to_bits() as u128); xmm6 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) / f32::from_bits(crate::simd_ops::ru32(P7))).to_bits() as u128); rbx = ((rcx.wrapping_add(P8)) as u64) as u64; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P9), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r14 = (crate::simd_ops::ru32(rbp.wrapping_add(0x170u64))) as u32 as u64; r14 = (((r14 as u32)).wrapping_add(0x7u32)) as u32 as u64; r14 = ((((r14 as u32)) & (0xfffffff8u32))) as u32 as u64; rdx = ((r14.wrapping_add(P2)) as u64) as u64; rcx = rbp.wrapping_add(0x168); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P10, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x168u64))) as u64; xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; crate::simd_ops::wu128(rax.wrapping_add(r14), (xmm0) as u128); xmm1 = ((crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64) as u128; xmm1 = (xmm1 & !0xffff_ffffu128) | (xmm6 & 0xffff_ffffu128); crate::simd_ops::wu64(rax.wrapping_add(r14).wrapping_add(P1), ((xmm1 as u64)) as u64); rbp = ((rbp).wrapping_add(0x178u64)) as u64; rdx = (crate::simd_ops::ru32(rbp.wrapping_add(0x8u64))) as u32 as u64; r8 = (crate::simd_ops::ru32(rbp.wrapping_add(0xcu64))) as u32 as u64; rcx = ((r8.wrapping_mul(4)) as u64) as u64; rax = ((rdx.wrapping_mul(4)) as u64) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = ((rdx.wrapping_add(P11)) as u64) as u64; rdx = ((r8 as u32)) as u32 as u64; rdx = ((rdx).wrapping_shr(1u32)) as u64; rdx = ((rdx).wrapping_add(r8)) as u64; fl_l = rdx; fl_r = rax; rdx = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { rdx }) as u64; rcx = rbp; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P12, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru32(rbp.wrapping_add(0x8u64))) as u32 as u64; rax = (crate::simd_ops::ru64(rbp)) as u64; crate::simd_ops::wu32(rax.wrapping_add(rcx.wrapping_mul(4)), ((r14 as u32)) as u32); crate::simd_ops::lk_add_u32(rbp.wrapping_add(0x8u64), (1) as u32); rcx = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x40u64))) as u128; rsp = ((rsp).wrapping_add(0x50u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P13), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141866070(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x160; const P1: u64 = 0x1426455a8; const P2: u64 = 0x10; const P3: u64 = 0x140209af0; const P4: u64 = 0x1; const P5: u64 = 0x1401deb40; const P6: u64 = 0x142645598;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rbp = rcx; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x4u32) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rdx); rbx = ((rcx.wrapping_add(P0)) as u64) as u64; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r14 = (crate::simd_ops::ru32(rbp.wrapping_add(0x170u64))) as u32 as u64; r14 = (((r14 as u32)).wrapping_add(0x7u32)) as u32 as u64; r14 = ((((r14 as u32)) & (0xfffffff8u32))) as u32 as u64; rdx = ((r14.wrapping_add(P2)) as u64) as u64; rcx = rbp.wrapping_add(0x168); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x168u64))) as u64; xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; crate::simd_ops::wu128(r14.wrapping_add(rax), (xmm0) as u128); rbp = ((rbp).wrapping_add(0x178u64)) as u64; rdx = (crate::simd_ops::ru32(rbp.wrapping_add(0x8u64))) as u32 as u64; r8 = (crate::simd_ops::ru32(rbp.wrapping_add(0xcu64))) as u32 as u64; rcx = ((r8.wrapping_mul(4)) as u64) as u64; rax = ((rdx.wrapping_mul(4)) as u64) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = ((rdx.wrapping_add(P4)) as u64) as u64; rdx = ((r8 as u32)) as u32 as u64; rdx = ((rdx).wrapping_shr(1u32)) as u64; rdx = ((rdx).wrapping_add(r8)) as u64; fl_l = rdx; fl_r = rax; rdx = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { rdx }) as u64; rcx = rbp; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru32(rbp.wrapping_add(0x8u64))) as u32 as u64; rax = (crate::simd_ops::ru64(rbp)) as u64; crate::simd_ops::wu32(rax.wrapping_add(rcx.wrapping_mul(4)), ((r14 as u32)) as u32); crate::simd_ops::lk_add_u32(rbp.wrapping_add(0x8u64), (1) as u32); rcx = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141866160(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x140209990; const P1: u64 = 0x350; const P2: u64 = 0x338; const P3: u64 = 0x8; const P4: u64 = 0x30; const P5: u64 = 0x34; const P6: u64 = 0x3c; const P7: u64 = 0x38; const P8: u64 = 0x40; const P9: u64 = 0x1402098b0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rdi = rdx; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rdx); rsi = rcx; rdx = rsp.wrapping_add(0x20); rcx = ((rcx).wrapping_add(0x338u64)) as u64; r8 = rsp.wrapping_add(0x50); rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rsi.wrapping_add(P1))) as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(rsi.wrapping_add(P2)))) as u64; fl_l = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 9; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(P4))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 6; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru32(rcx.wrapping_add(P5))) as u32 as u64; fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(P6))) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru32(rcx.wrapping_add(P7))) as u32 as u64; fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(P8))) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(P6))) as u64; rdx = (rdx & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 7; continue; }
            6 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rdx = (rdx & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 7; continue; }
            7 => { crate::simd_ops::wu8(rcx.wrapping_add(P4), (0x0u8) as u8); fl_l = ((rdx as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { crate::simd_ops::wu32(rsp.wrapping_add(0x50u64), ((rax as u32)) as u32); rdx = rsp.wrapping_add(0x20); rax = ((rax).wrapping_shr(32u32)) as u64; rcx = rsi; crate::simd_ops::wu32(rsp.wrapping_add(0x54u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x3u32) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rdi); rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); bb = 9; continue; }
            9 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141866230(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x338; const P1: u64 = 0x140209990; const P2: u64 = 0x18; const P3: u64 = 0x8; const P4: u64 = 0x30; const P5: u64 = 0xa0; const P6: u64 = 0x34; const P7: u64 = 0x3c;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x30u64)) as u64;
    rbx = ((rcx.wrapping_add(P0)) as u64) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rdx);
    rcx = rbx;
    r8 = rsp.wrapping_add(0x40);
    rdx = rsp.wrapping_add(0x20);
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rax = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64;
    rax = ((rax).wrapping_add(crate::simd_ops::ru64(rbx))) as u64;
    fl_l = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; fl_r = rax;
    if (((fl_l as u64)) == ((fl_r as u64))) { rsp = ((rsp).wrapping_add(0x30u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64;
    rbx = (crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64;
    fl_l = rbx; fl_r = rbx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = rsp.wrapping_add(0x40); rcx = rbx; crate::simd_ops::wu8(rbx.wrapping_add(P4), (0x1u8) as u8); rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P5))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rax)) as u64; crate::simd_ops::wu64(rbx.wrapping_add(P6), rcx); crate::simd_ops::wu64(rbx.wrapping_add(P7), rcx); }
    rsp = ((rsp).wrapping_add(0x30u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1418662a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x140209990; const P1: u64 = 0x1402098b0; const P2: u64 = 0x8; const P3: u64 = 0x30; const P4: u64 = 0x44; const P5: u64 = 0x3c;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; r14 = r8; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rdx); rsi = rdx; r8 = rsp.wrapping_add(0x60); rbp = rcx; rdx = rsp.wrapping_add(0x20); rcx = ((rcx).wrapping_add(0x338u64)) as u64; rdi = r9; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x350u64))) as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(rbp.wrapping_add(0x338u64)))) as u64; fl_l = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 5; continue; } bb = 1; continue; }
            1 => { rax = (((rdi as u16)) as u32) as u32 as u64; fl_l = ((rdi).wrapping_shr(16u32)) as u64; rdi = ((fl_l as u64)) as u64; rcx = (((rdi as u16)) as u32) as u32 as u64; bb = 2; continue; }
            2 => { fl_l = r14; fl_r = (0x1u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu32(rsp.wrapping_add(0x64u64), ((rcx as u32)) as u32); rdx = rsp.wrapping_add(0x20); crate::simd_ops::wu32(rsp.wrapping_add(0x60u64), ((rax as u32)) as u32); rcx = rbp; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x3u32) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rsi); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            5 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rdx = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; rax = (((rdi as u16)) as u32) as u32 as u64; rdi = ((rdi).wrapping_shr(16u32)) as u64; rcx = (((rdi as u16)) as u32) as u32 as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 6; continue; }
            6 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(P3))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 2; continue; } bb = 7; continue; }
            7 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(P4))) as u8 as u64; fl_r = (0xcu8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 4; continue; } bb = 8; continue; }
            8 => { rcx = ((rcx).wrapping_shl(32u32)) as u64; fl_l = (((rcx) | (rax))) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; crate::simd_ops::wu64(rdx.wrapping_add(P5), rcx); bb = 4; continue; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141866380(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x160; const P1: u64 = 0x1426455a8; const P2: u64 = 0x10; const P3: u64 = 0x140209af0; const P4: u64 = 0x1; const P5: u64 = 0x1401deb40; const P6: u64 = 0x142645598;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rbp = rcx; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x0u32) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rdx); rbx = ((rcx.wrapping_add(P0)) as u64) as u64; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r14 = (crate::simd_ops::ru32(rbp.wrapping_add(0x170u64))) as u32 as u64; r14 = (((r14 as u32)).wrapping_add(0x7u32)) as u32 as u64; r14 = ((((r14 as u32)) & (0xfffffff8u32))) as u32 as u64; rdx = ((r14.wrapping_add(P2)) as u64) as u64; rcx = rbp.wrapping_add(0x168); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x168u64))) as u64; xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; crate::simd_ops::wu128(r14.wrapping_add(rax), (xmm0) as u128); rbp = ((rbp).wrapping_add(0x178u64)) as u64; rdx = (crate::simd_ops::ru32(rbp.wrapping_add(0x8u64))) as u32 as u64; r8 = (crate::simd_ops::ru32(rbp.wrapping_add(0xcu64))) as u32 as u64; rcx = ((r8.wrapping_mul(4)) as u64) as u64; rax = ((rdx.wrapping_mul(4)) as u64) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = ((rdx.wrapping_add(P4)) as u64) as u64; rdx = ((r8 as u32)) as u32 as u64; rdx = ((rdx).wrapping_shr(1u32)) as u64; rdx = ((rdx).wrapping_add(r8)) as u64; fl_l = rdx; fl_r = rax; rdx = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { rdx }) as u64; rcx = rbp; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru32(rbp.wrapping_add(0x8u64))) as u32 as u64; rax = (crate::simd_ops::ru64(rbp)) as u64; crate::simd_ops::wu32(rax.wrapping_add(rcx.wrapping_mul(4)), ((r14 as u32)) as u32); crate::simd_ops::lk_add_u32(rbp.wrapping_add(0x8u64), (1) as u32); rcx = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141866470(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x338; const P1: u64 = 0x140209990; const P2: u64 = 0x18; const P3: u64 = 0x10; const P4: u64 = 0xfffffffffffffff0; const P5: u64 = 0x1; const P6: u64 = 0x28;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rbx = ((rcx.wrapping_add(P0)) as u64) as u64; rcx = rbx; r8 = rsp.wrapping_add(0x48); rdx = rsp.wrapping_add(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64; r9 = (crate::simd_ops::ru64(rbx)) as u64; rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rax = ((r9.wrapping_add(rcx)) as u64) as u64; fl_l = rdx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 10; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::lk_sub_u64(rbx.wrapping_add(P3), (1) as u64); r10 = rdx; xmm0 = (crate::simd_ops::ru128(rdx)) as u128; xmm1 = crate::simd_ops::vpsignb(xmm0, xmm0); r10 = ((r10).wrapping_sub(r9)) as u64; rax = (crate::simd_ops::vpmovmskb(xmm1) as u32) as u32 as u64; r11 = ((r10.wrapping_add(P4)) as u64) as u64; rcx = (((rcx) & (r11))) as u64; xmm0 = (crate::simd_ops::ru128(rcx.wrapping_add(r9))) as u128; xmm1 = crate::simd_ops::vpsignb(xmm0, xmm0); rcx = (crate::simd_ops::vpmovmskb(xmm1) as u32) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 8; continue; } bb = 2; continue; }
            2 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 8; continue; } bb = 3; continue; }
            3 => { let bit_in: u32 = (rax as u32); if bit_in != 0 { r8 = ((bit_in.trailing_zeros() as u32)) as u32 as u64; } fl_l = bit_in as u64; rcx = (((rcx as u32)).wrapping_shl(16u32)) as u32 as u64; let bit_in: u32 = (rcx as u32); if bit_in != 0 { rdx = (((31u32.wrapping_sub(bit_in.leading_zeros())) as u32)) as u32 as u64; } fl_l = bit_in as u64; if (((fl_l as u32)) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = 0x1f; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 6; continue; }
            5 => { rax = 0x20; bb = 6; continue; }
            6 => { rax = (((rax as u32)).wrapping_add((r8 as u32))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x10; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 8; continue; } bb = 7; continue; }
            7 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 9; continue; }
            8 => { r8 = (r8 & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 9; continue; }
            9 => { fl_l = ((r8 as u8)) as u8 as u64; fl_r = ((r8 as u8)) as u8 as u64; rax = 0xffffff80; rdx = 0xfffffffe; rdx = (if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { (rax as u32) } else { (rdx as u32) }) as u32 as u64; crate::simd_ops::wu8(r9.wrapping_add(r10), ((rdx as u8)) as u8); rax = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64; rcx = rax; rax = ((((rax as u32)) & (0xfu32))) as u32 as u64; rcx = (((rcx) & (r11))) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu8(rcx.wrapping_add(rax).wrapping_add(P5), ((rdx as u8)) as u8); rax = (((r8 as u8)) as u32) as u32 as u64; fl_l = (crate::simd_ops::lk_add_u64(rbx.wrapping_add(P6), (rax) as u64)) as u64; fl_r = fl_l; bb = 10; continue; }
            10 => { rsp = ((rsp).wrapping_add(0x30u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141866550(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x140209da0; const P1: u64 = 0x8;
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x40u64)) as u64;
    rbx = (crate::simd_ops::ru64(r8)) as u64;
    rdx = rsp.wrapping_add(0x20);
    r8 = rsp.wrapping_add(0x58);
    rcx = ((rcx).wrapping_add(0x338u64)) as u64;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64;
    crate::simd_ops::wu64(rax.wrapping_add(P1), rbx);
    rsp = ((rsp).wrapping_add(0x40u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141866590(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142646148; const P1: u64 = 0x1426456f8; const P2: u64 = 0x1426455a8; const P3: u64 = 0x140209af0; const P4: u64 = 0x1401deb40; const P5: u64 = 0x142645598;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0; let mut ymm1_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 19>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r11 = rsp; crate::simd_ops::wu64(r11.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(r11.wrapping_add(0x20u64), rbp); crate::simd_ops::wu64(r11.wrapping_add(0x18u64), r8); crate::simd_ops::wu64(r11.wrapping_add(0x10u64), rdx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x80u64)) as u64; rax = r9; rbp = rcx; crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), (0x10u32) as u32); crate::simd_ops::wu64(r11.wrapping_sub(0x20u64), (0x0u64) as u64); crate::simd_ops::wu32(r11.wrapping_add(0x18u64), (0x40u32) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x18u32) as u32); r9 = r11.wrapping_add(0x18); r8 = r11.wrapping_sub(0x64); rdx = 0x10000003; rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rsp.wrapping_add(0xa8); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xa8u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), rax); rbx = rbp.wrapping_add(0x160); rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r14 = (crate::simd_ops::ru32(rbp.wrapping_add(0x170u64))) as u32 as u64; r14 = (((r14 as u32)).wrapping_add(0x7u32)) as u32 as u64; r14 = ((((r14 as u32)) & (0xfffffff8u32))) as u32 as u64; rdx = r14.wrapping_add(0x50); rcx = rbp.wrapping_add(0x168); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x168u64))) as u64; let y_addr: u64 = rsp.wrapping_add(0x30u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = r14.wrapping_add(rax); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); let y_addr: u64 = rsp.wrapping_add(0x50u64); xmm1 = crate::simd_ops::ru128(y_addr); ymm1_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = r14.wrapping_add(rax).wrapping_add(0x20u64); crate::simd_ops::wu128(y_addr, xmm1); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm1_hi); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x70u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(r14.wrapping_add(rax).wrapping_add(0x40u64), (xmm0) as u128); rbp = ((rbp).wrapping_add(0x178u64)) as u64; rdx = (crate::simd_ops::ru32(rbp.wrapping_add(0x8u64))) as u32 as u64; r8 = (crate::simd_ops::ru32(rbp.wrapping_add(0xcu64))) as u32 as u64; rcx = ((r8.wrapping_mul(4)) as u64) as u64; rax = ((rdx.wrapping_mul(4)) as u64) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = rdx.wrapping_add(0x1); rdx = ((r8 as u32)) as u32 as u64; rdx = ((rdx).wrapping_shr(1u32)) as u64; rdx = ((rdx).wrapping_add(r8)) as u64; fl_l = rdx; fl_r = rax; rdx = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { rdx }) as u64; rcx = rbp; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru32(rbp.wrapping_add(0x8u64))) as u32 as u64; rax = (crate::simd_ops::ru64(rbp)) as u64; crate::simd_ops::wu32(rax.wrapping_add(rcx.wrapping_mul(4)), ((r14 as u32)) as u32); crate::simd_ops::lk_add_u32(rbp.wrapping_add(0x8u64), (1) as u32); rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r11 = rsp.wrapping_add(0x80); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rbp = (crate::simd_ops::ru64(r11.wrapping_add(0x38u64))) as u64; rsp = r11; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1418666e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x368; const P1: u64 = 0x370; const P2: u64 = 0x5; const P3: u64 = 0x4;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; fl_r = (0x0u64) as u64;
    if (((fl_l as u64)) == ((fl_r as u64))) { return rax; }
    rdx = ((rcx.wrapping_add(P1)) as u64) as u64;
    r8 = 0;
    fl_l = 0; fl_r = 0;
    rax = ((rdx.wrapping_add(P2)) as u64) as u64;
    fl_l = rdx; fl_r = rax;
    if !((((fl_l as u64)) == ((fl_r as u64)))) { crate::simd_ops::wu32(rdx, ((r8 as u32)) as u32); crate::simd_ops::wu8(rdx.wrapping_add(P3), ((r8 as u8)) as u8); }
    crate::simd_ops::wu64(rcx.wrapping_add(P0), r8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141866720(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x140209990; const P1: u64 = 0x142646198; const P2: u64 = 0x1438583d4; const P3: u64 = 0x1438583d8; const P4: u64 = 0x142646088;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rsi = 0; fl_l = 0; fl_r = 0; rdi = r8; rbx = rdx; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 1; continue; }
            1 => { rsi = rcx.wrapping_add(0x338); bb = 2; continue; }
            2 => { r8 = rsp.wrapping_add(0x48); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rbx); rdx = rsp.wrapping_add(0x20); rcx = rsi; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x18u64))) as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(rsi))) as u64; fl_l = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 5; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::wu8(rcx.wrapping_add(0x48u64), (0x0u8) as u8); rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0xd8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 7; continue; } bb = 5; continue; }
            5 => { rdx = 0x4; rcx = rbx; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); rbx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru32(P2)) as u32 as u64; rcx = 0x1; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; crate::simd_ops::wu8(P3, ((rcx as u8)) as u8); rax = (if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { (rcx as u32) } else { (rax as u32) }) as u32 as u64; crate::simd_ops::wu32(P2, ((rax as u32)) as u32); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            7 => { rsi = rbx; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 8; continue; }
            8 => { fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 9; continue; }
            9 => { fl_l = rdi; fl_r = rbx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 6; continue; } bb = 10; continue; }
            10 => { rdx = 0x4; rcx = rdi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); rdi = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 9; continue; } bb = 11; continue; }
            11 => { rax = (crate::simd_ops::ru32(P2)) as u32 as u64; rcx = 0x1; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; crate::simd_ops::wu8(P3, ((rcx as u8)) as u8); rax = (if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { (rcx as u32) } else { (rax as u32) }) as u32 as u64; crate::simd_ops::wu32(P2, ((rax as u32)) as u32); bb = 13; continue; }
            12 => { rax = (crate::simd_ops::ru32(P2)) as u32 as u64; rcx = 0x1; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; crate::simd_ops::wu8(P3, ((rcx as u8)) as u8); rax = (if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { (rcx as u32) } else { (rax as u32) }) as u32 as u64; crate::simd_ops::wu32(P2, ((rax as u32)) as u32); fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 13; continue; }
            13 => { rdx = 0x9; rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); bb = 14; continue; }
            14 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141866870(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x140209990; const P1: u64 = 0x142646198; const P2: u64 = 0x1438583d4; const P3: u64 = 0x1438583d8; const P4: u64 = 0x142646088;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rbp = 0; fl_l = 0; fl_r = 0; rdi = r8; rbx = rdx; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 16; continue; } bb = 1; continue; }
            1 => { rsi = rcx.wrapping_add(0x338); bb = 2; continue; }
            2 => { r8 = rsp.wrapping_add(0x48); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rbx); rdx = rsp.wrapping_add(0x20); rcx = rsi; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x18u64))) as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(rsi))) as u64; fl_l = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 6; continue; } bb = 4; continue; }
            4 => { rdx = 0x4; rcx = rbx; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); rbx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 5; continue; }
            5 => { bb = 8; continue; }
            6 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0x48u64))) as u8 as u64; fl_r = ((rbp as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 9; continue; } bb = 7; continue; }
            7 => { crate::simd_ops::wu8(rcx.wrapping_add(0x48u64), ((rbp as u8)) as u8); bb = 8; continue; }
            8 => { rax = (crate::simd_ops::ru32(P2)) as u32 as u64; rcx = 0x1; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; crate::simd_ops::wu8(P3, ((rbp as u8)) as u8); rax = (if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { (rcx as u32) } else { (rax as u32) }) as u32 as u64; crate::simd_ops::wu32(P2, ((rax as u32)) as u32); bb = 18; continue; }
            9 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0xd8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 16; continue; } bb = 10; continue; }
            10 => { rbp = rbx; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 16; continue; } bb = 11; continue; }
            11 => { fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 16; continue; } bb = 12; continue; }
            12 => { fl_l = rdi; fl_r = rbx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 15; continue; } bb = 13; continue; }
            13 => { rdx = 0x4; rcx = rdi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); rdi = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 12; continue; } bb = 14; continue; }
            14 => { rax = (crate::simd_ops::ru32(P2)) as u32 as u64; rcx = 0x1; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; crate::simd_ops::wu8(P3, ((rdi as u8)) as u8); rax = (if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { (rcx as u32) } else { (rax as u32) }) as u32 as u64; crate::simd_ops::wu32(P2, ((rax as u32)) as u32); bb = 17; continue; }
            15 => { rax = (crate::simd_ops::ru32(P2)) as u32 as u64; rcx = 0x1; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; crate::simd_ops::wu8(P3, (0x0u8) as u8); rax = (if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { (rcx as u32) } else { (rax as u32) }) as u32 as u64; crate::simd_ops::wu32(P2, ((rax as u32)) as u32); bb = 18; continue; }
            16 => { rax = (crate::simd_ops::ru32(P2)) as u32 as u64; rcx = 0x1; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; crate::simd_ops::wu8(P3, (0x0u8) as u8); rax = (if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { (rcx as u32) } else { (rax as u32) }) as u32 as u64; crate::simd_ops::wu32(P2, ((rax as u32)) as u32); fl_l = rbp; fl_r = rbp; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 18; continue; } bb = 17; continue; }
            17 => { rdx = 0x6; rcx = rbp; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); bb = 18; continue; }
            18 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1418669f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142646118; const P1: u64 = 0x140209990; const P2: u64 = 0x142646068; const P3: u64 = 0x142646138; const P4: u64 = 0x1438583ca; const P5: u64 = 0x143c8c608; const P6: u64 = 0x143858388; const P7: u64 = 0x1433dd168; const P8: u64 = 0x142647308;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rbp = ((((r9 as u32)) as i32 as i64) as u64) as u64; rbx = ((r8 as u32)) as u32 as u64; rsi = rdx; rdi = rcx; r14 = ((rcx.wrapping_add(rbp)) as u64) as u64; fl_l = (crate::simd_ops::ru8(r14.wrapping_add(0x370u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 5; continue; } bb = 1; continue; }
            1 => { fl_l = ((rbx as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 5; continue; } bb = 2; continue; }
            2 => { r9 = rcx.wrapping_add(0x370); rax = r9.wrapping_add(0x5); fl_l = r9; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu32(r9, (0x0u32) as u32); crate::simd_ops::wu8(r9.wrapping_add(0x4u64), (0x0u8) as u8); bb = 4; continue; }
            4 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rdi.wrapping_add(0x368u64), (0x0u64) as u64); bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x368u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 12; continue; } bb = 6; continue; }
            6 => { fl_l = ((rbx as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 12; continue; } bb = 7; continue; }
            7 => { r8 = rsp.wrapping_add(0x50); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rsi); rdx = rsp.wrapping_add(0x20); rcx = rdi.wrapping_add(0x338); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x350u64))) as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(rdi.wrapping_add(0x338u64)))) as u64; fl_l = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 11; continue; } bb = 8; continue; }
            8 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x44u64))) as u32 as u64; rax = (((rax as u32)).wrapping_shr(9u32)) as u32 as u64; fl_l = ((rbx as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); bb = 11; continue; }
            11 => { rcx = rsi; crate::simd_ops::wu64(rdi.wrapping_add(0x368u64), rsi); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu8(r14.wrapping_add(0x370u64), (0x1u8) as u8); bb = 20; continue; }
            12 => { fl_l = rax; fl_r = rsi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 19; continue; } bb = 13; continue; }
            13 => { fl_l = ((rbx as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 15; continue; } bb = 14; continue; }
            14 => { crate::simd_ops::wu8(r14.wrapping_add(0x370u64), ((rbx as u8)) as u8); bb = 20; continue; }
            15 => { rax = rdi.wrapping_add(0x370); crate::simd_ops::wu8(r14.wrapping_add(0x370u64), (0x0u8) as u8); rcx = rax.wrapping_add(0x5); fl_l = rax; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 18; continue; } bb = 16; continue; }
            16 => { fl_l = (crate::simd_ops::ru8(rax)) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 26; continue; } bb = 17; continue; }
            17 => { rax = ((rax).wrapping_add(1)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 16; continue; } bb = 18; continue; }
            18 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rdi.wrapping_add(0x368u64), (0x0u64) as u64); bb = 26; continue; }
            19 => { fl_l = ((rbx as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 26; continue; } bb = 20; continue; }
            20 => { fl_l = ((rbp as u32)) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 26; continue; } bb = 21; continue; }
            21 => { fl_l = (crate::simd_ops::ru8(P4)) as u8 as u64; fl_r = ((rbp as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 26; continue; } bb = 22; continue; }
            22 => { rax = (crate::simd_ops::ru64(P5)) as u64; rbx = (crate::simd_ops::ru64(P6)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 25; continue; } bb = 23; continue; }
            23 => { r8 = P5; rdx = P7; rcx = ((rbp.wrapping_add(0x4u64)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P8), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 26; continue; } bb = 24; continue; }
            24 => { rax = (crate::simd_ops::ru64(P5)) as u64; bb = 25; continue; }
            25 => { rdx = rbx; rcx = 0x7; rax = crate::simd_ops::call_stk(rax, rax, rcx, rdx, r8, r9, rsp); bb = 26; continue; }
            26 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141866b80(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142646140;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rcx = ((rdx as u32)) as u32 as u64;
    rdx = 0x3;
    rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp);
    fl_l = ((rax as u32)) as u32 as u64; fl_r = 0xa1;
    rax = (rax & !0xffu64) | (((if (((fl_l as u32)) == ((fl_r as u32))) { 1u8 } else { 0u8 }) as u8 as u64));
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141866bb0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x20; const P1: u64 = 0x142646188; const P2: u64 = 0x18; const P3: u64 = 0x142646080; const P4: u64 = 0x142646120; const P5: u64 = 0x1c; const P6: u64 = 0x10;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rdi = rdx; rax = ((((r8 as u16)) as i16 as i32) as u32) as u32 as u64; r8 = ((r8).wrapping_shr(16u32)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), (0x0u64) as u64); crate::simd_ops::wu32(rsp.wrapping_add(0x50u64), ((rax as u32)) as u32); rax = ((((r8 as u16)) as i16 as i32) as u32) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x54u64), ((rax as u32)) as u32); fl_l = ((r9 as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { fl_l = ((r9 as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 4; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rdx = rsp.wrapping_add(0x50); rcx = rdi; crate::simd_ops::wu64(rbx.wrapping_add(P0), rax); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(P2), rax); bb = 4; continue; }
            3 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rdx = rsp.wrapping_add(0x50); rcx = rdi; crate::simd_ops::wu64(rbx.wrapping_add(P2), rax); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(P0), rax); bb = 4; continue; }
            4 => { xmm0 = 0u128; rdx = rsp.wrapping_add(0x20); rcx = rdi; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x28u64))) as u32 as u64; rax = (((rax as u32)).wrapping_sub(crate::simd_ops::ru32(rsp.wrapping_add(0x20u64)))) as u32 as u64; xmm1 = 0u128; xmm1 = (xmm1 & !0xffff_ffffu128) | ((((crate::simd_ops::ru32(rbx.wrapping_add(P2))) as i32) as f32).to_bits() as u128); xmm0 = 0u128; xmm0 = (xmm0 & !0xffff_ffffu128) | (((((rax as u32)) as i32) as f32).to_bits() as u128); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x2cu64))) as u32 as u64; rax = (((rax as u32)).wrapping_sub(crate::simd_ops::ru32(rsp.wrapping_add(0x24u64)))) as u32 as u64; xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) / f32::from_bits(xmm0 as u32)).to_bits() as u128); crate::simd_ops::wu32(rsp.wrapping_add(0x60u64), ((xmm1 as u32)) as u32); xmm0 = 0u128; xmm0 = (xmm0 & !0xffff_ffffu128) | (((((rax as u32)) as i32) as f32).to_bits() as u128); xmm2 = 0u128; xmm2 = (xmm2 & !0xffff_ffffu128) | ((((crate::simd_ops::ru32(rbx.wrapping_add(P5))) as i32) as f32).to_bits() as u128); xmm1 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) / f32::from_bits(xmm0 as u32)).to_bits() as u128); crate::simd_ops::wu32(rsp.wrapping_add(0x64u64), ((xmm1 as u32)) as u32); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(P6), rax); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141866cb0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142646158; const P1: u64 = 0x142646130;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 39>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x130u64)) as u64; rsi = 0; fl_l = 0; fl_r = 0; rbx = ((r8 as u32)) as u32 as u64; rdi = ((rdx as u32)) as u32 as u64; fl_l = ((r9 as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = rsp.wrapping_add(0x30); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); r9 = rsp.wrapping_add(0x158); crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0x4u32) as u32); r8 = rsp.wrapping_add(0x30); crate::simd_ops::wu16(rsp.wrapping_add(0x158u64), ((rsi as u16)) as u16); rdx = ((rbx as u32)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x1u32) as u32); rcx = ((rdi as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; rax = ((crate::simd_ops::ru16(rsp.wrapping_add(0x158u64))) as u32) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rax = (((rsi as u16)) as u32) as u32 as u64; bb = 3; continue; }
            3 => { r11 = rsp.wrapping_add(0x130); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x10u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x18u64))) as u64; rsp = r11; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141867840(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142646208;
    let (mut rax, mut fl_l) = (crate::simd_ops::get_rax_in(), 0u64);
    rcx = rdx;
    fl_l = ((rdx).wrapping_shr(32u32)) as u64;
    rdx = ((fl_l as u64)) as u64;
    return crate::simd_ops::call_fwd(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141867870(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142646110;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rcx = ((crate::simd_ops::ru8(rcx)) as u32) as u32 as u64;
    return crate::simd_ops::call_fwd(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141867880(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x378; const P1: u64 = 0x390; const P2: u64 = 0x1418682d0; const P3: u64 = 0x1f0; const P4: u64 = 0x140209ce0; const P5: u64 = 0x18; const P6: u64 = 0x10; const P7: u64 = 0x14186a310; const P8: u64 = 0x8; const P9: u64 = 0x142442de0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x38u64)) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x50u64), (0x0u32) as u32); fl_l = ((rdx as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::lk_add_u32(rcx.wrapping_add(P0), (1) as u32)) as u32 as u64; bb = 3; continue; }
            2 => { fl_l = (crate::simd_ops::lk_sub_u32(rcx.wrapping_add(P0), (1) as u32)) as u32 as u64; bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(P1))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rsp = ((rsp).wrapping_add(0x38u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P2, rax, rcx, rdx, r8, r9, rsp); }
            5 => { crate::simd_ops::wu8(rsp.wrapping_add(0x58u64), ((rdx as u8)) as u8); rbx = ((rcx.wrapping_add(P3)) as u64) as u64; r8 = rsp.wrapping_add(0x58); rcx = rsp.wrapping_add(0x60); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; crate::simd_ops::wu8(rax.wrapping_add(P5), (0x1u8) as u8); rdi = (crate::simd_ops::ru64(rax.wrapping_add(P6))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0x0u32) as u32); crate::simd_ops::lk_add_u32(rdi, (1) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x50u64), (0x2u32) as u32); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), (0x0u64) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), rax); rdx = rsp.wrapping_add(0x68); rcx = rbx; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 6; continue; }
            6 => { rax = 0xffffffff; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rdi, xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 10; continue; } bb = 7; continue; }
            7 => { rbx = (crate::simd_ops::ru64(rdi.wrapping_add(P8))) as u64; fl_l = (((rbx) & (0xfffffffffffffffcu64))) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P8))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); bb = 9; continue; }
            9 => { rcx = rdi; rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); bb = 10; continue; }
            10 => { rsp = ((rsp).wrapping_add(0x38u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141867970(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142646108; const P1: u64 = 0x142646170; const P2: u64 = 0x338; const P3: u64 = 0x140209990; const P4: u64 = 0x350; const P5: u64 = 0x8; const P6: u64 = 0x390; const P7: u64 = 0x378; const P8: u64 = 0x388; const P9: u64 = 0x142646128; const P10: u64 = 0x50; const P11: u64 = 0x380;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rdi = rcx; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), (0x0u64) as u64); rcx = rsp.wrapping_add(0x48); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 9; continue; } bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rbx); rbx = (crate::simd_ops::ru64(rdi)) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); r8 = rsp.wrapping_add(0x50); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rax); rdx = rsp.wrapping_add(0x20); rcx = ((rbx.wrapping_add(P2)) as u64) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rbx.wrapping_add(P4))) as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(rbx.wrapping_add(P2)))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; fl_l = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 9; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rdx = (crate::simd_ops::ru64(rax.wrapping_add(P5))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(rdi)) as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(P6))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 6; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru32(rcx.wrapping_add(P7))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) >= 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P8))) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P9), rax), rax, rcx, rdx, r8, r9, rsp); rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            6 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(P10))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { rcx = rax; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P9), rax), rax, rcx, rdx, r8, r9, rsp); rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            8 => { rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P11))) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P9), rax), rax, rcx, rdx, r8, r9, rsp); bb = 9; continue; }
            9 => { rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141867a50(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143080b48; const P1: u64 = 0x14020af00; const P2: u64 = 0x14186a310; const P3: u64 = 0x142442de0; const P4: u64 = 0x142645748; const P5: u64 = 0x142646178; const P6: u64 = 0x142646168; const P7: u64 = 0x1401e5070; const P8: u64 = 0x14186a0c0; const P9: u64 = 0x143406c90; const P10: u64 = 0x1423f0780; const P11: u64 = 0x14136eec0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rdi = rcx; rax = P0; crate::simd_ops::wu64(rcx, rax); rcx = rsp.wrapping_add(0x40); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; crate::simd_ops::wu8(rax.wrapping_add(0x18u64), (0x1u8) as u8); rsi = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; crate::simd_ops::lk_add_u32(rsi, (1) as u32); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rax); rdx = rsp.wrapping_add(0x48); rcx = rdi.wrapping_add(0x1f0); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 1; continue; }
            1 => { rax = 0xffffffff; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rsi, xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 5; continue; } bb = 2; continue; }
            2 => { rbx = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; fl_l = (((rbx) & (0xfffffffffffffffcu64))) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { rcx = rsi; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            5 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x2f8u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { rdx = 0xffffffff; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { rcx = ((crate::simd_ops::ru16(rdi.wrapping_add(0x330u64))) as u32) as u32 as u64; rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x58u64))) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x388u64))) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rdi.wrapping_add(0x338); rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rcx = rdi; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P8, rax, rcx, rdx, r8, r9, rsp); }
            8 => { rdx = P9; rcx = rsp.wrapping_add(0x20); rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp); rcx = rax; rax = crate::simd_ops::call_stk(P11, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x141867b51, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141867bb0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14397d820; const P1: u64 = 0x143080ca8; const P2: u64 = 0x142645728; const P3: u64 = 0x14183acc2; const P4: u64 = 0x143080c60; const P5: u64 = 0x1426469b8; const P6: u64 = 0x143080c78; const P7: u64 = 0x143080b48; const P8: u64 = 0x1430782a0; const P9: u64 = 0x142646190; const P10: u64 = 0x142646180; const P11: u64 = 0x140209810; const P12: u64 = 0x1426462b8; const P13: u64 = 0x14241c190; const P14: u64 = 0x143080c28; const P15: u64 = 0x142645008; const P16: u64 = 0x143080c18; const P17: u64 = 0x142645010; const P18: u64 = 0x142645000; const P19: u64 = 0x1426456e0; const P20: u64 = 0x142645698; const P21: u64 = 0x141867b60; const P22: u64 = 0x143080c08; const P23: u64 = 0x142646240; const P24: u64 = 0x142645208; const P25: u64 = 0x143080be8; const P26: u64 = 0x142646150; const P27: u64 = 0x1423f0b30;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 65>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x141867bb0, funcinfo_rva: 0x36f9c8c, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x208, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rcx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r12) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r13) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r15) as u64); rbp = ((rsp.wrapping_sub(0xc8u64)) as u64) as u64; rsp = ((rsp).wrapping_sub(0x1c8u64)) as u64; r15 = (rcx) as u64; xmm0 = 0u128; ymm0_hi = 0; crate::simd_ops::wu128(rcx.wrapping_add(0x8u64), (xmm0) as u128); rbx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rcx.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rcx.wrapping_add(0x10u64), (rbx) as u64); crate::simd_ops::wu64(rcx.wrapping_add(0x18u64), (rbx) as u64); crate::simd_ops::wu64(rcx.wrapping_add(0x20u64), (rbx) as u64); crate::simd_ops::wu128(rcx.wrapping_add(0x28u64), (xmm0) as u128); crate::simd_ops::wu64(rcx.wrapping_add(0x28u64), (rbx) as u64); crate::simd_ops::wu64(rcx.wrapping_add(0x30u64), (rbx) as u64); crate::simd_ops::wu64(rcx.wrapping_add(0x38u64), (rbx) as u64); crate::simd_ops::wu64(rcx.wrapping_add(0x40u64), (rbx) as u64); crate::simd_ops::wu128(rcx.wrapping_add(0x48u64), (xmm0) as u128); crate::simd_ops::wu64(rcx.wrapping_add(0x48u64), (rbx) as u64); crate::simd_ops::wu64(rcx.wrapping_add(0x50u64), (rbx) as u64); crate::simd_ops::wu64(P0, (rcx) as u64); rax = ((P1) as u64) as u64; crate::simd_ops::wu64(rcx, (rax) as u64); rcx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141867c23); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P2)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu64(r15.wrapping_add(0x58u64), (rax) as u64); rcx = ((r15.wrapping_add(0x60u64)) as u64) as u64; rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; r8 = (0x100u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141867c39); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu64(r15.wrapping_add(0x160u64), (rbx) as u64); xmm0 = 0u128; ymm0_hi = 0; let y_addr: u64 = r15.wrapping_add(0x168u64); core::ptr::write_unaligned(y_addr as *mut u128, xmm0); core::ptr::write_unaligned(y_addr.wrapping_add(16) as *mut u128, ymm0_hi); crate::simd_ops::wu64(r15.wrapping_add(0x168u64), (rbx) as u64); crate::simd_ops::wu64(r15.wrapping_add(0x170u64), (rbx) as u64); crate::simd_ops::wu64(r15.wrapping_add(0x178u64), (rbx) as u64); crate::simd_ops::wu64(r15.wrapping_add(0x180u64), (rbx) as u64); crate::simd_ops::wu64(r15.wrapping_add(0x188u64), (rbx) as u64); crate::simd_ops::wu8(r15.wrapping_add(0x1c8u64), ((rbx as u8)) as u8); let y_addr: u64 = r15.wrapping_add(0x1d0u64); core::ptr::write_unaligned(y_addr as *mut u128, xmm0); core::ptr::write_unaligned(y_addr.wrapping_add(16) as *mut u128, ymm0_hi); crate::simd_ops::wu64(r15.wrapping_add(0x1d0u64), (rbx) as u64); crate::simd_ops::wu64(r15.wrapping_add(0x1d8u64), (rbx) as u64); crate::simd_ops::wu64(r15.wrapping_add(0x1e0u64), (rbx) as u64); crate::simd_ops::wu64(r15.wrapping_add(0x1e8u64), (rbx) as u64); r13 = ((r15.wrapping_add(0x1f0u64)) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x40u64), (rbx) as u64); crate::simd_ops::wu32(rbp.wrapping_sub(0x38u64), (0xffffffffu32) as u32); crate::simd_ops::wu32(rbp.wrapping_sub(0x34u64), (0x4u32) as u32); crate::simd_ops::wu8(rbp.wrapping_sub(0x30u64), ((rbx as u8)) as u8); rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; r8 = (0xefu32) as u32 as u64; rcx = ((rbp.wrapping_sub(0x2fu64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141867ccc); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r8 = ((P4) as u64) as u64; rdx = (0xefu32) as u32 as u64; rcx = ((rbp.wrapping_sub(0x2fu64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141867ce1); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P5)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = ((r13.wrapping_add(0x8u64)) as u64) as u64; rax = ((rbp.wrapping_sub(0x40u64)) as u64) as u64; rdx = ((rbx.wrapping_add(0x2u64)) as u32) as u32 as u64; bb = 1; continue; }
            1 => { let y_addr: u64 = rax; xmm0 = core::ptr::read_unaligned(y_addr as *const u128); ymm0_hi = core::ptr::read_unaligned(y_addr.wrapping_add(16) as *const u128); let y_addr: u64 = rcx; core::ptr::write_unaligned(y_addr as *mut u128, xmm0); core::ptr::write_unaligned(y_addr.wrapping_add(16) as *mut u128, ymm0_hi); let y_addr: u64 = rax.wrapping_add(0x20u64); xmm0 = core::ptr::read_unaligned(y_addr as *const u128); ymm0_hi = core::ptr::read_unaligned(y_addr.wrapping_add(16) as *const u128); let y_addr: u64 = rcx.wrapping_add(0x20u64); core::ptr::write_unaligned(y_addr as *mut u128, xmm0); core::ptr::write_unaligned(y_addr.wrapping_add(16) as *mut u128, ymm0_hi); let y_addr: u64 = rax.wrapping_add(0x40u64); xmm0 = core::ptr::read_unaligned(y_addr as *const u128); ymm0_hi = core::ptr::read_unaligned(y_addr.wrapping_add(16) as *const u128); let y_addr: u64 = rcx.wrapping_add(0x40u64); core::ptr::write_unaligned(y_addr as *mut u128, xmm0); core::ptr::write_unaligned(y_addr.wrapping_add(16) as *mut u128, ymm0_hi); xmm0 = (crate::simd_ops::ru128(rax.wrapping_add(0x60u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rcx.wrapping_add(0x60u64), (xmm0) as u128); rcx = ((rcx.wrapping_add(0x80u64)) as u64) as u64; xmm1 = (crate::simd_ops::ru128(rax.wrapping_add(0x70u64))) as u128; crate::simd_ops::wu128(rcx.wrapping_sub(0x10u64), (xmm1) as u128); rax = ((rax.wrapping_add(0x80u64)) as u64) as u64; fl_l = (rdx) as u64 as u64; fl_r = (0x1u64) as u64 as u64; rdx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu64(r13.wrapping_add(0x108u64), (rbx) as u64); crate::simd_ops::wu64(r13.wrapping_add(0x110u64), (rbx) as u64); crate::simd_ops::wu8(r13.wrapping_add(0x118u64), ((rbx as u8)) as u8); rax = ((P6) as u64) as u64; crate::simd_ops::wu64(r13, (rax) as u64); crate::simd_ops::wu64(r13.wrapping_add(0x120u64), (rbx) as u64); crate::simd_ops::wu64(r13.wrapping_add(0x128u64), (rbx) as u64); crate::simd_ops::wu64(r13.wrapping_add(0x130u64), (rbx) as u64); crate::simd_ops::wu64(r13.wrapping_add(0x138u64), (rbx) as u64); rax = ((P7) as u64) as u64; crate::simd_ops::wu64(r15, (rax) as u64); rax = ((P8) as u64) as u64; crate::simd_ops::wu64(r15.wrapping_add(0x338u64), (rax) as u64); crate::simd_ops::wu64(r15.wrapping_add(0x340u64), (rbx) as u64); crate::simd_ops::wu64(r15.wrapping_add(0x348u64), (rbx) as u64); crate::simd_ops::wu64(r15.wrapping_add(0x350u64), (rbx) as u64); crate::simd_ops::wu64(r15.wrapping_add(0x360u64), (rbx) as u64); crate::simd_ops::wu64(r15.wrapping_add(0x368u64), (rbx) as u64); rax = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(r15.wrapping_add(0x370u64), ((rax as u32)) as u32); crate::simd_ops::wu8(r15.wrapping_add(0x374u64), ((rax as u8)) as u8); crate::simd_ops::wu32(r15.wrapping_add(0x378u64), ((rbx as u32)) as u32); rdx = (0x7f00u32) as u32 as u64; rcx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141867dce); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P9)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu64(r15.wrapping_add(0x380u64), (rax) as u64); r14 = (crate::simd_ops::ru64(r15.wrapping_add(0x58u64))) as u64; rcx = (0xdu32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141867de4); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P10)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rsi = ((rax as u32)) as u32 as u64; rcx = (0xeu32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141867df1); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P10)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdi = ((rax as u32)) as u32 as u64; rax = ((((rax as u32)) as i32).wrapping_mul(((rsi as u32)) as i32) as u32) as u32 as u64; rdx = (if (rax as u32 as i32) < 0 { 0xffffffffu32 } else { 0u32 }) as u64; rdx = ((((rdx as u32)) & (0x7u32))) as u32 as u64; rax = (((rax as u32)).wrapping_add((rdx as u32))) as u32 as u64; rax = (((((rax as u32)) as i32).wrapping_shr(3u32) as u32)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), (rbx) as u64); rbx = ((((rax as u32)) as i32 as i64) as u64) as u64; rdx = (rbx) as u64; rcx = ((rsp.wrapping_add(0x40u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141867e24); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P11) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = (rbx) as u64; rcx = ((rsp.wrapping_add(0x58u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141867e31); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P11) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r8 = (crate::simd_ops::ru32(rsp.wrapping_add(0x48u64))) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rdx = (0xffu32) as u32 as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141867e48); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r8 = (crate::simd_ops::ru32(rsp.wrapping_add(0x60u64))) as u32 as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rcx = (r12) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141867e5c); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (r12) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (rbx) as u64); crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rdi as u32)) as u32); r9 = ((rsi as u32)) as u32 as u64; r8 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rcx = (r14) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141867e7a); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P12)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdi = (rax) as u64; fl_l = (r12) as u64 as u64; fl_r = (r12) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rcx = (r12) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141867e8b); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P13) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 4; continue; }
            4 => { fl_l = (rbx) as u64 as u64; fl_r = (rbx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141867e99); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P13) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 6; continue; }
            6 => { crate::simd_ops::wu64(r15.wrapping_add(0x388u64), (rdi) as u64); rcx = (0x2001u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141867eab); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P10)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rsi = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { rdi = (rdi & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 15; continue; }
            8 => { crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), (rsi) as u64); rax = ((rsp.wrapping_add(0x40u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rax) as u64); r9 = (0x20019u32) as u32 as u64; r8 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rdx = ((P14) as u64) as u64; rcx = (0xffffffff80000002u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141867ee5); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P15)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 12; continue; } bb = 9; continue; }
            9 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x128u64), ((rsi as u32)) as u32); crate::simd_ops::wu32(rbp.wrapping_add(0x118u64), ((rsi as u32)) as u32); crate::simd_ops::wu32(rbp.wrapping_add(0x120u64), (0x4u32) as u32); rax = ((rbp.wrapping_add(0x120u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (rax) as u64); rax = ((rbp.wrapping_add(0x118u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rax) as u64); r9 = ((rbp.wrapping_add(0x128u64)) as u64) as u64; r8 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rdx = ((P16) as u64) as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141867f36); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P17)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 13; continue; } bb = 10; continue; }
            10 => { rcx = (rbx) as u64; bb = 11; continue; }
            11 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141867f43); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P18)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 12; continue; }
            12 => { rdi = (rdi & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 15; continue; }
            13 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141867f4e); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P19)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = ((rax as u32)) as u32 as u64; rdx = ((rsp.wrapping_add(0x50u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141867f5b); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P20)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (rbx) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 11; continue; } bb = 14; continue; }
            14 => { rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x118u64))) as u32 as u64; fl_l = (crate::simd_ops::ru32(rsp.wrapping_add(0x50u64))) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; rdi = (rdi & !0xffu64) | (((if (((fl_l as u32)) != ((fl_r as u32))) { 1u8 } else { 0u8 }) as u8 as u64)); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141867f76); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P18)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 15; continue; }
            15 => { crate::simd_ops::wu8(r15.wrapping_add(0x390u64), ((rdi as u8)) as u8); rcx = (0x17u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141867f88); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P10)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; rax = (rax & !0xffu64) | (((if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { 1u8 } else { 0u8 }) as u8 as u64)); crate::simd_ops::wu8(r15.wrapping_add(0x391u64), ((rax as u8)) as u8); crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), (0x8u64) as u64); rax = ((P21) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), (rax) as u64); crate::simd_ops::wu64(rbp.wrapping_sub(0x80u64), (0x0u64) as u64); rcx = (crate::simd_ops::ru64(r15.wrapping_add(0x58u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x78u64), (rcx) as u64); rdx = ((P22) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141867fc6); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P23)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu64(rbp.wrapping_sub(0x70u64), (rax) as u64); rax = (crate::simd_ops::ru64(r15.wrapping_add(0x380u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x68u64), (rax) as u64); rcx = (0x4u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141867fe0); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P24)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu64(rbp.wrapping_sub(0x60u64), (rax) as u64); crate::simd_ops::wu64(rbp.wrapping_sub(0x58u64), (rsi) as u64); rax = ((P25) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x50u64), (rax) as u64); rcx = ((rsp.wrapping_add(0x70u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141867ffe); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P26)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu16(r15.wrapping_add(0x330u64), ((rax as u16)) as u16); rcx = (r13) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14186800f); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P27) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = (r15) as u64; rsp = ((rsp).wrapping_add(0x1c8u64)) as u64; r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r13 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r12 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rsi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141868040(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142646180; const P1: u64 = 0x140209810; const P2: u64 = 0x14183acc2; const P3: u64 = 0x1426462b8; const P4: u64 = 0x14241c190;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x60u64)) as u64; rbp = rcx; rcx = 0xd; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); rsi = ((rax as u32)) as u32 as u64; rcx = 0xe; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); rdi = ((rax as u32)) as u32 as u64; rax = ((((rax as u32)) as i32).wrapping_mul(((rsi as u32)) as i32) as u32) as u32 as u64; rdx = (if (rax as u32 as i32) < 0 { 0xffffffffu32 } else { 0u32 }) as u64; rdx = ((((rdx as u32)) & (0x7u32))) as u32 as u64; rax = (((rax as u32)).wrapping_add((rdx as u32))) as u32 as u64; rax = (((((rax as u32)) as i32).wrapping_shr(3u32) as u32)) as u32 as u64; rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rcx); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rcx); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rcx); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rcx); rbx = ((((rax as u32)) as i32 as i64) as u64) as u64; rdx = rbx; rcx = rsp.wrapping_add(0x40); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rdx = rbx; rcx = rsp.wrapping_add(0x50); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); r8 = (crate::simd_ops::ru32(rsp.wrapping_add(0x48u64))) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rdx = 0xff; rcx = rbx; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); r8 = (crate::simd_ops::ru32(rsp.wrapping_add(0x58u64))) as u32 as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = r14; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), r14); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rbx); crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rdi as u32)) as u32); r9 = ((rsi as u32)) as u32 as u64; r8 = 0; fl_l = 0; fl_r = 0; rdx = 0; fl_l = 0; fl_r = 0; rcx = rbp; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); rdi = rax; fl_l = r14; fl_r = r14; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = r14; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            2 => { fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rcx = rbx; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { rax = rdi; r11 = rsp.wrapping_add(0x60); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x10u64))) as u64; rbp = (crate::simd_ops::ru64(r11.wrapping_add(0x18u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rdi = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; rsp = r11; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141868150(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142646180;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rcx = 0x17;
    rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp);
    fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64;
    rax = (rax & !0xffu64) | (((if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { 1u8 } else { 0u8 }) as u8 as u64));
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1418681d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142646180; const P1: u64 = 0x143080c28; const P2: u64 = 0x142645008; const P3: u64 = 0x143080c18; const P4: u64 = 0x142645010; const P5: u64 = 0x1426456e0; const P6: u64 = 0x142645698; const P7: u64 = 0x142645000;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x58u64)) as u64; rcx = 0x2001; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); rsp = ((rsp).wrapping_add(0x58u64)) as u64; return rax; }
            2 => { rax = rsp.wrapping_add(0x30); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rdi); rdi = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); r9 = 0x20019; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rdi); r8 = 0; fl_l = 0; fl_r = 0; rdx = P1; rcx = (0xffffffff80000002u64) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x58u64)) as u64; return rax; }
            4 => { rax = rsp.wrapping_add(0x68); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rbx); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; r9 = rsp.wrapping_add(0x70); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); rdx = P3; rax = rsp.wrapping_add(0x60); crate::simd_ops::wu32(rsp.wrapping_add(0x70u64), ((rdi as u32)) as u32); r8 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); rcx = rbx; crate::simd_ops::wu32(rsp.wrapping_add(0x60u64), ((rdi as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x68u64), (0x4u32) as u32); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 7; continue; } bb = 5; continue; }
            5 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); rcx = ((rax as u32)) as u32 as u64; rdx = rsp.wrapping_add(0x78); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x60u64))) as u32 as u64; fl_l = (crate::simd_ops::ru32(rsp.wrapping_add(0x78u64))) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; rdi = (rdi & !0xffu64) | (((if (((fl_l as u32)) != ((fl_r as u32))) { 1u8 } else { 0u8 }) as u8 as u64)); bb = 7; continue; }
            7 => { rcx = rbx; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P7), rax), rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rax = (((rdi as u8)) as u32) as u32 as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x58u64)) as u64; return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1418682d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1f0; const P1: u64 = 0x140209c20; const P2: u64 = 0x18; const P3: u64 = 0x10; const P4: u64 = 0x14186a310; const P5: u64 = 0x8; const P6: u64 = 0x142442de0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x38u64)) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x50u64), (0x0u32) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), rcx); rbx = ((rcx.wrapping_add(P0)) as u64) as u64; r8 = rsp.wrapping_add(0x68); rcx = rsp.wrapping_add(0x58); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; crate::simd_ops::wu8(rax.wrapping_add(P2), (0x1u8) as u8); rdi = (crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0x0u32) as u32); crate::simd_ops::lk_add_u32(rdi, (1) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x50u64), (0x2u32) as u32); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), (0x0u64) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rax); rdx = rsp.wrapping_add(0x60); rcx = rbx; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 1; continue; }
            1 => { rax = 0xffffffff; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rdi, xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 5; continue; } bb = 2; continue; }
            2 => { rbx = (crate::simd_ops::ru64(rdi.wrapping_add(P5))) as u64; fl_l = (((rbx) & (0xfffffffffffffffcu64))) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P5))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { rcx = rdi; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            5 => { rsp = ((rsp).wrapping_add(0x38u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1418683c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143080c90; const P1: u64 = 0x1401f3d90; const P2: u64 = 0x10; const P3: u64 = 0x18;
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rax = P0;
    rbx = rcx;
    crate::simd_ops::wu64(rcx, rax);
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    crate::simd_ops::wu64(rbx.wrapping_add(P2), rax);
    rax = rbx;
    crate::simd_ops::wu16(rbx.wrapping_add(P3), (0x0u16) as u16);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1418684c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1426461a0; const P1: u64 = 0x10; const P2: u64 = 0x1401f3ee0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    rcx = 0;
    fl_l = 0; fl_r = 0;
    rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp);
    rcx = ((rbx.wrapping_add(P1)) as u64) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(P2, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1418684f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x120; const P1: u64 = 0x1426455a8; const P2: u64 = 0x138; const P3: u64 = 0x128; const P4: u64 = 0x130; const P5: u64 = 0x13c; const P6: u64 = 0x134; const P7: u64 = 0xffffffffffffffff; const P8: u64 = 0x142442de0; const P9: u64 = 0x142645598;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = rdx; rdi = rcx; rbx = ((rcx.wrapping_add(P0)) as u64) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(P2))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsi, (0x0u64) as u64); bb = 6; continue; }
            2 => { r8 = (crate::simd_ops::ru64(rdi.wrapping_add(P3))) as u64; rax = (crate::simd_ops::ru32(rdi.wrapping_add(P2))) as u32 as u64; rax = (((rax as u32)).wrapping_sub(1)) as u32 as u64; rcx = (crate::simd_ops::ru32(rdi.wrapping_add(P4))) as u32 as u64; rax = ((rax).wrapping_add(rcx)) as u64; rcx = (crate::simd_ops::ru32(rdi.wrapping_add(P5))) as u32 as u64; rdx = 0; fl_l = 0; fl_r = 0; let d = (rcx) as u128; let n = ((rdx as u128) << 64) | (rax as u128); if d != 0 { rax = (n / d) as u64; rdx = (n % d) as u64; } rbp = (crate::simd_ops::ru64(r8.wrapping_add(rdx.wrapping_mul(8)))) as u64; crate::simd_ops::wu64(r8.wrapping_add(rdx.wrapping_mul(8)), (0x0u64) as u64); r8 = (crate::simd_ops::ru32(rdi.wrapping_add(P2))) as u32 as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 5; continue; } bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru32(rdi.wrapping_add(P5))) as u32 as u64; rax = (crate::simd_ops::ru32(rdi.wrapping_add(P6))) as u32 as u64; rax = ((rax).wrapping_sub(1)) as u64; rax = ((rax).wrapping_add(rcx)) as u64; rdx = 0; fl_l = 0; fl_r = 0; let d = (rcx) as u128; let n = ((rdx as u128) << 64) | (rax as u128); if d != 0 { rax = (n / d) as u64; rdx = (n % d) as u64; } crate::simd_ops::wu32(rdi.wrapping_add(P6), ((rdx as u32)) as u32); rax = ((r8.wrapping_add(P7)) as u32) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(P2), ((rax as u32)) as u32); rcx = ((rdx as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rdi.wrapping_add(P3))) as u64; rdi = (crate::simd_ops::ru64(rax.wrapping_add(rcx.wrapping_mul(8)))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rdi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = rdi; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            5 => { crate::simd_ops::wu64(rsi, rbp); bb = 6; continue; }
            6 => { rcx = rbx; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P9), rax), rax, rcx, rdx, r8, r9, rsp); rax = rsi; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1418685f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1426455a8; const P1: u64 = 0x142645598; const P2: u64 = 0x142442de0; const P3: u64 = 0x1425371a0; const P4: u64 = 0x142533200;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; r14 = rcx; r15 = 0; fl_l = 0; fl_r = 0; bb = 1; continue; }
            1 => { rsi = (crate::simd_ops::ru64(r14)) as u64; rdi = rsi.wrapping_add(0x120); rcx = rdi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru32(rsi.wrapping_add(0x138u64))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rbx = r15; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rbx); rcx = rdi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); rbp = r15; rsi = r15; rdi = r15; bb = 7; continue; }
            3 => { r8 = (crate::simd_ops::ru64(rsi.wrapping_add(0x128u64))) as u64; rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x138u64))) as u32 as u64; rax = (((rax as u32)).wrapping_sub(1)) as u32 as u64; rcx = (crate::simd_ops::ru32(rsi.wrapping_add(0x130u64))) as u32 as u64; rax = ((rax).wrapping_add(rcx)) as u64; rcx = (crate::simd_ops::ru32(rsi.wrapping_add(0x13cu64))) as u32 as u64; rdx = 0; fl_l = 0; fl_r = 0; let d = (rcx) as u128; let n = ((rdx as u128) << 64) | (rax as u128); if d != 0 { rax = (n / d) as u64; rdx = (n % d) as u64; } rbx = (crate::simd_ops::ru64(r8.wrapping_add(rdx.wrapping_mul(8)))) as u64; rbp = rbx; crate::simd_ops::wu64(r8.wrapping_add(rdx.wrapping_mul(8)), r15); r8 = (crate::simd_ops::ru32(rsi.wrapping_add(0x138u64))) as u32 as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 6; continue; } bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru32(rsi.wrapping_add(0x13cu64))) as u32 as u64; rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x134u64))) as u32 as u64; rax = ((rax).wrapping_sub(1)) as u64; rax = ((rax).wrapping_add(rcx)) as u64; rdx = 0; fl_l = 0; fl_r = 0; let d = (rcx) as u128; let n = ((rdx as u128) << 64) | (rax as u128); if d != 0 { rax = (n / d) as u64; rdx = (n % d) as u64; } crate::simd_ops::wu32(rsi.wrapping_add(0x134u64), ((rdx as u32)) as u32); rax = ((r8.wrapping_sub(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rsi.wrapping_add(0x138u64), ((rax as u32)) as u32); rcx = ((rdx as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x128u64))) as u64; rsi = (crate::simd_ops::ru64(rax.wrapping_add(rcx.wrapping_mul(8)))) as u64; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rsi)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rsi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = rsi; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            6 => { crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rbx); rcx = rdi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); rsi = rbx; rdi = rbx; bb = 7; continue; }
            7 => { fl_l = rbp; fl_r = rbp; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 16; continue; } bb = 8; continue; }
            8 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); r10 = rsi.wrapping_sub(0x1); r10 = (((r10) & (0xfffffffffe000000u64))) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(r10.wrapping_add(0x100u64))) as u64; rcx = (rcx & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); rdi = ((rdi).wrapping_sub(r10)) as u64; rdi = ((rdi).wrapping_shr(16u32)) as u64; rdx = ((rdi.wrapping_add(rdi.wrapping_mul(4))) as u64) as u64; rdx = ((rdx).wrapping_shl(4u32)) as u64; rdx = ((rdx).wrapping_add(0x108u64)) as u64; rdx = ((rdx).wrapping_add(r10)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32 as u64; rdx = ((rdx).wrapping_sub(rax)) as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 13; continue; } bb = 9; continue; }
            9 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0xeu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 12; continue; } bb = 10; continue; }
            10 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x20u64))) as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; crate::simd_ops::wu64(rax, rcx); crate::simd_ops::wu64(rdx.wrapping_add(0x20u64), rax); fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 15; continue; } bb = 11; continue; }
            11 => { rcx = rdx; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 15; continue; }
            12 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 14; continue; }
            13 => { r8 = 0; fl_l = 0; fl_r = 0; bb = 14; continue; }
            14 => { r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rcx = r10; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 15; continue; }
            15 => { bb = 1; continue; }
            16 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1418687b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1426461b8; const P1: u64 = 0x1418685f0; const P2: u64 = 0x142646228; const P3: u64 = 0x1426461a8; const P4: u64 = 0x142646218; const P5: u64 = 0x1426455a8; const P6: u64 = 0x142645598; const P7: u64 = 0x142442de0; const P8: u64 = 0x1425371a0; const P9: u64 = 0x142533200;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 17>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = rsp; crate::simd_ops::wu64(rax.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rax.wrapping_add(0x18u64), rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x60u64)) as u64; rsi = rcx; crate::simd_ops::wu64(rax.wrapping_add(0x8u64), rcx); r12 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rax.wrapping_sub(0x68u64), ((r12 as u32)) as u32); r9 = 0; fl_l = 0; fl_r = 0; r8 = 0; fl_l = 0; fl_r = 0; rdx = 0; fl_l = 0; fl_r = 0; rcx = rax.wrapping_sub(0x58); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rsp.wrapping_add(0x90); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); r9 = 0; fl_l = 0; fl_r = 0; r8 = 0; fl_l = 0; fl_r = 0; rdx = 0; fl_l = 0; fl_r = 0; rcx = rsp.wrapping_add(0x30); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 20; continue; } bb = 1; continue; }
            1 => { bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru32(rsp.wrapping_add(0x38u64))) as u32 as u64; fl_r = 0x8000; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rcx = rsp.wrapping_add(0x30); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rsp.wrapping_add(0x30); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { rcx = rsi.wrapping_add(0x120); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru32(rsi.wrapping_add(0x138u64))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rbx = r12; crate::simd_ops::wu64(rsp.wrapping_add(0x90u64), rbx); rcx = rsi.wrapping_add(0x120); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp); rbp = r12; r15 = r12; r14 = r12; bb = 10; continue; }
            6 => { r8 = (crate::simd_ops::ru64(rsi.wrapping_add(0x128u64))) as u64; rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x138u64))) as u32 as u64; rax = (((rax as u32)).wrapping_sub(1)) as u32 as u64; rcx = (crate::simd_ops::ru32(rsi.wrapping_add(0x130u64))) as u32 as u64; rax = ((rax).wrapping_add(rcx)) as u64; rcx = (crate::simd_ops::ru32(rsi.wrapping_add(0x13cu64))) as u32 as u64; rdx = 0; fl_l = 0; fl_r = 0; let d = (rcx) as u128; let n = ((rdx as u128) << 64) | (rax as u128); if d != 0 { rax = (n / d) as u64; rdx = (n % d) as u64; } rbx = (crate::simd_ops::ru64(r8.wrapping_add(rdx.wrapping_mul(8)))) as u64; rbp = rbx; crate::simd_ops::wu64(r8.wrapping_add(rdx.wrapping_mul(8)), r12); r8 = (crate::simd_ops::ru32(rsi.wrapping_add(0x138u64))) as u32 as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 9; continue; } bb = 7; continue; }
            7 => { rcx = (crate::simd_ops::ru32(rsi.wrapping_add(0x13cu64))) as u32 as u64; rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x134u64))) as u32 as u64; rax = ((rax).wrapping_sub(1)) as u64; rax = ((rax).wrapping_add(rcx)) as u64; rdx = 0; fl_l = 0; fl_r = 0; let d = (rcx) as u128; let n = ((rdx as u128) << 64) | (rax as u128); if d != 0 { rax = (n / d) as u64; rdx = (n % d) as u64; } crate::simd_ops::wu32(rsi.wrapping_add(0x134u64), ((rdx as u32)) as u32); rax = ((r8.wrapping_sub(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rsi.wrapping_add(0x138u64), ((rax as u32)) as u32); rcx = ((rdx as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x128u64))) as u64; r14 = (crate::simd_ops::ru64(rax.wrapping_add(rcx.wrapping_mul(8)))) as u64; fl_l = r14; fl_r = r14; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { rax = (crate::simd_ops::ru64(r14)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = r14; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = r14; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); bb = 9; continue; }
            9 => { crate::simd_ops::wu64(rsp.wrapping_add(0x90u64), rbx); rcx = rsi.wrapping_add(0x120); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp); r15 = rbx; r14 = rbx; bb = 10; continue; }
            10 => { fl_l = rbp; fl_r = rbp; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 19; continue; } bb = 11; continue; }
            11 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); r10 = r15.wrapping_sub(0x1); r10 = (((r10) & (0xfffffffffe000000u64))) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(r10.wrapping_add(0x100u64))) as u64; rcx = (rcx & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); r14 = ((r14).wrapping_sub(r10)) as u64; r14 = ((r14).wrapping_shr(16u32)) as u64; rdx = ((r14.wrapping_add(r14.wrapping_mul(4))) as u64) as u64; rdx = ((rdx).wrapping_shl(4u32)) as u64; rdx = ((rdx).wrapping_add(0x108u64)) as u64; rdx = ((rdx).wrapping_add(r10)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32 as u64; rdx = ((rdx).wrapping_sub(rax)) as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 16; continue; } bb = 12; continue; }
            12 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0xeu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 15; continue; } bb = 13; continue; }
            13 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x20u64))) as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; crate::simd_ops::wu64(rax, rcx); crate::simd_ops::wu64(rdx.wrapping_add(0x20u64), rax); fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 18; continue; } bb = 14; continue; }
            14 => { rcx = rdx; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); bb = 18; continue; }
            15 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 17; continue; }
            16 => { r8 = 0; fl_l = 0; fl_r = 0; bb = 17; continue; }
            17 => { r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; rcx = r10; rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); bb = 18; continue; }
            18 => { bb = 4; continue; }
            19 => { r9 = 0; fl_l = 0; fl_r = 0; r8 = 0; fl_l = 0; fl_r = 0; rdx = 0; fl_l = 0; fl_r = 0; rcx = rsp.wrapping_add(0x30); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 2; continue; } bb = 20; continue; }
            20 => { rax = 0; fl_l = 0; fl_r = 0; r11 = rsp.wrapping_add(0x60); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x38u64))) as u64; rbp = (crate::simd_ops::ru64(r11.wrapping_add(0x40u64))) as u64; rsp = r11; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1418689f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143080c78; const P1: u64 = 0x14020a7d0; const P2: u64 = 0x1423f0c20;
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rax = P0;
    rbx = rcx;
    crate::simd_ops::wu64(rcx, rax);
    rcx = ((rcx).wrapping_add(0x128u64)) as u64;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rcx = rbx;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(P2, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141868a30(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14183acc2; const P1: u64 = 0x143080c60; const P2: u64 = 0x1426469b8; const P3: u64 = 0x8; const P4: u64 = 0x2; const P5: u64 = 0x80; const P6: u64 = 0x70; const P7: u64 = 0xffffffffffffff80; const P8: u64 = 0xffffffffffffffa0; const P9: u64 = 0xffffffffffffffc0; const P10: u64 = 0xffffffffffffffe0; const P11: u64 = 0xfffffffffffffff0; const P12: u64 = 0x143080c78;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 37>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x120u64)) as u64; rbx = rcx; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0xffffffffu32) as u32); rdi = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rsp.wrapping_add(0x2cu64), (0x4u32) as u32); rcx = rsp.wrapping_add(0x31); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rdx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu8(rsp.wrapping_add(0x30u64), ((rdi as u8)) as u8); r8 = 0xef; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = P1; rdx = 0xef; rcx = rsp.wrapping_add(0x31); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = ((rbx.wrapping_add(P3)) as u64) as u64; rcx = rsp.wrapping_add(0x20); rdx = ((rdi.wrapping_add(P4)) as u32) as u32 as u64; bb = 1; continue; }
            1 => { rax = ((rax.wrapping_add(P5)) as u64) as u64; let y_addr: u64 = rcx; xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); xmm1 = (crate::simd_ops::ru128(rcx.wrapping_add(P6))) as u128; rcx = ((rcx.wrapping_add(P5)) as u64) as u64; let y_addr: u64 = rax.wrapping_add(P7); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); let y_addr: u64 = rcx.wrapping_add(P8); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rax.wrapping_add(P8); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); let y_addr: u64 = rcx.wrapping_add(P9); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rax.wrapping_add(P9); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); xmm0 = (crate::simd_ops::ru128(rcx.wrapping_add(P10))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rax.wrapping_add(P10), (xmm0) as u128); crate::simd_ops::wu128(rax.wrapping_add(P11), (xmm1) as u128); fl_l = rdx; fl_r = (0x1u64) as u64; rdx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 2; continue; }
            2 => { rax = P12; crate::simd_ops::wu64(rbx.wrapping_add(0x108u64), rdi); crate::simd_ops::wu64(rbx, rax); rax = rbx; crate::simd_ops::wu64(rbx.wrapping_add(0x110u64), rdi); crate::simd_ops::wu8(rbx.wrapping_add(0x118u64), ((rdi as u8)) as u8); crate::simd_ops::wu64(rbx.wrapping_add(0x120u64), rdi); crate::simd_ops::wu64(rbx.wrapping_add(0x128u64), rdi); crate::simd_ops::wu64(rbx.wrapping_add(0x130u64), rdi); crate::simd_ops::wu64(rbx.wrapping_add(0x138u64), rdi); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x130u64))) as u64; rsp = ((rsp).wrapping_add(0x120u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141868b40(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x60;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x100;
    if !((((fl_l as u32)) >= ((fl_r as u32)))) { rax = ((rdx as u32)) as u32 as u64; rax = ((crate::simd_ops::ru8(rax.wrapping_add(rcx).wrapping_add(P0))) as u32) as u32 as u64; return rax; }
    rax = (rax & !0xffu64) | (((0u8) as u8 as u64));
    fl_l = 0; fl_r = 0;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141868b60(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu32(rdx, ((rax as u32)) as u32);
    fl_l = ((r8 as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64;
    if !(((((fl_l as u8)) & ((fl_r as u8))) == 0)) { rax = 0x1; crate::simd_ops::wu32(rdx, ((rax as u32)) as u32); }
    fl_l = ((r8 as u8)) as u8 as u64; fl_r = (0x10u8) as u8 as u64;
    if !(((((fl_l as u8)) & ((fl_r as u8))) == 0)) { fl_l = ((((rax as u32)) | (0x2u32))) as u32 as u64; fl_r = fl_l; rax = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(rdx, ((rax as u32)) as u32); }
    fl_l = ((r8 as u8)) as u8 as u64; fl_r = (0x2u8) as u8 as u64;
    if !(((((fl_l as u8)) & ((fl_r as u8))) == 0)) { fl_l = ((((rax as u32)) | (0x4u32))) as u32 as u64; fl_r = fl_l; rax = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(rdx, ((rax as u32)) as u32); }
    fl_l = ((r8 as u8)) as u8 as u64; fl_r = (0x8u8) as u8 as u64;
    if !(((((fl_l as u8)) & ((fl_r as u8))) == 0)) { fl_l = ((((rax as u32)) | (0x8u32))) as u32 as u64; fl_r = fl_l; rax = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(rdx, ((rax as u32)) as u32); }
    fl_l = ((r8 as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64;
    if !(((((fl_l as u8)) & ((fl_r as u8))) == 0)) { fl_l = ((((rax as u32)) | (0x10u32))) as u32 as u64; fl_r = fl_l; rax = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(rdx, ((rax as u32)) as u32); }
    rax = rdx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141868c50(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1426455a8; const P1: u64 = 0x1c8; const P2: u64 = 0x1b0; const P3: u64 = 0x190; const P4: u64 = 0x20; const P5: u64 = 0x30; const P6: u64 = 0x1c0; const P7: u64 = 0x188; const P8: u64 = 0x142645598; const P9: u64 = 0x28; const P10: u64 = 0x14020ab80;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0; let mut ymm1_hi: u128 = 0; let mut ymm2_hi: u128 = 0; let mut ymm3_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 17>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x80u64)) as u64; rdi = rcx; rsi = rdx; rcx = ((rcx).wrapping_add(0x188u64)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(P1))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; let y_addr: u64 = rsi; xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rdi.wrapping_add(P2); xmm2 = crate::simd_ops::ru128(y_addr); ymm2_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rdi.wrapping_add(P3); xmm3 = crate::simd_ops::ru128(y_addr); ymm3_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rdi.wrapping_add(P3); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); xmm1 = (crate::simd_ops::ru128(rsi.wrapping_add(P4))) as u128; ymm1_hi = 0; crate::simd_ops::wu128(rdi.wrapping_add(P2), (xmm1) as u128); xmm0 = ((crate::simd_ops::ru64(rsi.wrapping_add(P5))) as u64) as u128; ymm0_hi = 0; crate::simd_ops::wu64(rdi.wrapping_add(P6), ((xmm0 as u64)) as u64); let y_addr: u64 = rsp.wrapping_add(0x20u64); crate::simd_ops::wu128(y_addr, xmm2); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm2_hi); if (((fl_l as u8)) != ((fl_r as u8))) { bb = 2; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu8(rdi.wrapping_add(P1), (0x1u8) as u8); bb = 2; continue; }
            2 => { let y_addr: u64 = rsp.wrapping_add(0x40u64); crate::simd_ops::wu128(y_addr, xmm3); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm3_hi); let y_addr: u64 = rsp.wrapping_add(0x60u64); crate::simd_ops::wu128(y_addr, xmm2); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm2_hi); rcx = ((rdi.wrapping_add(P7)) as u64) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P8), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); let y_addr: u64 = rsp.wrapping_add(0x20u64); xmm1 = crate::simd_ops::ru128(y_addr); ymm1_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); xmm0 = (ymm1_hi) as u128; ymm0_hi = 0; rax = (((xmm0 >> 64) as u8) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 5; continue; } bb = 3; continue; }
            3 => { rax = (((xmm1 >> 64) as u32) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rsi.wrapping_add(P9))) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rdx = rsp.wrapping_add(0x40); rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P10, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 5; continue; }
            5 => { r11 = rsp.wrapping_add(0x80); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x10u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x18u64))) as u64; rsp = r11; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141868d30(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141869400; const P1: u64 = 0x1426455a8; const P2: u64 = 0x140209af0; const P3: u64 = 0x1401deb40; const P4: u64 = 0x142645598;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r9); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; r15 = r8; r14 = rdx; rbp = rcx; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 1; continue; }
            1 => { r12 = 0; fl_l = 0; fl_r = 0; rdi = ((r12 as u32)) as u32 as u64; rsi = ((r12 as u32)) as u32 as u64; rbx = rcx.wrapping_add(0x60); bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru8(rbx)) as u8 as u64; fl_r = ((r12 as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 4; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((r12 as u32)) as u32); r9 = 0; fl_l = 0; fl_r = 0; r8 = rsi; rdx = r14; rcx = rbp; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { rdi = (((rdi as u32)).wrapping_add(1)) as u32 as u64; rsi = ((rsi).wrapping_add(1)) as u64; rbx = ((rbx).wrapping_add(1)) as u64; fl_l = ((rdi as u32)) as u32 as u64; fl_r = 0x100; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 2; continue; } bb = 5; continue; }
            5 => { crate::simd_ops::wu32(rsp.wrapping_add(0x78u64), (0x13u32) as u32); fl_l = r15; fl_r = r15; crate::simd_ops::wu8(rsp.wrapping_add(0x7cu64), (if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { 1u8 } else { 0u8 }) as u8); rbx = rbp.wrapping_add(0x160); rcx = rbx; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); r14 = (crate::simd_ops::ru32(rbp.wrapping_add(0x170u64))) as u32 as u64; r14 = (((r14 as u32)).wrapping_add(0x3u32)) as u32 as u64; r14 = ((((r14 as u32)) & (0xfffffffcu32))) as u32 as u64; rdx = r14.wrapping_add(0x8); rcx = rbp.wrapping_add(0x168); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x168u64))) as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; crate::simd_ops::wu64(r14.wrapping_add(rcx), rax); rbp = ((rbp).wrapping_add(0x178u64)) as u64; rdx = (crate::simd_ops::ru32(rbp.wrapping_add(0x8u64))) as u32 as u64; r8 = (crate::simd_ops::ru32(rbp.wrapping_add(0xcu64))) as u32 as u64; rcx = ((r8.wrapping_mul(4)) as u64) as u64; rax = ((rdx.wrapping_mul(4)) as u64) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rax = rdx.wrapping_add(0x1); rdx = ((r8 as u32)) as u32 as u64; rdx = ((rdx).wrapping_shr(1u32)) as u64; rdx = ((rdx).wrapping_add(r8)) as u64; fl_l = rdx; fl_r = rax; rdx = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { rdx }) as u64; rcx = rbp; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 7; continue; }
            7 => { rcx = (crate::simd_ops::ru32(rbp.wrapping_add(0x8u64))) as u32 as u64; rax = (crate::simd_ops::ru64(rbp)) as u64; crate::simd_ops::wu32(rax.wrapping_add(rcx.wrapping_mul(4)), ((r14 as u32)) as u32); crate::simd_ops::lk_add_u32(rbp.wrapping_add(0x8u64), (1) as u32); rcx = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141868e60(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x60; const P1: u64 = 0x1402098b0;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x40u64)) as u64;
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    rbx = rdx;
    rdi = rcx;
    rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp);
    rdx = rsp.wrapping_add(0x20);
    crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x1u32) as u32);
    rcx = rdi;
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rbx);
    crate::simd_ops::wu8(rsp.wrapping_add(0x30u64), (0x0u8) as u8);
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64;
    rsp = ((rsp).wrapping_add(0x40u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141868eb0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x68; const P1: u64 = 0x1402098b0;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x40u64)) as u64;
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    rbx = rdx;
    rdi = rcx;
    rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp);
    rdx = rsp.wrapping_add(0x20);
    crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x1u32) as u32);
    rcx = rdi;
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rbx);
    crate::simd_ops::wu8(rsp.wrapping_add(0x30u64), (0x1u8) as u8);
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64;
    rsp = ((rsp).wrapping_add(0x40u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141868f00(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1426455a8; const P1: u64 = 0x140209af0; const P2: u64 = 0x1401deb40; const P3: u64 = 0x142645598;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 15>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x70u64)) as u64; rdi = r9; r11 = rdx; r14 = rcx; rax = r8; rax = ((rax).wrapping_shr(16u32)) as u64; r10 = ((((rax as u16)) as i16 as i32) as u32) as u32 as u64; rax = 0x88888889; let prod = (rax as u32 as i32 as i64).wrapping_mul(((r10 as u32)) as i32 as i64) as u64; rax = prod as u32 as u64; rdx = (prod >> 32) as u32 as u64; rsi = ((r10.wrapping_add(rdx)) as u32) as u32 as u64; rsi = (((((rsi as u32)) as i32).wrapping_shr(6u32) as u32)) as u32 as u64; rax = ((rsi as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shr(31u32)) as u32 as u64; rsi = (((rsi as u32)).wrapping_add((rax as u32))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), (0xbu32) as u32); xmm0 = 0u128; ymm0_hi = 0; crate::simd_ops::wu32(rsp.wrapping_add(0x40u64), ((xmm0 as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x44u64), ((xmm0 as u32)) as u32); crate::simd_ops::wu128(rsp.wrapping_add(0x48u64), (xmm0) as u128); rbx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), r11); rcx = ((r8 as u32)) as u32 as u64; rcx = ((((rcx as u32)) & (0x1u32))) as u32 as u64; rdx = ((rcx as u32)) as u32 as u64; rdx = ((((rdx as u32)) | (0x2u32))) as u32 as u64; fl_l = ((r8 as u8)) as u8 as u64; fl_r = (0x10u8) as u8 as u64; rdx = (if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { (rcx as u32) } else { (rdx as u32) }) as u32 as u64; rcx = ((rdx as u32)) as u32 as u64; rcx = ((((rcx as u32)) | (0x4u32))) as u32 as u64; fl_l = ((r8 as u8)) as u8 as u64; fl_r = (0x2u8) as u8 as u64; rcx = (if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { (rdx as u32) } else { (rcx as u32) }) as u32 as u64; rdx = ((rcx as u32)) as u32 as u64; rdx = ((((rdx as u32)) | (0x8u32))) as u32 as u64; fl_l = ((r8 as u8)) as u8 as u64; fl_r = (0x8u8) as u8 as u64; rdx = (if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { (rcx as u32) } else { (rdx as u32) }) as u32 as u64; rax = ((rdx as u32)) as u32 as u64; rax = ((((rax as u32)) | (0x10u32))) as u32 as u64; fl_l = ((r8 as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64; rax = (if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { (rdx as u32) } else { (rax as u32) }) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x58u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(r14)) as u64; rcx = rsp.wrapping_add(0x30); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rcx); r9 = ((rbx.wrapping_add(0x1u64)) as u32) as u32 as u64; r8 = rdi; rdx = r11; rcx = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x48u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = ((rbx as u32)) as u32 as u64; rcx = (crate::simd_ops::ru32(rsp.wrapping_add(0xa0u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; rax = (if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { (rsi as u32) } else { (rax as u32) }) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x60u64), ((rax as u32)) as u32); fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; rbx = (if (((fl_l as u32)) == ((fl_r as u32))) { (rsi as u32) } else { (rbx as u32) }) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x64u64), ((rbx as u32)) as u32); rbx = r14.wrapping_add(0x160); rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbp = (crate::simd_ops::ru32(r14.wrapping_add(0x170u64))) as u32 as u64; rbp = (((rbp as u32)).wrapping_add(0x7u32)) as u32 as u64; rbp = ((((rbp as u32)) & (0xfffffff8u32))) as u32 as u64; rdx = rbp.wrapping_add(0x38); rcx = r14.wrapping_add(0x168); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(r14.wrapping_add(0x168u64))) as u64; let y_addr: u64 = rsp.wrapping_add(0x30u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rax.wrapping_add(rbp); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); xmm1 = (crate::simd_ops::ru128(rsp.wrapping_add(0x50u64))) as u128; crate::simd_ops::wu128(rax.wrapping_add(rbp).wrapping_add(0x20u64), (xmm1) as u128); xmm0 = ((crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64) as u128; ymm0_hi = 0; crate::simd_ops::wu64(rax.wrapping_add(rbp).wrapping_add(0x30u64), ((xmm0 as u64)) as u64); r14 = ((r14).wrapping_add(0x178u64)) as u64; rdx = (crate::simd_ops::ru32(r14.wrapping_add(0x8u64))) as u32 as u64; r8 = (crate::simd_ops::ru32(r14.wrapping_add(0xcu64))) as u32 as u64; rcx = ((r8.wrapping_mul(4)) as u64) as u64; rax = ((rdx.wrapping_mul(4)) as u64) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = rdx.wrapping_add(0x1); rdx = ((r8 as u32)) as u32 as u64; rdx = ((rdx).wrapping_shr(1u32)) as u64; rdx = ((rdx).wrapping_add(r8)) as u64; fl_l = rdx; fl_r = rax; rdx = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { rdx }) as u64; rcx = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru32(r14.wrapping_add(0x8u64))) as u32 as u64; rax = (crate::simd_ops::ru64(r14)) as u64; crate::simd_ops::wu32(rax.wrapping_add(rcx.wrapping_mul(4)), ((rbp as u32)) as u32); crate::simd_ops::lk_add_u32(r14.wrapping_add(0x8u64), (1) as u32); rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r11 = rsp.wrapping_add(0x70); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x10u64))) as u64; rbp = (crate::simd_ops::ru64(r11.wrapping_add(0x18u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rdi = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; rsp = r11; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1418690c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1426456f8; const P1: u64 = 0x1426455a8; const P2: u64 = 0x142645598; const P3: u64 = 0x14020ab80;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0; let mut ymm1_hi: u128 = 0; let mut ymm2_hi: u128 = 0; let mut ymm3_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 27>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0xd0u64)) as u64; rbx = rdx; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rdx); rsi = rcx; crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), (0xau32) as u32); xmm0 = 0u128; ymm0_hi = 0; r10 = ((r8 as u32)) as u32 as u64; rdi = r9; r10 = ((((r10 as u32)) & (0x1u32))) as u32 as u64; rdx = ((r10 as u32)) as u32 as u64; rdx = ((((rdx as u32)) | (0x2u32))) as u32 as u64; fl_l = ((r8 as u8)) as u8 as u64; fl_r = (0x10u8) as u8 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x40u64), ((xmm0 as u32)) as u32); rdx = (if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { (r10 as u32) } else { (rdx as u32) }) as u32 as u64; rcx = ((rdx as u32)) as u32 as u64; rcx = ((((rcx as u32)) | (0x4u32))) as u32 as u64; fl_l = ((r8 as u8)) as u8 as u64; fl_r = (0x2u8) as u8 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x44u64), ((xmm0 as u32)) as u32); rcx = (if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { (rdx as u32) } else { (rcx as u32) }) as u32 as u64; rdx = ((rcx as u32)) as u32 as u64; rdx = ((((rdx as u32)) | (0x8u32))) as u32 as u64; fl_l = ((r8 as u8)) as u8 as u64; fl_r = (0x8u8) as u8 as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x48u64), (xmm0) as u128); rdx = (if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { (rcx as u32) } else { (rdx as u32) }) as u32 as u64; rcx = rsp.wrapping_add(0xe0); rax = ((rdx as u32)) as u32 as u64; rax = ((((rax as u32)) | (0x10u32))) as u32 as u64; fl_l = ((r8 as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64; rax = (if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { (rdx as u32) } else { (rax as u32) }) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x58u64), ((rax as u32)) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xe0u64))) as u64; rcx = rsp.wrapping_add(0x30); crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rax); r9 = 0; fl_l = 0; fl_r = 0; rax = (crate::simd_ops::ru64(rsi)) as u64; r8 = rdi; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rcx); rdx = rbx; rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x48u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rsi.wrapping_add(0x188); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru8(rsi.wrapping_add(0x1c8u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; let y_addr: u64 = rsi.wrapping_add(0x1b0u64); xmm2 = crate::simd_ops::ru128(y_addr); ymm2_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rsi.wrapping_add(0x190u64); xmm3 = crate::simd_ops::ru128(y_addr); ymm3_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rsp.wrapping_add(0x30u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); xmm1 = (crate::simd_ops::ru128(rsp.wrapping_add(0x50u64))) as u128; ymm1_hi = 0; let y_addr: u64 = rsi.wrapping_add(0x190u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); xmm0 = ((crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rsi.wrapping_add(0x1b0u64), (xmm1) as u128); crate::simd_ops::wu64(rsi.wrapping_add(0x1c0u64), ((xmm0 as u64)) as u64); let y_addr: u64 = rsp.wrapping_add(0x68u64); crate::simd_ops::wu128(y_addr, xmm2); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm2_hi); if (((fl_l as u8)) != ((fl_r as u8))) { bb = 2; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu8(rsi.wrapping_add(0x1c8u64), (0x1u8) as u8); bb = 2; continue; }
            2 => { let y_addr: u64 = rsp.wrapping_add(0x90u64); crate::simd_ops::wu128(y_addr, xmm3); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm3_hi); let y_addr: u64 = rsp.wrapping_add(0xb0u64); crate::simd_ops::wu128(y_addr, xmm2); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm2_hi); rcx = rsi.wrapping_add(0x188); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); let y_addr: u64 = rsp.wrapping_add(0x68u64); xmm1 = crate::simd_ops::ru128(y_addr); ymm1_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); xmm0 = (ymm1_hi) as u128; ymm0_hi = 0; rax = (((xmm0 >> 64) as u8) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 5; continue; } bb = 3; continue; }
            3 => { rax = (((xmm1 >> 64) as u32) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rsp.wrapping_add(0x58u64))) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rdx = rsp.wrapping_add(0x90); rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 5; continue; }
            5 => { r11 = rsp.wrapping_add(0xd0); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x18u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rsp = r11; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141869250(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141868bb0; const P1: u64 = 0x1426455a8; const P2: u64 = 0x140209af0; const P3: u64 = 0x1401deb40; const P4: u64 = 0x142645598;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 17>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x70u64)) as u64; r14 = r9; rbx = r8; rbp = rdx; r15 = rcx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(r15)) as u64; rsi = (crate::simd_ops::ru32(rsp.wrapping_add(0xb0u64))) as u32 as u64; r9 = ((rsi as u32)) as u32 as u64; r8 = (crate::simd_ops::ru32(rsp.wrapping_add(0xb8u64))) as u32 as u64; rdx = rbp; rcx = r15; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x58u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), (0x9u32) as u32); xmm0 = 0u128; ymm0_hi = 0; crate::simd_ops::wu32(rsp.wrapping_add(0x40u64), ((xmm0 as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x44u64), ((xmm0 as u32)) as u32); crate::simd_ops::wu128(rsp.wrapping_add(0x48u64), (xmm0) as u128); crate::simd_ops::wu32(rsp.wrapping_add(0x60u64), (0x0u32) as u32); crate::simd_ops::wu8(rsp.wrapping_add(0x64u64), (0x0u8) as u8); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rbp); rcx = ((rbx as u32)) as u32 as u64; rcx = ((((rcx as u32)) & (0x1u32))) as u32 as u64; rdx = ((rcx as u32)) as u32 as u64; rdx = ((((rdx as u32)) | (0x2u32))) as u32 as u64; fl_l = ((rbx as u8)) as u8 as u64; fl_r = (0x10u8) as u8 as u64; rdx = (if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { (rcx as u32) } else { (rdx as u32) }) as u32 as u64; rcx = ((rdx as u32)) as u32 as u64; rcx = ((((rcx as u32)) | (0x4u32))) as u32 as u64; fl_l = ((rbx as u8)) as u8 as u64; fl_r = (0x2u8) as u8 as u64; rcx = (if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { (rdx as u32) } else { (rcx as u32) }) as u32 as u64; rdx = ((rcx as u32)) as u32 as u64; rdx = ((((rdx as u32)) | (0x8u32))) as u32 as u64; fl_l = ((rbx as u8)) as u8 as u64; fl_r = (0x8u8) as u8 as u64; rdx = (if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { (rcx as u32) } else { (rdx as u32) }) as u32 as u64; rax = ((rdx as u32)) as u32 as u64; rax = ((((rax as u32)) | (0x10u32))) as u32 as u64; fl_l = ((rbx as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64; rax = (if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { (rdx as u32) } else { (rax as u32) }) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x58u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(r15)) as u64; rcx = rsp.wrapping_add(0x30); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rcx); r9 = 0; fl_l = 0; fl_r = 0; r8 = r14; rdx = rbp; rcx = r15; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x48u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(rsp.wrapping_add(0x60u64), ((rsi as u32)) as u32); fl_l = (crate::simd_ops::ru32(rsp.wrapping_add(0xb8u64))) as u32 as u64; fl_r = 0x1; crate::simd_ops::wu8(rsp.wrapping_add(0x64u64), (if (((fl_l as u32)) == ((fl_r as u32))) { 1u8 } else { 0u8 }) as u8); rbx = r15.wrapping_add(0x160); rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbp = (crate::simd_ops::ru32(r15.wrapping_add(0x170u64))) as u32 as u64; rbp = (((rbp as u32)).wrapping_add(0x7u32)) as u32 as u64; rbp = ((((rbp as u32)) & (0xfffffff8u32))) as u32 as u64; rdx = rbp.wrapping_add(0x38); rcx = r15.wrapping_add(0x168); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(r15.wrapping_add(0x168u64))) as u64; let y_addr: u64 = rsp.wrapping_add(0x30u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rbp.wrapping_add(rax); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); xmm1 = (crate::simd_ops::ru128(rsp.wrapping_add(0x50u64))) as u128; crate::simd_ops::wu128(rbp.wrapping_add(rax).wrapping_add(0x20u64), (xmm1) as u128); xmm0 = ((crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64) as u128; ymm0_hi = 0; crate::simd_ops::wu64(rbp.wrapping_add(rax).wrapping_add(0x30u64), ((xmm0 as u64)) as u64); r15 = ((r15).wrapping_add(0x178u64)) as u64; rdx = (crate::simd_ops::ru32(r15.wrapping_add(0x8u64))) as u32 as u64; r8 = (crate::simd_ops::ru32(r15.wrapping_add(0xcu64))) as u32 as u64; rcx = ((r8.wrapping_mul(4)) as u64) as u64; rax = ((rdx.wrapping_mul(4)) as u64) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = rdx.wrapping_add(0x1); rdx = ((r8 as u32)) as u32 as u64; rdx = ((rdx).wrapping_shr(1u32)) as u64; rdx = ((rdx).wrapping_add(r8)) as u64; fl_l = rdx; fl_r = rax; rdx = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { rdx }) as u64; rcx = r15; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru32(r15.wrapping_add(0x8u64))) as u32 as u64; rax = (crate::simd_ops::ru64(r15)) as u64; crate::simd_ops::wu32(rax.wrapping_add(rcx.wrapping_mul(4)), ((rbp as u32)) as u32); crate::simd_ops::lk_add_u32(r15.wrapping_add(0x8u64), (1) as u32); rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r11 = rsp.wrapping_add(0x70); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rbp = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x30u64))) as u64; rsp = r11; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141869400(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14020aaa0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = r8; fl_r = (0xffu64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 30; continue; } bb = 1; continue; }
            1 => { rax = rsp; crate::simd_ops::wu64(rax.wrapping_add(0x10u64), rdx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rbp = rsp; rsp = ((rsp).wrapping_sub(0x40u64)) as u64; crate::simd_ops::wu64(rax.wrapping_add(0x8u64), rsi); rbx = rcx; crate::simd_ops::wu64(rax.wrapping_add(0x18u64), rdi); rcx = r9; crate::simd_ops::wu64(rax.wrapping_add(0x20u64), r15); rsi = 0; fl_l = 0; fl_r = 0; rcx = ((rcx).wrapping_shr(24u32)) as u64; rax = r8; r13 = r9; r12 = r8; r14 = rdx; rdi = r8; fl_l = rax; fl_r = (0x10u64) as u64; rax = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 6; continue; } bb = 2; continue; }
            2 => { fl_l = rax; fl_r = (0x1u64) as u64; rax = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 5; continue; } bb = 3; continue; }
            3 => { fl_l = rax; fl_r = (0x1u64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 7; continue; } bb = 4; continue; }
            4 => { rdi = (((rcx as u8)) as u32) as u32 as u64; rdi = ((((rdi as u32)) & ((rax as u32)))) as u32 as u64; fl_l = ((rdi).wrapping_add(0xa4u64)) as u64; fl_r = fl_l; rdi = ((fl_l as u64)) as u64; bb = 7; continue; }
            5 => { rdi = rcx; rdi = ((((rdi as u32)) & (0x1u32))) as u32 as u64; fl_l = ((rdi).wrapping_add(0xa2u64)) as u64; fl_r = fl_l; rdi = ((fl_l as u64)) as u64; bb = 7; continue; }
            6 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; r8 = (crate::simd_ops::ru64(rax.wrapping_add(0x50u64))) as u64; rax = r13; rax = ((((rax) as i64).wrapping_shr(16u32) as u64)) as u64; rdx = (((rax as u8)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(r8, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; rdi = rsi; rdi = (rdi & !0xffu64) | (((if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { 1u8 } else { 0u8 }) as u8 as u64)); fl_l = ((rdi).wrapping_add(0xa0u64)) as u64; fl_r = fl_l; rdi = ((fl_l as u64)) as u64; bb = 7; continue; }
            7 => { r9 = (crate::simd_ops::ru32(rbp.wrapping_add(0x50u64))) as u32 as u64; fl_l = ((r9 as u32)) as u32 as u64; fl_r = 0x1; crate::simd_ops::wu32(rbp.wrapping_sub(0x20u64), (0x8u32) as u32); r15 = (r15 & !0xffu64) | (((if (((fl_l as u32)) == ((fl_r as u32))) { 1u8 } else { 0u8 }) as u8 as u64)); crate::simd_ops::wu32(rbp.wrapping_sub(0x8u64), ((rsi as u32)) as u32); crate::simd_ops::wu8(rdi.wrapping_add(rbx).wrapping_add(0x60u64), ((r15 as u8)) as u8); crate::simd_ops::wu64(rbp.wrapping_sub(0x18u64), r14); crate::simd_ops::wu32(rbp.wrapping_sub(0x10u64), ((rdi as u32)) as u32); fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0x70u64))) as u8 as u64; fl_r = ((rsi as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 10; continue; } bb = 8; continue; }
            8 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0x100u64))) as u8 as u64; fl_r = ((rsi as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 10; continue; } bb = 9; continue; }
            9 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0x101u64))) as u8 as u64; fl_r = ((rsi as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 11; continue; } bb = 10; continue; }
            10 => { rsi = 0x1; crate::simd_ops::wu32(rbp.wrapping_sub(0x8u64), ((rsi as u32)) as u32); bb = 11; continue; }
            11 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0x71u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 14; continue; } bb = 12; continue; }
            12 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0x102u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 14; continue; } bb = 13; continue; }
            13 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0x103u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 15; continue; } bb = 14; continue; }
            14 => { fl_l = ((((rsi as u32)) | (0x2u32))) as u32 as u64; fl_r = fl_l; rsi = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_sub(0x8u64), ((rsi as u32)) as u32); bb = 15; continue; }
            15 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0x72u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 18; continue; } bb = 16; continue; }
            16 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0x104u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 18; continue; } bb = 17; continue; }
            17 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0x105u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 19; continue; } bb = 18; continue; }
            18 => { fl_l = ((((rsi as u32)) | (0x4u32))) as u32 as u64; fl_r = fl_l; rsi = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_sub(0x8u64), ((rsi as u32)) as u32); bb = 19; continue; }
            19 => { crate::simd_ops::wu8(rbp.wrapping_sub(0x4u64), ((r15 as u8)) as u8); fl_l = ((r9 as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 22; continue; } bb = 20; continue; }
            20 => { fl_l = (((r13) >> (30u32)) & 1) as u64; if ((fl_l as u64) == 0) { bb = 22; continue; } bb = 21; continue; }
            21 => { r14 = (((r9 as u8)) as u32) as u32 as u64; bb = 23; continue; }
            22 => { r14 = (r14 & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 23; continue; }
            23 => { rax = (crate::simd_ops::ru64(rbx)) as u64; r8 = ((r13 as u32)) as u32 as u64; rdx = ((r12 as u32)) as u32 as u64; crate::simd_ops::wu8(rbp.wrapping_sub(0x3u64), ((r14 as u8)) as u8); rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x40u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu16(rbp.wrapping_sub(0xcu64), ((rax as u16)) as u16); fl_l = r12; fl_r = (0x2cu64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 25; continue; } bb = 24; continue; }
            24 => { rdx = rbp.wrapping_sub(0x20); crate::simd_ops::wu8(rbp.wrapping_sub(0x4u64), (0x1u8) as u8); rcx = rbx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); r15 = (r15 & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; crate::simd_ops::wu8(rbp.wrapping_sub(0x4u64), ((r15 as u8)) as u8); bb = 25; continue; }
            25 => { rdx = rbp.wrapping_sub(0x20); rcx = rbx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); fl_l = ((r14 as u8)) as u8 as u64; fl_r = ((r14 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 29; continue; } bb = 26; continue; }
            26 => { rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x38u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x18u64), rax); crate::simd_ops::wu32(rbp.wrapping_sub(0x20u64), (0x7u32) as u32); crate::simd_ops::wu32(rbp.wrapping_sub(0x10u64), ((rdi as u32)) as u32); crate::simd_ops::wu32(rbp.wrapping_sub(0xcu64), ((rsi as u32)) as u32); crate::simd_ops::wu8(rbp.wrapping_sub(0x8u64), ((r15 as u8)) as u8); fl_l = r12; fl_r = (0x2cu64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 28; continue; } bb = 27; continue; }
            27 => { rdx = rbp.wrapping_sub(0x20); crate::simd_ops::wu8(rbp.wrapping_sub(0x8u64), (0x1u8) as u8); rcx = rbx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu8(rbp.wrapping_sub(0x8u64), ((r14 as u8)) as u8); bb = 28; continue; }
            28 => { rdx = rbp.wrapping_sub(0x20); rcx = rbx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); bb = 29; continue; }
            29 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x88u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); bb = 30; continue; }
            30 => { return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1418695f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x18; const P3: u64 = 0x20; const P4: u64 = 0xffffffffffffffd8; const P5: u64 = 0xffffffffffffffe0; const P6: u64 = 0xffffffffffffffe8; const P7: u64 = 0x160; const P8: u64 = 0x1426455a8; const P9: u64 = 0x140209af0; const P10: u64 = 0x1; const P11: u64 = 0x1401deb40; const P12: u64 = 0x142645598;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = rsp; crate::simd_ops::wu64(rax.wrapping_add(P0), rbx); crate::simd_ops::wu64(rax.wrapping_add(P1), rbp); crate::simd_ops::wu64(rax.wrapping_add(P2), rsi); crate::simd_ops::wu64(rax.wrapping_add(P3), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rbp = rcx; crate::simd_ops::wu32(rax.wrapping_add(P4), (0x6u32) as u32); crate::simd_ops::wu64(rax.wrapping_add(P5), rdx); crate::simd_ops::wu16(rax.wrapping_add(P6), ((r8 as u16)) as u16); rbx = ((rcx.wrapping_add(P7)) as u64) as u64; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P8), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r14 = (crate::simd_ops::ru32(rbp.wrapping_add(0x170u64))) as u32 as u64; r14 = (((r14 as u32)).wrapping_add(0x7u32)) as u32 as u64; r14 = ((((r14 as u32)) & (0xfffffff8u32))) as u32 as u64; rdx = ((r14.wrapping_add(P2)) as u64) as u64; rcx = rbp.wrapping_add(0x168); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x168u64))) as u64; xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; crate::simd_ops::wu128(rax.wrapping_add(r14), (xmm0) as u128); xmm1 = ((crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64) as u128; crate::simd_ops::wu64(rax.wrapping_add(r14).wrapping_add(P1), ((xmm1 as u64)) as u64); rbp = ((rbp).wrapping_add(0x178u64)) as u64; rdx = (crate::simd_ops::ru32(rbp.wrapping_add(0x8u64))) as u32 as u64; r8 = (crate::simd_ops::ru32(rbp.wrapping_add(0xcu64))) as u32 as u64; rcx = ((r8.wrapping_mul(4)) as u64) as u64; rax = ((rdx.wrapping_mul(4)) as u64) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = ((rdx.wrapping_add(P10)) as u64) as u64; rdx = ((r8 as u32)) as u32 as u64; rdx = ((rdx).wrapping_shr(1u32)) as u64; rdx = ((rdx).wrapping_add(r8)) as u64; fl_l = rdx; fl_r = rax; rdx = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { rdx }) as u64; rcx = rbp; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P11, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru32(rbp.wrapping_add(0x8u64))) as u32 as u64; rax = (crate::simd_ops::ru64(rbp)) as u64; crate::simd_ops::wu32(rax.wrapping_add(rcx.wrapping_mul(4)), ((r14 as u32)) as u32); crate::simd_ops::lk_add_u32(rbp.wrapping_add(0x8u64), (1) as u32); rcx = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P12), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141869700(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x140000000; const P1: u64 = 0x1426455a8; const P2: u64 = 0x140209af0; const P3: u64 = 0x1401deb40; const P4: u64 = 0x142645598; const P5: u64 = 0x141869400; const P6: u64 = 0x142646220; const P7: u64 = 0x1402098b0; const P8: u64 = 0x1426456f8; const P9: u64 = 0x14020ab80; const P10: u64 = 0x141869250; const P11: u64 = 0x141868f00;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut j_tgt, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0; let mut ymm1_hi: u128 = 0; let mut ymm2_hi: u128 = 0; let mut ymm3_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 33>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rbp = rsp.wrapping_sub(0x2f); rsp = ((rsp).wrapping_sub(0xe0u64)) as u64; rbx = r9; r15 = ((r8 as u32)) as u32 as u64; rdi = rdx; r14 = rcx; fl_l = ((r8 as u32)) as u32 as u64; fl_r = 0x200; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 32; continue; } bb = 1; continue; }
            1 => { if (((fl_l as u32)) == ((fl_r as u32))) { bb = 27; continue; } bb = 2; continue; }
            2 => { rax = ((r8.wrapping_sub(0x7u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0xfe; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 52; continue; } bb = 3; continue; }
            3 => { rdx = P0; rax = ((crate::simd_ops::ru8(rdx.wrapping_add(rax).wrapping_add(0x1869d6cu64))) as u32) as u32 as u64; rcx = (crate::simd_ops::ru32(rdx.wrapping_add(rax.wrapping_mul(4)).wrapping_add(0x1869d48u64))) as u32 as u64; fl_l = ((rcx).wrapping_add(rdx)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; j_tgt = rcx; bb = 0xffff_fffe; continue; }
            4 => { crate::simd_ops::wu32(rbp.wrapping_sub(0x79u64), (0x6u32) as u32); crate::simd_ops::wu64(rbp.wrapping_sub(0x71u64), rdi); crate::simd_ops::wu16(rbp.wrapping_sub(0x69u64), ((rbx as u16)) as u16); rbx = r14.wrapping_add(0x160); rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r15 = (crate::simd_ops::ru32(r14.wrapping_add(0x170u64))) as u32 as u64; r15 = (((r15 as u32)).wrapping_add(0x7u32)) as u32 as u64; r15 = ((((r15 as u32)) & (0xfffffff8u32))) as u32 as u64; rdx = r15.wrapping_add(0x18); rcx = r14.wrapping_add(0x168); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(r14.wrapping_add(0x168u64))) as u64; xmm0 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x79u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rax.wrapping_add(r15), (xmm0) as u128); xmm1 = ((crate::simd_ops::ru64(rbp.wrapping_sub(0x69u64))) as u64) as u128; ymm1_hi = 0; crate::simd_ops::wu64(rax.wrapping_add(r15).wrapping_add(0x10u64), ((xmm1 as u64)) as u64); r14 = ((r14).wrapping_add(0x178u64)) as u64; rdx = (crate::simd_ops::ru32(r14.wrapping_add(0x8u64))) as u32 as u64; r8 = (crate::simd_ops::ru32(r14.wrapping_add(0xcu64))) as u32 as u64; rcx = ((r8.wrapping_mul(4)) as u64) as u64; rax = ((rdx.wrapping_mul(4)) as u64) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = rdx.wrapping_add(0x1); rdx = ((r8 as u32)) as u32 as u64; rdx = ((rdx).wrapping_shr(1u32)) as u64; rdx = ((rdx).wrapping_add(r8)) as u64; fl_l = rdx; fl_r = rax; rdx = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { rdx }) as u64; rcx = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru32(r14.wrapping_add(0x8u64))) as u32 as u64; rax = (crate::simd_ops::ru64(r14)) as u64; crate::simd_ops::wu32(rax.wrapping_add(rcx.wrapping_mul(4)), ((r15 as u32)) as u32); crate::simd_ops::lk_add_u32(r14.wrapping_add(0x8u64), (1) as u32); rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = 0; fl_l = 0; fl_r = 0; bb = 10; continue; }
            7 => { crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x1u32) as u32); bb = 8; continue; }
            8 => { rsi = (crate::simd_ops::ru64(rbp.wrapping_add(0x7fu64))) as u64; r9 = rsi; r8 = rbx; rdx = rdi; rcx = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rbx; fl_r = (0x7bu64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 15; continue; } bb = 9; continue; }
            9 => { r9 = rsi; r8 = rbx; rdx = ((r15 as u32)) as u32 as u64; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 10; continue; }
            10 => { r11 = rsp.wrapping_add(0xe0); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x38u64))) as u64; rdi = (crate::simd_ops::ru64(r11.wrapping_add(0x40u64))) as u64; rsp = r11; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            11 => { rsi = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rsi as u32)) as u32); bb = 8; continue; }
            12 => { fl_l = rbx; fl_r = (0x73u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 52; continue; } bb = 13; continue; }
            13 => { crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x1u32) as u32); bb = 14; continue; }
            14 => { r9 = (crate::simd_ops::ru64(rbp.wrapping_add(0x7fu64))) as u64; r8 = rbx; rdx = rdi; rcx = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 15; continue; }
            15 => { rax = 0; fl_l = 0; fl_r = 0; bb = 10; continue; }
            16 => { rsi = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rsi as u32)) as u32); bb = 14; continue; }
            17 => { rax = (crate::simd_ops::ru64(r14)) as u64; r8 = rbx; rdx = rdi; rcx = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x68u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(rbp.wrapping_sub(0x79u64), (0x1u32) as u32); crate::simd_ops::wu64(rbp.wrapping_sub(0x71u64), rdi); crate::simd_ops::wu8(rbp.wrapping_sub(0x69u64), (0x1u8) as u8); rdx = rbp.wrapping_sub(0x79); rcx = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = 0; fl_l = 0; fl_r = 0; bb = 10; continue; }
            18 => { rax = (crate::simd_ops::ru64(r14)) as u64; r8 = rbx; rdx = rdi; rcx = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x60u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(rbp.wrapping_sub(0x79u64), (0x1u32) as u32); crate::simd_ops::wu64(rbp.wrapping_sub(0x71u64), rdi); crate::simd_ops::wu8(rbp.wrapping_sub(0x69u64), (0x0u8) as u8); rdx = rbp.wrapping_sub(0x79); rcx = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = 0; fl_l = 0; fl_r = 0; bb = 10; continue; }
            19 => { fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 24; continue; } bb = 20; continue; }
            20 => { rsi = 0; fl_l = 0; fl_r = 0; r12 = ((rsi as u32)) as u32 as u64; r13 = ((rsi as u32)) as u32 as u64; r15 = r14.wrapping_add(0x60); bb = 21; continue; }
            21 => { fl_l = (crate::simd_ops::ru8(r15)) as u8 as u64; fl_r = ((rsi as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 23; continue; } bb = 22; continue; }
            22 => { crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rsi as u32)) as u32); r9 = 0; fl_l = 0; fl_r = 0; r8 = r13; rdx = rdi; rcx = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 23; continue; }
            23 => { r12 = (((r12 as u32)).wrapping_add(1)) as u32 as u64; r13 = ((r13).wrapping_add(1)) as u64; r15 = ((r15).wrapping_add(1)) as u64; fl_l = ((r12 as u32)) as u32 as u64; fl_r = 0x100; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 21; continue; } bb = 24; continue; }
            24 => { crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), (0x13u32) as u32); fl_l = rbx; fl_r = rbx; crate::simd_ops::wu8(rbp.wrapping_sub(0x7du64), (if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { 1u8 } else { 0u8 }) as u8); rbx = r14.wrapping_add(0x160); rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r15 = (crate::simd_ops::ru32(r14.wrapping_add(0x170u64))) as u32 as u64; r15 = (((r15 as u32)).wrapping_add(0x3u32)) as u32 as u64; r15 = ((((r15 as u32)) & (0xfffffffcu32))) as u32 as u64; rdx = r15.wrapping_add(0x8); rcx = r14.wrapping_add(0x168); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(r14.wrapping_add(0x168u64))) as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu64(r15.wrapping_add(rcx), rax); r14 = ((r14).wrapping_add(0x178u64)) as u64; rdx = (crate::simd_ops::ru32(r14.wrapping_add(0x8u64))) as u32 as u64; r8 = (crate::simd_ops::ru32(r14.wrapping_add(0xcu64))) as u32 as u64; rcx = ((r8.wrapping_mul(4)) as u64) as u64; rax = ((rdx.wrapping_mul(4)) as u64) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 26; continue; } bb = 25; continue; }
            25 => { rax = rdx.wrapping_add(0x1); rdx = ((r8 as u32)) as u32 as u64; rdx = ((rdx).wrapping_shr(1u32)) as u64; rdx = ((rdx).wrapping_add(r8)) as u64; fl_l = rdx; fl_r = rax; rdx = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { rdx }) as u64; rcx = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 26; continue; }
            26 => { rcx = (crate::simd_ops::ru32(r14.wrapping_add(0x8u64))) as u32 as u64; rax = (crate::simd_ops::ru64(r14)) as u64; crate::simd_ops::wu32(rax.wrapping_add(rcx.wrapping_mul(4)), ((r15 as u32)) as u32); crate::simd_ops::lk_add_u32(r14.wrapping_add(0x8u64), (1) as u32); rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = 0; fl_l = 0; fl_r = 0; bb = 10; continue; }
            27 => { crate::simd_ops::wu32(rbp.wrapping_sub(0x59u64), (0xau32) as u32); xmm0 = 0u128; ymm0_hi = 0; crate::simd_ops::wu32(rbp.wrapping_sub(0x49u64), ((xmm0 as u32)) as u32); crate::simd_ops::wu32(rbp.wrapping_sub(0x45u64), ((xmm0 as u32)) as u32); crate::simd_ops::wu128(rbp.wrapping_sub(0x41u64), (xmm0) as u128); crate::simd_ops::wu64(rbp.wrapping_sub(0x51u64), rdi); rcx = ((rbx as u32)) as u32 as u64; rcx = ((((rcx as u32)) & (0x1u32))) as u32 as u64; rdx = ((rcx as u32)) as u32 as u64; rdx = ((((rdx as u32)) | (0x2u32))) as u32 as u64; fl_l = ((rbx as u8)) as u8 as u64; fl_r = (0x10u8) as u8 as u64; rdx = (if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { (rcx as u32) } else { (rdx as u32) }) as u32 as u64; rcx = ((rdx as u32)) as u32 as u64; rcx = ((((rcx as u32)) | (0x4u32))) as u32 as u64; fl_l = ((rbx as u8)) as u8 as u64; fl_r = (0x2u8) as u8 as u64; rcx = (if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { (rdx as u32) } else { (rcx as u32) }) as u32 as u64; rdx = ((rcx as u32)) as u32 as u64; rdx = ((((rdx as u32)) | (0x8u32))) as u32 as u64; fl_l = ((rbx as u8)) as u8 as u64; fl_r = (0x8u8) as u8 as u64; rdx = (if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { (rcx as u32) } else { (rdx as u32) }) as u32 as u64; rax = ((rdx as u32)) as u32 as u64; rax = ((((rax as u32)) | (0x10u32))) as u32 as u64; fl_l = ((rbx as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64; rax = (if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { (rdx as u32) } else { (rax as u32) }) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_sub(0x31u64), ((rax as u32)) as u32); rcx = rsp.wrapping_add(0x30); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P8), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x29u64), rax); rax = (crate::simd_ops::ru64(r14)) as u64; rcx = rbp.wrapping_sub(0x59); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rcx); r9 = 0; fl_l = 0; fl_r = 0; r8 = (crate::simd_ops::ru64(rbp.wrapping_add(0x7fu64))) as u64; rdx = rdi; rcx = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x48u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = r14.wrapping_add(0x188); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); let y_addr: u64 = r14.wrapping_add(0x190u64); xmm3 = crate::simd_ops::ru128(y_addr); ymm3_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = r14.wrapping_add(0x1b0u64); xmm2 = crate::simd_ops::ru128(y_addr); ymm2_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rbp.wrapping_sub(0x79u64); crate::simd_ops::wu128(y_addr, xmm2); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm2_hi); let y_addr: u64 = rbp.wrapping_sub(0x59u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); xmm1 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x39u64))) as u128; ymm1_hi = 0; let y_addr: u64 = r14.wrapping_add(0x190u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); crate::simd_ops::wu128(r14.wrapping_add(0x1b0u64), (xmm1) as u128); xmm0 = ((crate::simd_ops::ru64(rbp.wrapping_sub(0x29u64))) as u64) as u128; ymm0_hi = 0; crate::simd_ops::wu64(r14.wrapping_add(0x1c0u64), ((xmm0 as u64)) as u64); fl_l = (crate::simd_ops::ru8(r14.wrapping_add(0x1c8u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 29; continue; } bb = 28; continue; }
            28 => { crate::simd_ops::wu8(r14.wrapping_add(0x1c8u64), (0x1u8) as u8); bb = 29; continue; }
            29 => { let y_addr: u64 = rbp.wrapping_sub(0x11u64); crate::simd_ops::wu128(y_addr, xmm3); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm3_hi); let y_addr: u64 = rbp.wrapping_add(0xfu64); crate::simd_ops::wu128(y_addr, xmm2); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm2_hi); rcx = r14.wrapping_add(0x188); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); let y_addr: u64 = rbp.wrapping_sub(0x79u64); xmm1 = crate::simd_ops::ru128(y_addr); ymm1_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); xmm0 = (ymm1_hi) as u128; ymm0_hi = 0; rax = (((xmm0 >> 64) as u8) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 15; continue; } bb = 30; continue; }
            30 => { rax = (((xmm1 >> 64) as u32) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rbp.wrapping_sub(0x31u64))) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 15; continue; } bb = 31; continue; }
            31 => { rdx = rbp.wrapping_sub(0x11); rcx = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = 0; fl_l = 0; fl_r = 0; bb = 10; continue; }
            32 => { rax = ((r8.wrapping_sub(0x201u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0xd; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 52; continue; } bb = 33; continue; }
            33 => { rdx = P0; rax = (crate::simd_ops::ru32(rdx.wrapping_add(rax.wrapping_mul(4)).wrapping_add(0x1869e6cu64))) as u32 as u64; fl_l = ((rax).wrapping_add(rdx)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; j_tgt = rax; bb = 0xffff_fffe; continue; }
            34 => { crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0x1u32) as u32); rsi = 0; fl_l = 0; fl_r = 0; bb = 35; continue; }
            35 => { crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rsi as u32)) as u32); bb = 36; continue; }
            36 => { r8 = rbx; r9 = (crate::simd_ops::ru64(rbp.wrapping_add(0x7fu64))) as u64; rdx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P10, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = 0; fl_l = 0; fl_r = 0; bb = 10; continue; }
            37 => { crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0x1u32) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x1u32) as u32); r9 = (crate::simd_ops::ru64(rbp.wrapping_add(0x7fu64))) as u64; r8 = rbx; rdx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P10, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = 0; fl_l = 0; fl_r = 0; bb = 10; continue; }
            38 => { crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0x1u32) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x2u32) as u32); r9 = (crate::simd_ops::ru64(rbp.wrapping_add(0x7fu64))) as u64; r8 = rbx; rdx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P10, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = 0; fl_l = 0; fl_r = 0; bb = 10; continue; }
            39 => { rax = rbx; rax = ((rax).wrapping_shr(16u32)) as u64; rcx = (((rax as u16)) as u32) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 42; continue; } bb = 40; continue; }
            40 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 52; continue; } bb = 41; continue; }
            41 => { r8 = (((rbx as u16)) as u32) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rcx as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x4u32) as u32); r9 = (crate::simd_ops::ru64(rbp.wrapping_add(0x7fu64))) as u64; rdx = rdi; rcx = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P10, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = 0; fl_l = 0; fl_r = 0; bb = 10; continue; }
            42 => { r8 = (((rbx as u16)) as u32) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0x1u32) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x3u32) as u32); r9 = (crate::simd_ops::ru64(rbp.wrapping_add(0x7fu64))) as u64; rdx = rdi; rcx = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P10, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = 0; fl_l = 0; fl_r = 0; bb = 10; continue; }
            43 => { rsi = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rsi as u32)) as u32); bb = 35; continue; }
            44 => { rsi = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rsi as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x1u32) as u32); bb = 36; continue; }
            45 => { rsi = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rsi as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x2u32) as u32); bb = 36; continue; }
            46 => { rax = rbx; rax = ((rax).wrapping_shr(16u32)) as u64; rcx = (((rax as u16)) as u32) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 49; continue; } bb = 47; continue; }
            47 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 52; continue; } bb = 48; continue; }
            48 => { r8 = (((rbx as u16)) as u32) as u32 as u64; rsi = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rsi as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x4u32) as u32); rcx = r14; r9 = (crate::simd_ops::ru64(rbp.wrapping_add(0x7fu64))) as u64; rdx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P10, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = 0; fl_l = 0; fl_r = 0; bb = 10; continue; }
            49 => { r8 = (((rbx as u16)) as u32) as u32 as u64; rsi = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rsi as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x3u32) as u32); rcx = r14; r9 = (crate::simd_ops::ru64(rbp.wrapping_add(0x7fu64))) as u64; rdx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P10, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = 0; fl_l = 0; fl_r = 0; bb = 10; continue; }
            50 => { rsi = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rsi as u32)) as u32); r9 = (crate::simd_ops::ru64(rbp.wrapping_add(0x7fu64))) as u64; r8 = rbx; rdx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P11, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = 0; fl_l = 0; fl_r = 0; bb = 10; continue; }
            51 => { crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x1u32) as u32); r9 = (crate::simd_ops::ru64(rbp.wrapping_add(0x7fu64))) as u64; r8 = rbx; rdx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P11, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = 0; fl_l = 0; fl_r = 0; bb = 10; continue; }
            52 => { rax = (crate::simd_ops::ru64(r14)) as u64; rsi = (crate::simd_ops::ru64(rbp.wrapping_add(0x7fu64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbx); r9 = ((r15 as u32)) as u32 as u64; r8 = rdi; rdx = rbp.wrapping_sub(0x21); rcx = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x38u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru8(rbp.wrapping_sub(0x19u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 9; continue; } bb = 53; continue; }
            53 => { rax = (crate::simd_ops::ru64(rbp.wrapping_sub(0x21u64))) as u64; bb = 10; continue; }
            0xffff_fffe => { match j_tgt.wrapping_sub(0x141869700u64) { 0x3d => { bb = 1; continue; } 0x43 => { bb = 2; continue; } 0x52 => { bb = 3; continue; } 0x6d => { bb = 4; continue; } 0xed => { bb = 5; continue; } 0x109 => { bb = 6; continue; } 0x125 => { bb = 7; continue; } 0x12d => { bb = 8; continue; } 0x148 => { bb = 9; continue; } 0x15a => { bb = 10; continue; } 0x17e => { bb = 11; continue; } 0x186 => { bb = 12; continue; } 0x190 => { bb = 13; continue; } 0x198 => { bb = 14; continue; } 0x1aa => { bb = 15; continue; } 0x1ae => { bb = 16; continue; } 0x1b6 => { bb = 17; continue; } 0x1e7 => { bb = 18; continue; } 0x218 => { bb = 19; continue; } 0x21d => { bb = 20; continue; } 0x230 => { bb = 21; continue; } 0x235 => { bb = 22; continue; } 0x24a => { bb = 23; continue; } 0x25c => { bb = 24; continue; } 0x2ce => { bb = 25; continue; } 0x2ea => { bb = 26; continue; } 0x309 => { bb = 27; continue; } 0x3e3 => { bb = 28; continue; } 0x3eb => { bb = 29; continue; } 0x41e => { bb = 30; continue; } 0x42d => { bb = 31; continue; } 0x443 => { bb = 32; continue; } 0x453 => { bb = 33; continue; } 0x466 => { bb = 34; continue; } 0x470 => { bb = 35; continue; } 0x474 => { bb = 36; continue; } 0x48a => { bb = 37; continue; } 0x4b0 => { bb = 38; continue; } 0x4d6 => { bb = 39; continue; } 0x4e5 => { bb = 40; continue; } 0x4ee => { bb = 41; continue; } 0x514 => { bb = 42; continue; } 0x53e => { bb = 43; continue; } 0x549 => { bb = 44; continue; } 0x55c => { bb = 45; continue; } 0x56f => { bb = 46; continue; } 0x57e => { bb = 47; continue; } 0x587 => { bb = 48; continue; } 0x5af => { bb = 49; continue; } 0x5d7 => { bb = 50; continue; } 0x5f3 => { bb = 51; continue; } 0x611 => { bb = 52; continue; } 0x63c => { bb = 53; continue; } _ => { return crate::simd_ops::tail_stk_xmm(j_tgt, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); } } }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141869eb0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14020af00; const P1: u64 = 0x18; const P2: u64 = 0x10; const P3: u64 = 0x1f0; const P4: u64 = 0x14186a310; const P5: u64 = 0x8; const P6: u64 = 0x142442de0; const P7: u64 = 0x2f8; const P8: u64 = 0x142645748; const P9: u64 = 0x143406c90; const P10: u64 = 0x1423f0780; const P11: u64 = 0x14136eec0; const P12: u64 = 0x141869f8f;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x141869eb0, funcinfo_rva: 0x36fa03c, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x58, conts: &[] };
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rsi = (rcx) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x60u64), (0x0u32) as u32); rcx = ((rsp.wrapping_add(0x68u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141869ec8); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; crate::simd_ops::wu8(rax.wrapping_add(P1), (0x1u8) as u8); rdi = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rdi) as u64); crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0x0u32) as u32); crate::simd_ops::lk_add_u32(rdi, (1) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x60u64), (0x2u32) as u32); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), (0x0u64) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), (rax) as u64); rdx = ((rsp.wrapping_add(0x70u64)) as u64) as u64; rcx = ((rsi.wrapping_add(P3)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141869f12); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = (rdi) as u64 as u64; fl_r = (rdi) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 1; continue; }
            1 => { rax = (0xffffffffu32) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rdi, xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 5; continue; } bb = 2; continue; }
            2 => { rbx = (crate::simd_ops::ru64(rdi.wrapping_add(P5))) as u64; fl_l = (((rbx) & (0xfffffffffffffffcu64))) as u64 as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141869f3d); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P5))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141869f43); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 4; continue; }
            4 => { rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141869f4c); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 5; continue; }
            5 => { rcx = (crate::simd_ops::ru64(rsi.wrapping_add(P7))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { rdx = (0xffffffffu32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141869f63); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P8)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rsi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            8 => { rdx = ((P9) as u64) as u64; rcx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141869f81); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P10) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = (rax) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141869f8a); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P11) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::lift_ops::trap(0x80000003, P12, rcx, rsp); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14186a0c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143080ca8; const P1: u64 = 0x143080c78; const P2: u64 = 0x1f0; const P3: u64 = 0x14020a7d0; const P4: u64 = 0x1423f0c20; const P5: u64 = 0x1e0; const P6: u64 = 0x1401de830; const P7: u64 = 0x1d0; const P8: u64 = 0x178; const P9: u64 = 0x168; const P10: u64 = 0x14186b980;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdi = rcx;
    rax = P0;
    crate::simd_ops::wu64(rcx, rax);
    rax = P1;
    crate::simd_ops::wu64(rcx.wrapping_add(P2), rax);
    rcx = ((rcx).wrapping_add(0x318u64)) as u64;
    rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp);
    rcx = ((rdi.wrapping_add(P2)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp);
    rcx = ((rdi.wrapping_add(P5)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp);
    rcx = ((rdi.wrapping_add(P7)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp);
    rcx = ((rdi.wrapping_add(P8)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp);
    rcx = ((rdi.wrapping_add(P9)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp);
    rcx = rdi;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(P10, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14186a150(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14397d820; const P1: u64 = 0x143080ca8; const P2: u64 = 0x142645728; const P3: u64 = 0x14183acc2; const P4: u64 = 0x143080c60; const P5: u64 = 0x1426469b8; const P6: u64 = 0x143080c78;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 37>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x120u64)) as u64; rdi = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(P0, rcx); crate::simd_ops::wu64(rcx.wrapping_add(0x8u64), rdi); rax = P1; crate::simd_ops::wu64(rcx.wrapping_add(0x10u64), rdi); rbx = rcx; crate::simd_ops::wu64(rcx.wrapping_add(0x18u64), rdi); crate::simd_ops::wu64(rcx.wrapping_add(0x20u64), rdi); crate::simd_ops::wu64(rcx.wrapping_add(0x28u64), rdi); crate::simd_ops::wu64(rcx.wrapping_add(0x30u64), rdi); crate::simd_ops::wu64(rcx.wrapping_add(0x38u64), rdi); crate::simd_ops::wu64(rcx.wrapping_add(0x40u64), rdi); crate::simd_ops::wu64(rcx.wrapping_add(0x48u64), rdi); crate::simd_ops::wu64(rcx.wrapping_add(0x50u64), rdi); crate::simd_ops::wu64(rcx, rax); rcx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rbx.wrapping_add(0x60); rdx = 0; fl_l = 0; fl_r = 0; r8 = 0x100; crate::simd_ops::wu64(rbx.wrapping_add(0x58u64), rax); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rbx.wrapping_add(0x160u64), rdi); rcx = rsp.wrapping_add(0x31); crate::simd_ops::wu64(rbx.wrapping_add(0x168u64), rdi); rdx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rbx.wrapping_add(0x170u64), rdi); r8 = 0xef; crate::simd_ops::wu64(rbx.wrapping_add(0x178u64), rdi); crate::simd_ops::wu64(rbx.wrapping_add(0x180u64), rdi); crate::simd_ops::wu64(rbx.wrapping_add(0x188u64), rdi); crate::simd_ops::wu8(rbx.wrapping_add(0x1c8u64), ((rdi as u8)) as u8); crate::simd_ops::wu64(rbx.wrapping_add(0x1d0u64), rdi); crate::simd_ops::wu64(rbx.wrapping_add(0x1d8u64), rdi); crate::simd_ops::wu64(rbx.wrapping_add(0x1e0u64), rdi); crate::simd_ops::wu64(rbx.wrapping_add(0x1e8u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0xffffffffu32) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x2cu64), (0x4u32) as u32); crate::simd_ops::wu8(rsp.wrapping_add(0x30u64), ((rdi as u8)) as u8); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = P4; rdx = 0xef; rcx = rsp.wrapping_add(0x31); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rbx.wrapping_add(0x1f8); rax = rsp.wrapping_add(0x20); rdx = ((rdi.wrapping_add(0x2u64)) as u32) as u32 as u64; bb = 1; continue; }
            1 => { rcx = rcx.wrapping_add(0x80); let y_addr: u64 = rax; xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); xmm1 = (crate::simd_ops::ru128(rax.wrapping_add(0x70u64))) as u128; rax = rax.wrapping_add(0x80); let y_addr: u64 = rcx.wrapping_sub(0x80u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); let y_addr: u64 = rax.wrapping_sub(0x60u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rcx.wrapping_sub(0x60u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); let y_addr: u64 = rax.wrapping_sub(0x40u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rcx.wrapping_sub(0x40u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); xmm0 = (crate::simd_ops::ru128(rax.wrapping_sub(0x20u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rcx.wrapping_sub(0x20u64), (xmm0) as u128); crate::simd_ops::wu128(rcx.wrapping_sub(0x10u64), (xmm1) as u128); fl_l = rdx; fl_r = (0x1u64) as u64; rdx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 2; continue; }
            2 => { rax = P6; crate::simd_ops::wu64(rbx.wrapping_add(0x2f8u64), rdi); crate::simd_ops::wu64(rbx.wrapping_add(0x1f0u64), rax); rax = rbx; crate::simd_ops::wu64(rbx.wrapping_add(0x300u64), rdi); crate::simd_ops::wu8(rbx.wrapping_add(0x308u64), ((rdi as u8)) as u8); crate::simd_ops::wu64(rbx.wrapping_add(0x310u64), rdi); crate::simd_ops::wu64(rbx.wrapping_add(0x318u64), rdi); crate::simd_ops::wu64(rbx.wrapping_add(0x320u64), rdi); crate::simd_ops::wu64(rbx.wrapping_add(0x328u64), rdi); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x130u64))) as u64; rsp = ((rsp).wrapping_add(0x120u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14186a310(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1426455a8; const P1: u64 = 0x14020acc0; const P2: u64 = 0x142645598; const P3: u64 = 0x142646210; const P4: u64 = 0x142645708; const P5: u64 = 0x1425371a0; const P6: u64 = 0x142533200;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = rdx; rbp = rcx; rbx = rcx.wrapping_add(0x120); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rbx); rcx = rbx; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); rdi = rbp.wrapping_add(0x128); rdx = (crate::simd_ops::ru32(rdi.wrapping_add(0x10u64))) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rdi.wrapping_add(0x14u64))) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rdx = (((rdx as u32)).wrapping_add(1)) as u32 as u64; rcx = rdi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0xcu64))) as u32 as u64; rax = (crate::simd_ops::ru64(rdi)) as u64; rdx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; rax = (crate::simd_ops::ru64(rsi)) as u64; rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsi, rcx); crate::simd_ops::wu64(rdx, rax); crate::simd_ops::lk_add_u32(rdi.wrapping_add(0xcu64), (1) as u32); rax = (crate::simd_ops::ru32(rdi.wrapping_add(0xcu64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rdi.wrapping_add(0x14u64))) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu32(rdi.wrapping_add(0xcu64), ((rcx as u32)) as u32); bb = 4; continue; }
            4 => { crate::simd_ops::lk_add_u32(rdi.wrapping_add(0x10u64), (1) as u32); rcx = rbx; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); r9 = 0; fl_l = 0; fl_r = 0; r8 = 0; fl_l = 0; fl_r = 0; rdx = 0x8000; rcx = (crate::simd_ops::ru32(rbp.wrapping_add(0x110u64))) as u32 as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            6 => { rbx = (crate::simd_ops::ru64(rsi)) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); r10 = rbx.wrapping_sub(0x1); r10 = (((r10) & (0xfffffffffe000000u64))) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(r10.wrapping_add(0x100u64))) as u64; rdx = (rdx & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); rax = rbx; rax = ((rax).wrapping_sub(r10)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rcx = r10.wrapping_add(0x108); rcx = ((rcx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x4u64))) as u32 as u64; rcx = ((rcx).wrapping_sub(rax)) as u64; fl_l = ((rdx as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 12; continue; } bb = 8; continue; }
            8 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0xeu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 11; continue; } bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(rbx, rax); crate::simd_ops::wu64(rcx.wrapping_add(0x20u64), rbx); fl_l = (crate::simd_ops::lk_add_u32(rcx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 14; continue; } bb = 10; continue; }
            10 => { rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); bb = 14; continue; }
            11 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 13; continue; }
            12 => { r8 = 0; fl_l = 0; fl_r = 0; bb = 13; continue; }
            13 => { rdx = rcx; r9 = rbx; rcx = r10; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); bb = 14; continue; }
            14 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14186a450(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142646078;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 23>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rbp = rsp.wrapping_sub(0x57); rsp = ((rsp).wrapping_sub(0xb0u64)) as u64; xmm0 = 0u128; ymm0_hi = 0; let y_addr: u64 = rbp.wrapping_add(0x7u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rbp.wrapping_add(0x7u64), (0x48u32) as u32); rbx = r9; crate::simd_ops::wu64(rbp.wrapping_add(0x27u64), rax); rdi = rcx; rdx = rbp.wrapping_add(0x7); let y_addr: u64 = rbp.wrapping_add(0x2fu64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 5; continue; } bb = 1; continue; }
            1 => { r8 = (crate::simd_ops::ru32(rbp.wrapping_add(0x2bu64))) as u32 as u64; fl_l = ((r8 as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 5; continue; } bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru32(rbp.wrapping_add(0xfu64))) as u32 as u64; r8 = (r8 & !0xffu64) | ((((((r8 as u8)) & (0x1u8))) as u8 as u64)); rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x7u64))) as u64; rax = (crate::simd_ops::ru64(rbp.wrapping_add(0xfu64))) as u64; rax = ((rax).wrapping_shr(32u32)) as u64; rdx = ((rdx).wrapping_shr(32u32)) as u64; rax = (((rax as u32)).wrapping_sub((rdx as u32))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_sub(0x31u64), ((rdx as u32)) as u32); rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x17u64))) as u64; crate::simd_ops::wu32(rbp.wrapping_sub(0x39u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x17u64))) as u32 as u64; rax = (((rax as u32)).wrapping_sub((rcx as u32))) as u32 as u64; rdx = ((rdx).wrapping_shr(32u32)) as u64; crate::simd_ops::wu32(rbp.wrapping_sub(0x35u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rbp.wrapping_sub(0x39u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x29u64), rax); crate::simd_ops::wu32(rbp.wrapping_sub(0x39u64), ((rdx as u32)) as u32); crate::simd_ops::wu32(rbp.wrapping_sub(0x2du64), ((rcx as u32)) as u32); rcx = (crate::simd_ops::ru32(rbp.wrapping_add(0x1fu64))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_sub(0x35u64), ((rcx as u32)) as u32); rax = (crate::simd_ops::ru64(rbp.wrapping_sub(0x39u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x21u64), rax); rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x1fu64))) as u64; rax = ((rax).wrapping_shr(32u32)) as u64; rax = (((rax as u32)).wrapping_sub((rdx as u32))) as u32 as u64; crate::simd_ops::wu8(rbp.wrapping_sub(0x11u64), ((r8 as u8)) as u8); crate::simd_ops::wu32(rbp.wrapping_sub(0x39u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x27u64))) as u32 as u64; rax = (((rax as u32)).wrapping_sub((rcx as u32))) as u32 as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x9u64), rdi); fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0x30u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; xmm1 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x11u64))) as u128; crate::simd_ops::wu32(rbp.wrapping_sub(0x35u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rbp.wrapping_sub(0x39u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x19u64), rax); let y_addr: u64 = rbp.wrapping_sub(0x31u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rbx; crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); crate::simd_ops::wu128(rbx.wrapping_add(0x20u64), (xmm1) as u128); if (((fl_l as u8)) != ((fl_r as u8))) { bb = 4; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu8(rbx.wrapping_add(0x30u64), (0x1u8) as u8); bb = 4; continue; }
            4 => { rax = 0; fl_l = 0; fl_r = 0; bb = 6; continue; }
            5 => { rax = 0x1; bb = 6; continue; }
            6 => { r11 = rsp.wrapping_add(0xb0); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x10u64))) as u64; rdi = (crate::simd_ops::ru64(r11.wrapping_add(0x18u64))) as u64; rsp = r11; rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14186a560(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142646078;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 21>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r9); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rbp = rsp.wrapping_sub(0x4f); rsp = ((rsp).wrapping_sub(0xa0u64)) as u64; xmm0 = 0u128; ymm0_hi = 0; rbx = rdx; rax = 0; fl_l = 0; fl_r = 0; let y_addr: u64 = rbp.wrapping_sub(0x1u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); crate::simd_ops::wu32(rbp.wrapping_sub(0x1u64), (0x48u32) as u32); rdx = rbp.wrapping_sub(0x1); crate::simd_ops::wu64(rbp.wrapping_add(0x1fu64), rax); rcx = rbx; let y_addr: u64 = rbp.wrapping_add(0x27u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 5; continue; } bb = 1; continue; }
            1 => { r8 = (crate::simd_ops::ru32(rbp.wrapping_add(0x23u64))) as u32 as u64; fl_l = ((r8 as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 5; continue; } bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru32(rbp.wrapping_add(0x7u64))) as u32 as u64; r8 = (r8 & !0xffu64) | ((((((r8 as u8)) & (0x1u8))) as u8 as u64)); rdx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x1u64))) as u64; rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x7u64))) as u64; rax = ((rax).wrapping_shr(32u32)) as u64; rdx = ((rdx).wrapping_shr(32u32)) as u64; rax = (((rax as u32)).wrapping_sub((rdx as u32))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_sub(0x31u64), ((rdx as u32)) as u32); rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0xfu64))) as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x77u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbp.wrapping_add(0xfu64))) as u32 as u64; rax = (((rax as u32)).wrapping_sub((rcx as u32))) as u32 as u64; rdx = ((rdx).wrapping_shr(32u32)) as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x7bu64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x77u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x29u64), rax); crate::simd_ops::wu32(rbp.wrapping_add(0x77u64), ((rdx as u32)) as u32); crate::simd_ops::wu32(rbp.wrapping_sub(0x2du64), ((rcx as u32)) as u32); rcx = (crate::simd_ops::ru32(rbp.wrapping_add(0x17u64))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x7bu64), ((rcx as u32)) as u32); rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x77u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x21u64), rax); rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x17u64))) as u64; rax = ((rax).wrapping_shr(32u32)) as u64; rax = (((rax as u32)).wrapping_sub((rdx as u32))) as u32 as u64; crate::simd_ops::wu8(rbp.wrapping_sub(0x11u64), ((r8 as u8)) as u8); crate::simd_ops::wu32(rbp.wrapping_add(0x77u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x1fu64))) as u32 as u64; rax = (((rax as u32)).wrapping_sub((rcx as u32))) as u32 as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x9u64), rbx); xmm1 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x11u64))) as u128; crate::simd_ops::wu32(rbp.wrapping_add(0x7bu64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x77u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x19u64), rax); rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x7fu64))) as u64; let y_addr: u64 = rbp.wrapping_sub(0x31u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x30u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; let y_addr: u64 = rax; crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); crate::simd_ops::wu128(rax.wrapping_add(0x20u64), (xmm1) as u128); if (((fl_l as u8)) != ((fl_r as u8))) { bb = 4; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu8(rax.wrapping_add(0x30u64), (0x1u8) as u8); bb = 4; continue; }
            4 => { rax = 0; fl_l = 0; fl_r = 0; bb = 6; continue; }
            5 => { rax = 0x1; bb = 6; continue; }
            6 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0xb0u64))) as u64; rsp = ((rsp).wrapping_add(0xa0u64)) as u64; rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14186a670(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142646078; const P1: u64 = 0x14020afa0;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 23>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rbp = rsp.wrapping_sub(0x57); rsp = ((rsp).wrapping_sub(0xb0u64)) as u64; rbx = r9; rdi = rcx; xmm0 = 0u128; ymm0_hi = 0; rax = 0; fl_l = 0; fl_r = 0; let y_addr: u64 = rbp.wrapping_sub(0x29u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); crate::simd_ops::wu64(rbp.wrapping_sub(0x9u64), rax); let y_addr: u64 = rbp.wrapping_sub(0x1u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); crate::simd_ops::wu32(rbp.wrapping_sub(0x29u64), (0x48u32) as u32); rdx = rbp.wrapping_sub(0x29); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { rdx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x29u64))) as u64; rdx = ((rdx).wrapping_shr(32u32)) as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x27u64), ((rdx as u32)) as u32); rax = (crate::simd_ops::ru32(rbp.wrapping_sub(0x21u64))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x2bu64), ((rax as u32)) as u32); rcx = (crate::simd_ops::ru32(rbp.wrapping_sub(0x19u64))) as u32 as u64; rcx = (((rcx as u32)).wrapping_sub((rax as u32))) as u32 as u64; rax = (crate::simd_ops::ru64(rbp.wrapping_sub(0x21u64))) as u64; rax = ((rax).wrapping_shr(32u32)) as u64; rax = (((rax as u32)).wrapping_sub((rdx as u32))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_sub(0x39u64), ((rax as u32)) as u32); crate::simd_ops::wu32(rbp.wrapping_sub(0x35u64), ((rcx as u32)) as u32); rax = (crate::simd_ops::ru64(rbp.wrapping_sub(0x39u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x2fu64), rax); r8 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x19u64))) as u64; r8 = ((r8).wrapping_shr(32u32)) as u64; crate::simd_ops::wu32(rbp.wrapping_sub(0x39u64), ((r8 as u32)) as u32); rcx = (crate::simd_ops::ru32(rbp.wrapping_sub(0x11u64))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_sub(0x35u64), ((rcx as u32)) as u32); rax = (crate::simd_ops::ru64(rbp.wrapping_sub(0x39u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x37u64), rax); rdx = (crate::simd_ops::ru32(rbp.wrapping_sub(0x9u64))) as u32 as u64; rdx = (((rdx as u32)).wrapping_sub((rcx as u32))) as u32 as u64; rax = (crate::simd_ops::ru64(rbp.wrapping_sub(0x11u64))) as u64; rax = ((rax).wrapping_shr(32u32)) as u64; rax = (((rax as u32)).wrapping_sub((r8 as u32))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_sub(0x39u64), ((rax as u32)) as u32); crate::simd_ops::wu32(rbp.wrapping_sub(0x35u64), ((rdx as u32)) as u32); rax = (crate::simd_ops::ru64(rbp.wrapping_sub(0x39u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x3fu64), rax); rax = ((crate::simd_ops::ru8(rbp.wrapping_sub(0x5u64))) as u32) as u32 as u64; rax = (rax & !0xffu64) | ((((((rax as u8)) & (0x1u8))) as u8 as u64)); crate::simd_ops::wu8(rbp.wrapping_add(0x47u64), ((rax as u8)) as u8); crate::simd_ops::wu64(rbp.wrapping_add(0x4fu64), rdi); rdx = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_add(0xcu64))) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rax = ((rdx.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rdx = ((rdx).wrapping_add(1)) as u64; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rbx))) as u64; let y_addr: u64 = rbp.wrapping_add(0x27u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rcx; crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); xmm1 = (crate::simd_ops::ru128(rbp.wrapping_add(0x47u64))) as u128; crate::simd_ops::wu128(rcx.wrapping_add(0x20u64), (xmm1) as u128); fl_l = (crate::simd_ops::lk_add_u32(rbx.wrapping_add(0x8u64), (1) as u32)) as u32 as u64; bb = 4; continue; }
            4 => { rax = 0x1; r11 = rsp.wrapping_add(0xb0); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x10u64))) as u64; rdi = (crate::simd_ops::ru64(r11.wrapping_add(0x18u64))) as u64; rsp = r11; rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14186a7a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142646078; const P1: u64 = 0x14020afa0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 21>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r9); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rbp = rsp.wrapping_sub(0x4f); rsp = ((rsp).wrapping_sub(0xa0u64)) as u64; rbx = rdx; xmm0 = 0u128; ymm0_hi = 0; rax = 0; fl_l = 0; fl_r = 0; let y_addr: u64 = rbp.wrapping_sub(0x1u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); crate::simd_ops::wu64(rbp.wrapping_add(0x1fu64), rax); let y_addr: u64 = rbp.wrapping_add(0x27u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); crate::simd_ops::wu32(rbp.wrapping_sub(0x1u64), (0x48u32) as u32); rdx = rbp.wrapping_sub(0x1); rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { rdx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x1u64))) as u64; rdx = ((rdx).wrapping_shr(32u32)) as u64; crate::simd_ops::wu32(rbp.wrapping_sub(0x31u64), ((rdx as u32)) as u32); rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x7u64))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_sub(0x2du64), ((rax as u32)) as u32); rcx = (crate::simd_ops::ru32(rbp.wrapping_add(0xfu64))) as u32 as u64; rcx = (((rcx as u32)).wrapping_sub((rax as u32))) as u32 as u64; rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x7u64))) as u64; rax = ((rax).wrapping_shr(32u32)) as u64; rax = (((rax as u32)).wrapping_sub((rdx as u32))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x77u64), ((rax as u32)) as u32); crate::simd_ops::wu32(rbp.wrapping_add(0x7bu64), ((rcx as u32)) as u32); rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x77u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x29u64), rax); r8 = (crate::simd_ops::ru64(rbp.wrapping_add(0xfu64))) as u64; r8 = ((r8).wrapping_shr(32u32)) as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x77u64), ((r8 as u32)) as u32); rcx = (crate::simd_ops::ru32(rbp.wrapping_add(0x17u64))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x7bu64), ((rcx as u32)) as u32); rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x77u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x21u64), rax); rdx = (crate::simd_ops::ru32(rbp.wrapping_add(0x1fu64))) as u32 as u64; rdx = (((rdx as u32)).wrapping_sub((rcx as u32))) as u32 as u64; rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x17u64))) as u64; rax = ((rax).wrapping_shr(32u32)) as u64; rax = (((rax as u32)).wrapping_sub((r8 as u32))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x77u64), ((rax as u32)) as u32); crate::simd_ops::wu32(rbp.wrapping_add(0x7bu64), ((rdx as u32)) as u32); rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x77u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x19u64), rax); rax = ((crate::simd_ops::ru8(rbp.wrapping_add(0x23u64))) as u32) as u32 as u64; rax = (rax & !0xffu64) | ((((((rax as u8)) & (0x1u8))) as u8 as u64)); crate::simd_ops::wu8(rbp.wrapping_sub(0x11u64), ((rax as u8)) as u8); crate::simd_ops::wu64(rbp.wrapping_sub(0x9u64), rbx); rbx = (crate::simd_ops::ru64(rbp.wrapping_add(0x7fu64))) as u64; rdx = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_add(0xcu64))) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rax = ((rdx.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rdx = ((rdx).wrapping_add(1)) as u64; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rbx))) as u64; let y_addr: u64 = rbp.wrapping_sub(0x31u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rcx; crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); xmm1 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x11u64))) as u128; crate::simd_ops::wu128(rcx.wrapping_add(0x20u64), (xmm1) as u128); fl_l = (crate::simd_ops::lk_add_u32(rbx.wrapping_add(0x8u64), (1) as u32)) as u32 as u64; bb = 4; continue; }
            4 => { rax = 0x1; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0xb0u64))) as u64; rsp = ((rsp).wrapping_add(0xa0u64)) as u64; rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14186a8d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142646078; const P1: u64 = 0x48; const P2: u64 = 0x20; const P3: u64 = 0x40;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0; let mut ymm1_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 15>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x70u64)) as u64; xmm0 = 0u128; ymm0_hi = 0; rbx = rcx; rax = rdx; rcx = 0; fl_l = 0; fl_r = 0; rdx = rsp.wrapping_add(0x20); let y_addr: u64 = rsp.wrapping_add(0x20u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rcx); rcx = rax; let y_addr: u64 = rsp.wrapping_add(0x48u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x48u32) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; rax = rbx; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu8(rbx.wrapping_add(P1), (0x0u8) as u8); rsp = ((rsp).wrapping_add(0x70u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            2 => { let y_addr: u64 = rsp.wrapping_add(0x20u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rsp.wrapping_add(0x40u64); xmm1 = crate::simd_ops::ru128(y_addr); ymm1_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rbx; crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); xmm0 = ((crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64) as u128; ymm0_hi = 0; let y_addr: u64 = rbx.wrapping_add(P2); crate::simd_ops::wu128(y_addr, xmm1); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm1_hi); crate::simd_ops::wu64(rbx.wrapping_add(P3), ((xmm0 as u64)) as u64); crate::simd_ops::wu8(rbx.wrapping_add(P1), (0x1u8) as u8); rsp = ((rsp).wrapping_add(0x70u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14186a960(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x18; const P3: u64 = 0x20; const P4: u64 = 0x28; const P5: u64 = 0x4; const P6: u64 = 0xc; const P7: u64 = 0x14; const P8: u64 = 0x1c; const P9: u64 = 0x24;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rcx, rax); crate::simd_ops::wu64(rcx.wrapping_add(P0), rax); crate::simd_ops::wu64(rcx.wrapping_add(P1), rax); crate::simd_ops::wu64(rcx.wrapping_add(P2), rax); crate::simd_ops::wu8(rcx.wrapping_add(P3), ((rax as u8)) as u8); crate::simd_ops::wu64(rcx.wrapping_add(P4), rax); rax = (crate::simd_ops::ru32(r8.wrapping_add(P5))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x8u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(r8.wrapping_add(P0))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0xcu64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rcx, rax); rax = (crate::simd_ops::ru32(r8.wrapping_add(P6))) as u32 as u64; rax = (((rax as u32)).wrapping_sub(crate::simd_ops::ru32(r8.wrapping_add(P5)))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x8u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(r8.wrapping_add(P1))) as u32 as u64; rax = (((rax as u32)).wrapping_sub(crate::simd_ops::ru32(r8.wrapping_add(P0)))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0xcu64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rcx.wrapping_add(P0), rax); rax = (crate::simd_ops::ru32(r8.wrapping_add(P7))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x8u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(r8.wrapping_add(P2))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0xcu64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rcx.wrapping_add(P1), rax); rax = (crate::simd_ops::ru32(r8.wrapping_add(P8))) as u32 as u64; rax = (((rax as u32)).wrapping_sub(crate::simd_ops::ru32(r8.wrapping_add(P7)))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x8u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(r8.wrapping_add(P3))) as u32 as u64; rax = (((rax as u32)).wrapping_sub(crate::simd_ops::ru32(r8.wrapping_add(P2)))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0xcu64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rcx.wrapping_add(P2), rax); rax = ((crate::simd_ops::ru8(r8.wrapping_add(P9))) as u32) as u32 as u64; fl_l = ((((rax as u8)) & (0x1u8))) as u8 as u64; fl_r = fl_l; rax = (rax & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); crate::simd_ops::wu64(rcx.wrapping_add(P4), rdx); crate::simd_ops::wu8(rcx.wrapping_add(P3), ((rax as u8)) as u8); rax = rcx; return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14186aa10(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14186a450; const P1: u64 = 0x142646230; const P2: u64 = 0x20;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x60u64)) as u64;
    rbx = rcx;
    crate::simd_ops::wu8(rsp.wrapping_add(0x50u64), (0x0u8) as u8);
    rcx = 0;
    fl_l = 0; fl_r = 0;
    r9 = rsp.wrapping_add(0x20);
    r8 = P0;
    rdx = 0;
    fl_l = 0; fl_r = 0;
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    let y_addr: u64 = rsp.wrapping_add(0x20u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16));
    xmm1 = (crate::simd_ops::ru128(rsp.wrapping_add(0x40u64))) as u128;
    let y_addr: u64 = rbx; crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi);
    crate::simd_ops::wu128(rbx.wrapping_add(P2), (xmm1) as u128);
    rax = rbx;
    rsp = ((rsp).wrapping_add(0x60u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14186aa60(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14186a450; const P1: u64 = 0x142646230;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x68u64)) as u64;
    r9 = rsp.wrapping_add(0x20);
    crate::simd_ops::wu8(rsp.wrapping_add(0x50u64), (0x0u8) as u8);
    r8 = P0;
    rdx = 0;
    fl_l = 0; fl_r = 0;
    rcx = 0;
    fl_l = 0; fl_r = 0;
    rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64;
    rsp = ((rsp).wrapping_add(0x68u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14186aa90(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14186a670; const P1: u64 = 0x8; const P2: u64 = 0x142646230;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rax = 0;
    fl_l = 0; fl_r = 0;
    r8 = P0;
    crate::simd_ops::wu64(rcx, rax);
    rbx = rcx;
    crate::simd_ops::wu64(rcx.wrapping_add(P1), rax);
    r9 = rcx;
    rcx = 0;
    fl_l = 0; fl_r = 0;
    rdx = 0;
    fl_l = 0; fl_r = 0;
    rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp);
    rax = rbx;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14186aad0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142646078;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 21>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rbp = rsp.wrapping_sub(0x57); rsp = ((rsp).wrapping_sub(0xa0u64)) as u64; xmm0 = 0u128; ymm0_hi = 0; rdi = rdx; rax = 0; fl_l = 0; fl_r = 0; let y_addr: u64 = rbp.wrapping_add(0x7u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); rbx = rcx; crate::simd_ops::wu32(rbp.wrapping_add(0x7u64), (0x48u32) as u32); crate::simd_ops::wu64(rbp.wrapping_add(0x27u64), rax); rdx = rbp.wrapping_add(0x7); let y_addr: u64 = rbp.wrapping_add(0x2fu64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru32(rbp.wrapping_add(0xfu64))) as u32 as u64; rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x7u64))) as u64; rax = (crate::simd_ops::ru64(rbp.wrapping_add(0xfu64))) as u64; rax = ((rax).wrapping_shr(32u32)) as u64; rdx = ((rdx).wrapping_shr(32u32)) as u64; rax = (((rax as u32)).wrapping_sub((rdx as u32))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_sub(0x29u64), ((rdx as u32)) as u32); rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x17u64))) as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x77u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x17u64))) as u32 as u64; rax = (((rax as u32)).wrapping_sub((rcx as u32))) as u32 as u64; rdx = ((rdx).wrapping_shr(32u32)) as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x7bu64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x77u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x21u64), rax); crate::simd_ops::wu32(rbp.wrapping_add(0x77u64), ((rdx as u32)) as u32); crate::simd_ops::wu32(rbp.wrapping_sub(0x25u64), ((rcx as u32)) as u32); rcx = (crate::simd_ops::ru32(rbp.wrapping_add(0x1fu64))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x7bu64), ((rcx as u32)) as u32); rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x77u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x19u64), rax); rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x1fu64))) as u64; rax = ((rax).wrapping_shr(32u32)) as u64; rax = (((rax as u32)).wrapping_sub((rdx as u32))) as u32 as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x1u64), rdi); crate::simd_ops::wu32(rbp.wrapping_add(0x77u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x27u64))) as u32 as u64; rax = (((rax as u32)).wrapping_sub((rcx as u32))) as u32 as u64; crate::simd_ops::wu8(rbx.wrapping_add(0x30u64), (0x1u8) as u8); crate::simd_ops::wu32(rbp.wrapping_add(0x7bu64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x77u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x11u64), rax); rax = ((crate::simd_ops::ru8(rbp.wrapping_add(0x2bu64))) as u32) as u32 as u64; let y_addr: u64 = rbp.wrapping_sub(0x29u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); fl_l = ((((rax as u8)) & (0x1u8))) as u8 as u64; fl_r = fl_l; rax = (rax & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); crate::simd_ops::wu8(rbp.wrapping_sub(0x9u64), ((rax as u8)) as u8); xmm1 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x9u64))) as u128; let y_addr: u64 = rbx; crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); crate::simd_ops::wu128(rbx.wrapping_add(0x20u64), (xmm1) as u128); bb = 3; continue; }
            2 => { crate::simd_ops::wu8(rbx.wrapping_add(0x30u64), (0x0u8) as u8); bb = 3; continue; }
            3 => { rax = rbx; r11 = rsp.wrapping_add(0xa0); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x10u64))) as u64; rdi = (crate::simd_ops::ru64(r11.wrapping_add(0x18u64))) as u64; rsp = r11; rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14186add0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1426472c0; const P1: u64 = 0x10; const P2: u64 = 0x1426472b0; const P3: u64 = 0x8; const P4: u64 = 0x142647290; const P5: u64 = 0x18; const P6: u64 = 0x14020b700;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    rcx = 0;
    fl_l = 0; fl_r = 0;
    rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp);
    rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64;
    rdx = 0x1;
    r8 = ((rdx as u32)) as u32 as u64;
    rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp);
    rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64;
    rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P3))) as u64;
    rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp);
    rcx = ((rbx.wrapping_add(P5)) as u64) as u64;
    crate::simd_ops::wu8(rsp.wrapping_add(0x30u64), (0x1u8) as u8);
    rdx = rsp.wrapping_add(0x30);
    rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14186ae80(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x1426472d8; const P2: u64 = 0x10; const P3: u64 = 0x1; const P4: u64 = 0x1426472b0; const P5: u64 = 0x1426472e8; const P6: u64 = 0x18; const P7: u64 = 0x1401f3ee0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp);
    rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64;
    rdx = 0;
    fl_l = 0; fl_r = 0;
    r8 = ((rdx.wrapping_add(P3)) as u32) as u32 as u64;
    rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp);
    rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp);
    rcx = ((rbx.wrapping_add(P6)) as u64) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(P7, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14186aec0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8;
    let (mut rax, mut fl_l) = (crate::simd_ops::get_rax_in(), 0u64);
    rax = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64;
    fl_l = (((rax as u32)).wrapping_sub(1)) as u32 as u64;
    rax = ((fl_l as u32)) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(P0), ((rax as u32)) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14186aee0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1426cce48; const P1: u64 = 0x8; const P2: u64 = 0x1426cce50; const P3: u64 = 0x1426cce58; const P4: u64 = 0x1426cce60;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x28u64)) as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(P0)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(P1))) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(P2)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 4; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rdx)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(P3)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 5; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(P1))) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(P4)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 5; continue; } bb = 4; continue; }
            4 => { crate::simd_ops::wu64(r8, rcx); rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rax = 0; fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
            5 => { crate::simd_ops::wu64(r8, (0x0u64) as u64); rax = 0x80004002; rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14186af40(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x14186b660; const P2: u64 = 0x20; const P3: u64 = 0x18;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x30u64)) as u64;
    rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rbx = rcx;
    rcx = rsp.wrapping_add(0x20);
    r8 = r9;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    fl_l = (crate::simd_ops::ru8(rsp.wrapping_add(0x28u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64;
    if (((fl_l as u8)) == ((fl_r as u8))) { rax = 0; fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    fl_l = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; fl_r = (0x0u64) as u64;
    if !((((fl_l as u64)) == ((fl_r as u64)))) { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64; rdx = rsp.wrapping_add(0x20); rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); }
    rax = 0;
    fl_l = 0; fl_r = 0;
    rsp = ((rsp).wrapping_add(0x30u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14186af90(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x20; const P1: u64 = 0x10;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp);
    rax = 0;
    fl_l = 0; fl_r = 0;
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14186afb0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x14186b660; const P2: u64 = 0x20; const P3: u64 = 0x8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x30u64)) as u64;
    rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rdi = rcx;
    rcx = rsp.wrapping_add(0x20);
    rbx = r9;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    fl_l = (crate::simd_ops::ru8(rsp.wrapping_add(0x28u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64;
    crate::simd_ops::wu32(rbx, (0x0u32) as u32);
    if (((fl_l as u8)) == ((fl_r as u8))) { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rax = 0; fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P2))) as u64;
    rdx = rsp.wrapping_add(0x20);
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp);
    fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64;
    if !(((((fl_l as u8)) & ((fl_r as u8))) == 0)) { crate::simd_ops::wu32(rbx, (0x1u32) as u32); }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64;
    rax = 0;
    fl_l = 0; fl_r = 0;
    rsp = ((rsp).wrapping_add(0x30u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14186b010(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142645770; const P1: u64 = 0x142646050; const P2: u64 = 0x140209f30; const P3: u64 = 0x14183acc2; const P4: u64 = 0x143080d18; const P5: u64 = 0x1423fdfc0; const P6: u64 = 0x143079330; const P7: u64 = 0x141461fb0; const P8: u64 = 0x1401df850; const P9: u64 = 0x1401e01e0; const P10: u64 = 0x14020b3a0; const P11: u64 = 0x14240ab10; const P12: u64 = 0x1425371a0; const P13: u64 = 0x142533200; const P14: u64 = 0x14186b660; const P15: u64 = 0x142645778; const P16: u64 = 0x14241c190; const P17: u64 = 0x1426472e0;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 87>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x14186b010, funcinfo_rva: 0x36fa2a4, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x2b8, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (r9) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rcx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r12) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r13) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r15) as u64); rbp = ((rsp.wrapping_sub(0x180u64)) as u64) as u64; rsp = ((rsp).wrapping_sub(0x280u64)) as u64; rdi = (r9) as u64; rsi = (rdx) as u64; rax = (0xfu32) as u32 as u64; crate::simd_ops::wu16(rbp.wrapping_sub(0x70u64), ((rax as u16)) as u16); rbx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rbp.wrapping_sub(0x68u64), (rbx) as u64); crate::simd_ops::wu32(rbp.wrapping_sub(0x60u64), (0x1u32) as u32); crate::simd_ops::wu32(rbp.wrapping_sub(0x5cu64), (0xffffffffu32) as u32); crate::simd_ops::wu32(rbp.wrapping_sub(0x58u64), (0x1u32) as u32); rax = (crate::simd_ops::ru64(rdx)) as u64; rdx = ((rbp.wrapping_sub(0x70u64)) as u64) as u64; rcx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14186b06d); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x28u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r15 = (crate::simd_ops::ru64(rbp.wrapping_add(0x1e0u64))) as u64; crate::simd_ops::wu32(r15, ((rbx as u32)) as u32); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 37; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rsi)) as u64; r8 = ((rbp.wrapping_sub(0x10u64)) as u64) as u64; rdx = ((rbp.wrapping_sub(0x70u64)) as u64) as u64; rcx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14186b090); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r14 = ((rax as u32)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((rax as u32)) as u32); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 36; continue; } bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x8u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14186b0a6); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P0)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r13 = (rax) as u64; fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 36; continue; } bb = 3; continue; }
            3 => { r9 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; r8 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rdx = (0xffffffffu32) as u32 as u64; rcx = (rax) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14186b0c6); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P1)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu32(rbp.wrapping_add(0x1c8u64), ((rax as u32)) as u32); xmm0 = 0u128; crate::simd_ops::wu128(rsp.wrapping_add(0x38u64), (xmm0) as u128); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (rbx) as u64); rsi = ((rbx as u32)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), (rbx) as u64); r12 = ((rbx as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 28; continue; } bb = 4; continue; }
            4 => { rdi = ((rax as u32)) as u32 as u64; bb = 5; continue; }
            5 => { r9 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; r8 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rdx = ((r12 as u32)) as u32 as u64; rcx = (r13) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14186b10c); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P1)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 26; continue; } bb = 6; continue; }
            6 => { r15 = ((rax.wrapping_add(0x1u64)) as u32) as u32 as u64; r14 = ((r15 as u32)) as u32 as u64; xmm0 = 0u128; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); rsi = (rbx) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rbx) as u64); crate::simd_ops::wu32(rsp.wrapping_add(0x2cu64), ((rbx as u32)) as u32); fl_l = ((r15 as u32)) as u32 as u64; fl_r = ((r15 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 9; continue; } bb = 7; continue; }
            7 => { rdx = ((r14 as u32)) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14186b13f); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rsi = (rax) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rax) as u64); crate::simd_ops::wu32(rsp.wrapping_add(0x2cu64), ((r15 as u32)) as u32); fl_l = ((r15 as u32)) as u32 as u64; fl_r = ((r15 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { r8 = ((r14 as u32)) as u32 as u64; rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rcx = (rax) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14186b15e); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 9; continue; }
            9 => { crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((r15 as u32)) as u32); r9 = ((r15.wrapping_add(0x1u64)) as u32) as u32 as u64; r8 = (rsi) as u64; rdx = ((r12 as u32)) as u32 as u64; rcx = (r13) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14186b175); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P1)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 17; continue; } bb = 10; continue; }
            10 => { rax = ((P4) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x40u64), (rax) as u64); crate::simd_ops::wu64(rbp.wrapping_sub(0x38u64), (0x2u64) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14186b196); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r14 = (rax) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x10u64), (rsi) as u64); xmm0 = (crate::simd_ops::ru128(rbp.wrapping_add(0x10u64))) as u128; crate::simd_ops::wu128(rbp.wrapping_add(0x20u64), (xmm0) as u128); crate::simd_ops::wu64(rbp.wrapping_sub(0x50u64), (0xcu64) as u64); rax = ((rbp.wrapping_add(0x20u64)) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x48u64), (rax) as u64); crate::simd_ops::wu64(rbp.wrapping_add(0x60u64), (rbx) as u64); crate::simd_ops::wu64(rbp.wrapping_add(0x68u64), (0x7ffu64) as u64); crate::simd_ops::wu64(rbp.wrapping_add(0x48u64), (r14) as u64); crate::simd_ops::wu64(rbp.wrapping_add(0x50u64), (rbx) as u64); crate::simd_ops::wu64(rbp.wrapping_add(0x58u64), (0x7ffu64) as u64); r15 = ((P6) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x40u64), (r15) as u64); crate::simd_ops::wu64(rbp.wrapping_add(0x70u64), (r14) as u64); xmm0 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x50u64))) as u128; crate::simd_ops::wu128(rbp.wrapping_add(0x30u64), (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x40u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x70u64), (xmm1) as u128); r9 = (rbx) as u64; r8 = ((rbp.wrapping_add(0x30u64)) as u64) as u64; rdx = ((rsp.wrapping_add(0x70u64)) as u64) as u64; rcx = ((rbp.wrapping_add(0x40u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14186b20c); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r10 = (crate::simd_ops::ru64(rbp.wrapping_add(0x68u64))) as u64; rax = (r10) as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x60u64))) as u64; rax = ((rax).wrapping_sub(rcx)) as u64; rdx = (rbx) as u64; fl_l = (r10) as u64 as u64; fl_r = (rcx) as u64 as u64; rdx = (if (((fl_l as u64)) > ((fl_r as u64))) { rax } else { rdx }) as u64; rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x50u64))) as u64; r9 = ((rcx.wrapping_add(rax)) as u64) as u64; fl_l = (rax) as u64 as u64; fl_r = (rdx) as u64 as u64; rdx = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { rdx }) as u64; r8 = (crate::simd_ops::ru64(rbp.wrapping_add(0x48u64))) as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x70u64))) as u64; fl_l = (r8) as u64 as u64; fl_r = (rcx) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 12; continue; } bb = 11; continue; }
            11 => { fl_l = ((rcx).wrapping_add(rdx)) as u64 as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x70u64), (rcx) as u64); r8 = ((rbp.wrapping_add(0x78u64)) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x48u64), (r8) as u64); crate::simd_ops::wu64(rbp.wrapping_add(0x58u64), (0x100u64) as u64); bb = 12; continue; }
            12 => { crate::simd_ops::wu64(rbp.wrapping_add(0x40u64), (r15) as u64); rax = (r10) as u64; rax = ((rax).wrapping_sub(r9)) as u64; rdx = (rbx) as u64; fl_l = (r10) as u64 as u64; fl_r = (r9) as u64 as u64; rdx = (if (((fl_l as u64)) > ((fl_r as u64))) { rax } else { rdx }) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x60u64), (r9) as u64); fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; rdx = (if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { rbx } else { rdx }) as u64; fl_l = (r8) as u64 as u64; fl_r = (rcx) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 14; continue; } bb = 13; continue; }
            13 => { rax = ((rdx.wrapping_add(rcx)) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x70u64), (rax) as u64); rax = ((rbp.wrapping_add(0x78u64)) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x48u64), (rax) as u64); crate::simd_ops::wu64(rbp.wrapping_add(0x58u64), (0x100u64) as u64); bb = 14; continue; }
            14 => { crate::simd_ops::wu64(rbp.wrapping_add(0x50u64), (rbx) as u64); crate::simd_ops::wu8(rcx, (0x0u8) as u8); crate::simd_ops::wu64(rbp.wrapping_sub(0x30u64), (r14) as u64); rax = (0xffffffffffffffffu64) as u64; bb = 15; continue; }
            15 => { rax = ((rax).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(r14.wrapping_add(rax))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 15; continue; } bb = 16; continue; }
            16 => { crate::simd_ops::wu64(rbp.wrapping_sub(0x28u64), (rax) as u64); rdx = ((rbp.wrapping_sub(0x30u64)) as u64) as u64; rcx = ((rbp.wrapping_sub(0x80u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14186b2c6); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x48u64))) as u32 as u64; rax = ((((rax as u32)) & (0xff000002u32))) as u32 as u64; rax = ((((rax as u32)) | (0x2u32))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x48u64), ((rax as u32)) as u32); crate::simd_ops::wu8(rsp.wrapping_add(0x4bu64), (0x2u8) as u8); crate::simd_ops::wu32(rsp.wrapping_add(0x4cu64), ((rbx as u32)) as u32); crate::simd_ops::wu8(rsp.wrapping_add(0x50u64), (0x0u8) as u8); xmm0 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x80u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x70u64), (xmm0) as u128); rdx = ((rsp.wrapping_add(0x70u64)) as u64) as u64; rcx = ((rsp.wrapping_add(0x48u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14186b2fe); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = ((rsp.wrapping_add(0x48u64)) as u64) as u64; rcx = ((rsp.wrapping_add(0x38u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14186b30e); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P10) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = ((rsp.wrapping_add(0x48u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14186b318); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P11) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 17; continue; }
            17 => { fl_l = (rsi) as u64 as u64; fl_r = (rsi) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 25; continue; } bb = 18; continue; }
            18 => { rsi = ((rsi).wrapping_sub(1)) as u64; rsi = (((rsi) & (0xfffffffffe000000u64))) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = (rax) as u64 as u64; fl_r = (crate::simd_ops::ru64(rsi.wrapping_add(0x100u64))) as u64 as u64; rdx = (rdx & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rax = (r9) as u64; rax = ((rax).wrapping_sub(rsi)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rcx = ((rcx).wrapping_add(0x108u64)) as u64; rcx = ((rcx).wrapping_add(rsi)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x4u64))) as u32 as u64; rcx = ((rcx).wrapping_sub(rax)) as u64; fl_l = ((rdx as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 23; continue; } bb = 19; continue; }
            19 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0xeu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 22; continue; } bb = 20; continue; }
            20 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(r9, (rax) as u64); crate::simd_ops::wu64(rcx.wrapping_add(0x20u64), (r9) as u64); fl_l = (crate::simd_ops::lk_add_u32(rcx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 25; continue; } bb = 21; continue; }
            21 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14186b382); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P12) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 25; continue; }
            22 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 24; continue; }
            23 => { r8 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 24; continue; }
            24 => { rdx = (rcx) as u64; rcx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14186b397); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P13) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 25; continue; }
            25 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (rbx) as u64); bb = 26; continue; }
            26 => { r12 = (((r12 as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((r12 as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 5; continue; } bb = 27; continue; }
            27 => { rsi = (crate::simd_ops::ru32(rsp.wrapping_add(0x40u64))) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rdi = (crate::simd_ops::ru64(rbp.wrapping_add(0x1d8u64))) as u64; r14 = (crate::simd_ops::ru32(rsp.wrapping_add(0x30u64))) as u32 as u64; r15 = (crate::simd_ops::ru64(rbp.wrapping_add(0x1e0u64))) as u64; bb = 28; continue; }
            28 => { r8 = (rdi) as u64; rdi = (crate::simd_ops::ru64(rbp.wrapping_add(0x1c0u64))) as u64; rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; rcx = ((rsp.wrapping_add(0x60u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14186b3e1); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P14) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = (crate::simd_ops::ru8(rsp.wrapping_add(0x68u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 31; continue; } bb = 29; continue; }
            29 => { r10 = (crate::simd_ops::ru64(rdi.wrapping_add(0x20u64))) as u64; rax = (crate::simd_ops::ru64(r10)) as u64; r9 = (crate::simd_ops::ru64(rax)) as u64; rax = ((rsi as u32)) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rdx = ((rbx.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; rdx = ((rdx).wrapping_sub(rbx)) as u64; rax = (0x2aaaaaaaaaaaaaabu64) as u64; let prod = (rax as i64 as i128).wrapping_mul((rdx) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rdx = ((((rdx) as i64).wrapping_shr(2u32) as u64)) as u64; rax = (rdx) as u64; rax = ((rax).wrapping_shr(63u32)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x20u64), (rdx) as u64); crate::simd_ops::wu64(rbp.wrapping_sub(0x18u64), (rbx) as u64); xmm0 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x20u64))) as u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x80u64), (xmm0) as u128); r8 = ((rbp.wrapping_sub(0x80u64)) as u64) as u64; rdx = ((rsp.wrapping_add(0x60u64)) as u64) as u64; rcx = (r10) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14186b43d); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((r9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 31; continue; } bb = 30; continue; }
            30 => { crate::simd_ops::wu32(r15, (0x1u32) as u32); bb = 31; continue; }
            31 => { rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x8u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14186b44f); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P15)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = ((rsi as u32)) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rsi = ((rbx.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rsi) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 34; continue; } bb = 32; continue; }
            32 => { rdi = (rbx) as u64; bb = 33; continue; }
            33 => { rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14186b46b); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P11) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdi = ((rdi).wrapping_add(0x18u64)) as u64; fl_l = (rdi) as u64 as u64; fl_r = (rsi) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 33; continue; } bb = 34; continue; }
            34 => { fl_l = (rbx) as u64 as u64; fl_r = (rbx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 36; continue; } bb = 35; continue; }
            35 => { rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14186b481); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P16) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 36; continue; }
            36 => { rcx = ((rbp.wrapping_sub(0x10u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14186b48b); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P17)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = ((r14 as u32)) as u32 as u64; bb = 37; continue; }
            37 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x2d0u64))) as u64; rsp = ((rsp).wrapping_add(0x280u64)) as u64; r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r13 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r12 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rsi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14186b4e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1426472d8; const P1: u64 = 0x8; const P2: u64 = 0x1; const P3: u64 = 0x1426472b0; const P4: u64 = 0x1426472e8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    rcx = (crate::simd_ops::ru64(rcx)) as u64;
    rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp);
    rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64;
    rdx = 0;
    fl_l = 0; fl_r = 0;
    r8 = ((rdx.wrapping_add(P2)) as u32) as u32 as u64;
    rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14186b520(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143080d40; const P1: u64 = 0x10; const P2: u64 = 0x14397d820; const P3: u64 = 0x14020b640; const P4: u64 = 0x20; const P5: u64 = 0x18; const P6: u64 = 0x1f0; const P7: u64 = 0x14186a310; const P8: u64 = 0x1401e5660; const P9: u64 = 0x1401e5470; const P10: u64 = 0x8; const P11: u64 = 0x142442de0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rax = P0; crate::simd_ops::wu64(rcx, rax); rax = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rcx); rbx = (crate::simd_ops::ru64(P2)) as u64; r8 = rsp.wrapping_add(0x30); rcx = rsp.wrapping_add(0x50); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; crate::simd_ops::wu8(rax.wrapping_add(P4), (0x1u8) as u8); rdx = (crate::simd_ops::ru64(rax.wrapping_add(P5))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdx); crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0x0u32) as u32); crate::simd_ops::lk_add_u32(rdx, (1) as u32); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rax); rdx = rsp.wrapping_add(0x58); rcx = ((rbx.wrapping_add(P6)) as u64) as u64; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); rcx = rsp.wrapping_add(0x20); rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 1; continue; }
            1 => { rax = 0xffffffff; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx, xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 6; continue; } bb = 2; continue; }
            2 => { fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 3; continue; }
            3 => { rdi = (crate::simd_ops::ru64(rbx.wrapping_add(P10))) as u64; fl_l = (((rdi) & (0xfffffffffffffffcu64))) as u64; fl_r = fl_l; rdi = ((fl_l as u64)) as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rdi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P10))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = rdi; rax = crate::simd_ops::call_stk(P11, rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            5 => { rcx = rbx; rax = crate::simd_ops::call_stk(P11, rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            6 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14186b620(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1426472c0; const P1: u64 = 0x8; const P2: u64 = 0x1426472b0; const P3: u64 = 0x142647290;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    rcx = 0;
    fl_l = 0; fl_r = 0;
    rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp);
    rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64;
    rdx = 0x1;
    r8 = ((rdx as u32)) as u32 as u64;
    rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp);
    rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64;
    rcx = (crate::simd_ops::ru64(rbx)) as u64;
    rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp);
    rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64));
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14186b660(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142646188; const P1: u64 = 0x142646120; const P2: u64 = 0x1434268d0; const P3: u64 = 0x8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut xmm4: u128 = 0; let mut xmm6: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rdi = rdx; crate::simd_ops::wu32(rsp.wrapping_add(0x60u64), ((r8 as u32)) as u32); r8 = ((r8).wrapping_shr(32u32)) as u64; rdx = rsp.wrapping_add(0x60); rbx = rcx; crate::simd_ops::wu32(rsp.wrapping_add(0x64u64), ((r8 as u32)) as u32); rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 5; continue; } bb = 1; continue; }
            1 => { rdx = rsp.wrapping_add(0x20); rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 5; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x28u64))) as u32 as u64; rax = (((rax as u32)).wrapping_sub(crate::simd_ops::ru32(rsp.wrapping_add(0x20u64)))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 5; continue; } bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru32(rsp.wrapping_add(0x2cu64))) as u32 as u64; rcx = (((rcx as u32)).wrapping_sub(crate::simd_ops::ru32(rsp.wrapping_add(0x24u64)))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { xmm0 = 0u128; xmm0 = (xmm0 & !0xffff_ffffu128) | (((((rcx as u32)) as i32) as f32).to_bits() as u128); xmm2 = 0u128; xmm2 = (xmm2 & !0xffff_ffffu128) | ((((crate::simd_ops::ru32(rsp.wrapping_add(0x64u64))) as i32) as f32).to_bits() as u128); xmm3 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) / f32::from_bits(xmm0 as u32)).to_bits() as u128); crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm6) as u128); xmm6 = 0u128; xmm4 = (xmm3 & !0xffff_ffffu128) | ((if f32::from_bits(xmm3 as u32) > f32::from_bits(xmm6 as u32) { f32::from_bits(xmm3 as u32) } else { f32::from_bits(xmm6 as u32) }).to_bits() as u128); xmm4 = (xmm4 & !0xffff_ffffu128) | ((if f32::from_bits(xmm4 as u32) < f32::from_bits(crate::simd_ops::ru32(P2)) { f32::from_bits(xmm4 as u32) } else { f32::from_bits(crate::simd_ops::ru32(P2)) }).to_bits() as u128); xmm0 = 0u128; xmm0 = (xmm0 & !0xffff_ffffu128) | (((((rax as u32)) as i32) as f32).to_bits() as u128); xmm1 = 0u128; xmm1 = (xmm1 & !0xffff_ffffu128) | ((((crate::simd_ops::ru32(rsp.wrapping_add(0x60u64))) as i32) as f32).to_bits() as u128); xmm2 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) / f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm3 = (xmm2 & !0xffff_ffffu128) | ((if f32::from_bits(xmm2 as u32) > f32::from_bits(xmm6 as u32) { f32::from_bits(xmm2 as u32) } else { f32::from_bits(xmm6 as u32) }).to_bits() as u128); xmm3 = (xmm3 & !0xffff_ffffu128) | ((if f32::from_bits(xmm3 as u32) < f32::from_bits(crate::simd_ops::ru32(P2)) { f32::from_bits(xmm3 as u32) } else { f32::from_bits(crate::simd_ops::ru32(P2)) }).to_bits() as u128); xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; crate::simd_ops::wu32(rsp.wrapping_add(0x60u64), ((xmm3 as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x64u64), ((xmm4 as u32)) as u32); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; crate::simd_ops::wu64(rbx, rax); rax = rbx; crate::simd_ops::wu8(rbx.wrapping_add(P3), (0x1u8) as u8); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            5 => { crate::simd_ops::wu8(rbx.wrapping_add(P3), (0x0u8) as u8); rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14186b770(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143080d40; const P1: u64 = 0x14397d820; const P2: u64 = 0x14020b580; const P3: u64 = 0x14186a310; const P4: u64 = 0x14020b1f0; const P5: u64 = 0x142442de0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rdi = rcx; rsi = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rsp.wrapping_add(0x60u64), ((rsi as u32)) as u32); rax = P0; crate::simd_ops::wu64(rcx, rax); crate::simd_ops::wu32(rcx.wrapping_add(0x8u64), ((rsi as u32)) as u32); crate::simd_ops::wu64(rcx.wrapping_add(0x10u64), r8); crate::simd_ops::wu8(rcx.wrapping_add(0x18u64), ((rsi as u8)) as u8); crate::simd_ops::wu64(rcx.wrapping_add(0x20u64), rdx); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), r8); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rcx); rbx = (crate::simd_ops::ru64(P1)) as u64; r8 = rsp.wrapping_add(0x30); rcx = rsp.wrapping_add(0x68); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; crate::simd_ops::wu8(rax.wrapping_add(0x20u64), (0x1u8) as u8); rdx = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdx); crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rsi as u32)) as u32); crate::simd_ops::lk_add_u32(rdx.wrapping_add(0x4u64), (1) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x60u64), (0x2u32) as u32); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), rax); rdx = rsp.wrapping_add(0x70); rcx = rbx.wrapping_add(0x1f0); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = rsp.wrapping_add(0x20); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu8(rdi.wrapping_add(0x18u64), ((rax as u8)) as u8); rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 1; continue; }
            1 => { rax = 0xffffffff; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rsi.wrapping_add(0x4u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 6; continue; } bb = 2; continue; }
            2 => { fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 3; continue; }
            3 => { rbx = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; fl_l = (((rbx) & (0xfffffffffffffffcu64))) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            5 => { rcx = rsi; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            6 => { rax = rdi; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14186b860(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14020b9a0; const P1: u64 = 0x142442de0;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x14186b860, funcinfo_rva: 0x36fa3c0, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x58, conts: &[] };
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r11 = (rsp) as u64; crate::simd_ops::wu64(r11.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(r11.wrapping_add(0x10u64), (rdx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0x50u64)) as u64; rdi = (rcx) as u64; rbx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(r11.wrapping_sub(0x10u64), (rbx) as u64); rax = ((r11.wrapping_add(0x30u64)) as u64) as u64; crate::simd_ops::wu64(r11.wrapping_sub(0x28u64), (rax) as u64); rax = ((r11.wrapping_add(0x28u64)) as u64) as u64; crate::simd_ops::wu64(r11.wrapping_sub(0x30u64), (rax) as u64); crate::simd_ops::wu64(r11.wrapping_sub(0x38u64), (r9) as u64); r9 = (r8) as u64; r8 = ((r11.wrapping_sub(0x10u64)) as u64) as u64; rcx = ((r11.wrapping_add(0x10u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14186b898); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = (crate::simd_ops::ru64(rax)) as u64; crate::simd_ops::wu64(rax, (rbx) as u64); crate::simd_ops::wu64(rdi, (rcx) as u64); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rbx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14186b8b8); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14186b8bd); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 2; continue; }
            2 => { rax = (rdi) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsp = ((rsp).wrapping_add(0x50u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14186b8e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x48; const P1: u64 = 0x50; const P2: u64 = 0x8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rsi = rdx;
    rax = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64;
    rdi = ((rbx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64;
    fl_l = rbx; fl_r = rdi;
    if (((fl_l as u64)) == ((fl_r as u64))) { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    loop { rcx = (crate::simd_ops::ru64(rbx)) as u64; rdx = rsi; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rbx = ((rbx).wrapping_add(0x8u64)) as u64; fl_l = rbx; fl_r = rdi; if !((((fl_l as u64)) != ((fl_r as u64)))) { break; } }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14186b930(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x143080da0; const P1: u64 = 0x14397d820; const P2: u64 = 0x8; const P3: u64 = 0x10; const P4: u64 = 0x18; const P5: u64 = 0x20; const P6: u64 = 0x28; const P7: u64 = 0x30; const P8: u64 = 0x38; const P9: u64 = 0x40; const P10: u64 = 0x48; const P11: u64 = 0x50;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = P0;
    crate::simd_ops::wu64(P1, rcx);
    crate::simd_ops::wu64(rcx, rax);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rcx.wrapping_add(P2), rax);
    crate::simd_ops::wu64(rcx.wrapping_add(P3), rax);
    crate::simd_ops::wu64(rcx.wrapping_add(P4), rax);
    crate::simd_ops::wu64(rcx.wrapping_add(P5), rax);
    crate::simd_ops::wu64(rcx.wrapping_add(P6), rax);
    crate::simd_ops::wu64(rcx.wrapping_add(P7), rax);
    crate::simd_ops::wu64(rcx.wrapping_add(P8), rax);
    crate::simd_ops::wu64(rcx.wrapping_add(P9), rax);
    crate::simd_ops::wu64(rcx.wrapping_add(P10), rax);
    crate::simd_ops::wu64(rcx.wrapping_add(P11), rax);
    rax = rcx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14186b980(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143080da0; const P1: u64 = 0x14397d820; const P2: u64 = 0x1401de830; const P3: u64 = 0x38; const P4: u64 = 0x1401e3450; const P5: u64 = 0x28; const P6: u64 = 0x18; const P7: u64 = 0x8;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdi = rcx;
    rax = P0;
    crate::simd_ops::wu64(rcx, rax);
    crate::simd_ops::wu64(P1, (0x0u64) as u64);
    rcx = ((rcx).wrapping_add(0x48u64)) as u64;
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    rcx = ((rdi.wrapping_add(P3)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp);
    rcx = ((rdi.wrapping_add(P5)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    rcx = ((rdi.wrapping_add(P6)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp);
    rcx = ((rdi.wrapping_add(P7)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14186baa0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14020bc10; const P1: u64 = 0x14020baf0;
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x30u64)) as u64;
    rbx = rcx;
    rcx = rsp.wrapping_add(0x40);
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rcx = rsp.wrapping_add(0x40);
    rdx = (crate::simd_ops::ru64(rax)) as u64;
    crate::simd_ops::wu64(rax, (0x0u64) as u64);
    crate::simd_ops::wu64(rbx, rdx);
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rax = rbx;
    rsp = ((rsp).wrapping_add(0x30u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14186bae0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x140000000;
    let (mut rax, mut fl_l, mut fl_r, mut j_tgt) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = (((crate::simd_ops::ru32(rdx)) as i32 as i64) as u64) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x13u32) as u32 as u64; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 22; continue; } bb = 1; continue; }
            1 => { r9 = ((P0) as u64) as u64; r8 = (crate::simd_ops::ru32(r9.wrapping_add(rax.wrapping_mul(4)).wrapping_add(0x186bba4u64))) as u32 as u64; fl_l = ((r8).wrapping_add(r9)) as u64 as u64; fl_r = fl_l; r8 = ((fl_l as u64)) as u64; j_tgt = (r8) as u64; bb = 0xffff_fffe; continue; }
            2 => { rax = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0xa8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            3 => { rax = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0xa0u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            4 => { rax = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x98u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            5 => { rax = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x38u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            6 => { rax = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x30u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            7 => { rax = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x28u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            8 => { rax = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x90u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            9 => { rax = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x88u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            10 => { rax = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x80u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            11 => { rax = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x78u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            12 => { rax = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x70u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            13 => { rax = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x68u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            14 => { rax = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x60u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            15 => { rax = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x58u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            16 => { rax = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x50u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            17 => { rax = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x48u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            18 => { rax = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x40u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            19 => { rax = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x20u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            20 => { rax = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            21 => { rax = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            22 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; return rax; }
            0xffff_fffe => { match j_tgt.wrapping_sub(0x14186bae0u64) { 0xc => { bb = 1; continue; } 0x21 => { bb = 2; continue; } 0x2b => { bb = 3; continue; } 0x35 => { bb = 4; continue; } 0x3f => { bb = 5; continue; } 0x46 => { bb = 6; continue; } 0x4d => { bb = 7; continue; } 0x54 => { bb = 8; continue; } 0x5e => { bb = 9; continue; } 0x68 => { bb = 10; continue; } 0x72 => { bb = 11; continue; } 0x79 => { bb = 12; continue; } 0x80 => { bb = 13; continue; } 0x87 => { bb = 14; continue; } 0x8e => { bb = 15; continue; } 0x95 => { bb = 16; continue; } 0x9c => { bb = 17; continue; } 0xa3 => { bb = 18; continue; } 0xaa => { bb = 19; continue; } 0xb1 => { bb = 20; continue; } 0xb8 => { bb = 21; continue; } 0xbf => { bb = 22; continue; } _ => { crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(j_tgt, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); } } }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14186be30(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x140217440;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), r8); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = ((rsp).wrapping_sub(0x38u64)) as u64; xmm1 = (crate::simd_ops::ru128(rcx)) as u128; rcx = (crate::simd_ops::ru64(rcx)) as u64; r13 = r9; rax = (((xmm1 >> 64) as u64)) as u64; rbx = (((xmm1 >> 64) as u64)) as u64; r12 = ((rdx as u32)) as u32 as u64; rdi = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rdi); fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 8; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), r14); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r15); bb = 2; continue; }
            2 => { r15 = (crate::simd_ops::ru64(rbx)) as u64; rbp = r15; rbp = ((rbp).wrapping_shr(32u32)) as u64; fl_l = ((r15 as u32)) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 6; continue; } bb = 3; continue; }
            3 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; bb = 4; continue; }
            4 => { rax = ((r15 as u32)) as u32 as u64; r14 = (((rax) as i64).wrapping_mul((0x1c0u64) as i64) as u64) as u64; rax = ((r12 as u32)) as u32 as u64; rsi = (((rax) as i64).wrapping_mul((0x1c0u64) as i64) as u64) as u64; r14 = ((r14).wrapping_add(rdi)) as u64; rsi = ((rsi).wrapping_add(r13)) as u64; r12 = (((r12 as u32)).wrapping_add(1)) as u32 as u64; rax = (crate::simd_ops::ru64(r14)) as u64; crate::simd_ops::wu64(rsi, rax); rax = (crate::simd_ops::ru64(r14.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rsi.wrapping_add(0x8u64), rax); rax = ((crate::simd_ops::ru8(r14.wrapping_add(0x10u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rsi.wrapping_add(0x10u64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(r14.wrapping_add(0x11u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rsi.wrapping_add(0x11u64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(r14.wrapping_add(0x12u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rsi.wrapping_add(0x12u64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(r14.wrapping_add(0x13u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rsi.wrapping_add(0x13u64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(r14.wrapping_add(0x14u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rsi.wrapping_add(0x14u64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(r14.wrapping_add(0x15u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rsi.wrapping_add(0x15u64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(r14.wrapping_add(0x16u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rsi.wrapping_add(0x16u64), ((rax as u8)) as u8); rax = (crate::simd_ops::ru32(r14.wrapping_add(0x18u64))) as u32 as u64; crate::simd_ops::wu32(rsi.wrapping_add(0x18u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(r14.wrapping_add(0x1cu64))) as u32 as u64; crate::simd_ops::wu32(rsi.wrapping_add(0x1cu64), ((rax as u32)) as u32); rax = ((crate::simd_ops::ru8(r14.wrapping_add(0x20u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rsi.wrapping_add(0x20u64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(r14.wrapping_add(0x21u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rsi.wrapping_add(0x21u64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(r14.wrapping_add(0x22u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rsi.wrapping_add(0x22u64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(r14.wrapping_add(0x23u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rsi.wrapping_add(0x23u64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(r14.wrapping_add(0x24u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rsi.wrapping_add(0x24u64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(r14.wrapping_add(0x25u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rsi.wrapping_add(0x25u64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(r14.wrapping_add(0x26u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rsi.wrapping_add(0x26u64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(r14.wrapping_add(0x27u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rsi.wrapping_add(0x27u64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(r14.wrapping_add(0x28u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rsi.wrapping_add(0x28u64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(r14.wrapping_add(0x29u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rsi.wrapping_add(0x29u64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(r14.wrapping_add(0x2au64))) as u32) as u32 as u64; crate::simd_ops::wu8(rsi.wrapping_add(0x2au64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(r14.wrapping_add(0x2bu64))) as u32) as u32 as u64; crate::simd_ops::wu8(rsi.wrapping_add(0x2bu64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(r14.wrapping_add(0x2cu64))) as u32) as u32 as u64; crate::simd_ops::wu8(rsi.wrapping_add(0x2cu64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(r14.wrapping_add(0x2du64))) as u32) as u32 as u64; crate::simd_ops::wu8(rsi.wrapping_add(0x2du64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(r14.wrapping_add(0x2eu64))) as u32) as u32 as u64; crate::simd_ops::wu8(rsi.wrapping_add(0x2eu64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(r14.wrapping_add(0x2fu64))) as u32) as u32 as u64; crate::simd_ops::wu8(rsi.wrapping_add(0x2fu64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(r14.wrapping_add(0x30u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rsi.wrapping_add(0x30u64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(r14.wrapping_add(0x31u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rsi.wrapping_add(0x31u64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(r14.wrapping_add(0x32u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rsi.wrapping_add(0x32u64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(r14.wrapping_add(0x33u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rsi.wrapping_add(0x33u64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(r14.wrapping_add(0x34u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rsi.wrapping_add(0x34u64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(r14.wrapping_add(0x35u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rsi.wrapping_add(0x35u64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(r14.wrapping_add(0x36u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rsi.wrapping_add(0x36u64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(r14.wrapping_add(0x37u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rsi.wrapping_add(0x37u64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(r14.wrapping_add(0x38u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rsi.wrapping_add(0x38u64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(r14.wrapping_add(0x39u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rsi.wrapping_add(0x39u64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(r14.wrapping_add(0x3au64))) as u32) as u32 as u64; rdx = r14.wrapping_add(0xb0); crate::simd_ops::wu8(rsi.wrapping_add(0x3au64), ((rax as u8)) as u8); rcx = rsi.wrapping_add(0xb0); rax = ((crate::simd_ops::ru8(r14.wrapping_add(0x3bu64))) as u32) as u32 as u64; crate::simd_ops::wu8(rsi.wrapping_add(0x3bu64), ((rax as u8)) as u8); xmm0 = (crate::simd_ops::ru128(r14.wrapping_add(0x40u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rsi.wrapping_add(0x40u64), (xmm0) as u128); let y_addr: u64 = r14.wrapping_add(0x50u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rsi.wrapping_add(0x50u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); xmm0 = (crate::simd_ops::ru128(r14.wrapping_add(0x70u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rsi.wrapping_add(0x70u64), (xmm0) as u128); rax = (crate::simd_ops::ru32(r14.wrapping_add(0x80u64))) as u32 as u64; crate::simd_ops::wu32(rsi.wrapping_add(0x80u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(r14.wrapping_add(0x84u64))) as u64; crate::simd_ops::wu64(rsi.wrapping_add(0x84u64), rax); rax = ((crate::simd_ops::ru8(r14.wrapping_add(0x8cu64))) as u32) as u32 as u64; crate::simd_ops::wu8(rsi.wrapping_add(0x8cu64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(r14.wrapping_add(0x8du64))) as u32) as u32 as u64; crate::simd_ops::wu8(rsi.wrapping_add(0x8du64), ((rax as u8)) as u8); rax = (crate::simd_ops::ru64(r14.wrapping_add(0x90u64))) as u64; crate::simd_ops::wu64(rsi.wrapping_add(0x90u64), rax); rax = (crate::simd_ops::ru32(r14.wrapping_add(0x98u64))) as u32 as u64; crate::simd_ops::wu32(rsi.wrapping_add(0x98u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(r14.wrapping_add(0xa0u64))) as u64; crate::simd_ops::wu64(rsi.wrapping_add(0xa0u64), rax); rax = (crate::simd_ops::ru32(r14.wrapping_add(0xa8u64))) as u32 as u64; crate::simd_ops::wu32(rsi.wrapping_add(0xa8u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(r14.wrapping_add(0xacu64))) as u32 as u64; crate::simd_ops::wu32(rsi.wrapping_add(0xacu64), ((rax as u32)) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = ((crate::simd_ops::ru8(r14.wrapping_add(0x128u64))) as u32) as u32 as u64; r15 = (((r15 as u32)).wrapping_add(1)) as u32 as u64; crate::simd_ops::wu8(rsi.wrapping_add(0x128u64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(r14.wrapping_add(0x129u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rsi.wrapping_add(0x129u64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(r14.wrapping_add(0x12au64))) as u32) as u32 as u64; crate::simd_ops::wu8(rsi.wrapping_add(0x12au64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(r14.wrapping_add(0x12bu64))) as u32) as u32 as u64; crate::simd_ops::wu8(rsi.wrapping_add(0x12bu64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(r14.wrapping_add(0x12cu64))) as u32) as u32 as u64; crate::simd_ops::wu8(rsi.wrapping_add(0x12cu64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(r14.wrapping_add(0x12du64))) as u32) as u32 as u64; crate::simd_ops::wu8(rsi.wrapping_add(0x12du64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(r14.wrapping_add(0x12eu64))) as u32) as u32 as u64; crate::simd_ops::wu8(rsi.wrapping_add(0x12eu64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(r14.wrapping_add(0x12fu64))) as u32) as u32 as u64; crate::simd_ops::wu8(rsi.wrapping_add(0x12fu64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(r14.wrapping_add(0x130u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rsi.wrapping_add(0x130u64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(r14.wrapping_add(0x131u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rsi.wrapping_add(0x131u64), ((rax as u8)) as u8); rax = (crate::simd_ops::ru32(r14.wrapping_add(0x134u64))) as u32 as u64; crate::simd_ops::wu32(rsi.wrapping_add(0x134u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(r14.wrapping_add(0x138u64))) as u32 as u64; crate::simd_ops::wu32(rsi.wrapping_add(0x138u64), ((rax as u32)) as u32); rax = ((crate::simd_ops::ru8(r14.wrapping_add(0x13cu64))) as u32) as u32 as u64; crate::simd_ops::wu8(rsi.wrapping_add(0x13cu64), ((rax as u8)) as u8); let y_addr: u64 = r14.wrapping_add(0x140u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rsi.wrapping_add(0x140u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); xmm1 = (crate::simd_ops::ru128(r14.wrapping_add(0x160u64))) as u128; crate::simd_ops::wu128(rsi.wrapping_add(0x160u64), (xmm1) as u128); rax = (crate::simd_ops::ru32(r14.wrapping_add(0x170u64))) as u32 as u64; crate::simd_ops::wu32(rsi.wrapping_add(0x170u64), ((rax as u32)) as u32); let y_addr: u64 = r14.wrapping_add(0x174u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rsi.wrapping_add(0x174u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); xmm1 = (crate::simd_ops::ru128(r14.wrapping_add(0x194u64))) as u128; crate::simd_ops::wu128(rsi.wrapping_add(0x194u64), (xmm1) as u128); rax = (crate::simd_ops::ru32(r14.wrapping_add(0x1a4u64))) as u32 as u64; crate::simd_ops::wu32(rsi.wrapping_add(0x1a4u64), ((rax as u32)) as u32); rax = ((crate::simd_ops::ru8(r14.wrapping_add(0x1a8u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rsi.wrapping_add(0x1a8u64), ((rax as u8)) as u8); rax = (crate::simd_ops::ru64(r14.wrapping_add(0x1acu64))) as u64; crate::simd_ops::wu64(rsi.wrapping_add(0x1acu64), rax); rax = (crate::simd_ops::ru32(r14.wrapping_add(0x1b4u64))) as u32 as u64; crate::simd_ops::wu32(rsi.wrapping_add(0x1b4u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(r14.wrapping_add(0x1b8u64))) as u32 as u64; crate::simd_ops::wu32(rsi.wrapping_add(0x1b8u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(r14.wrapping_add(0x1bcu64))) as u32 as u64; crate::simd_ops::wu32(rsi.wrapping_add(0x1bcu64), ((rax as u32)) as u32); fl_l = ((r15 as u32)) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 4; continue; } bb = 5; continue; }
            5 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; bb = 6; continue; }
            6 => { rbx = ((rbx).wrapping_add(0x8u64)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 7; continue; }
            7 => { r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; bb = 8; continue; }
            8 => { rsp = ((rsp).wrapping_add(0x38u64)) as u64; r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14186c260(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xd0; const P1: u64 = 0x14240ab10;
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
    loop { rcx = rbx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rbx = ((rbx).wrapping_add(0x1c0u64)) as u64; fl_l = rdi; fl_r = (0x1u64) as u64; rdi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if !((((fl_l as u64)) != ((fl_r as u64)))) { break; } }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14186c3a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xc; const P2: u64 = 0x10;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 1>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); xmm1 = (crate::simd_ops::ru128(rcx)) as u128; rcx = (crate::simd_ops::ru64(rcx)) as u64; rdi = r9; rax = (((xmm1 >> 64) as u64)) as u64; rsi = r8; rbx = ((rdx as u32)) as u32 as u64; rcx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; rax = (((xmm1 >> 64) as u64)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 7; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); rbp = 0; fl_l = 0; fl_r = 0; bb = 2; continue; }
            2 => { rdx = (crate::simd_ops::ru64(rax)) as u64; r11 = rdx; r11 = ((r11).wrapping_shr(32u32)) as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((r11 as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 5; continue; } bb = 3; continue; }
            3 => { bb = 4; continue; }
            4 => { r8 = ((rdx as u32)) as u32 as u64; rdx = (((rdx as u32)).wrapping_add(1)) as u32 as u64; r9 = ((r8.wrapping_add(r8.wrapping_mul(2))) as u64) as u64; r8 = ((rbx as u32)) as u32 as u64; r10 = ((rsi.wrapping_add(r9.wrapping_mul(8))) as u64) as u64; rbx = (((rbx as u32)).wrapping_add(1)) as u32 as u64; r9 = ((r8.wrapping_add(r8.wrapping_mul(2))) as u64) as u64; crate::simd_ops::wu64(rdi.wrapping_add(r9.wrapping_mul(8)).wrapping_add(P0), rbp); crate::simd_ops::wu64(rdi.wrapping_add(r9.wrapping_mul(8)), rbp); r8 = (crate::simd_ops::ru64(r10)) as u64; crate::simd_ops::wu64(rdi.wrapping_add(r9.wrapping_mul(8)), r8); r8 = (crate::simd_ops::ru32(r10.wrapping_add(P0))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(r9.wrapping_mul(8)).wrapping_add(P0), ((r8 as u32)) as u32); r8 = (crate::simd_ops::ru32(r10.wrapping_add(P1))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(r9.wrapping_mul(8)).wrapping_add(P1), ((r8 as u32)) as u32); crate::simd_ops::wu64(r10, rbp); crate::simd_ops::wu64(r10.wrapping_add(P0), rbp); r8 = (crate::simd_ops::ru64(r10.wrapping_add(P2))) as u64; crate::simd_ops::wu64(rdi.wrapping_add(r9.wrapping_mul(8)).wrapping_add(P2), r8); fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((r11 as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 4; continue; } bb = 5; continue; }
            5 => { rax = ((rax).wrapping_add(0x8u64)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 6; continue; }
            6 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x10u64))) as u64; bb = 7; continue; }
            7 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x18u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14186c460(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xc; const P2: u64 = 0x10;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    r9 = 0;
    fl_l = 0; fl_r = 0;
    rax = ((r8 as u32)) as u32 as u64;
    r8 = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64;
    crate::simd_ops::wu64(rdx.wrapping_add(r8.wrapping_mul(8)), r9);
    crate::simd_ops::wu64(rdx.wrapping_add(r8.wrapping_mul(8)).wrapping_add(P0), r9);
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    crate::simd_ops::wu64(rdx.wrapping_add(r8.wrapping_mul(8)), rax);
    rax = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64;
    crate::simd_ops::wu32(rdx.wrapping_add(r8.wrapping_mul(8)).wrapping_add(P0), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64;
    crate::simd_ops::wu32(rdx.wrapping_add(r8.wrapping_mul(8)).wrapping_add(P1), ((rax as u32)) as u32);
    crate::simd_ops::wu64(rcx, r9);
    crate::simd_ops::wu64(rcx.wrapping_add(P0), r9);
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64;
    crate::simd_ops::wu64(rdx.wrapping_add(r8.wrapping_mul(8)).wrapping_add(P2), rax);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14186c560(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14240a730;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0; let mut ymm1_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), r8); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; r13 = r9; r12 = ((rdx as u32)) as u32 as u64; rbx = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; r10 = (crate::simd_ops::ru64(rcx)) as u64; rdi = ((rbx.wrapping_add(r10.wrapping_mul(8))) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), rdi); fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 7; continue; } bb = 1; continue; }
            1 => { bb = 2; continue; }
            2 => { rbp = (crate::simd_ops::ru64(rbx)) as u64; r15 = rbp; r15 = ((r15).wrapping_shr(32u32)) as u64; fl_l = ((rbp as u32)) as u32 as u64; fl_r = ((r15 as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 6; continue; } bb = 3; continue; }
            3 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; bb = 4; continue; }
            4 => { rax = ((rbp as u32)) as u32 as u64; r14 = (((rax) as i64).wrapping_mul((0x160u64) as i64) as u64) as u64; r14 = ((r14).wrapping_add(rdi)) as u64; rax = ((r12 as u32)) as u32 as u64; rsi = (((rax) as i64).wrapping_mul((0x160u64) as i64) as u64) as u64; rsi = ((rsi).wrapping_add(r13)) as u64; r12 = (((r12 as u32)).wrapping_add(1)) as u32 as u64; rax = (crate::simd_ops::ru64(r14)) as u64; crate::simd_ops::wu64(rsi, rax); rax = (crate::simd_ops::ru32(r14.wrapping_add(0x8u64))) as u32 as u64; crate::simd_ops::wu32(rsi.wrapping_add(0x8u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(r14.wrapping_add(0xcu64))) as u32 as u64; crate::simd_ops::wu32(rsi.wrapping_add(0xcu64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(r14.wrapping_add(0x10u64))) as u64; crate::simd_ops::wu64(rsi.wrapping_add(0x10u64), rax); rax = ((crate::simd_ops::ru8(r14.wrapping_add(0x18u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rsi.wrapping_add(0x18u64), ((rax as u8)) as u8); rax = (crate::simd_ops::ru32(r14.wrapping_add(0x1cu64))) as u32 as u64; crate::simd_ops::wu32(rsi.wrapping_add(0x1cu64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(r14.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(rsi.wrapping_add(0x20u64), rax); rax = (crate::simd_ops::ru32(r14.wrapping_add(0x28u64))) as u32 as u64; crate::simd_ops::wu32(rsi.wrapping_add(0x28u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(r14.wrapping_add(0x2cu64))) as u32 as u64; crate::simd_ops::wu32(rsi.wrapping_add(0x2cu64), ((rax as u32)) as u32); rax = ((crate::simd_ops::ru8(r14.wrapping_add(0x30u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rsi.wrapping_add(0x30u64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(r14.wrapping_add(0x31u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rsi.wrapping_add(0x31u64), ((rax as u8)) as u8); rcx = rsi.wrapping_add(0x34); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rcx); crate::simd_ops::lk_and_u32(rcx, (0xff000010u32) as u32); crate::simd_ops::lk_or_u32(rcx, (0x10u32) as u32); crate::simd_ops::wu8(rsi.wrapping_add(0x37u64), (0x10u8) as u8); crate::simd_ops::wu32(rsi.wrapping_add(0x38u64), (0x0u32) as u32); crate::simd_ops::wu8(rsi.wrapping_add(0x3cu64), (0x0u8) as u8); rdx = r14.wrapping_add(0x34); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru32(r14.wrapping_add(0xbcu64))) as u32 as u64; crate::simd_ops::wu32(rsi.wrapping_add(0xbcu64), ((rax as u32)) as u32); rax = ((crate::simd_ops::ru8(r14.wrapping_add(0xc0u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rsi.wrapping_add(0xc0u64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(r14.wrapping_add(0xc1u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rsi.wrapping_add(0xc1u64), ((rax as u8)) as u8); let y_addr: u64 = r14.wrapping_add(0xd0u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rsi.wrapping_add(0xd0u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); let y_addr: u64 = r14.wrapping_add(0xf0u64); xmm1 = crate::simd_ops::ru128(y_addr); ymm1_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rsi.wrapping_add(0xf0u64); crate::simd_ops::wu128(y_addr, xmm1); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm1_hi); let y_addr: u64 = r14.wrapping_add(0x110u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rsi.wrapping_add(0x110u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); let y_addr: u64 = r14.wrapping_add(0x130u64); xmm1 = crate::simd_ops::ru128(y_addr); ymm1_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rsi.wrapping_add(0x130u64); crate::simd_ops::wu128(y_addr, xmm1); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm1_hi); rax = ((crate::simd_ops::ru8(r14.wrapping_add(0x150u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rsi.wrapping_add(0x150u64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(r14.wrapping_add(0x151u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rsi.wrapping_add(0x151u64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(r14.wrapping_add(0x152u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rsi.wrapping_add(0x152u64), ((rax as u8)) as u8); rax = (crate::simd_ops::ru32(r14.wrapping_add(0x154u64))) as u32 as u64; crate::simd_ops::wu32(rsi.wrapping_add(0x154u64), ((rax as u32)) as u32); rbp = (((rbp as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((rbp as u32)) as u32 as u64; fl_r = ((r15 as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 4; continue; } bb = 5; continue; }
            5 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; bb = 6; continue; }
            6 => { rbx = ((rbx).wrapping_add(0x8u64)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 7; continue; }
            7 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14186c760(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x34; const P1: u64 = 0x14240ab10;
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
    loop { rcx = rbx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rbx = ((rbx).wrapping_add(0x160u64)) as u64; fl_l = rdi; fl_r = (0x1u64) as u64; rdi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if !((((fl_l as u64)) != ((fl_r as u64)))) { break; } }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14186c890(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x20;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut ymm1_hi: u128 = 0;
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let y_addr: u64 = rcx; xmm1 = crate::simd_ops::ru128(y_addr); ymm1_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16));
    xmm0 = ((crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64) as u128;
    rax = ((r8 as u32)) as u32 as u64;
    rcx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64;
    let y_addr: u64 = rdx.wrapping_add(rcx.wrapping_mul(8)); crate::simd_ops::wu128(y_addr, xmm1); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm1_hi);
    crate::simd_ops::wu64(rdx.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(P0), ((xmm0 as u64)) as u64);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14186cd40(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143084fd0; const P1: u64 = 0x14240a730;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 15>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), r8); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; r13 = r9; r12 = ((rdx as u32)) as u32 as u64; rbx = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x80u64), rbx); r10 = (crate::simd_ops::ru64(rcx)) as u64; rdi = ((rbx.wrapping_add(r10.wrapping_mul(8))) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 6; continue; } bb = 1; continue; }
            1 => { r14 = (crate::simd_ops::ru64(rbx)) as u64; r15 = r14; r15 = ((r15).wrapping_shr(32u32)) as u64; fl_l = ((r14 as u32)) as u32 as u64; fl_r = ((r15 as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 5; continue; } bb = 2; continue; }
            2 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; rbx = P0; bb = 3; continue; }
            3 => { rbp = ((r14 as u32)) as u32 as u64; rbp = ((rbp).wrapping_shl(7u32)) as u64; rbp = ((rbp).wrapping_add(rdi)) as u64; rsi = ((r12 as u32)) as u32 as u64; rsi = ((rsi).wrapping_shl(7u32)) as u64; rsi = ((rsi).wrapping_add(r13)) as u64; r12 = (((r12 as u32)).wrapping_add(1)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rsi); crate::simd_ops::wu64(rsi, rbx); rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x8u64))) as u32 as u64; crate::simd_ops::wu32(rsi.wrapping_add(0x8u64), ((rax as u32)) as u32); rax = ((crate::simd_ops::ru8(rbp.wrapping_add(0xcu64))) as u32) as u32 as u64; crate::simd_ops::wu8(rsi.wrapping_add(0xcu64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(rbp.wrapping_add(0xdu64))) as u32) as u32 as u64; crate::simd_ops::wu8(rsi.wrapping_add(0xdu64), ((rax as u8)) as u8); rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x10u64))) as u32 as u64; crate::simd_ops::wu32(rsi.wrapping_add(0x10u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x14u64))) as u32 as u64; crate::simd_ops::wu32(rsi.wrapping_add(0x14u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x18u64))) as u32 as u64; crate::simd_ops::wu32(rsi.wrapping_add(0x18u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x1cu64))) as u32 as u64; crate::simd_ops::wu32(rsi.wrapping_add(0x1cu64), ((rax as u32)) as u32); rax = ((crate::simd_ops::ru8(rbp.wrapping_add(0x20u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rsi.wrapping_add(0x20u64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(rbp.wrapping_add(0x21u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rsi.wrapping_add(0x21u64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(rbp.wrapping_add(0x22u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rsi.wrapping_add(0x22u64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(rbp.wrapping_add(0x23u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rsi.wrapping_add(0x23u64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(rbp.wrapping_add(0x24u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rsi.wrapping_add(0x24u64), ((rax as u8)) as u8); rcx = rsi.wrapping_add(0x28); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rcx); crate::simd_ops::lk_and_u32(rcx, (0xff000004u32) as u32); crate::simd_ops::lk_or_u32(rcx, (0x4u32) as u32); crate::simd_ops::wu8(rsi.wrapping_add(0x2bu64), (0x4u8) as u8); crate::simd_ops::wu32(rsi.wrapping_add(0x2cu64), (0x0u32) as u32); crate::simd_ops::wu8(rsi.wrapping_add(0x30u64), (0x0u8) as u8); rdx = rbp.wrapping_add(0x28); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rax = ((crate::simd_ops::ru8(rbp.wrapping_add(0x50u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rsi.wrapping_add(0x50u64), ((rax as u8)) as u8); rcx = rsi.wrapping_add(0x54); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rcx); crate::simd_ops::lk_and_u32(rcx, (0xff000004u32) as u32); crate::simd_ops::lk_or_u32(rcx, (0x4u32) as u32); crate::simd_ops::wu8(rsi.wrapping_add(0x57u64), (0x4u8) as u8); crate::simd_ops::wu32(rsi.wrapping_add(0x58u64), (0x0u32) as u32); crate::simd_ops::wu8(rsi.wrapping_add(0x5cu64), (0x0u8) as u8); rdx = rbp.wrapping_add(0x54); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); r14 = (((r14 as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((r14 as u32)) as u32 as u64; fl_r = ((r15 as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 3; continue; } bb = 4; continue; }
            4 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; bb = 5; continue; }
            5 => { rbx = ((rbx).wrapping_add(0x8u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x80u64), rbx); fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 6; continue; }
            6 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x88u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14186d600(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x10;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; r8 = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rcx = (crate::simd_ops::ru32(rax.wrapping_add(r8.wrapping_mul(4)))) as u32 as u64; r9 = ((rax.wrapping_add(r8.wrapping_mul(4))) as u64) as u64; rax = ((rcx as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shr(12u32)) as u32 as u64; fl_l = (crate::simd_ops::ru8(rdx)) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 3; continue; } bb = 1; continue; }
            1 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 5; continue; } bb = 2; continue; }
            2 => { fl_l = ((((rcx as u32)) >> (12u32)) & 1) as u64; rcx = (((rcx as u32)) | ((1u32) << (12u32))) as u32 as u64; crate::simd_ops::wu32(r9, ((rcx as u32)) as u32); return rax; }
            3 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { fl_l = ((((rcx as u32)) >> (12u32)) & 1) as u64; rcx = (((rcx as u32)) & !((1u32) << (12u32))) as u32 as u64; crate::simd_ops::wu32(r9, ((rcx as u32)) as u32); bb = 5; continue; }
            5 => { return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14186d650(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x154;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    r8 = (((crate::simd_ops::ru64(rcx.wrapping_add(P0))) as i64).wrapping_mul((0x160u64) as i64) as u64) as u64;
    fl_l = (crate::simd_ops::ru32(r8.wrapping_add(rax).wrapping_add(P1))) as u32 as u64; fl_r = 0x2;
    rax = (rax & !0xffu64) | (((if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { 1u8 } else { 0u8 }) as u8 as u64));
    crate::simd_ops::wu8(rdx, ((rax as u8)) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14186d680(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x28; const P1: u64 = 0x8; const P2: u64 = 0x26;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut ymm1_hi: u128 = 0;
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let y_addr: u64 = rcx; xmm1 = crate::simd_ops::ru128(y_addr); ymm1_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16));
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    r9 = rdx;
    xmm0 = (ymm1_hi) as u128;
    rdx = (((xmm0 >> 64) as u64)) as u64;
    xmm3 = 0u128;
    rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64;
    r8 = (((xmm1 >> 64) as u64)) as u64;
    xmm2 = ((crate::simd_ops::ru32(rdx.wrapping_add(rax.wrapping_mul(4)))) as u32) as u128;
    fl_l = ((xmm2 as u32)) as u32 as u64; fl_r = ((xmm3 as u32)) as u32 as u64;
    if !(f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
    xmm0 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) - f32::from_bits(crate::simd_ops::ru32(r9))).to_bits() as u128);
    fl_l = ((xmm3 as u32)) as u32 as u64; fl_r = ((xmm0 as u32)) as u32 as u64;
    crate::simd_ops::wu32(rdx.wrapping_add(rax.wrapping_mul(4)), ((xmm0 as u32)) as u32);
    if !(f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
    fl_l = (crate::simd_ops::ru8(r8.wrapping_add(rcx.wrapping_mul(4)).wrapping_add(P1))) as u8 as u64; fl_r = (0x0u8) as u8 as u64;
    if !((((fl_l as u8)) == ((fl_r as u8)))) { crate::simd_ops::wu8(r8.wrapping_add(rcx.wrapping_mul(4)).wrapping_add(P1), (0x0u8) as u8); rcx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; xmm0 = (ymm1_hi) as u128; rax = ((xmm0 as u64)) as u64; crate::simd_ops::wu8(rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(P2), (0x1u8) as u8); }
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14186d700(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x25;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rdx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64;
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    crate::simd_ops::wu8(rax.wrapping_add(rdx.wrapping_mul(8)).wrapping_add(P1), (0x0u8) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14186d750(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14020d420; const P1: u64 = 0x14020d4a0; const P2: u64 = 0x143c8d160; const P3: u64 = 0x142489ee0; const P4: u64 = 0x14020d3b0; const P5: u64 = 0x14020d5c0; const P6: u64 = 0x14020d730; const P7: u64 = 0x14186e320; const P8: u64 = 0x1426467e8;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm1_hi: u128 = 0; let mut ymm2_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 33>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = rsp; crate::simd_ops::wu64(rax.wrapping_add(0x20u64), r9); crate::simd_ops::wu64(rax.wrapping_add(0x18u64), r8); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0xd8u64)) as u64; let y_addr: u64 = rcx.wrapping_add(0x20u64); xmm2 = crate::simd_ops::ru128(y_addr); ymm2_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rcx; xmm1 = crate::simd_ops::ru128(y_addr); ymm1_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rax.wrapping_sub(0x78u64); crate::simd_ops::wu128(y_addr, xmm1); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm1_hi); r8 = (crate::simd_ops::ru64(rax.wrapping_sub(0x78u64))) as u64; r11 = rcx; xmm0 = (ymm2_hi) as u128; r10 = (((xmm0 >> 64) as u64)) as u64; rax = (((xmm1 >> 64) as u64)) as u64; xmm0 = (ymm1_hi) as u128; rbx = rdx; rcx = ((r10.wrapping_add(r10.wrapping_mul(2))) as u64) as u64; r14 = ((rax.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64; rcx = (((r10) as i64).wrapping_mul((0x1c0u64) as i64) as u64) as u64; rax = ((xmm0 as u64)) as u64; rdx = ((r10.wrapping_add(r10.wrapping_mul(2))) as u64) as u64; r12 = ((rax.wrapping_add(r10.wrapping_mul(8))) as u64) as u64; rax = ((xmm2 as u64)) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x110u64), r12); fl_l = (crate::simd_ops::ru64(r11.wrapping_add(0x18u64))) as u64; fl_r = (0x0u64) as u64; rax = (((xmm2 >> 64) as u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x80u64), rcx); r15 = ((rax.wrapping_add(r10.wrapping_mul(8))) as u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rbp = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x110u64), (0xffffffffffffffffu64) as u64); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x110u64))) as u64; crate::simd_ops::wu64(r14.wrapping_add(0x4u64), rax); crate::simd_ops::wu32(r14, ((rbp as u32)) as u32); crate::simd_ops::wu32(r12, ((rbp as u32)) as u32); crate::simd_ops::wu8(r15, ((rbp as u8)) as u8); crate::simd_ops::wu32(r15.wrapping_add(0x4u64), ((rbp as u32)) as u32); bb = 51; continue; }
            2 => { crate::simd_ops::wu64(rsp.wrapping_add(0x118u64), rsi); rsi = ((r8.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64; rcx = rsi.wrapping_add(0x10); crate::simd_ops::wu64(rsp.wrapping_add(0xd0u64), r13); rdx = r14.wrapping_add(0x4); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbp = 0; fl_l = 0; fl_r = 0; r13 = 0xffff; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 8; continue; } bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru32(r14)) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 7; continue; } bb = 4; continue; }
            4 => { xmm0 = (crate::simd_ops::ru128(rbx)) as u128; xmm1 = ((crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64) as u128; ymm1_hi = 0; rdx = (crate::simd_ops::ru64(r14.wrapping_add(0x4u64))) as u64; r8 = rsp.wrapping_add(0x60); rcx = rsp.wrapping_add(0x50); crate::simd_ops::wu128(rsp.wrapping_add(0x60u64), (xmm0) as u128); crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), ((xmm1 as u64)) as u64); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0xffffffff; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 7; continue; } bb = 5; continue; }
            5 => { r13 = (crate::simd_ops::ru64(rbx)) as u64; rcx = ((rax as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(r13.wrapping_add(0x30u64))) as u64; r12 = (crate::simd_ops::ru64(rax.wrapping_add(rcx.wrapping_mul(8)))) as u64; rax = 0xffff; fl_l = ((r12 as u16)) as u16 as u64; fl_r = ((rax as u16)) as u16 as u64; if (((fl_l as u16)) == ((fl_r as u16))) { bb = 48; continue; } bb = 6; continue; }
            6 => { r8 = ((crate::simd_ops::ru16(P2)) as u32) as u32 as u64; r11 = (((r12 as u16)) as u32) as u32 as u64; rcx = ((r11 as u32)) as u32 as u64; rcx = ((rcx).wrapping_shl(5u32)) as u64; rdx = (crate::simd_ops::ru32(rcx.wrapping_add(r13).wrapping_add(0x180acu64))) as u32 as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(r13).wrapping_add(0x18098u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = ((((rax as u32)) as i32 as i64) as u64) as u64; r8 = ((rbp.wrapping_add(0xau64)) as u32) as u32 as u64; rax = r11.wrapping_add(0xc03); r12 = ((r12).wrapping_shr(32u32)) as u64; rax = ((rax).wrapping_shl(5u32)) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(r13).wrapping_add(0x40u64))) as u64; rdx = (crate::simd_ops::ru32(rax.wrapping_add(rcx.wrapping_mul(4)))) as u32 as u64; rdx = ((rdx).wrapping_add(crate::simd_ops::ru64(r13.wrapping_add(r11.wrapping_mul(8)).wrapping_add(0x88u64)))) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x120u64))) as u64; rdx = (crate::simd_ops::ru64(rdx.wrapping_add(r12.wrapping_mul(8)))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x110u64))) as u64; r13 = 0xffff; crate::simd_ops::wu32(r14, ((rbp as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), (0xffffffffffffffffu64) as u64); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; crate::simd_ops::wu64(r14.wrapping_add(0x4u64), rax); crate::simd_ops::wu32(r12, ((rbp as u32)) as u32); crate::simd_ops::wu8(r15, ((rbp as u8)) as u8); crate::simd_ops::wu32(r15.wrapping_add(0x4u64), ((rbp as u32)) as u32); bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x10u64))) as u64; crate::simd_ops::wu64(r14.wrapping_add(0x4u64), rax); bb = 8; continue; }
            8 => { xmm0 = (crate::simd_ops::ru128(rbx)) as u128; xmm1 = ((crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64) as u128; ymm1_hi = 0; rdx = (crate::simd_ops::ru64(rsi.wrapping_add(0x10u64))) as u64; r8 = rsp.wrapping_add(0x60); rcx = rsp.wrapping_add(0x50); crate::simd_ops::wu128(rsp.wrapping_add(0x60u64), (xmm0) as u128); crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), ((xmm1 as u64)) as u64); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0xffffffff; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 49; continue; } bb = 9; continue; }
            9 => { r8 = (crate::simd_ops::ru64(rbx)) as u64; rcx = ((rax as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(r8.wrapping_add(0x30u64))) as u64; rdx = (crate::simd_ops::ru64(rax.wrapping_add(rcx.wrapping_mul(8)))) as u64; fl_l = ((rdx as u16)) as u16 as u64; fl_r = ((r13 as u16)) as u16 as u64; if (((fl_l as u16)) == ((fl_r as u16))) { bb = 48; continue; } bb = 10; continue; }
            10 => { r9 = (((rdx as u16)) as u32) as u32 as u64; rcx = rsp.wrapping_add(0x90); r8 = ((r8).wrapping_add(0x40u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rdx); r12 = (crate::simd_ops::ru64(r8.wrapping_add(r9.wrapping_mul(8)).wrapping_add(0x48u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 11; continue; }
            11 => { fl_l = ((rdx).wrapping_add(r12)) as u64; fl_r = fl_l; rdx = ((fl_l as u64)) as u64; bb = 12; continue; }
            12 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x98u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 13; continue; }
            13 => { fl_l = ((rcx).wrapping_add(r12)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; bb = 14; continue; }
            14 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xa0u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 16; continue; } bb = 15; continue; }
            15 => { fl_l = ((rax).wrapping_add(r12)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; bb = 16; continue; }
            16 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0xa8u64))) as u64; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 18; continue; } bb = 17; continue; }
            17 => { fl_l = ((rsi).wrapping_add(r12)) as u64; fl_r = fl_l; rsi = ((fl_l as u64)) as u64; bb = 18; continue; }
            18 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0xb0u64))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 20; continue; } bb = 19; continue; }
            19 => { fl_l = ((rbx).wrapping_add(r12)) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; bb = 20; continue; }
            20 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0xb8u64))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 22; continue; } bb = 21; continue; }
            21 => { fl_l = ((rdi).wrapping_add(r12)) as u64; fl_r = fl_l; rdi = ((fl_l as u64)) as u64; bb = 22; continue; }
            22 => { r12 = (crate::simd_ops::ru32(rsp.wrapping_add(0x64u64))) as u32 as u64; r13 = ((r12 as u32)) as u32 as u64; r13 = ((r13).wrapping_shl(7u32)) as u64; r13 = ((r13).wrapping_add(rcx)) as u64; r9 = ((rdx.wrapping_add(r12.wrapping_mul(8))) as u64) as u64; r8 = ((r12.wrapping_add(r12.wrapping_mul(2))) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), r8); rdx = ((r12.wrapping_add(r12.wrapping_mul(4))) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rdx); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 24; continue; } bb = 23; continue; }
            23 => { rcx = ((crate::simd_ops::ru8(r12.wrapping_add(rax))) as u32) as u32 as u64; bb = 25; continue; }
            24 => { rcx = (rcx & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 25; continue; }
            25 => { fl_l = (crate::simd_ops::ru32(rsi.wrapping_add(r8.wrapping_mul(4)).wrapping_add(0x4u64))) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 44; continue; } bb = 26; continue; }
            26 => { fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 29; continue; } bb = 27; continue; }
            27 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x13u64))) as u8 as u64; fl_r = ((rbp as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 29; continue; } bb = 28; continue; }
            28 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 30; continue; }
            29 => { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 30; continue; }
            30 => { rdi = (crate::simd_ops::ru32(r14)) as u32 as u64; fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 33; continue; } bb = 31; continue; }
            31 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x120u64))) as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 40; continue; } bb = 32; continue; }
            32 => { crate::simd_ops::wu32(r14, (0x1u32) as u32); bb = 40; continue; }
            33 => { fl_l = ((rdi as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 36; continue; } bb = 34; continue; }
            34 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x120u64))) as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 40; continue; } bb = 35; continue; }
            35 => { crate::simd_ops::wu32(r14, (0x4u32) as u32); r8 = ((rdi.wrapping_add(0xau64)) as u32) as u32 as u64; rdx = (crate::simd_ops::ru64(r9)) as u64; rcx = rbp; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; r8 = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; bb = 40; continue; }
            36 => { fl_l = ((rdi as u32)) as u32 as u64; fl_r = 0x4; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 39; continue; } bb = 37; continue; }
            37 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 39; continue; } bb = 38; continue; }
            38 => { crate::simd_ops::wu32(r14, ((rbp as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), (0xffffffffffffffffu64) as u64); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; crate::simd_ops::wu64(r14.wrapping_add(0x4u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x110u64))) as u64; crate::simd_ops::wu32(rax, ((rbp as u32)) as u32); crate::simd_ops::wu8(r15, ((rbp as u8)) as u8); crate::simd_ops::wu32(r15.wrapping_add(0x4u64), ((rbp as u32)) as u32); bb = 39; continue; }
            39 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x120u64))) as u64; bb = 40; continue; }
            40 => { fl_l = (crate::simd_ops::ru32(r14)) as u32 as u64; fl_r = 0x4; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 50; continue; } bb = 41; continue; }
            41 => { fl_l = ((rdi as u32)) as u32 as u64; fl_r = 0x4; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 50; continue; } bb = 42; continue; }
            42 => { crate::simd_ops::wu16(rbx.wrapping_add(rdx.wrapping_mul(8)).wrapping_add(0x24u64), (0x101u16) as u16); rax = (crate::simd_ops::ru32(rsi.wrapping_add(r8.wrapping_mul(4)).wrapping_add(0x4u64))) as u32 as u64; rax = (((rax as u32)).wrapping_shr(1u32)) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 50; continue; } bb = 43; continue; }
            43 => { rdx = 0x5e; rcx = rbp; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 50; continue; }
            44 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; r8 = (crate::simd_ops::ru64(rsp.wrapping_add(0x130u64))) as u64; rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x128u64))) as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x48u64), ((rcx as u8)) as u8); rax = ((crate::simd_ops::ru8(rax.wrapping_add(0x13u64))) as u32) as u32 as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x120u64))) as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x40u64), ((rax as u8)) as u8); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x110u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), r15); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), r14); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r9); r9 = r13; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 50; continue; } bb = 45; continue; }
            45 => { rax = ((r12.wrapping_add(r12.wrapping_mul(4))) as u64) as u64; crate::simd_ops::wu8(rbx.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x24u64), (0x1u8) as u8); rax = ((r12.wrapping_add(r12.wrapping_mul(2))) as u64) as u64; crate::simd_ops::wu8(rsi.wrapping_add(rax.wrapping_mul(4)).wrapping_add(0x8u64), (0x1u8) as u8); xmm1 = ((crate::simd_ops::ru32(r13.wrapping_add(0x10u64))) as u32) as u128; ymm1_hi = 0; xmm0 = 0u128; fl_l = ((xmm1 as u32)) as u32 as u64; fl_r = ((xmm0 as u32)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 50; continue; } bb = 46; continue; }
            46 => { fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 50; continue; } bb = 47; continue; }
            47 => { crate::simd_ops::wu32(rdi.wrapping_add(r12.wrapping_mul(4)), ((xmm1 as u32)) as u32); bb = 50; continue; }
            48 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P8), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, 0x14186db9e, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            49 => { crate::simd_ops::wu32(r14, ((rbp as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x110u64), (0xffffffffffffffffu64) as u64); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x110u64))) as u64; crate::simd_ops::wu64(r14.wrapping_add(0x4u64), rax); crate::simd_ops::wu32(r12, ((rbp as u32)) as u32); crate::simd_ops::wu8(r15, ((rbp as u8)) as u8); crate::simd_ops::wu32(r15.wrapping_add(0x4u64), ((rbp as u32)) as u32); bb = 50; continue; }
            50 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x118u64))) as u64; r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0xd0u64))) as u64; bb = 51; continue; }
            51 => { rsp = ((rsp).wrapping_add(0xd8u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14186dc00(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x20;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut ymm1_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x38u64)) as u64; xmm0 = ((crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64) as u128; let y_addr: u64 = rcx; xmm1 = crate::simd_ops::ru128(y_addr); ymm1_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), ((xmm0 as u64)) as u64); rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rax = (((xmm1 >> 64) as u64)) as u64; xmm0 = (ymm1_hi) as u128; rcx = ((rdx.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64; r9 = ((rax.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64; rcx = (crate::simd_ops::ru32(rax.wrapping_add(rcx.wrapping_mul(4)))) as u32 as u64; r8 = ((rcx as u32)) as u32 as u64; r8 = (((r8 as u32)).wrapping_shr(1u32)) as u32 as u64; rax = ((xmm0 as u64)) as u64; r8 = (r8 & !0xffu64) | ((((((r8 as u8)) & (0x1u8))) as u8 as u64)); fl_l = (crate::simd_ops::ru32(rax.wrapping_add(rdx.wrapping_mul(4)))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 3; continue; } bb = 1; continue; }
            1 => { fl_l = ((r8 as u8)) as u8 as u64; fl_r = ((r8 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 5; continue; } bb = 2; continue; }
            2 => { rcx = ((((rcx as u32)) | (0x2u32))) as u32 as u64; crate::simd_ops::wu32(r9, ((rcx as u32)) as u32); rsp = ((rsp).wrapping_add(0x38u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            3 => { fl_l = ((r8 as u8)) as u8 as u64; fl_r = ((r8 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { fl_l = ((((rcx as u32)) & (0xfffffffdu32))) as u32 as u64; fl_r = fl_l; rcx = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(r9, ((rcx as u32)) as u32); bb = 5; continue; }
            5 => { rsp = ((rsp).wrapping_add(0x38u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14186dc80(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    xmm1 = ((crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64) as u128;
    xmm0 = (crate::simd_ops::ru128(rcx)) as u128;
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), ((xmm1 as u64)) as u64);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x10u64))) as u64;
    crate::simd_ops::wu128(rsp, (xmm0) as u128);
    rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64;
    rax = (crate::simd_ops::ru64(rsp)) as u64;
    rdx = ((rax.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64;
    rcx = (crate::simd_ops::ru32(rax.wrapping_add(rcx.wrapping_mul(4)))) as u32 as u64;
    rax = ((rcx as u32)) as u32 as u64;
    rax = (((rax as u32)).wrapping_shr(2u32)) as u32 as u64;
    fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64;
    if !(((((fl_l as u8)) & ((fl_r as u8))) != 0)) { fl_l = ((((rcx as u32)) | (0x4u32))) as u32 as u64; fl_r = fl_l; rcx = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(rdx, ((rcx as u32)) as u32); }
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14186dce0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    xmm1 = ((crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64) as u128;
    xmm0 = (crate::simd_ops::ru128(rcx)) as u128;
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), ((xmm1 as u64)) as u64);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x10u64))) as u64;
    crate::simd_ops::wu128(rsp, (xmm0) as u128);
    rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64;
    rax = (crate::simd_ops::ru64(rsp)) as u64;
    rdx = ((rax.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64;
    rcx = (crate::simd_ops::ru32(rax.wrapping_add(rcx.wrapping_mul(4)))) as u32 as u64;
    rax = ((rcx as u32)) as u32 as u64;
    rax = (((rax as u32)).wrapping_shr(2u32)) as u32 as u64;
    fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64;
    if !(((((fl_l as u8)) & ((fl_r as u8))) == 0)) { fl_l = ((((rcx as u32)) & (0xfffffffbu32))) as u32 as u64; fl_r = fl_l; rcx = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(rdx, ((rcx as u32)) as u32); }
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14186dfb0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x8; const P2: u64 = 0x14020fdc0; const P3: u64 = 0x140210100; const P4: u64 = 0x14020fa90;
    let (mut rax, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rbp = rsp; rsp = ((rsp).wrapping_sub(0x50u64)) as u64; rbx = rdx; crate::simd_ops::wu32(rbp.wrapping_sub(0x30u64), (0x0u32) as u32); crate::simd_ops::wu64(rbp.wrapping_sub(0x2cu64), (0xffffffffffffffffu64) as u64); rsi = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rdi = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; rax = (crate::simd_ops::ru64(rdi.wrapping_add(rsi.wrapping_mul(8)).wrapping_add(P0))) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x20u64), rax); crate::simd_ops::wu8(rbp.wrapping_sub(0x18u64), (0x0u8) as u8); xmm0 = ((crate::simd_ops::ru64(rbp.wrapping_sub(0x20u64))) as u64) as u128; crate::simd_ops::wu64(rbp.wrapping_sub(0x10u64), ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(rbp.wrapping_sub(0x18u64))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_sub(0x8u64), ((rax as u32)) as u32); r8 = rbp.wrapping_sub(0x30); rdx = rbp.wrapping_sub(0x10); rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm0 = 0u128; crate::simd_ops::wu32(rbp.wrapping_add(0x10u64), ((xmm0 as u32)) as u32); crate::simd_ops::wu32(rbp.wrapping_add(0x14u64), ((xmm0 as u32)) as u32); rax = (crate::simd_ops::ru64(rdi.wrapping_add(rsi.wrapping_mul(8)).wrapping_add(P0))) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x20u64), rax); crate::simd_ops::wu8(rbp.wrapping_sub(0x18u64), (0x0u8) as u8); xmm0 = ((crate::simd_ops::ru64(rbp.wrapping_sub(0x20u64))) as u64) as u128; crate::simd_ops::wu64(rbp.wrapping_sub(0x10u64), ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(rbp.wrapping_sub(0x18u64))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_sub(0x8u64), ((rax as u32)) as u32); r8 = rbp.wrapping_add(0x10); rdx = rbp.wrapping_sub(0x10); rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu8(rbp.wrapping_add(0x10u64), (0x0u8) as u8); crate::simd_ops::wu32(rbp.wrapping_add(0x14u64), (0x0u32) as u32); rax = (crate::simd_ops::ru64(rdi.wrapping_add(rsi.wrapping_mul(8)).wrapping_add(P0))) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x20u64), rax); crate::simd_ops::wu8(rbp.wrapping_sub(0x18u64), (0x0u8) as u8); xmm0 = ((crate::simd_ops::ru64(rbp.wrapping_sub(0x20u64))) as u64) as u128; crate::simd_ops::wu64(rbp.wrapping_sub(0x10u64), ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(rbp.wrapping_sub(0x18u64))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_sub(0x8u64), ((rax as u32)) as u32); r8 = rbp.wrapping_add(0x10); rdx = rbp.wrapping_sub(0x10); rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rsp = ((rsp).wrapping_add(0x50u64)) as u64; rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14186e320(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14020d3b0; const P1: u64 = 0x143427080; const P2: u64 = 0x143843de8; const P3: u64 = 0x143843d50; const P4: u64 = 0x143843d10; const P5: u64 = 0x1434268d0; const P6: u64 = 0x14397d850; const P7: u64 = 0x143080e40; const P8: u64 = 0x141a0dc30; const P9: u64 = 0x143843e68; const P10: u64 = 0x143843e28; const P11: u64 = 0x143843e88; const P12: u64 = 0x143080ea0; const P13: u64 = 0x143080e80; const P14: u64 = 0x143080e60; const P15: u64 = 0x143843cf0; const P16: u64 = 0x143843e08; const P17: u64 = 0x143843dc8; const P18: u64 = 0x143843d30; const P19: u64 = 0x143843e48; const P20: u64 = 0x143843cd0; const P21: u64 = 0x143843bd0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm4: u128 = 0; let mut xmm5: u128 = 0; let mut xmm6: u128 = 0; let mut xmm7: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 17>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), r8); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = ((rsp).wrapping_sub(0x60u64)) as u64; fl_l = (crate::simd_ops::ru8(rsp.wrapping_add(0xd8u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; rbp = r9; r9 = rcx; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 3; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru8(rsp.wrapping_add(0xd0u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rsi = (rsi & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 4; continue; }
            3 => { rsi = (rsi & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 4; continue; }
            4 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0xb8u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xa8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), r14); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), r15); r12 = (crate::simd_ops::ru32(rdi)) as u32 as u64; r13 = ((r12 as u32)) as u32 as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm6) as u128); crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm7) as u128); fl_l = ((r12 as u32)) as u32 as u64; fl_r = ((r12 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 7; continue; } bb = 5; continue; }
            5 => { fl_l = ((rsi as u8)) as u8 as u64; fl_r = ((rsi as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 129; continue; } bb = 6; continue; }
            6 => { crate::simd_ops::wu32(rdi, (0x1u32) as u32); r13 = ((r12.wrapping_add(0x1u64)) as u32) as u32 as u64; bb = 7; continue; }
            7 => { rcx = (crate::simd_ops::ru32(rbp.wrapping_add(0x8u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 16; continue; } bb = 8; continue; }
            8 => { fl_l = ((r12 as u32)) as u32 as u64; fl_r = 0x4; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 11; continue; } bb = 9; continue; }
            9 => { fl_l = ((rsi as u8)) as u8 as u64; fl_r = ((rsi as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 13; continue; } bb = 10; continue; }
            10 => { rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0xb0u64))) as u64; r8 = ((rcx.wrapping_add(0x8u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rdi, (0x4u32) as u32); rcx = r9; rdx = (crate::simd_ops::ru64(rdx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 13; continue; }
            11 => { fl_l = ((rsi as u8)) as u8 as u64; fl_r = ((rsi as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 13; continue; } bb = 12; continue; }
            12 => { rbx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0xb8u64), (0xffffffffffffffffu64) as u64); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xb8u64))) as u64; crate::simd_ops::wu64(rdi.wrapping_add(0x4u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xc0u64))) as u64; crate::simd_ops::wu32(rdi, ((rbx as u32)) as u32); crate::simd_ops::wu32(rax, ((rbx as u32)) as u32); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xc8u64))) as u64; crate::simd_ops::wu8(rax, ((rbx as u8)) as u8); crate::simd_ops::wu32(rax.wrapping_add(0x4u64), ((rbx as u32)) as u32); bb = 13; continue; }
            13 => { fl_l = (crate::simd_ops::ru32(rdi)) as u32 as u64; fl_r = 0x4; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 129; continue; } bb = 14; continue; }
            14 => { fl_l = ((r12 as u32)) as u32 as u64; fl_r = 0x4; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 129; continue; } bb = 15; continue; }
            15 => { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 130; continue; }
            16 => { rax = ((rcx.wrapping_sub(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 48; continue; } bb = 17; continue; }
            17 => { r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0xc0u64))) as u64; rax = ((r13.wrapping_sub(0x1u64)) as u32) as u32 as u64; r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0xb0u64))) as u64; rbx = 0; fl_l = 0; fl_r = 0; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0xfffffffd; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 20; continue; } bb = 18; continue; }
            18 => { fl_l = ((rsi as u8)) as u8 as u64; fl_r = ((rsi as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 20; continue; } bb = 19; continue; }
            19 => { crate::simd_ops::wu32(rdi, (0x2u32) as u32); r8 = ((rbx.wrapping_add(0x9u64)) as u32) as u32 as u64; rdx = (crate::simd_ops::ru64(r15)) as u64; rcx = r9; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = (crate::simd_ops::ru64(rsp.wrapping_add(0xa0u64))) as u64; rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x98u64))) as u64; bb = 26; continue; }
            20 => { fl_l = ((r13 as u32)) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 23; continue; } bb = 21; continue; }
            21 => { fl_l = ((rsi as u8)) as u8 as u64; fl_r = ((rsi as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 23; continue; } bb = 22; continue; }
            22 => { crate::simd_ops::wu32(rdi, (0x3u32) as u32); r8 = ((r13.wrapping_add(0x8u64)) as u32) as u32 as u64; rdx = (crate::simd_ops::ru64(r15)) as u64; rcx = r9; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 27; continue; }
            23 => { fl_l = ((r12 as u32)) as u32 as u64; fl_r = 0x4; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 26; continue; } bb = 24; continue; }
            24 => { fl_l = ((rsi as u8)) as u8 as u64; fl_r = ((rsi as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 26; continue; } bb = 25; continue; }
            25 => { crate::simd_ops::wu32(rdi, ((rbx as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0xb8u64), (0xffffffffffffffffu64) as u64); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xb8u64))) as u64; crate::simd_ops::wu64(rdi.wrapping_add(0x4u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xc8u64))) as u64; crate::simd_ops::wu32(r14, ((rbx as u32)) as u32); crate::simd_ops::wu8(rax, ((rbx as u8)) as u8); crate::simd_ops::wu32(rax.wrapping_add(0x4u64), ((rbx as u32)) as u32); bb = 26; continue; }
            26 => { fl_l = ((r13 as u32)) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 41; continue; } bb = 27; continue; }
            27 => { rcx = (crate::simd_ops::ru32(rbp.wrapping_add(0x8u64))) as u32 as u64; xmm0 = 0u128; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 35; continue; } bb = 28; continue; }
            28 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 34; continue; } bb = 29; continue; }
            29 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 33; continue; } bb = 30; continue; }
            30 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 32; continue; } bb = 31; continue; }
            31 => { xmm1 = ((crate::simd_ops::ru32(P1)) as u32) as u128; bb = 36; continue; }
            32 => { xmm1 = ((crate::simd_ops::ru32(P2)) as u32) as u128; bb = 36; continue; }
            33 => { xmm1 = ((crate::simd_ops::ru32(P3)) as u32) as u128; bb = 36; continue; }
            34 => { xmm1 = ((crate::simd_ops::ru32(P4)) as u32) as u128; bb = 36; continue; }
            35 => { xmm1 = 0u128; bb = 36; continue; }
            36 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xa0u64))) as u64; xmm5 = ((crate::simd_ops::ru32(P5)) as u32) as u128; xmm2 = ((crate::simd_ops::ru32(rax.wrapping_add(0x4u64))) as u32) as u128; fl_l = ((xmm2 as u32)) as u32 as u64; fl_r = ((xmm0 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)).is_nan() || f32::from_bits((fl_r as u32)).is_nan()) { bb = 38; continue; } bb = 37; continue; }
            37 => { if (f32::from_bits((fl_l as u32)) == f32::from_bits((fl_r as u32)) || f32::from_bits((fl_l as u32)).is_nan() || f32::from_bits((fl_r as u32)).is_nan()) { bb = 39; continue; } bb = 38; continue; }
            38 => { xmm0 = (xmm5 & !0xffff_ffffu128) | ((f32::from_bits(xmm5 as u32) / f32::from_bits(xmm2 as u32)).to_bits() as u128); bb = 39; continue; }
            39 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x98u64))) as u64; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(crate::simd_ops::ru32(rax))).to_bits() as u128); xmm1 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) / f32::from_bits(xmm1 as u32)).to_bits() as u128); xmm3 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) + f32::from_bits(crate::simd_ops::ru32(r14))).to_bits() as u128); xmm1 = (xmm5) as u128; xmm2 = (xmm1 & !0xffff_ffffu128) | ((if f32::from_bits(xmm1 as u32) < f32::from_bits(xmm3 as u32) { f32::from_bits(xmm1 as u32) } else { f32::from_bits(xmm3 as u32) }).to_bits() as u128); fl_l = ((xmm2 as u32)) as u32 as u64; fl_r = ((xmm5 as u32)) as u32 as u64; crate::simd_ops::wu32(r14, ((xmm2 as u32)) as u32); if !(f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 13; continue; } bb = 40; continue; }
            40 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; r8 = 0x8; crate::simd_ops::wu32(r14, ((rbx as u32)) as u32); crate::simd_ops::wu32(rdi, (0x4u32) as u32); rdx = (crate::simd_ops::ru64(r15)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 13; continue; }
            41 => { fl_l = ((r13 as u32)) as u32 as u64; fl_r = 0x3; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 13; continue; } bb = 42; continue; }
            42 => { xmm1 = ((crate::simd_ops::ru32(r8.wrapping_add(0x4u64))) as u32) as u128; xmm5 = 0u128; fl_l = ((xmm1 as u32)) as u32 as u64; fl_r = ((xmm5 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)).is_nan() || f32::from_bits((fl_r as u32)).is_nan()) { bb = 45; continue; } bb = 43; continue; }
            43 => { if (f32::from_bits((fl_l as u32)) != f32::from_bits((fl_r as u32)) && !f32::from_bits((fl_l as u32)).is_nan() && !f32::from_bits((fl_r as u32)).is_nan()) { bb = 45; continue; } bb = 44; continue; }
            44 => { xmm0 = 0u128; bb = 46; continue; }
            45 => { xmm0 = ((crate::simd_ops::ru32(P5)) as u32) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) / f32::from_bits(xmm1 as u32)).to_bits() as u128); bb = 46; continue; }
            46 => { xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(crate::simd_ops::ru32(rdx))).to_bits() as u128); xmm2 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) / f32::from_bits(crate::simd_ops::ru32(P4))).to_bits() as u128); xmm1 = ((crate::simd_ops::ru32(r14)) as u32) as u128; xmm3 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) - f32::from_bits(xmm2 as u32)).to_bits() as u128); xmm2 = 0u128; xmm0 = (xmm2 & !0xffff_ffffu128) | ((if f32::from_bits(xmm2 as u32) > f32::from_bits(xmm3 as u32) { f32::from_bits(xmm2 as u32) } else { f32::from_bits(xmm3 as u32) }).to_bits() as u128); fl_l = ((xmm5 as u32)) as u32 as u64; fl_r = ((xmm0 as u32)) as u32 as u64; crate::simd_ops::wu32(r14, ((xmm0 as u32)) as u32); if !(f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 13; continue; } bb = 47; continue; }
            47 => { crate::simd_ops::wu32(rdi, (0x1u32) as u32); bb = 129; continue; }
            48 => { rax = ((rcx.wrapping_sub(0x5u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 129; continue; } bb = 49; continue; }
            49 => { r14 = (crate::simd_ops::ru64(P6)) as u64; rax = P7; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); rdx = rsp.wrapping_add(0x20); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (0x1fu64) as u64); rcx = r14; xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = (crate::simd_ops::ru32(rbp.wrapping_add(0x8u64))) as u32 as u64; crate::simd_ops::wu8(rsp.wrapping_add(0xd8u64), ((rax as u8)) as u8); xmm7 = ((crate::simd_ops::ru32(P5)) as u32) as u128; fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x5; rdx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 54; continue; } bb = 50; continue; }
            50 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x1; rdx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 53; continue; } bb = 51; continue; }
            51 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 58; continue; } bb = 52; continue; }
            52 => { xmm0 = ((crate::simd_ops::ru32(P9)) as u32) as u128; bb = 55; continue; }
            53 => { xmm0 = ((crate::simd_ops::ru32(P10)) as u32) as u128; bb = 55; continue; }
            54 => { xmm0 = ((crate::simd_ops::ru32(P11)) as u32) as u128; bb = 55; continue; }
            55 => { fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = ((xmm7 as u32)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 58; continue; } bb = 56; continue; }
            56 => { crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (0x28u64) as u64); rax = P12; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); rdx = rsp.wrapping_add(0x20); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; rcx = r14; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 58; continue; } bb = 57; continue; }
            57 => { rbx = (rbx & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 59; continue; }
            58 => { rbx = (rbx & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 59; continue; }
            59 => { crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (0x18u64) as u64); rax = P13; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); rdx = rsp.wrapping_add(0x20); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; rcx = r14; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (0x1fu64) as u64); rdx = rsp.wrapping_add(0x20); r15 = (((rax as u8)) as u32) as u32 as u64; rcx = r14; rax = P14; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru8(rbp.wrapping_add(0x50u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 63; continue; } bb = 60; continue; }
            60 => { fl_l = ((r15 as u8)) as u8 as u64; fl_r = ((r15 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 62; continue; } bb = 61; continue; }
            61 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 63; continue; } bb = 62; continue; }
            62 => { rcx = (rcx & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 64; continue; }
            63 => { rcx = (rcx & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 64; continue; }
            64 => { rdx = ((crate::simd_ops::ru8(rsp.wrapping_add(0xd8u64))) as u32) as u32 as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0xc8u64))) as u64; fl_l = ((rdx as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 66; continue; } bb = 65; continue; }
            65 => { fl_l = (crate::simd_ops::ru32(r14.wrapping_add(0x4u64))) as u32 as u64; fl_r = 0x0; if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { bb = 68; continue; } bb = 66; continue; }
            66 => { fl_l = ((rbx as u8)) as u8 as u64; fl_r = ((rbx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 68; continue; } bb = 67; continue; }
            67 => { fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 69; continue; } bb = 68; continue; }
            68 => { rsi = (rsi & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 69; continue; }
            69 => { rax = ((r13.wrapping_sub(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0xfffffffd; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 90; continue; } bb = 70; continue; }
            70 => { fl_l = ((rsi as u8)) as u8 as u64; fl_r = ((rsi as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 90; continue; } bb = 71; continue; }
            71 => { fl_l = (crate::simd_ops::ru8(r14)) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 73; continue; } bb = 72; continue; }
            72 => { fl_l = (crate::simd_ops::lk_add_u32(r14.wrapping_add(0x4u64), (1) as u32)) as u32 as u64; crate::simd_ops::wu8(r14, (0x1u8) as u8); bb = 73; continue; }
            73 => { rcx = (crate::simd_ops::ru32(rbp.wrapping_add(0x8u64))) as u32 as u64; xmm6 = 0u128; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x5; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 79; continue; } bb = 74; continue; }
            74 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 78; continue; } bb = 75; continue; }
            75 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 77; continue; } bb = 76; continue; }
            76 => { xmm0 = 0u128; bb = 80; continue; }
            77 => { xmm0 = ((crate::simd_ops::ru32(P15)) as u32) as u128; bb = 80; continue; }
            78 => { xmm0 = ((crate::simd_ops::ru32(P16)) as u32) as u128; bb = 80; continue; }
            79 => { xmm0 = ((crate::simd_ops::ru32(P17)) as u32) as u128; bb = 80; continue; }
            80 => { r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0xc0u64))) as u64; r8 = 0x9; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0xb0u64))) as u64; r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; rcx = r13; crate::simd_ops::wu32(r15.wrapping_add(0x4u64), ((xmm0 as u32)) as u32); crate::simd_ops::wu32(rdi, (0x2u32) as u32); rdx = (crate::simd_ops::ru64(r12)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru32(rbp.wrapping_add(0x8u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x5; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 86; continue; } bb = 81; continue; }
            81 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 85; continue; } bb = 82; continue; }
            82 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 84; continue; } bb = 83; continue; }
            83 => { xmm0 = 0u128; bb = 87; continue; }
            84 => { xmm0 = ((crate::simd_ops::ru32(P9)) as u32) as u128; bb = 87; continue; }
            85 => { xmm0 = ((crate::simd_ops::ru32(P10)) as u32) as u128; bb = 87; continue; }
            86 => { xmm0 = ((crate::simd_ops::ru32(P11)) as u32) as u128; bb = 87; continue; }
            87 => { xmm0 = (xmm7 & !0xffff_ffffu128) | ((f32::from_bits(xmm7 as u32) / f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm2 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(crate::simd_ops::ru32(r15))).to_bits() as u128); xmm4 = (xmm2 & !0xffff_ffffu128) | ((if f32::from_bits(xmm2 as u32) < f32::from_bits(xmm7 as u32) { f32::from_bits(xmm2 as u32) } else { f32::from_bits(xmm7 as u32) }).to_bits() as u128); fl_l = ((xmm4 as u32)) as u32 as u64; fl_r = ((xmm7 as u32)) as u32 as u64; crate::simd_ops::wu32(r15, ((xmm4 as u32)) as u32); xmm3 = (xmm7) as u128; if (f32::from_bits((fl_l as u32)).is_nan() || f32::from_bits((fl_r as u32)).is_nan()) { bb = 109; continue; } bb = 88; continue; }
            88 => { if (f32::from_bits((fl_l as u32)) != f32::from_bits((fl_r as u32)) && !f32::from_bits((fl_l as u32)).is_nan() && !f32::from_bits((fl_r as u32)).is_nan()) { bb = 109; continue; } bb = 89; continue; }
            89 => { rbx = 0; fl_l = 0; fl_r = 0; rcx = r13; crate::simd_ops::wu64(r15, rbx); crate::simd_ops::wu32(rdi, (0x4u32) as u32); crate::simd_ops::wu8(r14, ((rbx as u8)) as u8); crate::simd_ops::wu32(r14.wrapping_add(0x4u64), ((rbx as u32)) as u32); r8 = ((rbx.wrapping_add(0x8u64)) as u32) as u32 as u64; rdx = (crate::simd_ops::ru64(r12)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 130; continue; }
            90 => { r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0xc0u64))) as u64; xmm6 = 0u128; fl_l = ((rdx as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 97; continue; } bb = 91; continue; }
            91 => { fl_l = ((rbx as u8)) as u8 as u64; fl_r = ((rbx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 97; continue; } bb = 92; continue; }
            92 => { fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 97; continue; } bb = 93; continue; }
            93 => { fl_l = ((r13 as u32)) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 106; continue; } bb = 94; continue; }
            94 => { fl_l = ((r12 as u32)) as u32 as u64; fl_r = 0x4; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 108; continue; } bb = 95; continue; }
            95 => { r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0xb0u64))) as u64; r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; fl_l = ((rsi as u8)) as u8 as u64; fl_r = ((rsi as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 109; continue; } bb = 96; continue; }
            96 => { rbx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0xb8u64), (0xffffffffffffffffu64) as u64); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xb8u64))) as u64; crate::simd_ops::wu64(rdi.wrapping_add(0x4u64), rax); crate::simd_ops::wu32(rdi, ((rbx as u32)) as u32); crate::simd_ops::wu32(r15, ((rbx as u32)) as u32); crate::simd_ops::wu8(r14, ((rbx as u8)) as u8); crate::simd_ops::wu32(r14.wrapping_add(0x4u64), ((rbx as u32)) as u32); bb = 109; continue; }
            97 => { fl_l = ((r13 as u32)) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 94; continue; } bb = 98; continue; }
            98 => { rcx = (crate::simd_ops::ru32(rbp.wrapping_add(0x8u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x5; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 104; continue; } bb = 99; continue; }
            99 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 103; continue; } bb = 100; continue; }
            100 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 102; continue; } bb = 101; continue; }
            101 => { xmm0 = 0u128; bb = 105; continue; }
            102 => { xmm0 = ((crate::simd_ops::ru32(P15)) as u32) as u128; bb = 105; continue; }
            103 => { xmm0 = ((crate::simd_ops::ru32(P16)) as u32) as u128; bb = 105; continue; }
            104 => { xmm0 = ((crate::simd_ops::ru32(P17)) as u32) as u128; bb = 105; continue; }
            105 => { xmm1 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) - f32::from_bits(crate::simd_ops::ru32(P18))).to_bits() as u128); xmm2 = 0u128; xmm0 = (xmm2 & !0xffff_ffffu128) | ((if f32::from_bits(xmm2 as u32) > f32::from_bits(xmm1 as u32) { f32::from_bits(xmm2 as u32) } else { f32::from_bits(xmm1 as u32) }).to_bits() as u128); fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(r15.wrapping_add(0x4u64))) as u32 as u64; if (f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 107; continue; } bb = 106; continue; }
            106 => { fl_l = ((rsi as u8)) as u8 as u64; fl_r = ((rsi as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 94; continue; } bb = 107; continue; }
            107 => { crate::simd_ops::wu32(rdi, (0x3u32) as u32); crate::simd_ops::wu8(r14, (0x0u8) as u8); bb = 108; continue; }
            108 => { r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0xb0u64))) as u64; bb = 109; continue; }
            109 => { rcx = (crate::simd_ops::ru32(rbp.wrapping_add(0x8u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x5; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 115; continue; } bb = 110; continue; }
            110 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 114; continue; } bb = 111; continue; }
            111 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 113; continue; } bb = 112; continue; }
            112 => { xmm0 = 0u128; bb = 116; continue; }
            113 => { xmm0 = ((crate::simd_ops::ru32(P19)) as u32) as u128; bb = 116; continue; }
            114 => { xmm0 = ((crate::simd_ops::ru32(P20)) as u32) as u128; bb = 116; continue; }
            115 => { xmm0 = ((crate::simd_ops::ru32(P21)) as u32) as u128; bb = 116; continue; }
            116 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xa0u64))) as u64; xmm1 = ((crate::simd_ops::ru32(rax.wrapping_add(0x4u64))) as u32) as u128; fl_l = ((xmm1 as u32)) as u32 as u64; fl_r = ((xmm6 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)).is_nan() || f32::from_bits((fl_r as u32)).is_nan()) { bb = 119; continue; } bb = 117; continue; }
            117 => { if (f32::from_bits((fl_l as u32)) != f32::from_bits((fl_r as u32)) && !f32::from_bits((fl_l as u32)).is_nan() && !f32::from_bits((fl_r as u32)).is_nan()) { bb = 119; continue; } bb = 118; continue; }
            118 => { xmm1 = 0u128; bb = 120; continue; }
            119 => { xmm1 = (xmm7 & !0xffff_ffffu128) | ((f32::from_bits(xmm7 as u32) / f32::from_bits(xmm1 as u32)).to_bits() as u128); bb = 120; continue; }
            120 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x98u64))) as u64; xmm2 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(crate::simd_ops::ru32(rax))).to_bits() as u128); xmm7 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) / f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm0 = ((crate::simd_ops::ru32(r15.wrapping_add(0x4u64))) as u32) as u128; fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = ((xmm6 as u32)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 124; continue; } bb = 121; continue; }
            121 => { xmm1 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) - f32::from_bits(xmm2 as u32)).to_bits() as u128); xmm2 = 0u128; xmm0 = (xmm2 & !0xffff_ffffu128) | ((if f32::from_bits(xmm2 as u32) > f32::from_bits(xmm1 as u32) { f32::from_bits(xmm2 as u32) } else { f32::from_bits(xmm1 as u32) }).to_bits() as u128); fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = ((xmm6 as u32)) as u32 as u64; crate::simd_ops::wu32(r15.wrapping_add(0x4u64), ((xmm0 as u32)) as u32); if (f32::from_bits((fl_l as u32)).is_nan() || f32::from_bits((fl_r as u32)).is_nan()) { bb = 124; continue; } bb = 122; continue; }
            122 => { if (f32::from_bits((fl_l as u32)) != f32::from_bits((fl_r as u32)) && !f32::from_bits((fl_l as u32)).is_nan() && !f32::from_bits((fl_r as u32)).is_nan()) { bb = 124; continue; } bb = 123; continue; }
            123 => { rdx = (crate::simd_ops::ru64(r12)) as u64; r8 = 0xa; rcx = r13; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 124; continue; }
            124 => { xmm0 = ((crate::simd_ops::ru32(r15.wrapping_add(0x4u64))) as u32) as u128; fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = ((xmm6 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)).is_nan() || f32::from_bits((fl_r as u32)).is_nan()) { bb = 129; continue; } bb = 125; continue; }
            125 => { if (f32::from_bits((fl_l as u32)) != f32::from_bits((fl_r as u32)) && !f32::from_bits((fl_l as u32)).is_nan() && !f32::from_bits((fl_r as u32)).is_nan()) { bb = 129; continue; } bb = 126; continue; }
            126 => { xmm0 = ((crate::simd_ops::ru32(r15)) as u32) as u128; xmm2 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) - f32::from_bits(xmm7 as u32)).to_bits() as u128); xmm3 = 0u128; xmm1 = (xmm3 & !0xffff_ffffu128) | ((if f32::from_bits(xmm3 as u32) > f32::from_bits(xmm2 as u32) { f32::from_bits(xmm3 as u32) } else { f32::from_bits(xmm2 as u32) }).to_bits() as u128); fl_l = ((xmm1 as u32)) as u32 as u64; fl_r = ((xmm6 as u32)) as u32 as u64; crate::simd_ops::wu32(r15, ((xmm1 as u32)) as u32); if (f32::from_bits((fl_l as u32)).is_nan() || f32::from_bits((fl_r as u32)).is_nan()) { bb = 129; continue; } bb = 127; continue; }
            127 => { if (f32::from_bits((fl_l as u32)) != f32::from_bits((fl_r as u32)) && !f32::from_bits((fl_l as u32)).is_nan() && !f32::from_bits((fl_r as u32)).is_nan()) { bb = 129; continue; } bb = 128; continue; }
            128 => { rax = (((rsi as u8)) as u32) as u32 as u64; fl_l = (((rax as u32)).wrapping_add(1)) as u32 as u64; rax = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(rdi, ((rax as u32)) as u32); bb = 129; continue; }
            129 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 130; continue; }
            130 => { xmm7 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x40u64))) as u128; r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0xa8u64))) as u64; rsp = ((rsp).wrapping_add(0x60u64)) as u64; r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14186ea50(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xa; const P1: u64 = 0x14020d3b0; const P2: u64 = 0x4;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; fl_l = (crate::simd_ops::ru8(rsp.wrapping_add(0x60u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; rbx = r8; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 3; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru8(rsp.wrapping_add(0x58u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 4; continue; }
            3 => { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 4; continue; }
            4 => { rdi = (crate::simd_ops::ru32(r8)) as u32 as u64; fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 7; continue; } bb = 5; continue; }
            5 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 13; continue; } bb = 6; continue; }
            6 => { crate::simd_ops::wu32(r8, (0x1u32) as u32); bb = 13; continue; }
            7 => { fl_l = ((rdi as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 10; continue; } bb = 8; continue; }
            8 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 13; continue; } bb = 9; continue; }
            9 => { crate::simd_ops::wu32(r8, (0x4u32) as u32); r8 = ((rdi.wrapping_add(P0)) as u32) as u32 as u64; rdx = (crate::simd_ops::ru64(rdx)) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 13; continue; }
            10 => { fl_l = ((rdi as u32)) as u32 as u64; fl_r = 0x4; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 13; continue; } bb = 11; continue; }
            11 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 13; continue; } bb = 12; continue; }
            12 => { rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), (0xffffffffffffffffu64) as u64); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; crate::simd_ops::wu64(r8.wrapping_add(P2), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; crate::simd_ops::wu32(r8, ((rcx as u32)) as u32); crate::simd_ops::wu32(r9, ((rcx as u32)) as u32); crate::simd_ops::wu8(rax, ((rcx as u8)) as u8); crate::simd_ops::wu32(rax.wrapping_add(P2), ((rcx as u32)) as u32); bb = 13; continue; }
            13 => { fl_l = (crate::simd_ops::ru32(rbx)) as u32 as u64; fl_r = 0x4; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 16; continue; } bb = 14; continue; }
            14 => { fl_l = ((rdi as u32)) as u32 as u64; fl_r = 0x4; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 16; continue; } bb = 15; continue; }
            15 => { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            16 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14186eb00(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x4;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    r9 = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (0xffffffffffffffffu64) as u64);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x8u64))) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(P0), rax);
    crate::simd_ops::wu32(rcx, ((r9 as u32)) as u32);
    crate::simd_ops::wu32(rdx, ((r9 as u32)) as u32);
    crate::simd_ops::wu8(r8, ((r9 as u8)) as u8);
    crate::simd_ops::wu32(r8.wrapping_add(P0), ((r9 as u32)) as u32);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14186eb30(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142407c80; const P1: u64 = 0x18; const P2: u64 = 0x4; const P3: u64 = 0x8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdi = r8;
    rbx = rcx;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rax = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64;
    rdx = (crate::simd_ops::ru64(rbx)) as u64;
    r9 = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64;
    rax = (crate::simd_ops::ru32(rdi)) as u32 as u64;
    crate::simd_ops::wu32(rdx.wrapping_add(r9.wrapping_mul(4)), ((rax as u32)) as u32);
    rax = ((crate::simd_ops::ru8(rdi.wrapping_add(P2))) as u32) as u32 as u64;
    crate::simd_ops::wu8(rdx.wrapping_add(r9.wrapping_mul(4)).wrapping_add(P3), ((rax as u8)) as u8);
    rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P3))) as u64;
    fl_l = rdx; fl_r = rdx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64; rax = (crate::simd_ops::ru32(rdi.wrapping_add(P3))) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_add(rcx.wrapping_mul(4)), ((rax as u32)) as u32); }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14186eb90(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x18; const P1: u64 = 0x8; const P2: u64 = 0x4; const P3: u64 = 0x143427080;
    let (mut rax, mut r10, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    r8 = (crate::simd_ops::ru64(rdx)) as u64;
    r10 = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    r9 = ((r10.wrapping_add(r10.wrapping_mul(2))) as u64) as u64;
    rax = (crate::simd_ops::ru32(r8.wrapping_add(r9.wrapping_mul(4)))) as u32 as u64;
    crate::simd_ops::wu32(rcx, ((rax as u32)) as u32);
    rax = ((crate::simd_ops::ru8(r8.wrapping_add(r9.wrapping_mul(4)).wrapping_add(P1))) as u32) as u32 as u64;
    crate::simd_ops::wu8(rcx.wrapping_add(P2), ((rax as u8)) as u8);
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P1))) as u64;
    fl_l = rax; fl_r = rax;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { xmm0 = ((crate::simd_ops::ru32(rax.wrapping_add(r10.wrapping_mul(4)))) as u32) as u128; crate::simd_ops::wu32(rcx.wrapping_add(P1), ((xmm0 as u32)) as u32); rax = rcx; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
    xmm0 = ((crate::simd_ops::ru32(P3)) as u32) as u128;
    crate::simd_ops::wu32(rcx.wrapping_add(P1), ((xmm0 as u32)) as u32);
    rax = rcx;
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14186eca0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x143427080; const P1: u64 = 0x143843de8; const P2: u64 = 0x143843d50; const P3: u64 = 0x143843d10;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64;
    if !(((((fl_l as u32)) & ((fl_r as u32))) == 0)) { fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if !((((fl_l as u32)) == ((fl_r as u32)))) { fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if !((((fl_l as u32)) == ((fl_r as u32)))) { fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; if !((((fl_l as u32)) == ((fl_r as u32)))) { xmm0 = ((crate::simd_ops::ru32(P0)) as u32) as u128; crate::simd_ops::set_xmm_ret(xmm0); return rax; } xmm0 = ((crate::simd_ops::ru32(P1)) as u32) as u128; crate::simd_ops::set_xmm_ret(xmm0); return rax; } xmm0 = ((crate::simd_ops::ru32(P2)) as u32) as u128; crate::simd_ops::set_xmm_ret(xmm0); return rax; } xmm0 = ((crate::simd_ops::ru32(P3)) as u32) as u128; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
    xmm0 = 0u128;
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14186efb0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x4; const P1: u64 = 0x8;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P0))) as u32) as u32 as u64;
    r9 = rdx;
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P1))) as u64;
    rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    crate::simd_ops::wu8(rcx.wrapping_add(rdx).wrapping_add(P0), ((rax as u8)) as u8);
    rax = (crate::simd_ops::ru32(rdx)) as u32 as u64;
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P1))) as u64;
    r9 = ((r9).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    crate::simd_ops::wu32(r9.wrapping_add(rcx), ((rax as u32)) as u32);
    rax = ((rdx.wrapping_add(P1)) as u64) as u64;
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P1))) as u64;
    rdx = rax;
    fl_l = rdx; fl_r = (crate::simd_ops::ru64(r8)) as u64;
    rdx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64;
    rax = (crate::simd_ops::ru32(rax)) as u32 as u64;
    crate::simd_ops::wu32(rdx.wrapping_add(rcx), ((rax as u32)) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14186eff0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x4;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P1))) as u32) as u32 as u64;
    crate::simd_ops::wu8(rcx.wrapping_add(rdx).wrapping_add(P1), ((rax as u8)) as u8);
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    rax = (crate::simd_ops::ru32(rdx)) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(rdx), ((rax as u32)) as u32);
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    fl_l = rcx; fl_r = (crate::simd_ops::ru64(r8)) as u64;
    rcx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64;
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(rdx).wrapping_add(P0), ((rax as u32)) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14186f2f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143c8d468; const P1: u64 = 0x143c8d52c; const P2: u64 = 0x140215890; const P3: u64 = 0x14387ebf8; const P4: u64 = 0x140210440; const P5: u64 = 0x1402116e0; const P6: u64 = 0x1426e57b0; const P7: u64 = 0x1410d6ca0; const P8: u64 = 0x140210470; const P9: u64 = 0x143823778; const P10: u64 = 0x14020e980; const P11: u64 = 0x14020afa0; const P12: u64 = 0x1420a6b70; const P13: u64 = 0x140210750; const P14: u64 = 0x143823788; const P15: u64 = 0x140210a30; const P16: u64 = 0x140210d30; const P17: u64 = 0x1426e57a0; const P18: u64 = 0x14020d780; const P19: u64 = 0x143823120; const P20: u64 = 0x143823190; const P21: u64 = 0x14020c350; const P22: u64 = 0x14020d9d0; const P23: u64 = 0x14020dcb0; const P24: u64 = 0x14020dfc0; const P25: u64 = 0x14020e2f0; const P26: u64 = 0x14020e610;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 29>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rbp = rsp.wrapping_sub(0x37); rsp = ((rsp).wrapping_sub(0xc0u64)) as u64; r15 = rcx.wrapping_add(0x8); r9 = ((crate::simd_ops::ru16(P0)) as u32) as u32 as u64; r8 = ((r9 as u32)) as u32 as u64; r8 = ((r8).wrapping_shl(6u32)) as u64; r8 = ((r8).wrapping_add(crate::simd_ops::ru64(rcx.wrapping_add(0x90u64)))) as u64; rax = (crate::simd_ops::ru32(P1)) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(0x3cu64), ((rax as u32)) as u32); rax = P2; crate::simd_ops::wu64(r8.wrapping_add(0x30u64), rax); crate::simd_ops::wu8(r8.wrapping_add(0x38u64), (0x1u8) as u8); crate::simd_ops::wu8(r8, (0x0u8) as u8); rax = (crate::simd_ops::ru64(rcx.wrapping_add(0xa0u64))) as u64; crate::simd_ops::wu64(rax.wrapping_add(r9.wrapping_mul(8)), r8); rcx = (crate::simd_ops::ru64(rcx)) as u64; rcx = ((rcx).wrapping_add(0xc0u64)) as u64; rax = (crate::simd_ops::ru32(P3)) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_sub(0x9u64), ((rax as u32)) as u32); rax = P4; crate::simd_ops::wu64(rbp.wrapping_sub(0x1u64), rax); r8 = rbp.wrapping_sub(0x9); rdx = rbp.wrapping_add(0x67); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm0 = (crate::simd_ops::ru128(P6)) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rbp.wrapping_sub(0x9u64), (xmm0) as u128); r8 = rbp.wrapping_sub(0x9); rdx = rbp.wrapping_sub(0x59); rcx = r15.wrapping_add(0x18); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rbp.wrapping_sub(0x29u64), r15); rax = (crate::simd_ops::ru64(rbp.wrapping_sub(0x51u64))) as u64; xmm0 = (crate::simd_ops::ru128(rax.wrapping_add(0x10u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rbp.wrapping_sub(0x21u64), (xmm0) as u128); rdx = rbp.wrapping_sub(0x49); rcx = rbp.wrapping_sub(0x29); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rsi = (crate::simd_ops::ru64(rbp.wrapping_sub(0x49u64))) as u64; rcx = rsi.wrapping_add(0x48); xmm0 = (crate::simd_ops::ru128(P9)) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rbp.wrapping_sub(0x59u64), (xmm0) as u128); rdx = rbp.wrapping_sub(0x59); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P10, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r14 = rax; rbx = ((crate::simd_ops::ru16(rax)) as u32) as u32 as u64; rdx = (0x2aaaaaaaaaaaaaabu64) as u64; r13 = 0xffff; fl_l = ((rbx as u16)) as u16 as u64; fl_r = ((r13 as u16)) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 4; continue; } bb = 1; continue; }
            1 => { rdi = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; xmm0 = (crate::simd_ops::ru128(P9)) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rbp.wrapping_sub(0x59u64), (xmm0) as u128); rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x18u64))) as u32 as u64; r8 = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; r8 = ((r8).wrapping_shl(4u32)) as u64; rax = rdx; let prod = (rax as i64 as i128).wrapping_mul((r8) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rbx = rdx; rbx = ((((rbx) as i64).wrapping_shr(3u32) as u64)) as u64; rax = rbx; rax = ((rax).wrapping_shr(63u32)) as u64; rbx = ((rbx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x1cu64))) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; fl_l = r8; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rdx = rbx.wrapping_add(0x1); rcx = rdi.wrapping_add(0x10); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P11, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x18u64))) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rdi.wrapping_add(0x10u64)))) as u64; xmm0 = 0u128; ymm0_hi = 0; let y_addr: u64 = rcx; crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); crate::simd_ops::wu128(rcx.wrapping_add(0x20u64), (xmm0) as u128); crate::simd_ops::lk_add_u32(rdi.wrapping_add(0x18u64), (1) as u32); rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x18u64))) as u32 as u64; rdx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rdx = ((rdx).wrapping_shl(4u32)) as u64; fl_l = ((rdx).wrapping_add(crate::simd_ops::ru64(rdi.wrapping_add(0x10u64)))) as u64; fl_r = fl_l; rdx = ((fl_l as u64)) as u64; xmm0 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x59u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rdx.wrapping_sub(0x30u64), (xmm0) as u128); crate::simd_ops::wu32(rdx.wrapping_sub(0xeu64), (0xffffffffu32) as u32); rax = (crate::simd_ops::ru64(rbp.wrapping_sub(0x39u64))) as u64; crate::simd_ops::wu64(rdx.wrapping_sub(0x20u64), rax); crate::simd_ops::wu64(rdx.wrapping_sub(0x18u64), rbx); crate::simd_ops::wu16(r14, ((rbx as u16)) as u16); bb = 4; continue; }
            4 => { r9 = 0x2; r8 = (((rbx as u16)) as u32) as u32 as u64; rdx = ((crate::simd_ops::ru16(rbp.wrapping_sub(0x3fu64))) as u32) as u32 as u64; rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P12, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rbp.wrapping_sub(0x49); rcx = rbp.wrapping_sub(0x29); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P13, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rsi = (crate::simd_ops::ru64(rbp.wrapping_sub(0x49u64))) as u64; rcx = rsi.wrapping_add(0x48); xmm0 = (crate::simd_ops::ru128(P14)) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rbp.wrapping_sub(0x59u64), (xmm0) as u128); rdx = rbp.wrapping_sub(0x59); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P10, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r14 = rax; rbx = ((crate::simd_ops::ru16(rax)) as u32) as u32 as u64; fl_l = ((rbx as u16)) as u16 as u64; fl_r = ((r13 as u16)) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 8; continue; } bb = 5; continue; }
            5 => { rdi = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; xmm0 = (crate::simd_ops::ru128(P14)) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rbp.wrapping_sub(0x59u64), (xmm0) as u128); rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x18u64))) as u32 as u64; r8 = ((rcx.wrapping_add(rcx.wrapping_mul(2))) as u64) as u64; r8 = ((r8).wrapping_shl(4u32)) as u64; rax = (0x2aaaaaaaaaaaaaabu64) as u64; let prod = (rax as i64 as i128).wrapping_mul((r8) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rbx = rdx; rbx = ((((rbx) as i64).wrapping_shr(3u32) as u64)) as u64; rax = rbx; rax = ((rax).wrapping_shr(63u32)) as u64; rbx = ((rbx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x1cu64))) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; fl_l = r8; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rdx = rbx.wrapping_add(0x1); rcx = rdi.wrapping_add(0x10); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P11, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x18u64))) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rdi.wrapping_add(0x10u64)))) as u64; xmm0 = 0u128; ymm0_hi = 0; let y_addr: u64 = rcx; crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); crate::simd_ops::wu128(rcx.wrapping_add(0x20u64), (xmm0) as u128); crate::simd_ops::lk_add_u32(rdi.wrapping_add(0x18u64), (1) as u32); rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x18u64))) as u32 as u64; rdx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rdx = ((rdx).wrapping_shl(4u32)) as u64; fl_l = ((rdx).wrapping_add(crate::simd_ops::ru64(rdi.wrapping_add(0x10u64)))) as u64; fl_r = fl_l; rdx = ((fl_l as u64)) as u64; xmm0 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x59u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rdx.wrapping_sub(0x30u64), (xmm0) as u128); crate::simd_ops::wu32(rdx.wrapping_sub(0xeu64), (0xffffffffu32) as u32); rax = (crate::simd_ops::ru64(rbp.wrapping_sub(0x39u64))) as u64; crate::simd_ops::wu64(rdx.wrapping_sub(0x20u64), rax); crate::simd_ops::wu64(rdx.wrapping_sub(0x18u64), rbx); crate::simd_ops::wu16(r14, ((rbx as u16)) as u16); bb = 8; continue; }
            8 => { r9 = 0x2; r8 = (((rbx as u16)) as u32) as u32 as u64; rdx = ((crate::simd_ops::ru16(rbp.wrapping_sub(0x3fu64))) as u32) as u32 as u64; rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P12, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rbp.wrapping_sub(0x49); rcx = rbp.wrapping_sub(0x29); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P15, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rsi = (crate::simd_ops::ru64(rbp.wrapping_sub(0x49u64))) as u64; rcx = rsi.wrapping_add(0x48); xmm0 = (crate::simd_ops::ru128(P9)) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rbp.wrapping_sub(0x59u64), (xmm0) as u128); rdx = rbp.wrapping_sub(0x59); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P10, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r14 = rax; rbx = ((crate::simd_ops::ru16(rax)) as u32) as u32 as u64; fl_l = ((rbx as u16)) as u16 as u64; fl_r = ((r13 as u16)) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 12; continue; } bb = 9; continue; }
            9 => { rdi = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; xmm0 = (crate::simd_ops::ru128(P9)) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rbp.wrapping_sub(0x59u64), (xmm0) as u128); rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x18u64))) as u32 as u64; r8 = ((rcx.wrapping_add(rcx.wrapping_mul(2))) as u64) as u64; r8 = ((r8).wrapping_shl(4u32)) as u64; rax = (0x2aaaaaaaaaaaaaabu64) as u64; let prod = (rax as i64 as i128).wrapping_mul((r8) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rbx = rdx; rbx = ((((rbx) as i64).wrapping_shr(3u32) as u64)) as u64; rax = rbx; rax = ((rax).wrapping_shr(63u32)) as u64; rbx = ((rbx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x1cu64))) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; fl_l = r8; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 11; continue; } bb = 10; continue; }
            10 => { rdx = rbx.wrapping_add(0x1); rcx = rdi.wrapping_add(0x10); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P11, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 11; continue; }
            11 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x18u64))) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rdi.wrapping_add(0x10u64)))) as u64; xmm0 = 0u128; ymm0_hi = 0; let y_addr: u64 = rcx; crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); crate::simd_ops::wu128(rcx.wrapping_add(0x20u64), (xmm0) as u128); crate::simd_ops::lk_add_u32(rdi.wrapping_add(0x18u64), (1) as u32); rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x18u64))) as u32 as u64; rdx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rdx = ((rdx).wrapping_shl(4u32)) as u64; fl_l = ((rdx).wrapping_add(crate::simd_ops::ru64(rdi.wrapping_add(0x10u64)))) as u64; fl_r = fl_l; rdx = ((fl_l as u64)) as u64; xmm0 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x59u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rdx.wrapping_sub(0x30u64), (xmm0) as u128); crate::simd_ops::wu32(rdx.wrapping_sub(0xeu64), (0xffffffffu32) as u32); rax = (crate::simd_ops::ru64(rbp.wrapping_sub(0x39u64))) as u64; crate::simd_ops::wu64(rdx.wrapping_sub(0x20u64), rax); crate::simd_ops::wu64(rdx.wrapping_sub(0x18u64), rbx); crate::simd_ops::wu16(r14, ((rbx as u16)) as u16); bb = 12; continue; }
            12 => { r9 = 0x2; r8 = (((rbx as u16)) as u32) as u32 as u64; rdx = ((crate::simd_ops::ru16(rbp.wrapping_sub(0x3fu64))) as u32) as u32 as u64; rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P12, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rbp.wrapping_sub(0x49); rcx = rbp.wrapping_sub(0x29); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P16, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rsi = (crate::simd_ops::ru64(rbp.wrapping_sub(0x49u64))) as u64; rcx = rsi.wrapping_add(0x48); xmm0 = (crate::simd_ops::ru128(P14)) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rbp.wrapping_sub(0x59u64), (xmm0) as u128); rdx = rbp.wrapping_sub(0x59); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P10, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r14 = rax; rbx = ((crate::simd_ops::ru16(rax)) as u32) as u32 as u64; fl_l = ((rbx as u16)) as u16 as u64; fl_r = ((r13 as u16)) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 16; continue; } bb = 13; continue; }
            13 => { rdi = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; xmm0 = (crate::simd_ops::ru128(P14)) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rbp.wrapping_sub(0x59u64), (xmm0) as u128); rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x18u64))) as u32 as u64; r8 = ((rcx.wrapping_add(rcx.wrapping_mul(2))) as u64) as u64; r8 = ((r8).wrapping_shl(4u32)) as u64; rax = (0x2aaaaaaaaaaaaaabu64) as u64; let prod = (rax as i64 as i128).wrapping_mul((r8) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rbx = rdx; rbx = ((((rbx) as i64).wrapping_shr(3u32) as u64)) as u64; rax = rbx; rax = ((rax).wrapping_shr(63u32)) as u64; rbx = ((rbx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x1cu64))) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; fl_l = r8; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 15; continue; } bb = 14; continue; }
            14 => { rdx = rbx.wrapping_add(0x1); rcx = rdi.wrapping_add(0x10); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P11, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 15; continue; }
            15 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x18u64))) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rdi.wrapping_add(0x10u64)))) as u64; xmm0 = 0u128; ymm0_hi = 0; let y_addr: u64 = rcx; crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); crate::simd_ops::wu128(rcx.wrapping_add(0x20u64), (xmm0) as u128); crate::simd_ops::lk_add_u32(rdi.wrapping_add(0x18u64), (1) as u32); rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x18u64))) as u32 as u64; rdx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rdx = ((rdx).wrapping_shl(4u32)) as u64; fl_l = ((rdx).wrapping_add(crate::simd_ops::ru64(rdi.wrapping_add(0x10u64)))) as u64; fl_r = fl_l; rdx = ((fl_l as u64)) as u64; xmm0 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x59u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rdx.wrapping_sub(0x30u64), (xmm0) as u128); crate::simd_ops::wu32(rdx.wrapping_sub(0xeu64), (0xffffffffu32) as u32); rax = (crate::simd_ops::ru64(rbp.wrapping_sub(0x39u64))) as u64; crate::simd_ops::wu64(rdx.wrapping_sub(0x20u64), rax); crate::simd_ops::wu64(rdx.wrapping_sub(0x18u64), rbx); crate::simd_ops::wu16(r14, ((rbx as u16)) as u16); bb = 16; continue; }
            16 => { r9 = 0x2; r8 = (((rbx as u16)) as u32) as u32 as u64; rdx = ((crate::simd_ops::ru16(rbp.wrapping_sub(0x3fu64))) as u32) as u32 as u64; rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P12, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm0 = (crate::simd_ops::ru128(P17)) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rbp.wrapping_sub(0x59u64), (xmm0) as u128); r8 = rbp.wrapping_sub(0x59); rdx = rbp.wrapping_sub(0x9); rcx = r15.wrapping_add(0x18); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rbp.wrapping_sub(0x29u64), r15); rax = (crate::simd_ops::ru64(rbp.wrapping_sub(0x1u64))) as u64; xmm0 = (crate::simd_ops::ru128(rax.wrapping_add(0x10u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rbp.wrapping_sub(0x21u64), (xmm0) as u128); rdx = rbp.wrapping_add(0x17); rcx = rbp.wrapping_sub(0x29); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P18, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r13 = (crate::simd_ops::ru64(rax)) as u64; rdi = ((crate::simd_ops::ru16(rax.wrapping_add(0xau64))) as u32) as u32 as u64; xmm0 = (crate::simd_ops::ru128(rax.wrapping_add(0x10u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rbp.wrapping_sub(0x49u64), (xmm0) as u128); crate::simd_ops::wu128(rbp.wrapping_add(0x7u64), (xmm0) as u128); rcx = r13.wrapping_add(0x48); xmm1 = (crate::simd_ops::ru128(P19)) as u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x59u64), (xmm1) as u128); rdx = rbp.wrapping_sub(0x59); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P10, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r15 = rax; rbx = ((crate::simd_ops::ru16(rax)) as u32) as u32 as u64; r12 = 0xffff; fl_l = ((rbx as u16)) as u16 as u64; fl_r = ((r12 as u16)) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 20; continue; } bb = 17; continue; }
            17 => { rsi = (crate::simd_ops::ru64(r13.wrapping_add(0x8u64))) as u64; xmm0 = (crate::simd_ops::ru128(P19)) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rbp.wrapping_sub(0x9u64), (xmm0) as u128); rcx = (crate::simd_ops::ru32(rsi.wrapping_add(0x18u64))) as u32 as u64; r8 = ((rcx.wrapping_add(rcx.wrapping_mul(2))) as u64) as u64; r8 = ((r8).wrapping_shl(4u32)) as u64; rax = (0x2aaaaaaaaaaaaaabu64) as u64; let prod = (rax as i64 as i128).wrapping_mul((r8) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rbx = rdx; rbx = ((((rbx) as i64).wrapping_shr(3u32) as u64)) as u64; rax = rbx; rax = ((rax).wrapping_shr(63u32)) as u64; rbx = ((rbx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x1cu64))) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; fl_l = r8; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 19; continue; } bb = 18; continue; }
            18 => { rdx = rbx.wrapping_add(0x1); rcx = rsi.wrapping_add(0x10); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P11, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 19; continue; }
            19 => { rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x18u64))) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rsi.wrapping_add(0x10u64)))) as u64; xmm0 = 0u128; ymm0_hi = 0; let y_addr: u64 = rcx; crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); crate::simd_ops::wu128(rcx.wrapping_add(0x20u64), (xmm0) as u128); crate::simd_ops::lk_add_u32(rsi.wrapping_add(0x18u64), (1) as u32); rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x18u64))) as u32 as u64; rdx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rdx = ((rdx).wrapping_shl(4u32)) as u64; fl_l = ((rdx).wrapping_add(crate::simd_ops::ru64(rsi.wrapping_add(0x10u64)))) as u64; fl_r = fl_l; rdx = ((fl_l as u64)) as u64; xmm0 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x9u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rdx.wrapping_sub(0x30u64), (xmm0) as u128); crate::simd_ops::wu32(rdx.wrapping_sub(0xeu64), (0xffffffffu32) as u32); rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x7u64))) as u64; crate::simd_ops::wu64(rdx.wrapping_sub(0x20u64), rax); crate::simd_ops::wu64(rdx.wrapping_sub(0x18u64), rbx); crate::simd_ops::wu16(r15, ((rbx as u16)) as u16); bb = 20; continue; }
            20 => { r9 = 0x2; r8 = (((rbx as u16)) as u32) as u32 as u64; rdx = (((rdi as u16)) as u32) as u32 as u64; rcx = (crate::simd_ops::ru64(r13.wrapping_add(0x8u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P12, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm0 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x49u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rbp.wrapping_sub(0x49u64), (xmm0) as u128); crate::simd_ops::wu8(rsp.wrapping_add(0x20u64), (0x0u8) as u8); r9 = rbp.wrapping_sub(0x49); r8 = P20; rdx = r13; rcx = rbp.wrapping_add(0x67); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P21, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r9 = 0x1; r8 = (((rdi as u16)) as u32) as u32 as u64; rdx = ((crate::simd_ops::ru16(rax)) as u32) as u32 as u64; rcx = (crate::simd_ops::ru64(r13.wrapping_add(0x8u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P12, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rbp.wrapping_add(0x17); rcx = rbp.wrapping_sub(0x29); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P22, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rsi = (crate::simd_ops::ru64(rax)) as u64; rbx = ((crate::simd_ops::ru16(rax.wrapping_add(0xau64))) as u32) as u32 as u64; xmm0 = (crate::simd_ops::ru128(rax.wrapping_add(0x10u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rbp.wrapping_sub(0x49u64), (xmm0) as u128); crate::simd_ops::wu8(rsp.wrapping_add(0x20u64), (0x0u8) as u8); r9 = rbp.wrapping_sub(0x49); r8 = P19; rdx = rsi; rcx = rbp.wrapping_add(0x67); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P21, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r9 = 0x2; r8 = ((crate::simd_ops::ru16(rbp.wrapping_add(0x67u64))) as u32) as u32 as u64; rdx = (((rbx as u16)) as u32) as u32 as u64; rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P12, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r9 = 0x1; r8 = (((rbx as u16)) as u32) as u32 as u64; rdx = (((rdi as u16)) as u32) as u32 as u64; rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P12, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rbp.wrapping_add(0x17); rcx = rbp.wrapping_sub(0x29); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P23, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rsi = (crate::simd_ops::ru64(rax)) as u64; rbx = ((crate::simd_ops::ru16(rax.wrapping_add(0xau64))) as u32) as u32 as u64; xmm0 = (crate::simd_ops::ru128(rax.wrapping_add(0x10u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rbp.wrapping_add(0x7u64), (xmm0) as u128); crate::simd_ops::wu128(rbp.wrapping_sub(0x49u64), (xmm0) as u128); crate::simd_ops::wu8(rsp.wrapping_add(0x20u64), (0x0u8) as u8); r9 = rbp.wrapping_sub(0x49); r8 = P19; rdx = rsi; rcx = rbp.wrapping_add(0x67); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P21, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r9 = 0x2; r8 = ((crate::simd_ops::ru16(rbp.wrapping_add(0x67u64))) as u32) as u32 as u64; rdx = (((rbx as u16)) as u32) as u32 as u64; rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P12, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r9 = 0x1; r8 = (((rdi as u16)) as u32) as u32 as u64; rdx = (((rbx as u16)) as u32) as u32 as u64; rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P12, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm0 = (crate::simd_ops::ru128(rbp.wrapping_add(0x7u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rbp.wrapping_sub(0x49u64), (xmm0) as u128); crate::simd_ops::wu8(rsp.wrapping_add(0x20u64), (0x0u8) as u8); r9 = rbp.wrapping_sub(0x49); r8 = P20; rdx = rsi; rcx = rbp.wrapping_add(0x67); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P21, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r9 = 0x1; r8 = (((rbx as u16)) as u32) as u32 as u64; rdx = ((crate::simd_ops::ru16(rax)) as u32) as u32 as u64; rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P12, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rbp.wrapping_add(0x17); rcx = rbp.wrapping_sub(0x29); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P24, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = (crate::simd_ops::ru64(rax)) as u64; rdi = ((crate::simd_ops::ru16(rax.wrapping_add(0xau64))) as u32) as u32 as u64; xmm0 = (crate::simd_ops::ru128(rax.wrapping_add(0x10u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rbp.wrapping_sub(0x49u64), (xmm0) as u128); crate::simd_ops::wu8(rsp.wrapping_add(0x20u64), (0x0u8) as u8); r9 = rbp.wrapping_sub(0x49); r8 = P20; rdx = rbx; rcx = rbp.wrapping_add(0x67); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P21, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r9 = 0x2; r8 = ((crate::simd_ops::ru16(rbp.wrapping_add(0x67u64))) as u32) as u32 as u64; rdx = (((rdi as u16)) as u32) as u32 as u64; rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P12, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rbp.wrapping_add(0x17); rcx = rbp.wrapping_sub(0x29); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P25, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rsi = (crate::simd_ops::ru64(rax)) as u64; rbx = ((crate::simd_ops::ru16(rax.wrapping_add(0xau64))) as u32) as u32 as u64; xmm0 = (crate::simd_ops::ru128(rax.wrapping_add(0x10u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rbp.wrapping_sub(0x49u64), (xmm0) as u128); crate::simd_ops::wu8(rsp.wrapping_add(0x20u64), (0x0u8) as u8); r9 = rbp.wrapping_sub(0x49); r8 = P20; rdx = rsi; rcx = rbp.wrapping_add(0x67); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P21, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r9 = 0x2; r8 = ((crate::simd_ops::ru16(rbp.wrapping_add(0x67u64))) as u32) as u32 as u64; rdx = (((rbx as u16)) as u32) as u32 as u64; rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P12, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r9 = 0x1; r8 = (((rdi as u16)) as u32) as u32 as u64; rdx = (((rbx as u16)) as u32) as u32 as u64; rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P12, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rbp.wrapping_add(0x17); rcx = rbp.wrapping_sub(0x29); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P26, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(rax)) as u64; r9 = 0x1; r8 = (((rbx as u16)) as u32) as u32 as u64; rdx = ((crate::simd_ops::ru16(rax.wrapping_add(0xau64))) as u32) as u32 as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P12, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r11 = rsp.wrapping_add(0xc0); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x40u64))) as u64; rdi = (crate::simd_ops::ru64(r11.wrapping_add(0x48u64))) as u64; rsp = r11; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14186fd70(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14240a730;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0; let mut ymm1_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), r8); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; r13 = r9; r12 = ((rdx as u32)) as u32 as u64; rbx = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; r10 = (crate::simd_ops::ru64(rcx)) as u64; rdi = ((rbx.wrapping_add(r10.wrapping_mul(8))) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), rdi); fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 7; continue; } bb = 1; continue; }
            1 => { rdx = 0; fl_l = 0; fl_r = 0; bb = 2; continue; }
            2 => { rbp = (crate::simd_ops::ru64(rbx)) as u64; rsi = rbp; rsi = ((rsi).wrapping_shr(32u32)) as u64; fl_l = ((rbp as u32)) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 6; continue; } bb = 3; continue; }
            3 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; bb = 4; continue; }
            4 => { rax = ((rbp as u32)) as u32 as u64; r15 = (((rax) as i64).wrapping_mul((0x198u64) as i64) as u64) as u64; r15 = ((r15).wrapping_add(rdi)) as u64; rax = ((r12 as u32)) as u32 as u64; r14 = (((rax) as i64).wrapping_mul((0x198u64) as i64) as u64) as u64; r14 = ((r14).wrapping_add(r13)) as u64; r12 = (((r12 as u32)).wrapping_add(1)) as u32 as u64; rax = (crate::simd_ops::ru32(r15)) as u32 as u64; crate::simd_ops::wu32(r14, ((rax as u32)) as u32); rcx = r14.wrapping_add(0x4); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rcx); crate::simd_ops::lk_and_u32(rcx, (0xff000004u32) as u32); crate::simd_ops::lk_or_u32(rcx, (0x4u32) as u32); crate::simd_ops::wu8(r14.wrapping_add(0x7u64), (0x4u8) as u8); crate::simd_ops::wu32(r14.wrapping_add(0x8u64), ((rdx as u32)) as u32); crate::simd_ops::wu8(r14.wrapping_add(0xcu64), (0x0u8) as u8); rdx = r15.wrapping_add(0x4); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); let y_addr: u64 = r15.wrapping_add(0x2cu64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = r14.wrapping_add(0x2cu64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); let y_addr: u64 = r15.wrapping_add(0x4cu64); xmm1 = crate::simd_ops::ru128(y_addr); ymm1_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = r14.wrapping_add(0x4cu64); crate::simd_ops::wu128(y_addr, xmm1); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm1_hi); let y_addr: u64 = r15.wrapping_add(0x6cu64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = r14.wrapping_add(0x6cu64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); xmm1 = (crate::simd_ops::ru128(r15.wrapping_add(0x8cu64))) as u128; ymm1_hi = 0; crate::simd_ops::wu128(r14.wrapping_add(0x8cu64), (xmm1) as u128); xmm0 = ((crate::simd_ops::ru64(r15.wrapping_add(0x9cu64))) as u64) as u128; ymm0_hi = 0; crate::simd_ops::wu64(r14.wrapping_add(0x9cu64), ((xmm0 as u64)) as u64); rdx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(r14.wrapping_add(0xa8u64), rdx); crate::simd_ops::wu64(r14.wrapping_add(0xb0u64), rdx); rax = (crate::simd_ops::ru64(r15.wrapping_add(0xa8u64))) as u64; crate::simd_ops::wu64(r14.wrapping_add(0xa8u64), rax); rax = (crate::simd_ops::ru32(r15.wrapping_add(0xb0u64))) as u32 as u64; crate::simd_ops::wu32(r14.wrapping_add(0xb0u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(r15.wrapping_add(0xb4u64))) as u32 as u64; crate::simd_ops::wu32(r14.wrapping_add(0xb4u64), ((rax as u32)) as u32); crate::simd_ops::wu64(r15.wrapping_add(0xa8u64), rdx); crate::simd_ops::wu64(r15.wrapping_add(0xb0u64), rdx); crate::simd_ops::wu64(r14.wrapping_add(0xb8u64), rdx); crate::simd_ops::wu64(r14.wrapping_add(0xc0u64), rdx); rax = (crate::simd_ops::ru64(r15.wrapping_add(0xb8u64))) as u64; crate::simd_ops::wu64(r14.wrapping_add(0xb8u64), rax); rax = (crate::simd_ops::ru32(r15.wrapping_add(0xc0u64))) as u32 as u64; crate::simd_ops::wu32(r14.wrapping_add(0xc0u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(r15.wrapping_add(0xc4u64))) as u32 as u64; crate::simd_ops::wu32(r14.wrapping_add(0xc4u64), ((rax as u32)) as u32); crate::simd_ops::wu64(r15.wrapping_add(0xb8u64), rdx); crate::simd_ops::wu64(r15.wrapping_add(0xc0u64), rdx); rax = ((crate::simd_ops::ru8(r15.wrapping_add(0xc8u64))) as u32) as u32 as u64; crate::simd_ops::wu8(r14.wrapping_add(0xc8u64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(r15.wrapping_add(0xc9u64))) as u32) as u32 as u64; crate::simd_ops::wu8(r14.wrapping_add(0xc9u64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(r15.wrapping_add(0xcau64))) as u32) as u32 as u64; crate::simd_ops::wu8(r14.wrapping_add(0xcau64), ((rax as u8)) as u8); rax = (crate::simd_ops::ru32(r15.wrapping_add(0xccu64))) as u32 as u64; crate::simd_ops::wu32(r14.wrapping_add(0xccu64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(r15.wrapping_add(0xd0u64))) as u32 as u64; crate::simd_ops::wu32(r14.wrapping_add(0xd0u64), ((rax as u32)) as u32); xmm0 = (crate::simd_ops::ru128(r15.wrapping_add(0xd4u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(r14.wrapping_add(0xd4u64), (xmm0) as u128); rax = ((crate::simd_ops::ru8(r15.wrapping_add(0xe4u64))) as u32) as u32 as u64; crate::simd_ops::wu8(r14.wrapping_add(0xe4u64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(r15.wrapping_add(0xe5u64))) as u32) as u32 as u64; crate::simd_ops::wu8(r14.wrapping_add(0xe5u64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(r15.wrapping_add(0xe6u64))) as u32) as u32 as u64; crate::simd_ops::wu8(r14.wrapping_add(0xe6u64), ((rax as u8)) as u8); rcx = r14.wrapping_add(0xe8); rax = r15.wrapping_add(0xe8); xmm0 = (crate::simd_ops::ru128(rax)) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rcx, (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(rax.wrapping_add(0x10u64))) as u128; ymm1_hi = 0; crate::simd_ops::wu128(rcx.wrapping_add(0x10u64), (xmm1) as u128); xmm0 = (crate::simd_ops::ru128(rax.wrapping_add(0x20u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rcx.wrapping_add(0x20u64), (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(rax.wrapping_add(0x30u64))) as u128; ymm1_hi = 0; crate::simd_ops::wu128(rcx.wrapping_add(0x30u64), (xmm1) as u128); xmm0 = (crate::simd_ops::ru128(rax.wrapping_add(0x40u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rcx.wrapping_add(0x40u64), (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(rax.wrapping_add(0x50u64))) as u128; ymm1_hi = 0; crate::simd_ops::wu128(rcx.wrapping_add(0x50u64), (xmm1) as u128); xmm0 = (crate::simd_ops::ru128(rax.wrapping_add(0x60u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rcx.wrapping_add(0x60u64), (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(rax.wrapping_add(0x70u64))) as u128; ymm1_hi = 0; crate::simd_ops::wu128(rcx.wrapping_add(0x70u64), (xmm1) as u128); xmm0 = (crate::simd_ops::ru128(rax.wrapping_add(0x80u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rcx.wrapping_add(0x80u64), (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(rax.wrapping_add(0x90u64))) as u128; ymm1_hi = 0; crate::simd_ops::wu128(rcx.wrapping_add(0x90u64), (xmm1) as u128); crate::simd_ops::wu64(r14.wrapping_add(0x188u64), rdx); crate::simd_ops::wu64(r14.wrapping_add(0x190u64), rdx); rax = (crate::simd_ops::ru64(r15.wrapping_add(0x188u64))) as u64; crate::simd_ops::wu64(r14.wrapping_add(0x188u64), rax); rax = (crate::simd_ops::ru32(r15.wrapping_add(0x190u64))) as u32 as u64; crate::simd_ops::wu32(r14.wrapping_add(0x190u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(r15.wrapping_add(0x194u64))) as u32 as u64; crate::simd_ops::wu32(r14.wrapping_add(0x194u64), ((rax as u32)) as u32); crate::simd_ops::wu64(r15.wrapping_add(0x188u64), rdx); crate::simd_ops::wu64(r15.wrapping_add(0x190u64), rdx); rbp = (((rbp as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((rbp as u32)) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 4; continue; } bb = 5; continue; }
            5 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; bb = 6; continue; }
            6 => { rbx = ((rbx).wrapping_add(0x8u64)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 7; continue; }
            7 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1418700a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xb8; const P1: u64 = 0xd0; const P2: u64 = 0x1401de830; const P3: u64 = 0xfffffffffffffff0; const P4: u64 = 0x14021b7b0; const P5: u64 = 0xffffffffffffff4c; const P6: u64 = 0x14240ab10;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    fl_l = rdx; fl_r = rdx;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { return rax; }
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdi = rdx;
    rbx = ((rcx.wrapping_add(P0)) as u64) as u64;
    loop { rcx = ((rbx.wrapping_add(P1)) as u64) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = ((rbx.wrapping_add(P3)) as u64) as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rcx = ((rbx.wrapping_add(P5)) as u64) as u64; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rbx = ((rbx).wrapping_add(0x198u64)) as u64; fl_l = rdi; fl_r = (0x1u64) as u64; rdi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if !((((fl_l as u64)) != ((fl_r as u64)))) { break; } }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141870180(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143085238; const P1: u64 = 0x140226630; const P2: u64 = 0x1402264d0; const P3: u64 = 0x140226330;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), r8); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = ((rsp).wrapping_sub(0x48u64)) as u64; xmm1 = (crate::simd_ops::ru128(rcx)) as u128; rcx = (crate::simd_ops::ru64(rcx)) as u64; r13 = r9; rbx = (((xmm1 >> 64) as u64)) as u64; rax = (((xmm1 >> 64) as u64)) as u64; r12 = ((rdx as u32)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), rbx); rdi = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 8; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), r14); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), r15); bb = 2; continue; }
            2 => { rsi = (crate::simd_ops::ru64(rbx)) as u64; r15 = rsi; r15 = ((r15).wrapping_shr(32u32)) as u64; fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((r15 as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 6; continue; } bb = 3; continue; }
            3 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; rbx = P0; bb = 4; continue; }
            4 => { r14 = ((r12 as u32)) as u32 as u64; r12 = (((r12 as u32)).wrapping_add(1)) as u32 as u64; rbp = ((rsi as u32)) as u32 as u64; rbp = ((rbp).wrapping_shl(6u32)) as u64; r14 = ((r14).wrapping_shl(6u32)) as u64; rbp = ((rbp).wrapping_add(rdi)) as u64; r14 = ((r14).wrapping_add(r13)) as u64; rdx = rbp.wrapping_add(0x8); rcx = r14.wrapping_add(0x8); crate::simd_ops::wu64(r14, rbx); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = ((crate::simd_ops::ru8(rbp.wrapping_add(0x18u64))) as u32) as u32 as u64; rdx = rbp.wrapping_add(0x20); rcx = r14.wrapping_add(0x20); crate::simd_ops::wu8(r14.wrapping_add(0x18u64), ((rax as u8)) as u8); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rbp.wrapping_add(0x30); rcx = r14.wrapping_add(0x30); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rsi = (((rsi as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((r15 as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 4; continue; } bb = 5; continue; }
            5 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; bb = 6; continue; }
            6 => { rbx = ((rbx).wrapping_add(0x8u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), rbx); fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 7; continue; }
            7 => { r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; bb = 8; continue; }
            8 => { rsp = ((rsp).wrapping_add(0x48u64)) as u64; r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141870290(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143085238; const P1: u64 = 0x8; const P2: u64 = 0x140226630; const P3: u64 = 0x18; const P4: u64 = 0x20; const P5: u64 = 0x1402264d0; const P6: u64 = 0x30; const P7: u64 = 0x140226330;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = ((r8 as u32)) as u32 as u64;
    rax = P0;
    rdi = rcx;
    rbx = ((rbx).wrapping_shl(6u32)) as u64;
    rbx = ((rbx).wrapping_add(rdx)) as u64;
    rdx = ((rcx.wrapping_add(P1)) as u64) as u64;
    rcx = ((rbx.wrapping_add(P1)) as u64) as u64;
    crate::simd_ops::wu64(rbx, rax);
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    rax = ((crate::simd_ops::ru8(rdi.wrapping_add(P3))) as u32) as u32 as u64;
    rdx = ((rdi.wrapping_add(P4)) as u64) as u64;
    rcx = ((rbx.wrapping_add(P4)) as u64) as u64;
    crate::simd_ops::wu8(rbx.wrapping_add(P3), ((rax as u8)) as u8);
    rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp);
    rdx = ((rdi.wrapping_add(P6)) as u64) as u64;
    rcx = ((rbx.wrapping_add(P6)) as u64) as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(P7, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1418706f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1401ff5b0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), r8); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = ((rsp).wrapping_sub(0x38u64)) as u64; xmm1 = (crate::simd_ops::ru128(rcx)) as u128; rcx = (crate::simd_ops::ru64(rcx)) as u64; r13 = r9; rax = (((xmm1 >> 64) as u64)) as u64; rbx = (((xmm1 >> 64) as u64)) as u64; r12 = ((rdx as u32)) as u32 as u64; rdi = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rdi); fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 8; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), r14); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r15); bb = 2; continue; }
            2 => { rsi = (crate::simd_ops::ru64(rbx)) as u64; r15 = rsi; r15 = ((r15).wrapping_shr(32u32)) as u64; fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((r15 as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 6; continue; } bb = 3; continue; }
            3 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; bb = 4; continue; }
            4 => { rax = ((rsi as u32)) as u32 as u64; r14 = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((r12 as u32)) as u32 as u64; r14 = ((r14).wrapping_shl(4u32)) as u64; r12 = (((r12 as u32)).wrapping_add(1)) as u32 as u64; r14 = ((r14).wrapping_add(rdi)) as u64; rdx = r14; rbp = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rbp = ((rbp).wrapping_shl(4u32)) as u64; rbp = ((rbp).wrapping_add(r13)) as u64; rcx = rbp; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = r14.wrapping_add(0x10); rcx = rbp.wrapping_add(0x10); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = r14.wrapping_add(0x20); rcx = rbp.wrapping_add(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = r14.wrapping_add(0x30); rcx = rbp.wrapping_add(0x30); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = ((crate::simd_ops::ru8(r14.wrapping_add(0x40u64))) as u32) as u32 as u64; rsi = (((rsi as u32)).wrapping_add(1)) as u32 as u64; crate::simd_ops::wu8(rbp.wrapping_add(0x40u64), ((rax as u8)) as u8); rax = (crate::simd_ops::ru32(r14.wrapping_add(0x44u64))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x44u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(r14.wrapping_add(0x48u64))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x48u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(r14.wrapping_add(0x4cu64))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x4cu64), ((rax as u32)) as u32); fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((r15 as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 4; continue; } bb = 5; continue; }
            5 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; bb = 6; continue; }
            6 => { rbx = ((rbx).wrapping_add(0x8u64)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 7; continue; }
            7 => { r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; bb = 8; continue; }
            8 => { rsp = ((rsp).wrapping_add(0x38u64)) as u64; r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141870830(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x20; const P1: u64 = 0x10; const P2: u64 = 0x1401f2bc0; const P3: u64 = 0xfffffffffffffff0; const P4: u64 = 0xffffffffffffffe0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    fl_l = rdx; fl_r = rdx;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { return rax; }
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdi = rdx;
    rbx = ((rcx.wrapping_add(P0)) as u64) as u64;
    loop { rcx = ((rbx.wrapping_add(P1)) as u64) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = ((rbx.wrapping_add(P3)) as u64) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = ((rbx.wrapping_add(P4)) as u64) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rbx = ((rbx).wrapping_add(0x50u64)) as u64; fl_l = rdi; fl_r = (0x1u64) as u64; rdi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if !((((fl_l as u64)) != ((fl_r as u64)))) { break; } }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141870900(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x14240a730; const P2: u64 = 0x48; const P3: u64 = 0x50;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), r8); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; r13 = r9; r12 = ((rdx as u32)) as u32 as u64; rbx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; r10 = (crate::simd_ops::ru64(rcx)) as u64; rdi = ((rbx.wrapping_add(r10.wrapping_mul(8))) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), rdi); fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 7; continue; } bb = 1; continue; }
            1 => { bb = 2; continue; }
            2 => { rsi = (crate::simd_ops::ru64(rbx)) as u64; r15 = rsi; r15 = ((r15).wrapping_shr(32u32)) as u64; fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((r15 as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 6; continue; } bb = 3; continue; }
            3 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; bb = 4; continue; }
            4 => { rax = ((rsi as u32)) as u32 as u64; r14 = (((rax) as i64).wrapping_mul((0x54u64) as i64) as u64) as u64; r14 = ((r14).wrapping_add(rdi)) as u64; rax = ((r12 as u32)) as u32 as u64; rbp = (((rax) as i64).wrapping_mul((0x54u64) as i64) as u64) as u64; rbp = ((rbp).wrapping_add(r13)) as u64; r12 = (((r12 as u32)).wrapping_add(1)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbp); crate::simd_ops::lk_and_u32(rbp, (0xff000008u32) as u32); crate::simd_ops::lk_or_u32(rbp, (0x8u32) as u32); crate::simd_ops::wu8(rbp.wrapping_add(0x3u64), (0x8u8) as u8); crate::simd_ops::wu32(rbp.wrapping_add(0x4u64), (0x0u32) as u32); crate::simd_ops::wu8(rbp.wrapping_add(0x8u64), (0x0u8) as u8); rdx = r14; rcx = rbp; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(r14.wrapping_add(P2))) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x48u64), rax); rax = (crate::simd_ops::ru32(r14.wrapping_add(P3))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x50u64), ((rax as u32)) as u32); rsi = (((rsi as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((r15 as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 4; continue; } bb = 5; continue; }
            5 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; bb = 6; continue; }
            6 => { rbx = ((rbx).wrapping_add(0x8u64)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 7; continue; }
            7 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}
