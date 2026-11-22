#![allow(unused_imports, unused_variables, unused_mut, unused_assignments, non_snake_case, dead_code, clippy::all, unreachable_code, unused_parens)]

#[inline(never)]
pub unsafe extern "win64" fn fn_1415f01e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415ffc90; const P1: u64 = 0x143931c10; const P2: u64 = 0x143931c18; const P3: u64 = 0x143931c1c; const P4: u64 = 0x141524340; const P5: u64 = 0x1415edf30; const P6: u64 = 0x141524330; const P7: u64 = 0x1415242e0; const P8: u64 = 0x14183acc8; const P9: u64 = 0x1415eebd0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = ((crate::simd_ops::ru8(rcx.wrapping_add(0x60u64))) as u32) as u32 as u64; r15 = 0x1; rax = (rax & !0xffu64) | (((!((rax as u8))) as u8 as u64)); rsi = rcx; fl_l = ((r15 as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((r15 as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 10; continue; } bb = 2; continue; }
            2 => { rdi = (crate::simd_ops::ru64(P1)) as u64; rdx = (crate::simd_ops::ru32(P2)) as u32 as u64; rbx = rdi; rcx = ((rdi.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64; fl_l = rdi; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 7; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(0xe8u64))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0xe8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0xe8u64))) as u64; rdi = ((rax as u32)) as u32 as u64; rdx = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rdx.wrapping_add(0xe8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; rdi = (crate::simd_ops::ru64(P1)) as u64; if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { bb = 6; continue; } bb = 4; continue; }
            4 => { rdx = (crate::simd_ops::ru32(P2)) as u32 as u64; rbx = ((rbx).wrapping_add(0x8u64)) as u64; rax = ((rdi.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 5; continue; }
            5 => { bb = 7; continue; }
            6 => { rdx = (crate::simd_ops::ru32(P2)) as u32 as u64; bb = 7; continue; }
            7 => { r14 = (crate::simd_ops::ru32(P3)) as u32 as u64; rbx = ((rbx).wrapping_sub(rdi)) as u64; rbx = ((((rbx) as i64).wrapping_shr(3u32) as u64)) as u64; rbp = ((rdx as u32)) as u32 as u64; fl_l = rbp; fl_r = r14; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 20; continue; } bb = 8; continue; }
            8 => { rax = ((r14 as u32)) as u32 as u64; rcx = 0x5; rax = (((rax as u32)).wrapping_shr(1u32)) as u32 as u64; rax = (((rax as u32)).wrapping_add((r14 as u32))) as u32 as u64; fl_l = ((r14 as u32)) as u32 as u64; fl_r = ((r14 as u32)) as u32 as u64; rax = (if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { (r15 as u32) } else { (rax as u32) }) as u32 as u64; r14 = (((r14 as u32)).wrapping_add((rax as u32))) as u32 as u64; rdx = ((r14 as u32)) as u32 as u64; rdx = ((rdx).wrapping_shl(3u32)) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 9; continue; }
            9 => { r8 = rdx; rdx = rdi; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rdi = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 19; continue; } bb = 10; continue; }
            10 => { rdx = (((r15 as u8)) as u32) as u32 as u64; rcx = rsi; rbx = 0x2; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rax = ((rbx as u32)) as u32 as u64; bb = 11; continue; }
            11 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            12 => { rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rdi = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 13; continue; }
            13 => { rdx = (crate::simd_ops::ru64(P1)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 19; continue; } bb = 14; continue; }
            14 => { fl_l = rdx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 19; continue; } bb = 15; continue; }
            15 => { rcx = 0; fl_l = 0; fl_r = 0; fl_l = rbp; fl_r = rbp; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 18; continue; } bb = 16; continue; }
            16 => { bb = 17; continue; }
            17 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(rcx.wrapping_mul(8)))) as u64; crate::simd_ops::wu64(rdi.wrapping_add(rcx.wrapping_mul(8)), rax); rcx = ((rcx).wrapping_add(1)) as u64; rdx = (crate::simd_ops::ru64(P1)) as u64; fl_l = rcx; fl_r = rbp; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 17; continue; } bb = 18; continue; }
            18 => { rcx = 0x5; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); bb = 19; continue; }
            19 => { rdx = (crate::simd_ops::ru32(P2)) as u32 as u64; crate::simd_ops::wu64(P1, rdi); crate::simd_ops::wu32(P3, ((r14 as u32)) as u32); bb = 20; continue; }
            20 => { rax = ((r14 as u32)) as u32 as u64; fl_l = rbp; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 10; continue; } bb = 21; continue; }
            21 => { rax = ((rbx as u32)) as u32 as u64; r14 = ((rax.wrapping_mul(8)) as u64) as u64; rax = ((r14.wrapping_add(rdi)) as u64) as u64; fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 23; continue; } bb = 22; continue; }
            22 => { rdx = (((rdx as u32)).wrapping_sub((rbx as u32))) as u32 as u64; rcx = rax.wrapping_add(0x8); rdx = (((rdx as u32)).wrapping_shl(3u32)) as u32 as u64; r8 = ((rdx as u32)) as u32 as u64; rdx = rax; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru32(P2)) as u32 as u64; rdi = (crate::simd_ops::ru64(P1)) as u64; bb = 23; continue; }
            23 => { rdx = (((rdx as u32)).wrapping_add(1)) as u32 as u64; rcx = ((r14.wrapping_add(rdi)) as u64) as u64; crate::simd_ops::wu32(P2, ((rdx as u32)) as u32); fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 24; continue; }
            24 => { crate::simd_ops::wu64(rcx, rsi); rcx = rsi; rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); rax = ((r15 as u32)) as u32 as u64; bb = 11; continue; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415f03c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x14163ad80; const P2: u64 = 0x38; const P3: u64 = 0x70; const P4: u64 = 0x80; const P5: u64 = 0x1415e9250; const P6: u64 = 0x1415f1800; const P7: u64 = 0x1415b17c0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rdi = rdx; rcx = rbx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64; fl_r = (0x0u64) as u64; rcx = ((rbx.wrapping_add(P3)) as u64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 5; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rcx)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru64(rax.wrapping_add(P4))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 5; continue; } bb = 3; continue; }
            3 => { rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rdx = rax; rcx = rbx; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            5 => { rcx = rdi; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = rdi; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415f0430(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x143931c00; const P2: u64 = 0x1416377d0; const P3: u64 = 0x14268de78; const P4: u64 = 0x1415b1e50; const P5: u64 = 0x14268df08; const P6: u64 = 0x1415b1720; const P7: u64 = 0x14163ad80; const P8: u64 = 0x38; const P9: u64 = 0x70; const P10: u64 = 0x80; const P11: u64 = 0x1415e9250; const P12: u64 = 0x1415f1800; const P13: u64 = 0x1415b17c0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x50u64)) as u64; r8 = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; rdi = rdx; rax = (crate::simd_ops::ru64(P1)) as u64; rdx = rsp.wrapping_add(0x40); rbx = (crate::simd_ops::ru64(rax.wrapping_add(r8.wrapping_mul(8)))) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; fl_r = (0xffffffffffffffffu64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 4; continue; } bb = 1; continue; }
            1 => { r9 = 0x8; crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), (0x1u32) as u32); rax = P3; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0x0u32) as u32); rdx = rsp.wrapping_add(0x60); crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), (0xffffffffffffffffu64) as u64); rcx = rsp.wrapping_add(0x68); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); r8 = ((r9.wrapping_add(P0)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rcx = P5; crate::simd_ops::wu64(rax.wrapping_add(P0), rbx); crate::simd_ops::wu64(rax, rcx); bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rdx = rsp.wrapping_add(0x68); r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; r8 = 0x1; rcx = rdi; crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), (0xffffffffffffffffu64) as u64); rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rax = rdi; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rsp = ((rsp).wrapping_add(0x50u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            4 => { rcx = rbx; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(P8))) as u64; fl_r = (0x0u64) as u64; rcx = ((rbx.wrapping_add(P9)) as u64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 9; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rcx)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { fl_l = (crate::simd_ops::ru64(rax.wrapping_add(P10))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 9; continue; } bb = 7; continue; }
            7 => { rax = crate::simd_ops::call_stk(P11, rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { rdx = rax; rcx = rbx; rax = crate::simd_ops::call_stk(P12, rax, rcx, rdx, r8, r9, rsp); bb = 9; continue; }
            9 => { rcx = rdi; rax = crate::simd_ops::call_stk(P13, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rax = rdi; rsp = ((rsp).wrapping_add(0x50u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415f0550(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143931bea; const P1: u64 = 0x143931c08; const P2: u64 = 0x1415f1ab0;
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rdx;
    crate::simd_ops::wu8(P0, (0x0u8) as u8);
    rdx = (crate::simd_ops::ru32(P1)) as u32 as u64;
    rcx = rbx;
    rdx = (((rdx as u32)).wrapping_sub(1)) as u32 as u64;
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    rax = rbx;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415f0580(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141600c70; const P1: u64 = 0x141643e80; const P2: u64 = 0x141600a30; const P3: u64 = 0x1415b17c0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; r15 = rdx; r14 = rcx; bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(r14.wrapping_add(0x8u64))) as u64; r8 = 0xffffffff; let xa_src: u32 = (r8 as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rax, xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; r8 = (xa_old) as u32 as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = 0x1; r8 = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((((fl_l as u32)).wrapping_sub((fl_r as u32))) as i32) < 0) { bb = 21; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(r14.wrapping_add(0x10u64))) as u64; rcx = ((((r8 as u32)) as i32 as i64) as u64) as u64; rbp = (crate::simd_ops::ru64(rax.wrapping_add(rcx.wrapping_mul(8)))) as u64; fl_l = (crate::simd_ops::ru32(rbp.wrapping_add(0x158u64))) as u32 as u64; fl_r = 0x2b; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 1; continue; } bb = 3; continue; }
            3 => { rdi = rbp.wrapping_add(0x128); rbx = 0x4; rsi = ((rbx as u32)) as u32 as u64; bb = 4; continue; }
            4 => { rbx = (((rbx as u32)).wrapping_sub(1)) as u32 as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_add(rbx.wrapping_mul(8)).wrapping_add(0xb0u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = rdi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x78u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x30u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x2b; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 9; continue; } bb = 6; continue; }
            6 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x2d; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 8; continue; } bb = 7; continue; }
            7 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x11; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 1; continue; } bb = 8; continue; }
            8 => { rbx = ((rsi as u32)) as u32 as u64; bb = 12; continue; }
            9 => { rsi = ((rbx as u32)) as u32 as u64; fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 4; continue; } bb = 10; continue; }
            10 => { rdx = rdi; rcx = rbp; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x30u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x2d; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 12; continue; } bb = 11; continue; }
            11 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x11; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 1; continue; } bb = 12; continue; }
            12 => { rsi = ((rbx as u32)) as u32 as u64; fl_l = ((rbx as u32)) as u32 as u64; fl_r = 0x4; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 20; continue; } bb = 13; continue; }
            13 => { rax = ((rbx as u32)) as u32 as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0xb0u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 14; continue; }
            14 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = rdi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x80u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x30u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x2b; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 18; continue; } bb = 15; continue; }
            15 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x2d; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 17; continue; } bb = 16; continue; }
            16 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x11; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 1; continue; } bb = 17; continue; }
            17 => { fl_l = (((rbx as u32)).wrapping_add(1)) as u32 as u64; rbx = ((fl_l as u32)) as u32 as u64; bb = 12; continue; }
            18 => { fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 10; continue; } bb = 19; continue; }
            19 => { bb = 4; continue; }
            20 => { rcx = rbp.wrapping_add(0xd0); rdx = rdi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rdx = rdi; rcx = rbp; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 1; continue; }
            21 => { rcx = r15; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rax = r15; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415f06d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415f1390; const P1: u64 = 0x1415b17c0;
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rdx;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rcx = rbx;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rax = rbx;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415f06f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x1415f1ab0;
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rdx;
    rdx = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64;
    rcx = rbx;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rax = rbx;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415f0750(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143931c10; const P1: u64 = 0x143931c18; const P2: u64 = 0x141601420; const P3: u64 = 0x1416015e0; const P4: u64 = 0x141601470; const P5: u64 = 0x1416003b0; const P6: u64 = 0x1415ffc90; const P7: u64 = 0x1415cfbb0; const P8: u64 = 0x141601300; const P9: u64 = 0x1416026f0; const P10: u64 = 0x14183acc8; const P11: u64 = 0x1415edf30;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = (crate::simd_ops::ru64(P0)) as u64; rax = (crate::simd_ops::ru32(P1)) as u32 as u64; rcx = ((rbx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 34; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbp); rbp = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rdi); bb = 2; continue; }
            2 => { rdi = (crate::simd_ops::ru64(rbx)) as u64; rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x158u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x2c; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 6; continue; } bb = 3; continue; }
            3 => { rcx = rdi; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x2e; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            5 => { rdx = rdi.wrapping_add(0x128); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(0xe8u64))) as u64; rdx = (crate::simd_ops::ru32(rdi.wrapping_add(0x100u64))) as u32 as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; rax = rcx.wrapping_add(0x304); fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 10; continue; } bb = 7; continue; }
            7 => { fl_l = (crate::simd_ops::ru8(rax)) as u8 as u64; fl_r = (0x10u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(0x378u64))) as u32 as u64; fl_r = 0xffffffff; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 10; continue; } bb = 9; continue; }
            9 => { rcx = (rcx & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 11; continue; }
            10 => { rcx = (rcx & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 11; continue; }
            11 => { fl_l = (crate::simd_ops::ru8(rax)) as u8 as u64; fl_r = (0x40u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 14; continue; } bb = 12; continue; }
            12 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 14; continue; } bb = 13; continue; }
            13 => { rax = (((rdx as u8)) as u32) as u32 as u64; bb = 15; continue; }
            14 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 15; continue; }
            15 => { rdx = (crate::simd_ops::ru32(rdi.wrapping_add(0x158u64))) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x11; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 21; continue; } bb = 16; continue; }
            16 => { fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 25; continue; } bb = 17; continue; }
            17 => { rsi = (crate::simd_ops::ru64(rdi.wrapping_add(0xf0u64))) as u64; rcx = rdi; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 26; continue; } bb = 18; continue; }
            18 => { rdx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rdi.wrapping_add(0xf0u64), rbp); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); r9 = 0; fl_l = 0; fl_r = 0; r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); rdx = rsi; rcx = rdi; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 25; continue; } bb = 19; continue; }
            19 => { rcx = rdi; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 25; continue; } bb = 20; continue; }
            20 => { rcx = rsi; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); bb = 27; continue; }
            21 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 25; continue; } bb = 22; continue; }
            22 => { fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 25; continue; } bb = 23; continue; }
            23 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 27; continue; } bb = 24; continue; }
            24 => { rcx = rdi; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); bb = 27; continue; }
            25 => { rcx = rdi; bb = 26; continue; }
            26 => { rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); bb = 27; continue; }
            27 => { fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x100u64))) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 31; continue; } bb = 28; continue; }
            28 => { rcx = (crate::simd_ops::ru32(P1)) as u32 as u64; rax = (crate::simd_ops::ru64(P0)) as u64; rcx = (((rcx as u32)).wrapping_sub(1)) as u32 as u64; rdx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rdx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 30; continue; } bb = 29; continue; }
            29 => { rdx = ((rdx).wrapping_sub(rbx)) as u64; rcx = rbx; rdx = ((((rdx as u32)) & (0xfffffff8u32))) as u32 as u64; r8 = ((rdx as u32)) as u32 as u64; rdx = rbx.wrapping_add(0x8); rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp); bb = 30; continue; }
            30 => { crate::simd_ops::lk_sub_u32(P1, (1) as u32); rdx = 0; fl_l = 0; fl_r = 0; rcx = rdi; rax = crate::simd_ops::call_stk(P11, rax, rcx, rdx, r8, r9, rsp); bb = 32; continue; }
            31 => { fl_l = ((rbx).wrapping_add(0x8u64)) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; bb = 32; continue; }
            32 => { rcx = (crate::simd_ops::ru32(P1)) as u32 as u64; rax = (crate::simd_ops::ru64(P0)) as u64; rcx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 33; continue; }
            33 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; bb = 34; continue; }
            34 => { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415f08e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x38; const P1: u64 = 0x14162ee80; const P2: u64 = 0x141634020; const P3: u64 = 0x20;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rsi = r8; rdi = rdx; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 1; continue; }
            1 => { bb = 2; continue; }
            2 => { rcx = rbx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru64(rax)) as u64; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rcx = rbx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = rax; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rsi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 6; continue; } bb = 4; continue; }
            4 => { rbx = (crate::simd_ops::ru64(rbx.wrapping_add(P3))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 5; continue; }
            5 => { rax = 0; fl_l = 0; fl_r = 0; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            6 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415f0950(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x143931c00; const P1: u64 = 0x143931c08; const P2: u64 = 0x70; const P3: u64 = 0x8; const P4: u64 = 0x78; const P5: u64 = 0x10; const P6: u64 = 0x80;
    let (mut rax, mut r10, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = (crate::simd_ops::ru64(P0)) as u64; r8 = rcx; rdx = (crate::simd_ops::ru32(P1)) as u32 as u64; r9 = ((rax.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64; fl_l = rax; fl_r = r9; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 6; continue; } bb = 1; continue; }
            1 => { r10 = (crate::simd_ops::ru64(rcx)) as u64; bb = 2; continue; }
            2 => { rdx = (crate::simd_ops::ru64(rax)) as u64; fl_l = (crate::simd_ops::ru64(rdx.wrapping_add(P2))) as u64; fl_r = r10; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 5; continue; } bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(r8.wrapping_add(P3))) as u64; fl_l = (crate::simd_ops::ru64(rdx.wrapping_add(P4))) as u64; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru64(r8.wrapping_add(P5))) as u64; fl_l = (crate::simd_ops::ru64(rdx.wrapping_add(P6))) as u64; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 7; continue; } bb = 5; continue; }
            5 => { rax = ((rax).wrapping_add(0x8u64)) as u64; fl_l = rax; fl_r = r9; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 6; continue; }
            6 => { rax = 0; fl_l = 0; fl_r = 0; return rax; }
            7 => { rax = rdx; return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415f09a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142689534; const P1: u64 = 0x143930fa0; const P2: u64 = 0x14153ad70; const P3: u64 = 0x143814fe4;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = rsp; crate::simd_ops::wu64(rax.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rax.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x50u64)) as u64; r9 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu8(rax.wrapping_sub(0x20u64), (0xffu8) as u8); crate::simd_ops::wu64(rax.wrapping_sub(0x38u64), r9); rdi = r8; r8 = (crate::simd_ops::ru32(P0)) as u32 as u64; rsi = rdx; crate::simd_ops::wu32(rax.wrapping_sub(0x30u64), ((r9 as u32)) as u32); rbx = rcx; crate::simd_ops::wu32(rax.wrapping_sub(0x28u64), ((r9 as u32)) as u32); crate::simd_ops::wu64(rax.wrapping_sub(0x10u64), r9); crate::simd_ops::wu8(rax.wrapping_sub(0x18u64), (0xffu8) as u8); rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x58u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rax)) as u64; fl_l = (((rax) >> (((r8) as u32 & 63))) & 1) as u64; if ((fl_l as u64) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rdx = rcx.wrapping_add(0x48); rcx = (crate::simd_ops::ru64(P1)) as u64; r9 = rsp.wrapping_add(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 10; continue; }
            3 => { r8 = (crate::simd_ops::ru64(rcx.wrapping_add(0x90u64))) as u64; rax = (crate::simd_ops::ru32(P3)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x60u64), ((rax as u32)) as u32); fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 4; continue; }
            4 => { r10 = ((crate::simd_ops::ru8(r8)) as u32) as u32 as u64; rcx = ((r9 as u32)) as u32 as u64; bb = 5; continue; }
            5 => { rdx = ((rcx.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(r8))) as u8 as u64; fl_r = (0x1du8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 8; continue; } bb = 6; continue; }
            6 => { rcx = ((rdx as u32)) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((r10 as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 5; continue; } bb = 7; continue; }
            7 => { bb = 9; continue; }
            8 => { rax = ((r10.wrapping_add(0x4u64)) as u32) as u32 as u64; rdx = 0xfffffffc; rax = (((rax) & (rdx))) as u64; fl_l = ((rax).wrapping_add(r8)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; r9 = ((rax.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64; bb = 9; continue; }
            9 => { fl_l = r9; fl_r = r9; rax = rsp.wrapping_add(0x60); rax = (if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { r9 } else { rax }) as u64; xmm0 = ((crate::simd_ops::ru32(rax)) as u32) as u128; bb = 10; continue; }
            10 => { crate::simd_ops::wu32(rsi, ((xmm0 as u32)) as u32); rax = ((crate::simd_ops::ru8(rbx.wrapping_add(0x19cu64))) as u32) as u32 as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rax = (rax & !0xffu64) | (((((rax as u8)).wrapping_shr(3u32)) as u8 as u64)); rax = (rax & !0xffu64) | ((((((rax as u8)) & (0x1u8))) as u8 as u64)); crate::simd_ops::wu8(rdi, ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(rbx.wrapping_add(0x19cu64))) as u32) as u32 as u64; rcx = (((rax as u8)) as u32) as u32 as u64; rax = (rax & !0xffu64) | (((((rax as u8)).wrapping_shr(4u32)) as u8 as u64)); rcx = (rcx & !0xffu64) | ((((((rcx as u8)) & (0xefu8))) as u8 as u64)); crate::simd_ops::wu8(rbx.wrapping_add(0x19cu64), ((rcx as u8)) as u8); rax = (rax & !0xffu64) | ((((((rax as u8)) & (0x1u8))) as u8 as u64)); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rsp = ((rsp).wrapping_add(0x50u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415f0aa0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14268952c; const P1: u64 = 0x143930fa0; const P2: u64 = 0x14153ad70; const P3: u64 = 0x143814fdc; const P4: u64 = 0x142689530; const P5: u64 = 0x143814fe0;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = rsp; crate::simd_ops::wu64(rax.wrapping_add(0x18u64), rbx); crate::simd_ops::wu64(rax.wrapping_add(0x20u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x50u64)) as u64; rbx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rax.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rax.wrapping_sub(0x48u64), rbx); r14 = r8; r8 = (crate::simd_ops::ru32(P0)) as u32 as u64; rbp = rdx; crate::simd_ops::wu32(rax.wrapping_sub(0x40u64), ((rbx as u32)) as u32); rdi = rcx; crate::simd_ops::wu32(rax.wrapping_sub(0x38u64), ((rbx as u32)) as u32); r15 = 0xfffffffc; crate::simd_ops::wu64(rax.wrapping_sub(0x20u64), rbx); crate::simd_ops::wu8(rax.wrapping_sub(0x30u64), (0xffu8) as u8); crate::simd_ops::wu8(rax.wrapping_sub(0x28u64), (0xffu8) as u8); rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x58u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rax)) as u64; fl_l = (((rax) >> (((r8) as u32 & 63))) & 1) as u64; if ((fl_l as u64) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rsi = rcx.wrapping_add(0x48); rcx = (crate::simd_ops::ru64(P1)) as u64; rdx = rsi; r9 = rsp.wrapping_add(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 9; continue; }
            3 => { r8 = (crate::simd_ops::ru64(rcx.wrapping_add(0x90u64))) as u64; rax = (crate::simd_ops::ru32(P3)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x70u64), ((rax as u32)) as u32); fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 4; continue; }
            4 => { r9 = ((crate::simd_ops::ru8(r8)) as u32) as u32 as u64; rcx = ((rbx as u32)) as u32 as u64; bb = 5; continue; }
            5 => { rdx = ((rcx.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(r8))) as u8 as u64; fl_r = (0x1bu8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 12; continue; } bb = 6; continue; }
            6 => { rcx = ((rdx as u32)) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 5; continue; } bb = 7; continue; }
            7 => { rdx = rbx; bb = 8; continue; }
            8 => { fl_l = rdx; fl_r = rdx; rax = rsp.wrapping_add(0x70); rsi = rdi.wrapping_add(0x48); rax = (if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { rdx } else { rax }) as u64; xmm0 = ((crate::simd_ops::ru32(rax)) as u32) as u128; bb = 9; continue; }
            9 => { crate::simd_ops::wu32(rbp, ((xmm0 as u32)) as u32); rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x58u64))) as u64; r8 = (crate::simd_ops::ru32(P4)) as u32 as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbx); crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rbx as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((rbx as u32)) as u32); crate::simd_ops::wu8(rsp.wrapping_add(0x38u64), (0xffu8) as u8); crate::simd_ops::wu8(rsp.wrapping_add(0x40u64), (0xffu8) as u8); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rbx); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 10; continue; }
            10 => { rax = (crate::simd_ops::ru64(rax)) as u64; fl_l = (((rax) >> (((r8) as u32 & 63))) & 1) as u64; if ((fl_l as u64) == 0) { bb = 13; continue; } bb = 11; continue; }
            11 => { rcx = (crate::simd_ops::ru64(P1)) as u64; r9 = rsp.wrapping_add(0x20); rdx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 20; continue; }
            12 => { rax = ((r9.wrapping_add(0x4u64)) as u32) as u32 as u64; rax = (((rax) & (r15))) as u64; fl_l = ((rax).wrapping_add(r8)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; rdx = ((rax.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64; bb = 8; continue; }
            13 => { r8 = (crate::simd_ops::ru64(rdi.wrapping_add(0x90u64))) as u64; rax = (crate::simd_ops::ru32(P5)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x70u64), ((rax as u32)) as u32); fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 19; continue; } bb = 14; continue; }
            14 => { r9 = ((crate::simd_ops::ru8(r8)) as u32) as u32 as u64; rcx = ((rbx as u32)) as u32 as u64; bb = 15; continue; }
            15 => { rdx = ((rcx.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(r8))) as u8 as u64; fl_r = (0x1cu8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 18; continue; } bb = 16; continue; }
            16 => { rcx = ((rdx as u32)) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 15; continue; } bb = 17; continue; }
            17 => { bb = 19; continue; }
            18 => { rax = ((r9.wrapping_add(0x4u64)) as u32) as u32 as u64; rax = (((rax) & (r15))) as u64; fl_l = ((rax).wrapping_add(r8)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; rbx = ((rax.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64; bb = 19; continue; }
            19 => { fl_l = rbx; fl_r = rbx; rax = rsp.wrapping_add(0x70); rax = (if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { rbx } else { rax }) as u64; xmm0 = ((crate::simd_ops::ru32(rax)) as u32) as u128; bb = 20; continue; }
            20 => { crate::simd_ops::wu32(r14, ((xmm0 as u32)) as u32); r11 = rsp.wrapping_add(0x50); rax = ((crate::simd_ops::ru8(rdi.wrapping_add(0x19cu64))) as u32) as u32 as u64; rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x30u64))) as u64; rcx = (((rax as u8)) as u32) as u32 as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x38u64))) as u64; rcx = (rcx & !0xffu64) | ((((((rcx as u8)) & (0xdfu8))) as u8 as u64)); rax = (rax & !0xffu64) | (((((rax as u8)).wrapping_shr(5u32)) as u8 as u64)); crate::simd_ops::wu8(rdi.wrapping_add(0x19cu64), ((rcx as u8)) as u8); fl_l = ((((rax as u8)) & (0x1u8))) as u8 as u64; fl_r = fl_l; rax = (rax & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); rsp = r11; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415f0c50(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xc; const P1: u64 = 0x8; const P2: u64 = 0x141524340; const P3: u64 = 0x141524330; const P4: u64 = 0x1415242e0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdx = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; rdi = rcx; rbp = ((rdx as u32)) as u32 as u64; rcx = 0x1; rbp = (((rbp as u32)).wrapping_shr(1u32)) as u32 as u64; rbp = (((rbp as u32)).wrapping_add((rdx as u32))) as u32 as u64; rax = (crate::simd_ops::ru64(rdi)) as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; rsi = (crate::simd_ops::ru32(rdi.wrapping_add(P1))) as u32 as u64; rbp = (if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { (rcx as u32) } else { (rbp as u32) }) as u32 as u64; rcx = 0x5; rbp = (((rbp as u32)).wrapping_add((rdx as u32))) as u32 as u64; rdx = ((rbp as u32)) as u32 as u64; rdx = ((rdx).wrapping_shl(3u32)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { r8 = rdx; rdx = rax; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rbx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 10; continue; } bb = 2; continue; }
            2 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 11; continue; }
            3 => { rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rbx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rdi)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 5; continue; }
            5 => { fl_l = rax; fl_r = rbx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 10; continue; } bb = 6; continue; }
            6 => { rax = 0; fl_l = 0; fl_r = 0; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 7; continue; }
            7 => { bb = 8; continue; }
            8 => { rcx = (crate::simd_ops::ru64(rdi)) as u64; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(rax.wrapping_mul(8)))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(rax.wrapping_mul(8)), rdx); rax = ((rax).wrapping_add(1)) as u64; fl_l = rax; fl_r = rsi; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 8; continue; } bb = 9; continue; }
            9 => { rdx = (crate::simd_ops::ru64(rdi)) as u64; rcx = 0x5; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 10; continue; }
            10 => { crate::simd_ops::wu64(rdi, rbx); rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); crate::simd_ops::wu32(rdi.wrapping_add(P0), ((rbp as u32)) as u32); bb = 11; continue; }
            11 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415f0d10(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141524340; const P1: u64 = 0x141524330; const P2: u64 = 0x1415242e0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdx = (crate::simd_ops::ru32(rcx.wrapping_add(0xcu64))) as u32 as u64; rdi = rcx; rbp = ((rdx as u32)) as u32 as u64; rcx = 0x1; rbp = (((rbp as u32)).wrapping_shr(1u32)) as u32 as u64; rbp = (((rbp as u32)).wrapping_add((rdx as u32))) as u32 as u64; rsi = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; rbp = (if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { (rcx as u32) } else { (rbp as u32) }) as u32 as u64; rcx = (crate::simd_ops::ru64(rdi)) as u64; rbp = (((rbp as u32)).wrapping_add((rdx as u32))) as u32 as u64; rdx = ((rbp.wrapping_mul(2)) as u64) as u64; rdx = ((rdx).wrapping_add(rbp)) as u64; rdx = ((rdx).wrapping_shl(3u32)) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { r8 = rdx; rdx = rcx; rcx = 0x5; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 12; continue; } bb = 2; continue; }
            2 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 13; continue; }
            3 => { rcx = 0x5; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rdi)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 5; continue; }
            5 => { fl_l = rax; fl_r = rbx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 12; continue; } bb = 6; continue; }
            6 => { fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 7; continue; }
            7 => { rdx = rbx; bb = 8; continue; }
            8 => { fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { crate::simd_ops::wu32(rdx.wrapping_add(0x10u64), (0xff7fffffu32) as u32); bb = 10; continue; }
            10 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rcx = rdx; rcx = ((rcx).wrapping_sub(rbx)) as u64; xmm0 = (crate::simd_ops::ru128(rcx.wrapping_add(rax))) as u128; crate::simd_ops::wu128(rdx, (xmm0) as u128); xmm1 = ((crate::simd_ops::ru64(rcx.wrapping_add(rax).wrapping_add(0x10u64))) as u64) as u128; crate::simd_ops::wu64(rdx.wrapping_add(0x10u64), ((xmm1 as u64)) as u64); rdx = ((rdx).wrapping_add(0x18u64)) as u64; fl_l = rsi; fl_r = (0x1u64) as u64; rsi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 8; continue; } bb = 11; continue; }
            11 => { rdx = (crate::simd_ops::ru64(rdi)) as u64; rcx = 0x5; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 12; continue; }
            12 => { crate::simd_ops::wu64(rdi, rbx); rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); crate::simd_ops::wu32(rdi.wrapping_add(0xcu64), ((rbp as u32)) as u32); bb = 13; continue; }
            13 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415f0e00(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xc; const P1: u64 = 0x8; const P2: u64 = 0x141524340; const P3: u64 = 0x141524330; const P4: u64 = 0x10; const P5: u64 = 0xfffffffffffffff0; const P6: u64 = 0x1415242e0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbp = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; rbx = rcx; rax = (crate::simd_ops::ru64(rcx)) as u64; rbp = (((rbp as u32)).wrapping_add((rdx as u32))) as u32 as u64; rsi = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64; rcx = 0x5; rdx = ((rbp as u32)) as u32 as u64; rdx = ((rdx).wrapping_shl(4u32)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { r8 = rdx; rdx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 2; continue; }
            2 => { bb = 11; continue; }
            3 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 12; continue; }
            5 => { rax = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 6; continue; }
            6 => { fl_l = rax; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 11; continue; } bb = 7; continue; }
            7 => { fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 8; continue; }
            8 => { rcx = 0; fl_l = 0; fl_r = 0; bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = ((rcx.wrapping_add(P4)) as u64) as u64; xmm0 = (crate::simd_ops::ru128(rcx.wrapping_add(rax).wrapping_add(P5))) as u128; crate::simd_ops::wu128(rcx.wrapping_add(rdi).wrapping_add(P5), (xmm0) as u128); fl_l = rsi; fl_r = (0x1u64) as u64; rsi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 9; continue; } bb = 10; continue; }
            10 => { rdx = (crate::simd_ops::ru64(rbx)) as u64; rcx = 0x5; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 11; continue; }
            11 => { crate::simd_ops::wu64(rbx, rdi); rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); crate::simd_ops::wu32(rbx.wrapping_add(P0), ((rbp as u32)) as u32); bb = 12; continue; }
            12 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415f0ec0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1438152b4; const P1: u64 = 0x1415e9250; const P2: u64 = 0x141633d40; const P3: u64 = 0x141634040; const P4: u64 = 0x141633c80; const P5: u64 = 0x141639670; const P6: u64 = 0x141644270;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut fl_k: u32 = 0;
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbp) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r15) as u64); rsp = ((rsp).wrapping_sub(0x50u64)) as u64; r15 = ((crate::simd_ops::ru16(P0)) as u32) as u32 as u64; r14 = (rcx) as u64; rcx = (rdx) as u64; rbp = (r8) as u64; rdi = (rdx) as u64; rbx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rsi = (rax) as u64; rcx = (rdi) as u64; fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rax = ((rbx.wrapping_add(0x2u64)) as u32) as u32 as u64; bb = 24; continue; }
            2 => { rdx = ((rsp.wrapping_add(0x88u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = (crate::simd_ops::ru32(rax)) as u32 as u64; rax = ((rcx as u32)) as u32 as u64; rax = ((((rax as u32)) & (0xf00u32))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x300u32) as u32 as u64; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 6; continue; } bb = 3; continue; }
            3 => { fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { fl_k = 2; bb = 5; continue; } bb = 4; continue; }
            4 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x300u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 6; continue; } fl_k = 0; bb = 5; continue; }
            5 => { rdx = ((rsp.wrapping_add(0x88u64)) as u64) as u64; rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rbx = (crate::simd_ops::ru32(rax)) as u32 as u64; bb = 18; continue; }
            6 => { rdx = ((rsp.wrapping_add(0x88u64)) as u64) as u64; rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = (crate::simd_ops::ru32(rax)) as u32 as u64; fl_r = (0xf00u32) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 10; continue; } bb = 7; continue; }
            7 => { fl_l = (rbp) as u64 as u64; fl_r = (rbp) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { rbx = (crate::simd_ops::ru32(rbp.wrapping_add(0x168u64))) as u32 as u64; bb = 18; continue; }
            9 => { rbx = (crate::simd_ops::ru32(rsi.wrapping_add(0x60u64))) as u32 as u64; bb = 18; continue; }
            10 => { rdx = ((rsp.wrapping_add(0x88u64)) as u64) as u64; rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = (crate::simd_ops::ru32(rax)) as u32 as u64; rcx = ((((rcx as u32)) & (0xf00u32))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = (0xe00u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 12; continue; } bb = 11; continue; }
            11 => { fl_l = (crate::simd_ops::ru8(rsi.wrapping_add(0x568u64))) as u8 as u64; fl_r = ((rbx as u8)) as u8 as u64; fl_k = 1; bb = 17; continue; }
            12 => { rdx = ((rsp.wrapping_add(0x88u64)) as u64) as u64; rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = (crate::simd_ops::ru32(rax)) as u32 as u64; rcx = ((((rcx as u32)) & (0xf00u32))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = (0xf00u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 18; continue; } bb = 13; continue; }
            13 => { rdx = ((crate::simd_ops::ru8(rsi.wrapping_add(0x568u64))) as u32) as u32 as u64; fl_l = ((rdx as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { fl_k = 2; bb = 17; continue; } bb = 14; continue; }
            14 => { rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x98u64))) as u64; rax = ((rax).wrapping_shr(32u32)) as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 16; continue; } bb = 15; continue; }
            15 => { rbx = (0x3102u32) as u32 as u64; bb = 18; continue; }
            16 => { fl_l = ((rdx as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; fl_k = 2; bb = 17; continue; }
            17 => { rax = (0x90u32) as u32 as u64; rcx = (0x60u32) as u32 as u64; rax = (if (match fl_k { 1 => (((fl_l as u8)) == ((fl_r as u8))), 2 => ((((fl_l as u8)) & ((fl_r as u8))) == 0), _ => false }) { (rcx as u32) } else { (rax as u32) }) as u32 as u64; rbx = (crate::simd_ops::ru32(rax.wrapping_add(rsi))) as u32 as u64; bb = 18; continue; }
            18 => { rdx = (rdi) as u64; rcx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); r9 = (((r15 as u16)) as u32) as u32 as u64; r8 = ((rbx as u32)) as u32 as u64; rdx = (r14) as u64; rcx = (r14) as u64; fl_l = (rbp) as u64 as u64; fl_r = (rbp) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 21; continue; } bb = 19; continue; }
            19 => { crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (rbp) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rax) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rbx = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 23; continue; } bb = 20; continue; }
            20 => { r8 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rdx = (rbp) as u64; rcx = (r14) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 22; continue; }
            21 => { crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (0x0u64) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rax) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 22; continue; }
            22 => { rbx = ((rax as u32)) as u32 as u64; bb = 23; continue; }
            23 => { rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rax = ((rbx as u32)) as u32 as u64; bb = 24; continue; }
            24 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; rsp = ((rsp).wrapping_add(0x50u64)) as u64; r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415f1080(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415b19d0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rcx = 0x1;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1;
    rax = (rax & !0xffu64) | (((if (((fl_l as u32)) > ((fl_r as u32))) { 1u8 } else { 0u8 }) as u8 as u64));
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415f10a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143931c00; const P1: u64 = 0x143931c08;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rax = (crate::simd_ops::ru32(rdx)) as u32 as u64; rdi = rdx; rbp = ((rcx as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x5; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 3; continue; } bb = 1; continue; }
            1 => { rax = (((rax as u32)).wrapping_add(0xffffffd5u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rsi = (rsi & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 4; continue; }
            3 => { rsi = (rsi & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 4; continue; }
            4 => { rbx = (crate::simd_ops::ru64(P0)) as u64; rax = (crate::simd_ops::ru32(P1)) as u32 as u64; rcx = ((rbx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 16; continue; } bb = 5; continue; }
            5 => { crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), r14); r14 = 0; fl_l = 0; fl_r = 0; bb = 6; continue; }
            6 => { rdx = (crate::simd_ops::ru64(rbx)) as u64; rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x70u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { rcx = (crate::simd_ops::ru32(rax.wrapping_add(0x20u64))) as u32 as u64; bb = 9; continue; }
            8 => { rcx = ((r14 as u32)) as u32 as u64; bb = 9; continue; }
            9 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 14; continue; } bb = 10; continue; }
            10 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 11; continue; }
            11 => { r8 = (crate::simd_ops::ru32(rdi)) as u32 as u64; fl_l = ((rsi as u8)) as u8 as u64; fl_r = ((rsi as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 13; continue; } bb = 12; continue; }
            12 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x4u64))) as u32 as u64; rdx = rsp.wrapping_add(0x30); xmm0 = ((crate::simd_ops::ru32(rdi.wrapping_add(0x10u64))) as u32) as u128; xmm3 = ((crate::simd_ops::ru32(rdi.wrapping_add(0xcu64))) as u32) as u128; crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), ((rax as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((r14 as u32)) as u32); crate::simd_ops::wu16(rsp.wrapping_add(0x34u64), (0x101u16) as u16); rax = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((xmm0 as u32)) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 14; continue; }
            13 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = (rdx & !0xffu64) | (((0x1u8) as u8 as u64)); (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x20u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 14; continue; }
            14 => { rcx = (crate::simd_ops::ru32(P1)) as u32 as u64; rbx = ((rbx).wrapping_add(0x8u64)) as u64; rax = (crate::simd_ops::ru64(P0)) as u64; rcx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 6; continue; } bb = 15; continue; }
            15 => { r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; bb = 16; continue; }
            16 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415f1190(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143930f78; const P1: u64 = 0x1415f3790; const P2: u64 = 0x143930fa0; const P3: u64 = 0x1415c4f90;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 17>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x80u64)) as u64; rdi = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu128(rsp.wrapping_add(0x70u64), (xmm6) as u128); rbx = ((rdx as u32)) as u32 as u64; xmm6 = (xmm0) as u128; rax = ((rdi as u32)) as u32 as u64; fl_l = r8; fl_r = (0xffffffffffffffffu64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru64(P0)) as u64; rdx = r8; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru64(P2)) as u64; r8 = rsp.wrapping_add(0x40); crate::simd_ops::wu8(rsp.wrapping_add(0x30u64), ((rdi as u8)) as u8); xmm3 = (xmm6) as u128; crate::simd_ops::wu32(rsp.wrapping_add(0xa0u64), ((rbx as u32)) as u32); crate::simd_ops::wu8(rsp.wrapping_add(0xa4u64), ((rdi as u8)) as u8); rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0xa0u64))) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0x1u32) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x48u64), ((rdi as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x50u64), ((rdi as u32)) as u32); crate::simd_ops::wu8(rsp.wrapping_add(0x58u64), (0xffu8) as u8); crate::simd_ops::wu8(rsp.wrapping_add(0x60u64), (0xffu8) as u8); crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x70u64))) as u128; rsp = ((rsp).wrapping_add(0x80u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415f1230(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143931c00; const P1: u64 = 0x143931c08; const P2: u64 = 0x70; const P3: u64 = 0x20; const P4: u64 = 0x141634020; const P5: u64 = 0x58; const P6: u64 = 0x14163bee0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = (crate::simd_ops::ru64(P0)) as u64; rsi = r8; rax = (crate::simd_ops::ru32(P1)) as u32 as u64; r14 = ((rdx as u32)) as u32 as u64; rbp = ((rcx as u32)) as u32 as u64; r9 = ((rbx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = r9; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 11; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rdi); bb = 2; continue; }
            2 => { rdi = (crate::simd_ops::ru64(rbx)) as u64; rax = (crate::simd_ops::ru64(rdi.wrapping_add(P2))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru32(rax.wrapping_add(P3))) as u32 as u64; bb = 5; continue; }
            4 => { rcx = 0; fl_l = 0; fl_r = 0; bb = 5; continue; }
            5 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 9; continue; } bb = 6; continue; }
            6 => { fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { rcx = ((rdi.wrapping_add(P2)) as u64) as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = (crate::simd_ops::ru64(rsi.wrapping_add(P5))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 9; continue; } bb = 8; continue; }
            8 => { rcx = ((r14 as u32)) as u32 as u64; rdx = 0x1; rdx = (((rdx as u32)).wrapping_shl(((rcx as u32) & 31))) as u32 as u64; rcx = rdi; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); bb = 9; continue; }
            9 => { rcx = (crate::simd_ops::ru32(P1)) as u32 as u64; rbx = ((rbx).wrapping_add(0x8u64)) as u64; rax = (crate::simd_ops::ru64(P0)) as u64; rcx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 10; continue; }
            10 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; bb = 11; continue; }
            11 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415f12f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143931c00; const P1: u64 = 0x143931c08; const P2: u64 = 0x70; const P3: u64 = 0x20; const P4: u64 = 0x8; const P5: u64 = 0x1415920b0;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; r9 = (crate::simd_ops::ru64(P0)) as u64; rsi = ((r8 as u32)) as u32 as u64; r10 = (crate::simd_ops::ru32(P1)) as u32 as u64; rdi = ((rcx as u32)) as u32 as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm6) as u128); rbx = r9; xmm6 = (xmm1) as u128; rdx = ((r9.wrapping_add(r10.wrapping_mul(8))) as u64) as u64; fl_l = r9; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 9; continue; } bb = 1; continue; }
            1 => { bb = 2; continue; }
            2 => { rdx = (crate::simd_ops::ru64(rbx)) as u64; rax = (crate::simd_ops::ru64(rdx.wrapping_add(P2))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru32(rax.wrapping_add(P3))) as u32 as u64; bb = 5; continue; }
            4 => { rcx = 0; fl_l = 0; fl_r = 0; bb = 5; continue; }
            5 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 8; continue; } bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P4))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { r8 = ((rsi as u32)) as u32 as u64; xmm1 = (xmm6) as u128; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r10 = (crate::simd_ops::ru32(P1)) as u32 as u64; r9 = (crate::simd_ops::ru64(P0)) as u64; bb = 8; continue; }
            8 => { rax = ((r10 as u32)) as u32 as u64; rbx = ((rbx).wrapping_add(0x8u64)) as u64; rcx = ((r9.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 9; continue; }
            9 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415f1390(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143931c18; const P1: u64 = 0x143931c10; const P2: u64 = 0x14183acc2; const P3: u64 = 0x1416017b0; const P4: u64 = 0x1416010f0; const P5: u64 = 0x141641f90;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm0: u128 = 0; let mut xmm1: u128 = 0; let mut xmm2: u128 = 0; let mut xmm3: u128 = 0; let mut xmm6: u128 = 0;
    let mut stack = crate::simd_ops::LiftedStack::<1>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x38u64)) as u64; rbp = rsp.wrapping_add(0x20); rdx = (crate::simd_ops::ru32(P0)) as u32 as u64; r15 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu128(rbp, (xmm6) as u128); rsi = ((rdx as u32)) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x60u64), ((r15 as u32)) as u32); r8 = (0xffffffffffffff0u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x68u64), rdx); rcx = ((rdx.wrapping_mul(8)) as u64) as u64; rax = rcx.wrapping_add(0xf); fl_l = rax; fl_r = rcx; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = r8; bb = 2; continue; }
            2 => { rax = (((rax) & (0xfffffffffffffff0u64))) as u64; rsp = ((rsp).wrapping_sub(rax)) as u64; rax = rcx.wrapping_add(0xf); rdi = rsp.wrapping_add(0x20); crate::simd_ops::wu64(rbp.wrapping_add(0x70u64), rdi); fl_l = rax; fl_r = rcx; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = r8; bb = 4; continue; }
            4 => { rax = (((rax) & (0xfffffffffffffff0u64))) as u64; rsp = ((rsp).wrapping_sub(rax)) as u64; r12 = rsp.wrapping_add(0x20); crate::simd_ops::wu64(rbp.wrapping_add(0x78u64), r12); fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 23; continue; } bb = 5; continue; }
            5 => { r10 = 0; fl_l = 0; fl_r = 0; xmm6 = 0u128; r13 = ((r10 as u32)) as u32 as u64; bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(P1)) as u64; rbx = (crate::simd_ops::ru64(rax.wrapping_add(r13))) as u64; rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x158u64))) as u32 as u64; r14 = rbx.wrapping_add(0x128); fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x2d; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 8; continue; } bb = 7; continue; }
            7 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x11; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 21; continue; } bb = 8; continue; }
            8 => { fl_l = (crate::simd_ops::ru64(r14)) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 21; continue; } bb = 9; continue; }
            9 => { rdx = ((crate::simd_ops::ru16(rbx.wrapping_add(0x138u64))) as u32) as u32 as u64; rax = ((crate::simd_ops::ru16(rbx.wrapping_add(0x13au64))) as u32) as u32 as u64; r8 = ((crate::simd_ops::ru8(rbx.wrapping_add(0x130u64))) as u32) as u32 as u64; fl_l = ((rax as u16)) as u16 as u64; fl_r = ((rdx as u16)) as u16 as u64; rax = (rax & !0xffffu64) | (((if (((fl_l as u16)) >= ((fl_r as u16))) { (rdx as u16) } else { (rax as u16) }) as u16 as u64)); r9 = (((rax as u16)) as u32) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; rdx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 15; continue; } bb = 10; continue; }
            10 => { fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 14; continue; } bb = 11; continue; }
            11 => { r15 = ((rdx.wrapping_mul(4)) as u32) as u32 as u64; r12 = ((r9 as u32)) as u32 as u64; rdi = r10; rsi = ((r8 as u32)) as u32 as u64; bb = 12; continue; }
            12 => { rcx = ((crate::simd_ops::ru16(rbx.wrapping_add(0x138u64))) as u32) as u32 as u64; r8 = r15; rax = (crate::simd_ops::ru64(r14)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = (((rcx) as i64).wrapping_mul((rdi) as i64) as u64) as u64; rcx = ((rcx).wrapping_add(r12)) as u64; rcx = ((rax.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = ((rdi).wrapping_add(1)) as u64; fl_l = rsi; fl_r = (0x1u64) as u64; rsi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 12; continue; } bb = 13; continue; }
            13 => { rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x158u64))) as u32 as u64; r10 = 0; fl_l = 0; fl_r = 0; rsi = (crate::simd_ops::ru64(rbp.wrapping_add(0x68u64))) as u64; r15 = (crate::simd_ops::ru32(rbp.wrapping_add(0x60u64))) as u32 as u64; r12 = (crate::simd_ops::ru64(rbp.wrapping_add(0x78u64))) as u64; bb = 14; continue; }
            14 => { rax = ((crate::simd_ops::ru16(rbx.wrapping_add(0x138u64))) as u32) as u32 as u64; crate::simd_ops::wu16(rbx.wrapping_add(0x13au64), ((rax as u16)) as u16); bb = 15; continue; }
            15 => { rdi = rbx.wrapping_add(0x88); fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x11; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 20; continue; } bb = 16; continue; }
            16 => { fl_l = ((xmm6 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; if (f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 20; continue; } bb = 17; continue; }
            17 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x38u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 20; continue; } bb = 18; continue; }
            18 => { bb = 19; continue; }
            19 => { crate::simd_ops::wu32(rax.wrapping_add(0xcu64), ((r10 as u32)) as u32); rax = (crate::simd_ops::ru64(rax.wrapping_add(0x20u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 19; continue; } bb = 20; continue; }
            20 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x158u64))) as u32 as u64; rcx = rbx; crate::simd_ops::wu32(rbx.wrapping_add(0x134u64), ((rax as u32)) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = r14; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x70u64))) as u64; rax = ((r15 as u32)) as u32 as u64; r15 = (((r15 as u32)).wrapping_add(1)) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x60u64), ((r15 as u32)) as u32); r10 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rcx.wrapping_add(rax.wrapping_mul(8)), rdi); crate::simd_ops::wu64(r12.wrapping_add(rax.wrapping_mul(8)), r14); bb = 21; continue; }
            21 => { r13 = ((r13).wrapping_add(0x8u64)) as u64; fl_l = rsi; fl_r = (0x1u64) as u64; rsi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x68u64), rsi); if (((fl_l as u64)) != ((fl_r as u64))) { bb = 6; continue; } bb = 22; continue; }
            22 => { rdi = (crate::simd_ops::ru64(rbp.wrapping_add(0x70u64))) as u64; bb = 23; continue; }
            23 => { r9 = r12; r8 = 0; fl_l = 0; fl_r = 0; rdx = rdi; rcx = ((r15 as u32)) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm6 = (crate::simd_ops::ru128(rbp)) as u128; rsp = rbp.wrapping_add(0x18); r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415f1800(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415f2d10; const P1: u64 = 0x1414ea020; const P2: u64 = 0x1416488e0; const P3: u64 = 0x141648950; const P4: u64 = 0x1415eaa00; const P5: u64 = 0x141634020; const P6: u64 = 0x141648920;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 23>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0xa8u64)) as u64; fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0x56au64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; rbx = rdx; rdi = rcx; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 22; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu8(rdx.wrapping_add(0x56au64), (0x1u8) as u8); fl_l = (crate::simd_ops::ru16(rcx.wrapping_add(0x1b0u64))) as u16 as u64; fl_r = (0x0u16) as u16 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xd8u64), r14); r14 = rcx.wrapping_add(0x170); if (((fl_l as u16)) != ((fl_r as u16))) { bb = 8; continue; } bb = 2; continue; }
            2 => { r8 = 0x3; rax = rcx.wrapping_add(0x164); rdx = ((r8 as u32)) as u32 as u64; bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru8(rax.wrapping_sub(0x5u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 5; continue; } bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::ru16(rax)) as u16 as u64; fl_r = (0x0u16) as u16 as u64; if (((fl_l as u16)) == ((fl_r as u16))) { bb = 7; continue; } bb = 5; continue; }
            5 => { r8 = (((r8 as u32)).wrapping_sub(1)) as u32 as u64; rax = ((rax).wrapping_sub(0x30u64)) as u64; fl_l = rdx; fl_r = (0x1u64) as u64; rdx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((((fl_l as u64)).wrapping_sub((fl_r as u64))) as i64) >= 0) { bb = 3; continue; } bb = 6; continue; }
            6 => { bb = 8; continue; }
            7 => { rax = ((((r8 as u32)) as i32 as i64) as u64) as u64; r14 = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; r14 = ((r14).wrapping_shl(4u32)) as u64; r14 = ((r14).wrapping_add(0xb8u64)) as u64; fl_l = ((r14).wrapping_add(rdi)) as u64; fl_r = fl_l; r14 = ((fl_l as u64)) as u64; bb = 8; continue; }
            8 => { rax = (crate::simd_ops::ru32(r14.wrapping_add(0x8u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 21; continue; } bb = 9; continue; }
            9 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0x568u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xc0u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0xc8u64), rsi); if (((fl_l as u8)) != ((fl_r as u8))) { bb = 17; continue; } bb = 10; continue; }
            10 => { rbp = 0; fl_l = 0; fl_r = 0; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 15; continue; } bb = 11; continue; }
            11 => { rsi = (crate::simd_ops::ru64(r14)) as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x60u64))) as u32 as u64; rdx = ((rax as u32)) as u32 as u64; rcx = (crate::simd_ops::ru32(rsi.wrapping_add(0x8u64))) as u32 as u64; rdx = ((((rdx as u32)) ^ ((rcx as u32)))) as u32 as u64; fl_l = ((rdx as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 15; continue; } bb = 12; continue; }
            12 => { rdx = ((rax as u32)) as u32 as u64; rdx = ((((rdx as u32)) ^ ((rcx as u32)))) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0xf00; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 15; continue; } bb = 13; continue; }
            13 => { rax = ((((rax as u32)) ^ ((rcx as u32)))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0xfffff000; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 15; continue; } bb = 14; continue; }
            14 => { rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(r14)) as u64; r9 = (r9 & !0xffu64) | (((0x1u8) as u8 as u64)); rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; xmm2 = ((crate::simd_ops::ru32(rcx.wrapping_add(0x54u64))) as u32) as u128; xmm1 = ((crate::simd_ops::ru32(rcx.wrapping_add(0x50u64))) as u32) as u128; rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 16; continue; }
            15 => { xmm0 = (crate::simd_ops::ru128(rbx.wrapping_add(0x90u64))) as u128; rax = rsp.wrapping_add(0x30); r9 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); rsi = rbx.wrapping_add(0x4e0); rdx = rbx.wrapping_add(0xa0); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbp); r8 = rsi; rcx = rsp.wrapping_add(0x50); crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = rbx.wrapping_add(0x428); rdx = r14; rcx = rsp.wrapping_add(0x50); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 16; continue; }
            16 => { xmm1 = 0u128; crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((rbp as u32)) as u32); crate::simd_ops::wu128(rsp.wrapping_add(0x38u64), (xmm1) as u128); r9 = rsp.wrapping_add(0x30); r8 = 0; fl_l = 0; fl_r = 0; xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; rdx = rsi; rcx = rbx; xmm1 = crate::simd_ops::vunpckhpd(xmm1, xmm1); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), ((xmm1 as u64)) as u64); xmm1 = ((crate::simd_ops::ru64(rdi.wrapping_add(0x90u64))) as u64) as u128; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), ((xmm1 as u64)) as u64); crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 20; continue; }
            17 => { xmm0 = (crate::simd_ops::ru128(rbx.wrapping_add(0x90u64))) as u128; rax = rsp.wrapping_add(0x30); crate::simd_ops::wu64(rsp.wrapping_add(0xd0u64), r12); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); rsi = rbx.wrapping_add(0x540); rdx = rbx.wrapping_add(0xa0); crate::simd_ops::wu64(rsp.wrapping_add(0xa0u64), r15); r9 = rbx.wrapping_add(0x510); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); r8 = rbx.wrapping_add(0x4e0); rcx = rsp.wrapping_add(0x50); crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbp = (crate::simd_ops::ru64(rdi.wrapping_add(0x70u64))) as u64; rcx = rdi.wrapping_add(0x70); fl_l = rbp; fl_r = rbp; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 19; continue; } bb = 18; continue; }
            18 => { rbp = (crate::simd_ops::ru32(rbp.wrapping_add(0x20u64))) as u32 as u64; bb = 19; continue; }
            19 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rax; rcx = rsp.wrapping_add(0x50); r8 = ((rbp as u32)) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = rbx.wrapping_add(0x428); rdx = r14; rcx = rsp.wrapping_add(0x50); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm0 = (crate::simd_ops::ru128(rsi)) as u128; r9 = rsp.wrapping_add(0x30); rcx = rbx; xmm1 = ((crate::simd_ops::ru64(rsi.wrapping_add(0x10u64))) as u64) as u128; r8 = rbx.wrapping_add(0x510); crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128); rdx = rbx.wrapping_add(0x4e0); xmm0 = ((crate::simd_ops::ru64(rdi.wrapping_add(0x90u64))) as u64) as u128; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), ((xmm0 as u64)) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), ((xmm1 as u64)) as u64); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0xa0u64))) as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0xd0u64))) as u64; bb = 20; continue; }
            20 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0xc8u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0xc0u64))) as u64; bb = 21; continue; }
            21 => { r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0xd8u64))) as u64; bb = 22; continue; }
            22 => { rsp = ((rsp).wrapping_add(0xa8u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415f1a90(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x143931bf8;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = (crate::simd_ops::ru32(P0)) as u32 as u64; fl_r = 0x2;
    if !((((fl_l as u32)) == ((fl_r as u32)))) { rax = (((rcx as u8)) as u32) as u32 as u64; fl_l = (((rax as u32)).wrapping_add(1)) as u32 as u64; rax = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(P0, ((rax as u32)) as u32); }
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415f1ab0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143931c00; const P1: u64 = 0x14163b2e0; const P2: u64 = 0x143931be8; const P3: u64 = 0x141637700; const P4: u64 = 0x1415b17c0; const P5: u64 = 0x14268de88; const P6: u64 = 0x8; const P7: u64 = 0x1415b1e50; const P8: u64 = 0x14268df18; const P9: u64 = 0x1415b1720;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x50u64)) as u64; rbx = ((rdx as u32)) as u32 as u64; rsi = rcx; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 5; continue; } bb = 1; continue; }
            1 => { bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(P0)) as u64; rdx = ((rbx as u32)) as u32 as u64; rbx = (((rbx as u32)).wrapping_sub(1)) as u32 as u64; rdi = (crate::simd_ops::ru64(rax.wrapping_add(rdx.wrapping_mul(8)))) as u64; rcx = rdi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru8(P2)) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rdx = rsp.wrapping_add(0x40); rcx = rdi; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; fl_r = (0xffffffffffffffffu64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 6; continue; } bb = 4; continue; }
            4 => { fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) >= 0) { bb = 2; continue; } bb = 5; continue; }
            5 => { rcx = rsi; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rax = rsi; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rsp = ((rsp).wrapping_add(0x50u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            6 => { r9 = 0x8; crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), (0x1u32) as u32); rax = P5; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0x0u32) as u32); rdx = rsp.wrapping_add(0x70); crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), (0xffffffffffffffffu64) as u64); rcx = rsp.wrapping_add(0x78); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); r8 = ((r9.wrapping_add(P6)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { rcx = P8; crate::simd_ops::wu32(rax.wrapping_add(P6), ((rbx as u32)) as u32); crate::simd_ops::wu64(rax, rcx); bb = 8; continue; }
            8 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rdx = rsp.wrapping_add(0x78); r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; r8 = 0x1; rcx = rsi; crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), (0xffffffffffffffffu64) as u64); rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rax = rsi; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rsp = ((rsp).wrapping_add(0x50u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415f1bc0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143931c00; const P1: u64 = 0x143931c08; const P2: u64 = 0x70; const P3: u64 = 0x20; const P4: u64 = 0x8; const P5: u64 = 0x5;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; r8 = (crate::simd_ops::ru64(P0)) as u64; rdi = ((rcx as u32)) as u32 as u64; r9 = (crate::simd_ops::ru32(P1)) as u32 as u64; rbx = r8; rdx = ((r8.wrapping_add(r9.wrapping_mul(8))) as u64) as u64; fl_l = r8; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 9; continue; } bb = 1; continue; }
            1 => { bb = 2; continue; }
            2 => { rdx = (crate::simd_ops::ru64(rbx)) as u64; rax = (crate::simd_ops::ru64(rdx.wrapping_add(P2))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru32(rax.wrapping_add(P3))) as u32 as u64; bb = 5; continue; }
            4 => { rcx = 0; fl_l = 0; fl_r = 0; bb = 5; continue; }
            5 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 8; continue; } bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P4))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = 0; fl_l = 0; fl_r = 0; r8 = ((rdx.wrapping_add(P5)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); r9 = (crate::simd_ops::ru32(P1)) as u32 as u64; r8 = (crate::simd_ops::ru64(P0)) as u64; bb = 8; continue; }
            8 => { rax = ((r9 as u32)) as u32 as u64; rbx = ((rbx).wrapping_add(0x8u64)) as u64; rcx = ((r8.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 9; continue; }
            9 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415f1c50(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x143931c10; const P1: u64 = 0xc; const P2: u64 = 0x143931c18; const P3: u64 = 0xe8; const P4: u64 = 0x18; const P5: u64 = 0xf0; const P6: u64 = 0x100; const P7: u64 = 0x298;
    let (mut rax, mut r10, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = (crate::simd_ops::ru64(P0)) as u64; r8 = ((rcx.wrapping_add(P1)) as u64) as u64; rdx = (crate::simd_ops::ru32(P2)) as u32 as u64; r9 = (crate::simd_ops::ru64(rcx)) as u64; r10 = ((rax.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64; bb = 1; continue; }
            1 => { fl_l = rax; fl_r = r10; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 11; continue; } bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru64(rax)) as u64; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P3))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru64(rdx.wrapping_add(P4))) as u64; fl_r = r9; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 8; continue; } bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::ru8(r8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 7; continue; } bb = 5; continue; }
            5 => { rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P5))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { fl_l = (crate::simd_ops::ru64(rdx.wrapping_add(P4))) as u64; fl_r = r9; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 8; continue; } bb = 7; continue; }
            7 => { fl_l = ((rax).wrapping_add(0x8u64)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; bb = 1; continue; }
            8 => { fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(P6))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 10; continue; } bb = 9; continue; }
            9 => { fl_l = (crate::simd_ops::lk_or_u32(r8, (0x100u32) as u32)) as u32 as u64; fl_r = fl_l; bb = 10; continue; }
            10 => { rax = rcx; return rax; }
            11 => { rcx = (crate::simd_ops::ru64(r9.wrapping_add(P7))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 15; continue; } bb = 12; continue; }
            12 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(P3))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 13; continue; }
            13 => { fl_l = (crate::simd_ops::ru64(rax.wrapping_add(P4))) as u64; fl_r = r9; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 10; continue; } bb = 14; continue; }
            14 => { rax = 0; fl_l = 0; fl_r = 0; fl_l = (crate::simd_ops::ru8(r8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64; rax = (if (((fl_l as u8)) == ((fl_r as u8))) { rcx } else { rax }) as u64; return rax; }
            15 => { rax = 0; fl_l = 0; fl_r = 0; return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415f1ce0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415f0d10; const P1: u64 = 0x14183acc8;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = rsp; crate::simd_ops::wu64(rax.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rax.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rax.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rax.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; xmm0 = (crate::simd_ops::ru128(rdx)) as u128; r9 = (crate::simd_ops::ru32(rcx.wrapping_add(0x8u64))) as u32 as u64; r11 = 0; fl_l = 0; fl_r = 0; rsi = r8; r14 = rdx; crate::simd_ops::wu128(rax.wrapping_sub(0x18u64), (xmm0) as u128); rbx = rcx; fl_l = ((r9 as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 11; continue; } bb = 1; continue; }
            1 => { rbp = (crate::simd_ops::ru64(rcx)) as u64; r8 = (crate::simd_ops::ru64(rax.wrapping_sub(0x10u64))) as u64; rdx = (crate::simd_ops::ru32(rax.wrapping_sub(0x18u64))) as u32 as u64; bb = 2; continue; }
            2 => { r10 = ((r9 as u32)) as u32 as u64; r10 = (((r10 as u32)).wrapping_shr(1u32)) as u32 as u64; r10 = (((r10 as u32)).wrapping_add((r11 as u32))) as u32 as u64; rax = ((((r10 as u32)) as i32 as i64) as u64) as u64; rdi = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru32(rbp.wrapping_add(rdi.wrapping_mul(8)))) as u32 as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_add(rdi.wrapping_mul(8)).wrapping_add(0x8u64))) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 5; continue; } bb = 3; continue; }
            3 => { fl_l = rcx; fl_r = r8; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 20; continue; } bb = 4; continue; }
            4 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; bb = 5; continue; }
            5 => { if (((fl_l as u32)) < ((fl_r as u32))) { bb = 9; continue; } bb = 6; continue; }
            6 => { if (((fl_l as u32)) != ((fl_r as u32))) { bb = 8; continue; } bb = 7; continue; }
            7 => { fl_l = rcx; fl_r = r8; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 9; continue; } bb = 8; continue; }
            8 => { rcx = (rcx & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 10; continue; }
            9 => { rcx = (rcx & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 10; continue; }
            10 => { fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; rax = ((r10.wrapping_add(0x1u64)) as u32) as u32 as u64; rax = (if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { (r11 as u32) } else { (rax as u32) }) as u32 as u64; r11 = ((rax as u32)) as u32 as u64; rax = ((r9.wrapping_sub(0x1u64)) as u32) as u32 as u64; rax = (if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { (r9 as u32) } else { (rax as u32) }) as u32 as u64; r9 = ((rax as u32)) as u32 as u64; fl_l = (((r9 as u32)).wrapping_shr(1u32)) as u32 as u64; r9 = ((fl_l as u32)) as u32 as u64; if (((fl_l as u32)) != 0) { bb = 2; continue; } bb = 11; continue; }
            11 => { crate::simd_ops::wu8(rsi, (0x0u8) as u8); rsi = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; r8 = (crate::simd_ops::ru32(rbx.wrapping_add(0xcu64))) as u32 as u64; rax = ((((r11 as u32)) as i32 as i64) as u64) as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rdx = ((rcx.wrapping_mul(8)) as u64) as u64; rcx = (crate::simd_ops::ru64(rbx)) as u64; fl_l = ((rcx).wrapping_add(rdx)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; if (((fl_l as u64)) == 0) { bb = 21; continue; } bb = 12; continue; }
            12 => { rax = (0x2aaaaaaaaaaaaaabu64) as u64; let prod = (rax as i64 as i128).wrapping_mul((rdx) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rdi = rdx; rdi = ((((rdi) as i64).wrapping_shr(2u32) as u64)) as u64; rax = rdi; rax = ((rax).wrapping_shr(63u32)) as u64; rdi = ((rdi).wrapping_add(rax)) as u64; fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 14; continue; } bb = 13; continue; }
            13 => { rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 29; continue; } bb = 14; continue; }
            14 => { fl_l = ((rsi as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rbx.wrapping_add(0xcu64))) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 29; continue; } bb = 15; continue; }
            15 => { rax = ((rdi as u32)) as u32 as u64; rbp = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru64(rbx)) as u64; rsi = ((rax.wrapping_add(rbp.wrapping_mul(8))) as u64) as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 17; continue; } bb = 16; continue; }
            16 => { rax = (((rax as u32)).wrapping_sub((rdi as u32))) as u32 as u64; rcx = rsi.wrapping_add(0x18); rdx = rsi; r8 = ((rax.wrapping_add(rax.wrapping_mul(2))) as u32) as u32 as u64; r8 = (((r8 as u32)).wrapping_shl(3u32)) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 17; continue; }
            17 => { fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 19; continue; } bb = 18; continue; }
            18 => { crate::simd_ops::wu32(rsi.wrapping_add(0x10u64), (0xff7fffffu32) as u32); bb = 19; continue; }
            19 => { rax = (crate::simd_ops::ru64(rbx)) as u64; fl_l = (crate::simd_ops::lk_add_u32(rbx.wrapping_add(0x8u64), (1) as u32)) as u32 as u64; rcx = ((rax.wrapping_add(rbp.wrapping_mul(8))) as u64) as u64; bb = 26; continue; }
            20 => { crate::simd_ops::wu8(rsi, (0x1u8) as u8); rax = (crate::simd_ops::ru64(rbx)) as u64; rax = ((rax.wrapping_add(rdi.wrapping_mul(8))) as u64) as u64; bb = 30; continue; }
            21 => { fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 23; continue; } bb = 22; continue; }
            22 => { rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 29; continue; } bb = 23; continue; }
            23 => { fl_l = ((rsi as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rbx.wrapping_add(0xcu64))) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 29; continue; } bb = 24; continue; }
            24 => { rdx = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = ((rdx.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64; rcx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; rax = ((rdx.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x8u64), ((rax as u32)) as u32); fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 28; continue; } bb = 25; continue; }
            25 => { crate::simd_ops::wu32(rcx.wrapping_add(0x10u64), (0xff7fffffu32) as u32); bb = 26; continue; }
            26 => { fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 28; continue; } bb = 27; continue; }
            27 => { xmm0 = (crate::simd_ops::ru128(r14)) as u128; crate::simd_ops::wu128(rcx, (xmm0) as u128); bb = 28; continue; }
            28 => { rax = rcx; bb = 30; continue; }
            29 => { rax = 0; fl_l = 0; fl_r = 0; bb = 30; continue; }
            30 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415f1e90(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x143931c00; const P1: u64 = 0x143931c08; const P2: u64 = 0x8; const P3: u64 = 0x1f8; const P4: u64 = 0x143931c10; const P5: u64 = 0x143931c18; const P6: u64 = 0xffffffffffffffe8; const P7: u64 = 0x210;
    let (mut rax, mut r10, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r8 = (crate::simd_ops::ru64(P0)) as u64; r9 = (crate::simd_ops::ru32(P1)) as u32 as u64; rax = r8; rdx = ((r8.wrapping_add(r9.wrapping_mul(8))) as u64) as u64; fl_l = r8; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 5; continue; } bb = 1; continue; }
            1 => { bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru64(rax)) as u64; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::lk_and_u8(rdx.wrapping_add(P3), (0xfeu8) as u8)) as u8 as u64; fl_r = fl_l; r9 = (crate::simd_ops::ru32(P1)) as u32 as u64; r8 = (crate::simd_ops::ru64(P0)) as u64; bb = 4; continue; }
            4 => { rcx = ((r9 as u32)) as u32 as u64; rax = ((rax).wrapping_add(0x8u64)) as u64; rdx = ((r8.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 5; continue; }
            5 => { r8 = (crate::simd_ops::ru64(P4)) as u64; r9 = (crate::simd_ops::ru32(P5)) as u32 as u64; rax = r8; rdx = ((r8.wrapping_add(r9.wrapping_mul(8))) as u64) as u64; fl_l = r8; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 10; continue; } bb = 6; continue; }
            6 => { r10 = 0; fl_l = 0; fl_r = 0; bb = 7; continue; }
            7 => { rcx = (crate::simd_ops::ru64(rax)) as u64; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64; fl_l = rdx; fl_r = rdx; rcx = ((rdx.wrapping_add(P6)) as u64) as u64; rcx = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { r10 } else { rcx }) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { fl_l = (crate::simd_ops::lk_and_u8(rcx.wrapping_add(P7), (0xfeu8) as u8)) as u8 as u64; fl_r = fl_l; r9 = (crate::simd_ops::ru32(P5)) as u32 as u64; r8 = (crate::simd_ops::ru64(P4)) as u64; bb = 9; continue; }
            9 => { rcx = ((r9 as u32)) as u32 as u64; rax = ((rax).wrapping_add(0x8u64)) as u64; rdx = ((r8.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 7; continue; } bb = 10; continue; }
            10 => { return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415f1f40(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141643450; const P1: u64 = 0x14163cbd0; const P2: u64 = 0x141524310; const P3: u64 = 0x143932030; const P4: u64 = 0x14158e5c0; const P5: u64 = 0x143931168; const P6: u64 = 0x14158ecf0; const P7: u64 = 0x14160c320; const P8: u64 = 0x1415e4370; const P9: u64 = 0x1415e9a50; const P10: u64 = 0x143931c20; const P11: u64 = 0x1416475b0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x28u64)) as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 7; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru64(P3)) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 7; continue; } bb = 2; continue; }
            2 => { rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 8; continue; } bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(P5)) as u64; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 8; continue; } bb = 4; continue; }
            4 => { rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 8; continue; } bb = 5; continue; }
            5 => { rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 8; continue; } bb = 6; continue; }
            6 => { rdx = 0x40; rcx = P10; r8 = ((rdx as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P11, rax, rcx, rdx, r8, r9, rsp); rcx = 0x1; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; rcx = (if (((fl_l as u32)) != ((fl_r as u32))) { (rax as u32) } else { (rcx as u32) }) as u32 as u64; rax = ((rcx as u32)) as u32 as u64; rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
            7 => { rax = 0x2; bb = 8; continue; }
            8 => { rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415f20f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415e5fa0; const P1: u64 = 0x143931c58; const P2: u64 = 0x5; const P3: u64 = 0x143931c60; const P4: u64 = 0x1415242e0; const P5: u64 = 0x143931c64; const P6: u64 = 0x14160cca0; const P7: u64 = 0x143931c00; const P8: u64 = 0x143931c08; const P9: u64 = 0x143931c0c; const P10: u64 = 0x143931c10; const P11: u64 = 0x143931c18; const P12: u64 = 0x143931c1c; const P13: u64 = 0x143931c20; const P14: u64 = 0x1416473d0; const P15: u64 = 0x14158f2c0; const P16: u64 = 0x141611f20; const P17: u64 = 0x14158e7b0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru64(P1)) as u64; rbx = 0; fl_l = 0; fl_r = 0; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = ((rbx.wrapping_add(P2)) as u32) as u32 as u64; crate::simd_ops::wu32(P3, ((rbx as u32)) as u32); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(P1, rbx); crate::simd_ops::wu32(P5, ((rbx as u32)) as u32); bb = 2; continue; }
            2 => { rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru64(P7)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rcx = 0x5; crate::simd_ops::wu32(P8, ((rbx as u32)) as u32); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(P7, rbx); crate::simd_ops::wu32(P9, ((rbx as u32)) as u32); bb = 4; continue; }
            4 => { rdx = (crate::simd_ops::ru64(P10)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rcx = 0x5; crate::simd_ops::wu32(P11, ((rbx as u32)) as u32); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(P10, rbx); crate::simd_ops::wu32(P12, ((rbx as u32)) as u32); bb = 6; continue; }
            6 => { rcx = P13; rax = crate::simd_ops::call_stk(P14, rax, rcx, rdx, r8, r9, rsp); rax = crate::simd_ops::call_stk(P15, rax, rcx, rdx, r8, r9, rsp); rax = crate::simd_ops::call_stk(P16, rax, rcx, rdx, r8, r9, rsp); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P17, rax, rcx, rdx, r8, r9, rsp); }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415f21a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1416454d0; const P1: u64 = 0x14183acbc; const P2: u64 = 0x14183acc2;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedStack::<1>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((r9 as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x18u64), ((r8 as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x10u64), ((rdx as u32)) as u32); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbp = rsp.wrapping_add(0x20); r13 = ((((r9 as u32)) as i32 as i64) as u64) as u64; r12 = rcx; crate::simd_ops::wu64(rbp.wrapping_add(0x40u64), rbx); rsi = r13; rcx = (0xffffffffffffff0u64) as u64; rax = ((r13.wrapping_mul(4)) as u64) as u64; r10 = rax.wrapping_add(0xf); fl_l = r10; fl_r = rax; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { r10 = rcx; bb = 2; continue; }
            2 => { r10 = (((r10) & (0xfffffffffffffff0u64))) as u64; rax = r10; rsp = ((rsp).wrapping_sub(rax)) as u64; r14 = 0; fl_l = 0; fl_r = 0; rdi = rsp.wrapping_add(0x20); fl_l = ((r9 as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 22; continue; } bb = 3; continue; }
            3 => { r13 = (crate::simd_ops::ru32(rbp.wrapping_add(0x48u64))) as u32 as u64; rbx = 0; fl_l = 0; fl_r = 0; r15 = ((r8 as u32)) as u32 as u64; bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(r12)) as u64; rdx = ((r13 as u32)) as u32 as u64; rcx = ((r15 as u32)) as u32 as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(rcx.wrapping_mul(8)))) as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((r14 as u32)) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(rbx.wrapping_mul(4)), ((rax as u32)) as u32); r14 = (if (((fl_l as u32)) > ((fl_r as u32))) { (rax as u32) } else { (r14 as u32) }) as u32 as u64; r15 = (((r15 as u32)).wrapping_add(1)) as u32 as u64; rbx = ((rbx).wrapping_add(1)) as u64; fl_l = rbx; fl_r = rsi; if ((((fl_l as u64)) as i64) < (((fl_r as u64)) as i64)) { bb = 4; continue; } bb = 5; continue; }
            5 => { r13 = (crate::simd_ops::ru32(rbp.wrapping_add(0x58u64))) as u32 as u64; fl_l = ((r14 as u32)) as u32 as u64; fl_r = ((r14 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 22; continue; } bb = 6; continue; }
            6 => { r8 = ((rsi.wrapping_mul(8)) as u64) as u64; r15 = (0xffffffffffffff0u64) as u64; rax = r8.wrapping_add(0xf); fl_l = rax; fl_r = r8; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 8; continue; } bb = 7; continue; }
            7 => { rax = r15; bb = 8; continue; }
            8 => { rax = (((rax) & (0xfffffffffffffff0u64))) as u64; rcx = (crate::simd_ops::ru32(rbp.wrapping_add(0x50u64))) as u32 as u64; rsp = ((rsp).wrapping_sub(rax)) as u64; rax = (crate::simd_ops::ru64(r12)) as u64; rsi = rsp.wrapping_add(0x20); rdx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; rcx = rsi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); r8 = ((r14.wrapping_add(0x1u64)) as u32) as u32 as u64; r8 = ((r8).wrapping_shl(2u32)) as u64; rax = r8.wrapping_add(0xf); fl_l = rax; fl_r = r8; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 10; continue; } bb = 9; continue; }
            9 => { rax = r15; bb = 10; continue; }
            10 => { rax = (((rax) & (0xfffffffffffffff0u64))) as u64; rsp = ((rsp).wrapping_sub(rax)) as u64; rax = r8.wrapping_add(0xf); rbx = rsp.wrapping_add(0x20); fl_l = rax; fl_r = r8; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 12; continue; } bb = 11; continue; }
            11 => { rax = r15; bb = 12; continue; }
            12 => { rax = (((rax) & (0xfffffffffffffff0u64))) as u64; rsp = ((rsp).wrapping_sub(rax)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rbx; r15 = rsp.wrapping_add(0x20); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); r9 = r13; fl_l = ((r13 as u32)) as u32 as u64; fl_r = ((r13 as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 15; continue; } bb = 13; continue; }
            13 => { rcx = rdi; rdx = r13; bb = 14; continue; }
            14 => { rax = (crate::simd_ops::ru32(rcx)) as u32 as u64; rcx = rcx.wrapping_add(0x4); crate::simd_ops::lk_add_u32(rbx.wrapping_add(rax.wrapping_mul(4)), (1) as u32); fl_l = rdx; fl_r = (0x1u64) as u64; rdx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 14; continue; } bb = 15; continue; }
            15 => { rax = ((r14.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(r15, (0x0u32) as u32); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 18; continue; } bb = 16; continue; }
            16 => { r8 = r15; rdx = ((r14 as u32)) as u32 as u64; fl_l = r8; fl_r = rbx; r8 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; rcx = r15.wrapping_add(0x4); bb = 17; continue; }
            17 => { rax = (crate::simd_ops::ru32(r8.wrapping_add(rbx))) as u32 as u64; rcx = rcx.wrapping_add(0x4); rax = (((rax as u32)).wrapping_add(crate::simd_ops::ru32(rbx))) as u32 as u64; rbx = rbx.wrapping_add(0x4); crate::simd_ops::wu32(rcx.wrapping_sub(0x4u64), ((rax as u32)) as u32); fl_l = rdx; fl_r = (0x1u64) as u64; rdx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 17; continue; } bb = 18; continue; }
            18 => { fl_l = ((r13 as u32)) as u32 as u64; fl_r = ((r13 as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 21; continue; } bb = 19; continue; }
            19 => { r10 = (crate::simd_ops::ru32(rbp.wrapping_add(0x50u64))) as u32 as u64; bb = 20; continue; }
            20 => { rax = (crate::simd_ops::ru32(rdi)) as u32 as u64; rdi = rdi.wrapping_add(0x4); r8 = (crate::simd_ops::ru64(rsi)) as u64; rsi = rsi.wrapping_add(0x8); rdx = ((r15.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = (crate::simd_ops::ru32(r15.wrapping_add(rax.wrapping_mul(4)))) as u32 as u64; rcx = ((rax.wrapping_add(r10)) as u32) as u32 as u64; rax = (((rax as u32)).wrapping_add(1)) as u32 as u64; crate::simd_ops::wu32(rdx, ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(r12)) as u64; crate::simd_ops::wu64(rax.wrapping_add(rcx.wrapping_mul(8)), r8); fl_l = r9; fl_r = (0x1u64) as u64; r9 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 20; continue; } bb = 21; continue; }
            21 => { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 23; continue; }
            22 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 23; continue; }
            23 => { rbx = (crate::simd_ops::ru64(rbp.wrapping_add(0x40u64))) as u64; rsp = rbp; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415f2390(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143931be9; const P1: u64 = 0x143931c24; const P2: u64 = 0x143931c20; const P3: u64 = 0x143931c28; const P4: u64 = 0x14183acc2; const P5: u64 = 0x143931c08; const P6: u64 = 0x141647630; const P7: u64 = 0x143931c00; const P8: u64 = 0x141524330; const P9: u64 = 0x1415242e0; const P10: u64 = 0x1415f29c0; const P11: u64 = 0x143931be8; const P12: u64 = 0x1415f21a0; const P13: u64 = 0x1416476e0;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedStack::<1>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rbp = rsp.wrapping_add(0x20); fl_l = (crate::simd_ops::ru8(P0)) as u8 as u64; fl_r = (0x0u8) as u8 as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x50u64), rbx); crate::simd_ops::wu64(rbp.wrapping_add(0x58u64), rsi); crate::simd_ops::wu64(rbp.wrapping_add(0x60u64), rdi); if (((fl_l as u8)) != ((fl_r as u8))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = 0x1; bb = 68; continue; }
            2 => { r8 = (crate::simd_ops::ru32(P1)) as u32 as u64; rdx = 0; fl_l = 0; fl_r = 0; r8 = ((((r8 as u32)) as i32).wrapping_mul((crate::simd_ops::ru32(P2)) as i32) as u32) as u32 as u64; rcx = (crate::simd_ops::ru64(P3)) as u64; r8 = (((r8 as u32)).wrapping_add(0x1fu32)) as u32 as u64; r8 = (((r8 as u32)).wrapping_shr(5u32)) as u32 as u64; r8 = (((r8 as u32)).wrapping_shl(2u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rdi = (crate::simd_ops::ru32(P5)) as u32 as u64; r12 = 0x1; fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 67; continue; } bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru32(P2)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 5; continue; } bb = 4; continue; }
            4 => { r8 = ((rdi as u32)) as u32 as u64; rcx = P2; rdx = ((rdi as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((r12 as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 68; continue; } bb = 5; continue; }
            5 => { r15 = (crate::simd_ops::ru64(P7)) as u64; r14 = rdi; rcx = r15; rdx = rdi; bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rcx = rcx.wrapping_add(0x8); crate::simd_ops::wu32(rax.wrapping_add(0x1c0u64), (0xffffffffu32) as u32); fl_l = rdx; fl_r = r12; rdx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 6; continue; } bb = 7; continue; }
            7 => { r13 = 0; fl_l = 0; fl_r = 0; rsi = ((rdi.wrapping_mul(4)) as u32) as u32 as u64; crate::simd_ops::wu64(rbp, r13); crate::simd_ops::wu64(rbp.wrapping_add(0x8u64), r13); fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 18; continue; } bb = 8; continue; }
            8 => { rdx = ((rsi as u32)) as u32 as u64; rcx = ((r13.wrapping_add(0x5u64)) as u32) as u32 as u64; rdx = ((rdx).wrapping_shl(4u32)) as u64; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); rbx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { rbx = (crate::simd_ops::ru64(rbp)) as u64; bb = 14; continue; }
            10 => { rdx = (crate::simd_ops::ru64(rbp)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 11; continue; }
            11 => { fl_l = rdx; fl_r = rbx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 13; continue; } bb = 12; continue; }
            12 => { rcx = 0x5; rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); bb = 13; continue; }
            13 => { crate::simd_ops::wu64(rbp, rbx); rax = (((r12 as u8)) as u32) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0xcu64), ((rsi as u32)) as u32); bb = 14; continue; }
            14 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; rsi = 0x34; rsi = (if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { (r12 as u32) } else { (rsi as u32) }) as u32 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 18; continue; } bb = 15; continue; }
            15 => { fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 16; continue; }
            16 => { rdx = rbx; rcx = 0x5; rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); bb = 17; continue; }
            17 => { rax = ((rsi as u32)) as u32 as u64; bb = 68; continue; }
            18 => { crate::simd_ops::wu8(rbp.wrapping_add(0x14u64), ((r13 as u8)) as u8); rbx = r15; crate::simd_ops::wu32(rbp.wrapping_add(0x10u64), ((r13 as u32)) as u32); rsi = rdi; bb = 19; continue; }
            19 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(0x38u64))) as u64; fl_r = r13; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 23; continue; } bb = 20; continue; }
            20 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x70u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 22; continue; } bb = 21; continue; }
            21 => { fl_l = (crate::simd_ops::ru64(rax.wrapping_add(0x80u64))) as u64; fl_r = r13; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 23; continue; } bb = 22; continue; }
            22 => { r8 = rbp; rdx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp); bb = 23; continue; }
            23 => { rbx = ((rbx).wrapping_add(0x8u64)) as u64; fl_l = rsi; fl_r = r12; rsi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 19; continue; } bb = 24; continue; }
            24 => { rax = ((crate::simd_ops::ru8(rbp.wrapping_add(0x14u64))) as u32) as u32 as u64; rdx = (0xffffffffffffff0u64) as u64; crate::simd_ops::wu8(P11, ((rax as u8)) as u8); rax = ((rdi.wrapping_mul(8)) as u64) as u64; rcx = rax.wrapping_add(0xf); fl_l = rcx; fl_r = rax; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 26; continue; } bb = 25; continue; }
            25 => { rcx = rdx; bb = 26; continue; }
            26 => { rcx = (((rcx) & (0xfffffffffffffff0u64))) as u64; rax = rcx; rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x10u64))) as u32 as u64; rsp = ((rsp).wrapping_sub(rcx)) as u64; rax = (((rax as u32)).wrapping_add(1)) as u32 as u64; r12 = ((((rax as u32)) as i32 as i64) as u64) as u64; rbx = rsp.wrapping_add(0x20); rsi = ((r12.wrapping_mul(4)) as u64) as u64; rax = rsi.wrapping_add(0xf); fl_l = rax; fl_r = rsi; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 28; continue; } bb = 27; continue; }
            27 => { rax = rdx; bb = 28; continue; }
            28 => { rax = (((rax) & (0xfffffffffffffff0u64))) as u64; rsp = ((rsp).wrapping_sub(rax)) as u64; r8 = rsi; rdx = 0; fl_l = 0; fl_r = 0; r13 = rsp.wrapping_add(0x20); rcx = r13; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); r9 = r15; rcx = rbx; fl_l = r9; fl_r = rbx; r9 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; r8 = rdi; bb = 29; continue; }
            29 => { rax = (crate::simd_ops::ru64(r9.wrapping_add(rcx))) as u64; crate::simd_ops::wu64(rcx, rax); rdx = (((crate::simd_ops::ru32(rax.wrapping_add(0x1c0u64))) as i32 as i64) as u64) as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 31; continue; } bb = 30; continue; }
            30 => { fl_l = (crate::simd_ops::lk_add_u32(r13.wrapping_add(rdx.wrapping_mul(4)), (1) as u32)) as u32 as u64; bb = 31; continue; }
            31 => { rcx = ((rcx).wrapping_add(0x8u64)) as u64; fl_l = r8; fl_r = (0x1u64) as u64; r8 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 29; continue; } bb = 32; continue; }
            32 => { rax = rsi.wrapping_add(0xf); fl_l = rax; fl_r = rsi; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 34; continue; } bb = 33; continue; }
            33 => { rax = (0xffffffffffffff0u64) as u64; bb = 34; continue; }
            34 => { rax = (((rax) & (0xfffffffffffffff0u64))) as u64; rsp = ((rsp).wrapping_sub(rax)) as u64; r10 = 0x1; r9 = rsp.wrapping_add(0x20); crate::simd_ops::wu32(r9, (0x0u32) as u32); fl_l = r12; fl_r = r10; if ((((fl_l as u64)) as i64) <= (((fl_r as u64)) as i64)) { bb = 37; continue; } bb = 35; continue; }
            35 => { r8 = r9; rdx = ((r10 as u32)) as u32 as u64; fl_l = r8; fl_r = r13; r8 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; rcx = r13; bb = 36; continue; }
            36 => { rax = (crate::simd_ops::ru32(r8.wrapping_add(rcx))) as u32 as u64; rdx = ((rdx).wrapping_add(1)) as u64; rax = (((rax as u32)).wrapping_add(crate::simd_ops::ru32(rcx))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(rcx).wrapping_add(0x4u64), ((rax as u32)) as u32); rcx = rcx.wrapping_add(0x4); fl_l = rdx; fl_r = r12; if ((((fl_l as u64)) as i64) < (((fl_r as u64)) as i64)) { bb = 36; continue; } bb = 37; continue; }
            37 => { r8 = ((rdi as u32)) as u32 as u64; bb = 38; continue; }
            38 => { rdx = (crate::simd_ops::ru64(rbx)) as u64; rax = (((crate::simd_ops::ru32(rdx.wrapping_add(0x1c0u64))) as i32 as i64) as u64) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 40; continue; } bb = 39; continue; }
            39 => { rcx = (((crate::simd_ops::ru32(r9.wrapping_add(rax.wrapping_mul(4)))) as i32 as i64) as u64) as u64; crate::simd_ops::wu64(r15.wrapping_add(rcx.wrapping_mul(8)), rdx); rax = (((crate::simd_ops::ru32(rdx.wrapping_add(0x1c0u64))) as i32 as i64) as u64) as u64; fl_l = (crate::simd_ops::lk_add_u32(r9.wrapping_add(rax.wrapping_mul(4)), (1) as u32)) as u32 as u64; bb = 41; continue; }
            40 => { fl_l = (((r8 as u32)).wrapping_sub(1)) as u32 as u64; r8 = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu64(r15.wrapping_add(r8.wrapping_mul(8)), rdx); bb = 41; continue; }
            41 => { rbx = ((rbx).wrapping_add(0x8u64)) as u64; fl_l = r14; fl_r = (0x1u64) as u64; r14 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 38; continue; } bb = 42; continue; }
            42 => { r9 = ((r9).wrapping_sub(r13)) as u64; fl_l = (crate::simd_ops::ru8(P11)) as u8 as u64; fl_r = ((r14 as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 48; continue; } bb = 43; continue; }
            43 => { r14 = ((r10 as u32)) as u32 as u64; fl_l = r12; fl_r = (0x1u64) as u64; if ((((fl_l as u64)) as i64) <= (((fl_r as u64)) as i64)) { bb = 48; continue; } bb = 44; continue; }
            44 => { rsi = r10; rbx = r13.wrapping_add(0x4); r15 = r9.wrapping_sub(0x4); bb = 45; continue; }
            45 => { r9 = (crate::simd_ops::ru32(rbx)) as u32 as u64; fl_l = ((r9 as u32)) as u32 as u64; fl_r = 0x1; if ((((fl_l as u32)) as i32) <= (((fl_r as u32)) as i32)) { bb = 47; continue; } bb = 46; continue; }
            46 => { r8 = (crate::simd_ops::ru32(r15.wrapping_add(rbx))) as u32 as u64; rcx = P7; rdx = ((r14 as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P12, rax, rcx, rdx, r8, r9, rsp); bb = 47; continue; }
            47 => { r14 = (((r14 as u32)).wrapping_add(1)) as u32 as u64; rsi = ((rsi).wrapping_add(1)) as u64; rbx = ((rbx).wrapping_add(0x4u64)) as u64; fl_l = rsi; fl_r = r12; if ((((fl_l as u64)) as i64) < (((fl_r as u64)) as i64)) { bb = 45; continue; } bb = 48; continue; }
            48 => { r11 = (crate::simd_ops::ru64(rbp)) as u64; rsi = (crate::simd_ops::ru32(rbp.wrapping_add(0x8u64))) as u32 as u64; rbx = r11; rax = ((rsi as u32)) as u32 as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rax = ((rax).wrapping_add(r11)) as u64; fl_l = r11; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 61; continue; } bb = 49; continue; }
            49 => { r9 = (crate::simd_ops::ru64(P7)) as u64; bb = 50; continue; }
            50 => { rdx = 0; fl_l = 0; fl_r = 0; fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 60; continue; } bb = 51; continue; }
            51 => { r10 = (crate::simd_ops::ru64(rbx)) as u64; bb = 52; continue; }
            52 => { fl_l = (crate::simd_ops::ru64(r9.wrapping_add(rdx.wrapping_mul(8)))) as u64; fl_r = r10; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 57; continue; } bb = 53; continue; }
            53 => { r8 = ((rdx.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { bb = 57; continue; } bb = 54; continue; }
            54 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; bb = 55; continue; }
            55 => { fl_l = (crate::simd_ops::ru64(r9.wrapping_add(r8.wrapping_mul(8)))) as u64; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 59; continue; } bb = 56; continue; }
            56 => { r8 = (((r8 as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 55; continue; } bb = 57; continue; }
            57 => { rdx = (((rdx as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { bb = 60; continue; } bb = 58; continue; }
            58 => { bb = 52; continue; }
            59 => { rcx = P2; rax = crate::simd_ops::call_stk(P13, rax, rcx, rdx, r8, r9, rsp); r11 = (crate::simd_ops::ru64(rbp)) as u64; r9 = (crate::simd_ops::ru64(P7)) as u64; rsi = (crate::simd_ops::ru32(rbp.wrapping_add(0x8u64))) as u32 as u64; bb = 60; continue; }
            60 => { rax = ((rsi as u32)) as u32 as u64; rbx = ((rbx).wrapping_add(0x10u64)) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rax = ((rax).wrapping_add(r11)) as u64; fl_l = rbx; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 50; continue; } bb = 61; continue; }
            61 => { rdx = 0; fl_l = 0; fl_r = 0; rsi = ((rdi.wrapping_sub(0x1u64)) as u32) as u32 as u64; bb = 62; continue; }
            62 => { rcx = (crate::simd_ops::ru32(r13)) as u32 as u64; rax = ((rcx.wrapping_sub(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; rcx = (if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { (rdi as u32) } else { (rcx as u32) }) as u32 as u64; rcx = (((rcx as u32)).wrapping_sub(1)) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { bb = 64; continue; } bb = 63; continue; }
            63 => { rbx = ((rdx.wrapping_add(0x1u64)) as u32) as u32 as u64; r8 = ((rbx as u32)) as u32 as u64; rcx = P2; rax = crate::simd_ops::call_stk(P13, rax, rcx, rdx, r8, r9, rsp); rdx = ((rbx as u32)) as u32 as u64; bb = 62; continue; }
            64 => { rdx = (crate::simd_ops::ru64(rbp)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 66; continue; } bb = 65; continue; }
            65 => { rcx = 0x5; crate::simd_ops::wu32(rbp.wrapping_add(0x8u64), (0x0u32) as u32); rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); bb = 66; continue; }
            66 => { r12 = 0x1; bb = 67; continue; }
            67 => { crate::simd_ops::wu8(P0, (0x0u8) as u8); rax = ((r12 as u32)) as u32 as u64; bb = 68; continue; }
            68 => { rbx = (crate::simd_ops::ru64(rbp.wrapping_add(0x50u64))) as u64; rsi = (crate::simd_ops::ru64(rbp.wrapping_add(0x58u64))) as u64; rdi = (crate::simd_ops::ru64(rbp.wrapping_add(0x60u64))) as u64; rsp = rbp.wrapping_add(0x20); r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415f27a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143931c10; const P1: u64 = 0x143931c18; const P2: u64 = 0xffffffffffffffff; const P3: u64 = 0x8; const P4: u64 = 0x14183acc8; const P5: u64 = 0x1415edf30; const P6: u64 = 0x143931c08; const P7: u64 = 0x143931c00; const P8: u64 = 0x1415e7ac0; const P9: u64 = 0x1415242e0; const P10: u64 = 0x14160c1c0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; r8 = (crate::simd_ops::ru64(P0)) as u64; rdx = (crate::simd_ops::ru32(P1)) as u32 as u64; rbx = r8; rcx = ((r8.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64; fl_l = r8; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 5; continue; } bb = 1; continue; }
            1 => { bb = 2; continue; }
            2 => { rdi = (crate::simd_ops::ru64(rbx)) as u64; rax = ((rdx.wrapping_add(P2)) as u32) as u32 as u64; rax = ((r8.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = ((rax).wrapping_sub(rbx)) as u64; rdx = ((rbx.wrapping_add(P3)) as u64) as u64; rax = ((((rax as u32)) & (0xfffffff8u32))) as u32 as u64; rcx = rbx; r8 = ((rax as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru32(P1)) as u32 as u64; bb = 4; continue; }
            4 => { rdx = (((rdx as u32)).wrapping_sub(1)) as u32 as u64; rcx = rdi; crate::simd_ops::wu32(P1, ((rdx as u32)) as u32); rdx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru32(P1)) as u32 as u64; r8 = (crate::simd_ops::ru64(P0)) as u64; rax = ((r8.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru32(P6)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; rbx = ((((rax as u32)) as i32 as i64) as u64) as u64; if (((((fl_l as u32)).wrapping_sub((fl_r as u32))) as i32) < 0) { bb = 9; continue; } bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(P7)) as u64; rdi = (crate::simd_ops::ru64(rax.wrapping_add(rbx.wrapping_mul(8)))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { rcx = rdi; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); rdx = rdi; rcx = 0x5; rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); bb = 8; continue; }
            8 => { fl_l = rbx; fl_r = (0x1u64) as u64; rbx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((((fl_l as u64)).wrapping_sub((fl_r as u64))) as i64) >= 0) { bb = 6; continue; } bb = 9; continue; }
            9 => { crate::simd_ops::wu32(P6, (0x0u32) as u32); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P10, rax, rcx, rdx, r8, r9, rsp); }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415f2880(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x143931bf8;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru32(P0)) as u32 as u64;
    rcx = 0x2;
    fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64;
    rax = (if (((fl_l as u32)) != ((fl_r as u32))) { (rcx as u32) } else { (rax as u32) }) as u32 as u64;
    crate::simd_ops::wu32(P0, ((rax as u32)) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415f28a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143931b60; const P1: u64 = 0x38; const P2: u64 = 0x30; const P3: u64 = 0x1415ebeb0; const P4: u64 = 0x28;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = (crate::simd_ops::ru64(P0)) as u64; rbp = ((r8 as u32)) as u32 as u64; rsi = ((rdx as u32)) as u32 as u64; rdi = ((rcx as u32)) as u32 as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(P1))) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 4; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(P2))) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rdx = ((rbp as u32)) as u32 as u64; rcx = rbx; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { rbx = (crate::simd_ops::ru64(rbx.wrapping_add(P4))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 1; continue; } bb = 5; continue; }
            5 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415f2900(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x143931b60; const P1: u64 = 0x38; const P2: u64 = 0x30; const P3: u64 = 0x10; const P4: u64 = 0x14; const P5: u64 = 0x28;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = (crate::simd_ops::ru64(P0)) as u64; r9 = ((rcx as u32)) as u32 as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 1; continue; }
            1 => { bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru32(rax.wrapping_add(P1))) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 5; continue; } bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru32(rax.wrapping_add(P2))) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru32(rax.wrapping_add(P3))) as u32 as u64; crate::simd_ops::wu32(rax.wrapping_add(P4), ((rcx as u32)) as u32); crate::simd_ops::wu32(rax.wrapping_add(P3), ((r8 as u32)) as u32); bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rax.wrapping_add(P5))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 6; continue; }
            6 => { return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415f2930(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143931c00; const P1: u64 = 0x143931c08; const P2: u64 = 0x70; const P3: u64 = 0x20; const P4: u64 = 0x14163bee0;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; r9 = (crate::simd_ops::ru64(P0)) as u64; rsi = ((rdx as u32)) as u32 as u64; r10 = (crate::simd_ops::ru32(P1)) as u32 as u64; rdi = ((rcx as u32)) as u32 as u64; rbx = r9; r8 = ((r9.wrapping_add(r10.wrapping_mul(8))) as u64) as u64; fl_l = r9; fl_r = r8; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 8; continue; } bb = 1; continue; }
            1 => { bb = 2; continue; }
            2 => { r8 = (crate::simd_ops::ru64(rbx)) as u64; rax = (crate::simd_ops::ru64(r8.wrapping_add(P2))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru32(rax.wrapping_add(P3))) as u32 as u64; bb = 5; continue; }
            4 => { rcx = 0; fl_l = 0; fl_r = 0; bb = 5; continue; }
            5 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rcx = ((rsi as u32)) as u32 as u64; rdx = 0x1; rdx = (((rdx as u32)).wrapping_shl(((rcx as u32) & 31))) as u32 as u64; rcx = r8; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); r10 = (crate::simd_ops::ru32(P1)) as u32 as u64; r9 = (crate::simd_ops::ru64(P0)) as u64; bb = 7; continue; }
            7 => { rax = ((r10 as u32)) as u32 as u64; rbx = ((rbx).wrapping_add(0x8u64)) as u64; rcx = ((r9.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 8; continue; }
            8 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415f29c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415f0e00; const P1: u64 = 0x1415f29c0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; crate::simd_ops::lk_or_u8(rcx.wrapping_add(0x1c4u64), (0x1u8) as u8); rdi = r8; r8 = ((rdx as u32)) as u32 as u64; r15 = ((rdx as u32)) as u32 as u64; r14 = rcx; fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x10u64))) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; r8 = (if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { crate::simd_ops::ru32(rdi.wrapping_add(0x10u64)) } else { (r8 as u32) }) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x10u64), ((r8 as u32)) as u32); crate::simd_ops::wu32(rcx.wrapping_add(0x1c0u64), ((rdx as u32)) as u32); rbx = (crate::simd_ops::ru64(rcx.wrapping_add(0x20u64))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 15; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), r12); r12 = 0x1; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rbp); bb = 2; continue; }
            2 => { rsi = (crate::simd_ops::ru64(rbx.wrapping_add(0x150u64))) as u64; fl_l = (crate::simd_ops::ru8(rsi.wrapping_add(0x60u64))) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 13; continue; } bb = 3; continue; }
            3 => { rbp = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; rcx = ((rbp as u32)) as u32 as u64; rax = (crate::simd_ops::ru32(rdi.wrapping_add(0xcu64))) as u32 as u64; fl_l = ((rbp as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 6; continue; } bb = 4; continue; }
            4 => { rdx = ((rax as u32)) as u32 as u64; rcx = rdi; rdx = (((rdx as u32)).wrapping_shr(1u32)) as u32 as u64; rdx = (((rdx as u32)).wrapping_add((rax as u32))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; rdx = (if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { (r12 as u32) } else { (rdx as u32) }) as u32 as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 9; continue; } bb = 5; continue; }
            5 => { rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; bb = 6; continue; }
            6 => { fl_l = ((rbp as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rdi.wrapping_add(0xcu64))) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 9; continue; } bb = 7; continue; }
            7 => { rax = ((rcx.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x8u64), ((rax as u32)) as u32); rax = ((rcx as u32)) as u32 as u64; rax = ((rax).wrapping_shl(4u32)) as u64; fl_l = ((rax).wrapping_add(crate::simd_ops::ru64(rdi))) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; if (((fl_l as u64)) == 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { crate::simd_ops::wu64(rax, r14); crate::simd_ops::wu64(rax.wrapping_add(0x8u64), rsi); bb = 9; continue; }
            9 => { rax = ((crate::simd_ops::ru8(rsi.wrapping_add(0x1c4u64))) as u32) as u32 as u64; rax = (((rax as u32)).wrapping_shl(2u32)) as u32 as u64; rax = ((((rax as u32)) ^ (crate::simd_ops::ru32(rbx.wrapping_add(0x198u64))))) as u32 as u64; rax = ((((rax as u32)) & (0x4u32))) as u32 as u64; crate::simd_ops::lk_xor_u32(rbx.wrapping_add(0x198u64), ((rax as u32)) as u32); fl_l = (crate::simd_ops::ru8(rsi.wrapping_add(0x1c4u64))) as u8 as u64; fl_r = ((r12 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 12; continue; } bb = 10; continue; }
            10 => { fl_l = (crate::simd_ops::ru32(rsi.wrapping_add(0x1c0u64))) as u32 as u64; fl_r = ((r15 as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { bb = 13; continue; } bb = 11; continue; }
            11 => { rdx = ((r15.wrapping_add(0x1u64)) as u32) as u32 as u64; r8 = rdi; rcx = rsi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 13; continue; }
            12 => { crate::simd_ops::wu8(rdi.wrapping_add(0x14u64), ((r12 as u8)) as u8); bb = 13; continue; }
            13 => { rbx = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 14; continue; }
            14 => { crate::simd_ops::lk_and_u8(r14.wrapping_add(0x1c4u64), (0xfeu8) as u8); r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            15 => { crate::simd_ops::lk_and_u8(rcx.wrapping_add(0x1c4u64), (0xfeu8) as u8); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415f2b00(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141600b70; const P1: u64 = 0x143931d78; const P2: u64 = 0x70; const P3: u64 = 0x143931d70; const P4: u64 = 0x1415f01e0; const P5: u64 = 0x1415edf30;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rdx; rdi = rcx; rdx = rcx; rcx = rbx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); r10 = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x3f; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            2 => { r9 = (crate::simd_ops::ru64(P1)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = r9; fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 3; continue; }
            3 => { bb = 4; continue; }
            4 => { fl_l = rcx; fl_r = rbx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 7; continue; } bb = 5; continue; }
            5 => { rdx = rcx; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 4; continue; } bb = 6; continue; }
            6 => { bb = 11; continue; }
            7 => { r8 = (crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64; fl_l = rcx; fl_r = r9; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 9; continue; } bb = 8; continue; }
            8 => { crate::simd_ops::wu64(P1, r8); bb = 10; continue; }
            9 => { crate::simd_ops::wu64(rdx.wrapping_add(P2), r8); bb = 10; continue; }
            10 => { rax = (crate::simd_ops::ru64(P3)) as u64; fl_l = rcx; fl_r = rax; rax = (if (((fl_l as u64)) == ((fl_r as u64))) { rdx } else { rax }) as u64; crate::simd_ops::wu64(P3, rax); bb = 11; continue; }
            11 => { rcx = rbx; fl_l = ((r10 as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 13; continue; } bb = 12; continue; }
            12 => { rdx = rdi; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P4, rax, rcx, rdx, r8, r9, rsp); }
            13 => { rdx = (rdx & !0xffu64) | (((0x1u8) as u8 as u64)); rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = 0x2; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415f2bb0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415b19d0; const P1: u64 = 0x14268deb8; const P2: u64 = 0x1415b17e0; const P3: u64 = 0x1415b1e50; const P4: u64 = 0x14268def8; const P5: u64 = 0x1415b1c70; const P6: u64 = 0x1415b1910; const P7: u64 = 0x1415b1860;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedStack::<1>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x70u64)) as u64; rbp = rsp.wrapping_add(0x40); r15 = rcx; crate::simd_ops::wu64(rbp.wrapping_add(0x78u64), rbx); rcx = 0x1; r14 = r9; r13 = r8; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = (crate::simd_ops::ru32(rbp.wrapping_add(0x90u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = 0x1; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = ((rax as u32)) as u32 as u64; bb = 3; continue; }
            2 => { rbx = ((rdi as u32)) as u32 as u64; bb = 3; continue; }
            3 => { rsi = ((rbx as u32)) as u32 as u64; rax = ((rsi.wrapping_mul(8)) as u64) as u64; rcx = rax.wrapping_add(0xf); fl_l = rcx; fl_r = rax; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rcx = (0xffffffffffffff0u64) as u64; bb = 5; continue; }
            5 => { rcx = (((rcx) & (0xfffffffffffffff0u64))) as u64; rax = rcx; rsp = ((rsp).wrapping_sub(rcx)) as u64; r9 = P1; r8 = ((rbx as u32)) as u32 as u64; rcx = rbp.wrapping_add(0x10); rdx = rsp.wrapping_add(0x40); crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x1u32) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); let tmp_xchg: u32 = crate::simd_ops::atomic_xchg_u32(r14, ((rdi as u32)) as u32); rdi = (tmp_xchg) as u32 as u64; fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 11; continue; } bb = 6; continue; }
            6 => { r12 = (crate::simd_ops::ru32(rbp.wrapping_add(0x98u64))) as u32 as u64; rbx = 0; fl_l = 0; fl_r = 0; rdi = 0; fl_l = 0; fl_r = 0; bb = 7; continue; }
            7 => { rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x18u64))) as u64; rcx = rbp.wrapping_add(0x88); r9 = 0x8; crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), (0x1u32) as u32); rdx = ((rdx).wrapping_add(rdi)) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((r12 as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (0x0u64) as u64); r8 = ((r9.wrapping_add(0x10u64)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x88u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { crate::simd_ops::wu64(rbp, r14); rcx = P4; crate::simd_ops::wu64(rbp.wrapping_add(0x8u64), r13); xmm0 = (crate::simd_ops::ru128(rbp)) as u128; crate::simd_ops::wu128(rax.wrapping_add(0x8u64), (xmm0) as u128); crate::simd_ops::wu64(rax, rcx); bb = 9; continue; }
            9 => { rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x18u64))) as u64; rcx = rbp.wrapping_add(0x10); rdx = (crate::simd_ops::ru64(rbx.wrapping_add(rdx))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = ((rdi).wrapping_add(0x8u64)) as u64; rbx = ((rbx).wrapping_add(0x8u64)) as u64; fl_l = rsi; fl_r = (0x1u64) as u64; rsi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 7; continue; } bb = 10; continue; }
            10 => { r15 = (crate::simd_ops::ru64(rbp.wrapping_add(0x70u64))) as u64; bb = 11; continue; }
            11 => { rdx = r15; rcx = rbp.wrapping_add(0x10); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rbp.wrapping_add(0x10); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = (crate::simd_ops::ru64(rbp.wrapping_add(0x78u64))) as u64; rax = r15; rsp = rbp.wrapping_add(0x30); r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415f2d10(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x12; const P1: u64 = 0x8; const P2: u64 = 0x10; const P3: u64 = 0x14183acc2;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = ((crate::simd_ops::ru16(rcx.wrapping_add(P0))) as u32) as u32 as u64; rbx = rcx; rdx = ((crate::simd_ops::ru8(rcx.wrapping_add(P1))) as u32) as u32 as u64; rcx = ((crate::simd_ops::ru16(rcx.wrapping_add(P2))) as u32) as u32 as u64; fl_l = ((rax as u16)) as u16 as u64; fl_r = ((rcx as u16)) as u16 as u64; rax = (rax & !0xffffu64) | (((if (((fl_l as u16)) >= ((fl_r as u16))) { (rcx as u16) } else { (rax as u16) }) as u16 as u64)); r8 = (((rax as u16)) as u32) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 6; continue; } bb = 1; continue; }
            1 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 5; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbp); rbp = ((rcx.wrapping_mul(4)) as u32) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rsi); rsi = ((rdx as u32)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rdi); rdi = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), r14); r14 = ((r8 as u32)) as u32 as u64; bb = 3; continue; }
            3 => { rcx = ((crate::simd_ops::ru16(rbx.wrapping_add(P2))) as u32) as u32 as u64; r8 = rbp; rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = (((rcx) as i64).wrapping_mul((rdi) as i64) as u64) as u64; rcx = ((rcx).wrapping_add(r14)) as u64; rcx = ((rax.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rdi = ((rdi).wrapping_add(1)) as u64; fl_l = rsi; fl_r = (0x1u64) as u64; rsi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 4; continue; }
            4 => { r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; bb = 5; continue; }
            5 => { rax = ((crate::simd_ops::ru16(rbx.wrapping_add(P2))) as u32) as u32 as u64; crate::simd_ops::wu16(rbx.wrapping_add(P0), ((rax as u16)) as u16); bb = 6; continue; }
            6 => { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415f2db0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x88; const P2: u64 = 0x143426564; const P3: u64 = 0x142673708; const P4: u64 = 0x1426736fc; const P5: u64 = 0x142673700; const P6: u64 = 0x1426736ec; const P7: u64 = 0x1426736e8; const P8: u64 = 0x1426736f0; const P9: u64 = 0x100; const P10: u64 = 0x10; const P11: u64 = 0x160; const P12: u64 = 0x14158fd60; const P13: u64 = 0x1415d7890;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rdx = rcx; xmm0 = ((crate::simd_ops::ru32(rbx.wrapping_add(P1))) as u32) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(crate::simd_ops::ru32(P2))).to_bits() as u128); fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(P3)) as u32 as u64; if (f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { xmm1 = 0u128; bb = 3; continue; }
            2 => { xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(crate::simd_ops::ru32(P4))).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(crate::simd_ops::ru32(P5))).to_bits() as u128); rcx = (crate::simd_ops::cvt_f32_i64((f32::from_bits(xmm0 as u32)))) as u64; rax = ((rcx as u32)) as u32 as u64; rcx = ((((rcx as u32)) & (0x7fffffu32))) as u32 as u64; rax = ((((rax as u32)) & (0xff800000u32))) as u32 as u64; fl_l = (((rcx as u32)).wrapping_add(0x3f800000u32)) as u32 as u64; fl_r = fl_l; rcx = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), ((rax as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((rcx as u32)) as u32); xmm0 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x30u64))) as u32) as u128; xmm1 = (xmm0) as u128; xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(crate::simd_ops::ru32(P6))).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) + f32::from_bits(crate::simd_ops::ru32(P7))).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) + f32::from_bits(crate::simd_ops::ru32(P8))).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(crate::simd_ops::ru32(rsp.wrapping_add(0x38u64)))).to_bits() as u128); bb = 3; continue; }
            3 => { xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(crate::simd_ops::ru32(rbx.wrapping_add(P9)))).to_bits() as u128); crate::simd_ops::wu32(rdx.wrapping_add(P10), ((xmm1 as u32)) as u32); fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(P11))) as u8 as u64; fl_r = (0x8u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 6; continue; } bb = 4; continue; }
            4 => { rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P12, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rdx = rbx.wrapping_add(0x198); rcx = rbx; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk_xmm(P13, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); }
            6 => { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415f2e80(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x4; const P2: u64 = 0x141524330; const P3: u64 = 0x1415f3150; const P4: u64 = 0x8; const P5: u64 = 0x58; const P6: u64 = 0x1415242e0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rdi = rcx; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 6; continue; } bb = 1; continue; }
            1 => { rdx = 0xf0; rcx = ((rax.wrapping_add(P1)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 2; continue; }
            2 => { rcx = rax; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rbx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu64(rdi.wrapping_add(P0), rax); rcx = rbx; crate::simd_ops::wu64(rax.wrapping_add(P4), rdi); rax = (crate::simd_ops::ru64(rax)) as u64; rdx = (crate::simd_ops::ru64(rdi.wrapping_add(P5))) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P4))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 7; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rdx = rbx; rcx = 0x4; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rdi.wrapping_add(P0), (0x0u64) as u64); bb = 5; continue; }
            5 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; bb = 6; continue; }
            6 => { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            7 => { rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            8 => { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415f2f20(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x4; const P2: u64 = 0x141524330; const P3: u64 = 0x141604a30; const P4: u64 = 0x58; const P5: u64 = 0x1415242e0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rdi = rcx; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 6; continue; } bb = 1; continue; }
            1 => { rdx = 0x80; rcx = ((rax.wrapping_add(P1)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 2; continue; }
            2 => { rcx = rax; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rbx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu64(rdi.wrapping_add(P0), rax); rcx = rbx; crate::simd_ops::wu64(rax.wrapping_add(P0), rdi); rax = (crate::simd_ops::ru64(rax)) as u64; rdx = (crate::simd_ops::ru64(rdi.wrapping_add(P4))) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 7; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rdx = rbx; rcx = 0x4; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rdi.wrapping_add(P0), (0x0u64) as u64); bb = 5; continue; }
            5 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; bb = 6; continue; }
            6 => { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            7 => { rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            8 => { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415f2fc0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x20; const P1: u64 = 0x4; const P2: u64 = 0x141524330; const P3: u64 = 0x1415d4390; const P4: u64 = 0x8; const P5: u64 = 0x58; const P6: u64 = 0x1415242e0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rdi = rcx; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 6; continue; } bb = 1; continue; }
            1 => { rdx = 0xa0; rcx = ((rax.wrapping_add(P1)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 2; continue; }
            2 => { rcx = rax; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rbx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu64(rdi.wrapping_add(P0), rax); rcx = rbx; crate::simd_ops::wu64(rax.wrapping_add(P4), rdi); rax = (crate::simd_ops::ru64(rax)) as u64; rdx = (crate::simd_ops::ru64(rdi.wrapping_add(P5))) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P4))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 7; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rdx = rbx; rcx = 0x4; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rdi.wrapping_add(P0), (0x0u64) as u64); bb = 5; continue; }
            5 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; bb = 6; continue; }
            6 => { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            7 => { rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            8 => { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415f3060(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x30; const P1: u64 = 0x4; const P2: u64 = 0x141524330; const P3: u64 = 0x8; const P4: u64 = 0x14268dfb8; const P5: u64 = 0x14392d480; const P6: u64 = 0x18; const P7: u64 = 0x10; const P8: u64 = 0x14392d488; const P9: u64 = 0x14392d478; const P10: u64 = 0x14268dfc8; const P11: u64 = 0x20; const P12: u64 = 0x1415242e0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rsi = rcx; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 8; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx); rdx = ((rax.wrapping_add(P0)) as u32) as u32 as u64; rcx = ((rax.wrapping_add(P1)) as u32) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rdi); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rdi = 0; fl_l = 0; fl_r = 0; rbx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rax.wrapping_add(P3), rdi); rax = P4; crate::simd_ops::wu64(rbx, rax); rax = (crate::simd_ops::ru64(P5)) as u64; crate::simd_ops::wu64(rbx.wrapping_add(P6), rax); crate::simd_ops::wu64(rbx.wrapping_add(P7), rdi); fl_l = (crate::simd_ops::ru64(P8)) as u64; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu64(P8, rbx); bb = 5; continue; }
            4 => { rax = (crate::simd_ops::ru64(P5)) as u64; crate::simd_ops::wu64(rax.wrapping_add(P7), rbx); bb = 5; continue; }
            5 => { crate::simd_ops::lk_add_u32(P9, (1) as u32); rax = P10; crate::simd_ops::wu64(P5, rbx); rcx = rbx; crate::simd_ops::wu64(rbx, rax); crate::simd_ops::wu64(rbx.wrapping_add(P11), rdi); crate::simd_ops::wu64(rbx.wrapping_add(0x28u64), rdi); crate::simd_ops::wu64(rsi.wrapping_add(0x30u64), rbx); rax = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), rsi); rdx = (crate::simd_ops::ru64(rsi.wrapping_add(0x58u64))) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 9; continue; } bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rdx = rbx; rcx = 0x4; rax = crate::simd_ops::call_stk(P12, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rsi.wrapping_add(0x30u64), rdi); bb = 7; continue; }
            7 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = rdi; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; bb = 8; continue; }
            8 => { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            9 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415f3150(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x142688f98; const P1: u64 = 0x8; const P2: u64 = 0x14392d450; const P3: u64 = 0x18; const P4: u64 = 0x10; const P5: u64 = 0x14392d458; const P6: u64 = 0x14392d448; const P7: u64 = 0x142688fa8; const P8: u64 = 0x40; const P9: u64 = 0x48; const P10: u64 = 0x60; const P11: u64 = 0x68;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rdx = 0;
    fl_l = 0; fl_r = 0;
    rax = P0;
    crate::simd_ops::wu64(rcx.wrapping_add(P1), rdx);
    crate::simd_ops::wu64(rcx, rax);
    rax = (crate::simd_ops::ru64(P2)) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(P3), rax);
    crate::simd_ops::wu64(rcx.wrapping_add(P4), rdx);
    fl_l = (crate::simd_ops::ru64(P5)) as u64; fl_r = rdx;
    if !((((fl_l as u64)) != ((fl_r as u64)))) { crate::simd_ops::wu64(P5, rcx); } else { rax = (crate::simd_ops::ru64(P2)) as u64; crate::simd_ops::wu64(rax.wrapping_add(P4), rcx); }
    fl_l = (crate::simd_ops::lk_add_u32(P6, (1) as u32)) as u32 as u64;
    rax = P7;
    crate::simd_ops::wu64(P2, rcx);
    crate::simd_ops::wu64(rcx, rax);
    rax = rcx;
    crate::simd_ops::wu64(rcx.wrapping_add(P8), rdx);
    crate::simd_ops::wu64(rcx.wrapping_add(P9), rdx);
    crate::simd_ops::wu64(rcx.wrapping_add(P10), rdx);
    crate::simd_ops::wu64(rcx.wrapping_add(P11), rdx);
    crate::simd_ops::wu64(rcx.wrapping_add(0x90u64), rdx);
    crate::simd_ops::wu64(rcx.wrapping_add(0x98u64), rdx);
    crate::simd_ops::wu64(rcx.wrapping_add(0xb0u64), rdx);
    crate::simd_ops::wu64(rcx.wrapping_add(0xb8u64), rdx);
    crate::simd_ops::wu64(rcx.wrapping_add(0xd0u64), rdx);
    crate::simd_ops::wu64(rcx.wrapping_add(0xd8u64), rdx);
    crate::simd_ops::wu16(rcx.wrapping_add(0xe0u64), ((rdx as u16)) as u16);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415f32f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14268dfc8; const P1: u64 = 0x20; const P2: u64 = 0x28; const P3: u64 = 0x1415242e0; const P4: u64 = 0x2c; const P5: u64 = 0x10; const P6: u64 = 0x14268dfb8; const P7: u64 = 0x18; const P8: u64 = 0x14392d488; const P9: u64 = 0x14392d480; const P10: u64 = 0x14392d478; const P11: u64 = 0x142688f88; const P12: u64 = 0x141834d30;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = P0; rdi = ((rdx as u32)) as u32 as u64; crate::simd_ops::wu64(rcx, rax); rbx = rcx; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu32(rcx.wrapping_add(P2), (0x0u32) as u32); rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(P1), (0x0u64) as u64); crate::simd_ops::wu32(rbx.wrapping_add(P4), (0x0u32) as u32); bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P5))) as u64; rax = P6; crate::simd_ops::wu64(rbx, rax); fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(P7))) as u64; crate::simd_ops::wu64(rcx.wrapping_add(P7), rax); bb = 4; continue; }
            4 => { fl_l = rbx; fl_r = (crate::simd_ops::ru64(P8)) as u64; rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P7))) as u64; rax = (crate::simd_ops::ru64(rbx.wrapping_add(P5))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { crate::simd_ops::wu64(P8, rax); bb = 7; continue; }
            6 => { crate::simd_ops::wu64(rcx.wrapping_add(P5), rax); bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru64(P9)) as u64; fl_l = rbx; fl_r = rax; rax = (if (((fl_l as u64)) == ((fl_r as u64))) { rcx } else { rax }) as u64; crate::simd_ops::lk_sub_u32(P10, (1) as u32); crate::simd_ops::wu64(P9, rax); rax = P11; crate::simd_ops::wu64(rbx, rax); fl_l = ((rdi as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { rdx = 0x30; rcx = rbx; rax = crate::simd_ops::call_stk(P12, rax, rcx, rdx, r8, r9, rsp); bb = 9; continue; }
            9 => { rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415f33b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415242e0; const P1: u64 = 0x1415f4cb0;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x10u64))) as u32 as u64; r14 = rdx; rbx = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; rbp = rcx; r8 = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = ((rbx.wrapping_add(r8.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 14; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), r15); r15 = 0; fl_l = 0; fl_r = 0; bb = 2; continue; }
            2 => { rdx = (crate::simd_ops::ru32(r14.wrapping_add(0x18u64))) as u32 as u64; r11 = ((r15 as u32)) as u32 as u64; r10 = (crate::simd_ops::ru64(rbx)) as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 6; continue; } bb = 3; continue; }
            3 => { rdi = (crate::simd_ops::ru64(r14.wrapping_add(0x10u64))) as u64; bb = 4; continue; }
            4 => { rax = ((rdx as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shr(1u32)) as u32 as u64; rax = (((rax as u32)).wrapping_add((r11 as u32))) as u32 as u64; rcx = ((((rax as u32)) as i32 as i64) as u64) as u64; r8 = (crate::simd_ops::ru64(rdi.wrapping_add(rcx.wrapping_mul(8)))) as u64; r9 = ((rdi.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = r8; fl_r = r10; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 15; continue; } bb = 5; continue; }
            5 => { rcx = ((rdx.wrapping_sub(0x1u64)) as u32) as u32 as u64; rax = (((rax as u32)).wrapping_add(1)) as u32 as u64; rcx = (if (((fl_l as u64)) >= ((fl_r as u64))) { (rdx as u32) } else { (rcx as u32) }) as u32 as u64; fl_l = r8; fl_r = r10; rdx = ((rcx as u32)) as u32 as u64; rax = (if (((fl_l as u64)) >= ((fl_r as u64))) { (r11 as u32) } else { (rax as u32) }) as u32 as u64; fl_l = (((rdx as u32)).wrapping_shr(1u32)) as u32 as u64; rdx = ((fl_l as u32)) as u32 as u64; r11 = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) != 0) { bb = 4; continue; } bb = 6; continue; }
            6 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rbx.wrapping_add(0x10u64), ((r15 as u32)) as u32); rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), r15); crate::simd_ops::wu32(rbx.wrapping_add(0x14u64), ((r15 as u32)) as u32); bb = 8; continue; }
            8 => { rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x10u64))) as u32 as u64; rsi = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x8u64))) as u64; rsi = rsi.wrapping_sub(0x3); rsi = ((rax.wrapping_add(rsi.wrapping_mul(8))) as u64) as u64; rax = rbx; fl_l = rbx; fl_r = rsi; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 11; continue; } bb = 9; continue; }
            9 => { bb = 10; continue; }
            10 => { rdi = rax.wrapping_add(0x18); rcx = rax; rdx = rdi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rax = rdi; fl_l = rdi; fl_r = rsi; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 10; continue; } bb = 11; continue; }
            11 => { fl_l = (crate::simd_ops::lk_sub_u32(rbp.wrapping_add(0x10u64), (1) as u32)) as u32 as u64; bb = 12; continue; }
            12 => { rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x10u64))) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x8u64))) as u64; rcx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 13; continue; }
            13 => { r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; bb = 14; continue; }
            14 => { rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            15 => { fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 16; continue; }
            16 => { fl_l = ((rbx).wrapping_add(0x18u64)) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; bb = 12; continue; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415f34d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x30; const P1: u64 = 0x1415f5070; const P2: u64 = 0x14392d488; const P3: u64 = 0x20; const P4: u64 = 0x28; const P5: u64 = 0xffffffffffffffff; const P6: u64 = 0xc; const P7: u64 = 0x14183acc8; const P8: u64 = 0x10;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = ((rdx as u32)) as u32 as u64; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 16; continue; } bb = 2; continue; }
            2 => { rcx = ((rcx).wrapping_add(0x20u64)) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rax = 0x1; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            3 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx); rbx = (crate::simd_ops::ru64(P2)) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 15; continue; } bb = 4; continue; }
            4 => { crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rsi); rsi = (0x2aaaaaaaaaaaaaabu64) as u64; bb = 5; continue; }
            5 => { r8 = (crate::simd_ops::ru64(rbx.wrapping_add(P3))) as u64; rdx = (crate::simd_ops::ru32(rbx.wrapping_add(P4))) as u32 as u64; rcx = r8; rax = ((rdx.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64; rax = ((r8.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; fl_l = r8; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 13; continue; } bb = 6; continue; }
            6 => { bb = 7; continue; }
            7 => { fl_l = (crate::simd_ops::ru32(rcx)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 10; continue; } bb = 8; continue; }
            8 => { rcx = ((rcx).wrapping_add(0xcu64)) as u64; fl_l = rcx; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 7; continue; } bb = 9; continue; }
            9 => { bb = 13; continue; }
            10 => { rax = ((rdx.wrapping_add(P5)) as u32) as u32 as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rdx = ((r8.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; fl_l = rcx; fl_r = rdx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 12; continue; } bb = 11; continue; }
            11 => { rdx = ((rdx).wrapping_sub(rcx)) as u64; rax = rsi; let prod = (rax as i64 as i128).wrapping_mul((rdx) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rdx = ((((rdx) as i64).wrapping_shr(1u32) as u64)) as u64; rax = rdx; rax = ((rax).wrapping_shr(63u32)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; r8 = ((rdx.wrapping_add(rdx.wrapping_mul(2))) as u32) as u32 as u64; r8 = (((r8 as u32)).wrapping_shl(2u32)) as u32 as u64; rdx = ((rcx.wrapping_add(P6)) as u64) as u64; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); bb = 12; continue; }
            12 => { fl_l = (crate::simd_ops::lk_sub_u32(rbx.wrapping_add(P4), (1) as u32)) as u32 as u64; bb = 13; continue; }
            13 => { rbx = (crate::simd_ops::ru64(rbx.wrapping_add(P8))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 14; continue; }
            14 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; bb = 15; continue; }
            15 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; bb = 16; continue; }
            16 => { rax = 0x1; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415f35b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x18; const P2: u64 = 0x30; const P3: u64 = 0x28; const P4: u64 = 0x8; const P5: u64 = 0x1415f2fc0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; rdi = rdx; rax = (crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32 as u64; rsi = rcx; r8 = ((rbx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = r8; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 9; continue; } bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru32(rsi.wrapping_add(P2))) as u32 as u64; r8 = (crate::simd_ops::ru64(rbx)) as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 8; continue; } bb = 2; continue; }
            2 => { rdx = 0; fl_l = 0; fl_r = 0; rax = ((r8 as u32)) as u32 as u64; let d = ((rcx as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } rax = (crate::simd_ops::ru64(rsi.wrapping_add(P3))) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(rdx.wrapping_mul(8)))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru64(rax.wrapping_add(P4))) as u64; fl_r = r8; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 10; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rax)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } bb = 5; continue; }
            5 => { rcx = 0; fl_l = 0; fl_r = 0; bb = 6; continue; }
            6 => { fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { rcx = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); bb = 8; continue; }
            8 => { rcx = (crate::simd_ops::ru32(rdi.wrapping_add(P1))) as u32 as u64; rbx = ((rbx).wrapping_add(0x8u64)) as u64; rax = (crate::simd_ops::ru64(rdi.wrapping_add(P0))) as u64; rcx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 9; continue; }
            9 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            10 => { rcx = ((rax.wrapping_add(P0)) as u64) as u64; bb = 6; continue; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415f3640(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14153e940;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; r14 = rdx; rsi = rcx; rdx = (crate::simd_ops::ru32(rcx.wrapping_add(0x30u64))) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 20; continue; } bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x28u64))) as u64; rbx = 0; fl_l = 0; fl_r = 0; rdi = (crate::simd_ops::ru64(rcx)) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 7; continue; } bb = 2; continue; }
            2 => { rbx = (((rbx as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 5; continue; } bb = 3; continue; }
            3 => { rdi = (crate::simd_ops::ru64(rcx.wrapping_add(rbx.wrapping_mul(8)))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 4; continue; }
            4 => { bb = 7; continue; }
            5 => { fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 20; continue; } bb = 6; continue; }
            6 => { bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; fl_l = (crate::simd_ops::ru32(rax.wrapping_add(0x60u64))) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 9; continue; } bb = 8; continue; }
            8 => { fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x64u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 14; continue; } bb = 9; continue; }
            9 => { rbp = (crate::simd_ops::ru32(r14.wrapping_add(0x8u64))) as u32 as u64; r15 = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; fl_l = ((rbp as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(r14.wrapping_add(0xcu64))) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 12; continue; } bb = 10; continue; }
            10 => { rcx = r14; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 22; continue; } bb = 11; continue; }
            11 => { fl_l = ((rbp as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(r14.wrapping_add(0xcu64))) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 22; continue; } bb = 12; continue; }
            12 => { rcx = (crate::simd_ops::ru32(r14.wrapping_add(0x8u64))) as u32 as u64; rax = ((rcx.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(r14.wrapping_add(0x8u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(r14)) as u64; rdx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 22; continue; } bb = 13; continue; }
            13 => { crate::simd_ops::wu64(rdx, r15); bb = 14; continue; }
            14 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rdi = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 19; continue; } bb = 15; continue; }
            15 => { rdx = (crate::simd_ops::ru32(rsi.wrapping_add(0x30u64))) as u32 as u64; bb = 16; continue; }
            16 => { rbx = (((rbx as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 19; continue; } bb = 17; continue; }
            17 => { rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x28u64))) as u64; rdi = (crate::simd_ops::ru64(rax.wrapping_add(rbx.wrapping_mul(8)))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 16; continue; } bb = 18; continue; }
            18 => { bb = 7; continue; }
            19 => { fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 7; continue; } bb = 20; continue; }
            20 => { rax = 0x1; bb = 21; continue; }
            21 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            22 => { rax = 0x34; bb = 21; continue; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415f3730(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x143930f78; const P1: u64 = 0x30; const P2: u64 = 0x28; const P3: u64 = 0x8;
    let (mut rax, mut r10, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r10 = (crate::simd_ops::ru64(P0)) as u64; r8 = 0; fl_l = 0; fl_r = 0; r9 = rdx; rcx = (crate::simd_ops::ru32(r10.wrapping_add(P1))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 8; continue; } bb = 1; continue; }
            1 => { rdx = 0; fl_l = 0; fl_r = 0; rax = ((r9 as u32)) as u32 as u64; let d = ((rcx as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } rax = (crate::simd_ops::ru64(r10.wrapping_add(P2))) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(rdx.wrapping_mul(8)))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 2; continue; }
            2 => { bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64; fl_r = r9; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 9; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rax)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } bb = 5; continue; }
            5 => { rax = r8; bb = 6; continue; }
            6 => { fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { r8 = (crate::simd_ops::ru64(rax)) as u64; bb = 8; continue; }
            8 => { rax = r8; return rax; }
            9 => { fl_l = ((rax).wrapping_add(0x10u64)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; bb = 6; continue; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415f3790(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x30; const P1: u64 = 0x28; const P2: u64 = 0x8;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r8 = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; r9 = rdx; fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 8; continue; } bb = 1; continue; }
            1 => { rdx = 0; fl_l = 0; fl_r = 0; rax = ((r9 as u32)) as u32 as u64; let d = ((r8 as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } rax = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(rdx.wrapping_mul(8)))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; fl_r = r9; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 7; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rax)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 4; continue; }
            4 => { rax = 0; fl_l = 0; fl_r = 0; bb = 5; continue; }
            5 => { fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(rax)) as u64; return rax; }
            7 => { fl_l = ((rax).wrapping_add(0x10u64)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; bb = 5; continue; }
            8 => { rax = 0; fl_l = 0; fl_r = 0; return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415f37e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x30; const P1: u64 = 0x28; const P2: u64 = 0x8; const P3: u64 = 0x60; const P4: u64 = 0x64; const P5: u64 = 0x1; const P6: u64 = 0x1415af330; const P7: u64 = 0x10;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; r8 = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; r9 = rdx; fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 14; continue; } bb = 1; continue; }
            1 => { rdx = 0; fl_l = 0; fl_r = 0; rax = ((r9 as u32)) as u32 as u64; let d = ((r8 as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } rax = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(rdx.wrapping_mul(8)))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; fl_r = r9; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 11; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rax)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 4; continue; }
            4 => { rbx = 0; fl_l = 0; fl_r = 0; bb = 5; continue; }
            5 => { fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 6; continue; }
            6 => { rdx = (crate::simd_ops::ru64(rbx)) as u64; rcx = (crate::simd_ops::ru32(rdx.wrapping_add(P3))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 12; continue; } bb = 7; continue; }
            7 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(P4))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 12; continue; } bb = 8; continue; }
            8 => { rax = ((rcx.wrapping_add(P5)) as u32) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_add(P3), ((rax as u32)) as u32); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 10; continue; } bb = 9; continue; }
            9 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(P4))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 13; continue; } bb = 10; continue; }
            10 => { rcx = rdx; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rbx)) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            11 => { rbx = ((rax.wrapping_add(P7)) as u64) as u64; bb = 5; continue; }
            12 => { fl_l = (((rcx as u32)).wrapping_add(1)) as u32 as u64; rcx = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_add(P3), ((rcx as u32)) as u32); bb = 13; continue; }
            13 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            14 => { rax = 0; fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415f3870(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x30; const P1: u64 = 0x20; const P2: u64 = 0x28; const P3: u64 = 0x4;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r11 = 0; fl_l = 0; fl_r = 0; r10 = rdx; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; rcx = (crate::simd_ops::ru32(rcx.wrapping_add(P2))) as u32 as u64; rdx = ((rcx.wrapping_add(rcx.wrapping_mul(2))) as u64) as u64; rcx = ((rax.wrapping_add(rdx.wrapping_mul(4))) as u64) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 5; continue; } bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru32(rax)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = ((rax).wrapping_add(0xcu64)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 5; continue; }
            5 => { fl_l = rax; fl_r = rcx; rdx = ((rax.wrapping_add(P3)) as u64) as u64; rdx = (if (((fl_l as u64)) == ((fl_r as u64))) { r11 } else { rdx }) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(rdx)) as u64; crate::simd_ops::wu64(r10, rax); rax = r10; return rax; }
            7 => { rax = r11; crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), r11); crate::simd_ops::wu64(r10, rax); rax = r10; return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415f38d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141524330; const P1: u64 = 0x18; const P2: u64 = 0x34; const P3: u64 = 0x20; const P4: u64 = 0x10; const P5: u64 = 0x143931c70; const P6: u64 = 0x143931c78; const P7: u64 = 0x143931c74; const P8: u64 = 0x143931c68; const P9: u64 = 0x14159a6d0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rcx; rdx = 0x80; rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(P1), rax); rdx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = ((rdx.wrapping_add(P2)) as u32) as u32 as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            2 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rdi); rdi = 0; fl_l = 0; fl_r = 0; rcx = ((rdi as u32)) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(P3), (0x8u32) as u32); crate::simd_ops::wu64(rbx.wrapping_add(P4), rax); bb = 3; continue; }
            3 => { rax = ((rdx.wrapping_add(P4)) as u64) as u64; rcx = (((rcx as u32)).wrapping_add(1)) as u32 as u64; crate::simd_ops::wu64(rdx, rax); r8 = rdx; rdx = rax; fl_l = ((rcx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rbx.wrapping_add(P3))) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 3; continue; } bb = 4; continue; }
            4 => { crate::simd_ops::wu64(r8, rdi); fl_l = (crate::simd_ops::ru32(P5)) as u32 as u64; fl_r = 0x80; crate::simd_ops::wu32(P6, ((rdi as u32)) as u32); if (((fl_l as u32)) > ((fl_r as u32))) { bb = 8; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru32(P7)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 8; continue; } bb = 6; continue; }
            6 => { rdx = 0x80; rcx = P8; rdx = (((rdx as u32)).wrapping_sub((rax as u32))) as u32 as u64; rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { rcx = (rcx & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 11; continue; }
            8 => { crate::simd_ops::wu32(P5, (0x80u32) as u32); rdx = ((rdi as u32)) as u32 as u64; bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru64(P8)) as u64; rcx = ((rdx as u32)) as u32 as u64; rdx = (((rdx as u32)).wrapping_add(1)) as u32 as u64; crate::simd_ops::wu64(rax.wrapping_add(rcx.wrapping_mul(8)), rdi); fl_l = ((rdx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(P5)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 9; continue; } bb = 10; continue; }
            10 => { rcx = (rcx & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 11; continue; }
            11 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; rax = 0x34; rdx = 0x1; rax = (if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { (rdx as u32) } else { (rax as u32) }) as u32 as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415f39b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x30; const P1: u64 = 0x28; const P2: u64 = 0x8; const P3: u64 = 0x10; const P4: u64 = 0x60; const P5: u64 = 0x64;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r8 = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; r9 = rdx; fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 5; continue; } bb = 1; continue; }
            1 => { rdx = 0; fl_l = 0; fl_r = 0; rax = ((r9 as u32)) as u32 as u64; let d = ((r8 as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } rax = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(rdx.wrapping_mul(8)))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; fl_r = r9; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 6; continue; } bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(rax)) as u64; rax = rcx; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 4; continue; }
            4 => { bb = 6; continue; }
            5 => { rax = 0; fl_l = 0; fl_r = 0; bb = 6; continue; }
            6 => { fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 7; continue; }
            7 => { rcx = (crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64; fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(P4))) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 9; continue; } bb = 8; continue; }
            8 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(P5))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 10; continue; } bb = 9; continue; }
            9 => { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); return rax; }
            10 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415f3a10(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14392d4b8; const P1: u64 = 0x8; const P2: u64 = 0x10; const P3: u64 = 0x14158c580; const P4: u64 = 0x18; const P5: u64 = 0xd0; const P6: u64 = 0xd8; const P7: u64 = 0x7c;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = (crate::simd_ops::ru64(P0)) as u64; rdi = rdx; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 15; continue; } bb = 1; continue; }
            1 => { bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P2))) as u64; r8 = (crate::simd_ops::ru32(rdi.wrapping_add(P4))) as u32 as u64; rdx = (crate::simd_ops::ru64(rax.wrapping_add(P5))) as u64; r9 = ((rcx.wrapping_add(r8.wrapping_mul(8))) as u64) as u64; fl_l = rcx; fl_r = r9; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 14; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru32(rax.wrapping_add(P6))) as u32 as u64; r10 = ((rdx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; bb = 6; continue; }
            6 => { fl_l = rdx; fl_r = r10; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 14; continue; } bb = 7; continue; }
            7 => { r8 = (crate::simd_ops::ru64(rcx)) as u64; fl_l = r8; fl_r = (crate::simd_ops::ru64(rdx)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 13; continue; } bb = 8; continue; }
            8 => { if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 10; continue; } bb = 9; continue; }
            9 => { fl_l = ((rcx).wrapping_add(0x8u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; bb = 11; continue; }
            10 => { fl_l = ((rdx).wrapping_add(0x8u64)) as u64; fl_r = fl_l; rdx = ((fl_l as u64)) as u64; bb = 11; continue; }
            11 => { fl_l = rcx; fl_r = r9; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 6; continue; } bb = 12; continue; }
            12 => { bb = 14; continue; }
            13 => { fl_l = (crate::simd_ops::lk_or_u32(rbx.wrapping_add(P7), (0x1u32) as u32)) as u32 as u64; fl_r = fl_l; bb = 14; continue; }
            14 => { rbx = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 15; continue; }
            15 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415f3aa0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415f2f20; const P1: u64 = 0x142688fd8; const P2: u64 = 0x1439312e0; const P3: u64 = 0x142688fb8; const P4: u64 = 0x1439312e8; const P5: u64 = 0x142688fc8; const P6: u64 = 0x142688fe8; const P7: u64 = 0x1439312f8; const P8: u64 = 0x1439312f4; const P9: u64 = 0x142688fd0; const P10: u64 = 0x142688fe0; const P11: u64 = 0x1439312ec; const P12: u64 = 0x143931308; const P13: u64 = 0x143931310; const P14: u64 = 0x141605b80; const P15: u64 = 0x14392d458; const P16: u64 = 0x141590920; const P17: u64 = 0x1415f2fc0; const P18: u64 = 0x141524330; const P19: u64 = 0x1415af170; const P20: u64 = 0x14268dfb0; const P21: u64 = 0x143426890; const P22: u64 = 0x1415f3f50; const P23: u64 = 0x141605000; const P24: u64 = 0x1415242e0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; r12 = rcx.wrapping_add(0x28); rbp = (rbp & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rcx = (crate::simd_ops::ru32(r12.wrapping_add(0x8u64))) as u32 as u64; r15 = 0; fl_l = 0; fl_r = 0; rdi = rdx; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 23; continue; } bb = 1; continue; }
            1 => { rdx = 0; fl_l = 0; fl_r = 0; rax = ((rdi as u32)) as u32 as u64; let d = ((rcx as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } rax = (crate::simd_ops::ru64(r12)) as u64; rdx = (crate::simd_ops::ru64(rax.wrapping_add(rdx.wrapping_mul(8)))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 2; continue; }
            2 => { bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 22; continue; } bb = 4; continue; }
            4 => { rdx = (crate::simd_ops::ru64(rdx)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } bb = 5; continue; }
            5 => { rsi = r15; bb = 6; continue; }
            6 => { fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 23; continue; } bb = 7; continue; }
            7 => { r14 = (crate::simd_ops::ru64(rsi)) as u64; r13 = ((rdi as u32)) as u32 as u64; bb = 8; continue; }
            8 => { fl_l = r14; fl_r = r14; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 41; continue; } bb = 9; continue; }
            9 => { fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 40; continue; } bb = 10; continue; }
            10 => { rbx = 0x1; fl_l = (crate::simd_ops::ru64(r14.wrapping_add(0x8u64))) as u64; fl_r = r15; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 13; continue; } bb = 11; continue; }
            11 => { rcx = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 12; continue; }
            12 => { rcx = (crate::simd_ops::ru32(P1)) as u32 as u64; rdx = P2; xmm0 = (crate::simd_ops::ru128(P3)) as u128; r9 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(P4, ((rcx as u32)) as u32); xmm1 = ((crate::simd_ops::ru64(P5)) as u64) as u128; r8 = ((rbx as u32)) as u32 as u64; rcx = (crate::simd_ops::ru32(P6)) as u32 as u64; crate::simd_ops::wu128(P7, (xmm0) as u128); crate::simd_ops::wu32(P8, ((rcx as u32)) as u32); rcx = rax; xmm0 = ((crate::simd_ops::ru64(P9)) as u64) as u128; crate::simd_ops::wu64(P2, ((xmm0 as u64)) as u64); xmm0 = ((crate::simd_ops::ru64(P10)) as u64) as u128; crate::simd_ops::wu64(P11, ((xmm0 as u64)) as u64); crate::simd_ops::wu64(P12, ((xmm1 as u64)) as u64); crate::simd_ops::wu32(P13, (0x3ff3fu32) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P14, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 13; continue; }
            13 => { rbp = (((rbx as u8)) as u32) as u32 as u64; bb = 14; continue; }
            14 => { rbx = (crate::simd_ops::ru64(P15)) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 20; continue; } bb = 15; continue; }
            15 => { r15 = 0x1; bb = 16; continue; }
            16 => { rcx = rbx.wrapping_add(0xc0); rdx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P16, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 18; continue; } bb = 17; continue; }
            17 => { rcx = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P17, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rax; fl_r = rax; rbp = (((rbp as u8)) as u32) as u32 as u64; rbp = (if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { (r15 as u32) } else { (rbp as u32) }) as u32 as u64; bb = 18; continue; }
            18 => { rbx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 16; continue; } bb = 19; continue; }
            19 => { r15 = 0; fl_l = 0; fl_r = 0; bb = 20; continue; }
            20 => { fl_l = ((rbp as u8)) as u8 as u64; fl_r = ((rbp as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 40; continue; } bb = 21; continue; }
            21 => { rax = 0x1; bb = 53; continue; }
            22 => { rsi = rdx.wrapping_add(0x10); bb = 6; continue; }
            23 => { rdx = 0x78; rcx = ((rdx.wrapping_sub(0x74u64)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P18, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r14 = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 52; continue; } bb = 24; continue; }
            24 => { rdx = rdi; rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P19, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = P20; crate::simd_ops::wu64(r14.wrapping_add(0x68u64), r15); crate::simd_ops::wu64(r14, rax); crate::simd_ops::wu64(r14.wrapping_add(0x70u64), r15); rdx = (crate::simd_ops::ru32(r12.wrapping_add(0x8u64))) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 26; continue; } bb = 25; continue; }
            25 => { rax = (crate::simd_ops::ru32(r12.wrapping_add(0x10u64))) as u32 as u64; xmm1 = 0u128; xmm0 = 0u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((((rdx) as i64) as f32).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((((rax) as i64) as f32).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) / f32::from_bits(xmm0 as u32)).to_bits() as u128); fl_l = ((xmm1 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(P21)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 27; continue; } bb = 26; continue; }
            26 => { rcx = r12; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P22, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 27; continue; }
            27 => { rcx = (crate::simd_ops::ru32(r12.wrapping_add(0x8u64))) as u32 as u64; r13 = ((rdi as u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 39; continue; } bb = 28; continue; }
            28 => { rdx = 0; fl_l = 0; fl_r = 0; rax = ((rdi as u32)) as u32 as u64; let d = ((rcx as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } rax = (crate::simd_ops::ru64(r12)) as u64; rbx = ((rdx as u32)) as u32 as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(rdx.wrapping_mul(8)))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 34; continue; } bb = 29; continue; }
            29 => { bb = 30; continue; }
            30 => { fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 33; continue; } bb = 31; continue; }
            31 => { rcx = (crate::simd_ops::ru64(rcx)) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 30; continue; } bb = 32; continue; }
            32 => { bb = 34; continue; }
            33 => { rsi = rcx.wrapping_add(0x10); fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 38; continue; } bb = 34; continue; }
            34 => { rdx = 0x18; rcx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P18, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rsi = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 36; continue; } bb = 35; continue; }
            35 => { rsi = r15; bb = 37; continue; }
            36 => { rax = (crate::simd_ops::ru64(r12)) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(rbx.wrapping_mul(8)))) as u64; crate::simd_ops::wu64(rsi, rcx); crate::simd_ops::wu64(rsi.wrapping_add(0x8u64), rdi); rax = (crate::simd_ops::ru64(r12)) as u64; crate::simd_ops::wu64(rax.wrapping_add(rbx.wrapping_mul(8)), rsi); crate::simd_ops::lk_add_u32(r12.wrapping_add(0x10u64), (1) as u32); fl_l = ((rsi).wrapping_add(0x10u64)) as u64; fl_r = fl_l; rsi = ((fl_l as u64)) as u64; bb = 37; continue; }
            37 => { fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 38; continue; }
            38 => { crate::simd_ops::wu64(rsi, r14); bb = 10; continue; }
            39 => { rsi = r15; bb = 40; continue; }
            40 => { rcx = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P23, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = r14; rcx = 0x4; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P24, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 41; continue; }
            41 => { fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 52; continue; } bb = 42; continue; }
            42 => { rcx = (crate::simd_ops::ru32(r12.wrapping_add(0x8u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 52; continue; } bb = 43; continue; }
            43 => { r8 = (crate::simd_ops::ru64(r12)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rax = ((r13 as u32)) as u32 as u64; let d = ((rcx as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } rcx = ((rdx as u32)) as u32 as u64; rdx = (crate::simd_ops::ru64(r8.wrapping_add(rdx.wrapping_mul(8)))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 52; continue; } bb = 44; continue; }
            44 => { bb = 45; continue; }
            45 => { rax = (crate::simd_ops::ru64(rdx)) as u64; fl_l = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 48; continue; } bb = 46; continue; }
            46 => { r15 = rdx; rdx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 45; continue; } bb = 47; continue; }
            47 => { bb = 52; continue; }
            48 => { fl_l = r15; fl_r = r15; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 50; continue; } bb = 49; continue; }
            49 => { crate::simd_ops::wu64(r15, rax); bb = 51; continue; }
            50 => { crate::simd_ops::wu64(r8.wrapping_add(rcx.wrapping_mul(8)), rax); bb = 51; continue; }
            51 => { rcx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P24, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::lk_sub_u32(r12.wrapping_add(0x10u64), (1) as u32)) as u32 as u64; bb = 52; continue; }
            52 => { rax = 0x2; bb = 53; continue; }
            53 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415f3db0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143931c70; const P1: u64 = 0x143931c78; const P2: u64 = 0x143426890; const P3: u64 = 0x143931c68; const P4: u64 = 0x1415f40d0;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 1; continue; }
            1 => { r8 = (crate::simd_ops::ru32(P0)) as u32 as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru32(P1)) as u32 as u64; xmm1 = 0u128; xmm0 = 0u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((((r8) as i64) as f32).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((((rax) as i64) as f32).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) / f32::from_bits(xmm0 as u32)).to_bits() as u128); fl_l = ((xmm1 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(P2)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rdx = ((r8 as u32)) as u32 as u64; rcx = P3; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = (crate::simd_ops::ru32(P0)) as u32 as u64; bb = 4; continue; }
            4 => { fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 10; continue; } bb = 5; continue; }
            5 => { rdx = 0; fl_l = 0; fl_r = 0; rax = ((rbx as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shr(16u32)) as u32 as u64; rax = ((((rax as u32)) ^ ((rbx as u32)))) as u32 as u64; rcx = ((((rax as u32)) as i32).wrapping_mul((0x85ebca6bu32) as i32) as u32) as u32 as u64; rax = ((rcx as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shr(13u32)) as u32 as u64; rax = ((((rax as u32)) ^ ((rcx as u32)))) as u32 as u64; rcx = ((((rax as u32)) as i32).wrapping_mul((0xc2b2ae35u32) as i32) as u32) as u32 as u64; r9 = ((rcx as u32)) as u32 as u64; r9 = (((r9 as u32)).wrapping_shr(16u32)) as u32 as u64; r9 = ((((r9 as u32)) ^ ((rcx as u32)))) as u32 as u64; rax = ((r9 as u32)) as u32 as u64; let d = ((r8 as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } rax = (crate::simd_ops::ru64(P3)) as u64; r8 = ((rdx as u32)) as u32 as u64; r10 = (crate::simd_ops::ru64(rax.wrapping_add(rdx.wrapping_mul(8)))) as u64; rdx = r10; fl_l = r10; fl_r = r10; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 6; continue; }
            6 => { bb = 7; continue; }
            7 => { rax = ((rdx as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shr(16u32)) as u32 as u64; rax = ((((rax as u32)) ^ ((rdx as u32)))) as u32 as u64; rcx = ((((rax as u32)) as i32).wrapping_mul((0x85ebca6bu32) as i32) as u32) as u32 as u64; rax = ((rcx as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shr(13u32)) as u32 as u64; rax = ((((rax as u32)) ^ ((rcx as u32)))) as u32 as u64; rcx = ((((rax as u32)) as i32).wrapping_mul((0xc2b2ae35u32) as i32) as u32) as u32 as u64; rax = ((rcx as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shr(16u32)) as u32 as u64; rax = ((((rax as u32)) ^ ((rcx as u32)))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 10; continue; } bb = 8; continue; }
            8 => { rdx = (crate::simd_ops::ru64(rdx.wrapping_add(0x70u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 7; continue; } bb = 9; continue; }
            9 => { crate::simd_ops::wu64(rbx.wrapping_add(0x70u64), r10); rax = (crate::simd_ops::ru64(P3)) as u64; crate::simd_ops::wu64(rax.wrapping_add(r8.wrapping_mul(8)), rbx); fl_l = (crate::simd_ops::lk_add_u32(P1, (1) as u32)) as u32 as u64; bb = 10; continue; }
            10 => { rax = 0x1; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415f3eb0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x30; const P1: u64 = 0x28; const P2: u64 = 0x8; const P3: u64 = 0x10; const P4: u64 = 0x141603fb0; const P5: u64 = 0x1415f1a90; const P6: u64 = 0x1415f2f20; const P7: u64 = 0x7c; const P8: u64 = 0xc0; const P9: u64 = 0x20; const P10: u64 = 0x1415f33b0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x28u64)) as u64; r8 = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; r9 = rdx; fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 13; continue; } bb = 1; continue; }
            1 => { rdx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx); rax = ((r9 as u32)) as u32 as u64; let d = ((r8 as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } rax = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(rdx.wrapping_mul(8)))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 2; continue; }
            2 => { bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; fl_r = r9; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 14; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rax)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } bb = 5; continue; }
            5 => { rbx = 0; fl_l = 0; fl_r = 0; bb = 6; continue; }
            6 => { fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 7; continue; }
            7 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rdi = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 8; continue; }
            8 => { rdi = (crate::simd_ops::ru64(rdi.wrapping_add(P3))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 9; continue; }
            9 => { rcx = rdi; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rbx)) as u64; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { crate::simd_ops::lk_or_u32(rax.wrapping_add(P7), (0x1u32) as u32); rdx = ((rdi.wrapping_add(P8)) as u64) as u64; rcx = ((rax.wrapping_add(P9)) as u64) as u64; rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp); bb = 11; continue; }
            11 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; bb = 12; continue; }
            12 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; bb = 13; continue; }
            13 => { rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
            14 => { rbx = ((rax.wrapping_add(P3)) as u64) as u64; bb = 6; continue; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415f40d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14268df40; const P1: u64 = 0x141524330; const P2: u64 = 0x1415242e0;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x38u64)) as u64; rax = 0; fl_l = 0; fl_r = 0; r8 = P0; rdi = rcx; rcx = ((rax as u32)) as u32 as u64; bb = 1; continue; }
            1 => { rbx = (crate::simd_ops::ru32(r8)) as u32 as u64; fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 4; continue; } bb = 2; continue; }
            2 => { rcx = (((rcx as u32)).wrapping_add(1)) as u32 as u64; r8 = ((r8).wrapping_add(0x4u64)) as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1b; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 1; continue; } bb = 3; continue; }
            3 => { rax = 0x2; rsp = ((rsp).wrapping_add(0x38u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            4 => { fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 3; continue; } bb = 5; continue; }
            5 => { crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rbp); rdx = ((rbx.wrapping_mul(8)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rsi); rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), r12); rsi = rbx; r12 = (crate::simd_ops::ru32(rdi.wrapping_add(0xcu64))) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), r13); r13 = (crate::simd_ops::ru64(rdi)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), r14); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r15); r15 = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; crate::simd_ops::wu64(rdi.wrapping_add(0x8u64), rax); crate::simd_ops::wu64(rdi, rax); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru64(rdi)) as u64; rbp = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 9; continue; } bb = 6; continue; }
            6 => { fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { r8 = 0; fl_l = 0; fl_r = 0; rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rdi.wrapping_add(0x8u64), ((r8 as u32)) as u32); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 8; continue; }
            8 => { crate::simd_ops::wu64(rdi, r13); rax = 0x2; crate::simd_ops::wu32(rdi.wrapping_add(0x8u64), ((r15 as u32)) as u32); crate::simd_ops::wu32(rdi.wrapping_add(0xcu64), ((r12 as u32)) as u32); bb = 26; continue; }
            9 => { fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 10; continue; }
            10 => { fl_l = rdx; fl_r = rbp; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 12; continue; } bb = 11; continue; }
            11 => { rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 12; continue; }
            12 => { rdx = rdi; rcx = rdi.wrapping_add(0xc); rax = ((rbx as u32)) as u32 as u64; r8 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rdx, rbp); rbp = ((rbx as u32)) as u32 as u64; crate::simd_ops::wu32(rcx, ((rbx as u32)) as u32); bb = 13; continue; }
            13 => { rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 16; continue; } bb = 14; continue; }
            14 => { rax = ((rcx.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x8u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rdi)) as u64; rdx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 16; continue; } bb = 15; continue; }
            15 => { crate::simd_ops::wu64(rdx, r8); rbx = (crate::simd_ops::ru32(rdi.wrapping_add(0xcu64))) as u32 as u64; bb = 16; continue; }
            16 => { fl_l = rsi; fl_r = (0x1u64) as u64; rsi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 13; continue; } bb = 17; continue; }
            17 => { fl_l = ((r15 as u32)) as u32 as u64; fl_r = ((r15 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 23; continue; } bb = 18; continue; }
            18 => { r10 = r13; r11 = r15; bb = 19; continue; }
            19 => { r9 = (crate::simd_ops::ru64(r10)) as u64; fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 22; continue; } bb = 20; continue; }
            20 => { bb = 21; continue; }
            21 => { r8 = (crate::simd_ops::ru64(r9.wrapping_add(0x70u64))) as u64; rax = ((r9 as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shr(16u32)) as u32 as u64; rdx = 0; fl_l = 0; fl_r = 0; rax = ((((rax as u32)) ^ ((r9 as u32)))) as u32 as u64; rcx = ((((rax as u32)) as i32).wrapping_mul((0x85ebca6bu32) as i32) as u32) as u32 as u64; rax = ((rcx as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shr(13u32)) as u32 as u64; rax = ((((rax as u32)) ^ ((rcx as u32)))) as u32 as u64; rcx = ((((rax as u32)) as i32).wrapping_mul((0xc2b2ae35u32) as i32) as u32) as u32 as u64; rax = ((rcx as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shr(16u32)) as u32 as u64; rax = ((((rax as u32)) ^ ((rcx as u32)))) as u32 as u64; let d = ((rbp as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } rax = (crate::simd_ops::ru64(rdi)) as u64; rdx = ((rdx.wrapping_mul(8)) as u64) as u64; rcx = (crate::simd_ops::ru64(rdx.wrapping_add(rax))) as u64; crate::simd_ops::wu64(r9.wrapping_add(0x70u64), rcx); rax = (crate::simd_ops::ru64(rdi)) as u64; crate::simd_ops::wu64(rdx.wrapping_add(rax), r9); r9 = r8; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 21; continue; } bb = 22; continue; }
            22 => { r10 = ((r10).wrapping_add(0x8u64)) as u64; fl_l = r11; fl_r = (0x1u64) as u64; r11 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 19; continue; } bb = 23; continue; }
            23 => { fl_l = r13; fl_r = r13; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 25; continue; } bb = 24; continue; }
            24 => { rdx = r13; rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 25; continue; }
            25 => { rax = 0x1; bb = 26; continue; }
            26 => { r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rsp = ((rsp).wrapping_add(0x38u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415f4270(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x4; const P2: u64 = 0xc; const P3: u64 = 0x1415afa10; const P4: u64 = 0x1;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; r15 = r8; r14 = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; rdi = ((rdx as u32)) as u32 as u64; rsi = rcx; r9 = ((r14.wrapping_add(r14.wrapping_mul(2))) as u64) as u64; r9 = ((rax.wrapping_add(r9.wrapping_mul(4))) as u64) as u64; fl_l = rax; fl_r = r9; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 3; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru32(rax)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rax = ((rax).wrapping_add(0xcu64)) as u64; fl_l = rax; fl_r = r9; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 3; continue; }
            3 => { rbp = 0; fl_l = 0; fl_r = 0; rbx = ((rax.wrapping_add(P1)) as u64) as u64; fl_l = rax; fl_r = r9; rbx = (if (((fl_l as u64)) == ((fl_r as u64))) { rbp } else { rbx }) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(r8)) as u64; crate::simd_ops::wu64(rbx, rax); bb = 5; continue; }
            5 => { rax = rbx; bb = 12; continue; }
            6 => { rax = (crate::simd_ops::ru32(rcx.wrapping_add(P2))) as u32 as u64; fl_l = r14; fl_r = rax; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 8; continue; } bb = 7; continue; }
            7 => { rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 5; continue; } bb = 8; continue; }
            8 => { rax = (crate::simd_ops::ru32(rsi.wrapping_add(P2))) as u32 as u64; fl_l = r14; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 11; continue; } bb = 9; continue; }
            9 => { rcx = (crate::simd_ops::ru32(rsi.wrapping_add(P0))) as u32 as u64; rax = ((rcx.wrapping_add(P4)) as u32) as u32 as u64; crate::simd_ops::wu32(rsi.wrapping_add(P0), ((rax as u32)) as u32); rcx = ((rcx.wrapping_add(rcx.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru64(rsi)) as u64; rax = ((rax.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { crate::simd_ops::wu32(rax, ((rdi as u32)) as u32); fl_l = ((rax).wrapping_add(0x4u64)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; rcx = (crate::simd_ops::ru64(r15)) as u64; crate::simd_ops::wu64(rax, rcx); bb = 12; continue; }
            11 => { rax = rbp; bb = 12; continue; }
            12 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415f4330(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415f2fc0; const P1: u64 = 0x1415d8150; const P2: u64 = 0x1415d8040;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbp = rcx; rbx = r8; rcx = (crate::simd_ops::ru32(rcx.wrapping_add(0x30u64))) as u32 as u64; rdi = rdx; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 24; continue; } bb = 1; continue; }
            1 => { rdx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rsi); rax = ((rdi as u32)) as u32 as u64; rsi = 0; fl_l = 0; fl_r = 0; let d = ((rcx as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x28u64))) as u64; rdx = (crate::simd_ops::ru64(rax.wrapping_add(rdx.wrapping_mul(8)))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 2; continue; }
            2 => { bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 16; continue; } bb = 4; continue; }
            4 => { rdx = (crate::simd_ops::ru64(rdx)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } bb = 5; continue; }
            5 => { rcx = rsi; bb = 6; continue; }
            6 => { fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 23; continue; } bb = 7; continue; }
            7 => { rcx = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); r8 = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 23; continue; } bb = 8; continue; }
            8 => { fl_l = rbx; fl_r = (0xffffffffffffffffu64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 22; continue; } bb = 9; continue; }
            9 => { fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 22; continue; } bb = 10; continue; }
            10 => { rcx = (crate::simd_ops::ru32(rbp.wrapping_add(0x30u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 23; continue; } bb = 11; continue; }
            11 => { rdx = 0; fl_l = 0; fl_r = 0; rax = ((rbx as u32)) as u32 as u64; let d = ((rcx as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x28u64))) as u64; rdx = (crate::simd_ops::ru64(rax.wrapping_add(rdx.wrapping_mul(8)))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 18; continue; } bb = 12; continue; }
            12 => { bb = 13; continue; }
            13 => { fl_l = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; fl_r = rbx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 17; continue; } bb = 14; continue; }
            14 => { rdx = (crate::simd_ops::ru64(rdx)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 13; continue; } bb = 15; continue; }
            15 => { bb = 18; continue; }
            16 => { rcx = rdx.wrapping_add(0x10); bb = 6; continue; }
            17 => { rsi = rdx.wrapping_add(0x10); bb = 18; continue; }
            18 => { fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 23; continue; } bb = 19; continue; }
            19 => { rdx = (crate::simd_ops::ru64(rsi)) as u64; rcx = r8; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 20; continue; }
            20 => { rax = 0x1; bb = 21; continue; }
            21 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            22 => { rcx = r8; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 20; continue; }
            23 => { rax = 0x2; bb = 21; continue; }
            24 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rax = 0x2; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415f4430(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415f2e80; const P1: u64 = 0x1415f2fc0; const P2: u64 = 0x141604370; const P3: u64 = 0x1415f2f20; const P4: u64 = 0x1415f33b0;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x28u64)) as u64; r11 = (crate::simd_ops::ru32(rcx.wrapping_add(0x30u64))) as u32 as u64; rsi = r8; r10 = rdx; rdi = rcx; fl_l = ((r11 as u32)) as u32 as u64; fl_r = ((r11 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 22; continue; } bb = 1; continue; }
            1 => { rdx = 0; fl_l = 0; fl_r = 0; rax = ((r10 as u32)) as u32 as u64; let d = ((r11 as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x28u64))) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(rdx.wrapping_mul(8)))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; fl_r = r10; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 23; continue; } bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(rcx)) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 4; continue; }
            4 => { rcx = 0; fl_l = 0; fl_r = 0; bb = 5; continue; }
            5 => { fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 22; continue; } bb = 6; continue; }
            6 => { crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r14); r14 = (crate::simd_ops::ru64(rcx)) as u64; rcx = r14; crate::simd_ops::wu32(rsp.wrapping_add(0x40u64), ((r9 as u32)) as u32); rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rbp = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 21; continue; } bb = 7; continue; }
            7 => { r9 = (crate::simd_ops::ru32(rsp.wrapping_add(0x40u64))) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rbx); rbx = 0; fl_l = 0; fl_r = 0; fl_l = ((r9 as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 19; continue; } bb = 8; continue; }
            8 => { bb = 9; continue; }
            9 => { rcx = ((rbx as u32)) as u32 as u64; rcx = ((rcx).wrapping_add(rcx)) as u64; r8 = (crate::simd_ops::ru64(rsi.wrapping_add(rcx.wrapping_mul(8)))) as u64; fl_l = r8; fl_r = (0xffffffffffffffffu64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 18; continue; } bb = 10; continue; }
            10 => { rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x30u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 18; continue; } bb = 11; continue; }
            11 => { rdx = 0; fl_l = 0; fl_r = 0; rax = ((r8 as u32)) as u32 as u64; let d = ((rcx as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x28u64))) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(rdx.wrapping_mul(8)))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 15; continue; } bb = 12; continue; }
            12 => { bb = 13; continue; }
            13 => { fl_l = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; fl_r = r8; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 24; continue; } bb = 14; continue; }
            14 => { rax = (crate::simd_ops::ru64(rax)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 13; continue; } bb = 15; continue; }
            15 => { rcx = 0; fl_l = 0; fl_r = 0; bb = 16; continue; }
            16 => { fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 18; continue; } bb = 17; continue; }
            17 => { rcx = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); r9 = (crate::simd_ops::ru32(rsp.wrapping_add(0x40u64))) as u32 as u64; bb = 18; continue; }
            18 => { rbx = (((rbx as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 9; continue; } bb = 19; continue; }
            19 => { r8 = rsp.wrapping_add(0x40); rdx = rsi; rcx = rbp; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = r14; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 21; continue; } bb = 20; continue; }
            20 => { crate::simd_ops::lk_or_u32(rax.wrapping_add(0x7cu64), (0x1u32) as u32); rdx = rbp.wrapping_add(0xc0); rcx = rax.wrapping_add(0x20); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 21; continue; }
            21 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; bb = 22; continue; }
            22 => { rsp = ((rsp).wrapping_add(0x28u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            23 => { fl_l = ((rcx).wrapping_add(0x10u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; bb = 5; continue; }
            24 => { rcx = rax.wrapping_add(0x10); bb = 16; continue; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415f4570(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415f2e80; const P1: u64 = 0x1416042c0; const P2: u64 = 0x1415f2fc0; const P3: u64 = 0x141604890; const P4: u64 = 0x1415f33b0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rbp = rcx; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm6) as u128); rcx = (crate::simd_ops::ru32(rcx.wrapping_add(0x30u64))) as u32 as u64; xmm6 = (xmm3) as u128; rbx = r8; r9 = rdx; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 24; continue; } bb = 1; continue; }
            1 => { rdx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rdi); rax = ((r9 as u32)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), r14); let d = ((rcx as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x28u64))) as u64; r14 = 0; fl_l = 0; fl_r = 0; rcx = (crate::simd_ops::ru64(rax.wrapping_add(rdx.wrapping_mul(8)))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; fl_r = r9; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 9; continue; } bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(rcx)) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 4; continue; }
            4 => { rdi = r14; bb = 5; continue; }
            5 => { fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 23; continue; } bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru64(rdi)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rsi); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rsi = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 22; continue; } bb = 7; continue; }
            7 => { fl_l = rbx; fl_r = (0xffffffffffffffffu64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 10; continue; } bb = 8; continue; }
            8 => { xmm1 = (xmm6) as u128; rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 20; continue; }
            9 => { rdi = rcx.wrapping_add(0x10); bb = 5; continue; }
            10 => { rcx = (crate::simd_ops::ru32(rbp.wrapping_add(0x30u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 19; continue; } bb = 11; continue; }
            11 => { rdx = 0; fl_l = 0; fl_r = 0; rax = ((rbx as u32)) as u32 as u64; let d = ((rcx as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x28u64))) as u64; rdx = (crate::simd_ops::ru64(rax.wrapping_add(rdx.wrapping_mul(8)))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 12; continue; }
            12 => { bb = 13; continue; }
            13 => { fl_l = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; fl_r = rbx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 16; continue; } bb = 14; continue; }
            14 => { rdx = (crate::simd_ops::ru64(rdx)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 13; continue; } bb = 15; continue; }
            15 => { bb = 17; continue; }
            16 => { r14 = rdx.wrapping_add(0x10); bb = 17; continue; }
            17 => { fl_l = r14; fl_r = r14; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 19; continue; } bb = 18; continue; }
            18 => { rcx = (crate::simd_ops::ru64(r14)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 19; continue; }
            19 => { xmm2 = (xmm6) as u128; rdx = rbx; rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 20; continue; }
            20 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 22; continue; } bb = 21; continue; }
            21 => { crate::simd_ops::lk_or_u32(rcx.wrapping_add(0x7cu64), (0x1u32) as u32); rdx = rsi.wrapping_add(0xc0); rcx = ((rcx).wrapping_add(0x20u64)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 22; continue; }
            22 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; bb = 23; continue; }
            23 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; bb = 24; continue; }
            24 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415f4690(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x30; const P1: u64 = 0x28; const P2: u64 = 0x8; const P3: u64 = 0x7c; const P4: u64 = 0x78; const P5: u64 = 0x20; const P6: u64 = 0x1415d8820;
    let (mut rax, mut r10, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x28u64)) as u64; r8 = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; r9 = rdx; r10 = 0x2; fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 12; continue; } bb = 1; continue; }
            1 => { rdx = 0; fl_l = 0; fl_r = 0; rax = ((r9 as u32)) as u32 as u64; let d = ((r8 as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } rax = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(rdx.wrapping_mul(8)))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 2; continue; }
            2 => { bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64; fl_r = r9; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 11; continue; } bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru64(rcx)) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } bb = 5; continue; }
            5 => { rcx = 0; fl_l = 0; fl_r = 0; bb = 6; continue; }
            6 => { fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { fl_l = (crate::simd_ops::lk_or_u32(rdx.wrapping_add(P3), (0x1u32) as u32)) as u32 as u64; fl_r = fl_l; r10 = 0x1; crate::simd_ops::wu32(rdx.wrapping_add(P4), ((xmm2 as u32)) as u32); bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(P5))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 10; continue; }
            10 => { xmm1 = (xmm2) as u128; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = 0x1; rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
            11 => { fl_l = ((rcx).wrapping_add(0x10u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; bb = 6; continue; }
            12 => { rax = ((r10 as u32)) as u32 as u64; rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415f4720(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415f2fc0; const P1: u64 = 0x1415f2f20; const P2: u64 = 0x141605890;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; r10 = rcx; rbx = r8; rcx = (crate::simd_ops::ru32(rcx.wrapping_add(0x30u64))) as u32 as u64; rsi = r9; r8 = rdx; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 15; continue; } bb = 1; continue; }
            1 => { rdx = 0; fl_l = 0; fl_r = 0; rax = ((r8 as u32)) as u32 as u64; let d = ((rcx as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } rax = (crate::simd_ops::ru64(r10.wrapping_add(0x28u64))) as u64; rdx = (crate::simd_ops::ru64(rax.wrapping_add(rdx.wrapping_mul(8)))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; fl_r = r8; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 10; continue; } bb = 3; continue; }
            3 => { rdx = (crate::simd_ops::ru64(rdx)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 4; continue; }
            4 => { rdx = 0; fl_l = 0; fl_r = 0; bb = 5; continue; }
            5 => { fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 15; continue; } bb = 6; continue; }
            6 => { rax = ((rbx as u32)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rdi); rdi = (crate::simd_ops::ru64(rdx)) as u64; rdx = 0; fl_l = 0; fl_r = 0; let d = ((rcx as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } rax = (crate::simd_ops::ru64(r10.wrapping_add(0x28u64))) as u64; rdx = (crate::simd_ops::ru64(rax.wrapping_add(rdx.wrapping_mul(8)))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 7; continue; }
            7 => { fl_l = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; fl_r = rbx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 11; continue; } bb = 8; continue; }
            8 => { rdx = (crate::simd_ops::ru64(rdx)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 7; continue; } bb = 9; continue; }
            9 => { bb = 13; continue; }
            10 => { fl_l = ((rdx).wrapping_add(0x10u64)) as u64; fl_r = fl_l; rdx = ((fl_l as u64)) as u64; bb = 5; continue; }
            11 => { rcx = rdx.wrapping_add(0x10); fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 12; continue; }
            12 => { rcx = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); bb = 13; continue; }
            13 => { rcx = rdi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 15; continue; } bb = 14; continue; }
            14 => { r9 = (crate::simd_ops::ru32(rsp.wrapping_add(0x50u64))) as u32 as u64; r8 = rsi; rdx = rbx; rcx = rax; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P2, rax, rcx, rdx, r8, r9, rsp); }
            15 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rax = 0x2; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415f47f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x20; const P1: u64 = 0xd4; const P2: u64 = 0x8; const P3: u64 = 0x4; const P4: u64 = 0x10; const P5: u64 = 0x141524330; const P6: u64 = 0x24;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; r8 = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64; rbx = rcx; rcx = ((crate::simd_ops::ru8(rdx.wrapping_add(P1))) as u32) as u32 as u64; rcx = (rcx & !0xffu64) | (((((rcx as u8)).wrapping_shr(7u32)) as u8 as u64)); crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((r8 as u32)) as u32); rax = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x34u64), ((rcx as u8)) as u8); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 1; continue; }
            1 => { bb = 2; continue; }
            2 => { fl_l = ((r8 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rax.wrapping_add(P2))) as u32 as u64; rdx = ((rax.wrapping_add(P2)) as u64) as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { fl_l = ((rcx as u8)) as u8 as u64; fl_r = (crate::simd_ops::ru8(rdx.wrapping_add(P3))) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 6; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rax)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 5; continue; }
            5 => { bb = 7; continue; }
            6 => { fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 14; continue; } bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(P4))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 10; continue; } bb = 8; continue; }
            8 => { rdx = ((rax.wrapping_add(P4)) as u32) as u32 as u64; rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 9; continue; }
            9 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P4))) as u64; crate::simd_ops::wu64(rax, rcx); crate::simd_ops::wu64(rbx.wrapping_add(P4), rax); bb = 10; continue; }
            10 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 12; continue; } bb = 11; continue; }
            11 => { crate::simd_ops::wu64(rbx, rax); bb = 13; continue; }
            12 => { crate::simd_ops::wu64(rcx, rax); bb = 13; continue; }
            13 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P4))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(P2), rcx); rax = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::wu64(rbx.wrapping_add(P4), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu64(rcx, (0x0u64) as u64); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64; fl_l = (crate::simd_ops::lk_add_u32(rbx.wrapping_add(P6), (1) as u32)) as u32 as u64; crate::simd_ops::wu64(rcx.wrapping_add(P2), rax); bb = 14; continue; }
            14 => { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415f48a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415f2fc0; const P1: u64 = 0x1415f2f20; const P2: u64 = 0x141605960;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; r10 = rcx; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm6) as u128); rcx = (crate::simd_ops::ru32(rcx.wrapping_add(0x30u64))) as u32 as u64; rbx = r8; xmm6 = (xmm3) as u128; r8 = rdx; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 15; continue; } bb = 1; continue; }
            1 => { rdx = 0; fl_l = 0; fl_r = 0; rax = ((r8 as u32)) as u32 as u64; let d = ((rcx as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } rax = (crate::simd_ops::ru64(r10.wrapping_add(0x28u64))) as u64; rdx = (crate::simd_ops::ru64(rax.wrapping_add(rdx.wrapping_mul(8)))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; fl_r = r8; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 10; continue; } bb = 3; continue; }
            3 => { rdx = (crate::simd_ops::ru64(rdx)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 4; continue; }
            4 => { rdx = 0; fl_l = 0; fl_r = 0; bb = 5; continue; }
            5 => { fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 15; continue; } bb = 6; continue; }
            6 => { rax = ((rbx as u32)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rdi); rdi = (crate::simd_ops::ru64(rdx)) as u64; rdx = 0; fl_l = 0; fl_r = 0; let d = ((rcx as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } rax = (crate::simd_ops::ru64(r10.wrapping_add(0x28u64))) as u64; rdx = (crate::simd_ops::ru64(rax.wrapping_add(rdx.wrapping_mul(8)))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 7; continue; }
            7 => { fl_l = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; fl_r = rbx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 11; continue; } bb = 8; continue; }
            8 => { rdx = (crate::simd_ops::ru64(rdx)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 7; continue; } bb = 9; continue; }
            9 => { bb = 13; continue; }
            10 => { fl_l = ((rdx).wrapping_add(0x10u64)) as u64; fl_r = fl_l; rdx = ((fl_l as u64)) as u64; bb = 5; continue; }
            11 => { rcx = rdx.wrapping_add(0x10); fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 12; continue; }
            12 => { rcx = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 13; continue; }
            13 => { rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 15; continue; } bb = 14; continue; }
            14 => { xmm3 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x60u64))) as u32) as u128; xmm2 = (xmm6) as u128; rdx = rbx; rcx = rax; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); }
            15 => { xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; rax = 0x2; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415f4970(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141605b80; const P1: u64 = 0x1415d87e0;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; r10 = (crate::simd_ops::ru32(rcx.wrapping_add(0x30u64))) as u32 as u64; rsi = ((r9 as u32)) as u32 as u64; rbp = r8; r11 = rdx; fl_l = ((r10 as u32)) as u32 as u64; fl_r = ((r10 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 17; continue; } bb = 1; continue; }
            1 => { rdx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx); rax = ((r11 as u32)) as u32 as u64; let d = ((r10 as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x28u64))) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(rdx.wrapping_mul(8)))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 2; continue; }
            2 => { bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; fl_r = r11; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 15; continue; } bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru64(rcx)) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } bb = 5; continue; }
            5 => { rbx = 0; fl_l = 0; fl_r = 0; bb = 6; continue; }
            6 => { fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 16; continue; } bb = 7; continue; }
            7 => { fl_l = (crate::simd_ops::ru8(rsp.wrapping_add(0x58u64))) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 10; continue; } bb = 8; continue; }
            8 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { r9 = ((crate::simd_ops::ru8(rsp.wrapping_add(0x50u64))) as u32) as u32 as u64; r8 = ((rsi as u32)) as u32 as u64; rdx = rbp; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); bb = 10; continue; }
            10 => { fl_l = (crate::simd_ops::ru8(rsp.wrapping_add(0x58u64))) as u8 as u64; fl_r = (0x2u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 14; continue; } bb = 11; continue; }
            11 => { fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 14; continue; } bb = 12; continue; }
            12 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x20u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 13; continue; }
            13 => { rdx = rbp; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 14; continue; }
            14 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = 0x1; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            15 => { rbx = rcx.wrapping_add(0x10); bb = 6; continue; }
            16 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = 0x2; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            17 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rax = 0x2; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415f4a50(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415f3060; const P1: u64 = 0x20; const P2: u64 = 0x1415f4270;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = r9;
    rdi = ((r8 as u32)) as u32 as u64;
    fl_l = rdx; fl_r = rdx;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = 0x2; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    rcx = rdx;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    fl_l = rax; fl_r = rax;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rcx = ((rax.wrapping_add(P1)) as u64) as u64; r8 = rbx; rdx = ((rdi as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); fl_l = 0; fl_r = rax; rax = (0u64.wrapping_sub(fl_r as u64)) as u64; rax = (if (((fl_l as u64)) < ((fl_r as u64))) { 0xffffffffu32 } else { 0u32 }) as u32 as u64; rax = (((rax as u32)).wrapping_add(0x2u32)) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rax = 0x2;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415f4ab0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415f4d20; const P1: u64 = 0x1415242e0; const P2: u64 = 0x1415af340; const P3: u64 = 0x143931c68; const P4: u64 = 0x143931c70; const P5: u64 = 0x143931c74;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx); rdx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rsi); rbx = rcx; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rdi); rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x30u64))) as u32 as u64; rbp = 0; fl_l = 0; fl_r = 0; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 12; continue; } bb = 1; continue; }
            1 => { rdx = 0; fl_l = 0; fl_r = 0; rax = 0xfffffffe; let d = ((rcx as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x28u64))) as u64; r8 = ((rax.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64; rax = ((rbp as u32)) as u32 as u64; rdx = (crate::simd_ops::ru64(r8)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; fl_r = (0xfffffffffffffffeu64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 5; continue; } bb = 3; continue; }
            3 => { rax = rdx; rdx = (crate::simd_ops::ru64(rdx)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 4; continue; }
            4 => { bb = 12; continue; }
            5 => { rdi = (crate::simd_ops::ru64(rdx.wrapping_add(0x10u64))) as u64; rcx = (crate::simd_ops::ru64(rdx)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { crate::simd_ops::wu64(rax, rcx); bb = 8; continue; }
            7 => { crate::simd_ops::wu64(r8, rcx); bb = 8; continue; }
            8 => { rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::lk_sub_u32(rbx.wrapping_add(0x38u64), (1) as u32); fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x60u64))) as u32 as u64; fl_r = 0x1; crate::simd_ops::wu32(rdi.wrapping_add(0x60u64), ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32); crate::simd_ops::wu8(rdi.wrapping_add(0x64u64), ((rbp as u8)) as u8); if (((fl_l as u32)) != ((fl_r as u32))) { bb = 10; continue; } bb = 9; continue; }
            9 => { rcx = rdi; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 10; continue; }
            10 => { fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x60u64))) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 12; continue; } bb = 11; continue; }
            11 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rdi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rdx = rdi; rcx = 0x4; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 12; continue; }
            12 => { rsi = ((rbp as u32)) as u32 as u64; fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x30u64))) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 19; continue; } bb = 13; continue; }
            13 => { crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), r14); bb = 14; continue; }
            14 => { rax = ((rsi as u32)) as u32 as u64; r14 = ((rax.wrapping_mul(8)) as u64) as u64; rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x28u64))) as u64; rdx = (crate::simd_ops::ru64(r14.wrapping_add(rax))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 15; continue; }
            15 => { bb = 16; continue; }
            16 => { rdi = (crate::simd_ops::ru64(rdx)) as u64; rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rdx = rdi; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 16; continue; } bb = 17; continue; }
            17 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x28u64))) as u64; rsi = (((rsi as u32)).wrapping_add(1)) as u32 as u64; crate::simd_ops::wu64(r14.wrapping_add(rax), rbp); fl_l = ((rsi as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rbx.wrapping_add(0x30u64))) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 14; continue; } bb = 18; continue; }
            18 => { r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; bb = 19; continue; }
            19 => { crate::simd_ops::wu32(rbx.wrapping_add(0x38u64), ((rbp as u32)) as u32); rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x28u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 21; continue; } bb = 20; continue; }
            20 => { rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rbx.wrapping_add(0x30u64), ((rbp as u32)) as u32); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(0x28u64), rbp); crate::simd_ops::wu32(rbx.wrapping_add(0x34u64), ((rbp as u32)) as u32); bb = 21; continue; }
            21 => { rdx = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 28; continue; } bb = 22; continue; }
            22 => { rax = (crate::simd_ops::ru64(rdx)) as u64; crate::simd_ops::wu64(rbx, rax); fl_l = rdx; fl_r = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 24; continue; } bb = 23; continue; }
            23 => { crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), rbp); bb = 24; continue; }
            24 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; fl_l = rdx; fl_r = rcx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 27; continue; } bb = 25; continue; }
            25 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x20u64))) as u32 as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rax = ((rax).wrapping_add(rcx)) as u64; fl_l = rdx; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 27; continue; } bb = 26; continue; }
            26 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; crate::simd_ops::wu64(rdx, rax); fl_l = (crate::simd_ops::lk_sub_u32(rbx.wrapping_add(0x24u64), (1) as u32)) as u32 as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), rdx); bb = 21; continue; }
            27 => { rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::lk_sub_u32(rbx.wrapping_add(0x24u64), (1) as u32)) as u32 as u64; bb = 21; continue; }
            28 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 35; continue; } bb = 29; continue; }
            29 => { bb = 30; continue; }
            30 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; rdi = rbx.wrapping_add(0x20); rsi = (crate::simd_ops::ru64(rdx)) as u64; fl_l = rdx; fl_r = rcx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 32; continue; } bb = 31; continue; }
            31 => { rax = (crate::simd_ops::ru32(rdi)) as u32 as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rax = ((rax).wrapping_add(rcx)) as u64; fl_l = rdx; fl_r = rax; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 33; continue; } bb = 32; continue; }
            32 => { rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 33; continue; }
            33 => { rdx = rsi; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 30; continue; } bb = 34; continue; }
            34 => { bb = 36; continue; }
            35 => { rdi = rbx.wrapping_add(0x20); bb = 36; continue; }
            36 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 38; continue; } bb = 37; continue; }
            37 => { rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(0x18u64), rbp); bb = 38; continue; }
            38 => { crate::simd_ops::wu32(rdi, ((rbp as u32)) as u32); rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; crate::simd_ops::wu64(rbx, rbp); crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), rbp); crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), rbp); rdx = (crate::simd_ops::ru64(P3)) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 40; continue; } bb = 39; continue; }
            39 => { rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(P4, ((rbp as u32)) as u32); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(P3, rbp); crate::simd_ops::wu32(P5, ((rbp as u32)) as u32); bb = 40; continue; }
            40 => { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415f4cb0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x1415242e0; const P3: u64 = 0x14;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp);
    crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rax = (crate::simd_ops::ru64(rdx)) as u64;
    rdi = rdx;
    crate::simd_ops::wu64(rcx, rax);
    rbp = 0;
    fl_l = 0; fl_r = 0;
    rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rbx = rcx;
    fl_l = rdx; fl_r = rdx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { crate::simd_ops::wu32(rcx.wrapping_add(P1), ((rbp as u32)) as u32); rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(P0), rbp); crate::simd_ops::wu32(rbx.wrapping_add(P3), ((rbp as u32)) as u32); }
    rax = (crate::simd_ops::ru64(rdi.wrapping_add(P0))) as u64;
    rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64;
    crate::simd_ops::wu64(rbx.wrapping_add(P0), rax);
    rax = (crate::simd_ops::ru32(rdi.wrapping_add(P1))) as u32 as u64;
    crate::simd_ops::wu32(rbx.wrapping_add(P1), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdi.wrapping_add(P3))) as u32 as u64;
    crate::simd_ops::wu32(rbx.wrapping_add(P3), ((rax as u32)) as u32);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    crate::simd_ops::wu64(rdi.wrapping_add(P0), rbp);
    crate::simd_ops::wu64(rdi.wrapping_add(P1), rbp);
    rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415f4d20(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141590920; const P1: u64 = 0x1415af340; const P2: u64 = 0x1415242e0;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = rsp; rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = ((rsp).wrapping_sub(0x58u64)) as u64; rbp = rcx.wrapping_add(0x28); r12 = rdx; rcx = (crate::simd_ops::ru32(rbp.wrapping_add(0x8u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 26; continue; } bb = 1; continue; }
            1 => { rdx = (crate::simd_ops::ru64(rbp)) as u64; crate::simd_ops::wu64(rax.wrapping_add(0x10u64), rsi); crate::simd_ops::wu64(rax.wrapping_add(0x18u64), rdi); crate::simd_ops::wu64(rax.wrapping_sub(0x18u64), r13); r13 = 0; fl_l = 0; fl_r = 0; rsi = (crate::simd_ops::ru64(rdx)) as u64; rdi = ((r13 as u32)) as u32 as u64; crate::simd_ops::wu64(rax.wrapping_sub(0x20u64), r14); r14 = rbp; crate::simd_ops::wu64(rax.wrapping_sub(0x28u64), r15); r15 = ((r13 as u32)) as u32 as u64; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 6; continue; } bb = 2; continue; }
            2 => { rdi = (((rdi as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 5; continue; } bb = 3; continue; }
            3 => { rsi = (crate::simd_ops::ru64(rdx.wrapping_add(rdi.wrapping_mul(8)))) as u64; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 4; continue; }
            4 => { bb = 6; continue; }
            5 => { fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 25; continue; } bb = 6; continue; }
            6 => { crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), rbx); bb = 7; continue; }
            7 => { rdx = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; rbx = (crate::simd_ops::ru64(rsi.wrapping_add(0x10u64))) as u64; fl_l = rdx; fl_r = (0xfffffffffffffffeu64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 27; continue; } bb = 8; continue; }
            8 => { fl_l = r12; fl_r = r12; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { rcx = r12; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 27; continue; } bb = 10; continue; }
            10 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x60u64))) as u32 as u64; fl_r = 0x1; crate::simd_ops::wu32(rbx.wrapping_add(0x60u64), ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32); crate::simd_ops::wu8(rbx.wrapping_add(0x64u64), ((r13 as u8)) as u8); if (((fl_l as u32)) != ((fl_r as u32))) { bb = 12; continue; } bb = 11; continue; }
            11 => { rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 12; continue; }
            12 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x60u64))) as u32 as u64; fl_r = ((r13 as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 14; continue; } bb = 13; continue; }
            13 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rbx; rcx = 0x4; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 14; continue; }
            14 => { r9 = (crate::simd_ops::ru64(rsi)) as u64; r8 = ((rdi as u32)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r14); crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rdi as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), r9); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), r15); fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 19; continue; } bb = 15; continue; }
            15 => { r10 = (crate::simd_ops::ru32(rbp.wrapping_add(0x8u64))) as u32 as u64; bb = 16; continue; }
            16 => { r8 = (((r8 as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((r10 as u32)) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 18; continue; } bb = 17; continue; }
            17 => { rax = (crate::simd_ops::ru64(r14)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), r13); rdx = (crate::simd_ops::ru64(rax.wrapping_add(r8.wrapping_mul(8)))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rdx); fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 16; continue; } bb = 18; continue; }
            18 => { crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((r8 as u32)) as u32); bb = 19; continue; }
            19 => { fl_l = r15; fl_r = r15; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 21; continue; } bb = 20; continue; }
            20 => { crate::simd_ops::wu64(r15, r9); bb = 22; continue; }
            21 => { rax = (crate::simd_ops::ru64(rbp)) as u64; rcx = ((rdi as u32)) as u32 as u64; crate::simd_ops::wu64(rax.wrapping_add(rcx.wrapping_mul(8)), r9); bb = 22; continue; }
            22 => { rdx = rsi; rcx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm1 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; fl_l = (crate::simd_ops::lk_sub_u32(rbp.wrapping_add(0x10u64), (1) as u32)) as u32 as u64; xmm2 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; xmm0 = (xmm1) as u128; rsi = ((xmm1 as u64)) as u64; xmm0 = crate::simd_ops::vpsrldq(xmm0, 0x8u8); r14 = ((xmm2 as u64)) as u64; r15 = ((xmm0 as u64)) as u64; xmm0 = (xmm2) as u128; xmm0 = crate::simd_ops::vpsrldq(xmm0, 0x8u8); rdi = ((xmm0 as u32)) as u32 as u64; bb = 23; continue; }
            23 => { fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 7; continue; } bb = 24; continue; }
            24 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; bb = 25; continue; }
            25 => { r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; bb = 26; continue; }
            26 => { rsp = ((rsp).wrapping_add(0x58u64)) as u64; r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            27 => { rax = (crate::simd_ops::ru64(rsi)) as u64; r15 = rsi; rsi = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 23; continue; } bb = 28; continue; }
            28 => { rdx = (crate::simd_ops::ru32(r14.wrapping_add(0x8u64))) as u32 as u64; bb = 29; continue; }
            29 => { rdi = (((rdi as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 23; continue; } bb = 30; continue; }
            30 => { rax = (crate::simd_ops::ru64(r14)) as u64; r15 = r13; rsi = (crate::simd_ops::ru64(rax.wrapping_add(rdi.wrapping_mul(8)))) as u64; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 29; continue; } bb = 31; continue; }
            31 => { bb = 7; continue; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415f4ef0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x30; const P1: u64 = 0x28; const P2: u64 = 0x8; const P3: u64 = 0x10; const P4: u64 = 0x1415242e0; const P5: u64 = 0x38; const P6: u64 = 0x60; const P7: u64 = 0x64; const P8: u64 = 0x1415af340;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = rcx; r9 = rdx; rcx = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 13; continue; } bb = 1; continue; }
            1 => { rdx = 0; fl_l = 0; fl_r = 0; rax = ((r9 as u32)) as u32 as u64; let d = ((rcx as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } rax = (crate::simd_ops::ru64(rdi.wrapping_add(P1))) as u64; r10 = ((rax.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64; rax = 0; fl_l = 0; fl_r = 0; rdx = (crate::simd_ops::ru64(r10)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru64(rdx.wrapping_add(P2))) as u64; fl_r = r9; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 5; continue; } bb = 3; continue; }
            3 => { rax = rdx; rdx = (crate::simd_ops::ru64(rdx)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 4; continue; }
            4 => { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            5 => { rcx = (crate::simd_ops::ru64(rdx)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx); rbx = (crate::simd_ops::ru64(rdx.wrapping_add(P3))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { crate::simd_ops::wu64(rax, rcx); bb = 8; continue; }
            7 => { crate::simd_ops::wu64(r10, rcx); bb = 8; continue; }
            8 => { rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::lk_sub_u32(rdi.wrapping_add(P5), (1) as u32); fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(P6))) as u32 as u64; fl_r = 0x1; crate::simd_ops::wu32(rbx.wrapping_add(P6), ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32); crate::simd_ops::wu8(rbx.wrapping_add(P7), (0x0u8) as u8); if (((fl_l as u32)) != ((fl_r as u32))) { bb = 10; continue; } bb = 9; continue; }
            9 => { rcx = rbx; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); bb = 10; continue; }
            10 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(P6))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 12; continue; } bb = 11; continue; }
            11 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rdx = rbx; rcx = 0x4; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 12; continue; }
            12 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; bb = 13; continue; }
            13 => { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415f4fa0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x18; const P2: u64 = 0x48; const P3: u64 = 0x143931c70; const P4: u64 = 0x143931c68; const P5: u64 = 0x70; const P6: u64 = 0x143931c78;
    let (mut rax, mut r10, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(P1))) as u64; r8 = 0; fl_l = 0; fl_r = 0; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu16(rax.wrapping_add(P2), ((r8 as u16)) as u16); bb = 3; continue; }
            3 => { rax = ((rcx as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shr(16u32)) as u32 as u64; rax = ((((rax as u32)) ^ ((rcx as u32)))) as u32 as u64; rcx = ((((rax as u32)) as i32).wrapping_mul((0x85ebca6bu32) as i32) as u32) as u32 as u64; rax = ((rcx as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shr(13u32)) as u32 as u64; rax = ((((rax as u32)) ^ ((rcx as u32)))) as u32 as u64; rcx = (crate::simd_ops::ru32(P3)) as u32 as u64; r9 = ((((rax as u32)) as i32).wrapping_mul((0xc2b2ae35u32) as i32) as u32) as u32 as u64; rax = ((r9 as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shr(16u32)) as u32 as u64; r9 = ((((r9 as u32)) ^ ((rax as u32)))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 13; continue; } bb = 4; continue; }
            4 => { rdx = 0; fl_l = 0; fl_r = 0; rax = ((r9 as u32)) as u32 as u64; let d = ((rcx as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } rax = (crate::simd_ops::ru64(P4)) as u64; r10 = ((rax.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(rdx.wrapping_mul(8)))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 5; continue; }
            5 => { bb = 6; continue; }
            6 => { rcx = ((rax as u32)) as u32 as u64; rcx = (((rcx as u32)).wrapping_shr(16u32)) as u32 as u64; rcx = ((((rcx as u32)) ^ ((rax as u32)))) as u32 as u64; rdx = ((((rcx as u32)) as i32).wrapping_mul((0x85ebca6bu32) as i32) as u32) as u32 as u64; rcx = ((rdx as u32)) as u32 as u64; rcx = (((rcx as u32)).wrapping_shr(13u32)) as u32 as u64; rcx = ((((rcx as u32)) ^ ((rdx as u32)))) as u32 as u64; rdx = ((((rcx as u32)) as i32).wrapping_mul((0xc2b2ae35u32) as i32) as u32) as u32 as u64; rcx = ((rdx as u32)) as u32 as u64; rcx = (((rcx as u32)).wrapping_shr(16u32)) as u32 as u64; rcx = ((((rcx as u32)) ^ ((rdx as u32)))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 9; continue; } bb = 7; continue; }
            7 => { r8 = rax; rax = (crate::simd_ops::ru64(rax.wrapping_add(P5))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 6; continue; } bb = 8; continue; }
            8 => { return rax; }
            9 => { fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 10; continue; }
            10 => { rcx = (crate::simd_ops::ru64(rax.wrapping_add(P5))) as u64; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 11; continue; }
            11 => { crate::simd_ops::wu64(r8.wrapping_add(P5), rcx); fl_l = (crate::simd_ops::lk_sub_u32(P6, (1) as u32)) as u32 as u64; return rax; }
            12 => { crate::simd_ops::wu64(r10, rcx); fl_l = (crate::simd_ops::lk_sub_u32(P6, (1) as u32)) as u32 as u64; bb = 13; continue; }
            13 => { return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415f5070(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xffffffffffffffff; const P2: u64 = 0xc; const P3: u64 = 0x14183acc8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; r8 = (crate::simd_ops::ru64(rcx)) as u64; rbx = rcx; rcx = r8; r9 = (crate::simd_ops::ru32(rbx.wrapping_add(P0))) as u32 as u64; rax = ((r9.wrapping_add(r9.wrapping_mul(2))) as u64) as u64; rax = ((r8.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; fl_l = r8; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 7; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru32(rcx)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 4; continue; } bb = 2; continue; }
            2 => { rcx = ((rcx).wrapping_add(0xcu64)) as u64; fl_l = rcx; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 3; continue; }
            3 => { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            4 => { rax = ((r9.wrapping_add(P1)) as u32) as u32 as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rdx = ((r8.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; fl_l = rcx; fl_r = rdx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rdx = ((rdx).wrapping_sub(rcx)) as u64; rax = (0x2aaaaaaaaaaaaaabu64) as u64; let prod = (rax as i64 as i128).wrapping_mul((rdx) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rdx = ((((rdx) as i64).wrapping_shr(1u32) as u64)) as u64; rax = rdx; rax = ((rax).wrapping_shr(63u32)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; r8 = ((rdx.wrapping_add(rdx.wrapping_mul(2))) as u32) as u32 as u64; r8 = (((r8 as u32)).wrapping_shl(2u32)) as u32 as u64; rdx = ((rcx.wrapping_add(P2)) as u64) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            6 => { fl_l = (crate::simd_ops::lk_sub_u32(rbx.wrapping_add(P0), (1) as u32)) as u32 as u64; bb = 7; continue; }
            7 => { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415f50f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415f35b0; const P1: u64 = 0x14158c580; const P2: u64 = 0x1416046a0; const P3: u64 = 0x1416032b0; const P4: u64 = 0x141603dd0; const P5: u64 = 0x1415f1a90; const P6: u64 = 0x14392d4b8; const P7: u64 = 0x1415242e0; const P8: u64 = 0x1415f4cb0;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = ((r8 as u32)) as u32 as u64; rdi = rdx; fl_l = ((r8 as u32)) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            2 => { rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); r14 = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 28; continue; } bb = 3; continue; }
            3 => { fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rdx = rdi; rcx = rax; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 9; continue; }
            5 => { fl_l = ((rbx as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rdx = rdi; rcx = r14; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 9; continue; }
            7 => { fl_l = ((rbx as u32)) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 9; continue; } bb = 8; continue; }
            8 => { rdx = rdi; rcx = r14; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 9; continue; }
            9 => { rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rdi = (crate::simd_ops::ru64(P6)) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 28; continue; } bb = 10; continue; }
            10 => { crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), r15); r15 = 0; fl_l = 0; fl_r = 0; bb = 11; continue; }
            11 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 12; continue; }
            12 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0xe0u64))) as u8 as u64; fl_r = ((r15 as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 26; continue; } bb = 13; continue; }
            13 => { crate::simd_ops::lk_or_u32(rdi.wrapping_add(0x7cu64), (0x1u32) as u32); rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x30u64))) as u32 as u64; rbx = (crate::simd_ops::ru64(rdi.wrapping_add(0x28u64))) as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = ((rbx.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 26; continue; } bb = 14; continue; }
            14 => { bb = 15; continue; }
            15 => { rax = (crate::simd_ops::ru32(r14.wrapping_add(0xd8u64))) as u32 as u64; rcx = ((r15 as u32)) as u32 as u64; r8 = (crate::simd_ops::ru64(rbx)) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 19; continue; } bb = 16; continue; }
            16 => { rsi = (crate::simd_ops::ru64(r14.wrapping_add(0xd0u64))) as u64; bb = 17; continue; }
            17 => { r9 = ((rax as u32)) as u32 as u64; r11 = ((rcx as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shr(1u32)) as u32 as u64; rax = (((rax as u32)).wrapping_add((rcx as u32))) as u32 as u64; rcx = ((((rax as u32)) as i32 as i64) as u64) as u64; r10 = ((rax as u32)) as u32 as u64; fl_l = (crate::simd_ops::ru64(rsi.wrapping_add(rcx.wrapping_mul(8)))) as u64; fl_r = r8; rax = ((rsi.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 29; continue; } bb = 18; continue; }
            18 => { rax = ((r9.wrapping_sub(0x1u64)) as u32) as u32 as u64; rcx = ((r10.wrapping_add(0x1u64)) as u32) as u32 as u64; rax = (if (((fl_l as u64)) >= ((fl_r as u64))) { (r9 as u32) } else { (rax as u32) }) as u32 as u64; rcx = (if (((fl_l as u64)) >= ((fl_r as u64))) { (r11 as u32) } else { (rcx as u32) }) as u32 as u64; fl_l = (((rax as u32)).wrapping_shr(1u32)) as u32 as u64; rax = ((fl_l as u32)) as u32 as u64; if (((fl_l as u32)) != 0) { bb = 17; continue; } bb = 19; continue; }
            19 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 21; continue; } bb = 20; continue; }
            20 => { rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rbx.wrapping_add(0x10u64), ((r15 as u32)) as u32); rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), r15); crate::simd_ops::wu32(rbx.wrapping_add(0x14u64), ((r15 as u32)) as u32); bb = 21; continue; }
            21 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x30u64))) as u32 as u64; rbp = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x28u64))) as u64; rbp = rbp.wrapping_sub(0x3); rbp = ((rax.wrapping_add(rbp.wrapping_mul(8))) as u64) as u64; rax = rbx; fl_l = rbx; fl_r = rbp; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 24; continue; } bb = 22; continue; }
            22 => { bb = 23; continue; }
            23 => { rsi = rax.wrapping_add(0x18); rcx = rax; rdx = rsi; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); rax = rsi; fl_l = rsi; fl_r = rbp; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 23; continue; } bb = 24; continue; }
            24 => { fl_l = (crate::simd_ops::lk_sub_u32(rdi.wrapping_add(0x30u64), (1) as u32)) as u32 as u64; rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x30u64))) as u32 as u64; bb = 25; continue; }
            25 => { rax = ((rcx as u32)) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x28u64))) as u64; rcx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 15; continue; } bb = 26; continue; }
            26 => { rdi = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 11; continue; } bb = 27; continue; }
            27 => { r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; bb = 28; continue; }
            28 => { rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            29 => { rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x30u64))) as u32 as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 19; continue; } bb = 30; continue; }
            30 => { fl_l = ((rbx).wrapping_add(0x18u64)) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; bb = 25; continue; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415f5370(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415f2e80; const P1: u64 = 0x1415f35b0; const P2: u64 = 0x1416046a0; const P3: u64 = 0x1416032b0; const P4: u64 = 0x141603dd0; const P5: u64 = 0x1415f1a90; const P6: u64 = 0x1415f33b0;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; r14 = rcx; rbp = ((r9 as u32)) as u32 as u64; rcx = (crate::simd_ops::ru32(rcx.wrapping_add(0x30u64))) as u32 as u64; rsi = r8; r11 = rdx; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 19; continue; } bb = 1; continue; }
            1 => { rdx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx); rax = ((r11 as u32)) as u32 as u64; let d = ((rcx as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } rax = (crate::simd_ops::ru64(r14.wrapping_add(0x28u64))) as u64; r9 = (crate::simd_ops::ru64(rax.wrapping_add(rdx.wrapping_mul(8)))) as u64; fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru64(r9.wrapping_add(0x8u64))) as u64; fl_r = r11; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 10; continue; } bb = 3; continue; }
            3 => { r9 = (crate::simd_ops::ru64(r9)) as u64; fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 4; continue; }
            4 => { rbx = 0; fl_l = 0; fl_r = 0; bb = 5; continue; }
            5 => { fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 18; continue; } bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 18; continue; } bb = 7; continue; }
            7 => { crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rdi); rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rdi = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 8; continue; }
            8 => { fl_l = ((rbp as u32)) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 11; continue; } bb = 9; continue; }
            9 => { rdx = rsi; rcx = r14; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rdx = rsi; rcx = rdi; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 15; continue; }
            10 => { rbx = r9.wrapping_add(0x10); bb = 5; continue; }
            11 => { fl_l = ((rbp as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 13; continue; } bb = 12; continue; }
            12 => { rdx = rsi; rcx = r14; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rdx = rsi; rcx = rdi; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 15; continue; }
            13 => { fl_l = ((rbp as u32)) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 15; continue; } bb = 14; continue; }
            14 => { rdx = rsi; rcx = rdi; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 15; continue; }
            15 => { rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 16; continue; }
            16 => { crate::simd_ops::lk_or_u32(rcx.wrapping_add(0x7cu64), (0x1u32) as u32); rdx = rdi.wrapping_add(0xc0); rcx = ((rcx).wrapping_add(0x20u64)) as u64; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); bb = 17; continue; }
            17 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; bb = 18; continue; }
            18 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; bb = 19; continue; }
            19 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415f5480(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x1415864b0; const P2: u64 = 0x4; const P3: u64 = 0x8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    rdi = rdx;
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    fl_l = rcx; fl_r = rcx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); }
    rax = (crate::simd_ops::ru32(rdi)) as u32 as u64;
    crate::simd_ops::wu32(rbx, ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdi.wrapping_add(P2))) as u32 as u64;
    crate::simd_ops::wu32(rbx.wrapping_add(P2), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru64(rdi.wrapping_add(P3))) as u64;
    crate::simd_ops::wu64(rbx.wrapping_add(P3), rax);
    rax = (crate::simd_ops::ru64(rdi.wrapping_add(P0))) as u64;
    crate::simd_ops::wu64(rbx.wrapping_add(P0), rax);
    fl_l = rax; fl_r = rax;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { fl_l = (crate::simd_ops::lk_add_u32(rax, (1) as u32)) as u32 as u64; }
    rax = rbx;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415f54d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14268dfd8; const P1: u64 = 0x1415864b0; const P2: u64 = 0x1415242e0; const P3: u64 = 0x1415af340; const P4: u64 = 0x1426452a0; const P5: u64 = 0x1415bead0; const P6: u64 = 0x141834d30;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx); rax = P0; crate::simd_ops::wu64(rcx, rax); rsi = rcx; rbx = (crate::simd_ops::ru64(rcx.wrapping_add(0x18u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rbp); rbp = ((rdx as u32)) as u32 as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x20u64))) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rdi); r8 = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rdi = ((rbx.wrapping_add(r8.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 6; continue; } bb = 2; continue; }
            2 => { bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            5 => { rbx = ((rbx).wrapping_add(0x18u64)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 6; continue; }
            6 => { rdx = (crate::simd_ops::ru64(rsi.wrapping_add(0x18u64))) as u64; rbx = 0; fl_l = 0; fl_r = 0; rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rsi.wrapping_add(0x20u64), ((rbx as u32)) as u32); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; crate::simd_ops::wu64(rsi.wrapping_add(0x18u64), rbx); crate::simd_ops::wu32(rsi.wrapping_add(0x24u64), ((rbx as u32)) as u32); bb = 7; continue; }
            7 => { rbx = (crate::simd_ops::ru64(rsi.wrapping_add(0x58u64))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 19; continue; } bb = 8; continue; }
            8 => { rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x60u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 11; continue; } bb = 9; continue; }
            9 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0x64u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 11; continue; } bb = 10; continue; }
            10 => { rdx = (rdx & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 12; continue; }
            11 => { rdx = (rdx & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 12; continue; }
            12 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x60u64), ((rcx as u32)) as u32); if (((fl_l as u32)) == ((fl_r as u32))) { bb = 16; continue; } bb = 13; continue; }
            13 => { fl_l = ((rdx as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 17; continue; } bb = 14; continue; }
            14 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 17; continue; } bb = 15; continue; }
            15 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0x64u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 17; continue; } bb = 16; continue; }
            16 => { rcx = rbx; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 17; continue; }
            17 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x60u64))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 19; continue; } bb = 18; continue; }
            18 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rdx = rbx; rcx = 0x4; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 19; continue; }
            19 => { rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0xa8u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 21; continue; } bb = 20; continue; }
            20 => { rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 21; continue; }
            21 => { rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x90u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 23; continue; } bb = 22; continue; }
            22 => { rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 23; continue; }
            23 => { rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x60u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 25; continue; } bb = 24; continue; }
            24 => { rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 25; continue; }
            25 => { rcx = rsi.wrapping_add(0x28); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rsi; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rbp as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 27; continue; } bb = 26; continue; }
            26 => { rdx = 0xc0; rcx = rsi; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); bb = 27; continue; }
            27 => { rax = rsi; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415f5610(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x58; const P1: u64 = 0xb0; const P2: u64 = 0x50; const P3: u64 = 0x4; const P4: u64 = 0x1415f5d60; const P5: u64 = 0x28; const P6: u64 = 0x1426452b0; const P7: u64 = 0x143930f08; const P8: u64 = 0x88; const P9: u64 = 0xb4; const P10: u64 = 0xb8; const P11: u64 = 0x98; const P12: u64 = 0x1426452a8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = rcx; fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; fl_r = r8; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 16; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rsi); fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 13; continue; } bb = 3; continue; }
            3 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x1; rdx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 10; continue; } bb = 4; continue; }
            4 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 15; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 15; continue; } bb = 6; continue; }
            6 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(P1), ((rax as u32)) as u32); if (((fl_l as u32)) != ((fl_r as u32))) { bb = 15; continue; } bb = 7; continue; }
            7 => { fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(P2))) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 9; continue; } bb = 8; continue; }
            8 => { rdx = 0; fl_l = 0; fl_r = 0; r8 = ((rdx.wrapping_add(P3)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 9; continue; }
            9 => { rcx = ((rdi.wrapping_add(P5)) as u64) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(P7)) as u64; rcx = (crate::simd_ops::ru32(rax.wrapping_add(P8))) as u32 as u64; rcx = (((rcx as u32)).wrapping_sub(crate::simd_ops::ru32(rdi.wrapping_add(P9)))) as u32 as u64; crate::simd_ops::lk_add_u32(rdi.wrapping_add(P10), ((rcx as u32)) as u32); rsi = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rdi.wrapping_add(P9), ((rsi as u32)) as u32); bb = 14; continue; }
            10 => { fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(P2))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 15; continue; } bb = 11; continue; }
            11 => { crate::simd_ops::lk_add_u32(rcx.wrapping_add(P1), (1) as u32); fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 15; continue; } bb = 12; continue; }
            12 => { rax = (crate::simd_ops::ru64(P7)) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rcx = (crate::simd_ops::ru32(rax.wrapping_add(P8))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(P9), ((rcx as u32)) as u32); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            13 => { rsi = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rcx.wrapping_add(P2), ((rsi as u32)) as u32); crate::simd_ops::wu32(rcx.wrapping_add(P1), ((rsi as u32)) as u32); rcx = ((rcx).wrapping_add(0x28u64)) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu32(rdi.wrapping_add(P11), ((rsi as u32)) as u32); crate::simd_ops::wu64(rdi.wrapping_add(0xa0u64), rsi); bb = 14; continue; }
            14 => { rcx = rdi.wrapping_add(0x28); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P12), rax), rax, rcx, rdx, r8, r9, rsp); bb = 15; continue; }
            15 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; bb = 16; continue; }
            16 => { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415f5720(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x50; const P1: u64 = 0xbc; const P2: u64 = 0xfffffffffffffffd; const P3: u64 = 0x1415f6ad0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; fl_r = 0x0;
    crate::simd_ops::wu32(rcx.wrapping_add(P1), (0x0u32) as u32);
    if !((((fl_l as u32)) == ((fl_r as u32)))) { r9 = 0x4; r8 = 0; fl_l = 0; fl_r = 0; rdx = ((r9.wrapping_add(P2)) as u32) as u32 as u64; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
    rax = 0x1;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415f5750(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141524330; const P1: u64 = 0x1415bd140; const P2: u64 = 0x14268dfd8; const P3: u64 = 0x28; const P4: u64 = 0x18; const P5: u64 = 0x20; const P6: u64 = 0x1426452b8; const P7: u64 = 0x70; const P8: u64 = 0x74; const P9: u64 = 0x50; const P10: u64 = 0x58; const P11: u64 = 0x68;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = ((rdx as u32)) as u32 as u64; rbp = ((rcx as u32)) as u32 as u64; rdx = 0xc0; rcx = 0x2; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rbx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = rax; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rdi = 0; fl_l = 0; fl_r = 0; rax = P2; crate::simd_ops::wu64(rbx, rax); rcx = ((rbx.wrapping_add(P3)) as u64) as u64; crate::simd_ops::wu64(rbx.wrapping_add(P4), rdi); crate::simd_ops::wu64(rbx.wrapping_add(P5), rdi); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(P7), rdi); rax = rbx; crate::simd_ops::wu32(rbx.wrapping_add(P8), ((rbp as u32)) as u32); crate::simd_ops::wu32(rbx.wrapping_add(P9), ((rdi as u32)) as u32); crate::simd_ops::wu64(rbx.wrapping_add(P10), rdi); crate::simd_ops::wu64(rbx.wrapping_add(P11), rdi); crate::simd_ops::wu64(rbx.wrapping_add(0x60u64), rdi); crate::simd_ops::wu8(rbx.wrapping_add(0x78u64), ((rdi as u8)) as u8); crate::simd_ops::wu32(rbx.wrapping_add(0x7cu64), ((rsi as u32)) as u32); crate::simd_ops::wu64(rbx.wrapping_add(0x80u64), rdi); crate::simd_ops::wu64(rbx.wrapping_add(0x90u64), rdi); crate::simd_ops::wu64(rbx.wrapping_add(0x98u64), rdi); crate::simd_ops::wu64(rbx.wrapping_add(0xa8u64), rdi); crate::simd_ops::wu64(rbx.wrapping_add(0xb0u64), rdi); crate::simd_ops::wu64(rbx.wrapping_add(0xb8u64), rdi); crate::simd_ops::wu64(rbx.wrapping_add(0x88u64), rdi); crate::simd_ops::wu64(rbx.wrapping_add(0xa0u64), rdi); bb = 3; continue; }
            2 => { rdi = 0; fl_l = 0; fl_r = 0; rax = ((rdi as u32)) as u32 as u64; bb = 3; continue; }
            3 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415f5820(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141524330; const P1: u64 = 0x141648970;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp);
    crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x30u64)) as u64;
    rsi = rdx;
    rbp = rcx;
    rdx = 0x3f8;
    rcx = 0;
    fl_l = 0; fl_r = 0;
    rbx = r9;
    rdi = r8;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    fl_l = rax; fl_r = rax;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rcx = (crate::simd_ops::ru32(rbp.wrapping_add(0x7cu64))) as u32 as u64; r9 = rdi; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rcx as u32)) as u32); r8 = rsi; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rdx = rbx; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rcx); rcx = rax; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64;
    rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64;
    rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64;
    rsp = ((rsp).wrapping_add(0x30u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415f5890(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x1415864b0; const P2: u64 = 0x8; const P3: u64 = 0x1; const P4: u64 = 0xffffffffffffffe8; const P5: u64 = 0x18; const P6: u64 = 0x14183acc8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    rbx = rcx;
    rdi = ((rdx as u32)) as u32 as u64;
    r8 = ((rdi.wrapping_add(rdi.wrapping_mul(2))) as u64) as u64;
    rcx = (crate::simd_ops::ru64(rax.wrapping_add(r8.wrapping_mul(8)).wrapping_add(P0))) as u64;
    rsi = ((rax.wrapping_add(r8.wrapping_mul(8))) as u64) as u64;
    fl_l = rcx; fl_r = rcx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); }
    rdx = (crate::simd_ops::ru32(rbx.wrapping_add(P2))) as u32 as u64;
    rax = ((rdi.wrapping_add(P3)) as u32) as u32 as u64;
    fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64;
    if !((((fl_l as u32)) >= ((fl_r as u32)))) { rdx = (((rdx as u32)).wrapping_sub((rdi as u32))) as u32 as u64; rcx = rsi; rax = ((rdx.wrapping_add(rdx.wrapping_mul(2))) as u32) as u32 as u64; r8 = ((rax.wrapping_mul(8).wrapping_add(P4)) as u32) as u32 as u64; rdx = ((rsi.wrapping_add(P5)) as u64) as u64; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); }
    crate::simd_ops::lk_sub_u32(rbx.wrapping_add(P2), (1) as u32);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415f5900(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x90; const P1: u64 = 0x1415864b0; const P2: u64 = 0x98; const P3: u64 = 0x80; const P4: u64 = 0x9c; const P5: u64 = 0x84; const P6: u64 = 0xa0; const P7: u64 = 0x88; const P8: u64 = 0xa8; const P9: u64 = 0x1415f65b0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rcx; rdi = r9; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rsi = r8; rbp = rdx; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(P2))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(P3), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_add(P4))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(P5), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rbx.wrapping_add(P6))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(P7), rax); rax = (crate::simd_ops::ru64(rbx.wrapping_add(P8))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(P0), rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::lk_add_u32(rax, (1) as u32)) as u32 as u64; bb = 4; continue; }
            4 => { r8 = rsi; rdx = rbp; rcx = rbx; rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rdi)) as u64; rsi = ((rax as u32)) as u32 as u64; rbx = (crate::simd_ops::ru64(rbx.wrapping_add(P8))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            6 => { rax = ((rsi as u32)) as u32 as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { fl_l = (crate::simd_ops::lk_add_u32(rbx, (1) as u32)) as u32 as u64; bb = 8; continue; }
            8 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; crate::simd_ops::wu64(rdi, rbx); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415f59c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1426452b0; const P1: u64 = 0xb4; const P2: u64 = 0x28; const P3: u64 = 0xb8; const P4: u64 = 0x1426452a8;
    let (mut rax, mut rbx, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi);
    crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rdi);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, r14);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rsi = rcx;
    r14 = r8;
    rcx = ((rcx).wrapping_add(0x28u64)) as u64;
    rdi = rdx;
    rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp);
    rax = (crate::simd_ops::ru32(rsi.wrapping_add(P1))) as u32 as u64;
    rcx = ((rsi.wrapping_add(P2)) as u64) as u64;
    crate::simd_ops::wu32(rdi, ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rsi.wrapping_add(P3))) as u32 as u64;
    crate::simd_ops::wu32(r14, ((rax as u32)) as u32);
    crate::simd_ops::wu32(rsi.wrapping_add(P3), (0x0u32) as u32);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    r14 = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415f5a30(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1426452b0; const P1: u64 = 0x98; const P2: u64 = 0x28; const P3: u64 = 0xa0; const P4: u64 = 0x1426452a8;
    let (mut rax, mut rbx, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi);
    crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rdi);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, r14);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rsi = rcx;
    r14 = r8;
    rcx = ((rcx).wrapping_add(0x28u64)) as u64;
    rdi = rdx;
    rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp);
    rax = (crate::simd_ops::ru32(rsi.wrapping_add(P1))) as u32 as u64;
    rcx = ((rsi.wrapping_add(P2)) as u64) as u64;
    crate::simd_ops::wu32(rdi, ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru64(rsi.wrapping_add(P3))) as u64;
    crate::simd_ops::wu64(r14, rax);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    r14 = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415f5a90(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143930f08; const P1: u64 = 0x1415a6650; const P2: u64 = 0x143930f88; const P3: u64 = 0x1426452b0; const P4: u64 = 0x1426452a8; const P5: u64 = 0x142645560; const P6: u64 = 0x1415a0aa0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(0x50u64))) as u32 as u64; fl_r = 0x0; r12 = ((r8 as u32)) as u32 as u64; r13 = ((rdx as u32)) as u32 as u64; r14 = rcx; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 64; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::lk_add_u32(rcx.wrapping_add(0xb0u64), (1) as u32); fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(0xb0u64))) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 64; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(P0)) as u64; rdx = (crate::simd_ops::ru32(rcx.wrapping_add(0x74u64))) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), rbp); r9 = (crate::simd_ops::ru32(rax.wrapping_add(0x88u64))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(0xb4u64), ((r9 as u32)) as u32); rcx = (crate::simd_ops::ru64(P0)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), r15); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rbp = (crate::simd_ops::ru32(r14.wrapping_add(0x80u64))) as u32 as u64; r15 = 0xffffffff; fl_l = ((rbp as u32)) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 33; continue; } bb = 3; continue; }
            3 => { rbx = (crate::simd_ops::ru64(P2)) as u64; bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 15; continue; } bb = 5; continue; }
            5 => { rcx = ((rax.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; let cx_src: u32 = (rcx as u32); let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u32(rbx, fl_l as u32, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u32)) as u32 as u64; } if (((fl_l as u32)) != ((fl_r as u32))) { bb = 4; continue; } bb = 6; continue; }
            6 => { rsi = rbx.wrapping_add(0x8); bb = 7; continue; }
            7 => { rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x48u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 18; continue; } bb = 8; continue; }
            8 => { rdx = 0; fl_l = 0; fl_r = 0; rax = ((rbp as u32)) as u32 as u64; let d = ((rcx as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x40u64))) as u64; rdi = (crate::simd_ops::ru64(rax.wrapping_add(rdx.wrapping_mul(8)))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 9; continue; }
            9 => { bb = 10; continue; }
            10 => { fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x10u64))) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 17; continue; } bb = 11; continue; }
            11 => { rdi = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 10; continue; } bb = 12; continue; }
            12 => { r8 = 0; fl_l = 0; fl_r = 0; rdi = ((r8 as u32)) as u32 as u64; bb = 13; continue; }
            13 => { fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 19; continue; } bb = 14; continue; }
            14 => { fl_l = (crate::simd_ops::lk_add_u32(rdi.wrapping_add(0x14u64), (1) as u32)) as u32 as u64; bb = 19; continue; }
            15 => { rsi = rbx.wrapping_add(0x8); rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 7; continue; } bb = 16; continue; }
            16 => { rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            17 => { r8 = 0; fl_l = 0; fl_r = 0; bb = 13; continue; }
            18 => { r8 = 0; fl_l = 0; fl_r = 0; rdi = ((r8 as u32)) as u32 as u64; bb = 19; continue; }
            19 => { rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 27; continue; } bb = 20; continue; }
            20 => { bb = 21; continue; }
            21 => { rcx = ((rax.wrapping_sub(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; let cx_src: u32 = (rcx as u32); let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u32(rbx, fl_l as u32, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u32)) as u32 as u64; } if (((fl_l as u32)) == ((fl_r as u32))) { bb = 24; continue; } bb = 22; continue; }
            22 => { rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) >= 0) { bb = 21; continue; } bb = 23; continue; }
            23 => { bb = 28; continue; }
            24 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 30; continue; } bb = 25; continue; }
            25 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x30u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 30; continue; } bb = 26; continue; }
            26 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x38u64))) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); bb = 29; continue; }
            27 => { rsi = rbx.wrapping_add(0x8); bb = 28; continue; }
            28 => { rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); bb = 29; continue; }
            29 => { r8 = 0; fl_l = 0; fl_r = 0; bb = 30; continue; }
            30 => { fl_l = rdi; fl_r = rdi; rbx = rdi.wrapping_sub(0x10); rbx = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { r8 } else { rbx }) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 33; continue; } bb = 31; continue; }
            31 => { r8 = (crate::simd_ops::ru32(r14.wrapping_add(0x74u64))) as u32 as u64; r9 = ((r13 as u32)) as u32 as u64; rdx = (crate::simd_ops::ru64(r14.wrapping_add(0x58u64))) as u64; rcx = rbx; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((r12 as u32)) as u32); rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rcx = rbx.wrapping_add(0x10); rax = ((r15 as u32)) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rcx.wrapping_add(0x14u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 33; continue; } bb = 32; continue; }
            32 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 33; continue; }
            33 => { rbp = (crate::simd_ops::ru32(r14.wrapping_add(0x98u64))) as u32 as u64; fl_l = ((rbp as u32)) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 63; continue; } bb = 34; continue; }
            34 => { rbx = (crate::simd_ops::ru64(P2)) as u64; bb = 35; continue; }
            35 => { rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 46; continue; } bb = 36; continue; }
            36 => { rcx = ((rax.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; let cx_src: u32 = (rcx as u32); let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u32(rbx, fl_l as u32, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u32)) as u32 as u64; } if (((fl_l as u32)) != ((fl_r as u32))) { bb = 35; continue; } bb = 37; continue; }
            37 => { rsi = rbx.wrapping_add(0x8); bb = 38; continue; }
            38 => { rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x48u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 49; continue; } bb = 39; continue; }
            39 => { rdx = 0; fl_l = 0; fl_r = 0; rax = ((rbp as u32)) as u32 as u64; let d = ((rcx as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x40u64))) as u64; rdi = (crate::simd_ops::ru64(rax.wrapping_add(rdx.wrapping_mul(8)))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 43; continue; } bb = 40; continue; }
            40 => { bb = 41; continue; }
            41 => { fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x10u64))) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 48; continue; } bb = 42; continue; }
            42 => { rdi = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 41; continue; } bb = 43; continue; }
            43 => { rbp = 0; fl_l = 0; fl_r = 0; rdi = ((rbp as u32)) as u32 as u64; bb = 44; continue; }
            44 => { fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 50; continue; } bb = 45; continue; }
            45 => { fl_l = (crate::simd_ops::lk_add_u32(rdi.wrapping_add(0x14u64), (1) as u32)) as u32 as u64; bb = 50; continue; }
            46 => { rsi = rbx.wrapping_add(0x8); rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 38; continue; } bb = 47; continue; }
            47 => { rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); bb = 35; continue; }
            48 => { rbp = 0; fl_l = 0; fl_r = 0; bb = 44; continue; }
            49 => { rbp = 0; fl_l = 0; fl_r = 0; rdi = ((rbp as u32)) as u32 as u64; bb = 50; continue; }
            50 => { rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 58; continue; } bb = 51; continue; }
            51 => { bb = 52; continue; }
            52 => { rcx = ((rax.wrapping_sub(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; let cx_src: u32 = (rcx as u32); let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u32(rbx, fl_l as u32, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u32)) as u32 as u64; } if (((fl_l as u32)) == ((fl_r as u32))) { bb = 55; continue; } bb = 53; continue; }
            53 => { rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) >= 0) { bb = 52; continue; } bb = 54; continue; }
            54 => { bb = 59; continue; }
            55 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 60; continue; } bb = 56; continue; }
            56 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x30u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 60; continue; } bb = 57; continue; }
            57 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x38u64))) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); bb = 60; continue; }
            58 => { rsi = rbx.wrapping_add(0x8); bb = 59; continue; }
            59 => { rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); bb = 60; continue; }
            60 => { fl_l = rdi; fl_r = rdi; rbx = rdi.wrapping_sub(0x10); rbx = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rbp } else { rbx }) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 63; continue; } bb = 61; continue; }
            61 => { r8 = (crate::simd_ops::ru32(r14.wrapping_add(0x74u64))) as u32 as u64; r9 = ((r13 as u32)) as u32 as u64; rdx = (crate::simd_ops::ru64(r14.wrapping_add(0x58u64))) as u64; rcx = rbx; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((r12 as u32)) as u32); rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rcx = rbx.wrapping_add(0x10); let xa_src: u32 = (r15 as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rcx.wrapping_add(0x14u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; r15 = (xa_old) as u32 as u64; fl_l = ((r15 as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 63; continue; } bb = 62; continue; }
            62 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 63; continue; }
            63 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rax = 0x1; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            64 => { rax = 0x1; rsp = ((rsp).wrapping_add(0x40u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415f5d60(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1426452b0; const P1: u64 = 0x1415864b0; const P2: u64 = 0x14183acc8; const P3: u64 = 0x143930f90; const P4: u64 = 0x14159aa90; const P5: u64 = 0x1415f5480; const P6: u64 = 0x1415f5890; const P7: u64 = 0x1426452a8; const P8: u64 = 0x1415f66e0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu32(rsp.wrapping_add(0x10u64), ((rdx as u32)) as u32); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(0x50u64))) as u32 as u64; fl_r = 0xfffffffd; r13 = ((r8 as u32)) as u32 as u64; r14 = ((rdx as u32)) as u32 as u64; rdi = rcx; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 27; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(0xb0u64))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 27; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), rsi); rsi = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), r12); r12 = ((rsi as u32)) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x50u64), (0x1u32) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), r15); bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru64(rdi.wrapping_add(0x58u64))) as u64; fl_r = (0x0u64) as u64; rbp = rsi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 19; continue; } bb = 4; continue; }
            4 => { rcx = rdi.wrapping_add(0x28); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x50u64))) as u32 as u64; fl_r = 0x0; crate::simd_ops::wu32(rdi.wrapping_add(0x98u64), ((rsi as u32)) as u32); crate::simd_ops::wu64(rdi.wrapping_add(0xa0u64), rsi); if (((fl_l as u32)) == ((fl_r as u32))) { bb = 18; continue; } bb = 5; continue; }
            5 => { fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x20u64))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 16; continue; } bb = 6; continue; }
            6 => { bb = 7; continue; }
            7 => { rsi = (crate::simd_ops::ru64(rdi.wrapping_add(0x18u64))) as u64; fl_l = (crate::simd_ops::ru32(rsi)) as u32 as u64; fl_r = 0x0; rbp = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 14; continue; } bb = 8; continue; }
            8 => { rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x10u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 10; continue; }
            10 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x20u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 12; continue; } bb = 11; continue; }
            11 => { rax = ((rax.wrapping_add(rax.wrapping_mul(2))) as u32) as u32 as u64; rcx = rsi; r8 = ((rax.wrapping_mul(8).wrapping_sub(0x18u64)) as u32) as u32 as u64; rdx = rsi.wrapping_add(0x18); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 12; continue; }
            12 => { crate::simd_ops::lk_sub_u32(rdi.wrapping_add(0x20u64), (1) as u32); r9 = rbp; rdx = (crate::simd_ops::ru32(rdi.wrapping_add(0x74u64))) as u32 as u64; r8 = 0; fl_l = 0; fl_r = 0; rcx = (crate::simd_ops::ru64(P3)) as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x20u64))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 7; continue; } bb = 13; continue; }
            13 => { bb = 15; continue; }
            14 => { rdx = rsi; rcx = rdi.wrapping_add(0x98); rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); r12 = (crate::simd_ops::ru32(rsi.wrapping_add(0x4u64))) as u32 as u64; rcx = rdi.wrapping_add(0x18); rdx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); bb = 15; continue; }
            15 => { r14 = (crate::simd_ops::ru32(rsp.wrapping_add(0x68u64))) as u32 as u64; bb = 16; continue; }
            16 => { rsi = (crate::simd_ops::ru32(rdi.wrapping_add(0x98u64))) as u32 as u64; fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 18; continue; } bb = 17; continue; }
            17 => { crate::simd_ops::wu32(rdi.wrapping_add(0x50u64), (0x2u32) as u32); bb = 18; continue; }
            18 => { rcx = rdi.wrapping_add(0x28); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P7), rax), rax, rcx, rdx, r8, r9, rsp); bb = 19; continue; }
            19 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x60u64))) as u64; rbx = (crate::simd_ops::ru64(rdi.wrapping_add(0xa8u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 21; continue; } bb = 20; continue; }
            20 => { rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 21; continue; }
            21 => { fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 23; continue; } bb = 22; continue; }
            22 => { fl_l = (crate::simd_ops::lk_add_u32(rbx, (1) as u32)) as u32 as u64; bb = 23; continue; }
            23 => { crate::simd_ops::wu64(rdi.wrapping_add(0x60u64), rbx); fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 26; continue; } bb = 24; continue; }
            24 => { r9 = ((r14 as u32)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((r13 as u32)) as u32); r8 = ((r12 as u32)) as u32 as u64; rdx = ((rsi as u32)) as u32 as u64; rcx = rdi; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 26; continue; } bb = 25; continue; }
            25 => { rdx = (crate::simd_ops::ru32(rdi.wrapping_add(0x74u64))) as u32 as u64; r9 = rbp; rcx = (crate::simd_ops::ru64(P3)) as u64; r8 = ((rsi as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rsi = 0; fl_l = 0; fl_r = 0; bb = 3; continue; }
            26 => { r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = 0x1; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            27 => { rax = 0x1; rsp = ((rsp).wrapping_add(0x40u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415f5f30(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415bd1d0;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rdx = 0x8;
    return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415f5f40(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415f5d60; const P1: u64 = 0x143930f08; const P2: u64 = 0x1415aaed0; const P3: u64 = 0x143930f88; const P4: u64 = 0x1426452b0; const P5: u64 = 0x1426452a8; const P6: u64 = 0x142645560; const P7: u64 = 0x1415a1ff0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0xb0u64))) as u32 as u64; r12 = ((r8 as u32)) as u32 as u64; r13 = ((rdx as u32)) as u32 as u64; rbp = rcx; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 59; continue; } bb = 1; continue; }
            1 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(0xb0u64), ((rax as u32)) as u32); if (((fl_l as u32)) != ((fl_r as u32))) { bb = 59; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(0x50u64))) as u32 as u64; fl_r = 0x1; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), r14); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), r15); if (((fl_l as u32)) == ((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { rdx = (crate::simd_ops::ru32(rbp.wrapping_add(0x74u64))) as u32 as u64; rcx = (crate::simd_ops::ru64(P1)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), rsi); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); r14 = (crate::simd_ops::ru32(rbp.wrapping_add(0x80u64))) as u32 as u64; rdi = 0; fl_l = 0; fl_r = 0; r15 = ((rdi.wrapping_sub(0x1u64)) as u32) as u32 as u64; fl_l = ((r14 as u32)) as u32 as u64; fl_r = ((r14 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 31; continue; } bb = 5; continue; }
            5 => { rbx = (crate::simd_ops::ru64(P3)) as u64; bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 21; continue; } bb = 7; continue; }
            7 => { rcx = ((rax.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; let cx_src: u32 = (rcx as u32); let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u32(rbx, fl_l as u32, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u32)) as u32 as u64; } if (((fl_l as u32)) != ((fl_r as u32))) { bb = 6; continue; } bb = 8; continue; }
            8 => { rsi = rbx.wrapping_add(0x8); bb = 9; continue; }
            9 => { rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x48u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 17; continue; } bb = 10; continue; }
            10 => { rdx = 0; fl_l = 0; fl_r = 0; rax = ((r14 as u32)) as u32 as u64; let d = ((rcx as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x40u64))) as u64; rdi = (crate::simd_ops::ru64(rax.wrapping_add(rdx.wrapping_mul(8)))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 11; continue; }
            11 => { bb = 12; continue; }
            12 => { fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x10u64))) as u32 as u64; fl_r = ((r14 as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 15; continue; } bb = 13; continue; }
            13 => { rdi = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 12; continue; } bb = 14; continue; }
            14 => { rdi = 0; fl_l = 0; fl_r = 0; bb = 15; continue; }
            15 => { fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 16; continue; }
            16 => { fl_l = (crate::simd_ops::lk_add_u32(rdi.wrapping_add(0x14u64), (1) as u32)) as u32 as u64; bb = 17; continue; }
            17 => { rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 26; continue; } bb = 18; continue; }
            18 => { rcx = ((rax.wrapping_sub(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; let cx_src: u32 = (rcx as u32); let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u32(rbx, fl_l as u32, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u32)) as u32 as u64; } if (((fl_l as u32)) == ((fl_r as u32))) { bb = 23; continue; } bb = 19; continue; }
            19 => { rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) >= 0) { bb = 18; continue; } bb = 20; continue; }
            20 => { bb = 27; continue; }
            21 => { rsi = rbx.wrapping_add(0x8); rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 9; continue; } bb = 22; continue; }
            22 => { rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            23 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 28; continue; } bb = 24; continue; }
            24 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x30u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 28; continue; } bb = 25; continue; }
            25 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x38u64))) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp); bb = 28; continue; }
            26 => { rsi = rbx.wrapping_add(0x8); bb = 27; continue; }
            27 => { rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); bb = 28; continue; }
            28 => { fl_l = rdi; fl_r = rdi; rbx = rdi.wrapping_sub(0x10); rdi = 0x0; rbx = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rdi } else { rbx }) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 31; continue; } bb = 29; continue; }
            29 => { r8 = (crate::simd_ops::ru32(rbp.wrapping_add(0x74u64))) as u32 as u64; r9 = ((r13 as u32)) as u32 as u64; rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x58u64))) as u64; rcx = rbx; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((r12 as u32)) as u32); rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rcx = rbx.wrapping_add(0x10); rax = ((r15 as u32)) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rcx.wrapping_add(0x14u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 31; continue; } bb = 30; continue; }
            30 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 31; continue; }
            31 => { r14 = (crate::simd_ops::ru32(rbp.wrapping_add(0x98u64))) as u32 as u64; fl_l = ((r14 as u32)) as u32 as u64; fl_r = ((r14 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 58; continue; } bb = 32; continue; }
            32 => { rbx = (crate::simd_ops::ru64(P3)) as u64; bb = 33; continue; }
            33 => { rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 48; continue; } bb = 34; continue; }
            34 => { rcx = ((rax.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; let cx_src: u32 = (rcx as u32); let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u32(rbx, fl_l as u32, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u32)) as u32 as u64; } if (((fl_l as u32)) != ((fl_r as u32))) { bb = 33; continue; } bb = 35; continue; }
            35 => { rsi = rbx.wrapping_add(0x8); bb = 36; continue; }
            36 => { rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x48u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 44; continue; } bb = 37; continue; }
            37 => { rdx = 0; fl_l = 0; fl_r = 0; rax = ((r14 as u32)) as u32 as u64; let d = ((rcx as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x40u64))) as u64; rdi = (crate::simd_ops::ru64(rax.wrapping_add(rdx.wrapping_mul(8)))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 41; continue; } bb = 38; continue; }
            38 => { bb = 39; continue; }
            39 => { fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x10u64))) as u32 as u64; fl_r = ((r14 as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 42; continue; } bb = 40; continue; }
            40 => { rdi = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 39; continue; } bb = 41; continue; }
            41 => { rdi = 0; fl_l = 0; fl_r = 0; bb = 42; continue; }
            42 => { fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 44; continue; } bb = 43; continue; }
            43 => { fl_l = (crate::simd_ops::lk_add_u32(rdi.wrapping_add(0x14u64), (1) as u32)) as u32 as u64; bb = 44; continue; }
            44 => { rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 53; continue; } bb = 45; continue; }
            45 => { rcx = ((rax.wrapping_sub(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; let cx_src: u32 = (rcx as u32); let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u32(rbx, fl_l as u32, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u32)) as u32 as u64; } if (((fl_l as u32)) == ((fl_r as u32))) { bb = 50; continue; } bb = 46; continue; }
            46 => { rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) >= 0) { bb = 45; continue; } bb = 47; continue; }
            47 => { bb = 54; continue; }
            48 => { rsi = rbx.wrapping_add(0x8); rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 36; continue; } bb = 49; continue; }
            49 => { rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); bb = 33; continue; }
            50 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 55; continue; } bb = 51; continue; }
            51 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x30u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 55; continue; } bb = 52; continue; }
            52 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x38u64))) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp); bb = 55; continue; }
            53 => { rsi = rbx.wrapping_add(0x8); bb = 54; continue; }
            54 => { rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); bb = 55; continue; }
            55 => { fl_l = rdi; fl_r = rdi; rbx = rdi.wrapping_sub(0x10); rdi = 0x0; rbx = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rdi } else { rbx }) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 58; continue; } bb = 56; continue; }
            56 => { r8 = (crate::simd_ops::ru32(rbp.wrapping_add(0x74u64))) as u32 as u64; r9 = ((r13 as u32)) as u32 as u64; rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x58u64))) as u64; rcx = rbx; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((r12 as u32)) as u32); rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rcx = rbx.wrapping_add(0x10); let xa_src: u32 = (r15 as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rcx.wrapping_add(0x14u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; r15 = (xa_old) as u32 as u64; fl_l = ((r15 as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 58; continue; } bb = 57; continue; }
            57 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 58; continue; }
            58 => { rcx = rbp.wrapping_add(0x28); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(P1)) as u64; rcx = (crate::simd_ops::ru32(rax.wrapping_add(0x88u64))) as u32 as u64; rcx = (((rcx as u32)).wrapping_sub(crate::simd_ops::ru32(rbp.wrapping_add(0xb4u64)))) as u32 as u64; crate::simd_ops::lk_add_u32(rbp.wrapping_add(0xb8u64), ((rcx as u32)) as u32); rcx = rbp.wrapping_add(0x28); crate::simd_ops::wu32(rbp.wrapping_add(0xb4u64), ((rdi as u32)) as u32); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; bb = 59; continue; }
            59 => { rax = 0x1; rsp = ((rsp).wrapping_add(0x40u64)) as u64; r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415f6210(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x50; const P1: u64 = 0x20; const P2: u64 = 0x4; const P3: u64 = 0x1415f5d60;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; fl_r = 0x2;
    if (((fl_l as u32)) != ((fl_r as u32))) { return rax; }
    fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64; fl_r = 0x0;
    if !((((fl_l as u32)) == ((fl_r as u32)))) { rdx = 0; fl_l = 0; fl_r = 0; r8 = ((rdx.wrapping_add(P2)) as u32) as u32 as u64; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415f6230(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143930f88; const P1: u64 = 0x1426452b0; const P2: u64 = 0x1426452a8; const P3: u64 = 0x142645560;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x68u64)) as u64; fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(0x50u64))) as u32 as u64; fl_r = 0x0; r8 = rcx; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = 0x1; rsp = ((rsp).wrapping_add(0x68u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            2 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x58u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = 0x2; rsp = ((rsp).wrapping_add(0x68u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            4 => { crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x74u64))) as u32 as u64; rcx = ((crate::simd_ops::ru8(rdx.wrapping_add(0xdu64))) as u32) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((rax as u32)) as u32); rcx = (rcx & !0xffu64) | ((((((rcx as u8)) & (0x1u8))) as u8 as u64)); rax = ((crate::simd_ops::ru8(rdx.wrapping_add(0xcu64))) as u32) as u32 as u64; rcx = (rcx & !0xffu64) | (((((rcx as u8)).wrapping_add((rcx as u8))) as u8 as u64)); rax = (rax & !0xffu64) | ((((((rax as u8)) & (0x1u8))) as u8 as u64)); crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), rbp); rcx = (rcx & !0xffu64) | ((((((rcx as u8)) | ((rax as u8)))) as u8 as u64)); crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), r14); rax = ((crate::simd_ops::ru8(rsp.wrapping_add(0x5cu64))) as u32) as u32 as u64; r14 = 0; fl_l = 0; fl_r = 0; rax = (rax & !0xffu64) | ((((((rax as u8)) & (0xfcu8))) as u8 as u64)); crate::simd_ops::wu16(rsp.wrapping_add(0x3fu64), (0x0u16) as u16); rcx = (rcx & !0xffu64) | ((((((rcx as u8)) | ((rax as u8)))) as u8 as u64)); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), r14); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), r14); crate::simd_ops::wu16(rsp.wrapping_add(0x3cu64), ((r14 as u16)) as u16); crate::simd_ops::wu8(rsp.wrapping_add(0x3eu64), ((r14 as u8)) as u8); crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), (0x4u32) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x34u64), ((r14 as u32)) as u32); crate::simd_ops::wu8(rsp.wrapping_add(0x5cu64), ((rcx as u8)) as u8); fl_l = ((rcx as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { xmm0 = ((crate::simd_ops::ru32(rdx.wrapping_add(0x8u64))) as u32) as u128; crate::simd_ops::wu32(rsp.wrapping_add(0x58u64), ((xmm0 as u32)) as u32); bb = 7; continue; }
            6 => { rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x8u64))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x58u64), ((rax as u32)) as u32); bb = 7; continue; }
            7 => { rbp = (crate::simd_ops::ru32(r8.wrapping_add(0x80u64))) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x80u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x88u64), rdi); crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x4u32) as u32); fl_l = ((rbp as u32)) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 32; continue; } bb = 8; continue; }
            8 => { rbx = (crate::simd_ops::ru64(P0)) as u64; bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 19; continue; } bb = 10; continue; }
            10 => { rcx = ((rax.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; let cx_src: u32 = (rcx as u32); let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u32(rbx, fl_l as u32, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u32)) as u32 as u64; } if (((fl_l as u32)) != ((fl_r as u32))) { bb = 9; continue; } bb = 11; continue; }
            11 => { rsi = rbx.wrapping_add(0x8); bb = 12; continue; }
            12 => { rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x48u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 21; continue; } bb = 13; continue; }
            13 => { rdx = 0; fl_l = 0; fl_r = 0; rax = ((rbp as u32)) as u32 as u64; let d = ((rcx as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x40u64))) as u64; rdi = (crate::simd_ops::ru64(rax.wrapping_add(rdx.wrapping_mul(8)))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 16; continue; } bb = 14; continue; }
            14 => { fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x10u64))) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 17; continue; } bb = 15; continue; }
            15 => { rdi = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 14; continue; } bb = 16; continue; }
            16 => { rdi = r14; bb = 17; continue; }
            17 => { fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 22; continue; } bb = 18; continue; }
            18 => { fl_l = (crate::simd_ops::lk_add_u32(rdi.wrapping_add(0x14u64), (1) as u32)) as u32 as u64; bb = 22; continue; }
            19 => { rsi = rbx.wrapping_add(0x8); rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 12; continue; } bb = 20; continue; }
            20 => { rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 9; continue; }
            21 => { rdi = r14; bb = 22; continue; }
            22 => { rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 29; continue; } bb = 23; continue; }
            23 => { rcx = ((rax.wrapping_sub(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; let cx_src: u32 = (rcx as u32); let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u32(rbx, fl_l as u32, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u32)) as u32 as u64; } if (((fl_l as u32)) == ((fl_r as u32))) { bb = 26; continue; } bb = 24; continue; }
            24 => { rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) >= 0) { bb = 23; continue; } bb = 25; continue; }
            25 => { bb = 30; continue; }
            26 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 31; continue; } bb = 27; continue; }
            27 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x30u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 31; continue; } bb = 28; continue; }
            28 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x38u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rdi; fl_r = rdi; rbx = rdi.wrapping_sub(0x10); rbx = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { r14 } else { rbx }) as u64; bb = 59; continue; }
            29 => { rsi = rbx.wrapping_add(0x8); bb = 30; continue; }
            30 => { rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 31; continue; }
            31 => { fl_l = rdi; fl_r = rdi; rbx = rdi.wrapping_sub(0x10); rbx = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { r14 } else { rbx }) as u64; bb = 59; continue; }
            32 => { rbp = (crate::simd_ops::ru32(r8.wrapping_add(0x98u64))) as u32 as u64; fl_l = ((rbp as u32)) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 63; continue; } bb = 33; continue; }
            33 => { rbx = (crate::simd_ops::ru64(P0)) as u64; bb = 34; continue; }
            34 => { rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 45; continue; } bb = 35; continue; }
            35 => { rcx = ((rax.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; let cx_src: u32 = (rcx as u32); let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u32(rbx, fl_l as u32, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u32)) as u32 as u64; } if (((fl_l as u32)) != ((fl_r as u32))) { bb = 34; continue; } bb = 36; continue; }
            36 => { rsi = rbx.wrapping_add(0x8); bb = 37; continue; }
            37 => { rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x48u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 47; continue; } bb = 38; continue; }
            38 => { rdx = 0; fl_l = 0; fl_r = 0; rax = ((rbp as u32)) as u32 as u64; let d = ((rcx as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x40u64))) as u64; rdi = (crate::simd_ops::ru64(rax.wrapping_add(rdx.wrapping_mul(8)))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 42; continue; } bb = 39; continue; }
            39 => { bb = 40; continue; }
            40 => { fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x10u64))) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 43; continue; } bb = 41; continue; }
            41 => { rdi = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 40; continue; } bb = 42; continue; }
            42 => { rdi = r14; bb = 43; continue; }
            43 => { fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 48; continue; } bb = 44; continue; }
            44 => { fl_l = (crate::simd_ops::lk_add_u32(rdi.wrapping_add(0x14u64), (1) as u32)) as u32 as u64; bb = 48; continue; }
            45 => { rsi = rbx.wrapping_add(0x8); rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 37; continue; } bb = 46; continue; }
            46 => { rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 34; continue; }
            47 => { rdi = r14; bb = 48; continue; }
            48 => { rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 55; continue; } bb = 49; continue; }
            49 => { rcx = ((rax.wrapping_sub(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; let cx_src: u32 = (rcx as u32); let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u32(rbx, fl_l as u32, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u32)) as u32 as u64; } if (((fl_l as u32)) == ((fl_r as u32))) { bb = 52; continue; } bb = 50; continue; }
            50 => { rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) >= 0) { bb = 49; continue; } bb = 51; continue; }
            51 => { bb = 56; continue; }
            52 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 57; continue; } bb = 53; continue; }
            53 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x30u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 57; continue; } bb = 54; continue; }
            54 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x38u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 57; continue; }
            55 => { rsi = rbx.wrapping_add(0x8); bb = 56; continue; }
            56 => { rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 57; continue; }
            57 => { fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 63; continue; } bb = 58; continue; }
            58 => { rbx = rdi.wrapping_sub(0x10); bb = 59; continue; }
            59 => { fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 63; continue; } bb = 60; continue; }
            60 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = rsp.wrapping_add(0x20); rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x80u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rbx.wrapping_add(0x10); rax = 0xffffffff; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rcx.wrapping_add(0x14u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 62; continue; } bb = 61; continue; }
            61 => { rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 62; continue; }
            62 => { rax = 0x1; bb = 64; continue; }
            63 => { rax = 0xf; bb = 64; continue; }
            64 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x88u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsp = ((rsp).wrapping_add(0x68u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415f64f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x58; const P1: u64 = 0x60; const P2: u64 = 0x64; const P3: u64 = 0x1415af330;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rcx.wrapping_add(P0), rdx); fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 6; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(P2))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 6; continue; } bb = 3; continue; }
            3 => { rax = (((rax as u32)).wrapping_add(1)) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_add(P1), ((rax as u32)) as u32); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 5; continue; } bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(P2))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 7; continue; } bb = 5; continue; }
            5 => { rcx = rdx; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            6 => { fl_l = (((rax as u32)).wrapping_add(1)) as u32 as u64; rax = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_add(P1), ((rax as u32)) as u32); bb = 7; continue; }
            7 => { return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415f6530(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xbc; const P1: u64 = 0x1415f6ad0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    r9 = ((r8 as u32)) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(P0), (0x0u32) as u32);
    r8 = ((rdx as u32)) as u32 as u64;
    rdx = 0;
    fl_l = 0; fl_r = 0;
    return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415f6550(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x50; const P1: u64 = 0x1426452a8; const P2: u64 = 0x1415ab070; const P3: u64 = 0x143930f08; const P4: u64 = 0x17; const P5: u64 = 0x1415aa7b0; const P6: u64 = 0x14; const P7: u64 = 0x4; const P8: u64 = 0xc; const P9: u64 = 0xb4;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64;
    rdi = rcx;
    rcx = ((rcx).wrapping_add(0x28u64)) as u64;
    rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp);
    fl_l = ((rbx as u32)) as u32 as u64; fl_r = 0x2;
    if !((((fl_l as u32)) != ((fl_r as u32)))) { rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(P3)) as u64; rdx = ((rbx.wrapping_add(P4)) as u32) as u32 as u64; r8 = (((rax as u16)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::lk_add_u32(rdi.wrapping_add(P6), (1) as u32); crate::simd_ops::wu64(rax.wrapping_add(P7), rdi); crate::simd_ops::wu32(rax.wrapping_add(P8), (0x6u32) as u32); rax = (crate::simd_ops::ru64(P3)) as u64; fl_l = (crate::simd_ops::lk_sub_u32(rax.wrapping_add(P9), (1) as u32)) as u32 as u64; }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415f65b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1426452b0; const P1: u64 = 0x1415864b0; const P2: u64 = 0x14183acc8; const P3: u64 = 0x143930f90; const P4: u64 = 0x14159aa90; const P5: u64 = 0x1415f5480; const P6: u64 = 0x1415f5890; const P7: u64 = 0x1426452a8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(0x58u64))) as u64; fl_r = (0x0u64) as u64; r14 = r8; r12 = rdx; rbp = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = 0; fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            2 => { crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rbx); rcx = ((rcx).wrapping_add(0x28u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), r15); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); rdi = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rbp.wrapping_add(0x98u64), ((rdi as u32)) as u32); crate::simd_ops::wu64(rbp.wrapping_add(0xa0u64), rdi); fl_l = (crate::simd_ops::ru32(rbp.wrapping_add(0x50u64))) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 16; continue; } bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru32(rbp.wrapping_add(0x20u64))) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 14; continue; } bb = 4; continue; }
            4 => { crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rsi); bb = 5; continue; }
            5 => { rsi = (crate::simd_ops::ru64(rbp.wrapping_add(0x18u64))) as u64; rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(r14, rax); fl_l = (crate::simd_ops::ru32(rsi)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 12; continue; } bb = 6; continue; }
            6 => { rsi = (crate::simd_ops::ru64(rbp.wrapping_add(0x18u64))) as u64; rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x10u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 8; continue; }
            8 => { rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x20u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 10; continue; } bb = 9; continue; }
            9 => { rax = ((rax.wrapping_add(rax.wrapping_mul(2))) as u32) as u32 as u64; rcx = rsi; r8 = ((rax.wrapping_mul(8).wrapping_sub(0x18u64)) as u32) as u32 as u64; rdx = rsi.wrapping_add(0x18); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 10; continue; }
            10 => { crate::simd_ops::lk_sub_u32(rbp.wrapping_add(0x20u64), (1) as u32); r8 = 0; fl_l = 0; fl_r = 0; r9 = (crate::simd_ops::ru64(r14)) as u64; rdx = (crate::simd_ops::ru32(rbp.wrapping_add(0x74u64))) as u32 as u64; rcx = (crate::simd_ops::ru64(P3)) as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru32(rbp.wrapping_add(0x20u64))) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 5; continue; } bb = 11; continue; }
            11 => { bb = 13; continue; }
            12 => { rdx = rsi; rcx = rbp.wrapping_add(0x98); rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x4u64))) as u32 as u64; rcx = rbp.wrapping_add(0x18); rdx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(r12, ((rax as u32)) as u32); rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); bb = 13; continue; }
            13 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; bb = 14; continue; }
            14 => { rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x98u64))) as u32 as u64; rdi = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 16; continue; } bb = 15; continue; }
            15 => { crate::simd_ops::wu32(rbp.wrapping_add(0x50u64), (0x2u32) as u32); bb = 16; continue; }
            16 => { rcx = rbp.wrapping_add(0x28); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P7), rax), rax, rcx, rdx, r8, r9, rsp); r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rax = ((rdi as u32)) as u32 as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415f66e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143930f88; const P1: u64 = 0x1426452b0; const P2: u64 = 0x1426452a8; const P3: u64 = 0x142645560; const P4: u64 = 0x141620d10; const P5: u64 = 0x141620d50; const P6: u64 = 0x1415c2070; const P7: u64 = 0x14152ec40; const P8: u64 = 0x1415864b0; const P9: u64 = 0x1438152ac; const P10: u64 = 0x143814f60; const P11: u64 = 0x1415945c0; const P12: u64 = 0x1415bfb60; const P13: u64 = 0x1415c0d60; const P14: u64 = 0x14152ede0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 117>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu32(rsp.wrapping_add(0x18u64), ((r8 as u32)) as u32); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rbp = rsp.wrapping_sub(0x270); rsp = ((rsp).wrapping_sub(0x370u64)) as u64; fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(0x58u64))) as u64; fl_r = (0x0u64) as u64; r13 = ((r9 as u32)) as u32 as u64; r12 = ((rdx as u32)) as u32 as u64; r15 = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 32; continue; } bb = 1; continue; }
            1 => { rbx = (crate::simd_ops::ru64(P0)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x3b8u64), rdi); bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 12; continue; } bb = 3; continue; }
            3 => { rcx = ((rax.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; let cx_src: u32 = (rcx as u32); let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u32(rbx, fl_l as u32, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u32)) as u32 as u64; } if (((fl_l as u32)) != ((fl_r as u32))) { bb = 2; continue; } bb = 4; continue; }
            4 => { rsi = rbx.wrapping_add(0x8); bb = 5; continue; }
            5 => { rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x48u64))) as u32 as u64; r14 = 0; fl_l = 0; fl_r = 0; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 14; continue; } bb = 6; continue; }
            6 => { rdx = 0; fl_l = 0; fl_r = 0; rax = ((r12 as u32)) as u32 as u64; let d = ((rcx as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x40u64))) as u64; rdi = (crate::simd_ops::ru64(rax.wrapping_add(rdx.wrapping_mul(8)))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 7; continue; }
            7 => { fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x10u64))) as u32 as u64; fl_r = ((r12 as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 10; continue; } bb = 8; continue; }
            8 => { rdi = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 7; continue; } bb = 9; continue; }
            9 => { rdi = r14; bb = 10; continue; }
            10 => { fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 15; continue; } bb = 11; continue; }
            11 => { fl_l = (crate::simd_ops::lk_add_u32(rdi.wrapping_add(0x14u64), (1) as u32)) as u32 as u64; bb = 15; continue; }
            12 => { rsi = rbx.wrapping_add(0x8); rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 5; continue; } bb = 13; continue; }
            13 => { rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 2; continue; }
            14 => { rdi = r14; bb = 15; continue; }
            15 => { rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 22; continue; } bb = 16; continue; }
            16 => { rcx = ((rax.wrapping_sub(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; let cx_src: u32 = (rcx as u32); let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u32(rbx, fl_l as u32, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u32)) as u32 as u64; } if (((fl_l as u32)) == ((fl_r as u32))) { bb = 19; continue; } bb = 17; continue; }
            17 => { rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) >= 0) { bb = 16; continue; } bb = 18; continue; }
            18 => { bb = 23; continue; }
            19 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 24; continue; } bb = 20; continue; }
            20 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x30u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 24; continue; } bb = 21; continue; }
            21 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x38u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 24; continue; }
            22 => { rsi = rbx.wrapping_add(0x8); bb = 23; continue; }
            23 => { rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 24; continue; }
            24 => { fl_l = rdi; fl_r = rdi; rsi = rdi.wrapping_sub(0x10); rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x3b8u64))) as u64; rsi = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { r14 } else { rsi }) as u64; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 56; continue; } bb = 25; continue; }
            25 => { rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x2d0u64))) as u32 as u64; xmm0 = 0u128; xmm1 = 0u128; crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((r13 as u32)) as u32); crate::simd_ops::wu128(rsp.wrapping_add(0x38u64), (xmm0) as u128); crate::simd_ops::wu32(rsp.wrapping_add(0x34u64), ((rax as u32)) as u32); crate::simd_ops::wu128(rsp.wrapping_add(0x48u64), (xmm1) as u128); crate::simd_ops::wu32(rsp.wrapping_add(0x58u64), ((r14 as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), r14); crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), (0x0u64) as u64); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rax); fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 27; continue; } bb = 26; continue; }
            26 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; bb = 27; continue; }
            27 => { fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 34; continue; } bb = 28; continue; }
            28 => { rcx = rsi.wrapping_add(0x10); rax = 0xffffffff; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rcx.wrapping_add(0x14u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 30; continue; } bb = 29; continue; }
            29 => { rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 30; continue; }
            30 => { rcx = rsp.wrapping_add(0x60); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 32; continue; } bb = 31; continue; }
            31 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 32; continue; }
            32 => { rax = 0x2; bb = 33; continue; }
            33 => { rsp = ((rsp).wrapping_add(0x370u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            34 => { rcx = rbp.wrapping_add(0x20); crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), r14); crate::simd_ops::wu64(rbp.wrapping_sub(0x60u64), (0x0u64) as u64); crate::simd_ops::wu64(rbp.wrapping_sub(0x68u64), r14); crate::simd_ops::wu64(rbp.wrapping_sub(0x70u64), r14); crate::simd_ops::wu32(rbp.wrapping_sub(0x14u64), ((r14 as u32)) as u32); crate::simd_ops::wu64(rbp, (0x0u64) as u64); crate::simd_ops::wu16(rbp.wrapping_add(0x8u64), (0xff00u16) as u16); crate::simd_ops::wu64(rbp.wrapping_add(0x18u64), r14); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = ((crate::simd_ops::ru8(rbp.wrapping_add(0x268u64))) as u32) as u32 as u64; rax = (crate::simd_ops::ru32(r15.wrapping_add(0x74u64))) as u32 as u64; r8 = (r8 & !0xffu64) | ((((((r8 as u8)) & (0xf3u8))) as u8 as u64)); crate::simd_ops::lk_and_u8(rbp.wrapping_add(0x264u64), (0xfeu8) as u8); r8 = (r8 & !0xffu64) | ((((((r8 as u8)) | (0x3u8))) as u8 as u64)); rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x70u64))) as u64; rbx = (crate::simd_ops::ru64(r15.wrapping_add(0x60u64))) as u64; crate::simd_ops::wu32(rbp.wrapping_sub(0x5cu64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(r15.wrapping_add(0x68u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x68u64), rax); rax = (crate::simd_ops::ru32(r15.wrapping_add(0x70u64))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x260u64), ((r14 as u32)) as u32); crate::simd_ops::wu8(rbp.wrapping_add(0x268u64), ((r8 as u8)) as u8); crate::simd_ops::wu32(rbp.wrapping_sub(0x58u64), ((r14 as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x70u64), (0x2u32) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), r15); crate::simd_ops::wu32(rbp.wrapping_sub(0x60u64), ((rax as u32)) as u32); fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 36; continue; } bb = 35; continue; }
            35 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = ((crate::simd_ops::ru8(rbp.wrapping_add(0x268u64))) as u32) as u32 as u64; bb = 36; continue; }
            36 => { fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 38; continue; } bb = 37; continue; }
            37 => { fl_l = (crate::simd_ops::lk_add_u32(rbx, (1) as u32)) as u32 as u64; r8 = ((crate::simd_ops::ru8(rbp.wrapping_add(0x268u64))) as u32) as u32 as u64; bb = 38; continue; }
            38 => { rax = (((crate::simd_ops::ru32(rbp.wrapping_add(0x2c0u64))) as i32 as i64) as u64) as u64; r8 = (r8 & !0xffu64) | ((((((r8 as u8)) | (0x2u8))) as u8 as u64)); rcx = (crate::simd_ops::ru32(P9)) as u32 as u64; rcx = (((rcx) as i64).wrapping_mul((rax) as i64) as u64) as u64; rax = (0x20c49ba5e353f7cfu64) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x70u64), rbx); let prod = (rax as i64 as i128).wrapping_mul((rcx) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; crate::simd_ops::wu32(rbp.wrapping_sub(0x10u64), ((r14 as u32)) as u32); rdx = ((((rdx) as i64).wrapping_shr(7u32) as u64)) as u64; rax = rdx; crate::simd_ops::wu8(rbp.wrapping_add(0x268u64), ((r8 as u8)) as u8); rax = ((rax).wrapping_shr(63u32)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x4u64), ((r12 as u32)) as u32); fl_l = (crate::simd_ops::ru32(r15.wrapping_add(0x7cu64))) as u32 as u64; fl_r = 0x0; rax = (crate::simd_ops::ru64(r15.wrapping_add(0x58u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x80u64), rax); rax = rsp.wrapping_add(0x30); crate::simd_ops::wu64(rbp.wrapping_sub(0x78u64), rax); rax = rsp.wrapping_add(0x38); crate::simd_ops::wu64(rbp.wrapping_sub(0x8u64), rax); crate::simd_ops::wu32(rbp.wrapping_sub(0xcu64), ((rdx as u32)) as u32); if (((fl_l as u32)) != ((fl_r as u32))) { bb = 42; continue; } bb = 39; continue; }
            39 => { fl_l = (crate::simd_ops::ru32(r15.wrapping_add(0xbcu64))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 42; continue; } bb = 40; continue; }
            40 => { bb = 41; continue; }
            41 => { rcx = (crate::simd_ops::ru32(P10)) as u32 as u64; rax = ((rcx.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(P10, ((rax as u32)) as u32); crate::simd_ops::wu32(r15.wrapping_add(0xbcu64), ((rcx as u32)) as u32); fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 41; continue; } bb = 42; continue; }
            42 => { fl_l = (crate::simd_ops::ru8(rbp.wrapping_add(0x8u64))) as u8 as u64; fl_r = (0x90u8) as u8 as u64; rax = (crate::simd_ops::ru32(r15.wrapping_add(0xbcu64))) as u32 as u64; crate::simd_ops::wu32(rbp, ((rax as u32)) as u32); if (((fl_l as u8)) != ((fl_r as u8))) { bb = 46; continue; } bb = 43; continue; }
            43 => { fl_l = (crate::simd_ops::ru8(rbp.wrapping_add(0xdu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) <= ((fl_r as u8))) { bb = 46; continue; } bb = 44; continue; }
            44 => { r8 = (r8 & !0xffu64) | ((((((r8 as u8)) & (0x1u8))) as u8 as u64)); rcx = rsp.wrapping_add(0x70); crate::simd_ops::wu8(rbp.wrapping_add(0x2b0u64), ((r8 as u8)) as u8); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P11, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = ((rax as u32)) as u32 as u64; r9 = rbp.wrapping_add(0x2b0); rax = (crate::simd_ops::ru32(rbp.wrapping_sub(0x5cu64))) as u32 as u64; rdx = rbp.wrapping_add(0x8); crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rax as u32)) as u32); rcx = rsi; rax = (crate::simd_ops::ru64(rbp.wrapping_sub(0x80u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P12, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 50; continue; } bb = 45; continue; }
            45 => { rax = ((crate::simd_ops::ru8(rbp.wrapping_add(0x268u64))) as u32) as u32 as u64; rcx = ((crate::simd_ops::ru8(rbp.wrapping_add(0x2b0u64))) as u32) as u32 as u64; rcx = (rcx & !0xffu64) | ((((((rcx as u8)) ^ ((rax as u8)))) as u8 as u64)); rcx = (rcx & !0xffu64) | ((((((rcx as u8)) & ((rbx as u8)))) as u8 as u64)); fl_l = ((((rax as u8)) ^ ((rcx as u8)))) as u8 as u64; fl_r = fl_l; rax = (rax & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); crate::simd_ops::wu8(rbp.wrapping_add(0x268u64), ((rax as u8)) as u8); bb = 46; continue; }
            46 => { rdx = rsp.wrapping_add(0x70); rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P13, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x3; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 48; continue; } bb = 47; continue; }
            47 => { rbx = ((rax.wrapping_sub(0x2u64)) as u32) as u32 as u64; bb = 50; continue; }
            48 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 50; continue; } bb = 49; continue; }
            49 => { rax = (crate::simd_ops::ru64(rsi)) as u64; rdx = rsp.wrapping_add(0x70); rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x250u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = ((rax as u32)) as u32 as u64; bb = 50; continue; }
            50 => { rcx = rsi.wrapping_add(0x10); rax = 0xffffffff; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rcx.wrapping_add(0x14u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 52; continue; } bb = 51; continue; }
            51 => { rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 52; continue; }
            52 => { rcx = rbp.wrapping_add(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P14, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x70u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 54; continue; } bb = 53; continue; }
            53 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 54; continue; }
            54 => { rcx = rsp.wrapping_add(0x60); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 57; continue; } bb = 55; continue; }
            55 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = ((rbx as u32)) as u32 as u64; bb = 33; continue; }
            56 => { rbx = 0xf; bb = 57; continue; }
            57 => { rax = ((rbx as u32)) as u32 as u64; bb = 33; continue; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415f6ad0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143930f08; const P1: u64 = 0x1415a4a50; const P2: u64 = 0x143930f88; const P3: u64 = 0x1426452b0; const P4: u64 = 0x1426452a8; const P5: u64 = 0x142645560; const P6: u64 = 0x1415a34b0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); crate::simd_ops::wu32(rsp.wrapping_add(0x18u64), ((r8 as u32)) as u32); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rdi = 0; fl_l = 0; fl_r = 0; r12 = ((rdx as u32)) as u32 as u64; rdx = (crate::simd_ops::ru32(rcx.wrapping_add(0x74u64))) as u32 as u64; r14 = rcx; crate::simd_ops::wu32(rcx.wrapping_add(0x50u64), ((rdi as u32)) as u32); r13 = ((r9 as u32)) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(0xb0u64), ((rdi as u32)) as u32); rcx = (crate::simd_ops::ru64(P0)) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rbp = (crate::simd_ops::ru32(r14.wrapping_add(0x98u64))) as u32 as u64; r15 = ((rdi.wrapping_sub(0x1u64)) as u32) as u32 as u64; fl_l = ((rbp as u32)) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 28; continue; } bb = 1; continue; }
            1 => { fl_l = ((r12 as u32)) as u32 as u64; fl_r = ((r12 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 28; continue; } bb = 2; continue; }
            2 => { rbx = (crate::simd_ops::ru64(P2)) as u64; bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 17; continue; } bb = 4; continue; }
            4 => { rcx = ((rax.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; let cx_src: u32 = (rcx as u32); let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u32(rbx, fl_l as u32, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u32)) as u32 as u64; } if (((fl_l as u32)) != ((fl_r as u32))) { bb = 3; continue; } bb = 5; continue; }
            5 => { rsi = rbx.wrapping_add(0x8); bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x48u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 13; continue; } bb = 7; continue; }
            7 => { rdx = 0; fl_l = 0; fl_r = 0; rax = ((rbp as u32)) as u32 as u64; let d = ((rcx as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x40u64))) as u64; rdi = (crate::simd_ops::ru64(rax.wrapping_add(rdx.wrapping_mul(8)))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 8; continue; }
            8 => { fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x10u64))) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 11; continue; } bb = 9; continue; }
            9 => { rdi = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 8; continue; } bb = 10; continue; }
            10 => { rbp = 0; fl_l = 0; fl_r = 0; rdi = ((rbp as u32)) as u32 as u64; bb = 11; continue; }
            11 => { fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 12; continue; }
            12 => { fl_l = (crate::simd_ops::lk_add_u32(rdi.wrapping_add(0x14u64), (1) as u32)) as u32 as u64; bb = 13; continue; }
            13 => { rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 22; continue; } bb = 14; continue; }
            14 => { rcx = ((rax.wrapping_sub(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; let cx_src: u32 = (rcx as u32); let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u32(rbx, fl_l as u32, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u32)) as u32 as u64; } if (((fl_l as u32)) == ((fl_r as u32))) { bb = 19; continue; } bb = 15; continue; }
            15 => { rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) >= 0) { bb = 14; continue; } bb = 16; continue; }
            16 => { bb = 23; continue; }
            17 => { rsi = rbx.wrapping_add(0x8); rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 6; continue; } bb = 18; continue; }
            18 => { rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); bb = 3; continue; }
            19 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 24; continue; } bb = 20; continue; }
            20 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x30u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 24; continue; } bb = 21; continue; }
            21 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x38u64))) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); bb = 24; continue; }
            22 => { rsi = rbx.wrapping_add(0x8); bb = 23; continue; }
            23 => { rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); bb = 24; continue; }
            24 => { fl_l = rdi; fl_r = rdi; rbx = rdi.wrapping_sub(0x10); rdi = 0x0; rbx = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rdi } else { rbx }) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 27; continue; } bb = 25; continue; }
            25 => { r9 = (crate::simd_ops::ru32(rsp.wrapping_add(0x70u64))) as u32 as u64; rcx = rbx; r8 = (crate::simd_ops::ru32(r14.wrapping_add(0x74u64))) as u32 as u64; rdx = (crate::simd_ops::ru64(r14.wrapping_add(0x58u64))) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((r13 as u32)) as u32); rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rcx = rbx.wrapping_add(0x10); rax = ((r15 as u32)) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rcx.wrapping_add(0x14u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 27; continue; } bb = 26; continue; }
            26 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 27; continue; }
            27 => { rcx = r14.wrapping_add(0x28); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); rcx = r14.wrapping_add(0x28); crate::simd_ops::wu32(r14.wrapping_add(0x98u64), ((rdi as u32)) as u32); crate::simd_ops::wu64(r14.wrapping_add(0xa0u64), rdi); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); bb = 28; continue; }
            28 => { rbp = (crate::simd_ops::ru32(r14.wrapping_add(0x80u64))) as u32 as u64; fl_l = ((rbp as u32)) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 60; continue; } bb = 29; continue; }
            29 => { rbx = (crate::simd_ops::ru64(P2)) as u64; bb = 30; continue; }
            30 => { rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 40; continue; } bb = 31; continue; }
            31 => { rcx = ((rax.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; let cx_src: u32 = (rcx as u32); let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u32(rbx, fl_l as u32, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u32)) as u32 as u64; } if (((fl_l as u32)) != ((fl_r as u32))) { bb = 30; continue; } bb = 32; continue; }
            32 => { rsi = rbx.wrapping_add(0x8); bb = 33; continue; }
            33 => { rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x48u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 43; continue; } bb = 34; continue; }
            34 => { rdx = 0; fl_l = 0; fl_r = 0; rax = ((rbp as u32)) as u32 as u64; let d = ((rcx as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x40u64))) as u64; rdi = (crate::simd_ops::ru64(rax.wrapping_add(rdx.wrapping_mul(8)))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 37; continue; } bb = 35; continue; }
            35 => { fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x10u64))) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 42; continue; } bb = 36; continue; }
            36 => { rdi = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 35; continue; } bb = 37; continue; }
            37 => { rbp = 0; fl_l = 0; fl_r = 0; rdi = ((rbp as u32)) as u32 as u64; bb = 38; continue; }
            38 => { fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 44; continue; } bb = 39; continue; }
            39 => { fl_l = (crate::simd_ops::lk_add_u32(rdi.wrapping_add(0x14u64), (1) as u32)) as u32 as u64; bb = 44; continue; }
            40 => { rsi = rbx.wrapping_add(0x8); rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 33; continue; } bb = 41; continue; }
            41 => { rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); bb = 30; continue; }
            42 => { rbp = 0; fl_l = 0; fl_r = 0; bb = 38; continue; }
            43 => { rbp = 0; fl_l = 0; fl_r = 0; bb = 44; continue; }
            44 => { rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 52; continue; } bb = 45; continue; }
            45 => { bb = 46; continue; }
            46 => { rcx = ((rax.wrapping_sub(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; let cx_src: u32 = (rcx as u32); let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u32(rbx, fl_l as u32, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u32)) as u32 as u64; } if (((fl_l as u32)) == ((fl_r as u32))) { bb = 49; continue; } bb = 47; continue; }
            47 => { rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) >= 0) { bb = 46; continue; } bb = 48; continue; }
            48 => { bb = 53; continue; }
            49 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 54; continue; } bb = 50; continue; }
            50 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x30u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 54; continue; } bb = 51; continue; }
            51 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x38u64))) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); bb = 54; continue; }
            52 => { rsi = rbx.wrapping_add(0x8); bb = 53; continue; }
            53 => { rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); bb = 54; continue; }
            54 => { fl_l = rdi; fl_r = rdi; rbx = rdi.wrapping_sub(0x10); rbx = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rbp } else { rbx }) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 60; continue; } bb = 55; continue; }
            55 => { rdx = (crate::simd_ops::ru64(r14.wrapping_add(0x58u64))) as u64; rcx = rbx; fl_l = ((r12 as u32)) as u32 as u64; fl_r = ((r12 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 57; continue; } bb = 56; continue; }
            56 => { r9 = (crate::simd_ops::ru32(rsp.wrapping_add(0x70u64))) as u32 as u64; r8 = (crate::simd_ops::ru32(r14.wrapping_add(0x74u64))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((r13 as u32)) as u32); rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); bb = 58; continue; }
            57 => { rax = (crate::simd_ops::ru64(rbx)) as u64; r8 = rbx; r9 = (crate::simd_ops::ru32(r14.wrapping_add(0x74u64))) as u32 as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0xa0u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 58; continue; }
            58 => { rcx = rbx.wrapping_add(0x10); let xa_src: u32 = (r15 as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rcx.wrapping_add(0x14u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; r15 = (xa_old) as u32 as u64; fl_l = ((r15 as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 60; continue; } bb = 59; continue; }
            59 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 60; continue; }
            60 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rax = 0x1; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415f6d80(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x38; const P1: u64 = 0x1415242e0; const P2: u64 = 0x30; const P3: u64 = 0x18; const P4: u64 = 0x1415c5840; const P5: u64 = 0x1415bd160; const P6: u64 = 0x141834d30;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdi = ((rdx as u32)) as u32 as u64;
    rbx = rcx;
    rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    fl_l = rdx; fl_r = rdx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rcx = 0x1; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); }
    crate::simd_ops::wu64(rbx.wrapping_add(P0), (0x0u64) as u64);
    rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64;
    fl_l = rdx; fl_r = rdx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rcx = 0x1; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); }
    rcx = ((rbx.wrapping_add(P3)) as u64) as u64;
    crate::simd_ops::wu64(rbx.wrapping_add(P2), (0x0u64) as u64);
    rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp);
    rcx = rbx;
    rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp);
    fl_l = ((rdi as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64;
    if !(((((fl_l as u8)) & ((fl_r as u8))) == 0)) { rdx = 0x40; rcx = rbx; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); }
    rax = rbx;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415f6e00(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xffffffffffffffc1; const P1: u64 = 0x141524330; const P2: u64 = 0x1415bd140; const P3: u64 = 0x14268e000; const P4: u64 = 0x18; const P5: u64 = 0x1415c5820; const P6: u64 = 0x30; const P7: u64 = 0x38;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdx = 0x40;
    rcx = ((rdx.wrapping_add(P0)) as u32) as u32 as u64;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rbx = rax;
    fl_l = rax; fl_r = rax;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rcx = rax; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rax = P3; rcx = ((rbx.wrapping_add(P4)) as u64) as u64; crate::simd_ops::wu64(rbx, rax); rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rax = rbx; crate::simd_ops::wu64(rbx.wrapping_add(P6), (0x0u64) as u64); crate::simd_ops::wu64(rbx.wrapping_add(P7), (0x0u64) as u64); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415f6e60(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415242e0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    fl_l = rcx; fl_r = rcx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rbx = rcx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rdx = rbx; rcx = 0x1; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); }
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415f6ed0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415bd1d0;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rdx = 0x9;
    return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415f6ee0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143930f88; const P1: u64 = 0x1426452a8; const P2: u64 = 0x1426452b0; const P3: u64 = 0x142645560; const P4: u64 = 0x141581080;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = (crate::simd_ops::ru64(P0)) as u64; rsi = r8; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), r15); r14 = rdx; r15 = rcx; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rdi); rbp = ((r9 as u32)) as u32 as u64; bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x318u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 11; continue; } bb = 2; continue; }
            2 => { r9 = ((rax.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; let cx_src: u32 = (r9 as u32); let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u32(rbx.wrapping_add(0x318u64), fl_l as u32, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u32)) as u32 as u64; } if (((fl_l as u32)) != ((fl_r as u32))) { bb = 1; continue; } bb = 3; continue; }
            3 => { rdi = rbx.wrapping_add(0x320); bb = 4; continue; }
            4 => { fl_l = ((rbp as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(r15.wrapping_add(0x18u64))) as u32 as u64; r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x318u64))) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 16; continue; } bb = 5; continue; }
            5 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 9; continue; } bb = 6; continue; }
            6 => { bb = 7; continue; }
            7 => { rcx = ((rax.wrapping_sub(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; let cx_src: u32 = (rcx as u32); let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u32(rbx.wrapping_add(0x318u64), fl_l as u32, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u32)) as u32 as u64; } if (((fl_l as u32)) == ((fl_r as u32))) { bb = 13; continue; } bb = 8; continue; }
            8 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x318u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) >= 0) { bb = 7; continue; } bb = 9; continue; }
            9 => { rcx = rdi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); bb = 10; continue; }
            10 => { rax = 0x2; bb = 27; continue; }
            11 => { rdi = rbx.wrapping_add(0x320); rcx = rdi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x318u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 4; continue; } bb = 12; continue; }
            12 => { rcx = rdi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); bb = 1; continue; }
            13 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 10; continue; } bb = 14; continue; }
            14 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x348u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 10; continue; } bb = 15; continue; }
            15 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x350u64))) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); rax = 0x2; bb = 27; continue; }
            16 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 20; continue; } bb = 17; continue; }
            17 => { bb = 18; continue; }
            18 => { rcx = ((rax.wrapping_sub(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; let cx_src: u32 = (rcx as u32); let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u32(rbx.wrapping_add(0x318u64), fl_l as u32, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u32)) as u32 as u64; } if (((fl_l as u32)) == ((fl_r as u32))) { bb = 28; continue; } bb = 19; continue; }
            19 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x318u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) >= 0) { bb = 18; continue; } bb = 20; continue; }
            20 => { rcx = rdi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); bb = 21; continue; }
            21 => { fl_l = ((rbp as u32)) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 26; continue; } bb = 22; continue; }
            22 => { rbx = rbp; bb = 23; continue; }
            23 => { rcx = (crate::simd_ops::ru64(r14)) as u64; rax = 0; fl_l = 0; fl_r = 0; fl_l = (crate::simd_ops::ru8(rcx)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 25; continue; } bb = 24; continue; }
            24 => { rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 25; continue; }
            25 => { crate::simd_ops::wu32(rsi, ((rax as u32)) as u32); r14 = ((r14).wrapping_add(0x8u64)) as u64; rsi = ((rsi).wrapping_add(0x4u64)) as u64; fl_l = rbx; fl_r = (0x1u64) as u64; rbx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 23; continue; } bb = 26; continue; }
            26 => { rax = 0x1; bb = 27; continue; }
            27 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            28 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 21; continue; } bb = 29; continue; }
            29 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x348u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 21; continue; } bb = 30; continue; }
            30 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x350u64))) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); bb = 21; continue; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415f7050(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143930f88; const P1: u64 = 0x1426452a8; const P2: u64 = 0x1426452b0; const P3: u64 = 0x142645560; const P4: u64 = 0x1415810d0; const P5: u64 = 0x142645718;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 45>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = rsp; crate::simd_ops::wu64(rax.wrapping_add(0x18u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x150u64)) as u64; rbx = (crate::simd_ops::ru64(P0)) as u64; r14 = r8; crate::simd_ops::wu64(rax.wrapping_add(0x10u64), r15); rsi = rdx; r15 = rcx; crate::simd_ops::wu64(rax.wrapping_add(0x8u64), rdi); rbp = ((r9 as u32)) as u32 as u64; bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x318u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 11; continue; } bb = 2; continue; }
            2 => { r9 = ((rax.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; let cx_src: u32 = (r9 as u32); let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u32(rbx.wrapping_add(0x318u64), fl_l as u32, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u32)) as u32 as u64; } if (((fl_l as u32)) != ((fl_r as u32))) { bb = 1; continue; } bb = 3; continue; }
            3 => { rdi = rbx.wrapping_add(0x320); bb = 4; continue; }
            4 => { fl_l = ((rbp as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(r15.wrapping_add(0x18u64))) as u32 as u64; r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x178u64))) as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x318u64))) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 16; continue; } bb = 5; continue; }
            5 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 9; continue; } bb = 6; continue; }
            6 => { bb = 7; continue; }
            7 => { rcx = ((rax.wrapping_sub(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; let cx_src: u32 = (rcx as u32); let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u32(rbx.wrapping_add(0x318u64), fl_l as u32, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u32)) as u32 as u64; } if (((fl_l as u32)) == ((fl_r as u32))) { bb = 13; continue; } bb = 8; continue; }
            8 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x318u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) >= 0) { bb = 7; continue; } bb = 9; continue; }
            9 => { rcx = rdi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); bb = 10; continue; }
            10 => { rax = 0x2; bb = 38; continue; }
            11 => { rdi = rbx.wrapping_add(0x320); rcx = rdi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x318u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 4; continue; } bb = 12; continue; }
            12 => { rcx = rdi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); bb = 1; continue; }
            13 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 10; continue; } bb = 14; continue; }
            14 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x348u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 10; continue; } bb = 15; continue; }
            15 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x350u64))) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); rax = 0x2; bb = 38; continue; }
            16 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 20; continue; } bb = 17; continue; }
            17 => { bb = 18; continue; }
            18 => { rcx = ((rax.wrapping_sub(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; let cx_src: u32 = (rcx as u32); let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u32(rbx.wrapping_add(0x318u64), fl_l as u32, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u32)) as u32 as u64; } if (((fl_l as u32)) == ((fl_r as u32))) { bb = 31; continue; } bb = 19; continue; }
            19 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x318u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) >= 0) { bb = 18; continue; } bb = 20; continue; }
            20 => { rcx = rdi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); bb = 21; continue; }
            21 => { fl_l = ((rbp as u32)) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 37; continue; } bb = 22; continue; }
            22 => { rdi = rbp; rbp = 0; fl_l = 0; fl_r = 0; bb = 23; continue; }
            23 => { rcx = (crate::simd_ops::ru64(rsi)) as u64; rbx = ((rbp as u32)) as u32 as u64; fl_l = (crate::simd_ops::ru16(rcx)) as u16 as u64; fl_r = ((rbx as u16)) as u16 as u64; if (((fl_l as u16)) == ((fl_r as u16))) { bb = 36; continue; } bb = 24; continue; }
            24 => { rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rbx = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 36; continue; } bb = 25; continue; }
            25 => { r8 = (crate::simd_ops::ru64(rsi)) as u64; rbx = ((rbp as u32)) as u32 as u64; rax = (0xffffffffffffffffu64) as u64; bb = 26; continue; }
            26 => { rax = ((rax).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru16(r8.wrapping_add(rax.wrapping_mul(2)))) as u16 as u64; fl_r = ((rbx as u16)) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 26; continue; } bb = 27; continue; }
            27 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x102; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 34; continue; } bb = 28; continue; }
            28 => { rax = (0xffffffffffffffffu64) as u64; bb = 29; continue; }
            29 => { rax = ((rax).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru16(r8.wrapping_add(rax.wrapping_mul(2)))) as u16 as u64; fl_r = ((rbx as u16)) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 29; continue; } bb = 30; continue; }
            30 => { bb = 35; continue; }
            31 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 21; continue; } bb = 32; continue; }
            32 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x348u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 21; continue; } bb = 33; continue; }
            33 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x350u64))) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); bb = 21; continue; }
            34 => { rax = 0x102; bb = 35; continue; }
            35 => { crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rbp); rcx = rsp.wrapping_add(0x40); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbp); r9 = ((rax as u32)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0x103u32) as u32); rdx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rcx); rcx = 0xfde9; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); rcx = ((((rax as u32)) as i32 as i64) as u64) as u64; crate::simd_ops::wu8(rsp.wrapping_add(rcx).wrapping_add(0x40u64), ((rbx as u8)) as u8); bb = 36; continue; }
            36 => { crate::simd_ops::wu32(r14, ((rbx as u32)) as u32); rsi = ((rsi).wrapping_add(0x8u64)) as u64; r14 = ((r14).wrapping_add(0x4u64)) as u64; fl_l = rdi; fl_r = (0x1u64) as u64; rdi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 23; continue; } bb = 37; continue; }
            37 => { rax = 0x1; bb = 38; continue; }
            38 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x170u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x180u64))) as u64; rsp = ((rsp).wrapping_add(0x150u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415f7260(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415c5d20; const P1: u64 = 0x141524340; const P2: u64 = 0x14183acbc; const P3: u64 = 0x1415242e0;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; r10 = ((crate::simd_ops::ru8(rdx.wrapping_add(0x4u64))) as u32) as u32 as u64; rsi = rcx; r9 = (crate::simd_ops::ru32(rdx.wrapping_add(0x5u64))) as u32 as u64; rdx = ((rdx).wrapping_add(0x5u64)) as u64; rdi = ((r9.wrapping_add(r9.wrapping_mul(4))) as u32) as u32 as u64; rdi = ((rdi).wrapping_add(rdx)) as u64; rbx = (crate::simd_ops::ru32(rdi.wrapping_add(0x4u64))) as u32 as u64; crate::simd_ops::wu16(rcx.wrapping_add(0x28u64), ((r10 as u16)) as u16); r8 = ((rbx as u32)) as u32 as u64; rax = ((crate::simd_ops::ru8(rdi.wrapping_add(0x8u64))) as u32) as u32 as u64; rdi = ((rdi).wrapping_add(0x9u64)) as u64; crate::simd_ops::wu8(rcx.wrapping_add(0x2au64), ((rax as u8)) as u8); rdx = rdi; rcx = ((rcx).wrapping_add(0x18u64)) as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 16; continue; } bb = 1; continue; }
            1 => { rdx = (crate::simd_ops::ru64(rsi.wrapping_add(0x30u64))) as u64; rdi = ((rdi).wrapping_add(rbx)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), r14); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), r15); rbx = ((crate::simd_ops::ru8(rdi)) as u32) as u32 as u64; rdi = ((rdi).wrapping_add(1)) as u64; fl_l = ((rbx as u8)) as u8 as u64; fl_r = ((rbx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 5; continue; } bb = 2; continue; }
            2 => { r15 = ((rbx.wrapping_add(0x4u64)) as u32) as u32 as u64; rcx = ((rax as u32)) as u32 as u64; r15 = ((((r15 as u32)) & (0xfffffffcu32))) as u32 as u64; r8 = ((r15.wrapping_add(rbx.wrapping_mul(4))) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); r14 = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = ((r14.wrapping_add(0x34u64)) as u32) as u32 as u64; bb = 15; continue; }
            4 => { rcx = rax.wrapping_add(0x1); crate::simd_ops::wu8(rax, ((rbx as u8)) as u8); r8 = rbx; rdx = rdi; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rdi = ((rdi).wrapping_add(rbx)) as u64; rcx = ((r15 as u32)) as u32 as u64; rbx = ((rbx.wrapping_mul(4)) as u64) as u64; rcx = ((rcx).wrapping_add(r14)) as u64; r8 = rbx; rdx = rdi; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rdi).wrapping_add(rbx)) as u64; fl_r = fl_l; rdi = ((fl_l as u64)) as u64; crate::simd_ops::wu64(rsi.wrapping_add(0x30u64), r14); bb = 7; continue; }
            5 => { fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { rcx = 0x1; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rsi.wrapping_add(0x30u64), (0x0u64) as u64); bb = 7; continue; }
            7 => { rbx = ((crate::simd_ops::ru8(rdi)) as u32) as u32 as u64; rdx = (crate::simd_ops::ru64(rsi.wrapping_add(0x38u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rbp); rbp = rdi.wrapping_add(0x1); fl_l = ((rbx as u8)) as u8 as u64; fl_r = ((rbx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 11; continue; } bb = 8; continue; }
            8 => { r14 = ((rbx.wrapping_add(0x4u64)) as u32) as u32 as u64; rcx = 0x1; r14 = ((((r14 as u32)) & (0xfffffffcu32))) as u32 as u64; r8 = ((r14.wrapping_add(rbx.wrapping_mul(8))) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rdi = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { rax = ((rdi.wrapping_add(0x34u64)) as u32) as u32 as u64; bb = 14; continue; }
            10 => { rcx = rax.wrapping_add(0x1); crate::simd_ops::wu8(rax, ((rbx as u8)) as u8); r8 = rbx; rdx = rbp; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = ((r14 as u32)) as u32 as u64; r8 = ((rbx.wrapping_mul(8)) as u64) as u64; rcx = ((rcx).wrapping_add(rdi)) as u64; rdx = ((rbx.wrapping_add(rbp)) as u64) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rsi.wrapping_add(0x38u64), rdi); bb = 13; continue; }
            11 => { fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 12; continue; }
            12 => { rcx = 0x1; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rsi.wrapping_add(0x38u64), (0x0u64) as u64); bb = 13; continue; }
            13 => { rax = 0x1; bb = 14; continue; }
            14 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; bb = 15; continue; }
            15 => { r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; bb = 16; continue; }
            16 => { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415f73d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x18; const P3: u64 = 0x20; const P4: u64 = 0x28; const P5: u64 = 0x1426452b8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rax = 0;
    fl_l = 0; fl_r = 0;
    rbx = rcx;
    crate::simd_ops::wu64(rcx, rax);
    crate::simd_ops::wu64(rcx.wrapping_add(P0), rax);
    crate::simd_ops::wu64(rcx.wrapping_add(P1), rax);
    crate::simd_ops::wu64(rcx.wrapping_add(P2), rax);
    crate::simd_ops::wu64(rcx.wrapping_add(P3), rax);
    crate::simd_ops::wu64(rcx.wrapping_add(P4), rax);
    rcx = ((rcx).wrapping_add(0x30u64)) as u64;
    rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp);
    rax = rbx;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415f7410(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x1415242e0; const P2: u64 = 0xc; const P3: u64 = 0x10; const P4: u64 = 0x18; const P5: u64 = 0x1c; const P6: u64 = 0x20; const P7: u64 = 0x28; const P8: u64 = 0x2c; const P9: u64 = 0x30; const P10: u64 = 0x1426452a0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdx = (crate::simd_ops::ru64(rcx)) as u64; rdi = 0; fl_l = 0; fl_r = 0; rbx = rcx; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu32(rcx.wrapping_add(P0), ((rdi as u32)) as u32); rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx, rdi); crate::simd_ops::wu32(rbx.wrapping_add(P2), ((rdi as u32)) as u32); bb = 2; continue; }
            2 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P3))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rbx.wrapping_add(P4), ((rdi as u32)) as u32); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(P3), rdi); crate::simd_ops::wu32(rbx.wrapping_add(P5), ((rdi as u32)) as u32); bb = 4; continue; }
            4 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P6))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rbx.wrapping_add(P7), ((rdi as u32)) as u32); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(P6), rdi); crate::simd_ops::wu32(rbx.wrapping_add(P8), ((rdi as u32)) as u32); bb = 6; continue; }
            6 => { rcx = ((rbx.wrapping_add(P9)) as u64) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P10), rax), rax, rcx, rdx, r8, r9, rsp); }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415f7480(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x30; const P1: u64 = 0x1426452b0; const P2: u64 = 0x10; const P3: u64 = 0x18; const P4: u64 = 0x8; const P5: u64 = 0x1426452a8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rbx = ((rcx.wrapping_add(P0)) as u64) as u64; rdi = rcx; rcx = rbx; rbp = r9; r14 = r8; rsi = ((rdx as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rdi.wrapping_add(P2))) as u64; rcx = (crate::simd_ops::ru32(rdi.wrapping_add(P3))) as u32 as u64; rcx = ((rcx).wrapping_shl(5u32)) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 3; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru32(rax)) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rax = ((rax).wrapping_add(0x20u64)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 3; continue; }
            3 => { r8 = 0; fl_l = 0; fl_r = 0; rdx = ((rax.wrapping_add(P4)) as u64) as u64; fl_l = rax; fl_r = rcx; rdx = (if (((fl_l as u64)) == ((fl_r as u64))) { r8 } else { rdx }) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(P2))) as u64; rcx = rsp.wrapping_add(0x20); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x70u64))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), ((rax as u32)) as u32); rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P4))) as u32) as u32 as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x3cu64), ((rax as u8)) as u8); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), r14); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbp); rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rdx)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rdi = (rdi & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 6; continue; }
            5 => { rdi = (rdi & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 6; continue; }
            6 => { rcx = rbx; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rax = (((rdi as u8)) as u32) as u32 as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415f7540(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x30; const P1: u64 = 0x1426452b0; const P2: u64 = 0x8; const P3: u64 = 0x1426452a8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = ((rcx.wrapping_add(P0)) as u64) as u64; rdi = rcx; rcx = rbx; r14 = r8; rsi = ((rdx as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru32(rdi.wrapping_add(P2))) as u32 as u64; rax = (crate::simd_ops::ru64(rdi)) as u64; r9 = ((rcx.wrapping_add(rcx.wrapping_mul(2))) as u64) as u64; rcx = ((rax.wrapping_add(r9.wrapping_mul(8))) as u64) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 4; continue; } bb = 1; continue; }
            1 => { bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru32(rax)) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = ((rax).wrapping_add(0x18u64)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 4; continue; }
            4 => { r8 = 0; fl_l = 0; fl_r = 0; rdx = ((rax.wrapping_add(P2)) as u64) as u64; fl_l = rax; fl_r = rcx; rdx = (if (((fl_l as u64)) == ((fl_r as u64))) { r8 } else { rdx }) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(P2))) as u64; rcx = r14; crate::simd_ops::wu64(r14, rax); rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rdx)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rdi = (rdi & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 7; continue; }
            6 => { rdi = (rdi & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 7; continue; }
            7 => { rcx = rbx; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = (((rdi as u8)) as u32) as u32 as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415f75e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x30; const P1: u64 = 0x1426452b0; const P2: u64 = 0x10; const P3: u64 = 0x18; const P4: u64 = 0x8; const P5: u64 = 0x1426452a8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = ((rcx.wrapping_add(P0)) as u64) as u64; rdi = rcx; rcx = rbx; rsi = ((rdx as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rdi.wrapping_add(P2))) as u64; rcx = (crate::simd_ops::ru32(rdi.wrapping_add(P3))) as u32 as u64; rcx = ((rcx).wrapping_shl(5u32)) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 3; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru32(rax)) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rax = ((rax).wrapping_add(0x20u64)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 3; continue; }
            3 => { r8 = 0; fl_l = 0; fl_r = 0; rdx = ((rax.wrapping_add(P4)) as u64) as u64; fl_l = rax; fl_r = rcx; rdx = (if (((fl_l as u64)) == ((fl_r as u64))) { r8 } else { rdx }) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rdi = ((crate::simd_ops::ru8(rdx.wrapping_add(P4))) as u32) as u32 as u64; bb = 6; continue; }
            5 => { rdi = (rdi & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 6; continue; }
            6 => { rcx = rbx; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = (((rdi as u8)) as u32) as u32 as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415f7660(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x30; const P1: u64 = 0x1426452b0; const P2: u64 = 0x8; const P3: u64 = 0x1426452a8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = ((rcx.wrapping_add(P0)) as u64) as u64; rdi = rcx; rcx = rsi; rbp = ((rdx as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rdi.wrapping_add(P2))) as u32 as u64; rbx = (crate::simd_ops::ru64(rdi)) as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rdi = ((rbx.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 4; continue; } bb = 1; continue; }
            1 => { bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru32(rbx)) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rbx = ((rbx).wrapping_add(0x18u64)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 4; continue; }
            4 => { rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rax = ((rbx.wrapping_add(P2)) as u64) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rcx = 0; fl_l = 0; fl_r = 0; fl_l = rbx; fl_r = rdi; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = (if (((fl_l as u64)) == ((fl_r as u64))) { rcx } else { rax }) as u64; fl_l = rax; fl_r = rax; rax = (rax & !0xffu64) | (((if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { 1u8 } else { 0u8 }) as u8 as u64)); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415f76e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415b16f0; const P1: u64 = 0x1415e9510; const P2: u64 = 0x1415b1830; const P3: u64 = 0x1426452b0; const P4: u64 = 0x14183acc8; const P5: u64 = 0x14158ec20; const P6: u64 = 0x1426452a8; const P7: u64 = 0x1415e90b0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rsi = rcx; r14 = (((r9 as u8)) as u32) as u32 as u64; rcx = rsp.wrapping_add(0x20); r12 = r8; rdi = rdx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rcx = rdi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = rsp.wrapping_add(0x20); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rax = 0x5d; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            2 => { crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), r15); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = rsi.wrapping_add(0x30); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); r15 = 0; fl_l = 0; fl_r = 0; fl_l = r12; fl_r = r12; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 11; continue; } bb = 3; continue; }
            3 => { fl_l = ((r14 as u8)) as u8 as u64; fl_r = ((r14 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 11; continue; } bb = 4; continue; }
            4 => { r8 = (crate::simd_ops::ru64(rsi.wrapping_add(0x20u64))) as u64; rdx = (crate::simd_ops::ru32(rsi.wrapping_add(0x28u64))) as u32 as u64; rcx = r8; rax = ((rdx as u32)) as u32 as u64; rax = ((rax).wrapping_shl(5u32)) as u64; rax = ((rax).wrapping_add(r8)) as u64; fl_l = r8; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 22; continue; } bb = 5; continue; }
            5 => { fl_l = (crate::simd_ops::ru64(rcx)) as u64; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 8; continue; } bb = 6; continue; }
            6 => { rcx = ((rcx).wrapping_add(0x20u64)) as u64; fl_l = rcx; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 5; continue; } bb = 7; continue; }
            7 => { bb = 22; continue; }
            8 => { rdx = (((rdx as u32)).wrapping_sub(1)) as u32 as u64; rdx = ((rdx).wrapping_shl(5u32)) as u64; rdx = ((rdx).wrapping_add(r8)) as u64; fl_l = rcx; fl_r = rdx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 10; continue; } bb = 9; continue; }
            9 => { rdx = ((rdx).wrapping_sub(rcx)) as u64; rdx = ((((rdx as u32)) & (0xffffffe0u32))) as u32 as u64; r8 = ((rdx as u32)) as u32 as u64; rdx = rcx.wrapping_add(0x20); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 10; continue; }
            10 => { fl_l = (crate::simd_ops::lk_sub_u32(rsi.wrapping_add(0x28u64), (1) as u32)) as u32 as u64; bb = 22; continue; }
            11 => { rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x20u64))) as u64; rbp = (crate::simd_ops::ru32(rsi.wrapping_add(0x28u64))) as u32 as u64; rcx = ((rbp as u32)) as u32 as u64; rcx = ((rcx).wrapping_shl(5u32)) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 14; continue; } bb = 12; continue; }
            12 => { fl_l = (crate::simd_ops::ru64(rax)) as u64; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 14; continue; } bb = 13; continue; }
            13 => { rax = ((rax).wrapping_add(0x20u64)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 12; continue; } bb = 14; continue; }
            14 => { fl_l = rax; fl_r = rcx; rdx = rax.wrapping_add(0x8); rdx = (if (((fl_l as u64)) == ((fl_r as u64))) { r15 } else { rdx }) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 21; continue; } bb = 15; continue; }
            15 => { rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x2cu64))) as u32 as u64; fl_l = rbp; fl_r = rax; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 17; continue; } bb = 16; continue; }
            16 => { rcx = rsi.wrapping_add(0x20); rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 20; continue; } bb = 17; continue; }
            17 => { rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x2cu64))) as u32 as u64; fl_l = rbp; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 20; continue; } bb = 18; continue; }
            18 => { rcx = (crate::simd_ops::ru32(rsi.wrapping_add(0x28u64))) as u32 as u64; rdx = ((rcx as u32)) as u32 as u64; rdx = ((rdx).wrapping_shl(5u32)) as u64; rax = ((rcx.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rsi.wrapping_add(0x28u64), ((rax as u32)) as u32); fl_l = ((rdx).wrapping_add(crate::simd_ops::ru64(rsi.wrapping_add(0x20u64)))) as u64; fl_r = fl_l; rdx = ((fl_l as u64)) as u64; if (((fl_l as u64)) == 0) { bb = 20; continue; } bb = 19; continue; }
            19 => { crate::simd_ops::wu64(rdx, rdi); rdx = ((rdx).wrapping_add(0x8u64)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 21; continue; } bb = 20; continue; }
            20 => { rcx = rsi.wrapping_add(0x30); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp); rax = 0x34; bb = 23; continue; }
            21 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; crate::simd_ops::wu64(rdx.wrapping_add(0x10u64), rax); crate::simd_ops::wu64(rdx, r12); crate::simd_ops::wu8(rdx.wrapping_add(0x8u64), ((r14 as u8)) as u8); bb = 22; continue; }
            22 => { rcx = rsi.wrapping_add(0x30); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rsp.wrapping_add(0x21); rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); fl_l = r12; fl_r = r12; rcx = rdi; r15 = (if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { (r14 as u32) } else { (r15 as u32) }) as u32 as u64; rdx = (((r15 as u8)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rcx = rsp.wrapping_add(0x21); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rax = 0x1; bb = 23; continue; }
            23 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415f78a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143930f88; const P1: u64 = 0x1426452b0; const P2: u64 = 0x1426452a8; const P3: u64 = 0x142645560; const P4: u64 = 0x14183acc8; const P5: u64 = 0x14158ec20; const P6: u64 = 0x1415b16f0; const P7: u64 = 0x1415f00c0; const P8: u64 = 0x1415b1830;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rbx = (crate::simd_ops::ru64(P0)) as u64; r13 = r8; r12 = (((r9 as u8)) as u32) as u32 as u64; rbp = ((rdx as u32)) as u32 as u64; r15 = rcx; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rsi); bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x58u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 12; continue; } bb = 2; continue; }
            2 => { r9 = ((rax.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; let cx_src: u32 = (r9 as u32); let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u32(rbx.wrapping_add(0x58u64), fl_l as u32, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u32)) as u32 as u64; } if (((fl_l as u32)) != ((fl_r as u32))) { bb = 1; continue; } bb = 3; continue; }
            3 => { rsi = rbx.wrapping_add(0x60); bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0xa0u64))) as u32 as u64; r14 = 0; fl_l = 0; fl_r = 0; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 14; continue; } bb = 5; continue; }
            5 => { rdx = 0; fl_l = 0; fl_r = 0; rax = ((rbp as u32)) as u32 as u64; let d = ((rcx as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x98u64))) as u64; rdi = (crate::simd_ops::ru64(rax.wrapping_add(rdx.wrapping_mul(8)))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 6; continue; }
            6 => { bb = 7; continue; }
            7 => { fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x10u64))) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 10; continue; } bb = 8; continue; }
            8 => { rdi = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 7; continue; } bb = 9; continue; }
            9 => { rdi = r14; bb = 10; continue; }
            10 => { fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 15; continue; } bb = 11; continue; }
            11 => { fl_l = (crate::simd_ops::lk_add_u32(rdi.wrapping_add(0x14u64), (1) as u32)) as u32 as u64; bb = 15; continue; }
            12 => { rsi = rbx.wrapping_add(0x60); rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x58u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 4; continue; } bb = 13; continue; }
            13 => { rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); bb = 1; continue; }
            14 => { rdi = r14; bb = 15; continue; }
            15 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x58u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 22; continue; } bb = 16; continue; }
            16 => { rcx = ((rax.wrapping_sub(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; let cx_src: u32 = (rcx as u32); let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u32(rbx.wrapping_add(0x58u64), fl_l as u32, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u32)) as u32 as u64; } if (((fl_l as u32)) == ((fl_r as u32))) { bb = 19; continue; } bb = 17; continue; }
            17 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x58u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) >= 0) { bb = 16; continue; } bb = 18; continue; }
            18 => { bb = 23; continue; }
            19 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 24; continue; } bb = 20; continue; }
            20 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x88u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 24; continue; } bb = 21; continue; }
            21 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x90u64))) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); bb = 24; continue; }
            22 => { rsi = rbx.wrapping_add(0x60); bb = 23; continue; }
            23 => { rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); bb = 24; continue; }
            24 => { fl_l = rdi; fl_r = rdi; rcx = rdi.wrapping_sub(0x10); rcx = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { r14 } else { rcx }) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 26; continue; } bb = 25; continue; }
            25 => { rax = ((rcx.wrapping_add(0xfu64)) as u32) as u32 as u64; bb = 49; continue; }
            26 => { rcx = ((rcx).wrapping_add(0x10u64)) as u64; rax = 0xffffffff; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rcx.wrapping_add(0x14u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 28; continue; } bb = 27; continue; }
            27 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 28; continue; }
            28 => { rcx = r15.wrapping_add(0x30); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = r13; fl_r = r13; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 37; continue; } bb = 29; continue; }
            29 => { fl_l = ((r12 as u8)) as u8 as u64; fl_r = ((r12 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 37; continue; } bb = 30; continue; }
            30 => { r8 = (crate::simd_ops::ru64(r15.wrapping_add(0x10u64))) as u64; rdx = (crate::simd_ops::ru32(r15.wrapping_add(0x18u64))) as u32 as u64; rcx = r8; rax = ((rdx as u32)) as u32 as u64; rax = ((rax).wrapping_shl(5u32)) as u64; rax = ((rax).wrapping_add(r8)) as u64; fl_l = r8; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 48; continue; } bb = 31; continue; }
            31 => { fl_l = (crate::simd_ops::ru32(rcx)) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 34; continue; } bb = 32; continue; }
            32 => { rcx = ((rcx).wrapping_add(0x20u64)) as u64; fl_l = rcx; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 31; continue; } bb = 33; continue; }
            33 => { bb = 48; continue; }
            34 => { rdx = (((rdx as u32)).wrapping_sub(1)) as u32 as u64; rdx = ((rdx).wrapping_shl(5u32)) as u64; rdx = ((rdx).wrapping_add(r8)) as u64; fl_l = rcx; fl_r = rdx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 36; continue; } bb = 35; continue; }
            35 => { rdx = ((rdx).wrapping_sub(rcx)) as u64; rdx = ((((rdx as u32)) & (0xffffffe0u32))) as u32 as u64; r8 = ((rdx as u32)) as u32 as u64; rdx = rcx.wrapping_add(0x20); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 36; continue; }
            36 => { fl_l = (crate::simd_ops::lk_sub_u32(r15.wrapping_add(0x18u64), (1) as u32)) as u32 as u64; bb = 48; continue; }
            37 => { rax = (crate::simd_ops::ru64(r15.wrapping_add(0x10u64))) as u64; rsi = (crate::simd_ops::ru32(r15.wrapping_add(0x18u64))) as u32 as u64; rcx = ((rsi as u32)) as u32 as u64; rcx = ((rcx).wrapping_shl(5u32)) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 40; continue; } bb = 38; continue; }
            38 => { fl_l = (crate::simd_ops::ru32(rax)) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 40; continue; } bb = 39; continue; }
            39 => { rax = ((rax).wrapping_add(0x20u64)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 38; continue; } bb = 40; continue; }
            40 => { fl_l = rax; fl_r = rcx; rdx = rax.wrapping_add(0x8); rdx = (if (((fl_l as u64)) == ((fl_r as u64))) { r14 } else { rdx }) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 47; continue; } bb = 41; continue; }
            41 => { rax = (crate::simd_ops::ru32(r15.wrapping_add(0x1cu64))) as u32 as u64; fl_l = rsi; fl_r = rax; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 43; continue; } bb = 42; continue; }
            42 => { rcx = r15.wrapping_add(0x10); rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 46; continue; } bb = 43; continue; }
            43 => { rax = (crate::simd_ops::ru32(r15.wrapping_add(0x1cu64))) as u32 as u64; fl_l = rsi; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 46; continue; } bb = 44; continue; }
            44 => { rcx = (crate::simd_ops::ru32(r15.wrapping_add(0x18u64))) as u32 as u64; rdx = ((rcx as u32)) as u32 as u64; rdx = ((rdx).wrapping_shl(5u32)) as u64; rax = ((rcx.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(r15.wrapping_add(0x18u64), ((rax as u32)) as u32); fl_l = ((rdx).wrapping_add(crate::simd_ops::ru64(r15.wrapping_add(0x10u64)))) as u64; fl_r = fl_l; rdx = ((fl_l as u64)) as u64; if (((fl_l as u64)) == 0) { bb = 46; continue; } bb = 45; continue; }
            45 => { crate::simd_ops::wu32(rdx, ((rbp as u32)) as u32); rdx = ((rdx).wrapping_add(0x8u64)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 47; continue; } bb = 46; continue; }
            46 => { rcx = r15.wrapping_add(0x30); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); rax = 0x34; bb = 49; continue; }
            47 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; crate::simd_ops::wu64(rdx.wrapping_add(0x10u64), rax); crate::simd_ops::wu64(rdx, r13); crate::simd_ops::wu8(rdx.wrapping_add(0x8u64), ((r12 as u8)) as u8); bb = 48; continue; }
            48 => { rcx = r15.wrapping_add(0x30); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rsp.wrapping_add(0x20); rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); fl_l = r13; fl_r = r13; rcx = ((rbp as u32)) as u32 as u64; r14 = (if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { (r12 as u32) } else { (r14 as u32) }) as u32 as u64; rdx = (((r14 as u8)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rcx = rsp.wrapping_add(0x20); rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); rax = 0x1; bb = 49; continue; }
            49 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415f7b20(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143930f88; const P1: u64 = 0x1426452b0; const P2: u64 = 0x1426452a8; const P3: u64 = 0x142645560; const P4: u64 = 0x1415d08f0; const P5: u64 = 0x14183acc8; const P6: u64 = 0x1415b16f0; const P7: u64 = 0x1415f0150; const P8: u64 = 0x1415b1830;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rbx = (crate::simd_ops::ru64(P0)) as u64; r13 = r9; r12 = r8; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rsi); rbp = ((rdx as u32)) as u32 as u64; r14 = rcx; bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x58u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 12; continue; } bb = 2; continue; }
            2 => { r8 = ((rax.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; let cx_src: u32 = (r8 as u32); let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u32(rbx.wrapping_add(0x58u64), fl_l as u32, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u32)) as u32 as u64; } if (((fl_l as u32)) != ((fl_r as u32))) { bb = 1; continue; } bb = 3; continue; }
            3 => { rsi = rbx.wrapping_add(0x60); bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0xa0u64))) as u32 as u64; r15 = 0; fl_l = 0; fl_r = 0; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 14; continue; } bb = 5; continue; }
            5 => { rdx = 0; fl_l = 0; fl_r = 0; rax = ((rbp as u32)) as u32 as u64; let d = ((rcx as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x98u64))) as u64; rdi = (crate::simd_ops::ru64(rax.wrapping_add(rdx.wrapping_mul(8)))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 6; continue; }
            6 => { bb = 7; continue; }
            7 => { fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x10u64))) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 10; continue; } bb = 8; continue; }
            8 => { rdi = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 7; continue; } bb = 9; continue; }
            9 => { rdi = r15; bb = 10; continue; }
            10 => { fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 15; continue; } bb = 11; continue; }
            11 => { fl_l = (crate::simd_ops::lk_add_u32(rdi.wrapping_add(0x14u64), (1) as u32)) as u32 as u64; bb = 15; continue; }
            12 => { rsi = rbx.wrapping_add(0x60); rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x58u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 4; continue; } bb = 13; continue; }
            13 => { rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); bb = 1; continue; }
            14 => { rdi = r15; bb = 15; continue; }
            15 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x58u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 22; continue; } bb = 16; continue; }
            16 => { rcx = ((rax.wrapping_sub(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; let cx_src: u32 = (rcx as u32); let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u32(rbx.wrapping_add(0x58u64), fl_l as u32, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u32)) as u32 as u64; } if (((fl_l as u32)) == ((fl_r as u32))) { bb = 19; continue; } bb = 17; continue; }
            17 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x58u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) >= 0) { bb = 16; continue; } bb = 18; continue; }
            18 => { bb = 23; continue; }
            19 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 24; continue; } bb = 20; continue; }
            20 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x88u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 24; continue; } bb = 21; continue; }
            21 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x90u64))) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); bb = 24; continue; }
            22 => { rsi = rbx.wrapping_add(0x60); bb = 23; continue; }
            23 => { rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); bb = 24; continue; }
            24 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rcx = rdi.wrapping_sub(0x10); fl_l = rdi; fl_r = rdi; rcx = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { r15 } else { rcx }) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 26; continue; } bb = 25; continue; }
            25 => { rax = ((rcx.wrapping_add(0xfu64)) as u32) as u32 as u64; bb = 49; continue; }
            26 => { rcx = ((rcx).wrapping_add(0x10u64)) as u64; rax = 0xffffffff; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rcx.wrapping_add(0x14u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 28; continue; } bb = 27; continue; }
            27 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 28; continue; }
            28 => { rcx = r14.wrapping_add(0x30); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); rdi = (crate::simd_ops::ru32(r14.wrapping_add(0x8u64))) as u32 as u64; rax = (crate::simd_ops::ru64(r14)) as u64; rcx = ((rdi.wrapping_add(rdi.wrapping_mul(2))) as u64) as u64; rdx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = r12; fl_r = r12; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 41; continue; } bb = 29; continue; }
            29 => { fl_l = rax; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 33; continue; } bb = 30; continue; }
            30 => { bb = 31; continue; }
            31 => { fl_l = (crate::simd_ops::ru32(rax)) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 33; continue; } bb = 32; continue; }
            32 => { rax = ((rax).wrapping_add(0x18u64)) as u64; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 31; continue; } bb = 33; continue; }
            33 => { fl_l = rax; fl_r = rdx; rcx = rax.wrapping_add(0x8); rcx = (if (((fl_l as u64)) == ((fl_r as u64))) { r15 } else { rcx }) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 40; continue; } bb = 34; continue; }
            34 => { rax = (crate::simd_ops::ru32(r14.wrapping_add(0xcu64))) as u32 as u64; fl_l = rdi; fl_r = rax; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 36; continue; } bb = 35; continue; }
            35 => { rcx = r14; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 39; continue; } bb = 36; continue; }
            36 => { rax = (crate::simd_ops::ru32(r14.wrapping_add(0xcu64))) as u32 as u64; fl_l = rdi; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 39; continue; } bb = 37; continue; }
            37 => { rcx = (crate::simd_ops::ru32(r14.wrapping_add(0x8u64))) as u32 as u64; rax = ((rcx.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(r14.wrapping_add(0x8u64), ((rax as u32)) as u32); rcx = ((rcx.wrapping_add(rcx.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru64(r14)) as u64; rcx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 39; continue; } bb = 38; continue; }
            38 => { crate::simd_ops::wu32(rcx, ((rbp as u32)) as u32); rcx = ((rcx).wrapping_add(0x8u64)) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 40; continue; } bb = 39; continue; }
            39 => { rcx = r14.wrapping_add(0x30); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); rax = 0x34; bb = 49; continue; }
            40 => { crate::simd_ops::wu64(rcx, r12); crate::simd_ops::wu64(rcx.wrapping_add(0x8u64), r13); bb = 48; continue; }
            41 => { rcx = rax; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 48; continue; } bb = 42; continue; }
            42 => { fl_l = (crate::simd_ops::ru32(rcx)) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 45; continue; } bb = 43; continue; }
            43 => { rcx = ((rcx).wrapping_add(0x18u64)) as u64; fl_l = rcx; fl_r = rdx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 42; continue; } bb = 44; continue; }
            44 => { bb = 48; continue; }
            45 => { rdx = ((rdi.wrapping_sub(0x1u64)) as u32) as u32 as u64; rdx = ((rdx.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64; r8 = ((rax.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64; fl_l = rcx; fl_r = r8; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 47; continue; } bb = 46; continue; }
            46 => { r8 = ((r8).wrapping_sub(rcx)) as u64; rax = (0x2aaaaaaaaaaaaaabu64) as u64; let prod = (rax as i64 as i128).wrapping_mul((r8) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rdx = ((((rdx) as i64).wrapping_shr(2u32) as u64)) as u64; rax = rdx; rax = ((rax).wrapping_shr(63u32)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; r8 = ((rdx.wrapping_add(rdx.wrapping_mul(2))) as u32) as u32 as u64; r8 = (((r8 as u32)).wrapping_shl(3u32)) as u32 as u64; rdx = rcx.wrapping_add(0x18); rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); bb = 47; continue; }
            47 => { fl_l = (crate::simd_ops::lk_sub_u32(r14.wrapping_add(0x8u64), (1) as u32)) as u32 as u64; bb = 48; continue; }
            48 => { rcx = r14.wrapping_add(0x30); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rsp.wrapping_add(0x20); rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); fl_l = r12; fl_r = r12; rcx = ((rbp as u32)) as u32 as u64; rdx = (rdx & !0xffu64) | (((if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { 1u8 } else { 0u8 }) as u8 as u64)); rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rcx = rsp.wrapping_add(0x20); rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); rax = 0x1; bb = 49; continue; }
            49 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415f7d90(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x8; const P2: u64 = 0x1415242e0; const P3: u64 = 0xc;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = (crate::simd_ops::ru64(rcx)) as u64; rsi = rcx; rdi = (crate::simd_ops::ru64(rdx)) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 1; continue; }
            1 => { rax = ((crate::simd_ops::ru16(rbx.wrapping_add(P0))) as u32) as u32 as u64; fl_l = ((rax as u16)) as u16 as u64; fl_r = (0x1u16) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 5; continue; } bb = 2; continue; }
            2 => { rdx = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rbx.wrapping_add(P1), (0x0u32) as u32); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx, (0x0u64) as u64); crate::simd_ops::wu32(rbx.wrapping_add(P3), (0x0u32) as u32); bb = 4; continue; }
            4 => { rdx = rbx; rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            5 => { fl_l = (((rax as u16)).wrapping_sub(1)) as u16 as u64; rax = (rax & !0xffffu64) | ((((fl_l as u16)) as u16 as u64)); crate::simd_ops::wu16(rbx.wrapping_add(P0), ((rax as u16)) as u16); bb = 6; continue; }
            6 => { crate::simd_ops::wu64(rsi, rdi); rax = rsi; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { fl_l = (crate::simd_ops::lk_add_u16(rdi.wrapping_add(P0), (1) as u16)) as u16 as u64; bb = 8; continue; }
            8 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415f7e10(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x8; const P2: u64 = 0x141524330; const P3: u64 = 0x141606940; const P4: u64 = 0xc; const P5: u64 = 0x1; const P6: u64 = 0x141606990; const P7: u64 = 0x1415242e0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; rdi = rcx; fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 8; continue; } bb = 1; continue; }
            1 => { rdx = 0x98; rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 2; continue; }
            2 => { rcx = rax; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rbx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(P1))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rdi.wrapping_add(P4))) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 7; continue; } bb = 4; continue; }
            4 => { rcx = ((rax.wrapping_add(P5)) as u32) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(P1), ((rcx as u32)) as u32); rcx = (crate::simd_ops::ru64(rdi)) as u64; rax = ((rcx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 5; continue; }
            5 => { crate::simd_ops::wu64(rax, rbx); rax = rbx; bb = 6; continue; }
            6 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            7 => { rcx = rbx; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rdx = rbx; rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = 0; fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            8 => { rax = 0; fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415f7ea0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14153e940; const P1: u64 = 0x143930f08; const P2: u64 = 0x1416077b0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rbp = r8; rcx = (crate::simd_ops::ru32(rcx.wrapping_add(0x8u64))) as u32 as u64; rbx = rdx; rcx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 4; continue; } bb = 1; continue; }
            1 => { bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru64(rax)) as u64; fl_r = rbx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = ((rax).wrapping_add(0x8u64)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 4; continue; }
            4 => { r8 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rsi); fl_l = rax; fl_r = rcx; rdx = ((r8 as u32)) as u32 as u64; rdx = (if (((fl_l as u64)) != ((fl_r as u64))) { rax } else { rdx }) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = ((rdx.wrapping_add(0x2u64)) as u32) as u32 as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            6 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x30u64))) as u64; rsi = 0x28; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), r14); r14 = (crate::simd_ops::ru32(rbx.wrapping_add(0x38u64))) as u32 as u64; rcx = ((rax.wrapping_add(r14.wrapping_mul(8))) as u64) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 10; continue; } bb = 7; continue; }
            7 => { fl_l = (crate::simd_ops::ru64(rax)) as u64; fl_r = rbp; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 9; continue; } bb = 8; continue; }
            8 => { rax = ((rax).wrapping_add(0x8u64)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 7; continue; } bb = 9; continue; }
            9 => { fl_l = rax; fl_r = rcx; bb = 10; continue; }
            10 => { r8 = (if (((fl_l as u64)) != ((fl_r as u64))) { rax } else { r8 }) as u64; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 19; continue; } bb = 11; continue; }
            11 => { rax = 0xffff; fl_l = (crate::simd_ops::ru16(rbx.wrapping_add(0x42u64))) as u16 as u64; fl_r = ((rax as u16)) as u16 as u64; if (((fl_l as u16)) == ((fl_r as u16))) { bb = 18; continue; } bb = 12; continue; }
            12 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x3cu64))) as u32 as u64; fl_l = r14; fl_r = rax; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 14; continue; } bb = 13; continue; }
            13 => { rcx = rbx.wrapping_add(0x30); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 18; continue; } bb = 14; continue; }
            14 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x3cu64))) as u32 as u64; fl_l = r14; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 18; continue; } bb = 15; continue; }
            15 => { rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x38u64))) as u32 as u64; rax = ((rcx.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x38u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x30u64))) as u64; rdx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 18; continue; } bb = 16; continue; }
            16 => { crate::simd_ops::wu64(rdx, rbp); rsi = 0x1; crate::simd_ops::lk_add_u16(rbx.wrapping_add(0x42u64), (1) as u16); fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0x2cu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 19; continue; } bb = 17; continue; }
            17 => { rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x1b8u64))) as u64; xmm0 = ((crate::simd_ops::ru32(rcx.wrapping_add(0x4u64))) as u32) as u128; xmm1 = ((crate::simd_ops::ru32(rcx.wrapping_add(0x8u64))) as u32) as u128; xmm2 = ((crate::simd_ops::ru32(rcx.wrapping_add(0xcu64))) as u32) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(crate::simd_ops::ru32(rbx.wrapping_add(0x70u64)))).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) + f32::from_bits(crate::simd_ops::ru32(rbx.wrapping_add(0x74u64)))).to_bits() as u128); xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) + f32::from_bits(crate::simd_ops::ru32(rbx.wrapping_add(0x78u64)))).to_bits() as u128); xmm0 = crate::simd_ops::vunpcklps(xmm0, xmm1); crate::simd_ops::wu64(rcx.wrapping_add(0x4u64), ((xmm0 as u64)) as u64); crate::simd_ops::wu32(rcx.wrapping_add(0xcu64), ((xmm2 as u32)) as u32); rcx = rbx; rax = (crate::simd_ops::ru64(P1)) as u64; rdx = (crate::simd_ops::ru32(rax.wrapping_add(0x88u64))) as u32 as u64; rdx = (((rdx as u32)).wrapping_add(1)) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 19; continue; }
            18 => { rsi = 0x2; bb = 19; continue; }
            19 => { r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rax = ((rsi as u32)) as u32 as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415f8000(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x40;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = (crate::simd_ops::ru64(rcx)) as u64; r8 = rdx; rcx = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; rcx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 3; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru64(rax)) as u64; fl_r = r8; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rax = ((rax).wrapping_add(0x8u64)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 3; continue; }
            3 => { rdx = 0; fl_l = 0; fl_r = 0; fl_l = rax; fl_r = rcx; rdx = (if (((fl_l as u64)) != ((fl_r as u64))) { rax } else { rdx }) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 4; continue; }
            4 => { rax = ((crate::simd_ops::ru16(r8.wrapping_add(P1))) as u32) as u32 as u64; rcx = 0xffff; fl_l = ((rax as u16)) as u16 as u64; fl_r = ((rcx as u16)) as u16 as u64; if (((fl_l as u16)) == ((fl_r as u16))) { bb = 6; continue; } bb = 5; continue; }
            5 => { fl_l = (((rax as u16)).wrapping_add(1)) as u16 as u64; rax = (rax & !0xffffu64) | ((((fl_l as u16)) as u16 as u64)); crate::simd_ops::wu16(r8.wrapping_add(P1), ((rax as u16)) as u16); rax = 0x1; return rax; }
            6 => { rax = 0x2; return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415f8060(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0xc; const P2: u64 = 0x14159a6d0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64;
    rax = 0xff;
    rax = (if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { (rdx as u32) } else { (rax as u32) }) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(P0), ((rax as u32)) as u32);
    rdx = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64;
    fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64;
    if !((((fl_l as u32)) > ((fl_r as u32)))) { rax = 0x1; rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
    fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64;
    if !(((((fl_l as u32)) & ((fl_r as u32))) == 0)) { rax = 0x1f; rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
    rax = (((rax as u32)).wrapping_sub((rdx as u32))) as u32 as u64;
    rdx = ((rax as u32)) as u32 as u64;
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64;
    rcx = 0x34;
    rdx = 0x1;
    rcx = (if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { (rdx as u32) } else { (rcx as u32) }) as u32 as u64;
    rax = ((rcx as u32)) as u32 as u64;
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415f80b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143930f08; const P1: u64 = 0x88; const P2: u64 = 0x141607110;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    fl_l = (crate::simd_ops::ru32(rdx)) as u32 as u64; fl_r = 0x1;
    r8 = rdx;
    if !((((fl_l as u32)) != ((fl_r as u32)))) { rax = (crate::simd_ops::ru64(P0)) as u64; rcx = r8; rdx = (crate::simd_ops::ru32(rax.wrapping_add(P1))) as u32 as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rax = 0x1; rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
    rax = 0x26;
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415f80f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x1416077b0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = (crate::simd_ops::ru64(rcx)) as u64; rsi = ((rdx as u32)) as u32 as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; rdi = rcx; r8 = ((rbx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = r8; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 4; continue; } bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; fl_l = (crate::simd_ops::ru32(rcx)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rdx = ((rsi as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru32(rdi.wrapping_add(P0))) as u32 as u64; rbx = ((rbx).wrapping_add(0x8u64)) as u64; rax = (crate::simd_ops::ru64(rdi)) as u64; rcx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 4; continue; }
            4 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415f8150(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xfffffffffffffff8; const P2: u64 = 0x141607700; const P3: u64 = 0x141606990; const P4: u64 = 0x1415242e0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x28u64)) as u64; r8 = (crate::simd_ops::ru64(rcx)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; r9 = ((r8.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = r8; fl_r = r9; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 3; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru64(r8)) as u64; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 4; continue; } bb = 2; continue; }
            2 => { r8 = ((r8).wrapping_add(0x8u64)) as u64; fl_l = r8; fl_r = r9; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 3; continue; }
            3 => { rax = 0x24; rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
            4 => { rax = ((r9.wrapping_add(P1)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbx); rbx = (crate::simd_ops::ru64(r8)) as u64; fl_l = r8; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rax)) as u64; crate::simd_ops::wu64(r8, rax); bb = 6; continue; }
            6 => { crate::simd_ops::lk_sub_u32(rcx.wrapping_add(P0), (1) as u32); rcx = rbx; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { rcx = rbx; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rdx = rbx; rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 8; continue; }
            8 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rax = 0x1; rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415f81d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x1; const P2: u64 = 0x30; const P3: u64 = 0x38; const P4: u64 = 0x42; const P5: u64 = 0x40; const P6: u64 = 0x1415f8150;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x28u64)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; r10 = r8; r9 = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; r11 = rcx; r9 = ((rax.wrapping_add(r9.wrapping_mul(8))) as u64) as u64; fl_l = rax; fl_r = r9; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 4; continue; } bb = 1; continue; }
            1 => { bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru64(rax)) as u64; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = ((rax).wrapping_add(0x8u64)) as u64; fl_l = rax; fl_r = r9; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 4; continue; }
            4 => { rcx = 0; fl_l = 0; fl_r = 0; fl_l = rax; fl_r = r9; rcx = (if (((fl_l as u64)) != ((fl_r as u64))) { rax } else { rcx }) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = ((rcx.wrapping_add(P1)) as u32) as u32 as u64; rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
            6 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(P2))) as u64; r8 = 0x29; rcx = (crate::simd_ops::ru32(rdx.wrapping_add(P3))) as u32 as u64; rcx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 17; continue; } bb = 7; continue; }
            7 => { bb = 8; continue; }
            8 => { fl_l = (crate::simd_ops::ru64(rax)) as u64; fl_r = r10; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 11; continue; } bb = 9; continue; }
            9 => { rax = ((rax).wrapping_add(0x8u64)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 8; continue; } bb = 10; continue; }
            10 => { rax = ((r8 as u32)) as u32 as u64; rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
            11 => { rcx = ((rcx).wrapping_add(0xfffffffffffffff8u64)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 13; continue; } bb = 12; continue; }
            12 => { rcx = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::wu64(rax, rcx); bb = 13; continue; }
            13 => { crate::simd_ops::lk_sub_u32(rdx.wrapping_add(P3), (1) as u32); rax = ((crate::simd_ops::ru16(rdx.wrapping_add(P4))) as u32) as u32 as u64; rax = (rax & !0xffffu64) | (((((rax as u16)).wrapping_sub(1)) as u16 as u64)); fl_l = (crate::simd_ops::ru16(rdx.wrapping_add(P5))) as u16 as u64; fl_r = (0x0u16) as u16 as u64; crate::simd_ops::wu16(rdx.wrapping_add(P4), ((rax as u16)) as u16); if (((fl_l as u16)) != ((fl_r as u16))) { bb = 16; continue; } bb = 14; continue; }
            14 => { fl_l = ((rax as u16)) as u16 as u64; fl_r = ((rax as u16)) as u16 as u64; if ((((fl_l as u16)) & ((fl_r as u16))) != 0) { bb = 16; continue; } bb = 15; continue; }
            15 => { rcx = r11; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); r8 = ((rax as u32)) as u32 as u64; rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
            16 => { r8 = 0x1; bb = 17; continue; }
            17 => { rax = ((r8 as u32)) as u32 as u64; rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415f8290(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141607700; const P1: u64 = 0x141606990; const P2: u64 = 0x1415242e0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x28u64)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; r9 = rcx; r8 = (crate::simd_ops::ru32(rcx.wrapping_add(0x8u64))) as u32 as u64; r8 = ((rax.wrapping_add(r8.wrapping_mul(8))) as u64) as u64; fl_l = rax; fl_r = r8; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 3; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru64(rax)) as u64; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rax = ((rax).wrapping_add(0x8u64)) as u64; fl_l = rax; fl_r = r8; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 3; continue; }
            3 => { rcx = 0; fl_l = 0; fl_r = 0; fl_l = rax; fl_r = r8; rcx = (if (((fl_l as u64)) != ((fl_r as u64))) { rax } else { rcx }) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 18; continue; } bb = 4; continue; }
            4 => { rax = ((crate::simd_ops::ru16(rdx.wrapping_add(0x40u64))) as u32) as u32 as u64; fl_l = ((rax as u16)) as u16 as u64; fl_r = ((rax as u16)) as u16 as u64; if ((((fl_l as u16)) & ((fl_r as u16))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = (rax & !0xffffu64) | (((((rax as u16)).wrapping_sub(1)) as u16 as u64)); crate::simd_ops::wu16(rdx.wrapping_add(0x40u64), ((rax as u16)) as u16); fl_l = ((rax as u16)) as u16 as u64; fl_r = ((rax as u16)) as u16 as u64; if ((((fl_l as u16)) & ((fl_r as u16))) != 0) { bb = 18; continue; } bb = 6; continue; }
            6 => { fl_l = (crate::simd_ops::ru16(rdx.wrapping_add(0x42u64))) as u16 as u64; fl_r = (0x0u16) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 18; continue; } bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru64(r9)) as u64; r8 = 0x24; rcx = (crate::simd_ops::ru32(r9.wrapping_add(0x8u64))) as u32 as u64; rcx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 17; continue; } bb = 8; continue; }
            8 => { bb = 9; continue; }
            9 => { fl_l = (crate::simd_ops::ru64(rax)) as u64; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 12; continue; } bb = 10; continue; }
            10 => { rax = ((rax).wrapping_add(0x8u64)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 9; continue; } bb = 11; continue; }
            11 => { rax = ((r8 as u32)) as u32 as u64; rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
            12 => { rcx = ((rcx).wrapping_add(0xfffffffffffffff8u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbx); rbx = (crate::simd_ops::ru64(rax)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 14; continue; } bb = 13; continue; }
            13 => { rcx = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::wu64(rax, rcx); bb = 14; continue; }
            14 => { crate::simd_ops::lk_sub_u32(r9.wrapping_add(0x8u64), (1) as u32); rcx = rbx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 16; continue; } bb = 15; continue; }
            15 => { rcx = rbx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rdx = rbx; rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 16; continue; }
            16 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; r8 = 0x1; bb = 17; continue; }
            17 => { rax = ((r8 as u32)) as u32 as u64; rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
            18 => { rax = 0x1; rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415f8370(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143930f08; const P1: u64 = 0x88; const P2: u64 = 0x1416072c0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    fl_l = (crate::simd_ops::ru32(rdx)) as u32 as u64; fl_r = 0x2;
    r8 = rdx;
    if !((((fl_l as u32)) != ((fl_r as u32)))) { rax = (crate::simd_ops::ru64(P0)) as u64; rcx = r8; rdx = (crate::simd_ops::ru32(rax.wrapping_add(P1))) as u32 as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rax = 0x1; rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
    rax = 0x27;
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415f83b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1416072e0;
    let (mut rax, mut r10, mut r11, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x30u64)) as u64;
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64;
    r10 = ((r9 as u32)) as u32 as u64;
    r9 = (crate::simd_ops::ru32(rsp.wrapping_add(0x60u64))) as u32 as u64;
    r11 = r8;
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax);
    rcx = rdx;
    rax = ((crate::simd_ops::ru8(rsp.wrapping_add(0x68u64))) as u32) as u32 as u64;
    r8 = ((r10 as u32)) as u32 as u64;
    rdx = r11;
    crate::simd_ops::wu8(rsp.wrapping_add(0x20u64), ((rax as u8)) as u8);
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x30u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415f83f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x2c; const P1: u64 = 0x8; const P2: u64 = 0x10; const P3: u64 = 0x1415f7d90; const P4: u64 = 0x20; const P5: u64 = 0x18; const P6: u64 = 0x143930f08; const P7: u64 = 0x88; const P8: u64 = 0x1416074f0; const P9: u64 = 0x28; const P10: u64 = 0x1416069f0; const P11: u64 = 0x1416078b0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; fl_l = (crate::simd_ops::ru32(rdx)) as u32 as u64; fl_r = 0x0; rsi = r8; rbx = rdx; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 8; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(P0))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 7; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru64(r8.wrapping_add(P1))) as u64; fl_r = (0x0u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), r14); if (((fl_l as u64)) == ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rdx = ((r8.wrapping_add(P1)) as u64) as u64; rcx = ((rbx.wrapping_add(P2)) as u64) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rax = ((crate::simd_ops::ru16(rsi)) as u32) as u32 as u64; crate::simd_ops::wu16(rbx.wrapping_add(P4), ((rax as u16)) as u16); rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64; rcx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; crate::simd_ops::wu64(rbx.wrapping_add(P5), rcx); bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(P6)) as u64; rcx = rbx; rdx = (crate::simd_ops::ru32(rax.wrapping_add(P7))) as u32 as u64; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(P9))) as u8 as u64; fl_r = (0x2u8) as u8 as u64; rbp = ((rax as u32)) as u32 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rcx = rbx; rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp); rax = ((crate::simd_ops::ru16(rbx.wrapping_add(P4))) as u32) as u32 as u64; rdx = ((rbx.wrapping_add(P2)) as u64) as u64; rcx = ((rsi.wrapping_add(P1)) as u64) as u64; crate::simd_ops::wu32(rsi, ((rax as u32)) as u32); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rax = ((rbp as u32)) as u32 as u64; bb = 6; continue; }
            6 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            7 => { rcx = rbx; rax = crate::simd_ops::call_stk(P11, rax, rcx, rdx, r8, r9, rsp); bb = 8; continue; }
            8 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rax = 0x2; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415f84c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x141607700; const P2: u64 = 0x141606990; const P3: u64 = 0x1415242e0; const P4: u64 = 0xc;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = (crate::simd_ops::ru64(rcx)) as u64; rsi = rcx; rax = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; rdx = ((rbx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 5; continue; } bb = 1; continue; }
            1 => { rdi = (crate::simd_ops::ru64(rbx)) as u64; rcx = rdi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rcx = rdi; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rdx = rdi; rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 3; continue; }
            3 => { rdx = (crate::simd_ops::ru64(rsi)) as u64; rbx = ((rbx).wrapping_add(0x8u64)) as u64; rax = (crate::simd_ops::ru32(rsi.wrapping_add(P0))) as u32 as u64; rcx = ((rdx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 4; continue; }
            4 => { rbx = rdx; bb = 5; continue; }
            5 => { fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { rdi = 0; fl_l = 0; fl_r = 0; rdx = rbx; rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rsi.wrapping_add(P0), ((rdi as u32)) as u32); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rsi, rdi); crate::simd_ops::wu32(rsi.wrapping_add(P4), ((rdi as u32)) as u32); bb = 7; continue; }
            7 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415f8550(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedStack::<1>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rbp = rsp.wrapping_add(0x20); rdx = (crate::simd_ops::ru32(rcx.wrapping_add(0x50u64))) as u32 as u64; r14 = rcx; rax = ((rdx as u32)) as u32 as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x70u64), rbx); rax = ((rax).wrapping_shl(2u32)) as u64; rcx = rax.wrapping_add(0xf); fl_l = rcx; fl_r = rax; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = (0xffffffffffffff0u64) as u64; bb = 2; continue; }
            2 => { rcx = (((rcx) & (0xfffffffffffffff0u64))) as u64; rax = rcx; rax = ((rdx.wrapping_add(rdx.wrapping_mul(2))) as u32) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x60u64), ((rdx as u32)) as u32); rsp = ((rsp).wrapping_sub(rcx)) as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x68u64), (0x20657563u32) as u32); rcx = (crate::simd_ops::ru64(r14.wrapping_add(0x60u64))) as u64; rax = ((rax.wrapping_mul(8).wrapping_add(0x4u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x6cu64), ((rax as u32)) as u32); rdx = rbp.wrapping_add(0x68); r8 = 0x8; r15 = rsp.wrapping_add(0x20); rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(r14.wrapping_add(0x60u64))) as u64; rdx = rbp.wrapping_add(0x60); r8 = 0x4; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(r14.wrapping_add(0x50u64))) as u32 as u64; rbx = 0; fl_l = 0; fl_r = 0; r13 = ((rbx as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 5; continue; } bb = 3; continue; }
            3 => { r13 = ((rax.wrapping_add(rax.wrapping_mul(2))) as u32) as u32 as u64; rdi = ((rbx as u32)) as u32 as u64; fl_l = (((r13 as u32)).wrapping_shl(3u32)) as u32 as u64; r13 = ((fl_l as u32)) as u32 as u64; rsi = ((rax as u32)) as u32 as u64; bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru64(r14.wrapping_add(0x48u64))) as u64; rdx = rbp; r8 = 0x18; rax = (crate::simd_ops::ru32(rcx.wrapping_add(rdi))) as u32 as u64; crate::simd_ops::wu32(rbp, ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rcx.wrapping_add(rdi).wrapping_add(0x4u64))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x4u64), ((rax as u32)) as u32); crate::simd_ops::wu64(rbp.wrapping_add(0x8u64), (0x61746164u64) as u64); crate::simd_ops::wu32(rbp.wrapping_add(0x10u64), ((rbx as u32)) as u32); rax = (crate::simd_ops::ru32(rcx.wrapping_add(rdi).wrapping_add(0x4u64))) as u32 as u64; rcx = (crate::simd_ops::ru64(r14.wrapping_add(0x60u64))) as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x14u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rdi = rdi.wrapping_add(0x10); fl_l = rsi; fl_r = (0x1u64) as u64; rsi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru32(r14.wrapping_add(0x50u64))) as u32 as u64; rdx = ((rbx as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 10; continue; } bb = 6; continue; }
            6 => { r9 = rbx; r10 = r15; r11 = ((rax as u32)) as u32 as u64; bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru64(r14.wrapping_add(0x48u64))) as u64; rcx = (crate::simd_ops::ru64(r9.wrapping_add(rax).wrapping_add(0x8u64))) as u64; rax = (0xffffffffffffffffu64) as u64; bb = 8; continue; }
            8 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(rax).wrapping_add(0x1u64))) as u8 as u64; fl_r = ((rbx as u8)) as u8 as u64; rax = rax.wrapping_add(0x1); if (((fl_l as u8)) != ((fl_r as u8))) { bb = 8; continue; } bb = 9; continue; }
            9 => { rcx = ((rax.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; crate::simd_ops::wu32(r10, ((rcx as u32)) as u32); rax = ((rdx.wrapping_add(0x1u64)) as u32) as u32 as u64; rax = (if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { (rdx as u32) } else { (rax as u32) }) as u32 as u64; r9 = ((r9).wrapping_add(0x10u64)) as u64; r10 = ((r10).wrapping_add(0x4u64)) as u64; rdx = ((rcx.wrapping_add(rax)) as u32) as u32 as u64; fl_l = r11; fl_r = (0x1u64) as u64; r11 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 7; continue; } bb = 10; continue; }
            10 => { rax = (crate::simd_ops::ru32(r14.wrapping_add(0x50u64))) as u32 as u64; r8 = 0x8; crate::simd_ops::wu32(rbp.wrapping_add(0x68u64), (0x5453494cu32) as u32); rcx = ((rax.wrapping_mul(2).wrapping_add(0x1u64)) as u32) as u32 as u64; rcx = (((rcx as u32)).wrapping_add((rax as u32))) as u32 as u64; rax = ((rdx.wrapping_add(rcx.wrapping_mul(4))) as u32) as u32 as u64; rcx = (crate::simd_ops::ru64(r14.wrapping_add(0x60u64))) as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x6cu64), ((rax as u32)) as u32); rdx = rbp.wrapping_add(0x68); rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(r14.wrapping_add(0x60u64))) as u64; rdx = rbp.wrapping_add(0x60); crate::simd_ops::wu32(rbp.wrapping_add(0x60u64), (0x6c746461u32) as u32); r8 = 0x4; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(r14.wrapping_add(0x50u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 16; continue; } bb = 11; continue; }
            11 => { rdi = rbx; rsi = ((rax as u32)) as u32 as u64; bb = 12; continue; }
            12 => { rcx = (crate::simd_ops::ru64(r14.wrapping_add(0x48u64))) as u64; rdx = rbp; rax = (crate::simd_ops::ru32(r15)) as u32 as u64; r8 = 0xc; rax = (((rax as u32)).wrapping_add(0x4u32)) as u32 as u64; crate::simd_ops::wu32(rbp, (0x6c62616cu32) as u32); crate::simd_ops::wu32(rbp.wrapping_add(0x4u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rcx.wrapping_add(rdi))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x8u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rcx.wrapping_add(rdi).wrapping_add(0x8u64))) as u64; rcx = (crate::simd_ops::ru64(r14.wrapping_add(0x60u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0xcu64), rax); rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(r14.wrapping_add(0x60u64))) as u64; r8 = (crate::simd_ops::ru32(r15)) as u32 as u64; rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0xcu64))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(r15)) as u32 as u64; rbx = (((rbx as u32)).wrapping_add(0xcu32)) as u32 as u64; rbx = (((rbx as u32)).wrapping_add((rax as u32))) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 14; continue; } bb = 13; continue; }
            13 => { rcx = (crate::simd_ops::ru64(r14.wrapping_add(0x60u64))) as u64; rdx = rbp.wrapping_add(0x60); crate::simd_ops::wu8(rbp.wrapping_add(0x60u64), (0x0u8) as u8); r8 = 0x1; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = (((rbx as u32)).wrapping_add(1)) as u32 as u64; rbx = ((fl_l as u32)) as u32 as u64; bb = 14; continue; }
            14 => { rdi = ((rdi).wrapping_add(0x10u64)) as u64; r15 = ((r15).wrapping_add(0x4u64)) as u64; fl_l = rsi; fl_r = (0x1u64) as u64; rsi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 12; continue; } bb = 15; continue; }
            15 => { rax = ((r13.wrapping_add(0x18u64)) as u32) as u32 as u64; fl_l = (((rax as u32)).wrapping_add((rbx as u32))) as u32 as u64; fl_r = fl_l; rax = ((fl_l as u32)) as u32 as u64; bb = 17; continue; }
            16 => { rax = ((rbx.wrapping_add(0x18u64)) as u32) as u32 as u64; fl_l = (((rax as u32)).wrapping_add((r13 as u32))) as u32 as u64; fl_r = fl_l; rax = ((fl_l as u32)) as u32 as u64; bb = 17; continue; }
            17 => { rbx = (crate::simd_ops::ru64(rbp.wrapping_add(0x70u64))) as u64; rsp = rbp.wrapping_add(0x20); r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415f87a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141524330; const P1: u64 = 0x1426469f0; const P2: u64 = 0x1415242e0; const P3: u64 = 0x142646930; const P4: u64 = 0x14183acbc; const P5: u64 = 0x141540670;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(0x60u64))) as u64; fl_r = (0x0u64) as u64; rsi = rdx; r8 = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = 0x2; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            2 => { rcx = ((crate::simd_ops::ru16(rcx.wrapping_add(0x20u64))) as u32) as u32 as u64; rdx = 0; fl_l = 0; fl_r = 0; rax = (crate::simd_ops::ru32(r8.wrapping_add(0x5cu64))) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rbp); let d = ((rcx as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } fl_l = (crate::simd_ops::ru32(r8.wrapping_add(0x50u64))) as u32 as u64; fl_r = 0x0; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rdi); rbp = ((rax as u32)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), r14); rdi = r8.wrapping_add(0x48); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r15); if (((fl_l as u32)) == ((fl_r as u32))) { bb = 23; continue; } bb = 3; continue; }
            3 => { r15 = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; r15 = ((r15).wrapping_shl(4u32)) as u64; r15 = ((r15).wrapping_add(crate::simd_ops::ru64(rdi))) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(r15.wrapping_sub(0xcu64))) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 23; continue; } bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru64(r15.wrapping_sub(0x8u64))) as u64; rbx = (0xffffffffffffffffu64) as u64; rax = rbx; bb = 5; continue; }
            5 => { rax = ((rax).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(rax))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 5; continue; } bb = 6; continue; }
            6 => { rcx = rbx; bb = 7; continue; }
            7 => { rcx = ((rcx).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(rsi.wrapping_add(rcx))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 7; continue; } bb = 8; continue; }
            8 => { rax = ((rax).wrapping_add(1)) as u64; rax = ((rax).wrapping_add(rcx)) as u64; rcx = 0; fl_l = 0; fl_r = 0; rdx = ((rax as u32)) as u32 as u64; r14 = ((rax as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rbp = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 32; continue; } bb = 9; continue; }
            9 => { r8 = (crate::simd_ops::ru64(r15.wrapping_sub(0x8u64))) as u64; rdi = r14.wrapping_sub(0x1); rcx = rbx; bb = 10; continue; }
            10 => { rcx = ((rcx).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(r8.wrapping_add(rcx))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 10; continue; } bb = 11; continue; }
            11 => { rcx = ((rcx).wrapping_add(1)) as u64; fl_l = rdi; fl_r = rcx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 15; continue; } bb = 12; continue; }
            12 => { rdi = rbx; bb = 13; continue; }
            13 => { rdi = ((rdi).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(r8.wrapping_add(rdi))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 13; continue; } bb = 14; continue; }
            14 => { fl_l = ((rdi).wrapping_add(1)) as u64; rdi = ((fl_l as u64)) as u64; bb = 15; continue; }
            15 => { r9 = rdi; rdx = r14; rcx = rbp; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu8(rdi.wrapping_add(rbp), (0x0u8) as u8); rcx = 0; fl_l = 0; fl_r = 0; rdx = (crate::simd_ops::ru64(r15.wrapping_sub(0x8u64))) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rax = rbx; bb = 16; continue; }
            16 => { rax = ((rax).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(rbp))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 16; continue; } bb = 17; continue; }
            17 => { rcx = ((r14 as u32)) as u32 as u64; rcx = (((rcx as u32)).wrapping_sub((rax as u32))) as u32 as u64; rax = rbx; fl_l = (((rcx as u32)).wrapping_sub(1)) as u32 as u64; rcx = ((fl_l as u32)) as u32 as u64; bb = 18; continue; }
            18 => { rax = ((rax).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(rsi.wrapping_add(rax))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 18; continue; } bb = 19; continue; }
            19 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { bb = 21; continue; } bb = 20; continue; }
            20 => { rbx = ((rcx as u32)) as u32 as u64; bb = 22; continue; }
            21 => { rbx = ((rbx).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(rsi.wrapping_add(rbx))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 21; continue; } bb = 22; continue; }
            22 => { r9 = ((((rbx as u32)) as i32 as i64) as u64) as u64; r8 = rsi; rdx = r14; rcx = rbp; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); rax = 0x1; crate::simd_ops::wu64(r15.wrapping_sub(0x8u64), rbp); bb = 33; continue; }
            23 => { r15 = (crate::simd_ops::ru32(r8.wrapping_add(0x58u64))) as u32 as u64; rbx = (0xffffffffffffffffu64) as u64; fl_l = (((r15 as u32)).wrapping_add(1)) as u32 as u64; r15 = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(0x58u64), ((r15 as u32)) as u32); bb = 24; continue; }
            24 => { rbx = ((rbx).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(rsi.wrapping_add(rbx))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 24; continue; } bb = 25; continue; }
            25 => { rbx = (((rbx as u32)).wrapping_add(1)) as u32 as u64; rcx = 0; fl_l = 0; fl_r = 0; rdx = ((rbx as u32)) as u32 as u64; r14 = ((rbx as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rbx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 32; continue; } bb = 26; continue; }
            26 => { r8 = ((r14 as u32)) as u32 as u64; rdx = rsi; rcx = rax; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rsi = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; fl_l = ((rsi as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rdi.wrapping_add(0xcu64))) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 29; continue; } bb = 27; continue; }
            27 => { rcx = rdi; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 31; continue; } bb = 28; continue; }
            28 => { fl_l = ((rsi as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rdi.wrapping_add(0xcu64))) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 31; continue; } bb = 29; continue; }
            29 => { rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; rax = ((rcx.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x8u64), ((rax as u32)) as u32); rax = ((rcx as u32)) as u32 as u64; rax = ((rax).wrapping_shl(4u32)) as u64; fl_l = ((rax).wrapping_add(crate::simd_ops::ru64(rdi))) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; if (((fl_l as u64)) == 0) { bb = 31; continue; } bb = 30; continue; }
            30 => { crate::simd_ops::wu32(rax, ((r15 as u32)) as u32); crate::simd_ops::wu32(rax.wrapping_add(0x4u64), ((rbp as u32)) as u32); crate::simd_ops::wu64(rax.wrapping_add(0x8u64), rbx); rax = 0x1; bb = 33; continue; }
            31 => { rdx = rbx; rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 32; continue; }
            32 => { rax = 0x34; bb = 33; continue; }
            33 => { r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415f8a10(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x60; const P1: u64 = 0x10; const P2: u64 = 0x5c;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rcx; rdi = ((r8 as u32)) as u32 as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 5; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::lk_add_u32(rbx.wrapping_add(P2), ((rdi as u32)) as u32); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            4 => { rax = 0x2; bb = 5; continue; }
            5 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415f8a60(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141649010; const P1: u64 = 0x141524330; const P2: u64 = 0x141572170; const P3: u64 = 0x1415242e0;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; fl_r = (0x0u64) as u64; rdi = ((r8 as u32)) as u32 as u64; rbp = rdx; rbx = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), rax); bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rsi = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 3; continue; }
            3 => { rdx = 0x68; rcx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 4; continue; }
            4 => { r9 = (crate::simd_ops::ru32(rsp.wrapping_add(0x68u64))) as u32 as u64; xmm0 = 0u128; rdx = (crate::simd_ops::ru32(rsp.wrapping_add(0x60u64))) as u32 as u64; r8 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rax.wrapping_add(0x60u64), rsi); rcx = ((rdi as u32)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), r14); r14 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rax.wrapping_add(0x48u64), r14); crate::simd_ops::wu64(rax.wrapping_add(0x50u64), r14); crate::simd_ops::wu64(rax.wrapping_add(0x58u64), r14); rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu128(rbx, (xmm0) as u128); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbx); crate::simd_ops::wu128(rbx.wrapping_add(0x10u64), (xmm0) as u128); crate::simd_ops::wu128(rbx.wrapping_add(0x20u64), (xmm0) as u128); crate::simd_ops::wu128(rbx.wrapping_add(0x30u64), (xmm0) as u128); crate::simd_ops::wu32(rbx.wrapping_add(0x40u64), ((rax as u32)) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x60u64))) as u64; r9 = ((rax as u32)) as u32 as u64; r8 = rbx; rdx = rbp; r10 = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(r10)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 11; continue; } bb = 5; continue; }
            5 => { rdi = ((r14 as u32)) as u32 as u64; fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x50u64))) as u32 as u64; fl_r = ((r14 as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 8; continue; } bb = 6; continue; }
            6 => { bb = 7; continue; }
            7 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x48u64))) as u64; rcx = 0; fl_l = 0; fl_r = 0; rax = ((rdi as u32)) as u32 as u64; rax = ((rax).wrapping_add(rax)) as u64; rdx = (crate::simd_ops::ru64(rdx.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x8u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = (((rdi as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((rdi as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rbx.wrapping_add(0x50u64))) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 7; continue; } bb = 8; continue; }
            8 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x48u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rbx.wrapping_add(0x50u64), ((r14 as u32)) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rbx.wrapping_add(0x48u64), r14); crate::simd_ops::wu32(rbx.wrapping_add(0x54u64), ((r14 as u32)) as u32); bb = 10; continue; }
            10 => { rdx = rbx; rcx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rsi)) as u64; rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x20u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = r14; bb = 11; continue; }
            11 => { r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rax = rbx; bb = 14; continue; }
            12 => { rax = (crate::simd_ops::ru64(rsi)) as u64; rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x20u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 13; continue; }
            13 => { rax = 0; fl_l = 0; fl_r = 0; bb = 14; continue; }
            14 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415f8bb0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x60; const P1: u64 = 0x50; const P2: u64 = 0x1415f8550; const P3: u64 = 0x5c; const P4: u64 = 0x40; const P5: u64 = 0x4; const P6: u64 = 0x8; const P7: u64 = 0x20; const P8: u64 = 0x48; const P9: u64 = 0x1415242e0; const P10: u64 = 0x54;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; fl_r = (0x0u64) as u64; rbx = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = 0x1; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            2 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbp); rbp = 0; fl_l = 0; fl_r = 0; rax = ((rbp as u32)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rdi); fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru32(rbx.wrapping_add(P3))) as u32 as u64; rax = (((rax as u32)).wrapping_add(0x3cu32)) as u32 as u64; rax = (((rax as u32)).wrapping_add((rcx as u32))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(P4), ((rcx as u32)) as u32); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64; r8 = 0x44; crate::simd_ops::wu32(rbx.wrapping_add(P5), ((rax as u32)) as u32); rdx = rbx; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P6))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64; rsi = ((rax as u32)) as u32 as u64; rdx = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rdx.wrapping_add(P7))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rdi = ((rbp as u32)) as u32 as u64; crate::simd_ops::wu64(rbx.wrapping_add(P0), rbp); fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(P1))) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P8))) as u64; rcx = 0; fl_l = 0; fl_r = 0; rax = ((rdi as u32)) as u32 as u64; rax = ((rax).wrapping_add(rax)) as u64; rdx = (crate::simd_ops::ru64(rdx.wrapping_add(rax.wrapping_mul(8)).wrapping_add(P6))) as u64; rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); rdi = (((rdi as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((rdi as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rbx.wrapping_add(P1))) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 5; continue; } bb = 6; continue; }
            6 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P8))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rbx.wrapping_add(P1), ((rbp as u32)) as u32); rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(P8), rbp); crate::simd_ops::wu32(rbx.wrapping_add(P10), ((rbp as u32)) as u32); bb = 8; continue; }
            8 => { rdx = rbx; rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = ((rsi as u32)) as u32 as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415f8c80(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415b5a50; const P1: u64 = 0x14268e028; const P2: u64 = 0x100; const P3: u64 = 0x108; const P4: u64 = 0x1426452b8; const P5: u64 = 0x130; const P6: u64 = 0x138; const P7: u64 = 0x140; const P8: u64 = 0x148; const P9: u64 = 0x150; const P10: u64 = 0xf8; const P11: u64 = 0x143931c88;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rax = P1;
    crate::simd_ops::wu8(rbx.wrapping_add(P2), (0x0u8) as u8);
    rcx = ((rbx.wrapping_add(P3)) as u64) as u64;
    crate::simd_ops::wu64(rbx, rax);
    rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rbx.wrapping_add(P5), rax);
    crate::simd_ops::wu64(rbx.wrapping_add(P6), rax);
    crate::simd_ops::wu64(rbx.wrapping_add(P7), rax);
    crate::simd_ops::wu64(rbx.wrapping_add(P8), rax);
    crate::simd_ops::wu64(rbx.wrapping_add(P9), rax);
    crate::simd_ops::wu64(rbx.wrapping_add(P10), rax);
    crate::simd_ops::wu64(P11, rax);
    rax = rbx;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415f8cf0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14268e028; const P1: u64 = 0x1426452a0; const P2: u64 = 0x1415b5af0; const P3: u64 = 0x141834d30;
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
    rcx = ((rcx).wrapping_add(0x108u64)) as u64;
    rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp);
    rcx = rdi;
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    fl_l = ((rbx as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64;
    if !(((((fl_l as u8)) & ((fl_r as u8))) == 0)) { rdx = 0x158; rcx = rdi; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rax = rdi;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}
