#![allow(unused_imports, unused_variables, unused_mut, unused_assignments, non_snake_case, dead_code, clippy::all, unreachable_code, unused_parens)]

#[inline(never)]
pub unsafe extern "win64" fn fn_1411c4ac0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1405af8d0; const P1: u64 = 0x14183acc8; const P2: u64 = 0x1410c38f0; const P3: u64 = 0x1401decf0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rdx; r15 = rcx; rsi = (crate::simd_ops::ru64(rcx)) as u64; rax = (((crate::simd_ops::ru8(rsi.wrapping_add(0x40u64))) as i8 as i64) as u64) as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x40u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 13; continue; } bb = 1; continue; }
            1 => { fl_l = rsi; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 19; continue; } bb = 2; continue; }
            2 => { rbp = (crate::simd_ops::ru32(rdx.wrapping_add(0x8u64))) as u32 as u64; r15 = ((rbp.wrapping_mul(4)) as u64) as u64; r14 = (crate::simd_ops::ru64(rdx)) as u64; rbx = (crate::simd_ops::ru64(rsi)) as u64; rdi = (crate::simd_ops::ru32(rsi.wrapping_add(0x8u64))) as u32 as u64; rax = (crate::simd_ops::ru32(rsi.wrapping_add(0xcu64))) as u32 as u64; fl_l = rbp; fl_r = rax; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rdi = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rsi.wrapping_add(0x8u64), ((rdi as u32)) as u32); rdx = ((rbp as u32)) as u32 as u64; rcx = rsi; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rsi)) as u64; bb = 10; continue; }
            4 => { fl_l = rbp; fl_r = rdi; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 9; continue; } bb = 5; continue; }
            5 => { r15 = ((rdi.wrapping_mul(4)) as u64) as u64; r12 = ((r15.wrapping_add(r14)) as u64) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { r8 = r15; rdx = r14; rcx = rbx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rbx).wrapping_add(r15)) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; bb = 7; continue; }
            7 => { r8 = rbp; fl_l = r8; fl_r = rdi; r8 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 12; continue; } bb = 8; continue; }
            8 => { r8 = ((r8.wrapping_mul(4)) as u64) as u64; rdx = r12; rcx = rbx; bb = 11; continue; }
            9 => { rcx = rbx; bb = 10; continue; }
            10 => { r8 = r15; rdx = r14; bb = 11; continue; }
            11 => { rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 12; continue; }
            12 => { crate::simd_ops::wu32(rsi.wrapping_add(0x8u64), ((rbp as u32)) as u32); bb = 19; continue; }
            13 => { crate::simd_ops::wu8(rsp.wrapping_add(0x50u64), (0x0u8) as u8); rcx = rax.wrapping_add(0x1); r8 = rsi; rdx = rsp.wrapping_add(0x50); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu8(rsi.wrapping_add(0x40u64), (0xffu8) as u8); r14 = (crate::simd_ops::ru64(r15)) as u64; rdi = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(r14, rdi); crate::simd_ops::wu64(r14.wrapping_add(0x8u64), rdi); rbp = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; rbx = (crate::simd_ops::ru64(rbx)) as u64; rsi = ((rbx.wrapping_add(rbp.wrapping_mul(4))) as u64) as u64; fl_l = rbp; fl_r = rbp; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 15; continue; } bb = 14; continue; }
            14 => { rdx = ((rbp as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(r14, rax); crate::simd_ops::wu32(r14.wrapping_add(0x8u64), ((rdi as u32)) as u32); crate::simd_ops::wu32(r14.wrapping_add(0xcu64), ((rbp as u32)) as u32); rdi = rax; bb = 15; continue; }
            15 => { fl_l = rbx; fl_r = rsi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 18; continue; } bb = 16; continue; }
            16 => { bb = 17; continue; }
            17 => { rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; crate::simd_ops::wu32(rdi, ((rax as u32)) as u32); rdi = rdi.wrapping_add(0x4); rbx = ((rbx).wrapping_add(0x4u64)) as u64; fl_l = rbx; fl_r = rsi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 17; continue; } bb = 18; continue; }
            18 => { crate::simd_ops::wu32(r14.wrapping_add(0x8u64), ((rbp as u32)) as u32); rax = (crate::simd_ops::ru64(r15)) as u64; crate::simd_ops::wu8(rax.wrapping_add(0x40u64), (0x40u8) as u8); bb = 19; continue; }
            19 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411c4c10(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x40; const P1: u64 = 0x1; const P2: u64 = 0x1410c38f0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = (crate::simd_ops::ru64(rcx)) as u64; rbx = rdx; rsi = rcx; rax = (((crate::simd_ops::ru8(rdi.wrapping_add(P0))) as i8 as i64) as u64) as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x41u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru32(rdx)) as u32 as u64; crate::simd_ops::wu32(rdi, ((rax as u32)) as u32); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            2 => { rcx = ((rax.wrapping_add(P1)) as u64) as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x30u64), (0x0u8) as u8); r8 = rdi; rdx = rsp.wrapping_add(0x30); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu8(rdi.wrapping_add(P0), (0xffu8) as u8); rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; rcx = (crate::simd_ops::ru64(rsi)) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; crate::simd_ops::wu32(rcx, ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rsi)) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; crate::simd_ops::wu8(rax.wrapping_add(P0), (0x41u8) as u8); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411c4c80(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x40; const P1: u64 = 0x1411c7250; const P2: u64 = 0x1; const P3: u64 = 0x1410c38f0; const P4: u64 = 0x8; const P5: u64 = 0x1401decf0; const P6: u64 = 0xc; const P7: u64 = 0x4;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rdx; r15 = rcx; rdi = (crate::simd_ops::ru64(rcx)) as u64; rax = (((crate::simd_ops::ru8(rdi.wrapping_add(P0))) as i8 as i64) as u64) as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x42u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = rdi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 8; continue; }
            2 => { crate::simd_ops::wu8(rsp.wrapping_add(0x40u64), (0x0u8) as u8); rcx = ((rax.wrapping_add(P2)) as u64) as u64; r8 = rdi; rdx = rsp.wrapping_add(0x40); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu8(rdi.wrapping_add(P0), (0xffu8) as u8); r14 = (crate::simd_ops::ru64(r15)) as u64; rdi = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(r14, rdi); crate::simd_ops::wu64(r14.wrapping_add(P4), rdi); rbp = (crate::simd_ops::ru32(rbx.wrapping_add(P4))) as u32 as u64; rbx = (crate::simd_ops::ru64(rbx)) as u64; rsi = ((rbx.wrapping_add(rbp.wrapping_mul(4))) as u64) as u64; fl_l = rbp; fl_r = rbp; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rdx = ((rbp as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(r14, rax); crate::simd_ops::wu32(r14.wrapping_add(P4), ((rdi as u32)) as u32); crate::simd_ops::wu32(r14.wrapping_add(P6), ((rbp as u32)) as u32); rdi = rax; bb = 4; continue; }
            4 => { fl_l = rbx; fl_r = rsi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 7; continue; } bb = 5; continue; }
            5 => { bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; crate::simd_ops::wu32(rdi, ((rax as u32)) as u32); rdi = ((rdi.wrapping_add(P7)) as u64) as u64; rbx = ((rbx).wrapping_add(0x4u64)) as u64; fl_l = rbx; fl_r = rsi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 6; continue; } bb = 7; continue; }
            7 => { crate::simd_ops::wu32(r14.wrapping_add(P4), ((rbp as u32)) as u32); rax = (crate::simd_ops::ru64(r15)) as u64; crate::simd_ops::wu8(rax.wrapping_add(P0), (0x42u8) as u8); bb = 8; continue; }
            8 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411c4d50(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x40; const P1: u64 = 0x1; const P2: u64 = 0x1410c38f0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = (crate::simd_ops::ru64(rcx)) as u64; rbx = rdx; rsi = rcx; rax = (((crate::simd_ops::ru8(rdi.wrapping_add(P0))) as i8 as i64) as u64) as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x43u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru32(rdx)) as u32 as u64; crate::simd_ops::wu32(rdi, ((rax as u32)) as u32); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            2 => { rcx = ((rax.wrapping_add(P1)) as u64) as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x30u64), (0x0u8) as u8); r8 = rdi; rdx = rsp.wrapping_add(0x30); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu8(rdi.wrapping_add(P0), (0xffu8) as u8); rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; rcx = (crate::simd_ops::ru64(rsi)) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; crate::simd_ops::wu32(rcx, ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rsi)) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; crate::simd_ops::wu8(rax.wrapping_add(P0), (0x43u8) as u8); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411c4dc0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x40; const P1: u64 = 0x1411c7250; const P2: u64 = 0x1; const P3: u64 = 0x1410c38f0; const P4: u64 = 0x8; const P5: u64 = 0x1401decf0; const P6: u64 = 0xc; const P7: u64 = 0x4;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rdx; r15 = rcx; rdi = (crate::simd_ops::ru64(rcx)) as u64; rax = (((crate::simd_ops::ru8(rdi.wrapping_add(P0))) as i8 as i64) as u64) as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x44u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = rdi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 8; continue; }
            2 => { crate::simd_ops::wu8(rsp.wrapping_add(0x40u64), (0x0u8) as u8); rcx = ((rax.wrapping_add(P2)) as u64) as u64; r8 = rdi; rdx = rsp.wrapping_add(0x40); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu8(rdi.wrapping_add(P0), (0xffu8) as u8); r14 = (crate::simd_ops::ru64(r15)) as u64; rdi = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(r14, rdi); crate::simd_ops::wu64(r14.wrapping_add(P4), rdi); rbp = (crate::simd_ops::ru32(rbx.wrapping_add(P4))) as u32 as u64; rbx = (crate::simd_ops::ru64(rbx)) as u64; rsi = ((rbx.wrapping_add(rbp.wrapping_mul(4))) as u64) as u64; fl_l = rbp; fl_r = rbp; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rdx = ((rbp as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(r14, rax); crate::simd_ops::wu32(r14.wrapping_add(P4), ((rdi as u32)) as u32); crate::simd_ops::wu32(r14.wrapping_add(P6), ((rbp as u32)) as u32); rdi = rax; bb = 4; continue; }
            4 => { fl_l = rbx; fl_r = rsi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 7; continue; } bb = 5; continue; }
            5 => { bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; crate::simd_ops::wu32(rdi, ((rax as u32)) as u32); rdi = ((rdi.wrapping_add(P7)) as u64) as u64; rbx = ((rbx).wrapping_add(0x4u64)) as u64; fl_l = rbx; fl_r = rsi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 6; continue; } bb = 7; continue; }
            7 => { crate::simd_ops::wu32(r14.wrapping_add(P4), ((rbp as u32)) as u32); rax = (crate::simd_ops::ru64(r15)) as u64; crate::simd_ops::wu8(rax.wrapping_add(P0), (0x44u8) as u8); bb = 8; continue; }
            8 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411c4e90(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x40; const P1: u64 = 0x1; const P2: u64 = 0x1410c38f0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = (crate::simd_ops::ru64(rcx)) as u64; rbx = rdx; rsi = rcx; rax = (((crate::simd_ops::ru8(rdi.wrapping_add(P0))) as i8 as i64) as u64) as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x45u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rdx)) as u64; crate::simd_ops::wu64(rdi, rax); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            2 => { rcx = ((rax.wrapping_add(P1)) as u64) as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x30u64), (0x0u8) as u8); r8 = rdi; rdx = rsp.wrapping_add(0x30); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu8(rdi.wrapping_add(P0), (0xffu8) as u8); rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = (crate::simd_ops::ru64(rsi)) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; crate::simd_ops::wu64(rcx, rax); rax = (crate::simd_ops::ru64(rsi)) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; crate::simd_ops::wu8(rax.wrapping_add(P0), (0x45u8) as u8); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411c4f10(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x40; const P1: u64 = 0x1405a4d20; const P2: u64 = 0x1; const P3: u64 = 0x1410c38f0; const P4: u64 = 0x1403c7e70;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdi = (crate::simd_ops::ru64(rcx)) as u64;
    rbx = rdx;
    rsi = rcx;
    rax = (((crate::simd_ops::ru8(rdi.wrapping_add(P0))) as i8 as i64) as u64) as u64;
    fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x46u8) as u8 as u64;
    if !((((fl_l as u8)) != ((fl_r as u8)))) { rcx = rdi; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P1, rax, rcx, rdx, r8, r9, rsp); }
    rcx = ((rax.wrapping_add(P2)) as u64) as u64;
    crate::simd_ops::wu8(rsp.wrapping_add(0x30u64), (0x0u8) as u8);
    r8 = rdi;
    rdx = rsp.wrapping_add(0x30);
    rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp);
    crate::simd_ops::wu8(rdi.wrapping_add(P0), (0xffu8) as u8);
    rdx = rbx;
    rcx = (crate::simd_ops::ru64(rsi)) as u64;
    rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp);
    rax = (crate::simd_ops::ru64(rsi)) as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64;
    crate::simd_ops::wu8(rax.wrapping_add(P0), (0x46u8) as u8);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411c4f90(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x40; const P1: u64 = 0x1; const P2: u64 = 0x1410c38f0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = (crate::simd_ops::ru64(rcx)) as u64; rbx = rdx; rsi = rcx; rax = (((crate::simd_ops::ru8(rdi.wrapping_add(P0))) as i8 as i64) as u64) as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x47u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rdx)) as u64; crate::simd_ops::wu64(rdi, rax); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            2 => { rcx = ((rax.wrapping_add(P1)) as u64) as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x30u64), (0x0u8) as u8); r8 = rdi; rdx = rsp.wrapping_add(0x30); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu8(rdi.wrapping_add(P0), (0xffu8) as u8); rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = (crate::simd_ops::ru64(rsi)) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; crate::simd_ops::wu64(rcx, rax); rax = (crate::simd_ops::ru64(rsi)) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; crate::simd_ops::wu8(rax.wrapping_add(P0), (0x47u8) as u8); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411c5010(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x40; const P1: u64 = 0x1405a4d20; const P2: u64 = 0x1; const P3: u64 = 0x1410c38f0; const P4: u64 = 0x1403c7e70;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdi = (crate::simd_ops::ru64(rcx)) as u64;
    rbx = rdx;
    rsi = rcx;
    rax = (((crate::simd_ops::ru8(rdi.wrapping_add(P0))) as i8 as i64) as u64) as u64;
    fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x48u8) as u8 as u64;
    if !((((fl_l as u8)) != ((fl_r as u8)))) { rcx = rdi; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P1, rax, rcx, rdx, r8, r9, rsp); }
    rcx = ((rax.wrapping_add(P2)) as u64) as u64;
    crate::simd_ops::wu8(rsp.wrapping_add(0x30u64), (0x0u8) as u8);
    r8 = rdi;
    rdx = rsp.wrapping_add(0x30);
    rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp);
    crate::simd_ops::wu8(rdi.wrapping_add(P0), (0xffu8) as u8);
    rdx = rbx;
    rcx = (crate::simd_ops::ru64(rsi)) as u64;
    rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp);
    rax = (crate::simd_ops::ru64(rsi)) as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64;
    crate::simd_ops::wu8(rax.wrapping_add(P0), (0x48u8) as u8);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411c5090(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x40; const P1: u64 = 0x1; const P2: u64 = 0x1410c38f0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = (crate::simd_ops::ru64(rcx)) as u64; rbx = rdx; rsi = rcx; rax = (((crate::simd_ops::ru8(rdi.wrapping_add(P0))) as i8 as i64) as u64) as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x49u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rdx)) as u64; crate::simd_ops::wu64(rdi, rax); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            2 => { rcx = ((rax.wrapping_add(P1)) as u64) as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x30u64), (0x0u8) as u8); r8 = rdi; rdx = rsp.wrapping_add(0x30); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu8(rdi.wrapping_add(P0), (0xffu8) as u8); rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = (crate::simd_ops::ru64(rsi)) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; crate::simd_ops::wu64(rcx, rax); rax = (crate::simd_ops::ru64(rsi)) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; crate::simd_ops::wu8(rax.wrapping_add(P0), (0x49u8) as u8); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411c5110(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x40; const P1: u64 = 0x1411c7350; const P2: u64 = 0x1; const P3: u64 = 0x1410c38f0; const P4: u64 = 0x8; const P5: u64 = 0x1401dec00; const P6: u64 = 0xc;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rdx; r15 = rcx; rdi = (crate::simd_ops::ru64(rcx)) as u64; rax = (((crate::simd_ops::ru8(rdi.wrapping_add(P0))) as i8 as i64) as u64) as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x4au8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = rdi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 8; continue; }
            2 => { crate::simd_ops::wu8(rsp.wrapping_add(0x40u64), (0x0u8) as u8); rcx = ((rax.wrapping_add(P2)) as u64) as u64; r8 = rdi; rdx = rsp.wrapping_add(0x40); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu8(rdi.wrapping_add(P0), (0xffu8) as u8); r14 = (crate::simd_ops::ru64(r15)) as u64; rdi = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(r14, rdi); crate::simd_ops::wu64(r14.wrapping_add(P4), rdi); rbp = (crate::simd_ops::ru32(rbx.wrapping_add(P4))) as u32 as u64; rbx = (crate::simd_ops::ru64(rbx)) as u64; rsi = ((rbx.wrapping_add(rbp.wrapping_mul(8))) as u64) as u64; fl_l = rbp; fl_r = rbp; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rdx = ((rbp as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(r14, rax); crate::simd_ops::wu32(r14.wrapping_add(P4), ((rdi as u32)) as u32); crate::simd_ops::wu32(r14.wrapping_add(P6), ((rbp as u32)) as u32); rdi = rax; bb = 4; continue; }
            4 => { fl_l = rbx; fl_r = rsi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 7; continue; } bb = 5; continue; }
            5 => { bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu64(rdi, rax); rdi = ((rdi.wrapping_add(P4)) as u64) as u64; rbx = ((rbx).wrapping_add(0x8u64)) as u64; fl_l = rbx; fl_r = rsi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 6; continue; } bb = 7; continue; }
            7 => { crate::simd_ops::wu32(r14.wrapping_add(P4), ((rbp as u32)) as u32); rax = (crate::simd_ops::ru64(r15)) as u64; crate::simd_ops::wu8(rax.wrapping_add(P0), (0x4au8) as u8); bb = 8; continue; }
            8 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411c51e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x40; const P1: u64 = 0x1; const P2: u64 = 0x1410c38f0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = (crate::simd_ops::ru64(rcx)) as u64; rbx = rdx; rsi = rcx; rax = (((crate::simd_ops::ru8(rdi.wrapping_add(P0))) as i8 as i64) as u64) as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x4bu8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru32(rdx)) as u32 as u64; crate::simd_ops::wu32(rdi, ((rax as u32)) as u32); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            2 => { rcx = ((rax.wrapping_add(P1)) as u64) as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x30u64), (0x0u8) as u8); r8 = rdi; rdx = rsp.wrapping_add(0x30); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu8(rdi.wrapping_add(P0), (0xffu8) as u8); rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; rcx = (crate::simd_ops::ru64(rsi)) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; crate::simd_ops::wu32(rcx, ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rsi)) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; crate::simd_ops::wu8(rax.wrapping_add(P0), (0x4bu8) as u8); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411c5250(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1405af8d0; const P1: u64 = 0x14183acc8; const P2: u64 = 0x1410c38f0; const P3: u64 = 0x1401decf0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rdx; r15 = rcx; rsi = (crate::simd_ops::ru64(rcx)) as u64; rax = (((crate::simd_ops::ru8(rsi.wrapping_add(0x40u64))) as i8 as i64) as u64) as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x4cu8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 13; continue; } bb = 1; continue; }
            1 => { fl_l = rsi; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 19; continue; } bb = 2; continue; }
            2 => { rbp = (crate::simd_ops::ru32(rdx.wrapping_add(0x8u64))) as u32 as u64; r15 = ((rbp.wrapping_mul(4)) as u64) as u64; r14 = (crate::simd_ops::ru64(rdx)) as u64; rbx = (crate::simd_ops::ru64(rsi)) as u64; rdi = (crate::simd_ops::ru32(rsi.wrapping_add(0x8u64))) as u32 as u64; rax = (crate::simd_ops::ru32(rsi.wrapping_add(0xcu64))) as u32 as u64; fl_l = rbp; fl_r = rax; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rdi = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rsi.wrapping_add(0x8u64), ((rdi as u32)) as u32); rdx = ((rbp as u32)) as u32 as u64; rcx = rsi; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rsi)) as u64; bb = 10; continue; }
            4 => { fl_l = rbp; fl_r = rdi; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 9; continue; } bb = 5; continue; }
            5 => { r15 = ((rdi.wrapping_mul(4)) as u64) as u64; r12 = ((r14.wrapping_add(r15)) as u64) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { r8 = r15; rdx = r14; rcx = rbx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rbx).wrapping_add(r15)) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; bb = 7; continue; }
            7 => { r8 = rbp; fl_l = r8; fl_r = rdi; r8 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 12; continue; } bb = 8; continue; }
            8 => { r8 = ((r8.wrapping_mul(4)) as u64) as u64; rdx = r12; rcx = rbx; bb = 11; continue; }
            9 => { rcx = rbx; bb = 10; continue; }
            10 => { r8 = r15; rdx = r14; bb = 11; continue; }
            11 => { rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 12; continue; }
            12 => { crate::simd_ops::wu32(rsi.wrapping_add(0x8u64), ((rbp as u32)) as u32); bb = 19; continue; }
            13 => { crate::simd_ops::wu8(rsp.wrapping_add(0x50u64), (0x0u8) as u8); rcx = rax.wrapping_add(0x1); r8 = rsi; rdx = rsp.wrapping_add(0x50); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu8(rsi.wrapping_add(0x40u64), (0xffu8) as u8); r14 = (crate::simd_ops::ru64(r15)) as u64; rdi = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(r14, rdi); crate::simd_ops::wu64(r14.wrapping_add(0x8u64), rdi); rbp = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; rbx = (crate::simd_ops::ru64(rbx)) as u64; rsi = ((rbx.wrapping_add(rbp.wrapping_mul(4))) as u64) as u64; fl_l = rbp; fl_r = rbp; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 15; continue; } bb = 14; continue; }
            14 => { rdx = ((rbp as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(r14, rax); crate::simd_ops::wu32(r14.wrapping_add(0x8u64), ((rdi as u32)) as u32); crate::simd_ops::wu32(r14.wrapping_add(0xcu64), ((rbp as u32)) as u32); rdi = rax; bb = 15; continue; }
            15 => { fl_l = rbx; fl_r = rsi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 18; continue; } bb = 16; continue; }
            16 => { bb = 17; continue; }
            17 => { rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; crate::simd_ops::wu32(rdi, ((rax as u32)) as u32); rdi = rdi.wrapping_add(0x4); rbx = ((rbx).wrapping_add(0x4u64)) as u64; fl_l = rbx; fl_r = rsi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 17; continue; } bb = 18; continue; }
            18 => { crate::simd_ops::wu32(r14.wrapping_add(0x8u64), ((rbp as u32)) as u32); rax = (crate::simd_ops::ru64(r15)) as u64; crate::simd_ops::wu8(rax.wrapping_add(0x40u64), (0x4cu8) as u8); bb = 19; continue; }
            19 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411c53a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x40; const P1: u64 = 0x1; const P2: u64 = 0x1410c38f0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = (crate::simd_ops::ru64(rcx)) as u64; rbx = rdx; rsi = rcx; rax = (((crate::simd_ops::ru8(rdi.wrapping_add(P0))) as i8 as i64) as u64) as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x4du8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru32(rdx)) as u32 as u64; crate::simd_ops::wu32(rdi, ((rax as u32)) as u32); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            2 => { rcx = ((rax.wrapping_add(P1)) as u64) as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x30u64), (0x0u8) as u8); r8 = rdi; rdx = rsp.wrapping_add(0x30); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu8(rdi.wrapping_add(P0), (0xffu8) as u8); rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; rcx = (crate::simd_ops::ru64(rsi)) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; crate::simd_ops::wu32(rcx, ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rsi)) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; crate::simd_ops::wu8(rax.wrapping_add(P0), (0x4du8) as u8); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411c5410(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1405af8d0; const P1: u64 = 0x14183acc8; const P2: u64 = 0x1410c38f0; const P3: u64 = 0x1401decf0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rdx; r15 = rcx; rsi = (crate::simd_ops::ru64(rcx)) as u64; rax = (((crate::simd_ops::ru8(rsi.wrapping_add(0x40u64))) as i8 as i64) as u64) as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x4eu8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 13; continue; } bb = 1; continue; }
            1 => { fl_l = rsi; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 19; continue; } bb = 2; continue; }
            2 => { rbp = (crate::simd_ops::ru32(rdx.wrapping_add(0x8u64))) as u32 as u64; r15 = ((rbp.wrapping_mul(4)) as u64) as u64; r14 = (crate::simd_ops::ru64(rdx)) as u64; rbx = (crate::simd_ops::ru64(rsi)) as u64; rdi = (crate::simd_ops::ru32(rsi.wrapping_add(0x8u64))) as u32 as u64; rax = (crate::simd_ops::ru32(rsi.wrapping_add(0xcu64))) as u32 as u64; fl_l = rbp; fl_r = rax; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rdi = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rsi.wrapping_add(0x8u64), ((rdi as u32)) as u32); rdx = ((rbp as u32)) as u32 as u64; rcx = rsi; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rsi)) as u64; bb = 10; continue; }
            4 => { fl_l = rbp; fl_r = rdi; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 9; continue; } bb = 5; continue; }
            5 => { r15 = ((rdi.wrapping_mul(4)) as u64) as u64; r12 = ((r14.wrapping_add(r15)) as u64) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { r8 = r15; rdx = r14; rcx = rbx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rbx).wrapping_add(r15)) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; bb = 7; continue; }
            7 => { r8 = rbp; fl_l = r8; fl_r = rdi; r8 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 12; continue; } bb = 8; continue; }
            8 => { r8 = ((r8.wrapping_mul(4)) as u64) as u64; rdx = r12; rcx = rbx; bb = 11; continue; }
            9 => { rcx = rbx; bb = 10; continue; }
            10 => { r8 = r15; rdx = r14; bb = 11; continue; }
            11 => { rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 12; continue; }
            12 => { crate::simd_ops::wu32(rsi.wrapping_add(0x8u64), ((rbp as u32)) as u32); bb = 19; continue; }
            13 => { crate::simd_ops::wu8(rsp.wrapping_add(0x50u64), (0x0u8) as u8); rcx = rax.wrapping_add(0x1); r8 = rsi; rdx = rsp.wrapping_add(0x50); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu8(rsi.wrapping_add(0x40u64), (0xffu8) as u8); r14 = (crate::simd_ops::ru64(r15)) as u64; rdi = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(r14, rdi); crate::simd_ops::wu64(r14.wrapping_add(0x8u64), rdi); rbp = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; rbx = (crate::simd_ops::ru64(rbx)) as u64; rsi = ((rbx.wrapping_add(rbp.wrapping_mul(4))) as u64) as u64; fl_l = rbp; fl_r = rbp; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 15; continue; } bb = 14; continue; }
            14 => { rdx = ((rbp as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(r14, rax); crate::simd_ops::wu32(r14.wrapping_add(0x8u64), ((rdi as u32)) as u32); crate::simd_ops::wu32(r14.wrapping_add(0xcu64), ((rbp as u32)) as u32); rdi = rax; bb = 15; continue; }
            15 => { fl_l = rbx; fl_r = rsi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 18; continue; } bb = 16; continue; }
            16 => { bb = 17; continue; }
            17 => { rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; crate::simd_ops::wu32(rdi, ((rax as u32)) as u32); rdi = rdi.wrapping_add(0x4); rbx = ((rbx).wrapping_add(0x4u64)) as u64; fl_l = rbx; fl_r = rsi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 17; continue; } bb = 18; continue; }
            18 => { crate::simd_ops::wu32(r14.wrapping_add(0x8u64), ((rbp as u32)) as u32); rax = (crate::simd_ops::ru64(r15)) as u64; crate::simd_ops::wu8(rax.wrapping_add(0x40u64), (0x4eu8) as u8); bb = 19; continue; }
            19 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411c5560(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x40; const P1: u64 = 0x1; const P2: u64 = 0x1410c38f0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = (crate::simd_ops::ru64(rcx)) as u64; rbx = rdx; rsi = rcx; rax = (((crate::simd_ops::ru8(rdi.wrapping_add(P0))) as i8 as i64) as u64) as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x4fu8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru32(rdx)) as u32 as u64; crate::simd_ops::wu32(rdi, ((rax as u32)) as u32); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            2 => { rcx = ((rax.wrapping_add(P1)) as u64) as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x30u64), (0x0u8) as u8); r8 = rdi; rdx = rsp.wrapping_add(0x30); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu8(rdi.wrapping_add(P0), (0xffu8) as u8); rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; rcx = (crate::simd_ops::ru64(rsi)) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; crate::simd_ops::wu32(rcx, ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rsi)) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; crate::simd_ops::wu8(rax.wrapping_add(P0), (0x4fu8) as u8); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411c55d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x40; const P1: u64 = 0x1411c7250; const P2: u64 = 0x1; const P3: u64 = 0x1410c38f0; const P4: u64 = 0x8; const P5: u64 = 0x1401decf0; const P6: u64 = 0xc; const P7: u64 = 0x4;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rdx; r15 = rcx; rdi = (crate::simd_ops::ru64(rcx)) as u64; rax = (((crate::simd_ops::ru8(rdi.wrapping_add(P0))) as i8 as i64) as u64) as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x50u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = rdi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 8; continue; }
            2 => { crate::simd_ops::wu8(rsp.wrapping_add(0x40u64), (0x0u8) as u8); rcx = ((rax.wrapping_add(P2)) as u64) as u64; r8 = rdi; rdx = rsp.wrapping_add(0x40); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu8(rdi.wrapping_add(P0), (0xffu8) as u8); r14 = (crate::simd_ops::ru64(r15)) as u64; rdi = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(r14, rdi); crate::simd_ops::wu64(r14.wrapping_add(P4), rdi); rbp = (crate::simd_ops::ru32(rbx.wrapping_add(P4))) as u32 as u64; rbx = (crate::simd_ops::ru64(rbx)) as u64; rsi = ((rbx.wrapping_add(rbp.wrapping_mul(4))) as u64) as u64; fl_l = rbp; fl_r = rbp; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rdx = ((rbp as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(r14, rax); crate::simd_ops::wu32(r14.wrapping_add(P4), ((rdi as u32)) as u32); crate::simd_ops::wu32(r14.wrapping_add(P6), ((rbp as u32)) as u32); rdi = rax; bb = 4; continue; }
            4 => { fl_l = rbx; fl_r = rsi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 7; continue; } bb = 5; continue; }
            5 => { bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; crate::simd_ops::wu32(rdi, ((rax as u32)) as u32); rdi = ((rdi.wrapping_add(P7)) as u64) as u64; rbx = ((rbx).wrapping_add(0x4u64)) as u64; fl_l = rbx; fl_r = rsi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 6; continue; } bb = 7; continue; }
            7 => { crate::simd_ops::wu32(r14.wrapping_add(P4), ((rbp as u32)) as u32); rax = (crate::simd_ops::ru64(r15)) as u64; crate::simd_ops::wu8(rax.wrapping_add(P0), (0x50u8) as u8); bb = 8; continue; }
            8 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411c56a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x40; const P1: u64 = 0x1; const P2: u64 = 0x1410c38f0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = (crate::simd_ops::ru64(rcx)) as u64; rbx = rdx; rsi = rcx; rax = (((crate::simd_ops::ru8(rdi.wrapping_add(P0))) as i8 as i64) as u64) as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x51u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru32(rdx)) as u32 as u64; crate::simd_ops::wu32(rdi, ((rax as u32)) as u32); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            2 => { rcx = ((rax.wrapping_add(P1)) as u64) as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x30u64), (0x0u8) as u8); r8 = rdi; rdx = rsp.wrapping_add(0x30); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu8(rdi.wrapping_add(P0), (0xffu8) as u8); rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; rcx = (crate::simd_ops::ru64(rsi)) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; crate::simd_ops::wu32(rcx, ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rsi)) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; crate::simd_ops::wu8(rax.wrapping_add(P0), (0x51u8) as u8); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411c5710(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1405af8d0; const P1: u64 = 0x14183acc8; const P2: u64 = 0x1410c38f0; const P3: u64 = 0x1401decf0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rdx; r15 = rcx; rsi = (crate::simd_ops::ru64(rcx)) as u64; rax = (((crate::simd_ops::ru8(rsi.wrapping_add(0x40u64))) as i8 as i64) as u64) as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x52u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 13; continue; } bb = 1; continue; }
            1 => { fl_l = rsi; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 19; continue; } bb = 2; continue; }
            2 => { rbp = (crate::simd_ops::ru32(rdx.wrapping_add(0x8u64))) as u32 as u64; r15 = ((rbp.wrapping_mul(4)) as u64) as u64; r14 = (crate::simd_ops::ru64(rdx)) as u64; rbx = (crate::simd_ops::ru64(rsi)) as u64; rdi = (crate::simd_ops::ru32(rsi.wrapping_add(0x8u64))) as u32 as u64; rax = (crate::simd_ops::ru32(rsi.wrapping_add(0xcu64))) as u32 as u64; fl_l = rbp; fl_r = rax; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rdi = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rsi.wrapping_add(0x8u64), ((rdi as u32)) as u32); rdx = ((rbp as u32)) as u32 as u64; rcx = rsi; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rsi)) as u64; bb = 10; continue; }
            4 => { fl_l = rbp; fl_r = rdi; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 9; continue; } bb = 5; continue; }
            5 => { r15 = ((rdi.wrapping_mul(4)) as u64) as u64; r12 = ((r15.wrapping_add(r14)) as u64) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { r8 = r15; rdx = r14; rcx = rbx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rbx).wrapping_add(r15)) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; bb = 7; continue; }
            7 => { r8 = rbp; fl_l = r8; fl_r = rdi; r8 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 12; continue; } bb = 8; continue; }
            8 => { r8 = ((r8.wrapping_mul(4)) as u64) as u64; rdx = r12; rcx = rbx; bb = 11; continue; }
            9 => { rcx = rbx; bb = 10; continue; }
            10 => { r8 = r15; rdx = r14; bb = 11; continue; }
            11 => { rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 12; continue; }
            12 => { crate::simd_ops::wu32(rsi.wrapping_add(0x8u64), ((rbp as u32)) as u32); bb = 19; continue; }
            13 => { crate::simd_ops::wu8(rsp.wrapping_add(0x50u64), (0x0u8) as u8); rcx = rax.wrapping_add(0x1); r8 = rsi; rdx = rsp.wrapping_add(0x50); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu8(rsi.wrapping_add(0x40u64), (0xffu8) as u8); r14 = (crate::simd_ops::ru64(r15)) as u64; rdi = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(r14, rdi); crate::simd_ops::wu64(r14.wrapping_add(0x8u64), rdi); rbp = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; rbx = (crate::simd_ops::ru64(rbx)) as u64; rsi = ((rbx.wrapping_add(rbp.wrapping_mul(4))) as u64) as u64; fl_l = rbp; fl_r = rbp; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 15; continue; } bb = 14; continue; }
            14 => { rdx = ((rbp as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(r14, rax); crate::simd_ops::wu32(r14.wrapping_add(0x8u64), ((rdi as u32)) as u32); crate::simd_ops::wu32(r14.wrapping_add(0xcu64), ((rbp as u32)) as u32); rdi = rax; bb = 15; continue; }
            15 => { fl_l = rbx; fl_r = rsi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 18; continue; } bb = 16; continue; }
            16 => { bb = 17; continue; }
            17 => { rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; crate::simd_ops::wu32(rdi, ((rax as u32)) as u32); rdi = rdi.wrapping_add(0x4); rbx = ((rbx).wrapping_add(0x4u64)) as u64; fl_l = rbx; fl_r = rsi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 17; continue; } bb = 18; continue; }
            18 => { crate::simd_ops::wu32(r14.wrapping_add(0x8u64), ((rbp as u32)) as u32); rax = (crate::simd_ops::ru64(r15)) as u64; crate::simd_ops::wu8(rax.wrapping_add(0x40u64), (0x52u8) as u8); bb = 19; continue; }
            19 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411c5860(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x140000000; const P1: u64 = 0x14020a8f0; const P2: u64 = 0x14241d6f0; const P3: u64 = 0x1411c7450; const P4: u64 = 0x1411c7650; const P5: u64 = 0x1410fd410; const P6: u64 = 0x1411c7850; const P7: u64 = 0x1411c7a50; const P8: u64 = 0x1411c7c50;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut j_tgt, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm2_hi: u128 = 0; let mut xmm4: u128 = 0; let mut ymm4_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rbx = (r8) as u64; r8 = (rdx) as u64; rdx = ((P0) as u64) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(rcx.wrapping_mul(4)).wrapping_add(0x11c6114u64))) as u32 as u64; fl_l = ((rax).wrapping_add(rdx)) as u64 as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; j_tgt = (rax) as u64; bb = 0xffff_fffe; continue; }
            1 => { rax = (crate::simd_ops::ru64(r8)) as u64; xmm1 = (crate::simd_ops::ru128(rax)) as u128; xmm0 = (crate::simd_ops::ru128(rbx)) as u128; crate::simd_ops::wu128(rax, (xmm0) as u128); crate::simd_ops::wu128(rbx, (xmm1) as u128); bb = 2; continue; }
            2 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            3 => { rdx = (crate::simd_ops::ru64(r8)) as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; rcx = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu64(rdx, (rcx) as u64); crate::simd_ops::wu64(rbx, (rax) as u64); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            4 => { rcx = (crate::simd_ops::ru64(r8)) as u64; rdx = ((crate::simd_ops::ru8(rcx)) as u32) as u32 as u64; rax = ((crate::simd_ops::ru8(rbx)) as u32) as u32 as u64; crate::simd_ops::wu8(rcx, ((rax as u8)) as u8); crate::simd_ops::wu8(rbx, ((rdx as u8)) as u8); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            5 => { rdx = (rbx) as u64; rcx = (crate::simd_ops::ru64(r8)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            6 => { rcx = (crate::simd_ops::ru64(r8)) as u64; xmm0 = ((crate::simd_ops::ru32(rcx)) as u32) as u128; rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; crate::simd_ops::wu32(rcx, ((rax as u32)) as u32); crate::simd_ops::wu32(rbx, ((xmm0 as u32)) as u32); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            7 => { rdx = (rbx) as u64; rcx = (crate::simd_ops::ru64(r8)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            8 => { rcx = (crate::simd_ops::ru64(r8)) as u64; rdx = (crate::simd_ops::ru32(rcx)) as u32 as u64; rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; crate::simd_ops::wu32(rcx, ((rax as u32)) as u32); crate::simd_ops::wu32(rbx, ((rdx as u32)) as u32); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            9 => { rdx = (rbx) as u64; rcx = (crate::simd_ops::ru64(r8)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            10 => { rdx = (rbx) as u64; rcx = (crate::simd_ops::ru64(r8)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            11 => { rcx = (crate::simd_ops::ru64(r8)) as u64; xmm1 = ((crate::simd_ops::ru64(rcx)) as u64) as u128; rdx = (crate::simd_ops::ru32(rcx.wrapping_add(0x8u64))) as u32 as u64; xmm0 = ((crate::simd_ops::ru64(rbx)) as u64) as u128; crate::simd_ops::wu64(rcx, ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x8u64), ((rax as u32)) as u32); crate::simd_ops::wu64(rbx, ((xmm1 as u64)) as u64); crate::simd_ops::wu32(rbx.wrapping_add(0x8u64), ((rdx as u32)) as u32); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            12 => { rdx = (rbx) as u64; rcx = (crate::simd_ops::ru64(r8)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            13 => { rdx = (rbx) as u64; rcx = (crate::simd_ops::ru64(r8)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            14 => { rdx = (rbx) as u64; rcx = (crate::simd_ops::ru64(r8)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            15 => { rdx = (rbx) as u64; rcx = (crate::simd_ops::ru64(r8)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            16 => { rdx = (rbx) as u64; rcx = (crate::simd_ops::ru64(r8)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            17 => { rdx = (crate::simd_ops::ru64(r8)) as u64; xmm3 = ((crate::simd_ops::ru64(rdx)) as u64) as u128; r9 = (crate::simd_ops::ru32(rdx.wrapping_add(0x8u64))) as u32 as u64; xmm2 = ((crate::simd_ops::ru64(rdx.wrapping_add(0xcu64))) as u64) as u128; ymm2_hi = 0; r8 = (crate::simd_ops::ru32(rdx.wrapping_add(0x14u64))) as u32 as u64; xmm1 = ((crate::simd_ops::ru64(rdx.wrapping_add(0x18u64))) as u64) as u128; rcx = (crate::simd_ops::ru32(rdx.wrapping_add(0x20u64))) as u32 as u64; xmm0 = ((crate::simd_ops::ru64(rbx)) as u64) as u128; crate::simd_ops::wu64(rdx, ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_add(0x8u64), ((rax as u32)) as u32); xmm0 = ((crate::simd_ops::ru64(rbx.wrapping_add(0xcu64))) as u64) as u128; crate::simd_ops::wu64(rdx.wrapping_add(0xcu64), ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x14u64))) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_add(0x14u64), ((rax as u32)) as u32); xmm0 = ((crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64) as u128; crate::simd_ops::wu64(rdx.wrapping_add(0x18u64), ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x20u64))) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_add(0x20u64), ((rax as u32)) as u32); crate::simd_ops::wu64(rbx, ((xmm3 as u64)) as u64); crate::simd_ops::wu32(rbx.wrapping_add(0x8u64), ((r9 as u32)) as u32); crate::simd_ops::wu64(rbx.wrapping_add(0xcu64), ((xmm2 as u64)) as u64); crate::simd_ops::wu32(rbx.wrapping_add(0x14u64), ((r8 as u32)) as u32); crate::simd_ops::wu64(rbx.wrapping_add(0x18u64), ((xmm1 as u64)) as u64); crate::simd_ops::wu32(rbx.wrapping_add(0x20u64), ((rcx as u32)) as u32); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            18 => { rdx = (rbx) as u64; rcx = (crate::simd_ops::ru64(r8)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            19 => { r8 = (crate::simd_ops::ru64(r8)) as u64; xmm4 = ((crate::simd_ops::ru64(r8)) as u64) as u128; ymm4_hi = 0; r10 = (crate::simd_ops::ru32(r8.wrapping_add(0x8u64))) as u32 as u64; xmm3 = ((crate::simd_ops::ru64(r8.wrapping_add(0xcu64))) as u64) as u128; r9 = (crate::simd_ops::ru32(r8.wrapping_add(0x14u64))) as u32 as u64; xmm2 = ((crate::simd_ops::ru64(r8.wrapping_add(0x18u64))) as u64) as u128; ymm2_hi = 0; rdx = (crate::simd_ops::ru32(r8.wrapping_add(0x20u64))) as u32 as u64; xmm1 = ((crate::simd_ops::ru64(r8.wrapping_add(0x24u64))) as u64) as u128; rcx = (crate::simd_ops::ru32(r8.wrapping_add(0x2cu64))) as u32 as u64; xmm0 = ((crate::simd_ops::ru64(rbx)) as u64) as u128; crate::simd_ops::wu64(r8, ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(0x8u64), ((rax as u32)) as u32); xmm0 = ((crate::simd_ops::ru64(rbx.wrapping_add(0xcu64))) as u64) as u128; crate::simd_ops::wu64(r8.wrapping_add(0xcu64), ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x14u64))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(0x14u64), ((rax as u32)) as u32); xmm0 = ((crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64) as u128; crate::simd_ops::wu64(r8.wrapping_add(0x18u64), ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x20u64))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(0x20u64), ((rax as u32)) as u32); xmm0 = ((crate::simd_ops::ru64(rbx.wrapping_add(0x24u64))) as u64) as u128; crate::simd_ops::wu64(r8.wrapping_add(0x24u64), ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x2cu64))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(0x2cu64), ((rax as u32)) as u32); crate::simd_ops::wu64(rbx, ((xmm4 as u64)) as u64); crate::simd_ops::wu32(rbx.wrapping_add(0x8u64), ((r10 as u32)) as u32); crate::simd_ops::wu64(rbx.wrapping_add(0xcu64), ((xmm3 as u64)) as u64); crate::simd_ops::wu32(rbx.wrapping_add(0x14u64), ((r9 as u32)) as u32); crate::simd_ops::wu64(rbx.wrapping_add(0x18u64), ((xmm2 as u64)) as u64); crate::simd_ops::wu32(rbx.wrapping_add(0x20u64), ((rdx as u32)) as u32); crate::simd_ops::wu64(rbx.wrapping_add(0x24u64), ((xmm1 as u64)) as u64); crate::simd_ops::wu32(rbx.wrapping_add(0x2cu64), ((rcx as u32)) as u32); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            20 => { rdx = (rbx) as u64; rcx = (crate::simd_ops::ru64(r8)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            21 => { rax = (crate::simd_ops::ru64(r8)) as u64; let y_addr: u64 = rax; xmm4 = core::ptr::read_unaligned(y_addr as *const u128); ymm4_hi = core::ptr::read_unaligned(y_addr.wrapping_add(16) as *const u128); let y_addr: u64 = rax.wrapping_add(0x20u64); xmm2 = core::ptr::read_unaligned(y_addr as *const u128); ymm2_hi = core::ptr::read_unaligned(y_addr.wrapping_add(16) as *const u128); xmm0 = (crate::simd_ops::ru128(rbx)) as u128; crate::simd_ops::wu128(rax, (xmm0) as u128); xmm0 = (crate::simd_ops::ru128(rbx.wrapping_add(0x10u64))) as u128; crate::simd_ops::wu128(rax.wrapping_add(0x10u64), (xmm0) as u128); xmm0 = (crate::simd_ops::ru128(rbx.wrapping_add(0x20u64))) as u128; crate::simd_ops::wu128(rax.wrapping_add(0x20u64), (xmm0) as u128); xmm0 = (crate::simd_ops::ru128(rbx.wrapping_add(0x30u64))) as u128; crate::simd_ops::wu128(rax.wrapping_add(0x30u64), (xmm0) as u128); let y_addr: u64 = rbx; core::ptr::write_unaligned(y_addr as *mut u128, xmm4); core::ptr::write_unaligned(y_addr.wrapping_add(16) as *mut u128, ymm4_hi); let y_addr: u64 = rbx.wrapping_add(0x20u64); core::ptr::write_unaligned(y_addr as *mut u128, xmm2); core::ptr::write_unaligned(y_addr.wrapping_add(16) as *mut u128, ymm2_hi); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            22 => { rdx = (rbx) as u64; rcx = (crate::simd_ops::ru64(r8)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            23 => { rcx = (crate::simd_ops::ru64(r8)) as u64; rdx = (crate::simd_ops::ru64(rcx)) as u64; rax = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu64(rcx, (rax) as u64); crate::simd_ops::wu64(rbx, (rdx) as u64); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            24 => { rdx = (rbx) as u64; rcx = (crate::simd_ops::ru64(r8)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            25 => { rdx = (rbx) as u64; rcx = (crate::simd_ops::ru64(r8)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            26 => { rdx = (rbx) as u64; rcx = (crate::simd_ops::ru64(r8)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            27 => { rdx = (crate::simd_ops::ru64(r8)) as u64; rax = (crate::simd_ops::ru32(rdx)) as u32 as u64; rcx = (crate::simd_ops::ru32(rbx)) as u32 as u64; crate::simd_ops::wu32(rdx, ((rcx as u32)) as u32); crate::simd_ops::wu32(rbx, ((rax as u32)) as u32); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            28 => { rdx = (rbx) as u64; rcx = (crate::simd_ops::ru64(r8)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            29 => { r8 = (crate::simd_ops::ru64(r8)) as u64; rcx = (crate::simd_ops::ru64(r8)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rcx) as u64); rax = (crate::simd_ops::ru32(r8.wrapping_add(0x8u64))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rax as u32)) as u32); rdi = (crate::simd_ops::ru64(r8.wrapping_add(0x10u64))) as u64; r9 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(r8.wrapping_add(0x10u64), (r9) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (rdi) as u64); rdx = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu64(r8, (rdx) as u64); rdx = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(0x8u64), ((rdx as u32)) as u32); crate::simd_ops::wu64(r8.wrapping_add(0x10u64), (r9) as u64); rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), (r9) as u64); crate::simd_ops::wu64(r8.wrapping_add(0x10u64), (rdx) as u64); crate::simd_ops::wu64(rbx, (rcx) as u64); crate::simd_ops::wu32(rbx.wrapping_add(0x8u64), ((rax as u32)) as u32); rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 33; continue; } bb = 30; continue; }
            30 => { rax = (0xffffffffu32) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rdx.wrapping_add(0x24u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 33; continue; } bb = 31; continue; }
            31 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x28u64))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 33; continue; } bb = 32; continue; }
            32 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 33; continue; }
            33 => { crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), (rdi) as u64); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            34 => { rdx = (rbx) as u64; rcx = (crate::simd_ops::ru64(r8)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            35 => { rcx = (crate::simd_ops::ru64(r8)) as u64; rdi = (crate::simd_ops::ru64(rcx)) as u64; r9 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rcx, (r9) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rdi) as u64); rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (rax) as u64); rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (rax) as u64); crate::simd_ops::wu64(rcx, (r9) as u64); rax = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu64(rbx, (r9) as u64); crate::simd_ops::wu64(rcx, (rax) as u64); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rcx.wrapping_add(0x8u64), (rax) as u64); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; crate::simd_ops::wu64(rcx.wrapping_add(0x10u64), (rax) as u64); rdx = (crate::simd_ops::ru64(rbx)) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 39; continue; } bb = 36; continue; }
            36 => { rax = (0xffffffffu32) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rdx.wrapping_add(0x24u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 39; continue; } bb = 37; continue; }
            37 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x28u64))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 39; continue; } bb = 38; continue; }
            38 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 39; continue; }
            39 => { crate::simd_ops::wu64(rbx, (rdi) as u64); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), (rax) as u64); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), (rax) as u64); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            40 => { rdx = (rbx) as u64; rcx = (crate::simd_ops::ru64(r8)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            41 => { rdx = (rbx) as u64; rcx = (crate::simd_ops::ru64(r8)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            42 => { rdx = (rbx) as u64; rcx = (crate::simd_ops::ru64(r8)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            43 => { rdx = (rbx) as u64; rcx = (crate::simd_ops::ru64(r8)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            44 => { rdx = (rbx) as u64; rcx = (crate::simd_ops::ru64(r8)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            45 => { rdx = (rbx) as u64; rcx = (crate::simd_ops::ru64(r8)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            46 => { rdx = (rbx) as u64; rcx = (crate::simd_ops::ru64(r8)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            47 => { rdx = (rbx) as u64; rcx = (crate::simd_ops::ru64(r8)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            48 => { rdx = (rbx) as u64; rcx = (crate::simd_ops::ru64(r8)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            49 => { rcx = (crate::simd_ops::ru64(r8)) as u64; rdi = (crate::simd_ops::ru64(rcx)) as u64; r9 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), (rdi) as u64); crate::simd_ops::wu64(rcx, (r9) as u64); rax = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu64(rbx, (r9) as u64); crate::simd_ops::wu64(rcx, (rax) as u64); rdx = (crate::simd_ops::ru64(rbx)) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 53; continue; } bb = 50; continue; }
            50 => { rax = (0xffffffffu32) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rdx.wrapping_add(0x24u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 53; continue; } bb = 51; continue; }
            51 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x28u64))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 53; continue; } bb = 52; continue; }
            52 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 53; continue; }
            53 => { crate::simd_ops::wu64(rbx, (rdi) as u64); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            54 => { rdx = (rbx) as u64; rcx = (crate::simd_ops::ru64(r8)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            55 => { rcx = (crate::simd_ops::ru64(r8)) as u64; rdi = (crate::simd_ops::ru64(rcx)) as u64; r9 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), (rdi) as u64); crate::simd_ops::wu64(rcx, (r9) as u64); rax = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu64(rbx, (r9) as u64); crate::simd_ops::wu64(rcx, (rax) as u64); rdx = (crate::simd_ops::ru64(rbx)) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 59; continue; } bb = 56; continue; }
            56 => { rax = (0xffffffffu32) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rdx.wrapping_add(0x24u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 59; continue; } bb = 57; continue; }
            57 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x28u64))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 59; continue; } bb = 58; continue; }
            58 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 59; continue; }
            59 => { crate::simd_ops::wu64(rbx, (rdi) as u64); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            60 => { rdx = (rbx) as u64; rcx = (crate::simd_ops::ru64(r8)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            61 => { rcx = (crate::simd_ops::ru64(r8)) as u64; rdi = (crate::simd_ops::ru64(rcx)) as u64; r9 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), (rdi) as u64); crate::simd_ops::wu64(rcx, (r9) as u64); rax = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu64(rbx, (r9) as u64); crate::simd_ops::wu64(rcx, (rax) as u64); rdx = (crate::simd_ops::ru64(rbx)) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 65; continue; } bb = 62; continue; }
            62 => { rax = (0xffffffffu32) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rdx.wrapping_add(0x24u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 65; continue; } bb = 63; continue; }
            63 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x28u64))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 65; continue; } bb = 64; continue; }
            64 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 65; continue; }
            65 => { crate::simd_ops::wu64(rbx, (rdi) as u64); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            66 => { rdx = (rbx) as u64; rcx = (crate::simd_ops::ru64(r8)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            67 => { rcx = (crate::simd_ops::ru64(r8)) as u64; rdi = (crate::simd_ops::ru64(rcx)) as u64; r9 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), (rdi) as u64); crate::simd_ops::wu64(rcx, (r9) as u64); rax = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu64(rbx, (r9) as u64); crate::simd_ops::wu64(rcx, (rax) as u64); rdx = (crate::simd_ops::ru64(rbx)) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 71; continue; } bb = 68; continue; }
            68 => { rax = (0xffffffffu32) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rdx.wrapping_add(0x24u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 71; continue; } bb = 69; continue; }
            69 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x28u64))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 71; continue; } bb = 70; continue; }
            70 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 71; continue; }
            71 => { crate::simd_ops::wu64(rbx, (rdi) as u64); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            72 => { rdx = (rbx) as u64; rcx = (crate::simd_ops::ru64(r8)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            73 => { rdx = (rbx) as u64; rcx = (crate::simd_ops::ru64(r8)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            74 => { rdx = (rbx) as u64; rcx = (crate::simd_ops::ru64(r8)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            75 => { rdx = (rbx) as u64; rcx = (crate::simd_ops::ru64(r8)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            76 => { rdx = (rbx) as u64; rcx = (crate::simd_ops::ru64(r8)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            77 => { rdx = (rbx) as u64; rcx = (crate::simd_ops::ru64(r8)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            78 => { rdx = (rbx) as u64; rcx = (crate::simd_ops::ru64(r8)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            79 => { rdx = (rbx) as u64; rcx = (crate::simd_ops::ru64(r8)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            80 => { rdx = (rbx) as u64; rcx = (crate::simd_ops::ru64(r8)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            81 => { rdx = (rbx) as u64; rcx = (crate::simd_ops::ru64(r8)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            82 => { rdx = (rbx) as u64; rcx = (crate::simd_ops::ru64(r8)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            83 => { rdx = (rbx) as u64; rcx = (crate::simd_ops::ru64(r8)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            0xffff_fffe => { match j_tgt.wrapping_sub(0x1411c5860u64) { 0x23 => { bb = 1; continue; } 0x36 => { bb = 2; continue; } 0x44 => { bb = 3; continue; } 0x61 => { bb = 4; continue; } 0x7c => { bb = 5; continue; } 0x95 => { bb = 6; continue; } 0xb2 => { bb = 7; continue; } 0xcb => { bb = 8; continue; } 0xe4 => { bb = 9; continue; } 0xfd => { bb = 10; continue; } 0x116 => { bb = 11; continue; } 0x143 => { bb = 12; continue; } 0x15c => { bb = 13; continue; } 0x175 => { bb = 14; continue; } 0x18e => { bb = 15; continue; } 0x1a7 => { bb = 16; continue; } 0x1c0 => { bb = 17; continue; } 0x231 => { bb = 18; continue; } 0x24a => { bb = 19; continue; } 0x2e9 => { bb = 20; continue; } 0x302 => { bb = 21; continue; } 0x34b => { bb = 22; continue; } 0x364 => { bb = 23; continue; } 0x381 => { bb = 24; continue; } 0x39a => { bb = 25; continue; } 0x3b3 => { bb = 26; continue; } 0x3cc => { bb = 27; continue; } 0x3e5 => { bb = 28; continue; } 0x3fe => { bb = 29; continue; } 0x44d => { bb = 30; continue; } 0x45c => { bb = 31; continue; } 0x465 => { bb = 32; continue; } 0x46a => { bb = 33; continue; } 0x47c => { bb = 34; continue; } 0x495 => { bb = 35; continue; } 0x4dc => { bb = 36; continue; } 0x4eb => { bb = 37; continue; } 0x4f4 => { bb = 38; continue; } 0x4f9 => { bb = 39; continue; } 0x51c => { bb = 40; continue; } 0x535 => { bb = 41; continue; } 0x54e => { bb = 42; continue; } 0x567 => { bb = 43; continue; } 0x580 => { bb = 44; continue; } 0x599 => { bb = 45; continue; } 0x5b2 => { bb = 46; continue; } 0x5cb => { bb = 47; continue; } 0x5e4 => { bb = 48; continue; } 0x5fd => { bb = 49; continue; } 0x61f => { bb = 50; continue; } 0x62e => { bb = 51; continue; } 0x637 => { bb = 52; continue; } 0x63c => { bb = 53; continue; } 0x64d => { bb = 54; continue; } 0x666 => { bb = 55; continue; } 0x688 => { bb = 56; continue; } 0x697 => { bb = 57; continue; } 0x6a0 => { bb = 58; continue; } 0x6a5 => { bb = 59; continue; } 0x6b6 => { bb = 60; continue; } 0x6cf => { bb = 61; continue; } 0x6f1 => { bb = 62; continue; } 0x700 => { bb = 63; continue; } 0x709 => { bb = 64; continue; } 0x70e => { bb = 65; continue; } 0x71f => { bb = 66; continue; } 0x738 => { bb = 67; continue; } 0x75a => { bb = 68; continue; } 0x769 => { bb = 69; continue; } 0x772 => { bb = 70; continue; } 0x777 => { bb = 71; continue; } 0x788 => { bb = 72; continue; } 0x7a1 => { bb = 73; continue; } 0x7ba => { bb = 74; continue; } 0x7d3 => { bb = 75; continue; } 0x7ec => { bb = 76; continue; } 0x805 => { bb = 77; continue; } 0x81e => { bb = 78; continue; } 0x837 => { bb = 79; continue; } 0x850 => { bb = 80; continue; } 0x869 => { bb = 81; continue; } 0x882 => { bb = 82; continue; } 0x89b => { bb = 83; continue; } _ => { let s5 = crate::simd_ops::read_s5_tail(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x40) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(j_tgt, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::set_xmm_ret(c_x0); return c_rax; } } }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411c6270(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x140000000; const P1: u64 = 0x1411c68b0; const P2: u64 = 0x1411c68e0; const P3: u64 = 0x1411c6910; const P4: u64 = 0x1411c6940; const P5: u64 = 0x1411c6980; const P6: u64 = 0x1411c69b0; const P7: u64 = 0x1411c6a00; const P8: u64 = 0x1411c6a30; const P9: u64 = 0x1411c6a60; const P10: u64 = 0x1411c6a90; const P11: u64 = 0x1411c6ac0; const P12: u64 = 0x1411c6af0; const P13: u64 = 0x1411c6b20; const P14: u64 = 0x1411c6b50; const P15: u64 = 0x1411c6b80; const P16: u64 = 0x1411c6bb0; const P17: u64 = 0x1411c6be0; const P18: u64 = 0x1411c6c10; const P19: u64 = 0x1411c6c40; const P20: u64 = 0x1411c6c70; const P21: u64 = 0x1411c6ca0; const P22: u64 = 0x1411c6cd0; const P23: u64 = 0x1411c6d00; const P24: u64 = 0x1411c6d30; const P25: u64 = 0x1411c6d60; const P26: u64 = 0x1411c6d90; const P27: u64 = 0x1411c6db0; const P28: u64 = 0x1411c6de0; const P29: u64 = 0x1411c6e00; const P30: u64 = 0x1411c6e30; const P31: u64 = 0x1411c6e50; const P32: u64 = 0x1411c6e80; const P33: u64 = 0x1411c6ea0; const P34: u64 = 0x1411c6ed0; const P35: u64 = 0x1411c6f00; const P36: u64 = 0x1411c6f30; const P37: u64 = 0x1411c6f60; const P38: u64 = 0x1411c6f90; const P39: u64 = 0x1411c6fc0; const P40: u64 = 0x1411c6ff0; const P41: u64 = 0x1411c7020; const P42: u64 = 0x1411c7050; const P43: u64 = 0x1411c7080; const P44: u64 = 0x1411c70b0;
    let (mut rax, mut fl_l, mut fl_r, mut j_tgt) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r9 = rdx; rdx = P0; rax = (crate::simd_ops::ru32(rdx.wrapping_add(rcx.wrapping_mul(4)).wrapping_add(0x11c6758u64))) as u32 as u64; fl_l = ((rax).wrapping_add(rdx)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; j_tgt = rax; bb = 0xffff_fffe; continue; }
            1 => { rax = (crate::simd_ops::ru64(r9)) as u64; crate::simd_ops::wu8(rax.wrapping_add(0x40u64), (0x0u8) as u8); bb = 2; continue; }
            2 => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            3 => { rax = (crate::simd_ops::ru64(r9)) as u64; xmm0 = (crate::simd_ops::ru128(r8)) as u128; crate::simd_ops::wu128(rax, (xmm0) as u128); crate::simd_ops::wu8(rax.wrapping_add(0x40u64), (0x1u8) as u8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            4 => { rcx = (crate::simd_ops::ru64(r9)) as u64; rax = (crate::simd_ops::ru64(r8)) as u64; crate::simd_ops::wu64(rcx, rax); crate::simd_ops::wu8(rcx.wrapping_add(0x40u64), (0x2u8) as u8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            5 => { rcx = (crate::simd_ops::ru64(r9)) as u64; rax = ((crate::simd_ops::ru8(r8)) as u32) as u32 as u64; crate::simd_ops::wu8(rcx, ((rax as u8)) as u8); crate::simd_ops::wu8(rcx.wrapping_add(0x40u64), (0x3u8) as u8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            6 => { rcx = (crate::simd_ops::ru64(r9)) as u64; rdx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rcx.wrapping_add(0x8u64), rdx); crate::simd_ops::wu64(rcx, rdx); rax = (crate::simd_ops::ru64(r8)) as u64; crate::simd_ops::wu64(rcx, rax); rax = (crate::simd_ops::ru32(r8.wrapping_add(0x8u64))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x8u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(r8.wrapping_add(0xcu64))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(0xcu64), ((rax as u32)) as u32); crate::simd_ops::wu64(r8, rdx); crate::simd_ops::wu64(r8.wrapping_add(0x8u64), rdx); crate::simd_ops::wu8(rcx.wrapping_add(0x40u64), (0x4u8) as u8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            7 => { rcx = (crate::simd_ops::ru64(r9)) as u64; rax = (crate::simd_ops::ru32(r8)) as u32 as u64; crate::simd_ops::wu32(rcx, ((rax as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x40u64), (0x5u8) as u8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            8 => { rcx = (crate::simd_ops::ru64(r9)) as u64; rdx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rcx.wrapping_add(0x8u64), rdx); crate::simd_ops::wu64(rcx, rdx); rax = (crate::simd_ops::ru64(r8)) as u64; crate::simd_ops::wu64(rcx, rax); rax = (crate::simd_ops::ru32(r8.wrapping_add(0x8u64))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x8u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(r8.wrapping_add(0xcu64))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(0xcu64), ((rax as u32)) as u32); crate::simd_ops::wu64(r8, rdx); crate::simd_ops::wu64(r8.wrapping_add(0x8u64), rdx); crate::simd_ops::wu8(rcx.wrapping_add(0x40u64), (0x6u8) as u8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            9 => { rcx = (crate::simd_ops::ru64(r9)) as u64; rax = (crate::simd_ops::ru32(r8)) as u32 as u64; crate::simd_ops::wu32(rcx, ((rax as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x40u64), (0x7u8) as u8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            10 => { rcx = (crate::simd_ops::ru64(r9)) as u64; rdx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rcx.wrapping_add(0x8u64), rdx); crate::simd_ops::wu64(rcx, rdx); rax = (crate::simd_ops::ru64(r8)) as u64; crate::simd_ops::wu64(rcx, rax); rax = (crate::simd_ops::ru32(r8.wrapping_add(0x8u64))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x8u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(r8.wrapping_add(0xcu64))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(0xcu64), ((rax as u32)) as u32); crate::simd_ops::wu64(r8, rdx); crate::simd_ops::wu64(r8.wrapping_add(0x8u64), rdx); crate::simd_ops::wu8(rcx.wrapping_add(0x40u64), (0x8u8) as u8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            11 => { rcx = (crate::simd_ops::ru64(r9)) as u64; rax = (crate::simd_ops::ru64(r8)) as u64; crate::simd_ops::wu64(rcx, rax); crate::simd_ops::wu8(rcx.wrapping_add(0x40u64), (0x9u8) as u8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            12 => { rcx = (crate::simd_ops::ru64(r9)) as u64; rdx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rcx.wrapping_add(0x8u64), rdx); crate::simd_ops::wu64(rcx, rdx); rax = (crate::simd_ops::ru64(r8)) as u64; crate::simd_ops::wu64(rcx, rax); rax = (crate::simd_ops::ru32(r8.wrapping_add(0x8u64))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x8u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(r8.wrapping_add(0xcu64))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(0xcu64), ((rax as u32)) as u32); crate::simd_ops::wu64(r8, rdx); crate::simd_ops::wu64(r8.wrapping_add(0x8u64), rdx); crate::simd_ops::wu8(rcx.wrapping_add(0x40u64), (0xau8) as u8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            13 => { rcx = (crate::simd_ops::ru64(r9)) as u64; xmm0 = ((crate::simd_ops::ru64(r8)) as u64) as u128; crate::simd_ops::wu64(rcx, ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(r8.wrapping_add(0x8u64))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x8u64), ((rax as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x40u64), (0xbu8) as u8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            14 => { rcx = (crate::simd_ops::ru64(r9)) as u64; rdx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rcx.wrapping_add(0x8u64), rdx); crate::simd_ops::wu64(rcx, rdx); rax = (crate::simd_ops::ru64(r8)) as u64; crate::simd_ops::wu64(rcx, rax); rax = (crate::simd_ops::ru32(r8.wrapping_add(0x8u64))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x8u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(r8.wrapping_add(0xcu64))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(0xcu64), ((rax as u32)) as u32); crate::simd_ops::wu64(r8, rdx); crate::simd_ops::wu64(r8.wrapping_add(0x8u64), rdx); crate::simd_ops::wu8(rcx.wrapping_add(0x40u64), (0xcu8) as u8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            15 => { rax = (crate::simd_ops::ru64(r9)) as u64; xmm0 = (crate::simd_ops::ru128(r8)) as u128; crate::simd_ops::wu128(rax, (xmm0) as u128); crate::simd_ops::wu8(rax.wrapping_add(0x40u64), (0xdu8) as u8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            16 => { rcx = (crate::simd_ops::ru64(r9)) as u64; rdx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rcx.wrapping_add(0x8u64), rdx); crate::simd_ops::wu64(rcx, rdx); rax = (crate::simd_ops::ru64(r8)) as u64; crate::simd_ops::wu64(rcx, rax); rax = (crate::simd_ops::ru32(r8.wrapping_add(0x8u64))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x8u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(r8.wrapping_add(0xcu64))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(0xcu64), ((rax as u32)) as u32); crate::simd_ops::wu64(r8, rdx); crate::simd_ops::wu64(r8.wrapping_add(0x8u64), rdx); crate::simd_ops::wu8(rcx.wrapping_add(0x40u64), (0xeu8) as u8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            17 => { rcx = (crate::simd_ops::ru64(r9)) as u64; rax = (crate::simd_ops::ru64(r8)) as u64; crate::simd_ops::wu64(rcx, rax); crate::simd_ops::wu8(rcx.wrapping_add(0x40u64), (0xfu8) as u8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            18 => { rdx = r8; rcx = r9; return crate::simd_ops::tail_fwd_xmm(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8, [xmm0, xmm1, xmm2, xmm3]); }
            19 => { rcx = (crate::simd_ops::ru64(r9)) as u64; xmm0 = ((crate::simd_ops::ru64(r8)) as u64) as u128; crate::simd_ops::wu64(rcx, ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(r8.wrapping_add(0x8u64))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x8u64), ((rax as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x40u64), (0x11u8) as u8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            20 => { rdx = r8; rcx = r9; return crate::simd_ops::tail_fwd_xmm(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8, [xmm0, xmm1, xmm2, xmm3]); }
            21 => { rax = (crate::simd_ops::ru64(r9)) as u64; xmm0 = (crate::simd_ops::ru128(r8)) as u128; crate::simd_ops::wu128(rax, (xmm0) as u128); crate::simd_ops::wu8(rax.wrapping_add(0x40u64), (0x13u8) as u8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            22 => { rdx = r8; rcx = r9; return crate::simd_ops::tail_fwd_xmm(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8, [xmm0, xmm1, xmm2, xmm3]); }
            23 => { rdx = r8; rcx = r9; return crate::simd_ops::tail_fwd_xmm(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8, [xmm0, xmm1, xmm2, xmm3]); }
            24 => { rdx = r8; rcx = r9; return crate::simd_ops::tail_fwd_xmm(P5, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8, [xmm0, xmm1, xmm2, xmm3]); }
            25 => { rdx = r8; rcx = r9; return crate::simd_ops::tail_fwd_xmm(P6, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8, [xmm0, xmm1, xmm2, xmm3]); }
            26 => { rdx = r8; rcx = r9; return crate::simd_ops::tail_fwd_xmm(P7, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8, [xmm0, xmm1, xmm2, xmm3]); }
            27 => { rdx = r8; rcx = r9; return crate::simd_ops::tail_fwd_xmm(P8, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8, [xmm0, xmm1, xmm2, xmm3]); }
            28 => { rdx = r8; rcx = r9; return crate::simd_ops::tail_fwd_xmm(P9, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8, [xmm0, xmm1, xmm2, xmm3]); }
            29 => { rcx = (crate::simd_ops::ru64(r9)) as u64; rax = (crate::simd_ops::ru64(r8)) as u64; crate::simd_ops::wu64(rcx, rax); crate::simd_ops::wu8(rcx.wrapping_add(0x40u64), (0x1bu8) as u8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            30 => { rdx = r8; rcx = r9; return crate::simd_ops::tail_fwd_xmm(P10, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8, [xmm0, xmm1, xmm2, xmm3]); }
            31 => { rcx = (crate::simd_ops::ru64(r9)) as u64; rax = (crate::simd_ops::ru64(r8)) as u64; crate::simd_ops::wu64(rcx, rax); crate::simd_ops::wu8(rcx.wrapping_add(0x40u64), (0x1du8) as u8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            32 => { rdx = r8; rcx = r9; return crate::simd_ops::tail_fwd_xmm(P11, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8, [xmm0, xmm1, xmm2, xmm3]); }
            33 => { rax = (crate::simd_ops::ru64(r9)) as u64; xmm0 = (crate::simd_ops::ru128(r8)) as u128; crate::simd_ops::wu128(rax, (xmm0) as u128); crate::simd_ops::wu8(rax.wrapping_add(0x40u64), (0x1fu8) as u8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            34 => { rdx = r8; rcx = r9; return crate::simd_ops::tail_fwd_xmm(P12, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8, [xmm0, xmm1, xmm2, xmm3]); }
            35 => { rcx = (crate::simd_ops::ru64(r9)) as u64; rax = (crate::simd_ops::ru32(r8)) as u32 as u64; crate::simd_ops::wu32(rcx, ((rax as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x40u64), (0x21u8) as u8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            36 => { rdx = r8; rcx = r9; return crate::simd_ops::tail_fwd_xmm(P13, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8, [xmm0, xmm1, xmm2, xmm3]); }
            37 => { rdx = r8; rcx = r9; return crate::simd_ops::tail_fwd_xmm(P14, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8, [xmm0, xmm1, xmm2, xmm3]); }
            38 => { rdx = r8; rcx = r9; return crate::simd_ops::tail_fwd_xmm(P15, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8, [xmm0, xmm1, xmm2, xmm3]); }
            39 => { rdx = r8; rcx = r9; return crate::simd_ops::tail_fwd_xmm(P16, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8, [xmm0, xmm1, xmm2, xmm3]); }
            40 => { rdx = r8; rcx = r9; return crate::simd_ops::tail_fwd_xmm(P17, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8, [xmm0, xmm1, xmm2, xmm3]); }
            41 => { rax = (crate::simd_ops::ru64(r9)) as u64; xmm0 = (crate::simd_ops::ru128(r8)) as u128; crate::simd_ops::wu128(rax, (xmm0) as u128); crate::simd_ops::wu8(rax.wrapping_add(0x40u64), (0x27u8) as u8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            42 => { rdx = r8; rcx = r9; return crate::simd_ops::tail_fwd_xmm(P18, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8, [xmm0, xmm1, xmm2, xmm3]); }
            43 => { rax = (crate::simd_ops::ru64(r9)) as u64; xmm0 = (crate::simd_ops::ru128(r8)) as u128; crate::simd_ops::wu128(rax, (xmm0) as u128); crate::simd_ops::wu8(rax.wrapping_add(0x40u64), (0x29u8) as u8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            44 => { rdx = r8; rcx = r9; return crate::simd_ops::tail_fwd_xmm(P19, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8, [xmm0, xmm1, xmm2, xmm3]); }
            45 => { rax = (crate::simd_ops::ru64(r9)) as u64; xmm0 = (crate::simd_ops::ru128(r8)) as u128; crate::simd_ops::wu128(rax, (xmm0) as u128); crate::simd_ops::wu8(rax.wrapping_add(0x40u64), (0x2bu8) as u8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            46 => { rdx = r8; rcx = r9; return crate::simd_ops::tail_fwd_xmm(P20, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8, [xmm0, xmm1, xmm2, xmm3]); }
            47 => { rcx = (crate::simd_ops::ru64(r9)) as u64; rax = (crate::simd_ops::ru64(r8)) as u64; crate::simd_ops::wu64(rcx, rax); crate::simd_ops::wu8(rcx.wrapping_add(0x40u64), (0x2du8) as u8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            48 => { rdx = r8; rcx = r9; return crate::simd_ops::tail_fwd_xmm(P21, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8, [xmm0, xmm1, xmm2, xmm3]); }
            49 => { rcx = (crate::simd_ops::ru64(r9)) as u64; rax = (crate::simd_ops::ru64(r8)) as u64; crate::simd_ops::wu64(rcx, rax); crate::simd_ops::wu8(rcx.wrapping_add(0x40u64), (0x2fu8) as u8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            50 => { rdx = r8; rcx = r9; return crate::simd_ops::tail_fwd_xmm(P22, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8, [xmm0, xmm1, xmm2, xmm3]); }
            51 => { rcx = (crate::simd_ops::ru64(r9)) as u64; rax = (crate::simd_ops::ru64(r8)) as u64; crate::simd_ops::wu64(rcx, rax); crate::simd_ops::wu8(rcx.wrapping_add(0x40u64), (0x31u8) as u8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            52 => { rdx = r8; rcx = r9; return crate::simd_ops::tail_fwd_xmm(P23, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8, [xmm0, xmm1, xmm2, xmm3]); }
            53 => { rdx = r8; rcx = r9; return crate::simd_ops::tail_fwd_xmm(P24, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8, [xmm0, xmm1, xmm2, xmm3]); }
            54 => { rdx = r8; rcx = r9; return crate::simd_ops::tail_fwd_xmm(P25, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8, [xmm0, xmm1, xmm2, xmm3]); }
            55 => { rdx = r8; rcx = r9; return crate::simd_ops::tail_fwd_xmm(P26, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8, [xmm0, xmm1, xmm2, xmm3]); }
            56 => { rdx = r8; rcx = r9; return crate::simd_ops::tail_fwd_xmm(P27, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8, [xmm0, xmm1, xmm2, xmm3]); }
            57 => { rdx = r8; rcx = r9; return crate::simd_ops::tail_fwd_xmm(P28, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8, [xmm0, xmm1, xmm2, xmm3]); }
            58 => { rdx = r8; rcx = r9; return crate::simd_ops::tail_fwd_xmm(P29, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8, [xmm0, xmm1, xmm2, xmm3]); }
            59 => { rdx = r8; rcx = r9; return crate::simd_ops::tail_fwd_xmm(P30, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8, [xmm0, xmm1, xmm2, xmm3]); }
            60 => { rdx = r8; rcx = r9; return crate::simd_ops::tail_fwd_xmm(P31, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8, [xmm0, xmm1, xmm2, xmm3]); }
            61 => { rdx = r8; rcx = r9; return crate::simd_ops::tail_fwd_xmm(P32, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8, [xmm0, xmm1, xmm2, xmm3]); }
            62 => { rdx = r8; rcx = r9; return crate::simd_ops::tail_fwd_xmm(P33, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8, [xmm0, xmm1, xmm2, xmm3]); }
            63 => { rcx = (crate::simd_ops::ru64(r9)) as u64; rax = (crate::simd_ops::ru32(r8)) as u32 as u64; crate::simd_ops::wu32(rcx, ((rax as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x40u64), (0x3du8) as u8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            64 => { rdx = r8; rcx = r9; return crate::simd_ops::tail_fwd_xmm(P34, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8, [xmm0, xmm1, xmm2, xmm3]); }
            65 => { rcx = (crate::simd_ops::ru64(r9)) as u64; rax = (crate::simd_ops::ru32(r8)) as u32 as u64; crate::simd_ops::wu32(rcx, ((rax as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x40u64), (0x3fu8) as u8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            66 => { rdx = r8; rcx = r9; return crate::simd_ops::tail_fwd_xmm(P35, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8, [xmm0, xmm1, xmm2, xmm3]); }
            67 => { rcx = (crate::simd_ops::ru64(r9)) as u64; rax = (crate::simd_ops::ru32(r8)) as u32 as u64; crate::simd_ops::wu32(rcx, ((rax as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x40u64), (0x41u8) as u8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            68 => { rdx = r8; rcx = r9; return crate::simd_ops::tail_fwd_xmm(P36, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8, [xmm0, xmm1, xmm2, xmm3]); }
            69 => { rcx = (crate::simd_ops::ru64(r9)) as u64; rax = (crate::simd_ops::ru32(r8)) as u32 as u64; crate::simd_ops::wu32(rcx, ((rax as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x40u64), (0x43u8) as u8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            70 => { rdx = r8; rcx = r9; return crate::simd_ops::tail_fwd_xmm(P37, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8, [xmm0, xmm1, xmm2, xmm3]); }
            71 => { rcx = (crate::simd_ops::ru64(r9)) as u64; rax = (crate::simd_ops::ru64(r8)) as u64; crate::simd_ops::wu64(rcx, rax); crate::simd_ops::wu8(rcx.wrapping_add(0x40u64), (0x45u8) as u8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            72 => { rdx = r8; rcx = r9; return crate::simd_ops::tail_fwd_xmm(P38, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8, [xmm0, xmm1, xmm2, xmm3]); }
            73 => { rcx = (crate::simd_ops::ru64(r9)) as u64; rax = (crate::simd_ops::ru64(r8)) as u64; crate::simd_ops::wu64(rcx, rax); crate::simd_ops::wu8(rcx.wrapping_add(0x40u64), (0x47u8) as u8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            74 => { rdx = r8; rcx = r9; return crate::simd_ops::tail_fwd_xmm(P39, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8, [xmm0, xmm1, xmm2, xmm3]); }
            75 => { rcx = (crate::simd_ops::ru64(r9)) as u64; rax = (crate::simd_ops::ru64(r8)) as u64; crate::simd_ops::wu64(rcx, rax); crate::simd_ops::wu8(rcx.wrapping_add(0x40u64), (0x49u8) as u8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            76 => { rdx = r8; rcx = r9; return crate::simd_ops::tail_fwd_xmm(P40, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8, [xmm0, xmm1, xmm2, xmm3]); }
            77 => { rcx = (crate::simd_ops::ru64(r9)) as u64; rax = (crate::simd_ops::ru32(r8)) as u32 as u64; crate::simd_ops::wu32(rcx, ((rax as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x40u64), (0x4bu8) as u8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            78 => { rdx = r8; rcx = r9; return crate::simd_ops::tail_fwd_xmm(P41, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8, [xmm0, xmm1, xmm2, xmm3]); }
            79 => { rcx = (crate::simd_ops::ru64(r9)) as u64; rax = (crate::simd_ops::ru32(r8)) as u32 as u64; crate::simd_ops::wu32(rcx, ((rax as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x40u64), (0x4du8) as u8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            80 => { rdx = r8; rcx = r9; return crate::simd_ops::tail_fwd_xmm(P42, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8, [xmm0, xmm1, xmm2, xmm3]); }
            81 => { rcx = (crate::simd_ops::ru64(r9)) as u64; rax = (crate::simd_ops::ru32(r8)) as u32 as u64; crate::simd_ops::wu32(rcx, ((rax as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x40u64), (0x4fu8) as u8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            82 => { rdx = r8; rcx = r9; return crate::simd_ops::tail_fwd_xmm(P43, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8, [xmm0, xmm1, xmm2, xmm3]); }
            83 => { rcx = (crate::simd_ops::ru64(r9)) as u64; rax = (crate::simd_ops::ru32(r8)) as u32 as u64; crate::simd_ops::wu32(rcx, ((rax as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x40u64), (0x51u8) as u8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            84 => { rdx = r8; rcx = r9; return crate::simd_ops::tail_fwd_xmm(P44, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8, [xmm0, xmm1, xmm2, xmm3]); }
            0xffff_fffe => { match j_tgt.wrapping_sub(0x1411c6270u64) { 0x16 => { bb = 1; continue; } 0x1d => { bb = 2; continue; } 0x1e => { bb = 3; continue; } 0x2f => { bb = 4; continue; } 0x3d => { bb = 5; continue; } 0x4b => { bb = 6; continue; } 0x77 => { bb = 7; continue; } 0x84 => { bb = 8; continue; } 0xb0 => { bb = 9; continue; } 0xbd => { bb = 10; continue; } 0xe9 => { bb = 11; continue; } 0xf7 => { bb = 12; continue; } 0x123 => { bb = 13; continue; } 0x13b => { bb = 14; continue; } 0x167 => { bb = 15; continue; } 0x178 => { bb = 16; continue; } 0x1a4 => { bb = 17; continue; } 0x1b2 => { bb = 18; continue; } 0x1bd => { bb = 19; continue; } 0x1d5 => { bb = 20; continue; } 0x1e0 => { bb = 21; continue; } 0x1f1 => { bb = 22; continue; } 0x1fc => { bb = 23; continue; } 0x207 => { bb = 24; continue; } 0x212 => { bb = 25; continue; } 0x21d => { bb = 26; continue; } 0x228 => { bb = 27; continue; } 0x233 => { bb = 28; continue; } 0x23e => { bb = 29; continue; } 0x24c => { bb = 30; continue; } 0x257 => { bb = 31; continue; } 0x265 => { bb = 32; continue; } 0x270 => { bb = 33; continue; } 0x281 => { bb = 34; continue; } 0x28c => { bb = 35; continue; } 0x299 => { bb = 36; continue; } 0x2a4 => { bb = 37; continue; } 0x2af => { bb = 38; continue; } 0x2ba => { bb = 39; continue; } 0x2c5 => { bb = 40; continue; } 0x2d0 => { bb = 41; continue; } 0x2e1 => { bb = 42; continue; } 0x2ec => { bb = 43; continue; } 0x2fd => { bb = 44; continue; } 0x308 => { bb = 45; continue; } 0x319 => { bb = 46; continue; } 0x324 => { bb = 47; continue; } 0x332 => { bb = 48; continue; } 0x33d => { bb = 49; continue; } 0x34b => { bb = 50; continue; } 0x356 => { bb = 51; continue; } 0x364 => { bb = 52; continue; } 0x36f => { bb = 53; continue; } 0x37a => { bb = 54; continue; } 0x385 => { bb = 55; continue; } 0x390 => { bb = 56; continue; } 0x39b => { bb = 57; continue; } 0x3a6 => { bb = 58; continue; } 0x3b1 => { bb = 59; continue; } 0x3bc => { bb = 60; continue; } 0x3c7 => { bb = 61; continue; } 0x3d2 => { bb = 62; continue; } 0x3dd => { bb = 63; continue; } 0x3ea => { bb = 64; continue; } 0x3f5 => { bb = 65; continue; } 0x402 => { bb = 66; continue; } 0x40d => { bb = 67; continue; } 0x41a => { bb = 68; continue; } 0x425 => { bb = 69; continue; } 0x432 => { bb = 70; continue; } 0x43d => { bb = 71; continue; } 0x44b => { bb = 72; continue; } 0x456 => { bb = 73; continue; } 0x464 => { bb = 74; continue; } 0x46f => { bb = 75; continue; } 0x47d => { bb = 76; continue; } 0x488 => { bb = 77; continue; } 0x495 => { bb = 78; continue; } 0x4a0 => { bb = 79; continue; } 0x4ad => { bb = 80; continue; } 0x4b8 => { bb = 81; continue; } 0x4c5 => { bb = 82; continue; } 0x4d0 => { bb = 83; continue; } 0x4dd => { bb = 84; continue; } _ => { return crate::simd_ops::tail_fwd_xmm(j_tgt, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8, [xmm0, xmm1, xmm2, xmm3]); } } }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411c68b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xc; const P2: u64 = 0x40;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    r8 = (crate::simd_ops::ru64(rcx)) as u64;
    rcx = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(r8.wrapping_add(P0), rcx);
    crate::simd_ops::wu64(r8, rcx);
    rax = (crate::simd_ops::ru64(rdx)) as u64;
    crate::simd_ops::wu64(r8, rax);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P0), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P1), ((rax as u32)) as u32);
    crate::simd_ops::wu64(rdx, rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P0), rcx);
    crate::simd_ops::wu8(r8.wrapping_add(P2), (0x10u8) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411c68e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xc; const P2: u64 = 0x40;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    r8 = (crate::simd_ops::ru64(rcx)) as u64;
    rcx = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(r8.wrapping_add(P0), rcx);
    crate::simd_ops::wu64(r8, rcx);
    rax = (crate::simd_ops::ru64(rdx)) as u64;
    crate::simd_ops::wu64(r8, rax);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P0), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P1), ((rax as u32)) as u32);
    crate::simd_ops::wu64(rdx, rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P0), rcx);
    crate::simd_ops::wu8(r8.wrapping_add(P2), (0x12u8) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411c6910(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xc; const P2: u64 = 0x40;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    r8 = (crate::simd_ops::ru64(rcx)) as u64;
    rcx = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(r8.wrapping_add(P0), rcx);
    crate::simd_ops::wu64(r8, rcx);
    rax = (crate::simd_ops::ru64(rdx)) as u64;
    crate::simd_ops::wu64(r8, rax);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P0), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P1), ((rax as u32)) as u32);
    crate::simd_ops::wu64(rdx, rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P0), rcx);
    crate::simd_ops::wu8(r8.wrapping_add(P2), (0x14u8) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411c6940(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xc; const P2: u64 = 0x14; const P3: u64 = 0x18; const P4: u64 = 0x20; const P5: u64 = 0x40;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    xmm0 = ((crate::simd_ops::ru64(rdx)) as u64) as u128;
    r8 = (crate::simd_ops::ru64(rcx)) as u64;
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
    crate::simd_ops::wu8(r8.wrapping_add(P5), (0x15u8) as u8);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411c6980(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xc; const P2: u64 = 0x40;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    r8 = (crate::simd_ops::ru64(rcx)) as u64;
    rcx = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(r8.wrapping_add(P0), rcx);
    crate::simd_ops::wu64(r8, rcx);
    rax = (crate::simd_ops::ru64(rdx)) as u64;
    crate::simd_ops::wu64(r8, rax);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P0), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P1), ((rax as u32)) as u32);
    crate::simd_ops::wu64(rdx, rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P0), rcx);
    crate::simd_ops::wu8(r8.wrapping_add(P2), (0x16u8) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411c69b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xc; const P2: u64 = 0x14; const P3: u64 = 0x18; const P4: u64 = 0x20; const P5: u64 = 0x24; const P6: u64 = 0x2c; const P7: u64 = 0x40;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    xmm0 = ((crate::simd_ops::ru64(rdx)) as u64) as u128;
    r8 = (crate::simd_ops::ru64(rcx)) as u64;
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
    crate::simd_ops::wu8(r8.wrapping_add(P7), (0x17u8) as u8);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411c6a00(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xc; const P2: u64 = 0x40;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    r8 = (crate::simd_ops::ru64(rcx)) as u64;
    rcx = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(r8.wrapping_add(P0), rcx);
    crate::simd_ops::wu64(r8, rcx);
    rax = (crate::simd_ops::ru64(rdx)) as u64;
    crate::simd_ops::wu64(r8, rax);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P0), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P1), ((rax as u32)) as u32);
    crate::simd_ops::wu64(rdx, rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P0), rcx);
    crate::simd_ops::wu8(r8.wrapping_add(P2), (0x18u8) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411c6a30(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x20; const P2: u64 = 0x30; const P3: u64 = 0x40;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    xmm0 = (crate::simd_ops::ru128(rdx)) as u128;
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    crate::simd_ops::wu128(rax, (xmm0) as u128);
    xmm0 = (crate::simd_ops::ru128(rdx.wrapping_add(P0))) as u128;
    crate::simd_ops::wu128(rax.wrapping_add(P0), (xmm0) as u128);
    xmm0 = (crate::simd_ops::ru128(rdx.wrapping_add(P1))) as u128;
    crate::simd_ops::wu128(rax.wrapping_add(P1), (xmm0) as u128);
    xmm0 = (crate::simd_ops::ru128(rdx.wrapping_add(P2))) as u128;
    crate::simd_ops::wu128(rax.wrapping_add(P2), (xmm0) as u128);
    crate::simd_ops::wu8(rax.wrapping_add(P3), (0x19u8) as u8);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411c6a60(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xc; const P2: u64 = 0x40;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    r8 = (crate::simd_ops::ru64(rcx)) as u64;
    rcx = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(r8.wrapping_add(P0), rcx);
    crate::simd_ops::wu64(r8, rcx);
    rax = (crate::simd_ops::ru64(rdx)) as u64;
    crate::simd_ops::wu64(r8, rax);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P0), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P1), ((rax as u32)) as u32);
    crate::simd_ops::wu64(rdx, rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P0), rcx);
    crate::simd_ops::wu8(r8.wrapping_add(P2), (0x1au8) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411c6a90(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xc; const P2: u64 = 0x40;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    r8 = (crate::simd_ops::ru64(rcx)) as u64;
    rcx = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(r8.wrapping_add(P0), rcx);
    crate::simd_ops::wu64(r8, rcx);
    rax = (crate::simd_ops::ru64(rdx)) as u64;
    crate::simd_ops::wu64(r8, rax);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P0), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P1), ((rax as u32)) as u32);
    crate::simd_ops::wu64(rdx, rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P0), rcx);
    crate::simd_ops::wu8(r8.wrapping_add(P2), (0x1cu8) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411c6ac0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xc; const P2: u64 = 0x40;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    r8 = (crate::simd_ops::ru64(rcx)) as u64;
    rcx = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(r8.wrapping_add(P0), rcx);
    crate::simd_ops::wu64(r8, rcx);
    rax = (crate::simd_ops::ru64(rdx)) as u64;
    crate::simd_ops::wu64(r8, rax);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P0), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P1), ((rax as u32)) as u32);
    crate::simd_ops::wu64(rdx, rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P0), rcx);
    crate::simd_ops::wu8(r8.wrapping_add(P2), (0x1eu8) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411c6af0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xc; const P2: u64 = 0x40;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    r8 = (crate::simd_ops::ru64(rcx)) as u64;
    rcx = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(r8.wrapping_add(P0), rcx);
    crate::simd_ops::wu64(r8, rcx);
    rax = (crate::simd_ops::ru64(rdx)) as u64;
    crate::simd_ops::wu64(r8, rax);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P0), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P1), ((rax as u32)) as u32);
    crate::simd_ops::wu64(rdx, rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P0), rcx);
    crate::simd_ops::wu8(r8.wrapping_add(P2), (0x20u8) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411c6b20(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xc; const P2: u64 = 0x40;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    r8 = (crate::simd_ops::ru64(rcx)) as u64;
    rcx = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(r8.wrapping_add(P0), rcx);
    crate::simd_ops::wu64(r8, rcx);
    rax = (crate::simd_ops::ru64(rdx)) as u64;
    crate::simd_ops::wu64(r8, rax);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P0), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P1), ((rax as u32)) as u32);
    crate::simd_ops::wu64(rdx, rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P0), rcx);
    crate::simd_ops::wu8(r8.wrapping_add(P2), (0x22u8) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411c6b50(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x40;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = (crate::simd_ops::ru64(rdx)) as u64;
    r8 = (crate::simd_ops::ru64(rcx)) as u64;
    crate::simd_ops::wu64(r8, rax);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P0), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P1))) as u64;
    crate::simd_ops::wu64(rdx.wrapping_add(P1), (0x0u64) as u64);
    crate::simd_ops::wu64(r8.wrapping_add(P1), rax);
    crate::simd_ops::wu8(r8.wrapping_add(P2), (0x23u8) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411c6b80(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xc; const P2: u64 = 0x40;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    r8 = (crate::simd_ops::ru64(rcx)) as u64;
    rcx = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(r8.wrapping_add(P0), rcx);
    crate::simd_ops::wu64(r8, rcx);
    rax = (crate::simd_ops::ru64(rdx)) as u64;
    crate::simd_ops::wu64(r8, rax);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P0), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P1), ((rax as u32)) as u32);
    crate::simd_ops::wu64(rdx, rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P0), rcx);
    crate::simd_ops::wu8(r8.wrapping_add(P2), (0x24u8) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411c6bb0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x40;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = (crate::simd_ops::ru64(rdx)) as u64;
    r8 = (crate::simd_ops::ru64(rcx)) as u64;
    crate::simd_ops::wu64(rdx, (0x0u64) as u64);
    crate::simd_ops::wu64(r8, rax);
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    crate::simd_ops::wu64(r8.wrapping_add(P0), rax);
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P1))) as u64;
    crate::simd_ops::wu64(r8.wrapping_add(P1), rax);
    crate::simd_ops::wu8(r8.wrapping_add(P2), (0x25u8) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411c6be0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xc; const P2: u64 = 0x40;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    r8 = (crate::simd_ops::ru64(rcx)) as u64;
    rcx = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(r8.wrapping_add(P0), rcx);
    crate::simd_ops::wu64(r8, rcx);
    rax = (crate::simd_ops::ru64(rdx)) as u64;
    crate::simd_ops::wu64(r8, rax);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P0), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P1), ((rax as u32)) as u32);
    crate::simd_ops::wu64(rdx, rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P0), rcx);
    crate::simd_ops::wu8(r8.wrapping_add(P2), (0x26u8) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411c6c10(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xc; const P2: u64 = 0x40;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    r8 = (crate::simd_ops::ru64(rcx)) as u64;
    rcx = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(r8.wrapping_add(P0), rcx);
    crate::simd_ops::wu64(r8, rcx);
    rax = (crate::simd_ops::ru64(rdx)) as u64;
    crate::simd_ops::wu64(r8, rax);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P0), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P1), ((rax as u32)) as u32);
    crate::simd_ops::wu64(rdx, rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P0), rcx);
    crate::simd_ops::wu8(r8.wrapping_add(P2), (0x28u8) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411c6c40(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xc; const P2: u64 = 0x40;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    r8 = (crate::simd_ops::ru64(rcx)) as u64;
    rcx = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(r8.wrapping_add(P0), rcx);
    crate::simd_ops::wu64(r8, rcx);
    rax = (crate::simd_ops::ru64(rdx)) as u64;
    crate::simd_ops::wu64(r8, rax);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P0), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P1), ((rax as u32)) as u32);
    crate::simd_ops::wu64(rdx, rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P0), rcx);
    crate::simd_ops::wu8(r8.wrapping_add(P2), (0x2au8) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411c6c70(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xc; const P2: u64 = 0x40;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    r8 = (crate::simd_ops::ru64(rcx)) as u64;
    rcx = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(r8.wrapping_add(P0), rcx);
    crate::simd_ops::wu64(r8, rcx);
    rax = (crate::simd_ops::ru64(rdx)) as u64;
    crate::simd_ops::wu64(r8, rax);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P0), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P1), ((rax as u32)) as u32);
    crate::simd_ops::wu64(rdx, rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P0), rcx);
    crate::simd_ops::wu8(r8.wrapping_add(P2), (0x2cu8) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411c6ca0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xc; const P2: u64 = 0x40;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    r8 = (crate::simd_ops::ru64(rcx)) as u64;
    rcx = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(r8.wrapping_add(P0), rcx);
    crate::simd_ops::wu64(r8, rcx);
    rax = (crate::simd_ops::ru64(rdx)) as u64;
    crate::simd_ops::wu64(r8, rax);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P0), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P1), ((rax as u32)) as u32);
    crate::simd_ops::wu64(rdx, rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P0), rcx);
    crate::simd_ops::wu8(r8.wrapping_add(P2), (0x2eu8) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411c6cd0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xc; const P2: u64 = 0x40;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    r8 = (crate::simd_ops::ru64(rcx)) as u64;
    rcx = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(r8.wrapping_add(P0), rcx);
    crate::simd_ops::wu64(r8, rcx);
    rax = (crate::simd_ops::ru64(rdx)) as u64;
    crate::simd_ops::wu64(r8, rax);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P0), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P1), ((rax as u32)) as u32);
    crate::simd_ops::wu64(rdx, rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P0), rcx);
    crate::simd_ops::wu8(r8.wrapping_add(P2), (0x30u8) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411c6d00(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xc; const P2: u64 = 0x40;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    r8 = (crate::simd_ops::ru64(rcx)) as u64;
    rcx = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(r8.wrapping_add(P0), rcx);
    crate::simd_ops::wu64(r8, rcx);
    rax = (crate::simd_ops::ru64(rdx)) as u64;
    crate::simd_ops::wu64(r8, rax);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P0), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P1), ((rax as u32)) as u32);
    crate::simd_ops::wu64(rdx, rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P0), rcx);
    crate::simd_ops::wu8(r8.wrapping_add(P2), (0x32u8) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411c6d30(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x14; const P3: u64 = 0x1401e84a0; const P4: u64 = 0x40;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = (crate::simd_ops::ru64(rcx)) as u64;
    rax = 0;
    fl_l = 0; fl_r = 0;
    rcx = rbx;
    crate::simd_ops::wu64(rbx, (0x0u64) as u64);
    crate::simd_ops::wu64(rbx.wrapping_add(P0), rax);
    crate::simd_ops::wu32(rbx.wrapping_add(P1), ((rax as u32)) as u32);
    crate::simd_ops::wu8(rbx.wrapping_add(P2), ((rax as u8)) as u8);
    rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp);
    crate::simd_ops::wu8(rbx.wrapping_add(P4), (0x33u8) as u8);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411c6d60(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xc; const P2: u64 = 0x40;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    r8 = (crate::simd_ops::ru64(rcx)) as u64;
    rcx = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(r8.wrapping_add(P0), rcx);
    crate::simd_ops::wu64(r8, rcx);
    rax = (crate::simd_ops::ru64(rdx)) as u64;
    crate::simd_ops::wu64(r8, rax);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P0), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P1), ((rax as u32)) as u32);
    crate::simd_ops::wu64(rdx, rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P0), rcx);
    crate::simd_ops::wu8(r8.wrapping_add(P2), (0x34u8) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411c6d90(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x40;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    r8 = (crate::simd_ops::ru64(rcx)) as u64;
    rax = (crate::simd_ops::ru64(rdx)) as u64;
    crate::simd_ops::wu64(rdx, (0x0u64) as u64);
    crate::simd_ops::wu64(r8, rax);
    crate::simd_ops::wu8(r8.wrapping_add(P0), (0x35u8) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411c6db0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xc; const P2: u64 = 0x40;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    r8 = (crate::simd_ops::ru64(rcx)) as u64;
    rcx = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(r8.wrapping_add(P0), rcx);
    crate::simd_ops::wu64(r8, rcx);
    rax = (crate::simd_ops::ru64(rdx)) as u64;
    crate::simd_ops::wu64(r8, rax);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P0), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P1), ((rax as u32)) as u32);
    crate::simd_ops::wu64(rdx, rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P0), rcx);
    crate::simd_ops::wu8(r8.wrapping_add(P2), (0x36u8) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411c6de0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x40;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    r8 = (crate::simd_ops::ru64(rcx)) as u64;
    rax = (crate::simd_ops::ru64(rdx)) as u64;
    crate::simd_ops::wu64(rdx, (0x0u64) as u64);
    crate::simd_ops::wu64(r8, rax);
    crate::simd_ops::wu8(r8.wrapping_add(P0), (0x37u8) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411c6e00(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xc; const P2: u64 = 0x40;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    r8 = (crate::simd_ops::ru64(rcx)) as u64;
    rcx = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(r8.wrapping_add(P0), rcx);
    crate::simd_ops::wu64(r8, rcx);
    rax = (crate::simd_ops::ru64(rdx)) as u64;
    crate::simd_ops::wu64(r8, rax);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P0), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P1), ((rax as u32)) as u32);
    crate::simd_ops::wu64(rdx, rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P0), rcx);
    crate::simd_ops::wu8(r8.wrapping_add(P2), (0x38u8) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411c6e30(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x40;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    r8 = (crate::simd_ops::ru64(rcx)) as u64;
    rax = (crate::simd_ops::ru64(rdx)) as u64;
    crate::simd_ops::wu64(rdx, (0x0u64) as u64);
    crate::simd_ops::wu64(r8, rax);
    crate::simd_ops::wu8(r8.wrapping_add(P0), (0x39u8) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411c6e50(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xc; const P2: u64 = 0x40;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    r8 = (crate::simd_ops::ru64(rcx)) as u64;
    rcx = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(r8.wrapping_add(P0), rcx);
    crate::simd_ops::wu64(r8, rcx);
    rax = (crate::simd_ops::ru64(rdx)) as u64;
    crate::simd_ops::wu64(r8, rax);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P0), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P1), ((rax as u32)) as u32);
    crate::simd_ops::wu64(rdx, rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P0), rcx);
    crate::simd_ops::wu8(r8.wrapping_add(P2), (0x3au8) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411c6e80(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x40;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    r8 = (crate::simd_ops::ru64(rcx)) as u64;
    rax = (crate::simd_ops::ru64(rdx)) as u64;
    crate::simd_ops::wu64(rdx, (0x0u64) as u64);
    crate::simd_ops::wu64(r8, rax);
    crate::simd_ops::wu8(r8.wrapping_add(P0), (0x3bu8) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411c6ea0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xc; const P2: u64 = 0x40;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    r8 = (crate::simd_ops::ru64(rcx)) as u64;
    rcx = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(r8.wrapping_add(P0), rcx);
    crate::simd_ops::wu64(r8, rcx);
    rax = (crate::simd_ops::ru64(rdx)) as u64;
    crate::simd_ops::wu64(r8, rax);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P0), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P1), ((rax as u32)) as u32);
    crate::simd_ops::wu64(rdx, rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P0), rcx);
    crate::simd_ops::wu8(r8.wrapping_add(P2), (0x3cu8) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411c6ed0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xc; const P2: u64 = 0x40;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    r8 = (crate::simd_ops::ru64(rcx)) as u64;
    rcx = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(r8.wrapping_add(P0), rcx);
    crate::simd_ops::wu64(r8, rcx);
    rax = (crate::simd_ops::ru64(rdx)) as u64;
    crate::simd_ops::wu64(r8, rax);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P0), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P1), ((rax as u32)) as u32);
    crate::simd_ops::wu64(rdx, rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P0), rcx);
    crate::simd_ops::wu8(r8.wrapping_add(P2), (0x3eu8) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411c6f00(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xc; const P2: u64 = 0x40;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    r8 = (crate::simd_ops::ru64(rcx)) as u64;
    rcx = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(r8.wrapping_add(P0), rcx);
    crate::simd_ops::wu64(r8, rcx);
    rax = (crate::simd_ops::ru64(rdx)) as u64;
    crate::simd_ops::wu64(r8, rax);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P0), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P1), ((rax as u32)) as u32);
    crate::simd_ops::wu64(rdx, rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P0), rcx);
    crate::simd_ops::wu8(r8.wrapping_add(P2), (0x40u8) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411c6f30(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xc; const P2: u64 = 0x40;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    r8 = (crate::simd_ops::ru64(rcx)) as u64;
    rcx = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(r8.wrapping_add(P0), rcx);
    crate::simd_ops::wu64(r8, rcx);
    rax = (crate::simd_ops::ru64(rdx)) as u64;
    crate::simd_ops::wu64(r8, rax);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P0), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P1), ((rax as u32)) as u32);
    crate::simd_ops::wu64(rdx, rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P0), rcx);
    crate::simd_ops::wu8(r8.wrapping_add(P2), (0x42u8) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411c6f60(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xc; const P2: u64 = 0x40;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    r8 = (crate::simd_ops::ru64(rcx)) as u64;
    rcx = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(r8.wrapping_add(P0), rcx);
    crate::simd_ops::wu64(r8, rcx);
    rax = (crate::simd_ops::ru64(rdx)) as u64;
    crate::simd_ops::wu64(r8, rax);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P0), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P1), ((rax as u32)) as u32);
    crate::simd_ops::wu64(rdx, rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P0), rcx);
    crate::simd_ops::wu8(r8.wrapping_add(P2), (0x44u8) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411c6f90(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xc; const P2: u64 = 0x40;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    r8 = (crate::simd_ops::ru64(rcx)) as u64;
    rcx = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(r8.wrapping_add(P0), rcx);
    crate::simd_ops::wu64(r8, rcx);
    rax = (crate::simd_ops::ru64(rdx)) as u64;
    crate::simd_ops::wu64(r8, rax);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P0), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P1), ((rax as u32)) as u32);
    crate::simd_ops::wu64(rdx, rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P0), rcx);
    crate::simd_ops::wu8(r8.wrapping_add(P2), (0x46u8) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411c6fc0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xc; const P2: u64 = 0x40;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    r8 = (crate::simd_ops::ru64(rcx)) as u64;
    rcx = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(r8.wrapping_add(P0), rcx);
    crate::simd_ops::wu64(r8, rcx);
    rax = (crate::simd_ops::ru64(rdx)) as u64;
    crate::simd_ops::wu64(r8, rax);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P0), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P1), ((rax as u32)) as u32);
    crate::simd_ops::wu64(rdx, rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P0), rcx);
    crate::simd_ops::wu8(r8.wrapping_add(P2), (0x48u8) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411c6ff0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xc; const P2: u64 = 0x40;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    r8 = (crate::simd_ops::ru64(rcx)) as u64;
    rcx = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(r8.wrapping_add(P0), rcx);
    crate::simd_ops::wu64(r8, rcx);
    rax = (crate::simd_ops::ru64(rdx)) as u64;
    crate::simd_ops::wu64(r8, rax);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P0), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P1), ((rax as u32)) as u32);
    crate::simd_ops::wu64(rdx, rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P0), rcx);
    crate::simd_ops::wu8(r8.wrapping_add(P2), (0x4au8) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411c7020(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xc; const P2: u64 = 0x40;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    r8 = (crate::simd_ops::ru64(rcx)) as u64;
    rcx = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(r8.wrapping_add(P0), rcx);
    crate::simd_ops::wu64(r8, rcx);
    rax = (crate::simd_ops::ru64(rdx)) as u64;
    crate::simd_ops::wu64(r8, rax);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P0), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P1), ((rax as u32)) as u32);
    crate::simd_ops::wu64(rdx, rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P0), rcx);
    crate::simd_ops::wu8(r8.wrapping_add(P2), (0x4cu8) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411c7050(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xc; const P2: u64 = 0x40;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    r8 = (crate::simd_ops::ru64(rcx)) as u64;
    rcx = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(r8.wrapping_add(P0), rcx);
    crate::simd_ops::wu64(r8, rcx);
    rax = (crate::simd_ops::ru64(rdx)) as u64;
    crate::simd_ops::wu64(r8, rax);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P0), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P1), ((rax as u32)) as u32);
    crate::simd_ops::wu64(rdx, rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P0), rcx);
    crate::simd_ops::wu8(r8.wrapping_add(P2), (0x4eu8) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411c7080(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xc; const P2: u64 = 0x40;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    r8 = (crate::simd_ops::ru64(rcx)) as u64;
    rcx = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(r8.wrapping_add(P0), rcx);
    crate::simd_ops::wu64(r8, rcx);
    rax = (crate::simd_ops::ru64(rdx)) as u64;
    crate::simd_ops::wu64(r8, rax);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P0), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P1), ((rax as u32)) as u32);
    crate::simd_ops::wu64(rdx, rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P0), rcx);
    crate::simd_ops::wu8(r8.wrapping_add(P2), (0x50u8) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411c70b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xc; const P2: u64 = 0x40;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    r8 = (crate::simd_ops::ru64(rcx)) as u64;
    rcx = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(r8.wrapping_add(P0), rcx);
    crate::simd_ops::wu64(r8, rcx);
    rax = (crate::simd_ops::ru64(rdx)) as u64;
    crate::simd_ops::wu64(r8, rax);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P0), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P1), ((rax as u32)) as u32);
    crate::simd_ops::wu64(rdx, rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P0), rcx);
    crate::simd_ops::wu8(r8.wrapping_add(P2), (0x52u8) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411c70e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x24; const P1: u64 = 0x28; const P2: u64 = 0x14241d6f0; const P3: u64 = 0x8; const P4: u64 = 0x10;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdi = rdx;
    rbx = rcx;
    rdx = (crate::simd_ops::ru64(rcx)) as u64;
    fl_l = rdx; fl_r = rdx;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rbx, rcx); rax = (crate::simd_ops::ru64(rdi)) as u64; crate::simd_ops::wu64(rdi, rcx); crate::simd_ops::wu64(rbx, rax); rax = (crate::simd_ops::ru64(rdi.wrapping_add(P3))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(P3), rax); rax = (crate::simd_ops::ru64(rdi.wrapping_add(P4))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(P4), rax); rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    rax = 0xffffffff;
    let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rdx.wrapping_add(P0), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64;
    rax = (xa_old) as u32 as u64;
    fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1;
    if (((fl_l as u32)) != ((fl_r as u32))) { rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rbx, rcx); rax = (crate::simd_ops::ru64(rdi)) as u64; crate::simd_ops::wu64(rdi, rcx); crate::simd_ops::wu64(rbx, rax); rax = (crate::simd_ops::ru64(rdi.wrapping_add(P3))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(P3), rax); rax = (crate::simd_ops::ru64(rdi.wrapping_add(P4))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(P4), rax); rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P1))) as u64;
    fl_l = rcx; fl_r = rcx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); }
    rcx = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rbx, rcx);
    rax = (crate::simd_ops::ru64(rdi)) as u64;
    crate::simd_ops::wu64(rdi, rcx);
    crate::simd_ops::wu64(rbx, rax);
    rax = (crate::simd_ops::ru64(rdi.wrapping_add(P3))) as u64;
    crate::simd_ops::wu64(rbx.wrapping_add(P3), rax);
    rax = (crate::simd_ops::ru64(rdi.wrapping_add(P4))) as u64;
    crate::simd_ops::wu64(rbx.wrapping_add(P4), rax);
    rax = rbx;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411c7150(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x140215e80; const P1: u64 = 0x14183acc8; const P2: u64 = 0x1401de830;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = rcx; fl_l = rcx; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 12; continue; } bb = 1; continue; }
            1 => { rbp = (crate::simd_ops::ru32(rdx.wrapping_add(0x8u64))) as u32 as u64; r12 = ((rbp as u32)) as u32 as u64; r12 = ((r12).wrapping_shl(4u32)) as u64; r15 = (crate::simd_ops::ru64(rdx)) as u64; rdi = (crate::simd_ops::ru64(rcx)) as u64; r14 = (crate::simd_ops::ru32(rcx.wrapping_add(0x8u64))) as u32 as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0xcu64))) as u32 as u64; fl_l = rbp; fl_r = rax; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 3; continue; } bb = 2; continue; }
            2 => { r14 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rcx.wrapping_add(0x8u64), ((r14 as u32)) as u32); rdi = ((rax as u32)) as u32 as u64; rdi = ((rdi).wrapping_shr(1u32)) as u64; rdi = ((rdi).wrapping_add(rax)) as u64; fl_l = rdi; fl_r = rbp; rdi = (if (((fl_l as u64)) < ((fl_r as u64))) { rbp } else { rdi }) as u64; rdx = rdi; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rbx = rax; r8 = (crate::simd_ops::ru32(rsi.wrapping_add(0x8u64))) as u32 as u64; r8 = ((r8).wrapping_shl(4u32)) as u64; rdx = (crate::simd_ops::ru64(rsi)) as u64; rcx = rax; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = rsi; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rsi, rbx); crate::simd_ops::wu32(rsi.wrapping_add(0x8u64), ((r14 as u32)) as u32); crate::simd_ops::wu32(rsi.wrapping_add(0xcu64), ((rdi as u32)) as u32); rcx = rbx; bb = 9; continue; }
            3 => { fl_l = rbp; fl_r = r14; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 8; continue; } bb = 4; continue; }
            4 => { rbx = r14; rbx = ((rbx).wrapping_shl(4u32)) as u64; r12 = ((rbx.wrapping_add(r15)) as u64) as u64; fl_l = r14; fl_r = r14; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { r8 = rbx; rdx = r15; rcx = rdi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rdi).wrapping_add(rbx)) as u64; fl_r = fl_l; rdi = ((fl_l as u64)) as u64; bb = 6; continue; }
            6 => { r8 = rbp; fl_l = r8; fl_r = r14; r8 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 11; continue; } bb = 7; continue; }
            7 => { fl_l = ((r8).wrapping_shl(4u32)) as u64; r8 = ((fl_l as u64)) as u64; rdx = r12; rcx = rdi; bb = 10; continue; }
            8 => { rcx = rdi; bb = 9; continue; }
            9 => { r8 = r12; rdx = r15; bb = 10; continue; }
            10 => { rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 11; continue; }
            11 => { crate::simd_ops::wu32(rsi.wrapping_add(0x8u64), ((rbp as u32)) as u32); bb = 12; continue; }
            12 => { rax = rsi; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411c7250(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1401decf0; const P1: u64 = 0x14183acc8; const P2: u64 = 0x1401de830;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = rcx; fl_l = rcx; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 6; continue; } bb = 1; continue; }
            1 => { rbp = (crate::simd_ops::ru32(rdx.wrapping_add(0x8u64))) as u32 as u64; r15 = ((rbp.wrapping_mul(4)) as u64) as u64; r14 = (crate::simd_ops::ru64(rdx)) as u64; rbx = (crate::simd_ops::ru64(rcx)) as u64; rdi = (crate::simd_ops::ru32(rcx.wrapping_add(0x8u64))) as u32 as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0xcu64))) as u32 as u64; fl_l = rbp; fl_r = rax; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 7; continue; } bb = 2; continue; }
            2 => { r12 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rcx.wrapping_add(0x8u64), ((r12 as u32)) as u32); rdi = ((rax as u32)) as u32 as u64; rdi = ((rdi).wrapping_shr(1u32)) as u64; rdi = ((rdi).wrapping_add(rax)) as u64; fl_l = rdi; fl_r = rbp; rdi = (if (((fl_l as u64)) < ((fl_r as u64))) { rbp } else { rdi }) as u64; rdx = rdi; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rbx = rax; r8 = (crate::simd_ops::ru32(rsi.wrapping_add(0x8u64))) as u32 as u64; r8 = ((r8).wrapping_shl(2u32)) as u64; rdx = (crate::simd_ops::ru64(rsi)) as u64; rcx = rax; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = rsi; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rsi, rbx); crate::simd_ops::wu32(rsi.wrapping_add(0x8u64), ((r12 as u32)) as u32); crate::simd_ops::wu32(rsi.wrapping_add(0xcu64), ((rdi as u32)) as u32); bb = 3; continue; }
            3 => { r8 = r15; rdx = r14; bb = 4; continue; }
            4 => { rcx = rbx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            5 => { crate::simd_ops::wu32(rsi.wrapping_add(0x8u64), ((rbp as u32)) as u32); bb = 6; continue; }
            6 => { rax = rsi; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            7 => { fl_l = rbp; fl_r = rdi; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 3; continue; } bb = 8; continue; }
            8 => { r15 = ((rdi.wrapping_mul(4)) as u64) as u64; r12 = ((r15.wrapping_add(r14)) as u64) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { r8 = r15; rdx = r14; rcx = rbx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rbx).wrapping_add(r15)) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; bb = 10; continue; }
            10 => { r8 = rbp; fl_l = r8; fl_r = rdi; r8 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 5; continue; } bb = 11; continue; }
            11 => { r8 = ((r8.wrapping_mul(4)) as u64) as u64; rdx = r12; bb = 4; continue; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411c7350(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1401dec00; const P1: u64 = 0x14183acc8; const P2: u64 = 0x1401de830;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = rcx; fl_l = rcx; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 6; continue; } bb = 1; continue; }
            1 => { rbp = (crate::simd_ops::ru32(rdx.wrapping_add(0x8u64))) as u32 as u64; r15 = ((rbp.wrapping_mul(8)) as u64) as u64; r14 = (crate::simd_ops::ru64(rdx)) as u64; rbx = (crate::simd_ops::ru64(rcx)) as u64; rdi = (crate::simd_ops::ru32(rcx.wrapping_add(0x8u64))) as u32 as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0xcu64))) as u32 as u64; fl_l = rbp; fl_r = rax; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 7; continue; } bb = 2; continue; }
            2 => { r12 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rcx.wrapping_add(0x8u64), ((r12 as u32)) as u32); rdi = ((rax as u32)) as u32 as u64; rdi = ((rdi).wrapping_shr(1u32)) as u64; rdi = ((rdi).wrapping_add(rax)) as u64; fl_l = rdi; fl_r = rbp; rdi = (if (((fl_l as u64)) < ((fl_r as u64))) { rbp } else { rdi }) as u64; rdx = rdi; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rbx = rax; r8 = (crate::simd_ops::ru32(rsi.wrapping_add(0x8u64))) as u32 as u64; r8 = ((r8).wrapping_shl(3u32)) as u64; rdx = (crate::simd_ops::ru64(rsi)) as u64; rcx = rax; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = rsi; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rsi, rbx); crate::simd_ops::wu32(rsi.wrapping_add(0x8u64), ((r12 as u32)) as u32); crate::simd_ops::wu32(rsi.wrapping_add(0xcu64), ((rdi as u32)) as u32); bb = 3; continue; }
            3 => { r8 = r15; rdx = r14; bb = 4; continue; }
            4 => { rcx = rbx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            5 => { crate::simd_ops::wu32(rsi.wrapping_add(0x8u64), ((rbp as u32)) as u32); bb = 6; continue; }
            6 => { rax = rsi; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            7 => { fl_l = rbp; fl_r = rdi; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 3; continue; } bb = 8; continue; }
            8 => { r15 = ((rdi.wrapping_mul(8)) as u64) as u64; r12 = ((r15.wrapping_add(r14)) as u64) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { r8 = r15; rdx = r14; rcx = rbx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rbx).wrapping_add(r15)) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; bb = 10; continue; }
            10 => { r8 = rbp; fl_l = r8; fl_r = rdi; r8 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 5; continue; } bb = 11; continue; }
            11 => { r8 = ((r8.wrapping_mul(8)) as u64) as u64; rdx = r12; bb = 4; continue; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411c7450(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1410c3f20; const P1: u64 = 0x1425371a0; const P2: u64 = 0x142533200; const P3: u64 = 0x14241d6f0;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rbx = rdx; rdi = (crate::simd_ops::ru64(rcx)) as u64; rbp = (crate::simd_ops::ru32(rcx.wrapping_add(0x8u64))) as u32 as u64; r15 = (crate::simd_ops::ru32(rcx.wrapping_add(0xcu64))) as u32 as u64; r12 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rcx, r12); crate::simd_ops::wu64(rcx.wrapping_add(0x8u64), r12); rsi = rdx.wrapping_add(0x8); r14 = rdx.wrapping_add(0xc); fl_l = rcx; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rdx)) as u64; crate::simd_ops::wu64(rcx, rax); rax = (crate::simd_ops::ru32(rsi)) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x8u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(r14)) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(0xcu64), ((rax as u32)) as u32); crate::simd_ops::wu64(rdx, r12); crate::simd_ops::wu32(rsi, ((r12 as u32)) as u32); crate::simd_ops::wu32(r14, ((r12 as u32)) as u32); bb = 2; continue; }
            2 => { rax = rsp.wrapping_add(0x20); fl_l = rbx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 14; continue; } bb = 3; continue; }
            3 => { rdx = (crate::simd_ops::ru64(rdx)) as u64; rax = (crate::simd_ops::ru32(rsi)) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; r8 = ((rdx.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rdx; fl_r = r8; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            5 => { r9 = (crate::simd_ops::ru64(rbx)) as u64; fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 6; continue; }
            6 => { r10 = r9.wrapping_sub(0x1); r10 = (((r10) & (0xfffffffffe000000u64))) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(r10.wrapping_add(0x100u64))) as u64; rdx = (rdx & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); rax = r9; rax = ((rax).wrapping_sub(r10)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rcx = r10.wrapping_add(0x108); rcx = ((rcx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x4u64))) as u32 as u64; rcx = ((rcx).wrapping_sub(rax)) as u64; fl_l = ((rdx as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 12; continue; } bb = 7; continue; }
            7 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0xeu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 10; continue; } bb = 8; continue; }
            8 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(r9, rax); crate::simd_ops::wu64(rcx.wrapping_add(0x20u64), r9); fl_l = (crate::simd_ops::lk_add_u32(rcx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 11; continue; } bb = 9; continue; }
            9 => { rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rsi = rbx.wrapping_add(0x8); r14 = rbx.wrapping_add(0xc); bb = 13; continue; }
            10 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); rdx = rcx; rcx = r10; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 11; continue; }
            11 => { rsi = rbx.wrapping_add(0x8); r14 = rbx.wrapping_add(0xc); bb = 13; continue; }
            12 => { r8 = 0; fl_l = 0; fl_r = 0; rdx = rcx; rcx = r10; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 13; continue; }
            13 => { crate::simd_ops::wu64(rbx, rdi); crate::simd_ops::wu32(rsi, ((rbp as u32)) as u32); crate::simd_ops::wu32(r14, ((r15 as u32)) as u32); rdi = r12; rbp = ((r12 as u32)) as u32 as u64; bb = 14; continue; }
            14 => { rax = ((rbp as u32)) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rsi = ((rdi.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rdi; fl_r = rsi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 21; continue; } bb = 15; continue; }
            15 => { rbx = rdi.wrapping_add(0x10); bb = 16; continue; }
            16 => { rdx = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 20; continue; } bb = 17; continue; }
            17 => { rax = 0xffffffff; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rdx.wrapping_add(0x24u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 20; continue; } bb = 18; continue; }
            18 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x28u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 20; continue; } bb = 19; continue; }
            19 => { rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 20; continue; }
            20 => { crate::simd_ops::wu64(rbx, r12); rbx = ((rbx).wrapping_add(0x18u64)) as u64; rax = rbx.wrapping_sub(0x10); fl_l = rax; fl_r = rsi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 16; continue; } bb = 21; continue; }
            21 => { fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 29; continue; } bb = 22; continue; }
            22 => { r10 = rdi.wrapping_sub(0x1); r10 = (((r10) & (0xfffffffffe000000u64))) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(r10.wrapping_add(0x100u64))) as u64; rdx = (rdx & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); rax = rdi; rax = ((rax).wrapping_sub(r10)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rcx = ((rcx).wrapping_add(0x108u64)) as u64; rcx = ((rcx).wrapping_add(r10)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x4u64))) as u32 as u64; rcx = ((rcx).wrapping_sub(rax)) as u64; fl_l = ((rdx as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 27; continue; } bb = 23; continue; }
            23 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0xeu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 26; continue; } bb = 24; continue; }
            24 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(rdi, rax); crate::simd_ops::wu64(rcx.wrapping_add(0x20u64), rdi); fl_l = (crate::simd_ops::lk_add_u32(rcx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 29; continue; } bb = 25; continue; }
            25 => { rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 29; continue; }
            26 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 28; continue; }
            27 => { r8 = 0; fl_l = 0; fl_r = 0; bb = 28; continue; }
            28 => { rdx = rcx; r9 = rdi; rcx = r10; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 29; continue; }
            29 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411c7650(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1410c3f90; const P1: u64 = 0x1425371a0; const P2: u64 = 0x142533200; const P3: u64 = 0x14241d6f0;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rbx = rdx; rdi = (crate::simd_ops::ru64(rcx)) as u64; rbp = (crate::simd_ops::ru32(rcx.wrapping_add(0x8u64))) as u32 as u64; r15 = (crate::simd_ops::ru32(rcx.wrapping_add(0xcu64))) as u32 as u64; r12 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rcx, r12); crate::simd_ops::wu64(rcx.wrapping_add(0x8u64), r12); rsi = rdx.wrapping_add(0x8); r14 = rdx.wrapping_add(0xc); fl_l = rcx; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rdx)) as u64; crate::simd_ops::wu64(rcx, rax); rax = (crate::simd_ops::ru32(rsi)) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x8u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(r14)) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(0xcu64), ((rax as u32)) as u32); crate::simd_ops::wu64(rdx, r12); crate::simd_ops::wu32(rsi, ((r12 as u32)) as u32); crate::simd_ops::wu32(r14, ((r12 as u32)) as u32); bb = 2; continue; }
            2 => { rax = rsp.wrapping_add(0x20); fl_l = rbx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 14; continue; } bb = 3; continue; }
            3 => { rdx = (crate::simd_ops::ru64(rdx)) as u64; rax = (crate::simd_ops::ru32(rsi)) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; r8 = ((rdx.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rdx; fl_r = r8; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            5 => { r9 = (crate::simd_ops::ru64(rbx)) as u64; fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 6; continue; }
            6 => { r10 = r9.wrapping_sub(0x1); r10 = (((r10) & (0xfffffffffe000000u64))) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(r10.wrapping_add(0x100u64))) as u64; rdx = (rdx & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); rax = r9; rax = ((rax).wrapping_sub(r10)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rcx = r10.wrapping_add(0x108); rcx = ((rcx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x4u64))) as u32 as u64; rcx = ((rcx).wrapping_sub(rax)) as u64; fl_l = ((rdx as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 12; continue; } bb = 7; continue; }
            7 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0xeu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 10; continue; } bb = 8; continue; }
            8 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(r9, rax); crate::simd_ops::wu64(rcx.wrapping_add(0x20u64), r9); fl_l = (crate::simd_ops::lk_add_u32(rcx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 11; continue; } bb = 9; continue; }
            9 => { rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rsi = rbx.wrapping_add(0x8); r14 = rbx.wrapping_add(0xc); bb = 13; continue; }
            10 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); rdx = rcx; rcx = r10; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 11; continue; }
            11 => { rsi = rbx.wrapping_add(0x8); r14 = rbx.wrapping_add(0xc); bb = 13; continue; }
            12 => { r8 = 0; fl_l = 0; fl_r = 0; rdx = rcx; rcx = r10; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 13; continue; }
            13 => { crate::simd_ops::wu64(rbx, rdi); crate::simd_ops::wu32(rsi, ((rbp as u32)) as u32); crate::simd_ops::wu32(r14, ((r15 as u32)) as u32); rdi = r12; rbp = ((r12 as u32)) as u32 as u64; bb = 14; continue; }
            14 => { rax = ((rbp as u32)) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rsi = ((rdi.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rdi; fl_r = rsi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 21; continue; } bb = 15; continue; }
            15 => { rbx = rdi; bb = 16; continue; }
            16 => { rdx = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 20; continue; } bb = 17; continue; }
            17 => { rax = 0xffffffff; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rdx.wrapping_add(0x24u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 20; continue; } bb = 18; continue; }
            18 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x28u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 20; continue; } bb = 19; continue; }
            19 => { rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 20; continue; }
            20 => { crate::simd_ops::wu64(rbx, r12); rbx = ((rbx).wrapping_add(0x18u64)) as u64; fl_l = rbx; fl_r = rsi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 16; continue; } bb = 21; continue; }
            21 => { fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 29; continue; } bb = 22; continue; }
            22 => { r10 = rdi.wrapping_sub(0x1); r10 = (((r10) & (0xfffffffffe000000u64))) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(r10.wrapping_add(0x100u64))) as u64; rdx = (rdx & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); rax = rdi; rax = ((rax).wrapping_sub(r10)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rcx = ((rcx).wrapping_add(0x108u64)) as u64; rcx = ((rcx).wrapping_add(r10)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x4u64))) as u32 as u64; rcx = ((rcx).wrapping_sub(rax)) as u64; fl_l = ((rdx as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 27; continue; } bb = 23; continue; }
            23 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0xeu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 26; continue; } bb = 24; continue; }
            24 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(rdi, rax); crate::simd_ops::wu64(rcx.wrapping_add(0x20u64), rdi); fl_l = (crate::simd_ops::lk_add_u32(rcx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 29; continue; } bb = 25; continue; }
            25 => { rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 29; continue; }
            26 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 28; continue; }
            27 => { r8 = 0; fl_l = 0; fl_r = 0; bb = 28; continue; }
            28 => { rdx = rcx; r9 = rdi; rcx = r10; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 29; continue; }
            29 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411c7850(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1425371a0; const P1: u64 = 0x142533200;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rdi = rdx; rbx = (crate::simd_ops::ru64(rcx)) as u64; rbp = (crate::simd_ops::ru32(rcx.wrapping_add(0x8u64))) as u32 as u64; r14 = (crate::simd_ops::ru32(rcx.wrapping_add(0xcu64))) as u32 as u64; r15 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rcx, r15); crate::simd_ops::wu64(rcx.wrapping_add(0x8u64), r15); rsi = rdx.wrapping_add(0x8); fl_l = rcx; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rdx)) as u64; crate::simd_ops::wu64(rcx, rax); rax = (crate::simd_ops::ru32(rsi)) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x8u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(0xcu64))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(0xcu64), ((rax as u32)) as u32); crate::simd_ops::wu64(rdx, r15); crate::simd_ops::wu32(rsi, ((r15 as u32)) as u32); crate::simd_ops::wu32(rdx.wrapping_add(0xcu64), ((r15 as u32)) as u32); bb = 2; continue; }
            2 => { rax = rsp.wrapping_add(0x20); fl_l = rdi; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 16; continue; } bb = 3; continue; }
            3 => { rdx = (crate::simd_ops::ru64(rdx)) as u64; rax = (crate::simd_ops::ru32(rsi)) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; r8 = ((rdx.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rdx; fl_r = r8; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 7; continue; } bb = 4; continue; }
            4 => { bb = 5; continue; }
            5 => { rax = r15; let tmp_xchg: u64 = crate::simd_ops::atomic_xchg_u64(rdx.wrapping_add(0x8u64), rax); rax = (tmp_xchg) as u64; rdx = ((rdx).wrapping_add(0x18u64)) as u64; fl_l = rdx; fl_r = r8; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 5; continue; } bb = 6; continue; }
            6 => { rsi = rdi.wrapping_add(0x8); bb = 7; continue; }
            7 => { r9 = (crate::simd_ops::ru64(rdi)) as u64; fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 15; continue; } bb = 8; continue; }
            8 => { r10 = r9.wrapping_sub(0x1); r10 = (((r10) & (0xfffffffffe000000u64))) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(r10.wrapping_add(0x100u64))) as u64; rdx = (rdx & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); rax = r9; rax = ((rax).wrapping_sub(r10)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rcx = r10.wrapping_add(0x108); rcx = ((rcx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x4u64))) as u32 as u64; rcx = ((rcx).wrapping_sub(rax)) as u64; fl_l = ((rdx as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 14; continue; } bb = 9; continue; }
            9 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0xeu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 12; continue; } bb = 10; continue; }
            10 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(r9, rax); crate::simd_ops::wu64(rcx.wrapping_add(0x20u64), r9); fl_l = (crate::simd_ops::lk_add_u32(rcx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 13; continue; } bb = 11; continue; }
            11 => { rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rsi = rdi.wrapping_add(0x8); bb = 15; continue; }
            12 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); rdx = rcx; rcx = r10; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 13; continue; }
            13 => { rsi = rdi.wrapping_add(0x8); bb = 15; continue; }
            14 => { r8 = 0; fl_l = 0; fl_r = 0; rdx = rcx; rcx = r10; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 15; continue; }
            15 => { crate::simd_ops::wu64(rdi, rbx); crate::simd_ops::wu32(rsi, ((rbp as u32)) as u32); crate::simd_ops::wu32(rdi.wrapping_add(0xcu64), ((r14 as u32)) as u32); rbx = r15; rbp = ((r15 as u32)) as u32 as u64; bb = 16; continue; }
            16 => { rax = ((rbp as u32)) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rdx = ((rbx.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 19; continue; } bb = 17; continue; }
            17 => { rcx = rbx; bb = 18; continue; }
            18 => { rax = r15; let tmp_xchg: u64 = crate::simd_ops::atomic_xchg_u64(rcx.wrapping_add(0x8u64), rax); rax = (tmp_xchg) as u64; rcx = ((rcx).wrapping_add(0x18u64)) as u64; fl_l = rcx; fl_r = rdx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 18; continue; } bb = 19; continue; }
            19 => { fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 27; continue; } bb = 20; continue; }
            20 => { r10 = rbx.wrapping_sub(0x1); r10 = (((r10) & (0xfffffffffe000000u64))) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(r10.wrapping_add(0x100u64))) as u64; rdx = (rdx & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); rax = rbx; rax = ((rax).wrapping_sub(r10)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rcx = ((rcx).wrapping_add(0x108u64)) as u64; rcx = ((rcx).wrapping_add(r10)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x4u64))) as u32 as u64; rcx = ((rcx).wrapping_sub(rax)) as u64; fl_l = ((rdx as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 25; continue; } bb = 21; continue; }
            21 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0xeu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 24; continue; } bb = 22; continue; }
            22 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(rbx, rax); crate::simd_ops::wu64(rcx.wrapping_add(0x20u64), rbx); fl_l = (crate::simd_ops::lk_add_u32(rcx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 27; continue; } bb = 23; continue; }
            23 => { rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); bb = 27; continue; }
            24 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 26; continue; }
            25 => { r8 = 0; fl_l = 0; fl_r = 0; bb = 26; continue; }
            26 => { rdx = rcx; r9 = rbx; rcx = r10; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 27; continue; }
            27 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411c7e50(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x40; const P1: u64 = 0x1401de830; const P2: u64 = 0x8; const P3: u64 = 0xc; const P4: u64 = 0x1410c38f0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = rdx; rbx = rcx; rcx = (((crate::simd_ops::ru8(rcx.wrapping_add(P0))) as i8 as i64) as u64) as u64; fl_l = rcx; fl_r = (0x1eu64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } bb = 1; continue; }
            1 => { fl_l = rbx; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rcx = rbx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rdi)) as u64; crate::simd_ops::wu64(rbx, rax); rax = (crate::simd_ops::ru32(rdi.wrapping_add(P2))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(P2), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdi.wrapping_add(P3))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(P3), ((rax as u32)) as u32); rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rdi, rcx); crate::simd_ops::wu64(rdi.wrapping_add(P2), rcx); bb = 3; continue; }
            3 => { rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            4 => { crate::simd_ops::wu8(rsp.wrapping_add(0x30u64), (0x0u8) as u8); rcx = ((rcx).wrapping_add(1)) as u64; r8 = rbx; rdx = rsp.wrapping_add(0x30); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu8(rbx.wrapping_add(P0), (0xffu8) as u8); rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rbx, rcx); crate::simd_ops::wu64(rbx.wrapping_add(P2), rcx); rax = (crate::simd_ops::ru64(rdi)) as u64; crate::simd_ops::wu64(rbx, rax); rax = (crate::simd_ops::ru32(rdi.wrapping_add(P2))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(P2), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdi.wrapping_add(P3))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(P3), ((rax as u32)) as u32); crate::simd_ops::wu64(rdi, rcx); crate::simd_ops::wu64(rdi.wrapping_add(P2), rcx); crate::simd_ops::wu8(rbx.wrapping_add(P0), (0x1eu8) as u8); rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411c7ef0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x40; const P1: u64 = 0x1401de830; const P2: u64 = 0x8; const P3: u64 = 0xc; const P4: u64 = 0x1410c38f0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = rdx; rbx = rcx; rcx = (((crate::simd_ops::ru8(rcx.wrapping_add(P0))) as i8 as i64) as u64) as u64; fl_l = rcx; fl_r = (0x22u64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } bb = 1; continue; }
            1 => { fl_l = rbx; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rcx = rbx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rdi)) as u64; crate::simd_ops::wu64(rbx, rax); rax = (crate::simd_ops::ru32(rdi.wrapping_add(P2))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(P2), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdi.wrapping_add(P3))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(P3), ((rax as u32)) as u32); rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rdi, rcx); crate::simd_ops::wu64(rdi.wrapping_add(P2), rcx); bb = 3; continue; }
            3 => { rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            4 => { crate::simd_ops::wu8(rsp.wrapping_add(0x30u64), (0x0u8) as u8); rcx = ((rcx).wrapping_add(1)) as u64; r8 = rbx; rdx = rsp.wrapping_add(0x30); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu8(rbx.wrapping_add(P0), (0xffu8) as u8); rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rbx, rcx); crate::simd_ops::wu64(rbx.wrapping_add(P2), rcx); rax = (crate::simd_ops::ru64(rdi)) as u64; crate::simd_ops::wu64(rbx, rax); rax = (crate::simd_ops::ru32(rdi.wrapping_add(P2))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(P2), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdi.wrapping_add(P3))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(P3), ((rax as u32)) as u32); crate::simd_ops::wu64(rdi, rcx); crate::simd_ops::wu64(rdi.wrapping_add(P2), rcx); crate::simd_ops::wu8(rbx.wrapping_add(P0), (0x22u8) as u8); rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411c7f90(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x40; const P1: u64 = 0x1410c05d0; const P2: u64 = 0x8; const P3: u64 = 0xc; const P4: u64 = 0x1410c38f0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = rdx; rbx = rcx; rcx = (((crate::simd_ops::ru8(rcx.wrapping_add(P0))) as i8 as i64) as u64) as u64; fl_l = rcx; fl_r = (0x3au64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } bb = 1; continue; }
            1 => { fl_l = rbx; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rcx = rbx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rdi)) as u64; crate::simd_ops::wu64(rbx, rax); rax = (crate::simd_ops::ru32(rdi.wrapping_add(P2))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(P2), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdi.wrapping_add(P3))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(P3), ((rax as u32)) as u32); rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rdi, rcx); crate::simd_ops::wu64(rdi.wrapping_add(P2), rcx); bb = 3; continue; }
            3 => { rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            4 => { crate::simd_ops::wu8(rsp.wrapping_add(0x30u64), (0x0u8) as u8); rcx = ((rcx).wrapping_add(1)) as u64; r8 = rbx; rdx = rsp.wrapping_add(0x30); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu8(rbx.wrapping_add(P0), (0xffu8) as u8); rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rbx, rcx); crate::simd_ops::wu64(rbx.wrapping_add(P2), rcx); rax = (crate::simd_ops::ru64(rdi)) as u64; crate::simd_ops::wu64(rbx, rax); rax = (crate::simd_ops::ru32(rdi.wrapping_add(P2))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(P2), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdi.wrapping_add(P3))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(P3), ((rax as u32)) as u32); crate::simd_ops::wu64(rdi, rcx); crate::simd_ops::wu64(rdi.wrapping_add(P2), rcx); crate::simd_ops::wu8(rbx.wrapping_add(P0), (0x3au8) as u8); rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411c80b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1411c9860;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = (crate::simd_ops::ru64(rcx)) as u64; rsi = rdx; rcx = ((crate::simd_ops::ru8(r8)) as u32) as u32 as u64; rdx = (0x100000001b3u64) as u64; rax = (0xcbf29ce484222325u64) as u64; r12 = 0; fl_l = 0; fl_r = 0; rcx = (((rcx) ^ (rax))) as u64; r15 = r9; rax = ((crate::simd_ops::ru8(r8.wrapping_add(0x1u64))) as u32) as u32 as u64; rdi = r8; r10 = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; r11 = ((r12 as u32)) as u32 as u64; rbp = (crate::simd_ops::ru64(rbx)) as u64; rcx = (((rcx) as i64).wrapping_mul((rdx) as i64) as u64) as u64; rcx = (((rcx) ^ (rax))) as u64; rax = ((crate::simd_ops::ru8(r8.wrapping_add(0x2u64))) as u32) as u32 as u64; rcx = (((rcx) as i64).wrapping_mul((rdx) as i64) as u64) as u64; rcx = (((rcx) ^ (rax))) as u64; rax = ((crate::simd_ops::ru8(r8.wrapping_add(0x3u64))) as u32) as u32 as u64; rcx = (((rcx) as i64).wrapping_mul((rdx) as i64) as u64) as u64; rcx = (((rcx) ^ (rax))) as u64; rax = ((crate::simd_ops::ru8(r8.wrapping_add(0x4u64))) as u32) as u32 as u64; rcx = (((rcx) as i64).wrapping_mul((rdx) as i64) as u64) as u64; rcx = (((rcx) ^ (rax))) as u64; rax = ((crate::simd_ops::ru8(r8.wrapping_add(0x5u64))) as u32) as u32 as u64; rcx = (((rcx) as i64).wrapping_mul((rdx) as i64) as u64) as u64; rcx = (((rcx) ^ (rax))) as u64; rax = ((crate::simd_ops::ru8(r8.wrapping_add(0x6u64))) as u32) as u32 as u64; rcx = (((rcx) as i64).wrapping_mul((rdx) as i64) as u64) as u64; rcx = (((rcx) ^ (rax))) as u64; rax = ((crate::simd_ops::ru8(r8.wrapping_add(0x7u64))) as u32) as u32 as u64; rcx = (((rcx) as i64).wrapping_mul((rdx) as i64) as u64) as u64; rcx = (((rcx) ^ (rax))) as u64; rax = (0xde5fb9d2630458e9u64) as u64; rcx = (((rcx) as i64).wrapping_mul((rdx) as i64) as u64) as u64; let prod = (rax as u128).wrapping_mul((rcx) as u128); rax = prod as u64; rdx = (prod >> 64) as u64; fl_l = (rdx != 0) as u64; r14 = ((rdx.wrapping_add(rax)) as u64) as u64; rax = (((r14 as u8)) as u32) as u32 as u64; rdx = r14; rdx = ((rdx).wrapping_shr(7u32)) as u64; rdx = (((rdx) & (r10))) as u64; fl_l = ((((rax as u8)) & (0x7fu8))) as u8 as u64; fl_r = fl_l; rax = (rax & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); rax = ((((rax as u8)) as i8 as i32) as u32) as u32 as u64; xmm2 = (((rax as u32)) as u32) as u128; xmm2 = crate::simd_ops::vpbroadcastb((xmm2 as u8)); bb = 1; continue; }
            1 => { xmm1 = (crate::simd_ops::ru128(rdx.wrapping_add(rbp))) as u128; xmm0 = crate::simd_ops::vpcmpeqb(xmm1, xmm2); rax = (crate::simd_ops::vpmovmskb(xmm0) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 5; continue; } bb = 2; continue; }
            2 => { r8 = (crate::simd_ops::ru64(rdi)) as u64; r9 = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; bb = 3; continue; }
            3 => { let bit_in: u32 = (rax as u32); if bit_in != 0 { rcx = ((bit_in.trailing_zeros() as u32)) as u32 as u64; } fl_l = bit_in as u64; rcx = ((rcx).wrapping_add(rdx)) as u64; rcx = (((rcx) & (r10))) as u64; fl_l = (crate::simd_ops::ru64(r9.wrapping_add(rcx.wrapping_mul(8)))) as u64; fl_r = r8; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 7; continue; } bb = 4; continue; }
            4 => { fl_l = (((rax as u32)) & (((rax as u32)).wrapping_sub(1))) as u32 as u64; rax = ((fl_l as u32)) as u32 as u64; if (((fl_l as u32)) != 0) { bb = 3; continue; } bb = 5; continue; }
            5 => { xmm0 = crate::simd_ops::vpsignb(xmm1, xmm1); rax = (crate::simd_ops::vpmovmskb(xmm0) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 8; continue; } bb = 6; continue; }
            6 => { r11 = ((r11).wrapping_add(0x10u64)) as u64; rdx = ((rdx).wrapping_add(r11)) as u64; fl_l = (((rdx) & (r10))) as u64; fl_r = fl_l; rdx = ((fl_l as u64)) as u64; bb = 1; continue; }
            7 => { r9 = (r9 & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 9; continue; }
            8 => { rdx = r14; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rcx = rax; rax = (crate::simd_ops::ru64(r15)) as u64; r9 = (r9 & !0xffu64) | (((0x1u8) as u8 as u64)); crate::simd_ops::wu64(rdx.wrapping_add(rcx.wrapping_mul(8)), rax); bb = 9; continue; }
            9 => { r8 = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rdx = (crate::simd_ops::ru64(rbx)) as u64; rdx = ((rdx).wrapping_add(rcx)) as u64; crate::simd_ops::wu64(rsi, rdx); rax = ((r8.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; crate::simd_ops::wu64(rsi.wrapping_add(0x8u64), rax); rax = rsi; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; crate::simd_ops::wu8(rsi.wrapping_add(0x10u64), ((r9 as u8)) as u8); rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411c8230(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1410c3eb0; const P1: u64 = 0x1411c8030; const P2: u64 = 0x14241d6f0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 15>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rbp = r8; rdi = rdx; r13 = rcx; r15 = (crate::simd_ops::ru32(rcx.wrapping_add(0x8u64))) as u32 as u64; rbx = (crate::simd_ops::ru64(rcx)) as u64; r8 = ((rbx.wrapping_add(r15.wrapping_mul(8))) as u64) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0xcu64))) as u32 as u64; r12 = rbp; r12 = ((r12).wrapping_sub(rdx)) as u64; r12 = ((((r12) as i64).wrapping_shr(3u32) as u64)) as u64; fl_l = r12; fl_r = rax; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 10; continue; } bb = 1; continue; }
            1 => { rdx = rbx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu32(r13.wrapping_add(0x8u64), (0x0u32) as u32); rax = (crate::simd_ops::ru32(r13.wrapping_add(0xcu64))) as u32 as u64; rdx = ((rax as u32)) as u32 as u64; rdx = ((rdx).wrapping_shr(1u32)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; fl_l = rdx; fl_r = r12; rdx = (if (((fl_l as u64)) < ((fl_r as u64))) { r12 } else { rdx }) as u64; rcx = r13; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(r13)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); fl_l = rdi; fl_r = rbp; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 9; continue; } bb = 2; continue; }
            2 => { bb = 3; continue; }
            3 => { rdx = (crate::simd_ops::ru64(rdi)) as u64; crate::simd_ops::wu64(rax, rdx); fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 4; continue; }
            4 => { rax = 0x1; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rdx.wrapping_add(0x24u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; rax = (((rax as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 7; continue; } bb = 5; continue; }
            5 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x28u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; bb = 8; continue; }
            8 => { rax = ((rax).wrapping_add(0x8u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); rdi = ((rdi).wrapping_add(0x8u64)) as u64; fl_l = rdi; fl_r = rbp; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 9; continue; }
            9 => { bb = 44; continue; }
            10 => { fl_l = r12; fl_r = r15; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 32; continue; } bb = 11; continue; }
            11 => { r14 = ((rdx.wrapping_add(r15.wrapping_mul(8))) as u64) as u64; rbp = r15; fl_l = r15; fl_r = r15; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 23; continue; } bb = 12; continue; }
            12 => { bb = 13; continue; }
            13 => { rsi = (crate::simd_ops::ru64(rdi)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x80u64), rsi); fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 14; continue; }
            14 => { rax = 0x1; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rsi.wrapping_add(0x24u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; rax = (((rax as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 17; continue; } bb = 15; continue; }
            15 => { rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x28u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 16; continue; }
            16 => { rdx = rsi; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 17; continue; }
            17 => { rdx = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 21; continue; } bb = 18; continue; }
            18 => { rax = 0xffffffff; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rdx.wrapping_add(0x24u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 21; continue; } bb = 19; continue; }
            19 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x28u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 21; continue; } bb = 20; continue; }
            20 => { rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 21; continue; }
            21 => { crate::simd_ops::wu64(rbx, rsi); rbx = ((rbx).wrapping_add(0x8u64)) as u64; fl_l = rbp; fl_r = (0x1u64) as u64; rbp = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 23; continue; } bb = 22; continue; }
            22 => { fl_l = ((rdi).wrapping_add(0x8u64)) as u64; fl_r = fl_l; rdi = ((fl_l as u64)) as u64; bb = 13; continue; }
            23 => { rdi = r12; fl_l = rdi; fl_r = r15; rdi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 44; continue; } bb = 24; continue; }
            24 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rbx); bb = 25; continue; }
            25 => { rdx = (crate::simd_ops::ru64(r14)) as u64; crate::simd_ops::wu64(rbx, rdx); fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 30; continue; } bb = 26; continue; }
            26 => { rax = 0x1; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rdx.wrapping_add(0x24u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; rax = (((rax as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 29; continue; } bb = 27; continue; }
            27 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x28u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 29; continue; } bb = 28; continue; }
            28 => { rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 29; continue; }
            29 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; bb = 30; continue; }
            30 => { rbx = ((rbx).wrapping_add(0x8u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rbx); r14 = ((r14).wrapping_add(0x8u64)) as u64; fl_l = rdi; fl_r = (0x1u64) as u64; rdi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 25; continue; } bb = 31; continue; }
            31 => { bb = 44; continue; }
            32 => { fl_l = rdi; fl_r = rbp; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 43; continue; } bb = 33; continue; }
            33 => { bb = 34; continue; }
            34 => { rsi = (crate::simd_ops::ru64(rdi)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x88u64), rsi); fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 38; continue; } bb = 35; continue; }
            35 => { rax = 0x1; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rsi.wrapping_add(0x24u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; rax = (((rax as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 38; continue; } bb = 36; continue; }
            36 => { rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x28u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 38; continue; } bb = 37; continue; }
            37 => { rdx = rsi; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 38; continue; }
            38 => { rdx = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 42; continue; } bb = 39; continue; }
            39 => { rax = 0xffffffff; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rdx.wrapping_add(0x24u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 42; continue; } bb = 40; continue; }
            40 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x28u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 42; continue; } bb = 41; continue; }
            41 => { rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 42; continue; }
            42 => { crate::simd_ops::wu64(rbx, rsi); rbx = ((rbx).wrapping_add(0x8u64)) as u64; rdi = ((rdi).wrapping_add(0x8u64)) as u64; fl_l = rdi; fl_r = rbp; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 34; continue; } bb = 43; continue; }
            43 => { rcx = (crate::simd_ops::ru64(r13)) as u64; rax = (crate::simd_ops::ru32(r13.wrapping_add(0x8u64))) as u32 as u64; r8 = ((rcx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rdx = ((rcx.wrapping_add(r12.wrapping_mul(8))) as u64) as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); bb = 44; continue; }
            44 => { crate::simd_ops::wu32(r13.wrapping_add(0x8u64), ((r12 as u32)) as u32); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411c8480(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1410c3eb0; const P1: u64 = 0x1411ca040; const P2: u64 = 0x14241d6f0; const P3: u64 = 0x1410d7d70;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 15>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rbp = r8; rdi = rdx; r13 = rcx; r15 = (crate::simd_ops::ru32(rcx.wrapping_add(0x8u64))) as u32 as u64; rbx = (crate::simd_ops::ru64(rcx)) as u64; r8 = ((rbx.wrapping_add(r15.wrapping_mul(8))) as u64) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0xcu64))) as u32 as u64; r12 = rbp; r12 = ((r12).wrapping_sub(rdx)) as u64; r12 = ((((r12) as i64).wrapping_shr(3u32) as u64)) as u64; fl_l = r12; fl_r = rax; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 10; continue; } bb = 1; continue; }
            1 => { rdx = rbx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu32(r13.wrapping_add(0x8u64), (0x0u32) as u32); rax = (crate::simd_ops::ru32(r13.wrapping_add(0xcu64))) as u32 as u64; rdx = ((rax as u32)) as u32 as u64; rdx = ((rdx).wrapping_shr(1u32)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; fl_l = rdx; fl_r = r12; rdx = (if (((fl_l as u64)) < ((fl_r as u64))) { r12 } else { rdx }) as u64; rcx = r13; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(r13)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rcx); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rcx); fl_l = rdi; fl_r = rbp; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 9; continue; } bb = 2; continue; }
            2 => { bb = 3; continue; }
            3 => { rdx = (crate::simd_ops::ru64(rdi)) as u64; crate::simd_ops::wu64(rcx, rdx); fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 4; continue; }
            4 => { rax = 0x1; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rdx.wrapping_add(0x24u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; rax = (((rax as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 7; continue; } bb = 5; continue; }
            5 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x28u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 7; continue; }
            7 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; bb = 8; continue; }
            8 => { rcx = ((rcx).wrapping_add(0x8u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rcx); rdi = ((rdi).wrapping_add(0x8u64)) as u64; fl_l = rdi; fl_r = rbp; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 9; continue; }
            9 => { rdx = rcx; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 44; continue; }
            10 => { fl_l = r12; fl_r = r15; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 32; continue; } bb = 11; continue; }
            11 => { r14 = ((rdx.wrapping_add(r15.wrapping_mul(8))) as u64) as u64; rbp = r15; fl_l = r15; fl_r = r15; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 23; continue; } bb = 12; continue; }
            12 => { bb = 13; continue; }
            13 => { rsi = (crate::simd_ops::ru64(rdi)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x80u64), rsi); fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 14; continue; }
            14 => { rax = 0x1; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rsi.wrapping_add(0x24u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; rax = (((rax as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 17; continue; } bb = 15; continue; }
            15 => { rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x28u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 16; continue; }
            16 => { rdx = rsi; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 17; continue; }
            17 => { rdx = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 21; continue; } bb = 18; continue; }
            18 => { rax = 0xffffffff; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rdx.wrapping_add(0x24u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 21; continue; } bb = 19; continue; }
            19 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x28u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 21; continue; } bb = 20; continue; }
            20 => { rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 21; continue; }
            21 => { crate::simd_ops::wu64(rbx, rsi); rbx = ((rbx).wrapping_add(0x8u64)) as u64; fl_l = rbp; fl_r = (0x1u64) as u64; rbp = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 23; continue; } bb = 22; continue; }
            22 => { fl_l = ((rdi).wrapping_add(0x8u64)) as u64; fl_r = fl_l; rdi = ((fl_l as u64)) as u64; bb = 13; continue; }
            23 => { rdi = r12; fl_l = rdi; fl_r = r15; rdi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 44; continue; } bb = 24; continue; }
            24 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rbx); bb = 25; continue; }
            25 => { rdx = (crate::simd_ops::ru64(r14)) as u64; crate::simd_ops::wu64(rbx, rdx); fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 30; continue; } bb = 26; continue; }
            26 => { rax = 0x1; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rdx.wrapping_add(0x24u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; rax = (((rax as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 29; continue; } bb = 27; continue; }
            27 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x28u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 29; continue; } bb = 28; continue; }
            28 => { rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 29; continue; }
            29 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; bb = 30; continue; }
            30 => { rbx = ((rbx).wrapping_add(0x8u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rbx); r14 = ((r14).wrapping_add(0x8u64)) as u64; fl_l = rdi; fl_r = (0x1u64) as u64; rdi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 25; continue; } bb = 31; continue; }
            31 => { rdx = rbx; rcx = rbx; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 44; continue; }
            32 => { fl_l = rdi; fl_r = rbp; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 43; continue; } bb = 33; continue; }
            33 => { bb = 34; continue; }
            34 => { rsi = (crate::simd_ops::ru64(rdi)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x88u64), rsi); fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 38; continue; } bb = 35; continue; }
            35 => { rax = 0x1; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rsi.wrapping_add(0x24u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; rax = (((rax as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 38; continue; } bb = 36; continue; }
            36 => { rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x28u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 38; continue; } bb = 37; continue; }
            37 => { rdx = rsi; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 38; continue; }
            38 => { rdx = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 42; continue; } bb = 39; continue; }
            39 => { rax = 0xffffffff; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rdx.wrapping_add(0x24u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 42; continue; } bb = 40; continue; }
            40 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x28u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 42; continue; } bb = 41; continue; }
            41 => { rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 42; continue; }
            42 => { crate::simd_ops::wu64(rbx, rsi); rbx = ((rbx).wrapping_add(0x8u64)) as u64; rdi = ((rdi).wrapping_add(0x8u64)) as u64; fl_l = rdi; fl_r = rbp; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 34; continue; } bb = 43; continue; }
            43 => { rcx = (crate::simd_ops::ru64(r13)) as u64; rax = (crate::simd_ops::ru32(r13.wrapping_add(0x8u64))) as u32 as u64; r8 = ((rcx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rdx = ((rcx.wrapping_add(r12.wrapping_mul(8))) as u64) as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); bb = 44; continue; }
            44 => { crate::simd_ops::wu32(r13.wrapping_add(0x8u64), ((r12 as u32)) as u32); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411c86f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1411c99c0; const P1: u64 = 0x1401de830;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; r9 = (crate::simd_ops::ru32(rcx.wrapping_add(0x8u64))) as u32 as u64; rbx = rdx; rdx = r8; rax = (0xe38e38e38e38e39u64) as u64; rdx = ((rdx).wrapping_sub(rbx)) as u64; r14 = r8; let prod = (rax as i64 as i128).wrapping_mul((rdx) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rsi = rcx; rbp = rdx; rbp = ((((rbp) as i64).wrapping_shr(1u32) as u64)) as u64; rax = rbp; rax = ((rax).wrapping_shr(63u32)) as u64; rbp = ((rbp).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0xcu64))) as u32 as u64; fl_l = rbp; fl_r = rax; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 8; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), r13); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), r15); r15 = ((rax as u32)) as u32 as u64; r15 = ((r15).wrapping_shr(1u32)) as u64; r15 = ((r15).wrapping_add(rax)) as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x8u64), (0x0u32) as u32); fl_l = r15; fl_r = rbp; r15 = (if (((fl_l as u64)) < ((fl_r as u64))) { rbp } else { r15 }) as u64; rdx = r15; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = (crate::simd_ops::ru64(rsi)) as u64; r13 = rax; rcx = (crate::simd_ops::ru32(rsi.wrapping_add(0x8u64))) as u32 as u64; rdi = rax.wrapping_add(0x18); rdx = ((rcx.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; r9 = ((r8.wrapping_add(rdx.wrapping_mul(4))) as u64) as u64; fl_l = r8; fl_r = r9; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 4; continue; } bb = 2; continue; }
            2 => { rcx = rdi; rdx = r8.wrapping_add(0x18); bb = 3; continue; }
            3 => { xmm0 = ((crate::simd_ops::ru64(r8)) as u64) as u128; crate::simd_ops::wu64(rcx.wrapping_sub(0x18u64), ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(r8.wrapping_add(0x8u64))) as u32 as u64; r8 = ((r8).wrapping_add(0x24u64)) as u64; crate::simd_ops::wu32(rcx.wrapping_sub(0x10u64), ((rax as u32)) as u32); rcx = rcx.wrapping_add(0x24); xmm0 = ((crate::simd_ops::ru64(rdx.wrapping_sub(0xcu64))) as u64) as u128; rdx = rdx.wrapping_add(0x24); crate::simd_ops::wu64(rcx.wrapping_sub(0x30u64), ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(rdx.wrapping_sub(0x28u64))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_sub(0x28u64), ((rax as u32)) as u32); xmm0 = ((crate::simd_ops::ru64(rdx.wrapping_sub(0x24u64))) as u64) as u128; crate::simd_ops::wu64(rcx.wrapping_sub(0x24u64), ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(rdx.wrapping_sub(0x1cu64))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_sub(0x1cu64), ((rax as u32)) as u32); fl_l = r8; fl_r = r9; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 4; continue; }
            4 => { rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(rsi.wrapping_add(0xcu64), ((r15 as u32)) as u32); r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; crate::simd_ops::wu64(rsi, r13); crate::simd_ops::wu32(rsi.wrapping_add(0x8u64), (0x0u32) as u32); fl_l = rbx; fl_r = r14; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 7; continue; } bb = 5; continue; }
            5 => { fl_l = rbx; fl_r = r13; rbx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; bb = 6; continue; }
            6 => { xmm0 = ((crate::simd_ops::ru64(rbx.wrapping_add(rdi).wrapping_sub(0x18u64))) as u64) as u128; crate::simd_ops::wu64(rdi.wrapping_sub(0x18u64), ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(rbx.wrapping_add(rdi).wrapping_sub(0x10u64))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_sub(0x10u64), ((rax as u32)) as u32); xmm0 = ((crate::simd_ops::ru64(rbx.wrapping_add(rdi).wrapping_sub(0xcu64))) as u64) as u128; crate::simd_ops::wu64(rdi.wrapping_sub(0xcu64), ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(rbx.wrapping_add(rdi).wrapping_sub(0x4u64))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_sub(0x4u64), ((rax as u32)) as u32); xmm0 = ((crate::simd_ops::ru64(rbx.wrapping_add(rdi))) as u64) as u128; crate::simd_ops::wu64(rdi, ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(rbx.wrapping_add(rdi).wrapping_add(0x8u64))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x8u64), ((rax as u32)) as u32); rdi = ((rdi).wrapping_add(0x24u64)) as u64; rax = rbx.wrapping_sub(0x18); rax = ((rax).wrapping_add(rdi)) as u64; fl_l = rax; fl_r = r14; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 6; continue; } bb = 7; continue; }
            7 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; crate::simd_ops::wu32(rsi.wrapping_add(0x8u64), ((rbp as u32)) as u32); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            8 => { rcx = (crate::simd_ops::ru64(rcx)) as u64; fl_l = rbp; fl_r = r9; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 17; continue; } bb = 9; continue; }
            9 => { rax = ((r9.wrapping_add(r9.wrapping_mul(8))) as u64) as u64; r10 = ((rbx.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 10; continue; }
            10 => { xmm0 = ((crate::simd_ops::ru64(rbx)) as u64) as u128; crate::simd_ops::wu64(rcx, ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; r8 = r9.wrapping_sub(0x1); crate::simd_ops::wu32(rcx.wrapping_add(0x8u64), ((rax as u32)) as u32); xmm0 = ((crate::simd_ops::ru64(rbx.wrapping_add(0xcu64))) as u64) as u128; crate::simd_ops::wu64(rcx.wrapping_add(0xcu64), ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x14u64))) as u32 as u64; rbx = ((rbx).wrapping_add(0x18u64)) as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x14u64), ((rax as u32)) as u32); xmm0 = ((crate::simd_ops::ru64(rbx)) as u64) as u128; crate::simd_ops::wu64(rcx.wrapping_add(0x18u64), ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x20u64), ((rax as u32)) as u32); rcx = ((rcx).wrapping_add(0x24u64)) as u64; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 11; continue; }
            11 => { rdx = rcx.wrapping_add(0x18); bb = 12; continue; }
            12 => { rbx = rbx.wrapping_add(0x24); xmm0 = ((crate::simd_ops::ru64(rbx.wrapping_sub(0x18u64))) as u64) as u128; crate::simd_ops::wu64(rcx, ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(rbx.wrapping_sub(0x10u64))) as u32 as u64; rdx = rdx.wrapping_add(0x24); crate::simd_ops::wu32(rcx.wrapping_add(0x8u64), ((rax as u32)) as u32); rcx = ((rcx).wrapping_add(0x24u64)) as u64; xmm0 = ((crate::simd_ops::ru64(rbx.wrapping_sub(0xcu64))) as u64) as u128; crate::simd_ops::wu64(rdx.wrapping_sub(0x30u64), ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(rbx.wrapping_sub(0x4u64))) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_sub(0x28u64), ((rax as u32)) as u32); xmm0 = ((crate::simd_ops::ru64(rbx)) as u64) as u128; crate::simd_ops::wu64(rdx.wrapping_sub(0x24u64), ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_sub(0x1cu64), ((rax as u32)) as u32); fl_l = r8; fl_r = (0x1u64) as u64; r8 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 12; continue; } bb = 13; continue; }
            13 => { r8 = rbp; fl_l = r8; fl_r = r9; r8 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 16; continue; } bb = 14; continue; }
            14 => { rdx = r10.wrapping_add(0x18); fl_l = rcx; fl_r = r10; rcx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; bb = 15; continue; }
            15 => { xmm0 = ((crate::simd_ops::ru64(rdx.wrapping_sub(0x18u64))) as u64) as u128; crate::simd_ops::wu64(rcx.wrapping_add(rdx).wrapping_sub(0x18u64), ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(rdx.wrapping_sub(0x10u64))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(rdx).wrapping_sub(0x10u64), ((rax as u32)) as u32); xmm0 = ((crate::simd_ops::ru64(rdx.wrapping_sub(0xcu64))) as u64) as u128; crate::simd_ops::wu64(rcx.wrapping_add(rdx).wrapping_sub(0xcu64), ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(rdx.wrapping_sub(0x4u64))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(rdx).wrapping_sub(0x4u64), ((rax as u32)) as u32); xmm0 = ((crate::simd_ops::ru64(rdx)) as u64) as u128; crate::simd_ops::wu64(rcx.wrapping_add(rdx), ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x8u64))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(rdx).wrapping_add(0x8u64), ((rax as u32)) as u32); rdx = rdx.wrapping_add(0x24); fl_l = r8; fl_r = (0x1u64) as u64; r8 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 15; continue; } bb = 16; continue; }
            16 => { crate::simd_ops::wu32(rsi.wrapping_add(0x8u64), ((rbp as u32)) as u32); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            17 => { fl_l = rbx; fl_r = r14; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 16; continue; } bb = 18; continue; }
            18 => { rdx = rcx.wrapping_add(0x18); bb = 19; continue; }
            19 => { xmm0 = ((crate::simd_ops::ru64(rbx)) as u64) as u128; crate::simd_ops::wu64(rcx, ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; rdx = rdx.wrapping_add(0x24); crate::simd_ops::wu32(rcx.wrapping_add(0x8u64), ((rax as u32)) as u32); rcx = rcx.wrapping_add(0x24); xmm0 = ((crate::simd_ops::ru64(rbx.wrapping_add(0xcu64))) as u64) as u128; crate::simd_ops::wu64(rdx.wrapping_sub(0x30u64), ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x14u64))) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_sub(0x28u64), ((rax as u32)) as u32); xmm0 = ((crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64) as u128; crate::simd_ops::wu64(rdx.wrapping_sub(0x24u64), ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x20u64))) as u32 as u64; rbx = ((rbx).wrapping_add(0x24u64)) as u64; crate::simd_ops::wu32(rdx.wrapping_sub(0x1cu64), ((rax as u32)) as u32); fl_l = rbx; fl_r = r14; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 19; continue; } bb = 20; continue; }
            20 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; crate::simd_ops::wu32(rsi.wrapping_add(0x8u64), ((rbp as u32)) as u32); rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411c89c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1411ca0d0;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0xcu64))) as u32 as u64; rbx = r8; r9 = (crate::simd_ops::ru32(rcx.wrapping_add(0x8u64))) as u32 as u64; rbx = ((rbx).wrapping_sub(rdx)) as u64; rbx = ((((rbx) as i64).wrapping_shr(6u32) as u64)) as u64; rbp = r8; rsi = rdx; rdi = rcx; fl_l = rbx; fl_r = rax; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 5; continue; } bb = 1; continue; }
            1 => { rdx = ((rax as u32)) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x8u64), (0x0u32) as u32); rdx = ((rdx).wrapping_shr(1u32)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; fl_l = rdx; fl_r = rbx; rdx = (if (((fl_l as u64)) < ((fl_r as u64))) { rbx } else { rdx }) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = (crate::simd_ops::ru64(rdi)) as u64; fl_l = rsi; fl_r = rbp; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 17; continue; } bb = 2; continue; }
            2 => { rdx = r8.wrapping_add(0x20); rcx = rsi.wrapping_add(0x20); bb = 3; continue; }
            3 => { xmm0 = (crate::simd_ops::ru128(rcx.wrapping_sub(0x20u64))) as u128; crate::simd_ops::wu128(r8, (xmm0) as u128); xmm0 = (crate::simd_ops::ru128(rcx.wrapping_sub(0x10u64))) as u128; crate::simd_ops::wu128(rdx.wrapping_sub(0x10u64), (xmm0) as u128); xmm0 = (crate::simd_ops::ru128(rcx)) as u128; crate::simd_ops::wu128(rdx, (xmm0) as u128); xmm0 = (crate::simd_ops::ru128(rcx.wrapping_add(0x10u64))) as u128; rcx = rcx.wrapping_add(0x40); rax = rcx.wrapping_sub(0x20); r8 = r8.wrapping_add(0x40); crate::simd_ops::wu128(rdx.wrapping_add(0x10u64), (xmm0) as u128); rdx = rdx.wrapping_add(0x40); fl_l = rax; fl_r = rbp; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 4; continue; }
            4 => { bb = 17; continue; }
            5 => { rcx = (crate::simd_ops::ru64(rcx)) as u64; fl_l = rbx; fl_r = r9; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 14; continue; } bb = 6; continue; }
            6 => { r8 = r9; r8 = ((r8).wrapping_shl(6u32)) as u64; r8 = ((r8).wrapping_add(rsi)) as u64; fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 7; continue; }
            7 => { xmm0 = (crate::simd_ops::ru128(rdx)) as u128; crate::simd_ops::wu128(rcx, (xmm0) as u128); xmm0 = (crate::simd_ops::ru128(rdx.wrapping_add(0x10u64))) as u128; crate::simd_ops::wu128(rcx.wrapping_add(0x10u64), (xmm0) as u128); rax = rdx.wrapping_add(0x20); xmm0 = (crate::simd_ops::ru128(rax)) as u128; crate::simd_ops::wu128(rcx.wrapping_add(0x20u64), (xmm0) as u128); xmm0 = (crate::simd_ops::ru128(rdx.wrapping_add(0x30u64))) as u128; crate::simd_ops::wu128(rcx.wrapping_add(0x30u64), (xmm0) as u128); rcx = ((rcx).wrapping_add(0x40u64)) as u64; r10 = r9.wrapping_sub(0x1); fl_l = r10; fl_r = r10; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 8; continue; }
            8 => { rdx = rcx.wrapping_add(0x20); bb = 9; continue; }
            9 => { rax = rax.wrapping_add(0x40); xmm0 = (crate::simd_ops::ru128(rax.wrapping_sub(0x20u64))) as u128; crate::simd_ops::wu128(rcx, (xmm0) as u128); xmm0 = (crate::simd_ops::ru128(rax.wrapping_sub(0x10u64))) as u128; crate::simd_ops::wu128(rdx.wrapping_sub(0x10u64), (xmm0) as u128); xmm0 = (crate::simd_ops::ru128(rax)) as u128; crate::simd_ops::wu128(rdx, (xmm0) as u128); xmm0 = (crate::simd_ops::ru128(rax.wrapping_add(0x10u64))) as u128; rcx = ((rcx).wrapping_add(0x40u64)) as u64; crate::simd_ops::wu128(rdx.wrapping_add(0x10u64), (xmm0) as u128); rdx = rdx.wrapping_add(0x40); fl_l = r10; fl_r = (0x1u64) as u64; r10 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 9; continue; } bb = 10; continue; }
            10 => { rdx = rbx; fl_l = rdx; fl_r = r9; rdx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 17; continue; } bb = 11; continue; }
            11 => { rax = r8.wrapping_add(0x20); fl_l = rcx; fl_r = r8; rcx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; bb = 12; continue; }
            12 => { xmm0 = (crate::simd_ops::ru128(rax.wrapping_sub(0x20u64))) as u128; crate::simd_ops::wu128(rcx.wrapping_add(rax).wrapping_sub(0x20u64), (xmm0) as u128); xmm0 = (crate::simd_ops::ru128(rax.wrapping_sub(0x10u64))) as u128; crate::simd_ops::wu128(rcx.wrapping_add(rax).wrapping_sub(0x10u64), (xmm0) as u128); xmm0 = (crate::simd_ops::ru128(rax)) as u128; crate::simd_ops::wu128(rcx.wrapping_add(rax), (xmm0) as u128); xmm0 = (crate::simd_ops::ru128(rax.wrapping_add(0x10u64))) as u128; crate::simd_ops::wu128(rcx.wrapping_add(rax).wrapping_add(0x10u64), (xmm0) as u128); rax = rax.wrapping_add(0x40); fl_l = rdx; fl_r = (0x1u64) as u64; rdx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 12; continue; } bb = 13; continue; }
            13 => { bb = 17; continue; }
            14 => { fl_l = rsi; fl_r = rbp; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 17; continue; } bb = 15; continue; }
            15 => { r8 = rcx.wrapping_add(0x20); fl_l = ((rdx).wrapping_add(0x20u64)) as u64; fl_r = fl_l; rdx = ((fl_l as u64)) as u64; bb = 16; continue; }
            16 => { xmm0 = (crate::simd_ops::ru128(rdx.wrapping_sub(0x20u64))) as u128; crate::simd_ops::wu128(rcx, (xmm0) as u128); xmm0 = (crate::simd_ops::ru128(rdx.wrapping_sub(0x10u64))) as u128; crate::simd_ops::wu128(r8.wrapping_sub(0x10u64), (xmm0) as u128); xmm0 = (crate::simd_ops::ru128(rdx)) as u128; crate::simd_ops::wu128(r8, (xmm0) as u128); xmm0 = (crate::simd_ops::ru128(rdx.wrapping_add(0x10u64))) as u128; rdx = rdx.wrapping_add(0x40); rax = rdx.wrapping_sub(0x20); rcx = rcx.wrapping_add(0x40); crate::simd_ops::wu128(r8.wrapping_add(0x10u64), (xmm0) as u128); r8 = r8.wrapping_add(0x40); fl_l = rax; fl_r = rbp; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 16; continue; } bb = 17; continue; }
            17 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x8u64), ((rbx as u32)) as u32); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411c8bd0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1410c3f20; const P1: u64 = 0x1401e2180; const P2: u64 = 0x1410c0680; const P3: u64 = 0x14241d6f0; const P4: u64 = 0x1403b3290;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rbp = r8; rdi = rdx; r14 = rcx; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x8u64))) as u32 as u64; rdx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rdx = ((rdx.wrapping_mul(8)) as u64) as u64; rbx = (crate::simd_ops::ru64(rcx)) as u64; r8 = ((rbx.wrapping_add(rdx)) as u64) as u64; r10 = (0x2aaaaaaaaaaaaaabu64) as u64; rax = r10; let prod = (rax as i64 as i128).wrapping_mul((rdx) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; r15 = rdx; r15 = ((((r15) as i64).wrapping_shr(2u32) as u64)) as u64; rcx = r15; rcx = ((rcx).wrapping_shr(63u32)) as u64; r15 = ((r15).wrapping_add(rcx)) as u64; r9 = (crate::simd_ops::ru32(r14.wrapping_add(0xcu64))) as u32 as u64; rcx = rbp; rcx = ((rcx).wrapping_sub(rdi)) as u64; rax = r10; let prod = (rax as i64 as i128).wrapping_mul((rcx) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; r12 = rdx; r12 = ((((r12) as i64).wrapping_shr(2u32) as u64)) as u64; rax = r12; rax = ((rax).wrapping_shr(63u32)) as u64; r12 = ((r12).wrapping_add(rax)) as u64; fl_l = r12; fl_r = r9; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 13; continue; } bb = 1; continue; }
            1 => { rdx = rbx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); r15 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(r14.wrapping_add(0x8u64), ((r15 as u32)) as u32); rax = (crate::simd_ops::ru32(r14.wrapping_add(0xcu64))) as u32 as u64; rsi = ((rax as u32)) as u32 as u64; rsi = ((rsi).wrapping_shr(1u32)) as u64; rsi = ((rsi).wrapping_add(rax)) as u64; fl_l = rsi; fl_r = r12; rsi = (if (((fl_l as u64)) < ((fl_r as u64))) { r12 } else { rsi }) as u64; rdx = rsi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rbx = rax; r8 = (crate::simd_ops::ru64(r14)) as u64; rcx = (crate::simd_ops::ru32(r14.wrapping_add(0x8u64))) as u32 as u64; rdx = ((rcx.wrapping_add(rcx.wrapping_mul(2))) as u64) as u64; r9 = ((r8.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64; fl_l = r8; fl_r = r9; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 4; continue; } bb = 2; continue; }
            2 => { rdx = rax.wrapping_add(0x10); bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(r8)) as u64; crate::simd_ops::wu64(rdx.wrapping_sub(0x10u64), rcx); rax = (crate::simd_ops::ru32(r8.wrapping_add(0x8u64))) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_sub(0x8u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(r8.wrapping_add(0x10u64))) as u64; crate::simd_ops::wu64(r8.wrapping_add(0x10u64), r15); crate::simd_ops::wu64(rdx, rax); rdx = rdx.wrapping_add(0x18); r8 = ((r8).wrapping_add(0x18u64)) as u64; fl_l = r8; fl_r = r9; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 4; continue; }
            4 => { rcx = r14; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(r14, rbx); crate::simd_ops::wu32(r14.wrapping_add(0x8u64), ((r15 as u32)) as u32); crate::simd_ops::wu32(r14.wrapping_add(0xcu64), ((rsi as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rbx); fl_l = rdi; fl_r = rbp; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 12; continue; } bb = 5; continue; }
            5 => { bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(rdi)) as u64; crate::simd_ops::wu64(rbx, rax); rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x8u64), ((rax as u32)) as u32); rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), rdx); fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 7; continue; }
            7 => { rax = 0x1; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rdx.wrapping_add(0x24u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; rax = (((rax as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 10; continue; } bb = 8; continue; }
            8 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x28u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 10; continue; }
            10 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; bb = 11; continue; }
            11 => { rbx = ((rbx).wrapping_add(0x18u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rbx); rdi = ((rdi).wrapping_add(0x18u64)) as u64; fl_l = rdi; fl_r = rbp; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 6; continue; } bb = 12; continue; }
            12 => { bb = 39; continue; }
            13 => { fl_l = r12; fl_r = r15; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 27; continue; } bb = 14; continue; }
            14 => { rax = ((r15.wrapping_add(r15.wrapping_mul(2))) as u64) as u64; rbp = ((rdi.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = r15; fl_r = r15; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 18; continue; } bb = 15; continue; }
            15 => { rax = (crate::simd_ops::ru64(rdi)) as u64; crate::simd_ops::wu64(rbx, rax); rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x8u64), ((rax as u32)) as u32); rdi = ((rdi).wrapping_add(0x10u64)) as u64; rcx = rbx.wrapping_add(0x10); rdx = rdi; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rbx = ((rbx).wrapping_add(0x18u64)) as u64; rsi = r15.wrapping_sub(0x1); fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 18; continue; } bb = 16; continue; }
            16 => { bb = 17; continue; }
            17 => { rdi = ((rdi).wrapping_add(0x18u64)) as u64; rax = (crate::simd_ops::ru64(rdi.wrapping_sub(0x10u64))) as u64; crate::simd_ops::wu64(rbx, rax); rax = (crate::simd_ops::ru32(rdi.wrapping_sub(0x8u64))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x8u64), ((rax as u32)) as u32); rcx = rbx.wrapping_add(0x10); rdx = rdi; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rbx = ((rbx).wrapping_add(0x18u64)) as u64; fl_l = rsi; fl_r = (0x1u64) as u64; rsi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 17; continue; } bb = 18; continue; }
            18 => { rsi = r12; fl_l = rsi; fl_r = r15; rsi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 39; continue; } bb = 19; continue; }
            19 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rbx); rdi = rbp.wrapping_add(0x10); bb = 20; continue; }
            20 => { rax = (crate::simd_ops::ru64(rdi.wrapping_sub(0x10u64))) as u64; crate::simd_ops::wu64(rbx, rax); rax = (crate::simd_ops::ru32(rdi.wrapping_sub(0x8u64))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x8u64), ((rax as u32)) as u32); rdx = (crate::simd_ops::ru64(rdi)) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), rdx); fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 25; continue; } bb = 21; continue; }
            21 => { rax = 0x1; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rdx.wrapping_add(0x24u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; rax = (((rax as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 24; continue; } bb = 22; continue; }
            22 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x28u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 24; continue; } bb = 23; continue; }
            23 => { rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 24; continue; }
            24 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; bb = 25; continue; }
            25 => { rbx = ((rbx).wrapping_add(0x18u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rbx); rdi = ((rdi).wrapping_add(0x18u64)) as u64; fl_l = rsi; fl_r = (0x1u64) as u64; rsi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 20; continue; } bb = 26; continue; }
            26 => { bb = 39; continue; }
            27 => { fl_l = rdi; fl_r = rbp; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 38; continue; } bb = 28; continue; }
            28 => { bb = 29; continue; }
            29 => { rax = (crate::simd_ops::ru64(rdi)) as u64; crate::simd_ops::wu64(rbx, rax); rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x8u64), ((rax as u32)) as u32); rsi = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), rsi); fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 33; continue; } bb = 30; continue; }
            30 => { rax = 0x1; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rsi.wrapping_add(0x24u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; rax = (((rax as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 33; continue; } bb = 31; continue; }
            31 => { rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x28u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 33; continue; } bb = 32; continue; }
            32 => { rdx = rsi; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 33; continue; }
            33 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 37; continue; } bb = 34; continue; }
            34 => { rax = 0xffffffff; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rdx.wrapping_add(0x24u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 37; continue; } bb = 35; continue; }
            35 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x28u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 37; continue; } bb = 36; continue; }
            36 => { rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 37; continue; }
            37 => { crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), rsi); rbx = ((rbx).wrapping_add(0x18u64)) as u64; rdi = ((rdi).wrapping_add(0x18u64)) as u64; fl_l = rdi; fl_r = rbp; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 29; continue; } bb = 38; continue; }
            38 => { rdx = (crate::simd_ops::ru64(r14)) as u64; rax = (crate::simd_ops::ru32(r14.wrapping_add(0x8u64))) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; r8 = ((rdx.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; rax = ((r12.wrapping_add(r12.wrapping_mul(2))) as u64) as u64; rdx = ((rdx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); bb = 39; continue; }
            39 => { crate::simd_ops::wu32(r14.wrapping_add(0x8u64), ((r12 as u32)) as u32); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411c8ef0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1410c3f90; const P1: u64 = 0x1401e2180; const P2: u64 = 0x1410c0740; const P3: u64 = 0x14241d6f0; const P4: u64 = 0x1403b3290;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rbp = r8; rdi = rdx; r14 = rcx; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x8u64))) as u32 as u64; rdx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rdx = ((rdx.wrapping_mul(8)) as u64) as u64; rbx = (crate::simd_ops::ru64(rcx)) as u64; r8 = ((rbx.wrapping_add(rdx)) as u64) as u64; r10 = (0x2aaaaaaaaaaaaaabu64) as u64; rax = r10; let prod = (rax as i64 as i128).wrapping_mul((rdx) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; r15 = rdx; r15 = ((((r15) as i64).wrapping_shr(2u32) as u64)) as u64; rcx = r15; rcx = ((rcx).wrapping_shr(63u32)) as u64; r15 = ((r15).wrapping_add(rcx)) as u64; r9 = (crate::simd_ops::ru32(r14.wrapping_add(0xcu64))) as u32 as u64; rcx = rbp; rcx = ((rcx).wrapping_sub(rdi)) as u64; rax = r10; let prod = (rax as i64 as i128).wrapping_mul((rcx) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; r12 = rdx; r12 = ((((r12) as i64).wrapping_shr(2u32) as u64)) as u64; rax = r12; rax = ((rax).wrapping_shr(63u32)) as u64; r12 = ((r12).wrapping_add(rax)) as u64; fl_l = r12; fl_r = r9; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 13; continue; } bb = 1; continue; }
            1 => { rdx = rbx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); r15 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(r14.wrapping_add(0x8u64), ((r15 as u32)) as u32); rax = (crate::simd_ops::ru32(r14.wrapping_add(0xcu64))) as u32 as u64; rsi = ((rax as u32)) as u32 as u64; rsi = ((rsi).wrapping_shr(1u32)) as u64; rsi = ((rsi).wrapping_add(rax)) as u64; fl_l = rsi; fl_r = r12; rsi = (if (((fl_l as u64)) < ((fl_r as u64))) { r12 } else { rsi }) as u64; rdx = rsi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rbx = rax; r8 = (crate::simd_ops::ru64(r14)) as u64; rcx = (crate::simd_ops::ru32(r14.wrapping_add(0x8u64))) as u32 as u64; rdx = ((rcx.wrapping_add(rcx.wrapping_mul(2))) as u64) as u64; r9 = ((r8.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64; fl_l = r8; fl_r = r9; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 4; continue; } bb = 2; continue; }
            2 => { rdx = rax.wrapping_add(0x10); bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(r8)) as u64; crate::simd_ops::wu64(r8, r15); crate::simd_ops::wu64(rdx.wrapping_sub(0x10u64), rcx); rax = (crate::simd_ops::ru64(r8.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rdx.wrapping_sub(0x8u64), rax); rax = (crate::simd_ops::ru64(r8.wrapping_add(0x10u64))) as u64; crate::simd_ops::wu64(rdx, rax); rdx = rdx.wrapping_add(0x18); r8 = ((r8).wrapping_add(0x18u64)) as u64; fl_l = r8; fl_r = r9; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 4; continue; }
            4 => { rcx = r14; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(r14, rbx); crate::simd_ops::wu32(r14.wrapping_add(0x8u64), ((r15 as u32)) as u32); crate::simd_ops::wu32(r14.wrapping_add(0xcu64), ((rsi as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rbx); fl_l = rdi; fl_r = rbp; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 12; continue; } bb = 5; continue; }
            5 => { bb = 6; continue; }
            6 => { rdx = (crate::simd_ops::ru64(rdi)) as u64; crate::simd_ops::wu64(rbx, rdx); fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 7; continue; }
            7 => { rax = 0x1; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rdx.wrapping_add(0x24u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; rax = (((rax as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 10; continue; } bb = 8; continue; }
            8 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x28u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 10; continue; }
            10 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; bb = 11; continue; }
            11 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), rax); rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), rax); rbx = ((rbx).wrapping_add(0x18u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rbx); rdi = ((rdi).wrapping_add(0x18u64)) as u64; fl_l = rdi; fl_r = rbp; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 6; continue; } bb = 12; continue; }
            12 => { bb = 39; continue; }
            13 => { fl_l = r12; fl_r = r15; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 27; continue; } bb = 14; continue; }
            14 => { rax = ((r15.wrapping_add(r15.wrapping_mul(2))) as u64) as u64; rbp = ((rdi.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = r15; fl_r = r15; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 18; continue; } bb = 15; continue; }
            15 => { rdx = rdi; rcx = rbx; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), rax); rdi = ((rdi).wrapping_add(0x10u64)) as u64; rax = (crate::simd_ops::ru64(rdi)) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), rax); rbx = ((rbx).wrapping_add(0x18u64)) as u64; rsi = r15.wrapping_sub(0x1); fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 18; continue; } bb = 16; continue; }
            16 => { bb = 17; continue; }
            17 => { rdi = rdi.wrapping_add(0x18); rdx = rdi.wrapping_sub(0x10); rcx = rbx; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rdi.wrapping_sub(0x8u64))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), rax); rax = (crate::simd_ops::ru64(rdi)) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), rax); rbx = ((rbx).wrapping_add(0x18u64)) as u64; fl_l = rsi; fl_r = (0x1u64) as u64; rsi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 17; continue; } bb = 18; continue; }
            18 => { rsi = r12; fl_l = rsi; fl_r = r15; rsi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 39; continue; } bb = 19; continue; }
            19 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rbx); rdi = rbp.wrapping_add(0x10); bb = 20; continue; }
            20 => { rdx = (crate::simd_ops::ru64(rdi.wrapping_sub(0x10u64))) as u64; crate::simd_ops::wu64(rbx, rdx); fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 25; continue; } bb = 21; continue; }
            21 => { rax = 0x1; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rdx.wrapping_add(0x24u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; rax = (((rax as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 24; continue; } bb = 22; continue; }
            22 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x28u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 24; continue; } bb = 23; continue; }
            23 => { rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 24; continue; }
            24 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; bb = 25; continue; }
            25 => { rax = (crate::simd_ops::ru64(rdi.wrapping_sub(0x8u64))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), rax); rax = (crate::simd_ops::ru64(rdi)) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), rax); rbx = ((rbx).wrapping_add(0x18u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rbx); rdi = ((rdi).wrapping_add(0x18u64)) as u64; fl_l = rsi; fl_r = (0x1u64) as u64; rsi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 20; continue; } bb = 26; continue; }
            26 => { bb = 39; continue; }
            27 => { fl_l = rdi; fl_r = rbp; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 38; continue; } bb = 28; continue; }
            28 => { bb = 29; continue; }
            29 => { rsi = (crate::simd_ops::ru64(rdi)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), rsi); fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 33; continue; } bb = 30; continue; }
            30 => { rax = 0x1; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rsi.wrapping_add(0x24u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; rax = (((rax as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 33; continue; } bb = 31; continue; }
            31 => { rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x28u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 33; continue; } bb = 32; continue; }
            32 => { rdx = rsi; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 33; continue; }
            33 => { rdx = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 37; continue; } bb = 34; continue; }
            34 => { rax = 0xffffffff; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rdx.wrapping_add(0x24u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 37; continue; } bb = 35; continue; }
            35 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x28u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 37; continue; } bb = 36; continue; }
            36 => { rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 37; continue; }
            37 => { crate::simd_ops::wu64(rbx, rsi); rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), rax); rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), rax); rbx = ((rbx).wrapping_add(0x18u64)) as u64; rdi = ((rdi).wrapping_add(0x18u64)) as u64; fl_l = rdi; fl_r = rbp; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 29; continue; } bb = 38; continue; }
            38 => { rdx = (crate::simd_ops::ru64(r14)) as u64; rax = (crate::simd_ops::ru32(r14.wrapping_add(0x8u64))) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; r8 = ((rdx.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; rax = ((r12.wrapping_add(r12.wrapping_mul(2))) as u64) as u64; rdx = ((rdx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); bb = 39; continue; }
            39 => { crate::simd_ops::wu32(r14.wrapping_add(0x8u64), ((r12 as u32)) as u32); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411c9200(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1401e2180; const P1: u64 = 0x1401e84a0; const P2: u64 = 0x140236fa0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), r8); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x8u64))) as u32 as u64; r15 = rcx; rdi = rdx; r9 = (0x2aaaaaaaaaaaaaabu64) as u64; r13 = r8; rbx = (crate::simd_ops::ru64(r15)) as u64; rdx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; r8 = ((rdx.wrapping_mul(8)) as u64) as u64; rax = r9; let prod = (rax as i64 as i128).wrapping_mul((r8) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rax = r9; r14 = rdx; r14 = ((((r14) as i64).wrapping_shr(2u32) as u64)) as u64; rcx = r14; rcx = ((rcx).wrapping_shr(63u32)) as u64; r14 = ((r14).wrapping_add(rcx)) as u64; rcx = r13; rcx = ((rcx).wrapping_sub(rdi)) as u64; let prod = (rax as i64 as i128).wrapping_mul((rcx) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; r12 = rdx; r12 = ((((r12) as i64).wrapping_shr(2u32) as u64)) as u64; rax = r12; rax = ((rax).wrapping_shr(63u32)) as u64; r12 = ((r12).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(r15.wrapping_add(0xcu64))) as u32 as u64; fl_l = r12; fl_r = rax; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 12; continue; } bb = 1; continue; }
            1 => { rcx = ((rbx.wrapping_add(r8)) as u64) as u64; rsi = 0; fl_l = 0; fl_r = 0; fl_l = rbx; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 4; continue; } bb = 2; continue; }
            2 => { bb = 3; continue; }
            3 => { rax = rsi; let tmp_xchg: u64 = crate::simd_ops::atomic_xchg_u64(rbx.wrapping_add(0x8u64), rax); rax = (tmp_xchg) as u64; rbx = ((rbx).wrapping_add(0x18u64)) as u64; fl_l = rbx; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru32(r15.wrapping_add(0xcu64))) as u32 as u64; rcx = ((rax as u32)) as u32 as u64; crate::simd_ops::wu32(r15.wrapping_add(0x8u64), ((rsi as u32)) as u32); rcx = ((rcx).wrapping_shr(1u32)) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; fl_l = rcx; fl_r = r12; rcx = (if (((fl_l as u64)) < ((fl_r as u64))) { r12 } else { rcx }) as u64; rdx = rcx; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rcx); rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rbp = (crate::simd_ops::ru64(r15)) as u64; r14 = rax; rcx = (crate::simd_ops::ru32(r15.wrapping_add(0x8u64))) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), rax); rbx = r14.wrapping_add(0x10); rdx = ((rcx.wrapping_add(rcx.wrapping_mul(2))) as u64) as u64; rax = ((rdx.wrapping_mul(8)) as u64) as u64; rax = ((rax).wrapping_add(rbp)) as u64; fl_l = rbp; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 8; continue; } bb = 5; continue; }
            5 => { r14 = rbx; r13 = rax; bb = 6; continue; }
            6 => { rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(r14.wrapping_sub(0x10u64), rsi); crate::simd_ops::wu64(r14.wrapping_sub(0x8u64), rax); rcx = r14.wrapping_sub(0x10); crate::simd_ops::wu32(r14, ((rax as u32)) as u32); rdx = rbp; crate::simd_ops::wu8(r14.wrapping_add(0x4u64), ((rax as u8)) as u8); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rbp = ((rbp).wrapping_add(0x18u64)) as u64; r14 = r14.wrapping_add(0x18); fl_l = rbp; fl_r = r13; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 6; continue; } bb = 7; continue; }
            7 => { r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; bb = 8; continue; }
            8 => { rcx = r15; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu32(r15.wrapping_add(0xcu64), ((rax as u32)) as u32); crate::simd_ops::wu64(r15, r14); crate::simd_ops::wu32(r15.wrapping_add(0x8u64), ((rsi as u32)) as u32); fl_l = rdi; fl_r = r13; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 26; continue; } bb = 9; continue; }
            9 => { rdi = ((rdi).wrapping_sub(r14)) as u64; fl_l = ((rdi).wrapping_add(0xfffffffffffffff0u64)) as u64; fl_r = fl_l; rdi = ((fl_l as u64)) as u64; bb = 10; continue; }
            10 => { rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rbx.wrapping_sub(0x10u64), rsi); crate::simd_ops::wu64(rbx.wrapping_sub(0x8u64), rax); rcx = rbx.wrapping_sub(0x10); crate::simd_ops::wu32(rbx, ((rax as u32)) as u32); rdx = ((rdi.wrapping_add(rbx)) as u64) as u64; crate::simd_ops::wu8(rbx.wrapping_add(0x4u64), ((rax as u8)) as u8); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rbx = rbx.wrapping_add(0x18); rax = ((rdi.wrapping_add(rbx)) as u64) as u64; fl_l = rax; fl_r = r13; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 10; continue; } bb = 11; continue; }
            11 => { bb = 26; continue; }
            12 => { fl_l = r12; fl_r = r14; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 21; continue; } bb = 13; continue; }
            13 => { rax = ((r14.wrapping_add(r14.wrapping_mul(2))) as u64) as u64; r13 = ((rdi.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = r14; fl_r = r14; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 14; continue; }
            14 => { rdx = rdi; rcx = rbx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rbx = ((rbx).wrapping_add(0x18u64)) as u64; rsi = r14.wrapping_sub(0x1); fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 15; continue; }
            15 => { bb = 16; continue; }
            16 => { rdi = ((rdi).wrapping_add(0x18u64)) as u64; rcx = rbx; rdx = rdi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rbx = ((rbx).wrapping_add(0x18u64)) as u64; fl_l = rsi; fl_r = (0x1u64) as u64; rsi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 16; continue; } bb = 17; continue; }
            17 => { rbp = r12; fl_l = rbp; fl_r = r14; rbp = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 26; continue; } bb = 18; continue; }
            18 => { r13 = ((r13).wrapping_sub(rbx)) as u64; rdi = rbx.wrapping_add(0x10); rsi = 0; fl_l = 0; fl_r = 0; bb = 19; continue; }
            19 => { rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rdi.wrapping_sub(0x10u64), rsi); crate::simd_ops::wu64(rdi.wrapping_sub(0x8u64), rax); rcx = rdi.wrapping_sub(0x10); crate::simd_ops::wu32(rdi, ((rax as u32)) as u32); rdx = r13.wrapping_sub(0x10); rdx = ((rdx).wrapping_add(rdi)) as u64; crate::simd_ops::wu8(rdi.wrapping_add(0x4u64), ((rax as u8)) as u8); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rdi = ((rdi).wrapping_add(0x18u64)) as u64; fl_l = rbp; fl_r = (0x1u64) as u64; rbp = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 19; continue; } bb = 20; continue; }
            20 => { bb = 26; continue; }
            21 => { fl_l = rdi; fl_r = r13; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 23; continue; } bb = 22; continue; }
            22 => { rdx = rdi; rcx = rbx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rbx = ((rbx).wrapping_add(0x18u64)) as u64; rdi = ((rdi).wrapping_add(0x18u64)) as u64; fl_l = rdi; fl_r = r13; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 22; continue; } bb = 23; continue; }
            23 => { rax = (crate::simd_ops::ru32(r15.wrapping_add(0x8u64))) as u32 as u64; rdx = (crate::simd_ops::ru64(r15)) as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; r8 = ((rdx.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; rax = ((r12.wrapping_add(r12.wrapping_mul(2))) as u64) as u64; rcx = ((rdx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = rcx; fl_r = r8; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 26; continue; } bb = 24; continue; }
            24 => { rsi = 0; fl_l = 0; fl_r = 0; bb = 25; continue; }
            25 => { rax = rsi; let tmp_xchg: u64 = crate::simd_ops::atomic_xchg_u64(rcx.wrapping_add(0x8u64), rax); rax = (tmp_xchg) as u64; rcx = ((rcx).wrapping_add(0x18u64)) as u64; fl_l = rcx; fl_r = r8; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 25; continue; } bb = 26; continue; }
            26 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x88u64))) as u64; crate::simd_ops::wu32(r15.wrapping_add(0x8u64), ((r12 as u32)) as u32); rsp = ((rsp).wrapping_add(0x30u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411c9480(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1410c3eb0; const P1: u64 = 0x1401dec00; const P2: u64 = 0x1410c05d0; const P3: u64 = 0x14241d6f0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 15>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rbp = r8; rdi = rdx; r15 = rcx; r13 = (crate::simd_ops::ru32(rcx.wrapping_add(0x8u64))) as u32 as u64; rbx = (crate::simd_ops::ru64(rcx)) as u64; r8 = ((rbx.wrapping_add(r13.wrapping_mul(8))) as u64) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0xcu64))) as u32 as u64; r12 = rbp; r12 = ((r12).wrapping_sub(rdx)) as u64; r12 = ((((r12) as i64).wrapping_shr(3u32) as u64)) as u64; fl_l = r12; fl_r = rax; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 13; continue; } bb = 1; continue; }
            1 => { rdx = rbx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); r14 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(r15.wrapping_add(0x8u64), ((r14 as u32)) as u32); rax = (crate::simd_ops::ru32(r15.wrapping_add(0xcu64))) as u32 as u64; rsi = ((rax as u32)) as u32 as u64; rsi = ((rsi).wrapping_shr(1u32)) as u64; rsi = ((rsi).wrapping_add(rax)) as u64; fl_l = rsi; fl_r = r12; rsi = (if (((fl_l as u64)) < ((fl_r as u64))) { r12 } else { rsi }) as u64; rdx = rsi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rbx = rax; rax = (crate::simd_ops::ru64(r15)) as u64; rcx = (crate::simd_ops::ru32(r15.wrapping_add(0x8u64))) as u32 as u64; r8 = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; rdx = rbx; fl_l = rax; fl_r = r8; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 4; continue; } bb = 2; continue; }
            2 => { bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(rax)) as u64; crate::simd_ops::wu64(rax, r14); crate::simd_ops::wu64(rdx, rcx); rdx = rdx.wrapping_add(0x8); rax = ((rax).wrapping_add(0x8u64)) as u64; fl_l = rax; fl_r = r8; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 4; continue; }
            4 => { rcx = r15; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(r15, rbx); crate::simd_ops::wu32(r15.wrapping_add(0x8u64), ((r14 as u32)) as u32); crate::simd_ops::wu32(r15.wrapping_add(0xcu64), ((rsi as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rbx); fl_l = rdi; fl_r = rbp; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 12; continue; } bb = 5; continue; }
            5 => { bb = 6; continue; }
            6 => { rdx = (crate::simd_ops::ru64(rdi)) as u64; crate::simd_ops::wu64(rbx, rdx); fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 7; continue; }
            7 => { rax = 0x1; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rdx.wrapping_add(0x24u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; rax = (((rax as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 10; continue; } bb = 8; continue; }
            8 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x28u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 10; continue; }
            10 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; bb = 11; continue; }
            11 => { rbx = ((rbx).wrapping_add(0x8u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rbx); rdi = ((rdi).wrapping_add(0x8u64)) as u64; fl_l = rdi; fl_r = rbp; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 6; continue; } bb = 12; continue; }
            12 => { bb = 47; continue; }
            13 => { fl_l = r12; fl_r = r13; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 35; continue; } bb = 14; continue; }
            14 => { r14 = ((rdx.wrapping_add(r13.wrapping_mul(8))) as u64) as u64; rbp = r13; fl_l = r13; fl_r = r13; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 26; continue; } bb = 15; continue; }
            15 => { bb = 16; continue; }
            16 => { rsi = (crate::simd_ops::ru64(rdi)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x80u64), rsi); fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 20; continue; } bb = 17; continue; }
            17 => { rax = 0x1; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rsi.wrapping_add(0x24u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; rax = (((rax as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 20; continue; } bb = 18; continue; }
            18 => { rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x28u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 20; continue; } bb = 19; continue; }
            19 => { rdx = rsi; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 20; continue; }
            20 => { rdx = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 24; continue; } bb = 21; continue; }
            21 => { rax = 0xffffffff; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rdx.wrapping_add(0x24u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 24; continue; } bb = 22; continue; }
            22 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x28u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 24; continue; } bb = 23; continue; }
            23 => { rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 24; continue; }
            24 => { crate::simd_ops::wu64(rbx, rsi); rbx = ((rbx).wrapping_add(0x8u64)) as u64; fl_l = rbp; fl_r = (0x1u64) as u64; rbp = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 26; continue; } bb = 25; continue; }
            25 => { fl_l = ((rdi).wrapping_add(0x8u64)) as u64; fl_r = fl_l; rdi = ((fl_l as u64)) as u64; bb = 16; continue; }
            26 => { rdi = r12; fl_l = rdi; fl_r = r13; rdi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 47; continue; } bb = 27; continue; }
            27 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rbx); bb = 28; continue; }
            28 => { rdx = (crate::simd_ops::ru64(r14)) as u64; crate::simd_ops::wu64(rbx, rdx); fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 33; continue; } bb = 29; continue; }
            29 => { rax = 0x1; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rdx.wrapping_add(0x24u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; rax = (((rax as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 32; continue; } bb = 30; continue; }
            30 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x28u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 32; continue; } bb = 31; continue; }
            31 => { rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 32; continue; }
            32 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; bb = 33; continue; }
            33 => { rbx = ((rbx).wrapping_add(0x8u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rbx); r14 = ((r14).wrapping_add(0x8u64)) as u64; fl_l = rdi; fl_r = (0x1u64) as u64; rdi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 28; continue; } bb = 34; continue; }
            34 => { bb = 47; continue; }
            35 => { fl_l = rdi; fl_r = rbp; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 46; continue; } bb = 36; continue; }
            36 => { bb = 37; continue; }
            37 => { rsi = (crate::simd_ops::ru64(rdi)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x88u64), rsi); fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 41; continue; } bb = 38; continue; }
            38 => { rax = 0x1; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rsi.wrapping_add(0x24u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; rax = (((rax as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 41; continue; } bb = 39; continue; }
            39 => { rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x28u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 41; continue; } bb = 40; continue; }
            40 => { rdx = rsi; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 41; continue; }
            41 => { rdx = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 45; continue; } bb = 42; continue; }
            42 => { rax = 0xffffffff; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rdx.wrapping_add(0x24u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 45; continue; } bb = 43; continue; }
            43 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x28u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 45; continue; } bb = 44; continue; }
            44 => { rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 45; continue; }
            45 => { crate::simd_ops::wu64(rbx, rsi); rbx = ((rbx).wrapping_add(0x8u64)) as u64; rdi = ((rdi).wrapping_add(0x8u64)) as u64; fl_l = rdi; fl_r = rbp; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 37; continue; } bb = 46; continue; }
            46 => { rcx = (crate::simd_ops::ru64(r15)) as u64; rax = (crate::simd_ops::ru32(r15.wrapping_add(0x8u64))) as u32 as u64; r8 = ((rcx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rdx = ((rcx.wrapping_add(r12.wrapping_mul(8))) as u64) as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); bb = 47; continue; }
            47 => { crate::simd_ops::wu32(r15.wrapping_add(0x8u64), ((r12 as u32)) as u32); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411c9710(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x24; const P2: u64 = 0x28; const P3: u64 = 0x14241d6f0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rbx = (crate::simd_ops::ru64(rcx)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 7; continue; } bb = 1; continue; }
            1 => { rsi = 0; fl_l = 0; fl_r = 0; bb = 2; continue; }
            2 => { rdx = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 3; continue; }
            3 => { rax = 0xffffffff; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rdx.wrapping_add(P1), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 6; continue; } bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P2))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            6 => { crate::simd_ops::wu64(rbx, rsi); rbx = ((rbx).wrapping_add(0x8u64)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 7; continue; }
            7 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411c9780(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x24; const P2: u64 = 0x28; const P3: u64 = 0x14241d6f0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rbx = (crate::simd_ops::ru64(rcx)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 7; continue; } bb = 1; continue; }
            1 => { rsi = 0; fl_l = 0; fl_r = 0; bb = 2; continue; }
            2 => { rdx = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 3; continue; }
            3 => { rax = 0xffffffff; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rdx.wrapping_add(P1), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 6; continue; } bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P2))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            6 => { crate::simd_ops::wu64(rbx, rsi); rbx = ((rbx).wrapping_add(0x18u64)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 7; continue; }
            7 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411c97f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x24; const P3: u64 = 0x28; const P4: u64 = 0x14241d6f0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rbx = (crate::simd_ops::ru64(rcx)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 7; continue; } bb = 1; continue; }
            1 => { rsi = 0; fl_l = 0; fl_r = 0; bb = 2; continue; }
            2 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 3; continue; }
            3 => { rax = 0xffffffff; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rdx.wrapping_add(P2), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 6; continue; } bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P3))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            6 => { crate::simd_ops::wu64(rbx.wrapping_add(P1), rsi); rbx = ((rbx).wrapping_add(0x18u64)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 7; continue; }
            7 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411c9ab0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1434286b0; const P1: u64 = 0x143428770;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdx = (crate::simd_ops::ru64(rcx)) as u64; r8 = rcx; r9 = (crate::simd_ops::ru64(rcx.wrapping_add(0x18u64))) as u64; rcx = rdx; rax = rdx.wrapping_add(0x1); rax = ((rax).wrapping_add(r9)) as u64; fl_l = rdx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 3; continue; } bb = 1; continue; }
            1 => { xmm2 = (crate::simd_ops::ru128(P0)) as u128; xmm3 = (crate::simd_ops::ru128(P1)) as u128; bb = 2; continue; }
            2 => { xmm0 = crate::simd_ops::vpshufb(xmm2, crate::simd_ops::ru128(rcx)); xmm1 = (xmm0) | (xmm3); crate::simd_ops::wu128(rcx, (xmm1) as u128); rcx = ((rcx).wrapping_add(0x10u64)) as u64; fl_l = rcx; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 3; continue; }
            3 => { xmm0 = (crate::simd_ops::ru128(rdx)) as u128; crate::simd_ops::wu128(rax, (xmm0) as u128); rbp = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu8(r9.wrapping_add(rdx), (0xffu8) as u8); r9 = (crate::simd_ops::ru64(r8.wrapping_add(0x18u64))) as u64; r10 = ((rbp as u32)) as u32 as u64; fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 16; continue; } bb = 4; continue; }
            4 => { crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rsi); rsi = rbp.wrapping_sub(0x10); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), r12); r12 = (0xde5fb9d2630458e9u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), r13); r13 = (0xcbf29ce484222325u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), r14); crate::simd_ops::wu64(rsp, r15); r15 = (0x100000001b3u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rbx); bb = 5; continue; }
            5 => { rdi = (crate::simd_ops::ru64(r8)) as u64; fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(r10))) as u8 as u64; fl_r = (0xfeu8) as u8 as u64; r14 = ((rdi.wrapping_add(r10)) as u64) as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 14; continue; } bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(r8.wrapping_add(0x8u64))) as u64; rdx = ((crate::simd_ops::ru8(rax.wrapping_add(r10.wrapping_mul(8)))) as u32) as u32 as u64; rcx = ((rax.wrapping_add(r10.wrapping_mul(8))) as u64) as u64; rax = ((crate::simd_ops::ru8(rax.wrapping_add(r10.wrapping_mul(8)).wrapping_add(0x1u64))) as u32) as u32 as u64; rdx = (((rdx) ^ (r13))) as u64; rdx = (((rdx) as i64).wrapping_mul((r15) as i64) as u64) as u64; xmm1 = crate::simd_ops::vpcmpeqd(xmm1, xmm1); rdx = (((rdx) ^ (rax))) as u64; rax = ((crate::simd_ops::ru8(rcx.wrapping_add(0x2u64))) as u32) as u32 as u64; rdx = (((rdx) as i64).wrapping_mul((r15) as i64) as u64) as u64; rdx = (((rdx) ^ (rax))) as u64; rax = ((crate::simd_ops::ru8(rcx.wrapping_add(0x3u64))) as u32) as u32 as u64; rdx = (((rdx) as i64).wrapping_mul((r15) as i64) as u64) as u64; rdx = (((rdx) ^ (rax))) as u64; rax = ((crate::simd_ops::ru8(rcx.wrapping_add(0x4u64))) as u32) as u32 as u64; rdx = (((rdx) as i64).wrapping_mul((r15) as i64) as u64) as u64; rdx = (((rdx) ^ (rax))) as u64; rax = ((crate::simd_ops::ru8(rcx.wrapping_add(0x5u64))) as u32) as u32 as u64; rdx = (((rdx) as i64).wrapping_mul((r15) as i64) as u64) as u64; rdx = (((rdx) ^ (rax))) as u64; rax = ((crate::simd_ops::ru8(rcx.wrapping_add(0x6u64))) as u32) as u32 as u64; rdx = (((rdx) as i64).wrapping_mul((r15) as i64) as u64) as u64; rdx = (((rdx) ^ (rax))) as u64; rax = ((crate::simd_ops::ru8(rcx.wrapping_add(0x7u64))) as u32) as u32 as u64; rdx = (((rdx) as i64).wrapping_mul((r15) as i64) as u64) as u64; rdx = (((rdx) ^ (rax))) as u64; rax = r12; rdx = (((rdx) as i64).wrapping_mul((r15) as i64) as u64) as u64; let prod = (rax as u128).wrapping_mul((rdx) as u128); rax = prod as u64; rdx = (prod >> 64) as u64; fl_l = (rdx != 0) as u64; r11 = ((rdx.wrapping_add(rax)) as u64) as u64; rdx = rbp; rcx = r11; rcx = ((rcx).wrapping_shr(7u32)) as u64; rcx = (((rcx) & (r9))) as u64; xmm0 = crate::simd_ops::vpcmpgtb(xmm1, crate::simd_ops::ru128(rdi.wrapping_add(rcx))); rax = (crate::simd_ops::vpmovmskb(xmm0) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 9; continue; } bb = 7; continue; }
            7 => { bb = 8; continue; }
            8 => { rdx = ((rdx).wrapping_add(0x10u64)) as u64; rcx = ((rcx).wrapping_add(rdx)) as u64; rcx = (((rcx) & (r9))) as u64; xmm0 = crate::simd_ops::vpcmpgtb(xmm1, crate::simd_ops::ru128(rdi.wrapping_add(rcx))); rax = (crate::simd_ops::vpmovmskb(xmm0) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 8; continue; } bb = 9; continue; }
            9 => { let bit_in: u32 = (rax as u32); if bit_in != 0 { rbx = ((bit_in.trailing_zeros() as u32)) as u32 as u64; } fl_l = bit_in as u64; rdx = r11; rax = r10; rbx = ((rbx).wrapping_add(rcx)) as u64; rdx = ((rdx).wrapping_shr(7u32)) as u64; rdx = (((rdx) & (r9))) as u64; rbx = (((rbx) & (r9))) as u64; rax = ((rax).wrapping_sub(rdx)) as u64; rcx = rbx; rcx = ((rcx).wrapping_sub(rdx)) as u64; r11 = (r11 & !0xffu64) | ((((((r11 as u8)) & (0x7fu8))) as u8 as u64)); rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) & (r9))) as u64; fl_l = rcx; fl_r = (0xfffffffffffffff0u64) as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { crate::simd_ops::wu8(r14, ((r11 as u8)) as u8); rax = (crate::simd_ops::ru64(r8.wrapping_add(0x18u64))) as u64; rcx = rax; rax = ((((rax as u32)) & (0xfu32))) as u32 as u64; rcx = (((rcx) & (rsi))) as u64; fl_l = ((rcx).wrapping_add(rax)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; rax = (crate::simd_ops::ru64(r8)) as u64; crate::simd_ops::wu8(rcx.wrapping_add(rax).wrapping_add(0x1u64), ((r11 as u8)) as u8); bb = 14; continue; }
            11 => { fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(rbx))) as u8 as u64; fl_r = (0x80u8) as u8 as u64; rcx = rbx.wrapping_sub(0x10); crate::simd_ops::wu8(rdi.wrapping_add(rbx), ((r11 as u8)) as u8); rax = (crate::simd_ops::ru64(r8.wrapping_add(0x18u64))) as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 13; continue; } bb = 12; continue; }
            12 => { rcx = (((rcx) & (rax))) as u64; rax = ((((rax as u32)) & (0xfu32))) as u32 as u64; rcx = ((rcx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru64(r8)) as u64; crate::simd_ops::wu8(rcx.wrapping_add(rax).wrapping_add(0x1u64), ((r11 as u8)) as u8); rcx = (crate::simd_ops::ru64(r8.wrapping_add(0x8u64))) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(r10.wrapping_mul(8)))) as u64; crate::simd_ops::wu64(rcx.wrapping_add(rbx.wrapping_mul(8)), rax); rax = (crate::simd_ops::ru64(r8)) as u64; crate::simd_ops::wu8(r10.wrapping_add(rax), (0x80u8) as u8); rax = (crate::simd_ops::ru64(r8.wrapping_add(0x18u64))) as u64; rcx = rax; rax = ((((rax as u32)) & (0xfu32))) as u32 as u64; rcx = (((rcx) & (rsi))) as u64; fl_l = ((rcx).wrapping_add(rax)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; rax = (crate::simd_ops::ru64(r8)) as u64; crate::simd_ops::wu8(rcx.wrapping_add(rax).wrapping_add(0x1u64), (0x80u8) as u8); bb = 14; continue; }
            13 => { rcx = (((rcx) & (rax))) as u64; rax = ((((rax as u32)) & (0xfu32))) as u32 as u64; rcx = ((rcx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru64(r8)) as u64; crate::simd_ops::wu8(rcx.wrapping_add(rax).wrapping_add(0x1u64), ((r11 as u8)) as u8); rcx = (crate::simd_ops::ru64(r8.wrapping_add(0x8u64))) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(r10.wrapping_mul(8)))) as u64; rdx = ((rcx.wrapping_add(r10.wrapping_mul(8))) as u64) as u64; r10 = ((r10).wrapping_sub(1)) as u64; fl_l = ((rsi).wrapping_sub(1)) as u64; rsi = ((fl_l as u64)) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(rbx.wrapping_mul(8)))) as u64; crate::simd_ops::wu64(rdx, rcx); rcx = (crate::simd_ops::ru64(r8.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rcx.wrapping_add(rbx.wrapping_mul(8)), rax); bb = 14; continue; }
            14 => { r9 = (crate::simd_ops::ru64(r8.wrapping_add(0x18u64))) as u64; r10 = ((r10).wrapping_add(1)) as u64; rsi = ((rsi).wrapping_add(1)) as u64; fl_l = r10; fl_r = r9; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 5; continue; } bb = 15; continue; }
            15 => { r15 = (crate::simd_ops::ru64(rsp)) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x8u64))) as u64; r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x10u64))) as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x18u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; bb = 16; continue; }
            16 => { rax = r9; rax = ((rax).wrapping_shr(3u32)) as u64; r9 = ((r9).wrapping_sub(rax)) as u64; r9 = ((r9).wrapping_sub(crate::simd_ops::ru64(r8.wrapping_add(0x10u64)))) as u64; crate::simd_ops::wu64(r8.wrapping_add(0x28u64), r9); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411c9d40(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142537280; const P1: u64 = 0x142535070; const P2: u64 = 0x1426456c8; const P3: u64 = 0x1426453a0; const P4: u64 = 0x14183acc2; const P5: u64 = 0x1425371a0; const P6: u64 = 0x142533200;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rax = ((crate::exe::tls_pointer() as u64)) as u64; rsi = rdx.wrapping_add(0x18); r14 = (crate::simd_ops::ru64(rcx)) as u64; rsi = (((rsi) & (0xfffffffffffffff8u64))) as u64; r15 = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; rbx = rcx; rbp = (crate::simd_ops::ru64(rcx.wrapping_add(0x18u64))) as u64; rdi = rdx; rcx = (crate::simd_ops::ru64(rax)) as u64; r12 = 0; fl_l = 0; fl_r = 0; r10 = 0x6348; rdx = ((rsi.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64; fl_l = ((rdx) & ((rdx).wrapping_sub(1))) as u64; rax = ((fl_l as u64)) as u64; r10 = (crate::simd_ops::ru64(r10.wrapping_add(rcx))) as u64; if (((fl_l as u64)) != 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { rax = rdx.wrapping_sub(0x10); fl_l = rax; fl_r = (0x3f0u64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 4; continue; } bb = 2; continue; }
            2 => { rax = rdx; rax = ((rax).wrapping_shr(3u32)) as u64; r8 = (crate::simd_ops::ru64(r10.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x8u64))) as u64; rcx = (crate::simd_ops::ru64(r8.wrapping_add(0x10u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 9; continue; } bb = 3; continue; }
            3 => { r9 = 0; fl_l = 0; fl_r = 0; r8 = 0; fl_l = 0; fl_r = 0; rcx = r10; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 11; continue; }
            4 => { rax = (0x7fffffffffffffffu64) as u64; fl_l = rdx; fl_r = rax; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rcx = r12; bb = 12; continue; }
            6 => { rax = rdx.wrapping_sub(0x10); fl_l = rax; fl_r = (0x3f0u64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 10; continue; } bb = 7; continue; }
            7 => { rax = rdx; rax = ((rax).wrapping_shr(3u32)) as u64; r8 = (crate::simd_ops::ru64(r10.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x8u64))) as u64; rcx = (crate::simd_ops::ru64(r8.wrapping_add(0x10u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 8; continue; }
            8 => { fl_l = ((rcx as u8)) as u8 as u64; fl_r = (0xfu8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { fl_l = (crate::simd_ops::lk_add_u32(r8.wrapping_add(0x18u64), (1) as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::wu64(r8.wrapping_add(0x10u64), rax); bb = 12; continue; }
            10 => { r8 = 0x10; rcx = r10; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 11; continue; }
            11 => { rcx = rax; bb = 12; continue; }
            12 => { fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 14; continue; } bb = 13; continue; }
            13 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rax; rdx = 0xdeadd00d; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = r12; bb = 14; continue; }
            14 => { rax = ((rcx.wrapping_add(rsi)) as u64) as u64; crate::simd_ops::wu64(rbx, rcx); r8 = rdi.wrapping_add(0x10); crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), rax); rdx = 0xffffff80; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rdi; r11 = r12; crate::simd_ops::wu8(rdi.wrapping_add(rax), (0xffu8) as u8); rax = rdi; rax = ((rax).wrapping_shr(3u32)) as u64; rcx = ((rcx).wrapping_sub(rax)) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x18u64), rdi); rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(rbx.wrapping_add(0x10u64)))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x28u64), rcx); fl_l = rbp; fl_r = rbp; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 30; continue; } bb = 15; continue; }
            15 => { crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), r13); rdi = r15.wrapping_add(0x2); r15 = (0x100000001b3u64) as u64; r13 = (0xde5fb9d2630458e9u64) as u64; rax = (0xcbf29ce484222325u64) as u64; bb = 16; continue; }
            16 => { fl_l = (crate::simd_ops::ru8(r14.wrapping_add(r11))) as u8 as u64; fl_r = ((r12 as u8)) as u8 as u64; if ((((fl_l as u8)) as i8) < (((fl_r as u8)) as i8)) { bb = 21; continue; } bb = 17; continue; }
            17 => { rcx = ((crate::simd_ops::ru8(rdi.wrapping_sub(0x2u64))) as u32) as u32 as u64; rcx = (((rcx) ^ (rax))) as u64; r9 = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; rax = ((crate::simd_ops::ru8(rdi.wrapping_sub(0x1u64))) as u32) as u32 as u64; r10 = (crate::simd_ops::ru64(rbx)) as u64; rcx = (((rcx) as i64).wrapping_mul((r15) as i64) as u64) as u64; xmm1 = crate::simd_ops::vpcmpeqd(xmm1, xmm1); rcx = (((rcx) ^ (rax))) as u64; rax = ((crate::simd_ops::ru8(rdi)) as u32) as u32 as u64; rcx = (((rcx) as i64).wrapping_mul((r15) as i64) as u64) as u64; rcx = (((rcx) ^ (rax))) as u64; rax = ((crate::simd_ops::ru8(rdi.wrapping_add(0x1u64))) as u32) as u32 as u64; rcx = (((rcx) as i64).wrapping_mul((r15) as i64) as u64) as u64; rcx = (((rcx) ^ (rax))) as u64; rax = ((crate::simd_ops::ru8(rdi.wrapping_add(0x2u64))) as u32) as u32 as u64; rcx = (((rcx) as i64).wrapping_mul((r15) as i64) as u64) as u64; rcx = (((rcx) ^ (rax))) as u64; rax = ((crate::simd_ops::ru8(rdi.wrapping_add(0x3u64))) as u32) as u32 as u64; rcx = (((rcx) as i64).wrapping_mul((r15) as i64) as u64) as u64; rcx = (((rcx) ^ (rax))) as u64; rax = ((crate::simd_ops::ru8(rdi.wrapping_add(0x4u64))) as u32) as u32 as u64; rcx = (((rcx) as i64).wrapping_mul((r15) as i64) as u64) as u64; rcx = (((rcx) ^ (rax))) as u64; rax = ((crate::simd_ops::ru8(rdi.wrapping_add(0x5u64))) as u32) as u32 as u64; rcx = (((rcx) as i64).wrapping_mul((r15) as i64) as u64) as u64; rcx = (((rcx) ^ (rax))) as u64; rax = r13; rcx = (((rcx) as i64).wrapping_mul((r15) as i64) as u64) as u64; let prod = (rax as u128).wrapping_mul((rcx) as u128); rax = prod as u64; rdx = (prod >> 64) as u64; fl_l = (rdx != 0) as u64; rsi = ((rdx.wrapping_add(rax)) as u64) as u64; rdx = r12; rcx = rsi; rcx = ((rcx).wrapping_shr(7u32)) as u64; rcx = (((rcx) & (r9))) as u64; xmm0 = crate::simd_ops::vpcmpgtb(xmm1, crate::simd_ops::ru128(r10.wrapping_add(rcx))); rax = (crate::simd_ops::vpmovmskb(xmm0) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 20; continue; } bb = 18; continue; }
            18 => { bb = 19; continue; }
            19 => { rdx = ((rdx).wrapping_add(0x10u64)) as u64; rcx = ((rcx).wrapping_add(rdx)) as u64; rcx = (((rcx) & (r9))) as u64; xmm0 = crate::simd_ops::vpcmpgtb(xmm1, crate::simd_ops::ru128(r10.wrapping_add(rcx))); rax = (crate::simd_ops::vpmovmskb(xmm0) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 19; continue; } bb = 20; continue; }
            20 => { let bit_in: u32 = (rax as u32); if bit_in != 0 { r8 = ((bit_in.trailing_zeros() as u32)) as u32 as u64; } fl_l = bit_in as u64; rsi = (rsi & !0xffu64) | ((((((rsi as u8)) & (0x7fu8))) as u8 as u64)); r8 = ((r8).wrapping_add(rcx)) as u64; r8 = (((r8) & (r9))) as u64; crate::simd_ops::wu8(r10.wrapping_add(r8), ((rsi as u8)) as u8); rcx = r8.wrapping_sub(0x10); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; rcx = (((rcx) & (rax))) as u64; rax = ((((rax as u32)) & (0xfu32))) as u32 as u64; fl_l = ((rcx).wrapping_add(rax)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; rax = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu8(rcx.wrapping_add(rax).wrapping_add(0x1u64), ((rsi as u8)) as u8); rax = (crate::simd_ops::ru64(rdi.wrapping_sub(0x2u64))) as u64; rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rcx.wrapping_add(r8.wrapping_mul(8)), rax); rax = (0xcbf29ce484222325u64) as u64; bb = 21; continue; }
            21 => { r11 = ((r11).wrapping_add(1)) as u64; rdi = ((rdi).wrapping_add(0x8u64)) as u64; fl_l = r11; fl_r = rbp; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 16; continue; } bb = 22; continue; }
            22 => { r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; fl_l = r14; fl_r = r14; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 30; continue; } bb = 23; continue; }
            23 => { rcx = r14.wrapping_sub(0x1); rax = r14; rcx = (((rcx) & (0xfffffffffe000000u64))) as u64; rax = ((rax).wrapping_sub(rcx)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rdx = rcx.wrapping_add(0x108); rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32 as u64; rdx = ((rdx).wrapping_sub(rax)) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(rcx.wrapping_add(0x100u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 28; continue; } bb = 24; continue; }
            24 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0xeu64))) as u8 as u64; fl_r = ((r12 as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 27; continue; } bb = 25; continue; }
            25 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(r14, rax); fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; crate::simd_ops::wu64(rdx.wrapping_add(0x20u64), r14); if (((fl_l as u32)) != 0) { bb = 30; continue; } bb = 26; continue; }
            26 => { rcx = rdx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 30; continue; }
            27 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 29; continue; }
            28 => { r8 = 0; fl_l = 0; fl_r = 0; bb = 29; continue; }
            29 => { r9 = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 30; continue; }
            30 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411ca040(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x1401dec00; const P2: u64 = 0x1410d7d70; const P3: u64 = 0x1410c05d0; const P4: u64 = 0xc;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbp = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; rdi = rdx; rbx = rcx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); r8 = (crate::simd_ops::ru64(rbx)) as u64; rsi = rax; r9 = (crate::simd_ops::ru32(rbx.wrapping_add(P0))) as u32 as u64; rcx = rax; r10 = ((r8.wrapping_add(r9.wrapping_mul(8))) as u64) as u64; fl_l = r8; fl_r = r10; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 3; continue; } bb = 1; continue; }
            1 => { rax = 0; fl_l = 0; fl_r = 0; bb = 2; continue; }
            2 => { rdx = (crate::simd_ops::ru64(r8)) as u64; crate::simd_ops::wu64(r8, rax); r8 = ((r8).wrapping_add(0x8u64)) as u64; crate::simd_ops::wu64(rcx, rdx); rcx = ((rcx).wrapping_add(0x8u64)) as u64; fl_l = r8; fl_r = r10; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 3; continue; }
            3 => { rdx = rcx; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx, rsi); rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; crate::simd_ops::wu32(rbx.wrapping_add(P0), ((rbp as u32)) as u32); rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; crate::simd_ops::wu32(rbx.wrapping_add(P4), ((rdi as u32)) as u32); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411ca0d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x14023b7a0; const P2: u64 = 0x20; const P3: u64 = 0xffffffffffffffe0; const P4: u64 = 0xfffffffffffffff0; const P5: u64 = 0x10; const P6: u64 = 0x40; const P7: u64 = 0x1401de830; const P8: u64 = 0xc;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbp = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; rsi = rdx; rbx = rcx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = (crate::simd_ops::ru64(rbx)) as u64; rdi = rax; r9 = (crate::simd_ops::ru32(rbx.wrapping_add(P0))) as u32 as u64; r9 = ((r9).wrapping_shl(6u32)) as u64; r9 = ((r9).wrapping_add(r8)) as u64; fl_l = r8; fl_r = r9; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 3; continue; } bb = 1; continue; }
            1 => { rcx = ((rax.wrapping_add(P2)) as u64) as u64; rdx = ((r8.wrapping_add(P2)) as u64) as u64; bb = 2; continue; }
            2 => { xmm0 = (crate::simd_ops::ru128(r8)) as u128; crate::simd_ops::wu128(rcx.wrapping_add(P3), (xmm0) as u128); xmm0 = (crate::simd_ops::ru128(rdx.wrapping_add(P4))) as u128; crate::simd_ops::wu128(rcx.wrapping_add(P4), (xmm0) as u128); xmm0 = (crate::simd_ops::ru128(rdx)) as u128; crate::simd_ops::wu128(rcx, (xmm0) as u128); xmm0 = (crate::simd_ops::ru128(rdx.wrapping_add(P5))) as u128; r8 = ((r8).wrapping_add(0x40u64)) as u64; rdx = ((rdx.wrapping_add(P6)) as u64) as u64; crate::simd_ops::wu128(rcx.wrapping_add(P5), (xmm0) as u128); rcx = ((rcx.wrapping_add(P6)) as u64) as u64; fl_l = r8; fl_r = r9; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 3; continue; }
            3 => { rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(rbx.wrapping_add(P0), ((rbp as u32)) as u32); rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; crate::simd_ops::wu32(rbx.wrapping_add(P8), ((rsi as u32)) as u32); rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; crate::simd_ops::wu64(rbx, rdi); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411ca170(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x1401dec00; const P2: u64 = 0xc; const P3: u64 = 0x24; const P4: u64 = 0x28; const P5: u64 = 0x14241d6f0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; r14 = rcx; rdi = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rcx, rdi); crate::simd_ops::wu64(rcx.wrapping_add(P0), rdi); rbp = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64; rbx = (crate::simd_ops::ru64(rdx)) as u64; rsi = ((rbx.wrapping_add(rbp.wrapping_mul(8))) as u64) as u64; fl_l = rbp; fl_r = rbp; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rdx = ((rbp as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(r14, rax); crate::simd_ops::wu32(r14.wrapping_add(P0), ((rdi as u32)) as u32); crate::simd_ops::wu32(r14.wrapping_add(P2), ((rbp as u32)) as u32); rdi = rax; bb = 2; continue; }
            2 => { fl_l = rbx; fl_r = rsi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 9; continue; } bb = 3; continue; }
            3 => { bb = 4; continue; }
            4 => { rdx = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu64(rdi, rdx); fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 5; continue; }
            5 => { rax = 0x1; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rdx.wrapping_add(P3), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; rax = (((rax as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 8; continue; } bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P4))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); bb = 8; continue; }
            8 => { rdi = ((rdi).wrapping_add(0x8u64)) as u64; rbx = ((rbx).wrapping_add(0x8u64)) as u64; fl_l = rbx; fl_r = rsi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } bb = 9; continue; }
            9 => { crate::simd_ops::wu32(r14.wrapping_add(P0), ((rbp as u32)) as u32); rax = r14; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411ca220(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x4;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = (crate::simd_ops::ru64(rdx)) as u64;
    crate::simd_ops::wu64(rcx, rax);
    rdx = (if (rax as u32 as i32) < 0 { 0xffffffffu32 } else { 0u32 }) as u64;
    let d = ((r8 as u32)) as i32 as i64; let n = (((rdx as u32 as u64) << 32) | (rax as u32 as u64)) as i64;
    if d != 0 { rax = n.wrapping_div(d) as i32 as u32 as u64; rdx = n.wrapping_rem(d) as i32 as u32 as u64; }
    crate::simd_ops::wu32(rcx, ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64;
    rdx = (if (rax as u32 as i32) < 0 { 0xffffffffu32 } else { 0u32 }) as u64;
    let d = ((r8 as u32)) as i32 as i64; let n = (((rdx as u32 as u64) << 32) | (rax as u32 as u64)) as i64;
    if d != 0 { rax = n.wrapping_div(d) as i32 as u32 as u64; rdx = n.wrapping_rem(d) as i32 as u32 as u64; }
    crate::simd_ops::wu32(rcx.wrapping_add(P0), ((rax as u32)) as u32);
    rax = rcx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411ca240(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1420472c0; const P1: u64 = 0x10; const P2: u64 = 0x40; const P3: u64 = 0x8; const P4: u64 = 0x1; const P5: u64 = 0x1410c38f0; const P6: u64 = 0x2a0; const P7: u64 = 0x140200070;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = rcx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rbx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rbx = ((rbx).wrapping_shl(4u32)) as u64; fl_l = ((rbx).wrapping_add(crate::simd_ops::ru64(rsi.wrapping_add(P1)))) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; if (((fl_l as u64)) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(P2))) as u8 as u64; fl_r = (0x13u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 12; continue; } bb = 2; continue; }
            2 => { fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 3; continue; }
            3 => { rax = (((crate::simd_ops::ru8(rbx.wrapping_add(P2))) as i8 as i64) as u64) as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x13u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rbx, rax); crate::simd_ops::wu64(rbx.wrapping_add(P3), rax); bb = 6; continue; }
            5 => { rcx = ((rax.wrapping_add(P4)) as u64) as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x30u64), (0x0u8) as u8); r8 = rbx; rdx = rsp.wrapping_add(0x30); rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rbx, rax); crate::simd_ops::wu64(rbx.wrapping_add(P3), rax); crate::simd_ops::wu8(rbx.wrapping_add(P2), (0x13u8) as u8); bb = 6; continue; }
            6 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(P2))) as u8 as u64; fl_r = (0x13u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 14; continue; } bb = 7; continue; }
            7 => { bb = 12; continue; }
            8 => { rbx = ((rsi.wrapping_add(P6)) as u64) as u64; rax = (((crate::simd_ops::ru8(rbx.wrapping_add(P2))) as i8 as i64) as u64) as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x13u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 10; continue; } bb = 9; continue; }
            9 => { rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rbx, rax); crate::simd_ops::wu64(rbx.wrapping_add(P3), rax); bb = 11; continue; }
            10 => { rcx = ((rax.wrapping_add(P4)) as u64) as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x48u64), (0x0u8) as u8); r8 = rbx; rdx = rsp.wrapping_add(0x48); rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rbx, rax); crate::simd_ops::wu64(rbx.wrapping_add(P3), rax); crate::simd_ops::wu8(rbx.wrapping_add(P2), (0x13u8) as u8); bb = 11; continue; }
            11 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(P2))) as u8 as u64; fl_r = (0x13u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 13; continue; } bb = 12; continue; }
            12 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            13 => { rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x1411ca30f, rcx, rsp); return rax; }
            14 => { rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x1411ca315, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411ca320(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1420472c0; const P1: u64 = 0x10; const P2: u64 = 0x40; const P3: u64 = 0x8; const P4: u64 = 0x1; const P5: u64 = 0x1410c38f0; const P6: u64 = 0x2a0; const P7: u64 = 0x140200070;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = rcx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rbx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rbx = ((rbx).wrapping_shl(4u32)) as u64; fl_l = ((rbx).wrapping_add(crate::simd_ops::ru64(rsi.wrapping_add(P1)))) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; if (((fl_l as u64)) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(P2))) as u8 as u64; fl_r = (0xdu8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 12; continue; } bb = 2; continue; }
            2 => { fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 3; continue; }
            3 => { rax = (((crate::simd_ops::ru8(rbx.wrapping_add(P2))) as i8 as i64) as u64) as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0xdu8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rbx, rax); crate::simd_ops::wu64(rbx.wrapping_add(P3), rax); bb = 6; continue; }
            5 => { rcx = ((rax.wrapping_add(P4)) as u64) as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x30u64), (0x0u8) as u8); r8 = rbx; rdx = rsp.wrapping_add(0x30); rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rbx, rax); crate::simd_ops::wu64(rbx.wrapping_add(P3), rax); crate::simd_ops::wu8(rbx.wrapping_add(P2), (0xdu8) as u8); bb = 6; continue; }
            6 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(P2))) as u8 as u64; fl_r = (0xdu8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 14; continue; } bb = 7; continue; }
            7 => { bb = 12; continue; }
            8 => { rbx = ((rsi.wrapping_add(P6)) as u64) as u64; rax = (((crate::simd_ops::ru8(rbx.wrapping_add(P2))) as i8 as i64) as u64) as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0xdu8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 10; continue; } bb = 9; continue; }
            9 => { rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rbx, rax); crate::simd_ops::wu64(rbx.wrapping_add(P3), rax); bb = 11; continue; }
            10 => { rcx = ((rax.wrapping_add(P4)) as u64) as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x48u64), (0x0u8) as u8); r8 = rbx; rdx = rsp.wrapping_add(0x48); rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rbx, rax); crate::simd_ops::wu64(rbx.wrapping_add(P3), rax); crate::simd_ops::wu8(rbx.wrapping_add(P2), (0xdu8) as u8); bb = 11; continue; }
            11 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(P2))) as u8 as u64; fl_r = (0xdu8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 13; continue; } bb = 12; continue; }
            12 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            13 => { rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x1411ca3ef, rcx, rsp); return rax; }
            14 => { rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x1411ca3f5, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411ca400(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1420472c0; const P1: u64 = 0x10; const P2: u64 = 0x40; const P3: u64 = 0x2a0; const P4: u64 = 0x8; const P5: u64 = 0x1; const P6: u64 = 0x1410c38f0; const P7: u64 = 0x140200070;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = rcx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rbx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rbx = ((rbx).wrapping_shl(4u32)) as u64; fl_l = ((rbx).wrapping_add(crate::simd_ops::ru64(rdi.wrapping_add(P1)))) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; if (((fl_l as u64)) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(P2))) as u8 as u64; fl_r = (0x13u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 6; continue; } bb = 2; continue; }
            2 => { rbx = ((rdi.wrapping_add(P3)) as u64) as u64; rax = (((crate::simd_ops::ru8(rbx.wrapping_add(P2))) as i8 as i64) as u64) as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x13u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rbx, rax); crate::simd_ops::wu64(rbx.wrapping_add(P4), rax); bb = 5; continue; }
            4 => { rcx = ((rax.wrapping_add(P5)) as u64) as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x30u64), (0x0u8) as u8); r8 = rbx; rdx = rsp.wrapping_add(0x30); rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rbx, rax); crate::simd_ops::wu64(rbx.wrapping_add(P4), rax); crate::simd_ops::wu8(rbx.wrapping_add(P2), (0x13u8) as u8); bb = 5; continue; }
            5 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(P2))) as u8 as u64; fl_r = (0x13u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            7 => { rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x1411ca47d, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411ca480(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1420472c0; const P1: u64 = 0x10; const P2: u64 = 0x40; const P3: u64 = 0x1; const P4: u64 = 0x1410c38f0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rcx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64; rdx = ((rdx).wrapping_shl(4u32)) as u64; fl_l = ((rax).wrapping_add(rdx)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; if (((fl_l as u64)) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru8(rax.wrapping_add(P2))) as u8 as u64; fl_r = (0x9u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 5; continue; } bb = 2; continue; }
            2 => { rbx = ((rbx).wrapping_add(0x2a0u64)) as u64; xmm0 = 0u128; crate::simd_ops::wu32(rsp.wrapping_add(0x48u64), ((xmm0 as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x4cu64), ((xmm0 as u32)) as u32); rax = (((crate::simd_ops::ru8(rbx.wrapping_add(P2))) as i8 as i64) as u64) as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x9u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; crate::simd_ops::wu64(rbx, rax); rax = rbx; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            4 => { rcx = ((rax.wrapping_add(P3)) as u64) as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x30u64), (0x0u8) as u8); r8 = rbx; rdx = rsp.wrapping_add(0x30); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rax = rbx; crate::simd_ops::wu64(rbx, rcx); crate::simd_ops::wu8(rbx.wrapping_add(P2), (0x9u8) as u8); bb = 5; continue; }
            5 => { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411ca510(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1420472c0; const P1: u64 = 0x10; const P2: u64 = 0x40; const P3: u64 = 0x2a0; const P4: u64 = 0x8; const P5: u64 = 0x1; const P6: u64 = 0x1410c38f0; const P7: u64 = 0x140200070;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = rcx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rbx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rbx = ((rbx).wrapping_shl(4u32)) as u64; fl_l = ((rbx).wrapping_add(crate::simd_ops::ru64(rdi.wrapping_add(P1)))) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; if (((fl_l as u64)) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(P2))) as u8 as u64; fl_r = (0xbu8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 6; continue; } bb = 2; continue; }
            2 => { rbx = ((rdi.wrapping_add(P3)) as u64) as u64; rax = (((crate::simd_ops::ru8(rbx.wrapping_add(P2))) as i8 as i64) as u64) as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0xbu8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rbx, rax); crate::simd_ops::wu32(rbx.wrapping_add(P4), ((rax as u32)) as u32); bb = 5; continue; }
            4 => { rcx = ((rax.wrapping_add(P5)) as u64) as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x30u64), (0x0u8) as u8); r8 = rbx; rdx = rsp.wrapping_add(0x30); rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rbx, rax); crate::simd_ops::wu32(rbx.wrapping_add(P4), ((rax as u32)) as u32); crate::simd_ops::wu8(rbx.wrapping_add(P2), (0xbu8) as u8); bb = 5; continue; }
            5 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(P2))) as u8 as u64; fl_r = (0xbu8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            7 => { rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x1411ca58b, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411ca590(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1420472c0; const P1: u64 = 0x10; const P2: u64 = 0x40; const P3: u64 = 0x143427730; const P4: u64 = 0x1434268d0; const P5: u64 = 0x2a0; const P6: u64 = 0x1411ca9f0; const P7: u64 = 0x140200070;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 15>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x70u64)) as u64; rdi = rcx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rbx = ((rbx).wrapping_shl(4u32)) as u64; fl_l = ((rbx).wrapping_add(crate::simd_ops::ru64(rdi.wrapping_add(P1)))) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; if (((fl_l as u64)) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(P2))) as u8 as u64; fl_r = (0x15u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 3; continue; } bb = 2; continue; }
            2 => { xmm1 = (crate::simd_ops::ru128(P3)) as u128; xmm3 = ((crate::simd_ops::ru32(P4)) as u32) as u128; xmm2 = 0u128; crate::simd_ops::wu128(rsp.wrapping_add(0x50u64), (xmm1) as u128); xmm0 = ((crate::simd_ops::ru64(rsp.wrapping_add(0x5cu64))) as u64) as u128; xmm0 = crate::simd_ops::vinsertps(xmm0, xmm2, 0x10u8); rbx = ((rdi.wrapping_add(P5)) as u64) as u64; rdx = rsp.wrapping_add(0x20); rcx = rbx; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), ((xmm0 as u64)) as u64); crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((xmm3 as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x24u64), ((xmm2 as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((xmm2 as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x40u64), ((xmm3 as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x2cu64), ((xmm1 as u64)) as u64); crate::simd_ops::wu32(rsp.wrapping_add(0x34u64), (((xmm1 >> 64) as u32) as u32) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(P2))) as u8 as u64; fl_r = (0x15u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; rsp = ((rsp).wrapping_add(0x70u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            4 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, 0x1411ca638, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411ca640(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1420472c0; const P1: u64 = 0x10; const P2: u64 = 0x40; const P3: u64 = 0x143427660; const P4: u64 = 0x1434268d0; const P5: u64 = 0x2a0; const P6: u64 = 0x1410cd970; const P7: u64 = 0x140200070;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 17>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x80u64)) as u64; rdi = rcx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rbx = ((rbx).wrapping_shl(4u32)) as u64; fl_l = ((rbx).wrapping_add(crate::simd_ops::ru64(rdi.wrapping_add(P1)))) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; if (((fl_l as u64)) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(P2))) as u8 as u64; fl_r = (0x17u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 3; continue; } bb = 2; continue; }
            2 => { xmm2 = (crate::simd_ops::ru128(P3)) as u128; xmm0 = ((crate::simd_ops::ru32(P4)) as u32) as u128; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((xmm0 as u32)) as u32); xmm1 = 0u128; crate::simd_ops::wu128(rsp.wrapping_add(0x60u64), (xmm2) as u128); xmm0 = ((crate::simd_ops::ru64(rsp.wrapping_add(0x5cu64))) as u64) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | (xmm1 & 0xffff_ffffu128); crate::simd_ops::wu64(rsp.wrapping_add(0x2cu64), ((xmm0 as u64)) as u64); crate::simd_ops::wu128(rsp.wrapping_add(0x70u64), (xmm2) as u128); xmm0 = ((crate::simd_ops::ru64(rsp.wrapping_add(0x74u64))) as u64) as u128; xmm3 = (xmm2) as u128; rbx = ((rdi.wrapping_add(P5)) as u64) as u64; rdx = rsp.wrapping_add(0x20); rcx = rbx; crate::simd_ops::wu64(rsp.wrapping_add(0x44u64), ((xmm0 as u64)) as u64); crate::simd_ops::wu32(rsp.wrapping_add(0x24u64), ((xmm1 as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((xmm1 as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x34u64), (((xmm2 >> 32) as u32) as u32) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (((xmm2 >> 64) as u64)) as u64); crate::simd_ops::wu32(rsp.wrapping_add(0x40u64), ((xmm3 as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x4cu64), (((xmm2 >> 96) as u32) as u32) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(P2))) as u8 as u64; fl_r = (0x17u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; rsp = ((rsp).wrapping_add(0x80u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            4 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, 0x1411ca70e, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411ca710(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1420472c0; const P1: u64 = 0x10; const P2: u64 = 0x40; const P3: u64 = 0x143427730; const P4: u64 = 0x1434268d0; const P5: u64 = 0x2a0; const P6: u64 = 0x1411ca9f0; const P7: u64 = 0x140200070;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 15>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x70u64)) as u64; rsi = rcx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rbx = ((rbx).wrapping_shl(4u32)) as u64; fl_l = ((rbx).wrapping_add(crate::simd_ops::ru64(rsi.wrapping_add(P1)))) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; if (((fl_l as u64)) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(P2))) as u8 as u64; fl_r = (0x15u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 5; continue; } bb = 2; continue; }
            2 => { rdx = rsp.wrapping_add(0x20); xmm1 = (crate::simd_ops::ru128(P3)) as u128; xmm3 = ((crate::simd_ops::ru32(P4)) as u32) as u128; xmm2 = 0u128; crate::simd_ops::wu128(rsp.wrapping_add(0x50u64), (xmm1) as u128); xmm0 = ((crate::simd_ops::ru64(rsp.wrapping_add(0x5cu64))) as u64) as u128; xmm0 = crate::simd_ops::vinsertps(xmm0, xmm2, 0x10u8); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), ((xmm0 as u64)) as u64); crate::simd_ops::wu32(rsp.wrapping_add(0x24u64), ((xmm2 as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((xmm2 as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((xmm3 as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x40u64), ((xmm3 as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x2cu64), ((xmm1 as u64)) as u64); crate::simd_ops::wu32(rsp.wrapping_add(0x34u64), (((xmm1 >> 64) as u32) as u32) as u32); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rbx = ((rsi.wrapping_add(P5)) as u64) as u64; bb = 4; continue; }
            4 => { rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(P2))) as u8 as u64; fl_r = (0x15u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 6; continue; } bb = 5; continue; }
            5 => { r11 = rsp.wrapping_add(0x70); rax = rbx; rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x10u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x18u64))) as u64; rsp = r11; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            6 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, 0x1411ca7c6, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411ca7d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1420472c0; const P1: u64 = 0x10; const P2: u64 = 0x40; const P3: u64 = 0x143428de0; const P4: u64 = 0x1434281d0; const P5: u64 = 0x143427660; const P6: u64 = 0x2a0; const P7: u64 = 0x1411c2960; const P8: u64 = 0x140200070;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut ymm3_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x60u64)) as u64; rdi = rcx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rbx = ((rbx).wrapping_shl(4u32)) as u64; fl_l = ((rbx).wrapping_add(crate::simd_ops::ru64(rdi.wrapping_add(P1)))) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; if (((fl_l as u64)) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(P2))) as u8 as u64; fl_r = (0x19u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 3; continue; } bb = 2; continue; }
            2 => { let y_addr: u64 = P3; xmm3 = crate::simd_ops::ru128(y_addr); ymm3_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); xmm1 = (crate::simd_ops::ru128(P4)) as u128; xmm0 = (crate::simd_ops::ru128(P5)) as u128; rbx = ((rdi.wrapping_add(P6)) as u64) as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); let y_addr: u64 = rsp.wrapping_add(0x30u64); crate::simd_ops::wu128(y_addr, xmm3); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm3_hi); crate::simd_ops::wu128(rsp.wrapping_add(0x50u64), (xmm1) as u128); rcx = rbx; rdx = rsp.wrapping_add(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(P2))) as u8 as u64; fl_r = (0x19u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rsp = ((rsp).wrapping_add(0x60u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            4 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, 0x1411ca850, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411ca860(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1411caaa0; const P1: u64 = 0x1402d7910;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1411ca860, funcinfo_rva: 0x367defc, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x38, conts: &[] };
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbp) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rsi) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r12) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r15) as u64); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; r12 = (rdx) as u64; r14 = (rcx) as u64; r15 = ((r8.wrapping_add(r8.wrapping_mul(4))) as u64) as u64; r15 = ((r15).wrapping_shl(4u32)) as u64; fl_l = ((r15).wrapping_add(crate::simd_ops::ru64(rdx.wrapping_add(0x10u64)))) as u64 as u64; fl_r = fl_l; r15 = ((fl_l as u64)) as u64; if (((fl_l as u64)) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru8(r15.wrapping_add(0x40u64))) as u8 as u64; fl_r = (0x3u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 3; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rcx, (0x1u64) as u64); crate::simd_ops::wu64(rcx.wrapping_add(0x8u64), (r15) as u64); bb = 24; continue; }
            3 => { fl_l = (r15) as u64 as u64; fl_r = (r15) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 23; continue; } bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::ru8(r15.wrapping_add(0x40u64))) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rcx = (crate::simd_ops::ru64(r15)) as u64; rax = (crate::simd_ops::ru32(r15.wrapping_add(0x8u64))) as u32 as u64; crate::simd_ops::wu64(r14, (rax) as u64); crate::simd_ops::wu64(r14.wrapping_add(0x8u64), (rcx) as u64); bb = 24; continue; }
            6 => { fl_l = (crate::simd_ops::ru8(r15.wrapping_add(0x40u64))) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 23; continue; } bb = 7; continue; }
            7 => { rdx = (crate::simd_ops::ru64(r15)) as u64; rcx = (r12) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1411ca8dd); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rsi = (rax) as u64; rbp = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rax.wrapping_add(0x8u64), ((rbp as u32)) as u32); fl_l = (crate::simd_ops::ru32(r15.wrapping_add(0xcu64))) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 22; continue; } bb = 8; continue; }
            8 => { bb = 9; continue; }
            9 => { rdx = (crate::simd_ops::ru64(r12)) as u64; rcx = (crate::simd_ops::ru32(r15.wrapping_add(0x8u64))) as u32 as u64; rax = ((rbp as u32)) as u32 as u64; rcx = ((rcx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x40u64))) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(rcx.wrapping_mul(8)))) as u64; rdi = ((rcx.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64; rdi = ((rdi).wrapping_shl(4u32)) as u64; fl_l = ((rdi).wrapping_add(crate::simd_ops::ru64(r12.wrapping_add(0x10u64)))) as u64 as u64; fl_r = fl_l; rdi = ((fl_l as u64)) as u64; if (((fl_l as u64)) == 0) { bb = 14; continue; } bb = 10; continue; }
            10 => { fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(0x40u64))) as u8 as u64; fl_r = (0x3u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 14; continue; } bb = 11; continue; }
            11 => { rdx = (crate::simd_ops::ru32(rsi.wrapping_add(0x8u64))) as u32 as u64; rax = (crate::simd_ops::ru32(rsi.wrapping_add(0xcu64))) as u32 as u64; fl_l = (rdx) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 13; continue; } bb = 12; continue; }
            12 => { rdx = ((rdx).wrapping_add(1)) as u64; rcx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1411ca93b); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 13; continue; }
            13 => { rcx = (crate::simd_ops::ru32(rsi.wrapping_add(0x8u64))) as u32 as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rsi))) as u64; rax = ((crate::simd_ops::ru8(rdi)) as u32) as u32 as u64; crate::simd_ops::wu8(rcx, ((rax as u8)) as u8); fl_l = (crate::simd_ops::lk_add_u32(rsi.wrapping_add(0x8u64), (1) as u32)) as u32 as u64; bb = 21; continue; }
            14 => { fl_l = (rdi) as u64 as u64; fl_r = (rdi) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 21; continue; } bb = 15; continue; }
            15 => { fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(0x40u64))) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 21; continue; } bb = 16; continue; }
            16 => { rbx = (crate::simd_ops::ru64(rdi)) as u64; rdi = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; rdi = ((rdi).wrapping_add(rbx)) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rdi) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 21; continue; } bb = 17; continue; }
            17 => { bb = 18; continue; }
            18 => { rdx = (crate::simd_ops::ru32(rsi.wrapping_add(0x8u64))) as u32 as u64; rax = (crate::simd_ops::ru32(rsi.wrapping_add(0xcu64))) as u32 as u64; fl_l = (rdx) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 20; continue; } bb = 19; continue; }
            19 => { rdx = ((rdx).wrapping_add(1)) as u64; rcx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1411ca981); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 20; continue; }
            20 => { rcx = (crate::simd_ops::ru32(rsi.wrapping_add(0x8u64))) as u32 as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rsi))) as u64; rax = ((crate::simd_ops::ru8(rbx)) as u32) as u32 as u64; crate::simd_ops::wu8(rcx, ((rax as u8)) as u8); crate::simd_ops::lk_add_u32(rsi.wrapping_add(0x8u64), (1) as u32); rbx = ((rbx).wrapping_add(1)) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rdi) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 18; continue; } bb = 21; continue; }
            21 => { rbp = (((rbp as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((rbp as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(r15.wrapping_add(0xcu64))) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 9; continue; } bb = 22; continue; }
            22 => { rcx = (crate::simd_ops::ru64(rsi)) as u64; rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x8u64))) as u32 as u64; crate::simd_ops::wu64(r14, (rax) as u64); crate::simd_ops::wu64(r14.wrapping_add(0x8u64), (rcx) as u64); bb = 24; continue; }
            23 => { rbp = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rcx, (rbp) as u64); crate::simd_ops::wu64(rcx.wrapping_add(0x8u64), (rbp) as u64); bb = 24; continue; }
            24 => { rax = (r14) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r12 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411ca9f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x40; const P1: u64 = 0x8; const P2: u64 = 0xc; const P3: u64 = 0x14; const P4: u64 = 0x18; const P5: u64 = 0x20; const P6: u64 = 0x1; const P7: u64 = 0x1410c38f0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (((crate::simd_ops::ru8(rcx.wrapping_add(P0))) as i8 as i64) as u64) as u64; rdi = rdx; rbx = rcx; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x15u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 2; continue; } bb = 1; continue; }
            1 => { xmm0 = ((crate::simd_ops::ru64(rdx)) as u64) as u128; crate::simd_ops::wu64(rcx, ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(P1), ((rax as u32)) as u32); xmm0 = ((crate::simd_ops::ru64(rdx.wrapping_add(P2))) as u64) as u128; crate::simd_ops::wu64(rcx.wrapping_add(P2), ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(rdx.wrapping_add(P3))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(P3), ((rax as u32)) as u32); xmm0 = ((crate::simd_ops::ru64(rdx.wrapping_add(P4))) as u64) as u128; crate::simd_ops::wu64(rcx.wrapping_add(P4), ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(rdx.wrapping_add(P5))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(P5), ((rax as u32)) as u32); rax = rcx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            2 => { rcx = ((rax.wrapping_add(P6)) as u64) as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x30u64), (0x0u8) as u8); r8 = rbx; rdx = rsp.wrapping_add(0x30); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu8(rbx.wrapping_add(P0), (0xffu8) as u8); xmm0 = ((crate::simd_ops::ru64(rdi)) as u64) as u128; crate::simd_ops::wu64(rbx, ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(rdi.wrapping_add(P1))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(P1), ((rax as u32)) as u32); xmm0 = ((crate::simd_ops::ru64(rdi.wrapping_add(P2))) as u64) as u128; crate::simd_ops::wu64(rbx.wrapping_add(P2), ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(rdi.wrapping_add(P3))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(P3), ((rax as u32)) as u32); xmm0 = ((crate::simd_ops::ru64(rdi.wrapping_add(P4))) as u64) as u128; crate::simd_ops::wu64(rbx.wrapping_add(P4), ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(rdi.wrapping_add(P5))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(P5), ((rax as u32)) as u32); rax = rbx; crate::simd_ops::wu8(rbx.wrapping_add(P0), (0x15u8) as u8); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411caaa0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1411cac30; const P1: u64 = 0x1425371a0; const P2: u64 = 0x142533200; const P3: u64 = 0x140200070; const P4: u64 = 0x1411cac2d;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1411caaa0, funcinfo_rva: 0x366ef00, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x38, conts: &[] };
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rdi = (rcx) as u64; rbx = ((rdx.wrapping_add(rdx.wrapping_mul(4))) as u64) as u64; rbx = ((rbx).wrapping_shl(4u32)) as u64; fl_l = ((rbx).wrapping_add(crate::simd_ops::ru64(rcx.wrapping_add(0x10u64)))) as u64 as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; if (((fl_l as u64)) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0x40u64))) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 12; continue; } bb = 2; continue; }
            2 => { rax = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rax) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (rax) as u64); fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; rdx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 3; continue; }
            3 => { rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1411caae2); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); r10 = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; fl_l = (r10) as u64 as u64; fl_r = (r10) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 4; continue; }
            4 => { r10 = ((r10).wrapping_sub(1)) as u64; r10 = (((r10) & (0xfffffffffe000000u64))) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = (rax) as u64 as u64; fl_r = (crate::simd_ops::ru64(r10.wrapping_add(0x100u64))) as u64 as u64; rcx = (rcx & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rax = (r9) as u64; rax = ((rax).wrapping_sub(r10)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rdx = ((r10.wrapping_add(0x108u64)) as u64) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32 as u64; rdx = ((rdx).wrapping_sub(rax)) as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 9; continue; } bb = 5; continue; }
            5 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0xeu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 8; continue; } bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(r9, (rax) as u64); crate::simd_ops::wu64(rdx.wrapping_add(0x20u64), (r9) as u64); fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 11; continue; } bb = 7; continue; }
            7 => { rcx = (rdx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1411cab54); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 11; continue; }
            8 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 10; continue; }
            9 => { r8 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 10; continue; }
            10 => { rcx = (r10) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1411cab66); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 11; continue; }
            11 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0x40u64))) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 22; continue; } bb = 12; continue; }
            12 => { rax = (rbx) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            13 => { rbx = ((rcx.wrapping_add(0x2a0u64)) as u64) as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1411cab8e); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); r10 = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; fl_l = (r10) as u64 as u64; fl_r = (r10) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 21; continue; } bb = 14; continue; }
            14 => { r10 = ((r10).wrapping_sub(1)) as u64; r10 = (((r10) & (0xfffffffffe000000u64))) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = (rax) as u64 as u64; fl_r = (crate::simd_ops::ru64(r10.wrapping_add(0x100u64))) as u64 as u64; rcx = (rcx & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rax = (r9) as u64; rax = ((rax).wrapping_sub(r10)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rdx = ((r10.wrapping_add(0x108u64)) as u64) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32 as u64; rdx = ((rdx).wrapping_sub(rax)) as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 19; continue; } bb = 15; continue; }
            15 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0xeu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 18; continue; } bb = 16; continue; }
            16 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(r9, (rax) as u64); crate::simd_ops::wu64(rdx.wrapping_add(0x20u64), (r9) as u64); fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 21; continue; } bb = 17; continue; }
            17 => { rcx = (rdx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1411cac04); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rbx = ((rdi.wrapping_add(0x2a0u64)) as u64) as u64; bb = 21; continue; }
            18 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 20; continue; }
            19 => { r8 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 20; continue; }
            20 => { rcx = (r10) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1411cac1d); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 21; continue; }
            21 => { bb = 11; continue; }
            22 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1411cac28); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::lift_ops::trap(0x80000003, P4, rcx, rsp); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411cac30(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x40; const P1: u64 = 0x1401de830; const P2: u64 = 0x8; const P3: u64 = 0xc; const P4: u64 = 0x1410c38f0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = rdx; rbx = rcx; rcx = (((crate::simd_ops::ru8(rcx.wrapping_add(P0))) as i8 as i64) as u64) as u64; fl_l = rcx; fl_r = (0x4u64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } bb = 1; continue; }
            1 => { fl_l = rbx; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rcx = rbx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rdi)) as u64; crate::simd_ops::wu64(rbx, rax); rax = (crate::simd_ops::ru32(rdi.wrapping_add(P2))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(P2), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdi.wrapping_add(P3))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(P3), ((rax as u32)) as u32); rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rdi, rcx); crate::simd_ops::wu64(rdi.wrapping_add(P2), rcx); bb = 3; continue; }
            3 => { rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            4 => { crate::simd_ops::wu8(rsp.wrapping_add(0x30u64), (0x0u8) as u8); rcx = ((rcx).wrapping_add(1)) as u64; r8 = rbx; rdx = rsp.wrapping_add(0x30); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu8(rbx.wrapping_add(P0), (0xffu8) as u8); rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rbx, rcx); crate::simd_ops::wu64(rbx.wrapping_add(P2), rcx); rax = (crate::simd_ops::ru64(rdi)) as u64; crate::simd_ops::wu64(rbx, rax); rax = (crate::simd_ops::ru32(rdi.wrapping_add(P2))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(P2), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdi.wrapping_add(P3))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(P3), ((rax as u32)) as u32); crate::simd_ops::wu64(rdi, rcx); crate::simd_ops::wu64(rdi.wrapping_add(P2), rcx); crate::simd_ops::wu8(rbx.wrapping_add(P0), (0x4u8) as u8); rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411cacd0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x24; const P1: u64 = 0x28; const P2: u64 = 0x14241d6f0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rdx; rdi = rcx; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rdx); fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 1; continue; }
            1 => { fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 5; continue; } bb = 2; continue; }
            2 => { rax = 0x1; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rdx.wrapping_add(P0), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; rax = (((rax as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 5; continue; } bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P1))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            5 => { rdx = (crate::simd_ops::ru64(rdi)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 6; continue; }
            6 => { rax = 0xffffffff; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rdx.wrapping_add(P0), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 9; continue; } bb = 7; continue; }
            7 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P1))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 9; continue; }
            9 => { crate::simd_ops::wu64(rdi, rbx); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411cad70(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64;
    r8 = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64;
    rax = rdx;
    r8 = ((r8).wrapping_shl(5u32)) as u64;
    fl_l = ((r8).wrapping_add(crate::simd_ops::ru64(rcx))) as u64; fl_r = fl_l;
    r8 = ((fl_l as u64)) as u64;
    crate::simd_ops::wu64(rdx, r8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411cad90(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1411cc3d0; const P1: u64 = 0x1410c4000;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rdx; rdi = rcx; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x8u64))) as u32 as u64; r8 = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; r8 = ((r8).wrapping_shl(4u32)) as u64; rax = (0x6666666666666667u64) as u64; let prod = (rax as i64 as i128).wrapping_mul((r8) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rsi = rdx; rsi = ((((rsi) as i64).wrapping_shr(5u32) as u64)) as u64; rax = rsi; rax = ((rax).wrapping_shr(63u32)) as u64; rsi = ((rsi).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0xcu64))) as u32 as u64; fl_l = rbx; fl_r = rax; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 5; continue; } bb = 1; continue; }
            1 => { rdx = ((rax as u32)) as u32 as u64; rdx = ((rdx).wrapping_shr(1u32)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; fl_l = rdx; fl_r = rbx; rdx = (if (((fl_l as u64)) < ((fl_r as u64))) { rbx } else { rdx }) as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rdx = rbx; fl_l = rdx; fl_r = rsi; rdx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 12; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rcx = ((rcx).wrapping_add(0x40u64)) as u64; fl_l = ((rcx).wrapping_add(crate::simd_ops::ru64(rdi))) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; bb = 3; continue; }
            3 => { crate::simd_ops::wu8(rcx, (0x0u8) as u8); rcx = rcx.wrapping_add(0x50); fl_l = rdx; fl_r = (0x1u64) as u64; rdx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 4; continue; }
            4 => { crate::simd_ops::wu32(rdi.wrapping_add(0x8u64), ((rbx as u32)) as u32); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            5 => { fl_l = rbx; fl_r = rsi; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 10; continue; } bb = 6; continue; }
            6 => { rcx = rbx; fl_l = rcx; fl_r = rsi; rcx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 12; continue; } bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rax = ((rax).wrapping_add(0x40u64)) as u64; fl_l = ((rax).wrapping_add(r8)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; bb = 8; continue; }
            8 => { crate::simd_ops::wu8(rax, (0x0u8) as u8); rax = rax.wrapping_add(0x50); fl_l = rcx; fl_r = (0x1u64) as u64; rcx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 8; continue; } bb = 9; continue; }
            9 => { crate::simd_ops::wu32(rdi.wrapping_add(0x8u64), ((rbx as u32)) as u32); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            10 => { if (((fl_l as u64)) == ((fl_r as u64))) { bb = 13; continue; } bb = 11; continue; }
            11 => { r8 = ((r8).wrapping_add(crate::simd_ops::ru64(rcx))) as u64; rdx = ((rbx.wrapping_add(rbx.wrapping_mul(4))) as u64) as u64; rdx = ((rdx).wrapping_shl(4u32)) as u64; rdx = ((rdx).wrapping_add(crate::simd_ops::ru64(rcx))) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 12; continue; }
            12 => { crate::simd_ops::wu32(rdi.wrapping_add(0x8u64), ((rbx as u32)) as u32); bb = 13; continue; }
            13 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411caea0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x40; const P1: u64 = 0x1; const P2: u64 = 0x1410c38f0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rax = (((crate::simd_ops::ru8(rcx.wrapping_add(P0))) as i8 as i64) as u64) as u64;
    rdi = rdx;
    rbx = rcx;
    fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x3u8) as u8 as u64;
    if !((((fl_l as u8)) != ((fl_r as u8)))) { rax = ((crate::simd_ops::ru8(rdx)) as u32) as u32 as u64; crate::simd_ops::wu8(rcx, ((rax as u8)) as u8); rax = rcx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    rcx = ((rax.wrapping_add(P1)) as u64) as u64;
    crate::simd_ops::wu8(rsp.wrapping_add(0x30u64), (0x0u8) as u8);
    r8 = rbx;
    rdx = rsp.wrapping_add(0x30);
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    crate::simd_ops::wu8(rbx.wrapping_add(P0), (0xffu8) as u8);
    rax = ((crate::simd_ops::ru8(rdi)) as u32) as u32 as u64;
    crate::simd_ops::wu8(rbx.wrapping_add(P0), (0x3u8) as u8);
    crate::simd_ops::wu8(rbx, ((rax as u8)) as u8);
    rax = rbx;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411caf00(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x40; const P1: u64 = 0x1; const P2: u64 = 0x1410c38f0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rax = (((crate::simd_ops::ru8(rcx.wrapping_add(P0))) as i8 as i64) as u64) as u64;
    rdi = rdx;
    rbx = rcx;
    fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x5u8) as u8 as u64;
    if !((((fl_l as u8)) != ((fl_r as u8)))) { rax = (crate::simd_ops::ru32(rdx)) as u32 as u64; crate::simd_ops::wu32(rcx, ((rax as u32)) as u32); rax = rcx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    rcx = ((rax.wrapping_add(P1)) as u64) as u64;
    crate::simd_ops::wu8(rsp.wrapping_add(0x30u64), (0x0u8) as u8);
    r8 = rbx;
    rdx = rsp.wrapping_add(0x30);
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    crate::simd_ops::wu8(rbx.wrapping_add(P0), (0xffu8) as u8);
    rax = (crate::simd_ops::ru32(rdi)) as u32 as u64;
    crate::simd_ops::wu32(rbx, ((rax as u32)) as u32);
    rax = rbx;
    crate::simd_ops::wu8(rbx.wrapping_add(P0), (0x5u8) as u8);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411caf60(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x40; const P1: u64 = 0x1; const P2: u64 = 0x1410c38f0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rax = (((crate::simd_ops::ru8(rcx.wrapping_add(P0))) as i8 as i64) as u64) as u64;
    rdi = rdx;
    rbx = rcx;
    fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x7u8) as u8 as u64;
    if !((((fl_l as u8)) != ((fl_r as u8)))) { rax = (crate::simd_ops::ru32(rdx)) as u32 as u64; crate::simd_ops::wu32(rcx, ((rax as u32)) as u32); rax = rcx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    rcx = ((rax.wrapping_add(P1)) as u64) as u64;
    crate::simd_ops::wu8(rsp.wrapping_add(0x30u64), (0x0u8) as u8);
    r8 = rbx;
    rdx = rsp.wrapping_add(0x30);
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    crate::simd_ops::wu8(rbx.wrapping_add(P0), (0xffu8) as u8);
    rax = (crate::simd_ops::ru32(rdi)) as u32 as u64;
    crate::simd_ops::wu8(rbx.wrapping_add(P0), (0x7u8) as u8);
    crate::simd_ops::wu32(rbx, ((rax as u32)) as u32);
    rax = rbx;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411cafc0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x40; const P1: u64 = 0x1; const P2: u64 = 0x1410c38f0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rax = (((crate::simd_ops::ru8(rcx.wrapping_add(P0))) as i8 as i64) as u64) as u64;
    rdi = rdx;
    rbx = rcx;
    fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x9u8) as u8 as u64;
    if !((((fl_l as u8)) != ((fl_r as u8)))) { rax = (crate::simd_ops::ru64(rdx)) as u64; crate::simd_ops::wu64(rcx, rax); rax = rcx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    rcx = ((rax.wrapping_add(P1)) as u64) as u64;
    crate::simd_ops::wu8(rsp.wrapping_add(0x30u64), (0x0u8) as u8);
    r8 = rbx;
    rdx = rsp.wrapping_add(0x30);
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    crate::simd_ops::wu8(rbx.wrapping_add(P0), (0xffu8) as u8);
    rax = (crate::simd_ops::ru64(rdi)) as u64;
    crate::simd_ops::wu8(rbx.wrapping_add(P0), (0x9u8) as u8);
    crate::simd_ops::wu64(rbx, rax);
    rax = rbx;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411cb020(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x40; const P1: u64 = 0x1; const P2: u64 = 0x1410c38f0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rax = (((crate::simd_ops::ru8(rcx.wrapping_add(P0))) as i8 as i64) as u64) as u64;
    rdi = rdx;
    rbx = rcx;
    fl_l = ((rax as u8)) as u8 as u64; fl_r = (0xfu8) as u8 as u64;
    if !((((fl_l as u8)) != ((fl_r as u8)))) { rax = (crate::simd_ops::ru64(rdx)) as u64; crate::simd_ops::wu64(rcx, rax); rax = rcx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    rcx = ((rax.wrapping_add(P1)) as u64) as u64;
    crate::simd_ops::wu8(rsp.wrapping_add(0x30u64), (0x0u8) as u8);
    r8 = rbx;
    rdx = rsp.wrapping_add(0x30);
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    crate::simd_ops::wu8(rbx.wrapping_add(P0), (0xffu8) as u8);
    rax = (crate::simd_ops::ru64(rdi)) as u64;
    crate::simd_ops::wu8(rbx.wrapping_add(P0), (0xfu8) as u8);
    crate::simd_ops::wu64(rbx, rax);
    rax = rbx;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411cb080(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x40; const P1: u64 = 0x8; const P2: u64 = 0x1; const P3: u64 = 0x1410c38f0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (((crate::simd_ops::ru8(rcx.wrapping_add(P0))) as i8 as i64) as u64) as u64; rdi = rdx; rbx = rcx; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0xbu8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 2; continue; } bb = 1; continue; }
            1 => { xmm0 = ((crate::simd_ops::ru64(rdx)) as u64) as u128; crate::simd_ops::wu64(rcx, ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(P1), ((rax as u32)) as u32); rax = rcx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            2 => { rcx = ((rax.wrapping_add(P2)) as u64) as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x30u64), (0x0u8) as u8); r8 = rbx; rdx = rsp.wrapping_add(0x30); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu8(rbx.wrapping_add(P0), (0xffu8) as u8); xmm0 = ((crate::simd_ops::ru64(rdi)) as u64) as u128; crate::simd_ops::wu64(rbx, ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(rdi.wrapping_add(P1))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(P1), ((rax as u32)) as u32); rax = rbx; crate::simd_ops::wu8(rbx.wrapping_add(P0), (0xbu8) as u8); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411cb0f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x40; const P1: u64 = 0x8; const P2: u64 = 0x1; const P3: u64 = 0x1410c38f0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (((crate::simd_ops::ru8(rcx.wrapping_add(P0))) as i8 as i64) as u64) as u64; rdi = rdx; rbx = rcx; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x11u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 2; continue; } bb = 1; continue; }
            1 => { xmm0 = ((crate::simd_ops::ru64(rdx)) as u64) as u128; crate::simd_ops::wu64(rcx, ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(P1), ((rax as u32)) as u32); rax = rcx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            2 => { rcx = ((rax.wrapping_add(P2)) as u64) as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x30u64), (0x0u8) as u8); r8 = rbx; rdx = rsp.wrapping_add(0x30); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu8(rbx.wrapping_add(P0), (0xffu8) as u8); xmm0 = ((crate::simd_ops::ru64(rdi)) as u64) as u128; crate::simd_ops::wu64(rbx, ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(rdi.wrapping_add(P1))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(P1), ((rax as u32)) as u32); rax = rbx; crate::simd_ops::wu8(rbx.wrapping_add(P0), (0x11u8) as u8); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411cb160(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x40; const P1: u64 = 0x1; const P2: u64 = 0x1410c38f0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rax = (((crate::simd_ops::ru8(rcx.wrapping_add(P0))) as i8 as i64) as u64) as u64;
    rdi = rdx;
    rbx = rcx;
    fl_l = ((rax as u8)) as u8 as u64; fl_r = (0xdu8) as u8 as u64;
    if !((((fl_l as u8)) != ((fl_r as u8)))) { xmm0 = (crate::simd_ops::ru128(rdx)) as u128; crate::simd_ops::wu128(rcx, (xmm0) as u128); rax = rcx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
    rcx = ((rax.wrapping_add(P1)) as u64) as u64;
    crate::simd_ops::wu8(rsp.wrapping_add(0x30u64), (0x0u8) as u8);
    r8 = rbx;
    rdx = rsp.wrapping_add(0x30);
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    crate::simd_ops::wu8(rbx.wrapping_add(P0), (0xffu8) as u8);
    rax = rbx;
    xmm0 = (crate::simd_ops::ru128(rdi)) as u128;
    crate::simd_ops::wu128(rbx, (xmm0) as u128);
    crate::simd_ops::wu8(rbx.wrapping_add(P0), (0xdu8) as u8);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411cb1d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x40; const P1: u64 = 0x1; const P2: u64 = 0x1410c38f0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rax = (((crate::simd_ops::ru8(rcx.wrapping_add(P0))) as i8 as i64) as u64) as u64;
    rdi = rdx;
    rbx = rcx;
    fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x13u8) as u8 as u64;
    if !((((fl_l as u8)) != ((fl_r as u8)))) { xmm0 = (crate::simd_ops::ru128(rdx)) as u128; crate::simd_ops::wu128(rcx, (xmm0) as u128); rax = rcx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
    rcx = ((rax.wrapping_add(P1)) as u64) as u64;
    crate::simd_ops::wu8(rsp.wrapping_add(0x30u64), (0x0u8) as u8);
    r8 = rbx;
    rdx = rsp.wrapping_add(0x30);
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    crate::simd_ops::wu8(rbx.wrapping_add(P0), (0xffu8) as u8);
    rax = rbx;
    xmm0 = (crate::simd_ops::ru128(rdi)) as u128;
    crate::simd_ops::wu128(rbx, (xmm0) as u128);
    crate::simd_ops::wu8(rbx.wrapping_add(P0), (0x13u8) as u8);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411cb240(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x40; const P1: u64 = 0x1; const P2: u64 = 0x1410c38f0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rax = (((crate::simd_ops::ru8(rcx.wrapping_add(P0))) as i8 as i64) as u64) as u64;
    rdi = rdx;
    rbx = rcx;
    fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x2fu8) as u8 as u64;
    if !((((fl_l as u8)) != ((fl_r as u8)))) { rax = (crate::simd_ops::ru64(rdx)) as u64; crate::simd_ops::wu64(rcx, rax); rax = rcx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    rcx = ((rax.wrapping_add(P1)) as u64) as u64;
    crate::simd_ops::wu8(rsp.wrapping_add(0x30u64), (0x0u8) as u8);
    r8 = rbx;
    rdx = rsp.wrapping_add(0x30);
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    crate::simd_ops::wu8(rbx.wrapping_add(P0), (0xffu8) as u8);
    rax = (crate::simd_ops::ru64(rdi)) as u64;
    crate::simd_ops::wu8(rbx.wrapping_add(P0), (0x2fu8) as u8);
    crate::simd_ops::wu64(rbx, rax);
    rax = rbx;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411cb2a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x40; const P1: u64 = 0x1; const P2: u64 = 0x1410c38f0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rax = (((crate::simd_ops::ru8(rcx.wrapping_add(P0))) as i8 as i64) as u64) as u64;
    rdi = rdx;
    rbx = rcx;
    fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x31u8) as u8 as u64;
    if !((((fl_l as u8)) != ((fl_r as u8)))) { rax = (crate::simd_ops::ru64(rdx)) as u64; crate::simd_ops::wu64(rcx, rax); rax = rcx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    rcx = ((rax.wrapping_add(P1)) as u64) as u64;
    crate::simd_ops::wu8(rsp.wrapping_add(0x30u64), (0x0u8) as u8);
    r8 = rbx;
    rdx = rsp.wrapping_add(0x30);
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    crate::simd_ops::wu8(rbx.wrapping_add(P0), (0xffu8) as u8);
    rax = (crate::simd_ops::ru64(rdi)) as u64;
    crate::simd_ops::wu8(rbx.wrapping_add(P0), (0x31u8) as u8);
    crate::simd_ops::wu64(rbx, rax);
    rax = rbx;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411cb300(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14397ff58; const P1: u64 = 0x1426455a8; const P2: u64 = 0x142645598; const P3: u64 = 0x14241d6f0; const P4: u64 = 0x142645580;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x60u64)) as u64; r14 = rcx; rax = (crate::simd_ops::ru64(P0)) as u64; rdi = (crate::simd_ops::ru64(rax.wrapping_add(0x168u64))) as u64; rcx = rdi.wrapping_add(0x18); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x20u64))) as u64; rsi = (crate::simd_ops::ru64(rdi.wrapping_add(0x28u64))) as u64; r8 = rsi.wrapping_add(0x18); fl_l = r8; fl_r = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 9; continue; } bb = 1; continue; }
            1 => { fl_l = rsi; fl_r = rdx; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 6; continue; } bb = 2; continue; }
            2 => { bb = 3; continue; }
            3 => { fl_l = r8; fl_r = rdx; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 6; continue; } bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x30u64))) as u32 as u64; rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x34u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x20u64))) as u64; fl_l = rsi; fl_r = rdx; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 3; continue; } bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; rax = ((rax).wrapping_sub(0x8u64)) as u64; fl_l = rsi; fl_r = rax; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 8; continue; } bb = 7; continue; }
            7 => { rax = (0xffffffffffu64) as u64; crate::simd_ops::wu64(rsi, rax); bb = 8; continue; }
            8 => { rsi = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; r8 = rsi.wrapping_add(0x18); bb = 9; continue; }
            9 => { fl_l = rsi; fl_r = rdx; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 13; continue; } bb = 10; continue; }
            10 => { fl_l = r8; fl_r = rdx; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 13; continue; } bb = 11; continue; }
            11 => { rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x30u64))) as u32 as u64; rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x34u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 13; continue; } bb = 12; continue; }
            12 => { rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x20u64))) as u64; fl_l = rsi; fl_r = rdx; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 10; continue; } bb = 13; continue; }
            13 => { rax = (0x69fafafafaffu64) as u64; crate::simd_ops::wu64(rsi, rax); rsi = ((rsi).wrapping_add(0x8u64)) as u64; let tmp_xchg: u64 = crate::simd_ops::atomic_xchg_u64(rdi.wrapping_add(0x28u64), r8); r8 = (tmp_xchg) as u64; rcx = rdi.wrapping_add(0x18); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(rsi, (0x22bu32) as u32); rax = rsi.wrapping_add(0x4); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (0xcu64) as u64); let y_addr: u64 = rsp.wrapping_add(0x20u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rsp.wrapping_add(0x40u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); rdx = (crate::simd_ops::ru64(r14)) as u64; crate::simd_ops::wu64(rax, rdx); fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 14; continue; }
            14 => { rax = 0x1; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rdx.wrapping_add(0x24u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; rax = (((rax as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 17; continue; } bb = 15; continue; }
            15 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x28u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 16; continue; }
            16 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); let y_addr: u64 = rsp.wrapping_add(0x40u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); bb = 17; continue; }
            17 => { rcx = (crate::simd_ops::ru64(P0)) as u64; rax = ((xmm0 as u64)) as u64; crate::simd_ops::lk_and_u32(rax, (0x1ffu32) as u32); crate::simd_ops::lk_or_u32(rax, (0x200u32) as u32); r8 = (crate::simd_ops::ru64(rcx.wrapping_add(0x168u64))) as u64; xmm0 = (ymm0_hi) as u128; ymm0_hi = 0; crate::simd_ops::wu64(rsi.wrapping_sub(0x8u64), (((xmm0 >> 64) as u64)) as u64); crate::simd_ops::lk_add_u32(r8.wrapping_add(0x30u64), (1) as u32); fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0x198u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 19; continue; } bb = 18; continue; }
            18 => { r8 = 0; fl_l = 0; fl_r = 0; rdx = ((r8.wrapping_add(0x1u64)) as u32) as u32 as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x1a0u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 19; continue; }
            19 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; rsp = ((rsp).wrapping_add(0x60u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411cb4a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14397ff58; const P1: u64 = 0x1426455a8; const P2: u64 = 0x142645598; const P3: u64 = 0x14241d6f0; const P4: u64 = 0x142645580;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x60u64)) as u64; r14 = rcx; rax = (crate::simd_ops::ru64(P0)) as u64; rdi = (crate::simd_ops::ru64(rax.wrapping_add(0x168u64))) as u64; rcx = rdi.wrapping_add(0x18); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x20u64))) as u64; rsi = (crate::simd_ops::ru64(rdi.wrapping_add(0x28u64))) as u64; r8 = rsi.wrapping_add(0x18); fl_l = r8; fl_r = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 9; continue; } bb = 1; continue; }
            1 => { fl_l = rsi; fl_r = rdx; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 6; continue; } bb = 2; continue; }
            2 => { bb = 3; continue; }
            3 => { fl_l = r8; fl_r = rdx; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 6; continue; } bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x30u64))) as u32 as u64; rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x34u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x20u64))) as u64; fl_l = rsi; fl_r = rdx; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 3; continue; } bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; rax = ((rax).wrapping_sub(0x8u64)) as u64; fl_l = rsi; fl_r = rax; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 8; continue; } bb = 7; continue; }
            7 => { rax = (0xffffffffffu64) as u64; crate::simd_ops::wu64(rsi, rax); bb = 8; continue; }
            8 => { rsi = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; r8 = rsi.wrapping_add(0x18); bb = 9; continue; }
            9 => { fl_l = rsi; fl_r = rdx; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 13; continue; } bb = 10; continue; }
            10 => { fl_l = r8; fl_r = rdx; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 13; continue; } bb = 11; continue; }
            11 => { rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x30u64))) as u32 as u64; rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x34u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 13; continue; } bb = 12; continue; }
            12 => { rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x20u64))) as u64; fl_l = rsi; fl_r = rdx; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 10; continue; } bb = 13; continue; }
            13 => { rax = (0x69fafafafaffu64) as u64; crate::simd_ops::wu64(rsi, rax); rsi = ((rsi).wrapping_add(0x8u64)) as u64; let tmp_xchg: u64 = crate::simd_ops::atomic_xchg_u64(rdi.wrapping_add(0x28u64), r8); r8 = (tmp_xchg) as u64; rcx = rdi.wrapping_add(0x18); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(rsi, (0x22cu32) as u32); rax = rsi.wrapping_add(0x4); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (0xcu64) as u64); let y_addr: u64 = rsp.wrapping_add(0x20u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rsp.wrapping_add(0x40u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); rdx = (crate::simd_ops::ru64(r14)) as u64; crate::simd_ops::wu64(rax, rdx); fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 14; continue; }
            14 => { rax = 0x1; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rdx.wrapping_add(0x24u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; rax = (((rax as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 17; continue; } bb = 15; continue; }
            15 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x28u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 16; continue; }
            16 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); let y_addr: u64 = rsp.wrapping_add(0x40u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); bb = 17; continue; }
            17 => { rcx = (crate::simd_ops::ru64(P0)) as u64; rax = ((xmm0 as u64)) as u64; crate::simd_ops::lk_and_u32(rax, (0x1ffu32) as u32); crate::simd_ops::lk_or_u32(rax, (0x200u32) as u32); r8 = (crate::simd_ops::ru64(rcx.wrapping_add(0x168u64))) as u64; xmm0 = (ymm0_hi) as u128; ymm0_hi = 0; crate::simd_ops::wu64(rsi.wrapping_sub(0x8u64), (((xmm0 >> 64) as u64)) as u64); crate::simd_ops::lk_add_u32(r8.wrapping_add(0x30u64), (1) as u32); fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0x198u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 19; continue; } bb = 18; continue; }
            18 => { r8 = 0; fl_l = 0; fl_r = 0; rdx = ((r8.wrapping_add(0x1u64)) as u32) as u32 as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x1a0u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 19; continue; }
            19 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; rsp = ((rsp).wrapping_add(0x60u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411cb640(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x40;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = rcx; fl_r = rcx;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rax = 0; fl_l = 0; fl_r = 0; return rax; }
    fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(P0))) as u8 as u64; fl_r = (0xcu8) as u8 as u64;
    if !((((fl_l as u8)) != ((fl_r as u8)))) { rax = rcx; return rax; }
    rax = 0;
    fl_l = 0; fl_r = 0;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411cb660(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x40; const P1: u64 = 0x1403b3290; const P2: u64 = 0x24; const P3: u64 = 0x28; const P4: u64 = 0x14241d6f0; const P5: u64 = 0x1410c38f0; const P6: u64 = 0x140280170;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(P0))) as u8 as u64; fl_r = (0x3bu8) as u8 as u64; rbx = rcx; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rax = rbx; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            2 => { crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rdi); rdi = (crate::simd_ops::ru64(rdx)) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 3; continue; }
            3 => { rax = 0x1; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rdi.wrapping_add(P2), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; rax = (((rax as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 6; continue; } bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P3))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rdx = rdi; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            6 => { rcx = (((crate::simd_ops::ru8(rbx.wrapping_add(P0))) as i8 as i64) as u64) as u64; rdx = rsp.wrapping_add(0x30); rcx = ((rcx).wrapping_add(1)) as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x30u64), (0x0u8) as u8); r8 = rbx; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx, rdi); rcx = rsp.wrapping_add(0x30); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (0x0u64) as u64); crate::simd_ops::wu8(rbx.wrapping_add(P0), (0x3bu8) as u8); rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rax = rbx; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411cb6f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x40;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = rcx; fl_r = rcx;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rax = 0; fl_l = 0; fl_r = 0; return rax; }
    fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(P0))) as u8 as u64; fl_r = (0xeu8) as u8 as u64;
    if !((((fl_l as u8)) != ((fl_r as u8)))) { rax = rcx; return rax; }
    rax = 0;
    fl_l = 0; fl_r = 0;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411cb710(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x40; const P1: u64 = 0x1; const P2: u64 = 0x1410c38f0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rax = (((crate::simd_ops::ru8(rcx.wrapping_add(P0))) as i8 as i64) as u64) as u64;
    rdi = rdx;
    rbx = rcx;
    fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x3du8) as u8 as u64;
    if !((((fl_l as u8)) != ((fl_r as u8)))) { rax = (crate::simd_ops::ru32(rdx)) as u32 as u64; crate::simd_ops::wu32(rcx, ((rax as u32)) as u32); rax = rcx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    rcx = ((rax.wrapping_add(P1)) as u64) as u64;
    crate::simd_ops::wu8(rsp.wrapping_add(0x30u64), (0x0u8) as u8);
    r8 = rbx;
    rdx = rsp.wrapping_add(0x30);
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    crate::simd_ops::wu8(rbx.wrapping_add(P0), (0xffu8) as u8);
    rax = (crate::simd_ops::ru32(rdi)) as u32 as u64;
    crate::simd_ops::wu8(rbx.wrapping_add(P0), (0x3du8) as u8);
    crate::simd_ops::wu32(rbx, ((rax as u32)) as u32);
    rax = rbx;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411cb770(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x40;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = rcx; fl_r = rcx;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rax = 0; fl_l = 0; fl_r = 0; return rax; }
    fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(P0))) as u8 as u64; fl_r = (0xfu8) as u8 as u64;
    if !((((fl_l as u8)) != ((fl_r as u8)))) { rax = rcx; return rax; }
    rax = 0;
    fl_l = 0; fl_r = 0;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411cb790(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x40; const P1: u64 = 0x1; const P2: u64 = 0x1410c38f0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rax = (((crate::simd_ops::ru8(rcx.wrapping_add(P0))) as i8 as i64) as u64) as u64;
    rdi = rdx;
    rbx = rcx;
    fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x3fu8) as u8 as u64;
    if !((((fl_l as u8)) != ((fl_r as u8)))) { rax = (crate::simd_ops::ru32(rdx)) as u32 as u64; crate::simd_ops::wu32(rcx, ((rax as u32)) as u32); rax = rcx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    rcx = ((rax.wrapping_add(P1)) as u64) as u64;
    crate::simd_ops::wu8(rsp.wrapping_add(0x30u64), (0x0u8) as u8);
    r8 = rbx;
    rdx = rsp.wrapping_add(0x30);
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    crate::simd_ops::wu8(rbx.wrapping_add(P0), (0xffu8) as u8);
    rax = (crate::simd_ops::ru32(rdi)) as u32 as u64;
    crate::simd_ops::wu8(rbx.wrapping_add(P0), (0x3fu8) as u8);
    crate::simd_ops::wu32(rbx, ((rax as u32)) as u32);
    rax = rbx;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411cb7f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x40;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = rcx; fl_r = rcx;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rax = 0; fl_l = 0; fl_r = 0; return rax; }
    fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(P0))) as u8 as u64; fl_r = (0x10u8) as u8 as u64;
    if !((((fl_l as u8)) != ((fl_r as u8)))) { rax = rcx; return rax; }
    rax = 0;
    fl_l = 0; fl_r = 0;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411cb810(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x40; const P1: u64 = 0x1; const P2: u64 = 0x1410c38f0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rax = (((crate::simd_ops::ru8(rcx.wrapping_add(P0))) as i8 as i64) as u64) as u64;
    rdi = rdx;
    rbx = rcx;
    fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x41u8) as u8 as u64;
    if !((((fl_l as u8)) != ((fl_r as u8)))) { rax = (crate::simd_ops::ru32(rdx)) as u32 as u64; crate::simd_ops::wu32(rcx, ((rax as u32)) as u32); rax = rcx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    rcx = ((rax.wrapping_add(P1)) as u64) as u64;
    crate::simd_ops::wu8(rsp.wrapping_add(0x30u64), (0x0u8) as u8);
    r8 = rbx;
    rdx = rsp.wrapping_add(0x30);
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    crate::simd_ops::wu8(rbx.wrapping_add(P0), (0xffu8) as u8);
    rax = (crate::simd_ops::ru32(rdi)) as u32 as u64;
    crate::simd_ops::wu8(rbx.wrapping_add(P0), (0x41u8) as u8);
    crate::simd_ops::wu32(rbx, ((rax as u32)) as u32);
    rax = rbx;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411cb870(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x40;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = rcx; fl_r = rcx;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rax = 0; fl_l = 0; fl_r = 0; return rax; }
    fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(P0))) as u8 as u64; fl_r = (0x11u8) as u8 as u64;
    if !((((fl_l as u8)) != ((fl_r as u8)))) { rax = rcx; return rax; }
    rax = 0;
    fl_l = 0; fl_r = 0;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411cb890(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x40; const P1: u64 = 0x1; const P2: u64 = 0x1410c38f0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rax = (((crate::simd_ops::ru8(rcx.wrapping_add(P0))) as i8 as i64) as u64) as u64;
    rdi = rdx;
    rbx = rcx;
    fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x43u8) as u8 as u64;
    if !((((fl_l as u8)) != ((fl_r as u8)))) { rax = (crate::simd_ops::ru32(rdx)) as u32 as u64; crate::simd_ops::wu32(rcx, ((rax as u32)) as u32); rax = rcx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    rcx = ((rax.wrapping_add(P1)) as u64) as u64;
    crate::simd_ops::wu8(rsp.wrapping_add(0x30u64), (0x0u8) as u8);
    r8 = rbx;
    rdx = rsp.wrapping_add(0x30);
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    crate::simd_ops::wu8(rbx.wrapping_add(P0), (0xffu8) as u8);
    rax = (crate::simd_ops::ru32(rdi)) as u32 as u64;
    crate::simd_ops::wu8(rbx.wrapping_add(P0), (0x43u8) as u8);
    crate::simd_ops::wu32(rbx, ((rax as u32)) as u32);
    rax = rbx;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411cb8f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x40;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = rcx; fl_r = rcx;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rax = 0; fl_l = 0; fl_r = 0; return rax; }
    fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(P0))) as u8 as u64; fl_r = (0x12u8) as u8 as u64;
    if !((((fl_l as u8)) != ((fl_r as u8)))) { rax = rcx; return rax; }
    rax = 0;
    fl_l = 0; fl_r = 0;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411cb910(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x40; const P1: u64 = 0x1; const P2: u64 = 0x1410c38f0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rax = (((crate::simd_ops::ru8(rcx.wrapping_add(P0))) as i8 as i64) as u64) as u64;
    rdi = rdx;
    rbx = rcx;
    fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x4bu8) as u8 as u64;
    if !((((fl_l as u8)) != ((fl_r as u8)))) { rax = (crate::simd_ops::ru32(rdx)) as u32 as u64; crate::simd_ops::wu32(rcx, ((rax as u32)) as u32); rax = rcx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    rcx = ((rax.wrapping_add(P1)) as u64) as u64;
    crate::simd_ops::wu8(rsp.wrapping_add(0x30u64), (0x0u8) as u8);
    r8 = rbx;
    rdx = rsp.wrapping_add(0x30);
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    crate::simd_ops::wu8(rbx.wrapping_add(P0), (0xffu8) as u8);
    rax = (crate::simd_ops::ru32(rdi)) as u32 as u64;
    crate::simd_ops::wu8(rbx.wrapping_add(P0), (0x4bu8) as u8);
    crate::simd_ops::wu32(rbx, ((rax as u32)) as u32);
    rax = rbx;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411cb970(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x40; const P1: u64 = 0x1405a4d20; const P2: u64 = 0x1; const P3: u64 = 0x1410c38f0; const P4: u64 = 0x1403c7e70;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rax = (((crate::simd_ops::ru8(rcx.wrapping_add(P0))) as i8 as i64) as u64) as u64;
    rdi = rdx;
    rbx = rcx;
    fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x48u8) as u8 as u64;
    if !((((fl_l as u8)) != ((fl_r as u8)))) { rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    rcx = ((rax.wrapping_add(P2)) as u64) as u64;
    crate::simd_ops::wu8(rsp.wrapping_add(0x30u64), (0x0u8) as u8);
    r8 = rbx;
    rdx = rsp.wrapping_add(0x30);
    rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp);
    rdx = rdi;
    crate::simd_ops::wu8(rbx.wrapping_add(P0), (0xffu8) as u8);
    rcx = rbx;
    rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp);
    rax = rbx;
    crate::simd_ops::wu8(rbx.wrapping_add(P0), (0x48u8) as u8);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411cb9e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x40; const P1: u64 = 0x1405a4d20; const P2: u64 = 0x1; const P3: u64 = 0x1410c38f0; const P4: u64 = 0x1403c7e70;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rax = (((crate::simd_ops::ru8(rcx.wrapping_add(P0))) as i8 as i64) as u64) as u64;
    rdi = rdx;
    rbx = rcx;
    fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x46u8) as u8 as u64;
    if !((((fl_l as u8)) != ((fl_r as u8)))) { rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    rcx = ((rax.wrapping_add(P2)) as u64) as u64;
    crate::simd_ops::wu8(rsp.wrapping_add(0x30u64), (0x0u8) as u8);
    r8 = rbx;
    rdx = rsp.wrapping_add(0x30);
    rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp);
    rdx = rdi;
    crate::simd_ops::wu8(rbx.wrapping_add(P0), (0xffu8) as u8);
    rcx = rbx;
    rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp);
    rax = rbx;
    crate::simd_ops::wu8(rbx.wrapping_add(P0), (0x46u8) as u8);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411cba50(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x24; const P1: u64 = 0x28; const P2: u64 = 0x14241d6f0; const P3: u64 = 0x14202be30; const P4: u64 = 0x140427310;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 17>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x80u64)) as u64; rbx = r8; rdi = ((rdx as u32)) as u32 as u64; rsi = rcx; rdx = (crate::simd_ops::ru64(r9)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rdx); fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { rax = 0x1; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rdx.wrapping_add(P0), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; rax = (((rax as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 4; continue; } bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P1))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 4; continue; }
            4 => { crate::simd_ops::wu8(rsp.wrapping_add(0x70u64), (0xcu8) as u8); xmm0 = ((crate::simd_ops::ru32(rsp.wrapping_add(0xb0u64))) as u32) as u128; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((xmm0 as u32)) as u32); r9 = rsp.wrapping_add(0x30); r8 = rbx; rdx = ((rdi as u32)) as u32 as u64; rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rsp.wrapping_add(0x30); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r11 = rsp.wrapping_add(0x80); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x10u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x18u64))) as u64; rsp = r11; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411cbae0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1401e84a0; const P1: u64 = 0x14202be30; const P2: u64 = 0x140000000; const P3: u64 = 0x14241d6f0; const P4: u64 = 0x1401de830;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut j_tgt, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 19>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x90u64)) as u64; rbx = r8; rdi = ((rdx as u32)) as u32 as u64; rsi = rcx; rbp = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rbp); rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rax); crate::simd_ops::wu32(rsp.wrapping_add(0x50u64), ((rax as u32)) as u32); crate::simd_ops::wu8(rsp.wrapping_add(0x54u64), ((rax as u8)) as u8); rdx = r9; rcx = rsp.wrapping_add(0x40); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu8(rsp.wrapping_add(0x80u64), (0xdu8) as u8); xmm0 = ((crate::simd_ops::ru32(rsp.wrapping_add(0xc0u64))) as u32) as u128; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((xmm0 as u32)) as u32); r9 = rsp.wrapping_add(0x40); r8 = rbx; rdx = ((rdi as u32)) as u32 as u64; rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (((crate::simd_ops::ru8(rsp.wrapping_add(0x80u64))) as i8 as i64) as u64) as u64; rdx = P2; rax = ((crate::simd_ops::ru8(rdx.wrapping_add(rax).wrapping_add(0x11cbbedu64))) as u32) as u32 as u64; rcx = (crate::simd_ops::ru32(rdx.wrapping_add(rax.wrapping_mul(4)).wrapping_add(0x11cbbd4u64))) as u32 as u64; fl_l = ((rcx).wrapping_add(rdx)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; j_tgt = rcx; bb = 0xffff_fffe; continue; }
            1 => { bb = 2; continue; }
            2 => { rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 3; continue; }
            3 => { rax = 0xffffffff; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rdx.wrapping_add(0x24u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 6; continue; } bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x28u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 6; continue; }
            6 => { r11 = rsp.wrapping_add(0x90); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x10u64))) as u64; rbp = (crate::simd_ops::ru64(r11.wrapping_add(0x18u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rsp = r11; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            7 => { let tmp_xchg: u64 = crate::simd_ops::atomic_xchg_u64(rsp.wrapping_add(0x48u64), rbp); rbp = (tmp_xchg) as u64; bb = 6; continue; }
            8 => { rcx = rsp.wrapping_add(0x40); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 6; continue; }
            9 => { rcx = rsp.wrapping_add(0x40); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 6; continue; }
            0xffff_fffe => { match j_tgt.wrapping_sub(0x1411cbae0u64) { 0x90 => { bb = 1; continue; } 0x92 => { bb = 2; continue; } 0x9c => { bb = 3; continue; } 0xab => { bb = 4; continue; } 0xb4 => { bb = 5; continue; } 0xba => { bb = 6; continue; } 0xd3 => { bb = 7; continue; } 0xda => { bb = 8; continue; } 0xe7 => { bb = 9; continue; } _ => { return crate::simd_ops::tail_stk_xmm(j_tgt, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); } } }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411cbc10(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14202be30; const P1: u64 = 0x140427310;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 17>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = ((rsp).wrapping_sub(0x88u64)) as u64;
    rax = (crate::simd_ops::ru32(r9)) as u32 as u64;
    crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((rax as u32)) as u32);
    crate::simd_ops::wu8(rsp.wrapping_add(0x70u64), (0xeu8) as u8);
    xmm0 = ((crate::simd_ops::ru32(rsp.wrapping_add(0xb0u64))) as u32) as u128;
    crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((xmm0 as u32)) as u32);
    r9 = rsp.wrapping_add(0x30);
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rcx = rsp.wrapping_add(0x30);
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rsp = ((rsp).wrapping_add(0x88u64)) as u64;
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411cbc50(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14202be30; const P1: u64 = 0x140427310;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 17>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = ((rsp).wrapping_sub(0x88u64)) as u64;
    rax = (crate::simd_ops::ru32(r9)) as u32 as u64;
    crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((rax as u32)) as u32);
    crate::simd_ops::wu8(rsp.wrapping_add(0x70u64), (0xfu8) as u8);
    xmm0 = ((crate::simd_ops::ru32(rsp.wrapping_add(0xb0u64))) as u32) as u128;
    crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((xmm0 as u32)) as u32);
    r9 = rsp.wrapping_add(0x30);
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rcx = rsp.wrapping_add(0x30);
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rsp = ((rsp).wrapping_add(0x88u64)) as u64;
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411cbc90(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14202be30; const P1: u64 = 0x140427310;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 17>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = ((rsp).wrapping_sub(0x88u64)) as u64;
    rax = (crate::simd_ops::ru32(r9)) as u32 as u64;
    crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((rax as u32)) as u32);
    crate::simd_ops::wu8(rsp.wrapping_add(0x70u64), (0x10u8) as u8);
    xmm0 = ((crate::simd_ops::ru32(rsp.wrapping_add(0xb0u64))) as u32) as u128;
    crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((xmm0 as u32)) as u32);
    r9 = rsp.wrapping_add(0x30);
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rcx = rsp.wrapping_add(0x30);
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rsp = ((rsp).wrapping_add(0x88u64)) as u64;
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411cbcd0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14202be30; const P1: u64 = 0x140427310;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 17>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = ((rsp).wrapping_sub(0x88u64)) as u64;
    rax = (crate::simd_ops::ru32(r9)) as u32 as u64;
    crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((rax as u32)) as u32);
    crate::simd_ops::wu8(rsp.wrapping_add(0x70u64), (0x11u8) as u8);
    xmm0 = ((crate::simd_ops::ru32(rsp.wrapping_add(0xb0u64))) as u32) as u128;
    crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((xmm0 as u32)) as u32);
    r9 = rsp.wrapping_add(0x30);
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rcx = rsp.wrapping_add(0x30);
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rsp = ((rsp).wrapping_add(0x88u64)) as u64;
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411cbd10(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14202be30; const P1: u64 = 0x140427310;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 17>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = ((rsp).wrapping_sub(0x88u64)) as u64;
    rax = (crate::simd_ops::ru32(r9)) as u32 as u64;
    crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((rax as u32)) as u32);
    crate::simd_ops::wu8(rsp.wrapping_add(0x70u64), (0x12u8) as u8);
    xmm0 = ((crate::simd_ops::ru32(rsp.wrapping_add(0xb0u64))) as u32) as u128;
    crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((xmm0 as u32)) as u32);
    r9 = rsp.wrapping_add(0x30);
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rcx = rsp.wrapping_add(0x30);
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rsp = ((rsp).wrapping_add(0x88u64)) as u64;
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411cbd50(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1403c7e70; const P1: u64 = 0x14202be30; const P2: u64 = 0x140427310;
    let (mut rax, mut r11, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 17>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x80u64)) as u64;
    rbx = r8;
    rdi = ((rdx as u32)) as u32 as u64;
    rsi = rcx;
    rdx = r9;
    rcx = rsp.wrapping_add(0x30);
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    crate::simd_ops::wu8(rsp.wrapping_add(0x70u64), (0x14u8) as u8);
    xmm0 = ((crate::simd_ops::ru32(rsp.wrapping_add(0xb0u64))) as u32) as u128;
    crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((xmm0 as u32)) as u32);
    r9 = rsp.wrapping_add(0x30);
    r8 = rbx;
    rdx = ((rdi as u32)) as u32 as u64;
    rcx = rsi;
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rcx = rsp.wrapping_add(0x30);
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    r11 = rsp.wrapping_add(0x80);
    rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x10u64))) as u64;
    rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x18u64))) as u64;
    rsp = r11;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411cbdc0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1403c7e70; const P1: u64 = 0x14202be30; const P2: u64 = 0x140427310;
    let (mut rax, mut r11, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 17>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x80u64)) as u64;
    rbx = r8;
    rdi = ((rdx as u32)) as u32 as u64;
    rsi = rcx;
    rdx = r9;
    rcx = rsp.wrapping_add(0x30);
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    crate::simd_ops::wu8(rsp.wrapping_add(0x70u64), (0x13u8) as u8);
    xmm0 = ((crate::simd_ops::ru32(rsp.wrapping_add(0xb0u64))) as u32) as u128;
    crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((xmm0 as u32)) as u32);
    r9 = rsp.wrapping_add(0x30);
    r8 = rbx;
    rdx = ((rdi as u32)) as u32 as u64;
    rcx = rsi;
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rcx = rsp.wrapping_add(0x30);
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    r11 = rsp.wrapping_add(0x80);
    rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x10u64))) as u64;
    rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x18u64))) as u64;
    rsp = r11;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411cbe30(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14397ff58; const P1: u64 = 0x1426455a8; const P2: u64 = 0x142645598; const P3: u64 = 0x142645580;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rax = (crate::simd_ops::ru64(P0)) as u64; r14 = rcx; rdi = (crate::simd_ops::ru64(rax.wrapping_add(0x168u64))) as u64; rcx = rdi.wrapping_add(0x18); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x20u64))) as u64; rsi = (crate::simd_ops::ru64(rdi.wrapping_add(0x28u64))) as u64; r8 = rsi.wrapping_add(0x18); fl_l = r8; fl_r = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 9; continue; } bb = 1; continue; }
            1 => { fl_l = rsi; fl_r = rax; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 6; continue; } bb = 2; continue; }
            2 => { bb = 3; continue; }
            3 => { fl_l = r8; fl_r = rax; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 6; continue; } bb = 4; continue; }
            4 => { rdx = (crate::simd_ops::ru32(rdi.wrapping_add(0x30u64))) as u32 as u64; rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x34u64))) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x20u64))) as u64; fl_l = rsi; fl_r = rax; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 3; continue; } bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; rcx = ((rcx).wrapping_sub(0x8u64)) as u64; fl_l = rsi; fl_r = rcx; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 8; continue; } bb = 7; continue; }
            7 => { rcx = (0xffffffffffu64) as u64; crate::simd_ops::wu64(rsi, rcx); bb = 8; continue; }
            8 => { rsi = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; r8 = rsi.wrapping_add(0x18); bb = 9; continue; }
            9 => { fl_l = rsi; fl_r = rax; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 13; continue; } bb = 10; continue; }
            10 => { fl_l = r8; fl_r = rax; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 13; continue; } bb = 11; continue; }
            11 => { rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x30u64))) as u32 as u64; rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x34u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 13; continue; } bb = 12; continue; }
            12 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x20u64))) as u64; fl_l = rsi; fl_r = rax; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 10; continue; } bb = 13; continue; }
            13 => { rax = (0x69fafafafaffu64) as u64; rcx = rdi.wrapping_add(0x18); crate::simd_ops::wu64(rsi, rax); let tmp_xchg: u64 = crate::simd_ops::atomic_xchg_u64(rdi.wrapping_add(0x28u64), r8); r8 = (tmp_xchg) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(rsi.wrapping_add(0x8u64), (0x22fu32) as u32); r8 = rsi.wrapping_add(0x8); rax = (crate::simd_ops::ru64(r14)) as u64; rcx = r8.wrapping_add(0x4); crate::simd_ops::wu64(rcx, rax); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rcx); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rcx); rcx = (crate::simd_ops::ru64(P0)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r8); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (0xcu64) as u64); let y_addr: u64 = rsp.wrapping_add(0x20u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); rax = ((xmm0 as u64)) as u64; xmm0 = (ymm0_hi) as u128; ymm0_hi = 0; crate::simd_ops::lk_and_u32(rax, (0x1ffu32) as u32); crate::simd_ops::lk_or_u32(rax, (0x200u32) as u32); rdx = (crate::simd_ops::ru64(rcx.wrapping_add(0x168u64))) as u64; crate::simd_ops::wu64(r8.wrapping_sub(0x8u64), (((xmm0 >> 64) as u64)) as u64); crate::simd_ops::lk_add_u32(rdx.wrapping_add(0x30u64), (1) as u32); fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0x198u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 15; continue; } bb = 14; continue; }
            14 => { rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x1a0u64))) as u64; r8 = 0; fl_l = 0; fl_r = 0; rdx = ((r8.wrapping_add(0x1u64)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 15; continue; }
            15 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411cbfa0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x40; const P2: u64 = 0x2a0; const P3: u64 = 0x1; const P4: u64 = 0x1410c38f0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x28u64)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rdx = ((rdx.wrapping_add(rdx.wrapping_mul(4))) as u64) as u64; rdx = ((rdx).wrapping_shl(4u32)) as u64; fl_l = ((rax).wrapping_add(rdx)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; if (((fl_l as u64)) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru8(rax.wrapping_add(P1))) as u8 as u64; fl_r = (0x29u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 5; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbx); rbx = ((rcx.wrapping_add(P2)) as u64) as u64; rax = (((crate::simd_ops::ru8(rbx.wrapping_add(P1))) as i8 as i64) as u64) as u64; xmm0 = 0u128; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x29u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 4; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu128(rbx, (xmm0) as u128); rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rsp = ((rsp).wrapping_add(0x28u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            4 => { rcx = ((rax.wrapping_add(P3)) as u64) as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x30u64), (0x0u8) as u8); r8 = rbx; rdx = rsp.wrapping_add(0x30); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm0 = 0u128; crate::simd_ops::wu128(rbx, (xmm0) as u128); crate::simd_ops::wu8(rbx.wrapping_add(P1), (0x29u8) as u8); rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; bb = 5; continue; }
            5 => { rsp = ((rsp).wrapping_add(0x28u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411cc020(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x40; const P2: u64 = 0x2a0; const P3: u64 = 0x1; const P4: u64 = 0x1410c38f0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x38u64)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rdx = ((rdx.wrapping_add(rdx.wrapping_mul(4))) as u64) as u64; rdx = ((rdx).wrapping_shl(4u32)) as u64; fl_l = ((rax).wrapping_add(rdx)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; if (((fl_l as u64)) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru8(rax.wrapping_add(P1))) as u8 as u64; fl_r = (0x27u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 5; continue; } bb = 2; continue; }
            2 => { rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx); rbx = ((rcx.wrapping_add(P2)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); rax = (((crate::simd_ops::ru8(rbx.wrapping_add(P1))) as i8 as i64) as u64) as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x27u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 4; continue; } bb = 3; continue; }
            3 => { xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; crate::simd_ops::wu128(rbx, (xmm0) as u128); rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x38u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            4 => { rcx = ((rax.wrapping_add(P3)) as u64) as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x40u64), (0x0u8) as u8); r8 = rbx; rdx = rsp.wrapping_add(0x40); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; crate::simd_ops::wu128(rbx, (xmm0) as u128); crate::simd_ops::wu8(rbx.wrapping_add(P1), (0x27u8) as u8); rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; bb = 5; continue; }
            5 => { rsp = ((rsp).wrapping_add(0x38u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411cc0b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143bbcee8; const P1: u64 = 0x142421ed0;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x38u64)) as u64;
    r8 = rdx;
    crate::simd_ops::wu8(rsp.wrapping_add(0x20u64), ((r9 as u8)) as u8);
    rdx = P0;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x38u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411cc0d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x40; const P2: u64 = 0x2a0; const P3: u64 = 0x1; const P4: u64 = 0x1410c38f0; const P5: u64 = 0x140200070;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x28u64)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rdx = ((rdx.wrapping_add(rdx.wrapping_mul(4))) as u64) as u64; rdx = ((rdx).wrapping_shl(4u32)) as u64; fl_l = ((rax).wrapping_add(rdx)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; if (((fl_l as u64)) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru8(rax.wrapping_add(P1))) as u8 as u64; fl_r = (0x31u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 6; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbx); rbx = ((rcx.wrapping_add(P2)) as u64) as u64; rax = (((crate::simd_ops::ru8(rbx.wrapping_add(P1))) as i8 as i64) as u64) as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x31u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rcx = ((rax.wrapping_add(P3)) as u64) as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x30u64), (0x0u8) as u8); r8 = rbx; rdx = rsp.wrapping_add(0x30); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu8(rbx.wrapping_add(P1), (0x31u8) as u8); bb = 4; continue; }
            4 => { crate::simd_ops::wu64(rbx, (0x0u64) as u64); fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(P1))) as u8 as u64; fl_r = (0x31u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 7; continue; } bb = 5; continue; }
            5 => { rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; bb = 6; continue; }
            6 => { rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
            7 => { rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x1411cc139, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411cc140(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x40; const P2: u64 = 0x2a0; const P3: u64 = 0x1; const P4: u64 = 0x1410c38f0; const P5: u64 = 0x140200070;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x28u64)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rdx = ((rdx.wrapping_add(rdx.wrapping_mul(4))) as u64) as u64; rdx = ((rdx).wrapping_shl(4u32)) as u64; fl_l = ((rax).wrapping_add(rdx)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; if (((fl_l as u64)) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru8(rax.wrapping_add(P1))) as u8 as u64; fl_r = (0x2fu8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 6; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbx); rbx = ((rcx.wrapping_add(P2)) as u64) as u64; rax = (((crate::simd_ops::ru8(rbx.wrapping_add(P1))) as i8 as i64) as u64) as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x2fu8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rcx = ((rax.wrapping_add(P3)) as u64) as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x30u64), (0x0u8) as u8); r8 = rbx; rdx = rsp.wrapping_add(0x30); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu8(rbx.wrapping_add(P1), (0x2fu8) as u8); bb = 4; continue; }
            4 => { crate::simd_ops::wu64(rbx, (0x0u64) as u64); fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(P1))) as u8 as u64; fl_r = (0x2fu8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 7; continue; } bb = 5; continue; }
            5 => { rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; bb = 6; continue; }
            6 => { rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
            7 => { rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x1411cc1a9, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411cc1b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x40; const P1: u64 = 0x24; const P2: u64 = 0x28; const P3: u64 = 0x14241d6f0; const P4: u64 = 0x1; const P5: u64 = 0x1410c38f0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (((crate::simd_ops::ru8(rcx.wrapping_add(P0))) as i8 as i64) as u64) as u64; rdi = rdx; rbx = rcx; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x39u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 6; continue; } bb = 1; continue; }
            1 => { rdx = (crate::simd_ops::ru64(rcx)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 2; continue; }
            2 => { rax = 0xffffffff; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rdx.wrapping_add(P1), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 5; continue; } bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P2))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            5 => { rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rbx, rcx); rax = (crate::simd_ops::ru64(rdi)) as u64; crate::simd_ops::wu64(rdi, rcx); crate::simd_ops::wu64(rbx, rax); rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            6 => { rcx = ((rax.wrapping_add(P4)) as u64) as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x30u64), (0x0u8) as u8); r8 = rbx; rdx = rsp.wrapping_add(0x30); rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu8(rbx.wrapping_add(P0), (0xffu8) as u8); rcx = 0; fl_l = 0; fl_r = 0; rax = (crate::simd_ops::ru64(rdi)) as u64; crate::simd_ops::wu64(rdi, rcx); crate::simd_ops::wu64(rbx, rax); rax = rbx; crate::simd_ops::wu8(rbx.wrapping_add(P0), (0x39u8) as u8); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411cc250(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142537280; const P1: u64 = 0x142535070; const P2: u64 = 0x1426456c8; const P3: u64 = 0x1426453a0; const P4: u64 = 0x1410c4e50; const P5: u64 = 0x1411bc240;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rax = ((crate::exe::tls_pointer() as u64)) as u64; r15 = rdx; r12 = (crate::simd_ops::ru32(rcx.wrapping_add(0x8u64))) as u32 as u64; rdx = ((rdx.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64; r10 = 0x6348; rbp = 0; fl_l = 0; fl_r = 0; rdx = ((rdx).wrapping_shl(5u32)) as u64; r14 = rcx; r8 = (crate::simd_ops::ru64(rax)) as u64; fl_l = ((rdx) & ((rdx).wrapping_sub(1))) as u64; rax = ((fl_l as u64)) as u64; r10 = (crate::simd_ops::ru64(r10.wrapping_add(r8))) as u64; if (((fl_l as u64)) != 0) { bb = 5; continue; } bb = 1; continue; }
            1 => { rax = rdx.wrapping_sub(0x10); fl_l = rax; fl_r = (0x3f0u64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 5; continue; } bb = 2; continue; }
            2 => { rax = rdx; rax = ((rax).wrapping_shr(3u32)) as u64; r9 = (crate::simd_ops::ru64(r10.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x8u64))) as u64; r8 = (crate::simd_ops::ru64(r9.wrapping_add(0x10u64))) as u64; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { r9 = 0; fl_l = 0; fl_r = 0; rcx = r10; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); bb = 12; continue; }
            4 => { fl_l = (crate::simd_ops::lk_add_u32(r9.wrapping_add(0x18u64), (1) as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(r8)) as u64; crate::simd_ops::wu64(r9.wrapping_add(0x10u64), rax); bb = 13; continue; }
            5 => { rax = (0x7fffffffffffffffu64) as u64; fl_l = rdx; fl_r = rax; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 7; continue; } bb = 6; continue; }
            6 => { r8 = rbp; bb = 13; continue; }
            7 => { rax = rdx.wrapping_sub(0x10); fl_l = rax; fl_r = (0x3f0u64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 11; continue; } bb = 8; continue; }
            8 => { rax = rdx; rax = ((rax).wrapping_shr(3u32)) as u64; rcx = (crate::simd_ops::ru64(r10.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x8u64))) as u64; r8 = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 9; continue; }
            9 => { fl_l = ((r8 as u8)) as u8 as u64; fl_r = (0xfu8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { fl_l = (crate::simd_ops::lk_add_u32(rcx.wrapping_add(0x18u64), (1) as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(r8)) as u64; crate::simd_ops::wu64(rcx.wrapping_add(0x10u64), rax); bb = 13; continue; }
            11 => { r8 = 0x10; rcx = r10; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 12; continue; }
            12 => { r8 = rax; bb = 13; continue; }
            13 => { fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 15; continue; } bb = 14; continue; }
            14 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rax; rdx = 0xdeadd00d; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); bb = 16; continue; }
            15 => { rbp = r8; bb = 16; continue; }
            16 => { rbx = (crate::simd_ops::ru64(r14)) as u64; rax = (crate::simd_ops::ru32(r14.wrapping_add(0x8u64))) as u32 as u64; rsi = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rsi = ((rsi).wrapping_shl(5u32)) as u64; rsi = ((rsi).wrapping_add(rbx)) as u64; fl_l = rbx; fl_r = rsi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 20; continue; } bb = 17; continue; }
            17 => { crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), rdi); rdi = rbp.wrapping_add(0x10); bb = 18; continue; }
            18 => { rax = (crate::simd_ops::ru64(rbx)) as u64; r8 = rbx.wrapping_add(0x10); crate::simd_ops::wu64(rdi.wrapping_sub(0x10u64), rax); rdx = rsp.wrapping_add(0x60); crate::simd_ops::wu8(rdi.wrapping_add(0x40u64), (0xffu8) as u8); rcx = (((crate::simd_ops::ru8(rbx.wrapping_add(0x50u64))) as i8 as i64) as u64) as u64; rcx = ((rcx).wrapping_add(1)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rdi); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rdi = ((rdi).wrapping_add(0x60u64)) as u64; rbx = ((rbx).wrapping_add(0x60u64)) as u64; fl_l = rbx; fl_r = rsi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 18; continue; } bb = 19; continue; }
            19 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; bb = 20; continue; }
            20 => { rcx = r14; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; crate::simd_ops::wu64(r14, rbp); crate::simd_ops::wu32(r14.wrapping_add(0x8u64), ((r12 as u32)) as u32); crate::simd_ops::wu32(r14.wrapping_add(0xcu64), ((r15 as u32)) as u32); rsp = ((rsp).wrapping_add(0x30u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411cc3d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142537280; const P1: u64 = 0x142535070; const P2: u64 = 0x1426456c8; const P3: u64 = 0x1426453a0; const P4: u64 = 0x1410c4e50; const P5: u64 = 0x1410c0950;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rax = ((crate::exe::tls_pointer() as u64)) as u64; r15 = rdx; r12 = (crate::simd_ops::ru32(rcx.wrapping_add(0x8u64))) as u32 as u64; rdx = ((rdx.wrapping_add(rdx.wrapping_mul(4))) as u64) as u64; r10 = 0x6348; rbp = 0; fl_l = 0; fl_r = 0; rdx = ((rdx).wrapping_shl(4u32)) as u64; r14 = rcx; r8 = (crate::simd_ops::ru64(rax)) as u64; fl_l = ((rdx) & ((rdx).wrapping_sub(1))) as u64; rax = ((fl_l as u64)) as u64; r10 = (crate::simd_ops::ru64(r10.wrapping_add(r8))) as u64; if (((fl_l as u64)) != 0) { bb = 5; continue; } bb = 1; continue; }
            1 => { rax = rdx.wrapping_sub(0x10); fl_l = rax; fl_r = (0x3f0u64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 5; continue; } bb = 2; continue; }
            2 => { rax = rdx; rax = ((rax).wrapping_shr(3u32)) as u64; r9 = (crate::simd_ops::ru64(r10.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x8u64))) as u64; r8 = (crate::simd_ops::ru64(r9.wrapping_add(0x10u64))) as u64; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { r9 = 0; fl_l = 0; fl_r = 0; rcx = r10; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); bb = 12; continue; }
            4 => { fl_l = (crate::simd_ops::lk_add_u32(r9.wrapping_add(0x18u64), (1) as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(r8)) as u64; crate::simd_ops::wu64(r9.wrapping_add(0x10u64), rax); bb = 13; continue; }
            5 => { rax = (0x7fffffffffffffffu64) as u64; fl_l = rdx; fl_r = rax; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 7; continue; } bb = 6; continue; }
            6 => { r8 = rbp; bb = 13; continue; }
            7 => { rax = rdx.wrapping_sub(0x10); fl_l = rax; fl_r = (0x3f0u64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 11; continue; } bb = 8; continue; }
            8 => { rax = rdx; rax = ((rax).wrapping_shr(3u32)) as u64; rcx = (crate::simd_ops::ru64(r10.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x8u64))) as u64; r8 = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 9; continue; }
            9 => { fl_l = ((r8 as u8)) as u8 as u64; fl_r = (0xfu8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { fl_l = (crate::simd_ops::lk_add_u32(rcx.wrapping_add(0x18u64), (1) as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(r8)) as u64; crate::simd_ops::wu64(rcx.wrapping_add(0x10u64), rax); bb = 13; continue; }
            11 => { r8 = 0x10; rcx = r10; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 12; continue; }
            12 => { r8 = rax; bb = 13; continue; }
            13 => { fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 15; continue; } bb = 14; continue; }
            14 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rax; rdx = 0xdeadd00d; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); bb = 16; continue; }
            15 => { rbp = r8; bb = 16; continue; }
            16 => { rbx = (crate::simd_ops::ru64(r14)) as u64; rdi = rbp; rax = (crate::simd_ops::ru32(r14.wrapping_add(0x8u64))) as u32 as u64; rsi = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rsi = ((rsi).wrapping_shl(4u32)) as u64; rsi = ((rsi).wrapping_add(rbx)) as u64; fl_l = rbx; fl_r = rsi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 18; continue; } bb = 17; continue; }
            17 => { crate::simd_ops::wu8(rdi.wrapping_add(0x40u64), (0xffu8) as u8); rdx = rsp.wrapping_add(0x60); rcx = (((crate::simd_ops::ru8(rbx.wrapping_add(0x40u64))) as i8 as i64) as u64) as u64; r8 = rbx; rcx = ((rcx).wrapping_add(1)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rdi); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rdi = ((rdi).wrapping_add(0x50u64)) as u64; rbx = ((rbx).wrapping_add(0x50u64)) as u64; fl_l = rbx; fl_r = rsi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 17; continue; } bb = 18; continue; }
            18 => { rcx = r14; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; crate::simd_ops::wu64(r14, rbp); rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; crate::simd_ops::wu32(r14.wrapping_add(0x8u64), ((r12 as u32)) as u32); crate::simd_ops::wu32(r14.wrapping_add(0xcu64), ((r15 as u32)) as u32); rsp = ((rsp).wrapping_add(0x30u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411cc6b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143857980; const P1: u64 = 0x143857970; const P2: u64 = 0x143857968; const P3: u64 = 0x1411cc860;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rcx = ((crate::simd_ops::ru8(r8)) as u32) as u32 as u64; rax = (0xcbf29ce484222325u64) as u64; r10 = (crate::simd_ops::ru64(P0)) as u64; rcx = (((rcx) ^ (rax))) as u64; rax = ((crate::simd_ops::ru8(r8.wrapping_add(0x1u64))) as u32) as u32 as u64; r14 = rdx; r11 = (crate::simd_ops::ru64(P1)) as u64; rdx = (0x100000001b3u64) as u64; rdi = (crate::simd_ops::ru64(P2)) as u64; rsi = r8; rcx = (((rcx) as i64).wrapping_mul((rdx) as i64) as u64) as u64; r15 = 0; fl_l = 0; fl_r = 0; rcx = (((rcx) ^ (rax))) as u64; rbx = ((r15 as u32)) as u32 as u64; rax = ((crate::simd_ops::ru8(r8.wrapping_add(0x2u64))) as u32) as u32 as u64; rcx = (((rcx) as i64).wrapping_mul((rdx) as i64) as u64) as u64; rcx = (((rcx) ^ (rax))) as u64; rax = ((crate::simd_ops::ru8(r8.wrapping_add(0x3u64))) as u32) as u32 as u64; rcx = (((rcx) as i64).wrapping_mul((rdx) as i64) as u64) as u64; rcx = (((rcx) ^ (rax))) as u64; rax = ((crate::simd_ops::ru8(r8.wrapping_add(0x4u64))) as u32) as u32 as u64; rcx = (((rcx) as i64).wrapping_mul((rdx) as i64) as u64) as u64; rcx = (((rcx) ^ (rax))) as u64; rax = ((crate::simd_ops::ru8(r8.wrapping_add(0x5u64))) as u32) as u32 as u64; rcx = (((rcx) as i64).wrapping_mul((rdx) as i64) as u64) as u64; rcx = (((rcx) ^ (rax))) as u64; rax = ((crate::simd_ops::ru8(r8.wrapping_add(0x6u64))) as u32) as u32 as u64; rcx = (((rcx) as i64).wrapping_mul((rdx) as i64) as u64) as u64; rcx = (((rcx) ^ (rax))) as u64; rax = ((crate::simd_ops::ru8(r8.wrapping_add(0x7u64))) as u32) as u32 as u64; rcx = (((rcx) as i64).wrapping_mul((rdx) as i64) as u64) as u64; rcx = (((rcx) ^ (rax))) as u64; rax = (0xde5fb9d2630458e9u64) as u64; rcx = (((rcx) as i64).wrapping_mul((rdx) as i64) as u64) as u64; let prod = (rax as u128).wrapping_mul((rcx) as u128); rax = prod as u64; rdx = (prod >> 64) as u64; fl_l = (rdx != 0) as u64; rbp = ((rdx.wrapping_add(rax)) as u64) as u64; rax = (((rbp as u8)) as u32) as u32 as u64; r8 = rbp; r8 = ((r8).wrapping_shr(7u32)) as u64; r8 = (((r8) & (r10))) as u64; fl_l = ((((rax as u8)) & (0x7fu8))) as u8 as u64; fl_r = fl_l; rax = (rax & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); rax = ((((rax as u8)) as i8 as i32) as u32) as u32 as u64; xmm2 = (((rax as u32)) as u32) as u128; xmm2 = crate::simd_ops::vpbroadcastb((xmm2 as u8)); bb = 1; continue; }
            1 => { xmm1 = (crate::simd_ops::ru128(r8.wrapping_add(rdi))) as u128; xmm0 = crate::simd_ops::vpcmpeqb(xmm1, xmm2); rax = (crate::simd_ops::vpmovmskb(xmm0) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 5; continue; } bb = 2; continue; }
            2 => { r9 = (crate::simd_ops::ru64(rsi)) as u64; bb = 3; continue; }
            3 => { let bit_in: u32 = (rax as u32); if bit_in != 0 { rcx = ((bit_in.trailing_zeros() as u32)) as u32 as u64; } fl_l = bit_in as u64; rcx = ((rcx).wrapping_add(r8)) as u64; rcx = (((rcx) & (r10))) as u64; rdx = rcx; rdx = ((rdx).wrapping_add(rdx)) as u64; fl_l = (crate::simd_ops::ru64(r11.wrapping_add(rdx.wrapping_mul(8)))) as u64; fl_r = r9; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 7; continue; } bb = 4; continue; }
            4 => { fl_l = (((rax as u32)) & (((rax as u32)).wrapping_sub(1))) as u32 as u64; rax = ((fl_l as u32)) as u32 as u64; if (((fl_l as u32)) != 0) { bb = 3; continue; } bb = 5; continue; }
            5 => { xmm0 = crate::simd_ops::vpsignb(xmm1, xmm1); rax = (crate::simd_ops::vpmovmskb(xmm0) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 8; continue; } bb = 6; continue; }
            6 => { rbx = ((rbx).wrapping_add(0x10u64)) as u64; r8 = ((r8).wrapping_add(rbx)) as u64; fl_l = (((r8) & (r10))) as u64; fl_r = fl_l; r8 = ((fl_l as u64)) as u64; bb = 1; continue; }
            7 => { r8 = (r8 & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 9; continue; }
            8 => { rdx = rbp; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rax; rcx = rax; rax = (crate::simd_ops::ru64(rsi)) as u64; r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); rdx = ((rdx).wrapping_shl(4u32)) as u64; fl_l = ((rdx).wrapping_add(crate::simd_ops::ru64(P1))) as u64; fl_r = fl_l; rdx = ((fl_l as u64)) as u64; crate::simd_ops::wu64(rdx, rax); crate::simd_ops::wu64(rdx.wrapping_add(0x8u64), r15); r11 = (crate::simd_ops::ru64(P1)) as u64; rdi = (crate::simd_ops::ru64(P2)) as u64; bb = 9; continue; }
            9 => { rax = ((rcx.wrapping_add(rdi)) as u64) as u64; rdx = rcx; crate::simd_ops::wu64(r14, rax); rax = r14; rdx = ((rdx).wrapping_shl(4u32)) as u64; rdx = ((rdx).wrapping_add(r11)) as u64; crate::simd_ops::wu64(r14.wrapping_add(0x8u64), rdx); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; crate::simd_ops::wu8(r14.wrapping_add(0x10u64), ((r8 as u8)) as u8); rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411cd1e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1411cd220; const P1: u64 = 0x141834d30;
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
    if !(((((fl_l as u8)) & ((fl_r as u8))) == 0)) { rdx = 0xc0; rcx = rdi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rax = rdi;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411cd220(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xb8; const P1: u64 = 0x142442de0; const P2: u64 = 0x142428ce0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rsi = rcx;
    rbx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    fl_l = rbx; fl_r = rbx;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rcx = rsi; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P2, rax, rcx, rdx, r8, r9, rsp); }
    rdi = (crate::simd_ops::ru64(rbx)) as u64;
    fl_l = rdi; fl_r = rdi;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rax = (crate::simd_ops::ru64(rdi)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rdi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = rdi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); }
    crate::simd_ops::wu64(rbx, (0x0u64) as u64);
    rcx = rbx;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rcx = rsi;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(P2, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411cd4a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142442de0; const P1: u64 = 0x1425371a0; const P2: u64 = 0x142533200; const P3: u64 = 0x1424287f0;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = rcx; rdi = (crate::simd_ops::ru64(rcx.wrapping_add(0xb8u64))) as u64; rbx = (crate::simd_ops::ru64(rdi)) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rdi, (0x0u64) as u64); rdi = (crate::simd_ops::ru64(rsi.wrapping_add(0xb8u64))) as u64; crate::simd_ops::wu64(rsi.wrapping_add(0xb8u64), (0x0u64) as u64); fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 3; continue; }
            3 => { rbx = (crate::simd_ops::ru64(rdi)) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); r10 = rbx.wrapping_sub(0x1); r10 = (((r10) & (0xfffffffffe000000u64))) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(r10.wrapping_add(0x100u64))) as u64; rdx = (rdx & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); rax = rbx; rax = ((rax).wrapping_sub(r10)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rcx = r10.wrapping_add(0x108); rcx = ((rcx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x4u64))) as u32 as u64; rcx = ((rcx).wrapping_sub(rax)) as u64; fl_l = ((rdx as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 9; continue; } bb = 5; continue; }
            5 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0xeu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 8; continue; } bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(rbx, rax); crate::simd_ops::wu64(rcx.wrapping_add(0x20u64), rbx); fl_l = (crate::simd_ops::lk_add_u32(rcx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 11; continue; } bb = 7; continue; }
            7 => { rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 11; continue; }
            8 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 10; continue; }
            9 => { r8 = 0; fl_l = 0; fl_r = 0; bb = 10; continue; }
            10 => { rdx = rcx; r9 = rbx; rcx = r10; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 11; continue; }
            11 => { crate::simd_ops::wu64(rdi, (0x0u64) as u64); rcx = rdi; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); bb = 12; continue; }
            12 => { rcx = rsi; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P3, rax, rcx, rdx, r8, r9, rsp); }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411cddb0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x142535070; const P2: u64 = 0x1426456c8; const P3: u64 = 0x1426453a0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x40u64)) as u64;
    rax = ((crate::exe::tls_pointer() as u64)) as u64;
    rbx = rcx;
    rcx = 0x6348;
    rdx = (crate::simd_ops::ru64(rax)) as u64;
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(rdx))) as u64;
    rdx = 0x8;
    r8 = ((rdx.wrapping_add(P0)) as u32) as u32 as u64;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    fl_l = rax; fl_r = rax;
    if !(((((fl_l as u64)) & ((fl_r as u64))) != 0)) { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rax; rdx = 0xdeadd00d; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); rax = 0; fl_l = 0; fl_r = 0; }
    crate::simd_ops::wu64(rax, (0x0u64) as u64);
    crate::simd_ops::wu64(rbx, rax);
    rax = rbx;
    rsp = ((rsp).wrapping_add(0x40u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411ce960(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143426780; const P1: u64 = 0x1434268d0; const P2: u64 = 0x14342661c; const P3: u64 = 0x1434264d8; const P4: u64 = 0x143426480; const P5: u64 = 0x143426458; const P6: u64 = 0x143426470; const P7: u64 = 0x143426554; const P8: u64 = 0x143426498; const P9: u64 = 0x143426478; const P10: u64 = 0x143426464; const P11: u64 = 0x14342644c; const P12: u64 = 0x143428760;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut xmm4: u128 = 0; let mut xmm5: u128 = 0; let mut xmm6: u128 = 0; let mut xmm8: u128 = 0; let mut xmm9: u128 = 0; let mut xmm10: u128 = 0; let mut xmm11: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x58u64)) as u64; xmm5 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(crate::simd_ops::ru32(P0))).to_bits() as u128); xmm4 = ((crate::simd_ops::ru32(P1)) as u32) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm6) as u128); rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm8) as u128); crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm9) as u128); crate::simd_ops::wu128(rsp.wrapping_add(0x10u64), (xmm10) as u128); xmm6 = (xmm5 & !0xffff_ffffu128) | ((f32::from_bits(xmm5 as u32) * f32::from_bits(xmm5 as u32)).to_bits() as u128); xmm0 = (xmm6 & !0xffff_ffffu128) | ((f32::from_bits(xmm6 as u32) * f32::from_bits(crate::simd_ops::ru32(P2))).to_bits() as u128); xmm1 = (xmm4 & !0xffff_ffffu128) | ((f32::from_bits(xmm4 as u32) - f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm8 = (xmm6 & !0xffff_ffffu128) | ((f32::from_bits(xmm6 as u32) * f32::from_bits(xmm6 as u32)).to_bits() as u128); xmm0 = (xmm8 & !0xffff_ffffu128) | ((f32::from_bits(xmm8 as u32) * f32::from_bits(crate::simd_ops::ru32(P3))).to_bits() as u128); xmm2 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) + f32::from_bits(xmm0 as u32)).to_bits() as u128); crate::simd_ops::wu128(rsp, (xmm11) as u128); xmm11 = (xmm8 & !0xffff_ffffu128) | ((f32::from_bits(xmm8 as u32) * f32::from_bits(xmm6 as u32)).to_bits() as u128); xmm1 = (xmm11 & !0xffff_ffffu128) | ((f32::from_bits(xmm11 as u32) * f32::from_bits(crate::simd_ops::ru32(P4))).to_bits() as u128); xmm3 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) - f32::from_bits(xmm1 as u32)).to_bits() as u128); xmm10 = (xmm11 & !0xffff_ffffu128) | ((f32::from_bits(xmm11 as u32) * f32::from_bits(xmm8 as u32)).to_bits() as u128); xmm1 = (xmm10 & !0xffff_ffffu128) | ((f32::from_bits(xmm10 as u32) * f32::from_bits(crate::simd_ops::ru32(P5))).to_bits() as u128); xmm9 = (xmm8 & !0xffff_ffffu128) | ((f32::from_bits(xmm8 as u32) * f32::from_bits(xmm8 as u32)).to_bits() as u128); xmm0 = (xmm9 & !0xffff_ffffu128) | ((f32::from_bits(xmm9 as u32) * f32::from_bits(crate::simd_ops::ru32(P6))).to_bits() as u128); xmm2 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) + f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm0 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) - f32::from_bits(xmm1 as u32)).to_bits() as u128); xmm1 = (xmm6 & !0xffff_ffffu128) | ((f32::from_bits(xmm6 as u32) * f32::from_bits(crate::simd_ops::ru32(P0))).to_bits() as u128); xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x40u64))) as u128; xmm5 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm5 as u32)).to_bits() as u128); xmm0 = (xmm8 & !0xffff_ffffu128) | ((f32::from_bits(xmm8 as u32) * f32::from_bits(crate::simd_ops::ru32(P7))).to_bits() as u128); xmm8 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; xmm2 = (xmm4 & !0xffff_ffffu128) | ((f32::from_bits(xmm4 as u32) - f32::from_bits(xmm1 as u32)).to_bits() as u128); xmm1 = (xmm11 & !0xffff_ffffu128) | ((f32::from_bits(xmm11 as u32) * f32::from_bits(crate::simd_ops::ru32(P8))).to_bits() as u128); xmm3 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) + f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm0 = (xmm9 & !0xffff_ffffu128) | ((f32::from_bits(xmm9 as u32) * f32::from_bits(crate::simd_ops::ru32(P9))).to_bits() as u128); xmm9 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; xmm2 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) - f32::from_bits(xmm1 as u32)).to_bits() as u128); xmm1 = (xmm10 & !0xffff_ffffu128) | ((f32::from_bits(xmm10 as u32) * f32::from_bits(crate::simd_ops::ru32(P10))).to_bits() as u128); xmm10 = (crate::simd_ops::ru128(rsp.wrapping_add(0x10u64))) as u128; xmm3 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) + f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm2 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) - f32::from_bits(xmm1 as u32)).to_bits() as u128); xmm0 = (xmm11 & !0xffff_ffffu128) | ((f32::from_bits(xmm11 as u32) * f32::from_bits(xmm11 as u32)).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(crate::simd_ops::ru32(P11))).to_bits() as u128); xmm11 = (crate::simd_ops::ru128(rsp)) as u128; xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) + f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm1 = (xmm5 & !0xffff_ffffu128) | ((f32::from_bits(xmm5 as u32) + f32::from_bits(xmm5 as u32)).to_bits() as u128); xmm3 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm2 as u32)).to_bits() as u128); xmm0 = (xmm5 & !0xffff_ffffu128) | ((f32::from_bits(xmm5 as u32) * f32::from_bits(xmm5 as u32)).to_bits() as u128); xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) * f32::from_bits(xmm2 as u32)).to_bits() as u128); xmm1 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) - f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm0 = (xmm3) ^ (crate::simd_ops::ru128(P12)); crate::simd_ops::wu32(rcx.wrapping_add(0xcu64), ((xmm0 as u32)) as u32); crate::simd_ops::wu32(rcx, ((xmm1 as u32)) as u32); crate::simd_ops::wu32(rcx.wrapping_add(0x4u64), ((xmm3 as u32)) as u32); crate::simd_ops::wu32(rcx.wrapping_add(0x10u64), ((xmm1 as u32)) as u32); crate::simd_ops::wu32(rcx.wrapping_add(0x8u64), ((rax as u32)) as u32); crate::simd_ops::wu64(rcx.wrapping_add(0x14u64), rax); crate::simd_ops::wu32(rcx.wrapping_add(0x1cu64), ((rax as u32)) as u32); crate::simd_ops::wu32(rcx.wrapping_add(0x20u64), (0x3f800000u32) as u32); rsp = ((rsp).wrapping_add(0x58u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411cf5e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143881620; const P1: u64 = 0x14243b0b0; const P2: u64 = 0x1426d4580;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; r8 = (crate::simd_ops::ru64(P0)) as u64; rbx = rdx; rdx = rcx; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0x1u32) as u32); rdi = rcx; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x1u32) as u32); rcx = rbx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x338u64))) as u32 as u64; r11 = P2; r9 = 0x4; rax = (!((rax as u32))) as u32 as u64; r8 = ((r9 as u32)) as u32 as u64; r10 = rdi; bb = 1; continue; }
            1 => { rcx = ((crate::simd_ops::ru8(r10)) as u32) as u32 as u64; r10 = r10.wrapping_add(0x1); rdx = (((rax as u8)) as u32) as u32 as u64; rdx = (((rdx) ^ (rcx))) as u64; rax = (((rax as u32)).wrapping_shr(8u32)) as u32 as u64; rax = ((((rax as u32)) ^ (crate::simd_ops::ru32(r11.wrapping_add(rdx.wrapping_mul(4)))))) as u32 as u64; fl_l = r8; fl_r = (0x1u64) as u64; r8 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 2; continue; }
            2 => { rcx = ((rax as u32)) as u32 as u64; r10 = rdi.wrapping_add(0x4); rcx = (!((rcx as u32))) as u32 as u64; r8 = r9; crate::simd_ops::wu32(rbx.wrapping_add(0x338u64), ((rcx as u32)) as u32); bb = 3; continue; }
            3 => { rcx = ((crate::simd_ops::ru8(r10)) as u32) as u32 as u64; r10 = r10.wrapping_add(0x1); rdx = (((rax as u8)) as u32) as u32 as u64; rdx = (((rdx) ^ (rcx))) as u64; rax = (((rax as u32)).wrapping_shr(8u32)) as u32 as u64; rax = ((((rax as u32)) ^ (crate::simd_ops::ru32(r11.wrapping_add(rdx.wrapping_mul(4)))))) as u32 as u64; fl_l = r8; fl_r = (0x1u64) as u64; r8 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 4; continue; }
            4 => { rcx = ((rax as u32)) as u32 as u64; r8 = rdi.wrapping_add(0x8); rcx = (!((rcx as u32))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x338u64), ((rcx as u32)) as u32); bb = 5; continue; }
            5 => { rcx = ((crate::simd_ops::ru8(r8)) as u32) as u32 as u64; r8 = r8.wrapping_add(0x1); rdx = (((rax as u8)) as u32) as u32 as u64; rdx = (((rdx) ^ (rcx))) as u64; rax = (((rax as u32)).wrapping_shr(8u32)) as u32 as u64; rax = ((((rax as u32)) ^ (crate::simd_ops::ru32(r11.wrapping_add(rdx.wrapping_mul(4)))))) as u32 as u64; fl_l = r9; fl_r = (0x1u64) as u64; r9 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 5; continue; } bb = 6; continue; }
            6 => { rax = (!((rax as u32))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x338u64), ((rax as u32)) as u32); rdx = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = ((rdx.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_add(rcx)) as u64; rcx = (crate::simd_ops::ru32(rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_sub(0x8u64))) as u32 as u64; rax = ((rdx.wrapping_sub(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x8u64), ((rax as u32)) as u32); rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rbx.wrapping_add(0x328u64), ((rcx as u32)) as u32); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411cf850(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143881618; const P1: u64 = 0x143881620; const P2: u64 = 0x1401e9b60; const P3: u64 = 0x369; const P4: u64 = 0x328; const P5: u64 = 0x338; const P6: u64 = 0x120; const P7: u64 = 0x4; const P8: u64 = 0x8; const P9: u64 = 0x1401e9a20;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; r9 = (crate::simd_ops::ru32(P0)) as u32 as u64; rbx = rdx; r8 = (crate::simd_ops::ru64(P1)) as u64; rdx = rcx; rdi = rcx; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0x1u32) as u32); rcx = rbx; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x1u32) as u32); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); r9 = 0; fl_l = 0; fl_r = 0; r8 = ((rbx.wrapping_add(P3)) as u64) as u64; fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(P4))) as u32 as u64; fl_r = 0x1; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 2; continue; } bb = 1; continue; }
            1 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P5))) as u64; rax = (crate::simd_ops::ru64(rdx.wrapping_add(P6))) as u64; rcx = (crate::simd_ops::ru32(rax)) as u32 as u64; crate::simd_ops::wu32(rdi, ((rcx as u32)) as u32); fl_l = (crate::simd_ops::lk_add_u64(rdx.wrapping_add(P6), (0x4u64) as u64)) as u64; fl_r = fl_l; bb = 4; continue; }
            2 => { fl_l = (crate::simd_ops::ru8(r8)) as u8 as u64; fl_r = ((r9 as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 4; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu32(rdi, ((r9 as u32)) as u32); bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(P4))) as u32 as u64; fl_r = 0x1; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 6; continue; } bb = 5; continue; }
            5 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P5))) as u64; rax = (crate::simd_ops::ru64(rdx.wrapping_add(P6))) as u64; rcx = (crate::simd_ops::ru32(rax)) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(P7), ((rcx as u32)) as u32); fl_l = (crate::simd_ops::lk_add_u64(rdx.wrapping_add(P6), (0x4u64) as u64)) as u64; fl_r = fl_l; bb = 8; continue; }
            6 => { fl_l = (crate::simd_ops::ru8(r8)) as u8 as u64; fl_r = ((r9 as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 8; continue; } bb = 7; continue; }
            7 => { crate::simd_ops::wu32(rdi.wrapping_add(P7), ((r9 as u32)) as u32); bb = 8; continue; }
            8 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(P4))) as u32 as u64; fl_r = 0x1; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 10; continue; } bb = 9; continue; }
            9 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P5))) as u64; rax = (crate::simd_ops::ru64(rdx.wrapping_add(P6))) as u64; rcx = (crate::simd_ops::ru32(rax)) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(P8), ((rcx as u32)) as u32); fl_l = (crate::simd_ops::lk_add_u64(rdx.wrapping_add(P6), (0x4u64) as u64)) as u64; fl_r = fl_l; bb = 12; continue; }
            10 => { fl_l = (crate::simd_ops::ru8(r8)) as u8 as u64; fl_r = ((r9 as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 12; continue; } bb = 11; continue; }
            11 => { crate::simd_ops::wu32(rdi.wrapping_add(P8), ((r9 as u32)) as u32); bb = 12; continue; }
            12 => { rcx = rbx; rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rax = 0; fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411cfe90(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143881618; const P1: u64 = 0x143881620; const P2: u64 = 0x1401e9770; const P3: u64 = 0x338; const P4: u64 = 0x120; const P5: u64 = 0x4; const P6: u64 = 0x8; const P7: u64 = 0x1401e9590;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; r9 = (crate::simd_ops::ru32(P0)) as u32 as u64; rdi = rdx; r8 = (crate::simd_ops::ru64(P1)) as u64; rdx = rcx; rbx = rcx; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0x1u32) as u32); rcx = rdi; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x1u32) as u32); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); r8 = (crate::simd_ops::ru64(rdi.wrapping_add(P3))) as u64; rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; rcx = (crate::simd_ops::ru64(r8.wrapping_add(P4))) as u64; crate::simd_ops::wu32(rcx, ((rax as u32)) as u32); crate::simd_ops::lk_add_u64(r8.wrapping_add(P4), (0x4u64) as u64); rdx = (crate::simd_ops::ru64(rdi.wrapping_add(P3))) as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_add(P5))) as u32 as u64; rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P4))) as u64; crate::simd_ops::wu32(rcx, ((rax as u32)) as u32); crate::simd_ops::lk_add_u64(rdx.wrapping_add(P4), (0x4u64) as u64); rdx = (crate::simd_ops::ru64(rdi.wrapping_add(P3))) as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_add(P6))) as u32 as u64; rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P4))) as u64; crate::simd_ops::wu32(rcx, ((rax as u32)) as u32); rcx = rdi; crate::simd_ops::lk_add_u64(rdx.wrapping_add(P4), (0x4u64) as u64); rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rax = 0; fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}
