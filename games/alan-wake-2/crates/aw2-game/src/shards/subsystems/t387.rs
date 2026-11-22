#![allow(unused_imports, unused_variables, unused_mut, unused_assignments, non_snake_case, dead_code, clippy::all, unreachable_code, unused_parens)]

#[inline(never)]
pub unsafe extern "win64" fn fn_1424a1410(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14333ccd8; const P1: u64 = 0x14183acda; const P2: u64 = 0x142646560; const P3: u64 = 0x1433dc26c; const P4: u64 = 0x14340f59c; const P5: u64 = 0x14183acc8; const P6: u64 = 0x1433dc270; const P7: u64 = 0x14340f868; const P8: u64 = 0x1426467c8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbp = rcx; rbx = rdx; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x18u64))) as u64; r8 = (crate::simd_ops::ru64(rbp.wrapping_add(0x20u64))) as u64; r8 = ((r8).wrapping_sub(rcx)) as u64; fl_l = r8; fl_r = (0x1u64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 7; continue; } bb = 1; continue; }
            1 => { rdx = P0; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 7; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rax = ((rax).wrapping_add(1)) as u64; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 6; continue; } bb = 3; continue; }
            3 => { rdx = ((rdx).wrapping_add(rdx)) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), rdx); fl_l = rdx; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 5; continue; } bb = 4; continue; }
            4 => { crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), rax); rdx = rax; bb = 5; continue; }
            5 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx, rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 40; continue; } bb = 6; continue; }
            6 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rax = ((crate::simd_ops::ru16(P3)) as u32) as u32 as u64; rcx = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu8(rdx.wrapping_add(rcx), ((rax as u8)) as u8); fl_l = (crate::simd_ops::lk_add_u64(rbx.wrapping_add(0x8u64), (1) as u64)) as u64; bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rax = ((rax).wrapping_add(1)) as u64; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 11; continue; } bb = 8; continue; }
            8 => { rdx = ((rdx).wrapping_add(rdx)) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), rdx); fl_l = rdx; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 10; continue; } bb = 9; continue; }
            9 => { crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), rax); rdx = rax; bb = 10; continue; }
            10 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx, rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 40; continue; } bb = 11; continue; }
            11 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rdi = rbx.wrapping_add(0x8); rax = ((crate::simd_ops::ru16(P3)) as u32) as u32 as u64; rdx = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu8(rdx.wrapping_add(rcx), ((rax as u8)) as u8); rdx = rbx; crate::simd_ops::lk_add_u64(rbx.wrapping_add(0x8u64), (1) as u64); rsi = (crate::simd_ops::ru64(rbp.wrapping_add(0x10u64))) as u64; rcx = rsi; rax = (crate::simd_ops::ru64(rsi)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x20u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru8(rsi.wrapping_add(0x9u64))) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 13; continue; } bb = 12; continue; }
            12 => { rax = (crate::simd_ops::ru64(rsi)) as u64; rdx = rbx; rcx = rsi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x28u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 13; continue; }
            13 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rax = ((rax).wrapping_add(0x2u64)) as u64; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 17; continue; } bb = 14; continue; }
            14 => { rdx = ((rdx).wrapping_add(rdx)) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), rdx); fl_l = rdx; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 16; continue; } bb = 15; continue; }
            15 => { crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), rax); rdi = rbx.wrapping_add(0x8); rdx = rax; bb = 16; continue; }
            16 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx, rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 40; continue; } bb = 17; continue; }
            17 => { rdx = (crate::simd_ops::ru64(rdi)) as u64; rsi = rbx.wrapping_add(0x10); rax = ((crate::simd_ops::ru16(P4)) as u32) as u32 as u64; r14 = rdi; rcx = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu16(rdx.wrapping_add(rcx), ((rax as u16)) as u16); crate::simd_ops::lk_add_u64(rdi, (0x2u64) as u64); r12 = (crate::simd_ops::ru64(rbp.wrapping_add(0x18u64))) as u64; r15 = (crate::simd_ops::ru64(rbp.wrapping_add(0x20u64))) as u64; fl_l = r15; fl_r = r12; r15 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 24; continue; } bb = 18; continue; }
            18 => { rcx = (crate::simd_ops::ru64(rdi)) as u64; rdx = (crate::simd_ops::ru64(rsi)) as u64; rcx = ((rcx).wrapping_add(r15)) as u64; fl_l = rcx; fl_r = rdx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 23; continue; } bb = 19; continue; }
            19 => { rdx = ((rdx).wrapping_add(rdx)) as u64; crate::simd_ops::wu64(rsi, rdx); fl_l = rdx; fl_r = rcx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 21; continue; } bb = 20; continue; }
            20 => { crate::simd_ops::wu64(rsi, rcx); r14 = rbx.wrapping_add(0x8); rdx = rcx; bb = 21; continue; }
            21 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx, rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 40; continue; } bb = 22; continue; }
            22 => { rsi = rbx.wrapping_add(0x10); bb = 23; continue; }
            23 => { rcx = (crate::simd_ops::ru64(rdi)) as u64; r8 = r15; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rbx))) as u64; rdx = r12; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::lk_add_u64(r14, (r15) as u64)) as u64; fl_r = fl_l; rdi = r14; bb = 24; continue; }
            24 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rdi = r14; rdx = (crate::simd_ops::ru64(rsi)) as u64; rax = ((rax).wrapping_add(0x2u64)) as u64; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 28; continue; } bb = 25; continue; }
            25 => { rdx = ((rdx).wrapping_add(rdx)) as u64; crate::simd_ops::wu64(rsi, rdx); fl_l = rdx; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 27; continue; } bb = 26; continue; }
            26 => { crate::simd_ops::wu64(rsi, rax); rdi = rbx.wrapping_add(0x8); rdx = rax; bb = 27; continue; }
            27 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx, rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 40; continue; } bb = 28; continue; }
            28 => { rcx = (crate::simd_ops::ru64(r14)) as u64; rax = ((crate::simd_ops::ru16(P6)) as u32) as u32 as u64; rdx = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu16(rdx.wrapping_add(rcx), ((rax as u16)) as u16); rdx = rbx; crate::simd_ops::lk_add_u64(rdi, (0x2u64) as u64); rsi = (crate::simd_ops::ru64(rbp.wrapping_add(0x28u64))) as u64; rcx = rsi; rax = (crate::simd_ops::ru64(rsi)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x20u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru8(rsi.wrapping_add(0x9u64))) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 30; continue; } bb = 29; continue; }
            29 => { rax = (crate::simd_ops::ru64(rsi)) as u64; rdx = rbx; rcx = rsi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x28u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 30; continue; }
            30 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rax = ((rax).wrapping_add(1)) as u64; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 34; continue; } bb = 31; continue; }
            31 => { rdx = ((rdx).wrapping_add(rdx)) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), rdx); fl_l = rdx; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 33; continue; } bb = 32; continue; }
            32 => { crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), rax); rdi = rbx.wrapping_add(0x8); rdx = rax; bb = 33; continue; }
            33 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx, rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 40; continue; } bb = 34; continue; }
            34 => { r8 = (crate::simd_ops::ru64(rdi)) as u64; rcx = (crate::simd_ops::ru64(rbx)) as u64; rax = ((crate::simd_ops::ru16(P7)) as u32) as u32 as u64; crate::simd_ops::wu8(r8.wrapping_add(rcx), ((rax as u8)) as u8); crate::simd_ops::lk_add_u64(rdi, (1) as u64); rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x18u64))) as u64; r8 = (crate::simd_ops::ru64(rbp.wrapping_add(0x20u64))) as u64; r8 = ((r8).wrapping_sub(rcx)) as u64; fl_l = r8; fl_r = (0x1u64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 42; continue; } bb = 35; continue; }
            35 => { rdx = P0; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 42; continue; } bb = 36; continue; }
            36 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rax = ((rax).wrapping_add(1)) as u64; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 41; continue; } bb = 37; continue; }
            37 => { rdx = ((rdx).wrapping_add(rdx)) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), rdx); fl_l = rdx; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 39; continue; } bb = 38; continue; }
            38 => { crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), rax); rdx = rax; bb = 39; continue; }
            39 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx, rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 41; continue; } bb = 40; continue; }
            40 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P8), rax), rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x1424a16e6, rcx, rsp); return rax; }
            41 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rax = ((crate::simd_ops::ru16(P7)) as u32) as u32 as u64; rcx = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu8(rdx.wrapping_add(rcx), ((rax as u8)) as u8); fl_l = (crate::simd_ops::lk_add_u64(rbx.wrapping_add(0x8u64), (1) as u64)) as u64; bb = 42; continue; }
            42 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1424a1770(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1424a6050; const P1: u64 = 0x10; const P2: u64 = 0x1424a5d60;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdi = rdx;
    rbx = rcx;
    rcx = rdi;
    rdx = (rdx & !0xffu64) | (((0x5bu8) as u8 as u64));
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rcx = ((rbx.wrapping_add(P1)) as u64) as u64;
    rdx = rdi;
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    rdx = (rdx & !0xffu64) | (((0x5du8) as u8 as u64));
    rcx = rdi;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(P0, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1424a17f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142646560; const P1: u64 = 0x14340f570; const P2: u64 = 0x14183acc8; const P3: u64 = 0x14340f5a0; const P4: u64 = 0x1424a5d60; const P5: u64 = 0x1426467c8; const P6: u64 = 0x14340f868;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; rdi = rdx.wrapping_add(0x10); rbx = rdx; rax = ((rax).wrapping_add(0x7u64)) as u64; rdx = (crate::simd_ops::ru64(rdi)) as u64; r15 = rcx; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 5; continue; } bb = 1; continue; }
            1 => { rdx = ((rdx).wrapping_add(rdx)) as u64; crate::simd_ops::wu64(rdi, rdx); fl_l = rdx; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 3; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rdi, rax); rdx = rax; bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx, rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 21; continue; } bb = 4; continue; }
            4 => { rdi = rbx.wrapping_add(0x10); bb = 5; continue; }
            5 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rdx = P1; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rbx))) as u64; rsi = rbx.wrapping_add(0x8); r8 = 0x7; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::lk_add_u64(rbx.wrapping_add(0x8u64), (0x7u64) as u64); r14 = rsi; r12 = (crate::simd_ops::ru64(r15.wrapping_add(0x20u64))) as u64; rbp = (crate::simd_ops::ru64(r15.wrapping_add(0x28u64))) as u64; fl_l = rbp; fl_r = r12; rbp = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 12; continue; } bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru64(rsi)) as u64; rdx = (crate::simd_ops::ru64(rdi)) as u64; rcx = ((rcx).wrapping_add(rbp)) as u64; fl_l = rcx; fl_r = rdx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 11; continue; } bb = 7; continue; }
            7 => { rdx = ((rdx).wrapping_add(rdx)) as u64; crate::simd_ops::wu64(rdi, rdx); fl_l = rdx; fl_r = rcx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 9; continue; } bb = 8; continue; }
            8 => { crate::simd_ops::wu64(rdi, rcx); r14 = rbx.wrapping_add(0x8); rdx = rcx; bb = 9; continue; }
            9 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx, rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 21; continue; } bb = 10; continue; }
            10 => { rdi = rbx.wrapping_add(0x10); bb = 11; continue; }
            11 => { rcx = (crate::simd_ops::ru64(rsi)) as u64; r8 = rbp; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rbx))) as u64; rdx = r12; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::lk_add_u64(r14, (rbp) as u64)) as u64; fl_r = fl_l; rsi = r14; bb = 12; continue; }
            12 => { rax = (crate::simd_ops::ru64(rsi)) as u64; rcx = rdi; rax = ((rax).wrapping_add(0x2u64)) as u64; rsi = r14; rdx = (crate::simd_ops::ru64(rcx)) as u64; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 17; continue; } bb = 13; continue; }
            13 => { rdx = ((rdx).wrapping_add(rdx)) as u64; crate::simd_ops::wu64(rcx, rdx); fl_l = rdx; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 15; continue; } bb = 14; continue; }
            14 => { crate::simd_ops::wu64(rcx, rax); rsi = rbx.wrapping_add(0x8); rdx = rax; bb = 15; continue; }
            15 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx, rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 21; continue; } bb = 16; continue; }
            16 => { rdi = rbx.wrapping_add(0x10); bb = 17; continue; }
            17 => { rdx = (crate::simd_ops::ru64(r14)) as u64; rcx = (crate::simd_ops::ru64(rbx)) as u64; rax = ((crate::simd_ops::ru16(P3)) as u32) as u32 as u64; crate::simd_ops::wu16(rdx.wrapping_add(rcx), ((rax as u16)) as u16); rcx = r15.wrapping_add(0x10); crate::simd_ops::lk_add_u64(rsi, (0x2u64) as u64); rdx = rbx; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rsi)) as u64; rbp = rsi; rdx = (crate::simd_ops::ru64(rdi)) as u64; rax = ((rax).wrapping_add(1)) as u64; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 22; continue; } bb = 18; continue; }
            18 => { rdx = ((rdx).wrapping_add(rdx)) as u64; crate::simd_ops::wu64(rdi, rdx); fl_l = rdx; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 20; continue; } bb = 19; continue; }
            19 => { crate::simd_ops::wu64(rdi, rax); rbp = rbx.wrapping_add(0x8); rdx = rax; bb = 20; continue; }
            20 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx, rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 22; continue; } bb = 21; continue; }
            21 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x1424a1974, rcx, rsp); return rax; }
            22 => { rdx = (crate::simd_ops::ru64(rsi)) as u64; rcx = (crate::simd_ops::ru64(rbx)) as u64; rax = ((crate::simd_ops::ru16(P6)) as u32) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; crate::simd_ops::wu8(rdx.wrapping_add(rcx), ((rax as u8)) as u8); crate::simd_ops::lk_add_u64(rbp, (1) as u64); rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1424a19f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142646560; const P1: u64 = 0x14340f580; const P2: u64 = 0x14183acc8; const P3: u64 = 0x1426467c8; const P4: u64 = 0x14340f578;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; rdi = rdx.wrapping_add(0x10); rbx = rdx; rax = ((rax).wrapping_add(0x8u64)) as u64; rdx = (crate::simd_ops::ru64(rdi)) as u64; rbp = rcx; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 5; continue; } bb = 1; continue; }
            1 => { rdx = ((rdx).wrapping_add(rdx)) as u64; crate::simd_ops::wu64(rdi, rdx); fl_l = rdx; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 3; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rdi, rax); rdx = rax; bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx, rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 16; continue; } bb = 4; continue; }
            4 => { rdi = rbx.wrapping_add(0x10); bb = 5; continue; }
            5 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; r14 = rbx.wrapping_add(0x8); rax = (crate::simd_ops::ru64(P1)) as u64; rsi = r14; rcx = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu64(rdx.wrapping_add(rcx), rax); crate::simd_ops::lk_add_u64(rbx.wrapping_add(0x8u64), (0x8u64) as u64); r15 = (crate::simd_ops::ru64(rbp.wrapping_add(0x10u64))) as u64; rbp = (crate::simd_ops::ru64(rbp.wrapping_add(0x18u64))) as u64; fl_l = rbp; fl_r = r15; rbp = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 12; continue; } bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru64(r14)) as u64; rdx = (crate::simd_ops::ru64(rdi)) as u64; rcx = ((rcx).wrapping_add(rbp)) as u64; fl_l = rcx; fl_r = rdx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 11; continue; } bb = 7; continue; }
            7 => { rdx = ((rdx).wrapping_add(rdx)) as u64; crate::simd_ops::wu64(rdi, rdx); fl_l = rdx; fl_r = rcx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 9; continue; } bb = 8; continue; }
            8 => { crate::simd_ops::wu64(rdi, rcx); rsi = rbx.wrapping_add(0x8); rdx = rcx; bb = 9; continue; }
            9 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx, rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 16; continue; } bb = 10; continue; }
            10 => { rdi = rbx.wrapping_add(0x10); bb = 11; continue; }
            11 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; r8 = rbp; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(r14))) as u64; rdx = r15; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::lk_add_u64(rsi, (rbp) as u64)) as u64; fl_r = fl_l; r14 = rsi; bb = 12; continue; }
            12 => { rax = (crate::simd_ops::ru64(r14)) as u64; rbp = rsi; rdx = (crate::simd_ops::ru64(rdi)) as u64; rax = ((rax).wrapping_add(1)) as u64; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 17; continue; } bb = 13; continue; }
            13 => { rdx = ((rdx).wrapping_add(rdx)) as u64; crate::simd_ops::wu64(rdi, rdx); fl_l = rdx; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 15; continue; } bb = 14; continue; }
            14 => { crate::simd_ops::wu64(rdi, rax); rbp = rbx.wrapping_add(0x8); rdx = rax; bb = 15; continue; }
            15 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx, rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 17; continue; } bb = 16; continue; }
            16 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x1424a1b03, rcx, rsp); return rax; }
            17 => { rdx = (crate::simd_ops::ru64(rsi)) as u64; rcx = (crate::simd_ops::ru64(rbx)) as u64; rax = ((crate::simd_ops::ru16(P4)) as u32) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; crate::simd_ops::wu8(rdx.wrapping_add(rcx), ((rax as u8)) as u8); crate::simd_ops::lk_add_u64(rbp, (1) as u64); rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1424a1b80(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x142646560; const P3: u64 = 0x1426467c8; const P4: u64 = 0x14340f58c; const P5: u64 = 0x20;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; rbx = rdx; rdx = (crate::simd_ops::ru64(rdx.wrapping_add(P1))) as u64; rax = ((rax).wrapping_add(1)) as u64; rdi = rcx; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 5; continue; } bb = 1; continue; }
            1 => { rdx = ((rdx).wrapping_add(rdx)) as u64; crate::simd_ops::wu64(rbx.wrapping_add(P1), rdx); fl_l = rdx; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 3; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rbx.wrapping_add(P1), rax); rdx = rax; bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx, rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x1424a1bca, rcx, rsp); return rax; }
            5 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64; rax = ((crate::simd_ops::ru16(P4)) as u32) as u32 as u64; rcx = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu8(rdx.wrapping_add(rcx), ((rax as u8)) as u8); rdx = rbx; crate::simd_ops::lk_add_u64(rbx.wrapping_add(P0), (1) as u64); rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P1))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P5))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1424a1c40(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142646560; const P1: u64 = 0x1426467c8; const P2: u64 = 0x14340f58c; const P3: u64 = 0x14183acc8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0x18u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; rbx = rdx; rdi = rcx; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 7; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; rdx = (crate::simd_ops::ru64(rdx.wrapping_add(0x10u64))) as u64; rax = ((rax).wrapping_add(1)) as u64; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 6; continue; } bb = 2; continue; }
            2 => { rdx = ((rdx).wrapping_add(rdx)) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), rdx); fl_l = rdx; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), rax); rdx = rax; bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx, rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x1424a1c90, rcx, rsp); return rax; }
            6 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rax = ((crate::simd_ops::ru16(P2)) as u32) as u32 as u64; rdx = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu8(rdx.wrapping_add(rcx), ((rax as u8)) as u8); fl_l = (crate::simd_ops::lk_add_u64(rbx.wrapping_add(0x8u64), (1) as u64)) as u64; bb = 7; continue; }
            7 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; rdx = rsp.wrapping_add(0x20); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rsi); rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x30u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rsi = (crate::simd_ops::ru64(rax)) as u64; rdi = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; fl_l = rdi; fl_r = rsi; rdi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 14; continue; } bb = 8; continue; }
            8 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rcx = ((rcx).wrapping_add(rdi)) as u64; fl_l = rcx; fl_r = rdx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 13; continue; } bb = 9; continue; }
            9 => { rdx = ((rdx).wrapping_add(rdx)) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), rdx); fl_l = rdx; fl_r = rcx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 11; continue; } bb = 10; continue; }
            10 => { crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), rcx); rdx = rcx; bb = 11; continue; }
            11 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx, rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 13; continue; } bb = 12; continue; }
            12 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x1424a1d00, rcx, rsp); return rax; }
            13 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; r8 = rdi; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rbx))) as u64; rdx = rsi; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::lk_add_u64(rbx.wrapping_add(0x8u64), (rdi) as u64)) as u64; fl_r = fl_l; bb = 14; continue; }
            14 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1424a1d70(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x140000000; const P1: u64 = 0x14340f750; const P2: u64 = 0x14340f75e; const P3: u64 = 0x14340f738; const P4: u64 = 0x14340f749; const P5: u64 = 0x14340f658; const P6: u64 = 0x14340f663; const P7: u64 = 0x14340f648; const P8: u64 = 0x14340f654; const P9: u64 = 0x14340f638; const P10: u64 = 0x14340f644; const P11: u64 = 0x14340f628; const P12: u64 = 0x14340f635; const P13: u64 = 0x1424a60c0;
    let (mut rax, mut fl_l, mut fl_r, mut j_tgt) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x38u64)) as u64; rax = (((crate::simd_ops::ru32(rcx.wrapping_add(0x10u64))) as i32 as i64) as u64) as u64; r8 = (rdx) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x5u32) as u32 as u64; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 9; continue; } bb = 1; continue; }
            1 => { rdx = ((P0) as u64) as u64; rcx = (crate::simd_ops::ru32(rdx.wrapping_add(rax.wrapping_mul(4)).wrapping_add(0x24a1e38u64))) as u32 as u64; fl_l = ((rcx).wrapping_add(rdx)) as u64 as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; j_tgt = (rcx) as u64; bb = 0xffff_fffe; continue; }
            2 => { rax = ((P1) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rax) as u64); rax = ((P2) as u64) as u64; bb = 8; continue; }
            3 => { rax = ((P3) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rax) as u64); rax = ((P4) as u64) as u64; bb = 8; continue; }
            4 => { rax = ((P5) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rax) as u64); rax = ((P6) as u64) as u64; bb = 8; continue; }
            5 => { rax = ((P7) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rax) as u64); rax = ((P8) as u64) as u64; bb = 8; continue; }
            6 => { rax = ((P9) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rax) as u64); rax = ((P10) as u64) as u64; bb = 8; continue; }
            7 => { rax = ((P11) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rax) as u64); rax = ((P12) as u64) as u64; bb = 8; continue; }
            8 => { crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (rax) as u64); rdx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; rcx = (r8) as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P13) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 9; continue; }
            9 => { rsp = ((rsp).wrapping_add(0x38u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            0xffff_fffe => { match j_tgt.wrapping_sub(0x1424a1d70u64) { 0x14 => { bb = 1; continue; } 0x27 => { bb = 2; continue; } 0x3c => { bb = 3; continue; } 0x51 => { bb = 4; continue; } 0x66 => { bb = 5; continue; } 0x7b => { bb = 6; continue; } 0x90 => { bb = 7; continue; } 0xa3 => { bb = 8; continue; } 0xc1 => { bb = 9; continue; } _ => { let s5 = crate::simd_ops::read_s5_tail(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x40) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(j_tgt, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::set_xmm_ret(c_x0); return c_rax; } } }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1424a1e60(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x140000000; const P1: u64 = 0x14340f7a8; const P2: u64 = 0x14340f7b1; const P3: u64 = 0x14340f798; const P4: u64 = 0x14340f7a4; const P5: u64 = 0x14336adcc; const P6: u64 = 0x14336add2; const P7: u64 = 0x14340f5b0; const P8: u64 = 0x14340f5b7; const P9: u64 = 0x14340f5a8; const P10: u64 = 0x14340f5af; const P11: u64 = 0x14340f870; const P12: u64 = 0x14340f878; const P13: u64 = 0x14340f668; const P14: u64 = 0x14340f67d;
    let (mut rax, mut fl_l, mut fl_r, mut j_tgt) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = (((crate::simd_ops::ru32(rcx.wrapping_add(0x10u64))) as i32 as i64) as u64) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x5u32) as u32 as u64; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 8; continue; } bb = 1; continue; }
            1 => { r8 = ((P0) as u64) as u64; rcx = (crate::simd_ops::ru32(r8.wrapping_add(rax.wrapping_mul(4)).wrapping_add(0x24a1f30u64))) as u32 as u64; fl_l = ((rcx).wrapping_add(r8)) as u64 as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; j_tgt = (rcx) as u64; bb = 0xffff_fffe; continue; }
            2 => { rax = ((P1) as u64) as u64; crate::simd_ops::wu64(rdx, (rax) as u64); rax = ((P2) as u64) as u64; crate::simd_ops::wu64(rdx.wrapping_add(0x8u64), (rax) as u64); rax = (rdx) as u64; return rax; }
            3 => { rax = ((P3) as u64) as u64; crate::simd_ops::wu64(rdx, (rax) as u64); rax = ((P4) as u64) as u64; crate::simd_ops::wu64(rdx.wrapping_add(0x8u64), (rax) as u64); rax = (rdx) as u64; return rax; }
            4 => { rax = ((P5) as u64) as u64; crate::simd_ops::wu64(rdx, (rax) as u64); rax = ((P6) as u64) as u64; crate::simd_ops::wu64(rdx.wrapping_add(0x8u64), (rax) as u64); rax = (rdx) as u64; return rax; }
            5 => { rax = ((P7) as u64) as u64; crate::simd_ops::wu64(rdx, (rax) as u64); rax = ((P8) as u64) as u64; crate::simd_ops::wu64(rdx.wrapping_add(0x8u64), (rax) as u64); rax = (rdx) as u64; return rax; }
            6 => { rax = ((P9) as u64) as u64; crate::simd_ops::wu64(rdx, (rax) as u64); rax = ((P10) as u64) as u64; crate::simd_ops::wu64(rdx.wrapping_add(0x8u64), (rax) as u64); rax = (rdx) as u64; return rax; }
            7 => { rax = ((P11) as u64) as u64; crate::simd_ops::wu64(rdx, (rax) as u64); rax = ((P12) as u64) as u64; crate::simd_ops::wu64(rdx.wrapping_add(0x8u64), (rax) as u64); rax = (rdx) as u64; return rax; }
            8 => { rax = ((P13) as u64) as u64; crate::simd_ops::wu64(rdx, (rax) as u64); rax = ((P14) as u64) as u64; crate::simd_ops::wu64(rdx.wrapping_add(0x8u64), (rax) as u64); rax = (rdx) as u64; return rax; }
            0xffff_fffe => { match j_tgt.wrapping_sub(0x1424a1e60u64) { 0xd => { bb = 1; continue; } 0x21 => { bb = 2; continue; } 0x3a => { bb = 3; continue; } 0x53 => { bb = 4; continue; } 0x6c => { bb = 5; continue; } 0x85 => { bb = 6; continue; } 0x9e => { bb = 7; continue; } 0xb7 => { bb = 8; continue; } _ => { crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(j_tgt, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); } } }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1424a1f90(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x140000000; const P1: u64 = 0x14340f750; const P2: u64 = 0x14340f75e; const P3: u64 = 0x14340f738; const P4: u64 = 0x14340f749; const P5: u64 = 0x14340f6f0; const P6: u64 = 0x14340f736; const P7: u64 = 0x14340f6b0; const P8: u64 = 0x14340f6e1; const P9: u64 = 0x14340f5f0; const P10: u64 = 0x14340f621; const P11: u64 = 0x14340f5b8; const P12: u64 = 0x14340f5ea; const P13: u64 = 0x1424a60c0;
    let (mut rax, mut fl_l, mut fl_r, mut j_tgt) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x38u64)) as u64; rax = (((crate::simd_ops::ru32(rcx.wrapping_add(0x10u64))) as i32 as i64) as u64) as u64; r8 = (rdx) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x5u32) as u32 as u64; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 9; continue; } bb = 1; continue; }
            1 => { rdx = ((P0) as u64) as u64; rcx = (crate::simd_ops::ru32(rdx.wrapping_add(rax.wrapping_mul(4)).wrapping_add(0x24a2058u64))) as u32 as u64; fl_l = ((rcx).wrapping_add(rdx)) as u64 as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; j_tgt = (rcx) as u64; bb = 0xffff_fffe; continue; }
            2 => { rax = ((P1) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rax) as u64); rax = ((P2) as u64) as u64; bb = 8; continue; }
            3 => { rax = ((P3) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rax) as u64); rax = ((P4) as u64) as u64; bb = 8; continue; }
            4 => { rax = ((P5) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rax) as u64); rax = ((P6) as u64) as u64; bb = 8; continue; }
            5 => { rax = ((P7) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rax) as u64); rax = ((P8) as u64) as u64; bb = 8; continue; }
            6 => { rax = ((P9) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rax) as u64); rax = ((P10) as u64) as u64; bb = 8; continue; }
            7 => { rax = ((P11) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rax) as u64); rax = ((P12) as u64) as u64; bb = 8; continue; }
            8 => { crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (rax) as u64); rdx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; rcx = (r8) as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P13) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 9; continue; }
            9 => { rsp = ((rsp).wrapping_add(0x38u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            0xffff_fffe => { match j_tgt.wrapping_sub(0x1424a1f90u64) { 0x14 => { bb = 1; continue; } 0x27 => { bb = 2; continue; } 0x3c => { bb = 3; continue; } 0x51 => { bb = 4; continue; } 0x66 => { bb = 5; continue; } 0x7b => { bb = 6; continue; } 0x90 => { bb = 7; continue; } 0xa3 => { bb = 8; continue; } 0xc1 => { bb = 9; continue; } _ => { let s5 = crate::simd_ops::read_s5_tail(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x40) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(j_tgt, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::set_xmm_ret(c_x0); return c_rax; } } }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1424a2080(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x140000000; const P1: u64 = 0x14340f7a8; const P2: u64 = 0x14340f7b1; const P3: u64 = 0x14340f798; const P4: u64 = 0x14340f7a4; const P5: u64 = 0x14340f6a0; const P6: u64 = 0x14340f6ad; const P7: u64 = 0x14340f690; const P8: u64 = 0x14340f69d; const P9: u64 = 0x14340f680; const P10: u64 = 0x14340f68e; const P11: u64 = 0x14340f668; const P12: u64 = 0x14340f67d;
    let (mut rax, mut fl_l, mut fl_r, mut j_tgt) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = (((crate::simd_ops::ru32(rcx.wrapping_add(0x10u64))) as i32 as i64) as u64) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x5u32) as u32 as u64; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 7; continue; } bb = 1; continue; }
            1 => { r8 = ((P0) as u64) as u64; rcx = (crate::simd_ops::ru32(r8.wrapping_add(rax.wrapping_mul(4)).wrapping_add(0x24a2138u64))) as u32 as u64; fl_l = ((rcx).wrapping_add(r8)) as u64 as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; j_tgt = (rcx) as u64; bb = 0xffff_fffe; continue; }
            2 => { rax = ((P1) as u64) as u64; crate::simd_ops::wu64(rdx, (rax) as u64); rax = ((P2) as u64) as u64; crate::simd_ops::wu64(rdx.wrapping_add(0x8u64), (rax) as u64); rax = (rdx) as u64; return rax; }
            3 => { rax = ((P3) as u64) as u64; crate::simd_ops::wu64(rdx, (rax) as u64); rax = ((P4) as u64) as u64; crate::simd_ops::wu64(rdx.wrapping_add(0x8u64), (rax) as u64); rax = (rdx) as u64; return rax; }
            4 => { rax = ((P5) as u64) as u64; crate::simd_ops::wu64(rdx, (rax) as u64); rax = ((P6) as u64) as u64; crate::simd_ops::wu64(rdx.wrapping_add(0x8u64), (rax) as u64); rax = (rdx) as u64; return rax; }
            5 => { rax = ((P7) as u64) as u64; crate::simd_ops::wu64(rdx, (rax) as u64); rax = ((P8) as u64) as u64; crate::simd_ops::wu64(rdx.wrapping_add(0x8u64), (rax) as u64); rax = (rdx) as u64; return rax; }
            6 => { rax = ((P9) as u64) as u64; crate::simd_ops::wu64(rdx, (rax) as u64); rax = ((P10) as u64) as u64; crate::simd_ops::wu64(rdx.wrapping_add(0x8u64), (rax) as u64); rax = (rdx) as u64; return rax; }
            7 => { rax = ((P11) as u64) as u64; crate::simd_ops::wu64(rdx, (rax) as u64); rax = ((P12) as u64) as u64; crate::simd_ops::wu64(rdx.wrapping_add(0x8u64), (rax) as u64); rax = (rdx) as u64; return rax; }
            0xffff_fffe => { match j_tgt.wrapping_sub(0x1424a2080u64) { 0xd => { bb = 1; continue; } 0x21 => { bb = 2; continue; } 0x3a => { bb = 3; continue; } 0x53 => { bb = 4; continue; } 0x6c => { bb = 5; continue; } 0x85 => { bb = 6; continue; } 0x9e => { bb = 7; continue; } _ => { crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(j_tgt, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); } } }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1424a21a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x142646560; const P3: u64 = 0x1426467c8; const P4: u64 = 0x14340f7b4; const P5: u64 = 0x14183acc8; const P6: u64 = 0x20; const P7: u64 = 0x9; const P8: u64 = 0x28;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; rbx = rdx; rdx = (crate::simd_ops::ru64(rdx.wrapping_add(P1))) as u64; rax = ((rax).wrapping_add(0x5u64)) as u64; rdi = rcx; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 5; continue; } bb = 1; continue; }
            1 => { rdx = ((rdx).wrapping_add(rdx)) as u64; crate::simd_ops::wu64(rbx.wrapping_add(P1), rdx); fl_l = rdx; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 3; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rbx.wrapping_add(P1), rax); rdx = rax; bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx, rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x1424a21eb, rcx, rsp); return rax; }
            5 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64; rdx = P4; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rbx))) as u64; r8 = 0x5; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::lk_add_u64(rbx.wrapping_add(P0), (0x5u64) as u64); rdx = rbx; rdi = (crate::simd_ops::ru64(rdi.wrapping_add(P1))) as u64; rcx = rdi; rax = (crate::simd_ops::ru64(rdi)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P6))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(P7))) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rdx = rbx; rcx = rdi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P8))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 7; continue; }
            7 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1424a22b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x142646560; const P3: u64 = 0x1426467c8; const P4: u64 = 0x14340f76c; const P5: u64 = 0x20; const P6: u64 = 0x9; const P7: u64 = 0x28;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; rbx = rdx; rdx = (crate::simd_ops::ru64(rdx.wrapping_add(P1))) as u64; rax = ((rax).wrapping_add(0x2u64)) as u64; rdi = rcx; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 5; continue; } bb = 1; continue; }
            1 => { rdx = ((rdx).wrapping_add(rdx)) as u64; crate::simd_ops::wu64(rbx.wrapping_add(P1), rdx); fl_l = rdx; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 3; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rbx.wrapping_add(P1), rax); rdx = rax; bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx, rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x1424a22fb, rcx, rsp); return rax; }
            5 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64; rax = ((crate::simd_ops::ru16(P4)) as u32) as u32 as u64; rcx = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu16(rdx.wrapping_add(rcx), ((rax as u16)) as u16); rdx = rbx; crate::simd_ops::lk_add_u64(rbx.wrapping_add(P0), (0x2u64) as u64); rdi = (crate::simd_ops::ru64(rdi.wrapping_add(P1))) as u64; rcx = rdi; rax = (crate::simd_ops::ru64(rdi)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P5))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(P6))) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rdx = rbx; rcx = rdi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P7))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 7; continue; }
            7 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1424a23c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x20; const P2: u64 = 0x9; const P3: u64 = 0x28; const P4: u64 = 0x18;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rsi = rcx;
    rcx = rbx;
    rdi = rdx;
    rax = (crate::simd_ops::ru64(rbx)) as u64;
    rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp);
    fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(P2))) as u8 as u64; fl_r = (0x1u8) as u8 as u64;
    if !((((fl_l as u8)) == ((fl_r as u8)))) { rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = rdi; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); }
    rbx = (crate::simd_ops::ru64(rsi.wrapping_add(P4))) as u64;
    rdx = rdi;
    rcx = rbx;
    rax = (crate::simd_ops::ru64(rbx)) as u64;
    rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp);
    fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(P2))) as u8 as u64; fl_r = (0x1u8) as u8 as u64;
    if !((((fl_l as u8)) == ((fl_r as u8)))) { rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = rdi; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1424a24a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x20; const P1: u64 = 0x18; const P2: u64 = 0x28;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x30u64)) as u64;
    rdi = ((rcx.wrapping_add(P0)) as u64) as u64;
    fl_l = (crate::simd_ops::ru8(rdi)) as u8 as u64; fl_r = (0x0u8) as u8 as u64;
    if !((((fl_l as u8)) != ((fl_r as u8)))) { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rbx = ((crate::simd_ops::ru8(rdi)) as u32) as u32 as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x28u64), ((rbx as u8)) as u8); crate::simd_ops::wu8(rsp.wrapping_add(0x29u64), (0x1u8) as u8); crate::simd_ops::wu8(rdi, (0x1u8) as u8); rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu8(rdi, ((rbx as u8)) as u8); }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64;
    rsp = ((rsp).wrapping_add(0x30u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1424a24f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x20; const P1: u64 = 0x18;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x30u64)) as u64;
    rdi = ((rcx.wrapping_add(P0)) as u64) as u64;
    fl_l = (crate::simd_ops::ru8(rdi)) as u8 as u64; fl_r = (0x0u8) as u8 as u64;
    if !((((fl_l as u8)) != ((fl_r as u8)))) { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rbx = ((crate::simd_ops::ru8(rdi)) as u32) as u32 as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x28u64), ((rbx as u8)) as u8); crate::simd_ops::wu8(rsp.wrapping_add(0x29u64), (0x1u8) as u8); crate::simd_ops::wu8(rdi, (0x1u8) as u8); rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu8(rdi, ((rbx as u8)) as u8); }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64;
    rsp = ((rsp).wrapping_add(0x30u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1424a2540(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x20; const P1: u64 = 0x18;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x30u64)) as u64;
    rdi = ((rcx.wrapping_add(P0)) as u64) as u64;
    fl_l = (crate::simd_ops::ru8(rdi)) as u8 as u64; fl_r = (0x0u8) as u8 as u64;
    if !((((fl_l as u8)) == ((fl_r as u8)))) { rax = rcx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi);
    rbx = ((crate::simd_ops::ru8(rdi)) as u32) as u32 as u64;
    crate::simd_ops::wu8(rsp.wrapping_add(0x28u64), ((rbx as u8)) as u8);
    crate::simd_ops::wu8(rsp.wrapping_add(0x29u64), (0x1u8) as u8);
    crate::simd_ops::wu8(rdi, (0x1u8) as u8);
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64;
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp);
    crate::simd_ops::wu8(rdi, ((rbx as u8)) as u8);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64;
    rsp = ((rsp).wrapping_add(0x30u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1424a2680(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x20; const P1: u64 = 0x18; const P2: u64 = 0x9;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x30u64)) as u64;
    rbx = ((rcx.wrapping_add(P0)) as u64) as u64;
    fl_l = (crate::simd_ops::ru8(rbx)) as u8 as u64; fl_r = (0x0u8) as u8 as u64;
    if !((((fl_l as u8)) == ((fl_r as u8)))) { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbx);
    rdi = ((crate::simd_ops::ru8(rbx)) as u32) as u32 as u64;
    crate::simd_ops::wu8(rsp.wrapping_add(0x28u64), ((rdi as u8)) as u8);
    crate::simd_ops::wu8(rsp.wrapping_add(0x29u64), (0x1u8) as u8);
    crate::simd_ops::wu8(rbx, (0x1u8) as u8);
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64;
    rax = ((crate::simd_ops::ru8(rcx.wrapping_add(P2))) as u32) as u32 as u64;
    fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x2u8) as u8 as u64;
    if !((((fl_l as u8)) == ((fl_r as u8)))) { fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; rax = (rax & !0xffu64) | (((if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { 1u8 } else { 0u8 }) as u8 as u64)); } else { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); }
    crate::simd_ops::wu8(rbx, ((rdi as u8)) as u8);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64;
    rsp = ((rsp).wrapping_add(0x30u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1424a2730(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142646560; const P1: u64 = 0x14339f760; const P2: u64 = 0x1424a5d60; const P3: u64 = 0x1433dc2d4; const P4: u64 = 0x1426467c8; const P5: u64 = 0x14333ccd8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; rbx = rdx; rdx = (crate::simd_ops::ru64(rdx.wrapping_add(0x10u64))) as u64; rax = ((rax).wrapping_add(1)) as u64; rbp = rcx; rdi = rbx.wrapping_add(0x8); fl_l = rax; fl_r = rdx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 4; continue; } bb = 1; continue; }
            1 => { rdx = ((rdx).wrapping_add(rdx)) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), rdx); fl_l = rdx; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 3; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), rax); rdi = rbx.wrapping_add(0x8); rdx = rax; bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx, rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 16; continue; } bb = 4; continue; }
            4 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rcx = (crate::simd_ops::ru64(rbx)) as u64; rax = ((crate::simd_ops::ru16(P1)) as u32) as u32 as u64; crate::simd_ops::wu8(rdx.wrapping_add(rcx), ((rax as u8)) as u8); rcx = rbp.wrapping_add(0x10); crate::simd_ops::lk_add_u64(rbx.wrapping_add(0x8u64), (1) as u64); rdx = rbx; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rdi)) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rbx)) as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(rax).wrapping_sub(0x1u64))) as u8 as u64; fl_r = (0x3eu8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 12; continue; } bb = 6; continue; }
            6 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rcx = ((rcx).wrapping_add(1)) as u64; fl_l = rcx; fl_r = rdx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 11; continue; } bb = 7; continue; }
            7 => { rdx = ((rdx).wrapping_add(rdx)) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), rdx); fl_l = rdx; fl_r = rcx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 9; continue; } bb = 8; continue; }
            8 => { crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), rcx); rdx = rcx; bb = 9; continue; }
            9 => { rcx = rax; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx, rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 16; continue; } bb = 10; continue; }
            10 => { rdi = rbx.wrapping_add(0x8); bb = 11; continue; }
            11 => { rdx = (crate::simd_ops::ru64(rdi)) as u64; rcx = (crate::simd_ops::ru64(rbx)) as u64; rax = ((crate::simd_ops::ru16(P3)) as u32) as u32 as u64; crate::simd_ops::wu8(rdx.wrapping_add(rcx), ((rax as u8)) as u8); fl_l = (crate::simd_ops::lk_add_u64(rdi, (1) as u64)) as u64; rcx = (crate::simd_ops::ru64(rdi)) as u64; bb = 12; continue; }
            12 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rcx = ((rcx).wrapping_add(1)) as u64; fl_l = rcx; fl_r = rdx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 17; continue; } bb = 13; continue; }
            13 => { rdx = ((rdx).wrapping_add(rdx)) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), rdx); fl_l = rdx; fl_r = rcx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 15; continue; } bb = 14; continue; }
            14 => { crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), rcx); rdx = rcx; bb = 15; continue; }
            15 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx, rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 17; continue; } bb = 16; continue; }
            16 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x1424a283d, rcx, rsp); return rax; }
            17 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rcx = (crate::simd_ops::ru64(rbx)) as u64; rax = ((crate::simd_ops::ru16(P5)) as u32) as u32 as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; crate::simd_ops::wu8(rdx.wrapping_add(rcx), ((rax as u8)) as u8); crate::simd_ops::lk_add_u64(rbx.wrapping_add(0x8u64), (1) as u64); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1424a28b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142646560; const P1: u64 = 0x1433e0608; const P2: u64 = 0x14183acc8; const P3: u64 = 0x14332bab8; const P4: u64 = 0x1426467c8;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 15>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r11 = rsp; crate::simd_ops::wu64(r11.wrapping_add(0x18u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rbx = rdx; r13 = rcx; r15 = rdx.wrapping_add(0x18); crate::simd_ops::wu64(r11.wrapping_sub(0x58u64), r15); rax = (crate::simd_ops::ru32(r15)) as u32 as u64; crate::simd_ops::wu32(r11.wrapping_add(0x10u64), ((rax as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rax as u32)) as u32); crate::simd_ops::wu8(rsp.wrapping_add(0x2cu64), (0x1u8) as u8); crate::simd_ops::wu32(r15, (0xffffffffu32) as u32); r12 = rdx.wrapping_add(0x1c); crate::simd_ops::wu64(r11.wrapping_sub(0x48u64), r12); rax = (crate::simd_ops::ru32(r12)) as u32 as u64; crate::simd_ops::wu32(r11.wrapping_add(0x8u64), ((rax as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), ((rax as u32)) as u32); crate::simd_ops::wu8(rsp.wrapping_add(0x3cu64), (0x1u8) as u8); crate::simd_ops::wu32(r12, (0xffffffffu32) as u32); rsi = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; rdi = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; rax = (crate::simd_ops::ru64(rdi)) as u64; rcx = rdi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x20u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(0x9u64))) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rdx = rbx; rcx = rdi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x28u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            2 => { rbp = (crate::simd_ops::ru32(r12)) as u32 as u64; fl_l = ((rbp as u32)) as u32 as u64; fl_r = 0xffffffff; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 9; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rax = ((rax).wrapping_add(0x3u64)) as u64; rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 7; continue; } bb = 4; continue; }
            4 => { rdx = ((rdx).wrapping_add(rdx)) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), rdx); fl_l = rdx; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), rax); rdx = rax; bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx, rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 21; continue; } bb = 7; continue; }
            7 => { rdi = rbx.wrapping_add(0x8); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rbx))) as u64; r8 = 0x3; rdx = P1; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::lk_add_u64(rdi, (0x3u64) as u64)) as u64; fl_r = fl_l; bb = 8; continue; }
            8 => { rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x80u64))) as u32 as u64; crate::simd_ops::wu32(r12, ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x88u64))) as u32 as u64; crate::simd_ops::wu32(r15, ((rax as u32)) as u32); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            9 => { fl_l = ((rbp as u32)) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), rsi); bb = 8; continue; }
            11 => { fl_l = ((rbp as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 8; continue; } bb = 12; continue; }
            12 => { rsi = 0x1; bb = 13; continue; }
            13 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rax = ((rax).wrapping_add(0x2u64)) as u64; rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 17; continue; } bb = 14; continue; }
            14 => { rdx = ((rdx).wrapping_add(rdx)) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), rdx); fl_l = rdx; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 16; continue; } bb = 15; continue; }
            15 => { crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), rax); rdx = rax; bb = 16; continue; }
            16 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx, rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 21; continue; } bb = 17; continue; }
            17 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rcx = (crate::simd_ops::ru64(rbx)) as u64; rax = ((crate::simd_ops::ru16(P3)) as u32) as u32 as u64; crate::simd_ops::wu16(rdx.wrapping_add(rcx), ((rax as u16)) as u16); crate::simd_ops::lk_add_u64(rbx.wrapping_add(0x8u64), (0x2u64) as u64); crate::simd_ops::wu32(r15, ((rsi as u32)) as u32); rdi = (crate::simd_ops::ru64(r13.wrapping_add(0x10u64))) as u64; rax = (crate::simd_ops::ru64(rdi)) as u64; rdx = rbx; rcx = rdi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x20u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(0x9u64))) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 19; continue; } bb = 18; continue; }
            18 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rdx = rbx; rcx = rdi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x28u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 19; continue; }
            19 => { rsi = (((rsi as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 13; continue; } bb = 20; continue; }
            20 => { bb = 8; continue; }
            21 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x1424a2a59, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1424a2a60(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x14340f8d0; const P1: u64 = 0x8; const P2: u64 = 0x10;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = P0;
    crate::simd_ops::wu32(rcx.wrapping_add(P1), (0x101011du32) as u32);
    crate::simd_ops::wu64(rcx, rax);
    rax = rcx;
    crate::simd_ops::wu64(rcx.wrapping_add(P2), rdx);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1424a2b90(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1c; const P1: u64 = 0x18; const P2: u64 = 0x10;
    let (mut rax, mut r10, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    fl_l = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64; fl_r = 0xffffffff;
    if !((((fl_l as u32)) != ((fl_r as u32)))) { rax = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_add(P0), ((rax as u32)) as u32); crate::simd_ops::wu32(rdx.wrapping_add(P1), (0x0u32) as u32); }
    r10 = (crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32 as u64;
    fl_l = r10; fl_r = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64;
    if !((((fl_l as u64)) >= ((fl_r as u64)))) { rax = (crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(r10.wrapping_mul(8)))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; return crate::simd_ops::call_fwd(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
    rax = rcx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1424a2bd0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1c; const P1: u64 = 0x18; const P2: u64 = 0x10; const P3: u64 = 0xb;
    let (mut rax, mut r10, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    fl_l = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64; fl_r = 0xffffffff;
    r10 = rcx;
    if !((((fl_l as u32)) != ((fl_r as u32)))) { rax = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_add(P0), ((rax as u32)) as u32); crate::simd_ops::wu32(rdx.wrapping_add(P1), (0x0u32) as u32); }
    rcx = (crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32 as u64;
    fl_l = rcx; fl_r = (crate::simd_ops::ru64(r10.wrapping_add(P1))) as u64;
    if (((fl_l as u64)) >= ((fl_r as u64))) { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
    rax = (crate::simd_ops::ru64(r10.wrapping_add(P2))) as u64;
    rcx = (crate::simd_ops::ru64(rax.wrapping_add(rcx.wrapping_mul(8)))) as u64;
    rax = ((crate::simd_ops::ru8(rcx.wrapping_add(P3))) as u32) as u32 as u64;
    fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x2u8) as u8 as u64;
    if !((((fl_l as u8)) == ((fl_r as u8)))) { fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; rax = (rax & !0xffu64) | (((if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { 1u8 } else { 0u8 }) as u8 as u64)); } else { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); }
    fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64;
    if !(((((fl_l as u8)) & ((fl_r as u8))) == 0)) { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
    rax = (rax & !0xffu64) | (((0u8) as u8 as u64));
    fl_l = 0; fl_r = 0;
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1424a2c30(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1c; const P1: u64 = 0x18; const P2: u64 = 0x10; const P3: u64 = 0xa; const P4: u64 = 0x8;
    let (mut rax, mut r10, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    fl_l = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64; fl_r = 0xffffffff;
    r10 = rcx;
    if !((((fl_l as u32)) != ((fl_r as u32)))) { rax = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_add(P0), ((rax as u32)) as u32); crate::simd_ops::wu32(rdx.wrapping_add(P1), (0x0u32) as u32); }
    rcx = (crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32 as u64;
    fl_l = rcx; fl_r = (crate::simd_ops::ru64(r10.wrapping_add(P1))) as u64;
    if (((fl_l as u64)) >= ((fl_r as u64))) { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
    rax = (crate::simd_ops::ru64(r10.wrapping_add(P2))) as u64;
    rcx = (crate::simd_ops::ru64(rax.wrapping_add(rcx.wrapping_mul(8)))) as u64;
    rax = ((crate::simd_ops::ru8(rcx.wrapping_add(P3))) as u32) as u32 as u64;
    fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x2u8) as u8 as u64;
    if !((((fl_l as u8)) == ((fl_r as u8)))) { fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; rax = (rax & !0xffu64) | (((if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { 1u8 } else { 0u8 }) as u8 as u64)); } else { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P4))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); }
    fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64;
    if !(((((fl_l as u8)) & ((fl_r as u8))) == 0)) { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
    rax = (rax & !0xffu64) | (((0u8) as u8 as u64));
    fl_l = 0; fl_r = 0;
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1424a2c90(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1c; const P1: u64 = 0x18; const P2: u64 = 0x10; const P3: u64 = 0x9;
    let (mut rax, mut r10, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    fl_l = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64; fl_r = 0xffffffff;
    r10 = rcx;
    if !((((fl_l as u32)) != ((fl_r as u32)))) { rax = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_add(P0), ((rax as u32)) as u32); crate::simd_ops::wu32(rdx.wrapping_add(P1), (0x0u32) as u32); }
    rcx = (crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32 as u64;
    fl_l = rcx; fl_r = (crate::simd_ops::ru64(r10.wrapping_add(P1))) as u64;
    if (((fl_l as u64)) >= ((fl_r as u64))) { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
    rax = (crate::simd_ops::ru64(r10.wrapping_add(P2))) as u64;
    rcx = (crate::simd_ops::ru64(rax.wrapping_add(rcx.wrapping_mul(8)))) as u64;
    rax = ((crate::simd_ops::ru8(rcx.wrapping_add(P3))) as u32) as u32 as u64;
    fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x2u8) as u8 as u64;
    if !((((fl_l as u8)) == ((fl_r as u8)))) { fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; rax = (rax & !0xffu64) | (((if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { 1u8 } else { 0u8 }) as u8 as u64)); } else { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); }
    fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64;
    if !(((((fl_l as u8)) & ((fl_r as u8))) == 0)) { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
    rax = (rax & !0xffu64) | (((0u8) as u8 as u64));
    fl_l = 0; fl_r = 0;
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1424a2cf0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x1c; const P1: u64 = 0x18;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64; fl_r = 0xffffffff;
    if !((((fl_l as u32)) != ((fl_r as u32)))) { rax = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_add(P0), ((rax as u32)) as u32); crate::simd_ops::wu32(rdx.wrapping_add(P1), (0x0u32) as u32); }
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1424a2d50(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142646560; const P1: u64 = 0x14340f7c0; const P2: u64 = 0x14183acc8; const P3: u64 = 0x1426467c8; const P4: u64 = 0x1433e091c;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; rdi = rdx.wrapping_add(0x10); rbx = rdx; rax = ((rax).wrapping_add(0xdu64)) as u64; rdx = (crate::simd_ops::ru64(rdi)) as u64; r15 = rcx; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 5; continue; } bb = 1; continue; }
            1 => { rdx = ((rdx).wrapping_add(rdx)) as u64; crate::simd_ops::wu64(rdi, rdx); fl_l = rdx; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 3; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rdi, rax); rdx = rax; bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx, rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 16; continue; } bb = 4; continue; }
            4 => { rdi = rbx.wrapping_add(0x10); bb = 5; continue; }
            5 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; rdx = P1; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rbx.wrapping_add(0x8u64)))) as u64; r14 = rbx.wrapping_add(0x8); r8 = 0xd; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::lk_add_u64(rbx.wrapping_add(0x8u64), (0xdu64) as u64); rsi = r14; rbp = (crate::simd_ops::ru64(r15.wrapping_add(0x10u64))) as u64; r15 = (crate::simd_ops::ru64(r15.wrapping_add(0x18u64))) as u64; fl_l = r15; fl_r = rbp; r15 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 12; continue; } bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru64(r14)) as u64; rdx = (crate::simd_ops::ru64(rdi)) as u64; rcx = ((rcx).wrapping_add(r15)) as u64; fl_l = rcx; fl_r = rdx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 11; continue; } bb = 7; continue; }
            7 => { rdx = ((rdx).wrapping_add(rdx)) as u64; crate::simd_ops::wu64(rdi, rdx); fl_l = rdx; fl_r = rcx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 9; continue; } bb = 8; continue; }
            8 => { crate::simd_ops::wu64(rdi, rcx); rsi = rbx.wrapping_add(0x8); rdx = rcx; bb = 9; continue; }
            9 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx, rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 16; continue; } bb = 10; continue; }
            10 => { rdi = rbx.wrapping_add(0x10); bb = 11; continue; }
            11 => { rcx = (crate::simd_ops::ru64(r14)) as u64; r8 = r15; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rbx))) as u64; rdx = rbp; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::lk_add_u64(rsi, (r15) as u64)) as u64; fl_r = fl_l; r14 = rsi; bb = 12; continue; }
            12 => { rax = (crate::simd_ops::ru64(r14)) as u64; rbp = rsi; rdx = (crate::simd_ops::ru64(rdi)) as u64; rax = ((rax).wrapping_add(1)) as u64; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 17; continue; } bb = 13; continue; }
            13 => { rdx = ((rdx).wrapping_add(rdx)) as u64; crate::simd_ops::wu64(rdi, rdx); fl_l = rdx; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 15; continue; } bb = 14; continue; }
            14 => { crate::simd_ops::wu64(rdi, rax); rbp = rbx.wrapping_add(0x8); rdx = rax; bb = 15; continue; }
            15 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx, rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 17; continue; } bb = 16; continue; }
            16 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x1424a2e6a, rcx, rsp); return rax; }
            17 => { rdx = (crate::simd_ops::ru64(rsi)) as u64; rcx = (crate::simd_ops::ru64(rbx)) as u64; rax = ((crate::simd_ops::ru16(P4)) as u32) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; crate::simd_ops::wu8(rdx.wrapping_add(rcx), ((rax as u8)) as u8); crate::simd_ops::lk_add_u64(rbp, (1) as u64); rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1424a2ee0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142646560; const P1: u64 = 0x14340f760; const P2: u64 = 0x14183acc8; const P3: u64 = 0x1426467c8; const P4: u64 = 0x1433e091c;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; rsi = rcx; rcx = rdi; rbx = rdx; rax = (crate::simd_ops::ru64(rdi)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x20u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(0x9u64))) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rdx = rbx; rcx = rdi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x28u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rax = ((rax).wrapping_add(0x8u64)) as u64; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 6; continue; } bb = 3; continue; }
            3 => { rdx = ((rdx).wrapping_add(rdx)) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), rdx); fl_l = rdx; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 5; continue; } bb = 4; continue; }
            4 => { crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), rax); rdx = rax; bb = 5; continue; }
            5 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx, rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 22; continue; } bb = 6; continue; }
            6 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rax = (crate::simd_ops::ru64(P1)) as u64; rcx = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu64(rdx.wrapping_add(rcx), rax); crate::simd_ops::lk_add_u64(rbx.wrapping_add(0x8u64), (0x8u64) as u64); rdi = (crate::simd_ops::ru64(rsi.wrapping_add(0x18u64))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 7; continue; }
            7 => { fl_l = (crate::simd_ops::ru64(rsi.wrapping_add(0x20u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 10; continue; } bb = 8; continue; }
            8 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rdx = rbx; rcx = rdi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x20u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(0x9u64))) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 18; continue; } bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rdx = rbx; rcx = rdi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x28u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 18; continue; }
            10 => { rsi = (crate::simd_ops::ru64(rsi.wrapping_add(0x20u64))) as u64; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 18; continue; } bb = 11; continue; }
            11 => { fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 18; continue; } bb = 12; continue; }
            12 => { fl_l = rsi; fl_r = rdi; rsi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 18; continue; } bb = 13; continue; }
            13 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rcx = ((rcx).wrapping_add(rsi)) as u64; fl_l = rcx; fl_r = rdx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 17; continue; } bb = 14; continue; }
            14 => { rdx = ((rdx).wrapping_add(rdx)) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), rdx); fl_l = rdx; fl_r = rcx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 16; continue; } bb = 15; continue; }
            15 => { crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), rcx); rdx = rcx; bb = 16; continue; }
            16 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx, rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 22; continue; } bb = 17; continue; }
            17 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; r8 = rsi; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rbx))) as u64; rdx = rdi; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::lk_add_u64(rbx.wrapping_add(0x8u64), (rsi) as u64)) as u64; fl_r = fl_l; bb = 18; continue; }
            18 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rax = ((rax).wrapping_add(1)) as u64; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 23; continue; } bb = 19; continue; }
            19 => { rdx = ((rdx).wrapping_add(rdx)) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), rdx); fl_l = rdx; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 21; continue; } bb = 20; continue; }
            20 => { crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), rax); rdx = rax; bb = 21; continue; }
            21 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx, rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 23; continue; } bb = 22; continue; }
            22 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x1424a302b, rcx, rsp); return rax; }
            23 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rdx = (crate::simd_ops::ru64(rbx)) as u64; rax = ((crate::simd_ops::ru16(P4)) as u32) as u32 as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; crate::simd_ops::wu8(rdx.wrapping_add(rcx), ((rax as u8)) as u8); crate::simd_ops::lk_add_u64(rbx.wrapping_add(0x8u64), (1) as u64); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1424a3400(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142646560; const P1: u64 = 0x14340f778; const P2: u64 = 0x14183acc8; const P3: u64 = 0x1426467c8; const P4: u64 = 0x14340f770;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; rbx = rdx; rdx = (crate::simd_ops::ru64(rdx.wrapping_add(0x10u64))) as u64; rax = ((rax).wrapping_add(0x18u64)) as u64; rbp = rcx; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 4; continue; } bb = 1; continue; }
            1 => { rdx = ((rdx).wrapping_add(rdx)) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), rdx); fl_l = rdx; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 3; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), rax); rdx = rax; bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx, rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; rdx = P1; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rbx.wrapping_add(0x8u64)))) as u64; rdi = rbx.wrapping_add(0x8); r8 = 0x18; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::lk_add_u64(rbx.wrapping_add(0x8u64), (0x18u64) as u64); rdx = rbx; rsi = (crate::simd_ops::ru64(rbp.wrapping_add(0x10u64))) as u64; rcx = rsi; rax = (crate::simd_ops::ru64(rsi)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x20u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru8(rsi.wrapping_add(0x9u64))) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rsi)) as u64; rdx = rbx; rcx = rsi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x28u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rax = ((rax).wrapping_add(0x4u64)) as u64; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 11; continue; } bb = 7; continue; }
            7 => { rdx = ((rdx).wrapping_add(rdx)) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), rdx); fl_l = rdx; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 9; continue; } bb = 8; continue; }
            8 => { crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), rax); rdi = rbx.wrapping_add(0x8); rdx = rax; bb = 9; continue; }
            9 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx, rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x1424a34d1, rcx, rsp); return rax; }
            11 => { rdx = (crate::simd_ops::ru64(rdi)) as u64; rax = (crate::simd_ops::ru32(P4)) as u32 as u64; rcx = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu32(rdx.wrapping_add(rcx), ((rax as u32)) as u32); rdx = rbx; crate::simd_ops::lk_add_u64(rdi, (0x4u64) as u64); rdi = (crate::simd_ops::ru64(rbp.wrapping_add(0x18u64))) as u64; rcx = rdi; rax = (crate::simd_ops::ru64(rdi)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x20u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(0x9u64))) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 13; continue; } bb = 12; continue; }
            12 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rdx = rbx; rcx = rdi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x28u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 13; continue; }
            13 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1424a3570(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x18; const P2: u64 = 0x8; const P3: u64 = 0x142646560; const P4: u64 = 0x1426467c8; const P5: u64 = 0x14183acc8; const P6: u64 = 0x20; const P7: u64 = 0x9; const P8: u64 = 0x28;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbp = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rbx = rdx; rdi = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; rsi = rcx; fl_l = rdi; fl_r = rbp; rdi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 7; continue; } bb = 1; continue; }
            1 => { rdx = (crate::simd_ops::ru64(rdx.wrapping_add(P2))) as u64; rax = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64; rdx = ((rdx).wrapping_add(rdi)) as u64; fl_l = rdx; fl_r = rax; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 6; continue; } bb = 2; continue; }
            2 => { rax = ((rax).wrapping_add(rax)) as u64; crate::simd_ops::wu64(rbx.wrapping_add(P0), rax); fl_l = rax; fl_r = rdx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu64(rbx.wrapping_add(P0), rdx); rax = rdx; bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; rdx = rax; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx, rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x1424a35d4, rcx, rsp); return rax; }
            6 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64; r8 = rdi; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rbx))) as u64; rdx = rbp; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::lk_add_u64(rbx.wrapping_add(P2), (rdi) as u64)) as u64; fl_r = fl_l; bb = 7; continue; }
            7 => { rdi = (crate::simd_ops::ru64(rsi.wrapping_add(P6))) as u64; rdx = rbx; rcx = rdi; rax = (crate::simd_ops::ru64(rdi)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P6))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(P7))) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 9; continue; } bb = 8; continue; }
            8 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rdx = rbx; rcx = rdi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P8))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 9; continue; }
            9 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1424a3670(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x142646560; const P3: u64 = 0x1426467c8; const P4: u64 = 0x14340f7e0; const P5: u64 = 0x14183acc8; const P6: u64 = 0x20; const P7: u64 = 0x9; const P8: u64 = 0x28;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; rbx = rdx; rdx = (crate::simd_ops::ru64(rdx.wrapping_add(P1))) as u64; rax = ((rax).wrapping_add(0xbu64)) as u64; rdi = rcx; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 5; continue; } bb = 1; continue; }
            1 => { rdx = ((rdx).wrapping_add(rdx)) as u64; crate::simd_ops::wu64(rbx.wrapping_add(P1), rdx); fl_l = rdx; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 3; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rbx.wrapping_add(P1), rax); rdx = rax; bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx, rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x1424a36bb, rcx, rsp); return rax; }
            5 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64; rdx = P4; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rbx))) as u64; r8 = 0xb; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::lk_add_u64(rbx.wrapping_add(P0), (0xbu64) as u64); rdx = rbx; rdi = (crate::simd_ops::ru64(rdi.wrapping_add(P1))) as u64; rcx = rdi; rax = (crate::simd_ops::ru64(rdi)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P6))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(P7))) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rdx = rbx; rcx = rdi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P8))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 7; continue; }
            7 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1424a3750(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142646560; const P1: u64 = 0x1433dc26c; const P2: u64 = 0x1424a5d60; const P3: u64 = 0x14340f868; const P4: u64 = 0x14340f834; const P5: u64 = 0x14183acc8; const P6: u64 = 0x14340f828; const P7: u64 = 0x14340f818; const P8: u64 = 0x14340f7d0; const P9: u64 = 0x1426467c8; const P10: u64 = 0x14340f804;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; rbx = rdx; rdx = (crate::simd_ops::ru64(rdx.wrapping_add(0x10u64))) as u64; rax = ((rax).wrapping_add(1)) as u64; r14 = rcx; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 4; continue; } bb = 1; continue; }
            1 => { rdx = ((rdx).wrapping_add(rdx)) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), rdx); fl_l = rdx; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 3; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), rax); rdx = rax; bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx, rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 46; continue; } bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rsi = rbx.wrapping_add(0x8); rdx = (crate::simd_ops::ru64(rbx)) as u64; rax = ((crate::simd_ops::ru16(P1)) as u32) as u32 as u64; crate::simd_ops::wu8(rdx.wrapping_add(rcx), ((rax as u8)) as u8); rcx = r14.wrapping_add(0x20); crate::simd_ops::lk_add_u64(rbx.wrapping_add(0x8u64), (1) as u64); rdx = rbx; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rsi)) as u64; rdi = rsi; rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rax = ((rax).wrapping_add(1)) as u64; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 8; continue; } bb = 5; continue; }
            5 => { rdx = ((rdx).wrapping_add(rdx)) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), rdx); fl_l = rdx; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 7; continue; } bb = 6; continue; }
            6 => { crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), rax); rdi = rbx.wrapping_add(0x8); rdx = rax; bb = 7; continue; }
            7 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx, rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 46; continue; } bb = 8; continue; }
            8 => { rcx = (crate::simd_ops::ru64(rsi)) as u64; rax = ((crate::simd_ops::ru16(P3)) as u32) as u32 as u64; rdx = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu8(rdx.wrapping_add(rcx), ((rax as u8)) as u8); crate::simd_ops::lk_add_u64(rdi, (1) as u64); rcx = (crate::simd_ops::ru64(r14.wrapping_add(0x10u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x28u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 10; continue; }
            10 => { fl_l = (crate::simd_ops::ru8(r14.wrapping_add(0x38u64))) as u8 as u64; fl_r = (0x1u8) as u8 as u64; rsi = r14.wrapping_add(0x38); if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 17; continue; } bb = 11; continue; }
            11 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rax = ((rax).wrapping_add(0x6u64)) as u64; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 16; continue; } bb = 12; continue; }
            12 => { rdx = ((rdx).wrapping_add(rdx)) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), rdx); fl_l = rdx; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 14; continue; } bb = 13; continue; }
            13 => { crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), rax); rdx = rax; bb = 14; continue; }
            14 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx, rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 46; continue; } bb = 15; continue; }
            15 => { rdi = rbx.wrapping_add(0x8); rsi = r14.wrapping_add(0x38); bb = 16; continue; }
            16 => { rcx = (crate::simd_ops::ru64(rdi)) as u64; rdx = P4; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rbx))) as u64; r8 = 0x6; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::lk_add_u64(rdi, (0x6u64) as u64)) as u64; fl_r = fl_l; bb = 17; continue; }
            17 => { fl_l = (crate::simd_ops::ru8(rsi)) as u8 as u64; fl_r = (0x2u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 25; continue; } bb = 18; continue; }
            18 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rax = ((rax).wrapping_add(0x9u64)) as u64; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 23; continue; } bb = 19; continue; }
            19 => { rdx = ((rdx).wrapping_add(rdx)) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), rdx); fl_l = rdx; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 21; continue; } bb = 20; continue; }
            20 => { crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), rax); rdx = rax; bb = 21; continue; }
            21 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx, rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 46; continue; } bb = 22; continue; }
            22 => { rbp = rbx.wrapping_add(0x8); rsi = r14.wrapping_add(0x38); bb = 24; continue; }
            23 => { rbp = rdi; bb = 24; continue; }
            24 => { rcx = (crate::simd_ops::ru64(rdi)) as u64; rdx = P6; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rbx))) as u64; r8 = 0x9; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::lk_add_u64(rbp, (0x9u64) as u64)) as u64; fl_r = fl_l; rdi = rbp; bb = 25; continue; }
            25 => { fl_l = (crate::simd_ops::ru8(rsi)) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 33; continue; } bb = 26; continue; }
            26 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rax = ((rax).wrapping_add(0x9u64)) as u64; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 31; continue; } bb = 27; continue; }
            27 => { rdx = ((rdx).wrapping_add(rdx)) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), rdx); fl_l = rdx; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 29; continue; } bb = 28; continue; }
            28 => { crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), rax); rdx = rax; bb = 29; continue; }
            29 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx, rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 46; continue; } bb = 30; continue; }
            30 => { rsi = rbx.wrapping_add(0x8); bb = 32; continue; }
            31 => { rsi = rdi; bb = 32; continue; }
            32 => { rcx = (crate::simd_ops::ru64(rdi)) as u64; rdx = P7; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rbx))) as u64; r8 = 0x9; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::lk_add_u64(rsi, (0x9u64) as u64)) as u64; fl_r = fl_l; rdi = rsi; bb = 33; continue; }
            33 => { rax = ((crate::simd_ops::ru8(r14.wrapping_add(0x3cu64))) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 41; continue; } bb = 34; continue; }
            34 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rax = ((rax).wrapping_add(0x2u64)) as u64; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 39; continue; } bb = 35; continue; }
            35 => { rdx = ((rdx).wrapping_add(rdx)) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), rdx); fl_l = rdx; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 37; continue; } bb = 36; continue; }
            36 => { crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), rax); rdx = rax; bb = 37; continue; }
            37 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx, rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 46; continue; } bb = 38; continue; }
            38 => { r8 = rbx.wrapping_add(0x8); bb = 40; continue; }
            39 => { r8 = rdi; bb = 40; continue; }
            40 => { rdx = (crate::simd_ops::ru64(rdi)) as u64; rax = ((crate::simd_ops::ru16(P8)) as u32) as u32 as u64; rcx = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu16(rdx.wrapping_add(rcx), ((rax as u16)) as u16); fl_l = (crate::simd_ops::lk_add_u64(r8, (0x2u64) as u64)) as u64; fl_r = fl_l; bb = 48; continue; }
            41 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x2u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 48; continue; } bb = 42; continue; }
            42 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rax = ((rax).wrapping_add(0x3u64)) as u64; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 47; continue; } bb = 43; continue; }
            43 => { rdx = ((rdx).wrapping_add(rdx)) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), rdx); fl_l = rdx; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 45; continue; } bb = 44; continue; }
            44 => { crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), rax); rdi = rbx.wrapping_add(0x8); rdx = rax; bb = 45; continue; }
            45 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx, rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 47; continue; } bb = 46; continue; }
            46 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P9), rax), rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x1424a3a05, rcx, rsp); return rax; }
            47 => { rcx = (crate::simd_ops::ru64(rdi)) as u64; rdx = P10; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rbx))) as u64; r8 = 0x3; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::lk_add_u64(rdi, (0x3u64) as u64)) as u64; fl_r = fl_l; bb = 48; continue; }
            48 => { rdi = (crate::simd_ops::ru64(r14.wrapping_add(0x30u64))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 51; continue; } bb = 49; continue; }
            49 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rdx = rbx; rcx = rdi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x20u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(0x9u64))) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 51; continue; } bb = 50; continue; }
            50 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rdx = rbx; rcx = rdi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x28u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 51; continue; }
            51 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1424a3a70(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x20; const P2: u64 = 0x9; const P3: u64 = 0x8; const P4: u64 = 0x142646560; const P5: u64 = 0x1426467c8; const P6: u64 = 0x1433dc2d4; const P7: u64 = 0x18; const P8: u64 = 0x28;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = rcx; rbx = rdx; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P0))) as u64; rax = ((crate::simd_ops::ru8(rcx.wrapping_add(P2))) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x2u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 3; continue; } bb = 2; continue; }
            2 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; rax = (rax & !0xffu64) | (((if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { 1u8 } else { 0u8 }) as u8 as u64)); bb = 4; continue; }
            3 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 11; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(P3))) as u64; rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64; rax = ((rax).wrapping_add(1)) as u64; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 10; continue; } bb = 6; continue; }
            6 => { rdx = ((rdx).wrapping_add(rdx)) as u64; crate::simd_ops::wu64(rbx.wrapping_add(P0), rdx); fl_l = rdx; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 8; continue; } bb = 7; continue; }
            7 => { crate::simd_ops::wu64(rbx.wrapping_add(P0), rax); rdx = rax; bb = 8; continue; }
            8 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx, rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x1424a3ae8, rcx, rsp); return rax; }
            10 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P3))) as u64; rax = ((crate::simd_ops::ru16(P6)) as u32) as u32 as u64; rcx = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu8(rdx.wrapping_add(rcx), ((rax as u8)) as u8); fl_l = (crate::simd_ops::lk_add_u64(rbx.wrapping_add(P3), (1) as u64)) as u64; bb = 11; continue; }
            11 => { rdi = (crate::simd_ops::ru64(rdi.wrapping_add(P7))) as u64; rdx = rbx; rcx = rdi; rax = (crate::simd_ops::ru64(rdi)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(P2))) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 13; continue; } bb = 12; continue; }
            12 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rdx = rbx; rcx = rdi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P8))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 13; continue; }
            13 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1424a3ba0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x142646560; const P3: u64 = 0x1426467c8; const P4: u64 = 0x14340f7ec; const P5: u64 = 0x14183acc8; const P6: u64 = 0x1424a5d60; const P7: u64 = 0x1424a6050;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; rbx = rdx; rdx = (crate::simd_ops::ru64(rdx.wrapping_add(P1))) as u64; rax = ((rax).wrapping_add(0x6u64)) as u64; rdi = rcx; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 5; continue; } bb = 1; continue; }
            1 => { rdx = ((rdx).wrapping_add(rdx)) as u64; crate::simd_ops::wu64(rbx.wrapping_add(P1), rdx); fl_l = rdx; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 3; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rbx.wrapping_add(P1), rax); rdx = rax; bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx, rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x1424a3beb, rcx, rsp); return rax; }
            5 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64; rdx = P4; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rbx))) as u64; r8 = 0x6; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::lk_add_u64(rbx.wrapping_add(P0), (0x6u64) as u64); rcx = ((rdi.wrapping_add(P1)) as u64) as u64; rdx = rbx; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rdx = (rdx & !0xffu64) | (((0x29u8) as u8 as u64)); rcx = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P7, rax, rcx, rdx, r8, r9, rsp); }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1424a3c70(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142646560; const P1: u64 = 0x14340f7f8; const P2: u64 = 0x14183acc8; const P3: u64 = 0x1426467c8; const P4: u64 = 0x14340f868;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; rbx = rdx; rdx = (crate::simd_ops::ru64(rdx.wrapping_add(0x10u64))) as u64; rax = ((rax).wrapping_add(0x9u64)) as u64; rbp = rcx; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 4; continue; } bb = 1; continue; }
            1 => { rdx = ((rdx).wrapping_add(rdx)) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), rdx); fl_l = rdx; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 3; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), rax); rdx = rax; bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx, rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rdx = P1; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rbx))) as u64; rdi = rbx.wrapping_add(0x8); r8 = 0x9; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::lk_add_u64(rbx.wrapping_add(0x8u64), (0x9u64) as u64); rdx = rbx; rsi = (crate::simd_ops::ru64(rbp.wrapping_add(0x10u64))) as u64; rcx = rsi; rax = (crate::simd_ops::ru64(rsi)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x20u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru8(rsi.wrapping_add(0x9u64))) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rsi)) as u64; rdx = rbx; rcx = rsi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x28u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rax = ((rax).wrapping_add(1)) as u64; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 11; continue; } bb = 7; continue; }
            7 => { rdx = ((rdx).wrapping_add(rdx)) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), rdx); fl_l = rdx; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 9; continue; } bb = 8; continue; }
            8 => { crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), rax); rdi = rbx.wrapping_add(0x8); rdx = rax; bb = 9; continue; }
            9 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx, rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x1424a3d40, rcx, rsp); return rax; }
            11 => { rdx = (crate::simd_ops::ru64(rdi)) as u64; rcx = (crate::simd_ops::ru64(rbx)) as u64; rax = ((crate::simd_ops::ru16(P4)) as u32) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; crate::simd_ops::wu8(rdx.wrapping_add(rcx), ((rax as u8)) as u8); crate::simd_ops::lk_add_u64(rdi, (1) as u64); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1424a3db0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142646560; const P1: u64 = 0x1433dc26c; const P2: u64 = 0x1424a5d60; const P3: u64 = 0x14340f868; const P4: u64 = 0x14340f834; const P5: u64 = 0x14183acc8; const P6: u64 = 0x14340f828; const P7: u64 = 0x14340f818; const P8: u64 = 0x14340f7d0; const P9: u64 = 0x1426467c8; const P10: u64 = 0x14340f804; const P11: u64 = 0x1424a6050;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; rbx = rdx; rdx = (crate::simd_ops::ru64(rdx.wrapping_add(0x10u64))) as u64; rax = ((rax).wrapping_add(1)) as u64; rbp = rcx; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 4; continue; } bb = 1; continue; }
            1 => { rdx = ((rdx).wrapping_add(rdx)) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), rdx); fl_l = rdx; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 3; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), rax); rdx = rax; bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx, rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 45; continue; } bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rsi = rbx.wrapping_add(0x8); rdx = (crate::simd_ops::ru64(rbx)) as u64; rax = ((crate::simd_ops::ru16(P1)) as u32) as u32 as u64; crate::simd_ops::wu8(rdx.wrapping_add(rcx), ((rax as u8)) as u8); rcx = rbp.wrapping_add(0x18); crate::simd_ops::lk_add_u64(rbx.wrapping_add(0x8u64), (1) as u64); rdx = rbx; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rsi)) as u64; rdi = rsi; rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rax = ((rax).wrapping_add(1)) as u64; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 8; continue; } bb = 5; continue; }
            5 => { rdx = ((rdx).wrapping_add(rdx)) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), rdx); fl_l = rdx; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 7; continue; } bb = 6; continue; }
            6 => { crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), rax); rdi = rbx.wrapping_add(0x8); rdx = rax; bb = 7; continue; }
            7 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx, rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 45; continue; } bb = 8; continue; }
            8 => { rdx = (crate::simd_ops::ru64(rsi)) as u64; rax = ((crate::simd_ops::ru16(P3)) as u32) as u32 as u64; rcx = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu8(rdx.wrapping_add(rcx), ((rax as u8)) as u8); rdx = rbx; crate::simd_ops::lk_add_u64(rdi, (1) as u64); rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x10u64))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x28u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru8(rbp.wrapping_add(0x28u64))) as u8 as u64; fl_r = (0x1u8) as u8 as u64; rsi = rbp.wrapping_add(0x28); if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 16; continue; } bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rax = ((rax).wrapping_add(0x6u64)) as u64; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 14; continue; } bb = 10; continue; }
            10 => { rdx = ((rdx).wrapping_add(rdx)) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), rdx); fl_l = rdx; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 12; continue; } bb = 11; continue; }
            11 => { crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), rax); rdx = rax; bb = 12; continue; }
            12 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx, rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 45; continue; } bb = 13; continue; }
            13 => { r14 = rbx.wrapping_add(0x8); rsi = rbp.wrapping_add(0x28); bb = 15; continue; }
            14 => { r14 = rdi; bb = 15; continue; }
            15 => { rcx = (crate::simd_ops::ru64(rdi)) as u64; rdx = P4; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rbx))) as u64; r8 = 0x6; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::lk_add_u64(r14, (0x6u64) as u64)) as u64; fl_r = fl_l; rdi = r14; bb = 16; continue; }
            16 => { fl_l = (crate::simd_ops::ru8(rsi)) as u8 as u64; fl_r = (0x2u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 24; continue; } bb = 17; continue; }
            17 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rax = ((rax).wrapping_add(0x9u64)) as u64; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 22; continue; } bb = 18; continue; }
            18 => { rdx = ((rdx).wrapping_add(rdx)) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), rdx); fl_l = rdx; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 20; continue; } bb = 19; continue; }
            19 => { crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), rax); rdx = rax; bb = 20; continue; }
            20 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx, rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 45; continue; } bb = 21; continue; }
            21 => { r14 = rbx.wrapping_add(0x8); rsi = rbp.wrapping_add(0x28); bb = 23; continue; }
            22 => { r14 = rdi; bb = 23; continue; }
            23 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; rdx = P6; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rdi))) as u64; r8 = 0x9; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::lk_add_u64(r14, (0x9u64) as u64)) as u64; fl_r = fl_l; rdi = r14; bb = 24; continue; }
            24 => { fl_l = (crate::simd_ops::ru8(rsi)) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 32; continue; } bb = 25; continue; }
            25 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rax = ((rax).wrapping_add(0x9u64)) as u64; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 30; continue; } bb = 26; continue; }
            26 => { rdx = ((rdx).wrapping_add(rdx)) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), rdx); fl_l = rdx; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 28; continue; } bb = 27; continue; }
            27 => { crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), rax); rdx = rax; bb = 28; continue; }
            28 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx, rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 45; continue; } bb = 29; continue; }
            29 => { rsi = rbx.wrapping_add(0x8); bb = 31; continue; }
            30 => { rsi = rdi; bb = 31; continue; }
            31 => { rcx = (crate::simd_ops::ru64(rdi)) as u64; rdx = P7; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rbx))) as u64; r8 = 0x9; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::lk_add_u64(rsi, (0x9u64) as u64)) as u64; fl_r = fl_l; rdi = rsi; bb = 32; continue; }
            32 => { rax = ((crate::simd_ops::ru8(rbp.wrapping_add(0x2cu64))) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 40; continue; } bb = 33; continue; }
            33 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rax = ((rax).wrapping_add(0x2u64)) as u64; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 38; continue; } bb = 34; continue; }
            34 => { rdx = ((rdx).wrapping_add(rdx)) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), rdx); fl_l = rdx; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 36; continue; } bb = 35; continue; }
            35 => { crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), rax); rdx = rax; bb = 36; continue; }
            36 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx, rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 45; continue; } bb = 37; continue; }
            37 => { r8 = rbx.wrapping_add(0x8); bb = 39; continue; }
            38 => { r8 = rdi; bb = 39; continue; }
            39 => { rdx = (crate::simd_ops::ru64(rdi)) as u64; rax = ((crate::simd_ops::ru16(P8)) as u32) as u32 as u64; rcx = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu16(rdx.wrapping_add(rcx), ((rax as u16)) as u16); fl_l = (crate::simd_ops::lk_add_u64(r8, (0x2u64) as u64)) as u64; fl_r = fl_l; bb = 47; continue; }
            40 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x2u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 47; continue; } bb = 41; continue; }
            41 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rax = ((rax).wrapping_add(0x3u64)) as u64; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 46; continue; } bb = 42; continue; }
            42 => { rdx = ((rdx).wrapping_add(rdx)) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), rdx); fl_l = rdx; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 44; continue; } bb = 43; continue; }
            43 => { crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), rax); rdi = rbx.wrapping_add(0x8); rdx = rax; bb = 44; continue; }
            44 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx, rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 46; continue; } bb = 45; continue; }
            45 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P9), rax), rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x1424a4065, rcx, rsp); return rax; }
            46 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; rdx = P10; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rdi))) as u64; r8 = 0x3; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::lk_add_u64(rdi, (0x3u64) as u64)) as u64; fl_r = fl_l; bb = 47; continue; }
            47 => { fl_l = (crate::simd_ops::ru64(rbp.wrapping_add(0x30u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 50; continue; } bb = 48; continue; }
            48 => { rdx = (rdx & !0xffu64) | (((0x20u8) as u8 as u64)); rcx = rbx; rax = crate::simd_ops::call_stk(P11, rax, rcx, rdx, r8, r9, rsp); rdi = (crate::simd_ops::ru64(rbp.wrapping_add(0x30u64))) as u64; rdx = rbx; rcx = rdi; rax = (crate::simd_ops::ru64(rdi)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x20u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(0x9u64))) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 50; continue; } bb = 49; continue; }
            49 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rdx = rbx; rcx = rdi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x28u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 50; continue; }
            50 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1424a40e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x20; const P2: u64 = 0x8; const P3: u64 = 0x142646560; const P4: u64 = 0x1426467c8; const P5: u64 = 0x1433dc2d4;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rbx = rdx;
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp);
    rax = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64;
    rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64;
    rax = ((rax).wrapping_add(1)) as u64;
    fl_l = rax; fl_r = rdx;
    if (((fl_l as u64)) < ((fl_r as u64))) { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64; rax = ((crate::simd_ops::ru16(P5)) as u32) as u32 as u64; rcx = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu8(rdx.wrapping_add(rcx), ((rax as u8)) as u8); crate::simd_ops::lk_add_u64(rbx.wrapping_add(P2), (1) as u64); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    rdx = ((rdx).wrapping_add(rdx)) as u64;
    crate::simd_ops::wu64(rbx.wrapping_add(P0), rdx);
    fl_l = rdx; fl_r = rax;
    if !((((fl_l as u64)) >= ((fl_r as u64)))) { crate::simd_ops::wu64(rbx.wrapping_add(P0), rax); rdx = rax; }
    rcx = (crate::simd_ops::ru64(rbx)) as u64;
    rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp);
    crate::simd_ops::wu64(rbx, rax);
    fl_l = rax; fl_r = rax;
    if !(((((fl_l as u64)) & ((fl_r as u64))) != 0)) { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x1424a412d, rcx, rsp); return rax; }
    rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64;
    rax = ((crate::simd_ops::ru16(P5)) as u32) as u32 as u64;
    rcx = (crate::simd_ops::ru64(rbx)) as u64;
    crate::simd_ops::wu8(rdx.wrapping_add(rcx), ((rax as u8)) as u8);
    crate::simd_ops::lk_add_u64(rbx.wrapping_add(P2), (1) as u64);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1424a41b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142646560; const P1: u64 = 0x1433dc2d4; const P2: u64 = 0x1433e73c4; const P3: u64 = 0x14183acc8; const P4: u64 = 0x1426467c8; const P5: u64 = 0x1433e091c;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rdx; rbp = rcx; rdx = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rbx)) as u64; fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(rax).wrapping_sub(0x1u64))) as u8 as u64; fl_r = (0x5du8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 7; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rdx = ((rdx).wrapping_add(1)) as u64; fl_l = rdx; fl_r = rax; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 6; continue; } bb = 3; continue; }
            3 => { rax = ((rax).wrapping_add(rax)) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), rax); fl_l = rax; fl_r = rdx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 5; continue; } bb = 4; continue; }
            4 => { crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), rdx); rax = rdx; bb = 5; continue; }
            5 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; rdx = rax; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx, rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 27; continue; } bb = 6; continue; }
            6 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; r8 = rbx.wrapping_add(0x8); rax = ((crate::simd_ops::ru16(P1)) as u32) as u32 as u64; rcx = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu8(rdx.wrapping_add(rcx), ((rax as u8)) as u8); fl_l = (crate::simd_ops::lk_add_u64(r8, (1) as u64)) as u64; rdx = (crate::simd_ops::ru64(r8)) as u64; bb = 7; continue; }
            7 => { rax = rdx.wrapping_add(0x1); rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 11; continue; } bb = 8; continue; }
            8 => { rdx = ((rdx).wrapping_add(rdx)) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), rdx); fl_l = rdx; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 10; continue; } bb = 9; continue; }
            9 => { crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), rax); rdx = rax; bb = 10; continue; }
            10 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx, rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 27; continue; } bb = 11; continue; }
            11 => { rax = ((crate::simd_ops::ru16(P2)) as u32) as u32 as u64; rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rcx = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu8(rdx.wrapping_add(rcx), ((rax as u8)) as u8); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rax = ((rax).wrapping_add(1)) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), rax); rsi = (crate::simd_ops::ru64(rbp.wrapping_add(0x20u64))) as u64; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 19; continue; } bb = 12; continue; }
            12 => { fl_l = (crate::simd_ops::ru64(rbp.wrapping_add(0x18u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 19; continue; } bb = 13; continue; }
            13 => { rdi = (crate::simd_ops::ru64(rbp.wrapping_add(0x18u64))) as u64; fl_l = rsi; fl_r = rdi; rsi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 23; continue; } bb = 14; continue; }
            14 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rax = ((rax).wrapping_add(rsi)) as u64; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 18; continue; } bb = 15; continue; }
            15 => { rdx = ((rdx).wrapping_add(rdx)) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), rdx); fl_l = rdx; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 17; continue; } bb = 16; continue; }
            16 => { crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), rax); rdx = rax; bb = 17; continue; }
            17 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx, rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 27; continue; } bb = 18; continue; }
            18 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; r8 = rsi; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rbx))) as u64; rdx = rdi; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::lk_add_u64(rbx.wrapping_add(0x8u64), (rsi) as u64)) as u64; fl_r = fl_l; bb = 23; continue; }
            19 => { rdi = (crate::simd_ops::ru64(rbp.wrapping_add(0x18u64))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 23; continue; } bb = 20; continue; }
            20 => { fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 23; continue; } bb = 21; continue; }
            21 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rdx = rbx; rcx = rdi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x20u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(0x9u64))) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 23; continue; } bb = 22; continue; }
            22 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rdx = rbx; rcx = rdi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x28u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 23; continue; }
            23 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rax = ((rax).wrapping_add(1)) as u64; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 28; continue; } bb = 24; continue; }
            24 => { rdx = ((rdx).wrapping_add(rdx)) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), rdx); fl_l = rdx; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 26; continue; } bb = 25; continue; }
            25 => { crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), rax); rdx = rax; bb = 26; continue; }
            26 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx, rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 28; continue; } bb = 27; continue; }
            27 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x1424a4347, rcx, rsp); return rax; }
            28 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rax = ((crate::simd_ops::ru16(P5)) as u32) as u32 as u64; rcx = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu8(rdx.wrapping_add(rcx), ((rax as u8)) as u8); rdx = rbx; crate::simd_ops::lk_add_u64(rbx.wrapping_add(0x8u64), (1) as u64); rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x10u64))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x28u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1424a43e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    crate::simd_ops::wu64(rdx, rax);
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    crate::simd_ops::wu64(rdx.wrapping_add(P0), rax);
    rax = rdx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1424a4400(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = (crate::simd_ops::ru64(rcx)) as u64; fl_r = (0x0u64) as u64;
    if (((fl_l as u64)) == ((fl_r as u64))) { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; return rax; }
    fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; fl_r = (0x0u64) as u64;
    if !((((fl_l as u64)) != ((fl_r as u64)))) { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); return rax; }
    rax = (rax & !0xffu64) | (((0u8) as u8 as u64));
    fl_l = 0; fl_r = 0;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1424a4420(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; fl_r = (0x0u64) as u64;
    if (((fl_l as u64)) == ((fl_r as u64))) { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; return rax; }
    fl_l = (crate::simd_ops::ru64(rcx)) as u64; fl_r = (0x0u64) as u64;
    if !((((fl_l as u64)) == ((fl_r as u64)))) { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); return rax; }
    rax = (rax & !0xffu64) | (((0u8) as u8 as u64));
    fl_l = 0; fl_r = 0;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1424a4480(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x18; const P1: u64 = 0xa; const P2: u64 = 0x8; const P3: u64 = 0xb; const P4: u64 = 0x10; const P5: u64 = 0x142646560; const P6: u64 = 0x1426467c8; const P7: u64 = 0x14340f868; const P8: u64 = 0x28;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = ((rcx.wrapping_add(P0)) as u64) as u64; rsi = rcx; rcx = (crate::simd_ops::ru64(rdi)) as u64; rbx = rdx; rax = ((crate::simd_ops::ru8(rcx.wrapping_add(P1))) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x2u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 2; continue; } bb = 1; continue; }
            1 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; rax = (rax & !0xffu64) | (((if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { 1u8 } else { 0u8 }) as u8 as u64)); bb = 3; continue; }
            2 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 3; continue; }
            3 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 9; continue; } bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru64(rdi)) as u64; rax = ((crate::simd_ops::ru8(rcx.wrapping_add(P3))) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x2u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 6; continue; } bb = 5; continue; }
            5 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; rax = (rax & !0xffu64) | (((if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { 1u8 } else { 0u8 }) as u8 as u64)); bb = 7; continue; }
            6 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P4))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 7; continue; }
            7 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 16; continue; } bb = 8; continue; }
            8 => { rdi = ((rsi.wrapping_add(P0)) as u64) as u64; bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64; rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P4))) as u64; rax = ((rax).wrapping_add(1)) as u64; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 15; continue; } bb = 10; continue; }
            10 => { rdx = ((rdx).wrapping_add(rdx)) as u64; crate::simd_ops::wu64(rbx.wrapping_add(P4), rdx); fl_l = rdx; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 12; continue; } bb = 11; continue; }
            11 => { crate::simd_ops::wu64(rbx.wrapping_add(P4), rax); rdx = rax; bb = 12; continue; }
            12 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx, rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 14; continue; } bb = 13; continue; }
            13 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x1424a4512, rcx, rsp); return rax; }
            14 => { rdi = ((rsi.wrapping_add(P0)) as u64) as u64; bb = 15; continue; }
            15 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64; rax = ((crate::simd_ops::ru16(P7)) as u32) as u32 as u64; rcx = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu8(rdx.wrapping_add(rcx), ((rax as u8)) as u8); fl_l = (crate::simd_ops::lk_add_u64(rbx.wrapping_add(P2), (1) as u64)) as u64; bb = 16; continue; }
            16 => { rcx = (crate::simd_ops::ru64(rdi)) as u64; rdx = rbx; rax = (crate::simd_ops::ru64(rcx)) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P8))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1424a4550(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142646560; const P1: u64 = 0x1433dc2d4; const P2: u64 = 0x1433dc26c; const P3: u64 = 0x1426467c8; const P4: u64 = 0x14340f7d4; const P5: u64 = 0x14183acc8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = rcx; rbx = rdx; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x18u64))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x20u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x18u64))) as u64; rax = ((crate::simd_ops::ru8(rcx.wrapping_add(0xau64))) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x2u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 2; continue; } bb = 1; continue; }
            1 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; rax = (rax & !0xffu64) | (((if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { 1u8 } else { 0u8 }) as u8 as u64)); bb = 3; continue; }
            2 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 3; continue; }
            3 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 13; continue; } bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x18u64))) as u64; rax = ((crate::simd_ops::ru8(rcx.wrapping_add(0xbu64))) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x2u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 6; continue; } bb = 5; continue; }
            5 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; rax = (rax & !0xffu64) | (((if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { 1u8 } else { 0u8 }) as u8 as u64)); bb = 7; continue; }
            6 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 7; continue; }
            7 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 13; continue; } bb = 8; continue; }
            8 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rax = ((rax).wrapping_add(1)) as u64; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 12; continue; } bb = 9; continue; }
            9 => { rdx = ((rdx).wrapping_add(rdx)) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), rdx); fl_l = rdx; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 11; continue; } bb = 10; continue; }
            10 => { crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), rax); rdx = rax; bb = 11; continue; }
            11 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx, rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 24; continue; } bb = 12; continue; }
            12 => { rax = ((crate::simd_ops::ru16(P1)) as u32) as u32 as u64; bb = 18; continue; }
            13 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rax = ((rax).wrapping_add(1)) as u64; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 17; continue; } bb = 14; continue; }
            14 => { rdx = ((rdx).wrapping_add(rdx)) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), rdx); fl_l = rdx; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 16; continue; } bb = 15; continue; }
            15 => { crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), rax); rdx = rax; bb = 16; continue; }
            16 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx, rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 24; continue; } bb = 17; continue; }
            17 => { rax = ((crate::simd_ops::ru16(P2)) as u32) as u32 as u64; bb = 18; continue; }
            18 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rdx = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu8(rdx.wrapping_add(rcx), ((rax as u8)) as u8); rdx = rbx; crate::simd_ops::lk_add_u64(rbx.wrapping_add(0x8u64), (1) as u64); rdi = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; rcx = rdi; rax = (crate::simd_ops::ru64(rdi)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x20u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(0x9u64))) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 20; continue; } bb = 19; continue; }
            19 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rdx = rbx; rcx = rdi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x28u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 20; continue; }
            20 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rax = ((rax).wrapping_add(0x3u64)) as u64; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 25; continue; } bb = 21; continue; }
            21 => { rdx = ((rdx).wrapping_add(rdx)) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), rdx); fl_l = rdx; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 23; continue; } bb = 22; continue; }
            22 => { crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), rax); rdx = rax; bb = 23; continue; }
            23 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx, rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 25; continue; } bb = 24; continue; }
            24 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x1424a4691, rcx, rsp); return rax; }
            25 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rdx = P4; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rbx))) as u64; r8 = 0x3; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::lk_add_u64(rbx.wrapping_add(0x8u64), (0x3u64) as u64); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1424a4730(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142646560; const P1: u64 = 0x1426467c8; const P2: u64 = 0x14340f868;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rdi = rdx; rsi = rcx.wrapping_add(0x1c); fl_l = (crate::simd_ops::ru8(rsi)) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 19; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); rbp = ((crate::simd_ops::ru8(rsi)) as u32) as u32 as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x28u64), ((rbp as u8)) as u8); crate::simd_ops::wu8(rsp.wrapping_add(0x29u64), (0x1u8) as u8); crate::simd_ops::wu8(rsi, (0x1u8) as u8); rbx = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x8u64))) as u8 as u64; fl_r = (0xcu8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 4; continue; } bb = 2; continue; }
            2 => { bb = 3; continue; }
            3 => { rbx = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = rdi; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x8u64))) as u8 as u64; fl_r = (0xcu8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 3; continue; } bb = 4; continue; }
            4 => { rax = ((crate::simd_ops::ru8(rbx.wrapping_add(0xau64))) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x2u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 6; continue; } bb = 5; continue; }
            5 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; rax = (rax & !0xffu64) | (((if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { 1u8 } else { 0u8 }) as u8 as u64)); bb = 7; continue; }
            6 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = rdi; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 7; continue; }
            7 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 12; continue; } bb = 8; continue; }
            8 => { rax = ((crate::simd_ops::ru8(rbx.wrapping_add(0xbu64))) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x2u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 10; continue; } bb = 9; continue; }
            9 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; rax = (rax & !0xffu64) | (((if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { 1u8 } else { 0u8 }) as u8 as u64)); bb = 11; continue; }
            10 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = rdi; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 11; continue; }
            11 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 18; continue; } bb = 12; continue; }
            12 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; rax = ((rax).wrapping_add(1)) as u64; rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 17; continue; } bb = 13; continue; }
            13 => { rdx = ((rdx).wrapping_add(rdx)) as u64; crate::simd_ops::wu64(rdi.wrapping_add(0x10u64), rdx); fl_l = rdx; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 15; continue; } bb = 14; continue; }
            14 => { crate::simd_ops::wu64(rdi.wrapping_add(0x10u64), rax); rdx = rax; bb = 15; continue; }
            15 => { rcx = (crate::simd_ops::ru64(rdi)) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rdi, rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 17; continue; } bb = 16; continue; }
            16 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x1424a480e, rcx, rsp); return rax; }
            17 => { r8 = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; rdx = (crate::simd_ops::ru64(rdi)) as u64; rax = ((crate::simd_ops::ru16(P2)) as u32) as u32 as u64; crate::simd_ops::wu8(r8.wrapping_add(rdx), ((rax as u8)) as u8); fl_l = (crate::simd_ops::lk_add_u64(rdi.wrapping_add(0x8u64), (1) as u64)) as u64; bb = 18; continue; }
            18 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = rdi; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x28u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu8(rsi, ((rbp as u8)) as u8); bb = 19; continue; }
            19 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1424a4850(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142646560; const P1: u64 = 0x1433dc2d4; const P2: u64 = 0x1433dc26c; const P3: u64 = 0x14340f7dc; const P4: u64 = 0x14340f7d8; const P5: u64 = 0x1426467c8; const P6: u64 = 0x14183acc8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = rsp; crate::simd_ops::wu64(rax.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rax.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rax.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rbx = rdx; r14 = rcx.wrapping_add(0x1c); fl_l = (crate::simd_ops::ru8(r14)) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 36; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rax.wrapping_sub(0x28u64), r14); r15 = ((crate::simd_ops::ru8(r14)) as u32) as u32 as u64; crate::simd_ops::wu8(rax.wrapping_sub(0x20u64), ((r15 as u8)) as u8); crate::simd_ops::wu8(rax.wrapping_sub(0x1fu64), (0x1u8) as u8); crate::simd_ops::wu8(r14, (0x1u8) as u8); rbp = (crate::simd_ops::ru32(rcx.wrapping_add(0x18u64))) as u32 as u64; crate::simd_ops::wu32(rax.wrapping_sub(0x38u64), ((rbp as u32)) as u32); rdi = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; rax = (crate::simd_ops::ru64(rdi)) as u64; rcx = rdi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x8u64))) as u8 as u64; fl_r = (0xcu8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 4; continue; } bb = 2; continue; }
            2 => { bb = 3; continue; }
            3 => { rdi = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rdi); rax = ((rax).wrapping_add(0x18u64)) as u64; rcx = rsp.wrapping_add(0x20); fl_l = (crate::simd_ops::ru32(rax)) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; rcx = (if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { rax } else { rcx }) as u64; rbp = (crate::simd_ops::ru32(rcx)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rbp as u32)) as u32); rax = (crate::simd_ops::ru64(rdi)) as u64; rdx = rbx; rcx = rdi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x8u64))) as u8 as u64; fl_r = (0xcu8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 3; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rdx = rbx; rcx = rdi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x20u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rax = ((crate::simd_ops::ru8(rdi.wrapping_add(0xau64))) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x2u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 6; continue; } bb = 5; continue; }
            5 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; rax = (rax & !0xffu64) | (((if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { 1u8 } else { 0u8 }) as u8 as u64)); bb = 7; continue; }
            6 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rdx = rbx; rcx = rdi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 7; continue; }
            7 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 13; continue; } bb = 8; continue; }
            8 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rax = ((rax).wrapping_add(1)) as u64; rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 12; continue; } bb = 9; continue; }
            9 => { rdx = ((rdx).wrapping_add(rdx)) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), rdx); fl_l = rdx; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 11; continue; } bb = 10; continue; }
            10 => { crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), rax); rdx = rax; bb = 11; continue; }
            11 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx, rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 33; continue; } bb = 12; continue; }
            12 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rcx = (crate::simd_ops::ru64(rbx)) as u64; rax = ((crate::simd_ops::ru16(P1)) as u32) as u32 as u64; crate::simd_ops::wu8(rdx.wrapping_add(rcx), ((rax as u8)) as u8); fl_l = (crate::simd_ops::lk_add_u64(rbx.wrapping_add(0x8u64), (1) as u64)) as u64; bb = 13; continue; }
            13 => { rax = ((crate::simd_ops::ru8(rdi.wrapping_add(0xau64))) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x2u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 15; continue; } bb = 14; continue; }
            14 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; rax = (rax & !0xffu64) | (((if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { 1u8 } else { 0u8 }) as u8 as u64)); bb = 16; continue; }
            15 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rdx = rbx; rcx = rdi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 16; continue; }
            16 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 21; continue; } bb = 17; continue; }
            17 => { rax = ((crate::simd_ops::ru8(rdi.wrapping_add(0xbu64))) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x2u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 19; continue; } bb = 18; continue; }
            18 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; rax = (rax & !0xffu64) | (((if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { 1u8 } else { 0u8 }) as u8 as u64)); bb = 20; continue; }
            19 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rdx = rbx; rcx = rdi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 20; continue; }
            20 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 26; continue; } bb = 21; continue; }
            21 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rax = ((rax).wrapping_add(1)) as u64; rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 25; continue; } bb = 22; continue; }
            22 => { rdx = ((rdx).wrapping_add(rdx)) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), rdx); fl_l = rdx; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 24; continue; } bb = 23; continue; }
            23 => { crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), rax); rdx = rax; bb = 24; continue; }
            24 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx, rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 33; continue; } bb = 25; continue; }
            25 => { rdx = (crate::simd_ops::ru64(rbx)) as u64; rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rax = ((crate::simd_ops::ru16(P2)) as u32) as u32 as u64; crate::simd_ops::wu8(rdx.wrapping_add(rcx), ((rax as u8)) as u8); fl_l = (crate::simd_ops::lk_add_u64(rbx.wrapping_add(0x8u64), (1) as u64)) as u64; bb = 26; continue; }
            26 => { rax = P3; rsi = P4; fl_l = ((rbp as u32)) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; rsi = (if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { rax } else { rsi }) as u64; rdi = (0xffffffffffffffffu64) as u64; bb = 27; continue; }
            27 => { rdi = ((rdi).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(rsi.wrapping_add(rdi))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 27; continue; } bb = 28; continue; }
            28 => { rax = rsi; rax = ((rax).wrapping_sub(rsi)) as u64; fl_l = ((rdi).wrapping_add(rax)) as u64; fl_r = fl_l; rdi = ((fl_l as u64)) as u64; if (((fl_l as u64)) == 0) { bb = 35; continue; } bb = 29; continue; }
            29 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rcx = ((rcx).wrapping_add(rdi)) as u64; rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; fl_l = rcx; fl_r = rdx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 34; continue; } bb = 30; continue; }
            30 => { rdx = ((rdx).wrapping_add(rdx)) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), rdx); fl_l = rdx; fl_r = rcx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 32; continue; } bb = 31; continue; }
            31 => { crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), rcx); rdx = rcx; bb = 32; continue; }
            32 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx, rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 34; continue; } bb = 33; continue; }
            33 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x1424a4a4e, rcx, rsp); return rax; }
            34 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rbx))) as u64; r8 = rdi; rdx = rsi; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::lk_add_u64(rbx.wrapping_add(0x8u64), (rdi) as u64)) as u64; fl_r = fl_l; bb = 35; continue; }
            35 => { crate::simd_ops::wu8(r14, ((r15 as u8)) as u8); bb = 36; continue; }
            36 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1424a4ab0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x18; const P1: u64 = 0x10; const P2: u64 = 0x8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rax = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64;
    rbx = rdx;
    crate::simd_ops::wu32(rdx, ((rax as u32)) as u32);
    rdi = r8;
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64;
    crate::simd_ops::wu64(rdx.wrapping_add(P2), rcx);
    rdx = r8;
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp);
    fl_l = (crate::simd_ops::ru8(rax.wrapping_add(P2))) as u8 as u64; fl_r = (0xcu8) as u8 as u64;
    if (((fl_l as u8)) != ((fl_r as u8))) { rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    loop { rcx = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(P2), rcx); rdx = (crate::simd_ops::ru32(rax.wrapping_add(P0))) as u32 as u64; rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; rax = (if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { (rdx as u32) } else { (rax as u32) }) as u32 as u64; rdx = rdi; crate::simd_ops::wu32(rbx, ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru8(rax.wrapping_add(P2))) as u8 as u64; fl_r = (0xcu8) as u8 as u64; if !((((fl_l as u8)) == ((fl_r as u8)))) { break; } }
    rax = rbx;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1424a4b60(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14340f840; const P1: u64 = 0x14183acda; const P2: u64 = 0x142646560; const P3: u64 = 0x1426467c8; const P4: u64 = 0x14340f868;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; rbx = rdx; rsi = rcx; fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(0x8u64))) as u8 as u64; fl_r = (0xau8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 4; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x8u64))) as u8 as u64; fl_r = (0x7u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 4; continue; } bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; r8 = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; r8 = ((r8).wrapping_sub(rcx)) as u64; fl_l = r8; fl_r = (0xbu64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rdx = P0; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 19; continue; } bb = 4; continue; }
            4 => { rax = ((crate::simd_ops::ru8(rdi.wrapping_add(0xau64))) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x2u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 6; continue; } bb = 5; continue; }
            5 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; rax = (rax & !0xffu64) | (((if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { 1u8 } else { 0u8 }) as u8 as u64)); bb = 7; continue; }
            6 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rdx = rbx; rcx = rdi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 7; continue; }
            7 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 12; continue; } bb = 8; continue; }
            8 => { rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x10u64))) as u64; rax = ((crate::simd_ops::ru8(rcx.wrapping_add(0xbu64))) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x2u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 10; continue; } bb = 9; continue; }
            9 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; rax = (rax & !0xffu64) | (((if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { 1u8 } else { 0u8 }) as u8 as u64)); bb = 11; continue; }
            10 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 11; continue; }
            11 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 18; continue; } bb = 12; continue; }
            12 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rax = ((rax).wrapping_add(1)) as u64; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 17; continue; } bb = 13; continue; }
            13 => { rdx = ((rdx).wrapping_add(rdx)) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), rdx); fl_l = rdx; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 15; continue; } bb = 14; continue; }
            14 => { crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), rax); rdx = rax; bb = 15; continue; }
            15 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx, rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 17; continue; } bb = 16; continue; }
            16 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x1424a4c27, rcx, rsp); return rax; }
            17 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rax = ((crate::simd_ops::ru16(P4)) as u32) as u32 as u64; rcx = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu8(rdx.wrapping_add(rcx), ((rax as u8)) as u8); fl_l = (crate::simd_ops::lk_add_u64(rbx.wrapping_add(0x8u64), (1) as u64)) as u64; bb = 18; continue; }
            18 => { rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x10u64))) as u64; rdx = rbx; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x28u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 19; continue; }
            19 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1424a4c60(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14340f840; const P1: u64 = 0x14183acda; const P2: u64 = 0x142646560; const P3: u64 = 0x14340f83c; const P4: u64 = 0x14183acc8; const P5: u64 = 0x14333ccd8; const P6: u64 = 0x1433dc2d4; const P7: u64 = 0x1433dc26c; const P8: u64 = 0x1426467c8; const P9: u64 = 0x14340e95c;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; rdi = rcx.wrapping_add(0x10); rbx = rdx; rbp = rcx; fl_l = (crate::simd_ops::ru8(rsi.wrapping_add(0x8u64))) as u8 as u64; fl_r = (0xau8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 22; continue; } bb = 1; continue; }
            1 => { r8 = (crate::simd_ops::ru64(rsi.wrapping_add(0x10u64))) as u64; fl_l = (crate::simd_ops::ru8(r8.wrapping_add(0x8u64))) as u8 as u64; fl_r = (0x7u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 22; continue; } bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru64(r8.wrapping_add(0x10u64))) as u64; r8 = (crate::simd_ops::ru64(r8.wrapping_add(0x18u64))) as u64; r8 = ((r8).wrapping_sub(rcx)) as u64; fl_l = r8; fl_r = (0xbu64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 21; continue; } bb = 3; continue; }
            3 => { rdx = P0; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 21; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rdi = rbx.wrapping_add(0x10); rdx = (crate::simd_ops::ru64(rdi)) as u64; rax = ((rax).wrapping_add(0x3u64)) as u64; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 9; continue; } bb = 5; continue; }
            5 => { rdx = ((rdx).wrapping_add(rdx)) as u64; crate::simd_ops::wu64(rdi, rdx); fl_l = rdx; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 7; continue; } bb = 6; continue; }
            6 => { crate::simd_ops::wu64(rdi, rax); rdx = rax; bb = 7; continue; }
            7 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx, rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 48; continue; } bb = 8; continue; }
            8 => { rdi = rbx.wrapping_add(0x10); bb = 9; continue; }
            9 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rdx = P3; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rbx))) as u64; r14 = rbx.wrapping_add(0x8); r8 = 0x3; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::lk_add_u64(rbx.wrapping_add(0x8u64), (0x3u64) as u64); r15 = (crate::simd_ops::ru64(rsi.wrapping_add(0x18u64))) as u64; rbp = (crate::simd_ops::ru64(rsi.wrapping_add(0x20u64))) as u64; rsi = r14; fl_l = rbp; fl_r = r15; rbp = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 16; continue; } bb = 10; continue; }
            10 => { rcx = (crate::simd_ops::ru64(r14)) as u64; rdx = (crate::simd_ops::ru64(rdi)) as u64; rcx = ((rcx).wrapping_add(rbp)) as u64; fl_l = rcx; fl_r = rdx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 15; continue; } bb = 11; continue; }
            11 => { rdx = ((rdx).wrapping_add(rdx)) as u64; crate::simd_ops::wu64(rdi, rdx); fl_l = rdx; fl_r = rcx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 13; continue; } bb = 12; continue; }
            12 => { crate::simd_ops::wu64(rdi, rcx); rsi = rbx.wrapping_add(0x8); rdx = rcx; bb = 13; continue; }
            13 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx, rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 48; continue; } bb = 14; continue; }
            14 => { rdi = rbx.wrapping_add(0x10); bb = 15; continue; }
            15 => { rcx = (crate::simd_ops::ru64(r14)) as u64; r8 = rbp; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rbx))) as u64; rdx = r15; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::lk_add_u64(rsi, (rbp) as u64)) as u64; fl_r = fl_l; r14 = rsi; bb = 16; continue; }
            16 => { rax = (crate::simd_ops::ru64(r14)) as u64; rbp = rsi; rdx = (crate::simd_ops::ru64(rdi)) as u64; rax = ((rax).wrapping_add(1)) as u64; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 20; continue; } bb = 17; continue; }
            17 => { rdx = ((rdx).wrapping_add(rdx)) as u64; crate::simd_ops::wu64(rdi, rdx); fl_l = rdx; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 19; continue; } bb = 18; continue; }
            18 => { crate::simd_ops::wu64(rdi, rax); rbp = rbx.wrapping_add(0x8); rdx = rax; bb = 19; continue; }
            19 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx, rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 48; continue; } bb = 20; continue; }
            20 => { rdx = (crate::simd_ops::ru64(rsi)) as u64; rax = ((crate::simd_ops::ru16(P5)) as u32) as u32 as u64; rcx = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu8(rdx.wrapping_add(rcx), ((rax as u8)) as u8); fl_l = (crate::simd_ops::lk_add_u64(rbp, (1) as u64)) as u64; bb = 50; continue; }
            21 => { rdi = rbp.wrapping_add(0x10); bb = 22; continue; }
            22 => { rax = (crate::simd_ops::ru64(rsi)) as u64; rdx = rbx; rcx = rsi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x20u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rdi)) as u64; rax = ((crate::simd_ops::ru8(rcx.wrapping_add(0xau64))) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x2u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 24; continue; } bb = 23; continue; }
            23 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; rax = (rax & !0xffu64) | (((if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { 1u8 } else { 0u8 }) as u8 as u64)); bb = 25; continue; }
            24 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 25; continue; }
            25 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 31; continue; } bb = 26; continue; }
            26 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rax = ((rax).wrapping_add(1)) as u64; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 30; continue; } bb = 27; continue; }
            27 => { rdx = ((rdx).wrapping_add(rdx)) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), rdx); fl_l = rdx; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 29; continue; } bb = 28; continue; }
            28 => { crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), rax); rdx = rax; bb = 29; continue; }
            29 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx, rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 48; continue; } bb = 30; continue; }
            30 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rdi = rbp.wrapping_add(0x10); rax = ((crate::simd_ops::ru16(P6)) as u32) as u32 as u64; rdx = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu8(rdx.wrapping_add(rcx), ((rax as u8)) as u8); fl_l = (crate::simd_ops::lk_add_u64(rbx.wrapping_add(0x8u64), (1) as u64)) as u64; bb = 31; continue; }
            31 => { rcx = (crate::simd_ops::ru64(rdi)) as u64; rax = ((crate::simd_ops::ru8(rcx.wrapping_add(0xau64))) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x2u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 33; continue; } bb = 32; continue; }
            32 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; rax = (rax & !0xffu64) | (((if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { 1u8 } else { 0u8 }) as u8 as u64)); bb = 34; continue; }
            33 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 34; continue; }
            34 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 39; continue; } bb = 35; continue; }
            35 => { rcx = (crate::simd_ops::ru64(rdi)) as u64; rax = ((crate::simd_ops::ru8(rcx.wrapping_add(0xbu64))) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x2u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 37; continue; } bb = 36; continue; }
            36 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; rax = (rax & !0xffu64) | (((if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { 1u8 } else { 0u8 }) as u8 as u64)); bb = 38; continue; }
            37 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 38; continue; }
            38 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 44; continue; } bb = 39; continue; }
            39 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rax = ((rax).wrapping_add(1)) as u64; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 43; continue; } bb = 40; continue; }
            40 => { rdx = ((rdx).wrapping_add(rdx)) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), rdx); fl_l = rdx; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 42; continue; } bb = 41; continue; }
            41 => { crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), rax); rdx = rax; bb = 42; continue; }
            42 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx, rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 48; continue; } bb = 43; continue; }
            43 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rax = ((crate::simd_ops::ru16(P7)) as u32) as u32 as u64; rcx = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu8(rdx.wrapping_add(rcx), ((rax as u8)) as u8); fl_l = (crate::simd_ops::lk_add_u64(rbx.wrapping_add(0x8u64), (1) as u64)) as u64; bb = 44; continue; }
            44 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rax = ((rax).wrapping_add(1)) as u64; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 49; continue; } bb = 45; continue; }
            45 => { rdx = ((rdx).wrapping_add(rdx)) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), rdx); fl_l = rdx; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 47; continue; } bb = 46; continue; }
            46 => { crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), rax); rdx = rax; bb = 47; continue; }
            47 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx, rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 49; continue; } bb = 48; continue; }
            48 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P8), rax), rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x1424a4f20, rcx, rsp); return rax; }
            49 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rax = ((crate::simd_ops::ru16(P9)) as u32) as u32 as u64; rcx = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu8(rdx.wrapping_add(rcx), ((rax as u8)) as u8); fl_l = (crate::simd_ops::lk_add_u64(rbx.wrapping_add(0x8u64), (1) as u64)) as u64; bb = 50; continue; }
            50 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1424a4fc0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142646560; const P1: u64 = 0x14339f760; const P2: u64 = 0x14183acc8; const P3: u64 = 0x1426467c8; const P4: u64 = 0x14333ccd8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; rbp = rcx; rcx = rdi; rbx = rdx; rax = (crate::simd_ops::ru64(rdi)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x20u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(0x9u64))) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rdx = rbx; rcx = rdi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x28u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rdi = rbx.wrapping_add(0x10); rdx = (crate::simd_ops::ru64(rdi)) as u64; rax = ((rax).wrapping_add(1)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), r14); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), r15); fl_l = rax; fl_r = rdx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 7; continue; } bb = 3; continue; }
            3 => { rdx = ((rdx).wrapping_add(rdx)) as u64; crate::simd_ops::wu64(rdi, rdx); fl_l = rdx; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 5; continue; } bb = 4; continue; }
            4 => { crate::simd_ops::wu64(rdi, rax); rdx = rax; bb = 5; continue; }
            5 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx, rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 18; continue; } bb = 6; continue; }
            6 => { rdi = rbx.wrapping_add(0x10); bb = 7; continue; }
            7 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; r14 = rbx.wrapping_add(0x8); rax = ((crate::simd_ops::ru16(P1)) as u32) as u32 as u64; rsi = r14; rcx = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu8(rdx.wrapping_add(rcx), ((rax as u8)) as u8); crate::simd_ops::lk_add_u64(rbx.wrapping_add(0x8u64), (1) as u64); r15 = (crate::simd_ops::ru64(rbp.wrapping_add(0x18u64))) as u64; rbp = (crate::simd_ops::ru64(rbp.wrapping_add(0x20u64))) as u64; fl_l = rbp; fl_r = r15; rbp = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 14; continue; } bb = 8; continue; }
            8 => { rcx = (crate::simd_ops::ru64(r14)) as u64; rdx = (crate::simd_ops::ru64(rdi)) as u64; rcx = ((rcx).wrapping_add(rbp)) as u64; fl_l = rcx; fl_r = rdx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 13; continue; } bb = 9; continue; }
            9 => { rdx = ((rdx).wrapping_add(rdx)) as u64; crate::simd_ops::wu64(rdi, rdx); fl_l = rdx; fl_r = rcx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 11; continue; } bb = 10; continue; }
            10 => { crate::simd_ops::wu64(rdi, rcx); rsi = rbx.wrapping_add(0x8); rdx = rcx; bb = 11; continue; }
            11 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx, rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 18; continue; } bb = 12; continue; }
            12 => { rdi = rbx.wrapping_add(0x10); bb = 13; continue; }
            13 => { rcx = (crate::simd_ops::ru64(r14)) as u64; r8 = rbp; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rbx))) as u64; rdx = r15; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::lk_add_u64(rsi, (rbp) as u64)) as u64; fl_r = fl_l; r14 = rsi; bb = 14; continue; }
            14 => { rax = (crate::simd_ops::ru64(r14)) as u64; rbp = rsi; rdx = (crate::simd_ops::ru64(rdi)) as u64; rax = ((rax).wrapping_add(1)) as u64; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 19; continue; } bb = 15; continue; }
            15 => { rdx = ((rdx).wrapping_add(rdx)) as u64; crate::simd_ops::wu64(rdi, rdx); fl_l = rdx; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 17; continue; } bb = 16; continue; }
            16 => { crate::simd_ops::wu64(rdi, rax); rbp = rbx.wrapping_add(0x8); rdx = rax; bb = 17; continue; }
            17 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx, rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 19; continue; } bb = 18; continue; }
            18 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x1424a50ed, rcx, rsp); return rax; }
            19 => { rdx = (crate::simd_ops::ru64(rbx)) as u64; rcx = (crate::simd_ops::ru64(rsi)) as u64; rax = ((crate::simd_ops::ru16(P4)) as u32) as u32 as u64; r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; crate::simd_ops::wu8(rdx.wrapping_add(rcx), ((rax as u8)) as u8); crate::simd_ops::lk_add_u64(rbp, (1) as u64); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1424a5120(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x8; const P2: u64 = 0x18; const P3: u64 = 0x14340f840; const P4: u64 = 0x14183acda;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    fl_l = (crate::simd_ops::ru8(rax.wrapping_add(P1))) as u8 as u64; fl_r = (0x7u8) as u8 as u64;
    if (((fl_l as u8)) != ((fl_r as u8))) { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
    rcx = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64;
    r8 = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64;
    r8 = ((r8).wrapping_sub(rcx)) as u64;
    fl_l = r8; fl_r = (0xbu64) as u64;
    if (((fl_l as u64)) != ((fl_r as u64))) { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
    rdx = P3;
    rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp);
    fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64;
    if !(((((fl_l as u32)) & ((fl_r as u32))) != 0)) { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
    rax = (rax & !0xffu64) | (((0u8) as u8 as u64));
    fl_l = 0; fl_r = 0;
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1424a51b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x142646560; const P3: u64 = 0x1426467c8; const P4: u64 = 0x14340f850; const P5: u64 = 0x14183acc8; const P6: u64 = 0x1424a5d60; const P7: u64 = 0x1424a6050;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; rbx = rdx; rdx = (crate::simd_ops::ru64(rdx.wrapping_add(P1))) as u64; rax = ((rax).wrapping_add(0xcu64)) as u64; rdi = rcx; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 5; continue; } bb = 1; continue; }
            1 => { rdx = ((rdx).wrapping_add(rdx)) as u64; crate::simd_ops::wu64(rbx.wrapping_add(P1), rdx); fl_l = rdx; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 3; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rbx.wrapping_add(P1), rax); rdx = rax; bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx, rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x1424a51fb, rcx, rsp); return rax; }
            5 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64; rdx = P4; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rbx))) as u64; r8 = 0xc; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::lk_add_u64(rbx.wrapping_add(P0), (0xcu64) as u64); rcx = ((rdi.wrapping_add(P1)) as u64) as u64; rdx = rbx; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rdx = (rdx & !0xffu64) | (((0x5du8) as u8 as u64)); rcx = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P7, rax, rcx, rdx, r8, r9, rsp); }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1424a5280(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142646560; const P1: u64 = 0x14340f860; const P2: u64 = 0x14183acc8; const P3: u64 = 0x1426467c8; const P4: u64 = 0x1433e091c;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbp = rcx; rbx = rdx; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x20u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rdi = rbx.wrapping_add(0x10); rdx = (crate::simd_ops::ru64(rdi)) as u64; rax = ((rax).wrapping_add(0x5u64)) as u64; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 5; continue; } bb = 1; continue; }
            1 => { rdx = ((rdx).wrapping_add(rdx)) as u64; crate::simd_ops::wu64(rdi, rdx); fl_l = rdx; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 3; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rdi, rax); rdx = rax; bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx, rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 16; continue; } bb = 4; continue; }
            4 => { rdi = rbx.wrapping_add(0x10); bb = 5; continue; }
            5 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rdx = P1; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rbx))) as u64; r14 = rbx.wrapping_add(0x8); r8 = 0x5; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::lk_add_u64(rbx.wrapping_add(0x8u64), (0x5u64) as u64); rsi = r14; r15 = (crate::simd_ops::ru64(rbp.wrapping_add(0x18u64))) as u64; rbp = (crate::simd_ops::ru64(rbp.wrapping_add(0x20u64))) as u64; fl_l = rbp; fl_r = r15; rbp = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 12; continue; } bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru64(r14)) as u64; rdx = (crate::simd_ops::ru64(rdi)) as u64; rcx = ((rcx).wrapping_add(rbp)) as u64; fl_l = rcx; fl_r = rdx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 11; continue; } bb = 7; continue; }
            7 => { rdx = ((rdx).wrapping_add(rdx)) as u64; crate::simd_ops::wu64(rdi, rdx); fl_l = rdx; fl_r = rcx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 9; continue; } bb = 8; continue; }
            8 => { crate::simd_ops::wu64(rdi, rcx); rsi = rbx.wrapping_add(0x8); rdx = rcx; bb = 9; continue; }
            9 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx, rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 16; continue; } bb = 10; continue; }
            10 => { rdi = rbx.wrapping_add(0x10); bb = 11; continue; }
            11 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; r8 = rbp; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(r14))) as u64; rdx = r15; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::lk_add_u64(rsi, (rbp) as u64)) as u64; fl_r = fl_l; r14 = rsi; bb = 12; continue; }
            12 => { rax = (crate::simd_ops::ru64(r14)) as u64; rbp = rsi; rdx = (crate::simd_ops::ru64(rdi)) as u64; rax = ((rax).wrapping_add(1)) as u64; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 17; continue; } bb = 13; continue; }
            13 => { rdx = ((rdx).wrapping_add(rdx)) as u64; crate::simd_ops::wu64(rdi, rdx); fl_l = rdx; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 15; continue; } bb = 14; continue; }
            14 => { crate::simd_ops::wu64(rdi, rax); rbp = rbx.wrapping_add(0x8); rdx = rax; bb = 15; continue; }
            15 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx, rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 17; continue; } bb = 16; continue; }
            16 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x1424a53a4, rcx, rsp); return rax; }
            17 => { rdx = (crate::simd_ops::ru64(rsi)) as u64; rcx = (crate::simd_ops::ru64(rbx)) as u64; rax = ((crate::simd_ops::ru16(P4)) as u32) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; crate::simd_ops::wu8(rdx.wrapping_add(rcx), ((rax as u8)) as u8); crate::simd_ops::lk_add_u64(rbp, (1) as u64); rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1424a5420(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x18; const P2: u64 = 0x8; const P3: u64 = 0x142646560; const P4: u64 = 0x1426467c8; const P5: u64 = 0x14183acc8; const P6: u64 = 0x1424a6050; const P7: u64 = 0x20; const P8: u64 = 0x9; const P9: u64 = 0x28;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbp = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rbx = rdx; rdi = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; rsi = rcx; fl_l = rdi; fl_r = rbp; rdi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 7; continue; } bb = 1; continue; }
            1 => { rdx = (crate::simd_ops::ru64(rdx.wrapping_add(P2))) as u64; rax = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64; rdx = ((rdx).wrapping_add(rdi)) as u64; fl_l = rdx; fl_r = rax; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 6; continue; } bb = 2; continue; }
            2 => { rax = ((rax).wrapping_add(rax)) as u64; crate::simd_ops::wu64(rbx.wrapping_add(P0), rax); fl_l = rax; fl_r = rdx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu64(rbx.wrapping_add(P0), rdx); rax = rdx; bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; rdx = rax; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx, rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x1424a5484, rcx, rsp); return rax; }
            6 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; r8 = rdi; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rbx.wrapping_add(P2)))) as u64; rdx = rbp; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::lk_add_u64(rbx.wrapping_add(P2), (rdi) as u64)) as u64; fl_r = fl_l; bb = 7; continue; }
            7 => { rdx = (rdx & !0xffu64) | (((0x20u8) as u8 as u64)); rcx = rbx; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rdi = (crate::simd_ops::ru64(rsi.wrapping_add(P7))) as u64; rdx = rbx; rcx = rdi; rax = (crate::simd_ops::ru64(rdi)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P7))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(P8))) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 9; continue; } bb = 8; continue; }
            8 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rdx = rbx; rcx = rdi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P9))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 9; continue; }
            9 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1424a5530(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x18; const P2: u64 = 0x8; const P3: u64 = 0x142646560; const P4: u64 = 0x1426467c8; const P5: u64 = 0x14183acc8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rbx = rdx; rdi = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; fl_l = rdi; fl_r = rsi; rdi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 7; continue; } bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P2))) as u64; rdx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; rcx = ((rcx).wrapping_add(rdi)) as u64; fl_l = rcx; fl_r = rdx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 6; continue; } bb = 2; continue; }
            2 => { rdx = ((rdx).wrapping_add(rdx)) as u64; crate::simd_ops::wu64(rbx.wrapping_add(P0), rdx); fl_l = rdx; fl_r = rcx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu64(rbx.wrapping_add(P0), rcx); rdx = rcx; bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx, rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x1424a5589, rcx, rsp); return rax; }
            6 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; r8 = rdi; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rbx.wrapping_add(P2)))) as u64; rdx = rsi; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::lk_add_u64(rbx.wrapping_add(P2), (rdi) as u64)) as u64; fl_r = fl_l; bb = 7; continue; }
            7 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1424a5640(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x20; const P2: u64 = 0x18; const P3: u64 = 0x8; const P4: u64 = 0x142646560; const P5: u64 = 0x1426467c8; const P6: u64 = 0x14183acc8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rcx; rdi = rdx; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rsi = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64; rbx = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64; fl_l = rbx; fl_r = rsi; rbx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 7; continue; } bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P3))) as u64; rdx = (crate::simd_ops::ru64(rdi.wrapping_add(P0))) as u64; rcx = ((rcx).wrapping_add(rbx)) as u64; fl_l = rcx; fl_r = rdx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 6; continue; } bb = 2; continue; }
            2 => { rdx = ((rdx).wrapping_add(rdx)) as u64; crate::simd_ops::wu64(rdi.wrapping_add(P0), rdx); fl_l = rdx; fl_r = rcx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu64(rdi.wrapping_add(P0), rcx); rdx = rcx; bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru64(rdi)) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rdi, rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x1424a56a6, rcx, rsp); return rax; }
            6 => { rcx = (crate::simd_ops::ru64(rdi)) as u64; r8 = rbx; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rdi.wrapping_add(P3)))) as u64; rdx = rsi; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::lk_add_u64(rdi.wrapping_add(P3), (rbx) as u64)) as u64; fl_r = fl_l; bb = 7; continue; }
            7 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1424a5720(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x142646560; const P3: u64 = 0x1426467c8; const P4: u64 = 0x14340f808; const P5: u64 = 0x14183acc8; const P6: u64 = 0x20; const P7: u64 = 0x9; const P8: u64 = 0x28;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; rbx = rdx; rdx = (crate::simd_ops::ru64(rdx.wrapping_add(P1))) as u64; rax = ((rax).wrapping_add(0x9u64)) as u64; rdi = rcx; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 5; continue; } bb = 1; continue; }
            1 => { rdx = ((rdx).wrapping_add(rdx)) as u64; crate::simd_ops::wu64(rbx.wrapping_add(P1), rdx); fl_l = rdx; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 3; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rbx.wrapping_add(P1), rax); rdx = rax; bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx, rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x1424a576b, rcx, rsp); return rax; }
            5 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64; rdx = P4; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rbx))) as u64; r8 = 0x9; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::lk_add_u64(rbx.wrapping_add(P0), (0x9u64) as u64); rdx = rbx; rdi = (crate::simd_ops::ru64(rdi.wrapping_add(P1))) as u64; rcx = rdi; rax = (crate::simd_ops::ru64(rdi)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P6))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(P7))) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rdx = rbx; rcx = rdi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P8))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 7; continue; }
            7 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1424a5820(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x18; const P1: u64 = 0x20; const P2: u64 = 0x1424a58c0;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdi = rcx;
    rbx = rdx;
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp);
    rdx = rbx;
    rcx = rdi;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(P2, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1424a58c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142646560; const P1: u64 = 0x14340f834; const P2: u64 = 0x14183acc8; const P3: u64 = 0x14340f828; const P4: u64 = 0x1426467c8; const P5: u64 = 0x14340f818;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0x10u64))) as u8 as u64; fl_r = (0x1u8) as u8 as u64; rdi = rcx.wrapping_add(0x10); rbx = rdx; rsi = rcx; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 7; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; rdx = (crate::simd_ops::ru64(rdx.wrapping_add(0x10u64))) as u64; rax = ((rax).wrapping_add(0x6u64)) as u64; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 6; continue; } bb = 2; continue; }
            2 => { rdx = ((rdx).wrapping_add(rdx)) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), rdx); fl_l = rdx; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), rax); rdx = rax; bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx, rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 19; continue; } bb = 5; continue; }
            5 => { rdi = rsi.wrapping_add(0x10); bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rdx = P1; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rbx))) as u64; r8 = 0x6; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::lk_add_u64(rbx.wrapping_add(0x8u64), (0x6u64) as u64)) as u64; fl_r = fl_l; bb = 7; continue; }
            7 => { fl_l = (crate::simd_ops::ru8(rdi)) as u8 as u64; fl_r = (0x2u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 14; continue; } bb = 8; continue; }
            8 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rax = ((rax).wrapping_add(0x9u64)) as u64; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 13; continue; } bb = 9; continue; }
            9 => { rdx = ((rdx).wrapping_add(rdx)) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), rdx); fl_l = rdx; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 11; continue; } bb = 10; continue; }
            10 => { crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), rax); rdx = rax; bb = 11; continue; }
            11 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx, rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 19; continue; } bb = 12; continue; }
            12 => { rdi = rsi.wrapping_add(0x10); bb = 13; continue; }
            13 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; rdx = P3; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rbx.wrapping_add(0x8u64)))) as u64; r8 = 0x9; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::lk_add_u64(rbx.wrapping_add(0x8u64), (0x9u64) as u64)) as u64; fl_r = fl_l; bb = 14; continue; }
            14 => { fl_l = (crate::simd_ops::ru8(rdi)) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 21; continue; } bb = 15; continue; }
            15 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rax = ((rax).wrapping_add(0x9u64)) as u64; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 20; continue; } bb = 16; continue; }
            16 => { rdx = ((rdx).wrapping_add(rdx)) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), rdx); fl_l = rdx; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 18; continue; } bb = 17; continue; }
            17 => { crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), rax); rdx = rax; bb = 18; continue; }
            18 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx, rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 20; continue; } bb = 19; continue; }
            19 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x1424a59d6, rcx, rsp); return rax; }
            20 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; rdx = P5; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rbx.wrapping_add(0x8u64)))) as u64; r8 = 0x9; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::lk_add_u64(rbx.wrapping_add(0x8u64), (0x9u64) as u64)) as u64; fl_r = fl_l; bb = 21; continue; }
            21 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1424a5a50(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142646560; const P1: u64 = 0x1433dc2d4; const P2: u64 = 0x1426467c8; const P3: u64 = 0x14183acc8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; rbp = rcx; rcx = rdi; rbx = rdx; rax = (crate::simd_ops::ru64(rdi)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x20u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(0x9u64))) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rdx = rbx; rcx = rdi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x28u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rax = ((rax).wrapping_add(1)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), r14); fl_l = rax; fl_r = rdx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 6; continue; } bb = 3; continue; }
            3 => { rdx = ((rdx).wrapping_add(rdx)) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), rdx); fl_l = rdx; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 5; continue; } bb = 4; continue; }
            4 => { crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), rax); rdx = rax; bb = 5; continue; }
            5 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx, rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 6; continue; }
            6 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rdi = rbx.wrapping_add(0x8); rax = ((crate::simd_ops::ru16(P1)) as u32) as u32 as u64; rcx = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu8(rdx.wrapping_add(rcx), ((rax as u8)) as u8); crate::simd_ops::lk_add_u64(rbx.wrapping_add(0x8u64), (1) as u64); r14 = (crate::simd_ops::ru64(rbp.wrapping_add(0x18u64))) as u64; rbp = (crate::simd_ops::ru64(rbp.wrapping_add(0x20u64))) as u64; fl_l = rbp; fl_r = r14; rbp = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 13; continue; } bb = 7; continue; }
            7 => { rcx = (crate::simd_ops::ru64(rdi)) as u64; rsi = rdi; rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rcx = ((rcx).wrapping_add(rbp)) as u64; fl_l = rcx; fl_r = rdx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 12; continue; } bb = 8; continue; }
            8 => { rdx = ((rdx).wrapping_add(rdx)) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), rdx); fl_l = rdx; fl_r = rcx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 10; continue; } bb = 9; continue; }
            9 => { crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), rcx); rsi = rbx.wrapping_add(0x8); rdx = rcx; bb = 10; continue; }
            10 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx, rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 12; continue; } bb = 11; continue; }
            11 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x1424a5b28, rcx, rsp); return rax; }
            12 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; r8 = rbp; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rdi))) as u64; rdx = r14; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::lk_add_u64(rsi, (rbp) as u64)) as u64; fl_r = fl_l; bb = 13; continue; }
            13 => { r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1424a5ba0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142646560; const P1: u64 = 0x1433dc270; const P2: u64 = 0x14183acc8; const P3: u64 = 0x1426467c8; const P4: u64 = 0x14340f868;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; rbp = rcx; rcx = rdi; rbx = rdx; rax = (crate::simd_ops::ru64(rdi)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x20u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(0x9u64))) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rdx = rbx; rcx = rdi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x28u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rdi = rbx.wrapping_add(0x10); rdx = (crate::simd_ops::ru64(rdi)) as u64; rax = ((rax).wrapping_add(0x2u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), r14); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), r15); fl_l = rax; fl_r = rdx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 7; continue; } bb = 3; continue; }
            3 => { rdx = ((rdx).wrapping_add(rdx)) as u64; crate::simd_ops::wu64(rdi, rdx); fl_l = rdx; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 5; continue; } bb = 4; continue; }
            4 => { crate::simd_ops::wu64(rdi, rax); rdx = rax; bb = 5; continue; }
            5 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx, rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 18; continue; } bb = 6; continue; }
            6 => { rdi = rbx.wrapping_add(0x10); bb = 7; continue; }
            7 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; r14 = rbx.wrapping_add(0x8); rax = ((crate::simd_ops::ru16(P1)) as u32) as u32 as u64; rsi = r14; rcx = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu16(rdx.wrapping_add(rcx), ((rax as u16)) as u16); crate::simd_ops::lk_add_u64(rbx.wrapping_add(0x8u64), (0x2u64) as u64); r15 = (crate::simd_ops::ru64(rbp.wrapping_add(0x18u64))) as u64; rbp = (crate::simd_ops::ru64(rbp.wrapping_add(0x20u64))) as u64; fl_l = rbp; fl_r = r15; rbp = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 14; continue; } bb = 8; continue; }
            8 => { rcx = (crate::simd_ops::ru64(r14)) as u64; rdx = (crate::simd_ops::ru64(rdi)) as u64; rcx = ((rcx).wrapping_add(rbp)) as u64; fl_l = rcx; fl_r = rdx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 13; continue; } bb = 9; continue; }
            9 => { rdx = ((rdx).wrapping_add(rdx)) as u64; crate::simd_ops::wu64(rdi, rdx); fl_l = rdx; fl_r = rcx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 11; continue; } bb = 10; continue; }
            10 => { crate::simd_ops::wu64(rdi, rcx); rsi = rbx.wrapping_add(0x8); rdx = rcx; bb = 11; continue; }
            11 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx, rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 18; continue; } bb = 12; continue; }
            12 => { rdi = rbx.wrapping_add(0x10); bb = 13; continue; }
            13 => { rcx = (crate::simd_ops::ru64(r14)) as u64; r8 = rbp; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rbx))) as u64; rdx = r15; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::lk_add_u64(rsi, (rbp) as u64)) as u64; fl_r = fl_l; r14 = rsi; bb = 14; continue; }
            14 => { rax = (crate::simd_ops::ru64(r14)) as u64; rbp = rsi; rdx = (crate::simd_ops::ru64(rdi)) as u64; rax = ((rax).wrapping_add(1)) as u64; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 19; continue; } bb = 15; continue; }
            15 => { rdx = ((rdx).wrapping_add(rdx)) as u64; crate::simd_ops::wu64(rdi, rdx); fl_l = rdx; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 17; continue; } bb = 16; continue; }
            16 => { crate::simd_ops::wu64(rdi, rax); rbp = rbx.wrapping_add(0x8); rdx = rax; bb = 17; continue; }
            17 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx, rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 19; continue; } bb = 18; continue; }
            18 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x1424a5cd0, rcx, rsp); return rax; }
            19 => { rdx = (crate::simd_ops::ru64(rbx)) as u64; rcx = (crate::simd_ops::ru64(rsi)) as u64; rax = ((crate::simd_ops::ru16(P4)) as u32) as u32 as u64; r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; crate::simd_ops::wu8(rdx.wrapping_add(rcx), ((rax as u8)) as u8); crate::simd_ops::lk_add_u64(rbp, (1) as u64); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1424a5d60(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142646560; const P1: u64 = 0x14332bab8; const P2: u64 = 0x1426467c8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = 0; fl_l = 0; fl_r = 0; rbx = rdx; r12 = rcx; r15 = (r15 & !0xffu64) | (((0x1u8) as u8 as u64)); fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; fl_r = rsi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 15; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), r14); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rdi); bb = 2; continue; }
            2 => { rbp = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; r14 = rbp; fl_l = ((r15 as u8)) as u8 as u64; fl_r = ((r15 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 8; continue; } bb = 3; continue; }
            3 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rax = rbp.wrapping_add(0x2); fl_l = rax; fl_r = rdx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 7; continue; } bb = 4; continue; }
            4 => { rdx = ((rdx).wrapping_add(rdx)) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), rdx); fl_l = rdx; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), rax); rdx = rax; bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx, rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 16; continue; } bb = 7; continue; }
            7 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rax = ((crate::simd_ops::ru16(P1)) as u32) as u32 as u64; rdx = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu16(rdx.wrapping_add(rcx), ((rax as u16)) as u16); fl_l = (crate::simd_ops::lk_add_u64(rbx.wrapping_add(0x8u64), (0x2u64) as u64)) as u64; fl_r = fl_l; r14 = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; bb = 8; continue; }
            8 => { rax = (crate::simd_ops::ru64(r12)) as u64; rdx = rbx; rdi = (crate::simd_ops::ru64(rax.wrapping_add(rsi.wrapping_mul(8)))) as u64; rcx = rdi; rax = (crate::simd_ops::ru64(rdi)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x20u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(0x9u64))) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 10; continue; } bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rdx = rbx; rcx = rdi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x28u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 10; continue; }
            10 => { fl_l = r14; fl_r = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 12; continue; } bb = 11; continue; }
            11 => { crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), rbp); bb = 13; continue; }
            12 => { r15 = (r15 & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 13; continue; }
            13 => { rsi = ((rsi).wrapping_add(1)) as u64; fl_l = rsi; fl_r = (crate::simd_ops::ru64(r12.wrapping_add(0x8u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 14; continue; }
            14 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; bb = 15; continue; }
            15 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            16 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x1424a5e51, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1424a5e60(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    rax = (crate::simd_ops::ru64(rax.wrapping_add(rdx.wrapping_mul(8)))) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1424a5f00(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x20; const P1: u64 = 0x9; const P2: u64 = 0x28;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    rdi = rdx;
    rbx = rcx;
    rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp);
    fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(P1))) as u8 as u64; fl_r = (0x1u8) as u8 as u64;
    if !((((fl_l as u8)) == ((fl_r as u8)))) { rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = rdi; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1424a5fd0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x9;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = ((crate::simd_ops::ru8(rcx.wrapping_add(P0))) as u32) as u32 as u64;
    fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x2u8) as u8 as u64;
    if !((((fl_l as u8)) == ((fl_r as u8)))) { fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; rax = (rax & !0xffu64) | (((if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { 1u8 } else { 0u8 }) as u8 as u64)); return rax; }
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    return crate::simd_ops::call_fwd(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1424a5ff0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x14340f888; const P1: u64 = 0x8;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = P0;
    crate::simd_ops::wu32(rcx.wrapping_add(P1), (0x101011du32) as u32);
    crate::simd_ops::wu64(rcx, rax);
    rax = rcx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1424a6010(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xffffffffffffffff;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    fl_l = rdx; fl_r = rdx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = ((crate::simd_ops::ru8(rdx.wrapping_add(rax).wrapping_add(P1))) as u32) as u32 as u64; return rax; }
    rax = (rax & !0xffu64) | (((0u8) as u8 as u64));
    fl_l = 0; fl_r = 0;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1424a6050(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x142646560; const P3: u64 = 0x1426467c8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rdi = (((rdx as u8)) as u32) as u32 as u64;
    rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64;
    rax = ((rax).wrapping_add(1)) as u64;
    rbx = rcx;
    fl_l = rax; fl_r = rdx;
    if (((fl_l as u64)) < ((fl_r as u64))) { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64; rax = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu8(rcx.wrapping_add(rax), ((rdi as u8)) as u8); rax = rbx; crate::simd_ops::lk_add_u64(rbx.wrapping_add(P0), (1) as u64); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    rdx = ((rdx).wrapping_add(rdx)) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(P1), rdx);
    fl_l = rdx; fl_r = rax;
    if !((((fl_l as u64)) >= ((fl_r as u64)))) { crate::simd_ops::wu64(rcx.wrapping_add(P1), rax); rdx = rax; }
    rcx = (crate::simd_ops::ru64(rcx)) as u64;
    rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp);
    crate::simd_ops::wu64(rbx, rax);
    fl_l = rax; fl_r = rax;
    if !(((((fl_l as u64)) & ((fl_r as u64))) != 0)) { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x1424a609a, rcx, rsp); return rax; }
    rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64;
    rax = (crate::simd_ops::ru64(rbx)) as u64;
    crate::simd_ops::wu8(rcx.wrapping_add(rax), ((rdi as u8)) as u8);
    rax = rbx;
    crate::simd_ops::lk_add_u64(rbx.wrapping_add(P0), (1) as u64);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1424a60c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x142646560; const P3: u64 = 0x1426467c8; const P4: u64 = 0x14183acc8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = (crate::simd_ops::ru64(rdx)) as u64; rbx = rcx; rdi = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_l = rdi; fl_r = rsi; rdi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 7; continue; } bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64; rcx = ((rcx).wrapping_add(rdi)) as u64; fl_l = rcx; fl_r = rdx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 6; continue; } bb = 2; continue; }
            2 => { rdx = ((rdx).wrapping_add(rdx)) as u64; crate::simd_ops::wu64(rbx.wrapping_add(P1), rdx); fl_l = rdx; fl_r = rcx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu64(rbx.wrapping_add(P1), rcx); rdx = rcx; bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx, rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x1424a6118, rcx, rsp); return rax; }
            6 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; r8 = rdi; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rbx.wrapping_add(P0)))) as u64; rdx = rsi; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::lk_add_u64(rbx.wrapping_add(P0), (rdi) as u64)) as u64; fl_r = fl_l; bb = 7; continue; }
            7 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1424a6150(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x142646560; const P3: u64 = 0x1426467c8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rcx = ((rcx).wrapping_add(rdx)) as u64;
    rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64;
    fl_l = rcx; fl_r = rdx;
    if (((fl_l as u64)) < ((fl_r as u64))) { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    rdx = ((rdx).wrapping_add(rdx)) as u64;
    crate::simd_ops::wu64(rbx.wrapping_add(P1), rdx);
    fl_l = rdx; fl_r = rcx;
    if !((((fl_l as u64)) >= ((fl_r as u64)))) { crate::simd_ops::wu64(rbx.wrapping_add(P1), rcx); rdx = rcx; }
    rcx = (crate::simd_ops::ru64(rbx)) as u64;
    rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp);
    crate::simd_ops::wu64(rbx, rax);
    fl_l = rax; fl_r = rax;
    if !(((((fl_l as u64)) & ((fl_r as u64))) != 0)) { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x1424a6193, rcx, rsp); return rax; }
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1424a61a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x14183acda;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    r9 = (crate::simd_ops::ru64(rcx)) as u64;
    r8 = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rcx = (crate::simd_ops::ru64(rdx)) as u64;
    r8 = ((r8).wrapping_sub(r9)) as u64;
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    rax = ((rax).wrapping_sub(rcx)) as u64;
    fl_l = r8; fl_r = rax;
    if (((fl_l as u64)) != ((fl_r as u64))) { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
    rdx = rcx;
    rcx = r9;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64;
    if !(((((fl_l as u32)) & ((fl_r as u32))) != 0)) { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
    rax = (rax & !0xffu64) | (((0u8) as u8 as u64));
    fl_l = 0; fl_r = 0;
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1424a61e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    fl_l = rax; fl_r = (crate::simd_ops::ru64(rcx)) as u64;
    rax = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1424a6220(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xffffffffffffffff;
    let (mut rax, mut r10, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    r9 = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    r8 = 0x1;
    r10 = (crate::simd_ops::ru64(rcx)) as u64;
    rax = r9;
    rax = ((rax).wrapping_sub(r10)) as u64;
    fl_l = rax; fl_r = r8;
    if !((((fl_l as u64)) > ((fl_r as u64)))) { r8 = ((rax.wrapping_add(P1)) as u64) as u64; }
    rax = ((r10.wrapping_add(r8)) as u64) as u64;
    crate::simd_ops::wu64(rdx.wrapping_add(P0), r9);
    crate::simd_ops::wu64(rdx, rax);
    rax = rdx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1424a6280(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    crate::simd_ops::wu64(rcx, rdx);
    rax = (0xffffffffffffffffu64) as u64;
    loop { rax = ((rax).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(rax))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if !((((fl_l as u8)) != ((fl_r as u8)))) { break; } }
    fl_l = ((rax).wrapping_add(rdx)) as u64; fl_r = fl_l;
    rax = ((fl_l as u64)) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(P0), rax);
    rax = rcx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1424a6420(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1426456c8; const P1: u64 = 0x1426453a0;
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = ((rcx as u32)) as u32 as u64;
    rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp);
    rcx = rax;
    rdx = ((rbx as u32)) as u32 as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1424a6440(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143c1bf80; const P1: u64 = 0x143c1bf83; const P2: u64 = 0x143c1bf84; const P3: u64 = 0x143c1bf88; const P4: u64 = 0x143c1bf98; const P5: u64 = 0x142409b50; const P6: u64 = 0x1424a6900; const P7: u64 = 0x143c1bfa4; const P8: u64 = 0x142645398; const P9: u64 = 0x142645708; const P10: u64 = 0x143c1bfa0; const P11: u64 = 0x143c1bf9c;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedStack::<1>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x50u64)) as u64; rbp = rsp.wrapping_add(0x20); crate::simd_ops::wu64(rbp.wrapping_add(0x58u64), rbx); crate::simd_ops::wu64(rbp.wrapping_add(0x60u64), rsi); r14 = P0; crate::simd_ops::wu64(rbp.wrapping_add(0x50u64), r14); rax = (crate::simd_ops::ru32(P0)) as u32 as u64; rax = ((((rax as u32)) & (0xff000002u32))) as u32 as u64; rax = ((((rax as u32)) | (0x2u32))) as u32 as u64; crate::simd_ops::wu32(P0, ((rax as u32)) as u32); crate::simd_ops::wu8(P1, (0x2u8) as u8); rsi = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(P2, ((rsi as u32)) as u32); crate::simd_ops::wu8(P3, ((rsi as u8)) as u8); xmm0 = 0u128; crate::simd_ops::wu128(P4, (xmm0) as u128); xmm1 = 0u128; rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu128(rbp.wrapping_add(0x10u64), (xmm1) as u128); crate::simd_ops::wu32(rbp.wrapping_add(0x20u64), ((rax as u32)) as u32); rcx = 0; fl_l = 0; fl_r = 0; let (cp_a, cp_b, cp_c, cp_d) = crate::simd_ops::cpuid_x64(rax as u32, rcx as u32); rax = cp_a; rbx = cp_b; rcx = cp_c; rdx = cp_d; rdi = ((rax as u32)) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x10u64), ((rax as u32)) as u32); crate::simd_ops::wu32(rbp.wrapping_add(0x14u64), ((rbx as u32)) as u32); crate::simd_ops::wu32(rbp.wrapping_add(0x18u64), ((rdx as u32)) as u32); crate::simd_ops::wu32(rbp.wrapping_add(0x1cu64), ((rcx as u32)) as u32); rax = rbp.wrapping_add(0x14); r9 = (0xffffffffffffffffu64) as u64; bb = 1; continue; }
            1 => { r9 = ((r9).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(r9))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 1; continue; } bb = 2; continue; }
            2 => { rax = rbp.wrapping_add(0x14); crate::simd_ops::wu64(rbp, rax); crate::simd_ops::wu64(rbp.wrapping_add(0x8u64), r9); r8 = 0; fl_l = 0; fl_r = 0; rdx = rbp; rcx = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(P4, ((rax as u32)) as u32); rax = 0x80000000; rcx = 0; fl_l = 0; fl_r = 0; let (cp_a, cp_b, cp_c, cp_d) = crate::simd_ops::cpuid_x64(rax as u32, rcx as u32); rax = cp_a; rbx = cp_b; rcx = cp_c; rdx = cp_d; r9 = ((rax as u32)) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x10u64), ((rax as u32)) as u32); crate::simd_ops::wu32(rbp.wrapping_add(0x14u64), ((rbx as u32)) as u32); crate::simd_ops::wu32(rbp.wrapping_add(0x18u64), ((rcx as u32)) as u32); crate::simd_ops::wu32(rbp.wrapping_add(0x1cu64), ((rdx as u32)) as u32); r8 = (crate::simd_ops::ru32(P7)) as u32 as u64; fl_l = ((rdi as u32)) as u32 as u64; fl_r = 0x1; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 27; continue; } bb = 3; continue; }
            3 => { rax = 0x1; rcx = 0; fl_l = 0; fl_r = 0; let (cp_a, cp_b, cp_c, cp_d) = crate::simd_ops::cpuid_x64(rax as u32, rcx as u32); rax = cp_a; rbx = cp_b; rcx = cp_c; rdx = cp_d; crate::simd_ops::wu32(rbp.wrapping_add(0x10u64), ((rax as u32)) as u32); crate::simd_ops::wu32(rbp.wrapping_add(0x14u64), ((rbx as u32)) as u32); crate::simd_ops::wu32(rbp.wrapping_add(0x18u64), ((rcx as u32)) as u32); crate::simd_ops::wu32(rbp.wrapping_add(0x1cu64), ((rdx as u32)) as u32); fl_l = ((rcx as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { fl_l = ((((r8 as u32)) >> (4u32)) & 1) as u64; r8 = (((r8 as u32)) | ((1u32) << (4u32))) as u32 as u64; crate::simd_ops::wu32(P7, ((r8 as u32)) as u32); bb = 5; continue; }
            5 => { fl_l = ((rcx as u8)) as u8 as u64; fl_r = (0x20u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { fl_l = ((((r8 as u32)) >> (25u32)) & 1) as u64; r8 = (((r8 as u32)) | ((1u32) << (25u32))) as u32 as u64; crate::simd_ops::wu32(P7, ((r8 as u32)) as u32); bb = 7; continue; }
            7 => { fl_l = ((((rcx as u32)) >> (9u32)) & 1) as u64; if ((fl_l as u32) == 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { fl_l = ((((r8 as u32)) >> (5u32)) & 1) as u64; r8 = (((r8 as u32)) | ((1u32) << (5u32))) as u32 as u64; crate::simd_ops::wu32(P7, ((r8 as u32)) as u32); bb = 9; continue; }
            9 => { fl_l = ((((rcx as u32)) >> (12u32)) & 1) as u64; if ((fl_l as u32) == 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { fl_l = ((((r8 as u32)) >> (11u32)) & 1) as u64; r8 = (((r8 as u32)) | ((1u32) << (11u32))) as u32 as u64; crate::simd_ops::wu32(P7, ((r8 as u32)) as u32); bb = 11; continue; }
            11 => { fl_l = ((((rcx as u32)) >> (19u32)) & 1) as u64; if ((fl_l as u32) == 0) { bb = 13; continue; } bb = 12; continue; }
            12 => { fl_l = ((((r8 as u32)) >> (6u32)) & 1) as u64; r8 = (((r8 as u32)) | ((1u32) << (6u32))) as u32 as u64; crate::simd_ops::wu32(P7, ((r8 as u32)) as u32); bb = 13; continue; }
            13 => { fl_l = ((((rcx as u32)) >> (20u32)) & 1) as u64; if ((fl_l as u32) == 0) { bb = 15; continue; } bb = 14; continue; }
            14 => { fl_l = ((((r8 as u32)) >> (7u32)) & 1) as u64; r8 = (((r8 as u32)) | ((1u32) << (7u32))) as u32 as u64; crate::simd_ops::wu32(P7, ((r8 as u32)) as u32); bb = 15; continue; }
            15 => { fl_l = ((((rcx as u32)) >> (23u32)) & 1) as u64; if ((fl_l as u32) == 0) { bb = 17; continue; } bb = 16; continue; }
            16 => { fl_l = ((((r8 as u32)) >> (24u32)) & 1) as u64; r8 = (((r8 as u32)) | ((1u32) << (24u32))) as u32 as u64; crate::simd_ops::wu32(P7, ((r8 as u32)) as u32); bb = 17; continue; }
            17 => { fl_l = ((((rcx as u32)) >> (28u32)) & 1) as u64; if ((fl_l as u32) == 0) { bb = 19; continue; } bb = 18; continue; }
            18 => { fl_l = ((((r8 as u32)) >> (9u32)) & 1) as u64; r8 = (((r8 as u32)) | ((1u32) << (9u32))) as u32 as u64; crate::simd_ops::wu32(P7, ((r8 as u32)) as u32); bb = 19; continue; }
            19 => { fl_l = ((rdx as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 21; continue; } bb = 20; continue; }
            20 => { fl_l = ((((r8 as u32)) >> (0u32)) & 1) as u64; r8 = (((r8 as u32)) | ((1u32) << (0u32))) as u32 as u64; crate::simd_ops::wu32(P7, ((r8 as u32)) as u32); bb = 21; continue; }
            21 => { fl_l = ((((rdx as u32)) >> (23u32)) & 1) as u64; if ((fl_l as u32) == 0) { bb = 23; continue; } bb = 22; continue; }
            22 => { fl_l = ((((r8 as u32)) >> (1u32)) & 1) as u64; r8 = (((r8 as u32)) | ((1u32) << (1u32))) as u32 as u64; crate::simd_ops::wu32(P7, ((r8 as u32)) as u32); bb = 23; continue; }
            23 => { fl_l = ((((rdx as u32)) >> (25u32)) & 1) as u64; if ((fl_l as u32) == 0) { bb = 25; continue; } bb = 24; continue; }
            24 => { fl_l = ((((r8 as u32)) >> (2u32)) & 1) as u64; r8 = (((r8 as u32)) | ((1u32) << (2u32))) as u32 as u64; crate::simd_ops::wu32(P7, ((r8 as u32)) as u32); bb = 25; continue; }
            25 => { fl_l = ((((rdx as u32)) >> (26u32)) & 1) as u64; if ((fl_l as u32) == 0) { bb = 27; continue; } bb = 26; continue; }
            26 => { fl_l = ((((r8 as u32)) >> (3u32)) & 1) as u64; r8 = (((r8 as u32)) | ((1u32) << (3u32))) as u32 as u64; crate::simd_ops::wu32(P7, ((r8 as u32)) as u32); bb = 27; continue; }
            27 => { fl_l = ((rdi as u32)) as u32 as u64; fl_r = 0x7; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 46; continue; } bb = 28; continue; }
            28 => { rax = 0x7; rcx = 0; fl_l = 0; fl_r = 0; let (cp_a, cp_b, cp_c, cp_d) = crate::simd_ops::cpuid_x64(rax as u32, rcx as u32); rax = cp_a; rbx = cp_b; rcx = cp_c; rdx = cp_d; crate::simd_ops::wu32(rbp.wrapping_add(0x10u64), ((rax as u32)) as u32); crate::simd_ops::wu32(rbp.wrapping_add(0x14u64), ((rbx as u32)) as u32); crate::simd_ops::wu32(rbp.wrapping_add(0x18u64), ((rcx as u32)) as u32); crate::simd_ops::wu32(rbp.wrapping_add(0x1cu64), ((rdx as u32)) as u32); fl_l = ((rbx as u8)) as u8 as u64; fl_r = (0x20u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 30; continue; } bb = 29; continue; }
            29 => { fl_l = ((((r8 as u32)) >> (10u32)) & 1) as u64; r8 = (((r8 as u32)) | ((1u32) << (10u32))) as u32 as u64; crate::simd_ops::wu32(P7, ((r8 as u32)) as u32); bb = 30; continue; }
            30 => { fl_l = ((((rbx as u32)) >> (16u32)) & 1) as u64; if ((fl_l as u32) == 0) { bb = 32; continue; } bb = 31; continue; }
            31 => { fl_l = ((((r8 as u32)) >> (12u32)) & 1) as u64; r8 = (((r8 as u32)) | ((1u32) << (12u32))) as u32 as u64; crate::simd_ops::wu32(P7, ((r8 as u32)) as u32); bb = 32; continue; }
            32 => { fl_l = ((((rbx as u32)) >> (17u32)) & 1) as u64; if ((fl_l as u32) == 0) { bb = 34; continue; } bb = 33; continue; }
            33 => { fl_l = ((((r8 as u32)) >> (17u32)) & 1) as u64; r8 = (((r8 as u32)) | ((1u32) << (17u32))) as u32 as u64; crate::simd_ops::wu32(P7, ((r8 as u32)) as u32); bb = 34; continue; }
            34 => { fl_l = ((((rbx as u32)) >> (21u32)) & 1) as u64; if ((fl_l as u32) == 0) { bb = 36; continue; } bb = 35; continue; }
            35 => { fl_l = ((((r8 as u32)) >> (19u32)) & 1) as u64; r8 = (((r8 as u32)) | ((1u32) << (19u32))) as u32 as u64; crate::simd_ops::wu32(P7, ((r8 as u32)) as u32); bb = 36; continue; }
            36 => { fl_l = ((((rbx as u32)) >> (26u32)) & 1) as u64; if ((fl_l as u32) == 0) { bb = 38; continue; } bb = 37; continue; }
            37 => { fl_l = ((((r8 as u32)) >> (15u32)) & 1) as u64; r8 = (((r8 as u32)) | ((1u32) << (15u32))) as u32 as u64; crate::simd_ops::wu32(P7, ((r8 as u32)) as u32); bb = 38; continue; }
            38 => { fl_l = ((((rbx as u32)) >> (27u32)) & 1) as u64; if ((fl_l as u32) == 0) { bb = 40; continue; } bb = 39; continue; }
            39 => { fl_l = ((((r8 as u32)) >> (14u32)) & 1) as u64; r8 = (((r8 as u32)) | ((1u32) << (14u32))) as u32 as u64; crate::simd_ops::wu32(P7, ((r8 as u32)) as u32); bb = 40; continue; }
            40 => { fl_l = ((((rbx as u32)) >> (28u32)) & 1) as u64; if ((fl_l as u32) == 0) { bb = 42; continue; } bb = 41; continue; }
            41 => { fl_l = ((((r8 as u32)) >> (13u32)) & 1) as u64; r8 = (((r8 as u32)) | ((1u32) << (13u32))) as u32 as u64; crate::simd_ops::wu32(P7, ((r8 as u32)) as u32); bb = 42; continue; }
            42 => { fl_l = ((((rbx as u32)) >> (30u32)) & 1) as u64; if ((fl_l as u32) == 0) { bb = 44; continue; } bb = 43; continue; }
            43 => { fl_l = ((((r8 as u32)) >> (16u32)) & 1) as u64; r8 = (((r8 as u32)) | ((1u32) << (16u32))) as u32 as u64; crate::simd_ops::wu32(P7, ((r8 as u32)) as u32); bb = 44; continue; }
            44 => { fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) >= 0) { bb = 46; continue; } bb = 45; continue; }
            45 => { fl_l = ((((r8 as u32)) >> (18u32)) & 1) as u64; r8 = (((r8 as u32)) | ((1u32) << (18u32))) as u32 as u64; crate::simd_ops::wu32(P7, ((r8 as u32)) as u32); bb = 46; continue; }
            46 => { fl_l = ((r9 as u32)) as u32 as u64; fl_r = 0x80000001; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 51; continue; } bb = 47; continue; }
            47 => { rax = 0x80000001; rcx = 0; fl_l = 0; fl_r = 0; let (cp_a, cp_b, cp_c, cp_d) = crate::simd_ops::cpuid_x64(rax as u32, rcx as u32); rax = cp_a; rbx = cp_b; rcx = cp_c; rdx = cp_d; crate::simd_ops::wu32(rbp.wrapping_add(0x10u64), ((rax as u32)) as u32); crate::simd_ops::wu32(rbp.wrapping_add(0x14u64), ((rbx as u32)) as u32); crate::simd_ops::wu32(rbp.wrapping_add(0x18u64), ((rcx as u32)) as u32); crate::simd_ops::wu32(rbp.wrapping_add(0x1cu64), ((rdx as u32)) as u32); fl_l = ((rcx as u8)) as u8 as u64; fl_r = (0x20u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 49; continue; } bb = 48; continue; }
            48 => { fl_l = ((((r8 as u32)) >> (23u32)) & 1) as u64; r8 = (((r8 as u32)) | ((1u32) << (23u32))) as u32 as u64; fl_l = ((((r8 as u32)) >> (24u32)) & 1) as u64; r8 = (((r8 as u32)) | ((1u32) << (24u32))) as u32 as u64; crate::simd_ops::wu32(P7, ((r8 as u32)) as u32); bb = 49; continue; }
            49 => { fl_l = ((rcx as u8)) as u8 as u64; fl_r = (0x40u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 51; continue; } bb = 50; continue; }
            50 => { fl_l = ((((r8 as u32)) >> (8u32)) & 1) as u64; r8 = (((r8 as u32)) | ((1u32) << (8u32))) as u32 as u64; crate::simd_ops::wu32(P7, ((r8 as u32)) as u32); bb = 51; continue; }
            51 => { crate::simd_ops::wu32(rbp.wrapping_add(0x50u64), ((rsi as u32)) as u32); r8 = rbp.wrapping_add(0x50); rdx = 0; fl_l = 0; fl_r = 0; rcx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P8), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 87; continue; } bb = 52; continue; }
            52 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P9), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x7a; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 87; continue; } bb = 53; continue; }
            53 => { rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x50u64))) as u32 as u64; rcx = rax.wrapping_add(0xf); fl_l = rcx; fl_r = rax; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 55; continue; } bb = 54; continue; }
            54 => { rcx = (0xffffffffffffff0u64) as u64; bb = 55; continue; }
            55 => { rcx = (((rcx) & (0xfffffffffffffff0u64))) as u64; rax = rcx; rsp = ((rsp).wrapping_sub(rcx)) as u64; rbx = rsp.wrapping_add(0x20); r8 = rbp.wrapping_add(0x50); rdx = rbx; rcx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P8), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r11 = (crate::simd_ops::ru32(rbp.wrapping_add(0x50u64))) as u32 as u64; r11 = ((r11).wrapping_add(rbx)) as u64; fl_l = rbx; fl_r = r11; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 87; continue; } bb = 56; continue; }
            56 => { r9 = (crate::simd_ops::ru32(P7)) as u32 as u64; rdx = (crate::simd_ops::ru32(P10)) as u32 as u64; r10 = (crate::simd_ops::ru32(P11)) as u32 as u64; bb = 57; continue; }
            57 => { fl_l = (crate::simd_ops::ru32(rbx)) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 85; continue; } bb = 58; continue; }
            58 => { r10 = (((r10 as u32)).wrapping_add(1)) as u32 as u64; crate::simd_ops::wu32(P11, ((r10 as u32)) as u32); fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0x8u64))) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 84; continue; } bb = 59; continue; }
            59 => { rax = ((crate::simd_ops::ru16(rbx.wrapping_add(0x1eu64))) as u32) as u32 as u64; r8 = ((rsi as u32)) as u32 as u64; fl_l = ((((r9 as u32)) >> (24u32)) & 1) as u64; if ((fl_l as u32) == 0) { bb = 64; continue; } bb = 60; continue; }
            60 => { fl_l = ((rsi as u16)) as u16 as u64; fl_r = ((rax as u16)) as u16 as u64; if (((fl_l as u16)) >= ((fl_r as u16))) { bb = 85; continue; } bb = 61; continue; }
            61 => { rcx = rbx.wrapping_add(0x20); bb = 62; continue; }
            62 => { let bit_in: u64 = crate::simd_ops::ru64(rcx); rax = ((bit_in.count_ones() as u64)) as u64; fl_l = (rax as u64) as u64; rdx = (((rdx as u32)).wrapping_add((rax as u32))) as u32 as u64; crate::simd_ops::wu32(P10, ((rdx as u32)) as u32); r8 = (((r8 as u32)).wrapping_add(1)) as u32 as u64; rcx = rcx.wrapping_add(0x10); rax = ((crate::simd_ops::ru16(rbx.wrapping_add(0x1eu64))) as u32) as u32 as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 62; continue; } bb = 63; continue; }
            63 => { bb = 85; continue; }
            64 => { fl_l = ((rsi as u16)) as u16 as u64; fl_r = ((rax as u16)) as u16 as u64; if (((fl_l as u16)) >= ((fl_r as u16))) { bb = 85; continue; } bb = 65; continue; }
            65 => { rax = rbx.wrapping_add(0x20); bb = 66; continue; }
            66 => { fl_l = (crate::simd_ops::ru8(rax)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 68; continue; } bb = 67; continue; }
            67 => { fl_l = (((rdx as u32)).wrapping_add(1)) as u32 as u64; rdx = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(P10, ((rdx as u32)) as u32); bb = 68; continue; }
            68 => { fl_l = (crate::simd_ops::ru8(rax)) as u8 as u64; fl_r = (0x2u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 70; continue; } bb = 69; continue; }
            69 => { fl_l = (((rdx as u32)).wrapping_add(1)) as u32 as u64; rdx = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(P10, ((rdx as u32)) as u32); bb = 70; continue; }
            70 => { fl_l = (crate::simd_ops::ru8(rax)) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 72; continue; } bb = 71; continue; }
            71 => { fl_l = (((rdx as u32)).wrapping_add(1)) as u32 as u64; rdx = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(P10, ((rdx as u32)) as u32); bb = 72; continue; }
            72 => { fl_l = (crate::simd_ops::ru8(rax)) as u8 as u64; fl_r = (0x8u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 74; continue; } bb = 73; continue; }
            73 => { fl_l = (((rdx as u32)).wrapping_add(1)) as u32 as u64; rdx = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(P10, ((rdx as u32)) as u32); bb = 74; continue; }
            74 => { fl_l = (crate::simd_ops::ru8(rax)) as u8 as u64; fl_r = (0x10u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 76; continue; } bb = 75; continue; }
            75 => { fl_l = (((rdx as u32)).wrapping_add(1)) as u32 as u64; rdx = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(P10, ((rdx as u32)) as u32); bb = 76; continue; }
            76 => { fl_l = (crate::simd_ops::ru8(rax)) as u8 as u64; fl_r = (0x20u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 78; continue; } bb = 77; continue; }
            77 => { fl_l = (((rdx as u32)).wrapping_add(1)) as u32 as u64; rdx = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(P10, ((rdx as u32)) as u32); bb = 78; continue; }
            78 => { fl_l = (crate::simd_ops::ru8(rax)) as u8 as u64; fl_r = (0x40u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 80; continue; } bb = 79; continue; }
            79 => { fl_l = (((rdx as u32)).wrapping_add(1)) as u32 as u64; rdx = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(P10, ((rdx as u32)) as u32); bb = 80; continue; }
            80 => { fl_l = (crate::simd_ops::ru8(rax)) as u8 as u64; fl_r = (0x80u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 82; continue; } bb = 81; continue; }
            81 => { fl_l = (((rdx as u32)).wrapping_add(1)) as u32 as u64; rdx = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(P10, ((rdx as u32)) as u32); bb = 82; continue; }
            82 => { r8 = (((r8 as u32)).wrapping_add(1)) as u32 as u64; rax = ((rax).wrapping_add(0x10u64)) as u64; rcx = ((crate::simd_ops::ru16(rbx.wrapping_add(0x1eu64))) as u32) as u32 as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 66; continue; } bb = 83; continue; }
            83 => { bb = 85; continue; }
            84 => { fl_l = (((rdx as u32)).wrapping_add(1)) as u32 as u64; rdx = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(P10, ((rdx as u32)) as u32); bb = 85; continue; }
            85 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x4u64))) as u32 as u64; rbx = ((rbx).wrapping_add(rax)) as u64; fl_l = rbx; fl_r = r11; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 57; continue; } bb = 86; continue; }
            86 => { bb = 88; continue; }
            87 => { r9 = (crate::simd_ops::ru32(P7)) as u32 as u64; rdx = (crate::simd_ops::ru32(P10)) as u32 as u64; r10 = (crate::simd_ops::ru32(P11)) as u32 as u64; bb = 88; continue; }
            88 => { fl_l = ((r10 as u32)) as u32 as u64; fl_r = 0x1; if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { bb = 90; continue; } bb = 89; continue; }
            89 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x1; if ((((fl_l as u32)) as i32) <= (((fl_r as u32)) as i32)) { bb = 91; continue; } bb = 90; continue; }
            90 => { fl_l = ((((r9 as u32)) >> (20u32)) & 1) as u64; r9 = (((r9 as u32)) | ((1u32) << (20u32))) as u32 as u64; crate::simd_ops::wu32(P7, ((r9 as u32)) as u32); bb = 91; continue; }
            91 => { fl_l = ((r10 as u32)) as u32 as u64; fl_r = 0x1; if ((((fl_l as u32)) as i32) <= (((fl_r as u32)) as i32)) { bb = 93; continue; } bb = 92; continue; }
            92 => { fl_l = ((((r9 as u32)) >> (21u32)) & 1) as u64; r9 = (((r9 as u32)) | ((1u32) << (21u32))) as u32 as u64; crate::simd_ops::wu32(P7, ((r9 as u32)) as u32); bb = 93; continue; }
            93 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((r10 as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) <= (((fl_r as u32)) as i32)) { bb = 95; continue; } bb = 94; continue; }
            94 => { fl_l = ((((r9 as u32)) >> (22u32)) & 1) as u64; r9 = (((r9 as u32)) | ((1u32) << (22u32))) as u32 as u64; crate::simd_ops::wu32(P7, ((r9 as u32)) as u32); bb = 95; continue; }
            95 => { rax = r14; rbx = (crate::simd_ops::ru64(rbp.wrapping_add(0x58u64))) as u64; rsi = (crate::simd_ops::ru64(rbp.wrapping_add(0x60u64))) as u64; rsp = rbp.wrapping_add(0x30); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1424a6900(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143c1bf83; const P1: u64 = 0x143c1bf80; const P2: u64 = 0x143c1bf88; const P3: u64 = 0x143c1bf84; const P4: u64 = 0x1426d01e0; const P5: u64 = 0x14340f970; const P6: u64 = 0x14340f940; const P7: u64 = 0x14340f930; const P8: u64 = 0x14340f920; const P9: u64 = 0x1401df700; const P10: u64 = 0x14340f910; const P11: u64 = 0x14340f960; const P12: u64 = 0x14340f950;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; r8 = ((crate::simd_ops::ru8(P0)) as u32) as u32 as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = ((r8.wrapping_mul(8)) as u32) as u32 as u64; bb = 3; continue; }
            2 => { rax = ((r8 as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x7c00; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 3; continue; }
            3 => { rsi = (crate::simd_ops::ru32(P1)) as u32 as u64; rbx = P2; rdx = (crate::simd_ops::ru32(P3)) as u32 as u64; r14 = P4; rsi = ((((rsi as u32)) & (0xffffffu32))) as u32 as u64; rdi = rbx; rsi = (((rsi as u32)).wrapping_shl(3u32)) as u32 as u64; fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; rdi = (if (((fl_l as u32)) > ((fl_r as u32))) { crate::simd_ops::ru64(P2) } else { rdi }) as u64; fl_l = rdx; fl_r = (0xcu64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 9; continue; } bb = 4; continue; }
            4 => { rcx = P5; r9 = ((rdx as u32)) as u32 as u64; fl_l = rdi; fl_r = rcx; rdi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; bb = 5; continue; }
            5 => { rax = ((crate::simd_ops::ru8(rcx.wrapping_add(rdi))) as u32) as u32 as u64; r9 = ((r9).wrapping_sub(1)) as u64; r10 = ((crate::simd_ops::ru8(rax.wrapping_add(r14))) as u32) as u32 as u64; rax = ((crate::simd_ops::ru8(rcx)) as u32) as u32 as u64; fl_l = ((r10 as u8)) as u8 as u64; fl_r = (crate::simd_ops::ru8(rax.wrapping_add(r14))) as u8 as u64; if ((((fl_l as u8)) as i8) < (((fl_r as u8)) as i8)) { bb = 9; continue; } bb = 6; continue; }
            6 => { if ((((fl_l as u8)) as i8) > (((fl_r as u8)) as i8)) { bb = 9; continue; } bb = 7; continue; }
            7 => { rcx = ((rcx).wrapping_add(1)) as u64; fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 8; continue; }
            8 => { rax = ((r9.wrapping_add(0x2u64)) as u32) as u32 as u64; bb = 32; continue; }
            9 => { fl_l = ((r8 as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 11; continue; } bb = 10; continue; }
            10 => { rax = ((r8.wrapping_mul(8)) as u32) as u32 as u64; bb = 12; continue; }
            11 => { rax = ((r8 as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = (((rax as u32)).wrapping_add(0xffff8400u32)) as u32 as u64; fl_r = fl_l; rax = ((fl_l as u32)) as u32 as u64; bb = 12; continue; }
            12 => { fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; rdi = rbx; rdi = (if (((fl_l as u32)) > ((fl_r as u32))) { crate::simd_ops::ru64(P2) } else { rdi }) as u64; fl_l = rdx; fl_r = (0xcu64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 18; continue; } bb = 13; continue; }
            13 => { rcx = P6; r9 = rdx; fl_l = rdi; fl_r = rcx; rdi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; bb = 14; continue; }
            14 => { rax = ((crate::simd_ops::ru8(rcx.wrapping_add(rdi))) as u32) as u32 as u64; r9 = ((r9).wrapping_sub(1)) as u64; r10 = ((crate::simd_ops::ru8(rax.wrapping_add(r14))) as u32) as u32 as u64; rax = ((crate::simd_ops::ru8(rcx)) as u32) as u32 as u64; fl_l = ((r10 as u8)) as u8 as u64; fl_r = (crate::simd_ops::ru8(rax.wrapping_add(r14))) as u8 as u64; if ((((fl_l as u8)) as i8) < (((fl_r as u8)) as i8)) { bb = 18; continue; } bb = 15; continue; }
            15 => { if ((((fl_l as u8)) as i8) > (((fl_r as u8)) as i8)) { bb = 18; continue; } bb = 16; continue; }
            16 => { rcx = ((rcx).wrapping_add(1)) as u64; fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 14; continue; } bb = 17; continue; }
            17 => { rax = ((r9.wrapping_add(0x3u64)) as u32) as u32 as u64; bb = 32; continue; }
            18 => { fl_l = ((r8 as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 20; continue; } bb = 19; continue; }
            19 => { r8 = ((r8.wrapping_mul(8)) as u32) as u32 as u64; bb = 21; continue; }
            20 => { r8 = (((r8 as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = (((r8 as u32)).wrapping_add(0xffff8400u32)) as u32 as u64; fl_r = fl_l; r8 = ((fl_l as u32)) as u32 as u64; bb = 21; continue; }
            21 => { fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; rbx = (if (((fl_l as u32)) > ((fl_r as u32))) { crate::simd_ops::ru64(P2) } else { rbx }) as u64; fl_l = rdx; fl_r = (0xcu64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 27; continue; } bb = 22; continue; }
            22 => { rcx = P7; fl_l = rbx; fl_r = rcx; rbx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; bb = 23; continue; }
            23 => { rax = ((crate::simd_ops::ru8(rcx.wrapping_add(rbx))) as u32) as u32 as u64; rdx = ((rdx).wrapping_sub(1)) as u64; r8 = ((crate::simd_ops::ru8(rax.wrapping_add(r14))) as u32) as u32 as u64; rax = ((crate::simd_ops::ru8(rcx)) as u32) as u32 as u64; fl_l = ((r8 as u8)) as u8 as u64; fl_r = (crate::simd_ops::ru8(rax.wrapping_add(r14))) as u8 as u64; if ((((fl_l as u8)) as i8) < (((fl_r as u8)) as i8)) { bb = 27; continue; } bb = 24; continue; }
            24 => { if ((((fl_l as u8)) as i8) > (((fl_r as u8)) as i8)) { bb = 27; continue; } bb = 25; continue; }
            25 => { rcx = ((rcx).wrapping_add(1)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 23; continue; } bb = 26; continue; }
            26 => { rax = ((rdx.wrapping_add(0x3u64)) as u32) as u32 as u64; bb = 32; continue; }
            27 => { crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (0x9u64) as u64); rax = P8; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); rdx = rsp.wrapping_add(0x20); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; rcx = P1; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 31; continue; } bb = 28; continue; }
            28 => { crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (0xcu64) as u64); rax = P10; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); rdx = rsp.wrapping_add(0x20); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; rcx = P1; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 31; continue; } bb = 29; continue; }
            29 => { crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (0xbu64) as u64); rax = P11; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); rdx = rsp.wrapping_add(0x20); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; rcx = P1; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 31; continue; } bb = 30; continue; }
            30 => { crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (0xcu64) as u64); rax = P12; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); rdx = rsp.wrapping_add(0x20); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; rcx = P1; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; rcx = 0x1; rdx = 0x4; rcx = (if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { (rdx as u32) } else { (rcx as u32) }) as u32 as u64; rax = ((rcx as u32)) as u32 as u64; bb = 32; continue; }
            31 => { rax = 0x1; bb = 32; continue; }
            32 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1424a6b90(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x4; const P1: u64 = 0x8; const P2: u64 = 0xc;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    r8 = rdx;
    rax = ((rcx as u32)) as u32 as u64;
    rcx = 0;
    fl_l = 0; fl_r = 0;
    let (cp_a, cp_b, cp_c, cp_d) = crate::simd_ops::cpuid_x64(rax as u32, rcx as u32); rax = cp_a; rbx = cp_b; rcx = cp_c; rdx = cp_d;
    crate::simd_ops::wu32(r8, ((rax as u32)) as u32);
    crate::simd_ops::wu32(r8.wrapping_add(P0), ((rbx as u32)) as u32);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x8u64))) as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P1), ((rcx as u32)) as u32);
    crate::simd_ops::wu32(r8.wrapping_add(P2), ((rdx as u32)) as u32);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1424a6bc0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14340fa40; const P1: u64 = 0x1433dc9a8; const P2: u64 = 0x1423fddd0; const P3: u64 = 0x1433df968; const P4: u64 = 0x1423ef3d0; const P5: u64 = 0x1426469c8; const P6: u64 = 0x1425371a0; const P7: u64 = 0x142533200; const P8: u64 = 0x1426469d0; const P9: u64 = 0x140000000; const P10: u64 = 0x1433dca18; const P11: u64 = 0x143c1bfa0; const P12: u64 = 0x14340fa10; const P13: u64 = 0x14340f9f8; const P14: u64 = 0x1423ef2b0; const P15: u64 = 0x1433dc548; const P16: u64 = 0x1423fe420; const P17: u64 = 0x14340f9c8; const P18: u64 = 0x14242c560; const P19: u64 = 0x14240ab10; const P20: u64 = 0x142442de0;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<512, 255>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rbp = rsp.wrapping_sub(0x6c0); rsp = ((rsp).wrapping_sub(0x7c0u64)) as u64; r15 = rdx; rbx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rbp.wrapping_add(0x710u64), ((rbx as u32)) as u32); rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; rdx = P0; fl_l = rax; fl_r = rax; rdx = (if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { rax } else { rdx }) as u64; rcx = P1; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r14 = rbp.wrapping_sub(0x60); crate::simd_ops::wu64(rbp.wrapping_sub(0x80u64), r14); rcx = 0x200; crate::simd_ops::wu32(rbp.wrapping_sub(0x78u64), ((rcx as u32)) as u32); crate::simd_ops::wu64(rbp.wrapping_sub(0x74u64), rcx); rdx = rbp.wrapping_sub(0x60); crate::simd_ops::wu64(rbp.wrapping_sub(0x68u64), rdx); crate::simd_ops::wu16(rbp.wrapping_sub(0x60u64), ((rbx as u16)) as u16); rsi = P3; fl_l = rax; fl_r = rax; rsi = (if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { rax } else { rsi }) as u64; r13 = (0xffffffffffffffffu64) as u64; rdi = r13; bb = 1; continue; }
            1 => { rdi = ((rdi).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru16(rsi.wrapping_add(rdi.wrapping_mul(2)))) as u16 as u64; fl_r = (0x0u16) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 1; continue; } bb = 2; continue; }
            2 => { rdx = ((rdi as u32)) as u32 as u64; rdx = ((rdx).wrapping_add(1)) as u64; fl_l = rdx; fl_r = rcx; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 14; continue; } bb = 3; continue; }
            3 => { rax = ((rdx as u32)) as u32 as u64; rax = ((((rax as u32)) & (0xfffffff0u32))) as u32 as u64; fl_l = ((rdx as u8)) as u8 as u64; fl_r = (0xfu8) as u8 as u64; r15 = ((rax.wrapping_add(0x10u64)) as u32) as u32 as u64; r15 = (if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { (rax as u32) } else { (r15 as u32) }) as u32 as u64; rax = 0x300; fl_l = ((r15 as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; r15 = (if (((fl_l as u32)) < ((fl_r as u32))) { (rax as u32) } else { (r15 as u32) }) as u32 as u64; r9 = ((r15 as u32)) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r14 = rax; r12 = ((r15.wrapping_sub(0x1u64)) as u32) as u32 as u64; r9 = ((r12 as u32)) as u32 as u64; r8 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x80u64))) as u64; rdx = ((r15 as u32)) as u32 as u64; rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r10 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x80u64))) as u64; fl_l = r10; fl_r = (crate::simd_ops::ru64(rbp.wrapping_sub(0x68u64))) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 12; continue; } bb = 4; continue; }
            4 => { fl_l = r10; fl_r = r10; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 5; continue; }
            5 => { r10 = ((r10).wrapping_sub(1)) as u64; r10 = (((r10) & (0xfffffffffe000000u64))) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(r10.wrapping_add(0x100u64))) as u64; rcx = (rcx & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); r9 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x80u64))) as u64; rax = r9; rax = ((rax).wrapping_sub(r10)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rdx = r10.wrapping_add(0x108); rdx = ((rdx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32 as u64; rdx = ((rdx).wrapping_sub(rax)) as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 10; continue; } bb = 6; continue; }
            6 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0xeu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 9; continue; } bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(r9, rax); crate::simd_ops::wu64(rdx.wrapping_add(0x20u64), r9); fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 12; continue; } bb = 8; continue; }
            8 => { rcx = rdx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 12; continue; }
            9 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 11; continue; }
            10 => { r8 = 0; fl_l = 0; fl_r = 0; bb = 11; continue; }
            11 => { rcx = r10; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 12; continue; }
            12 => { crate::simd_ops::wu64(rbp.wrapping_sub(0x80u64), r14); rcx = ((r15 as u32)) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_sub(0x74u64), ((rcx as u32)) as u32); fl_l = ((r15 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rbp.wrapping_sub(0x70u64))) as u32 as u64; r15 = (crate::simd_ops::ru64(rbp.wrapping_add(0x708u64))) as u64; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 14; continue; } bb = 13; continue; }
            13 => { crate::simd_ops::wu32(rbp.wrapping_sub(0x70u64), ((r12 as u32)) as u32); rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu16(r14.wrapping_add(r12.wrapping_mul(2)), ((rcx as u16)) as u16); rcx = (crate::simd_ops::ru32(rbp.wrapping_sub(0x74u64))) as u32 as u64; r14 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x80u64))) as u64; bb = 14; continue; }
            14 => { rdx = ((rcx as u32)) as u32 as u64; r8 = rsi; rcx = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P8), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(rbp.wrapping_sub(0x70u64), ((rdi as u32)) as u32); rbx = (rbx & !0xffu64) | (((0x1u8) as u8 as u64)); r14 = 0; fl_l = 0; fl_r = 0; r12 = ((r14 as u32)) as u32 as u64; rax = ((r14 as u32)) as u32 as u64; r9 = P9; bb = 15; continue; }
            15 => { rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x700u64))) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x14; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 54; continue; } bb = 16; continue; }
            16 => { r8 = ((r12 as u32)) as u32 as u64; rcx = ((((rax as u32)) as i32 as i64) as u64) as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(rdx))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 54; continue; } bb = 17; continue; }
            17 => { rdx = (if (rax as u32 as i32) < 0 { 0xffffffffu32 } else { 0u32 }) as u64; rdx = ((((rdx as u32)) & (0x1fu32))) as u32 as u64; rax = (((rax as u32)).wrapping_add((rdx as u32))) as u32 as u64; rcx = ((rax as u32)) as u32 as u64; rax = ((((rax as u32)) & (0x1fu32))) as u32 as u64; rax = (((rax as u32)).wrapping_sub((rdx as u32))) as u32 as u64; rcx = (((((rcx as u32)) as i32).wrapping_shr(5u32) as u32)) as u32 as u64; rcx = ((((rcx as u32)) as i32 as i64) as u64) as u64; rcx = (crate::simd_ops::ru32(r9.wrapping_add(rcx.wrapping_mul(4)).wrapping_add(0x3c1bfa4u64))) as u32 as u64; fl_l = ((((rcx as u32)) >> ((((rax as u32)) as u32 & 31))) & 1) as u64; if ((fl_l as u32) != 0) { bb = 53; continue; } bb = 18; continue; }
            18 => { rdx = (crate::simd_ops::ru64(r15.wrapping_add(r8.wrapping_mul(8)).wrapping_add(0x10u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 20; continue; } bb = 19; continue; }
            19 => { rdx = (crate::simd_ops::ru64(r9.wrapping_add(r8.wrapping_mul(8)).wrapping_add(0x38716a0u64))) as u64; bb = 20; continue; }
            20 => { rcx = P10; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r15 = rax; rdi = r13; bb = 21; continue; }
            21 => { rdi = ((rdi).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru16(rax.wrapping_add(rdi.wrapping_mul(2)))) as u16 as u64; fl_r = (0x0u16) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 21; continue; } bb = 22; continue; }
            22 => { r8 = (crate::simd_ops::ru32(rbp.wrapping_sub(0x70u64))) as u32 as u64; rcx = ((rdi as u32)) as u32 as u64; rcx = ((rcx).wrapping_add(r8)) as u64; fl_l = rcx; fl_r = r8; r8 = (if (((fl_l as u64)) >= ((fl_r as u64))) { rcx } else { r8 }) as u64; r8 = ((r8).wrapping_add(1)) as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x68u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 35; continue; } bb = 23; continue; }
            23 => { rdx = (crate::simd_ops::ru32(rbp.wrapping_sub(0x78u64))) as u32 as u64; fl_l = r8; fl_r = rdx; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 35; continue; } bb = 24; continue; }
            24 => { rsi = (crate::simd_ops::ru64(rbp.wrapping_sub(0x80u64))) as u64; fl_l = rsi; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 34; continue; } bb = 25; continue; }
            25 => { r9 = rdx.wrapping_sub(0x1); r8 = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r10 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x80u64))) as u64; fl_l = r10; fl_r = r10; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 33; continue; } bb = 26; continue; }
            26 => { r10 = ((r10).wrapping_sub(1)) as u64; r10 = (((r10) & (0xfffffffffe000000u64))) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(r10.wrapping_add(0x100u64))) as u64; rcx = (rcx & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); r9 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x80u64))) as u64; rax = r9; rax = ((rax).wrapping_sub(r10)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rdx = r10.wrapping_add(0x108); rdx = ((rdx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32 as u64; rdx = ((rdx).wrapping_sub(rax)) as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 31; continue; } bb = 27; continue; }
            27 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0xeu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 30; continue; } bb = 28; continue; }
            28 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(r9, rax); crate::simd_ops::wu64(rdx.wrapping_add(0x20u64), r9); fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 33; continue; } bb = 29; continue; }
            29 => { rcx = rdx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rsi = (crate::simd_ops::ru64(rbp.wrapping_sub(0x68u64))) as u64; rcx = (crate::simd_ops::ru32(rbp.wrapping_sub(0x78u64))) as u32 as u64; bb = 48; continue; }
            30 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 32; continue; }
            31 => { r8 = 0; fl_l = 0; fl_r = 0; bb = 32; continue; }
            32 => { rcx = r10; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 33; continue; }
            33 => { rsi = (crate::simd_ops::ru64(rbp.wrapping_sub(0x68u64))) as u64; rcx = (crate::simd_ops::ru32(rbp.wrapping_sub(0x78u64))) as u32 as u64; bb = 48; continue; }
            34 => { rcx = (crate::simd_ops::ru32(rbp.wrapping_sub(0x74u64))) as u32 as u64; bb = 49; continue; }
            35 => { rcx = (crate::simd_ops::ru32(rbp.wrapping_sub(0x74u64))) as u32 as u64; fl_l = r8; fl_r = rcx; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 51; continue; } bb = 36; continue; }
            36 => { rax = ((r8 as u32)) as u32 as u64; rax = ((((rax as u32)) & (0xfffffff0u32))) as u32 as u64; fl_l = ((r8 as u8)) as u8 as u64; fl_r = (0xfu8) as u8 as u64; r14 = ((rax.wrapping_add(0x10u64)) as u32) as u32 as u64; r14 = (if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { (rax as u32) } else { (r14 as u32) }) as u32 as u64; fl_l = r8; fl_r = (0x100u64) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 38; continue; } bb = 37; continue; }
            37 => { rax = ((rcx as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shr(1u32)) as u32 as u64; rax = (((rax as u32)).wrapping_add((rcx as u32))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((r14 as u32)) as u32 as u64; r14 = (if (((fl_l as u32)) > ((fl_r as u32))) { (rax as u32) } else { (r14 as u32) }) as u32 as u64; bb = 38; continue; }
            38 => { r9 = ((r14 as u32)) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rsi = rax; r9 = ((r14.wrapping_sub(0x1u64)) as u32) as u32 as u64; r8 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x80u64))) as u64; rdx = ((r14 as u32)) as u32 as u64; rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r10 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x80u64))) as u64; fl_l = r10; fl_r = (crate::simd_ops::ru64(rbp.wrapping_sub(0x68u64))) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 47; continue; } bb = 39; continue; }
            39 => { fl_l = r10; fl_r = r10; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 47; continue; } bb = 40; continue; }
            40 => { r10 = ((r10).wrapping_sub(1)) as u64; r10 = (((r10) & (0xfffffffffe000000u64))) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(r10.wrapping_add(0x100u64))) as u64; rcx = (rcx & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); r9 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x80u64))) as u64; rax = r9; rax = ((rax).wrapping_sub(r10)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rdx = r10.wrapping_add(0x108); rdx = ((rdx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32 as u64; rdx = ((rdx).wrapping_sub(rax)) as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 45; continue; } bb = 41; continue; }
            41 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0xeu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 44; continue; } bb = 42; continue; }
            42 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(r9, rax); crate::simd_ops::wu64(rdx.wrapping_add(0x20u64), r9); fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 47; continue; } bb = 43; continue; }
            43 => { rcx = rdx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 47; continue; }
            44 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 46; continue; }
            45 => { r8 = 0; fl_l = 0; fl_r = 0; bb = 46; continue; }
            46 => { rcx = r10; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 47; continue; }
            47 => { rcx = ((r14 as u32)) as u32 as u64; r14 = 0; fl_l = 0; fl_r = 0; bb = 48; continue; }
            48 => { crate::simd_ops::wu32(rbp.wrapping_sub(0x74u64), ((rcx as u32)) as u32); crate::simd_ops::wu64(rbp.wrapping_sub(0x80u64), rsi); bb = 49; continue; }
            49 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rbp.wrapping_sub(0x70u64))) as u32 as u64; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 52; continue; } bb = 50; continue; }
            50 => { rax = ((rcx.wrapping_sub(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_sub(0x70u64), ((rax as u32)) as u32); crate::simd_ops::wu16(rsi.wrapping_add(rax.wrapping_mul(2)), ((r14 as u16)) as u16); rcx = (crate::simd_ops::ru32(rbp.wrapping_sub(0x74u64))) as u32 as u64; bb = 51; continue; }
            51 => { rsi = (crate::simd_ops::ru64(rbp.wrapping_sub(0x80u64))) as u64; bb = 52; continue; }
            52 => { rax = (crate::simd_ops::ru32(rbp.wrapping_sub(0x70u64))) as u32 as u64; rdx = ((rcx as u32)) as u32 as u64; rdx = ((rdx).wrapping_sub(rax)) as u64; rcx = ((rsi.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; r8 = r15; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P8), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::lk_add_u32(rbp.wrapping_sub(0x70u64), ((rdi as u32)) as u32); rbx = (rbx & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; r9 = P9; bb = 53; continue; }
            53 => { rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x700u64))) as u64; bb = 54; continue; }
            54 => { r12 = (((r12 as u32)).wrapping_add(1)) as u32 as u64; rax = ((r12 as u32)) as u32 as u64; fl_l = ((r12 as u32)) as u32 as u64; fl_r = 0x1a; r15 = (crate::simd_ops::ru64(rbp.wrapping_add(0x708u64))) as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 15; continue; } bb = 55; continue; }
            55 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0x14u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 60; continue; } bb = 56; continue; }
            56 => { rdx = (crate::simd_ops::ru32(rdx.wrapping_add(0x1cu64))) as u32 as u64; fl_l = (crate::simd_ops::ru32(P11)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { bb = 60; continue; } bb = 57; continue; }
            57 => { r14 = r15; rax = (crate::simd_ops::ru64(r15.wrapping_add(0xe0u64))) as u64; r8 = P12; fl_l = rax; fl_r = rax; r8 = (if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { rax } else { r8 }) as u64; rcx = P13; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = rax; rbx = r13; bb = 58; continue; }
            58 => { rbx = ((rbx).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru16(rax.wrapping_add(rbx.wrapping_mul(2)))) as u16 as u64; fl_r = (0x0u16) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 58; continue; } bb = 59; continue; }
            59 => { rdx = (crate::simd_ops::ru32(rbp.wrapping_sub(0x70u64))) as u32 as u64; rcx = ((rbx as u32)) as u32 as u64; rcx = ((rcx).wrapping_add(rdx)) as u64; fl_l = rcx; fl_r = rdx; rdx = (if (((fl_l as u64)) >= ((fl_r as u64))) { rcx } else { rdx }) as u64; rcx = rbp.wrapping_sub(0x80); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P14, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru32(rbp.wrapping_sub(0x70u64))) as u32 as u64; rdx = (crate::simd_ops::ru32(rbp.wrapping_sub(0x74u64))) as u32 as u64; rdx = ((rdx).wrapping_sub(rcx)) as u64; rax = (crate::simd_ops::ru64(rbp.wrapping_sub(0x80u64))) as u64; rcx = ((rax.wrapping_add(rcx.wrapping_mul(2))) as u64) as u64; r8 = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P8), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::lk_add_u32(rbp.wrapping_sub(0x70u64), ((rbx as u32)) as u32); rbx = (rbx & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 62; continue; }
            60 => { fl_l = ((rbx as u8)) as u8 as u64; fl_r = ((rbx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 76; continue; } bb = 61; continue; }
            61 => { r14 = (crate::simd_ops::ru64(rbp.wrapping_add(0x708u64))) as u64; bb = 62; continue; }
            62 => { rax = (crate::simd_ops::ru64(r14.wrapping_add(0xe8u64))) as u64; rcx = P15; fl_l = rax; fl_r = rax; rcx = (if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { rax } else { rcx }) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rcx); rax = r13; bb = 63; continue; }
            63 => { rax = ((rax).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru16(rcx.wrapping_add(rax.wrapping_mul(2)))) as u16 as u64; fl_r = (0x0u16) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 63; continue; } bb = 64; continue; }
            64 => { crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rax); crate::simd_ops::wu64(rbp.wrapping_add(0x5b0u64), (0x20000020u64) as u64); crate::simd_ops::wu8(rbp.wrapping_add(0x5b8u64), (0x0u8) as u8); crate::simd_ops::wu32(rbp.wrapping_add(0x710u64), (0x1u32) as u32); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x50u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x50u64), (xmm0) as u128); rdx = rsp.wrapping_add(0x50); rcx = rbp.wrapping_add(0x5b0); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P16, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = ((crate::simd_ops::ru8(rbp.wrapping_add(0x5b3u64))) as u32) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 66; continue; } bb = 65; continue; }
            65 => { rcx = ((rcx.wrapping_mul(8)) as u32) as u32 as u64; bb = 67; continue; }
            66 => { rcx = (((rcx as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x7c00; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 67; continue; }
            67 => { rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x5b0u64))) as u32 as u64; rax = ((((rax as u32)) & (0xffffffu32))) as u32 as u64; rax = (((rax as u32)).wrapping_shl(3u32)) as u32 as u64; rsi = rbp.wrapping_add(0x5b8); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; rsi = (if (((fl_l as u32)) > ((fl_r as u32))) { crate::simd_ops::ru64(rbp.wrapping_add(0x5b8u64)) } else { rsi }) as u64; rax = (crate::simd_ops::ru64(r14)) as u64; rcx = P17; fl_l = rax; fl_r = rax; rcx = (if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { rax } else { rcx }) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rcx); bb = 68; continue; }
            68 => { r13 = ((r13).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru16(rcx.wrapping_add(r13.wrapping_mul(2)))) as u16 as u64; fl_r = (0x0u16) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 68; continue; } bb = 69; continue; }
            69 => { crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), r13); crate::simd_ops::wu64(rbp.wrapping_add(0x4a8u64), (0x20000020u64) as u64); r14 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu8(rbp.wrapping_add(0x4b0u64), ((r14 as u8)) as u8); crate::simd_ops::wu32(rbp.wrapping_add(0x710u64), (0x3u32) as u32); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x60u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x60u64), (xmm0) as u128); rdx = rsp.wrapping_add(0x60); rcx = rbp.wrapping_add(0x4a8); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P16, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = ((crate::simd_ops::ru8(rbp.wrapping_add(0x4abu64))) as u32) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 71; continue; } bb = 70; continue; }
            70 => { rcx = ((rcx.wrapping_mul(8)) as u32) as u32 as u64; bb = 72; continue; }
            71 => { rcx = (((rcx as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x7c00; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 72; continue; }
            72 => { rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x4a8u64))) as u32 as u64; rax = ((((rax as u32)) & (0xffffffu32))) as u32 as u64; rax = (((rax as u32)).wrapping_shl(3u32)) as u32 as u64; rdi = rbp.wrapping_add(0x4b0); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; rdi = (if (((fl_l as u32)) > ((fl_r as u32))) { crate::simd_ops::ru64(rbp.wrapping_add(0x4b0u64)) } else { rdi }) as u64; rcx = (crate::simd_ops::ru32(rbp.wrapping_sub(0x70u64))) as u32 as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x3a0u64), (0x20000020u64) as u64); crate::simd_ops::wu8(rbp.wrapping_add(0x3a8u64), (0x0u8) as u8); crate::simd_ops::wu32(rbp.wrapping_add(0x710u64), (0x7u32) as u32); rax = (crate::simd_ops::ru64(rbp.wrapping_sub(0x80u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), rcx); rdx = rsp.wrapping_add(0x70); rcx = rbp.wrapping_add(0x3a0); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P16, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = ((crate::simd_ops::ru8(rbp.wrapping_add(0x3a3u64))) as u32) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 74; continue; } bb = 73; continue; }
            73 => { rdx = ((rdx.wrapping_mul(8)) as u32) as u32 as u64; bb = 75; continue; }
            74 => { rdx = (((rdx as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x7c00; rdx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 75; continue; }
            75 => { rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x3a0u64))) as u32 as u64; rax = ((((rax as u32)) & (0xffffffu32))) as u32 as u64; rax = (((rax as u32)).wrapping_shl(3u32)) as u32 as u64; rcx = rbp.wrapping_add(0x3a8); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; rcx = (if (((fl_l as u32)) > ((fl_r as u32))) { crate::simd_ops::ru64(rbp.wrapping_add(0x3a8u64)) } else { rcx }) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x48u64), ((r14 as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r14); r9 = 0; fl_l = 0; fl_r = 0; r8 = rsi; rdx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P18, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rbp.wrapping_add(0x3a0); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P19, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rbp.wrapping_add(0x4a8); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P19, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rbp.wrapping_add(0x5b0); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P19, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 76; continue; }
            76 => { rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x80u64))) as u64; rax = (crate::simd_ops::ru64(rbp.wrapping_sub(0x68u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 79; continue; } bb = 77; continue; }
            77 => { fl_l = rcx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 79; continue; } bb = 78; continue; }
            78 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P20, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rbp.wrapping_sub(0x68u64))) as u64; bb = 79; continue; }
            79 => { fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 81; continue; } bb = 80; continue; }
            80 => { crate::simd_ops::wu16(rax, ((r14 as u16)) as u16); bb = 81; continue; }
            81 => { rax = (((rbx as u8)) as u32) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x818u64))) as u64; rsp = ((rsp).wrapping_add(0x7c0u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1424a7300(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143c1bfb4; const P1: u64 = 0x143c1bfa8; const P2: u64 = 0x1413d4870; const P3: u64 = 0xffffffffffffffff; const P4: u64 = 0x143c1bfb0; const P5: u64 = 0x4; const P6: u64 = 0x8;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = ((rcx as u32)) as u32 as u64; r10 = (crate::simd_ops::ru32(P0)) as u32 as u64; rdx = ((r10.wrapping_add(r10.wrapping_mul(2))) as u64) as u64; r9 = (crate::simd_ops::ru64(P1)) as u64; r8 = ((r9.wrapping_add(rdx.wrapping_mul(4))) as u64) as u64; r8 = ((r8).wrapping_sub(r9)) as u64; rsi = (0x2aaaaaaaaaaaaaabu64) as u64; rax = rsi; let prod = (rax as i64 as i128).wrapping_mul((r8) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rdx = ((((rdx) as i64).wrapping_shr(1u32) as u64)) as u64; rax = rdx; rax = ((rax).wrapping_shr(63u32)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; fl_l = rdx; fl_r = (0x1u64) as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rdx = 0x1; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); r10 = (crate::simd_ops::ru32(P0)) as u32 as u64; r9 = (crate::simd_ops::ru64(P1)) as u64; bb = 2; continue; }
            2 => { rdi = ((rbx.wrapping_add(P3)) as u32) as u32 as u64; r8 = (crate::simd_ops::ru32(P4)) as u32 as u64; rdx = ((r8.wrapping_add(r8.wrapping_mul(2))) as u64) as u64; r11 = ((r9.wrapping_add(rdx.wrapping_mul(4))) as u64) as u64; rcx = ((r10 as u32)) as u32 as u64; rdx = ((rcx.wrapping_add(rcx.wrapping_mul(2))) as u64) as u64; rcx = ((r9.wrapping_add(rdx.wrapping_mul(4))) as u64) as u64; fl_l = r11; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { r11 = ((r11).wrapping_sub(r9)) as u64; rax = rsi; let prod = (rax as i64 as i128).wrapping_mul((r11) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rdx = ((((rdx) as i64).wrapping_shr(1u32) as u64)) as u64; rax = rdx; rax = ((rax).wrapping_shr(63u32)) as u64; rax = ((rax).wrapping_add(1)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); r8 = (crate::simd_ops::ru32(P4)) as u32 as u64; r9 = (crate::simd_ops::ru64(P1)) as u64; bb = 4; continue; }
            4 => { rax = ((r8 as u32)) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; crate::simd_ops::wu32(r9.wrapping_add(rcx.wrapping_mul(4)), ((rbx as u32)) as u32); crate::simd_ops::wu32(r9.wrapping_add(rcx.wrapping_mul(4)).wrapping_add(P5), (0x0u32) as u32); crate::simd_ops::wu32(r9.wrapping_add(rcx.wrapping_mul(4)).wrapping_add(P6), ((rdi as u32)) as u32); crate::simd_ops::lk_add_u32(P4, (1) as u32); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1424a73f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x143c1bfb0; const P1: u64 = 0x143c1bfa8; const P2: u64 = 0x4; const P3: u64 = 0x8;
    let (mut rax, mut r10, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r9 = (crate::simd_ops::ru32(P0)) as u32 as u64; rax = 0; fl_l = 0; fl_r = 0; fl_l = ((r9 as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 5; continue; } bb = 1; continue; }
            1 => { r10 = (crate::simd_ops::ru64(P1)) as u64; bb = 2; continue; }
            2 => { r8 = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(r10.wrapping_add(r8.wrapping_mul(4)).wrapping_add(P2))) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(r10.wrapping_add(r8.wrapping_mul(4)).wrapping_add(P3))) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 6; continue; } bb = 4; continue; }
            4 => { rax = (((rax as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 2; continue; } bb = 5; continue; }
            5 => { rax = 0; fl_l = 0; fl_r = 0; bb = 6; continue; }
            6 => { return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1424a8a00(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1434104a8; const P1: u64 = 0x18; const P2: u64 = 0x10; const P3: u64 = 0x142442de0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    rax = P0;
    crate::simd_ops::wu64(rcx, rax);
    fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(P1))) as u8 as u64; fl_r = (0x0u8) as u8 as u64;
    if (((fl_l as u8)) == ((fl_r as u8))) { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    rdi = (crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64;
    fl_l = rdi; fl_r = rdi;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rax = (crate::simd_ops::ru64(rdi)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rdi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = rdi; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); }
    crate::simd_ops::wu64(rbx.wrapping_add(P2), (0x0u64) as u64);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1424a8a60(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1434104d8; const P1: u64 = 0x1413d6c50; const P2: u64 = 0x1d8; const P3: u64 = 0x1401e5380; const P4: u64 = 0x1c8; const P5: u64 = 0x1413d6d80; const P6: u64 = 0x1b0; const P7: u64 = 0x142442de0; const P8: u64 = 0x124; const P9: u64 = 0x14240ab10; const P10: u64 = 0x1423f0c20;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    rax = P0;
    crate::simd_ops::wu64(rcx, rax);
    rcx = ((rcx).wrapping_add(0x208u64)) as u64;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rcx = ((rbx.wrapping_add(P2)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp);
    rcx = ((rbx.wrapping_add(P4)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp);
    rdi = (crate::simd_ops::ru64(rbx.wrapping_add(P6))) as u64;
    fl_l = rdi; fl_r = rdi;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rax = (crate::simd_ops::ru64(rdi)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rdi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = rdi; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); }
    rcx = ((rbx.wrapping_add(P8)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp);
    rcx = rbx;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(P10, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1424a8ae0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14240ab10; const P1: u64 = 0x30; const P2: u64 = 0x18; const P3: u64 = 0x1413d6e70;
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    rcx = ((rcx).wrapping_add(0x78u64)) as u64;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rcx = ((rbx.wrapping_add(P1)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rcx = ((rbx.wrapping_add(P2)) as u64) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(P3, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1424a8b10(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142645378; const P1: u64 = 0x143410580; const P2: u64 = 0x1423fe030; const P3: u64 = 0x142409b50; const P4: u64 = 0x140000000; const P5: u64 = 0x142409430; const P6: u64 = 0x142645388; const P7: u64 = 0x1401f2340; const P8: u64 = 0x1401f2480; const P9: u64 = 0x14240ab10; const P10: u64 = 0x1425371a0; const P11: u64 = 0x142533200; const P12: u64 = 0x14240a730; const P13: u64 = 0x142645370; const P14: u64 = 0x1434104e8; const P15: u64 = 0x1426e8331; const P16: u64 = 0x1405fc3c0; const P17: u64 = 0x1413d70f0; const P18: u64 = 0x14020c080; const P19: u64 = 0x142442de0; const P20: u64 = 0x14183acbc; const P21: u64 = 0x1401ea4e0;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 81>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (r9) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rcx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r12) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r13) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r15) as u64); rbp = ((rsp.wrapping_sub(0x150u64)) as u64) as u64; rsp = ((rsp).wrapping_sub(0x250u64)) as u64; rbx = (r8) as u64; rdi = (rdx) as u64; r14 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(0x228u64))) as u32 as u64; fl_r = ((r14 as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 113; continue; } bb = 1; continue; }
            1 => { xmm0 = 0u128; crate::simd_ops::wu128(rsp.wrapping_add(0x38u64), (xmm0) as u128); r13 = ((r14 as u32)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (r14) as u64); r12 = ((r14 as u32)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), (r12) as u64); crate::simd_ops::wu32(rsp.wrapping_add(0x40u64), ((r14 as u32)) as u32); rsi = ((r14 as u32)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), (rsi) as u64); crate::simd_ops::wu32(rsp.wrapping_add(0x44u64), ((r14 as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), (r14) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P0)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), (rax) as u64); fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 13; continue; } bb = 2; continue; }
            2 => { rdx = (rdi) as u64; rcx = ((P1) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 9; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu32(rbx.wrapping_add(0x4u64), ((r14 as u32)) as u32); rdx = ((rbx.wrapping_add(0x8u64)) as u64) as u64; rcx = ((crate::simd_ops::ru8(rbx.wrapping_add(0x3u64))) as u32) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = (0x80u32) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rcx = ((rcx.wrapping_mul(8)) as u32) as u32 as u64; bb = 6; continue; }
            5 => { rcx = (((rcx as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = (0x7c00u32) as u32 as u64; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; rax = ((((rax as u32)) & (0xffffffu32))) as u32 as u64; rax = (((rax as u32)).wrapping_shl(3u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 8; continue; } bb = 7; continue; }
            7 => { rdx = (crate::simd_ops::ru64(rdx)) as u64; bb = 8; continue; }
            8 => { crate::simd_ops::wu8(rdx, (0x0u8) as u8); bb = 12; continue; }
            9 => { rcx = (0xffffffffffffffffu64) as u64; bb = 10; continue; }
            10 => { rcx = ((rcx).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(rcx))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 10; continue; } bb = 11; continue; }
            11 => { crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), (rax) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), (rcx) as u64); r9 = (rcx) as u64; r8 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rdx = ((rsp.wrapping_add(0x60u64)) as u64) as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 12; continue; }
            12 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 114; continue; }
            13 => { r15 = (rax) as u64; rdi = (0x7878787878787879u64) as u64; fl_l = (crate::simd_ops::ru8(rax)) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 65; continue; } bb = 14; continue; }
            14 => { rbx = (0xffffffffffffffffu64) as u64; rdi = ((P4) as u64) as u64; bb = 15; continue; }
            15 => { crate::simd_ops::wu64(rbp.wrapping_sub(0x60u64), (0x10000010u64) as u64); crate::simd_ops::wu8(rbp.wrapping_sub(0x58u64), (0x0u8) as u8); r9 = (rbx) as u64; bb = 16; continue; }
            16 => { r9 = ((r9).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(r15.wrapping_add(r9))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 16; continue; } bb = 17; continue; }
            17 => { crate::simd_ops::wu64(rbp.wrapping_sub(0x80u64), (r15) as u64); crate::simd_ops::wu64(rbp.wrapping_sub(0x78u64), (r9) as u64); r8 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rdx = ((rbp.wrapping_sub(0x80u64)) as u64) as u64; rcx = ((rbp.wrapping_sub(0x60u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = (crate::simd_ops::ru32(rbp.wrapping_sub(0x60u64))) as u32 as u64; rax = ((rdx as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shr(24u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x80u32) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 19; continue; } bb = 18; continue; }
            18 => { rax = ((rax.wrapping_mul(8)) as u32) as u32 as u64; bb = 20; continue; }
            19 => { rax = (((rax as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x7c00u32) as u32 as u64; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 20; continue; }
            20 => { rdx = ((((rdx as u32)) & (0xffffffu32))) as u32 as u64; rdx = (((rdx as u32)).wrapping_shl(3u32)) as u32 as u64; rcx = ((rbp.wrapping_sub(0x58u64)) as u64) as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; rcx = (if (((fl_l as u32)) > ((fl_r as u32))) { crate::simd_ops::ru64(rbp.wrapping_sub(0x58u64)) } else { rcx }) as u64; rdx = (r14) as u64; r8 = (crate::simd_ops::ru32(rbp.wrapping_sub(0x5cu64))) as u32 as u64; fl_l = (r8) as u64 as u64; fl_r = (r8) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 24; continue; } bb = 21; continue; }
            21 => { bb = 22; continue; }
            22 => { rax = ((crate::simd_ops::ru8(rdx.wrapping_add(rcx))) as u32) as u32 as u64; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(rdi).wrapping_add(0x27e54e0u64))) as u8 as u64; fl_r = (0x3du8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 25; continue; } bb = 23; continue; }
            23 => { rdx = ((rdx).wrapping_add(1)) as u64; fl_l = (rdx) as u64 as u64; fl_r = (r8) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 22; continue; } bb = 24; continue; }
            24 => { rdx = (rbx) as u64; bb = 25; continue; }
            25 => { r8 = (rbx) as u64; rcx = ((rbp.wrapping_sub(0x60u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu8(rsp.wrapping_add(0x30u64), (0x0u8) as u8); rbx = ((r14 as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x190u64))) as u64; rdi = (crate::simd_ops::ru32(rax.wrapping_add(0x228u64))) as u32 as u64; rsi = (crate::simd_ops::ru64(rax.wrapping_add(0x220u64))) as u64; fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 42; continue; } bb = 26; continue; }
            26 => { rcx = (r14) as u64; rax = (crate::simd_ops::ru32(rbp.wrapping_sub(0x60u64))) as u32 as u64; r11 = ((rax as u32)) as u32 as u64; r11 = (((r11 as u32)).wrapping_shr(24u32)) as u32 as u64; fl_l = ((((rax as u32)) & (0xffffffu32))) as u32 as u64; fl_r = fl_l; rax = ((fl_l as u32)) as u32 as u64; rdx = ((rax.wrapping_mul(8)) as u32) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x48u64), ((rdx as u32)) as u32); r9 = (crate::simd_ops::ru32(rbp.wrapping_sub(0x5cu64))) as u32 as u64; bb = 27; continue; }
            27 => { r8 = (((rcx) as i64).wrapping_mul((0x110u64) as i64) as u64) as u64; r8 = ((r8).wrapping_add(rsi)) as u64; fl_l = ((r11 as u32)) as u32 as u64; fl_r = (0x80u32) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 29; continue; } bb = 28; continue; }
            28 => { rcx = ((r11.wrapping_mul(8)) as u32) as u32 as u64; bb = 30; continue; }
            29 => { rcx = ((r11 as u32)) as u32 as u64; rcx = (((rcx as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = (0x7c00u32) as u32 as u64; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 30; continue; }
            30 => { rax = ((rbp.wrapping_sub(0x58u64)) as u64) as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; rax = (if (((fl_l as u32)) > ((fl_r as u32))) { crate::simd_ops::ru64(rbp.wrapping_sub(0x58u64)) } else { rax }) as u64; r10 = ((r8.wrapping_add(0x8u64)) as u64) as u64; rdx = ((crate::simd_ops::ru8(r8.wrapping_add(0x3u64))) as u32) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = (0x80u32) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 32; continue; } bb = 31; continue; }
            31 => { rdx = ((rdx.wrapping_mul(8)) as u32) as u32 as u64; bb = 33; continue; }
            32 => { rdx = (((rdx as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = (0x7c00u32) as u32 as u64; rdx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 33; continue; }
            33 => { rcx = (crate::simd_ops::ru32(r8)) as u32 as u64; rcx = ((((rcx as u32)) & (0xffffffu32))) as u32 as u64; rcx = (((rcx as u32)).wrapping_shl(3u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 35; continue; } bb = 34; continue; }
            34 => { r10 = (crate::simd_ops::ru64(r10)) as u64; bb = 35; continue; }
            35 => { rdx = (crate::simd_ops::ru32(r8.wrapping_add(0x4u64))) as u32 as u64; fl_l = (rdx) as u64 as u64; fl_r = (r9) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 56; continue; } bb = 36; continue; }
            36 => { fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 41; continue; } bb = 37; continue; }
            37 => { fl_l = (r10) as u64 as u64; fl_r = (rax) as u64 as u64; r10 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; bb = 38; continue; }
            38 => { rdx = ((rdx).wrapping_sub(1)) as u64; rcx = ((crate::simd_ops::ru8(rax.wrapping_add(r10))) as u32) as u32 as u64; r9 = ((P4) as u64) as u64; r8 = ((crate::simd_ops::ru8(rcx.wrapping_add(r9).wrapping_add(0x26d01e0u64))) as u32) as u32 as u64; rcx = ((crate::simd_ops::ru8(rax)) as u32) as u32 as u64; fl_l = ((r8 as u8)) as u8 as u64; fl_r = (crate::simd_ops::ru8(rcx.wrapping_add(r9).wrapping_add(0x26d01e0u64))) as u8 as u64; if ((((fl_l as u8)) as i8) < (((fl_r as u8)) as i8)) { bb = 55; continue; } bb = 39; continue; }
            39 => { if ((((fl_l as u8)) as i8) > (((fl_r as u8)) as i8)) { bb = 55; continue; } bb = 40; continue; }
            40 => { rax = ((rax).wrapping_add(1)) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 38; continue; } bb = 41; continue; }
            41 => { crate::simd_ops::wu8(rsp.wrapping_add(0x30u64), (0x1u8) as u8); bb = 42; continue; }
            42 => { rcx = (r15) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P6)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = (((rax as u32)).wrapping_add(1)) as u32 as u64; rsi = ((((rax as u32)) as i32 as i64) as u64) as u64; fl_l = (crate::simd_ops::ru8(rsp.wrapping_add(0x30u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 63; continue; } bb = 43; continue; }
            43 => { crate::simd_ops::wu64(rbp.wrapping_add(0xc0u64), (0x10000010u64) as u64); crate::simd_ops::wu8(rbp.wrapping_add(0xc8u64), (0x0u8) as u8); r9 = (0xffffffffffffffffu64) as u64; bb = 44; continue; }
            44 => { r9 = ((r9).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(r15.wrapping_add(r9))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 44; continue; } bb = 45; continue; }
            45 => { crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), (r15) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), (r9) as u64); r8 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rdx = ((rsp.wrapping_add(0x70u64)) as u64) as u64; rcx = ((rbp.wrapping_add(0xc0u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = ((r12 as u32)) as u32 as u64; rdi = (((rax) as i64).wrapping_mul((0x88u64) as i64) as u64) as u64; rdi = ((rdi).wrapping_add(r13)) as u64; rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x58u64))) as u32 as u64; r8 = (((rax) as i64).wrapping_mul((0x88u64) as i64) as u64) as u64; r8 = ((r8).wrapping_add(r13)) as u64; fl_l = (rdi) as u64 as u64; fl_r = (r8) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 62; continue; } bb = 46; continue; }
            46 => { rcx = (rdi) as u64; rcx = ((rcx).wrapping_sub(r13)) as u64; r9 = (0x7878787878787879u64) as u64; rax = (r9) as u64; let prod = (rax as i64 as i128).wrapping_mul((rcx) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; r12 = (rdx) as u64; r12 = ((((r12) as i64).wrapping_shr(6u32) as u64)) as u64; rax = (r12) as u64; rax = ((rax).wrapping_shr(63u32)) as u64; r12 = ((r12).wrapping_add(rax)) as u64; rcx = ((r12.wrapping_add(0x1u64)) as u64) as u64; r8 = ((r8).wrapping_sub(r13)) as u64; rax = (r9) as u64; let prod = (rax as i64 as i128).wrapping_mul((r8) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rdx = ((((rdx) as i64).wrapping_shr(6u32) as u64)) as u64; rax = (rdx) as u64; rax = ((rax).wrapping_shr(63u32)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; rbx = (rdx) as u64; rbx = ((rbx).wrapping_shr(1u32)) as u64; rbx = ((rbx).wrapping_add(rdx)) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rcx) as u64 as u64; rbx = (if (((fl_l as u64)) < ((fl_r as u64))) { rcx } else { rbx }) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), (rbx) as u64); rdx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r14 = (rax) as u64; r8 = (rax) as u64; rdx = (rdi) as u64; rcx = (r13) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = (r13) as u64 as u64; fl_r = (rdi) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 50; continue; } bb = 47; continue; }
            47 => { rbx = (r13) as u64; bb = 48; continue; }
            48 => { rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rbx = ((rbx).wrapping_add(0x88u64)) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rdi) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 48; continue; } bb = 49; continue; }
            49 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; bb = 50; continue; }
            50 => { fl_l = (r13) as u64 as u64; fl_r = (r13) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 61; continue; } bb = 51; continue; }
            51 => { r10 = ((r13.wrapping_sub(0x1u64)) as u64) as u64; r10 = (((r10) & (0xfffffffffe000000u64))) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = (rax) as u64 as u64; fl_r = (crate::simd_ops::ru64(r10.wrapping_add(0x100u64))) as u64 as u64; rdx = (rdx & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rax = (r9) as u64; rax = ((rax).wrapping_sub(r10)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rcx = ((rcx).wrapping_add(0x108u64)) as u64; rcx = ((rcx).wrapping_add(r10)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x4u64))) as u32 as u64; rcx = ((rcx).wrapping_sub(rax)) as u64; fl_l = ((rdx as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 59; continue; } bb = 52; continue; }
            52 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0xeu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 58; continue; } bb = 53; continue; }
            53 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(r9, (rax) as u64); crate::simd_ops::wu64(rcx.wrapping_add(0x20u64), (r9) as u64); fl_l = (crate::simd_ops::lk_add_u32(rcx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 61; continue; } bb = 54; continue; }
            54 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P10) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 61; continue; }
            55 => { r9 = (crate::simd_ops::ru32(rbp.wrapping_sub(0x5cu64))) as u32 as u64; bb = 56; continue; }
            56 => { rbx = (((rbx as u32)).wrapping_add(1)) as u32 as u64; rcx = ((((rbx as u32)) as i32 as i64) as u64) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rdi) as u64 as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 42; continue; } bb = 57; continue; }
            57 => { rdx = (crate::simd_ops::ru32(rsp.wrapping_add(0x48u64))) as u32 as u64; bb = 27; continue; }
            58 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 60; continue; }
            59 => { r8 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 60; continue; }
            60 => { rdx = (rcx) as u64; rcx = (r10) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P11) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 61; continue; }
            61 => { r13 = (r14) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (r14) as u64); crate::simd_ops::wu32(rsp.wrapping_add(0x40u64), ((r12 as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x44u64), ((rbx as u32)) as u32); r14 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 62; continue; }
            62 => { rax = ((r12 as u32)) as u32 as u64; rcx = (((rax) as i64).wrapping_mul((0x88u64) as i64) as u64) as u64; rcx = ((rcx).wrapping_add(r13)) as u64; crate::simd_ops::lk_and_u32(rcx, (0xff000010u32) as u32); crate::simd_ops::lk_or_u32(rcx, (0x10u32) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x3u64), (0x10u8) as u8); crate::simd_ops::wu32(rcx.wrapping_add(0x4u64), ((r14 as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x8u64), (0x0u8) as u8); rdx = ((rbp.wrapping_add(0xc0u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P12) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r12 = (((r12 as u32)).wrapping_add(1)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x40u64), ((r12 as u32)) as u32); rcx = ((rbp.wrapping_add(0xc0u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = (crate::simd_ops::lk_add_u64(rsp.wrapping_add(0x50u64), (rsi) as u64)) as u64; fl_r = fl_l; bb = 63; continue; }
            63 => { r15 = ((r15).wrapping_add(rsi)) as u64; rcx = ((rbp.wrapping_sub(0x60u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = (crate::simd_ops::ru8(r15)) as u8 as u64; fl_r = (0x0u8) as u8 as u64; rbx = (0xffffffffffffffffu64) as u64; rdi = ((P4) as u64) as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 15; continue; } bb = 64; continue; }
            64 => { crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), (r12) as u64); rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rdi = (0x7878787878787879u64) as u64; bb = 65; continue; }
            65 => { rcx = (rax) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P13)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r12 = ((r14 as u32)) as u32 as u64; rbx = (crate::simd_ops::ru64(rbp.wrapping_add(0x190u64))) as u64; rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x220u64))) as u64; r15 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x228u64))) as u32 as u64; fl_r = ((r15 as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 98; continue; } bb = 66; continue; }
            66 => { rax = (r14) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; bb = 67; continue; }
            67 => { rax = (((rax) as i64).wrapping_mul((0x110u64) as i64) as u64) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; r8 = ((rdx.wrapping_add(0x90u64)) as u64) as u64; rcx = ((crate::simd_ops::ru8(rdx.wrapping_add(0x8bu64))) as u32) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = (0x80u32) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 69; continue; } bb = 68; continue; }
            68 => { rcx = ((rcx.wrapping_mul(8)) as u32) as u32 as u64; bb = 70; continue; }
            69 => { rcx = (((rcx as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = (0x7c00u32) as u32 as u64; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 70; continue; }
            70 => { rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x88u64))) as u32 as u64; rax = ((((rax as u32)) & (0xffffffu32))) as u32 as u64; rax = (((rax as u32)).wrapping_shl(3u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 72; continue; } bb = 71; continue; }
            71 => { r8 = (crate::simd_ops::ru64(r8)) as u64; bb = 72; continue; }
            72 => { r9 = ((rdx.wrapping_add(0x8u64)) as u64) as u64; rcx = ((crate::simd_ops::ru8(rdx.wrapping_add(0x3u64))) as u32) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = (0x80u32) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 74; continue; } bb = 73; continue; }
            73 => { rcx = ((rcx.wrapping_mul(8)) as u32) as u32 as u64; bb = 75; continue; }
            74 => { rcx = (((rcx as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = (0x7c00u32) as u32 as u64; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 75; continue; }
            75 => { rax = (crate::simd_ops::ru32(rdx)) as u32 as u64; rax = ((((rax as u32)) & (0xffffffu32))) as u32 as u64; rax = (((rax as u32)).wrapping_shl(3u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 77; continue; } bb = 76; continue; }
            76 => { r9 = (crate::simd_ops::ru64(r9)) as u64; bb = 77; continue; }
            77 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (r8) as u64); r8 = ((P14) as u64) as u64; rdx = ((crate::simd_ops::ru8(P15)) as u32) as u32 as u64; rcx = ((rbp.wrapping_add(0x30u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P16) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = ((r14 as u32)) as u32 as u64; rdi = (((rax) as i64).wrapping_mul((0x88u64) as i64) as u64) as u64; rdi = ((rdi).wrapping_add(r13)) as u64; rax = ((rsi as u32)) as u32 as u64; r8 = (((rax) as i64).wrapping_mul((0x88u64) as i64) as u64) as u64; r8 = ((r8).wrapping_add(r13)) as u64; fl_l = (rdi) as u64 as u64; fl_r = (r8) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 91; continue; } bb = 78; continue; }
            78 => { rcx = (rdi) as u64; rcx = ((rcx).wrapping_sub(r13)) as u64; r9 = (0x7878787878787879u64) as u64; rax = (r9) as u64; let prod = (rax as i64 as i128).wrapping_mul((rcx) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; r14 = (rdx) as u64; r14 = ((((r14) as i64).wrapping_shr(6u32) as u64)) as u64; rax = (r14) as u64; rax = ((rax).wrapping_shr(63u32)) as u64; r14 = ((r14).wrapping_add(rax)) as u64; rcx = ((r14.wrapping_add(0x1u64)) as u64) as u64; r8 = ((r8).wrapping_sub(r13)) as u64; rax = (r9) as u64; let prod = (rax as i64 as i128).wrapping_mul((r8) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rdx = ((((rdx) as i64).wrapping_shr(6u32) as u64)) as u64; rax = (rdx) as u64; rax = ((rax).wrapping_shr(63u32)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; rsi = (rdx) as u64; rsi = ((rsi).wrapping_shr(1u32)) as u64; rsi = ((rsi).wrapping_add(rdx)) as u64; fl_l = (rsi) as u64 as u64; fl_r = (rcx) as u64 as u64; rsi = (if (((fl_l as u64)) < ((fl_r as u64))) { rcx } else { rsi }) as u64; rdx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r15 = (rax) as u64; r8 = (rax) as u64; rdx = (rdi) as u64; rcx = (r13) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = (r13) as u64 as u64; fl_r = (rdi) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 82; continue; } bb = 79; continue; }
            79 => { rbx = (r13) as u64; bb = 80; continue; }
            80 => { rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rbx = ((rbx).wrapping_add(0x88u64)) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rdi) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 80; continue; } bb = 81; continue; }
            81 => { rbx = (crate::simd_ops::ru64(rbp.wrapping_add(0x190u64))) as u64; bb = 82; continue; }
            82 => { fl_l = (r13) as u64 as u64; fl_r = (r13) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 90; continue; } bb = 83; continue; }
            83 => { r10 = ((r13.wrapping_sub(0x1u64)) as u64) as u64; r10 = (((r10) & (0xfffffffffe000000u64))) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = (rax) as u64 as u64; fl_r = (crate::simd_ops::ru64(r10.wrapping_add(0x100u64))) as u64 as u64; rdx = (rdx & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rax = (r9) as u64; rax = ((rax).wrapping_sub(r10)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rcx = ((rcx).wrapping_add(0x108u64)) as u64; rcx = ((rcx).wrapping_add(r10)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x4u64))) as u32 as u64; rcx = ((rcx).wrapping_sub(rax)) as u64; fl_l = ((rdx as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 88; continue; } bb = 84; continue; }
            84 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0xeu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 87; continue; } bb = 85; continue; }
            85 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(r9, (rax) as u64); crate::simd_ops::wu64(rcx.wrapping_add(0x20u64), (r9) as u64); fl_l = (crate::simd_ops::lk_add_u32(rcx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 90; continue; } bb = 86; continue; }
            86 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P10) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 90; continue; }
            87 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 89; continue; }
            88 => { r8 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 89; continue; }
            89 => { rdx = (rcx) as u64; rcx = (r10) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P11) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 90; continue; }
            90 => { r13 = (r15) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (r15) as u64); crate::simd_ops::wu32(rsp.wrapping_add(0x40u64), ((r14 as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x44u64), ((rsi as u32)) as u32); r15 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 91; continue; }
            91 => { rax = ((r14 as u32)) as u32 as u64; rcx = (((rax) as i64).wrapping_mul((0x88u64) as i64) as u64) as u64; rcx = ((rcx).wrapping_add(r13)) as u64; crate::simd_ops::lk_and_u32(rcx, (0xff000010u32) as u32); crate::simd_ops::lk_or_u32(rcx, (0x10u32) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x3u64), (0x10u8) as u8); crate::simd_ops::wu32(rcx.wrapping_add(0x4u64), ((r15 as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x8u64), (0x0u8) as u8); rax = ((rbp.wrapping_add(0x30u64)) as u64) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 96; continue; } bb = 92; continue; }
            92 => { r9 = (crate::simd_ops::ru32(rbp.wrapping_add(0x34u64))) as u32 as u64; r8 = ((crate::simd_ops::ru8(rbp.wrapping_add(0x33u64))) as u32) as u32 as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = (0x80u32) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 94; continue; } bb = 93; continue; }
            93 => { r8 = ((r8.wrapping_mul(8)) as u32) as u32 as u64; bb = 95; continue; }
            94 => { r8 = (((r8 as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = (0x7c00u32) as u32 as u64; r8 = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 95; continue; }
            95 => { rdx = (crate::simd_ops::ru32(rbp.wrapping_add(0x30u64))) as u32 as u64; rdx = ((((rdx as u32)) & (0xffffffu32))) as u32 as u64; rdx = (((rdx as u32)).wrapping_shl(3u32)) as u32 as u64; rax = ((rbp.wrapping_add(0x38u64)) as u64) as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; rax = (if (((fl_l as u32)) > ((fl_r as u32))) { crate::simd_ops::ru64(rbp.wrapping_add(0x38u64)) } else { rax }) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), (rax) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), (r9) as u64); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x60u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x70u64), (xmm0) as u128); r8 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rdx = ((rsp.wrapping_add(0x70u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 96; continue; }
            96 => { r14 = (((r14 as u32)).wrapping_add(1)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x40u64), ((r14 as u32)) as u32); rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x34u64))) as u32 as u64; rax = ((rax).wrapping_add(1)) as u64; crate::simd_ops::lk_add_u64(rsp.wrapping_add(0x50u64), (rax) as u64); rcx = ((rbp.wrapping_add(0x30u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r12 = (((r12 as u32)).wrapping_add(1)) as u32 as u64; rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x228u64))) as u32 as u64; rax = ((((r12 as u32)) as i32 as i64) as u64) as u64; rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x220u64))) as u64; fl_l = (rax) as u64 as u64; fl_r = (rcx) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 67; continue; } bb = 97; continue; }
            97 => { rdi = (0x7878787878787879u64) as u64; bb = 99; continue; }
            98 => { r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; bb = 99; continue; }
            99 => { rcx = ((r14 as u32)) as u32 as u64; rcx = (((rcx) as i64).wrapping_mul((0x88u64) as i64) as u64) as u64; rcx = ((rcx).wrapping_add(r13)) as u64; rdx = (rcx) as u64; rdx = ((rdx).wrapping_sub(r13)) as u64; rax = (rdi) as u64; let prod = (rax as i64 as i128).wrapping_mul((rdx) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rsi = (rdx) as u64; rsi = ((((rsi) as i64).wrapping_shr(6u32) as u64)) as u64; rax = (rsi) as u64; rax = ((rax).wrapping_shr(63u32)) as u64; rsi = ((rsi).wrapping_add(rax)) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x190u64), (rcx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), (r13) as u64); r9 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; r8 = (rsi) as u64; rdx = ((rbp.wrapping_add(0x190u64)) as u64) as u64; rcx = ((rsp.wrapping_add(0x60u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P17) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; r9 = ((r9).wrapping_add(1)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P18) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = ((rbp.wrapping_sub(0x70u64)) as u64) as u64; rbx = (crate::simd_ops::ru64(rbp.wrapping_add(0x1a8u64))) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rcx) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 102; continue; } bb = 100; continue; }
            100 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu64(rbx, (rax) as u64); fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 104; continue; } bb = 101; continue; }
            101 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P19) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 104; continue; }
            102 => { fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 104; continue; } bb = 103; continue; }
            103 => { rcx = (rax) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P19) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 104; continue; }
            104 => { rdi = (crate::simd_ops::ru64(rbx)) as u64; fl_l = (rsi) as u64 as u64; fl_r = (rsi) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 112; continue; } bb = 105; continue; }
            105 => { rax = (r15) as u64; bb = 106; continue; }
            106 => { rbx = (((rax) as i64).wrapping_mul((0x88u64) as i64) as u64) as u64; rbx = ((rbx).wrapping_add(r13)) as u64; rdx = ((rbx.wrapping_add(0x8u64)) as u64) as u64; rcx = ((crate::simd_ops::ru8(rbx.wrapping_add(0x3u64))) as u32) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = (0x80u32) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 108; continue; } bb = 107; continue; }
            107 => { rcx = ((rcx.wrapping_mul(8)) as u32) as u32 as u64; bb = 109; continue; }
            108 => { rcx = (((rcx as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = (0x7c00u32) as u32 as u64; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 109; continue; }
            109 => { rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; rax = ((((rax as u32)) & (0xffffffu32))) as u32 as u64; rax = (((rax as u32)).wrapping_shl(3u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 111; continue; } bb = 110; continue; }
            110 => { rdx = (crate::simd_ops::ru64(rdx)) as u64; bb = 111; continue; }
            111 => { r8 = (crate::simd_ops::ru32(rbx.wrapping_add(0x4u64))) as u32 as u64; r8 = ((r8).wrapping_add(1)) as u64; rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P20) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x4u64))) as u32 as u64; rax = ((rax).wrapping_add(1)) as u64; rdi = ((rdi).wrapping_add(rax)) as u64; r15 = (((r15 as u32)).wrapping_add(1)) as u32 as u64; rax = ((((r15 as u32)) as i32 as i64) as u64) as u64; fl_l = (rax) as u64 as u64; fl_r = (rsi) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 106; continue; } bb = 112; continue; }
            112 => { crate::simd_ops::wu8(rdi, (0x0u8) as u8); rcx = ((rsp.wrapping_add(0x38u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P21) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 113; continue; }
            113 => { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 114; continue; }
            114 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x298u64))) as u64; rsp = ((rsp).wrapping_add(0x250u64)) as u64; r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r13 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r12 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rsi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1424ab130(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143410618; const P1: u64 = 0x14240ab10; const P2: u64 = 0x220; const P3: u64 = 0x1413d7010; const P4: u64 = 0x110; const P5: u64 = 0x8;
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    rax = P0;
    crate::simd_ops::wu64(rcx, rax);
    rcx = ((rcx).wrapping_add(0x230u64)) as u64;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rcx = ((rbx.wrapping_add(P2)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp);
    rcx = ((rbx.wrapping_add(P4)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rcx = ((rbx.wrapping_add(P5)) as u64) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(P1, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1424ab1d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142645390; const P1: u64 = 0x142645548; const P2: u64 = 0x1434105c8; const P3: u64 = 0x1424bff90; const P4: u64 = 0x142409b50; const P5: u64 = 0x14240aea0; const P6: u64 = 0x14240a730; const P7: u64 = 0x14240ab10;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<512, 211>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 15; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rbp = rsp.wrapping_sub(0x570); rsp = ((rsp).wrapping_sub(0x670u64)) as u64; rsi = r8; r14 = rdx; r15 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rbp.wrapping_add(0x5a0u64), ((r15 as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), r15); rax = rbp.wrapping_add(0x5a0); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); r9 = 0; fl_l = 0; fl_r = 0; r8 = 0; fl_l = 0; fl_r = 0; rdx = 0; fl_l = 0; fl_r = 0; rcx = r14; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 14; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x5a0u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 14; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu32(rbp.wrapping_add(0x5a8u64), ((r15 as u32)) as u32); r8 = 0x1ff; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x200; r8 = (if (((fl_l as u32)) < ((fl_r as u32))) { (rax as u32) } else { (r8 as u32) }) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r15); r9 = rbp.wrapping_add(0x5a8); rdx = rbp.wrapping_add(0x370); rcx = r14; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rcx = P2; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x5a8u64))) as u32 as u64; crate::simd_ops::wu8(rbp.wrapping_add(rax).wrapping_add(0x370u64), (0x0u8) as u8); rbx = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; rdi = (crate::simd_ops::ru64(rsi)) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x50u64), (0x40000040u64) as u64); crate::simd_ops::wu8(rbp.wrapping_add(0x58u64), (0x0u8) as u8); rax = rbp.wrapping_add(0x370); r9 = (0xffffffffffffffffu64) as u64; bb = 6; continue; }
            6 => { r9 = ((r9).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(r9))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 6; continue; } bb = 7; continue; }
            7 => { rax = rbp.wrapping_add(0x370); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), r9); r8 = 0; fl_l = 0; fl_r = 0; rdx = rsp.wrapping_add(0x30); rcx = rbp.wrapping_add(0x50); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rdx = rbp.wrapping_add(0x50); rcx = rsp.wrapping_add(0x40); rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru32(rsp.wrapping_add(0x44u64))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 13; continue; } bb = 8; continue; }
            8 => { bb = 9; continue; }
            9 => { rcx = (crate::simd_ops::ru32(rsp.wrapping_add(0x40u64))) as u32 as u64; rax = ((rcx as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shr(24u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 11; continue; } bb = 10; continue; }
            10 => { rax = ((rax.wrapping_mul(8)) as u32) as u32 as u64; bb = 12; continue; }
            11 => { rax = (((rax as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x7c00; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 12; continue; }
            12 => { rcx = ((((rcx as u32)) & (0xffffffu32))) as u32 as u64; rcx = (((rcx as u32)).wrapping_shl(3u32)) as u32 as u64; rdx = rsp.wrapping_add(0x48); fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; rdx = (if (((fl_l as u32)) > ((fl_r as u32))) { crate::simd_ops::ru64(rsp.wrapping_add(0x48u64)) } else { rdx }) as u64; rcx = rdi; rax = crate::simd_ops::call_stk(rbx, rax, rcx, rdx, r8, r9, rsp); rdx = rbp.wrapping_add(0x50); rcx = rbp.wrapping_add(0x260); rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rdx = rbp.wrapping_add(0x260); rcx = rsp.wrapping_add(0x40); rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rcx = rbp.wrapping_add(0x260); rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru32(rsp.wrapping_add(0x44u64))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 9; continue; } bb = 13; continue; }
            13 => { rcx = rsp.wrapping_add(0x40); rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rcx = rbp.wrapping_add(0x50); rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x5a0u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rbp.wrapping_add(0x5a8u64))) as u32 as u64; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x5a0u64), ((rax as u32)) as u32); if (((fl_l as u32)) != ((fl_r as u32))) { bb = 3; continue; } bb = 14; continue; }
            14 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x6b0u64))) as u64; rsp = ((rsp).wrapping_add(0x670u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); bb = 15; continue; }
            15 => { return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1424ab3c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x120; const P1: u64 = 0x121; const P2: u64 = 0x1424c0150; const P3: u64 = 0x1424c01e0; const P4: u64 = 0x1413d7000; const P5: u64 = 0x128; const P6: u64 = 0x1424ab1d0; const P7: u64 = 0x130;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; crate::simd_ops::wu8(rcx.wrapping_add(P0), (0x1u8) as u8); rbx = rcx; rax = ((crate::simd_ops::ru8(rcx.wrapping_add(P1))) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rbp); rbp = P2; crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rsi); rsi = P3; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rdi); rdi = P4; bb = 2; continue; }
            2 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P5))) as u64; r8 = rsp.wrapping_add(0x20); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P7))) as u64; r8 = rsp.wrapping_add(0x30); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbp); rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rax = ((crate::simd_ops::ru8(rbx.wrapping_add(P1))) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 2; continue; } bb = 3; continue; }
            3 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rax = 0; fl_l = 0; fl_r = 0; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; crate::simd_ops::wu8(rbx.wrapping_add(P0), (0x0u8) as u8); rsp = ((rsp).wrapping_add(0x40u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            4 => { crate::simd_ops::wu8(rcx.wrapping_add(P0), (0x0u8) as u8); rax = 0; fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1424ab4a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x128; const P1: u64 = 0x130;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    crate::simd_ops::wu64(rcx.wrapping_add(P0), rdx);
    crate::simd_ops::wu64(rcx.wrapping_add(P1), r8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1424ab4c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14183acc2; const P1: u64 = 0x1434105b8; const P2: u64 = 0x1426469b8; const P3: u64 = 0x8; const P4: u64 = 0x2; const P5: u64 = 0x80; const P6: u64 = 0x70; const P7: u64 = 0xffffffffffffff80; const P8: u64 = 0xffffffffffffffa0; const P9: u64 = 0xffffffffffffffc0; const P10: u64 = 0xffffffffffffffe0; const P11: u64 = 0xfffffffffffffff0; const P12: u64 = 0x143410640;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 37>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x120u64)) as u64; rbx = rcx; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0xffffffffu32) as u32); rdi = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rsp.wrapping_add(0x2cu64), (0x4u32) as u32); rcx = rsp.wrapping_add(0x31); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rdx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu8(rsp.wrapping_add(0x30u64), ((rdi as u8)) as u8); r8 = 0xef; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = P1; rdx = 0xef; rcx = rsp.wrapping_add(0x31); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = ((rbx.wrapping_add(P3)) as u64) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x2cu64), (0x6u32) as u32); rax = rsp.wrapping_add(0x20); rdx = ((rdi.wrapping_add(P4)) as u32) as u32 as u64; bb = 1; continue; }
            1 => { rcx = ((rcx.wrapping_add(P5)) as u64) as u64; let y_addr: u64 = rax; xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); xmm1 = (crate::simd_ops::ru128(rax.wrapping_add(P6))) as u128; rax = ((rax.wrapping_add(P5)) as u64) as u64; let y_addr: u64 = rcx.wrapping_add(P7); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); let y_addr: u64 = rax.wrapping_add(P8); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rcx.wrapping_add(P8); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); let y_addr: u64 = rax.wrapping_add(P9); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rcx.wrapping_add(P9); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); xmm0 = (crate::simd_ops::ru128(rax.wrapping_add(P10))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rcx.wrapping_add(P10), (xmm0) as u128); crate::simd_ops::wu128(rcx.wrapping_add(P11), (xmm1) as u128); fl_l = rdx; fl_r = (0x1u64) as u64; rdx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 2; continue; }
            2 => { rax = P12; crate::simd_ops::wu64(rbx.wrapping_add(0x108u64), rdi); crate::simd_ops::wu64(rbx, rax); rax = rbx; crate::simd_ops::wu64(rbx.wrapping_add(0x110u64), rdi); crate::simd_ops::wu8(rbx.wrapping_add(0x118u64), ((rdi as u8)) as u8); crate::simd_ops::wu16(rbx.wrapping_add(0x120u64), ((rdi as u16)) as u16); crate::simd_ops::wu64(rbx.wrapping_add(0x128u64), rdi); crate::simd_ops::wu64(rbx.wrapping_add(0x130u64), rdi); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x130u64))) as u64; rsp = ((rsp).wrapping_add(0x120u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1424ad240(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143410988; const P1: u64 = 0x142409b50; const P2: u64 = 0x14020bcb0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), r8); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rsi = r8; rbp = rdx; rdi = rcx; rax = P0; crate::simd_ops::wu64(rcx, rax); rcx = ((rcx).wrapping_add(0x8u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), rcx); crate::simd_ops::lk_and_u32(rcx, (0xff000008u32) as u32); crate::simd_ops::lk_or_u32(rcx, (0x8u32) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x3u64), (0x8u8) as u8); r14 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rcx.wrapping_add(0x4u64), ((r14 as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x8u64), ((r14 as u8)) as u8); xmm0 = (crate::simd_ops::ru128(r9)) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); r9 = (crate::simd_ops::ru64(r9.wrapping_add(0x8u64))) as u64; r8 = 0; fl_l = 0; fl_r = 0; rdx = rsp.wrapping_add(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rdi.wrapping_add(0x50); crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), rcx); crate::simd_ops::lk_and_u32(rcx, (0xff000008u32) as u32); crate::simd_ops::lk_or_u32(rcx, (0x8u32) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x3u64), (0x8u8) as u8); crate::simd_ops::wu32(rcx.wrapping_add(0x4u64), ((r14 as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x8u64), ((r14 as u8)) as u8); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x88u64))) as u64; xmm0 = (crate::simd_ops::ru128(rax)) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); r9 = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; r8 = 0; fl_l = 0; fl_r = 0; rdx = rsp.wrapping_add(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu8(rdi.wrapping_add(0x98u64), ((r14 as u8)) as u8); xmm0 = 0u128; crate::simd_ops::wu128(rdi.wrapping_add(0xa0u64), (xmm0) as u128); crate::simd_ops::wu64(rdi.wrapping_add(0xa0u64), r14); crate::simd_ops::wu64(rdi.wrapping_add(0xa8u64), r14); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; crate::simd_ops::wu64(rdi.wrapping_add(0xb0u64), rax); rbx = rdi.wrapping_add(0xb8); crate::simd_ops::wu64(rsp.wrapping_add(0x88u64), rbx); crate::simd_ops::wu64(rbx.wrapping_add(0x38u64), r14); rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x38u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rbx.wrapping_add(0x38u64), rax); bb = 2; continue; }
            2 => { rbx = rbp.wrapping_add(0x1b0); rdx = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0xcu64))) as u32 as u64; rcx = ((rcx).wrapping_shl(3u32)) as u64; rax = ((rdx.wrapping_mul(8)) as u64) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rdx = ((rdx).wrapping_add(1)) as u64; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; rax = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu64(rax.wrapping_add(rcx.wrapping_mul(8)), rdi); crate::simd_ops::lk_add_u32(rbx.wrapping_add(0x8u64), (1) as u32); rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x38u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rcx)) as u64; fl_l = rcx; fl_r = rsi; rdx = (rdx & !0xffu64) | (((if (((fl_l as u64)) != ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x20u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rsi.wrapping_add(0x38u64), r14); bb = 6; continue; }
            6 => { rax = rdi; rsp = ((rsp).wrapping_add(0x30u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1424ad3b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143079708; const P1: u64 = 0x1424ad430; const P2: u64 = 0x1424ad240; const P3: u64 = 0x143410988;
    let (mut rax, mut r11, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 19>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    r11 = rsp;
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x90u64)) as u64;
    xmm1 = (crate::simd_ops::ru128(r8)) as u128;
    rax = P0;
    rbx = rcx;
    crate::simd_ops::wu64(r11.wrapping_sub(0x48u64), rax);
    r8 = r11.wrapping_sub(0x48);
    rax = P1;
    crate::simd_ops::wu64(r11.wrapping_sub(0x40u64), rax);
    rax = r11.wrapping_sub(0x48);
    crate::simd_ops::wu64(r11.wrapping_sub(0x10u64), rax);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xc0u64))) as u64;
    crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm1) as u128);
    xmm0 = (crate::simd_ops::ru128(rax)) as u128;
    rax = r11.wrapping_sub(0x68);
    crate::simd_ops::wu64(r11.wrapping_sub(0x70u64), rax);
    crate::simd_ops::wu64(r11.wrapping_sub(0x78u64), r9);
    r9 = r11.wrapping_sub(0x58);
    crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128);
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rax = P3;
    crate::simd_ops::wu64(rbx, rax);
    rax = rbx;
    rsp = ((rsp).wrapping_add(0x90u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1424ad440(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1434107fc; const P1: u64 = 0x1426d01e0; const P2: u64 = 0x1401e1a40; const P3: u64 = 0x1434078c4;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rbx = (crate::simd_ops::ru64(r8.wrapping_add(0x8u64))) as u64; rdi = P0; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rdi); r14 = r8; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (0x2u64) as u64); r15 = rdx; r12 = P1; rbp = rcx; xmm0 = (crate::simd_ops::ru128(r8)) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 20; continue; } bb = 1; continue; }
            1 => { r11 = ((xmm0 as u64)) as u64; r10 = ((rbx.wrapping_add(r11)) as u64) as u64; fl_l = r11; fl_r = r10; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 20; continue; } bb = 2; continue; }
            2 => { rax = ((crate::simd_ops::ru8(r11)) as u32) as u32 as u64; rcx = r11; bb = 3; continue; }
            3 => { r9 = ((crate::simd_ops::ru8(rax.wrapping_add(r12))) as u32) as u32 as u64; rdx = rdi; r8 = 0x2; bb = 4; continue; }
            4 => { rax = ((crate::simd_ops::ru8(rdx)) as u32) as u32 as u64; r8 = ((r8).wrapping_sub(1)) as u64; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(r12))) as u8 as u64; fl_r = ((r9 as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 15; continue; } bb = 5; continue; }
            5 => { rdx = ((rdx).wrapping_add(1)) as u64; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 4; continue; } bb = 6; continue; }
            6 => { rcx = ((rcx).wrapping_sub(r11)) as u64; fl_l = rcx; fl_r = (0xffffffffffffffffu64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 20; continue; } bb = 7; continue; }
            7 => { r11 = ((r11).wrapping_add(rcx)) as u64; fl_l = rbx; fl_r = rcx; rbx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r11); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rbx); if (((fl_l as u64)) == ((fl_r as u64))) { bb = 19; continue; } bb = 8; continue; }
            8 => { r10 = ((r11.wrapping_add(rbx)) as u64) as u64; fl_l = r11; fl_r = r10; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 19; continue; } bb = 9; continue; }
            9 => { r8 = r11; bb = 10; continue; }
            10 => { rcx = ((crate::simd_ops::ru8(r8)) as u32) as u32 as u64; rdx = 0x2; rax = rdi; r9 = ((crate::simd_ops::ru8(rcx.wrapping_add(r12))) as u32) as u32 as u64; bb = 11; continue; }
            11 => { rcx = ((crate::simd_ops::ru8(rax)) as u32) as u32 as u64; rdx = ((rdx).wrapping_sub(1)) as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(r12))) as u8 as u64; fl_r = ((r9 as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 17; continue; } bb = 12; continue; }
            12 => { rax = ((rax).wrapping_add(1)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 11; continue; } bb = 13; continue; }
            13 => { r8 = ((r8).wrapping_add(1)) as u64; fl_l = r8; fl_r = r10; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 10; continue; } bb = 14; continue; }
            14 => { bb = 19; continue; }
            15 => { rcx = ((rcx).wrapping_add(1)) as u64; fl_l = rcx; fl_r = r10; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 20; continue; } bb = 16; continue; }
            16 => { rax = ((crate::simd_ops::ru8(rcx)) as u32) as u32 as u64; bb = 3; continue; }
            17 => { r8 = ((r8).wrapping_sub(r11)) as u64; fl_l = r8; fl_r = (0xffffffffffffffffu64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 19; continue; } bb = 18; continue; }
            18 => { fl_l = rbx; fl_r = r8; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r11); r8 = (if (((fl_l as u64)) < ((fl_r as u64))) { rbx } else { r8 }) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), r8); bb = 19; continue; }
            19 => { xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; rdi = (((xmm0 >> 64) as u64)) as u64; rsi = ((xmm0 as u64)) as u64; bb = 21; continue; }
            20 => { rsi = 0; fl_l = 0; fl_r = 0; rdi = 0; fl_l = 0; fl_r = 0; bb = 21; continue; }
            21 => { xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; rbx = (crate::simd_ops::ru64(r14)) as u64; rdx = rsp.wrapping_add(0x30); r8 = rdi; rcx = r14; r8 = ((r8).wrapping_sub(rbx)) as u64; r8 = ((r8).wrapping_add(1)) as u64; r8 = ((r8).wrapping_add(rsi)) as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = (crate::simd_ops::ru64(r14.wrapping_add(0x8u64))) as u64; xmm0 = (crate::simd_ops::ru128(r15)) as u128; fl_l = rax; fl_r = rdx; rcx = rdx; rcx = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { rcx }) as u64; rdx = ((rdx).wrapping_sub(rcx)) as u64; crate::simd_ops::wu64(r14.wrapping_add(0x8u64), rdx); rax = ((rbx.wrapping_add(rcx)) as u64) as u64; crate::simd_ops::wu64(r14, rax); fl_l = rdi; fl_r = (crate::simd_ops::ru64(r15.wrapping_add(0x8u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 50; continue; } bb = 22; continue; }
            22 => { rcx = ((xmm0 as u64)) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 27; continue; } bb = 23; continue; }
            23 => { fl_l = rsi; fl_r = rcx; rsi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; bb = 24; continue; }
            24 => { rax = ((crate::simd_ops::ru8(rcx.wrapping_add(rsi))) as u32) as u32 as u64; rdi = ((rdi).wrapping_sub(1)) as u64; rdx = ((crate::simd_ops::ru8(rax.wrapping_add(r12))) as u32) as u32 as u64; rax = ((crate::simd_ops::ru8(rcx)) as u32) as u32 as u64; fl_l = ((rdx as u8)) as u8 as u64; fl_r = (crate::simd_ops::ru8(rax.wrapping_add(r12))) as u8 as u64; if ((((fl_l as u8)) as i8) < (((fl_r as u8)) as i8)) { bb = 50; continue; } bb = 25; continue; }
            25 => { if ((((fl_l as u8)) as i8) > (((fl_r as u8)) as i8)) { bb = 50; continue; } bb = 26; continue; }
            26 => { rcx = ((rcx).wrapping_add(1)) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 24; continue; } bb = 27; continue; }
            27 => { xmm0 = (crate::simd_ops::ru128(r14)) as u128; rbx = (((xmm0 >> 64) as u64)) as u64; r14 = P3; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); r11 = ((xmm0 as u64)) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 37; continue; } bb = 28; continue; }
            28 => { r10 = ((rbx.wrapping_add(r11)) as u64) as u64; fl_l = r11; fl_r = r10; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 37; continue; } bb = 29; continue; }
            29 => { rax = ((crate::simd_ops::ru8(r11)) as u32) as u32 as u64; rcx = r11; bb = 30; continue; }
            30 => { r9 = ((crate::simd_ops::ru8(rax.wrapping_add(r12))) as u32) as u32 as u64; rdx = r14; r8 = 0x2; bb = 31; continue; }
            31 => { rax = ((crate::simd_ops::ru8(rdx)) as u32) as u32 as u64; r8 = ((r8).wrapping_sub(1)) as u64; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(r12))) as u8 as u64; fl_r = ((r9 as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 35; continue; } bb = 32; continue; }
            32 => { rdx = ((rdx).wrapping_add(1)) as u64; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 31; continue; } bb = 33; continue; }
            33 => { rcx = ((rcx).wrapping_sub(r11)) as u64; fl_l = rcx; fl_r = (0xffffffffffffffffu64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 37; continue; } bb = 34; continue; }
            34 => { r11 = ((r11).wrapping_add(rcx)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r11); fl_l = rbx; fl_r = rcx; rbx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; bb = 38; continue; }
            35 => { rcx = ((rcx).wrapping_add(1)) as u64; fl_l = rcx; fl_r = r10; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 37; continue; } bb = 36; continue; }
            36 => { rax = ((crate::simd_ops::ru8(rcx)) as u32) as u32 as u64; bb = 30; continue; }
            37 => { rbx = 0; fl_l = 0; fl_r = 0; bb = 38; continue; }
            38 => { rdx = ((rbx.wrapping_add(r11)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rbx); rcx = rdx; rax = rdx; r8 = rdx; fl_l = rdx; fl_r = r11; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 44; continue; } bb = 39; continue; }
            39 => { r8 = ((crate::simd_ops::ru8(rdx.wrapping_sub(0x1u64))) as u32) as u32 as u64; rdi = rdx.wrapping_sub(0x1); bb = 40; continue; }
            40 => { rsi = ((crate::simd_ops::ru8(r8.wrapping_add(r12))) as u32) as u32 as u64; r9 = r14; r10 = 0x2; bb = 41; continue; }
            41 => { r8 = ((crate::simd_ops::ru8(r9)) as u32) as u32 as u64; r10 = ((r10).wrapping_sub(1)) as u64; fl_l = (crate::simd_ops::ru8(r8.wrapping_add(r12))) as u8 as u64; fl_r = ((rsi as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 47; continue; } bb = 42; continue; }
            42 => { r9 = ((r9).wrapping_add(1)) as u64; fl_l = r10; fl_r = r10; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 41; continue; } bb = 43; continue; }
            43 => { r8 = rax; bb = 44; continue; }
            44 => { fl_l = r8; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 46; continue; } bb = 45; continue; }
            45 => { r8 = ((r8).wrapping_sub(rdx)) as u64; fl_l = ((rbx).wrapping_add(r8)) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rbx); bb = 46; continue; }
            46 => { xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; crate::simd_ops::wu128(rbp, (xmm0) as u128); crate::simd_ops::wu8(rbp.wrapping_add(0x10u64), (0x1u8) as u8); bb = 51; continue; }
            47 => { r9 = ((r9).wrapping_sub(r14)) as u64; fl_l = r9; fl_r = (0xffffffffffffffffu64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 43; continue; } bb = 48; continue; }
            48 => { rcx = ((rcx).wrapping_sub(1)) as u64; rax = rdi; r8 = rcx; rsi = rdi; fl_l = rdi; fl_r = r11; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 44; continue; } bb = 49; continue; }
            49 => { r8 = ((crate::simd_ops::ru8(rsi.wrapping_sub(0x1u64))) as u32) as u32 as u64; fl_l = ((rdi).wrapping_sub(1)) as u64; rdi = ((fl_l as u64)) as u64; bb = 40; continue; }
            50 => { crate::simd_ops::wu8(rbp.wrapping_add(0x10u64), (0x0u8) as u8); bb = 51; continue; }
            51 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rax = rbp; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1424ad740(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1434098b8; const P1: u64 = 0x1426d01e0; const P2: u64 = 0x1401e1a40; const P3: u64 = 0x1403d9450;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0; let mut xmm7: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 21>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x80u64)) as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x70u64), (xmm6) as u128); crate::simd_ops::wu128(rsp.wrapping_add(0x60u64), (xmm7) as u128); r14 = rdx; rsi = rcx; r12 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((r12 as u32)) as u32); xmm0 = 0u128; crate::simd_ops::wu128(rcx, (xmm0) as u128); crate::simd_ops::wu64(rcx, r12); crate::simd_ops::wu64(rcx.wrapping_add(0x8u64), r12); crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x1u32) as u32); r15 = P0; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), r15); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), (0x2u64) as u64); rbp = P1; xmm7 = (crate::simd_ops::ru128(rsp.wrapping_add(0x40u64))) as u128; bb = 1; continue; }
            1 => { xmm0 = (crate::simd_ops::ru128(r14)) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128); rbx = (crate::simd_ops::ru64(r14.wrapping_add(0x8u64))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 21; continue; } bb = 2; continue; }
            2 => { r11 = ((xmm0 as u64)) as u64; r10 = ((r11.wrapping_add(rbx)) as u64) as u64; fl_l = r11; fl_r = r10; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 21; continue; } bb = 3; continue; }
            3 => { rax = ((crate::simd_ops::ru8(r11)) as u32) as u32 as u64; rcx = r11; bb = 4; continue; }
            4 => { r9 = ((crate::simd_ops::ru8(rax.wrapping_add(rbp))) as u32) as u32 as u64; rdx = r15; r8 = 0x2; bb = 5; continue; }
            5 => { r8 = ((r8).wrapping_sub(1)) as u64; rax = ((crate::simd_ops::ru8(rdx)) as u32) as u32 as u64; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(rbp))) as u8 as u64; fl_r = ((r9 as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 17; continue; } bb = 6; continue; }
            6 => { rdx = ((rdx).wrapping_add(1)) as u64; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 7; continue; }
            7 => { rcx = ((rcx).wrapping_sub(r11)) as u64; fl_l = rcx; fl_r = (0xffffffffffffffffu64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 21; continue; } bb = 8; continue; }
            8 => { r11 = ((r11).wrapping_add(rcx)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), r11); fl_l = rbx; fl_r = rcx; rbx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rbx); if (((fl_l as u64)) == ((fl_r as u64))) { bb = 15; continue; } bb = 9; continue; }
            9 => { r10 = ((rbx.wrapping_add(r11)) as u64) as u64; fl_l = r11; fl_r = r10; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 15; continue; } bb = 10; continue; }
            10 => { r8 = r11; bb = 11; continue; }
            11 => { rdx = 0x2; rax = r15; rcx = ((crate::simd_ops::ru8(r8)) as u32) as u32 as u64; r9 = ((crate::simd_ops::ru8(rcx.wrapping_add(rbp))) as u32) as u32 as u64; bb = 12; continue; }
            12 => { rdx = ((rdx).wrapping_sub(1)) as u64; rcx = ((crate::simd_ops::ru8(rax)) as u32) as u32 as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(rbp))) as u8 as u64; fl_r = ((r9 as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 19; continue; } bb = 13; continue; }
            13 => { rax = ((rax).wrapping_add(1)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 12; continue; } bb = 14; continue; }
            14 => { r8 = ((r8).wrapping_add(1)) as u64; fl_l = r8; fl_r = r10; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 11; continue; } bb = 15; continue; }
            15 => { xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; bb = 16; continue; }
            16 => { crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm6) as u128); rdi = (((xmm6 >> 64) as u64)) as u64; rcx = ((xmm6 as u64)) as u64; bb = 22; continue; }
            17 => { rcx = ((rcx).wrapping_add(1)) as u64; fl_l = rcx; fl_r = r10; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 21; continue; } bb = 18; continue; }
            18 => { rax = ((crate::simd_ops::ru8(rcx)) as u32) as u32 as u64; bb = 4; continue; }
            19 => { r8 = ((r8).wrapping_sub(r11)) as u64; fl_l = r8; fl_r = (0xffffffffffffffffu64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 15; continue; } bb = 20; continue; }
            20 => { fl_l = rbx; fl_r = r8; r8 = (if (((fl_l as u64)) < ((fl_r as u64))) { rbx } else { r8 }) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), r11); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), r8); xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x50u64))) as u128; bb = 16; continue; }
            21 => { rcx = r12; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rcx); rdi = r12; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), r12); xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x40u64))) as u128; bb = 22; continue; }
            22 => { rbx = (crate::simd_ops::ru64(r14)) as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm7) as u128); rax = rdi; rax = ((rax).wrapping_sub(rbx)) as u64; r8 = rcx.wrapping_add(0x1); r8 = ((r8).wrapping_add(rax)) as u64; rdx = rsp.wrapping_add(0x30); rcx = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = (crate::simd_ops::ru64(r14.wrapping_add(0x8u64))) as u64; rcx = rdx; fl_l = rax; fl_r = rdx; rcx = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { rcx }) as u64; rax = ((rbx.wrapping_add(rcx)) as u64) as u64; crate::simd_ops::wu64(r14, rax); rdx = ((rdx).wrapping_sub(rcx)) as u64; crate::simd_ops::wu64(r14.wrapping_add(0x8u64), rdx); fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 26; continue; } bb = 23; continue; }
            23 => { rdx = (crate::simd_ops::ru32(rsi.wrapping_add(0x8u64))) as u32 as u64; rcx = (crate::simd_ops::ru32(rsi.wrapping_add(0xcu64))) as u32 as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rax = ((rdx as u32)) as u32 as u64; rax = ((rax).wrapping_shl(4u32)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 25; continue; } bb = 24; continue; }
            24 => { rdx = ((rdx).wrapping_add(1)) as u64; rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 25; continue; }
            25 => { rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x8u64))) as u32 as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(rsi))) as u64; crate::simd_ops::wu128(rax, (xmm6) as u128); fl_l = (crate::simd_ops::lk_add_u32(rsi.wrapping_add(0x8u64), (1) as u32)) as u32 as u64; bb = 1; continue; }
            26 => { rax = rsi; r11 = rsp.wrapping_add(0x80); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x38u64))) as u64; rbp = (crate::simd_ops::ru64(r11.wrapping_add(0x40u64))) as u64; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x70u64))) as u128; xmm7 = (crate::simd_ops::ru128(rsp.wrapping_add(0x60u64))) as u128; rsp = r11; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1424ad970(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1424afb00;
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x30u64)) as u64;
    rcx = ((rcx).wrapping_add(0x1c0u64)) as u64;
    rbx = rdx;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rax = rbx;
    rsp = ((rsp).wrapping_add(0x30u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1424ad9a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14020bcb0; const P1: u64 = 0x142498c90; const P2: u64 = 0x1424af1a0; const P3: u64 = 0x1434107e0; const P4: u64 = 0x1401fa880; const P5: u64 = 0x1424ae070; const P6: u64 = 0x142645878; const P7: u64 = 0x1424adbbc; const P8: u64 = 0x142409b50; const P9: u64 = 0x1401df980; const P10: u64 = 0x142491120; const P11: u64 = 0x14240ab10; const P12: u64 = 0x1424adcc8; const P13: u64 = 0x14240a730; const P14: u64 = 0x1424b5b70; const P15: u64 = 0x1424ae2b0; const P16: u64 = 0x142442de0; const P17: u64 = 0x1424ade6a; const P18: u64 = 0x143410848; const P19: u64 = 0x1423fdfc0; const P20: u64 = 0x1401febb0; const P21: u64 = 0x143079330; const P22: u64 = 0x141461fb0; const P23: u64 = 0x1424ae05f;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 123>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1424ad9a0, funcinfo_rva: 0x37efb0c, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x3d8, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rsi) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r12) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r13) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r15) as u64); rbp = ((rsp.wrapping_sub(0x2b0u64)) as u64) as u64; rsp = ((rsp).wrapping_sub(0x3b0u64)) as u64; r12 = (rdx) as u64; rsi = (rcx) as u64; rbx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; r13 = ((rbx as u32)) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x2e0u64), ((rbx as u32)) as u32); r14 = ((rcx.wrapping_add(0x1b0u64)) as u64) as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0x1a8u64))) as u8 as u64; fl_r = ((rbx as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 9; continue; } bb = 1; continue; }
            1 => { r15 = ((rbx as u32)) as u32 as u64; r13 = (crate::simd_ops::ru64(r14)) as u64; fl_l = (crate::simd_ops::ru32(r14.wrapping_add(0x8u64))) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 8; continue; } bb = 2; continue; }
            2 => { rcx = ((rbx as u32)) as u32 as u64; bb = 3; continue; }
            3 => { r13 = (crate::simd_ops::ru64(r13.wrapping_add(rcx.wrapping_mul(8)))) as u64; rdi = (crate::simd_ops::ru64(r13.wrapping_add(0xb0u64))) as u64; fl_l = (rdi) as u64 as u64; fl_r = (rdi) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 4; continue; }
            4 => { rdi = ((rdi).wrapping_add(0xa0u64)) as u64; rdx = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0xcu64))) as u32 as u64; rcx = ((rcx).wrapping_shl(3u32)) as u64; rax = ((rdx.wrapping_mul(8)) as u64) as u64; fl_l = (rax) as u64 as u64; fl_r = (rcx) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rdx = ((rdx).wrapping_add(1)) as u64; rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1424ada35); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; rax = (crate::simd_ops::ru64(rdi)) as u64; crate::simd_ops::wu64(rax.wrapping_add(rcx.wrapping_mul(8)), (r13) as u64); fl_l = (crate::simd_ops::lk_add_u32(rdi.wrapping_add(0x8u64), (1) as u32)) as u32 as u64; bb = 7; continue; }
            7 => { r15 = (((r15 as u32)).wrapping_add(1)) as u32 as u64; rax = (crate::simd_ops::ru32(r14.wrapping_add(0x8u64))) as u32 as u64; rcx = ((((r15 as u32)) as i32 as i64) as u64) as u64; r13 = (crate::simd_ops::ru64(r14)) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 3; continue; } bb = 8; continue; }
            8 => { crate::simd_ops::wu8(rsi.wrapping_add(0x1a8u64), (0x1u8) as u8); r13 = (crate::simd_ops::ru32(rbp.wrapping_add(0x2e0u64))) as u32 as u64; bb = 9; continue; }
            9 => { xmm0 = (crate::simd_ops::ru128(r12)) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); rcx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1424ada78); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 19; continue; }
            11 => { rdi = ((rbx as u32)) as u32 as u64; fl_l = (crate::simd_ops::ru32(rsi.wrapping_add(0x1b8u64))) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 17; continue; } bb = 12; continue; }
            12 => { rcx = (rbx) as u64; bb = 13; continue; }
            13 => { crate::simd_ops::wu8(rbp.wrapping_add(0x2e0u64), (0x1u8) as u8); r15 = ((rcx.wrapping_mul(8)) as u64) as u64; rax = (crate::simd_ops::ru64(r14)) as u64; rcx = (crate::simd_ops::ru64(r15.wrapping_add(rax))) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0xf0u64))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 83; continue; } bb = 14; continue; }
            14 => { rax = (crate::simd_ops::ru64(rcx)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1424adac9); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 16; continue; } bb = 15; continue; }
            15 => { rax = (crate::simd_ops::ru64(r14)) as u64; xmm0 = (crate::simd_ops::ru128(r12)) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); r8 = ((rbp.wrapping_add(0x2e0u64)) as u64) as u64; rdx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; rcx = (crate::simd_ops::ru64(r15.wrapping_add(rax))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1424adaef); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 20; continue; } bb = 16; continue; }
            16 => { rdi = (((rdi as u32)).wrapping_add(1)) as u32 as u64; rcx = ((((rdi as u32)) as i32 as i64) as u64) as u64; rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x1b8u64))) as u32 as u64; fl_l = (rcx) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 13; continue; } bb = 17; continue; }
            17 => { rax = ((P3) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rax) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (0x1au64) as u64); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); rdx = (r12) as u64; rcx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1424adb31); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = (rax) as u64; rcx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1424adb3c); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 18; continue; }
            18 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 19; continue; }
            19 => { r11 = ((rsp.wrapping_add(0x3b0u64)) as u64) as u64; rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x40u64))) as u64; rdi = (crate::simd_ops::ru64(r11.wrapping_add(0x48u64))) as u64; rsp = (r11) as u64; r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r13 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r12 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            20 => { fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 22; continue; } bb = 21; continue; }
            21 => { rax = ((crate::simd_ops::ru8(rbp.wrapping_add(0x2e0u64))) as u32) as u32 as u64; bb = 19; continue; }
            22 => { rdi = (0x388u32) as u32 as u64; fl_l = (crate::simd_ops::ru8(rsi.wrapping_add(0x258u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 39; continue; } bb = 23; continue; }
            23 => { rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x2b0u64))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 83; continue; } bb = 24; continue; }
            24 => { rax = (crate::simd_ops::ru64(rcx)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1424adb96); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 39; continue; } bb = 25; continue; }
            25 => { fl_l = (crate::simd_ops::ru8(rsi.wrapping_add(0x258u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 29; continue; } bb = 26; continue; }
            26 => { rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x2b0u64))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 28; continue; } bb = 27; continue; }
            27 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1424adbb6); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P6)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::lift_ops::trap(0x80000003, P7, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            28 => { rax = (crate::simd_ops::ru64(rcx)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1424adbc0); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; rax = (rdi) as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 30; continue; } bb = 29; continue; }
            29 => { rax = (0x340u32) as u32 as u64; bb = 30; continue; }
            30 => { rax = ((rax).wrapping_add(rsi)) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x90u64), (0x8000008u64) as u64); crate::simd_ops::wu8(rbp.wrapping_add(0x98u64), (0x0u8) as u8); rcx = ((rbp.wrapping_add(0x90u64)) as u64) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 37; continue; } bb = 31; continue; }
            31 => { r9 = (crate::simd_ops::ru32(rax.wrapping_add(0x4u64))) as u32 as u64; rdx = ((rax.wrapping_add(0x8u64)) as u64) as u64; rcx = ((crate::simd_ops::ru8(rax.wrapping_add(0x3u64))) as u32) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = (0x80u32) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 33; continue; } bb = 32; continue; }
            32 => { rcx = ((rcx.wrapping_mul(8)) as u32) as u32 as u64; bb = 34; continue; }
            33 => { rcx = (((rcx as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = (0x7c00u32) as u32 as u64; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 34; continue; }
            34 => { rax = (crate::simd_ops::ru32(rax)) as u32 as u64; rax = ((((rax as u32)) & (0xffffffu32))) as u32 as u64; rax = (((rax as u32)).wrapping_shl(3u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 36; continue; } bb = 35; continue; }
            35 => { rdx = (crate::simd_ops::ru64(rdx)) as u64; bb = 36; continue; }
            36 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (rdx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (r9) as u64); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); r8 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rdx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; rcx = ((rbp.wrapping_add(0x90u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1424adc4c); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 37; continue; }
            37 => { r13 = (0x7u32) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x2e0u64), ((r13 as u32)) as u32); rdx = ((rbp.wrapping_add(0x90u64)) as u64) as u64; rcx = ((rsp.wrapping_add(0x40u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1424adc6b); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; xmm0 = (crate::simd_ops::ru128(rax)) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); rcx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1424adc7f); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P10) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 39; continue; } bb = 38; continue; }
            38 => { r14 = (r14 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 40; continue; }
            39 => { r14 = (r14 & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 40; continue; }
            40 => { fl_l = ((r13 as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 42; continue; } bb = 41; continue; }
            41 => { rcx = ((rbp.wrapping_add(0x90u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1424adc9d); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P11) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 42; continue; }
            42 => { rax = ((crate::simd_ops::ru8(rsi.wrapping_add(0x258u64))) as u32) as u32 as u64; fl_l = ((r14 as u8)) as u8 as u64; fl_r = ((r14 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 63; continue; } bb = 43; continue; }
            43 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 47; continue; } bb = 44; continue; }
            44 => { rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x2b0u64))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 46; continue; } bb = 45; continue; }
            45 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1424adcc2); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P6)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::lift_ops::trap(0x80000003, P12, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            46 => { rax = (crate::simd_ops::ru64(rcx)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1424adccc); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 48; continue; } bb = 47; continue; }
            47 => { rdi = (0x340u32) as u32 as u64; bb = 48; continue; }
            48 => { rax = ((rdi.wrapping_add(rsi)) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0xd8u64), (0x8000008u64) as u64); crate::simd_ops::wu8(rbp.wrapping_add(0xe0u64), (0x0u8) as u8); rcx = ((rbp.wrapping_add(0xd8u64)) as u64) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 55; continue; } bb = 49; continue; }
            49 => { r9 = (crate::simd_ops::ru32(rax.wrapping_add(0x4u64))) as u32 as u64; rdx = ((rax.wrapping_add(0x8u64)) as u64) as u64; rcx = ((crate::simd_ops::ru8(rax.wrapping_add(0x3u64))) as u32) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = (0x80u32) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 51; continue; } bb = 50; continue; }
            50 => { rcx = ((rcx.wrapping_mul(8)) as u32) as u32 as u64; bb = 52; continue; }
            51 => { rcx = (((rcx as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = (0x7c00u32) as u32 as u64; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 52; continue; }
            52 => { rax = (crate::simd_ops::ru32(rax)) as u32 as u64; rax = ((((rax as u32)) & (0xffffffu32))) as u32 as u64; rax = (((rax as u32)).wrapping_shl(3u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 54; continue; } bb = 53; continue; }
            53 => { rdx = (crate::simd_ops::ru64(rdx)) as u64; bb = 54; continue; }
            54 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (rdx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (r9) as u64); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); r8 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rdx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; rcx = ((rbp.wrapping_add(0xd8u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1424add56); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 55; continue; }
            55 => { crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), (0x10000010u64) as u64); crate::simd_ops::wu8(rsp.wrapping_add(0x58u64), (0x0u8) as u8); rdx = ((rbp.wrapping_add(0xd8u64)) as u64) as u64; rcx = ((rsp.wrapping_add(0x50u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1424add76); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P13) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = ((crate::simd_ops::ru8(rsp.wrapping_add(0x53u64))) as u32) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = (0x80u32) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 57; continue; } bb = 56; continue; }
            56 => { rdx = ((rdx.wrapping_mul(8)) as u32) as u32 as u64; bb = 58; continue; }
            57 => { rdx = (((rdx as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = (0x7c00u32) as u32 as u64; rdx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 58; continue; }
            58 => { rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x50u64))) as u32 as u64; rax = ((((rax as u32)) & (0xffffffu32))) as u32 as u64; rax = (((rax as u32)).wrapping_shl(3u32)) as u32 as u64; rcx = ((rsp.wrapping_add(0x58u64)) as u64) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; rcx = (if (((fl_l as u32)) > ((fl_r as u32))) { crate::simd_ops::ru64(rsp.wrapping_add(0x58u64)) } else { rcx }) as u64; rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x54u64))) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (rcx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (rax) as u64); rdx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; rcx = ((rbp.wrapping_add(0x170u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1424addce); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P14) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = ((rsp.wrapping_add(0x50u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1424addd9); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P11) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = ((rbp.wrapping_add(0xd8u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1424adde6); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P11) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x288u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (rcx) as u64); rax = (0xffffffffffffffffu64) as u64; bb = 59; continue; }
            59 => { rax = ((rax).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(rax))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 59; continue; } bb = 60; continue; }
            60 => { crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (rax) as u64); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); rdx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; rcx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1424ade22); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P15) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = (crate::simd_ops::ru8(rsi.wrapping_add(0x8u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; rbx = (rbx & !0xffu64) | (((if (((fl_l as u8)) == ((fl_r as u8))) { 1u8 } else { 0u8 }) as u8 as u64)); rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x288u64))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 62; continue; } bb = 61; continue; }
            61 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1424ade3a); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P16) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 62; continue; }
            62 => { rcx = ((rbp.wrapping_add(0x178u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1424ade47); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P11) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = (((rbx as u8)) as u32) as u32 as u64; bb = 19; continue; }
            63 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 67; continue; } bb = 64; continue; }
            64 => { rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x2b0u64))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 66; continue; } bb = 65; continue; }
            65 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1424ade64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P6)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::lift_ops::trap(0x80000003, P17, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            66 => { rax = (crate::simd_ops::ru64(rcx)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1424ade6e); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 68; continue; } bb = 67; continue; }
            67 => { rdi = (0x340u32) as u32 as u64; bb = 68; continue; }
            68 => { rax = ((rdi.wrapping_add(rsi)) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x120u64), (0x8000008u64) as u64); crate::simd_ops::wu8(rbp.wrapping_add(0x128u64), (0x0u8) as u8); rcx = ((rbp.wrapping_add(0x120u64)) as u64) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 75; continue; } bb = 69; continue; }
            69 => { r9 = (crate::simd_ops::ru32(rax.wrapping_add(0x4u64))) as u32 as u64; rdx = ((rax.wrapping_add(0x8u64)) as u64) as u64; rcx = ((crate::simd_ops::ru8(rax.wrapping_add(0x3u64))) as u32) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = (0x80u32) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 71; continue; } bb = 70; continue; }
            70 => { rcx = ((rcx.wrapping_mul(8)) as u32) as u32 as u64; bb = 72; continue; }
            71 => { rcx = (((rcx as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = (0x7c00u32) as u32 as u64; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 72; continue; }
            72 => { rax = (crate::simd_ops::ru32(rax)) as u32 as u64; rax = ((((rax as u32)) & (0xffffffu32))) as u32 as u64; rax = (((rax as u32)).wrapping_shl(3u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 74; continue; } bb = 73; continue; }
            73 => { rdx = (crate::simd_ops::ru64(rdx)) as u64; bb = 74; continue; }
            74 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (rdx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (r9) as u64); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); r8 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rdx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; rcx = ((rbp.wrapping_add(0x120u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1424adef8); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 75; continue; }
            75 => { rax = ((P18) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rax) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (0x23u64) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1424adf13); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P19) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdi = (rax) as u64; rax = ((rbp.wrapping_add(0x120u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (rax) as u64); rax = ((P20) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (rax) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), (0xfu64) as u64); rax = ((rsp.wrapping_add(0x30u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), (rax) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), (0x7ffu64) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), (rdi) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), (0x7ffu64) as u64); r14 = ((P21) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), (r14) as u64); crate::simd_ops::wu64(rbp.wrapping_sub(0x80u64), (rdi) as u64); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x40u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm1) as u128); r9 = (rbx) as u64; r8 = ((rsp.wrapping_add(0x40u64)) as u64) as u64; rdx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; rcx = ((rsp.wrapping_add(0x50u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1424adfa1); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P22) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; r8 = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rax = (r8) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rax = ((rax).wrapping_sub(rcx)) as u64; r9 = (rbx) as u64; fl_l = (r8) as u64 as u64; fl_r = (rcx) as u64 as u64; r9 = (if (((fl_l as u64)) > ((fl_r as u64))) { rax } else { r9 }) as u64; rax = ((rcx.wrapping_add(rdx)) as u64) as u64; fl_l = (rdx) as u64 as u64; fl_r = (r9) as u64 as u64; r9 = (if (((fl_l as u64)) < ((fl_r as u64))) { rdx } else { r9 }) as u64; rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x80u64))) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rcx) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 77; continue; } bb = 76; continue; }
            76 => { fl_l = ((rcx).wrapping_add(r9)) as u64 as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x80u64), (rcx) as u64); rdx = ((rbp.wrapping_sub(0x78u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), (rdx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), (0x100u64) as u64); bb = 77; continue; }
            77 => { crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), (r14) as u64); fl_l = (r8) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 79; continue; } bb = 78; continue; }
            78 => { fl_l = (r8) as u64 as u64; fl_r = (rax) as u64 as u64; r8 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; bb = 80; continue; }
            79 => { r8 = (rbx) as u64; bb = 80; continue; }
            80 => { crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), (rax) as u64); fl_l = (r8) as u64 as u64; fl_r = (r8) as u64 as u64; r8 = (if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { rbx } else { r8 }) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rcx) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 82; continue; } bb = 81; continue; }
            81 => { rax = ((rcx.wrapping_add(r8)) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x80u64), (rax) as u64); rax = ((rbp.wrapping_sub(0x78u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), (rax) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), (0x100u64) as u64); bb = 82; continue; }
            82 => { crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), (rbx) as u64); crate::simd_ops::wu8(rcx, (0x0u8) as u8); rdx = (rdi) as u64; rcx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1424ae042); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = ((rbp.wrapping_add(0x120u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1424ae04f); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P11) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 18; continue; }
            83 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1424ae059); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P6)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::lift_ops::trap(0x80000003, P23, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1424ae070(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x14240a590; const P2: u64 = 0xc; const P3: u64 = 0x14240a630; const P4: u64 = 0x9; const P5: u64 = 0x1424bff90;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x30u64)) as u64;
    fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; fl_r = 0x0;
    rbx = rdx;
    rdi = rcx;
    if !((((fl_l as u32)) == ((fl_r as u32)))) { rdx = (rdx & !0xffu64) | (((0xau8) as u8 as u64)); rcx = ((rcx).wrapping_add(0xcu64)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); }
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbx);
    rax = (0xffffffffffffffffu64) as u64;
    loop { rax = ((rax).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(rax))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if !((((fl_l as u8)) != ((fl_r as u8)))) { break; } }
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax);
    rdx = rsp.wrapping_add(0x20);
    xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128;
    rcx = ((rdi.wrapping_add(P2)) as u64) as u64;
    crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128);
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(P4))) as u8 as u64; fl_r = (0x0u8) as u8 as u64;
    if !((((fl_l as u8)) != ((fl_r as u8)))) { rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64;
    rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64;
    rsp = ((rsp).wrapping_add(0x30u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1424ae0f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1a8; const P1: u64 = 0x1b8; const P2: u64 = 0x1b0; const P3: u64 = 0x8; const P4: u64 = 0xc; const P5: u64 = 0x14020bcb0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = rcx; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(P0))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 9; continue; } bb = 1; continue; }
            1 => { rdi = 0; fl_l = 0; fl_r = 0; fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 8; continue; } bb = 2; continue; }
            2 => { rcx = ((rdi as u32)) as u32 as u64; bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rsi.wrapping_add(P2))) as u64; rbp = (crate::simd_ops::ru64(rax.wrapping_add(rcx.wrapping_mul(8)))) as u64; rbx = (crate::simd_ops::ru64(rbp.wrapping_add(0xb0u64))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 4; continue; }
            4 => { rbx = ((rbx).wrapping_add(0xa0u64)) as u64; rdx = (crate::simd_ops::ru32(rbx.wrapping_add(P3))) as u32 as u64; rcx = (crate::simd_ops::ru32(rbx.wrapping_add(P4))) as u32 as u64; rcx = ((rcx).wrapping_shl(3u32)) as u64; rax = ((rdx.wrapping_mul(8)) as u64) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rdx = ((rdx).wrapping_add(1)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru32(rbx.wrapping_add(P3))) as u32 as u64; rax = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu64(rax.wrapping_add(rcx.wrapping_mul(8)), rbp); fl_l = (crate::simd_ops::lk_add_u32(rbx.wrapping_add(P3), (1) as u32)) as u32 as u64; bb = 7; continue; }
            7 => { rdi = (((rdi as u32)).wrapping_add(1)) as u32 as u64; rcx = ((((rdi as u32)) as i32 as i64) as u64) as u64; rax = (crate::simd_ops::ru32(rsi.wrapping_add(P1))) as u32 as u64; fl_l = rcx; fl_r = rax; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 3; continue; } bb = 8; continue; }
            8 => { crate::simd_ops::wu8(rsi.wrapping_add(P0), (0x1u8) as u8); bb = 9; continue; }
            9 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1424ae200(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x1; const P2: u64 = 0x1424ad9a0; const P3: u64 = 0x1401de590;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rbx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; rsi = rcx; rdi = (crate::simd_ops::ru64(rdx)) as u64; rdi = ((rdi).wrapping_shl(4u32)) as u64; rdi = ((rdi).wrapping_add(rbx)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 7; continue; } bb = 1; continue; }
            1 => { bb = 2; continue; }
            2 => { rdx = (crate::simd_ops::ru64(rbx)) as u64; rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64; rax = ((crate::simd_ops::ru8(rdx)) as u32) as u32 as u64; rax = (rax & !0xffu64) | (((((rax as u8)).wrapping_sub(0x2du8)) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = (0xfdu8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 6; continue; } bb = 3; continue; }
            3 => { fl_l = rcx; fl_r = (0x1u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 8; continue; } bb = 4; continue; }
            4 => { rax = ((rdx.wrapping_add(P1)) as u64) as u64; rcx = ((rcx).wrapping_sub(1)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); rdx = rsp.wrapping_add(0x30); rax = (0xffffffffffffffffu64) as u64; fl_l = rcx; fl_r = rax; rax = (if (((fl_l as u64)) < ((fl_r as u64))) { rcx } else { rax }) as u64; rcx = rsi; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { crate::simd_ops::wu8(rsi.wrapping_add(P0), (0x1u8) as u8); bb = 6; continue; }
            6 => { rbx = ((rbx).wrapping_add(0x10u64)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 7; continue; }
            7 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            8 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, 0x1424ae2a4, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1424ae2b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141378dc0; const P1: u64 = 0x1426d01e0; const P2: u64 = 0x1433d377c; const P3: u64 = 0x140215e80; const P4: u64 = 0x14183acc8; const P5: u64 = 0x1425371a0; const P6: u64 = 0x142533200; const P7: u64 = 0x1424ae200;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 19>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1424ae2b0, funcinfo_rva: 0x37efc34, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x98, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rdx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rcx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r12) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r13) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r15) as u64); rbp = (rsp) as u64; rsp = ((rsp).wrapping_sub(0x60u64)) as u64; rsi = (rdx) as u64; xmm0 = 0u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x40u64), (xmm0) as u128); rbx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; r15 = ((rbx as u32)) as u32 as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x40u64), (rbx) as u64); r13 = ((rbx as u32)) as u32 as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x38u64), (rbx) as u64); r12 = ((rbx as u32)) as u32 as u64; rcx = ((rbp.wrapping_sub(0x30u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1424ae2f4); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = (crate::simd_ops::ru64(rbp.wrapping_sub(0x28u64))) as u64; fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 35; continue; } bb = 1; continue; }
            1 => { r14 = ((P1) as u64) as u64; xmm0 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x30u64))) as u128; bb = 2; continue; }
            2 => { rdi = ((P2) as u64) as u64; crate::simd_ops::wu128(rbp.wrapping_sub(0x30u64), (xmm0) as u128); r10 = ((xmm0 as u64)) as u64; fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 3; continue; }
            3 => { r11 = ((r10.wrapping_add(rax)) as u64) as u64; fl_l = (r10) as u64 as u64; fl_r = (r11) as u64 as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 10; continue; } bb = 4; continue; }
            4 => { rax = ((crate::simd_ops::ru8(r10)) as u32) as u32 as u64; r9 = ((crate::simd_ops::ru8(rax.wrapping_add(r14))) as u32) as u32 as u64; rax = (r10) as u64; bb = 5; continue; }
            5 => { rdx = (rdi) as u64; r8 = (0x4u32) as u32 as u64; bb = 6; continue; }
            6 => { r8 = ((r8).wrapping_sub(1)) as u64; rcx = ((crate::simd_ops::ru8(rdx)) as u32) as u32 as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(r14))) as u8 as u64; fl_r = ((r9 as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 24; continue; } bb = 7; continue; }
            7 => { rdx = ((rdx).wrapping_add(1)) as u64; fl_l = (r8) as u64 as u64; fl_r = (r8) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 6; continue; } bb = 8; continue; }
            8 => { rax = ((rax).wrapping_sub(r10)) as u64; fl_l = (rax) as u64 as u64; fl_r = (0xffffffffffffffffu64) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 10; continue; } bb = 9; continue; }
            9 => { r10 = ((r10).wrapping_add(rax)) as u64; rbx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x28u64))) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rax) as u64 as u64; rbx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; bb = 10; continue; }
            10 => { crate::simd_ops::wu64(rbp.wrapping_sub(0x28u64), (rbx) as u64); crate::simd_ops::wu64(rbp.wrapping_sub(0x30u64), (r10) as u64); rdi = ((rbx.wrapping_add(r10)) as u64) as u64; rcx = (rdi) as u64; rax = (rdi) as u64; fl_l = (rdi) as u64 as u64; fl_r = (r10) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 18; continue; } bb = 11; continue; }
            11 => { r11 = ((rdi.wrapping_sub(0x1u64)) as u64) as u64; r8 = ((P2) as u64) as u64; rdx = ((crate::simd_ops::ru8(rdi.wrapping_sub(0x1u64))) as u32) as u32 as u64; rsi = ((crate::simd_ops::ru8(rdx.wrapping_add(r14))) as u32) as u32 as u64; bb = 12; continue; }
            12 => { r14 = (r11) as u64; r9 = (0x4u32) as u32 as u64; bb = 13; continue; }
            13 => { r9 = ((r9).wrapping_sub(1)) as u64; rdx = ((crate::simd_ops::ru8(r8)) as u32) as u32 as u64; r15 = ((P1) as u64) as u64; fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(r15))) as u8 as u64; fl_r = ((rsi as u8)) as u8 as u64; r15 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x40u64))) as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 26; continue; } bb = 14; continue; }
            14 => { r8 = ((r8).wrapping_add(1)) as u64; fl_l = (r9) as u64 as u64; fl_r = (r9) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 13; continue; } bb = 15; continue; }
            15 => { rcx = (rax) as u64; bb = 16; continue; }
            16 => { rsi = (crate::simd_ops::ru64(rbp.wrapping_add(0x48u64))) as u64; r14 = ((P1) as u64) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rdi) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 18; continue; } bb = 17; continue; }
            17 => { rdi = ((rdi).wrapping_sub(rcx)) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rdi) as u64 as u64; rbx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x28u64), (rbx) as u64); bb = 18; continue; }
            18 => { rbx = ((r13 as u32)) as u32 as u64; rbx = ((rbx).wrapping_shl(4u32)) as u64; rbx = ((rbx).wrapping_add(r15)) as u64; rcx = ((r12 as u32)) as u32 as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rcx = ((rcx).wrapping_add(r15)) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rcx) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 33; continue; } bb = 19; continue; }
            19 => { rbx = ((rbx).wrapping_sub(r15)) as u64; r13 = (rbx) as u64; r13 = ((((r13) as i64).wrapping_shr(4u32) as u64)) as u64; rax = ((r13.wrapping_add(0x1u64)) as u64) as u64; rcx = ((rcx).wrapping_sub(r15)) as u64; rcx = ((((rcx) as i64).wrapping_shr(4u32) as u64)) as u64; r12 = (rcx) as u64; r12 = ((r12).wrapping_shr(1u32)) as u64; r12 = ((r12).wrapping_add(rcx)) as u64; fl_l = (r12) as u64 as u64; fl_r = (rax) as u64 as u64; r12 = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { r12 }) as u64; rdx = (r12) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1424ae436); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdi = (rax) as u64; r8 = (rbx) as u64; rdx = (r15) as u64; rcx = (rax) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1424ae447); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = (r15) as u64 as u64; fl_r = (r15) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 32; continue; } bb = 20; continue; }
            20 => { r10 = ((r15.wrapping_sub(0x1u64)) as u64) as u64; r10 = (((r10) & (0xfffffffffe000000u64))) as u64; rcx = ((crate::exe::teb_self() as u64)) as u64; fl_l = (rcx) as u64 as u64; fl_r = (crate::simd_ops::ru64(r10.wrapping_add(0x100u64))) as u64 as u64; rdx = (rdx & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); r9 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x40u64))) as u64; rax = (r9) as u64; rax = ((rax).wrapping_sub(r10)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rcx = ((rcx).wrapping_add(0x108u64)) as u64; rcx = ((rcx).wrapping_add(r10)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x4u64))) as u32 as u64; rcx = ((rcx).wrapping_sub(rax)) as u64; fl_l = ((rdx as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 30; continue; } bb = 21; continue; }
            21 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0xeu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 29; continue; } bb = 22; continue; }
            22 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(r9, (rax) as u64); crate::simd_ops::wu64(rcx.wrapping_add(0x20u64), (r9) as u64); fl_l = (crate::simd_ops::lk_add_u32(rcx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 32; continue; } bb = 23; continue; }
            23 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1424ae4b4); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 32; continue; }
            24 => { rax = ((rax).wrapping_add(1)) as u64; fl_l = (rax) as u64 as u64; fl_r = (r11) as u64 as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 10; continue; } bb = 25; continue; }
            25 => { rcx = ((crate::simd_ops::ru8(rax)) as u32) as u32 as u64; r9 = ((crate::simd_ops::ru8(rcx.wrapping_add(r14))) as u32) as u32 as u64; bb = 5; continue; }
            26 => { rsi = ((P2) as u64) as u64; r8 = ((r8).wrapping_sub(rsi)) as u64; fl_l = (r8) as u64 as u64; fl_r = (0xffffffffffffffffu64) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 15; continue; } bb = 27; continue; }
            27 => { rax = (r11) as u64; rcx = ((rcx).wrapping_sub(1)) as u64; rdx = (r11) as u64; fl_l = (r14) as u64 as u64; fl_r = (r10) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 16; continue; } bb = 28; continue; }
            28 => { fl_l = ((r11).wrapping_sub(1)) as u64 as u64; r11 = ((fl_l as u64)) as u64; r8 = (rsi) as u64; rdx = ((crate::simd_ops::ru8(rdx.wrapping_sub(0x1u64))) as u32) as u32 as u64; rsi = ((P1) as u64) as u64; rsi = ((crate::simd_ops::ru8(rdx.wrapping_add(rsi))) as u32) as u32 as u64; bb = 12; continue; }
            29 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 31; continue; }
            30 => { r8 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 31; continue; }
            31 => { rdx = (rcx) as u64; rcx = (r10) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1424ae522); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 32; continue; }
            32 => { r15 = (rdi) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x40u64), (rdi) as u64); crate::simd_ops::wu32(rbp.wrapping_sub(0x34u64), ((r12 as u32)) as u32); bb = 33; continue; }
            33 => { rax = ((r13 as u32)) as u32 as u64; rax = ((rax).wrapping_add(rax)) as u64; xmm0 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x30u64))) as u128; crate::simd_ops::wu128(r15.wrapping_add(rax.wrapping_mul(8)), (xmm0) as u128); rbx = ((r13.wrapping_add(0x1u64)) as u32) as u32 as u64; r13 = ((rbx as u32)) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_sub(0x38u64), ((rbx as u32)) as u32); rdx = (rsi) as u64; rcx = ((rbp.wrapping_sub(0x10u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1424ae554); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; xmm0 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x10u64))) as u128; rax = (((xmm0 >> 64) as u64)) as u64; fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 35; continue; } bb = 34; continue; }
            34 => { rax = (((xmm0 >> 64) as u64)) as u64; rbx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 2; continue; }
            35 => { rax = ((rbx as u32)) as u32 as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rax = ((((rax) as i64).wrapping_shr(4u32) as u64)) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x20u64), (rax) as u64); crate::simd_ops::wu64(rbp.wrapping_sub(0x18u64), (r15) as u64); xmm0 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x20u64))) as u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x10u64), (xmm0) as u128); rdx = ((rbp.wrapping_sub(0x10u64)) as u64) as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x40u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1424ae59a); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = (r15) as u64 as u64; fl_r = (r15) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 43; continue; } bb = 36; continue; }
            36 => { r10 = ((r15.wrapping_sub(0x1u64)) as u64) as u64; r10 = (((r10) & (0xfffffffffe000000u64))) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = (rax) as u64 as u64; fl_r = (crate::simd_ops::ru64(r10.wrapping_add(0x100u64))) as u64 as u64; rdx = (rdx & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); r9 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x40u64))) as u64; rax = (r9) as u64; rax = ((rax).wrapping_sub(r10)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rcx = ((rcx).wrapping_add(0x108u64)) as u64; rcx = ((rcx).wrapping_add(r10)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x4u64))) as u32 as u64; rcx = ((rcx).wrapping_sub(rax)) as u64; fl_l = ((rdx as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 41; continue; } bb = 37; continue; }
            37 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0xeu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 40; continue; } bb = 38; continue; }
            38 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(r9, (rax) as u64); crate::simd_ops::wu64(rcx.wrapping_add(0x20u64), (r9) as u64); fl_l = (crate::simd_ops::lk_add_u32(rcx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 43; continue; } bb = 39; continue; }
            39 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1424ae604); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 43; continue; }
            40 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 42; continue; }
            41 => { r8 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 42; continue; }
            42 => { rdx = (rcx) as u64; rcx = (r10) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1424ae619); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 43; continue; }
            43 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0xb0u64))) as u64; rsp = ((rsp).wrapping_add(0x60u64)) as u64; r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r13 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r12 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rsi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1424ae640(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143410978; const P1: u64 = 0x1426cdcb0; const P2: u64 = 0x143410870; const P3: u64 = 0x143410800; const P4: u64 = 0x143410838; const P5: u64 = 0x143079708; const P6: u64 = 0x1424ad430; const P7: u64 = 0x1424af340; const P8: u64 = 0x143410888; const P9: u64 = 0x14240a630; const P10: u64 = 0x142408f50; const P11: u64 = 0x14240ab10;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 71>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rbp = rsp.wrapping_sub(0x120); rsp = ((rsp).wrapping_sub(0x220u64)) as u64; rsi = rcx; rax = P0; crate::simd_ops::wu64(rcx, rax); crate::simd_ops::wu16(rcx.wrapping_add(0x8u64), (0x0u16) as u16); crate::simd_ops::lk_and_u32(rcx.wrapping_add(0xcu64), (0xff000010u32) as u32); crate::simd_ops::lk_or_u32(rcx.wrapping_add(0xcu64), (0x10u32) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0xfu64), (0x10u8) as u8); r14 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rcx.wrapping_add(0x10u64), ((r14 as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x14u64), ((r14 as u8)) as u8); crate::simd_ops::wu32(rcx.wrapping_add(0x94u64), ((r14 as u32)) as u32); crate::simd_ops::wu64(rcx.wrapping_add(0x98u64), r14); crate::simd_ops::lk_and_u32(rcx.wrapping_add(0xa0u64), (0xff000020u32) as u32); crate::simd_ops::lk_or_u32(rcx.wrapping_add(0xa0u64), (0x20u32) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0xa3u64), (0x20u8) as u8); crate::simd_ops::wu32(rcx.wrapping_add(0xa4u64), ((r14 as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0xa8u64), ((r14 as u8)) as u8); crate::simd_ops::wu8(rcx.wrapping_add(0x1a8u64), ((r14 as u8)) as u8); xmm0 = 0u128; crate::simd_ops::wu128(rcx.wrapping_add(0x1b0u64), (xmm0) as u128); crate::simd_ops::wu64(rcx.wrapping_add(0x1b0u64), r14); crate::simd_ops::wu64(rcx.wrapping_add(0x1b8u64), r14); rax = P1; crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), r14); rax = P2; crate::simd_ops::wu64(rbp.wrapping_sub(0x80u64), rax); rdi = 0x8; crate::simd_ops::wu64(rbp.wrapping_sub(0x78u64), rdi); rax = P3; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), (0x36u64) as u64); rax = P4; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), (0xau64) as u64); crate::simd_ops::wu64(rbp.wrapping_add(0x118u64), r14); rax = P5; crate::simd_ops::wu64(rbp.wrapping_sub(0x70u64), rax); rax = P6; crate::simd_ops::wu64(rbp.wrapping_sub(0x68u64), rax); rax = rbp.wrapping_sub(0x70); crate::simd_ops::wu64(rbp.wrapping_sub(0x38u64), rax); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x70u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x70u64), (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x80u64))) as u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x80u64), (xmm1) as u128); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x60u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x60u64), (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(rsp.wrapping_add(0x50u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x50u64), (xmm1) as u128); rax = rbp.wrapping_add(0xe0); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rax); rax = rsp.wrapping_add(0x70); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rax); rax = rbp.wrapping_sub(0x80); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); rax = rsp.wrapping_add(0x60); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r14); r9 = rsp.wrapping_add(0x50); r8 = rbp.wrapping_sub(0x70); rdx = rcx; rcx = ((rcx).wrapping_add(0x1c0u64)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = P8; crate::simd_ops::wu64(rsi.wrapping_add(0x1c0u64), rax); crate::simd_ops::wu64(rbp.wrapping_sub(0x30u64), (0x20000020u64) as u64); crate::simd_ops::wu8(rbp.wrapping_sub(0x28u64), ((r14 as u8)) as u8); rbx = 0x1; fl_l = (crate::simd_ops::ru32(rsi.wrapping_add(0x94u64))) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) <= (((fl_r as u32)) as i32)) { bb = 10; continue; } bb = 1; continue; }
            1 => { bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru64(rdi)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rcx); rax = (0xffffffffffffffffu64) as u64; bb = 3; continue; }
            3 => { rax = ((rax).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(rax))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 3; continue; } bb = 4; continue; }
            4 => { crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rax); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x50u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x60u64), (xmm0) as u128); rdx = rsp.wrapping_add(0x60); rcx = rbp.wrapping_sub(0x30); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r9 = (crate::simd_ops::ru32(rbp.wrapping_sub(0x2cu64))) as u32 as u64; rdx = r9.wrapping_add(0x1); r8 = (crate::simd_ops::ru32(rbp.wrapping_sub(0x30u64))) as u32 as u64; rax = ((r8 as u32)) as u32 as u64; rax = ((((rax as u32)) & (0xffffffu32))) as u32 as u64; rax = (((rax as u32)).wrapping_shl(3u32)) as u32 as u64; fl_l = rdx; fl_r = rax; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rcx = rbp.wrapping_sub(0x30); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P10, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r9 = (crate::simd_ops::ru32(rbp.wrapping_sub(0x2cu64))) as u32 as u64; r8 = (crate::simd_ops::ru32(rbp.wrapping_sub(0x30u64))) as u32 as u64; bb = 6; continue; }
            6 => { rax = ((r8 as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shr(24u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 8; continue; } bb = 7; continue; }
            7 => { rax = ((rax.wrapping_mul(8)) as u32) as u32 as u64; bb = 9; continue; }
            8 => { rax = (((rax as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x7c00; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 9; continue; }
            9 => { r8 = ((((r8 as u32)) & (0xffffffu32))) as u32 as u64; r8 = (((r8 as u32)).wrapping_shl(3u32)) as u32 as u64; rcx = rbp.wrapping_sub(0x28); fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; rcx = (if (((fl_l as u32)) > ((fl_r as u32))) { crate::simd_ops::ru64(rbp.wrapping_sub(0x28u64)) } else { rcx }) as u64; rax = ((r9 as u32)) as u32 as u64; crate::simd_ops::wu8(rax.wrapping_add(rcx), (0x20u8) as u8); rax = (crate::simd_ops::ru32(rbp.wrapping_sub(0x2cu64))) as u32 as u64; crate::simd_ops::wu8(rax.wrapping_add(rcx).wrapping_add(0x1u64), (0x0u8) as u8); crate::simd_ops::lk_add_u32(rbp.wrapping_sub(0x2cu64), (1) as u32); rbx = (((rbx as u32)).wrapping_add(1)) as u32 as u64; rdi = ((rdi).wrapping_add(0x8u64)) as u64; fl_l = ((rbx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rsi.wrapping_add(0x94u64))) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 2; continue; } bb = 10; continue; }
            10 => { rcx = rbp.wrapping_sub(0x30); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P11, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = rsi; r11 = rsp.wrapping_add(0x220); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x38u64))) as u64; rsp = r11; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1424ae8f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1424ad740; const P1: u64 = 0x1426d01e0; const P2: u64 = 0x1401de830; const P3: u64 = 0x142409b50; const P4: u64 = 0x1434106f8; const P5: u64 = 0x1411986e0; const P6: u64 = 0x1431edc44; const P7: u64 = 0x141341dd0;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 19>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1424ae8f0, funcinfo_rva: 0x37efcd4, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x98, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rdx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r12) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r13) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r15) as u64); rbp = (rsp) as u64; rsp = ((rsp).wrapping_sub(0x60u64)) as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x50u64), (xmm6) as u128); rbx = (rdx) as u64; r15 = (rcx) as u64; xmm6 = (crate::simd_ops::ru128(rdx)) as u128; r9 = ((rcx.wrapping_add(0x140u64)) as u64) as u64; r8 = ((crate::simd_ops::ru8(rcx.wrapping_add(0x13bu64))) as u32) as u32 as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = (0x80u32) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { r8 = ((r8.wrapping_mul(8)) as u32) as u32 as u64; bb = 3; continue; }
            2 => { r8 = (((r8 as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = (0x7c00u32) as u32 as u64; r8 = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x138u64))) as u32 as u64; rax = ((((rax as u32)) & (0xffffffu32))) as u32 as u64; rax = (((rax as u32)).wrapping_shl(3u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 5; continue; } bb = 4; continue; }
            4 => { r9 = (crate::simd_ops::ru64(r9)) as u64; bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x13cu64))) as u32 as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x20u64), (r9) as u64); crate::simd_ops::wu64(rbp.wrapping_sub(0x18u64), (rax) as u64); rdx = ((rbp.wrapping_sub(0x20u64)) as u64) as u64; rcx = ((rbp.wrapping_sub(0x30u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1424ae975); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu128(rbp.wrapping_sub(0x20u64), (xmm6) as u128); rdx = ((rbp.wrapping_sub(0x20u64)) as u64) as u64; rcx = ((rbp.wrapping_sub(0x40u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1424ae988); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x40u64))) as u64; r12 = (crate::simd_ops::ru32(rbp.wrapping_sub(0x38u64))) as u32 as u64; r12 = ((r12).wrapping_shl(4u32)) as u64; r12 = ((r12).wrapping_add(rcx)) as u64; fl_l = (rcx) as u64 as u64; fl_r = (r12) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 17; continue; } bb = 6; continue; }
            6 => { rdi = (crate::simd_ops::ru32(rbp.wrapping_sub(0x28u64))) as u32 as u64; rdi = ((rdi).wrapping_shl(4u32)) as u64; r13 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x30u64))) as u64; fl_l = ((rdi).wrapping_add(r13)) as u64 as u64; fl_r = fl_l; rdi = ((fl_l as u64)) as u64; r11 = ((P1) as u64) as u64; bb = 7; continue; }
            7 => { rax = (r13) as u64; fl_l = (r13) as u64 as u64; fl_r = (rdi) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 20; continue; } bb = 8; continue; }
            8 => { xmm0 = (crate::simd_ops::ru128(rcx)) as u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x20u64), (xmm0) as u128); rsi = (crate::simd_ops::ru64(rbp.wrapping_sub(0x18u64))) as u64; r14 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x20u64))) as u64; bb = 9; continue; }
            9 => { rbx = (crate::simd_ops::ru64(rax)) as u64; r9 = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; fl_l = (r9) as u64 as u64; fl_r = (rsi) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 19; continue; } bb = 10; continue; }
            10 => { rdx = (r14) as u64; fl_l = (r9) as u64 as u64; fl_r = (r9) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 15; continue; } bb = 11; continue; }
            11 => { fl_l = (rbx) as u64 as u64; fl_r = (rdx) as u64 as u64; rbx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; bb = 12; continue; }
            12 => { r9 = ((r9).wrapping_sub(1)) as u64; r8 = ((crate::simd_ops::ru8(rbx.wrapping_add(rdx))) as u32) as u32 as u64; r10 = ((crate::simd_ops::ru8(r8.wrapping_add(r11))) as u32) as u32 as u64; r8 = ((crate::simd_ops::ru8(rdx)) as u32) as u32 as u64; fl_l = ((r10 as u8)) as u8 as u64; fl_r = (crate::simd_ops::ru8(r8.wrapping_add(r11))) as u8 as u64; r11 = ((P1) as u64) as u64; if ((((fl_l as u8)) as i8) < (((fl_r as u8)) as i8)) { bb = 19; continue; } bb = 13; continue; }
            13 => { if ((((fl_l as u8)) as i8) > (((fl_r as u8)) as i8)) { bb = 19; continue; } bb = 14; continue; }
            14 => { rdx = ((rdx).wrapping_add(1)) as u64; fl_l = (r9) as u64 as u64; fl_r = (r9) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 12; continue; } bb = 15; continue; }
            15 => { rcx = ((rcx).wrapping_add(0x10u64)) as u64; fl_l = (rcx) as u64 as u64; fl_r = (r12) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 7; continue; } bb = 16; continue; }
            16 => { rbx = (crate::simd_ops::ru64(rbp.wrapping_add(0x48u64))) as u64; bb = 17; continue; }
            17 => { rcx = ((rbp.wrapping_sub(0x40u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1424aea39); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = ((rbp.wrapping_sub(0x30u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1424aea43); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; xmm0 = (crate::simd_ops::ru128(rbx)) as u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x20u64), (xmm0) as u128); rcx = ((r15.wrapping_add(0x180u64)) as u64) as u64; r9 = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; r8 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rdx = ((rbp.wrapping_sub(0x20u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1424aea64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 18; continue; }
            18 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0xb0u64))) as u64; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x50u64))) as u128; rsp = ((rsp).wrapping_add(0x60u64)) as u64; r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r13 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r12 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rsi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            19 => { rax = ((rax).wrapping_add(0x10u64)) as u64; fl_l = (rax) as u64 as u64; fl_r = (rdi) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 9; continue; } bb = 20; continue; }
            20 => { rcx = ((rbp.wrapping_sub(0x40u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1424aea98); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = ((rbp.wrapping_sub(0x30u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1424aeaa2); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = ((r15.wrapping_add(0x10u64)) as u64) as u64; rcx = ((crate::simd_ops::ru8(r15.wrapping_add(0xbu64))) as u32) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = (0x80u32) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 22; continue; } bb = 21; continue; }
            21 => { rcx = ((rcx.wrapping_mul(8)) as u32) as u32 as u64; bb = 23; continue; }
            22 => { rcx = (((rcx as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = (0x7c00u32) as u32 as u64; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 23; continue; }
            23 => { rax = (crate::simd_ops::ru32(r15.wrapping_add(0x8u64))) as u32 as u64; rax = ((((rax as u32)) & (0xffffffu32))) as u32 as u64; rax = (((rax as u32)).wrapping_shl(3u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 25; continue; } bb = 24; continue; }
            24 => { rdx = (crate::simd_ops::ru64(rdx)) as u64; bb = 25; continue; }
            25 => { crate::simd_ops::wu64(rbp.wrapping_add(0x40u64), (rdx) as u64); rax = ((P4) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x20u64), (rax) as u64); crate::simd_ops::wu64(rbp.wrapping_sub(0x18u64), (0x23u64) as u64); xmm0 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x20u64))) as u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x20u64), (xmm0) as u128); r8 = (crate::simd_ops::ru64(rbp.wrapping_add(0x48u64))) as u64; rdx = ((rbp.wrapping_add(0x40u64)) as u64) as u64; rcx = ((rbp.wrapping_sub(0x20u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1424aeb0b); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = (rax) as u64; rcx = ((P6) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1424aeb1a); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 18; continue; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1424aeb30(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1424ad240; const P1: u64 = 0x143410948; const P2: u64 = 0x142409b50;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 23>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r11 = rsp; crate::simd_ops::wu64(r11.wrapping_add(0x18u64), r8); crate::simd_ops::wu64(r11.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x90u64)) as u64; rsi = r9; rdi = r8; rbp = rdx; rbx = rcx; rax = r11.wrapping_sub(0x68); crate::simd_ops::wu64(r11.wrapping_add(0x20u64), rax); r14 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(r11.wrapping_sub(0x30u64), r14); rcx = (crate::simd_ops::ru64(r8.wrapping_add(0x38u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = r11.wrapping_sub(0x68); (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rsp.wrapping_add(0x88u64), rax); bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xe8u64))) as u64; xmm0 = (crate::simd_ops::ru128(rax)) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(rsi)) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm1) as u128); rax = rsp.wrapping_add(0x30); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xe0u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); r9 = rsp.wrapping_add(0x40); r8 = rsp.wrapping_add(0x50); rdx = rbp; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = P1; crate::simd_ops::wu64(rbx, rax); rbp = rbx.wrapping_add(0xf8); crate::simd_ops::wu64(rsp.wrapping_add(0xe8u64), rbp); crate::simd_ops::wu64(rbp.wrapping_add(0x38u64), r14); rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x100u64))) as u64; rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x38u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = rbp; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rbp.wrapping_add(0x38u64), rax); bb = 4; continue; }
            4 => { rcx = rbx.wrapping_add(0x138); crate::simd_ops::wu64(rsp.wrapping_add(0xe8u64), rcx); crate::simd_ops::lk_and_u32(rcx, (0xff000008u32) as u32); crate::simd_ops::lk_or_u32(rcx, (0x8u32) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x3u64), (0x8u8) as u8); crate::simd_ops::wu32(rcx.wrapping_add(0x4u64), ((r14 as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x8u64), (0x0u8) as u8); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xf0u64))) as u64; xmm0 = (crate::simd_ops::ru128(rax)) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm0) as u128); r9 = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; r8 = 0; fl_l = 0; fl_r = 0; rdx = rsp.wrapping_add(0x40); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rbx.wrapping_add(0x180); crate::simd_ops::wu64(rsp.wrapping_add(0xe8u64), rcx); crate::simd_ops::lk_and_u32(rcx, (0xff000008u32) as u32); crate::simd_ops::lk_or_u32(rcx, (0x8u32) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x3u64), (0x8u8) as u8); crate::simd_ops::wu32(rcx.wrapping_add(0x4u64), ((r14 as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x8u64), (0x0u8) as u8); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xf8u64))) as u64; xmm0 = (crate::simd_ops::ru128(rax)) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm0) as u128); r9 = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; r8 = 0; fl_l = 0; fl_r = 0; rdx = rsp.wrapping_add(0x40); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x38u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rcx)) as u64; fl_l = rcx; fl_r = rdi; rdx = (rdx & !0xffu64) | (((if (((fl_l as u64)) != ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x20u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rdi.wrapping_add(0x38u64), r14); bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x38u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru64(rcx)) as u64; fl_l = rcx; fl_r = rsi; rdx = (rdx & !0xffu64) | (((if (((fl_l as u64)) != ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x20u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rsi.wrapping_add(0x38u64), r14); bb = 8; continue; }
            8 => { rax = rbx; rsp = ((rsp).wrapping_add(0x90u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1424aece0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xb; const P1: u64 = 0x8; const P2: u64 = 0x1434107d4; const P3: u64 = 0x1411986e0; const P4: u64 = 0x1424af1a0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rbx = rcx; rcx = ((rcx).wrapping_add(0x10u64)) as u64; r8 = ((crate::simd_ops::ru8(rbx.wrapping_add(P0))) as u32) as u32 as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { r8 = ((r8.wrapping_mul(8)) as u32) as u32 as u64; bb = 3; continue; }
            2 => { r8 = (((r8 as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = 0x7c00; r8 = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(P1))) as u32 as u64; rax = ((((rax as u32)) & (0xffffffu32))) as u32 as u64; rax = (((rax as u32)).wrapping_shl(3u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru64(rcx)) as u64; bb = 5; continue; }
            5 => { crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (0x5u64) as u64); rax = P2; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); r8 = rdx; xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rcx); rdx = rsp.wrapping_add(0x40); rcx = rsp.wrapping_add(0x20); crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); rcx = (0xffffffffffffffffu64) as u64; bb = 6; continue; }
            6 => { rcx = ((rcx).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(rcx))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 6; continue; } bb = 7; continue; }
            7 => { crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rcx); r8 = rsp.wrapping_add(0x40); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; rdx = rsp.wrapping_add(0x20); rcx = rbx; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rsp = ((rsp).wrapping_add(0x30u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1424aeda0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143079708; const P1: u64 = 0x1424ad430; const P2: u64 = 0x1424af340; const P3: u64 = 0x143410888;
    let (mut rax, mut r11, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 35>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r11 = rsp; rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x110u64)) as u64; rax = P0; crate::simd_ops::wu64(r11.wrapping_sub(0x10u64), (0x0u64) as u64); crate::simd_ops::wu64(r11.wrapping_sub(0x88u64), rax); rbx = rcx; rax = P1; crate::simd_ops::wu64(r11.wrapping_sub(0x80u64), rax); rax = r11.wrapping_sub(0x88); crate::simd_ops::wu64(r11.wrapping_sub(0x50u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x150u64))) as u64; xmm0 = (crate::simd_ops::ru128(rax)) as u128; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x148u64))) as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x50u64), (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(rax)) as u128; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x140u64))) as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x60u64), (xmm1) as u128); xmm1 = (crate::simd_ops::ru128(r8)) as u128; r8 = r11.wrapping_sub(0x88); xmm0 = (crate::simd_ops::ru128(rax)) as u128; rax = r11.wrapping_sub(0x48); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rax); rax = rsp.wrapping_add(0x50); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rax); rax = rsp.wrapping_add(0x60); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); rax = rsp.wrapping_add(0x70); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r9); r9 = r11.wrapping_sub(0x98); crate::simd_ops::wu128(rsp.wrapping_add(0x70u64), (xmm0) as u128); crate::simd_ops::wu128(rsp.wrapping_add(0x80u64), (xmm1) as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = P3; crate::simd_ops::wu64(rbx, rax); rax = rbx; rsp = ((rsp).wrapping_add(0x110u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1424aee80(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x164; const P1: u64 = 0x168; const P2: u64 = 0x160;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = ((rdx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64;
    if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { rax = (crate::simd_ops::ru32(rcx.wrapping_add(P2))) as u32 as u64; return rax; }
    fl_l = ((rdx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64;
    if !(((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32))) { rax = (crate::simd_ops::ru32(rcx.wrapping_add(P2))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(P2), ((rdx as u32)) as u32); return rax; }
    rax = (crate::simd_ops::ru32(rcx.wrapping_add(P2))) as u32 as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1424aeeb0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1424ad240; const P1: u64 = 0x143410918; const P2: u64 = 0x142409b50;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 23>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r11 = rsp; crate::simd_ops::wu64(r11.wrapping_add(0x18u64), r8); crate::simd_ops::wu64(r11.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x90u64)) as u64; rsi = r9; rdi = r8; rbp = rdx; rbx = rcx; rax = r11.wrapping_sub(0x68); crate::simd_ops::wu64(r11.wrapping_add(0x20u64), rax); r14 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(r11.wrapping_sub(0x30u64), r14); rcx = (crate::simd_ops::ru64(r8.wrapping_add(0x38u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = r11.wrapping_sub(0x68); (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rsp.wrapping_add(0x88u64), rax); bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xe8u64))) as u64; xmm0 = (crate::simd_ops::ru128(rax)) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(rsi)) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm1) as u128); rax = rsp.wrapping_add(0x30); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xe0u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); r9 = rsp.wrapping_add(0x40); r8 = rsp.wrapping_add(0x50); rdx = rbp; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = P1; crate::simd_ops::wu64(rbx, rax); rbp = rbx.wrapping_add(0xf8); crate::simd_ops::wu64(rsp.wrapping_add(0xe8u64), rbp); crate::simd_ops::wu64(rbp.wrapping_add(0x38u64), r14); rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x110u64))) as u64; rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x38u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = rbp; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rbp.wrapping_add(0x38u64), rax); bb = 4; continue; }
            4 => { rcx = rbx.wrapping_add(0x138); crate::simd_ops::wu64(rsp.wrapping_add(0xe8u64), rcx); crate::simd_ops::lk_and_u32(rcx, (0xff000004u32) as u32); crate::simd_ops::lk_or_u32(rcx, (0x4u32) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x3u64), (0x4u8) as u8); crate::simd_ops::wu32(rcx.wrapping_add(0x4u64), ((r14 as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x8u64), (0x0u8) as u8); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xf0u64))) as u64; xmm0 = (crate::simd_ops::ru128(rax)) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm0) as u128); r9 = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; r8 = 0; fl_l = 0; fl_r = 0; rdx = rsp.wrapping_add(0x40); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0xf8u64))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x160u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x100u64))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x164u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x108u64))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x168u64), ((rax as u32)) as u32); rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x38u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rcx)) as u64; fl_l = rcx; fl_r = rdi; rdx = (rdx & !0xffu64) | (((if (((fl_l as u64)) != ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x20u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rdi.wrapping_add(0x38u64), r14); bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x38u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru64(rcx)) as u64; fl_l = rcx; fl_r = rsi; rdx = (rdx & !0xffu64) | (((if (((fl_l as u64)) != ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x20u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rsi.wrapping_add(0x38u64), r14); bb = 8; continue; }
            8 => { rax = rbx; rsp = ((rsp).wrapping_add(0x90u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1424af040(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143079708; const P1: u64 = 0x1424ad430; const P2: u64 = 0x1424ad240; const P3: u64 = 0x1434109b8; const P4: u64 = 0xf8; const P5: u64 = 0xfc; const P6: u64 = 0x3; const P7: u64 = 0x4; const P8: u64 = 0x8; const P9: u64 = 0x142409b50; const P10: u64 = 0x148; const P11: u64 = 0x150;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 19>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r11 = rsp; crate::simd_ops::wu64(r11.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(r11.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x90u64)) as u64; rbx = rcx; rax = P0; crate::simd_ops::wu64(r11.wrapping_sub(0x48u64), rax); rax = P1; crate::simd_ops::wu64(r11.wrapping_sub(0x40u64), rax); rax = r11.wrapping_sub(0x48); crate::simd_ops::wu64(r11.wrapping_sub(0x10u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xc0u64))) as u64; xmm0 = (crate::simd_ops::ru128(rax)) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(r8)) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm1) as u128); rax = r11.wrapping_sub(0x68); crate::simd_ops::wu64(r11.wrapping_sub(0x70u64), rax); crate::simd_ops::wu64(r11.wrapping_sub(0x78u64), r9); r9 = r11.wrapping_sub(0x58); r8 = r11.wrapping_sub(0x48); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = P3; crate::simd_ops::wu64(rbx, rax); crate::simd_ops::wu8(rbx.wrapping_add(P4), (0x1u8) as u8); rcx = ((rbx.wrapping_add(P5)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xc0u64), rcx); crate::simd_ops::lk_and_u32(rcx, (0xff000008u32) as u32); crate::simd_ops::lk_or_u32(rcx, (0x8u32) as u32); crate::simd_ops::wu8(rcx.wrapping_add(P6), (0x8u8) as u8); rdi = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rcx.wrapping_add(P7), ((rdi as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(P8), ((rdi as u8)) as u8); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xc8u64))) as u64; xmm0 = (crate::simd_ops::ru128(rax)) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm0) as u128); r9 = (crate::simd_ops::ru64(rax.wrapping_add(P8))) as u64; r8 = 0; fl_l = 0; fl_r = 0; rdx = rsp.wrapping_add(0x40); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm0 = 0u128; crate::simd_ops::wu128(rbx.wrapping_add(P10), (xmm0) as u128); crate::simd_ops::wu64(rbx.wrapping_add(P10), rdi); crate::simd_ops::wu64(rbx.wrapping_add(P11), rdi); rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0xa8u64))) as u64; rsp = ((rsp).wrapping_add(0x90u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1424af140(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1424ae640; const P1: u64 = 0x143410978; const P2: u64 = 0xa0; const P3: u64 = 0x8; const P4: u64 = 0x142409b50;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x30u64)) as u64;
    rbx = rdx;
    rdi = rcx;
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rax = P1;
    crate::simd_ops::wu64(rdi, rax);
    rcx = ((rdi.wrapping_add(P2)) as u64) as u64;
    xmm0 = (crate::simd_ops::ru128(rbx)) as u128;
    crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128);
    r9 = (crate::simd_ops::ru64(rbx.wrapping_add(P3))) as u64;
    r8 = 0;
    fl_l = 0; fl_r = 0;
    rdx = rsp.wrapping_add(0x20);
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rax = rdi;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64;
    rsp = ((rsp).wrapping_add(0x30u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1424af1a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142645878; const P1: u64 = 0x1424ad440;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x60u64)) as u64; rbx = rcx; rdi = r8; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0xf0u64))) as u64; rsi = rdx; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, 0x1424af1ca, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            2 => { rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 12; continue; } bb = 3; continue; }
            3 => { rdx = ((crate::simd_ops::ru8(rbx.wrapping_add(0xbu64))) as u32) as u32 as u64; rcx = rbx.wrapping_add(0x10); fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rdx = ((rdx.wrapping_mul(8)) as u32) as u32 as u64; bb = 6; continue; }
            5 => { rdx = (((rdx as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x7c00; rdx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; rax = ((((rax as u32)) & (0xffffffu32))) as u32 as u64; rax = (((rax as u32)).wrapping_shl(3u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 8; continue; } bb = 7; continue; }
            7 => { rcx = (crate::simd_ops::ru64(rcx)) as u64; bb = 8; continue; }
            8 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rcx); rax = (0xffffffffffffffffu64) as u64; bb = 9; continue; }
            9 => { rax = ((rax).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(rax))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 9; continue; } bb = 10; continue; }
            10 => { xmm0 = (crate::simd_ops::ru128(rsi)) as u128; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); r8 = rsp.wrapping_add(0x30); xmm1 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; rdx = rsp.wrapping_add(0x20); rcx = rsp.wrapping_add(0x40); crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm1) as u128); crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru8(rsp.wrapping_add(0x50u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 13; continue; } bb = 11; continue; }
            11 => { xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x40u64))) as u128; rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = rsp.wrapping_add(0x30); rcx = rbx; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x20u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu8(rbx.wrapping_add(0x98u64), ((rax as u8)) as u8); crate::simd_ops::wu8(rdi, ((rax as u8)) as u8); bb = 12; continue; }
            12 => { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rsp = ((rsp).wrapping_add(0x60u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            13 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; crate::simd_ops::wu8(rdi, (0x0u8) as u8); rsp = ((rsp).wrapping_add(0x60u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1424af2b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1426cdcb0; const P1: u64 = 0x143079708; const P2: u64 = 0x1424ad430; const P3: u64 = 0x1424ad240; const P4: u64 = 0x1434108e8;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 19>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    r11 = rsp;
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x90u64)) as u64;
    xmm1 = (crate::simd_ops::ru128(r8)) as u128;
    rbx = rcx;
    rax = P0;
    crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax);
    r9 = r11.wrapping_sub(0x58);
    rcx = 0;
    fl_l = 0; fl_r = 0;
    rax = P1;
    crate::simd_ops::wu64(r11.wrapping_sub(0x48u64), rax);
    r8 = r11.wrapping_sub(0x48);
    crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rcx);
    rax = P2;
    xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128;
    crate::simd_ops::wu64(r11.wrapping_sub(0x40u64), rax);
    rax = r11.wrapping_sub(0x48);
    crate::simd_ops::wu64(r11.wrapping_sub(0x10u64), rax);
    rax = r11.wrapping_sub(0x68);
    crate::simd_ops::wu64(r11.wrapping_sub(0x70u64), rax);
    crate::simd_ops::wu64(r11.wrapping_sub(0x78u64), rcx);
    rcx = rbx;
    crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128);
    crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm1) as u128);
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rax = P4;
    crate::simd_ops::wu64(rbx, rax);
    rax = rbx;
    rsp = ((rsp).wrapping_add(0x90u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1424af340(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1424ad240; const P1: u64 = 0x143410888; const P2: u64 = 0x142409b50;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 23>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r11 = rsp; crate::simd_ops::wu64(r11.wrapping_add(0x18u64), r8); crate::simd_ops::wu64(r11.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x90u64)) as u64; rbx = r9; rsi = r8; rbp = rdx; rdi = rcx; rax = r11.wrapping_sub(0x68); crate::simd_ops::wu64(r11.wrapping_add(0x20u64), rax); r14 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(r11.wrapping_sub(0x30u64), r14); rcx = (crate::simd_ops::ru64(r8.wrapping_add(0x38u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = r11.wrapping_sub(0x68); (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rsp.wrapping_add(0x88u64), rax); bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xe8u64))) as u64; xmm0 = (crate::simd_ops::ru128(rax)) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(rbx)) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm1) as u128); rax = rsp.wrapping_add(0x30); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xe0u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); r9 = rsp.wrapping_add(0x40); r8 = rsp.wrapping_add(0x50); rdx = rbp; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = P1; crate::simd_ops::wu64(rdi, rax); rbx = rdi.wrapping_add(0xf8); crate::simd_ops::wu64(rsp.wrapping_add(0xe8u64), rbx); crate::simd_ops::wu64(rbx.wrapping_add(0x38u64), r14); rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x100u64))) as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x38u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rbx.wrapping_add(0x38u64), rax); bb = 4; continue; }
            4 => { rcx = rdi.wrapping_add(0x138); crate::simd_ops::wu64(rsp.wrapping_add(0xe8u64), rcx); crate::simd_ops::lk_and_u32(rcx, (0xff000008u32) as u32); crate::simd_ops::lk_or_u32(rcx, (0x8u32) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x3u64), (0x8u8) as u8); crate::simd_ops::wu32(rcx.wrapping_add(0x4u64), ((r14 as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x8u64), (0x0u8) as u8); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xf0u64))) as u64; xmm0 = (crate::simd_ops::ru128(rax)) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm0) as u128); r9 = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; r8 = 0; fl_l = 0; fl_r = 0; rdx = rsp.wrapping_add(0x40); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rdi.wrapping_add(0x180); crate::simd_ops::wu64(rsp.wrapping_add(0xe8u64), rcx); crate::simd_ops::lk_and_u32(rcx, (0xff000008u32) as u32); crate::simd_ops::lk_or_u32(rcx, (0x8u32) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x3u64), (0x8u8) as u8); crate::simd_ops::wu32(rcx.wrapping_add(0x4u64), ((r14 as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x8u64), (0x0u8) as u8); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0xf8u64))) as u64; xmm0 = (crate::simd_ops::ru128(rbx)) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm0) as u128); r9 = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; r8 = 0; fl_l = 0; fl_r = 0; rdx = rsp.wrapping_add(0x40); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rdi.wrapping_add(0x1c8); crate::simd_ops::wu64(rsp.wrapping_add(0xe8u64), rcx); crate::simd_ops::lk_and_u32(rcx, (0xff000008u32) as u32); crate::simd_ops::lk_or_u32(rcx, (0x8u32) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x3u64), (0x8u8) as u8); crate::simd_ops::wu32(rcx.wrapping_add(0x4u64), ((r14 as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x8u64), (0x0u8) as u8); xmm0 = (crate::simd_ops::ru128(rbx)) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm0) as u128); r9 = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; r8 = 0; fl_l = 0; fl_r = 0; rdx = rsp.wrapping_add(0x40); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x38u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rcx)) as u64; fl_l = rcx; fl_r = rsi; rdx = (rdx & !0xffu64) | (((if (((fl_l as u64)) != ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x20u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rsi.wrapping_add(0x38u64), r14); bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x38u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru64(rcx)) as u64; fl_l = rcx; fl_r = rbp; rdx = (rdx & !0xffu64) | (((if (((fl_l as u64)) != ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x20u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rbp.wrapping_add(0x38u64), r14); bb = 8; continue; }
            8 => { rax = rdi; rsp = ((rsp).wrapping_add(0x90u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1424af530(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1424ad240; const P1: u64 = 0x1434108b8;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 23>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r11 = rsp; crate::simd_ops::wu64(r11.wrapping_add(0x18u64), r8); crate::simd_ops::wu64(r11.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x90u64)) as u64; rsi = r9; rdi = r8; rbp = rdx; rbx = rcx; rax = r11.wrapping_sub(0x68); crate::simd_ops::wu64(r11.wrapping_add(0x20u64), rax); r14 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(r11.wrapping_sub(0x30u64), r14); rcx = (crate::simd_ops::ru64(r8.wrapping_add(0x38u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = r11.wrapping_sub(0x68); (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rsp.wrapping_add(0x88u64), rax); bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xe8u64))) as u64; xmm0 = (crate::simd_ops::ru128(rax)) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(rsi)) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm1) as u128); rax = rsp.wrapping_add(0x30); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xe0u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); r9 = rsp.wrapping_add(0x40); r8 = rsp.wrapping_add(0x50); rdx = rbp; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = P1; crate::simd_ops::wu64(rbx, rax); rbp = rbx.wrapping_add(0xf8); crate::simd_ops::wu64(rsp.wrapping_add(0xe8u64), rbp); crate::simd_ops::wu64(rbp.wrapping_add(0x38u64), r14); rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0xf8u64))) as u64; rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x38u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = rbp; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rbp.wrapping_add(0x38u64), rax); bb = 4; continue; }
            4 => { rax = ((crate::simd_ops::ru8(rsp.wrapping_add(0xf0u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rbx.wrapping_add(0x138u64), ((rax as u8)) as u8); crate::simd_ops::wu8(rbx.wrapping_add(0x139u64), ((rax as u8)) as u8); rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x38u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rcx)) as u64; fl_l = rcx; fl_r = rdi; rdx = (rdx & !0xffu64) | (((if (((fl_l as u64)) != ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x20u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rdi.wrapping_add(0x38u64), r14); bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x38u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru64(rcx)) as u64; fl_l = rcx; fl_r = rsi; rdx = (rdx & !0xffu64) | (((if (((fl_l as u64)) != ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x20u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rsi.wrapping_add(0x38u64), r14); bb = 8; continue; }
            8 => { rax = rbx; rsp = ((rsp).wrapping_add(0x90u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1424af660(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x140215e80; const P1: u64 = 0x1424ad9a0; const P2: u64 = 0x1401de830; const P3: u64 = 0x1433d377c; const P4: u64 = 0x1426d01e0; const P5: u64 = 0x141378aa0; const P6: u64 = 0x14183acc8; const P7: u64 = 0x1424ae200; const P8: u64 = 0x1425371a0; const P9: u64 = 0x142533200; const P10: u64 = 0x1401de590;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 25>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1424af660, funcinfo_rva: 0x37eff58, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0xc8, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = (rsp) as u64; crate::simd_ops::wu64(rax.wrapping_add(0x18u64), (rbx) as u64); crate::simd_ops::wu64(rax.wrapping_add(0x8u64), (rcx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r12) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r13) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r15) as u64); rbp = ((rax.wrapping_sub(0x5fu64)) as u64) as u64; rsp = ((rsp).wrapping_sub(0x90u64)) as u64; crate::simd_ops::wu128(rax.wrapping_sub(0x48u64), (xmm6) as u128); r15 = (rcx) as u64; fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(0x98u64))) as u64 as u64; fl_r = (0x0u64) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 18; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x94u64))) as u32 as u64; rax = (((rax as u32)).wrapping_sub(1)) as u32 as u64; rdi = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; r14 = ((rdi as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; r14 = (if (((((fl_l as u32)) & ((fl_r as u32))) as i32) > 0) { (rax as u32) } else { (r14 as u32) }) as u32 as u64; rsi = ((((r14 as u32)) as i32 as i64) as u64) as u64; xmm0 = 0u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x39u64), (xmm0) as u128); rbx = ((rdi as u32)) as u32 as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x39u64), (rbx) as u64); crate::simd_ops::wu32(rbp.wrapping_sub(0x2du64), ((rdi as u32)) as u32); rax = ((rdi as u32)) as u32 as u64; fl_l = ((r14 as u32)) as u32 as u64; fl_r = ((r14 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 5; continue; } bb = 2; continue; }
            2 => { rdx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1424af6c9); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rbx = (rax) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x39u64), (rax) as u64); crate::simd_ops::wu32(rbp.wrapping_sub(0x2du64), ((r14 as u32)) as u32); fl_l = ((r14 as u32)) as u32 as u64; fl_r = ((r14 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 5; continue; } bb = 3; continue; }
            3 => { bb = 4; continue; }
            4 => { crate::simd_ops::wu64(rax, (rdi) as u64); crate::simd_ops::wu64(rax.wrapping_add(0x8u64), (rdi) as u64); rax = ((rax.wrapping_add(0x10u64)) as u64) as u64; fl_l = (rsi) as u64 as u64; fl_r = (0x1u64) as u64 as u64; rsi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } bb = 5; continue; }
            5 => { crate::simd_ops::wu32(rbp.wrapping_sub(0x31u64), ((r14 as u32)) as u32); rax = ((r14 as u32)) as u32 as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rax = ((((rax) as i64).wrapping_shr(4u32) as u64)) as u64; fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 6; continue; }
            6 => { rdx = (rdi) as u64; bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru64(r15.wrapping_add(0x98u64))) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(rdi.wrapping_mul(8)).wrapping_add(0x8u64))) as u64; rax = (0xffffffffffffffffu64) as u64; bb = 8; continue; }
            8 => { rax = ((rax).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(rax))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 8; continue; } bb = 9; continue; }
            9 => { crate::simd_ops::wu64(rdx.wrapping_add(rbx), (rcx) as u64); crate::simd_ops::wu64(rdx.wrapping_add(rbx).wrapping_add(0x8u64), (rax) as u64); rdi = ((rdi).wrapping_add(1)) as u64; rdx = ((rdx).wrapping_add(0x10u64)) as u64; rax = ((r14 as u32)) as u32 as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rax = ((((rax) as i64).wrapping_shr(4u32) as u64)) as u64; fl_l = (rdi) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 7; continue; } bb = 10; continue; }
            10 => { rdi = ((r14 as u32)) as u32 as u64; rdi = ((rdi).wrapping_shl(4u32)) as u64; rdi = (((rdi) & (0xfffffffffffffff0u64))) as u64; rdi = ((rdi).wrapping_add(rbx)) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rdi) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 17; continue; } bb = 11; continue; }
            11 => { bb = 12; continue; }
            12 => { rdx = (crate::simd_ops::ru64(rbx)) as u64; rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rax = ((crate::simd_ops::ru8(rdx)) as u32) as u32 as u64; rax = (rax & !0xffu64) | (((((rax as u8)).wrapping_sub(0x2du8)) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = (0xfdu8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 16; continue; } bb = 13; continue; }
            13 => { fl_l = (rcx) as u64 as u64; fl_r = (0x1u64) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 48; continue; } bb = 14; continue; }
            14 => { rax = ((rcx.wrapping_sub(0x1u64)) as u64) as u64; rcx = (0xffffffffffffffffu64) as u64; fl_l = (rax) as u64 as u64; fl_r = (rcx) as u64 as u64; rcx = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { rcx }) as u64; rax = ((rdx.wrapping_add(0x1u64)) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x19u64), (rax) as u64); crate::simd_ops::wu64(rbp.wrapping_sub(0x11u64), (rcx) as u64); xmm0 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x19u64))) as u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x29u64), (xmm0) as u128); rdx = ((rbp.wrapping_sub(0x29u64)) as u64) as u64; rcx = (r15) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1424af7a9); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 16; continue; } bb = 15; continue; }
            15 => { crate::simd_ops::wu8(r15.wrapping_add(0x8u64), (0x1u8) as u8); bb = 16; continue; }
            16 => { rbx = ((rbx).wrapping_add(0x10u64)) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rdi) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 12; continue; } bb = 17; continue; }
            17 => { rcx = ((rbp.wrapping_sub(0x39u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1424af7c4); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 47; continue; }
            18 => { fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(0xa4u64))) as u32 as u64; fl_r = (0x0u32) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 47; continue; } bb = 19; continue; }
            19 => { xmm0 = 0u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x39u64), (xmm0) as u128); rdi = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rbp.wrapping_sub(0x39u64), (rdi) as u64); r13 = ((rdi as u32)) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_sub(0x31u64), ((rdi as u32)) as u32); rax = ((rdi as u32)) as u32 as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x6fu64), (rax) as u64); crate::simd_ops::wu32(rbp.wrapping_sub(0x2du64), ((rax as u32)) as u32); r14 = ((rcx.wrapping_add(0xa8u64)) as u64) as u64; rcx = ((crate::simd_ops::ru8(rcx.wrapping_add(0xa3u64))) as u32) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = (0x80u32) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 21; continue; } bb = 20; continue; }
            20 => { rcx = ((rcx.wrapping_mul(8)) as u32) as u32 as u64; bb = 22; continue; }
            21 => { rcx = (((rcx as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = (0x7c00u32) as u32 as u64; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 22; continue; }
            22 => { rax = (crate::simd_ops::ru32(r15.wrapping_add(0xa0u64))) as u32 as u64; rax = ((((rax as u32)) & (0xffffffu32))) as u32 as u64; rax = (((rax as u32)).wrapping_shl(3u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 24; continue; } bb = 23; continue; }
            23 => { r14 = (crate::simd_ops::ru64(r14)) as u64; bb = 24; continue; }
            24 => { rsi = (crate::simd_ops::ru32(r15.wrapping_add(0xa4u64))) as u32 as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x29u64), (r14) as u64); crate::simd_ops::wu64(rbp.wrapping_sub(0x21u64), (rsi) as u64); rax = ((rdi as u32)) as u32 as u64; fl_l = (rsi) as u64 as u64; fl_r = (rsi) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 39; continue; } bb = 25; continue; }
            25 => { r12 = ((P3) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x19u64), (r12) as u64); crate::simd_ops::wu64(rbp.wrapping_sub(0x11u64), (0x4u64) as u64); rbx = ((P4) as u64) as u64; xmm6 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x19u64))) as u128; r15 = (crate::simd_ops::ru64(rbp.wrapping_add(0x6fu64))) as u64; bb = 26; continue; }
            26 => { crate::simd_ops::wu128(rbp.wrapping_sub(0x9u64), (xmm6) as u128); xmm0 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x29u64))) as u128; crate::simd_ops::wu128(rbp.wrapping_add(0x7u64), (xmm0) as u128); crate::simd_ops::wu8(rsp.wrapping_add(0x20u64), (0x1u8) as u8); r9 = (r9 & !0xffu64) | (((0x1u8) as u8 as u64)); r8 = ((rbp.wrapping_sub(0x9u64)) as u64) as u64; rdx = ((rbp.wrapping_add(0x7u64)) as u64) as u64; rcx = ((rbp.wrapping_sub(0x19u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1424af8a3); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x11u64))) as u64; rcx = ((rcx).wrapping_sub(r14)) as u64; rdx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x19u64))) as u64; rdx = ((rdx).wrapping_add(1)) as u64; rdx = ((rdx).wrapping_add(rcx)) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rsi) as u64 as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 34; continue; } bb = 27; continue; }
            27 => { r10 = ((rsi.wrapping_add(r14)) as u64) as u64; rax = ((rdx.wrapping_add(r14)) as u64) as u64; bb = 28; continue; }
            28 => { fl_l = (rax) as u64 as u64; fl_r = (r10) as u64 as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 34; continue; } bb = 29; continue; }
            29 => { rcx = ((crate::simd_ops::ru8(rax)) as u32) as u32 as u64; r9 = ((crate::simd_ops::ru8(rcx.wrapping_add(rbx))) as u32) as u32 as u64; rdx = (r12) as u64; r8 = (0x4u32) as u32 as u64; bb = 30; continue; }
            30 => { r8 = ((r8).wrapping_sub(1)) as u64; rcx = ((crate::simd_ops::ru8(rdx)) as u32) as u32 as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(rbx))) as u8 as u64; fl_r = ((r9 as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 33; continue; } bb = 31; continue; }
            31 => { rdx = ((rdx).wrapping_add(1)) as u64; fl_l = (r8) as u64 as u64; fl_r = (r8) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 30; continue; } bb = 32; continue; }
            32 => { fl_l = (rax) as u64 as u64; fl_r = (r14) as u64 as u64; rax = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; bb = 35; continue; }
            33 => { fl_l = ((rax).wrapping_add(1)) as u64 as u64; rax = ((fl_l as u64)) as u64; bb = 28; continue; }
            34 => { rax = (0xffffffffffffffffu64) as u64; bb = 35; continue; }
            35 => { rcx = (rsi) as u64; fl_l = (rax) as u64 as u64; fl_r = (rsi) as u64 as u64; rcx = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { rcx }) as u64; r14 = ((r14).wrapping_add(rcx)) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x29u64), (r14) as u64); rsi = ((rsi).wrapping_sub(rcx)) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x21u64), (rsi) as u64); r12 = ((r13 as u32)) as u32 as u64; r12 = ((r12).wrapping_shl(4u32)) as u64; r12 = ((r12).wrapping_add(rdi)) as u64; rcx = ((r15 as u32)) as u32 as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rcx = ((rcx).wrapping_add(rdi)) as u64; fl_l = (r12) as u64 as u64; fl_r = (rcx) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 37; continue; } bb = 36; continue; }
            36 => { r12 = ((r12).wrapping_sub(rdi)) as u64; r13 = (r12) as u64; r13 = ((((r13) as i64).wrapping_shr(4u32) as u64)) as u64; rax = ((r13.wrapping_add(0x1u64)) as u64) as u64; rcx = ((rcx).wrapping_sub(rdi)) as u64; rcx = ((((rcx) as i64).wrapping_shr(4u32) as u64)) as u64; r15 = (rcx) as u64; r15 = ((r15).wrapping_shr(1u32)) as u64; r15 = ((r15).wrapping_add(rcx)) as u64; fl_l = (r15) as u64 as u64; fl_r = (rax) as u64 as u64; r15 = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { r15 }) as u64; rdx = (r15) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1424af95e); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rbx = (rax) as u64; r8 = (r12) as u64; rdx = (rdi) as u64; rcx = (rax) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1424af96f); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = ((rbp.wrapping_sub(0x39u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1424af978); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdi = (rbx) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x39u64), (rbx) as u64); crate::simd_ops::wu32(rbp.wrapping_sub(0x2du64), ((r15 as u32)) as u32); rbx = ((P4) as u64) as u64; bb = 37; continue; }
            37 => { rax = ((r13 as u32)) as u32 as u64; rax = ((rax).wrapping_add(rax)) as u64; xmm0 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x19u64))) as u128; crate::simd_ops::wu128(rdi.wrapping_add(rax.wrapping_mul(8)), (xmm0) as u128); rax = ((r13.wrapping_add(0x1u64)) as u32) as u32 as u64; r13 = ((rax as u32)) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_sub(0x31u64), ((rax as u32)) as u32); fl_l = (rsi) as u64 as u64; fl_r = (rsi) as u64 as u64; r12 = ((P3) as u64) as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 26; continue; } bb = 38; continue; }
            38 => { r15 = (crate::simd_ops::ru64(rbp.wrapping_add(0x67u64))) as u64; bb = 39; continue; }
            39 => { rax = ((rax).wrapping_shl(4u32)) as u64; rax = ((((rax) as i64).wrapping_shr(4u32) as u64)) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x19u64), (rax) as u64); crate::simd_ops::wu64(rbp.wrapping_sub(0x11u64), (rdi) as u64); xmm0 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x19u64))) as u128; crate::simd_ops::wu128(rbp.wrapping_add(0x7u64), (xmm0) as u128); rdx = ((rbp.wrapping_add(0x7u64)) as u64) as u64; rcx = (r15) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1424af9de); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = (rdi) as u64 as u64; fl_r = (rdi) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 47; continue; } bb = 40; continue; }
            40 => { rdi = ((rdi).wrapping_sub(1)) as u64; rdi = (((rdi) & (0xfffffffffe000000u64))) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = (rax) as u64 as u64; fl_r = (crate::simd_ops::ru64(rdi.wrapping_add(0x100u64))) as u64 as u64; rdx = (rdx & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); r9 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x39u64))) as u64; rax = (r9) as u64; rax = ((rax).wrapping_sub(rdi)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rcx = ((rcx).wrapping_add(0x108u64)) as u64; rcx = ((rcx).wrapping_add(rdi)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x4u64))) as u32 as u64; rcx = ((rcx).wrapping_sub(rax)) as u64; fl_l = ((rdx as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 45; continue; } bb = 41; continue; }
            41 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0xeu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 44; continue; } bb = 42; continue; }
            42 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(r9, (rax) as u64); crate::simd_ops::wu64(rcx.wrapping_add(0x20u64), (r9) as u64); fl_l = (crate::simd_ops::lk_add_u32(rcx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 47; continue; } bb = 43; continue; }
            43 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1424afa47); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 47; continue; }
            44 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 46; continue; }
            45 => { r8 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 46; continue; }
            46 => { rdx = (rcx) as u64; rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1424afa5c); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 47; continue; }
            47 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0xe0u64))) as u64; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x80u64))) as u128; rsp = ((rsp).wrapping_add(0x90u64)) as u64; r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r13 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r12 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rsi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            48 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1424afa86); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P10) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1424afaa0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x9;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = ((crate::simd_ops::ru8(rcx.wrapping_add(P0))) as u32) as u32 as u64;
    crate::simd_ops::wu8(rcx.wrapping_add(P0), (0x1u8) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1424afb00(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x98; const P1: u64 = 0xf0; const P2: u64 = 0x142645878; const P3: u64 = 0x10; const P4: u64 = 0x3; const P5: u64 = 0x4; const P6: u64 = 0x8; const P7: u64 = 0x14240aa00;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rbx = rdx; rdi = rcx; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(P0))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 5; continue; } bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x1424afb30, rcx, rsp); return rax; }
            3 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 6; continue; }
            5 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 6; continue; }
            6 => { rdx = 0x1c8; rcx = 0x180; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; rdx = (if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { (rcx as u32) } else { (rdx as u32) }) as u32 as u64; rdx = ((rdx).wrapping_add(rdi)) as u64; crate::simd_ops::lk_and_u32(rbx, (0xff000008u32) as u32); crate::simd_ops::lk_or_u32(rbx, (0x8u32) as u32); crate::simd_ops::wu8(rbx.wrapping_add(P4), (0x8u8) as u8); crate::simd_ops::wu32(rbx.wrapping_add(P5), (0x0u32) as u32); crate::simd_ops::wu8(rbx.wrapping_add(P6), (0x0u8) as u8); rcx = rbx; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1424afb90(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143882050; const P1: u64 = 0x1413d85b0; const P2: u64 = 0x14183acbc;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 43>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = rsp; crate::simd_ops::wu64(rax.wrapping_add(0x18u64), r8); crate::simd_ops::wu64(rax.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x140u64)) as u64; crate::simd_ops::wu64(rax.wrapping_sub(0x20u64), rbp); rdx = 0x10; crate::simd_ops::wu64(rax.wrapping_sub(0x28u64), rsi); r13 = r8; crate::simd_ops::wu64(rax.wrapping_sub(0x30u64), rdi); r8 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rax.wrapping_sub(0x38u64), r12); r12 = ((r9 as u32)) as u32 as u64; r15 = ((rdx.wrapping_sub(0xdu64)) as u32) as u32 as u64; rax = rsp.wrapping_add(0x28); bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rax.wrapping_sub(0x8u64), r8); crate::simd_ops::wu64(rax, r8); rax = rax.wrapping_add(0x10); fl_l = rdx; fl_r = (0x1u64) as u64; rdx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 2; continue; }
            2 => { rax = r13; crate::simd_ops::wu64(rsp.wrapping_add(0x168u64), rbx); r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x160u64))) as u64; rdi = ((rdx.wrapping_add(0x1u64)) as u32) as u32 as u64; r14 = rcx; rsi = P0; rbp = ((rdx.wrapping_add(0x6u64)) as u32) as u32 as u64; bb = 3; continue; }
            3 => { fl_l = ((r12 as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { fl_l = rax; fl_r = r13; rbx = rsp.wrapping_add(0x20); r8 = rsi; rdx = ((r15 as u32)) as u32 as u64; rbx = (if (((fl_l as u64)) != ((fl_r as u64))) { rax } else { rbx }) as u64; rcx = r14; r9 = rbx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); r15 = ((rax as u32)) as u32 as u64; rax = r14; r14 = rbx; bb = 5; continue; }
            5 => { rsi = ((rsi).wrapping_add(0x10u64)) as u64; rdi = (((rdi as u32)).rotate_left(1u32)) as u32 as u64; fl_l = rbp; fl_r = (0x1u64) as u64; rbp = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 6; continue; }
            6 => { r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x170u64))) as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x120u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x128u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x130u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x138u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x168u64))) as u64; fl_l = r14; fl_r = r13; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 8; continue; } bb = 7; continue; }
            7 => { r8 = ((((r15 as u32)) as i32 as i64) as u64) as u64; rdx = r14; r8 = ((r8).wrapping_shl(4u32)) as u64; rcx = r13; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 8; continue; }
            8 => { rax = ((r15 as u32)) as u32 as u64; rsp = ((rsp).wrapping_add(0x140u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1424afcb0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x20; const P1: u64 = 0x8; const P2: u64 = 0x1424afcb0; const P3: u64 = 0x1b0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru64(r8)) as u64; rsi = rcx; rcx = r8; rdi = r8; rbx = rdx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); r9 = (crate::simd_ops::ru64(rdi)) as u64; rcx = rdi; r14 = rax; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(r9.wrapping_add(P1))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); r9 = (crate::simd_ops::ru64(r14)) as u64; rcx = r14; rbp = ((rax as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(r9.wrapping_add(P1))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rbp as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = 0; fl_l = 0; fl_r = 0; bb = 5; continue; }
            2 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = rsi; bb = 5; continue; }
            4 => { r8 = r14; rdx = rbx; rcx = rsi; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rdi)) as u64; rdx = rax; r8 = (crate::simd_ops::ru64(rcx.wrapping_add(P3))) as u64; rcx = rdi; rax = crate::simd_ops::call_stk(r8, rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            5 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1424afe20(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x30; const P1: u64 = 0x20; const P2: u64 = 0x143881f9c; const P3: u64 = 0x8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    rdi = rcx;
    crate::simd_ops::wu32(rcx.wrapping_add(P0), (0x0u32) as u32);
    rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp);
    rsi = (crate::simd_ops::ru32(P2)) as u32 as u64;
    rbx = rax;
    rcx = rbx;
    rax = (crate::simd_ops::ru64(rax)) as u64;
    rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp);
    fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64;
    if (((fl_l as u32)) == ((fl_r as u32))) { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    loop { crate::simd_ops::lk_add_u32(rdi.wrapping_add(P0), (1) as u32); rcx = rbx; rax = (crate::simd_ops::ru64(rbx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rbx = rax; rcx = rbx; rax = (crate::simd_ops::ru64(rax)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if !((((fl_l as u32)) != ((fl_r as u32)))) { break; } }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1424afea0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143881f9c; const P1: u64 = 0x8; const P2: u64 = 0x34; const P3: u64 = 0x20;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rbx = rcx; rdi = (crate::simd_ops::ru32(P0)) as u32 as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 4; continue; } bb = 1; continue; }
            1 => { bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(P2))) as u8 as u64; fl_r = (0x2u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 5; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rbx = rax; rcx = rbx; rax = (crate::simd_ops::ru64(rax)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 2; continue; } bb = 4; continue; }
            4 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            5 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1424aff00(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1434109e8; const P1: u64 = 0x34; const P2: u64 = 0x30; const P3: u64 = 0x8; const P4: u64 = 0x10; const P5: u64 = 0x18; const P6: u64 = 0x20; const P7: u64 = 0x28; const P8: u64 = 0x142645790; const P9: u64 = 0x143871788; const P10: u64 = 0x143871780; const P11: u64 = 0x1426455a8; const P12: u64 = 0x14387178c; const P13: u64 = 0x14397d8de; const P14: u64 = 0x143a083d8; const P15: u64 = 0x143a083d0; const P16: u64 = 0x14397d8df; const P17: u64 = 0x142645598;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = P0; crate::simd_ops::wu32(rcx.wrapping_add(P1), ((rdx as u32)) as u32); crate::simd_ops::wu64(rcx, rax); rdi = 0xffffffff; rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rcx.wrapping_add(P2), ((rdi as u32)) as u32); crate::simd_ops::wu64(rcx.wrapping_add(P3), rax); rbx = rcx; crate::simd_ops::wu64(rcx.wrapping_add(P4), rax); crate::simd_ops::wu64(rcx.wrapping_add(P5), rax); crate::simd_ops::wu64(rcx.wrapping_add(P6), rax); crate::simd_ops::wu64(rcx.wrapping_add(P7), rax); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P8), rax), rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru32(P9)) as u32 as u64; rsi = ((rax as u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = P10; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P11), rax), rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu32(P9, ((rsi as u32)) as u32); bb = 2; continue; }
            2 => { crate::simd_ops::lk_add_u32(P12, (1) as u32); fl_l = (crate::simd_ops::ru8(P13)) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(P14)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), rax); crate::simd_ops::wu64(rcx, rbx); bb = 5; continue; }
            4 => { rax = (crate::simd_ops::ru64(P15)) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), rax); crate::simd_ops::wu64(P15, rbx); bb = 5; continue; }
            5 => { fl_l = (crate::simd_ops::ru32(P12)) as u32 as u64; fl_r = 0x1; crate::simd_ops::wu32(P12, ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32); crate::simd_ops::wu8(P16, (0x1u8) as u8); if (((fl_l as u32)) != ((fl_r as u32))) { bb = 7; continue; } bb = 6; continue; }
            6 => { let tmp_xchg: u32 = crate::simd_ops::atomic_xchg_u32(P9, ((rdi as u32)) as u32); rdi = (tmp_xchg) as u32 as u64; rcx = P10; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P17), rax), rax, rcx, rdx, r8, r9, rsp); bb = 7; continue; }
            7 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1424affe0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x143881f9c; const P2: u64 = 0x20;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; rbx = rcx; rcx = rdx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; rsi = (crate::simd_ops::ru32(P1)) as u32 as u64; rdi = ((rax as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 3; continue; } bb = 1; continue; }
            1 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 4; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rbx = rax; rcx = rbx; rax = (crate::simd_ops::ru64(rax)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 1; continue; } bb = 3; continue; }
            3 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            4 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1424b0110(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x20; const P1: u64 = 0x28; const P2: u64 = 0x8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; rsi = rcx; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 8; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(P1))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 8; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rdx)) as u64; rcx = rdx; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rdi); rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rsi)) as u64; rbx = rax; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; rdi = ((rax as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rdx.wrapping_add(P2))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 6; continue; } bb = 3; continue; }
            3 => { bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 7; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rsi)) as u64; rbx = rax; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; rdi = ((rax as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rdx.wrapping_add(P2))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 4; continue; } bb = 6; continue; }
            6 => { rax = 0; fl_l = 0; fl_r = 0; bb = 7; continue; }
            7 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; bb = 8; continue; }
            8 => { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1424b01b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x90; const P2: u64 = 0x20; const P3: u64 = 0x28;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rsi = rcx; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rdi); bb = 2; continue; }
            2 => { rdx = (crate::simd_ops::ru64(rax)) as u64; rcx = rax; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rdx.wrapping_add(P1))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(rsi.wrapping_add(P0))) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { crate::simd_ops::wu64(rsi.wrapping_add(P0), rax); bb = 2; continue; }
            5 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(P3))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { crate::simd_ops::wu64(rsi.wrapping_add(P0), rax); bb = 2; continue; }
            7 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rsi)) as u64; rdi = rax; rdx = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru64(rdi)) as u64; rcx = rdi; rbx = ((rax as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 10; continue; } bb = 8; continue; }
            8 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(P3))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 4; continue; } bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rcx = rdi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rsi)) as u64; rdi = rax; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru64(rdi)) as u64; rcx = rdi; rbx = ((rax as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 8; continue; } bb = 10; continue; }
            10 => { crate::simd_ops::wu64(rsi.wrapping_add(P0), (0x0u64) as u64); bb = 11; continue; }
            11 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; bb = 12; continue; }
            12 => { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1424b0280(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x20; const P1: u64 = 0x8; const P2: u64 = 0x1413d8b90; const P3: u64 = 0x1424b01b0;
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    crate::simd_ops::wu64(rcx, rdx);
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(P1), rax);
    rcx = ((rcx).wrapping_add(0x10u64)) as u64;
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    rcx = rbx;
    rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp);
    rax = rbx;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1424b02c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    fl_l = rcx; fl_r = rcx;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { return rax; }
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    loop { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64; fl_l = rbx; fl_r = rbx; if !(((((fl_l as u64)) & ((fl_r as u64))) != 0)) { break; } }
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1424b02f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143a083f0; const P1: u64 = 0x8; const P2: u64 = 0x18; const P3: u64 = 0x20; const P4: u64 = 0x1413d8bf0; const P5: u64 = 0x28; const P6: u64 = 0x88;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rax = (crate::simd_ops::ru64(P0)) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; rbx = (crate::simd_ops::ru64(rcx)) as u64; fl_l = rbx; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 8; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rdi); bb = 2; continue; }
            2 => { rdi = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64; rcx = rdi; rax = (crate::simd_ops::ru64(rdi)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rdx = (crate::simd_ops::ru64(rax)) as u64; rcx = rax; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rdx.wrapping_add(P1))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(P0)) as u64; r8 = rsp.wrapping_add(0x50); rdx = rsp.wrapping_add(0x20); crate::simd_ops::wu32(rsp.wrapping_add(0x50u64), ((rax as u32)) as u32); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rax)) as u64; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64; rax = (crate::simd_ops::ru64(rdx.wrapping_add(P3))) as u64; crate::simd_ops::wu64(rdi.wrapping_add(P5), rax); crate::simd_ops::wu64(rdx.wrapping_add(P3), rdi); bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rcx = rdi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P6))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rdx = (crate::simd_ops::ru64(rax)) as u64; rcx = rax; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rdx.wrapping_add(P1))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(P0)) as u64; r8 = rsp.wrapping_add(0x58); rdx = rsp.wrapping_add(0x30); crate::simd_ops::wu32(rsp.wrapping_add(0x58u64), ((rax as u32)) as u32); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rax)) as u64; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64; rax = (crate::simd_ops::ru64(rdx.wrapping_add(P2))) as u64; crate::simd_ops::wu64(rdi.wrapping_add(P2), rax); crate::simd_ops::wu64(rdx.wrapping_add(P2), rdi); bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(P0)) as u64; rbx = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rbx; fl_r = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 7; continue; }
            7 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; bb = 8; continue; }
            8 => { rsp = ((rsp).wrapping_add(0x40u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1424b08c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142645790; const P1: u64 = 0x143871788; const P2: u64 = 0x143871780; const P3: u64 = 0x1426455a8; const P4: u64 = 0x14387178c; const P5: u64 = 0x14397d8de; const P6: u64 = 0x143a083d8; const P7: u64 = 0x10; const P8: u64 = 0x143a083d0; const P9: u64 = 0x14397d8df; const P10: u64 = 0x142645598;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rcx; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru32(P1)) as u32 as u64; rdi = ((rax as u32)) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = P2; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu32(P1, ((rdi as u32)) as u32); bb = 2; continue; }
            2 => { crate::simd_ops::lk_add_u32(P4, (1) as u32); fl_l = (crate::simd_ops::ru8(P5)) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(P6)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::wu64(rbx.wrapping_add(P7), rax); crate::simd_ops::wu64(rcx, rbx); bb = 5; continue; }
            4 => { rax = (crate::simd_ops::ru64(P8)) as u64; crate::simd_ops::wu64(rbx.wrapping_add(P7), rax); crate::simd_ops::wu64(P8, rbx); bb = 5; continue; }
            5 => { fl_l = (crate::simd_ops::ru32(P4)) as u32 as u64; fl_r = 0x1; crate::simd_ops::wu32(P4, ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32); crate::simd_ops::wu8(P9, (0x1u8) as u8); if (((fl_l as u32)) != ((fl_r as u32))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rax = 0xffffffff; rcx = P2; let tmp_xchg: u32 = crate::simd_ops::atomic_xchg_u32(P1, ((rax as u32)) as u32); rax = (tmp_xchg) as u32 as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P10), rax), rax, rcx, rdx, r8, r9, rsp); bb = 7; continue; }
            7 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1424b0960(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143a083e0; const P1: u64 = 0x18; const P2: u64 = 0x1401e5380; const P3: u64 = 0x8; const P4: u64 = 0x14028cc60; const P5: u64 = 0x142442de0; const P6: u64 = 0x143a083f0; const P7: u64 = 0x1401e66f0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = (crate::simd_ops::ru64(P0)) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = ((rbx.wrapping_add(P1)) as u64) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = ((rbx.wrapping_add(P3)) as u64) as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            2 => { rdi = (crate::simd_ops::ru64(P6)) as u64; rsi = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(P0, rsi); fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 3; continue; }
            3 => { rcx = ((rdi.wrapping_add(P1)) as u64) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P3))) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P3))) as u64; crate::simd_ops::wu64(rax, rsi); rdx = (crate::simd_ops::ru64(rcx)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rbx = (crate::simd_ops::ru64(rdx)) as u64; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rdx = rbx; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 4; continue; } bb = 5; continue; }
            5 => { rdx = (crate::simd_ops::ru64(rdi.wrapping_add(P3))) as u64; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rcx = rdi; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            6 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu64(P6, rsi); rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1424b0a10(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143a083e0; const P1: u64 = 0x1413d8950; const P2: u64 = 0x1413d8a30; const P3: u64 = 0x143a083f0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    fl_l = (crate::simd_ops::ru64(P0)) as u64; fl_r = (0x0u64) as u64;
    if !((((fl_l as u64)) != ((fl_r as u64)))) { rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(P0, rax); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(P3, rax); }
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1424b0a50(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143871780; const P1: u64 = 0x142645790; const P2: u64 = 0x143871788; const P3: u64 = 0x1426455a8; const P4: u64 = 0x14387178c; const P5: u64 = 0x1424b06b0; const P6: u64 = 0x8; const P7: u64 = 0x1424b14c0; const P8: u64 = 0x18; const P9: u64 = 0xc; const P10: u64 = 0x1401e6d50; const P11: u64 = 0x142645598;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rdx; rsi = rcx; rbp = P0; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbp); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); rdi = ((rax as u32)) as u32 as u64; rdx = (crate::simd_ops::ru32(P2)) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = rbp; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu32(P2, ((rdi as u32)) as u32); bb = 2; continue; }
            2 => { crate::simd_ops::lk_add_u32(P4, (1) as u32); rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rsi)) as u64; rcx = rsi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P6))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = ((rax as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rdi = (crate::simd_ops::ru64(rax.wrapping_add(P8))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 3; continue; }
            3 => { bb = 4; continue; }
            4 => { rdx = (crate::simd_ops::ru32(rbx.wrapping_add(P6))) as u32 as u64; rcx = (crate::simd_ops::ru32(rbx.wrapping_add(P9))) as u32 as u64; rcx = ((rcx).wrapping_shl(3u32)) as u64; rax = ((rdx.wrapping_mul(8)) as u64) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rdx = ((rdx).wrapping_add(1)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru32(rbx.wrapping_add(P6))) as u32 as u64; rax = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu64(rax.wrapping_add(rcx.wrapping_mul(8)), rdi); crate::simd_ops::lk_add_u32(rbx.wrapping_add(P6), (1) as u32); rdi = (crate::simd_ops::ru64(rdi.wrapping_add(P8))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 4; continue; } bb = 7; continue; }
            7 => { fl_l = (crate::simd_ops::ru32(P4)) as u32 as u64; fl_r = 0x1; crate::simd_ops::wu32(P4, ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32); if (((fl_l as u32)) != ((fl_r as u32))) { bb = 9; continue; } bb = 8; continue; }
            8 => { rax = 0xffffffff; let tmp_xchg: u32 = crate::simd_ops::atomic_xchg_u32(P2, ((rax as u32)) as u32); rax = (tmp_xchg) as u32 as u64; rcx = rbp; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P11), rax), rax, rcx, rdx, r8, r9, rsp); bb = 9; continue; }
            9 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1424b0b40(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x1424b0110; const P2: u64 = 0x1424b01b0;
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rbx = rcx;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rcx = rbx;
    crate::simd_ops::wu64(rbx.wrapping_add(P0), rax);
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    rax = rbx;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1424b0b80(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x1413d8b90;
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rax = (crate::simd_ops::ru64(rdx)) as u64;
    rbx = rcx;
    crate::simd_ops::wu64(rcx, rax);
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(P0), rax);
    rcx = ((rcx).wrapping_add(0x10u64)) as u64;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rax = rbx;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1424b0bb0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1424b06b0; const P1: u64 = 0x8; const P2: u64 = 0x1424b14c0; const P3: u64 = 0x20; const P4: u64 = 0x1413d8b90; const P5: u64 = 0x1424b01b0; const P6: u64 = 0x10; const P7: u64 = 0xc; const P8: u64 = 0x142645598;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x50u64)) as u64; rbx = rdx; rdi = rcx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = ((rax as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); rax = (crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); rcx = rsp.wrapping_add(0x38); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rcx = rsp.wrapping_add(0x28); rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; crate::simd_ops::wu64(rdi, rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu64(rdi.wrapping_add(P1), rax); rcx = ((rdi.wrapping_add(P6)) as u64) as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru8(rsp.wrapping_add(0x40u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 3; continue; } bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; fl_l = (crate::simd_ops::lk_add_u32(rcx.wrapping_add(P7), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rax = 0xffffffff; let tmp_xchg: u32 = crate::simd_ops::atomic_xchg_u32(rcx.wrapping_add(P1), ((rax as u32)) as u32); rax = (tmp_xchg) as u32 as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P8), rax), rax, rcx, rdx, r8, r9, rsp); bb = 3; continue; }
            3 => { rax = rdi; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsp = ((rsp).wrapping_add(0x50u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1424b0c50(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1424b0e40; const P1: u64 = 0x8;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = ((rsp).wrapping_sub(0x38u64)) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rcx);
    rax = (0xffffffffffffffffu64) as u64;
    loop { rax = ((rax).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(rax))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if !((((fl_l as u8)) != ((fl_r as u8)))) { break; } }
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax);
    rcx = rsp.wrapping_add(0x20);
    xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128;
    crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128);
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    fl_l = rax; fl_r = rax;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rdx = (crate::simd_ops::ru64(rax)) as u64; rcx = rax; rsp = ((rsp).wrapping_add(0x38u64)) as u64; return crate::simd_ops::tail_stk_xmm(({ let t = (crate::simd_ops::ru64(rdx.wrapping_add(P1))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); }
    rsp = ((rsp).wrapping_add(0x38u64)) as u64;
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1424b0cb0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143871780; const P1: u64 = 0x142645790; const P2: u64 = 0x143871788; const P3: u64 = 0x1426455a8; const P4: u64 = 0x14387178c; const P5: u64 = 0x1424b06b0; const P6: u64 = 0x8; const P7: u64 = 0x143881f9c; const P8: u64 = 0x20; const P9: u64 = 0x142645598;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; r14 = rdx; rdi = rcx; rbp = P0; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rbp); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); rbx = ((rax as u32)) as u32 as u64; r8 = (crate::simd_ops::ru32(P2)) as u32 as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = rbp; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu32(P2, ((rbx as u32)) as u32); bb = 2; continue; }
            2 => { crate::simd_ops::lk_add_u32(P4, (1) as u32); rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rdi)) as u64; rcx = rdi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P6))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rdi = ((rax as u32)) as u32 as u64; rsi = (crate::simd_ops::ru32(P7)) as u32 as u64; rdx = (crate::simd_ops::ru64(r14)) as u64; rcx = r14; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rdx.wrapping_add(P8))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rbx = rax; rdx = (crate::simd_ops::ru64(rax)) as u64; rcx = rax; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rdx.wrapping_add(P6))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 6; continue; } bb = 3; continue; }
            3 => { bb = 4; continue; }
            4 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 10; continue; } bb = 5; continue; }
            5 => { rdx = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rdx.wrapping_add(P8))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rbx = rax; rax = (crate::simd_ops::ru64(rax)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P6))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 4; continue; } bb = 6; continue; }
            6 => { rbx = (rbx & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 7; continue; }
            7 => { fl_l = (crate::simd_ops::ru32(P4)) as u32 as u64; fl_r = 0x1; crate::simd_ops::wu32(P4, ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32); if (((fl_l as u32)) != ((fl_r as u32))) { bb = 9; continue; } bb = 8; continue; }
            8 => { rdx = 0xffffffff; let tmp_xchg: u32 = crate::simd_ops::atomic_xchg_u32(P2, ((rdx as u32)) as u32); rdx = (tmp_xchg) as u32 as u64; rcx = rbp; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P9), rax), rax, rcx, rdx, r8, r9, rsp); bb = 9; continue; }
            9 => { rax = (((rbx as u8)) as u32) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            10 => { rbx = (rbx & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 7; continue; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1424b0e40(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143871780; const P1: u64 = 0x142645790; const P2: u64 = 0x143871788; const P3: u64 = 0x1426455a8; const P4: u64 = 0x14387178c; const P5: u64 = 0x1424b06b0; const P6: u64 = 0x142409b50; const P7: u64 = 0x143a083e0; const P8: u64 = 0x1401e6ff0; const P9: u64 = 0x14240ab10; const P10: u64 = 0x142645598;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 35>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0xd8u64)) as u64; rdi = rcx; rbp = P0; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rbp); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = ((rax as u32)) as u32 as u64; rdx = (crate::simd_ops::ru32(P2)) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = rbp; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(P2, ((rbx as u32)) as u32); bb = 2; continue; }
            2 => { crate::simd_ops::lk_add_u32(P4, (1) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), (0x10000010u64) as u64); r12 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu8(rsp.wrapping_add(0x48u64), ((r12 as u8)) as u8); xmm0 = (crate::simd_ops::ru128(rdi)) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128); r9 = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; r8 = 0; fl_l = 0; fl_r = 0; rdx = rsp.wrapping_add(0x30); rcx = rsp.wrapping_add(0x40); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = (crate::simd_ops::ru32(rsp.wrapping_add(0x44u64))) as u32 as u64; rcx = (crate::simd_ops::ru32(rsp.wrapping_add(0x40u64))) as u32 as u64; rax = ((rcx as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shr(24u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = ((rax.wrapping_mul(8)) as u32) as u32 as u64; bb = 5; continue; }
            4 => { rax = (((rax as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x7c00; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 5; continue; }
            5 => { rcx = ((((rcx as u32)) & (0xffffffu32))) as u32 as u64; rcx = (((rcx as u32)).wrapping_shl(3u32)) as u32 as u64; r15 = rsp.wrapping_add(0x48); fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; r15 = (if (((fl_l as u32)) > ((fl_r as u32))) { crate::simd_ops::ru64(rsp.wrapping_add(0x48u64)) } else { r15 }) as u64; fl_l = rbx; fl_r = (0x20u64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 16; continue; } bb = 6; continue; }
            6 => { fl_l = rbx; fl_r = (0x10u64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 15; continue; } bb = 7; continue; }
            7 => { fl_l = rbx; fl_r = (0x8u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 10; continue; } bb = 8; continue; }
            8 => { r8 = (0x9ae16a3b2f90404fu64) as u64; r8 = ((r8.wrapping_add(rbx.wrapping_mul(2))) as u64) as u64; rdx = (crate::simd_ops::ru64(r15)) as u64; rax = (0x651e95c4d06fbfb1u64) as u64; rdx = ((rdx).wrapping_sub(rax)) as u64; rcx = (crate::simd_ops::ru64(rbx.wrapping_add(r15).wrapping_sub(0x8u64))) as u64; rcx = ((rcx).rotate_right(37u32)) as u64; rcx = (((rcx) as i64).wrapping_mul((r8) as i64) as u64) as u64; rcx = ((rcx).wrapping_add(rdx)) as u64; rdx = ((rdx).rotate_right(25u32)) as u64; rdx = ((rdx).wrapping_add(crate::simd_ops::ru64(rbx.wrapping_add(r15).wrapping_sub(0x8u64)))) as u64; rdx = (((rdx) as i64).wrapping_mul((r8) as i64) as u64) as u64; rax = rdx; fl_l = (((rax) ^ (rcx))) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; bb = 9; continue; }
            9 => { rax = (((rax) as i64).wrapping_mul((r8) as i64) as u64) as u64; rcx = rax; rcx = ((rcx).wrapping_shr(47u32)) as u64; rax = (((rax) ^ (rcx))) as u64; rax = (((rax) ^ (rdx))) as u64; rax = (((rax) as i64).wrapping_mul((r8) as i64) as u64) as u64; rcx = rax; rcx = ((rcx).wrapping_shr(47u32)) as u64; rax = (((rax) ^ (rcx))) as u64; rax = (((rax) as i64).wrapping_mul((r8) as i64) as u64) as u64; bb = 22; continue; }
            10 => { fl_l = rbx; fl_r = (0x4u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 12; continue; } bb = 11; continue; }
            11 => { r8 = (0x9ae16a3b2f90404fu64) as u64; r8 = ((r8.wrapping_add(rbx.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru32(r15)) as u32 as u64; rdx = (crate::simd_ops::ru32(rbx.wrapping_add(r15).wrapping_sub(0x4u64))) as u32 as u64; rax = ((rbx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = (((rax) ^ (rdx))) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; bb = 9; continue; }
            12 => { fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 13; continue; }
            13 => { rax = rbx; rax = ((rax).wrapping_shr(1u32)) as u64; rdx = ((crate::simd_ops::ru8(rax.wrapping_add(r15))) as u32) as u32 as u64; rax = (0xe16a3b2f90404f00u64) as u64; rdx = (((rdx) as i64).wrapping_mul((rax) as i64) as u64) as u64; rax = ((crate::simd_ops::ru8(r15)) as u32) as u32 as u64; r8 = (0x9ae16a3b2f90404fu64) as u64; rax = (((rax) as i64).wrapping_mul((r8) as i64) as u64) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; rax = ((crate::simd_ops::ru8(rbx.wrapping_add(r15).wrapping_sub(0x1u64))) as u32) as u32 as u64; rax = ((rbx.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax as u32)) as u32 as u64; rcx = (0xc3a5c85c97cb3127u64) as u64; rax = (((rax) as i64).wrapping_mul((rcx) as i64) as u64) as u64; rdx = (((rdx) ^ (rax))) as u64; rax = rdx; rax = ((rax).wrapping_shr(47u32)) as u64; rax = (((rax) ^ (rdx))) as u64; rax = (((rax) as i64).wrapping_mul((r8) as i64) as u64) as u64; bb = 22; continue; }
            14 => { rax = (0x9ae16a3b2f90404fu64) as u64; bb = 22; continue; }
            15 => { r8 = (0x9ae16a3b2f90404fu64) as u64; r11 = ((r8.wrapping_add(rbx.wrapping_mul(2))) as u64) as u64; r13 = (0xb492b66fbe98f273u64) as u64; r9 = (crate::simd_ops::ru64(r15)) as u64; r9 = (((r9) as i64).wrapping_mul((r13) as i64) as u64) as u64; rdx = (crate::simd_ops::ru64(r15.wrapping_add(0x8u64))) as u64; r10 = r11; r10 = (((r10) as i64).wrapping_mul((crate::simd_ops::ru64(rbx.wrapping_add(r15).wrapping_sub(0x8u64))) as i64) as u64) as u64; rcx = (crate::simd_ops::ru64(rbx.wrapping_add(r15).wrapping_sub(0x10u64))) as u64; rcx = (((rcx) as i64).wrapping_mul((r8) as i64) as u64) as u64; r8 = ((r8).wrapping_add(rdx)) as u64; r8 = ((r8).rotate_right(18u32)) as u64; r8 = ((r8).wrapping_add(r10)) as u64; r8 = ((r8).wrapping_add(r9)) as u64; rax = ((rdx.wrapping_add(r9)) as u64) as u64; rax = ((rax).rotate_right(43u32)) as u64; r10 = ((r10).rotate_right(30u32)) as u64; rax = ((rax).wrapping_add(rcx)) as u64; rax = ((rax).wrapping_add(r10)) as u64; rax = (((rax) ^ (r8))) as u64; rax = (((rax) as i64).wrapping_mul((r11) as i64) as u64) as u64; rcx = rax; rcx = ((rcx).wrapping_shr(47u32)) as u64; rax = (((rax) ^ (rcx))) as u64; rax = (((rax) ^ (r8))) as u64; rax = (((rax) as i64).wrapping_mul((r11) as i64) as u64) as u64; rcx = rax; rcx = ((rcx).wrapping_shr(47u32)) as u64; rax = (((rax) ^ (rcx))) as u64; rax = (((rax) as i64).wrapping_mul((r11) as i64) as u64) as u64; bb = 22; continue; }
            16 => { r13 = ((r15.wrapping_add(rbx)) as u64) as u64; fl_l = rbx; fl_r = (0x40u64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 18; continue; } bb = 17; continue; }
            17 => { r8 = (0x9ae16a3b2f90404fu64) as u64; r14 = ((r8.wrapping_add(rbx.wrapping_mul(2))) as u64) as u64; r11 = (crate::simd_ops::ru64(r13.wrapping_sub(0x18u64))) as u64; r10 = (crate::simd_ops::ru64(r15.wrapping_add(0x10u64))) as u64; r10 = (((r10) as i64).wrapping_mul((r8) as i64) as u64) as u64; rax = (crate::simd_ops::ru64(r15.wrapping_add(0x18u64))) as u64; r9 = ((rax.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rdi = r14; rdi = (((rdi) as i64).wrapping_mul((crate::simd_ops::ru64(r13.wrapping_sub(0x10u64))) as i64) as u64) as u64; rdx = (crate::simd_ops::ru64(r15)) as u64; rdx = (((rdx) as i64).wrapping_mul((r8) as i64) as u64) as u64; rdx = ((rdx).wrapping_add(crate::simd_ops::ru64(r13.wrapping_sub(0x8u64)))) as u64; rax = (crate::simd_ops::ru64(r15.wrapping_add(0x8u64))) as u64; rax = ((rax).rotate_right(30u32)) as u64; rax = ((rax).wrapping_add(r11)) as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rax = rdx; rax = ((rax).rotate_right(43u32)) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; r8 = (crate::simd_ops::ru64(r13.wrapping_sub(0x20u64))) as u64; r8 = (((r8) ^ (rdx))) as u64; r8 = ((r8).wrapping_add(1)) as u64; r8 = ((r8).wrapping_add(r9)) as u64; rdx = ((r8.wrapping_add(rcx)) as u64) as u64; rdx = (((rdx) as i64).wrapping_mul((r14) as i64) as u64) as u64; rdx = ((rdx).swap_bytes()) as u64; rdx = ((rdx).wrapping_add(rdi)) as u64; rax = ((r9.wrapping_add(r10)) as u64) as u64; r9 = rax; r9 = ((r9).rotate_right(42u32)) as u64; r9 = ((r9).wrapping_add(r11)) as u64; rcx = ((rax.wrapping_add(r11)) as u64) as u64; rax = ((rdx.wrapping_add(r8)) as u64) as u64; rax = (((rax) as i64).wrapping_mul((r14) as i64) as u64) as u64; rax = ((rax).swap_bytes()) as u64; rax = ((rax).wrapping_add(rcx)) as u64; rax = ((rax).wrapping_add(r9)) as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(r13.wrapping_sub(0x8u64)))) as u64; rax = (((rax) as i64).wrapping_mul((r14) as i64) as u64) as u64; rax = ((rax).swap_bytes()) as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(r15.wrapping_add(0x8u64)))) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; rcx = (((rcx) as i64).wrapping_mul((r14) as i64) as u64) as u64; rcx = ((rcx).wrapping_add(rdi)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(r13.wrapping_sub(0x20u64)))) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rax = (((rax) ^ (rcx))) as u64; rax = (((rax) as i64).wrapping_mul((r14) as i64) as u64) as u64; fl_l = ((rax).wrapping_add(r9)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; bb = 21; continue; }
            18 => { rdi = (crate::simd_ops::ru64(r13.wrapping_sub(0x28u64))) as u64; r14 = (crate::simd_ops::ru64(r13.wrapping_sub(0x10u64))) as u64; r14 = ((r14).wrapping_add(crate::simd_ops::ru64(r13.wrapping_sub(0x38u64)))) as u64; rbp = (crate::simd_ops::ru64(r13.wrapping_sub(0x30u64))) as u64; rbp = ((rbp).wrapping_add(rbx)) as u64; rbp = (((rbp) ^ (crate::simd_ops::ru64(r13.wrapping_sub(0x18u64))))) as u64; rdx = (0x9ddfea08eb382d69u64) as u64; rbp = (((rbp) as i64).wrapping_mul((rdx) as i64) as u64) as u64; rax = rbp; rax = ((rax).wrapping_shr(47u32)) as u64; rbp = (((rbp) ^ (rax))) as u64; rbp = (((rbp) ^ (crate::simd_ops::ru64(r13.wrapping_sub(0x18u64))))) as u64; rbp = (((rbp) as i64).wrapping_mul((rdx) as i64) as u64) as u64; rax = rbp; rax = ((rax).wrapping_shr(47u32)) as u64; rbp = (((rbp) ^ (rax))) as u64; rbp = (((rbp) as i64).wrapping_mul((rdx) as i64) as u64) as u64; rax = (crate::simd_ops::ru64(r13.wrapping_sub(0x30u64))) as u64; r10 = (crate::simd_ops::ru64(r13.wrapping_sub(0x40u64))) as u64; r10 = ((r10).wrapping_add(rbx)) as u64; r9 = ((r10.wrapping_add(rdi)) as u64) as u64; r9 = ((r9).wrapping_add(rbp)) as u64; r8 = r10; rax = ((rax).wrapping_add(crate::simd_ops::ru64(r13.wrapping_sub(0x38u64)))) as u64; r10 = ((r10).wrapping_add(rax)) as u64; r9 = ((r9).rotate_right(21u32)) as u64; rax = r10; rax = ((rax).rotate_right(44u32)) as u64; rax = ((rax).wrapping_add(r9)) as u64; rax = ((rax).wrapping_add(r8)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x120u64), rax); rcx = ((r10.wrapping_add(rdi)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x130u64), rcx); r11 = (crate::simd_ops::ru64(r13.wrapping_sub(0x8u64))) as u64; rax = (crate::simd_ops::ru64(r13.wrapping_sub(0x10u64))) as u64; rcx = (crate::simd_ops::ru64(r13.wrapping_sub(0x18u64))) as u64; r10 = (crate::simd_ops::ru64(r13.wrapping_sub(0x20u64))) as u64; r13 = (0xb492b66fbe98f273u64) as u64; r10 = ((r10).wrapping_add(r13)) as u64; r10 = ((r10).wrapping_add(r14)) as u64; r9 = ((r11.wrapping_add(r10)) as u64) as u64; r9 = ((r9).wrapping_add(rdi)) as u64; r12 = r10; rax = ((rax).wrapping_add(rcx)) as u64; r10 = ((r10).wrapping_add(rax)) as u64; r9 = ((r9).rotate_right(21u32)) as u64; rax = r10; rax = ((rax).rotate_right(44u32)) as u64; r9 = ((r9).wrapping_add(rax)) as u64; r12 = ((r12).wrapping_add(r9)) as u64; r8 = ((r11.wrapping_add(r10)) as u64) as u64; rdi = (((rdi) as i64).wrapping_mul((r13) as i64) as u64) as u64; rdi = ((rdi).wrapping_add(crate::simd_ops::ru64(r15))) as u64; rbx = ((rbx).wrapping_sub(1)) as u64; fl_l = (((rbx) & (0xffffffffffffffc0u64))) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x138u64), rbx); rdx = r15.wrapping_add(0x30); crate::simd_ops::wu64(rsp.wrapping_add(0x128u64), rdx); r15 = rdi; bb = 19; continue; }
            19 => { r11 = (crate::simd_ops::ru64(rdx.wrapping_sub(0x28u64))) as u64; rsi = ((r11.wrapping_add(r14)) as u64) as u64; rsi = ((rsi).wrapping_add(r15)) as u64; rsi = ((rsi).wrapping_add(crate::simd_ops::ru64(rsp.wrapping_add(0x130u64)))) as u64; rdi = (crate::simd_ops::ru64(rdx)) as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x120u64))) as u64; rax = ((rax).wrapping_add(rdi)) as u64; r14 = ((r14).wrapping_add(rax)) as u64; rsi = ((rsi).rotate_right(37u32)) as u64; rsi = (((rsi) as i64).wrapping_mul((r13) as i64) as u64) as u64; rsi = (((rsi) ^ (r12))) as u64; rbx = (crate::simd_ops::ru64(rdx.wrapping_sub(0x8u64))) as u64; r14 = ((r14).rotate_right(42u32)) as u64; r14 = (((r14) as i64).wrapping_mul((r13) as i64) as u64) as u64; r14 = ((r14).wrapping_add(rbx)) as u64; r14 = ((r14).wrapping_add(crate::simd_ops::ru64(rsp.wrapping_add(0x130u64)))) as u64; r15 = ((r8.wrapping_add(rbp)) as u64) as u64; r15 = ((r15).rotate_right(33u32)) as u64; r15 = (((r15) as i64).wrapping_mul((r13) as i64) as u64) as u64; rcx = ((r8.wrapping_add(rsi)) as u64) as u64; r8 = (crate::simd_ops::ru64(rsp.wrapping_add(0x120u64))) as u64; r8 = (((r8) as i64).wrapping_mul((r13) as i64) as u64) as u64; r9 = (crate::simd_ops::ru64(rdx.wrapping_sub(0x18u64))) as u64; r10 = (crate::simd_ops::ru64(rdx.wrapping_sub(0x20u64))) as u64; r8 = ((r8).wrapping_add(crate::simd_ops::ru64(rdx.wrapping_sub(0x30u64)))) as u64; rdx = ((r9.wrapping_add(r8)) as u64) as u64; rdx = ((rdx).wrapping_add(rcx)) as u64; rcx = r8; rax = ((r10.wrapping_add(r11)) as u64) as u64; r8 = ((r8).wrapping_add(rax)) as u64; rdx = ((rdx).rotate_right(21u32)) as u64; r11 = r8; r11 = ((r11).rotate_right(44u32)) as u64; r11 = ((r11).wrapping_add(rdx)) as u64; r11 = ((r11).wrapping_add(rcx)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x120u64), r11); rax = ((r9.wrapping_add(r8)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x130u64), rax); rdx = ((r10.wrapping_add(r14)) as u64) as u64; r9 = ((r12.wrapping_add(r15)) as u64) as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x128u64))) as u64; r10 = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; r9 = ((r9).wrapping_add(crate::simd_ops::ru64(rax.wrapping_sub(0x10u64)))) as u64; r8 = ((r10.wrapping_add(r9)) as u64) as u64; r8 = ((r8).wrapping_add(rdx)) as u64; r12 = r9; rax = ((rdi.wrapping_add(rbx)) as u64) as u64; r9 = ((r9).wrapping_add(rax)) as u64; r8 = ((r8).rotate_right(21u32)) as u64; rax = r9; rax = ((rax).rotate_right(44u32)) as u64; r8 = ((r8).wrapping_add(rax)) as u64; r12 = ((r12).wrapping_add(r8)) as u64; r8 = ((r10.wrapping_add(r9)) as u64) as u64; rbp = rsi; rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x128u64))) as u64; rdx = ((rdx).wrapping_add(0x40u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x128u64), rdx); fl_l = (crate::simd_ops::ru64(rsp.wrapping_add(0x138u64))) as u64; fl_r = (0x40u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x138u64), ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64); if (((fl_l as u64)) != ((fl_r as u64))) { bb = 19; continue; } bb = 20; continue; }
            20 => { rcx = r12; rcx = (((rcx) ^ (r11))) as u64; r9 = (0x9ddfea08eb382d69u64) as u64; rcx = (((rcx) as i64).wrapping_mul((r9) as i64) as u64) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; r12 = (((r12) ^ (rcx))) as u64; r12 = (((r12) as i64).wrapping_mul((r9) as i64) as u64) as u64; rax = r12; rax = ((rax).wrapping_shr(47u32)) as u64; rax = (((rax) ^ (r12))) as u64; rax = (((rax) as i64).wrapping_mul((r9) as i64) as u64) as u64; rax = ((rax).wrapping_add(r15)) as u64; rdx = r8; rdx = (((rdx) ^ (crate::simd_ops::ru64(rsp.wrapping_add(0x130u64))))) as u64; rdx = (((rdx) as i64).wrapping_mul((r9) as i64) as u64) as u64; rcx = rdx; rcx = ((rcx).wrapping_shr(47u32)) as u64; rdx = (((rdx) ^ (rcx))) as u64; r8 = (((r8) ^ (rdx))) as u64; r8 = (((r8) as i64).wrapping_mul((r9) as i64) as u64) as u64; rcx = r8; rcx = ((rcx).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (r8))) as u64; rcx = (((rcx) as i64).wrapping_mul((r9) as i64) as u64) as u64; rdx = r14; rdx = ((rdx).wrapping_shr(47u32)) as u64; rdx = (((rdx) ^ (r14))) as u64; rdx = (((rdx) as i64).wrapping_mul((r13) as i64) as u64) as u64; rdx = ((rdx).wrapping_add(rcx)) as u64; rdx = ((rdx).wrapping_add(rsi)) as u64; rdx = (((rdx) ^ (rax))) as u64; rdx = (((rdx) as i64).wrapping_mul((r9) as i64) as u64) as u64; rcx = rdx; rcx = ((rcx).wrapping_shr(47u32)) as u64; rdx = (((rdx) ^ (rcx))) as u64; rax = (((rax) ^ (rdx))) as u64; rax = (((rax) as i64).wrapping_mul((r9) as i64) as u64) as u64; rcx = rax; rcx = ((rcx).wrapping_shr(47u32)) as u64; rax = (((rax) ^ (rcx))) as u64; rax = (((rax) as i64).wrapping_mul((r9) as i64) as u64) as u64; r12 = 0; fl_l = 0; fl_r = 0; bb = 21; continue; }
            21 => { rbp = P0; bb = 22; continue; }
            22 => { rdx = (crate::simd_ops::ru64(P7)) as u64; rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x30u64))) as u64; rcx = (((rcx) & (rax))) as u64; rcx = ((rcx).wrapping_add(rcx)) as u64; rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x18u64))) as u64; rbx = (crate::simd_ops::ru64(rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x8u64))) as u64; rsi = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; fl_l = rbx; fl_r = rsi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 28; continue; } bb = 23; continue; }
            23 => { rdi = (crate::simd_ops::ru64(rax.wrapping_add(rcx.wrapping_mul(8)))) as u64; rdx = rbx.wrapping_add(0x10); rcx = rsp.wrapping_add(0x40); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 29; continue; } bb = 24; continue; }
            24 => { bb = 25; continue; }
            25 => { fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 28; continue; } bb = 26; continue; }
            26 => { rbx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rdx = rbx.wrapping_add(0x10); rcx = rsp.wrapping_add(0x40); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 25; continue; } bb = 27; continue; }
            27 => { bb = 29; continue; }
            28 => { rbx = r12; bb = 29; continue; }
            29 => { fl_l = rbx; fl_r = rbx; rbx = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rsi } else { rbx }) as u64; rcx = rsp.wrapping_add(0x40); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(P7)) as u64; fl_l = rbx; fl_r = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 31; continue; } bb = 30; continue; }
            30 => { r12 = (crate::simd_ops::ru64(rbx.wrapping_add(0x98u64))) as u64; bb = 31; continue; }
            31 => { fl_l = (crate::simd_ops::ru32(P4)) as u32 as u64; fl_r = 0x1; crate::simd_ops::wu32(P4, ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32); if (((fl_l as u32)) != ((fl_r as u32))) { bb = 33; continue; } bb = 32; continue; }
            32 => { rdx = 0xffffffff; let tmp_xchg: u32 = crate::simd_ops::atomic_xchg_u32(P2, ((rdx as u32)) as u32); rdx = (tmp_xchg) as u32 as u64; rcx = rbp; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P10), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 33; continue; }
            33 => { rax = r12; rsp = ((rsp).wrapping_add(0xd8u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1424b15e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1424b16a0;
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = r9;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rdx = (0x9ddfea08eb382d69u64) as u64;
    rcx = (0x651e95c4d06fbfb1u64) as u64;
    rax = ((rax).wrapping_add(rcx)) as u64;
    rax = (((rax) ^ (rbx))) as u64;
    rax = (((rax) as i64).wrapping_mul((rdx) as i64) as u64) as u64;
    rcx = rax;
    rcx = ((rcx).wrapping_shr(47u32)) as u64;
    rcx = (((rcx) ^ (rax))) as u64;
    rcx = (((rcx) ^ (rbx))) as u64;
    rcx = (((rcx) as i64).wrapping_mul((rdx) as i64) as u64) as u64;
    rax = rcx;
    rax = ((rax).wrapping_shr(47u32)) as u64;
    rax = (((rax) ^ (rcx))) as u64;
    rax = (((rax) as i64).wrapping_mul((rdx) as i64) as u64) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1424b1ac0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xffffffffffffffe0; const P1: u64 = 0xffffffffffffffe8; const P2: u64 = 0xfffffffffffffff8; const P3: u64 = 0x8; const P4: u64 = 0x18; const P5: u64 = 0xfffffffffffffff0; const P6: u64 = 0x10; const P7: u64 = 0x1;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 1>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rbp = (crate::simd_ops::ru64(rcx.wrapping_add(rdx).wrapping_add(P0))) as u64; r11 = (0x9ae16a3b2f90404fu64) as u64; r9 = (crate::simd_ops::ru64(rcx.wrapping_add(rdx).wrapping_add(P1))) as u64; r10 = (crate::simd_ops::ru64(rcx.wrapping_add(rdx).wrapping_add(P2))) as u64; rsi = (crate::simd_ops::ru64(rcx.wrapping_add(P3))) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P4))) as u64; r14 = ((r11.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64; rdi = (crate::simd_ops::ru64(rcx)) as u64; rbx = r14; rbx = (((rbx) as i64).wrapping_mul((crate::simd_ops::ru64(rcx.wrapping_add(rdx).wrapping_add(P5))) as i64) as u64) as u64; rdi = (((rdi) as i64).wrapping_mul((r11) as i64) as u64) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P6))) as u64; r8 = ((rax.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rcx = (((rcx) as i64).wrapping_mul((r11) as i64) as u64) as u64; rdi = ((rdi).wrapping_add(r10)) as u64; rdx = ((r8.wrapping_add(P7)) as u64) as u64; rax = rdi; rcx = ((rcx).wrapping_add(r8)) as u64; rdi = ((rdi).rotate_right(43u32)) as u64; rax = (((rax) ^ (rbp))) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; r11 = rcx; r11 = ((r11).rotate_right(42u32)) as u64; r8 = ((rcx.wrapping_add(r9)) as u64) as u64; r11 = ((r11).wrapping_add(r9)) as u64; rcx = rsi; rcx = ((rcx).rotate_right(30u32)) as u64; rcx = ((rcx).wrapping_add(r9)) as u64; rax = ((rdx.wrapping_add(rcx)) as u64) as u64; rax = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; rdi = ((rdi).wrapping_add(rax)) as u64; rdi = (((rdi) as i64).wrapping_mul((r14) as i64) as u64) as u64; rdi = ((rdi).swap_bytes()) as u64; rdi = ((rdi).wrapping_add(rdx)) as u64; rdi = ((rdi).wrapping_add(rbx)) as u64; rdi = (((rdi) as i64).wrapping_mul((r14) as i64) as u64) as u64; rdi = ((rdi).swap_bytes()) as u64; rdi = ((rdi).wrapping_add(r8)) as u64; rdi = ((rdi).wrapping_add(r11)) as u64; rdi = ((rdi).wrapping_add(r10)) as u64; rdi = (((rdi) as i64).wrapping_mul((r14) as i64) as u64) as u64; rdi = ((rdi).swap_bytes()) as u64; rdi = ((rdi).wrapping_add(r8)) as u64; rdi = ((rdi).wrapping_add(rsi)) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rdi = (((rdi) as i64).wrapping_mul((r14) as i64) as u64) as u64; rdi = ((rdi).wrapping_add(rbx)) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x10u64))) as u64; rdi = ((rdi).wrapping_add(rbp)) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x18u64))) as u64; rax = rdi; rax = ((rax).wrapping_shr(47u32)) as u64; rax = (((rax) ^ (rdi))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rax = (((rax) as i64).wrapping_mul((r14) as i64) as u64) as u64; fl_l = ((rax).wrapping_add(r11)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1424b1bc0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x18; const P1: u64 = 0x10; const P2: u64 = 0x8;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    r10 = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    rbx = (crate::simd_ops::ru64(rdx)) as u64;
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P1))) as u64;
    rbx = ((rbx).wrapping_add(r8)) as u64;
    rax = ((rax).wrapping_add(crate::simd_ops::ru64(rdx.wrapping_add(P2)))) as u64;
    r11 = rbx;
    r8 = ((r10.wrapping_add(rbx)) as u64) as u64;
    rbx = ((rbx).wrapping_add(rax)) as u64;
    rax = ((r10.wrapping_add(rbx)) as u64) as u64;
    rbx = ((rbx).rotate_right(44u32)) as u64;
    crate::simd_ops::wu64(rcx, rax);
    rax = ((r8.wrapping_add(r9)) as u64) as u64;
    rax = ((rax).rotate_right(21u32)) as u64;
    rax = ((rax).wrapping_add(r11)) as u64;
    fl_l = ((rbx).wrapping_add(rax)) as u64; fl_r = fl_l;
    rbx = ((fl_l as u64)) as u64;
    rax = rcx;
    crate::simd_ops::wu64(rcx.wrapping_add(P2), rbx);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x8u64))) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1424b1c10(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rax = ((r8.wrapping_add(r9)) as u64) as u64;
    rbx = ((rbx).wrapping_add(rdx)) as u64;
    rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64;
    r11 = rbx;
    r10 = ((rdx.wrapping_add(rbx)) as u64) as u64;
    rbx = ((rbx).wrapping_add(rax)) as u64;
    rax = ((rdx.wrapping_add(rbx)) as u64) as u64;
    rbx = ((rbx).rotate_right(44u32)) as u64;
    crate::simd_ops::wu64(rcx, rax);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    rax = ((rax).wrapping_add(r10)) as u64;
    rax = ((rax).rotate_right(21u32)) as u64;
    rax = ((rax).wrapping_add(r11)) as u64;
    fl_l = ((rbx).wrapping_add(rax)) as u64; fl_r = fl_l;
    rbx = ((fl_l as u64)) as u64;
    rax = rcx;
    crate::simd_ops::wu64(rcx.wrapping_add(P0), rbx);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x8u64))) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1424b1c60(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xfffffffffffffff0; const P2: u64 = 0xfffffffffffffff8;
    let (mut rax, mut r10, mut r11, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdi); r8 = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; r10 = ((rcx.wrapping_add(rdx)) as u64) as u64; r9 = (crate::simd_ops::ru64(rcx)) as u64; rdi = (0x9ae16a3b2f90404fu64) as u64; rax = (0xb492b66fbe98f273u64) as u64; r9 = (((r9) as i64).wrapping_mul((rax) as i64) as u64) as u64; rax = (crate::simd_ops::ru64(r10.wrapping_add(P1))) as u64; rbx = ((rdi.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64; rax = (((rax) as i64).wrapping_mul((rdi) as i64) as u64) as u64; rcx = rbx; rdx = ((r8.wrapping_add(rdi)) as u64) as u64; rcx = (((rcx) as i64).wrapping_mul((crate::simd_ops::ru64(r10.wrapping_add(P2))) as i64) as u64) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x10u64))) as u64; r11 = ((r8.wrapping_add(r9)) as u64) as u64; r11 = ((r11).rotate_right(43u32)) as u64; rdx = ((rdx).rotate_right(18u32)) as u64; rdx = ((rdx).wrapping_add(rcx)) as u64; rcx = ((rcx).rotate_right(30u32)) as u64; r11 = ((r11).wrapping_add(rcx)) as u64; rdx = ((rdx).wrapping_add(r9)) as u64; r11 = ((r11).wrapping_add(rax)) as u64; r11 = (((r11) ^ (rdx))) as u64; r11 = (((r11) as i64).wrapping_mul((rbx) as i64) as u64) as u64; rcx = r11; rcx = ((rcx).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rdx))) as u64; rcx = (((rcx) ^ (r11))) as u64; rcx = (((rcx) as i64).wrapping_mul((rbx) as i64) as u64) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rax = (((rax) ^ (rcx))) as u64; rax = (((rax) as i64).wrapping_mul((rbx) as i64) as u64) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x8u64))) as u64; return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1424b1e20(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rcx = (((rcx) ^ (rdx))) as u64;
    rcx = (((rcx) as i64).wrapping_mul((r8) as i64) as u64) as u64;
    r9 = rcx;
    r9 = ((r9).wrapping_shr(47u32)) as u64;
    r9 = (((r9) ^ (rcx))) as u64;
    r9 = (((r9) ^ (rdx))) as u64;
    r9 = (((r9) as i64).wrapping_mul((r8) as i64) as u64) as u64;
    rax = r9;
    rax = ((rax).wrapping_shr(47u32)) as u64;
    rax = (((rax) ^ (r9))) as u64;
    rax = (((rax) as i64).wrapping_mul((r8) as i64) as u64) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1424b1e50(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    r9 = (0x9ddfea08eb382d69u64) as u64;
    rcx = (((rcx) ^ (rdx))) as u64;
    rcx = (((rcx) as i64).wrapping_mul((r9) as i64) as u64) as u64;
    r8 = rcx;
    r8 = ((r8).wrapping_shr(47u32)) as u64;
    r8 = (((r8) ^ (rcx))) as u64;
    r8 = (((r8) ^ (rdx))) as u64;
    r8 = (((r8) as i64).wrapping_mul((r9) as i64) as u64) as u64;
    rax = r8;
    rax = ((rax).wrapping_shr(47u32)) as u64;
    rax = (((rax) ^ (r8))) as u64;
    rax = (((rax) as i64).wrapping_mul((r9) as i64) as u64) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1424b1e90(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = rcx;
    rax = ((rax).wrapping_shr(47u32)) as u64;
    fl_l = (((rax) ^ (rcx))) as u64; fl_r = fl_l;
    rax = ((fl_l as u64)) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1424b1eb0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = rcx;
    rcx = (((rdx as u8)) as u32) as u32 as u64;
    rax = ((rax).rotate_right(((rcx as u32) & 63))) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1424b2300(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xfffffffffffffffc;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); r8 = (crate::simd_ops::ru32(rcx)) as u32 as u64; r10 = ((rdx.wrapping_add(rdx.wrapping_mul(4))) as u32) as u32 as u64; rbx = rdx; r8 = (((r8 as u32)).wrapping_add((rdx as u32))) as u32 as u64; r11 = rcx; rax = ((((r8 as u32)) as i32).wrapping_mul((0xcc9e2d51u32) as i32) as u32) as u32 as u64; rax = (((rax as u32)).rotate_right(17u32)) as u32 as u64; rdx = ((((rax as u32)) as i32).wrapping_mul((0x1b873593u32) as i32) as u32) as u32 as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(rbx).wrapping_add(P0))) as u32 as u64; rax = (((rax as u32)).wrapping_add((r10 as u32))) as u32 as u64; rbx = ((rbx).wrapping_shr(1u32)) as u64; rcx = ((((rax as u32)) as i32).wrapping_mul((0xcc9e2d51u32) as i32) as u32) as u32 as u64; rbx = ((((rbx as u32)) & (0x4u32))) as u32 as u64; rdx = ((((rdx as u32)) ^ ((r10 as u32)))) as u32 as u64; rdx = (((rdx as u32)).rotate_right(19u32)) as u32 as u64; rdx = (((rdx as u32)).wrapping_add(0xfaddaf14u32)) as u32 as u64; rcx = (((rcx as u32)).rotate_right(17u32)) as u32 as u64; rax = ((((rcx as u32)) as i32).wrapping_mul((0x1b873593u32) as i32) as u32) as u32 as u64; rcx = 0x318f97d9; rdx = ((rdx.wrapping_add(rdx.wrapping_mul(4))) as u32) as u32 as u64; rdx = ((((rdx as u32)) ^ ((rax as u32)))) as u32 as u64; rax = (((crate::simd_ops::ru32(rbx.wrapping_add(r11))) as i32).wrapping_mul((0x3361d2afu32) as i32) as u32) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x8u64))) as u64; rdx = (((rdx as u32)).rotate_right(19u32)) as u32 as u64; rdx = (((rdx as u32)).wrapping_add(0xfaddaf14u32)) as u32 as u64; rcx = (((rcx as u32)).wrapping_sub((rax as u32))) as u32 as u64; rcx = (((rcx as u32)).rotate_right(17u32)) as u32 as u64; rax = ((((rcx as u32)) as i32).wrapping_mul((0x1b873593u32) as i32) as u32) as u32 as u64; rdx = ((rdx.wrapping_add(rdx.wrapping_mul(4))) as u32) as u32 as u64; rdx = ((((rdx as u32)) ^ ((rax as u32)))) as u32 as u64; rdx = (((rdx as u32)).rotate_right(19u32)) as u32 as u64; rdx = (((rdx as u32)).wrapping_add(0xfaddaf14u32)) as u32 as u64; rax = ((rdx.wrapping_add(rdx.wrapping_mul(4))) as u32) as u32 as u64; rcx = ((rax as u32)) as u32 as u64; rcx = (((rcx as u32)).wrapping_shr(16u32)) as u32 as u64; rcx = ((((rcx as u32)) ^ ((rax as u32)))) as u32 as u64; rax = ((((rcx as u32)) as i32).wrapping_mul((0x85ebca6bu32) as i32) as u32) as u32 as u64; rcx = ((rax as u32)) as u32 as u64; rcx = (((rcx as u32)).wrapping_shr(13u32)) as u32 as u64; rcx = ((((rcx as u32)) ^ ((rax as u32)))) as u32 as u64; rdx = ((((rcx as u32)) as i32).wrapping_mul((0xc2b2ae35u32) as i32) as u32) as u32 as u64; rax = ((rdx as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shr(16u32)) as u32 as u64; fl_l = ((((rax as u32)) ^ ((rdx as u32)))) as u32 as u64; fl_r = fl_l; rax = ((fl_l as u32)) as u32 as u64; return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1424b23c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x9; const P1: u64 = 0x1;
    let (mut rax, mut r10, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = 0; fl_l = 0; fl_r = 0; r8 = rcx; r9 = ((rax as u32)) as u32 as u64; r10 = ((rax.wrapping_add(P0)) as u32) as u32 as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = ((((rax as u32)) as i32).wrapping_mul((0x3361d2afu32) as i32) as u32) as u32 as u64; r9 = (((r9 as u32)).wrapping_add(1)) as u32 as u64; rax = (((crate::simd_ops::ru8(r8)) as i8 as i32) as u32) as u32 as u64; r8 = ((r8.wrapping_add(P1)) as u64) as u64; rax = (((rax as u32)).wrapping_sub((rcx as u32))) as u32 as u64; rcx = ((((r9 as u32)) as i32 as i64) as u64) as u64; r10 = ((((r10 as u32)) ^ ((rax as u32)))) as u32 as u64; fl_l = rcx; fl_r = rdx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 1; continue; } bb = 2; continue; }
            2 => { rcx = ((((rdx as u32)) as i32).wrapping_mul((0xcc9e2d51u32) as i32) as u32) as u32 as u64; rcx = (((rcx as u32)).rotate_right(17u32)) as u32 as u64; rdx = ((((rcx as u32)) as i32).wrapping_mul((0x1b873593u32) as i32) as u32) as u32 as u64; rcx = ((((rax as u32)) as i32).wrapping_mul((0xcc9e2d51u32) as i32) as u32) as u32 as u64; rdx = ((((rdx as u32)) ^ ((r10 as u32)))) as u32 as u64; rcx = (((rcx as u32)).rotate_right(17u32)) as u32 as u64; rax = ((((rcx as u32)) as i32).wrapping_mul((0x1b873593u32) as i32) as u32) as u32 as u64; rdx = (((rdx as u32)).rotate_right(19u32)) as u32 as u64; rdx = (((rdx as u32)).wrapping_add(0xfaddaf14u32)) as u32 as u64; r8 = ((rdx.wrapping_add(rdx.wrapping_mul(4))) as u32) as u32 as u64; rax = ((((rax as u32)) ^ ((r8 as u32)))) as u32 as u64; rax = (((rax as u32)).rotate_right(19u32)) as u32 as u64; rax = (((rax as u32)).wrapping_add(0xfaddaf14u32)) as u32 as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u32) as u32 as u64; rcx = ((rax as u32)) as u32 as u64; rcx = (((rcx as u32)).wrapping_shr(16u32)) as u32 as u64; rcx = ((((rcx as u32)) ^ ((rax as u32)))) as u32 as u64; rax = ((((rcx as u32)) as i32).wrapping_mul((0x85ebca6bu32) as i32) as u32) as u32 as u64; rcx = ((rax as u32)) as u32 as u64; rcx = (((rcx as u32)).wrapping_shr(13u32)) as u32 as u64; rcx = ((((rcx as u32)) ^ ((rax as u32)))) as u32 as u64; rdx = ((((rcx as u32)) as i32).wrapping_mul((0xc2b2ae35u32) as i32) as u32) as u32 as u64; rax = ((rdx as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shr(16u32)) as u32 as u64; fl_l = ((((rax as u32)) ^ ((rdx as u32)))) as u32 as u64; fl_r = fl_l; rax = ((fl_l as u32)) as u32 as u64; return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1424b2460(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0xfffffffffffffffc; const P1: u64 = 0x4;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r11 = ((rcx.wrapping_add(rdx)) as u64) as u64; r9 = rdx; r10 = rcx; r9 = ((r9).wrapping_shr(1u32)) as u64; rax = (((crate::simd_ops::ru32(r9.wrapping_add(rcx).wrapping_add(P0))) as i32).wrapping_mul((0xcc9e2d51u32) as i32) as u32) as u32 as u64; rax = (((rax as u32)).rotate_right(17u32)) as u32 as u64; r8 = ((((rax as u32)) as i32).wrapping_mul((0x1b873593u32) as i32) as u32) as u32 as u64; rax = (((crate::simd_ops::ru32(rcx.wrapping_add(P1))) as i32).wrapping_mul((0xcc9e2d51u32) as i32) as u32) as u32 as u64; rdx = ((((rdx as u32)) ^ ((r8 as u32)))) as u32 as u64; rax = (((rax as u32)).rotate_right(17u32)) as u32 as u64; rax = ((((rax as u32)) as i32).wrapping_mul((0x1b873593u32) as i32) as u32) as u32 as u64; rdx = (((rdx as u32)).rotate_right(19u32)) as u32 as u64; rdx = (((rdx as u32)).wrapping_add(0xfaddaf14u32)) as u32 as u64; rdx = ((rdx.wrapping_add(rdx.wrapping_mul(4))) as u32) as u32 as u64; rdx = ((((rdx as u32)) ^ ((rax as u32)))) as u32 as u64; rax = (((crate::simd_ops::ru32(r11.wrapping_sub(0x8u64))) as i32).wrapping_mul((0xcc9e2d51u32) as i32) as u32) as u32 as u64; rdx = (((rdx as u32)).rotate_right(19u32)) as u32 as u64; rdx = (((rdx as u32)).wrapping_add(0xfaddaf14u32)) as u32 as u64; rax = (((rax as u32)).rotate_right(17u32)) as u32 as u64; rax = ((((rax as u32)) as i32).wrapping_mul((0x1b873593u32) as i32) as u32) as u32 as u64; rcx = ((rdx.wrapping_add(rdx.wrapping_mul(4))) as u32) as u32 as u64; rcx = ((((rcx as u32)) ^ ((rax as u32)))) as u32 as u64; rax = (((crate::simd_ops::ru32(r9.wrapping_add(r10))) as i32).wrapping_mul((0xcc9e2d51u32) as i32) as u32) as u32 as u64; rcx = (((rcx as u32)).rotate_right(19u32)) as u32 as u64; rcx = (((rcx as u32)).wrapping_add(0xfaddaf14u32)) as u32 as u64; rax = (((rax as u32)).rotate_right(17u32)) as u32 as u64; rax = ((((rax as u32)) as i32).wrapping_mul((0x1b873593u32) as i32) as u32) as u32 as u64; rdx = ((rcx.wrapping_add(rcx.wrapping_mul(4))) as u32) as u32 as u64; rdx = ((((rdx as u32)) ^ ((rax as u32)))) as u32 as u64; rax = (((crate::simd_ops::ru32(r10)) as i32).wrapping_mul((0xcc9e2d51u32) as i32) as u32) as u32 as u64; rdx = (((rdx as u32)).rotate_right(19u32)) as u32 as u64; rdx = (((rdx as u32)).wrapping_add(0xfaddaf14u32)) as u32 as u64; rax = (((rax as u32)).rotate_right(17u32)) as u32 as u64; rax = ((((rax as u32)) as i32).wrapping_mul((0x1b873593u32) as i32) as u32) as u32 as u64; rcx = ((rdx.wrapping_add(rdx.wrapping_mul(4))) as u32) as u32 as u64; rcx = ((((rcx as u32)) ^ ((rax as u32)))) as u32 as u64; rax = (((crate::simd_ops::ru32(r11.wrapping_sub(0x4u64))) as i32).wrapping_mul((0xcc9e2d51u32) as i32) as u32) as u32 as u64; rcx = (((rcx as u32)).rotate_right(19u32)) as u32 as u64; rcx = (((rcx as u32)).wrapping_add(0xfaddaf14u32)) as u32 as u64; rax = (((rax as u32)).rotate_right(17u32)) as u32 as u64; rdx = ((rcx.wrapping_add(rcx.wrapping_mul(4))) as u32) as u32 as u64; rax = ((((rax as u32)) as i32).wrapping_mul((0x1b873593u32) as i32) as u32) as u32 as u64; rdx = ((((rdx as u32)) ^ ((rax as u32)))) as u32 as u64; rdx = (((rdx as u32)).rotate_right(19u32)) as u32 as u64; rdx = (((rdx as u32)).wrapping_add(0xfaddaf14u32)) as u32 as u64; rax = ((rdx.wrapping_add(rdx.wrapping_mul(4))) as u32) as u32 as u64; rcx = ((rax as u32)) as u32 as u64; rcx = (((rcx as u32)).wrapping_shr(16u32)) as u32 as u64; rcx = ((((rcx as u32)) ^ ((rax as u32)))) as u32 as u64; rax = ((((rcx as u32)) as i32).wrapping_mul((0x85ebca6bu32) as i32) as u32) as u32 as u64; rcx = ((rax as u32)) as u32 as u64; rcx = (((rcx as u32)).wrapping_shr(13u32)) as u32 as u64; rcx = ((((rcx as u32)) ^ ((rax as u32)))) as u32 as u64; rdx = ((((rcx as u32)) as i32).wrapping_mul((0xc2b2ae35u32) as i32) as u32) as u32 as u64; rax = ((rdx as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shr(16u32)) as u32 as u64; fl_l = ((((rax as u32)) ^ ((rdx as u32)))) as u32 as u64; fl_r = fl_l; rax = ((fl_l as u32)) as u32 as u64; return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1424b2550(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = ((((rcx as u32)) as i32).wrapping_mul((0xcc9e2d51u32) as i32) as u32) as u32 as u64;
    rax = (((rax as u32)).rotate_right(17u32)) as u32 as u64;
    rax = ((((rax as u32)) as i32).wrapping_mul((0x1b873593u32) as i32) as u32) as u32 as u64;
    rax = ((((rax as u32)) ^ ((rdx as u32)))) as u32 as u64;
    rax = (((rax as u32)).rotate_right(19u32)) as u32 as u64;
    fl_l = (((rax as u32)).wrapping_add(0xfaddaf14u32)) as u32 as u64; fl_r = fl_l;
    rax = ((fl_l as u32)) as u32 as u64;
    rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u32) as u32 as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1424b2580(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = ((rcx as u32)) as u32 as u64;
    rcx = (((rdx as u8)) as u32) as u32 as u64;
    rax = (((rax as u32)).rotate_right(((rcx as u32) & 31))) as u32 as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1424b2590(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = ((rcx as u32)) as u32 as u64;
    rax = (((rax as u32)).wrapping_shr(16u32)) as u32 as u64;
    rax = ((((rax as u32)) ^ ((rcx as u32)))) as u32 as u64;
    rcx = ((((rax as u32)) as i32).wrapping_mul((0x85ebca6bu32) as i32) as u32) as u32 as u64;
    rax = ((rcx as u32)) as u32 as u64;
    rax = (((rax as u32)).wrapping_shr(13u32)) as u32 as u64;
    rax = ((((rax as u32)) ^ ((rcx as u32)))) as u32 as u64;
    rcx = ((((rax as u32)) as i32).wrapping_mul((0xc2b2ae35u32) as i32) as u32) as u32 as u64;
    rax = ((rcx as u32)) as u32 as u64;
    rax = (((rax as u32)).wrapping_shr(16u32)) as u32 as u64;
    fl_l = ((((rax as u32)) ^ ((rcx as u32)))) as u32 as u64; fl_r = fl_l;
    rax = ((fl_l as u32)) as u32 as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1424b2c00(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142409b50; const P1: u64 = 0x1427e54e0; const P2: u64 = 0x14240ab10;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<512, 273>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x848u64)) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), (0x84000100u32) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x34u64), (0x0u32) as u32); crate::simd_ops::wu8(rsp.wrapping_add(0x38u64), (0x0u8) as u8); xmm0 = (crate::simd_ops::ru128(rcx)) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); r9 = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; r8 = 0; fl_l = 0; fl_r = 0; rdx = rsp.wrapping_add(0x20); rcx = rsp.wrapping_add(0x30); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r11 = (crate::simd_ops::ru32(rsp.wrapping_add(0x34u64))) as u32 as u64; r8 = ((r11 as u32)) as u32 as u64; r9 = (crate::simd_ops::ru32(rsp.wrapping_add(0x30u64))) as u32 as u64; rdx = ((r9 as u32)) as u32 as u64; rdx = (((rdx as u32)).wrapping_shr(24u32)) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rdx = ((rdx.wrapping_mul(8)) as u32) as u32 as u64; bb = 3; continue; }
            2 => { rdx = (((rdx as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x7c00; rdx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 3; continue; }
            3 => { rax = ((r9 as u32)) as u32 as u64; rax = ((((rax as u32)) & (0xffffffu32))) as u32 as u64; rax = (((rax as u32)).wrapping_shl(3u32)) as u32 as u64; rcx = rsp.wrapping_add(0x38); r10 = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; rcx = (if (((fl_l as u32)) > ((fl_r as u32))) { r10 } else { rcx }) as u64; fl_l = ((r11 as u32)) as u32 as u64; fl_r = ((r11 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 7; continue; } bb = 4; continue; }
            4 => { rdx = P1; bb = 5; continue; }
            5 => { rax = ((crate::simd_ops::ru8(rcx)) as u32) as u32 as u64; rax = ((crate::simd_ops::ru8(rax.wrapping_add(rdx))) as u32) as u32 as u64; crate::simd_ops::wu8(rcx, ((rax as u8)) as u8); rcx = rcx.wrapping_add(0x1); fl_l = r8; fl_r = (0x1u64) as u64; r8 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 5; continue; } bb = 6; continue; }
            6 => { r10 = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; r11 = (crate::simd_ops::ru32(rsp.wrapping_add(0x34u64))) as u32 as u64; r9 = (crate::simd_ops::ru32(rsp.wrapping_add(0x30u64))) as u32 as u64; bb = 7; continue; }
            7 => { rbx = ((r11 as u32)) as u32 as u64; rax = ((r9 as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shr(24u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 9; continue; } bb = 8; continue; }
            8 => { rax = ((rax.wrapping_mul(8)) as u32) as u32 as u64; bb = 10; continue; }
            9 => { rax = (((rax as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x7c00; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 10; continue; }
            10 => { r9 = ((((r9 as u32)) & (0xffffffu32))) as u32 as u64; r9 = (((r9 as u32)).wrapping_shl(3u32)) as u32 as u64; r15 = rsp.wrapping_add(0x38); fl_l = ((r9 as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; r15 = (if (((fl_l as u32)) > ((fl_r as u32))) { r10 } else { r15 }) as u64; rdi = (0x9ddfea08eb382d69u64) as u64; rcx = (0x651e95c4d06fbfb1u64) as u64; fl_l = rbx; fl_r = (0x20u64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 20; continue; } bb = 11; continue; }
            11 => { fl_l = rbx; fl_r = (0x10u64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 19; continue; } bb = 12; continue; }
            12 => { fl_l = rbx; fl_r = (0x8u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 14; continue; } bb = 13; continue; }
            13 => { r12 = (0x9ae16a3b2f90404fu64) as u64; r9 = ((r12.wrapping_add(rbx.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru64(r15)) as u64; rax = ((rax).wrapping_sub(rcx)) as u64; rdx = (crate::simd_ops::ru64(rbx.wrapping_add(r15).wrapping_sub(0x8u64))) as u64; r8 = rax; r8 = ((r8).rotate_right(25u32)) as u64; r8 = ((r8).wrapping_add(rdx)) as u64; r8 = (((r8) as i64).wrapping_mul((r9) as i64) as u64) as u64; rdx = ((rdx).rotate_right(37u32)) as u64; rdx = (((rdx) as i64).wrapping_mul((r9) as i64) as u64) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; rdx = (((rdx) ^ (r8))) as u64; rdx = (((rdx) as i64).wrapping_mul((r9) as i64) as u64) as u64; rcx = rdx; rcx = ((rcx).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rdx))) as u64; rcx = (((rcx) ^ (r8))) as u64; rcx = (((rcx) as i64).wrapping_mul((r9) as i64) as u64) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rax = (((rax) ^ (rcx))) as u64; rax = (((rax) as i64).wrapping_mul((r9) as i64) as u64) as u64; bb = 26; continue; }
            14 => { fl_l = rbx; fl_r = (0x4u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 16; continue; } bb = 15; continue; }
            15 => { r12 = (0x9ae16a3b2f90404fu64) as u64; r9 = ((r12.wrapping_add(rbx.wrapping_mul(2))) as u64) as u64; rdx = (crate::simd_ops::ru32(rbx.wrapping_add(r15).wrapping_sub(0x4u64))) as u32 as u64; rax = (crate::simd_ops::ru32(r15)) as u32 as u64; rcx = ((rbx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rcx = (((rcx) ^ (rdx))) as u64; rcx = (((rcx) as i64).wrapping_mul((r9) as i64) as u64) as u64; r8 = rcx; r8 = ((r8).wrapping_shr(47u32)) as u64; r8 = (((r8) ^ (rcx))) as u64; r8 = (((r8) ^ (rdx))) as u64; r8 = (((r8) as i64).wrapping_mul((r9) as i64) as u64) as u64; rax = r8; rax = ((rax).wrapping_shr(47u32)) as u64; rax = (((rax) ^ (r8))) as u64; rax = (((rax) as i64).wrapping_mul((r9) as i64) as u64) as u64; bb = 26; continue; }
            16 => { fl_l = ((r11 as u32)) as u32 as u64; fl_r = ((r11 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 18; continue; } bb = 17; continue; }
            17 => { rax = rbx; rax = ((rax).wrapping_shr(1u32)) as u64; rdx = ((crate::simd_ops::ru8(rax.wrapping_add(r15))) as u32) as u32 as u64; rax = (0xe16a3b2f90404f00u64) as u64; rdx = (((rdx) as i64).wrapping_mul((rax) as i64) as u64) as u64; rax = ((crate::simd_ops::ru8(r15)) as u32) as u32 as u64; r12 = (0x9ae16a3b2f90404fu64) as u64; rax = (((rax) as i64).wrapping_mul((r12) as i64) as u64) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; rax = ((crate::simd_ops::ru8(rbx.wrapping_add(r15).wrapping_sub(0x1u64))) as u32) as u32 as u64; rax = ((rbx.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax as u32)) as u32 as u64; rcx = (0xc3a5c85c97cb3127u64) as u64; rax = (((rax) as i64).wrapping_mul((rcx) as i64) as u64) as u64; rdx = (((rdx) ^ (rax))) as u64; rax = rdx; rax = ((rax).wrapping_shr(47u32)) as u64; rax = (((rax) ^ (rdx))) as u64; rax = (((rax) as i64).wrapping_mul((r12) as i64) as u64) as u64; bb = 26; continue; }
            18 => { rax = (0x9ae16a3b2f90404fu64) as u64; bb = 26; continue; }
            19 => { r12 = (0x9ae16a3b2f90404fu64) as u64; r11 = ((r12.wrapping_add(rbx.wrapping_mul(2))) as u64) as u64; r13 = (0xb492b66fbe98f273u64) as u64; rdx = (crate::simd_ops::ru64(r15)) as u64; rdx = (((rdx) as i64).wrapping_mul((r13) as i64) as u64) as u64; rcx = (crate::simd_ops::ru64(r15.wrapping_add(0x8u64))) as u64; r8 = r11; r8 = (((r8) as i64).wrapping_mul((crate::simd_ops::ru64(rbx.wrapping_add(r15).wrapping_sub(0x8u64))) as i64) as u64) as u64; r9 = ((rcx.wrapping_add(r12)) as u64) as u64; r9 = ((r9).rotate_right(18u32)) as u64; r9 = ((r9).wrapping_add(r8)) as u64; r9 = ((r9).wrapping_add(rdx)) as u64; r10 = ((rcx.wrapping_add(rdx)) as u64) as u64; r10 = ((r10).rotate_right(43u32)) as u64; r8 = ((r8).rotate_right(30u32)) as u64; r10 = ((r10).wrapping_add(r8)) as u64; rax = (crate::simd_ops::ru64(rbx.wrapping_add(r15).wrapping_sub(0x10u64))) as u64; rax = (((rax) as i64).wrapping_mul((r12) as i64) as u64) as u64; r10 = ((r10).wrapping_add(rax)) as u64; r10 = (((r10) ^ (r9))) as u64; r10 = (((r10) as i64).wrapping_mul((r11) as i64) as u64) as u64; rcx = r10; rcx = ((rcx).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (r9))) as u64; rcx = (((rcx) ^ (r10))) as u64; rcx = (((rcx) as i64).wrapping_mul((r11) as i64) as u64) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rax = (((rax) ^ (rcx))) as u64; rax = (((rax) as i64).wrapping_mul((r11) as i64) as u64) as u64; bb = 26; continue; }
            20 => { r9 = ((r15.wrapping_add(rbx)) as u64) as u64; fl_l = rbx; fl_r = (0x40u64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 22; continue; } bb = 21; continue; }
            21 => { r12 = (0x9ae16a3b2f90404fu64) as u64; r14 = ((r12.wrapping_add(rbx.wrapping_mul(2))) as u64) as u64; rdx = (crate::simd_ops::ru64(r9.wrapping_sub(0x18u64))) as u64; rbp = (crate::simd_ops::ru64(r9.wrapping_sub(0x20u64))) as u64; rax = (crate::simd_ops::ru64(r15.wrapping_add(0x18u64))) as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; r10 = (crate::simd_ops::ru64(r9.wrapping_sub(0x8u64))) as u64; rbx = r14; rbx = (((rbx) as i64).wrapping_mul((crate::simd_ops::ru64(r9.wrapping_sub(0x10u64))) as i64) as u64) as u64; rdi = (crate::simd_ops::ru64(r15)) as u64; rdi = (((rdi) as i64).wrapping_mul((r12) as i64) as u64) as u64; rdi = ((rdi).wrapping_add(r10)) as u64; r8 = rbp; r8 = (((r8) ^ (rdi))) as u64; r8 = ((r8).wrapping_add(1)) as u64; r8 = ((r8).wrapping_add(rcx)) as u64; rax = (crate::simd_ops::ru64(r15.wrapping_add(0x10u64))) as u64; rax = (((rax) as i64).wrapping_mul((r12) as i64) as u64) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; r11 = rcx; r11 = ((r11).rotate_right(42u32)) as u64; r11 = ((r11).wrapping_add(rdx)) as u64; r9 = ((rdx.wrapping_add(rcx)) as u64) as u64; rcx = (crate::simd_ops::ru64(r15.wrapping_add(0x8u64))) as u64; rcx = ((rcx).rotate_right(30u32)) as u64; rcx = ((rcx).wrapping_add(rdx)) as u64; rax = ((r8.wrapping_add(rcx)) as u64) as u64; rdi = ((rdi).rotate_right(43u32)) as u64; rax = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; rdi = ((rdi).wrapping_add(rax)) as u64; rdi = (((rdi) as i64).wrapping_mul((r14) as i64) as u64) as u64; rdi = ((rdi).swap_bytes()) as u64; rdi = ((rdi).wrapping_add(r8)) as u64; rdi = ((rdi).wrapping_add(rbx)) as u64; rdi = (((rdi) as i64).wrapping_mul((r14) as i64) as u64) as u64; rdi = ((rdi).swap_bytes()) as u64; rdi = ((rdi).wrapping_add(r9)) as u64; rdi = ((rdi).wrapping_add(r11)) as u64; rdi = ((rdi).wrapping_add(r10)) as u64; rdi = (((rdi) as i64).wrapping_mul((r14) as i64) as u64) as u64; rdi = ((rdi).swap_bytes()) as u64; rdi = ((rdi).wrapping_add(r9)) as u64; rdi = ((rdi).wrapping_add(crate::simd_ops::ru64(r15.wrapping_add(0x8u64)))) as u64; rdi = (((rdi) as i64).wrapping_mul((r14) as i64) as u64) as u64; rdi = ((rdi).wrapping_add(rbx)) as u64; rdi = ((rdi).wrapping_add(rbp)) as u64; rax = rdi; rax = ((rax).wrapping_shr(47u32)) as u64; rax = (((rax) ^ (rdi))) as u64; rax = (((rax) as i64).wrapping_mul((r14) as i64) as u64) as u64; fl_l = ((rax).wrapping_add(r11)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; rdi = (0x9ddfea08eb382d69u64) as u64; bb = 26; continue; }
            22 => { r12 = (crate::simd_ops::ru64(r9.wrapping_sub(0x28u64))) as u64; r14 = (crate::simd_ops::ru64(r9.wrapping_sub(0x38u64))) as u64; r14 = ((r14).wrapping_add(crate::simd_ops::ru64(r9.wrapping_sub(0x10u64)))) as u64; rax = (crate::simd_ops::ru64(r9.wrapping_sub(0x30u64))) as u64; rax = ((rax).wrapping_add(rbx)) as u64; rax = (((rax) ^ (crate::simd_ops::ru64(r9.wrapping_sub(0x18u64))))) as u64; rax = (((rax) as i64).wrapping_mul((rdi) as i64) as u64) as u64; rcx = rax; rcx = ((rcx).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) ^ (crate::simd_ops::ru64(r9.wrapping_sub(0x18u64))))) as u64; rcx = (((rcx) as i64).wrapping_mul((rdi) as i64) as u64) as u64; r10 = rcx; r10 = ((r10).wrapping_shr(47u32)) as u64; r10 = (((r10) ^ (rcx))) as u64; r10 = (((r10) as i64).wrapping_mul((rdi) as i64) as u64) as u64; rsi = (crate::simd_ops::ru64(r9.wrapping_sub(0x40u64))) as u64; rsi = ((rsi).wrapping_add(rbx)) as u64; rcx = ((r12.wrapping_add(rsi)) as u64) as u64; r8 = rsi; rax = (crate::simd_ops::ru64(r9.wrapping_sub(0x30u64))) as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(r9.wrapping_sub(0x38u64)))) as u64; rsi = ((rsi).wrapping_add(rax)) as u64; r11 = ((r12.wrapping_add(rsi)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x890u64), r11); rax = ((rcx.wrapping_add(r10)) as u64) as u64; rax = ((rax).rotate_right(21u32)) as u64; rsi = ((rsi).rotate_right(44u32)) as u64; rax = ((rax).wrapping_add(r8)) as u64; rsi = ((rsi).wrapping_add(rax)) as u64; r8 = (crate::simd_ops::ru64(r9.wrapping_sub(0x8u64))) as u64; rbp = (crate::simd_ops::ru64(r9.wrapping_sub(0x20u64))) as u64; r13 = (0xb492b66fbe98f273u64) as u64; rbp = ((rbp).wrapping_add(r13)) as u64; rbp = ((rbp).wrapping_add(r14)) as u64; rcx = ((r8.wrapping_add(rbp)) as u64) as u64; rdx = rbp; rax = (crate::simd_ops::ru64(r9.wrapping_sub(0x10u64))) as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(r9.wrapping_sub(0x18u64)))) as u64; rbp = ((rbp).wrapping_add(rax)) as u64; r8 = ((r8).wrapping_add(rbp)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x8a0u64), r8); rax = ((rcx.wrapping_add(r12)) as u64) as u64; rax = ((rax).rotate_right(21u32)) as u64; rbp = ((rbp).rotate_right(44u32)) as u64; rax = ((rax).wrapping_add(rdx)) as u64; rbp = ((rbp).wrapping_add(rax)) as u64; r12 = (((r12) as i64).wrapping_mul((r13) as i64) as u64) as u64; r12 = ((r12).wrapping_add(crate::simd_ops::ru64(r15))) as u64; rbx = ((rbx).wrapping_sub(1)) as u64; rbx = (((rbx) & (0xffffffffffffffc0u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x8a8u64), rbx); fl_l = ((r15).wrapping_add(0x30u64)) as u64; fl_r = fl_l; r15 = ((fl_l as u64)) as u64; r9 = r10; bb = 24; continue; }
            23 => { r11 = r10; bb = 24; continue; }
            24 => { r10 = (crate::simd_ops::ru64(r15.wrapping_sub(0x28u64))) as u64; rdi = ((r11.wrapping_add(r10)) as u64) as u64; rdi = ((rdi).wrapping_add(r14)) as u64; rdi = ((rdi).wrapping_add(r12)) as u64; rdi = ((rdi).rotate_right(37u32)) as u64; rdi = (((rdi) as i64).wrapping_mul((r13) as i64) as u64) as u64; rdi = (((rdi) ^ (rbp))) as u64; rbx = (crate::simd_ops::ru64(r15)) as u64; r11 = (crate::simd_ops::ru64(r15.wrapping_sub(0x8u64))) as u64; rax = ((rsi.wrapping_add(rbx)) as u64) as u64; r14 = ((r14).wrapping_add(rax)) as u64; r14 = ((r14).rotate_right(42u32)) as u64; r14 = (((r14) as i64).wrapping_mul((r13) as i64) as u64) as u64; r14 = ((r14).wrapping_add(crate::simd_ops::ru64(rsp.wrapping_add(0x890u64)))) as u64; r14 = ((r14).wrapping_add(r11)) as u64; r12 = ((r8.wrapping_add(r9)) as u64) as u64; r12 = ((r12).rotate_right(33u32)) as u64; r12 = (((r12) as i64).wrapping_mul((r13) as i64) as u64) as u64; r8 = (crate::simd_ops::ru64(r15.wrapping_sub(0x18u64))) as u64; rsi = (((rsi) as i64).wrapping_mul((r13) as i64) as u64) as u64; rsi = ((rsi).wrapping_add(crate::simd_ops::ru64(r15.wrapping_sub(0x30u64)))) as u64; rcx = ((r8.wrapping_add(rsi)) as u64) as u64; rdx = rsi; r9 = (crate::simd_ops::ru64(r15.wrapping_sub(0x20u64))) as u64; rax = ((r9.wrapping_add(r10)) as u64) as u64; rsi = ((rsi).wrapping_add(rax)) as u64; r10 = ((r8.wrapping_add(rsi)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x890u64), r10); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x8a0u64))) as u64; rax = ((rax).wrapping_add(rcx)) as u64; rax = ((rax).wrapping_add(rdi)) as u64; rax = ((rax).rotate_right(21u32)) as u64; rsi = ((rsi).rotate_right(44u32)) as u64; rax = ((rax).wrapping_add(rdx)) as u64; rsi = ((rsi).wrapping_add(rax)) as u64; r8 = (crate::simd_ops::ru64(r15.wrapping_add(0x8u64))) as u64; rbp = ((rbp).wrapping_add(crate::simd_ops::ru64(r15.wrapping_sub(0x10u64)))) as u64; rbp = ((rbp).wrapping_add(r12)) as u64; rcx = ((r9.wrapping_add(r8)) as u64) as u64; rcx = ((rcx).wrapping_add(rbp)) as u64; rdx = rbp; rax = ((r11.wrapping_add(rbx)) as u64) as u64; rbp = ((rbp).wrapping_add(rax)) as u64; r8 = ((r8).wrapping_add(rbp)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x8a0u64), r8); rax = ((rcx.wrapping_add(r14)) as u64) as u64; rax = ((rax).rotate_right(21u32)) as u64; rbp = ((rbp).rotate_right(44u32)) as u64; rax = ((rax).wrapping_add(rdx)) as u64; rbp = ((rbp).wrapping_add(rax)) as u64; r9 = rdi; r15 = r15.wrapping_add(0x40); fl_l = (crate::simd_ops::ru64(rsp.wrapping_add(0x8a8u64))) as u64; fl_r = (0x40u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x8a8u64), ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64); if (((fl_l as u64)) != ((fl_r as u64))) { bb = 23; continue; } bb = 25; continue; }
            25 => { rsi = (((rsi) ^ (rbp))) as u64; rdi = (0x9ddfea08eb382d69u64) as u64; rsi = (((rsi) as i64).wrapping_mul((rdi) as i64) as u64) as u64; rdx = rsi; rdx = ((rdx).wrapping_shr(47u32)) as u64; rdx = (((rdx) ^ (rsi))) as u64; rdx = (((rdx) ^ (rbp))) as u64; rdx = (((rdx) as i64).wrapping_mul((rdi) as i64) as u64) as u64; rax = r8; rax = (((rax) ^ (r10))) as u64; rax = (((rax) as i64).wrapping_mul((rdi) as i64) as u64) as u64; rcx = rax; rcx = ((rcx).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (r8))) as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) as i64).wrapping_mul((rdi) as i64) as u64) as u64; r8 = rdx; r8 = ((r8).wrapping_shr(47u32)) as u64; r8 = (((r8) ^ (rdx))) as u64; r8 = (((r8) as i64).wrapping_mul((rdi) as i64) as u64) as u64; r8 = ((r8).wrapping_add(r12)) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rax = (((rax) ^ (rcx))) as u64; rax = (((rax) as i64).wrapping_mul((rdi) as i64) as u64) as u64; rdx = r14; rdx = ((rdx).wrapping_shr(47u32)) as u64; rdx = (((rdx) ^ (r14))) as u64; rdx = (((rdx) as i64).wrapping_mul((r13) as i64) as u64) as u64; rax = ((rax).wrapping_add(r9)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; rdx = (((rdx) ^ (r8))) as u64; rdx = (((rdx) as i64).wrapping_mul((rdi) as i64) as u64) as u64; rcx = rdx; rcx = ((rcx).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rdx))) as u64; rcx = (((rcx) ^ (r8))) as u64; rcx = (((rcx) as i64).wrapping_mul((rdi) as i64) as u64) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rax = (((rax) ^ (rcx))) as u64; rax = (((rax) as i64).wrapping_mul((rdi) as i64) as u64) as u64; bb = 26; continue; }
            26 => { rcx = (0x651e95c4d06fbfb1u64) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; rcx = (((rcx) ^ (crate::simd_ops::ru64(rsp.wrapping_add(0x898u64))))) as u64; rcx = (((rcx) as i64).wrapping_mul((rdi) as i64) as u64) as u64; rbx = rcx; rbx = ((rbx).wrapping_shr(47u32)) as u64; rbx = (((rbx) ^ (rcx))) as u64; rbx = (((rbx) ^ (crate::simd_ops::ru64(rsp.wrapping_add(0x898u64))))) as u64; rbx = (((rbx) as i64).wrapping_mul((rdi) as i64) as u64) as u64; rcx = rsp.wrapping_add(0x30); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = rbx; rax = ((rax).wrapping_shr(47u32)) as u64; rax = (((rax) ^ (rbx))) as u64; rax = (((rax) as i64).wrapping_mul((rdi) as i64) as u64) as u64; rsp = ((rsp).wrapping_add(0x848u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}
