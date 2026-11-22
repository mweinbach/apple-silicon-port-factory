#![allow(unused_imports, unused_variables, unused_mut, unused_assignments, non_snake_case, dead_code, clippy::all, unreachable_code, unused_parens)]

#[inline(never)]
pub unsafe extern "win64" fn fn_14171b970(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x30; const P1: u64 = 0x14169edb0; const P2: u64 = 0x28; const P3: u64 = 0x20; const P4: u64 = 0x14169ed80; const P5: u64 = 0x143932190; const P6: u64 = 0x60; const P7: u64 = 0x1401de410;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = ((rdx as u32)) as u32 as u64; rbx = rcx; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P3))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            6 => { rcx = rbx; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rdi as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 10; continue; } bb = 7; continue; }
            7 => { fl_l = ((rdi as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { rcx = (crate::simd_ops::ru64(P5)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P6))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            9 => { rdx = 0x38; rcx = rbx; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); bb = 10; continue; }
            10 => { rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14171ba00(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x14171bae0; const P2: u64 = 0x20; const P3: u64 = 0x18; const P4: u64 = 0x14169eda0; const P5: u64 = 0x14169edb0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rdx; rax = (crate::simd_ops::ru64(rcx)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 1; continue; }
            1 => { fl_l = ((r8 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rax.wrapping_add(P0))) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 14; continue; } bb = 2; continue; }
            2 => { rdx = rsp.wrapping_add(0x48); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rdi = (crate::simd_ops::ru64(rax)) as u64; fl_l = (crate::simd_ops::ru64(rdi.wrapping_add(P2))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 9; continue; } bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P3))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = rsp.wrapping_add(0x30); rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rsi = rax; rcx = (crate::simd_ops::ru64(rax)) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            5 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P2))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru64(rsi)) as u64; crate::simd_ops::wu64(rdi.wrapping_add(P2), rax); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(P2))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { rcx = rax; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rdi.wrapping_add(P2))) as u64; bb = 11; continue; }
            11 => { crate::simd_ops::wu64(rbx, rax); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 12; continue; }
            12 => { rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); bb = 13; continue; }
            13 => { rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            14 => { crate::simd_ops::wu64(rdx, (0x0u64) as u64); rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14171bae0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143932190; const P1: u64 = 0x1426abae8; const P2: u64 = 0x14169eda0; const P3: u64 = 0x14169edb0; const P4: u64 = 0x1416a2ea0;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = ((r8 as u32)) as u32 as u64; r15 = rdx; r14 = rcx; rsi = 0; fl_l = 0; fl_r = 0; r9 = ((rsi as u32)) as u32 as u64; r10 = (crate::simd_ops::ru32(rcx.wrapping_add(0x10u64))) as u32 as u64; fl_l = ((r10 as u32)) as u32 as u64; fl_r = ((r10 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { r11 = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; bb = 2; continue; }
            2 => { r8 = ((r9 as u32)) as u32 as u64; rcx = (crate::simd_ops::ru64(r11.wrapping_add(r8.wrapping_mul(8)))) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x18u64))) as u64; fl_l = (crate::simd_ops::ru32(rax.wrapping_add(0x10u64))) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 6; continue; } bb = 3; continue; }
            3 => { r9 = (((r9 as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((r9 as u32)) as u32 as u64; fl_r = ((r10 as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 2; continue; } bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru64(P0)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; r8 = 0; fl_l = 0; fl_r = 0; rdx = ((r8.wrapping_add(0x38u64)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x48u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rdi = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 5; continue; }
            5 => { crate::simd_ops::wu32(rax.wrapping_add(0x8u64), (0x1u32) as u32); rax = P1; crate::simd_ops::wu64(rdi, rax); crate::simd_ops::wu32(rdi.wrapping_add(0x10u64), (0xffffffffu32) as u32); crate::simd_ops::wu64(rdi.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rdi.wrapping_add(0x20u64), rsi); crate::simd_ops::wu64(rdi.wrapping_add(0x28u64), rsi); crate::simd_ops::wu64(rdi.wrapping_add(0x30u64), rsi); bb = 10; continue; }
            6 => { rbx = ((r11.wrapping_add(r8.wrapping_mul(8))) as u64) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rbx)) as u64; bb = 8; continue; }
            8 => { crate::simd_ops::wu64(r15, rcx); bb = 27; continue; }
            9 => { rdi = rsi; bb = 10; continue; }
            10 => { crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rdi); rax = (crate::simd_ops::ru64(r14)) as u64; rcx = (crate::simd_ops::ru64(rax)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(rbx.wrapping_mul(8)))) as u64; crate::simd_ops::wu64(rdi.wrapping_add(0x18u64), rax); rax = (crate::simd_ops::ru32(rax.wrapping_add(0x10u64))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x10u64), ((rax as u32)) as u32); rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x20u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 11; continue; }
            11 => { rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 12; continue; }
            12 => { crate::simd_ops::wu64(rdi.wrapping_add(0x20u64), rsi); rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x28u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 13; continue; }
            13 => { rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 14; continue; }
            14 => { crate::simd_ops::wu64(rdi.wrapping_add(0x28u64), rsi); rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x30u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 16; continue; } bb = 15; continue; }
            15 => { rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 16; continue; }
            16 => { crate::simd_ops::wu64(rdi.wrapping_add(0x30u64), rsi); r14 = ((r14).wrapping_add(0x8u64)) as u64; rax = (crate::simd_ops::ru64(r14.wrapping_add(0x8u64))) as u64; rbp = rax.wrapping_add(0x1); fl_l = rbp; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 23; continue; } bb = 17; continue; }
            17 => { rbx = (crate::simd_ops::ru64(r14)) as u64; fl_l = ((rbx).wrapping_add(0xfffffffffffffff0u64)) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; rbx = ((rbx.wrapping_add(rbp.wrapping_mul(8))) as u64) as u64; rsi = (0xffffffffffffffffu64) as u64; bb = 18; continue; }
            18 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 20; continue; } bb = 19; continue; }
            19 => { rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 20; continue; }
            20 => { rbx = ((rbx).wrapping_sub(0x8u64)) as u64; fl_l = rsi; fl_r = (0x1u64) as u64; rsi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 18; continue; } bb = 21; continue; }
            21 => { rax = (crate::simd_ops::ru64(r14.wrapping_add(0x10u64))) as u64; rax = ((rax).wrapping_shr(1u32)) as u64; fl_l = rbp; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 26; continue; } bb = 22; continue; }
            22 => { r8 = rbp; bb = 25; continue; }
            23 => { fl_l = rbp; fl_r = (crate::simd_ops::ru64(r14.wrapping_add(0x10u64))) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 26; continue; } bb = 24; continue; }
            24 => { r8 = rbp; r8 = ((r8).wrapping_shr(2u32)) as u64; fl_l = ((r8).wrapping_add(rbp)) as u64; fl_r = fl_l; r8 = ((fl_l as u64)) as u64; bb = 25; continue; }
            25 => { rdx = r14; rcx = r14; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 26; continue; }
            26 => { crate::simd_ops::wu64(r14.wrapping_add(0x8u64), rbp); rax = (crate::simd_ops::ru64(r14)) as u64; rbx = ((rax.wrapping_add(rbp.wrapping_mul(8))) as u64) as u64; rcx = rdi; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_sub(0x8u64), rdi); rcx = rdi; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(r15, rdi); rcx = rdi; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 27; continue; }
            27 => { rax = r15; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14171bca0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x14171bae0; const P2: u64 = 0x28; const P3: u64 = 0x18; const P4: u64 = 0x10; const P5: u64 = 0x14169eda0; const P6: u64 = 0x14169edb0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rdx; rax = (crate::simd_ops::ru64(rcx)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 1; continue; }
            1 => { fl_l = ((r8 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rax.wrapping_add(P0))) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 14; continue; } bb = 2; continue; }
            2 => { rdx = rsp.wrapping_add(0x48); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rdi = (crate::simd_ops::ru64(rax)) as u64; fl_l = (crate::simd_ops::ru64(rdi.wrapping_add(P2))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 9; continue; } bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P3))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = rsp.wrapping_add(0x30); rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P4))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rsi = rax; rcx = (crate::simd_ops::ru64(rax)) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            5 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P2))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru64(rsi)) as u64; crate::simd_ops::wu64(rdi.wrapping_add(P2), rax); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(P2))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { rcx = rax; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rdi.wrapping_add(P2))) as u64; bb = 11; continue; }
            11 => { crate::simd_ops::wu64(rbx, rax); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 12; continue; }
            12 => { rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); bb = 13; continue; }
            13 => { rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            14 => { crate::simd_ops::wu64(rdx, (0x0u64) as u64); rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14171bd80(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x14171bae0; const P2: u64 = 0x30; const P3: u64 = 0x18; const P4: u64 = 0x28; const P5: u64 = 0x14169eda0; const P6: u64 = 0x14169edb0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rsi = r9; rbx = rdx; rax = (crate::simd_ops::ru64(rcx)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 1; continue; }
            1 => { fl_l = ((r8 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rax.wrapping_add(P0))) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 14; continue; } bb = 2; continue; }
            2 => { rdx = rsp.wrapping_add(0x20); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rdi = (crate::simd_ops::ru64(rax)) as u64; fl_l = (crate::simd_ops::ru64(rdi.wrapping_add(P2))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 9; continue; } bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P3))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; r8 = rsi; rdx = rsp.wrapping_add(0x40); rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P4))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rsi = rax; rcx = (crate::simd_ops::ru64(rax)) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            5 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P2))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru64(rsi)) as u64; crate::simd_ops::wu64(rdi.wrapping_add(P2), rax); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(P2))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { rcx = rax; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rdi.wrapping_add(P2))) as u64; bb = 11; continue; }
            11 => { crate::simd_ops::wu64(rbx, rax); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 12; continue; }
            12 => { rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); bb = 13; continue; }
            13 => { rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            14 => { crate::simd_ops::wu64(rdx, (0x0u64) as u64); rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14171be60(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143426fd0; const P1: u64 = 0x143428760; const P2: u64 = 0x1411ff390; const P3: u64 = 0x143426d88;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x38u64)) as u64; xmm2 = ((crate::simd_ops::ru32(rdx)) as u32) as u128; xmm1 = ((crate::simd_ops::ru32(rcx.wrapping_add(0x18u64))) as u32) as u128; fl_l = ((xmm2 as u32)) as u32 as u64; fl_r = ((xmm1 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) != f32::from_bits((fl_r as u32)) && !f32::from_bits((fl_l as u32)).is_nan() && !f32::from_bits((fl_r as u32)).is_nan()) { bb = 4; continue; } bb = 1; continue; }
            1 => { xmm0 = ((crate::simd_ops::ru32(rcx.wrapping_add(0x1cu64))) as u32) as u128; fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32 as u64; if (f32::from_bits((fl_l as u32)) != f32::from_bits((fl_r as u32)) && !f32::from_bits((fl_l as u32)).is_nan() && !f32::from_bits((fl_r as u32)).is_nan()) { bb = 4; continue; } bb = 2; continue; }
            2 => { xmm0 = ((crate::simd_ops::ru32(rcx.wrapping_add(0x20u64))) as u32) as u128; fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rdx.wrapping_add(0x8u64))) as u32 as u64; if (f32::from_bits((fl_l as u32)) != f32::from_bits((fl_r as u32)) && !f32::from_bits((fl_l as u32)).is_nan() && !f32::from_bits((fl_r as u32)).is_nan()) { bb = 4; continue; } bb = 3; continue; }
            3 => { xmm0 = 0u128; rsp = ((rsp).wrapping_add(0x38u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            4 => { fl_l = ((xmm2 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rcx.wrapping_add(0x24u64))) as u32 as u64; if (f32::from_bits((fl_l as u32)) != f32::from_bits((fl_r as u32)) && !f32::from_bits((fl_l as u32)).is_nan() && !f32::from_bits((fl_r as u32)).is_nan()) { bb = 8; continue; } bb = 5; continue; }
            5 => { xmm0 = ((crate::simd_ops::ru32(rcx.wrapping_add(0x28u64))) as u32) as u128; fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32 as u64; if (f32::from_bits((fl_l as u32)) != f32::from_bits((fl_r as u32)) && !f32::from_bits((fl_l as u32)).is_nan() && !f32::from_bits((fl_r as u32)).is_nan()) { bb = 8; continue; } bb = 6; continue; }
            6 => { xmm0 = ((crate::simd_ops::ru32(rcx.wrapping_add(0x2cu64))) as u32) as u128; fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rdx.wrapping_add(0x8u64))) as u32 as u64; if (f32::from_bits((fl_l as u32)) != f32::from_bits((fl_r as u32)) && !f32::from_bits((fl_l as u32)).is_nan() && !f32::from_bits((fl_r as u32)).is_nan()) { bb = 8; continue; } bb = 7; continue; }
            7 => { xmm0 = ((crate::simd_ops::ru32(rcx.wrapping_add(0x48u64))) as u32) as u128; rsp = ((rsp).wrapping_add(0x38u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            8 => { crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm6) as u128); xmm6 = ((crate::simd_ops::ru32(rcx.wrapping_add(0xcu64))) as u32) as u128; fl_l = ((xmm6 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(P0)) as u32 as u64; if (f32::from_bits((fl_l as u32)) != f32::from_bits((fl_r as u32)) && !f32::from_bits((fl_l as u32)).is_nan() && !f32::from_bits((fl_r as u32)).is_nan()) { bb = 10; continue; } bb = 9; continue; }
            9 => { xmm0 = ((crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32) as u128; xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) - f32::from_bits(xmm1 as u32)).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) - f32::from_bits(crate::simd_ops::ru32(rcx.wrapping_add(0x1cu64)))).to_bits() as u128); xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) * f32::from_bits(crate::simd_ops::ru32(rcx.wrapping_add(0x30u64)))).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(crate::simd_ops::ru32(rcx.wrapping_add(0x34u64)))).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(xmm2 as u32)).to_bits() as u128); rsp = ((rsp).wrapping_add(0x38u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            10 => { xmm3 = ((crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32) as u128; rax = rsp.wrapping_add(0x40); xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) - f32::from_bits(crate::simd_ops::ru32(rcx))).to_bits() as u128); xmm3 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) - f32::from_bits(crate::simd_ops::ru32(rcx.wrapping_add(0x4u64)))).to_bits() as u128); xmm0 = ((crate::simd_ops::ru32(rcx.wrapping_add(0x34u64))) as u32) as u128; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx); rbx = (crate::simd_ops::ru32(rcx.wrapping_add(0x10u64))) as u32 as u64; fl_l = ((rbx as u32)) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 12; continue; } bb = 11; continue; }
            11 => { xmm0 = (xmm0) ^ (crate::simd_ops::ru128(P1)); crate::simd_ops::wu32(rsp.wrapping_add(0x40u64), ((xmm0 as u32)) as u32); xmm0 = ((crate::simd_ops::ru32(rcx.wrapping_add(0x30u64))) as u32) as u128; crate::simd_ops::wu32(rsp.wrapping_add(0x44u64), ((xmm0 as u32)) as u32); bb = 13; continue; }
            12 => { xmm1 = ((crate::simd_ops::ru32(rcx.wrapping_add(0x30u64))) as u32) as u128; xmm1 = (xmm1) ^ (crate::simd_ops::ru128(P1)); crate::simd_ops::wu32(rsp.wrapping_add(0x44u64), ((xmm1 as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x40u64), ((xmm0 as u32)) as u32); bb = 13; continue; }
            13 => { xmm0 = ((crate::simd_ops::ru64(rax)) as u64) as u128; rdx = rsp.wrapping_add(0x40); rcx = rsp.wrapping_add(0x48); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), ((xmm0 as u64)) as u64); crate::simd_ops::wu32(rsp.wrapping_add(0x40u64), ((xmm2 as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x44u64), ((xmm3 as u32)) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm1 = (xmm0) as u128; xmm0 = 0u128; fl_l = ((xmm1 as u32)) as u32 as u64; fl_r = ((xmm0 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) == f32::from_bits((fl_r as u32)) || f32::from_bits((fl_l as u32)).is_nan() || f32::from_bits((fl_r as u32)).is_nan()) { bb = 17; continue; } bb = 14; continue; }
            14 => { fl_l = ((rbx as u32)) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 16; continue; } bb = 15; continue; }
            15 => { xmm0 = ((crate::simd_ops::ru32(P3)) as u32) as u128; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) - f32::from_bits(xmm1 as u32)).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm6 as u32)).to_bits() as u128); xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; rsp = ((rsp).wrapping_add(0x38u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            16 => { xmm0 = (xmm1) as u128; bb = 17; continue; }
            17 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm6 as u32)).to_bits() as u128); xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; rsp = ((rsp).wrapping_add(0x38u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14171bfb0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143426fd0; const P1: u64 = 0x143428760; const P2: u64 = 0x1411ff390; const P3: u64 = 0x143426d88;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; xmm2 = ((crate::simd_ops::ru32(rdx)) as u32) as u128; rbx = rcx; xmm3 = ((crate::simd_ops::ru32(rcx.wrapping_add(0x18u64))) as u32) as u128; fl_l = ((xmm2 as u32)) as u32 as u64; fl_r = ((xmm3 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) != f32::from_bits((fl_r as u32)) && !f32::from_bits((fl_l as u32)).is_nan() && !f32::from_bits((fl_r as u32)).is_nan()) { bb = 4; continue; } bb = 1; continue; }
            1 => { xmm0 = ((crate::simd_ops::ru32(rcx.wrapping_add(0x1cu64))) as u32) as u128; fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32 as u64; if (f32::from_bits((fl_l as u32)) != f32::from_bits((fl_r as u32)) && !f32::from_bits((fl_l as u32)).is_nan() && !f32::from_bits((fl_r as u32)).is_nan()) { bb = 4; continue; } bb = 2; continue; }
            2 => { xmm0 = ((crate::simd_ops::ru32(rcx.wrapping_add(0x20u64))) as u32) as u128; fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rdx.wrapping_add(0x8u64))) as u32 as u64; if (f32::from_bits((fl_l as u32)) != f32::from_bits((fl_r as u32)) && !f32::from_bits((fl_l as u32)).is_nan() && !f32::from_bits((fl_r as u32)).is_nan()) { bb = 4; continue; } bb = 3; continue; }
            3 => { xmm0 = ((crate::simd_ops::ru32(rcx.wrapping_add(0x48u64))) as u32) as u128; xmm1 = 0u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) - f32::from_bits(xmm1 as u32)).to_bits() as u128); rsp = ((rsp).wrapping_add(0x30u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            4 => { fl_l = ((xmm2 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rcx.wrapping_add(0x24u64))) as u32 as u64; if (f32::from_bits((fl_l as u32)) != f32::from_bits((fl_r as u32)) && !f32::from_bits((fl_l as u32)).is_nan() && !f32::from_bits((fl_r as u32)).is_nan()) { bb = 8; continue; } bb = 5; continue; }
            5 => { xmm0 = ((crate::simd_ops::ru32(rcx.wrapping_add(0x28u64))) as u32) as u128; fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32 as u64; if (f32::from_bits((fl_l as u32)) != f32::from_bits((fl_r as u32)) && !f32::from_bits((fl_l as u32)).is_nan() && !f32::from_bits((fl_r as u32)).is_nan()) { bb = 8; continue; } bb = 6; continue; }
            6 => { xmm0 = ((crate::simd_ops::ru32(rcx.wrapping_add(0x2cu64))) as u32) as u128; fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rdx.wrapping_add(0x8u64))) as u32 as u64; if (f32::from_bits((fl_l as u32)) != f32::from_bits((fl_r as u32)) && !f32::from_bits((fl_l as u32)).is_nan() && !f32::from_bits((fl_r as u32)).is_nan()) { bb = 8; continue; } bb = 7; continue; }
            7 => { xmm1 = ((crate::simd_ops::ru32(rcx.wrapping_add(0x48u64))) as u32) as u128; xmm0 = ((crate::simd_ops::ru32(rbx.wrapping_add(0x48u64))) as u32) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) - f32::from_bits(xmm1 as u32)).to_bits() as u128); rsp = ((rsp).wrapping_add(0x30u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            8 => { crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm6) as u128); xmm6 = ((crate::simd_ops::ru32(rcx.wrapping_add(0xcu64))) as u32) as u128; fl_l = ((xmm6 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(P0)) as u32 as u64; if (f32::from_bits((fl_l as u32)) != f32::from_bits((fl_r as u32)) && !f32::from_bits((fl_l as u32)).is_nan() && !f32::from_bits((fl_r as u32)).is_nan()) { bb = 10; continue; } bb = 9; continue; }
            9 => { xmm1 = ((crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32) as u128; xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) - f32::from_bits(xmm3 as u32)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) - f32::from_bits(crate::simd_ops::ru32(rcx.wrapping_add(0x1cu64)))).to_bits() as u128); xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) * f32::from_bits(crate::simd_ops::ru32(rcx.wrapping_add(0x30u64)))).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(crate::simd_ops::ru32(rcx.wrapping_add(0x34u64)))).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) + f32::from_bits(xmm2 as u32)).to_bits() as u128); bb = 18; continue; }
            10 => { fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(0x10u64))) as u32 as u64; fl_r = 0x2; rax = rsp.wrapping_add(0x40); xmm3 = ((crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32) as u128; xmm3 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) - f32::from_bits(crate::simd_ops::ru32(rcx.wrapping_add(0x4u64)))).to_bits() as u128); xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) - f32::from_bits(crate::simd_ops::ru32(rcx))).to_bits() as u128); xmm0 = ((crate::simd_ops::ru32(rcx.wrapping_add(0x34u64))) as u32) as u128; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 12; continue; } bb = 11; continue; }
            11 => { xmm0 = (xmm0) ^ (crate::simd_ops::ru128(P1)); crate::simd_ops::wu32(rsp.wrapping_add(0x40u64), ((xmm0 as u32)) as u32); xmm0 = ((crate::simd_ops::ru32(rcx.wrapping_add(0x30u64))) as u32) as u128; crate::simd_ops::wu32(rsp.wrapping_add(0x44u64), ((xmm0 as u32)) as u32); bb = 13; continue; }
            12 => { xmm1 = ((crate::simd_ops::ru32(rcx.wrapping_add(0x30u64))) as u32) as u128; xmm1 = (xmm1) ^ (crate::simd_ops::ru128(P1)); crate::simd_ops::wu32(rsp.wrapping_add(0x44u64), ((xmm1 as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x40u64), ((xmm0 as u32)) as u32); bb = 13; continue; }
            13 => { xmm0 = ((crate::simd_ops::ru64(rax)) as u64) as u128; rdx = rsp.wrapping_add(0x40); rcx = rsp.wrapping_add(0x48); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), ((xmm0 as u64)) as u64); crate::simd_ops::wu32(rsp.wrapping_add(0x40u64), ((xmm2 as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x44u64), ((xmm3 as u32)) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm1 = 0u128; fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = ((xmm1 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) == f32::from_bits((fl_r as u32)) || f32::from_bits((fl_l as u32)).is_nan() || f32::from_bits((fl_r as u32)).is_nan()) { bb = 17; continue; } bb = 14; continue; }
            14 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x10u64))) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 16; continue; } bb = 15; continue; }
            15 => { xmm1 = ((crate::simd_ops::ru32(P3)) as u32) as u128; xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) - f32::from_bits(xmm0 as u32)).to_bits() as u128); bb = 17; continue; }
            16 => { xmm1 = (xmm0) as u128; bb = 17; continue; }
            17 => { xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm6 as u32)).to_bits() as u128); bb = 18; continue; }
            18 => { xmm0 = ((crate::simd_ops::ru32(rbx.wrapping_add(0x48u64))) as u32) as u128; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) - f32::from_bits(xmm1 as u32)).to_bits() as u128); rsp = ((rsp).wrapping_add(0x30u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14171c100(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143426fd0; const P1: u64 = 0xc; const P2: u64 = 0x30; const P3: u64 = 0x34; const P4: u64 = 0x24; const P5: u64 = 0x28; const P6: u64 = 0x18; const P7: u64 = 0x1c; const P8: u64 = 0x4; const P9: u64 = 0x1411ff4c0; const P10: u64 = 0x38; const P11: u64 = 0x1434268d0; const P12: u64 = 0x143428760;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut xmm4: u128 = 0; let mut xmm5: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { xmm0 = ((crate::simd_ops::ru32(P0)) as u32) as u128; rax = rcx; fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64; if (f32::from_bits((fl_l as u32)) != f32::from_bits((fl_r as u32)) && !f32::from_bits((fl_l as u32)).is_nan() && !f32::from_bits((fl_r as u32)).is_nan()) { bb = 5; continue; } bb = 1; continue; }
            1 => { xmm0 = 0u128; fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rcx.wrapping_add(P2))) as u32 as u64; if (f32::from_bits((fl_l as u32)) != f32::from_bits((fl_r as u32)) && !f32::from_bits((fl_l as u32)).is_nan() && !f32::from_bits((fl_r as u32)).is_nan()) { bb = 4; continue; } bb = 2; continue; }
            2 => { fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rcx.wrapping_add(P3))) as u32 as u64; if (f32::from_bits((fl_l as u32)) != f32::from_bits((fl_r as u32)) && !f32::from_bits((fl_l as u32)).is_nan() && !f32::from_bits((fl_r as u32)).is_nan()) { bb = 4; continue; } bb = 3; continue; }
            3 => { xmm1 = ((crate::simd_ops::ru32(rax.wrapping_add(P4))) as u32) as u128; fl_l = ((rcx).wrapping_add(0x38u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; xmm0 = ((crate::simd_ops::ru32(rax.wrapping_add(P5))) as u32) as u128; xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) - f32::from_bits(crate::simd_ops::ru32(rax.wrapping_add(P6)))).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) - f32::from_bits(crate::simd_ops::ru32(rax.wrapping_add(P7)))).to_bits() as u128); crate::simd_ops::wu32(rcx, ((xmm1 as u32)) as u32); crate::simd_ops::wu32(rcx.wrapping_add(P8), ((xmm0 as u32)) as u32); return crate::simd_ops::tail_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); }
            4 => { xmm0 = ((crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64) as u128; crate::simd_ops::wu64(rcx.wrapping_add(P10), ((xmm0 as u64)) as u64); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            5 => { xmm2 = ((crate::simd_ops::ru32(rcx.wrapping_add(P4))) as u32) as u128; xmm5 = 0u128; xmm3 = ((crate::simd_ops::ru32(rcx.wrapping_add(P5))) as u32) as u128; xmm4 = 0u128; xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) - f32::from_bits(crate::simd_ops::ru32(rcx))).to_bits() as u128); xmm3 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) - f32::from_bits(crate::simd_ops::ru32(rcx.wrapping_add(P8)))).to_bits() as u128); xmm0 = (xmm2) as u128; xmm1 = (xmm3) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm2 as u32)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm3 as u32)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) + f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm5 = (xmm5 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32)).sqrt().to_bits() as u128); fl_l = ((xmm5 as u32)) as u32 as u64; fl_r = ((xmm4 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) == f32::from_bits((fl_r as u32)) || f32::from_bits((fl_l as u32)).is_nan() || f32::from_bits((fl_r as u32)).is_nan()) { bb = 7; continue; } bb = 6; continue; }
            6 => { xmm0 = ((crate::simd_ops::ru32(P11)) as u32) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) / f32::from_bits(xmm5 as u32)).to_bits() as u128); xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) * f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm3 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) * f32::from_bits(xmm0 as u32)).to_bits() as u128); bb = 8; continue; }
            7 => { xmm2 = 0u128; xmm3 = 0u128; bb = 8; continue; }
            8 => { fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(0x10u64))) as u32 as u64; fl_r = 0x2; rcx = rsp.wrapping_add(0x8); if (((fl_l as u32)) != ((fl_r as u32))) { bb = 10; continue; } bb = 9; continue; }
            9 => { xmm2 = (xmm2) ^ (crate::simd_ops::ru128(P12)); bb = 11; continue; }
            10 => { xmm3 = (xmm3) ^ (crate::simd_ops::ru128(P12)); bb = 11; continue; }
            11 => { crate::simd_ops::wu32(rsp.wrapping_add(0xcu64), ((xmm2 as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x8u64), ((xmm3 as u32)) as u32); xmm0 = ((crate::simd_ops::ru64(rcx)) as u64) as u128; crate::simd_ops::wu64(rax.wrapping_add(0x38u64), ((xmm0 as u64)) as u64); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14171c1e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xc; const P1: u64 = 0x143426fd0; const P2: u64 = 0x24; const P3: u64 = 0x28; const P4: u64 = 0x1c; const P5: u64 = 0x18; const P6: u64 = 0x48; const P7: u64 = 0x38; const P8: u64 = 0x1411ff390; const P9: u64 = 0x10; const P10: u64 = 0x143426d88;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm6) as u128); rbx = rcx; xmm6 = ((crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32) as u128; fl_l = ((xmm6 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(P1)) as u32 as u64; if (f32::from_bits((fl_l as u32)) != f32::from_bits((fl_r as u32)) && !f32::from_bits((fl_l as u32)).is_nan() && !f32::from_bits((fl_r as u32)).is_nan()) { bb = 2; continue; } bb = 1; continue; }
            1 => { xmm1 = ((crate::simd_ops::ru32(rcx.wrapping_add(P2))) as u32) as u128; xmm0 = ((crate::simd_ops::ru32(rcx.wrapping_add(P3))) as u32) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) - f32::from_bits(crate::simd_ops::ru32(rcx.wrapping_add(P4)))).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) - f32::from_bits(crate::simd_ops::ru32(rcx.wrapping_add(P5)))).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm1 as u32)).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(xmm1 as u32)).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32)).sqrt().to_bits() as u128); crate::simd_ops::wu32(rcx.wrapping_add(P6), ((xmm0 as u32)) as u32); xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            2 => { rdx = ((rcx.wrapping_add(P7)) as u64) as u64; rcx = ((rcx).wrapping_add(0x30u64)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm1 = 0u128; fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = ((xmm1 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) == f32::from_bits((fl_r as u32)) || f32::from_bits((fl_l as u32)).is_nan() || f32::from_bits((fl_r as u32)).is_nan()) { bb = 6; continue; } bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(P9))) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 5; continue; } bb = 4; continue; }
            4 => { xmm1 = ((crate::simd_ops::ru32(P10)) as u32) as u128; xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) - f32::from_bits(xmm0 as u32)).to_bits() as u128); bb = 6; continue; }
            5 => { xmm1 = (xmm0) as u128; bb = 6; continue; }
            6 => { xmm6 = (xmm6 & !0xffff_ffffu128) | ((f32::from_bits(xmm6 as u32) * f32::from_bits(xmm1 as u32)).to_bits() as u128); crate::simd_ops::wu32(rbx.wrapping_add(P6), ((xmm6 as u32)) as u32); xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14171c270(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143426fd0; const P1: u64 = 0xc; const P2: u64 = 0x24; const P3: u64 = 0x28; const P4: u64 = 0x18; const P5: u64 = 0x1c; const P6: u64 = 0x4; const P7: u64 = 0x1411ff4c0; const P8: u64 = 0x1434268d0; const P9: u64 = 0x10; const P10: u64 = 0x143428760; const P11: u64 = 0x30;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut xmm4: u128 = 0; let mut xmm5: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { xmm0 = ((crate::simd_ops::ru32(P0)) as u32) as u128; fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64; if (f32::from_bits((fl_l as u32)) != f32::from_bits((fl_r as u32)) && !f32::from_bits((fl_l as u32)).is_nan() && !f32::from_bits((fl_r as u32)).is_nan()) { bb = 2; continue; } bb = 1; continue; }
            1 => { xmm1 = ((crate::simd_ops::ru32(rcx.wrapping_add(P2))) as u32) as u128; xmm0 = ((crate::simd_ops::ru32(rcx.wrapping_add(P3))) as u32) as u128; xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) - f32::from_bits(crate::simd_ops::ru32(rcx.wrapping_add(P4)))).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) - f32::from_bits(crate::simd_ops::ru32(rcx.wrapping_add(P5)))).to_bits() as u128); fl_l = ((rcx).wrapping_add(0x30u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; crate::simd_ops::wu32(rcx, ((xmm1 as u32)) as u32); crate::simd_ops::wu32(rcx.wrapping_add(P6), ((xmm0 as u32)) as u32); return crate::simd_ops::tail_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); }
            2 => { xmm2 = ((crate::simd_ops::ru32(rcx.wrapping_add(P4))) as u32) as u128; xmm5 = 0u128; xmm3 = ((crate::simd_ops::ru32(rcx.wrapping_add(P5))) as u32) as u128; xmm4 = 0u128; xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) - f32::from_bits(crate::simd_ops::ru32(rcx))).to_bits() as u128); xmm3 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) - f32::from_bits(crate::simd_ops::ru32(rcx.wrapping_add(P6)))).to_bits() as u128); xmm0 = (xmm2) as u128; xmm1 = (xmm3) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm2 as u32)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm3 as u32)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) + f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm5 = (xmm5 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32)).sqrt().to_bits() as u128); fl_l = ((xmm5 as u32)) as u32 as u64; fl_r = ((xmm4 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) == f32::from_bits((fl_r as u32)) || f32::from_bits((fl_l as u32)).is_nan() || f32::from_bits((fl_r as u32)).is_nan()) { bb = 4; continue; } bb = 3; continue; }
            3 => { xmm0 = ((crate::simd_ops::ru32(P8)) as u32) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) / f32::from_bits(xmm5 as u32)).to_bits() as u128); xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) * f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm3 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) * f32::from_bits(xmm0 as u32)).to_bits() as u128); bb = 5; continue; }
            4 => { xmm2 = 0u128; xmm3 = 0u128; bb = 5; continue; }
            5 => { fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(P9))) as u32 as u64; fl_r = 0x2; rax = rsp.wrapping_add(0x8); if (((fl_l as u32)) != ((fl_r as u32))) { bb = 7; continue; } bb = 6; continue; }
            6 => { xmm2 = (xmm2) ^ (crate::simd_ops::ru128(P10)); bb = 8; continue; }
            7 => { xmm3 = (xmm3) ^ (crate::simd_ops::ru128(P10)); bb = 8; continue; }
            8 => { crate::simd_ops::wu32(rsp.wrapping_add(0xcu64), ((xmm2 as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x8u64), ((xmm3 as u32)) as u32); xmm0 = ((crate::simd_ops::ru64(rax)) as u64) as u128; crate::simd_ops::wu64(rcx.wrapping_add(P11), ((xmm0 as u64)) as u64); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14171c330(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143426fd0; const P1: u64 = 0xc; const P2: u64 = 0x30; const P3: u64 = 0x143426494; const P4: u64 = 0x48; const P5: u64 = 0x18; const P6: u64 = 0x1c; const P7: u64 = 0x4; const P8: u64 = 0x20; const P9: u64 = 0x8; const P10: u64 = 0x1434268d0; const P11: u64 = 0x10; const P12: u64 = 0x143428760;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut xmm4: u128 = 0; let mut xmm5: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { xmm0 = ((crate::simd_ops::ru32(P0)) as u32) as u128; fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64; if (f32::from_bits((fl_l as u32)) != f32::from_bits((fl_r as u32)) && !f32::from_bits((fl_l as u32)).is_nan() && !f32::from_bits((fl_r as u32)).is_nan()) { bb = 2; continue; } bb = 1; continue; }
            1 => { xmm0 = ((crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64) as u128; rax = rdx; crate::simd_ops::wu64(rdx, ((xmm0 as u64)) as u64); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            2 => { xmm0 = ((crate::simd_ops::ru32(P3)) as u32) as u128; fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rcx.wrapping_add(P4))) as u32 as u64; if !(f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 8; continue; } bb = 3; continue; }
            3 => { xmm0 = ((crate::simd_ops::ru32(rcx.wrapping_add(P5))) as u32) as u128; fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(r8)) as u32 as u64; if (f32::from_bits((fl_l as u32)) != f32::from_bits((fl_r as u32)) && !f32::from_bits((fl_l as u32)).is_nan() && !f32::from_bits((fl_r as u32)).is_nan()) { bb = 6; continue; } bb = 4; continue; }
            4 => { xmm0 = ((crate::simd_ops::ru32(rcx.wrapping_add(P6))) as u32) as u128; fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(r8.wrapping_add(P7))) as u32 as u64; if (f32::from_bits((fl_l as u32)) != f32::from_bits((fl_r as u32)) && !f32::from_bits((fl_l as u32)).is_nan() && !f32::from_bits((fl_r as u32)).is_nan()) { bb = 6; continue; } bb = 5; continue; }
            5 => { xmm0 = ((crate::simd_ops::ru32(rcx.wrapping_add(P8))) as u32) as u128; rax = 0x30; fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(r8.wrapping_add(P9))) as u32 as u64; if (f32::from_bits((fl_l as u32)) == f32::from_bits((fl_r as u32)) || f32::from_bits((fl_l as u32)).is_nan() || f32::from_bits((fl_r as u32)).is_nan()) { bb = 7; continue; } bb = 6; continue; }
            6 => { rax = 0x38; bb = 7; continue; }
            7 => { xmm0 = ((crate::simd_ops::ru64(rax.wrapping_add(rcx))) as u64) as u128; rax = rdx; crate::simd_ops::wu64(rdx, ((xmm0 as u64)) as u64); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            8 => { xmm2 = ((crate::simd_ops::ru32(r8)) as u32) as u128; xmm5 = 0u128; xmm3 = ((crate::simd_ops::ru32(r8.wrapping_add(P7))) as u32) as u128; xmm4 = 0u128; xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) - f32::from_bits(crate::simd_ops::ru32(rcx))).to_bits() as u128); xmm3 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) - f32::from_bits(crate::simd_ops::ru32(rcx.wrapping_add(P7)))).to_bits() as u128); xmm0 = (xmm2) as u128; xmm1 = (xmm3) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm2 as u32)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm3 as u32)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) + f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm5 = (xmm5 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32)).sqrt().to_bits() as u128); fl_l = ((xmm5 as u32)) as u32 as u64; fl_r = ((xmm4 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) == f32::from_bits((fl_r as u32)) || f32::from_bits((fl_l as u32)).is_nan() || f32::from_bits((fl_r as u32)).is_nan()) { bb = 10; continue; } bb = 9; continue; }
            9 => { xmm0 = ((crate::simd_ops::ru32(P10)) as u32) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) / f32::from_bits(xmm5 as u32)).to_bits() as u128); xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) * f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm3 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) * f32::from_bits(xmm0 as u32)).to_bits() as u128); bb = 11; continue; }
            10 => { xmm2 = 0u128; xmm3 = 0u128; bb = 11; continue; }
            11 => { fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(P11))) as u32 as u64; fl_r = 0x2; rax = rsp.wrapping_add(0x8); if (((fl_l as u32)) != ((fl_r as u32))) { bb = 13; continue; } bb = 12; continue; }
            12 => { xmm2 = (xmm2) ^ (crate::simd_ops::ru128(P12)); bb = 14; continue; }
            13 => { xmm3 = (xmm3) ^ (crate::simd_ops::ru128(P12)); bb = 14; continue; }
            14 => { crate::simd_ops::wu32(rsp.wrapping_add(0xcu64), ((xmm2 as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x8u64), ((xmm3 as u32)) as u32); xmm0 = ((crate::simd_ops::ru64(rax)) as u64) as u128; rax = rdx; crate::simd_ops::wu64(rdx, ((xmm0 as u64)) as u64); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14171c420(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14397d1e0; const P1: u64 = 0x1439323c0; const P2: u64 = 0x141834a78; const P3: u64 = 0x1439323bc; const P4: u64 = 0x141834a0c; const P5: u64 = 0x1439323c8; const P6: u64 = 0x1439323c4; const P7: u64 = 0x1439323d0; const P8: u64 = 0x1439323cc; const P9: u64 = 0x1433dd4f4; const P10: u64 = 0x1411ff370; const P11: u64 = 0x143426fd0; const P12: u64 = 0x1426abaf8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rax = ((crate::exe::tls_pointer() as u64)) as u64; rdi = rdx; rbx = (crate::simd_ops::ru32(P0)) as u32 as u64; rsi = rcx; r14 = 0x74; r8 = (crate::simd_ops::ru64(rax.wrapping_add(rbx.wrapping_mul(8)))) as u64; rax = (crate::simd_ops::ru32(r14.wrapping_add(r8))) as u32 as u64; fl_l = (crate::simd_ops::ru32(P1)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) <= (((fl_r as u32)) as i32)) { bb = 3; continue; } bb = 1; continue; }
            1 => { rcx = P1; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru32(P1)) as u32 as u64; fl_r = 0xffffffff; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rcx = P1; crate::simd_ops::wu32(P3, (0xffffffffu32) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 3; continue; }
            3 => { rax = ((crate::exe::tls_pointer() as u64)) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(rbx.wrapping_mul(8)))) as u64; rax = (crate::simd_ops::ru32(r14.wrapping_add(rcx))) as u32 as u64; fl_l = (crate::simd_ops::ru32(P5)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) <= (((fl_r as u32)) as i32)) { bb = 6; continue; } bb = 4; continue; }
            4 => { rcx = P5; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru32(P5)) as u32 as u64; fl_r = 0xffffffff; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rcx = P5; crate::simd_ops::wu32(P6, (0xff00ffffu32) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 6; continue; }
            6 => { rax = ((crate::exe::tls_pointer() as u64)) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(rbx.wrapping_mul(8)))) as u64; rax = (crate::simd_ops::ru32(r14.wrapping_add(rcx))) as u32 as u64; fl_l = (crate::simd_ops::ru32(P7)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) <= (((fl_r as u32)) as i32)) { bb = 9; continue; } bb = 7; continue; }
            7 => { rcx = P7; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru32(P7)) as u32 as u64; fl_r = 0xffffffff; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 9; continue; } bb = 8; continue; }
            8 => { rcx = P7; crate::simd_ops::wu32(P8, (0xff00ffffu32) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 9; continue; }
            9 => { r9 = (crate::simd_ops::ru32(rsi.wrapping_add(0x40u64))) as u32 as u64; r8 = P9; rdx = 0x20; rcx = rsp.wrapping_add(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P10, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r9 = (crate::simd_ops::ru32(rsi.wrapping_add(0x44u64))) as u32 as u64; r8 = P9; rdx = 0x20; rcx = rsp.wrapping_add(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P10, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm0 = ((crate::simd_ops::ru32(rsi.wrapping_add(0xcu64))) as u32) as u128; fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(P11)) as u32 as u64; if (f32::from_bits((fl_l as u32)) == f32::from_bits((fl_r as u32)) || f32::from_bits((fl_l as u32)).is_nan() || f32::from_bits((fl_r as u32)).is_nan()) { bb = 12; continue; } bb = 10; continue; }
            10 => { fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(0x16u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 12; continue; } bb = 11; continue; }
            11 => { xmm3 = crate::simd_ops::vcvtps2pd(xmm0); r8 = P12; rdx = 0x20; rcx = rsp.wrapping_add(0x20); r9 = ((xmm3 as u64)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P10, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 12; continue; }
            12 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14171c590(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143426494; const P1: u64 = 0x143426fd0; const P2: u64 = 0x143428760; const P3: u64 = 0x1418354f0;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm4: u128 = 0; let mut xmm5: u128 = 0; let mut xmm6: u128 = 0; let mut xmm7: u128 = 0; let mut xmm8: u128 = 0; let mut xmm9: u128 = 0; let mut xmm10: u128 = 0; let mut xmm11: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 17>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x80u64)) as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x70u64), (xmm6) as u128); rdi = r9; crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm9) as u128); xmm6 = (xmm2) as u128; xmm9 = ((crate::simd_ops::ru32(rcx.wrapping_add(0x48u64))) as u32) as u128; rsi = rdx; fl_l = ((xmm9 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(P0)) as u32 as u64; rbx = rcx; if (f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 3; continue; } bb = 1; continue; }
            1 => { xmm0 = 0u128; rax = 0x18; fl_l = ((xmm6 as u32)) as u32 as u64; fl_r = ((xmm0 as u32)) as u32 as u64; rdx = 0x24; rcx = (rcx & !0xffu64) | (((if !(f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { 1u8 } else { 0u8 }) as u8 as u64)); fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; rax = (if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { (rdx as u32) } else { (rax as u32) }) as u32 as u64; xmm0 = ((crate::simd_ops::ru64(rax.wrapping_add(rbx))) as u64) as u128; crate::simd_ops::wu64(rsi, ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(rax.wrapping_add(rbx).wrapping_add(0x8u64))) as u32 as u64; crate::simd_ops::wu32(rsi.wrapping_add(0x8u64), ((rax as u32)) as u32); fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 18; continue; } bb = 2; continue; }
            2 => { rax = (((rcx as u8)) as u32) as u32 as u64; fl_l = (((rax) ^ (0x1u64))) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; xmm0 = ((crate::simd_ops::ru64(rbx.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x30u64))) as u64) as u128; crate::simd_ops::wu64(r9, ((xmm0 as u64)) as u64); bb = 18; continue; }
            3 => { xmm3 = ((crate::simd_ops::ru32(rcx.wrapping_add(0x18u64))) as u32) as u128; xmm2 = ((crate::simd_ops::ru32(rcx.wrapping_add(0x24u64))) as u32) as u128; xmm4 = ((crate::simd_ops::ru32(rcx.wrapping_add(0x1cu64))) as u32) as u128; xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) - f32::from_bits(xmm3 as u32)).to_bits() as u128); xmm0 = ((crate::simd_ops::ru32(rcx.wrapping_add(0x28u64))) as u32) as u128; xmm5 = ((crate::simd_ops::ru32(rcx.wrapping_add(0x20u64))) as u32) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) - f32::from_bits(xmm4 as u32)).to_bits() as u128); crate::simd_ops::wu128(rsp.wrapping_add(0x50u64), (xmm8) as u128); xmm8 = ((crate::simd_ops::ru32(rcx.wrapping_add(0x2cu64))) as u32) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm10) as u128); xmm8 = (xmm8 & !0xffff_ffffu128) | ((f32::from_bits(xmm8 as u32) - f32::from_bits(xmm5 as u32)).to_bits() as u128); xmm10 = ((crate::simd_ops::ru32(rcx.wrapping_add(0xcu64))) as u32) as u128; fl_l = ((xmm10 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(P1)) as u32 as u64; if (f32::from_bits((fl_l as u32)) != f32::from_bits((fl_r as u32)) && !f32::from_bits((fl_l as u32)).is_nan() && !f32::from_bits((fl_r as u32)).is_nan()) { bb = 6; continue; } bb = 4; continue; }
            4 => { xmm6 = (xmm6 & !0xffff_ffffu128) | ((f32::from_bits(xmm6 as u32) / f32::from_bits(xmm9 as u32)).to_bits() as u128); xmm1 = (xmm6) as u128; xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm0 = (xmm6) as u128; xmm6 = (xmm6 & !0xffff_ffffu128) | ((f32::from_bits(xmm6 as u32) * f32::from_bits(xmm2 as u32)).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm8 as u32)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) + f32::from_bits(xmm4 as u32)).to_bits() as u128); xmm6 = (xmm6 & !0xffff_ffffu128) | ((f32::from_bits(xmm6 as u32) + f32::from_bits(xmm3 as u32)).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(xmm5 as u32)).to_bits() as u128); crate::simd_ops::wu32(rdx, ((xmm6 as u32)) as u32); crate::simd_ops::wu32(rdx.wrapping_add(0x4u64), ((xmm1 as u32)) as u32); crate::simd_ops::wu32(rdx.wrapping_add(0x8u64), ((xmm0 as u32)) as u32); fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 5; continue; }
            5 => { xmm0 = ((crate::simd_ops::ru64(rcx.wrapping_add(0x30u64))) as u64) as u128; crate::simd_ops::wu64(r9, ((xmm0 as u64)) as u64); bb = 17; continue; }
            6 => { crate::simd_ops::wu64(rsp.wrapping_add(0x98u64), rbp); xmm1 = (xmm6) as u128; rbp = (crate::simd_ops::ru32(rcx.wrapping_add(0x10u64))) as u32 as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x60u64), (xmm7) as u128); xmm7 = ((crate::simd_ops::ru32(P2)) as u32) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm11) as u128); fl_l = ((rbp as u32)) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 8; continue; } bb = 7; continue; }
            7 => { xmm1 = (xmm1) ^ (xmm7); bb = 8; continue; }
            8 => { xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) / f32::from_bits(xmm10 as u32)).to_bits() as u128); xmm0 = 0u128; xmm0 = (xmm0 & !0xffff_ffffu128) | (xmm1 & 0xffff_ffffu128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm11 = ((crate::simd_ops::ru32(rbx.wrapping_add(0x4u64))) as u32) as u128; xmm5 = (xmm0) as u128; xmm5 = crate::simd_ops::vshufps(xmm5, xmm0, 0x1u8); xmm4 = (xmm0) as u128; rax = rsp.wrapping_add(0x90); fl_l = ((rbp as u32)) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 10; continue; } bb = 9; continue; }
            9 => { xmm1 = ((crate::simd_ops::ru32(rbx.wrapping_add(0x34u64))) as u32) as u128; xmm0 = ((crate::simd_ops::ru32(rbx.wrapping_add(0x30u64))) as u32) as u128; xmm1 = (xmm1) ^ (xmm7); crate::simd_ops::wu32(rsp.wrapping_add(0x90u64), ((xmm1 as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x94u64), ((xmm0 as u32)) as u32); bb = 11; continue; }
            10 => { xmm1 = ((crate::simd_ops::ru32(rbx.wrapping_add(0x30u64))) as u32) as u128; xmm0 = ((crate::simd_ops::ru32(rbx.wrapping_add(0x34u64))) as u32) as u128; xmm1 = (xmm1) ^ (xmm7); crate::simd_ops::wu32(rsp.wrapping_add(0x94u64), ((xmm1 as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x90u64), ((xmm0 as u32)) as u32); bb = 11; continue; }
            11 => { xmm2 = ((crate::simd_ops::ru32(rax)) as u32) as u128; xmm1 = ((crate::simd_ops::ru32(rax.wrapping_add(0x4u64))) as u32) as u128; xmm3 = (xmm2) as u128; xmm8 = (xmm8 & !0xffff_ffffu128) | ((f32::from_bits(xmm8 as u32) * f32::from_bits(xmm6 as u32)).to_bits() as u128); xmm0 = (xmm1) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm5 as u32)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm4 as u32)).to_bits() as u128); xmm3 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) * f32::from_bits(xmm4 as u32)).to_bits() as u128); xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) * f32::from_bits(xmm5 as u32)).to_bits() as u128); xmm3 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) + f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm8 = (xmm8 & !0xffff_ffffu128) | ((f32::from_bits(xmm8 as u32) / f32::from_bits(xmm9 as u32)).to_bits() as u128); xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) - f32::from_bits(xmm1 as u32)).to_bits() as u128); xmm1 = (xmm3) as u128; xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm10 as u32)).to_bits() as u128); xmm8 = (xmm8 & !0xffff_ffffu128) | ((f32::from_bits(xmm8 as u32) + f32::from_bits(crate::simd_ops::ru32(rbx.wrapping_add(0x20u64)))).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) + f32::from_bits(xmm11 as u32)).to_bits() as u128); xmm11 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; xmm0 = (xmm2) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm10 as u32)).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(crate::simd_ops::ru32(rbx))).to_bits() as u128); crate::simd_ops::wu32(rsi.wrapping_add(0x8u64), ((xmm8 as u32)) as u32); crate::simd_ops::wu32(rsi.wrapping_add(0x4u64), ((xmm1 as u32)) as u32); crate::simd_ops::wu32(rsi, ((xmm0 as u32)) as u32); fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 16; continue; } bb = 12; continue; }
            12 => { rax = rsp.wrapping_add(0x90); fl_l = ((rbp as u32)) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 14; continue; } bb = 13; continue; }
            13 => { xmm2 = (xmm2) ^ (xmm7); bb = 15; continue; }
            14 => { xmm3 = (xmm3) ^ (xmm7); bb = 15; continue; }
            15 => { crate::simd_ops::wu32(rsp.wrapping_add(0x94u64), ((xmm2 as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x90u64), ((xmm3 as u32)) as u32); xmm0 = ((crate::simd_ops::ru64(rax)) as u64) as u128; crate::simd_ops::wu64(rdi, ((xmm0 as u64)) as u64); bb = 16; continue; }
            16 => { xmm7 = (crate::simd_ops::ru128(rsp.wrapping_add(0x60u64))) as u128; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x98u64))) as u64; bb = 17; continue; }
            17 => { xmm8 = (crate::simd_ops::ru128(rsp.wrapping_add(0x50u64))) as u128; xmm10 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; bb = 18; continue; }
            18 => { xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x70u64))) as u128; r11 = rsp.wrapping_add(0x80); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; xmm9 = (crate::simd_ops::ru128(r11.wrapping_sub(0x40u64))) as u128; rsp = r11; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14171c810(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x24; const P3: u64 = 0x18; const P4: u64 = 0x2c; const P5: u64 = 0x20; const P6: u64 = 0x38; const P7: u64 = 0x30; const P8: u64 = 0x40; const P9: u64 = 0x48;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 3>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = ((rsp).wrapping_sub(0x18u64)) as u64;
    rdx = 0;
    fl_l = 0; fl_r = 0;
    xmm0 = 0u128;
    crate::simd_ops::wu64(rcx, rdx);
    xmm1 = 0u128;
    crate::simd_ops::wu64(rcx.wrapping_add(P0), rdx);
    rax = ((rdx as u32)) as u32 as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(P1), rdx);
    xmm1 = crate::simd_ops::vunpcklps(xmm1, xmm0);
    crate::simd_ops::wu64(rcx.wrapping_add(P2), ((xmm1 as u64)) as u64);
    crate::simd_ops::wu64(rcx.wrapping_add(P3), ((xmm1 as u64)) as u64);
    crate::simd_ops::wu32(rcx.wrapping_add(P4), ((rdx as u32)) as u32);
    crate::simd_ops::wu32(rcx.wrapping_add(P5), ((rdx as u32)) as u32);
    crate::simd_ops::wu32(rsp.wrapping_add(0x8u64), ((rdx as u32)) as u32);
    crate::simd_ops::wu64(rcx.wrapping_add(P6), rdx);
    crate::simd_ops::wu64(rcx.wrapping_add(P7), rdx);
    crate::simd_ops::wu64(rcx.wrapping_add(P8), rdx);
    crate::simd_ops::wu32(rcx.wrapping_add(P9), ((rdx as u32)) as u32);
    rsp = ((rsp).wrapping_add(0x18u64)) as u64;
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14171c8c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x48; const P1: u64 = 0x4c; const P2: u64 = 0x50; const P3: u64 = 0x54; const P4: u64 = 0x60; const P5: u64 = 0x5c; const P6: u64 = 0x58; const P7: u64 = 0x14171a510;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x28u64)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64; fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 9; continue; } bb = 1; continue; }
            1 => { xmm0 = ((crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32) as u128; fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64; if (f32::from_bits((fl_l as u32)) != f32::from_bits((fl_r as u32)) && !f32::from_bits((fl_l as u32)).is_nan() && !f32::from_bits((fl_r as u32)).is_nan()) { bb = 9; continue; } bb = 2; continue; }
            2 => { xmm0 = ((crate::simd_ops::ru32(rdx.wrapping_add(P2))) as u32) as u128; fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rcx.wrapping_add(P2))) as u32 as u64; if (f32::from_bits((fl_l as u32)) != f32::from_bits((fl_r as u32)) && !f32::from_bits((fl_l as u32)).is_nan() && !f32::from_bits((fl_r as u32)).is_nan()) { bb = 9; continue; } bb = 3; continue; }
            3 => { xmm0 = ((crate::simd_ops::ru32(rdx.wrapping_add(P3))) as u32) as u128; fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rcx.wrapping_add(P3))) as u32 as u64; if (f32::from_bits((fl_l as u32)) != f32::from_bits((fl_r as u32)) && !f32::from_bits((fl_l as u32)).is_nan() && !f32::from_bits((fl_r as u32)).is_nan()) { bb = 9; continue; } bb = 4; continue; }
            4 => { xmm0 = ((crate::simd_ops::ru32(rdx.wrapping_add(P4))) as u32) as u128; fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rcx.wrapping_add(P4))) as u32 as u64; if (f32::from_bits((fl_l as u32)) != f32::from_bits((fl_r as u32)) && !f32::from_bits((fl_l as u32)).is_nan() && !f32::from_bits((fl_r as u32)).is_nan()) { bb = 9; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru32(rdx.wrapping_add(P5))) as u32 as u64; fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(P5))) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 9; continue; } bb = 6; continue; }
            6 => { xmm0 = ((crate::simd_ops::ru32(rdx.wrapping_add(P6))) as u32) as u128; fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rcx.wrapping_add(P6))) as u32 as u64; if (f32::from_bits((fl_l as u32)) != f32::from_bits((fl_r as u32)) && !f32::from_bits((fl_l as u32)).is_nan() && !f32::from_bits((fl_r as u32)).is_nan()) { bb = 9; continue; } bb = 7; continue; }
            7 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); rsp = ((rsp).wrapping_add(0x28u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            9 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x28u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14171c930(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14171a740; const P1: u64 = 0x4c; const P2: u64 = 0x48; const P3: u64 = 0x50; const P4: u64 = 0x54; const P5: u64 = 0x60; const P6: u64 = 0x58;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu32(rbx.wrapping_add(P1), (0x3dcccccdu32) as u32);
    crate::simd_ops::wu32(rbx.wrapping_add(P2), ((rax as u32)) as u32);
    crate::simd_ops::wu32(rbx.wrapping_add(P3), (0x3e99999au32) as u32);
    crate::simd_ops::wu32(rbx.wrapping_add(P4), (0x40a00000u32) as u32);
    crate::simd_ops::wu32(rbx.wrapping_add(P5), (0x3f19999au32) as u32);
    crate::simd_ops::wu64(rbx.wrapping_add(P6), (0x3e99999au64) as u64);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14171c970(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14169e230; const P1: u64 = 0x1426abb08; const P2: u64 = 0xf0; const P3: u64 = 0xf8; const P4: u64 = 0x100;
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
    crate::simd_ops::wu64(rbx.wrapping_add(P2), rax);
    crate::simd_ops::wu64(rbx.wrapping_add(P3), rax);
    crate::simd_ops::wu64(rbx.wrapping_add(P4), rax);
    rax = rbx;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14171c9b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1426aa448; const P1: u64 = 0x141833c27; const P2: u64 = 0x141834d30;
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
    rcx = ((rcx).wrapping_add(0x98u64)) as u64;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    fl_l = ((rbx as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64;
    if !(((((fl_l as u8)) & ((fl_r as u8))) == 0)) { rdx = 0x108; rcx = rdi; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rax = rdi;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14171ca00(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1439324a0; const P1: u64 = 0x8; const P2: u64 = 0x141833c2d; const P3: u64 = 0x20; const P4: u64 = 0x141833c49; const P5: u64 = 0x1418348a3;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = (crate::simd_ops::ru64(P0)) as u64; rdi = rcx; rcx = ((rbx.wrapping_add(P1)) as u64) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(P3))) as u32 as u64; rcx = ((rbx.wrapping_add(P1)) as u64) as u64; rax = (((rax as u32)).wrapping_add(1)) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(P3), ((rax as u32)) as u32); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P4, rax, rcx, rdx, r8, r9, rsp); }
            2 => { rcx = ((rax as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x14171ca43, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14171ca50(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xe8; const P1: u64 = 0x98; const P2: u64 = 0x141833c2d; const P3: u64 = 0xf0; const P4: u64 = 0x1417458c0; const P5: u64 = 0x141833c49; const P6: u64 = 0x1418348a3;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rsi = r8; rbp = rdx; rdi = rcx; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(P0))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 3; continue; } bb = 1; continue; }
            1 => { rbx = ((rcx.wrapping_add(P1)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rbx); rcx = rbx; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 5; continue; } bb = 2; continue; }
            2 => { r9 = rsp.wrapping_add(0x20); r8 = rsi; rdx = rbp; rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P3))) as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rdi = rax; rcx = rbx; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rax = rdi; bb = 4; continue; }
            3 => { r9 = rsp.wrapping_add(0x20); rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P3))) as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            5 => { rcx = ((rax as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x14171cae2, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14171caf0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xe8; const P1: u64 = 0x98; const P2: u64 = 0x141833c2d; const P3: u64 = 0xf0; const P4: u64 = 0x141745a30; const P5: u64 = 0x141833c49; const P6: u64 = 0x1418348a3;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rsi = rdx; rdi = rcx; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(P0))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 3; continue; } bb = 1; continue; }
            1 => { rbx = ((rcx.wrapping_add(P1)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rbx); rcx = rbx; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 4; continue; } bb = 2; continue; }
            2 => { r8 = rsp.wrapping_add(0x20); rdx = rsi; rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P3))) as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rdi = rax; rcx = rbx; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rax = rdi; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            3 => { r8 = rsp.wrapping_add(0x20); rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P3))) as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            4 => { rcx = ((rax as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x14171cb80, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14171cb90(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1439324a0; const P1: u64 = 0x1417456f0; const P2: u64 = 0x141833c2d; const P3: u64 = 0x1417458c0; const P4: u64 = 0x141833c49; const P5: u64 = 0x14171d130; const P6: u64 = 0x1418348a3;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x50u64)) as u64; rbp = r9; r14 = r8; rdi = rdx; rcx = (crate::simd_ops::ru64(P0)) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 1; continue; }
            1 => { rdi = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(0xe8u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 4; continue; } bb = 2; continue; }
            2 => { rbx = rdi.wrapping_add(0x98); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbx); rcx = rbx; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 15; continue; } bb = 3; continue; }
            3 => { r9 = rsp.wrapping_add(0x30); r8 = rbp; rdx = r14; rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0xf0u64))) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rdi = rax; rcx = rbx; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rax = rdi; bb = 13; continue; }
            4 => { r9 = rsp.wrapping_add(0x30); r8 = rbp; rdx = r14; rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0xf0u64))) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 13; continue; }
            5 => { rbx = (crate::simd_ops::ru64(P0)) as u64; rcx = rbx.wrapping_add(0x8); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 16; continue; } bb = 6; continue; }
            6 => { rdx = rdi; rcx = (crate::simd_ops::ru64(P0)) as u64; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rsi = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; rsi = (((rsi) & (0xfffffffffffffffcu64))) as u64; rcx = rbx.wrapping_add(0x8); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru8(rsi.wrapping_add(0xe8u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 10; continue; } bb = 7; continue; }
            7 => { rdi = rsi.wrapping_add(0x98); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rcx = rdi; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 17; continue; } bb = 8; continue; }
            8 => { rbx = (crate::simd_ops::ru64(P0)) as u64; rbx = ((rbx).wrapping_add(0x8u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rbx); rcx = rbx; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 18; continue; } bb = 9; continue; }
            9 => { r9 = rsp.wrapping_add(0x30); r8 = rbp; rdx = r14; rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0xf0u64))) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rsi = rax; rcx = rbx; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 12; continue; }
            10 => { rdi = (crate::simd_ops::ru64(P0)) as u64; rdi = ((rdi).wrapping_add(0x8u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rdi); rcx = rdi; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 14; continue; } bb = 11; continue; }
            11 => { r9 = rsp.wrapping_add(0x30); r8 = rbp; rdx = r14; rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0xf0u64))) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rsi = rax; bb = 12; continue; }
            12 => { rcx = rdi; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rax = rsi; bb = 13; continue; }
            13 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rsp = ((rsp).wrapping_add(0x50u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            14 => { rcx = ((rax as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x14171cd38, rcx, rsp); return rax; }
            15 => { rcx = ((rax as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x14171cd40, rcx, rsp); return rax; }
            16 => { rcx = ((rax as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x14171cd48, rcx, rsp); return rax; }
            17 => { rcx = ((rax as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); bb = 18; continue; }
            18 => { rcx = ((rax as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x14171cd58, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14171cd60(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1439324a0; const P1: u64 = 0x1417456f0; const P2: u64 = 0x141833c2d; const P3: u64 = 0x141745a30; const P4: u64 = 0x141833c49; const P5: u64 = 0x14171d130; const P6: u64 = 0x1418348a3;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x50u64)) as u64; rbp = r8; rdi = rdx; rcx = (crate::simd_ops::ru64(P0)) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 1; continue; }
            1 => { rdi = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(0xe8u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 4; continue; } bb = 2; continue; }
            2 => { rbx = rdi.wrapping_add(0x98); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbx); rcx = rbx; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 15; continue; } bb = 3; continue; }
            3 => { r8 = rsp.wrapping_add(0x30); rdx = rbp; rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0xf0u64))) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rdi = rax; rcx = rbx; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rax = rdi; bb = 13; continue; }
            4 => { r8 = rsp.wrapping_add(0x30); rdx = rbp; rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0xf0u64))) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 13; continue; }
            5 => { rbx = (crate::simd_ops::ru64(P0)) as u64; rcx = rbx.wrapping_add(0x8); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 16; continue; } bb = 6; continue; }
            6 => { rdx = rdi; rcx = (crate::simd_ops::ru64(P0)) as u64; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rsi = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; rsi = (((rsi) & (0xfffffffffffffffcu64))) as u64; rcx = rbx.wrapping_add(0x8); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru8(rsi.wrapping_add(0xe8u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 10; continue; } bb = 7; continue; }
            7 => { rdi = rsi.wrapping_add(0x98); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rcx = rdi; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 17; continue; } bb = 8; continue; }
            8 => { rbx = (crate::simd_ops::ru64(P0)) as u64; rbx = ((rbx).wrapping_add(0x8u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rbx); rcx = rbx; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 18; continue; } bb = 9; continue; }
            9 => { r8 = rsp.wrapping_add(0x30); rdx = rbp; rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0xf0u64))) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rsi = rax; rcx = rbx; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 12; continue; }
            10 => { rdi = (crate::simd_ops::ru64(P0)) as u64; rdi = ((rdi).wrapping_add(0x8u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rdi); rcx = rdi; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 14; continue; } bb = 11; continue; }
            11 => { r8 = rsp.wrapping_add(0x30); rdx = rbp; rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0xf0u64))) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rsi = rax; bb = 12; continue; }
            12 => { rcx = rdi; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rax = rsi; bb = 13; continue; }
            13 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rsp = ((rsp).wrapping_add(0x50u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            14 => { rcx = ((rax as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x14171ceed, rcx, rsp); return rax; }
            15 => { rcx = ((rax as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x14171cef5, rcx, rsp); return rax; }
            16 => { rcx = ((rax as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x14171cefd, rcx, rsp); return rax; }
            17 => { rcx = ((rax as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); bb = 18; continue; }
            18 => { rcx = ((rax as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x14171cf0d, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14171cf10(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1439324a0; const P1: u64 = 0x141833c2d; const P2: u64 = 0x18; const P3: u64 = 0x141833c49; const P4: u64 = 0x1418348a3;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = rdx; rbx = (crate::simd_ops::ru64(P0)) as u64; rbx = ((rbx).wrapping_add(0x8u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rbx); rcx = rbx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(P0)) as u64; rcx = (crate::simd_ops::ru64(rax)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; r8 = 0x8; rdx = rdi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rdi = rax; rcx = rbx; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rax = rdi; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            2 => { rcx = ((rax as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x14171cf72, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14171cf80(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1439324a0; const P1: u64 = 0x141833c2d; const P2: u64 = 0x1417453b0; const P3: u64 = 0x20; const P4: u64 = 0x141833c49; const P5: u64 = 0x98; const P6: u64 = 0x88; const P7: u64 = 0x8; const P8: u64 = 0x80; const P9: u64 = 0x10; const P10: u64 = 0x1418348a3;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = r8; rbp = rdx; rdi = rcx; rbx = (crate::simd_ops::ru64(P0)) as u64; rbx = ((rbx).wrapping_add(0x8u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rbx); rcx = rbx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 8; continue; } bb = 1; continue; }
            1 => { r9 = rsi; r8 = rdi; rdx = rbp; rcx = (crate::simd_ops::ru64(P0)) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rsi = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru32(rdi.wrapping_add(P3))) as u32 as u64; fl_l = (((rcx as u32)).wrapping_add(1)) as u32 as u64; rcx = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(P3), ((rcx as u32)) as u32); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; bb = 3; continue; }
            3 => { rcx = rbx; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 4; continue; }
            4 => { rcx = ((rdi.wrapping_add(P5)) as u64) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 7; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(P6))) as u64; crate::simd_ops::wu64(rsi.wrapping_add(P7), rax); rax = ((rdi.wrapping_add(P8)) as u64) as u64; crate::simd_ops::wu64(rsi.wrapping_add(P9), rax); rax = (crate::simd_ops::ru64(rdi.wrapping_add(P6))) as u64; crate::simd_ops::wu64(rax.wrapping_add(P9), rsi); crate::simd_ops::wu64(rdi.wrapping_add(P6), rsi); rcx = ((rdi.wrapping_add(P5)) as u64) as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            6 => { rax = rsi; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            7 => { rcx = ((rax as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x14171d057, rcx, rsp); return rax; }
            8 => { rcx = ((rax as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x14171d05f, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14171d060(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1439324a0; const P1: u64 = 0x141833c2d; const P2: u64 = 0x143932190; const P3: u64 = 0x141833c49; const P4: u64 = 0x10; const P5: u64 = 0x20; const P6: u64 = 0x30; const P7: u64 = 0x143275f28; const P8: u64 = 0x1417453b0; const P9: u64 = 0x1418348a3;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x60u64)) as u64; rdi = rcx; rbx = (crate::simd_ops::ru64(P0)) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { rbx = ((rbx).wrapping_add(0x8u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), rbx); rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 6; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru64(P2)) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 5; continue; } bb = 3; continue; }
            3 => { rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 4; continue; }
            4 => { rax = 0; fl_l = 0; fl_r = 0; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rsp = ((rsp).wrapping_add(0x60u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            5 => { xmm0 = (crate::simd_ops::ru128(rdi)) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(rdi.wrapping_add(P4))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm1) as u128); xmm0 = (crate::simd_ops::ru128(rdi.wrapping_add(P5))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(rdi.wrapping_add(P6))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x50u64), (xmm1) as u128); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), (0x1u64) as u64); r9 = rsp.wrapping_add(0x20); r8 = 0; fl_l = 0; fl_r = 0; rdx = P7; rcx = (crate::simd_ops::ru64(P0)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(P2, rax); rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(P2)) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rsp = ((rsp).wrapping_add(0x60u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            6 => { rcx = ((rax as u32)) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, 0x14171d120, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14171d130(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x70; const P1: u64 = 0xffffffffffffffff; const P2: u64 = 0x10; const P3: u64 = 0x8;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); r8 = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rbx = 0; fl_l = 0; fl_r = 0; r11 = rdx; r9 = ((rbx.wrapping_add(P1)) as u64) as u64; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 16; continue; } bb = 1; continue; }
            1 => { rax = ((rbx as u32)) as u32 as u64; r10 = rdx; bb = 2; continue; }
            2 => { rcx = r8; rcx = ((rcx).wrapping_sub(r11)) as u64; fl_l = r8; fl_r = r11; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 5; continue; } bb = 3; continue; }
            3 => { fl_l = rcx; fl_r = r9; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rbx = r8; r9 = rcx; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 5; continue; }
            5 => { rdx = (crate::simd_ops::ru64(r8.wrapping_add(P2))) as u64; rcx = r10; rcx = ((rcx).wrapping_shr(63u32)) as u64; r8 = (crate::simd_ops::ru64(r8.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(P3))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { fl_l = rdx; fl_r = r8; rax = (if (((fl_l as u64)) != ((fl_r as u64))) { rdx } else { rax }) as u64; bb = 7; continue; }
            7 => { fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 8; continue; }
            8 => { fl_l = ((r10).wrapping_add(r10)) as u64; fl_r = fl_l; r10 = ((fl_l as u64)) as u64; bb = 2; continue; }
            9 => { rax = r8; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x8u64))) as u64; return rax; }
            10 => { fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 16; continue; } bb = 11; continue; }
            11 => { rdx = 0x10; bb = 12; continue; }
            12 => { rcx = rax; rcx = ((rcx).wrapping_sub(r11)) as u64; fl_l = rax; fl_r = r11; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 15; continue; } bb = 13; continue; }
            13 => { fl_l = rcx; fl_r = r9; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 15; continue; } bb = 14; continue; }
            14 => { r9 = rcx; rbx = rax; bb = 15; continue; }
            15 => { fl_l = (crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64; fl_r = (0x0u64) as u64; rcx = 0x8; rcx = (if (((fl_l as u64)) == ((fl_r as u64))) { rdx } else { rcx }) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(rax))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 12; continue; } bb = 16; continue; }
            16 => { rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x8u64))) as u64; return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14171d1e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1439324a0; const P1: u64 = 0x1417456f0; const P2: u64 = 0x141833c2d; const P3: u64 = 0x141745d70; const P4: u64 = 0x14171d130; const P5: u64 = 0x141833c49; const P6: u64 = 0x141745cd0; const P7: u64 = 0x1418348a3;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbp = rdx; rcx = (crate::simd_ops::ru64(P0)) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rdi = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 2; continue; }
            2 => { rsi = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; fl_l = (crate::simd_ops::ru8(rsi.wrapping_add(0xe8u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 5; continue; } bb = 3; continue; }
            3 => { rbx = rsi.wrapping_add(0x98); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rbx); rcx = rbx; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 19; continue; } bb = 4; continue; }
            4 => { r8 = rbp; rdx = rdi; rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0xf0u64))) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 13; continue; }
            5 => { r8 = rbp; rdx = rdi; rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0xf0u64))) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 14; continue; }
            6 => { rbx = (crate::simd_ops::ru64(P0)) as u64; rcx = rbx.wrapping_add(0x8); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 20; continue; } bb = 7; continue; }
            7 => { rdx = rbp; rcx = (crate::simd_ops::ru64(P0)) as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); r14 = rax; rsi = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; rsi = (((rsi) & (0xfffffffffffffffcu64))) as u64; rcx = rbx.wrapping_add(0x8); rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru8(rsi.wrapping_add(0xe8u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 11; continue; } bb = 8; continue; }
            8 => { rbx = rsi.wrapping_add(0x98); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rbx); rcx = rbx; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 21; continue; } bb = 9; continue; }
            9 => { rdi = (crate::simd_ops::ru64(P0)) as u64; rdi = ((rdi).wrapping_add(0x8u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rdi); rcx = rdi; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 22; continue; } bb = 10; continue; }
            10 => { r8 = rbp; rdx = r14; rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0xf0u64))) as u64; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rcx = rdi; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); bb = 13; continue; }
            11 => { rbx = (crate::simd_ops::ru64(P0)) as u64; rbx = ((rbx).wrapping_add(0x8u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rbx); rcx = rbx; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 18; continue; } bb = 12; continue; }
            12 => { r8 = rbp; rdx = r14; rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0xf0u64))) as u64; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); bb = 13; continue; }
            13 => { rcx = rbx; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); bb = 14; continue; }
            14 => { fl_l = rbp; fl_r = (crate::simd_ops::ru64(rsi.wrapping_add(0x30u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 16; continue; } bb = 15; continue; }
            15 => { rax = (crate::simd_ops::ru64(rsi)) as u64; rcx = rsi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x38u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 16; continue; }
            16 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); bb = 17; continue; }
            17 => { return rax; }
            18 => { rcx = ((rax as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x14171d368, rcx, rsp); return rax; }
            19 => { rcx = ((rax as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x14171d370, rcx, rsp); return rax; }
            20 => { rcx = ((rax as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x14171d378, rcx, rsp); return rax; }
            21 => { rcx = ((rax as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); bb = 22; continue; }
            22 => { rcx = ((rax as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x14171d388, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14171d390(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1439324a0; const P1: u64 = 0x141833c2d; const P2: u64 = 0x20; const P3: u64 = 0x141833c49; const P4: u64 = 0x1418348a3;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = r8; rsi = rdx; rbx = (crate::simd_ops::ru64(P0)) as u64; rbx = ((rbx).wrapping_add(0x8u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rbx); rcx = rbx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(P0)) as u64; rcx = (crate::simd_ops::ru64(rax)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; r9 = 0x8; r8 = rdi; rdx = rsi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P3, rax, rcx, rdx, r8, r9, rsp); }
            2 => { rcx = ((rax as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x14171d3fc, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14171d400(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1439324a0; const P1: u64 = 0x1417456f0; const P2: u64 = 0x10; const P3: u64 = 0x8; const P4: u64 = 0x141833c2d; const P5: u64 = 0x14171d130; const P6: u64 = 0x18; const P7: u64 = 0x141833c49; const P8: u64 = 0x1418348a3;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rcx = (crate::simd_ops::ru64(P0)) as u64; rdi = rdx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            2 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx); rbx = (crate::simd_ops::ru64(P0)) as u64; rcx = ((rbx.wrapping_add(P3)) as u64) as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(P0)) as u64; rdx = rdi; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rcx = ((rbx.wrapping_add(P3)) as u64) as u64; rdi = (crate::simd_ops::ru64(rax.wrapping_add(P6))) as u64; rdi = (((rdi) & (0xfffffffffffffffcu64))) as u64; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = rdi; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            4 => { rcx = ((rax as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x14171d472, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14171d4d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x40; const P1: u64 = 0x48; const P2: u64 = 0x108; const P3: u64 = 0xa0; const P4: u64 = 0xb0; const P5: u64 = 0x10; const P6: u64 = 0x18; const P7: u64 = 0x20; const P8: u64 = 0x28; const P9: u64 = 0x30; const P10: u64 = 0x38; const P11: u64 = 0x8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rsi = 0;
    fl_l = 0; fl_r = 0;
    rdi = rdx;
    crate::simd_ops::wu64(rdx.wrapping_add(P0), rsi);
    rbx = rcx;
    crate::simd_ops::wu64(rdx.wrapping_add(P1), rsi);
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp);
    rax = (crate::simd_ops::ru64(rbx)) as u64;
    rcx = rbx;
    rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp);
    rax = ((rax).wrapping_add(crate::simd_ops::ru64(rdi.wrapping_add(P0)))) as u64;
    rcx = rbx;
    crate::simd_ops::wu64(rdi, rax);
    rax = (crate::simd_ops::ru64(rbx)) as u64;
    rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P4))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp);
    rax = ((rax).wrapping_add(crate::simd_ops::ru64(rdi.wrapping_add(P0)))) as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    crate::simd_ops::wu64(rdi.wrapping_add(P5), rsi);
    crate::simd_ops::wu64(rdi.wrapping_add(P6), rsi);
    crate::simd_ops::wu64(rdi.wrapping_add(P7), rsi);
    crate::simd_ops::wu64(rdi.wrapping_add(P8), rsi);
    crate::simd_ops::wu64(rdi.wrapping_add(P9), rsi);
    crate::simd_ops::wu64(rdi.wrapping_add(P10), rsi);
    rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    crate::simd_ops::wu64(rdi.wrapping_add(P11), rax);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14171d6b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1439324a0; const P1: u64 = 0x1417456f0; const P2: u64 = 0x10; const P3: u64 = 0xf0; const P4: u64 = 0x141745e80; const P5: u64 = 0x141833c2d; const P6: u64 = 0x14171d130; const P7: u64 = 0x18; const P8: u64 = 0x141745e60; const P9: u64 = 0x141833c49; const P10: u64 = 0x1418348a3;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rdi = rdx; rcx = (crate::simd_ops::ru64(P0)) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; r9 = rsp.wrapping_add(0x20); r8 = rdi; rdx = rax; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P3))) as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            2 => { rbx = (crate::simd_ops::ru64(P0)) as u64; rbx = ((rbx).wrapping_add(0x8u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rbx); rcx = rbx; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rdx = rdi; rcx = (crate::simd_ops::ru64(P0)) as u64; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rax.wrapping_add(P7))) as u64; rcx = (((rcx) & (0xfffffffffffffffcu64))) as u64; r9 = rsp.wrapping_add(0x20); r8 = rdi; rdx = rax; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P3))) as u64; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); rax = rdi; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            4 => { rcx = ((rax as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x14171d765, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14171d7c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1439324a0; const P1: u64 = 0x1417456f0; const P2: u64 = 0x141833c2d; const P3: u64 = 0x14171dcf0; const P4: u64 = 0x14171d130; const P5: u64 = 0x141833c49; const P6: u64 = 0x141745f50; const P7: u64 = 0x1418348a3;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x50u64)) as u64; r14 = r8; rbp = rdx; rcx = (crate::simd_ops::ru64(P0)) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rsi = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 1; continue; }
            1 => { rdi = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(0xe8u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 4; continue; } bb = 2; continue; }
            2 => { rbx = rdi.wrapping_add(0x98); crate::simd_ops::wu64(rsp.wrapping_add(0x88u64), rbx); rcx = rbx; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 15; continue; } bb = 3; continue; }
            3 => { r9 = r14; r8 = rbp; rdx = rsi; rcx = rdi; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rsi = rax; bb = 12; continue; }
            4 => { r9 = r14; r8 = rbp; rdx = rsi; rcx = rdi; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rsi = rax; bb = 13; continue; }
            5 => { rbx = (crate::simd_ops::ru64(P0)) as u64; rcx = rbx.wrapping_add(0x8); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 16; continue; } bb = 6; continue; }
            6 => { rdx = rbp; rcx = (crate::simd_ops::ru64(P0)) as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); r15 = rax; rsi = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; rsi = (((rsi) & (0xfffffffffffffffcu64))) as u64; rcx = rbx.wrapping_add(0x8); rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru8(rsi.wrapping_add(0xe8u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 10; continue; } bb = 7; continue; }
            7 => { rbx = rsi.wrapping_add(0x98); crate::simd_ops::wu64(rsp.wrapping_add(0x88u64), rbx); rcx = rbx; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 17; continue; } bb = 8; continue; }
            8 => { rdi = (crate::simd_ops::ru64(P0)) as u64; rdi = ((rdi).wrapping_add(0x8u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rdi); rcx = rdi; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 18; continue; } bb = 9; continue; }
            9 => { rax = rsp.wrapping_add(0x38); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); r9 = r14; r8 = rbp; rdx = r15; rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0xf0u64))) as u64; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rsi = rax; rcx = rdi; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); bb = 12; continue; }
            10 => { rbx = (crate::simd_ops::ru64(P0)) as u64; rbx = ((rbx).wrapping_add(0x8u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x88u64), rbx); rcx = rbx; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 14; continue; } bb = 11; continue; }
            11 => { rax = rsp.wrapping_add(0x38); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); r9 = r14; r8 = rbp; rdx = r15; rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0xf0u64))) as u64; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rsi = rax; bb = 12; continue; }
            12 => { rcx = rbx; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); bb = 13; continue; }
            13 => { rax = rsi; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; rsp = ((rsp).wrapping_add(0x50u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            14 => { rcx = ((rax as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x14171d96f, rcx, rsp); return rax; }
            15 => { rcx = ((rax as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x14171d977, rcx, rsp); return rax; }
            16 => { rcx = ((rax as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x14171d97f, rcx, rsp); return rax; }
            17 => { rcx = ((rax as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); bb = 18; continue; }
            18 => { rcx = ((rax as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x14171d98f, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14171d990(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141833c2d; const P1: u64 = 0x1439324a0; const P2: u64 = 0x1417454f0; const P3: u64 = 0x141833c49; const P4: u64 = 0x1418348a3;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = rcx; rbx = (crate::simd_ops::ru64(rcx.wrapping_add(0x78u64))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 1; continue; }
            1 => { rcx = rbx.wrapping_add(0x98); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rcx); rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 12; continue; } bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru64(P1)) as u64; rcx = ((rcx).wrapping_add(0x8u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rcx); rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 13; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x20u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x20u64), ((rax as u32)) as u32); if (((fl_l as u32)) != ((fl_r as u32))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rcx = rdi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0xf8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; crate::simd_ops::wu64(rcx.wrapping_add(0x10u64), rax); rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rcx.wrapping_add(0x8u64), rax); rdx = rdi; rcx = (crate::simd_ops::ru64(P1)) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            5 => { rbx = 0; fl_l = 0; fl_r = 0; bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x38u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); }
            8 => { rbx = (crate::simd_ops::ru64(P1)) as u64; rcx = rbx.wrapping_add(0x8); rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 11; continue; } bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x20u64))) as u32 as u64; rax = (((rax as u32)).wrapping_sub(1)) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x20u64), ((rax as u32)) as u32); rcx = rbx.wrapping_add(0x8); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 10; continue; }
            10 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            11 => { rcx = ((rax as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x14171da87, rcx, rsp); return rax; }
            12 => { rcx = ((rax as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 13; continue; }
            13 => { rcx = ((rax as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x14171da97, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14171daa0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141833c2d; const P1: u64 = 0x60; const P2: u64 = 0xf0; const P3: u64 = 0x230; const P4: u64 = 0x98; const P5: u64 = 0x240; const P6: u64 = 0x141833c49; const P7: u64 = 0x1418348a3;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = rcx; rdi = rdx; rcx = ((rcx).wrapping_add(0x98u64)) as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { fl_l = rdi; fl_r = (crate::simd_ops::ru64(rsi.wrapping_add(P1))) as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rsi.wrapping_add(P2))) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64; fl_l = rdi; fl_r = rcx; rdi = (if (((fl_l as u64)) < ((fl_r as u64))) { rcx } else { rdi }) as u64; bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rsi.wrapping_add(P2))) as u64; rcx = ((rsi.wrapping_add(P4)) as u64) as u64; crate::simd_ops::wu64(rax.wrapping_add(P5), rdi); crate::simd_ops::wu64(rsi.wrapping_add(P1), rdi); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P6, rax, rcx, rdx, r8, r9, rsp); }
            4 => { rcx = ((rax as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x14171db14, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14171db20(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141833c2d; const P1: u64 = 0xf0; const P2: u64 = 0x98; const P3: u64 = 0x248; const P4: u64 = 0x141833c49; const P5: u64 = 0x1418348a3;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = rcx; rsi = rdx; rcx = ((rcx).wrapping_add(0x98u64)) as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(P1))) as u64; rcx = ((rdi.wrapping_add(P2)) as u64) as u64; crate::simd_ops::wu64(rax.wrapping_add(P3), rsi); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P4, rax, rcx, rdx, r8, r9, rsp); }
            2 => { rcx = ((rax as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x14171db75, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14171db80(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x90; const P1: u64 = 0x88; const P2: u64 = 0xfffffffffffffff8; const P3: u64 = 0x10; const P4: u64 = 0xe0; const P5: u64 = 0x1439324a0; const P6: u64 = 0x1417454f0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = rcx; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx); rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rdx = 0; fl_l = 0; fl_r = 0; rax = ((rsi.wrapping_add(P1)) as u64) as u64; fl_l = rax; fl_r = rax; rdi = ((rax.wrapping_add(P2)) as u64) as u64; rdi = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rdx } else { rdi }) as u64; bb = 1; continue; }
            1 => { fl_l = rcx; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rbx = (crate::simd_ops::ru64(rcx.wrapping_add(P3))) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P4))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; bb = 1; continue; }
            3 => { rcx = (crate::simd_ops::ru64(P5)) as u64; rdx = rsi; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P6, rax, rcx, rdx, r8, r9, rsp); }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14171dbf0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x98; const P1: u64 = 0x141833c2d; const P2: u64 = 0x38; const P3: u64 = 0xf0; const P4: u64 = 0x230; const P5: u64 = 0x238; const P6: u64 = 0x90; const P7: u64 = 0x88; const P8: u64 = 0xfffffffffffffff8; const P9: u64 = 0x108; const P10: u64 = 0x10; const P11: u64 = 0x141833c49; const P12: u64 = 0x1418348a3;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbp = rdx; rsi = rcx; rbx = ((rcx.wrapping_add(P0)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx); rcx = rbx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 7; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru32(rsi.wrapping_add(P2))) as u32 as u64; fl_r = 0x1000; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rsi.wrapping_add(P3))) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(P4))) as u64; crate::simd_ops::lk_add_u64(rbp.wrapping_add(0x40u64), (rcx) as u64); rax = (crate::simd_ops::ru64(rsi.wrapping_add(P3))) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(P5))) as u64; fl_l = (crate::simd_ops::lk_add_u64(rbp.wrapping_add(0x48u64), (rcx) as u64)) as u64; fl_r = fl_l; bb = 3; continue; }
            3 => { rdi = (crate::simd_ops::ru64(rsi.wrapping_add(P6))) as u64; rax = ((rsi.wrapping_add(P7)) as u64) as u64; rsi = ((rax.wrapping_add(P8)) as u64) as u64; rcx = 0; fl_l = 0; fl_r = 0; fl_l = rax; fl_r = rax; rsi = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rcx } else { rsi }) as u64; bb = 4; continue; }
            4 => { fl_l = rdi; fl_r = rsi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rdx = rbp; rcx = rdi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P9))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rdi = (crate::simd_ops::ru64(rdi.wrapping_add(P10))) as u64; bb = 4; continue; }
            6 => { rcx = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P11, rax, rcx, rdx, r8, r9, rsp); }
            7 => { rcx = ((rax as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P12, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x14171dcad, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14171dcb0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1439324a0; const P1: u64 = 0x1439323e0; const P2: u64 = 0x141745120; const P3: u64 = 0x14171d060;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    fl_l = (crate::simd_ops::ru64(P0)) as u64; fl_r = (0x0u64) as u64;
    rbx = rdx;
    if !((((fl_l as u64)) != ((fl_r as u64)))) { rdx = rcx; rcx = P1; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); }
    rcx = rbx;
    rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp);
    fl_l = rax; fl_r = rax;
    rax = (rax & !0xffu64) | (((if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { 1u8 } else { 0u8 }) as u8 as u64));
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14171dcf0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xf0; const P1: u64 = 0x141746130; const P2: u64 = 0x1439324a0; const P3: u64 = 0x141833c2d; const P4: u64 = 0x141745ec0; const P5: u64 = 0x141833c49; const P6: u64 = 0x1418348a3;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x50u64)) as u64; rsi = r9; rbp = r8; r14 = rdx; r15 = rcx; rax = rsp.wrapping_add(0x30); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rdi = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { rbx = (crate::simd_ops::ru64(P2)) as u64; rbx = ((rbx).wrapping_add(0x8u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), rbx); rcx = rbx; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 4; continue; } bb = 2; continue; }
            2 => { rax = rsp.wrapping_add(0x30); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); r9 = rsi; r8 = rbp; rdx = r14; rcx = (crate::simd_ops::ru64(r15.wrapping_add(P0))) as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rdi = rax; rcx = rbx; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); bb = 3; continue; }
            3 => { rax = rdi; r11 = rsp.wrapping_add(0x50); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; rbp = (crate::simd_ops::ru64(r11.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x38u64))) as u64; rsp = r11; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            4 => { rcx = ((rax as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x14171dd9c, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14171dda0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1439324a0; const P1: u64 = 0x141833c2d; const P2: u64 = 0x143932190; const P3: u64 = 0xf8; const P4: u64 = 0xe0; const P5: u64 = 0x141833c49; const P6: u64 = 0x141745190; const P7: u64 = 0x1418348a3;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = (crate::simd_ops::ru64(P0)) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 1; continue; }
            1 => { rbx = ((rbx).wrapping_add(0x8u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rbx); rcx = rbx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 7; continue; } bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru64(P2)) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(P2)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P4))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(P2, (0x0u64) as u64); bb = 4; continue; }
            4 => { rcx = rbx; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(P0)) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(P0, (0x0u64) as u64); bb = 6; continue; }
            6 => { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            7 => { rcx = ((rax as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x14171de28, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14171de30(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x18; const P2: u64 = 0x14169eda0; const P3: u64 = 0x70; const P4: u64 = 0x68; const P5: u64 = 0x14171e1d0; const P6: u64 = 0xfffffffffffffff8; const P7: u64 = 0x14169edb0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbp = rcx; fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 3; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64; fl_r = 0xffffffff; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rdi = (crate::simd_ops::ru64(rbp.wrapping_add(0x10u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbp); rsi = (crate::simd_ops::ru32(rdi.wrapping_add(P3))) as u32 as u64; r8 = (crate::simd_ops::ru64(rdi.wrapping_add(P3))) as u64; r8 = ((r8).wrapping_add(1)) as u64; rdx = ((rdi.wrapping_add(P4)) as u64) as u64; rcx = ((rdi.wrapping_add(P4)) as u64) as u64; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P3))) as u64; rax = (crate::simd_ops::ru64(rdi.wrapping_add(P4))) as u64; rbx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; rcx = rbp; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(P6), rbp); rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P3))) as u64; rax = (crate::simd_ops::ru64(rdi.wrapping_add(P4))) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(P6))) as u64; crate::simd_ops::wu32(rcx.wrapping_add(P1), ((rsi as u32)) as u32); rcx = rbp; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); bb = 3; continue; }
            3 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14171dec0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x48; const P1: u64 = 0x1416af2e0; const P2: u64 = 0x14169edb0; const P3: u64 = 0x10; const P4: u64 = 0x50; const P5: u64 = 0x58;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    fl_l = rcx; fl_r = rcx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); }
    rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64;
    fl_l = rcx; fl_r = rcx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); }
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rbx.wrapping_add(P0), rax);
    crate::simd_ops::wu64(rbx.wrapping_add(P3), rax);
    crate::simd_ops::wu64(rbx.wrapping_add(P4), rax);
    crate::simd_ops::wu32(rbx.wrapping_add(P5), ((rax as u32)) as u32);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14171df00(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1416b0360; const P1: u64 = 0x1416af700; const P2: u64 = 0x143932190; const P3: u64 = 0x1416af170; const P4: u64 = 0x14169edb0; const P5: u64 = 0x1416af4d0; const P6: u64 = 0x14169edd0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rdi = rdx; rsi = rcx; rax = (crate::simd_ops::ru64(rdx)) as u64; crate::simd_ops::wu64(rcx.wrapping_add(0x10u64), rax); rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x10u64))) as u64; crate::simd_ops::wu64(rcx.wrapping_add(0x50u64), rax); rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x28u64))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x58u64), ((rax as u32)) as u32); xmm0 = ((crate::simd_ops::ru64(rdx.wrapping_add(0x18u64))) as u64) as u128; crate::simd_ops::wu64(rcx.wrapping_add(0x38u64), ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x20u64))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x40u64), ((rax as u32)) as u32); rax = rdx.wrapping_add(0x8); rbx = (crate::simd_ops::ru64(rax)) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rdx)) as u64; rax = ((rax).wrapping_add(0x58u64)) as u64; rbx = (crate::simd_ops::ru64(rax)) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::lk_add_u32(rbx.wrapping_add(0x8u64), (1) as u32)) as u32 as u64; rbx = (crate::simd_ops::ru64(rax)) as u64; bb = 3; continue; }
            3 => { crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), rbx); rbp = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbp); rcx = rsp.wrapping_add(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::lk_add_u32(rbx.wrapping_add(0x8u64), (1) as u32)) as u32 as u64; bb = 5; continue; }
            5 => { crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rbx); xmm3 = ((crate::simd_ops::ru32(rdi.wrapping_add(0x24u64))) as u32) as u128; r8 = rsi; rdx = rsp.wrapping_add(0x60); rcx = rsp.wrapping_add(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(P2)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; r8 = 0; fl_l = 0; fl_r = 0; rdx = ((r8.wrapping_add(0x48u64)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x48u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbp = rax; bb = 7; continue; }
            7 => { rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x48u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 9; continue; }
            9 => { crate::simd_ops::wu64(rsi.wrapping_add(0x48u64), rbp); rdx = rsp.wrapping_add(0x20); rcx = rbp; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x48u64))) as u64; xmm0 = ((crate::simd_ops::ru64(rdi.wrapping_add(0x18u64))) as u64) as u128; crate::simd_ops::wu64(rcx.wrapping_add(0x10u64), ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x20u64))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x18u64), ((rax as u32)) as u32); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 11; continue; }
            11 => { fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 12; continue; }
            12 => { rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 13; continue; }
            13 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14171e020(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1416af7c0; const P1: u64 = 0x14169eda0; const P2: u64 = 0x14169edb0; const P3: u64 = 0x14171e1d0; const P4: u64 = 0x14183acc2;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = rcx; fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 12; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(0x18u64))) as u32 as u64; fl_r = 0xffffffff; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 12; continue; } bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x48u64))) as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rcx = rsi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rsi); rdi = (crate::simd_ops::ru64(rsi.wrapping_add(0x10u64))) as u64; rbp = (crate::simd_ops::ru32(rsi.wrapping_add(0x18u64))) as u32 as u64; rbx = (crate::simd_ops::ru32(rdi.wrapping_add(0x70u64))) as u32 as u64; rbx = (((rbx as u32)).wrapping_sub(1)) as u32 as u64; r12 = ((rbp.wrapping_mul(8)) as u64) as u64; rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x68u64))) as u64; rcx = (crate::simd_ops::ru64(r12.wrapping_add(rax))) as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x18u64), (0xffffffffu32) as u32); fl_l = ((rbp as u32)) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 8; continue; } bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x68u64))) as u64; r15 = ((rcx.wrapping_add(rbx.wrapping_mul(8))) as u64) as u64; r14 = ((rcx.wrapping_add(r12)) as u64) as u64; rcx = (crate::simd_ops::ru64(r15)) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            5 => { rcx = (crate::simd_ops::ru64(r14)) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru64(r15)) as u64; crate::simd_ops::wu64(r14, rax); rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x68u64))) as u64; rcx = (crate::simd_ops::ru64(r12.wrapping_add(rax))) as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x18u64), ((rbp as u32)) as u32); bb = 8; continue; }
            8 => { rbp = (crate::simd_ops::ru64(rdi.wrapping_add(0x70u64))) as u64; r8 = rbx; rdx = rdi.wrapping_add(0x68); rcx = rdi.wrapping_add(0x68); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); fl_l = rbx; fl_r = rbp; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 11; continue; } bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x68u64))) as u64; rcx = ((rax.wrapping_add(rbp.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rbp; rbx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 11; continue; } bb = 10; continue; }
            10 => { rbx = ((rbx).wrapping_shl(3u32)) as u64; r8 = rbx; rdx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 11; continue; }
            11 => { rcx = rsi; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 12; continue; }
            12 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14171e120(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x143932190; const P2: u64 = 0x60; const P3: u64 = 0x3; const P4: u64 = 0x50; const P5: u64 = 0x48;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rcx; fl_l = r8; fl_r = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 8; continue; } bb = 1; continue; }
            1 => { rdx = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rdi); fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 4; continue; } bb = 2; continue; }
            2 => { rdi = 0; fl_l = 0; fl_r = 0; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(P1)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx, rdi); crate::simd_ops::wu64(rbx.wrapping_add(P0), rdi); rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            4 => { rcx = (crate::simd_ops::ru64(P1)) as u64; rdi = ((r8.wrapping_add(P3)) as u64) as u64; rdi = (((rdi) & (0xfffffffffffffffcu64))) as u64; r9 = ((rdi.wrapping_mul(8)) as u64) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rcx)) as u64; r8 = r9; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P4))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx, rax); crate::simd_ops::wu64(rbx.wrapping_add(P0), rdi); rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            6 => { crate::simd_ops::wu32(rsp.wrapping_add(0x48u64), (0x85u32) as u32); r8 = rsp.wrapping_add(0x48); rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = r9; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P5))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx, rax); bb = 7; continue; }
            7 => { crate::simd_ops::wu64(rbx.wrapping_add(P0), rdi); rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; bb = 8; continue; }
            8 => { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14171e270(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x143932190; const P3: u64 = 0x60;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rcx; rax = (crate::simd_ops::ru64(rcx)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 1; continue; }
            1 => { r8 = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64; rdx = 0; fl_l = 0; fl_r = 0; fl_l = ((r8).wrapping_add(1)) as u64; r8 = ((fl_l as u64)) as u64; bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rcx)) as u64; fl_l = (crate::simd_ops::ru64(rdx.wrapping_add(rax).wrapping_add(P1))) as u64; fl_r = (0xfffffffffffffffeu64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu64(rdx.wrapping_add(rax).wrapping_add(P1), (0xfffffffffffffffeu64) as u64); bb = 4; continue; }
            4 => { rdx = ((rdx).wrapping_add(0x18u64)) as u64; fl_l = r8; fl_r = (0x1u64) as u64; r8 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 5; continue; }
            5 => { rdx = (crate::simd_ops::ru64(rcx)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru64(P2)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 7; continue; }
            7 => { crate::simd_ops::wu64(rbx, (0x0u64) as u64); bb = 8; continue; }
            8 => { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14171e310(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1426abc48; const P1: u64 = 0x1426abc60; const P2: u64 = 0x18; const P3: u64 = 0x20; const P4: u64 = 0x1426abc20; const P5: u64 = 0x60; const P6: u64 = 0x141833c27; const P7: u64 = 0x143932190; const P8: u64 = 0x1417464f0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    rax = P0;
    crate::simd_ops::wu64(rcx, rax);
    rax = P1;
    crate::simd_ops::wu64(rcx.wrapping_add(P2), rax);
    rdi = (crate::simd_ops::ru64(rcx.wrapping_add(P3))) as u64;
    rsi = P4;
    fl_l = rdi; fl_r = rdi;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { crate::simd_ops::wu64(rdi.wrapping_add(P5), rsi); rcx = rdi; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(P7)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = rdi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P5))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); }
    crate::simd_ops::wu64(rbx.wrapping_add(P2), rsi);
    rcx = rbx;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(P8, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14171e420(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x58; const P2: u64 = 0x1418344f3; const P3: u64 = 0x50;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64; fl_r = (0x0u64) as u64;
    if !((((fl_l as u64)) != ((fl_r as u64)))) { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    fl_l = ((rax as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rbx.wrapping_add(P3))) as u32 as u64;
    rax = (rax & !0xffu64) | (((if (((fl_l as u32)) == ((fl_r as u32))) { 1u8 } else { 0u8 }) as u8 as u64));
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14171e450(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143932458; const P1: u64 = 0x1418344f3; const P2: u64 = 0x141833c2d; const P3: u64 = 0x14171e710; const P4: u64 = 0x14171e680; const P5: u64 = 0x141834891; const P6: u64 = 0x141833c27; const P7: u64 = 0x143932190; const P8: u64 = 0x14171e310; const P9: u64 = 0x14171e270; const P10: u64 = 0x1418348a3; const P11: u64 = 0x14171e55b;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x14171e450, funcinfo_rva: 0x36e3570, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x28, conts: &[] };
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru64(P0)) as u64; fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 1; continue; }
            1 => { rdi = (crate::simd_ops::ru64(P0)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14171e476); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rbx = ((rdi.wrapping_add(0x8u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (rbx) as u64); rsi = (crate::simd_ops::ru64(rbx.wrapping_add(0x20u64))) as u64; rcx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14171e48b); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 12; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::lk_add_u64(rsi.wrapping_add(0x58u64), (1) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14171e49c); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::simd_ops::wu32(rsi.wrapping_add(0x50u64), ((rax as u32)) as u32); bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rdi)) as u64; fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::ru64(rax)) as u64 as u64; fl_r = (0x0u64) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { r8 = (0xffffffffu32) as u32 as u64; rdx = (rbx) as u64; rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x30u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14171e4bf); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 3; continue; }
            6 => { rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14171e4c9); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rax = (crate::simd_ops::ru64(P0)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (rax) as u64); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 7; continue; }
            7 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rdi = (crate::simd_ops::ru64(rbx.wrapping_add(0x30u64))) as u64; fl_l = (rdi) as u64 as u64; fl_r = (rdi) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { rcx = ((rdi.wrapping_add(0x50u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14171e4f7); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14171e4ff); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = (crate::simd_ops::ru64(P7)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14171e512); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x60u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 9; continue; }
            9 => { rcx = ((rbx.wrapping_add(0x8u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14171e51a); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14171e522); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = (crate::simd_ops::ru64(P7)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14171e535); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x60u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 10; continue; }
            10 => { crate::simd_ops::wu64(P0, (0x0u64) as u64); bb = 11; continue; }
            11 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            12 => { rcx = ((rax as u32)) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14171e556); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P10) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::lift_ops::trap(0x80000003, P11, rcx, rsp); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14171e560(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x20; const P1: u64 = 0x58; const P2: u64 = 0x1418344f3; const P3: u64 = 0x50; const P4: u64 = 0x60; const P5: u64 = 0x18;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rdi = rcx; fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rbx.wrapping_add(P3))) as u32 as u64; rax = ((rbx.wrapping_add(P4)) as u64) as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rax = ((rdi.wrapping_add(P5)) as u64) as u64; bb = 3; continue; }
            3 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14171e5a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1418344f3;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    r8 = (((rax as u8)) as u32) as u32 as u64;
    r9 = (0x100000001b3u64) as u64;
    rcx = (0xcbf29ce484222325u64) as u64;
    r8 = (((r8) ^ (rcx))) as u64;
    rcx = ((rax as u32)) as u32 as u64;
    r8 = (((r8) as i64).wrapping_mul((r9) as i64) as u64) as u64;
    rcx = (((rcx as u32)).wrapping_shr(8u32)) as u32 as u64;
    rdx = (((rcx as u8)) as u32) as u32 as u64;
    rcx = ((rax as u32)) as u32 as u64;
    r8 = (((r8) ^ (rdx))) as u64;
    rax = ((rax as u32)) as u32 as u64;
    r8 = (((r8) as i64).wrapping_mul((r9) as i64) as u64) as u64;
    rcx = (((rcx as u32)).wrapping_shr(16u32)) as u32 as u64;
    rax = ((rax).wrapping_shr(24u32)) as u64;
    rdx = (((rcx as u8)) as u32) as u32 as u64;
    r8 = (((r8) ^ (rdx))) as u64;
    r8 = (((r8) as i64).wrapping_mul((r9) as i64) as u64) as u64;
    rax = (((rax) ^ (r8))) as u64;
    rax = (((rax) as i64).wrapping_mul((r9) as i64) as u64) as u64;
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14171e600(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x20; const P1: u64 = 0x58;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    fl_l = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; fl_r = (0x0u64) as u64;
    rax = (rax & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64));
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14171e610(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x141834951; const P2: u64 = 0x58; const P3: u64 = 0x1418344f3; const P4: u64 = 0x50; const P5: u64 = 0x1418348a3;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x3; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 7; continue; } bb = 2; continue; }
            2 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 4; continue; }
            3 => { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 4; continue; }
            4 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { crate::simd_ops::lk_add_u64(rbx.wrapping_add(P2), (1) as u64); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu32(rbx.wrapping_add(P4), ((rax as u32)) as u32); rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            6 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            7 => { rcx = ((rax as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x14171e658, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14171e660(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14171e680;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rcx = ((rcx).wrapping_add(0xffffffffffffffe8u64)) as u64;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64));
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14171e680(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x20; const P1: u64 = 0x1418344f3; const P2: u64 = 0x58; const P3: u64 = 0x10; const P4: u64 = 0x141833c49; const P5: u64 = 0x1417465d0; const P6: u64 = 0x141746770;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = rcx; rsi = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::lk_sub_u64(rsi.wrapping_add(P2), (1) as u64); rbp = (crate::simd_ops::ru64(rsi.wrapping_add(P2))) as u64; rbx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx); rax = (crate::simd_ops::ru64(rdi.wrapping_add(P3))) as u64; rdi = ((rbx as u32)) as u32 as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rbx = rax; fl_l = (crate::simd_ops::lk_add_u32(rax, (1) as u32)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); rdi = rax; bb = 2; continue; }
            2 => { rcx = rsi; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); fl_l = rbp; fl_r = rbp; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 3; continue; }
            3 => { fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rcx = rbx; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            5 => { fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { rcx = rbx; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); bb = 7; continue; }
            7 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14171e710(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141833c2d; const P1: u64 = 0x141833c49; const P2: u64 = 0x1417466f0; const P3: u64 = 0x1418348d3; const P4: u64 = 0x141833c43; const P5: u64 = 0x1426abc88; const P6: u64 = 0x141834885; const P7: u64 = 0x141834897; const P8: u64 = 0x1418344f3; const P9: u64 = 0x1418348a3; const P10: u64 = 0x14171e8c4; const P11: u64 = 0x1412dfa60; const P12: u64 = 0x141833c1b; const P13: u64 = 0x14171e8e2; const P14: u64 = 0x14171e8ea;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x14171e710, funcinfo_rva: 0x36e35b4, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x58, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbp) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rsi) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r12) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r15) as u64); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; r15 = ((r8 as u32)) as u32 as u64; rsi = (rdx) as u64; rbp = (rcx) as u64; r12 = (r12 & !0xffu64) | (((0x1u8) as u8 as u64)); rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x20u64))) as u64; r14 = (crate::simd_ops::ru64(rax.wrapping_add(0x58u64))) as u64; fl_l = (r14) as u64 as u64; fl_r = (r14) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 2; continue; }
            2 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r12 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            3 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rbp) as u64); crate::simd_ops::wu8(rsp.wrapping_add(0x28u64), (0x0u8) as u8); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14171e772); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 28; continue; } bb = 4; continue; }
            4 => { crate::simd_ops::wu8(rsp.wrapping_add(0x28u64), ((r12 as u8)) as u8); rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x20u64))) as u64; rdi = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rax.wrapping_add(0x58u64), (rdi) as u64); fl_l = (r14) as u64 as u64; fl_r = (r14) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 5; continue; }
            5 => { rbx = (r14) as u64; bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x20u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14171e79a); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = (rbx) as u64 as u64; fl_r = (0x1u64) as u64 as u64; rbx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 6; continue; } bb = 7; continue; }
            7 => { rcx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14171e7a8); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = ((r15 as u32)) as u32 as u64; fl_r = (0xffffffffu32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 9; continue; } bb = 8; continue; }
            8 => { rcx = ((rbp.wrapping_add(0x50u64)) as u64) as u64; rdx = (rbp) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14171e7ba); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 20; continue; }
            9 => { rbx = (r15) as u64; fl_l = ((r15 as u32)) as u32 as u64; fl_r = ((r15 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 19; continue; } bb = 10; continue; }
            10 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14171e7d0); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (((rax) as i64).wrapping_mul((0x64u64) as i64) as u64) as u64; xmm0 = 0u128; xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((((rbx) as i64) as f64).to_bits() as u128); fl_l = ((xmm0 as u64)) as u64; fl_r = (crate::simd_ops::ru64(P5)) as u64; r15 = (r15 & !0xffu64) | (((if (f64::from_bits(fl_l) > f64::from_bits(fl_r)) { 1u8 } else { 0u8 }) as u8 as u64)); fl_l = ((r15 as u8)) as u8 as u64; fl_r = ((r15 as u8)) as u8 as u64; rax = (0x311cdad160000u64) as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 12; continue; } bb = 11; continue; }
            11 => { rax = (((rbx) as i64).wrapping_mul((0xf4240u64) as i64) as u64) as u64; bb = 12; continue; }
            12 => { rcx = ((rcx).wrapping_add(rax)) as u64; rax = (0x112e0be826d694b3u64) as u64; let prod = (rax as i64 as i128).wrapping_mul((rcx) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rdx = ((((rdx) as i64).wrapping_shr(26u32) as u64)) as u64; rax = (rdx) as u64; rax = ((rax).wrapping_shr(63u32)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (rdx) as u64); rax = (((rdx) as i64).wrapping_mul((0x3b9aca00u64) as i64) as u64) as u64; rcx = ((rcx).wrapping_sub(rax)) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), ((rcx as u32)) as u32); rcx = (rbp) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14171e837); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 29; continue; } bb = 13; continue; }
            13 => { rcx = ((rbp.wrapping_add(0x50u64)) as u64) as u64; r8 = ((rsp.wrapping_add(0x30u64)) as u64) as u64; rdx = (rbp) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14171e850); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 16; continue; } bb = 14; continue; }
            14 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x2u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 30; continue; } bb = 15; continue; }
            15 => { rax = (0x1u32) as u32 as u64; bb = 17; continue; }
            16 => { rax = ((rdi as u32)) as u32 as u64; bb = 17; continue; }
            17 => { fl_l = ((r15 as u8)) as u8 as u64; fl_r = ((r15 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 20; continue; } bb = 18; continue; }
            18 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 20; continue; } bb = 19; continue; }
            19 => { r12 = (r12 & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 20; continue; }
            20 => { fl_l = (rbp) as u64 as u64; fl_r = (rbp) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 27; continue; } bb = 21; continue; }
            21 => { rcx = (rbp) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14171e880); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu8(rsp.wrapping_add(0x28u64), ((rdi as u8)) as u8); fl_l = (r14) as u64 as u64; fl_r = (r14) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 25; continue; } bb = 22; continue; }
            22 => { bb = 23; continue; }
            23 => { rbx = (crate::simd_ops::ru64(rsi.wrapping_add(0x20u64))) as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14171e897); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 26; continue; } bb = 24; continue; }
            24 => { crate::simd_ops::lk_add_u64(rbx.wrapping_add(0x58u64), (1) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14171e8a4); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu32(rbx.wrapping_add(0x50u64), ((rax as u32)) as u32); rdi = ((rdi).wrapping_add(1)) as u64; fl_l = (rdi) as u64 as u64; fl_r = (r14) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 23; continue; } bb = 25; continue; }
            25 => { rax = (((r12 as u8)) as u32) as u32 as u64; bb = 2; continue; }
            26 => { rcx = ((rax as u32)) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14171e8bf); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::lift_ops::trap(0x80000003, P10, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            27 => { rcx = (0x1u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14171e8ca); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P11) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 28; continue; }
            28 => { rcx = ((rax as u32)) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14171e8d2); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 29; continue; }
            29 => { rcx = (0x4u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14171e8dd); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P12) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::lift_ops::trap(0x80000003, P13, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            30 => { rcx = ((rax as u32)) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14171e8e5); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::lift_ops::trap(0x80000003, P14, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14171e8f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut r10, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = (crate::simd_ops::ru64(rcx)) as u64; r9 = rcx; rdx = (((crate::simd_ops::ru8(rax)) as i8 as i32) as u32) as u32 as u64; r10 = rax.wrapping_add(0x1); crate::simd_ops::wu64(rcx, r10); fl_l = ((rdx as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 48; continue; } bb = 1; continue; }
            1 => { if (((((fl_l as u8)) & ((fl_r as u8))) as i8) < 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rax = ((rdx as u32)) as u32 as u64; return rax; }
            3 => { rax = (((rdx as u8)) as u32) as u32 as u64; rax = (rax & !0xffu64) | ((((((rax as u8)) & (0xe0u8))) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = (0xc0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 8; continue; } bb = 4; continue; }
            4 => { rax = ((rdx as u32)) as u32 as u64; rdx = (((crate::simd_ops::ru8(r10)) as i8 as i32) as u32) as u32 as u64; rax = ((((rax as u32)) & (0x1fu32))) as u32 as u64; rax = (((rax as u32)).wrapping_shl(6u32)) as u32 as u64; fl_l = ((rdx as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 48; continue; } bb = 5; continue; }
            5 => { rcx = (((rdx as u8)) as u32) as u32 as u64; rcx = (rcx & !0xffu64) | ((((((rcx as u8)) & (0xc0u8))) as u8 as u64)); fl_l = ((rcx as u8)) as u8 as u64; fl_r = (0x80u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rcx = r10.wrapping_add(0x1); crate::simd_ops::wu64(r9, rcx); rcx = ((rdx as u32)) as u32 as u64; rcx = ((((rcx as u32)) & (0x3fu32))) as u32 as u64; rax = ((((rax as u32)) | ((rcx as u32)))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 49; continue; } bb = 7; continue; }
            7 => { rax = 0xfffd; return rax; }
            8 => { rax = (((rdx as u8)) as u32) as u32 as u64; rax = (rax & !0xffu64) | ((((((rax as u8)) & (0xf0u8))) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = (0xe0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 15; continue; } bb = 9; continue; }
            9 => { rcx = (((crate::simd_ops::ru8(r10)) as i8 as i32) as u32) as u32 as u64; r8 = ((rdx as u32)) as u32 as u64; r8 = ((((r8 as u32)) & (0xfu32))) as u32 as u64; r8 = (((r8 as u32)).wrapping_shl(12u32)) as u32 as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 48; continue; } bb = 10; continue; }
            10 => { rax = (((rcx as u8)) as u32) as u32 as u64; rax = (rax & !0xffu64) | ((((((rax as u8)) & (0xc0u8))) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x80u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 7; continue; } bb = 11; continue; }
            11 => { r10 = ((r10).wrapping_add(1)) as u64; rax = ((rcx as u32)) as u32 as u64; rax = ((((rax as u32)) & (0x3fu32))) as u32 as u64; crate::simd_ops::wu64(r9, r10); rax = (((rax as u32)).wrapping_shl(6u32)) as u32 as u64; r8 = ((((r8 as u32)) | ((rax as u32)))) as u32 as u64; rax = (((crate::simd_ops::ru8(r10)) as i8 as i32) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 48; continue; } bb = 12; continue; }
            12 => { rcx = (((rax as u8)) as u32) as u32 as u64; rcx = (rcx & !0xffu64) | ((((((rcx as u8)) & (0xc0u8))) as u8 as u64)); fl_l = ((rcx as u8)) as u8 as u64; fl_r = (0x80u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 7; continue; } bb = 13; continue; }
            13 => { rcx = r10.wrapping_add(0x1); crate::simd_ops::wu64(r9, rcx); rcx = ((rax as u32)) as u32 as u64; rcx = ((((rcx as u32)) & (0x3fu32))) as u32 as u64; r8 = ((((r8 as u32)) | ((rcx as u32)))) as u32 as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = 0x800; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 7; continue; } bb = 14; continue; }
            14 => { rax = ((r8 as u32)) as u32 as u64; return rax; }
            15 => { rax = (((rdx as u8)) as u32) as u32 as u64; rax = (rax & !0xffu64) | ((((((rax as u8)) & (0xf8u8))) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = (0xf0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 24; continue; } bb = 16; continue; }
            16 => { rcx = (((crate::simd_ops::ru8(r10)) as i8 as i32) as u32) as u32 as u64; r8 = ((rdx as u32)) as u32 as u64; r8 = ((((r8 as u32)) & (0x7u32))) as u32 as u64; r8 = (((r8 as u32)).wrapping_shl(18u32)) as u32 as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 48; continue; } bb = 17; continue; }
            17 => { rax = (((rcx as u8)) as u32) as u32 as u64; rax = (rax & !0xffu64) | ((((((rax as u8)) & (0xc0u8))) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x80u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 7; continue; } bb = 18; continue; }
            18 => { rax = ((rcx as u32)) as u32 as u64; rdx = r10.wrapping_add(0x1); rax = ((((rax as u32)) & (0x3fu32))) as u32 as u64; crate::simd_ops::wu64(r9, rdx); rcx = (((crate::simd_ops::ru8(rdx)) as i8 as i32) as u32) as u32 as u64; rax = (((rax as u32)).wrapping_shl(12u32)) as u32 as u64; r8 = ((((r8 as u32)) | ((rax as u32)))) as u32 as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 48; continue; } bb = 19; continue; }
            19 => { rax = (((rcx as u8)) as u32) as u32 as u64; rax = (rax & !0xffu64) | ((((((rax as u8)) & (0xc0u8))) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x80u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 7; continue; } bb = 20; continue; }
            20 => { rdx = ((rdx).wrapping_add(1)) as u64; rax = ((rcx as u32)) as u32 as u64; rax = ((((rax as u32)) & (0x3fu32))) as u32 as u64; crate::simd_ops::wu64(r9, rdx); rax = (((rax as u32)).wrapping_shl(6u32)) as u32 as u64; r8 = ((((r8 as u32)) | ((rax as u32)))) as u32 as u64; rax = (((crate::simd_ops::ru8(rdx)) as i8 as i32) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 48; continue; } bb = 21; continue; }
            21 => { rcx = (((rax as u8)) as u32) as u32 as u64; rcx = (rcx & !0xffu64) | ((((((rcx as u8)) & (0xc0u8))) as u8 as u64)); fl_l = ((rcx as u8)) as u8 as u64; fl_r = (0x80u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 7; continue; } bb = 22; continue; }
            22 => { rcx = rdx.wrapping_add(0x1); crate::simd_ops::wu64(r9, rcx); rcx = ((rax as u32)) as u32 as u64; rcx = ((((rcx as u32)) & (0x3fu32))) as u32 as u64; r8 = ((((r8 as u32)) | ((rcx as u32)))) as u32 as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = 0x10000; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 7; continue; } bb = 23; continue; }
            23 => { rax = ((r8 as u32)) as u32 as u64; return rax; }
            24 => { rax = (((rdx as u8)) as u32) as u32 as u64; rax = (rax & !0xffu64) | ((((((rax as u8)) & (0xfcu8))) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = (0xf8u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 35; continue; } bb = 25; continue; }
            25 => { rcx = (((crate::simd_ops::ru8(r10)) as i8 as i32) as u32) as u32 as u64; r8 = ((rdx as u32)) as u32 as u64; r8 = ((((r8 as u32)) & (0x3u32))) as u32 as u64; r8 = (((r8 as u32)).wrapping_shl(24u32)) as u32 as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 48; continue; } bb = 26; continue; }
            26 => { rax = (((rcx as u8)) as u32) as u32 as u64; rax = (rax & !0xffu64) | ((((((rax as u8)) & (0xc0u8))) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x80u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 7; continue; } bb = 27; continue; }
            27 => { r10 = ((r10).wrapping_add(1)) as u64; rax = ((rcx as u32)) as u32 as u64; rax = ((((rax as u32)) & (0x3fu32))) as u32 as u64; crate::simd_ops::wu64(r9, r10); rax = (((rax as u32)).wrapping_shl(18u32)) as u32 as u64; r8 = ((((r8 as u32)) | ((rax as u32)))) as u32 as u64; rcx = (((crate::simd_ops::ru8(r10)) as i8 as i32) as u32) as u32 as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 48; continue; } bb = 28; continue; }
            28 => { rax = (((rcx as u8)) as u32) as u32 as u64; rax = (rax & !0xffu64) | ((((((rax as u8)) & (0xc0u8))) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x80u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 7; continue; } bb = 29; continue; }
            29 => { rax = ((rcx as u32)) as u32 as u64; rdx = r10.wrapping_add(0x1); rax = ((((rax as u32)) & (0x3fu32))) as u32 as u64; crate::simd_ops::wu64(r9, rdx); rcx = (((crate::simd_ops::ru8(rdx)) as i8 as i32) as u32) as u32 as u64; rax = (((rax as u32)).wrapping_shl(12u32)) as u32 as u64; r8 = ((((r8 as u32)) | ((rax as u32)))) as u32 as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 48; continue; } bb = 30; continue; }
            30 => { rax = (((rcx as u8)) as u32) as u32 as u64; rax = (rax & !0xffu64) | ((((((rax as u8)) & (0xc0u8))) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x80u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 7; continue; } bb = 31; continue; }
            31 => { rdx = ((rdx).wrapping_add(1)) as u64; rax = ((rcx as u32)) as u32 as u64; rax = ((((rax as u32)) & (0x3fu32))) as u32 as u64; crate::simd_ops::wu64(r9, rdx); rax = (((rax as u32)).wrapping_shl(6u32)) as u32 as u64; r8 = ((((r8 as u32)) | ((rax as u32)))) as u32 as u64; rax = (((crate::simd_ops::ru8(rdx)) as i8 as i32) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 48; continue; } bb = 32; continue; }
            32 => { rcx = (((rax as u8)) as u32) as u32 as u64; rcx = (rcx & !0xffu64) | ((((((rcx as u8)) & (0xc0u8))) as u8 as u64)); fl_l = ((rcx as u8)) as u8 as u64; fl_r = (0x80u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 7; continue; } bb = 33; continue; }
            33 => { rcx = rdx.wrapping_add(0x1); crate::simd_ops::wu64(r9, rcx); rcx = ((rax as u32)) as u32 as u64; rcx = ((((rcx as u32)) & (0x3fu32))) as u32 as u64; r8 = ((((r8 as u32)) | ((rcx as u32)))) as u32 as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = 0x200000; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 7; continue; } bb = 34; continue; }
            34 => { rax = ((r8 as u32)) as u32 as u64; return rax; }
            35 => { rax = (((rdx as u8)) as u32) as u32 as u64; rax = (rax & !0xffu64) | ((((((rax as u8)) & (0xfeu8))) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = (0xfcu8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 7; continue; } bb = 36; continue; }
            36 => { rcx = (((crate::simd_ops::ru8(r10)) as i8 as i32) as u32) as u32 as u64; r8 = ((rdx as u32)) as u32 as u64; r8 = ((((r8 as u32)) & (0x1u32))) as u32 as u64; r8 = (((r8 as u32)).wrapping_shl(30u32)) as u32 as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 48; continue; } bb = 37; continue; }
            37 => { rax = (((rcx as u8)) as u32) as u32 as u64; rax = (rax & !0xffu64) | ((((((rax as u8)) & (0xc0u8))) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x80u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 7; continue; } bb = 38; continue; }
            38 => { r10 = ((r10).wrapping_add(1)) as u64; rax = ((rcx as u32)) as u32 as u64; rax = ((((rax as u32)) & (0x3fu32))) as u32 as u64; crate::simd_ops::wu64(r9, r10); rax = (((rax as u32)).wrapping_shl(24u32)) as u32 as u64; r8 = ((((r8 as u32)) | ((rax as u32)))) as u32 as u64; rcx = (((crate::simd_ops::ru8(r10)) as i8 as i32) as u32) as u32 as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 48; continue; } bb = 39; continue; }
            39 => { rax = (((rcx as u8)) as u32) as u32 as u64; rax = (rax & !0xffu64) | ((((((rax as u8)) & (0xc0u8))) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x80u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 7; continue; } bb = 40; continue; }
            40 => { r10 = ((r10).wrapping_add(1)) as u64; rax = ((rcx as u32)) as u32 as u64; rax = ((((rax as u32)) & (0x3fu32))) as u32 as u64; crate::simd_ops::wu64(r9, r10); rax = (((rax as u32)).wrapping_shl(18u32)) as u32 as u64; r8 = ((((r8 as u32)) | ((rax as u32)))) as u32 as u64; rcx = (((crate::simd_ops::ru8(r10)) as i8 as i32) as u32) as u32 as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 48; continue; } bb = 41; continue; }
            41 => { rax = (((rcx as u8)) as u32) as u32 as u64; rax = (rax & !0xffu64) | ((((((rax as u8)) & (0xc0u8))) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x80u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 7; continue; } bb = 42; continue; }
            42 => { rax = ((rcx as u32)) as u32 as u64; rdx = r10.wrapping_add(0x1); rax = ((((rax as u32)) & (0x3fu32))) as u32 as u64; crate::simd_ops::wu64(r9, rdx); rcx = (((crate::simd_ops::ru8(rdx)) as i8 as i32) as u32) as u32 as u64; rax = (((rax as u32)).wrapping_shl(12u32)) as u32 as u64; r8 = ((((r8 as u32)) | ((rax as u32)))) as u32 as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 48; continue; } bb = 43; continue; }
            43 => { rax = (((rcx as u8)) as u32) as u32 as u64; rax = (rax & !0xffu64) | ((((((rax as u8)) & (0xc0u8))) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x80u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 7; continue; } bb = 44; continue; }
            44 => { rdx = ((rdx).wrapping_add(1)) as u64; rax = ((rcx as u32)) as u32 as u64; rax = ((((rax as u32)) & (0x3fu32))) as u32 as u64; crate::simd_ops::wu64(r9, rdx); rax = (((rax as u32)).wrapping_shl(6u32)) as u32 as u64; r8 = ((((r8 as u32)) | ((rax as u32)))) as u32 as u64; rax = (((crate::simd_ops::ru8(rdx)) as i8 as i32) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 48; continue; } bb = 45; continue; }
            45 => { rcx = (((rax as u8)) as u32) as u32 as u64; rcx = (rcx & !0xffu64) | ((((((rcx as u8)) & (0xc0u8))) as u8 as u64)); fl_l = ((rcx as u8)) as u8 as u64; fl_r = (0x80u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 7; continue; } bb = 46; continue; }
            46 => { rcx = rdx.wrapping_add(0x1); crate::simd_ops::wu64(r9, rcx); rcx = ((rax as u32)) as u32 as u64; rcx = ((((rcx as u32)) & (0x3fu32))) as u32 as u64; r8 = ((((r8 as u32)) | ((rcx as u32)))) as u32 as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = 0x4000000; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 7; continue; } bb = 47; continue; }
            47 => { rax = ((r8 as u32)) as u32 as u64; return rax; }
            48 => { rax = 0; fl_l = 0; fl_r = 0; bb = 49; continue; }
            49 => { return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14171ec30(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14171e8f0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = r8; rbx = rdx; r14 = rcx; rbp = rcx; fl_l = r8; fl_r = (0xffffffffffffffffu64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 38; continue; } bb = 1; continue; }
            1 => { rdi = 0xfffd; bb = 2; continue; }
            2 => { rcx = (((crate::simd_ops::ru8(rbx)) as i8 as i32) as u32) as u32 as u64; rbx = ((rbx).wrapping_add(1)) as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 41; continue; } bb = 3; continue; }
            3 => { if (((((fl_l as u8)) & ((fl_r as u8))) as i8) < 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = ((rcx as u32)) as u32 as u64; bb = 16; continue; }
            5 => { rax = (((rcx as u8)) as u32) as u32 as u64; rax = (rax & !0xffu64) | ((((((rax as u8)) & (0xe0u8))) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = (0xc0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 10; continue; } bb = 6; continue; }
            6 => { rdx = (((crate::simd_ops::ru8(rbx)) as i8 as i32) as u32) as u32 as u64; rax = ((rcx as u32)) as u32 as u64; rax = ((((rax as u32)) & (0x1fu32))) as u32 as u64; rax = (((rax as u32)).wrapping_shl(6u32)) as u32 as u64; fl_l = ((rdx as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 41; continue; } bb = 7; continue; }
            7 => { rcx = (((rdx as u8)) as u32) as u32 as u64; rcx = (rcx & !0xffu64) | ((((((rcx as u8)) & (0xc0u8))) as u8 as u64)); fl_l = ((rcx as u8)) as u8 as u64; fl_r = (0x80u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 36; continue; } bb = 8; continue; }
            8 => { rcx = ((rdx as u32)) as u32 as u64; rbx = ((rbx).wrapping_add(1)) as u64; rcx = ((((rcx as u32)) & (0x3fu32))) as u32 as u64; rax = ((((rax as u32)) | ((rcx as u32)))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 16; continue; } bb = 9; continue; }
            9 => { rax = ((rdi as u32)) as u32 as u64; crate::simd_ops::wu16(r14, ((rax as u16)) as u16); fl_l = ((r14).wrapping_add(0x2u64)) as u64; fl_r = fl_l; r14 = ((fl_l as u64)) as u64; bb = 2; continue; }
            10 => { rax = (((rcx as u8)) as u32) as u32 as u64; rax = (rax & !0xffu64) | ((((((rax as u8)) & (0xf0u8))) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = (0xe0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 18; continue; } bb = 11; continue; }
            11 => { rdx = (((crate::simd_ops::ru8(rbx)) as i8 as i32) as u32) as u32 as u64; rax = ((rcx as u32)) as u32 as u64; rax = ((((rax as u32)) & (0xfu32))) as u32 as u64; rax = (((rax as u32)).wrapping_shl(12u32)) as u32 as u64; fl_l = ((rdx as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 41; continue; } bb = 12; continue; }
            12 => { rcx = (((rdx as u8)) as u32) as u32 as u64; rcx = (rcx & !0xffu64) | ((((((rcx as u8)) & (0xc0u8))) as u8 as u64)); fl_l = ((rcx as u8)) as u8 as u64; fl_r = (0x80u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 36; continue; } bb = 13; continue; }
            13 => { rcx = ((rdx as u32)) as u32 as u64; rbx = ((rbx).wrapping_add(1)) as u64; rcx = ((((rcx as u32)) & (0x3fu32))) as u32 as u64; rcx = (((rcx as u32)).wrapping_shl(6u32)) as u32 as u64; rax = ((((rax as u32)) | ((rcx as u32)))) as u32 as u64; rdx = (((crate::simd_ops::ru8(rbx)) as i8 as i32) as u32) as u32 as u64; fl_l = ((rdx as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 41; continue; } bb = 14; continue; }
            14 => { rcx = (((rdx as u8)) as u32) as u32 as u64; rcx = (rcx & !0xffu64) | ((((((rcx as u8)) & (0xc0u8))) as u8 as u64)); fl_l = ((rcx as u8)) as u8 as u64; fl_r = (0x80u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 36; continue; } bb = 15; continue; }
            15 => { rcx = ((rdx as u32)) as u32 as u64; rbx = ((rbx).wrapping_add(1)) as u64; rcx = ((((rcx as u32)) & (0x3fu32))) as u32 as u64; rax = ((((rax as u32)) | ((rcx as u32)))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x800; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 36; continue; } bb = 16; continue; }
            16 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0xffff; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 37; continue; } bb = 17; continue; }
            17 => { rax = ((rdi as u32)) as u32 as u64; crate::simd_ops::wu16(r14, ((rax as u16)) as u16); fl_l = ((r14).wrapping_add(0x2u64)) as u64; fl_r = fl_l; r14 = ((fl_l as u64)) as u64; bb = 2; continue; }
            18 => { rax = (((rcx as u8)) as u32) as u32 as u64; rax = (rax & !0xffu64) | ((((((rax as u8)) & (0xf8u8))) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = (0xf0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 20; continue; } bb = 19; continue; }
            19 => { rdx = ((rcx as u32)) as u32 as u64; rdx = ((((rdx as u32)) & (0x7u32))) as u32 as u64; fl_l = (((rdx as u32)).wrapping_shl(18u32)) as u32 as u64; rdx = ((fl_l as u32)) as u32 as u64; bb = 29; continue; }
            20 => { rax = (((rcx as u8)) as u32) as u32 as u64; rax = (rax & !0xffu64) | ((((((rax as u8)) & (0xfcu8))) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = (0xf8u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 22; continue; } bb = 21; continue; }
            21 => { rdx = ((rcx as u32)) as u32 as u64; rdx = ((((rdx as u32)) & (0x3u32))) as u32 as u64; fl_l = (((rdx as u32)).wrapping_shl(24u32)) as u32 as u64; rdx = ((fl_l as u32)) as u32 as u64; bb = 26; continue; }
            22 => { rax = (((rcx as u8)) as u32) as u32 as u64; rax = (rax & !0xffu64) | ((((((rax as u8)) & (0xfeu8))) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = (0xfcu8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 36; continue; } bb = 23; continue; }
            23 => { rdx = ((rcx as u32)) as u32 as u64; rcx = (((crate::simd_ops::ru8(rbx)) as i8 as i32) as u32) as u32 as u64; rdx = ((((rdx as u32)) & (0x1u32))) as u32 as u64; rdx = (((rdx as u32)).wrapping_shl(30u32)) as u32 as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 41; continue; } bb = 24; continue; }
            24 => { rax = (((rcx as u8)) as u32) as u32 as u64; rax = (rax & !0xffu64) | ((((((rax as u8)) & (0xc0u8))) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x80u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 36; continue; } bb = 25; continue; }
            25 => { rax = ((rcx as u32)) as u32 as u64; rbx = ((rbx).wrapping_add(1)) as u64; rax = ((((rax as u32)) & (0x3fu32))) as u32 as u64; rax = (((rax as u32)).wrapping_shl(24u32)) as u32 as u64; fl_l = ((((rdx as u32)) | ((rax as u32)))) as u32 as u64; fl_r = fl_l; rdx = ((fl_l as u32)) as u32 as u64; bb = 26; continue; }
            26 => { rcx = (((crate::simd_ops::ru8(rbx)) as i8 as i32) as u32) as u32 as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 41; continue; } bb = 27; continue; }
            27 => { rax = (((rcx as u8)) as u32) as u32 as u64; rax = (rax & !0xffu64) | ((((((rax as u8)) & (0xc0u8))) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x80u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 36; continue; } bb = 28; continue; }
            28 => { rax = ((rcx as u32)) as u32 as u64; rbx = ((rbx).wrapping_add(1)) as u64; rax = ((((rax as u32)) & (0x3fu32))) as u32 as u64; rax = (((rax as u32)).wrapping_shl(18u32)) as u32 as u64; fl_l = ((((rdx as u32)) | ((rax as u32)))) as u32 as u64; fl_r = fl_l; rdx = ((fl_l as u32)) as u32 as u64; bb = 29; continue; }
            29 => { rcx = (((crate::simd_ops::ru8(rbx)) as i8 as i32) as u32) as u32 as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 41; continue; } bb = 30; continue; }
            30 => { rax = (((rcx as u8)) as u32) as u32 as u64; rax = (rax & !0xffu64) | ((((((rax as u8)) & (0xc0u8))) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x80u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 36; continue; } bb = 31; continue; }
            31 => { rax = ((rcx as u32)) as u32 as u64; rbx = ((rbx).wrapping_add(1)) as u64; rax = ((((rax as u32)) & (0x3fu32))) as u32 as u64; rax = (((rax as u32)).wrapping_shl(12u32)) as u32 as u64; rdx = ((((rdx as u32)) | ((rax as u32)))) as u32 as u64; rcx = (((crate::simd_ops::ru8(rbx)) as i8 as i32) as u32) as u32 as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 41; continue; } bb = 32; continue; }
            32 => { rax = (((rcx as u8)) as u32) as u32 as u64; rax = (rax & !0xffu64) | ((((((rax as u8)) & (0xc0u8))) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x80u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 36; continue; } bb = 33; continue; }
            33 => { rax = ((rcx as u32)) as u32 as u64; rbx = ((rbx).wrapping_add(1)) as u64; rax = ((((rax as u32)) & (0x3fu32))) as u32 as u64; rax = (((rax as u32)).wrapping_shl(6u32)) as u32 as u64; rdx = ((((rdx as u32)) | ((rax as u32)))) as u32 as u64; rcx = (((crate::simd_ops::ru8(rbx)) as i8 as i32) as u32) as u32 as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 41; continue; } bb = 34; continue; }
            34 => { rax = (((rcx as u8)) as u32) as u32 as u64; rax = (rax & !0xffu64) | ((((((rax as u8)) & (0xc0u8))) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x80u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 36; continue; } bb = 35; continue; }
            35 => { rax = ((rcx as u32)) as u32 as u64; rbx = ((rbx).wrapping_add(1)) as u64; rax = ((((rax as u32)) & (0x3fu32))) as u32 as u64; fl_l = ((((rdx as u32)) | ((rax as u32)))) as u32 as u64; fl_r = fl_l; rdx = ((fl_l as u32)) as u32 as u64; bb = 36; continue; }
            36 => { rax = ((rdi as u32)) as u32 as u64; bb = 37; continue; }
            37 => { crate::simd_ops::wu16(r14, ((rax as u16)) as u16); fl_l = ((r14).wrapping_add(0x2u64)) as u64; fl_r = fl_l; r14 = ((fl_l as u64)) as u64; bb = 2; continue; }
            38 => { crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rbx); fl_l = rsi; fl_r = rsi; if (((((fl_l as u64)) & ((fl_r as u64))) as i64) <= 0) { bb = 41; continue; } bb = 39; continue; }
            39 => { rdi = 0xfffd; bb = 40; continue; }
            40 => { rcx = rsp.wrapping_add(0x40); rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0xffff; rax = (if (((fl_l as u32)) >= ((fl_r as u32))) { (rdi as u32) } else { (rax as u32) }) as u32 as u64; crate::simd_ops::wu16(r14, ((rax as u16)) as u16); r14 = ((r14).wrapping_add(0x2u64)) as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rax = ((rax).wrapping_sub(rbx)) as u64; fl_l = rax; fl_r = rsi; if ((((fl_l as u64)) as i64) < (((fl_r as u64)) as i64)) { bb = 40; continue; } bb = 41; continue; }
            41 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu16(r14, ((rax as u16)) as u16); r14 = ((r14).wrapping_sub(rbp)) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; r14 = ((((r14) as i64).wrapping_shr(1u32) as u64)) as u64; rax = r14; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14171ee90(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14171e8f0;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rcx); r10 = rdx; r11 = rcx; r8 = rcx; fl_l = rdx; fl_r = (0xffffffffffffffffu64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 42; continue; } bb = 1; continue; }
            1 => { fl_l = rdx; fl_r = rdx; if (((((fl_l as u64)) & ((fl_r as u64))) as i64) <= 0) { bb = 45; continue; } bb = 2; continue; }
            2 => { bb = 3; continue; }
            3 => { r9 = ((crate::simd_ops::ru8(r8)) as u32) as u32 as u64; r8 = ((r8).wrapping_add(1)) as u64; fl_l = ((r9 as u8)) as u8 as u64; fl_r = (0xffu8) as u8 as u64; if ((((fl_l as u8)) as i8) > (((fl_r as u8)) as i8)) { bb = 40; continue; } bb = 4; continue; }
            4 => { rax = (((r9 as u8)) as u32) as u32 as u64; rax = (rax & !0xffu64) | ((((((rax as u8)) & (0xe0u8))) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = (0xc0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 8; continue; } bb = 5; continue; }
            5 => { rax = ((crate::simd_ops::ru8(r8)) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 40; continue; } bb = 6; continue; }
            6 => { rax = (rax & !0xffu64) | ((((((rax as u8)) & (0xc0u8))) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x80u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 40; continue; } bb = 7; continue; }
            7 => { fl_l = ((r8).wrapping_add(1)) as u64; r8 = ((fl_l as u64)) as u64; bb = 40; continue; }
            8 => { rax = (((r9 as u8)) as u32) as u32 as u64; rax = (rax & !0xffu64) | ((((((rax as u8)) & (0xf0u8))) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = (0xe0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 12; continue; } bb = 9; continue; }
            9 => { rax = ((crate::simd_ops::ru8(r8)) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 40; continue; } bb = 10; continue; }
            10 => { rax = (rax & !0xffu64) | ((((((rax as u8)) & (0xc0u8))) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x80u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 40; continue; } bb = 11; continue; }
            11 => { fl_l = ((r8).wrapping_add(1)) as u64; r8 = ((fl_l as u64)) as u64; bb = 5; continue; }
            12 => { rax = (((r9 as u8)) as u32) as u32 as u64; rax = (rax & !0xffu64) | ((((((rax as u8)) & (0xf8u8))) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = (0xf0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 20; continue; } bb = 13; continue; }
            13 => { rax = ((crate::simd_ops::ru8(r8)) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 40; continue; } bb = 14; continue; }
            14 => { rax = (rax & !0xffu64) | ((((((rax as u8)) & (0xc0u8))) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x80u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 40; continue; } bb = 15; continue; }
            15 => { rax = ((crate::simd_ops::ru8(r8.wrapping_add(0x1u64))) as u32) as u32 as u64; r8 = ((r8).wrapping_add(1)) as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 40; continue; } bb = 16; continue; }
            16 => { rax = (rax & !0xffu64) | ((((((rax as u8)) & (0xc0u8))) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x80u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 40; continue; } bb = 17; continue; }
            17 => { rax = ((crate::simd_ops::ru8(r8.wrapping_add(0x1u64))) as u32) as u32 as u64; rcx = r8.wrapping_add(0x1); r8 = rcx; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 40; continue; } bb = 18; continue; }
            18 => { rax = (rax & !0xffu64) | ((((((rax as u8)) & (0xc0u8))) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x80u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 40; continue; } bb = 19; continue; }
            19 => { r8 = rcx.wrapping_add(0x1); bb = 40; continue; }
            20 => { rax = (((r9 as u8)) as u32) as u32 as u64; rax = (rax & !0xffu64) | ((((((rax as u8)) & (0xfcu8))) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = (0xf8u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 28; continue; } bb = 21; continue; }
            21 => { rax = ((crate::simd_ops::ru8(r8)) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 40; continue; } bb = 22; continue; }
            22 => { rax = (rax & !0xffu64) | ((((((rax as u8)) & (0xc0u8))) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x80u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 40; continue; } bb = 23; continue; }
            23 => { rax = ((crate::simd_ops::ru8(r8.wrapping_add(0x1u64))) as u32) as u32 as u64; r8 = ((r8).wrapping_add(1)) as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 40; continue; } bb = 24; continue; }
            24 => { rax = (rax & !0xffu64) | ((((((rax as u8)) & (0xc0u8))) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x80u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 40; continue; } bb = 25; continue; }
            25 => { rax = ((crate::simd_ops::ru8(r8.wrapping_add(0x1u64))) as u32) as u32 as u64; rcx = r8.wrapping_add(0x1); r8 = rcx; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 40; continue; } bb = 26; continue; }
            26 => { rax = (rax & !0xffu64) | ((((((rax as u8)) & (0xc0u8))) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x80u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 40; continue; } bb = 27; continue; }
            27 => { r8 = rcx.wrapping_add(0x1); bb = 5; continue; }
            28 => { r9 = (r9 & !0xffu64) | ((((((r9 as u8)) & (0xfeu8))) as u8 as u64)); fl_l = ((r9 as u8)) as u8 as u64; fl_r = (0xfcu8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 40; continue; } bb = 29; continue; }
            29 => { rax = ((crate::simd_ops::ru8(r8)) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 40; continue; } bb = 30; continue; }
            30 => { rax = (rax & !0xffu64) | ((((((rax as u8)) & (0xc0u8))) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x80u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 40; continue; } bb = 31; continue; }
            31 => { rax = ((crate::simd_ops::ru8(r8.wrapping_add(0x1u64))) as u32) as u32 as u64; r8 = ((r8).wrapping_add(1)) as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 40; continue; } bb = 32; continue; }
            32 => { rax = (rax & !0xffu64) | ((((((rax as u8)) & (0xc0u8))) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x80u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 40; continue; } bb = 33; continue; }
            33 => { rax = ((crate::simd_ops::ru8(r8.wrapping_add(0x1u64))) as u32) as u32 as u64; rcx = r8.wrapping_add(0x1); r8 = rcx; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 40; continue; } bb = 34; continue; }
            34 => { rax = (rax & !0xffu64) | ((((((rax as u8)) & (0xc0u8))) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x80u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 40; continue; } bb = 35; continue; }
            35 => { rax = ((crate::simd_ops::ru8(rcx.wrapping_add(0x1u64))) as u32) as u32 as u64; r8 = rcx.wrapping_add(0x1); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 40; continue; } bb = 36; continue; }
            36 => { rax = (rax & !0xffu64) | ((((((rax as u8)) & (0xc0u8))) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x80u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 40; continue; } bb = 37; continue; }
            37 => { rcx = ((crate::simd_ops::ru8(r8.wrapping_add(0x1u64))) as u32) as u32 as u64; rdx = r8.wrapping_add(0x1); r8 = rdx; fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 40; continue; } bb = 38; continue; }
            38 => { rcx = (rcx & !0xffu64) | ((((((rcx as u8)) & (0xc0u8))) as u8 as u64)); fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 40; continue; } bb = 39; continue; }
            39 => { r8 = rdx.wrapping_add(0x1); bb = 40; continue; }
            40 => { rcx = r8; rbx = ((rbx).wrapping_add(1)) as u64; rcx = ((rcx).wrapping_sub(r11)) as u64; fl_l = rcx; fl_r = r10; if ((((fl_l as u64)) as i64) < (((fl_r as u64)) as i64)) { bb = 3; continue; } bb = 41; continue; }
            41 => { rax = rbx; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            42 => { rcx = rsp.wrapping_add(0x30); rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 45; continue; } bb = 43; continue; }
            43 => { bb = 44; continue; }
            44 => { rcx = rsp.wrapping_add(0x30); rbx = ((rbx).wrapping_add(1)) as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 44; continue; } bb = 45; continue; }
            45 => { rax = rbx; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14171f090(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1426abf38; const P1: u64 = 0x1417473c0; const P2: u64 = 0x14169edb0; const P3: u64 = 0x143932190; const P4: u64 = 0x141746880; const P5: u64 = 0x1426abfd8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rbp = ((r8 as u32)) as u32 as u64; rax = rdx; rbx = rcx; crate::simd_ops::wu32(rcx.wrapping_add(0x8u64), (0x1u32) as u32); rsi = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rcx.wrapping_add(0x10u64), rsi); rcx = P0; crate::simd_ops::wu64(rbx, rcx); r8 = ((r9 as u32)) as u32 as u64; rdx = ((rbp as u32)) as u32 as u64; rcx = rax; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rdi = rax; rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), rdi); fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rcx = rdi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 10; continue; } bb = 4; continue; }
            4 => { fl_l = ((rbp as u8)) as u8 as u64; fl_r = (0x20u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 16; continue; } bb = 5; continue; }
            5 => { rcx = (crate::simd_ops::ru64(P3)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; r8 = 0; fl_l = 0; fl_r = 0; rdx = ((r8.wrapping_add(0x38u64)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x48u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rcx = rax; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rsi = rax; bb = 7; continue; }
            7 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 9; continue; }
            9 => { crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), rsi); bb = 16; continue; }
            10 => { rcx = (crate::simd_ops::ru64(P3)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; r8 = 0; fl_l = 0; fl_r = 0; rdx = ((r8.wrapping_add(0x10u64)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x48u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rdi = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 11; continue; }
            11 => { crate::simd_ops::wu32(rax.wrapping_add(0x8u64), (0x1u32) as u32); rax = P5; crate::simd_ops::wu64(rdi, rax); bb = 13; continue; }
            12 => { rdi = rsi; bb = 13; continue; }
            13 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 15; continue; } bb = 14; continue; }
            14 => { rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 15; continue; }
            15 => { crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), rdi); bb = 16; continue; }
            16 => { rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14171f1b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x14169edb0; const P2: u64 = 0x14169ed80;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    fl_l = rcx; fl_r = rcx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); }
    rcx = rbx;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(P2, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14171f1e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x14169edb0; const P2: u64 = 0x14169ed80; const P3: u64 = 0x143932190; const P4: u64 = 0x60; const P5: u64 = 0x1401de410;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = ((rdx as u32)) as u32 as u64; rbx = rcx; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            2 => { rcx = rbx; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rdi as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 6; continue; } bb = 3; continue; }
            3 => { fl_l = ((rdi as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru64(P3)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P4))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            5 => { rdx = 0x18; rcx = rbx; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            6 => { rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14171f2f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x98; const P2: u64 = 0x143932190; const P3: u64 = 0x48; const P4: u64 = 0x8; const P5: u64 = 0x1426abfd8; const P6: u64 = 0x14169edb0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rdi = rcx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 7; continue; } bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P0))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rbx); rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(P2)) as u64; r8 = 0; fl_l = 0; fl_r = 0; rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = ((r8.wrapping_add(P0)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); rbx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu32(rax.wrapping_add(P4), (0x1u32) as u32); rax = P5; crate::simd_ops::wu64(rbx, rax); bb = 4; continue; }
            3 => { rbx = 0; fl_l = 0; fl_r = 0; bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P0))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            6 => { crate::simd_ops::wu64(rdi.wrapping_add(P0), rbx); rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            7 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14171f3c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x40;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    fl_l = rcx; fl_r = rcx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rax = (crate::simd_ops::ru64(rcx)) as u64; return crate::simd_ops::call_fwd(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
    rax = 0x1001;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14171f3e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = 0x1001;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14171f410(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    fl_l = rcx; fl_r = rcx;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp);
    fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64;
    if !(((((fl_l as u8)) & ((fl_r as u8))) == 0)) { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
    rax = (rax & !0xffu64) | (((0u8) as u8 as u64));
    fl_l = 0; fl_r = 0;
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14171f4e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143932190; const P1: u64 = 0x14171fa40; const P2: u64 = 0x14169edd0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; r14 = rdx; rsi = rcx; crate::simd_ops::lk_add_u32(rcx.wrapping_add(0x10u64), (1) as u32); rax = (crate::simd_ops::ru64(rdx)) as u64; rcx = rdx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rdi = ((rax as u32)) as u32 as u64; rbp = 0; fl_l = 0; fl_r = 0; r8 = ((rbp as u32)) as u32 as u64; r9 = (crate::simd_ops::ru32(rsi.wrapping_add(0x20u64))) as u32 as u64; fl_l = ((r9 as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 6; continue; } bb = 1; continue; }
            1 => { rdx = (crate::simd_ops::ru64(rsi.wrapping_add(0x18u64))) as u64; bb = 2; continue; }
            2 => { rcx = ((r8 as u32)) as u32 as u64; rbx = (crate::simd_ops::ru64(rdx.wrapping_add(rcx.wrapping_mul(8)))) as u64; fl_l = (crate::simd_ops::ru32(rbx)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 5; continue; } bb = 3; continue; }
            3 => { r8 = (((r8 as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 2; continue; } bb = 4; continue; }
            4 => { bb = 6; continue; }
            5 => { fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 16; continue; } bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru64(P0)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; r8 = 0; fl_l = 0; fl_r = 0; rdx = ((r8.wrapping_add(0x20u64)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x48u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rbx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { crate::simd_ops::wu32(rax, ((rdi as u32)) as u32); crate::simd_ops::wu64(rax.wrapping_add(0x8u64), rbp); crate::simd_ops::wu64(rax.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rax.wrapping_add(0x18u64), rbp); bb = 9; continue; }
            8 => { rbx = rbp; bb = 9; continue; }
            9 => { rsi = ((rsi).wrapping_add(0x18u64)) as u64; rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; rdi = rax.wrapping_add(0x1); rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x10u64))) as u64; fl_l = rdi; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 12; continue; } bb = 10; continue; }
            10 => { rcx = ((rcx).wrapping_shr(1u32)) as u64; fl_l = rdi; fl_r = rcx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 15; continue; } bb = 11; continue; }
            11 => { r8 = rdi; bb = 14; continue; }
            12 => { fl_l = rdi; fl_r = rcx; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 15; continue; } bb = 13; continue; }
            13 => { r8 = rdi; r8 = ((r8).wrapping_shr(2u32)) as u64; fl_l = ((r8).wrapping_add(rdi)) as u64; fl_r = fl_l; r8 = ((fl_l as u64)) as u64; bb = 14; continue; }
            14 => { rdx = rsi; rcx = rsi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 15; continue; }
            15 => { crate::simd_ops::wu64(rsi.wrapping_add(0x8u64), rdi); rax = (crate::simd_ops::ru64(rsi)) as u64; crate::simd_ops::wu64(rax.wrapping_add(rdi.wrapping_mul(8)).wrapping_sub(0x8u64), rbx); bb = 16; continue; }
            16 => { crate::simd_ops::lk_add_u32(r14.wrapping_add(0x8u64), (1) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), r14); rdi = rbx.wrapping_add(0x8); rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; rbp = rax.wrapping_add(0x1); fl_l = rbp; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 23; continue; } bb = 17; continue; }
            17 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rbx = rbp.wrapping_sub(0x2); rbx = ((rax.wrapping_add(rbx.wrapping_mul(8))) as u64) as u64; rsi = (0xffffffffffffffffu64) as u64; bb = 18; continue; }
            18 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 20; continue; } bb = 19; continue; }
            19 => { rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 20; continue; }
            20 => { rbx = ((rbx).wrapping_sub(0x8u64)) as u64; fl_l = rsi; fl_r = (0x1u64) as u64; rsi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 18; continue; } bb = 21; continue; }
            21 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; rax = ((rax).wrapping_shr(1u32)) as u64; fl_l = rbp; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 26; continue; } bb = 22; continue; }
            22 => { r8 = rbp; bb = 25; continue; }
            23 => { fl_l = rbp; fl_r = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 26; continue; } bb = 24; continue; }
            24 => { r8 = rbp; r8 = ((r8).wrapping_shr(2u32)) as u64; fl_l = ((r8).wrapping_add(rbp)) as u64; fl_r = fl_l; r8 = ((fl_l as u64)) as u64; bb = 25; continue; }
            25 => { rdx = rdi; rcx = rdi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 26; continue; }
            26 => { crate::simd_ops::wu64(rdi.wrapping_add(0x8u64), rbp); rax = (crate::simd_ops::ru64(rdi)) as u64; crate::simd_ops::lk_add_u32(r14.wrapping_add(0x8u64), (1) as u32); crate::simd_ops::wu64(rax.wrapping_add(rbp.wrapping_mul(8)).wrapping_sub(0x8u64), r14); rcx = r14; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14171f650(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x20; const P1: u64 = 0x18; const P2: u64 = 0x8; const P3: u64 = 0x1416bb5e0; const P4: u64 = 0x143932190; const P5: u64 = 0x60; const P6: u64 = 0x28;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rcx; rbp = 0; fl_l = 0; fl_r = 0; rdi = ((rbp as u32)) as u32 as u64; fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 4; continue; } bb = 1; continue; }
            1 => { rcx = ((rdi as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64; rsi = (crate::simd_ops::ru64(rax.wrapping_add(rcx.wrapping_mul(8)))) as u64; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rcx = ((rsi.wrapping_add(P2)) as u64) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(P4)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = rsi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P5))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 3; continue; }
            3 => { rdi = (((rdi as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((rdi as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rbx.wrapping_add(P0))) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 1; continue; } bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 10; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(P6))) as u64; fl_l = rax; fl_r = (0xfffffffffffffffeu64) as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 6; continue; }
            6 => { fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 10; continue; } bb = 7; continue; }
            7 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { rcx = (crate::simd_ops::ru64(P4)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P5))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(P1), rbp); bb = 9; continue; }
            9 => { crate::simd_ops::wu64(rbx.wrapping_add(P6), rbp); bb = 10; continue; }
            10 => { crate::simd_ops::wu64(rbx.wrapping_add(P0), rbp); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14171f790(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x4;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = (crate::simd_ops::ru32(r8)) as u32 as u64;
    rax = (((rax as u32)).swap_bytes()) as u32 as u64;
    crate::simd_ops::wu32(r8, ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(r8.wrapping_add(P0))) as u32 as u64;
    rax = (((rax as u32)).swap_bytes()) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P0), ((rax as u32)) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14171f7b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x20; const P1: u64 = 0x18;
    let (mut rax, mut r10, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r9 = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; rax = 0; fl_l = 0; fl_r = 0; fl_l = ((r9 as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { r10 = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; bb = 2; continue; }
            2 => { r8 = (crate::simd_ops::ru64(r10.wrapping_add(rax.wrapping_mul(8)))) as u64; fl_l = (crate::simd_ops::ru32(r8)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 5; continue; } bb = 3; continue; }
            3 => { rax = (((rax as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 2; continue; } bb = 4; continue; }
            4 => { rax = 0; fl_l = 0; fl_r = 0; return rax; }
            5 => { rax = r8; return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14171f7e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = 0x500;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14171f800(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1417355d0; const P1: u64 = 0x1416a32e0; const P2: u64 = 0x14169edd0; const P3: u64 = 0x14171f4e0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 17>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((r9 as u32)) as u32); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x50u64)) as u64; rbp = r8; r12 = rdx; r13 = rcx; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu8(rdx, ((r8 as u8)) as u8); bb = 33; continue; }
            2 => { xmm0 = 0u128; rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu128(rsp.wrapping_add(0x38u64), (xmm0) as u128); crate::simd_ops::wu32(rsp.wrapping_add(0x48u64), ((rax as u32)) as u32); rdi = r8.wrapping_add(0x14); rdx = (rdx & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; crate::simd_ops::wu8(rsp.wrapping_add(0xa0u64), ((rdx as u8)) as u8); rax = (0x46676741626f6c42u64) as u64; fl_l = (crate::simd_ops::ru64(r8)) as u64; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 32; continue; } bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru32(r8.wrapping_add(0x8u64))) as u32 as u64; fl_r = 0x656c69; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 32; continue; } bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::ru32(r8.wrapping_add(0xcu64))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 6; continue; } bb = 5; continue; }
            5 => { crate::simd_ops::wu32(r8.wrapping_add(0xcu64), (0x0u32) as u32); rax = (crate::simd_ops::ru32(r8.wrapping_add(0x10u64))) as u32 as u64; rax = (((rax as u32)).swap_bytes()) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(0x10u64), ((rax as u32)) as u32); rdx = (rdx & !0xffu64) | (((0x1u8) as u8 as u64)); crate::simd_ops::wu8(rsp.wrapping_add(0xa0u64), ((rdx as u8)) as u8); bb = 7; continue; }
            6 => { rax = (crate::simd_ops::ru32(r8.wrapping_add(0x10u64))) as u32 as u64; bb = 7; continue; }
            7 => { r15 = 0; fl_l = 0; fl_r = 0; r14 = 0; fl_l = 0; fl_r = 0; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 31; continue; } bb = 8; continue; }
            8 => { r8 = 0x3; xmm0 = 0u128; crate::simd_ops::wu128(rsp.wrapping_add(0x28u64), (xmm0) as u128); fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 32; continue; } bb = 9; continue; }
            9 => { rsi = rdi; rdi = ((rdi).wrapping_add(0x10u64)) as u64; rcx = (crate::simd_ops::ru32(rsi)) as u32 as u64; fl_l = ((rdx as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 11; continue; } bb = 10; continue; }
            10 => { rcx = (((rcx as u32)).swap_bytes()) as u32 as u64; crate::simd_ops::wu32(rsi, ((rcx as u32)) as u32); rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x4u64))) as u32 as u64; rax = (((rax as u32)).swap_bytes()) as u32 as u64; crate::simd_ops::wu32(rsi.wrapping_add(0x4u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x8u64))) as u32 as u64; rax = (((rax as u32)).swap_bytes()) as u32 as u64; crate::simd_ops::wu32(rsi.wrapping_add(0x8u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rsi.wrapping_add(0xcu64))) as u32 as u64; rax = (((rax as u32)).swap_bytes()) as u32 as u64; crate::simd_ops::wu32(rsi.wrapping_add(0xcu64), ((rax as u32)) as u32); bb = 11; continue; }
            11 => { rdx = 0; fl_l = 0; fl_r = 0; fl_l = ((r9 as u32)) as u32 as u64; fl_r = 0x1; rdx = (if (((fl_l as u32)) == ((fl_r as u32))) { (r8 as u32) } else { (rdx as u32) }) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 14; continue; } bb = 12; continue; }
            12 => { fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 27; continue; } bb = 13; continue; }
            13 => { rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x8u64))) as u32 as u64; fl_l = ((rdi).wrapping_add(rax)) as u64; fl_r = fl_l; rdi = ((fl_l as u64)) as u64; bb = 27; continue; }
            14 => { rax = (crate::simd_ops::ru64(rax)) as u64; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru32(rsi.wrapping_add(0x4u64))) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 32; continue; } bb = 15; continue; }
            15 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = 0xc1; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x30u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = rax; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 16; continue; }
            16 => { r9 = (crate::simd_ops::ru32(rsi.wrapping_add(0xcu64))) as u32 as u64; r8 = (crate::simd_ops::ru32(rsi.wrapping_add(0x8u64))) as u32 as u64; rdx = rdi; rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 17; continue; }
            17 => { r8 = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 30; continue; } bb = 18; continue; }
            18 => { fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 29; continue; } bb = 19; continue; }
            19 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x18u64))) as u32 as u64; rdi = ((rdi).wrapping_add(rax)) as u64; fl_l = (crate::simd_ops::ru8(rsp.wrapping_add(0xa0u64))) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 21; continue; } bb = 20; continue; }
            20 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x20u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 21; continue; }
            21 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x500; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 26; continue; } bb = 22; continue; }
            22 => { r15 = (((r15 as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((r15 as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 26; continue; } bb = 23; continue; }
            23 => { crate::simd_ops::lk_add_u32(rbx.wrapping_add(0x8u64), (1) as u32); rcx = (crate::simd_ops::ru64(r13.wrapping_add(0x30u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 25; continue; } bb = 24; continue; }
            24 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 25; continue; }
            25 => { crate::simd_ops::wu64(r13.wrapping_add(0x30u64), rbx); rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 27; continue; }
            26 => { rdx = rbx; rcx = r13; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 27; continue; }
            27 => { r14 = (((r14 as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((r14 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rbp.wrapping_add(0x10u64))) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 31; continue; } bb = 28; continue; }
            28 => { rdx = ((crate::simd_ops::ru8(rsp.wrapping_add(0xa0u64))) as u32) as u32 as u64; r9 = (crate::simd_ops::ru32(rsp.wrapping_add(0xa8u64))) as u32 as u64; bb = 8; continue; }
            29 => { crate::simd_ops::wu8(r12, (0x0u8) as u8); rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 33; continue; }
            30 => { crate::simd_ops::wu8(r12, (0x0u8) as u8); rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 33; continue; }
            31 => { crate::simd_ops::wu8(r12, (0x1u8) as u8); bb = 33; continue; }
            32 => { crate::simd_ops::wu8(r12, (0x0u8) as u8); bb = 33; continue; }
            33 => { rax = r12; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; rsp = ((rsp).wrapping_add(0x50u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14171fa40(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x143932190; const P2: u64 = 0x60; const P3: u64 = 0x3; const P4: u64 = 0x50; const P5: u64 = 0x48;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rcx; fl_l = r8; fl_r = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 8; continue; } bb = 1; continue; }
            1 => { rdx = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rdi); fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 4; continue; } bb = 2; continue; }
            2 => { rdi = 0; fl_l = 0; fl_r = 0; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(P1)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx, rdi); crate::simd_ops::wu64(rbx.wrapping_add(P0), rdi); rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            4 => { rcx = (crate::simd_ops::ru64(P1)) as u64; rdi = ((r8.wrapping_add(P3)) as u64) as u64; rdi = (((rdi) & (0xfffffffffffffffcu64))) as u64; r9 = ((rdi.wrapping_mul(8)) as u64) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rcx)) as u64; r8 = r9; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P4))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx, rax); crate::simd_ops::wu64(rbx.wrapping_add(P0), rdi); rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            6 => { crate::simd_ops::wu32(rsp.wrapping_add(0x48u64), (0xc1u32) as u32); r8 = rsp.wrapping_add(0x48); rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = r9; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P5))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx, rax); bb = 7; continue; }
            7 => { crate::simd_ops::wu64(rbx.wrapping_add(P0), rdi); rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; bb = 8; continue; }
            8 => { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14171faf0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14169e740; const P1: u64 = 0x141726240;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = 0; fl_l = 0; fl_r = 0; r14 = r8; r12 = rdx; r15 = rcx; fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(0x20u64))) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 15; continue; } bb = 1; continue; }
            1 => { bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(r15.wrapping_add(0x18u64))) as u64; rdx = ((rbx.wrapping_add(rbx.wrapping_mul(2))) as u64) as u64; r11 = (crate::simd_ops::ru32(r14.wrapping_add(0x10u64))) as u32 as u64; rbp = r14.wrapping_add(0x14); r9 = 0; fl_l = 0; fl_r = 0; rdi = ((rax.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64; rax = (((crate::simd_ops::ru32(r14.wrapping_add(0x14u64))) as i32 as i64) as u64) as u64; rbp = ((rbp).wrapping_add(rax)) as u64; fl_l = ((r11 as u32)) as u32 as u64; fl_r = ((r11 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 14; continue; } bb = 3; continue; }
            3 => { rsi = (crate::simd_ops::ru64(rdi)) as u64; bb = 4; continue; }
            4 => { r10 = ((r9 as u32)) as u32 as u64; rax = rsi; r10 = ((r10).wrapping_shl(4u32)) as u64; r10 = ((r10).wrapping_add(rbp)) as u64; r8 = (((crate::simd_ops::ru32(r10.wrapping_add(0x4u64))) as i32 as i64) as u64) as u64; r8 = ((r8).wrapping_add(0x4u64)) as u64; r8 = ((r8).wrapping_add(r10)) as u64; fl_l = r8; fl_r = rsi; r8 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; bb = 5; continue; }
            5 => { rdx = ((crate::simd_ops::ru8(rax)) as u32) as u32 as u64; rcx = ((crate::simd_ops::ru8(rax.wrapping_add(r8))) as u32) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; rdx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rax = ((rax).wrapping_add(1)) as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 5; continue; } bb = 7; continue; }
            7 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 10; continue; } bb = 8; continue; }
            8 => { r9 = (((r9 as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((r9 as u32)) as u32 as u64; fl_r = ((r11 as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 4; continue; } bb = 9; continue; }
            9 => { bb = 14; continue; }
            10 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 11; continue; }
            11 => { rax = (((crate::simd_ops::ru32(r10.wrapping_add(0xcu64))) as i32 as i64) as u64) as u64; rdx = r10.wrapping_add(0xc); rdx = ((rdx).wrapping_add(rax)) as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); bb = 14; continue; }
            12 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 13; continue; }
            13 => { rax = (((crate::simd_ops::ru32(r10.wrapping_add(0xcu64))) as i32 as i64) as u64) as u64; rdx = r10.wrapping_add(0xc); rdx = ((rdx).wrapping_add(rax)) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 14; continue; }
            14 => { rbx = (((rbx as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((rbx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(r15.wrapping_add(0x20u64))) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 2; continue; } bb = 15; continue; }
            15 => { rdi = 0; fl_l = 0; fl_r = 0; fl_l = (crate::simd_ops::ru32(r15.wrapping_add(0x8u64))) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 29; continue; } bb = 16; continue; }
            16 => { bb = 17; continue; }
            17 => { rax = (crate::simd_ops::ru64(r15)) as u64; rcx = ((rdi.wrapping_add(rdi.wrapping_mul(2))) as u64) as u64; rsi = r14.wrapping_add(0xc); r10 = 0; fl_l = 0; fl_r = 0; rbx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; rax = (((crate::simd_ops::ru32(r14.wrapping_add(0xcu64))) as i32 as i64) as u64) as u64; rsi = ((rsi).wrapping_add(rax)) as u64; fl_l = (crate::simd_ops::ru32(r14.wrapping_add(0x8u64))) as u32 as u64; fl_r = ((r10 as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 28; continue; } bb = 18; continue; }
            18 => { bb = 19; continue; }
            19 => { r11 = (crate::simd_ops::ru32(rbx)) as u32 as u64; r8 = ((r10 as u32)) as u32 as u64; r8 = ((r8).wrapping_shl(4u32)) as u64; r8 = ((r8).wrapping_add(rsi)) as u64; fl_l = (crate::simd_ops::ru32(r8.wrapping_add(0x8u64))) as u32 as u64; fl_r = ((r11 as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 27; continue; } bb = 20; continue; }
            20 => { r9 = (((crate::simd_ops::ru32(r8.wrapping_add(0x4u64))) as i32 as i64) as u64) as u64; rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; r9 = ((r9).wrapping_add(0x4u64)) as u64; r9 = ((r9).wrapping_add(r8)) as u64; fl_l = r9; fl_r = rax; r9 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; bb = 21; continue; }
            21 => { rdx = ((crate::simd_ops::ru8(rax)) as u32) as u32 as u64; rcx = ((crate::simd_ops::ru8(rax.wrapping_add(r9))) as u32) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; rdx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 23; continue; } bb = 22; continue; }
            22 => { rax = ((rax).wrapping_add(1)) as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 21; continue; } bb = 23; continue; }
            23 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 27; continue; } bb = 24; continue; }
            24 => { rax = (crate::simd_ops::ru32(r8.wrapping_add(0xcu64))) as u32 as u64; rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; fl_l = ((r11 as u32)) as u32 as u64; fl_r = 0x4; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 26; continue; } bb = 25; continue; }
            25 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; rax = (rax & !0xffu64) | (((if (((fl_l as u32)) != ((fl_r as u32))) { 1u8 } else { 0u8 }) as u8 as u64)); crate::simd_ops::wu8(rcx, ((rax as u8)) as u8); bb = 27; continue; }
            26 => { crate::simd_ops::wu32(rcx, ((rax as u32)) as u32); bb = 27; continue; }
            27 => { r10 = (((r10 as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((r10 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(r14.wrapping_add(0x8u64))) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 19; continue; } bb = 28; continue; }
            28 => { rdi = (((rdi as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((rdi as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(r15.wrapping_add(0x8u64))) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 17; continue; } bb = 29; continue; }
            29 => { rbp = 0; fl_l = 0; fl_r = 0; fl_l = (crate::simd_ops::ru32(r15.wrapping_add(0x38u64))) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 47; continue; } bb = 30; continue; }
            30 => { bb = 31; continue; }
            31 => { rax = (((crate::simd_ops::ru32(r14.wrapping_add(0x1cu64))) as i32 as i64) as u64) as u64; rbx = ((rbp.wrapping_mul(2)) as u64) as u64; r11 = (crate::simd_ops::ru32(r14.wrapping_add(0x18u64))) as u32 as u64; rsi = r14.wrapping_add(0x1c); rbx = ((rbx).wrapping_add(rbp)) as u64; rsi = ((rsi).wrapping_add(rax)) as u64; rbx = ((rbx).wrapping_shl(4u32)) as u64; r10 = 0; fl_l = 0; fl_r = 0; rbx = ((rbx).wrapping_add(crate::simd_ops::ru64(r15.wrapping_add(0x30u64)))) as u64; fl_l = ((r11 as u32)) as u32 as u64; fl_r = ((r11 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 46; continue; } bb = 32; continue; }
            32 => { rdi = (crate::simd_ops::ru32(rbx)) as u32 as u64; bb = 33; continue; }
            33 => { rax = ((r10 as u32)) as u32 as u64; r9 = (((rax) as i64).wrapping_mul((0x1cu64) as i64) as u64) as u64; r9 = ((r9).wrapping_add(rsi)) as u64; fl_l = (crate::simd_ops::ru32(r9.wrapping_add(0x10u64))) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 38; continue; } bb = 34; continue; }
            34 => { r8 = (((crate::simd_ops::ru32(r9.wrapping_add(0x4u64))) as i32 as i64) as u64) as u64; rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; r8 = ((r8).wrapping_add(0x4u64)) as u64; r8 = ((r8).wrapping_add(r9)) as u64; fl_l = r8; fl_r = rax; r8 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; bb = 35; continue; }
            35 => { rdx = ((crate::simd_ops::ru8(rax)) as u32) as u32 as u64; rcx = ((crate::simd_ops::ru8(rax.wrapping_add(r8))) as u32) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; rdx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 37; continue; } bb = 36; continue; }
            36 => { rax = ((rax).wrapping_add(1)) as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 35; continue; } bb = 37; continue; }
            37 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 40; continue; } bb = 38; continue; }
            38 => { r10 = (((r10 as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((r10 as u32)) as u32 as u64; fl_r = ((r11 as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 33; continue; } bb = 39; continue; }
            39 => { bb = 46; continue; }
            40 => { rax = (((crate::simd_ops::ru32(r9.wrapping_add(0xcu64))) as i32 as i64) as u64) as u64; r11 = r9.wrapping_add(0x14); r10 = (crate::simd_ops::ru32(rbx.wrapping_add(0x20u64))) as u32 as u64; rcx = r9.wrapping_add(0xc); rcx = ((rcx).wrapping_add(rax)) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), rcx); fl_l = ((r10 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(r11)) as u32 as u64; r10 = (if (((fl_l as u32)) >= ((fl_r as u32))) { crate::simd_ops::ru32(r11) } else { (r10 as u32) }) as u32 as u64; rdx = 0; fl_l = 0; fl_r = 0; fl_l = ((r10 as u32)) as u32 as u64; fl_r = ((r10 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 46; continue; } bb = 41; continue; }
            41 => { r8 = 0; fl_l = 0; fl_r = 0; r9 = 0; fl_l = 0; fl_r = 0; bb = 42; continue; }
            42 => { rax = (((crate::simd_ops::ru32(r11.wrapping_add(0x4u64))) as i32 as i64) as u64) as u64; rax = ((rax).wrapping_add(r9)) as u64; fl_l = (crate::simd_ops::ru32(rbx)) as u32 as u64; fl_r = 0x4; rcx = (crate::simd_ops::ru32(rax.wrapping_add(r11).wrapping_add(0x4u64))) as u32 as u64; rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(r8))) as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 44; continue; } bb = 43; continue; }
            43 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; rcx = (rcx & !0xffu64) | (((if (((fl_l as u32)) != ((fl_r as u32))) { 1u8 } else { 0u8 }) as u8 as u64)); crate::simd_ops::wu8(rax, ((rcx as u8)) as u8); bb = 45; continue; }
            44 => { crate::simd_ops::wu32(rax, ((rcx as u32)) as u32); bb = 45; continue; }
            45 => { rdx = (((rdx as u32)).wrapping_add(1)) as u32 as u64; r9 = ((r9).wrapping_add(0x4u64)) as u64; r8 = ((r8).wrapping_add(0x8u64)) as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((r10 as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 42; continue; } bb = 46; continue; }
            46 => { rbp = (((rbp as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((rbp as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(r15.wrapping_add(0x38u64))) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 31; continue; } bb = 47; continue; }
            47 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rax = r12; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; crate::simd_ops::wu8(r12, (0x1u8) as u8); rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14171fdb0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x4; const P1: u64 = 0x8; const P2: u64 = 0xc; const P3: u64 = 0x10; const P4: u64 = 0x14; const P5: u64 = 0x18;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { xmm0 = ((crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32) as u128; fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; if (f32::from_bits((fl_l as u32)) != f32::from_bits((fl_r as u32)) && !f32::from_bits((fl_l as u32)).is_nan() && !f32::from_bits((fl_r as u32)).is_nan()) { bb = 8; continue; } bb = 1; continue; }
            1 => { xmm0 = ((crate::simd_ops::ru32(rdx)) as u32) as u128; fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rcx)) as u32 as u64; if (f32::from_bits((fl_l as u32)) != f32::from_bits((fl_r as u32)) && !f32::from_bits((fl_l as u32)).is_nan() && !f32::from_bits((fl_r as u32)).is_nan()) { bb = 8; continue; } bb = 2; continue; }
            2 => { xmm0 = ((crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32) as u128; fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64; if (f32::from_bits((fl_l as u32)) != f32::from_bits((fl_r as u32)) && !f32::from_bits((fl_l as u32)).is_nan() && !f32::from_bits((fl_r as u32)).is_nan()) { bb = 8; continue; } bb = 3; continue; }
            3 => { xmm0 = ((crate::simd_ops::ru32(rdx.wrapping_add(P2))) as u32) as u128; fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rcx.wrapping_add(P2))) as u32 as u64; if (f32::from_bits((fl_l as u32)) != f32::from_bits((fl_r as u32)) && !f32::from_bits((fl_l as u32)).is_nan() && !f32::from_bits((fl_r as u32)).is_nan()) { bb = 8; continue; } bb = 4; continue; }
            4 => { xmm0 = ((crate::simd_ops::ru32(rdx.wrapping_add(P3))) as u32) as u128; fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rcx.wrapping_add(P3))) as u32 as u64; if (f32::from_bits((fl_l as u32)) != f32::from_bits((fl_r as u32)) && !f32::from_bits((fl_l as u32)).is_nan() && !f32::from_bits((fl_r as u32)).is_nan()) { bb = 8; continue; } bb = 5; continue; }
            5 => { xmm0 = ((crate::simd_ops::ru32(rdx.wrapping_add(P4))) as u32) as u128; fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rcx.wrapping_add(P4))) as u32 as u64; if (f32::from_bits((fl_l as u32)) != f32::from_bits((fl_r as u32)) && !f32::from_bits((fl_l as u32)).is_nan() && !f32::from_bits((fl_r as u32)).is_nan()) { bb = 8; continue; } bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru32(rdx.wrapping_add(P5))) as u32 as u64; fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(P5))) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 8; continue; } bb = 7; continue; }
            7 => { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            8 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14171fe00(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x4; const P1: u64 = 0x10; const P2: u64 = 0x8; const P3: u64 = 0x14; const P4: u64 = 0xc; const P5: u64 = 0x18; const P6: u64 = 0x1416b1650;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = (crate::simd_ops::ru32(rcx)) as u32 as u64;
    crate::simd_ops::wu32(rdx, ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64;
    crate::simd_ops::wu32(rdx.wrapping_add(P0), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64;
    crate::simd_ops::wu32(rdx.wrapping_add(P2), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rcx.wrapping_add(P3))) as u32 as u64;
    crate::simd_ops::wu32(rdx.wrapping_add(P4), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rcx.wrapping_add(P5))) as u32 as u64;
    rcx = rdx;
    crate::simd_ops::wu32(rdx.wrapping_add(P1), ((rax as u32)) as u32);
    return crate::simd_ops::call_fwd(P6, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14171fe30(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x8; const P2: u64 = 0x14; const P3: u64 = 0xc; const P4: u64 = 0x18;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x7fffffff; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            2 => { xmm0 = ((crate::simd_ops::ru32(rcx)) as u32) as u128; fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rdx)) as u32 as u64; if (f32::from_bits((fl_l as u32)) != f32::from_bits((fl_r as u32)) && !f32::from_bits((fl_l as u32)).is_nan() && !f32::from_bits((fl_r as u32)).is_nan()) { bb = 6; continue; } bb = 3; continue; }
            3 => { xmm0 = ((crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32) as u128; fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32 as u64; if (f32::from_bits((fl_l as u32)) != f32::from_bits((fl_r as u32)) && !f32::from_bits((fl_l as u32)).is_nan() && !f32::from_bits((fl_r as u32)).is_nan()) { bb = 6; continue; } bb = 4; continue; }
            4 => { xmm0 = ((crate::simd_ops::ru32(rcx.wrapping_add(P2))) as u32) as u128; fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rdx.wrapping_add(P3))) as u32 as u64; if (f32::from_bits((fl_l as u32)) != f32::from_bits((fl_r as u32)) && !f32::from_bits((fl_l as u32)).is_nan() && !f32::from_bits((fl_r as u32)).is_nan()) { bb = 6; continue; } bb = 5; continue; }
            5 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rcx.wrapping_add(P4))) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 1; continue; } bb = 6; continue; }
            6 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14171fe70(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143426978; const P1: u64 = 0x1411ffee0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm4: u128 = 0; let mut xmm5: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; xmm5 = ((crate::simd_ops::ru32(r9.wrapping_add(0x70u64))) as u32) as u128; rdi = r9.wrapping_add(0x58); rsi = rcx; xmm1 = 0u128; xmm0 = 0u128; xmm5 = (xmm5 & !0xffff_ffffu128) | ((f32::from_bits(xmm5 as u32) + f32::from_bits(crate::simd_ops::ru32(rdi))).to_bits() as u128); xmm5 = (xmm5 & !0xffff_ffffu128) | ((f32::from_bits(xmm5 as u32) * f32::from_bits(crate::simd_ops::ru32(P0))).to_bits() as u128); fl_l = ((r8 as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { xmm1 = ((crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32) as u128; bb = 2; continue; }
            2 => { fl_l = ((r8 as u8)) as u8 as u64; fl_r = (0x2u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { xmm0 = ((crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32) as u128; bb = 4; continue; }
            4 => { fl_l = ((r8 as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(crate::simd_ops::ru32(rdi.wrapping_add(0x4u64)))).to_bits() as u128); bb = 6; continue; }
            6 => { xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) + f32::from_bits(crate::simd_ops::ru32(rdx.wrapping_add(0x14u64)))).to_bits() as u128); xmm2 = ((crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32) as u128; r8 = rcx.wrapping_add(0xc); xmm4 = (xmm5) as u128; xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) - f32::from_bits(xmm5 as u32)).to_bits() as u128); xmm4 = (xmm4 & !0xffff_ffffu128) | ((f32::from_bits(xmm4 as u32) + f32::from_bits(crate::simd_ops::ru32(rdx.wrapping_add(0xcu64)))).to_bits() as u128); xmm3 = (xmm5) as u128; xmm3 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) + f32::from_bits(crate::simd_ops::ru32(rdx.wrapping_add(0x10u64)))).to_bits() as u128); crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((xmm1 as u32)) as u32); xmm1 = ((crate::simd_ops::ru32(rdx.wrapping_add(0x8u64))) as u32) as u128; xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) - f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm0 = ((crate::simd_ops::ru32(rdx)) as u32) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) - f32::from_bits(xmm5 as u32)).to_bits() as u128); rdx = rsp.wrapping_add(0x40); crate::simd_ops::wu32(rcx, ((xmm0 as u32)) as u32); xmm0 = (xmm4) as u128; crate::simd_ops::wu32(rcx.wrapping_add(0x4u64), ((xmm2 as u32)) as u32); crate::simd_ops::wu32(rcx.wrapping_add(0x8u64), ((xmm1 as u32)) as u32); rcx = rdi; rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x28u64))) as u32 as u64; xmm0 = crate::simd_ops::vunpcklps(xmm0, xmm3); crate::simd_ops::wu64(r8, ((xmm0 as u64)) as u64); crate::simd_ops::wu32(r8.wrapping_add(0x8u64), ((rax as u32)) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = rsi; rdx = rsp.wrapping_add(0x48); rcx = rdi; rbx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(rax)) as u64; crate::simd_ops::wu64(rsi.wrapping_add(0x18u64), rcx); rdx = (crate::simd_ops::ru64(rbx)) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rax = ((rdx as u32)) as u32 as u64; crate::simd_ops::wu64(rsi.wrapping_add(0x20u64), rdx); rax = (((rax as u32)).wrapping_sub((rcx as u32))) as u32 as u64; rcx = ((rcx).wrapping_shr(32u32)) as u64; rax = (((rax as u32)).wrapping_add(1)) as u32 as u64; rdx = ((rdx).wrapping_shr(32u32)) as u64; rdx = (((rdx as u32)).wrapping_sub((rcx as u32))) as u32 as u64; crate::simd_ops::wu32(rsi.wrapping_add(0x28u64), ((rax as u32)) as u32); rdx = (((rdx as u32)).wrapping_add(1)) as u32 as u64; crate::simd_ops::wu32(rsi.wrapping_add(0x2cu64), ((rdx as u32)) as u32); rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14171ff70(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143426978; const P1: u64 = 0x1411ffee0;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm4: u128 = 0; let mut xmm5: u128 = 0; let mut xmm6: u128 = 0; let mut xmm7: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 17>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r11 = rsp; crate::simd_ops::wu64(r11.wrapping_add(0x10u64), rdx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = ((rsp).wrapping_sub(0x78u64)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; r12 = rcx; rbp = ((r8 as u32)) as u32 as u64; rcx = (crate::simd_ops::ru32(rax.wrapping_add(0x20u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 13; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(r11.wrapping_sub(0x18u64), rsi); crate::simd_ops::wu64(r11.wrapping_sub(0x20u64), rdi); crate::simd_ops::wu64(r11.wrapping_sub(0x28u64), r13); r13 = ((rcx as u32)) as u32 as u64; crate::simd_ops::wu64(r11.wrapping_sub(0x30u64), r14); r14 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(r11.wrapping_sub(0x38u64), r15); r15 = ((r14 as u32)) as u32 as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm6) as u128); xmm6 = 0u128; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm7) as u128); xmm7 = ((crate::simd_ops::ru32(P0)) as u32) as u128; crate::simd_ops::wu64(r11.wrapping_add(0x18u64), rbx); bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(r12)) as u64; rdi = (crate::simd_ops::ru64(r12.wrapping_add(0x8u64))) as u64; rdi = ((rdi).wrapping_add(r14)) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; rsi = (crate::simd_ops::ru64(r15.wrapping_add(rcx))) as u64; fl_l = (crate::simd_ops::ru32(rsi.wrapping_add(0x68u64))) as u32 as u64; fl_r = 0x7fffffff; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 10; continue; } bb = 3; continue; }
            3 => { xmm5 = ((crate::simd_ops::ru32(rsi.wrapping_add(0x70u64))) as u32) as u128; xmm1 = (xmm6) as u128; xmm0 = (xmm6) as u128; xmm5 = (xmm5 & !0xffff_ffffu128) | ((f32::from_bits(xmm5 as u32) + f32::from_bits(crate::simd_ops::ru32(rsi.wrapping_add(0x58u64)))).to_bits() as u128); xmm5 = (xmm5 & !0xffff_ffffu128) | ((f32::from_bits(xmm5 as u32) * f32::from_bits(xmm7 as u32)).to_bits() as u128); fl_l = ((rbp as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { xmm1 = ((crate::simd_ops::ru32(rsi.wrapping_add(0x60u64))) as u32) as u128; bb = 5; continue; }
            5 => { fl_l = ((rbp as u8)) as u8 as u64; fl_r = (0x2u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { xmm0 = ((crate::simd_ops::ru32(rsi.wrapping_add(0x60u64))) as u32) as u128; bb = 7; continue; }
            7 => { fl_l = ((rbp as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(crate::simd_ops::ru32(rsi.wrapping_add(0x5cu64)))).to_bits() as u128); bb = 9; continue; }
            9 => { xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) + f32::from_bits(crate::simd_ops::ru32(rdx.wrapping_add(0x14u64)))).to_bits() as u128); xmm2 = ((crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32) as u128; r8 = rdi.wrapping_add(0xc); xmm4 = (xmm5) as u128; xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) - f32::from_bits(xmm5 as u32)).to_bits() as u128); xmm4 = (xmm4 & !0xffff_ffffu128) | ((f32::from_bits(xmm4 as u32) + f32::from_bits(crate::simd_ops::ru32(rdx.wrapping_add(0xcu64)))).to_bits() as u128); xmm3 = (xmm5) as u128; rcx = rsi.wrapping_add(0x58); xmm3 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) + f32::from_bits(crate::simd_ops::ru32(rdx.wrapping_add(0x10u64)))).to_bits() as u128); crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((xmm1 as u32)) as u32); xmm1 = ((crate::simd_ops::ru32(rdx.wrapping_add(0x8u64))) as u32) as u128; xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) - f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm0 = ((crate::simd_ops::ru32(rdx)) as u32) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) - f32::from_bits(xmm5 as u32)).to_bits() as u128); rdx = rsp.wrapping_add(0x90); crate::simd_ops::wu32(rdi, ((xmm0 as u32)) as u32); xmm0 = (xmm4) as u128; crate::simd_ops::wu32(rdi.wrapping_add(0x4u64), ((xmm2 as u32)) as u32); crate::simd_ops::wu32(rdi.wrapping_add(0x8u64), ((xmm1 as u32)) as u32); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x28u64))) as u32 as u64; xmm0 = crate::simd_ops::vunpcklps(xmm0, xmm3); crate::simd_ops::wu64(r8, ((xmm0 as u64)) as u64); crate::simd_ops::wu32(r8.wrapping_add(0x8u64), ((rax as u32)) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = rdi; rdx = rsp.wrapping_add(0xa8); rcx = rsi.wrapping_add(0x58); rbx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(rax)) as u64; crate::simd_ops::wu64(rdi.wrapping_add(0x18u64), rcx); rax = rcx; rdx = (crate::simd_ops::ru64(rbx)) as u64; r8 = rdx; rax = ((rax).wrapping_shr(32u32)) as u64; r8 = ((r8).wrapping_shr(32u32)) as u64; r8 = (((r8 as u32)).wrapping_sub((rax as u32))) as u32 as u64; crate::simd_ops::wu64(rdi.wrapping_add(0x20u64), rdx); r8 = (((r8 as u32)).wrapping_add(1)) as u32 as u64; rdx = (((rdx as u32)).wrapping_sub((rcx as u32))) as u32 as u64; fl_l = (((rdx as u32)).wrapping_add(1)) as u32 as u64; rdx = ((fl_l as u32)) as u32 as u64; bb = 11; continue; }
            10 => { rdx = 0xffffffff; crate::simd_ops::wu32(rdi, (0x7f7fffffu32) as u32); r8 = ((rdx as u32)) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x4u64), (0x7f7fffffu32) as u32); crate::simd_ops::wu32(rdi.wrapping_add(0x8u64), (0x7f7fffffu32) as u32); crate::simd_ops::wu32(rdi.wrapping_add(0xcu64), (0xff7fffffu32) as u32); crate::simd_ops::wu32(rdi.wrapping_add(0x10u64), (0xff7fffffu32) as u32); crate::simd_ops::wu32(rdi.wrapping_add(0x14u64), (0xff7fffffu32) as u32); crate::simd_ops::wu32(rdi.wrapping_add(0x18u64), (0x7fffffffu32) as u32); crate::simd_ops::wu32(rdi.wrapping_add(0x1cu64), (0x7fffffffu32) as u32); crate::simd_ops::wu32(rdi.wrapping_add(0x20u64), (0x80000000u32) as u32); crate::simd_ops::wu32(rdi.wrapping_add(0x24u64), (0x80000000u32) as u32); bb = 11; continue; }
            11 => { crate::simd_ops::wu32(rdi.wrapping_add(0x28u64), ((rdx as u32)) as u32); r15 = ((r15).wrapping_add(0x8u64)) as u64; r14 = ((r14).wrapping_add(0x30u64)) as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x2cu64), ((r8 as u32)) as u32); rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x98u64))) as u64; fl_l = r13; fl_r = (0x1u64) as u64; r13 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 12; continue; }
            12 => { xmm7 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x40u64))) as u128; r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0xa0u64))) as u64; bb = 13; continue; }
            13 => { rsp = ((rsp).wrapping_add(0x78u64)) as u64; r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141720180(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x14169edd0; const P2: u64 = 0x143932190; const P3: u64 = 0x60; const P4: u64 = 0x20; const P5: u64 = 0x48; const P6: u64 = 0x24; const P7: u64 = 0xffffffffffffffe0; const P8: u64 = 0xffffffffffffffe4; const P9: u64 = 0xffffffffffffffe8; const P10: u64 = 0xffffffffffffffec; const P11: u64 = 0xfffffffffffffff0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = rdx; rbx = rcx; rax = (crate::simd_ops::ru64(rdx)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::lk_add_u32(rax.wrapping_add(P0), (1) as u32)) as u32 as u64; bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru64(rcx)) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rdi)) as u64; crate::simd_ops::wu64(rbx, rax); rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rcx = (crate::simd_ops::ru64(P2)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(P0), (0x0u64) as u64); rax = (crate::simd_ops::ru64(rbx)) as u64; bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru64(P2)) as u64; r9 = (crate::simd_ops::ru64(rcx)) as u64; rax = (crate::simd_ops::ru32(rax.wrapping_add(P4))) as u32 as u64; rdx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rdx = ((rdx).wrapping_shl(4u32)) as u64; r8 = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(r9.wrapping_add(P5))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(P0), rax); rcx = (crate::simd_ops::ru64(rbx)) as u64; rdx = (crate::simd_ops::ru32(rcx.wrapping_add(P4))) as u32 as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 7; continue; }
            7 => { rcx = ((rax.wrapping_add(P6)) as u64) as u64; bb = 8; continue; }
            8 => { crate::simd_ops::wu32(rax, (0x7f7fffffu32) as u32); crate::simd_ops::wu32(rcx.wrapping_add(P7), (0x7f7fffffu32) as u32); crate::simd_ops::wu32(rcx.wrapping_add(P8), (0x7f7fffffu32) as u32); crate::simd_ops::wu32(rcx.wrapping_add(P9), (0xff7fffffu32) as u32); crate::simd_ops::wu32(rcx.wrapping_add(P10), (0xff7fffffu32) as u32); crate::simd_ops::wu32(rcx.wrapping_add(P11), (0xff7fffffu32) as u32); crate::simd_ops::wu32(rcx.wrapping_sub(0xcu64), (0x7fffffffu32) as u32); crate::simd_ops::wu32(rcx.wrapping_sub(0x8u64), (0x7fffffffu32) as u32); crate::simd_ops::wu32(rcx.wrapping_sub(0x4u64), (0x80000000u32) as u32); crate::simd_ops::wu64(rcx, (0xffffffff80000000u64) as u64); crate::simd_ops::wu32(rcx.wrapping_add(0x8u64), (0xffffffffu32) as u32); rax = rax.wrapping_add(0x30); rcx = rcx.wrapping_add(0x30); fl_l = rdx; fl_r = (0x1u64) as u64; rdx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 8; continue; } bb = 9; continue; }
            9 => { rcx = (crate::simd_ops::ru64(rdi)) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 11; continue; }
            11 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141720280(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xffffffffffffffd8; const P1: u64 = 0x10; const P2: u64 = 0xffffffffffffffe0; const P3: u64 = 0xffffffffffffffe8; const P4: u64 = 0xffffffffffffffdc; const P5: u64 = 0xffffffffffffffe4; const P6: u64 = 0xffffffffffffffec; const P7: u64 = 0x141720f80;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut xmm4: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rax = rsp;
    rsp = ((rsp).wrapping_sub(0x48u64)) as u64;
    xmm0 = (crate::simd_ops::ru128(rcx)) as u128;
    rdx = ((rax.wrapping_add(P0)) as u64) as u64;
    r8 = r9;
    xmm3 = ((crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64) as u128;
    xmm4 = (xmm1) as u128;
    crate::simd_ops::wu128(rax.wrapping_add(P0), (xmm0) as u128);
    rcx = ((rcx).wrapping_add(0x18u64)) as u64;
    xmm1 = ((crate::simd_ops::ru32(rax.wrapping_add(P2))) as u32) as u128;
    xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) - f32::from_bits(xmm4 as u32)).to_bits() as u128);
    crate::simd_ops::wu64(rax.wrapping_add(P3), ((xmm3 as u64)) as u64);
    xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) - f32::from_bits(xmm4 as u32)).to_bits() as u128);
    xmm3 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) + f32::from_bits(xmm4 as u32)).to_bits() as u128);
    crate::simd_ops::wu32(rax.wrapping_add(P0), ((xmm0 as u32)) as u32);
    xmm0 = ((crate::simd_ops::ru32(rax.wrapping_add(P4))) as u32) as u128;
    xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) - f32::from_bits(xmm4 as u32)).to_bits() as u128);
    crate::simd_ops::wu32(rax.wrapping_add(P3), ((xmm3 as u32)) as u32);
    xmm3 = (xmm2) as u128;
    crate::simd_ops::wu32(rax.wrapping_add(P2), ((xmm1 as u32)) as u32);
    crate::simd_ops::wu32(rax.wrapping_add(P4), ((xmm0 as u32)) as u32);
    xmm0 = ((crate::simd_ops::ru32(rax.wrapping_add(P5))) as u32) as u128;
    xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(xmm4 as u32)).to_bits() as u128);
    crate::simd_ops::wu32(rax.wrapping_add(P5), ((xmm0 as u32)) as u32);
    xmm0 = ((crate::simd_ops::ru32(rax.wrapping_add(P6))) as u32) as u128;
    xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(xmm4 as u32)).to_bits() as u128);
    crate::simd_ops::wu32(rax.wrapping_add(P6), ((xmm0 as u32)) as u32);
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rsp = ((rsp).wrapping_add(0x48u64)) as u64;
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141720300(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1434268d0; const P1: u64 = 0x1411fa550;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm4: u128 = 0; let mut xmm5: u128 = 0; let mut xmm6: u128 = 0; let mut xmm7: u128 = 0; let mut xmm8: u128 = 0; let mut xmm9: u128 = 0; let mut xmm10: u128 = 0; let mut xmm11: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 17>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 41; continue; } bb = 1; continue; }
            1 => { rax = rsp; rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x80u64)) as u64; crate::simd_ops::wu128(rax.wrapping_sub(0x48u64), (xmm9) as u128); rdi = r9; crate::simd_ops::wu64(rax.wrapping_add(0x8u64), rbx); xmm9 = (xmm2) as u128; rbx = (crate::simd_ops::ru32(rdx.wrapping_add(0x58u64))) as u32 as u64; fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 40; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu128(rax.wrapping_sub(0x18u64), (xmm6) as u128); xmm6 = 0u128; fl_l = ((xmm9 as u32)) as u32 as u64; fl_r = ((xmm6 as u32)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 39; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu64(rax.wrapping_add(0x10u64), rbp); rbp = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rax.wrapping_add(0x18u64), rsi); rsi = (crate::simd_ops::ru64(rdx.wrapping_add(0x50u64))) as u64; crate::simd_ops::wu128(rax.wrapping_sub(0x28u64), (xmm7) as u128); xmm7 = ((crate::simd_ops::ru32(rcx.wrapping_add(0x4u64))) as u32) as u128; crate::simd_ops::wu128(rax.wrapping_sub(0x38u64), (xmm8) as u128); xmm8 = ((crate::simd_ops::ru32(rcx.wrapping_add(0x8u64))) as u32) as u128; crate::simd_ops::wu128(rax.wrapping_sub(0x68u64), (xmm11) as u128); xmm11 = ((crate::simd_ops::ru32(P0)) as u32) as u128; fl_l = (crate::simd_ops::ru8(rcx)) as u8 as u64; fl_r = ((rbp as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 14; continue; } bb = 4; continue; }
            4 => { r8 = ((rbp as u32)) as u32 as u64; xmm7 = 0u128; xmm8 = 0u128; fl_l = ((rbx as u32)) as u32 as u64; fl_r = 0x4; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 9; continue; } bb = 5; continue; }
            5 => { rax = ((rbx.wrapping_sub(0x4u64)) as u32) as u32 as u64; rax = (((rax as u32)).wrapping_shr(2u32)) as u32 as u64; rcx = rsi.wrapping_add(0x14); fl_l = (((rax as u32)).wrapping_add(1)) as u32 as u64; rax = ((fl_l as u32)) as u32 as u64; rdx = ((rax as u32)) as u32 as u64; r8 = ((rax.wrapping_mul(4)) as u32) as u32 as u64; rax = ((rax.wrapping_mul(4)) as u64) as u64; bb = 6; continue; }
            6 => { xmm7 = (xmm7 & !0xffff_ffffu128) | ((f32::from_bits(xmm7 as u32) + f32::from_bits(crate::simd_ops::ru32(rcx.wrapping_sub(0x14u64)))).to_bits() as u128); xmm8 = (xmm8 & !0xffff_ffffu128) | ((f32::from_bits(xmm8 as u32) + f32::from_bits(crate::simd_ops::ru32(rcx.wrapping_sub(0x10u64)))).to_bits() as u128); xmm7 = (xmm7 & !0xffff_ffffu128) | ((f32::from_bits(xmm7 as u32) + f32::from_bits(crate::simd_ops::ru32(rcx.wrapping_sub(0xcu64)))).to_bits() as u128); xmm8 = (xmm8 & !0xffff_ffffu128) | ((f32::from_bits(xmm8 as u32) + f32::from_bits(crate::simd_ops::ru32(rcx.wrapping_sub(0x8u64)))).to_bits() as u128); xmm7 = (xmm7 & !0xffff_ffffu128) | ((f32::from_bits(xmm7 as u32) + f32::from_bits(crate::simd_ops::ru32(rcx.wrapping_sub(0x4u64)))).to_bits() as u128); xmm8 = (xmm8 & !0xffff_ffffu128) | ((f32::from_bits(xmm8 as u32) + f32::from_bits(crate::simd_ops::ru32(rcx))).to_bits() as u128); xmm7 = (xmm7 & !0xffff_ffffu128) | ((f32::from_bits(xmm7 as u32) + f32::from_bits(crate::simd_ops::ru32(rcx.wrapping_add(0x4u64)))).to_bits() as u128); xmm8 = (xmm8 & !0xffff_ffffu128) | ((f32::from_bits(xmm8 as u32) + f32::from_bits(crate::simd_ops::ru32(rcx.wrapping_add(0x8u64)))).to_bits() as u128); rcx = ((rcx).wrapping_add(0x20u64)) as u64; fl_l = rdx; fl_r = (0x1u64) as u64; rdx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 6; continue; } bb = 7; continue; }
            7 => { fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 11; continue; } bb = 8; continue; }
            8 => { bb = 13; continue; }
            9 => { fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 13; continue; } bb = 10; continue; }
            10 => { rax = rbp; bb = 11; continue; }
            11 => { rdx = ((rbx as u32)) as u32 as u64; rcx = ((rsi.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; rdx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 12; continue; }
            12 => { xmm7 = (xmm7 & !0xffff_ffffu128) | ((f32::from_bits(xmm7 as u32) + f32::from_bits(crate::simd_ops::ru32(rcx))).to_bits() as u128); xmm8 = (xmm8 & !0xffff_ffffu128) | ((f32::from_bits(xmm8 as u32) + f32::from_bits(crate::simd_ops::ru32(rcx.wrapping_add(0x4u64)))).to_bits() as u128); rcx = ((rcx).wrapping_add(0x8u64)) as u64; fl_l = rdx; fl_r = (0x1u64) as u64; rdx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 12; continue; } bb = 13; continue; }
            13 => { xmm0 = 0u128; xmm1 = (xmm11) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((((rbx) as i64) as f32).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) / f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm7 = (xmm7 & !0xffff_ffffu128) | ((f32::from_bits(xmm7 as u32) * f32::from_bits(xmm1 as u32)).to_bits() as u128); xmm8 = (xmm8 & !0xffff_ffffu128) | ((f32::from_bits(xmm8 as u32) * f32::from_bits(xmm1 as u32)).to_bits() as u128); bb = 14; continue; }
            14 => { crate::simd_ops::wu64(rsp.wrapping_add(0xa8u64), r14); rdx = rbx; rcx = rdi; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm10) as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rbx as u32)) as u32 as u64; fl_r = 0x4; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 31; continue; } bb = 15; continue; }
            15 => { rax = ((rbx.wrapping_sub(0x4u64)) as u32) as u32 as u64; r10 = (0xfffffffffffffffcu64) as u64; rax = (((rax as u32)).wrapping_shr(2u32)) as u32 as u64; rdx = rsi.wrapping_add(0x14); r9 = 0x4; r11 = (0xffffffffffffffecu64) as u64; r10 = ((r10).wrapping_sub(rsi)) as u64; r9 = ((r9).wrapping_sub(rsi)) as u64; r11 = ((r11).wrapping_sub(rsi)) as u64; fl_l = (((rax as u32)).wrapping_add(1)) as u32 as u64; rax = ((fl_l as u32)) as u32 as u64; r8 = ((rax as u32)) as u32 as u64; rbp = ((rax.wrapping_mul(4)) as u32) as u32 as u64; r14 = ((rax.wrapping_mul(4)) as u64) as u64; bb = 16; continue; }
            16 => { xmm5 = ((crate::simd_ops::ru32(rdx.wrapping_sub(0x14u64))) as u32) as u128; xmm4 = 0u128; xmm10 = ((crate::simd_ops::ru32(rdx.wrapping_sub(0x10u64))) as u32) as u128; xmm2 = (xmm5) as u128; xmm3 = (xmm10) as u128; xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) - f32::from_bits(xmm7 as u32)).to_bits() as u128); xmm3 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) - f32::from_bits(xmm8 as u32)).to_bits() as u128); xmm0 = (xmm2) as u128; xmm1 = (xmm3) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm2 as u32)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm3 as u32)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) + f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm4 = (xmm4 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32)).sqrt().to_bits() as u128); fl_l = ((xmm4 as u32)) as u32 as u64; fl_r = ((xmm6 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) == f32::from_bits((fl_r as u32)) || f32::from_bits((fl_l as u32)).is_nan() || f32::from_bits((fl_r as u32)).is_nan()) { bb = 18; continue; } bb = 17; continue; }
            17 => { xmm0 = (xmm11) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) / f32::from_bits(xmm4 as u32)).to_bits() as u128); xmm1 = (xmm0) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm3 as u32)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm2 as u32)).to_bits() as u128); bb = 19; continue; }
            18 => { xmm1 = (xmm6) as u128; xmm0 = (xmm6) as u128; bb = 19; continue; }
            19 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rcx = ((rdx.wrapping_add(r11)) as u64) as u64; xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm9 as u32)).to_bits() as u128); xmm4 = 0u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm9 as u32)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) + f32::from_bits(xmm5 as u32)).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(xmm10 as u32)).to_bits() as u128); crate::simd_ops::wu32(rax.wrapping_add(rcx), ((xmm1 as u32)) as u32); crate::simd_ops::wu32(rax.wrapping_add(rcx).wrapping_add(0x4u64), ((xmm0 as u32)) as u32); xmm5 = ((crate::simd_ops::ru32(rdx.wrapping_sub(0xcu64))) as u32) as u128; xmm10 = ((crate::simd_ops::ru32(rdx.wrapping_sub(0x8u64))) as u32) as u128; xmm2 = (xmm5) as u128; xmm3 = (xmm10) as u128; xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) - f32::from_bits(xmm7 as u32)).to_bits() as u128); xmm3 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) - f32::from_bits(xmm8 as u32)).to_bits() as u128); xmm0 = (xmm2) as u128; xmm1 = (xmm3) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm2 as u32)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm3 as u32)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) + f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm4 = (xmm4 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32)).sqrt().to_bits() as u128); fl_l = ((xmm4 as u32)) as u32 as u64; fl_r = ((xmm6 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) == f32::from_bits((fl_r as u32)) || f32::from_bits((fl_l as u32)).is_nan() || f32::from_bits((fl_r as u32)).is_nan()) { bb = 21; continue; } bb = 20; continue; }
            20 => { xmm0 = (xmm11) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) / f32::from_bits(xmm4 as u32)).to_bits() as u128); xmm1 = (xmm0) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm3 as u32)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm2 as u32)).to_bits() as u128); bb = 22; continue; }
            21 => { xmm1 = (xmm6) as u128; xmm0 = (xmm6) as u128; bb = 22; continue; }
            22 => { rax = (crate::simd_ops::ru64(rdi)) as u64; xmm4 = 0u128; xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm9 as u32)).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm9 as u32)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) + f32::from_bits(xmm5 as u32)).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(xmm10 as u32)).to_bits() as u128); crate::simd_ops::wu32(rax.wrapping_add(rcx).wrapping_add(0x8u64), ((xmm1 as u32)) as u32); crate::simd_ops::wu32(rax.wrapping_add(rcx).wrapping_add(0xcu64), ((xmm0 as u32)) as u32); xmm5 = ((crate::simd_ops::ru32(rdx.wrapping_sub(0x4u64))) as u32) as u128; xmm10 = ((crate::simd_ops::ru32(rdx)) as u32) as u128; xmm3 = (xmm5) as u128; xmm2 = (xmm10) as u128; xmm3 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) - f32::from_bits(xmm7 as u32)).to_bits() as u128); xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) - f32::from_bits(xmm8 as u32)).to_bits() as u128); xmm0 = (xmm3) as u128; xmm1 = (xmm2) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm3 as u32)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm2 as u32)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) + f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm4 = (xmm4 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32)).sqrt().to_bits() as u128); fl_l = ((xmm4 as u32)) as u32 as u64; fl_r = ((xmm6 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) == f32::from_bits((fl_r as u32)) || f32::from_bits((fl_l as u32)).is_nan() || f32::from_bits((fl_r as u32)).is_nan()) { bb = 24; continue; } bb = 23; continue; }
            23 => { xmm0 = (xmm11) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) / f32::from_bits(xmm4 as u32)).to_bits() as u128); xmm1 = (xmm0) as u128; xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) * f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm3 as u32)).to_bits() as u128); bb = 25; continue; }
            24 => { xmm1 = (xmm6) as u128; xmm2 = (xmm6) as u128; bb = 25; continue; }
            25 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rcx = ((r10.wrapping_add(rdx)) as u64) as u64; xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm9 as u32)).to_bits() as u128); xmm4 = 0u128; xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) * f32::from_bits(xmm9 as u32)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) + f32::from_bits(xmm5 as u32)).to_bits() as u128); xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) + f32::from_bits(xmm10 as u32)).to_bits() as u128); crate::simd_ops::wu32(rcx.wrapping_add(rax), ((xmm1 as u32)) as u32); crate::simd_ops::wu32(rcx.wrapping_add(rax).wrapping_add(0x4u64), ((xmm2 as u32)) as u32); xmm5 = ((crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32) as u128; xmm10 = ((crate::simd_ops::ru32(rdx.wrapping_add(0x8u64))) as u32) as u128; xmm3 = (xmm5) as u128; xmm2 = (xmm10) as u128; xmm3 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) - f32::from_bits(xmm7 as u32)).to_bits() as u128); xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) - f32::from_bits(xmm8 as u32)).to_bits() as u128); xmm0 = (xmm3) as u128; xmm1 = (xmm2) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm3 as u32)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm2 as u32)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) + f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm4 = (xmm4 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32)).sqrt().to_bits() as u128); fl_l = ((xmm4 as u32)) as u32 as u64; fl_r = ((xmm6 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) == f32::from_bits((fl_r as u32)) || f32::from_bits((fl_l as u32)).is_nan() || f32::from_bits((fl_r as u32)).is_nan()) { bb = 27; continue; } bb = 26; continue; }
            26 => { xmm0 = (xmm11) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) / f32::from_bits(xmm4 as u32)).to_bits() as u128); xmm1 = (xmm0) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm2 as u32)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm3 as u32)).to_bits() as u128); bb = 28; continue; }
            27 => { xmm1 = (xmm6) as u128; xmm0 = (xmm6) as u128; bb = 28; continue; }
            28 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rcx = ((r9.wrapping_add(rdx)) as u64) as u64; xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm9 as u32)).to_bits() as u128); rdx = ((rdx).wrapping_add(0x20u64)) as u64; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm9 as u32)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) + f32::from_bits(xmm5 as u32)).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(xmm10 as u32)).to_bits() as u128); crate::simd_ops::wu32(rcx.wrapping_add(rax), ((xmm1 as u32)) as u32); crate::simd_ops::wu32(rcx.wrapping_add(rax).wrapping_add(0x4u64), ((xmm0 as u32)) as u32); fl_l = r8; fl_r = (0x1u64) as u64; r8 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 16; continue; } bb = 29; continue; }
            29 => { fl_l = ((rbp as u32)) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 38; continue; } bb = 30; continue; }
            30 => { bb = 33; continue; }
            31 => { fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 38; continue; } bb = 32; continue; }
            32 => { r14 = rbp; bb = 33; continue; }
            33 => { fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; rbx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; rdx = ((rsi.wrapping_add(r14.wrapping_mul(8))) as u64) as u64; r8 = ((rbx as u32)) as u32 as u64; bb = 34; continue; }
            34 => { xmm5 = ((crate::simd_ops::ru32(rdx)) as u32) as u128; xmm4 = 0u128; xmm10 = ((crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32) as u128; xmm2 = (xmm5) as u128; xmm3 = (xmm10) as u128; xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) - f32::from_bits(xmm7 as u32)).to_bits() as u128); xmm3 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) - f32::from_bits(xmm8 as u32)).to_bits() as u128); xmm0 = (xmm2) as u128; xmm1 = (xmm3) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm2 as u32)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm3 as u32)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) + f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm4 = (xmm4 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32)).sqrt().to_bits() as u128); fl_l = ((xmm4 as u32)) as u32 as u64; fl_r = ((xmm6 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) == f32::from_bits((fl_r as u32)) || f32::from_bits((fl_l as u32)).is_nan() || f32::from_bits((fl_r as u32)).is_nan()) { bb = 36; continue; } bb = 35; continue; }
            35 => { xmm0 = (xmm11) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) / f32::from_bits(xmm4 as u32)).to_bits() as u128); xmm1 = (xmm0) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm3 as u32)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm2 as u32)).to_bits() as u128); bb = 37; continue; }
            36 => { xmm1 = (xmm6) as u128; xmm0 = (xmm6) as u128; bb = 37; continue; }
            37 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rcx = rdx; rcx = ((rcx).wrapping_sub(rsi)) as u64; xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm9 as u32)).to_bits() as u128); rdx = ((rdx).wrapping_add(0x8u64)) as u64; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm9 as u32)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) + f32::from_bits(xmm5 as u32)).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(xmm10 as u32)).to_bits() as u128); crate::simd_ops::wu32(rcx.wrapping_add(rax), ((xmm1 as u32)) as u32); crate::simd_ops::wu32(rcx.wrapping_add(rax).wrapping_add(0x4u64), ((xmm0 as u32)) as u32); fl_l = r8; fl_r = (0x1u64) as u64; r8 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 34; continue; } bb = 38; continue; }
            38 => { xmm10 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0xa8u64))) as u64; xmm7 = (crate::simd_ops::ru128(rsp.wrapping_add(0x60u64))) as u128; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x98u64))) as u64; xmm8 = (crate::simd_ops::ru128(rsp.wrapping_add(0x50u64))) as u128; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0xa0u64))) as u64; xmm11 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; bb = 39; continue; }
            39 => { xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x70u64))) as u128; bb = 40; continue; }
            40 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; xmm9 = (crate::simd_ops::ru128(rsp.wrapping_add(0x40u64))) as u128; rsp = ((rsp).wrapping_add(0x80u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); bb = 41; continue; }
            41 => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141720770(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143426780; const P1: u64 = 0x1411fa4c0; const P2: u64 = 0x143932190; const P3: u64 = 0x1411fa680;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0; let mut xmm7: u128 = 0; let mut xmm8: u128 = 0; let mut xmm9: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = rsp; crate::simd_ops::wu64(rax.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rax.wrapping_add(0x18u64), rbp); crate::simd_ops::wu64(rax.wrapping_add(0x20u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x60u64)) as u64; xmm0 = ((crate::simd_ops::ru32(rdx)) as u32) as u128; rbx = r9; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(crate::simd_ops::ru32(r8))).to_bits() as u128); crate::simd_ops::wu128(rax.wrapping_sub(0x18u64), (xmm6) as u128); rbp = r8; crate::simd_ops::wu128(rax.wrapping_sub(0x28u64), (xmm7) as u128); rsi = rdx; crate::simd_ops::wu128(rax.wrapping_sub(0x38u64), (xmm8) as u128); xmm8 = ((crate::simd_ops::ru32(rcx)) as u32) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(crate::simd_ops::ru32(P0))).to_bits() as u128); crate::simd_ops::wu128(rax.wrapping_sub(0x48u64), (xmm9) as u128); xmm9 = ((crate::simd_ops::ru32(rcx.wrapping_add(0x4u64))) as u32) as u128; rcx = r9; xmm8 = (xmm8 & !0xffff_ffffu128) | ((f32::from_bits(xmm8 as u32) - f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm0 = ((crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(crate::simd_ops::ru32(r8.wrapping_add(0x4u64)))).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(crate::simd_ops::ru32(P0))).to_bits() as u128); xmm9 = (xmm9 & !0xffff_ffffu128) | ((f32::from_bits(xmm9 as u32) - f32::from_bits(xmm0 as u32)).to_bits() as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; fl_l = rcx; fl_r = (0x5u64) as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 5; continue; } bb = 1; continue; }
            1 => { rdx = (crate::simd_ops::ru64(rbx)) as u64; rcx = (crate::simd_ops::ru64(P2)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rcx)) as u64; r8 = 0x40; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x50u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 4; continue; }
            3 => { crate::simd_ops::wu32(rsp.wrapping_add(0x70u64), (0x2u32) as u32); r8 = rsp.wrapping_add(0x70); rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = 0x40; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x48u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 4; continue; }
            4 => { rcx = 0x8; crate::simd_ops::wu64(rbx, rax); crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), rcx); bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rdi = rax.wrapping_add(0x1); fl_l = rdi; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 8; continue; } bb = 6; continue; }
            6 => { rcx = ((rcx).wrapping_shr(1u32)) as u64; fl_l = rdi; fl_r = rcx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 11; continue; } bb = 7; continue; }
            7 => { r8 = rdi; bb = 10; continue; }
            8 => { fl_l = rdi; fl_r = rcx; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 11; continue; } bb = 9; continue; }
            9 => { r8 = rdi; r8 = ((r8).wrapping_shr(2u32)) as u64; fl_l = ((r8).wrapping_add(rdi)) as u64; fl_r = fl_l; r8 = ((fl_l as u64)) as u64; bb = 10; continue; }
            10 => { rdx = rbx; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 11; continue; }
            11 => { rax = (crate::simd_ops::ru64(rbx)) as u64; xmm6 = (xmm8) as u128; crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), rdi); xmm7 = (xmm9) as u128; crate::simd_ops::wu32(rax.wrapping_add(rdi.wrapping_mul(8)).wrapping_sub(0x8u64), ((xmm8 as u32)) as u32); crate::simd_ops::wu32(rax.wrapping_add(rdi.wrapping_mul(8)).wrapping_sub(0x4u64), ((xmm9 as u32)) as u32); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; xmm6 = (xmm6 & !0xffff_ffffu128) | ((f32::from_bits(xmm6 as u32) + f32::from_bits(crate::simd_ops::ru32(rsi))).to_bits() as u128); xmm7 = (xmm7 & !0xffff_ffffu128) | ((f32::from_bits(xmm7 as u32) + f32::from_bits(crate::simd_ops::ru32(rsi.wrapping_add(0x4u64)))).to_bits() as u128); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rdi = rax.wrapping_add(0x1); fl_l = rdi; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 14; continue; } bb = 12; continue; }
            12 => { rcx = ((rcx).wrapping_shr(1u32)) as u64; fl_l = rdi; fl_r = rcx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 17; continue; } bb = 13; continue; }
            13 => { r8 = rdi; bb = 16; continue; }
            14 => { fl_l = rdi; fl_r = rcx; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 17; continue; } bb = 15; continue; }
            15 => { r8 = rdi; r8 = ((r8).wrapping_shr(2u32)) as u64; fl_l = ((r8).wrapping_add(rdi)) as u64; fl_r = fl_l; r8 = ((fl_l as u64)) as u64; bb = 16; continue; }
            16 => { rdx = rbx; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 17; continue; }
            17 => { rax = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), rdi); crate::simd_ops::wu32(rax.wrapping_add(rdi.wrapping_mul(8)).wrapping_sub(0x8u64), ((xmm6 as u32)) as u32); crate::simd_ops::wu32(rax.wrapping_add(rdi.wrapping_mul(8)).wrapping_sub(0x4u64), ((xmm7 as u32)) as u32); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; xmm6 = (xmm6 & !0xffff_ffffu128) | ((f32::from_bits(xmm6 as u32) + f32::from_bits(crate::simd_ops::ru32(rbp))).to_bits() as u128); xmm7 = (xmm7 & !0xffff_ffffu128) | ((f32::from_bits(xmm7 as u32) + f32::from_bits(crate::simd_ops::ru32(rbp.wrapping_add(0x4u64)))).to_bits() as u128); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rdi = rax.wrapping_add(0x1); fl_l = rdi; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 20; continue; } bb = 18; continue; }
            18 => { rcx = ((rcx).wrapping_shr(1u32)) as u64; fl_l = rdi; fl_r = rcx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 23; continue; } bb = 19; continue; }
            19 => { r8 = rdi; bb = 22; continue; }
            20 => { fl_l = rdi; fl_r = rcx; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 23; continue; } bb = 21; continue; }
            21 => { r8 = rdi; r8 = ((r8).wrapping_shr(2u32)) as u64; fl_l = ((r8).wrapping_add(rdi)) as u64; fl_r = fl_l; r8 = ((fl_l as u64)) as u64; bb = 22; continue; }
            22 => { rdx = rbx; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 23; continue; }
            23 => { rax = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), rdi); crate::simd_ops::wu32(rax.wrapping_add(rdi.wrapping_mul(8)).wrapping_sub(0x8u64), ((xmm6 as u32)) as u32); crate::simd_ops::wu32(rax.wrapping_add(rdi.wrapping_mul(8)).wrapping_sub(0x4u64), ((xmm7 as u32)) as u32); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; xmm6 = (xmm6 & !0xffff_ffffu128) | ((f32::from_bits(xmm6 as u32) - f32::from_bits(crate::simd_ops::ru32(rsi))).to_bits() as u128); xmm7 = (xmm7 & !0xffff_ffffu128) | ((f32::from_bits(xmm7 as u32) - f32::from_bits(crate::simd_ops::ru32(rsi.wrapping_add(0x4u64)))).to_bits() as u128); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rdi = rax.wrapping_add(0x1); fl_l = rdi; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 26; continue; } bb = 24; continue; }
            24 => { rcx = ((rcx).wrapping_shr(1u32)) as u64; fl_l = rdi; fl_r = rcx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 29; continue; } bb = 25; continue; }
            25 => { r8 = rdi; bb = 28; continue; }
            26 => { fl_l = rdi; fl_r = rcx; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 29; continue; } bb = 27; continue; }
            27 => { r8 = rdi; r8 = ((r8).wrapping_shr(2u32)) as u64; fl_l = ((r8).wrapping_add(rdi)) as u64; fl_r = fl_l; r8 = ((fl_l as u64)) as u64; bb = 28; continue; }
            28 => { rdx = rbx; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 29; continue; }
            29 => { rax = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), rdi); crate::simd_ops::wu32(rax.wrapping_add(rdi.wrapping_mul(8)).wrapping_sub(0x8u64), ((xmm6 as u32)) as u32); crate::simd_ops::wu32(rax.wrapping_add(rdi.wrapping_mul(8)).wrapping_sub(0x4u64), ((xmm7 as u32)) as u32); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rdi = rax.wrapping_add(0x1); fl_l = rdi; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 32; continue; } bb = 30; continue; }
            30 => { rcx = ((rcx).wrapping_shr(1u32)) as u64; fl_l = rdi; fl_r = rcx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 35; continue; } bb = 31; continue; }
            31 => { r8 = rdi; bb = 34; continue; }
            32 => { fl_l = rdi; fl_r = rcx; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 35; continue; } bb = 33; continue; }
            33 => { r8 = rdi; r8 = ((r8).wrapping_shr(2u32)) as u64; fl_l = ((r8).wrapping_add(rdi)) as u64; fl_r = fl_l; r8 = ((fl_l as u64)) as u64; bb = 34; continue; }
            34 => { rdx = rbx; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 35; continue; }
            35 => { rax = (crate::simd_ops::ru64(rbx)) as u64; r11 = rsp.wrapping_add(0x60); rbp = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x50u64))) as u128; xmm7 = (crate::simd_ops::ru128(rsp.wrapping_add(0x40u64))) as u128; crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), rdi); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x18u64))) as u64; crate::simd_ops::wu32(rax.wrapping_add(rdi.wrapping_mul(8)).wrapping_sub(0x8u64), ((xmm8 as u32)) as u32); xmm8 = (crate::simd_ops::ru128(r11.wrapping_sub(0x30u64))) as u128; crate::simd_ops::wu32(rax.wrapping_add(rdi.wrapping_mul(8)).wrapping_sub(0x4u64), ((xmm9 as u32)) as u32); xmm9 = (crate::simd_ops::ru128(r11.wrapping_sub(0x40u64))) as u128; rsp = r11; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1417209f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143426780; const P1: u64 = 0x1411fa4c0; const P2: u64 = 0x143932190; const P3: u64 = 0x1411fa680;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0; let mut xmm7: u128 = 0; let mut xmm8: u128 = 0; let mut xmm9: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = rsp; crate::simd_ops::wu64(rax.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rax.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rax.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rax.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x60u64)) as u64; xmm0 = ((crate::simd_ops::ru32(rdx)) as u32) as u128; r14 = r9; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(crate::simd_ops::ru32(r8))).to_bits() as u128); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; rbp = r8; crate::simd_ops::wu128(rax.wrapping_sub(0x18u64), (xmm6) as u128); rsi = rdx; crate::simd_ops::wu128(rax.wrapping_sub(0x28u64), (xmm7) as u128); crate::simd_ops::wu128(rax.wrapping_sub(0x38u64), (xmm8) as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(crate::simd_ops::ru32(r9))).to_bits() as u128); xmm8 = ((crate::simd_ops::ru32(rcx)) as u32) as u128; crate::simd_ops::wu128(rax.wrapping_sub(0x48u64), (xmm9) as u128); xmm9 = ((crate::simd_ops::ru32(rcx.wrapping_add(0x4u64))) as u32) as u128; rcx = rbx; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(crate::simd_ops::ru32(P0))).to_bits() as u128); xmm8 = (xmm8 & !0xffff_ffffu128) | ((f32::from_bits(xmm8 as u32) - f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm0 = ((crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(crate::simd_ops::ru32(r8.wrapping_add(0x4u64)))).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(crate::simd_ops::ru32(r9.wrapping_add(0x4u64)))).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(crate::simd_ops::ru32(P0))).to_bits() as u128); xmm9 = (xmm9 & !0xffff_ffffu128) | ((f32::from_bits(xmm9 as u32) - f32::from_bits(xmm0 as u32)).to_bits() as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; fl_l = rcx; fl_r = (0x7u64) as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 5; continue; } bb = 1; continue; }
            1 => { rdx = (crate::simd_ops::ru64(rbx)) as u64; rcx = (crate::simd_ops::ru64(P2)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rcx)) as u64; r8 = 0x40; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x50u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 4; continue; }
            3 => { crate::simd_ops::wu32(rsp.wrapping_add(0x90u64), (0x2u32) as u32); r8 = rsp.wrapping_add(0x90); rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = 0x40; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x48u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 4; continue; }
            4 => { rcx = 0x8; crate::simd_ops::wu64(rbx, rax); crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), rcx); bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rdi = rax.wrapping_add(0x1); fl_l = rdi; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 8; continue; } bb = 6; continue; }
            6 => { rcx = ((rcx).wrapping_shr(1u32)) as u64; fl_l = rdi; fl_r = rcx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 11; continue; } bb = 7; continue; }
            7 => { r8 = rdi; bb = 10; continue; }
            8 => { fl_l = rdi; fl_r = rcx; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 11; continue; } bb = 9; continue; }
            9 => { r8 = rdi; r8 = ((r8).wrapping_shr(2u32)) as u64; fl_l = ((r8).wrapping_add(rdi)) as u64; fl_r = fl_l; r8 = ((fl_l as u64)) as u64; bb = 10; continue; }
            10 => { rdx = rbx; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 11; continue; }
            11 => { rax = (crate::simd_ops::ru64(rbx)) as u64; xmm6 = (xmm8) as u128; crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), rdi); xmm7 = (xmm9) as u128; crate::simd_ops::wu32(rax.wrapping_add(rdi.wrapping_mul(8)).wrapping_sub(0x8u64), ((xmm8 as u32)) as u32); crate::simd_ops::wu32(rax.wrapping_add(rdi.wrapping_mul(8)).wrapping_sub(0x4u64), ((xmm9 as u32)) as u32); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; xmm6 = (xmm6 & !0xffff_ffffu128) | ((f32::from_bits(xmm6 as u32) + f32::from_bits(crate::simd_ops::ru32(rsi))).to_bits() as u128); xmm7 = (xmm7 & !0xffff_ffffu128) | ((f32::from_bits(xmm7 as u32) + f32::from_bits(crate::simd_ops::ru32(rsi.wrapping_add(0x4u64)))).to_bits() as u128); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rdi = rax.wrapping_add(0x1); fl_l = rdi; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 14; continue; } bb = 12; continue; }
            12 => { rcx = ((rcx).wrapping_shr(1u32)) as u64; fl_l = rdi; fl_r = rcx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 17; continue; } bb = 13; continue; }
            13 => { r8 = rdi; bb = 16; continue; }
            14 => { fl_l = rdi; fl_r = rcx; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 17; continue; } bb = 15; continue; }
            15 => { r8 = rdi; r8 = ((r8).wrapping_shr(2u32)) as u64; fl_l = ((r8).wrapping_add(rdi)) as u64; fl_r = fl_l; r8 = ((fl_l as u64)) as u64; bb = 16; continue; }
            16 => { rdx = rbx; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 17; continue; }
            17 => { rax = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), rdi); crate::simd_ops::wu32(rax.wrapping_add(rdi.wrapping_mul(8)).wrapping_sub(0x8u64), ((xmm6 as u32)) as u32); crate::simd_ops::wu32(rax.wrapping_add(rdi.wrapping_mul(8)).wrapping_sub(0x4u64), ((xmm7 as u32)) as u32); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; xmm6 = (xmm6 & !0xffff_ffffu128) | ((f32::from_bits(xmm6 as u32) + f32::from_bits(crate::simd_ops::ru32(rbp))).to_bits() as u128); xmm7 = (xmm7 & !0xffff_ffffu128) | ((f32::from_bits(xmm7 as u32) + f32::from_bits(crate::simd_ops::ru32(rbp.wrapping_add(0x4u64)))).to_bits() as u128); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rdi = rax.wrapping_add(0x1); fl_l = rdi; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 20; continue; } bb = 18; continue; }
            18 => { rcx = ((rcx).wrapping_shr(1u32)) as u64; fl_l = rdi; fl_r = rcx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 23; continue; } bb = 19; continue; }
            19 => { r8 = rdi; bb = 22; continue; }
            20 => { fl_l = rdi; fl_r = rcx; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 23; continue; } bb = 21; continue; }
            21 => { r8 = rdi; r8 = ((r8).wrapping_shr(2u32)) as u64; fl_l = ((r8).wrapping_add(rdi)) as u64; fl_r = fl_l; r8 = ((fl_l as u64)) as u64; bb = 22; continue; }
            22 => { rdx = rbx; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 23; continue; }
            23 => { rax = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), rdi); crate::simd_ops::wu32(rax.wrapping_add(rdi.wrapping_mul(8)).wrapping_sub(0x8u64), ((xmm6 as u32)) as u32); crate::simd_ops::wu32(rax.wrapping_add(rdi.wrapping_mul(8)).wrapping_sub(0x4u64), ((xmm7 as u32)) as u32); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; xmm6 = (xmm6 & !0xffff_ffffu128) | ((f32::from_bits(xmm6 as u32) + f32::from_bits(crate::simd_ops::ru32(r14))).to_bits() as u128); xmm7 = (xmm7 & !0xffff_ffffu128) | ((f32::from_bits(xmm7 as u32) + f32::from_bits(crate::simd_ops::ru32(r14.wrapping_add(0x4u64)))).to_bits() as u128); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rdi = rax.wrapping_add(0x1); fl_l = rdi; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 26; continue; } bb = 24; continue; }
            24 => { rcx = ((rcx).wrapping_shr(1u32)) as u64; fl_l = rdi; fl_r = rcx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 29; continue; } bb = 25; continue; }
            25 => { r8 = rdi; bb = 28; continue; }
            26 => { fl_l = rdi; fl_r = rcx; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 29; continue; } bb = 27; continue; }
            27 => { r8 = rdi; r8 = ((r8).wrapping_shr(2u32)) as u64; fl_l = ((r8).wrapping_add(rdi)) as u64; fl_r = fl_l; r8 = ((fl_l as u64)) as u64; bb = 28; continue; }
            28 => { rdx = rbx; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 29; continue; }
            29 => { rax = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), rdi); crate::simd_ops::wu32(rax.wrapping_add(rdi.wrapping_mul(8)).wrapping_sub(0x8u64), ((xmm6 as u32)) as u32); crate::simd_ops::wu32(rax.wrapping_add(rdi.wrapping_mul(8)).wrapping_sub(0x4u64), ((xmm7 as u32)) as u32); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; xmm6 = (xmm6 & !0xffff_ffffu128) | ((f32::from_bits(xmm6 as u32) - f32::from_bits(crate::simd_ops::ru32(rsi))).to_bits() as u128); xmm7 = (xmm7 & !0xffff_ffffu128) | ((f32::from_bits(xmm7 as u32) - f32::from_bits(crate::simd_ops::ru32(rsi.wrapping_add(0x4u64)))).to_bits() as u128); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rdi = rax.wrapping_add(0x1); fl_l = rdi; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 32; continue; } bb = 30; continue; }
            30 => { rcx = ((rcx).wrapping_shr(1u32)) as u64; fl_l = rdi; fl_r = rcx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 35; continue; } bb = 31; continue; }
            31 => { r8 = rdi; bb = 34; continue; }
            32 => { fl_l = rdi; fl_r = rcx; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 35; continue; } bb = 33; continue; }
            33 => { r8 = rdi; r8 = ((r8).wrapping_shr(2u32)) as u64; fl_l = ((r8).wrapping_add(rdi)) as u64; fl_r = fl_l; r8 = ((fl_l as u64)) as u64; bb = 34; continue; }
            34 => { rdx = rbx; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 35; continue; }
            35 => { rax = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), rdi); crate::simd_ops::wu32(rax.wrapping_add(rdi.wrapping_mul(8)).wrapping_sub(0x8u64), ((xmm6 as u32)) as u32); crate::simd_ops::wu32(rax.wrapping_add(rdi.wrapping_mul(8)).wrapping_sub(0x4u64), ((xmm7 as u32)) as u32); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; xmm6 = (xmm6 & !0xffff_ffffu128) | ((f32::from_bits(xmm6 as u32) - f32::from_bits(crate::simd_ops::ru32(rbp))).to_bits() as u128); xmm7 = (xmm7 & !0xffff_ffffu128) | ((f32::from_bits(xmm7 as u32) - f32::from_bits(crate::simd_ops::ru32(rbp.wrapping_add(0x4u64)))).to_bits() as u128); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rdi = rax.wrapping_add(0x1); fl_l = rdi; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 38; continue; } bb = 36; continue; }
            36 => { rcx = ((rcx).wrapping_shr(1u32)) as u64; fl_l = rdi; fl_r = rcx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 41; continue; } bb = 37; continue; }
            37 => { r8 = rdi; bb = 40; continue; }
            38 => { fl_l = rdi; fl_r = rcx; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 41; continue; } bb = 39; continue; }
            39 => { r8 = rdi; r8 = ((r8).wrapping_shr(2u32)) as u64; fl_l = ((r8).wrapping_add(rdi)) as u64; fl_r = fl_l; r8 = ((fl_l as u64)) as u64; bb = 40; continue; }
            40 => { rdx = rbx; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 41; continue; }
            41 => { rax = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), rdi); crate::simd_ops::wu32(rax.wrapping_add(rdi.wrapping_mul(8)).wrapping_sub(0x8u64), ((xmm6 as u32)) as u32); crate::simd_ops::wu32(rax.wrapping_add(rdi.wrapping_mul(8)).wrapping_sub(0x4u64), ((xmm7 as u32)) as u32); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rdi = rax.wrapping_add(0x1); fl_l = rdi; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 44; continue; } bb = 42; continue; }
            42 => { rcx = ((rcx).wrapping_shr(1u32)) as u64; fl_l = rdi; fl_r = rcx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 47; continue; } bb = 43; continue; }
            43 => { r8 = rdi; bb = 46; continue; }
            44 => { fl_l = rdi; fl_r = rcx; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 47; continue; } bb = 45; continue; }
            45 => { r8 = rdi; r8 = ((r8).wrapping_shr(2u32)) as u64; fl_l = ((r8).wrapping_add(rdi)) as u64; fl_r = fl_l; r8 = ((fl_l as u64)) as u64; bb = 46; continue; }
            46 => { rdx = rbx; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 47; continue; }
            47 => { rax = (crate::simd_ops::ru64(rbx)) as u64; r11 = rsp.wrapping_add(0x60); rbp = (crate::simd_ops::ru64(r11.wrapping_add(0x18u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x50u64))) as u128; xmm7 = (crate::simd_ops::ru128(rsp.wrapping_add(0x40u64))) as u128; crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), rdi); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x10u64))) as u64; crate::simd_ops::wu32(rax.wrapping_add(rdi.wrapping_mul(8)).wrapping_sub(0x8u64), ((xmm8 as u32)) as u32); xmm8 = (crate::simd_ops::ru128(r11.wrapping_sub(0x30u64))) as u128; crate::simd_ops::wu32(rax.wrapping_add(rdi.wrapping_mul(8)).wrapping_sub(0x4u64), ((xmm9 as u32)) as u32); rdi = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; xmm9 = (crate::simd_ops::ru128(r11.wrapping_sub(0x40u64))) as u128; rsp = r11; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141720d40(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143426780;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut xmm4: u128 = 0; let mut xmm5: u128 = 0; let mut xmm6: u128 = 0; let mut xmm7: u128 = 0; let mut xmm8: u128 = 0; let mut xmm9: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x58u64)) as u64; xmm5 = ((crate::simd_ops::ru32(r8)) as u32) as u128; xmm4 = ((crate::simd_ops::ru32(r8.wrapping_add(0x8u64))) as u32) as u128; xmm0 = ((crate::simd_ops::ru32(r8.wrapping_add(0xcu64))) as u32) as u128; xmm1 = ((crate::simd_ops::ru32(r8.wrapping_add(0x10u64))) as u32) as u128; xmm2 = ((crate::simd_ops::ru32(r8.wrapping_add(0x14u64))) as u32) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm6) as u128); xmm6 = ((crate::simd_ops::ru32(r8.wrapping_add(0x4u64))) as u32) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm7) as u128); xmm7 = (xmm0) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm8) as u128); xmm7 = (xmm7 & !0xffff_ffffu128) | ((f32::from_bits(xmm7 as u32) - f32::from_bits(xmm5 as u32)).to_bits() as u128); xmm5 = (xmm5 & !0xffff_ffffu128) | ((f32::from_bits(xmm5 as u32) + f32::from_bits(xmm0 as u32)).to_bits() as u128); crate::simd_ops::wu128(rsp.wrapping_add(0x10u64), (xmm9) as u128); xmm0 = ((crate::simd_ops::ru32(P0)) as u32) as u128; xmm8 = (xmm1) as u128; xmm8 = (xmm8 & !0xffff_ffffu128) | ((f32::from_bits(xmm8 as u32) - f32::from_bits(xmm6 as u32)).to_bits() as u128); xmm9 = (xmm2) as u128; xmm6 = (xmm6 & !0xffff_ffffu128) | ((f32::from_bits(xmm6 as u32) + f32::from_bits(xmm1 as u32)).to_bits() as u128); xmm9 = (xmm9 & !0xffff_ffffu128) | ((f32::from_bits(xmm9 as u32) - f32::from_bits(xmm4 as u32)).to_bits() as u128); xmm5 = (xmm5 & !0xffff_ffffu128) | ((f32::from_bits(xmm5 as u32) * f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm4 = (xmm4 & !0xffff_ffffu128) | ((f32::from_bits(xmm4 as u32) + f32::from_bits(xmm2 as u32)).to_bits() as u128); xmm6 = (xmm6 & !0xffff_ffffu128) | ((f32::from_bits(xmm6 as u32) * f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm4 = (xmm4 & !0xffff_ffffu128) | ((f32::from_bits(xmm4 as u32) * f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm0 = (xmm5) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(crate::simd_ops::ru32(rdx.wrapping_add(0x4u64)))).to_bits() as u128); xmm1 = (xmm6) as u128; xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(crate::simd_ops::ru32(rdx.wrapping_add(0x10u64)))).to_bits() as u128); xmm3 = (xmm4) as u128; xmm3 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) * f32::from_bits(crate::simd_ops::ru32(rdx.wrapping_add(0x1cu64)))).to_bits() as u128); xmm2 = (xmm4) as u128; xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) + f32::from_bits(crate::simd_ops::ru32(rdx.wrapping_add(0x28u64)))).to_bits() as u128); xmm4 = (xmm4 & !0xffff_ffffu128) | ((f32::from_bits(xmm4 as u32) * f32::from_bits(crate::simd_ops::ru32(rdx.wrapping_add(0x18u64)))).to_bits() as u128); xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) * f32::from_bits(crate::simd_ops::ru32(rdx.wrapping_add(0x20u64)))).to_bits() as u128); xmm3 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) + f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm0 = (xmm5) as u128; xmm5 = (xmm5 & !0xffff_ffffu128) | ((f32::from_bits(xmm5 as u32) * f32::from_bits(crate::simd_ops::ru32(rdx))).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(crate::simd_ops::ru32(rdx.wrapping_add(0x8u64)))).to_bits() as u128); xmm3 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) + f32::from_bits(xmm1 as u32)).to_bits() as u128); xmm1 = (xmm6) as u128; xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(crate::simd_ops::ru32(rdx.wrapping_add(0x14u64)))).to_bits() as u128); xmm4 = (xmm4 & !0xffff_ffffu128) | ((f32::from_bits(xmm4 as u32) + f32::from_bits(xmm5 as u32)).to_bits() as u128); xmm6 = (xmm6 & !0xffff_ffffu128) | ((f32::from_bits(xmm6 as u32) * f32::from_bits(crate::simd_ops::ru32(rdx.wrapping_add(0xcu64)))).to_bits() as u128); xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) + f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) + f32::from_bits(crate::simd_ops::ru32(rdx.wrapping_add(0x2cu64)))).to_bits() as u128); xmm0 = (xmm7) as u128; xmm6 = (xmm6 & !0xffff_ffffu128) | ((f32::from_bits(xmm6 as u32) + f32::from_bits(crate::simd_ops::ru32(rdx.wrapping_add(0x24u64)))).to_bits() as u128); xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) + f32::from_bits(xmm1 as u32)).to_bits() as u128); xmm1 = (xmm7) as u128; xmm4 = (xmm4 & !0xffff_ffffu128) | ((f32::from_bits(xmm4 as u32) + f32::from_bits(xmm6 as u32)).to_bits() as u128); crate::simd_ops::wu32(rcx, ((xmm4 as u32)) as u32); crate::simd_ops::wu32(rcx.wrapping_add(0x4u64), ((xmm3 as u32)) as u32); crate::simd_ops::wu32(rcx.wrapping_add(0x8u64), ((xmm2 as u32)) as u32); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(crate::simd_ops::ru32(rdx))).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(crate::simd_ops::ru32(rdx.wrapping_add(0x4u64)))).to_bits() as u128); xmm7 = (xmm7 & !0xffff_ffffu128) | ((f32::from_bits(xmm7 as u32) * f32::from_bits(crate::simd_ops::ru32(rdx.wrapping_add(0x8u64)))).to_bits() as u128); xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x40u64))) as u128; xmm0 = crate::simd_ops::vunpcklps(xmm0, xmm1); xmm1 = (xmm8) as u128; crate::simd_ops::wu64(rcx.wrapping_add(0xcu64), ((xmm0 as u64)) as u64); xmm0 = (xmm8) as u128; crate::simd_ops::wu32(rsp.wrapping_add(0x8u64), ((xmm7 as u32)) as u32); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x8u64))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x14u64), ((rax as u32)) as u32); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(crate::simd_ops::ru32(rdx.wrapping_add(0xcu64)))).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(crate::simd_ops::ru32(rdx.wrapping_add(0x10u64)))).to_bits() as u128); xmm8 = (xmm8 & !0xffff_ffffu128) | ((f32::from_bits(xmm8 as u32) * f32::from_bits(crate::simd_ops::ru32(rdx.wrapping_add(0x14u64)))).to_bits() as u128); xmm0 = crate::simd_ops::vunpcklps(xmm0, xmm1); xmm1 = (xmm9) as u128; crate::simd_ops::wu64(rcx.wrapping_add(0x18u64), ((xmm0 as u64)) as u64); xmm0 = (xmm9) as u128; crate::simd_ops::wu32(rsp.wrapping_add(0x8u64), ((xmm8 as u32)) as u32); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x8u64))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x20u64), ((rax as u32)) as u32); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(crate::simd_ops::ru32(rdx.wrapping_add(0x18u64)))).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(crate::simd_ops::ru32(rdx.wrapping_add(0x1cu64)))).to_bits() as u128); xmm9 = (xmm9 & !0xffff_ffffu128) | ((f32::from_bits(xmm9 as u32) * f32::from_bits(crate::simd_ops::ru32(rdx.wrapping_add(0x20u64)))).to_bits() as u128); xmm0 = crate::simd_ops::vunpcklps(xmm0, xmm1); crate::simd_ops::wu64(rcx.wrapping_add(0x24u64), ((xmm0 as u64)) as u64); crate::simd_ops::wu32(rsp.wrapping_add(0x8u64), ((xmm9 as u32)) as u32); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x8u64))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x2cu64), ((rax as u32)) as u32); xmm7 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; xmm8 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; xmm9 = (crate::simd_ops::ru128(rsp.wrapping_add(0x10u64))) as u128; rsp = ((rsp).wrapping_add(0x58u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141720ed0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141720d40; const P1: u64 = 0x143428720; const P2: u64 = 0x143426780; const P3: u64 = 0x141720fe0;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x50u64)) as u64; xmm0 = 0u128; rdi = r8; r8 = rdx; xmm1 = 0u128; rdx = rcx; rbx = r9; rcx = rsp.wrapping_add(0x20); crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm1) as u128); crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm0) as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm2 = ((crate::simd_ops::ru32(P1)) as u32) as u128; rcx = rsp.wrapping_add(0x20); xmm0 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x34u64))) as u32) as u128; rdx = rdi; xmm1 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x4cu64))) as u32) as u128; xmm0 = (xmm0) & (xmm2); xmm3 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x40u64))) as u32) as u128; xmm1 = (xmm1) & (xmm2); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; xmm3 = (xmm3) & (xmm2); xmm2 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x88u64))) as u32) as u128; xmm3 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) + f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm3 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) + f32::from_bits(xmm1 as u32)).to_bits() as u128); xmm1 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x28u64))) as u32) as u128; xmm0 = (xmm1) as u128; xmm3 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) * f32::from_bits(crate::simd_ops::ru32(P2))).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) - f32::from_bits(xmm3 as u32)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) + f32::from_bits(xmm3 as u32)).to_bits() as u128); crate::simd_ops::wu32(rbx, ((xmm0 as u32)) as u32); crate::simd_ops::wu32(rax, ((xmm1 as u32)) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsp = ((rsp).wrapping_add(0x50u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141720f80(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141720d40; const P1: u64 = 0x141720fe0;
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut xmm6: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x60u64)) as u64;
    xmm0 = 0u128;
    crate::simd_ops::wu128(rsp.wrapping_add(0x50u64), (xmm6) as u128);
    rbx = r8;
    xmm1 = 0u128;
    r8 = rdx;
    xmm6 = (xmm3) as u128;
    rdx = rcx;
    rcx = rsp.wrapping_add(0x20);
    crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128);
    crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm1) as u128);
    crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm0) as u128);
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    xmm2 = (xmm6) as u128;
    rcx = rsp.wrapping_add(0x20);
    rdx = rbx;
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x50u64))) as u128;
    rsp = ((rsp).wrapping_add(0x60u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141720fe0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141720770; const P1: u64 = 0x143428760; const P2: u64 = 0x1417209f0;
    let (mut rax, mut fl_l, mut fl_r, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut xmm4: u128 = 0; let mut xmm5: u128 = 0; let mut xmm6: u128 = 0; let mut xmm7: u128 = 0; let mut xmm8: u128 = 0; let mut xmm9: u128 = 0; let mut xmm10: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 19>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = rsp; rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rbp = rax.wrapping_sub(0x5f); rsp = ((rsp).wrapping_sub(0x90u64)) as u64; xmm0 = ((crate::simd_ops::ru32(rcx)) as u32) as u128; xmm1 = ((crate::simd_ops::ru32(rcx.wrapping_add(0x4u64))) as u32) as u128; xmm5 = ((crate::simd_ops::ru32(rcx.wrapping_add(0x18u64))) as u32) as u128; xmm4 = ((crate::simd_ops::ru32(rcx.wrapping_add(0x1cu64))) as u32) as u128; xmm3 = ((crate::simd_ops::ru32(rcx.wrapping_add(0x24u64))) as u32) as u128; crate::simd_ops::wu128(rax.wrapping_sub(0x18u64), (xmm6) as u128); xmm6 = ((crate::simd_ops::ru32(rcx.wrapping_add(0x10u64))) as u32) as u128; crate::simd_ops::wu128(rax.wrapping_sub(0x28u64), (xmm7) as u128); xmm7 = ((crate::simd_ops::ru32(rcx.wrapping_add(0xcu64))) as u32) as u128; crate::simd_ops::wu32(rbp.wrapping_add(0x83u64), ((xmm1 as u32)) as u32); xmm1 = (xmm6) as u128; crate::simd_ops::wu128(rax.wrapping_sub(0x38u64), (xmm8) as u128); xmm8 = (xmm2) as u128; xmm2 = ((crate::simd_ops::ru32(rcx.wrapping_add(0x28u64))) as u32) as u128; rax = rdx; crate::simd_ops::wu32(rbp.wrapping_add(0x7fu64), ((xmm0 as u32)) as u32); xmm0 = (xmm7) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm7 as u32)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm6 as u32)).to_bits() as u128); xmm8 = (xmm8 & !0xffff_ffffu128) | ((f32::from_bits(xmm8 as u32) * f32::from_bits(xmm8 as u32)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) + f32::from_bits(xmm0 as u32)).to_bits() as u128); crate::simd_ops::wu32(rbp.wrapping_add(0x67u64), ((xmm7 as u32)) as u32); crate::simd_ops::wu32(rbp.wrapping_add(0x6bu64), ((xmm6 as u32)) as u32); crate::simd_ops::wu32(rbp.wrapping_sub(0x9u64), ((xmm5 as u32)) as u32); crate::simd_ops::wu32(rbp.wrapping_sub(0x5u64), ((xmm4 as u32)) as u32); crate::simd_ops::wu32(rbp.wrapping_sub(0x1u64), ((xmm3 as u32)) as u32); fl_l = ((xmm1 as u32)) as u32 as u64; fl_r = ((xmm8 as u32)) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x3u64), ((xmm2 as u32)) as u32); if (f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 3; continue; } bb = 1; continue; }
            1 => { rcx = rbp.wrapping_sub(0x9); xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) * f32::from_bits(xmm5 as u32)).to_bits() as u128); r8 = rbp.wrapping_sub(0x1); xmm3 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) * f32::from_bits(xmm4 as u32)).to_bits() as u128); rdx = rbp.wrapping_sub(0x9); xmm0 = 0u128; xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) - f32::from_bits(xmm3 as u32)).to_bits() as u128); fl_l = ((xmm2 as u32)) as u32 as u64; fl_r = ((xmm0 as u32)) as u32 as u64; r8 = (if (f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { rcx } else { r8 }) as u64; rcx = rbp.wrapping_sub(0x1); bb = 2; continue; }
            2 => { rdx = (if (f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { rcx } else { rdx }) as u64; r9 = rax; rcx = rbp.wrapping_add(0x7f); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 39; continue; }
            3 => { xmm1 = (xmm4) as u128; xmm0 = (xmm5) as u128; xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm4 as u32)).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm5 as u32)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) + f32::from_bits(xmm0 as u32)).to_bits() as u128); fl_l = ((xmm1 as u32)) as u32 as u64; fl_r = ((xmm8 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rcx = rbp.wrapping_sub(0x1); xmm3 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) * f32::from_bits(xmm6 as u32)).to_bits() as u128); r8 = rbp.wrapping_add(0x67); xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) * f32::from_bits(xmm7 as u32)).to_bits() as u128); rdx = rbp.wrapping_sub(0x1); xmm0 = 0u128; xmm3 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) - f32::from_bits(xmm2 as u32)).to_bits() as u128); fl_l = ((xmm3 as u32)) as u32 as u64; fl_r = ((xmm0 as u32)) as u32 as u64; r8 = (if (f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { rcx } else { r8 }) as u64; rcx = rbp.wrapping_add(0x67); bb = 2; continue; }
            5 => { xmm1 = (xmm2) as u128; xmm0 = (xmm3) as u128; xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm2 as u32)).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm3 as u32)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) + f32::from_bits(xmm0 as u32)).to_bits() as u128); fl_l = ((xmm1 as u32)) as u32 as u64; fl_r = ((xmm8 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rcx = rbp.wrapping_add(0x67); xmm7 = (xmm7 & !0xffff_ffffu128) | ((f32::from_bits(xmm7 as u32) * f32::from_bits(xmm4 as u32)).to_bits() as u128); r8 = rbp.wrapping_sub(0x9); xmm6 = (xmm6 & !0xffff_ffffu128) | ((f32::from_bits(xmm6 as u32) * f32::from_bits(xmm5 as u32)).to_bits() as u128); rdx = rbp.wrapping_add(0x67); xmm0 = 0u128; xmm7 = (xmm7 & !0xffff_ffffu128) | ((f32::from_bits(xmm7 as u32) - f32::from_bits(xmm6 as u32)).to_bits() as u128); fl_l = ((xmm7 as u32)) as u32 as u64; fl_r = ((xmm0 as u32)) as u32 as u64; r8 = (if (f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { rcx } else { r8 }) as u64; rcx = rbp.wrapping_sub(0x9); bb = 2; continue; }
            7 => { crate::simd_ops::wu128(rsp.wrapping_add(0x50u64), (xmm9) as u128); xmm0 = (xmm6) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm5 as u32)).to_bits() as u128); xmm1 = (xmm2) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm10) as u128); xmm9 = (xmm3) as u128; xmm10 = (xmm7) as u128; xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm5 as u32)).to_bits() as u128); xmm10 = (xmm10 & !0xffff_ffffu128) | ((f32::from_bits(xmm10 as u32) * f32::from_bits(xmm4 as u32)).to_bits() as u128); xmm8 = 0u128; xmm9 = (xmm9 & !0xffff_ffffu128) | ((f32::from_bits(xmm9 as u32) * f32::from_bits(xmm6 as u32)).to_bits() as u128); xmm10 = (xmm10 & !0xffff_ffffu128) | ((f32::from_bits(xmm10 as u32) - f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm0 = (xmm3) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm4 as u32)).to_bits() as u128); fl_l = ((xmm10 as u32)) as u32 as u64; fl_r = ((xmm8 as u32)) as u32 as u64; xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) - f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm0 = (xmm2) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm7 as u32)).to_bits() as u128); xmm9 = (xmm9 & !0xffff_ffffu128) | ((f32::from_bits(xmm9 as u32) - f32::from_bits(xmm0 as u32)).to_bits() as u128); if (f32::from_bits((fl_l as u32)) != f32::from_bits((fl_r as u32)) && !f32::from_bits((fl_l as u32)).is_nan() && !f32::from_bits((fl_r as u32)).is_nan()) { bb = 12; continue; } bb = 8; continue; }
            8 => { xmm1 = (xmm5) as u128; rcx = rbp.wrapping_add(0x67); xmm0 = (xmm6) as u128; xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm7 as u32)).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm4 as u32)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) + f32::from_bits(xmm0 as u32)).to_bits() as u128); fl_l = ((xmm1 as u32)) as u32 as u64; fl_r = ((xmm8 as u32)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 10; continue; } bb = 9; continue; }
            9 => { xmm5 = (xmm5 & !0xffff_ffffu128) | ((f32::from_bits(xmm5 as u32) + f32::from_bits(xmm7 as u32)).to_bits() as u128); xmm6 = (xmm6 & !0xffff_ffffu128) | ((f32::from_bits(xmm6 as u32) + f32::from_bits(xmm4 as u32)).to_bits() as u128); crate::simd_ops::wu32(rbp.wrapping_add(0x67u64), ((xmm5 as u32)) as u32); bb = 11; continue; }
            10 => { xmm7 = (xmm7 & !0xffff_ffffu128) | ((f32::from_bits(xmm7 as u32) - f32::from_bits(xmm5 as u32)).to_bits() as u128); xmm6 = (xmm6 & !0xffff_ffffu128) | ((f32::from_bits(xmm6 as u32) - f32::from_bits(xmm4 as u32)).to_bits() as u128); crate::simd_ops::wu32(rbp.wrapping_add(0x67u64), ((xmm7 as u32)) as u32); bb = 11; continue; }
            11 => { crate::simd_ops::wu32(rbp.wrapping_add(0x6bu64), ((xmm6 as u32)) as u32); r8 = rbp.wrapping_sub(0x1); xmm1 = ((crate::simd_ops::ru64(rcx)) as u64) as u128; rcx = rbp.wrapping_add(0x67); xmm0 = (xmm1) as u128; xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) * f32::from_bits(xmm1 as u32)).to_bits() as u128); xmm0 = crate::simd_ops::vshufps(xmm0, xmm0, 0x55u8); xmm3 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) * f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) - f32::from_bits(xmm3 as u32)).to_bits() as u128); fl_l = ((xmm2 as u32)) as u32 as u64; fl_r = ((xmm8 as u32)) as u32 as u64; r8 = (if (f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { rcx } else { r8 }) as u64; rcx = rbp.wrapping_sub(0x1); bb = 22; continue; }
            12 => { fl_l = ((xmm1 as u32)) as u32 as u64; fl_r = ((xmm8 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) != f32::from_bits((fl_r as u32)) && !f32::from_bits((fl_l as u32)).is_nan() && !f32::from_bits((fl_r as u32)).is_nan()) { bb = 17; continue; } bb = 13; continue; }
            13 => { xmm1 = (xmm3) as u128; rcx = rbp.wrapping_sub(0x1); xmm0 = (xmm2) as u128; xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm5 as u32)).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm4 as u32)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) + f32::from_bits(xmm0 as u32)).to_bits() as u128); fl_l = ((xmm1 as u32)) as u32 as u64; fl_r = ((xmm8 as u32)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 15; continue; } bb = 14; continue; }
            14 => { xmm3 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) + f32::from_bits(xmm5 as u32)).to_bits() as u128); xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) + f32::from_bits(xmm4 as u32)).to_bits() as u128); crate::simd_ops::wu32(rbp.wrapping_sub(0x1u64), ((xmm3 as u32)) as u32); crate::simd_ops::wu32(rbp.wrapping_add(0x3u64), ((xmm2 as u32)) as u32); bb = 16; continue; }
            15 => { xmm5 = (xmm5 & !0xffff_ffffu128) | ((f32::from_bits(xmm5 as u32) - f32::from_bits(xmm3 as u32)).to_bits() as u128); xmm4 = (xmm4 & !0xffff_ffffu128) | ((f32::from_bits(xmm4 as u32) - f32::from_bits(xmm2 as u32)).to_bits() as u128); crate::simd_ops::wu32(rbp.wrapping_sub(0x1u64), ((xmm5 as u32)) as u32); crate::simd_ops::wu32(rbp.wrapping_add(0x3u64), ((xmm4 as u32)) as u32); bb = 16; continue; }
            16 => { xmm1 = ((crate::simd_ops::ru64(rcx)) as u64) as u128; r8 = rbp.wrapping_add(0x67); rcx = rbp.wrapping_sub(0x1); xmm6 = (xmm6 & !0xffff_ffffu128) | ((f32::from_bits(xmm6 as u32) * f32::from_bits(xmm1 as u32)).to_bits() as u128); rdx = rbp.wrapping_sub(0x1); xmm0 = (xmm1) as u128; crate::simd_ops::wu64(rbp.wrapping_sub(0x1u64), ((xmm1 as u64)) as u64); xmm0 = crate::simd_ops::vshufps(xmm0, xmm0, 0x55u8); xmm7 = (xmm7 & !0xffff_ffffu128) | ((f32::from_bits(xmm7 as u32) * f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm6 = (xmm6 & !0xffff_ffffu128) | ((f32::from_bits(xmm6 as u32) - f32::from_bits(xmm7 as u32)).to_bits() as u128); fl_l = ((xmm6 as u32)) as u32 as u64; fl_r = ((xmm8 as u32)) as u32 as u64; r8 = (if (f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { rcx } else { r8 }) as u64; rcx = rbp.wrapping_add(0x67); bb = 23; continue; }
            17 => { fl_l = ((xmm9 as u32)) as u32 as u64; fl_r = ((xmm8 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) != f32::from_bits((fl_r as u32)) && !f32::from_bits((fl_l as u32)).is_nan() && !f32::from_bits((fl_r as u32)).is_nan()) { bb = 24; continue; } bb = 18; continue; }
            18 => { xmm1 = (xmm3) as u128; rcx = rbp.wrapping_add(0x67); xmm0 = (xmm2) as u128; xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm7 as u32)).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm6 as u32)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) + f32::from_bits(xmm0 as u32)).to_bits() as u128); fl_l = ((xmm1 as u32)) as u32 as u64; fl_r = ((xmm8 as u32)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 20; continue; } bb = 19; continue; }
            19 => { xmm3 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) + f32::from_bits(xmm7 as u32)).to_bits() as u128); xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) + f32::from_bits(xmm6 as u32)).to_bits() as u128); bb = 21; continue; }
            20 => { xmm3 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) - f32::from_bits(xmm7 as u32)).to_bits() as u128); xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) - f32::from_bits(xmm6 as u32)).to_bits() as u128); bb = 21; continue; }
            21 => { crate::simd_ops::wu32(rbp.wrapping_add(0x6bu64), ((xmm2 as u32)) as u32); r8 = rbp.wrapping_sub(0x9); crate::simd_ops::wu32(rbp.wrapping_add(0x67u64), ((xmm3 as u32)) as u32); xmm1 = ((crate::simd_ops::ru64(rcx)) as u64) as u128; rcx = rbp.wrapping_add(0x67); xmm0 = (xmm1) as u128; xmm4 = (xmm4 & !0xffff_ffffu128) | ((f32::from_bits(xmm4 as u32) * f32::from_bits(xmm1 as u32)).to_bits() as u128); xmm0 = crate::simd_ops::vshufps(xmm0, xmm0, 0x55u8); xmm5 = (xmm5 & !0xffff_ffffu128) | ((f32::from_bits(xmm5 as u32) * f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm4 = (xmm4 & !0xffff_ffffu128) | ((f32::from_bits(xmm4 as u32) - f32::from_bits(xmm5 as u32)).to_bits() as u128); fl_l = ((xmm4 as u32)) as u32 as u64; fl_r = ((xmm8 as u32)) as u32 as u64; r8 = (if (f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { rcx } else { r8 }) as u64; rcx = rbp.wrapping_sub(0x9); bb = 22; continue; }
            22 => { rdx = rbp.wrapping_add(0x67); crate::simd_ops::wu64(rbp.wrapping_add(0x67u64), ((xmm1 as u64)) as u64); bb = 23; continue; }
            23 => { rdx = (if (f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { rcx } else { rdx }) as u64; r9 = rax; rcx = rbp.wrapping_add(0x7f); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 38; continue; }
            24 => { fl_l = ((xmm9 as u32)) as u32 as u64; fl_r = ((xmm8 as u32)) as u32 as u64; rdx = (rdx & !0xffu64) | (((if !(f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { 1u8 } else { 0u8 }) as u8 as u64)); fl_l = ((xmm1 as u32)) as u32 as u64; fl_r = ((xmm8 as u32)) as u32 as u64; rcx = (rcx & !0xffu64) | (((if (f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { 1u8 } else { 0u8 }) as u8 as u64)); fl_l = ((xmm10 as u32)) as u32 as u64; fl_r = ((xmm8 as u32)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 31; continue; } bb = 25; continue; }
            25 => { fl_l = ((rdx as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 27; continue; } bb = 26; continue; }
            26 => { fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; rdx = rbp.wrapping_sub(0x9); r8 = rbp.wrapping_sub(0x1); r8 = (if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { rdx } else { r8 }) as u64; r9 = rbp.wrapping_sub(0x1); rdx = rbp.wrapping_sub(0x9); rdx = (if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { r9 } else { rdx }) as u64; bb = 36; continue; }
            27 => { xmm3 = (xmm3) ^ (crate::simd_ops::ru128(P1)); xmm2 = (xmm2) ^ (crate::simd_ops::ru128(P1)); crate::simd_ops::wu32(rbp.wrapping_sub(0x1u64), ((xmm3 as u32)) as u32); crate::simd_ops::wu32(rbp.wrapping_add(0x3u64), ((xmm2 as u32)) as u32); bb = 28; continue; }
            28 => { fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); rcx = rbp.wrapping_add(0x7f); r9 = rbp.wrapping_add(0x67); if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 30; continue; } bb = 29; continue; }
            29 => { r8 = rbp.wrapping_sub(0x1); rdx = rbp.wrapping_sub(0x9); bb = 37; continue; }
            30 => { r8 = rbp.wrapping_sub(0x9); rdx = rbp.wrapping_sub(0x1); bb = 37; continue; }
            31 => { xmm0 = ((crate::simd_ops::ru32(P1)) as u32) as u128; xmm5 = (xmm5) ^ (xmm0); xmm4 = (xmm4) ^ (xmm0); fl_l = ((rdx as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 33; continue; } bb = 32; continue; }
            32 => { crate::simd_ops::wu32(rbp.wrapping_sub(0x9u64), ((xmm5 as u32)) as u32); crate::simd_ops::wu32(rbp.wrapping_sub(0x5u64), ((xmm4 as u32)) as u32); bb = 28; continue; }
            33 => { r8 = rbp.wrapping_sub(0x1); xmm3 = (xmm3) ^ (xmm0); rdx = rbp.wrapping_sub(0x9); xmm2 = (xmm2) ^ (xmm0); fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 35; continue; } bb = 34; continue; }
            34 => { crate::simd_ops::wu32(rbp.wrapping_sub(0x1u64), ((xmm5 as u32)) as u32); crate::simd_ops::wu32(rbp.wrapping_add(0x3u64), ((xmm4 as u32)) as u32); crate::simd_ops::wu32(rbp.wrapping_sub(0x9u64), ((xmm3 as u32)) as u32); crate::simd_ops::wu32(rbp.wrapping_sub(0x5u64), ((xmm2 as u32)) as u32); bb = 36; continue; }
            35 => { crate::simd_ops::wu32(rbp.wrapping_sub(0x1u64), ((xmm3 as u32)) as u32); crate::simd_ops::wu32(rbp.wrapping_add(0x3u64), ((xmm2 as u32)) as u32); crate::simd_ops::wu32(rbp.wrapping_sub(0x9u64), ((xmm5 as u32)) as u32); crate::simd_ops::wu32(rbp.wrapping_sub(0x5u64), ((xmm4 as u32)) as u32); bb = 36; continue; }
            36 => { rcx = rbp.wrapping_add(0x7f); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); r9 = rbp.wrapping_add(0x67); bb = 37; continue; }
            37 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 38; continue; }
            38 => { xmm9 = (crate::simd_ops::ru128(rsp.wrapping_add(0x50u64))) as u128; xmm10 = (crate::simd_ops::ru128(rsp.wrapping_add(0x40u64))) as u128; bb = 39; continue; }
            39 => { xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x80u64))) as u128; xmm7 = (crate::simd_ops::ru128(rsp.wrapping_add(0x70u64))) as u128; xmm8 = (crate::simd_ops::ru128(rsp.wrapping_add(0x60u64))) as u128; rsp = ((rsp).wrapping_add(0x90u64)) as u64; rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141721400(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x18; const P3: u64 = 0x20; const P4: u64 = 0x14169edb0; const P5: u64 = 0x28; const P6: u64 = 0x30; const P7: u64 = 0x38; const P8: u64 = 0x40; const P9: u64 = 0x143932190; const P10: u64 = 0x60; const P11: u64 = 0x48;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = 0; fl_l = 0; fl_r = 0; rbx = rcx; crate::simd_ops::wu64(rcx, rdi); crate::simd_ops::wu64(rcx.wrapping_add(P0), rdi); crate::simd_ops::wu64(rcx.wrapping_add(P1), rdi); crate::simd_ops::wu64(rcx.wrapping_add(P2), rdi); rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P3))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rbx.wrapping_add(P3), rdi); crate::simd_ops::wu64(rbx.wrapping_add(P5), rdi); crate::simd_ops::wu64(rbx.wrapping_add(P6), rdi); crate::simd_ops::wu64(rbx.wrapping_add(P7), rdi); rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P8))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 3; continue; }
            3 => { rax = 0xffffffff; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rdx, xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru64(P9)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P10))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            5 => { crate::simd_ops::wu64(rbx.wrapping_add(P8), rdi); rax = 0xffff; crate::simd_ops::wu16(rbx.wrapping_add(P11), ((rax as u16)) as u16); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141721480(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x18; const P3: u64 = 0x20; const P4: u64 = 0x14169edb0; const P5: u64 = 0x28; const P6: u64 = 0x30; const P7: u64 = 0x38; const P8: u64 = 0x40; const P9: u64 = 0x143932190; const P10: u64 = 0x60; const P11: u64 = 0x48; const P12: u64 = 0x143426e04; const P13: u64 = 0x143426514; const P14: u64 = 0x14169eda0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; r14 = 0; fl_l = 0; fl_r = 0; rbx = rcx; crate::simd_ops::wu64(rcx, r14); rsi = r9; crate::simd_ops::wu64(rcx.wrapping_add(P0), r14); rbp = r8; crate::simd_ops::wu64(rcx.wrapping_add(P1), r14); rdi = rdx; crate::simd_ops::wu64(rcx.wrapping_add(P2), r14); rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P3))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rbx.wrapping_add(P3), r14); crate::simd_ops::wu64(rbx.wrapping_add(P5), r14); crate::simd_ops::wu64(rbx.wrapping_add(P6), r14); crate::simd_ops::wu64(rbx.wrapping_add(P7), r14); rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P8))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 3; continue; }
            3 => { rax = 0xffffffff; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rdx, xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru64(P9)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P10))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 5; continue; }
            5 => { crate::simd_ops::wu64(rbx.wrapping_add(P8), r14); rax = 0xffff; crate::simd_ops::wu16(rbx.wrapping_add(P11), ((rax as u16)) as u16); crate::simd_ops::wu64(rbx, rdi); crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), r14); xmm0 = ((crate::simd_ops::ru32(rdi.wrapping_add(0x64u64))) as u32) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(crate::simd_ops::ru32(P12))).to_bits() as u128); rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x10u64), ((xmm0 as u32)) as u32); xmm1 = ((crate::simd_ops::ru32(rbp.wrapping_add(0x4u64))) as u32) as u128; xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(crate::simd_ops::ru32(P13))).to_bits() as u128); crate::simd_ops::wu64(rbx.wrapping_add(0x18u64), rsi); crate::simd_ops::wu32(rbx.wrapping_add(0x14u64), ((xmm1 as u32)) as u32); fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P14, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 7; continue; }
            7 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x20u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 9; continue; }
            9 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x20u64), rdi); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141721580(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141722590; const P1: u64 = 0x141722650; const P2: u64 = 0x141721f90;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 15>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((r9 as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x18u64), ((r8 as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x10u64), ((rdx as u32)) as u32); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x48u64)) as u64; rdi = (crate::simd_ops::ru32(rcx.wrapping_add(0x48u64))) as u32 as u64; rbx = rcx; r10 = (crate::simd_ops::ru64(rcx.wrapping_add(0x20u64))) as u64; r15 = (crate::simd_ops::ru32(rcx.wrapping_add(0x8u64))) as u32 as u64; rax = ((rdx as u32)) as u32 as u64; r14 = ((rdi.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x80u64), ((rdi as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x24u64), ((r14 as u32)) as u32); rbp = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = ((r8 as u32)) as u32 as u64; rbp = ((rbp).wrapping_shl(4u32)) as u64; rbp = ((rbp).wrapping_add(r10)) as u64; r13 = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = ((r9 as u32)) as u32 as u64; r13 = ((r13).wrapping_shl(4u32)) as u64; r13 = ((r13).wrapping_add(r10)) as u64; rdx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rdx = ((rdx).wrapping_shl(4u32)) as u64; rax = ((rdi.wrapping_add(0x2u64)) as u32) as u32 as u64; r9 = (crate::simd_ops::ru32(r13.wrapping_add(0x8u64))) as u32 as u64; rdx = ((rdx).wrapping_add(r10)) as u64; r9 = (((r9 as u32)).wrapping_sub(crate::simd_ops::ru32(rbp.wrapping_add(0x8u64)))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rax as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rdx); rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32 as u64; rax = (((rax as u32)).wrapping_sub(crate::simd_ops::ru32(rbp.wrapping_add(0x4u64)))) as u32 as u64; rdx = (crate::simd_ops::ru32(rdx.wrapping_add(0x8u64))) as u32 as u64; rdx = (((rdx as u32)).wrapping_sub(crate::simd_ops::ru32(rbp.wrapping_add(0x8u64)))) as u32 as u64; r9 = ((((r9 as u32)) as i32).wrapping_mul(((rax as u32)) as i32) as u32) as u32 as u64; rax = (crate::simd_ops::ru32(r13.wrapping_add(0x4u64))) as u32 as u64; rax = (((rax as u32)).wrapping_sub(crate::simd_ops::ru32(rbp.wrapping_add(0x4u64)))) as u32 as u64; rdx = ((((rdx as u32)) as i32).wrapping_mul(((rax as u32)) as i32) as u32) as u32 as u64; rdx = (((rdx as u32)).wrapping_sub((r9 as u32))) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) > 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = 0xffffffff; rsp = ((rsp).wrapping_add(0x48u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            2 => { crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rsi); rdx = ((rdi.wrapping_add(0x3u64)) as u32) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), r12); r12 = rcx.wrapping_add(0x40); rcx = r12; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rdx = ((r15.wrapping_add(0x1u64)) as u32) as u32 as u64; rcx = rbx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(r12)) as u64; rsi = rdi; rdx = (crate::simd_ops::ru32(rsp.wrapping_add(0x80u64))) as u32 as u64; r8 = (crate::simd_ops::ru32(rsp.wrapping_add(0x20u64))) as u32 as u64; rdi = ((r14 as u32)) as u32 as u64; r14 = ((r8 as u32)) as u32 as u64; r14 = ((r14).wrapping_shl(5u32)) as u64; r14 = ((r14).wrapping_add(rax)) as u64; rsi = ((rsi).wrapping_shl(5u32)) as u64; rsi = ((rsi).wrapping_add(rax)) as u64; rdi = ((rdi).wrapping_shl(5u32)) as u64; rdi = ((rdi).wrapping_add(rax)) as u64; rcx = ((rdx.wrapping_add(0x1u64)) as u32) as u32 as u64; rax = r15; rax = ((rax).wrapping_shl(4u32)) as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(rbx))) as u64; rbx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rsi, ((rdx as u32)) as u32); crate::simd_ops::wu32(rdi, ((rcx as u32)) as u32); crate::simd_ops::wu32(r14, ((r8 as u32)) as u32); crate::simd_ops::wu32(rax.wrapping_add(0x4u64), ((rdx as u32)) as u32); rdx = ((rbx as u32)) as u32 as u64; crate::simd_ops::wu32(rax, ((r15 as u32)) as u32); fl_l = (crate::simd_ops::ru32(rbp.wrapping_add(0x18u64))) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 8; continue; } bb = 3; continue; }
            3 => { r11 = (crate::simd_ops::ru64(rbp.wrapping_add(0x10u64))) as u64; r9 = (crate::simd_ops::ru64(r12)) as u64; bb = 4; continue; }
            4 => { r10 = (crate::simd_ops::ru32(rsp.wrapping_add(0x98u64))) as u32 as u64; rax = ((rdx as u32)) as u32 as u64; rcx = (crate::simd_ops::ru32(r11.wrapping_add(rax.wrapping_mul(4)))) as u32 as u64; r8 = ((r11.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rcx = ((rcx).wrapping_shl(5u32)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(r9).wrapping_add(0xcu64))) as u32 as u64; rax = ((rax).wrapping_shl(5u32)) as u64; fl_l = (crate::simd_ops::ru32(rax.wrapping_add(r9).wrapping_add(0x4u64))) as u32 as u64; fl_r = ((r10 as u32)) as u32 as u64; r10 = (crate::simd_ops::ru32(rbp.wrapping_add(0x18u64))) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 7; continue; } bb = 5; continue; }
            5 => { rdx = (((rdx as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((r10 as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 4; continue; } bb = 6; continue; }
            6 => { bb = 8; continue; }
            7 => { rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x20u64))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(r9).wrapping_add(0x10u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(r8)) as u32 as u64; crate::simd_ops::wu32(r14.wrapping_add(0x10u64), ((rax as u32)) as u32); bb = 8; continue; }
            8 => { rdx = (crate::simd_ops::ru32(rsp.wrapping_add(0x80u64))) as u32 as u64; rcx = rbp; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); r10 = (crate::simd_ops::ru32(r13.wrapping_add(0x18u64))) as u32 as u64; rdx = ((rbx as u32)) as u32 as u64; fl_l = ((r10 as u32)) as u32 as u64; fl_r = ((r10 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 14; continue; } bb = 9; continue; }
            9 => { r11 = (crate::simd_ops::ru64(r13.wrapping_add(0x10u64))) as u64; r9 = (crate::simd_ops::ru64(r12)) as u64; rbp = (crate::simd_ops::ru32(rsp.wrapping_add(0x88u64))) as u32 as u64; bb = 10; continue; }
            10 => { rax = ((rdx as u32)) as u32 as u64; rcx = (crate::simd_ops::ru32(r11.wrapping_add(rax.wrapping_mul(4)))) as u32 as u64; r8 = ((r11.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rcx = ((rcx).wrapping_shl(5u32)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(r9).wrapping_add(0xcu64))) as u32 as u64; rax = ((rax).wrapping_shl(5u32)) as u64; fl_l = (crate::simd_ops::ru32(rax.wrapping_add(r9).wrapping_add(0x4u64))) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 13; continue; } bb = 11; continue; }
            11 => { rdx = (((rdx as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((r10 as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 10; continue; } bb = 12; continue; }
            12 => { bb = 14; continue; }
            13 => { rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x80u64))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(r9).wrapping_add(0x10u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(r8)) as u32 as u64; crate::simd_ops::wu32(rsi.wrapping_add(0x10u64), ((rax as u32)) as u32); bb = 14; continue; }
            14 => { rdx = (crate::simd_ops::ru32(rsp.wrapping_add(0x24u64))) as u32 as u64; rcx = r13; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); r11 = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rbp = (crate::simd_ops::ru32(rsp.wrapping_add(0x90u64))) as u32 as u64; r9 = (crate::simd_ops::ru32(r11.wrapping_add(0x18u64))) as u32 as u64; fl_l = ((r9 as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 20; continue; } bb = 15; continue; }
            15 => { r10 = (crate::simd_ops::ru64(r11.wrapping_add(0x10u64))) as u64; r8 = (crate::simd_ops::ru64(r12)) as u64; bb = 16; continue; }
            16 => { rax = (crate::simd_ops::ru32(r10.wrapping_add(rbx.wrapping_mul(4)))) as u32 as u64; rdx = ((r10.wrapping_add(rbx.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(5u32)) as u64; rcx = (crate::simd_ops::ru32(rax.wrapping_add(r8).wrapping_add(0xcu64))) as u32 as u64; rcx = ((rcx).wrapping_shl(5u32)) as u64; fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(r8).wrapping_add(0x4u64))) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 19; continue; } bb = 17; continue; }
            17 => { rbx = (((rbx as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 16; continue; } bb = 18; continue; }
            18 => { bb = 20; continue; }
            19 => { rcx = (crate::simd_ops::ru32(rsp.wrapping_add(0x24u64))) as u32 as u64; crate::simd_ops::wu32(rax.wrapping_add(r8).wrapping_add(0x10u64), ((rcx as u32)) as u32); rax = (crate::simd_ops::ru32(rdx)) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x10u64), ((rax as u32)) as u32); bb = 20; continue; }
            20 => { rbx = (crate::simd_ops::ru32(rsp.wrapping_add(0x20u64))) as u32 as u64; rcx = r11; rdx = ((rbx as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x88u64))) as u32 as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; crate::simd_ops::wu32(rsi.wrapping_add(0x8u64), ((r15 as u32)) as u32); crate::simd_ops::wu32(rdi.wrapping_add(0x8u64), ((r15 as u32)) as u32); crate::simd_ops::wu32(r14.wrapping_add(0x8u64), ((r15 as u32)) as u32); crate::simd_ops::wu32(rsi.wrapping_add(0x4u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x98u64))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x4u64), ((rbp as u32)) as u32); crate::simd_ops::wu32(r14.wrapping_add(0x4u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x24u64))) as u32 as u64; crate::simd_ops::wu32(rsi.wrapping_add(0xcu64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x80u64))) as u32 as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; crate::simd_ops::wu32(rdi.wrapping_add(0xcu64), ((rbx as u32)) as u32); crate::simd_ops::wu32(r14.wrapping_add(0xcu64), ((rax as u32)) as u32); rax = ((r15 as u32)) as u32 as u64; rsp = ((rsp).wrapping_add(0x48u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141721820(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x4; const P1: u64 = 0x20; const P2: u64 = 0x28; const P3: u64 = 0x1; const P4: u64 = 0x141722700; const P5: u64 = 0xc;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; fl_l = (crate::simd_ops::ru32(rdx)) as u32 as u64; fl_r = 0xffffff00; rsi = rdx; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm6) as u128); xmm6 = (xmm2) as u128; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64; fl_r = 0xffffff00; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rbx); rbx = ((rcx.wrapping_add(P1)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rdi); rdi = (crate::simd_ops::ru32(rcx.wrapping_add(P2))) as u32 as u64; rcx = rbx; rdx = ((rdi.wrapping_add(P3)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = ((rdi.wrapping_add(rdi.wrapping_mul(2))) as u64) as u64; rax = ((rdi as u32)) as u32 as u64; rdx = ((rdx).wrapping_shl(4u32)) as u64; rdx = ((rdx).wrapping_add(crate::simd_ops::ru64(rbx))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; crate::simd_ops::wu32(rdx, ((rdi as u32)) as u32); rcx = (crate::simd_ops::ru64(rsi)) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; crate::simd_ops::wu64(rdx.wrapping_add(P0), rcx); crate::simd_ops::wu32(rdx.wrapping_add(P5), ((xmm6 as u32)) as u32); xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            3 => { xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; rax = 0xffffffff; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1417218a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1411ff9f0; const P1: u64 = 0x141722650; const P2: u64 = 0x20; const P3: u64 = 0x141722700; const P4: u64 = 0x40; const P5: u64 = 0x141722590;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    rcx = ((rcx).wrapping_add(0x60u64)) as u64;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rdx = 0;
    fl_l = 0; fl_r = 0;
    rcx = rbx;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rcx = ((rbx.wrapping_add(P2)) as u64) as u64;
    rdx = 0;
    fl_l = 0; fl_r = 0;
    rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp);
    rcx = ((rbx.wrapping_add(P4)) as u64) as u64;
    rdx = 0;
    fl_l = 0; fl_r = 0;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(P5, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1417218e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141722290; const P1: u64 = 0x141721f90;
    let (mut rax, mut r10, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; r9 = (crate::simd_ops::ru64(rcx.wrapping_add(0x40u64))) as u64; r10 = ((r8 as u32)) as u32 as u64; r8 = (crate::simd_ops::ru64(rcx.wrapping_add(0x20u64))) as u64; r10 = ((r10).wrapping_shl(5u32)) as u64; r10 = ((r10).wrapping_add(r9)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), r10); rax = (crate::simd_ops::ru32(r10.wrapping_add(0xcu64))) as u32 as u64; r13 = (crate::simd_ops::ru32(r10.wrapping_add(0x10u64))) as u32 as u64; r12 = (crate::simd_ops::ru32(r10.wrapping_add(0x8u64))) as u32 as u64; rdx = (crate::simd_ops::ru32(r10)) as u32 as u64; rax = ((rax).wrapping_shl(5u32)) as u64; rax = ((rax).wrapping_add(r9)) as u64; r13 = ((r13).wrapping_shl(5u32)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); r13 = ((r13).wrapping_add(r9)) as u64; r12 = ((r12).wrapping_shl(4u32)) as u64; r12 = ((r12).wrapping_add(crate::simd_ops::ru64(rcx))) as u64; rax = (crate::simd_ops::ru32(rax.wrapping_add(0xcu64))) as u32 as u64; r14 = (crate::simd_ops::ru32(r13.wrapping_add(0xcu64))) as u32 as u64; rdi = (crate::simd_ops::ru32(r13.wrapping_add(0x8u64))) as u32 as u64; rax = ((rax).wrapping_shl(5u32)) as u64; rax = ((rax).wrapping_add(r9)) as u64; r14 = ((r14).wrapping_shl(5u32)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x88u64), rax); r14 = ((r14).wrapping_add(r9)) as u64; rdi = ((rdi).wrapping_shl(4u32)) as u64; rdi = ((rdi).wrapping_add(crate::simd_ops::ru64(rcx))) as u64; rax = (crate::simd_ops::ru32(rax.wrapping_add(0x4u64))) as u32 as u64; r15 = (crate::simd_ops::ru32(r14.wrapping_add(0xcu64))) as u32 as u64; r15 = ((r15).wrapping_shl(5u32)) as u64; r15 = ((r15).wrapping_add(r9)) as u64; rbp = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru32(r13.wrapping_add(0x4u64))) as u32 as u64; rbp = ((rbp).wrapping_shl(4u32)) as u64; rbp = ((rbp).wrapping_add(r8)) as u64; rsi = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru32(r15.wrapping_add(0x4u64))) as u32 as u64; rsi = ((rsi).wrapping_shl(4u32)) as u64; rsi = ((rsi).wrapping_add(r8)) as u64; rbx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru32(r10.wrapping_add(0x4u64))) as u32 as u64; rbx = ((rbx).wrapping_shl(4u32)) as u64; rbx = ((rbx).wrapping_add(r8)) as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rcx = ((rcx).wrapping_add(r8)) as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru32(r13)) as u32 as u64; rcx = rsi; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rcx = rbx; rdx = (crate::simd_ops::ru32(rsi)) as u32 as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; rcx = rbp; crate::simd_ops::wu32(rsi.wrapping_add(0x4u64), ((rax as u32)) as u32); rdx = (crate::simd_ops::ru32(r13)) as u32 as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rbp)) as u32 as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x88u64))) as u64; rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu32(r13.wrapping_add(0x4u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rcx)) as u32 as u64; crate::simd_ops::wu32(r12.wrapping_add(0x4u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(r12)) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x8u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(r12)) as u32 as u64; crate::simd_ops::wu32(r14.wrapping_add(0x8u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(r15)) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x4u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdi)) as u32 as u64; crate::simd_ops::wu32(r15.wrapping_add(0x8u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdi)) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_add(0x8u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rcx)) as u32 as u64; crate::simd_ops::wu32(rsi.wrapping_add(0xcu64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(r14)) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(0xcu64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rsi)) as u32 as u64; crate::simd_ops::wu32(r14.wrapping_add(0xcu64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(r13)) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; crate::simd_ops::wu32(rdx.wrapping_add(0xcu64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(r15)) as u32 as u64; crate::simd_ops::wu32(r13.wrapping_add(0xcu64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx)) as u32 as u64; crate::simd_ops::wu32(r15.wrapping_add(0xcu64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; crate::simd_ops::wu8(rax, (0x1u8) as u8); rsp = ((rsp).wrapping_add(0x30u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141721a60(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141722000; const P1: u64 = 0x141722320; const P2: u64 = 0x141722520; const P3: u64 = 0x1417218e0; const P4: u64 = 0x1411ff9f0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x48u64))) as u32 as u64; rbp = 0; fl_l = 0; fl_r = 0; rdi = rcx; rsi = ((rbp as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 14; continue; } bb = 1; continue; }
            1 => { bb = 2; continue; }
            2 => { rbx = ((rsi as u32)) as u32 as u64; rbx = ((rbx).wrapping_shl(5u32)) as u64; rbx = ((rbx).wrapping_add(crate::simd_ops::ru64(rdi.wrapping_add(0x40u64)))) as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x10u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0xffffffff; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 13; continue; } bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x14u64))) as u32 as u64; fl_r = 0xfffffffd; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 13; continue; } bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::ru32(rbx)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 13; continue; } bb = 5; continue; }
            5 => { rdx = ((rsi as u32)) as u32 as u64; rcx = rdi; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 13; continue; } bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x10u64))) as u32 as u64; rax = ((rax).wrapping_shl(5u32)) as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(rdi.wrapping_add(0x40u64)))) as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x14u64), ((rbp as u32)) as u32); crate::simd_ops::wu32(rax.wrapping_add(0x14u64), ((rbp as u32)) as u32); rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x68u64))) as u64; rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x78u64))) as u64; rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x70u64))) as u64; rbx = rax.wrapping_add(0x1); fl_l = rbx; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 9; continue; } bb = 7; continue; }
            7 => { rcx = ((rcx).wrapping_shr(1u32)) as u64; fl_l = rbx; fl_r = rcx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 12; continue; } bb = 8; continue; }
            8 => { r8 = rbx; bb = 11; continue; }
            9 => { fl_l = rbx; fl_r = rcx; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 12; continue; } bb = 10; continue; }
            10 => { r8 = rbx; r8 = ((r8).wrapping_shr(2u32)) as u64; fl_l = ((r8).wrapping_add(rbx)) as u64; fl_r = fl_l; r8 = ((fl_l as u64)) as u64; bb = 11; continue; }
            11 => { rcx = rdi.wrapping_add(0x60); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 12; continue; }
            12 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x60u64))) as u64; crate::simd_ops::wu64(rdi.wrapping_add(0x68u64), rbx); crate::simd_ops::wu32(rax.wrapping_add(rbx.wrapping_mul(4)).wrapping_sub(0x4u64), ((rsi as u32)) as u32); bb = 13; continue; }
            13 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x48u64))) as u32 as u64; rsi = (((rsi as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 2; continue; } bb = 14; continue; }
            14 => { r12 = ((rax.wrapping_add(rax.wrapping_mul(4))) as u32) as u32 as u64; r12 = (((r12 as u32)).wrapping_add((r12 as u32))) as u32 as u64; fl_l = (crate::simd_ops::ru64(rdi.wrapping_add(0x68u64))) as u64; fl_r = rbp; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 68; continue; } bb = 15; continue; }
            15 => { crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), r15); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), r13); bb = 16; continue; }
            16 => { rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x68u64))) as u64; rcx = rdi.wrapping_add(0x60); rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x60u64))) as u64; rbp = (crate::simd_ops::ru32(rax.wrapping_add(rdx.wrapping_mul(4)).wrapping_sub(0x4u64))) as u32 as u64; rdx = ((rdx).wrapping_sub(1)) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rsi = ((rbp as u32)) as u32 as u64; rdx = ((rbp as u32)) as u32 as u64; rsi = ((rsi).wrapping_shl(5u32)) as u64; rcx = rdi; rsi = ((rsi).wrapping_add(crate::simd_ops::ru64(rdi.wrapping_add(0x40u64)))) as u64; r12 = (((r12 as u32)).wrapping_sub(1)) as u32 as u64; r15 = (crate::simd_ops::ru32(rsi.wrapping_add(0x10u64))) as u32 as u64; r15 = ((r15).wrapping_shl(5u32)) as u64; r15 = ((r15).wrapping_add(crate::simd_ops::ru64(rdi.wrapping_add(0x40u64)))) as u64; crate::simd_ops::wu32(rsi.wrapping_add(0x14u64), (0x1u32) as u32); crate::simd_ops::wu32(r15.wrapping_add(0x14u64), (0x1u32) as u32); rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 64; continue; } bb = 17; continue; }
            17 => { r8 = ((rbp as u32)) as u32 as u64; rdx = rsp.wrapping_add(0x50); rcx = rdi; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru8(rax)) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 64; continue; } bb = 18; continue; }
            18 => { fl_l = ((r12 as u32)) as u32 as u64; fl_r = ((r12 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 66; continue; } bb = 19; continue; }
            19 => { r14 = (crate::simd_ops::ru32(rsi.wrapping_add(0xcu64))) as u32 as u64; r13 = (crate::simd_ops::ru32(r15.wrapping_add(0xcu64))) as u32 as u64; rbp = ((r14 as u32)) as u32 as u64; rbp = ((rbp).wrapping_shl(5u32)) as u64; rbp = ((rbp).wrapping_add(crate::simd_ops::ru64(rdi.wrapping_add(0x40u64)))) as u64; r13 = ((r13).wrapping_shl(5u32)) as u64; r13 = ((r13).wrapping_add(crate::simd_ops::ru64(rdi.wrapping_add(0x40u64)))) as u64; rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x14u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 30; continue; } bb = 20; continue; }
            20 => { fl_l = (crate::simd_ops::ru32(rbp.wrapping_add(0x10u64))) as u32 as u64; fl_r = 0xffffffff; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 30; continue; } bb = 21; continue; }
            21 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 30; continue; } bb = 22; continue; }
            22 => { rdx = ((r14 as u32)) as u32 as u64; rcx = rdi; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 30; continue; } bb = 23; continue; }
            23 => { rcx = (crate::simd_ops::ru32(rbp.wrapping_add(0x10u64))) as u32 as u64; rdx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rbp.wrapping_add(0x14u64), ((rdx as u32)) as u32); rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x40u64))) as u64; rcx = ((rcx).wrapping_shl(5u32)) as u64; crate::simd_ops::wu32(rcx.wrapping_add(rax).wrapping_add(0x14u64), ((rdx as u32)) as u32); rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x68u64))) as u64; rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x78u64))) as u64; rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x70u64))) as u64; rsi = rax.wrapping_add(0x1); fl_l = rsi; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 26; continue; } bb = 24; continue; }
            24 => { rcx = ((rcx).wrapping_shr(1u32)) as u64; fl_l = rsi; fl_r = rcx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 29; continue; } bb = 25; continue; }
            25 => { r8 = rsi; bb = 28; continue; }
            26 => { fl_l = rsi; fl_r = rcx; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 29; continue; } bb = 27; continue; }
            27 => { r8 = rsi; r8 = ((r8).wrapping_shr(2u32)) as u64; fl_l = ((r8).wrapping_add(rsi)) as u64; fl_r = fl_l; r8 = ((fl_l as u64)) as u64; bb = 28; continue; }
            28 => { rcx = rdi.wrapping_add(0x60); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 29; continue; }
            29 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x60u64))) as u64; crate::simd_ops::wu64(rdi.wrapping_add(0x68u64), rsi); crate::simd_ops::wu32(rax.wrapping_add(rsi.wrapping_mul(4)).wrapping_sub(0x4u64), ((r14 as u32)) as u32); bb = 30; continue; }
            30 => { rbp = (crate::simd_ops::ru32(rbp.wrapping_add(0xcu64))) as u32 as u64; rsi = ((rbp as u32)) as u32 as u64; rsi = ((rsi).wrapping_shl(5u32)) as u64; rsi = ((rsi).wrapping_add(crate::simd_ops::ru64(rdi.wrapping_add(0x40u64)))) as u64; rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x14u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 41; continue; } bb = 31; continue; }
            31 => { fl_l = (crate::simd_ops::ru32(rsi.wrapping_add(0x10u64))) as u32 as u64; fl_r = 0xffffffff; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 41; continue; } bb = 32; continue; }
            32 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 41; continue; } bb = 33; continue; }
            33 => { rdx = ((rbp as u32)) as u32 as u64; rcx = rdi; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); r14 = 0; fl_l = 0; fl_r = 0; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 42; continue; } bb = 34; continue; }
            34 => { rcx = (crate::simd_ops::ru32(rsi.wrapping_add(0x10u64))) as u32 as u64; crate::simd_ops::wu32(rsi.wrapping_add(0x14u64), ((r14 as u32)) as u32); rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x40u64))) as u64; rcx = ((rcx).wrapping_shl(5u32)) as u64; crate::simd_ops::wu32(rcx.wrapping_add(rax).wrapping_add(0x14u64), ((r14 as u32)) as u32); rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x68u64))) as u64; rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x78u64))) as u64; rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x70u64))) as u64; rsi = rax.wrapping_add(0x1); fl_l = rsi; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 37; continue; } bb = 35; continue; }
            35 => { rcx = ((rcx).wrapping_shr(1u32)) as u64; fl_l = rsi; fl_r = rcx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 40; continue; } bb = 36; continue; }
            36 => { r8 = rsi; bb = 39; continue; }
            37 => { fl_l = rsi; fl_r = rcx; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 40; continue; } bb = 38; continue; }
            38 => { r8 = rsi; r8 = ((r8).wrapping_shr(2u32)) as u64; fl_l = ((r8).wrapping_add(rsi)) as u64; fl_r = fl_l; r8 = ((fl_l as u64)) as u64; bb = 39; continue; }
            39 => { rcx = rdi.wrapping_add(0x60); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 40; continue; }
            40 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x60u64))) as u64; crate::simd_ops::wu64(rdi.wrapping_add(0x68u64), rsi); crate::simd_ops::wu32(rax.wrapping_add(rsi.wrapping_mul(4)).wrapping_sub(0x4u64), ((rbp as u32)) as u32); bb = 42; continue; }
            41 => { r14 = 0; fl_l = 0; fl_r = 0; bb = 42; continue; }
            42 => { rbp = (crate::simd_ops::ru32(r15.wrapping_add(0xcu64))) as u32 as u64; rsi = ((rbp as u32)) as u32 as u64; rsi = ((rsi).wrapping_shl(5u32)) as u64; rsi = ((rsi).wrapping_add(crate::simd_ops::ru64(rdi.wrapping_add(0x40u64)))) as u64; rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x14u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 53; continue; } bb = 43; continue; }
            43 => { fl_l = (crate::simd_ops::ru32(rsi.wrapping_add(0x10u64))) as u32 as u64; fl_r = 0xffffffff; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 53; continue; } bb = 44; continue; }
            44 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 53; continue; } bb = 45; continue; }
            45 => { rdx = ((rbp as u32)) as u32 as u64; rcx = rdi; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 53; continue; } bb = 46; continue; }
            46 => { rcx = (crate::simd_ops::ru32(rsi.wrapping_add(0x10u64))) as u32 as u64; crate::simd_ops::wu32(rsi.wrapping_add(0x14u64), ((r14 as u32)) as u32); rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x40u64))) as u64; rcx = ((rcx).wrapping_shl(5u32)) as u64; crate::simd_ops::wu32(rcx.wrapping_add(rax).wrapping_add(0x14u64), ((r14 as u32)) as u32); rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x68u64))) as u64; rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x78u64))) as u64; rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x70u64))) as u64; rsi = rax.wrapping_add(0x1); fl_l = rsi; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 49; continue; } bb = 47; continue; }
            47 => { rcx = ((rcx).wrapping_shr(1u32)) as u64; fl_l = rsi; fl_r = rcx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 52; continue; } bb = 48; continue; }
            48 => { r8 = rsi; bb = 51; continue; }
            49 => { fl_l = rsi; fl_r = rcx; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 52; continue; } bb = 50; continue; }
            50 => { r8 = rsi; r8 = ((r8).wrapping_shr(2u32)) as u64; fl_l = ((r8).wrapping_add(rsi)) as u64; fl_r = fl_l; r8 = ((fl_l as u64)) as u64; bb = 51; continue; }
            51 => { rcx = rdi.wrapping_add(0x60); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 52; continue; }
            52 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x60u64))) as u64; crate::simd_ops::wu64(rdi.wrapping_add(0x68u64), rsi); crate::simd_ops::wu32(rax.wrapping_add(rsi.wrapping_mul(4)).wrapping_sub(0x4u64), ((rbp as u32)) as u32); bb = 53; continue; }
            53 => { rbp = (crate::simd_ops::ru32(r13.wrapping_add(0xcu64))) as u32 as u64; rsi = ((rbp as u32)) as u32 as u64; rsi = ((rsi).wrapping_shl(5u32)) as u64; rsi = ((rsi).wrapping_add(crate::simd_ops::ru64(rdi.wrapping_add(0x40u64)))) as u64; rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x14u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 64; continue; } bb = 54; continue; }
            54 => { fl_l = (crate::simd_ops::ru32(rsi.wrapping_add(0x10u64))) as u32 as u64; fl_r = 0xffffffff; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 64; continue; } bb = 55; continue; }
            55 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 64; continue; } bb = 56; continue; }
            56 => { rdx = ((rbp as u32)) as u32 as u64; rcx = rdi; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 64; continue; } bb = 57; continue; }
            57 => { rcx = (crate::simd_ops::ru32(rsi.wrapping_add(0x10u64))) as u32 as u64; crate::simd_ops::wu32(rsi.wrapping_add(0x14u64), ((r14 as u32)) as u32); rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x40u64))) as u64; rcx = ((rcx).wrapping_shl(5u32)) as u64; crate::simd_ops::wu32(rcx.wrapping_add(rax).wrapping_add(0x14u64), ((r14 as u32)) as u32); rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x68u64))) as u64; rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x78u64))) as u64; rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x70u64))) as u64; rsi = rax.wrapping_add(0x1); fl_l = rsi; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 60; continue; } bb = 58; continue; }
            58 => { rcx = ((rcx).wrapping_shr(1u32)) as u64; fl_l = rsi; fl_r = rcx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 63; continue; } bb = 59; continue; }
            59 => { r8 = rsi; bb = 62; continue; }
            60 => { fl_l = rsi; fl_r = rcx; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 63; continue; } bb = 61; continue; }
            61 => { r8 = rsi; r8 = ((r8).wrapping_shr(2u32)) as u64; fl_l = ((r8).wrapping_add(rsi)) as u64; fl_r = fl_l; r8 = ((fl_l as u64)) as u64; bb = 62; continue; }
            62 => { rcx = rdi.wrapping_add(0x60); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 63; continue; }
            63 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x60u64))) as u64; crate::simd_ops::wu64(rdi.wrapping_add(0x68u64), rsi); crate::simd_ops::wu32(rax.wrapping_add(rsi.wrapping_mul(4)).wrapping_sub(0x4u64), ((rbp as u32)) as u32); bb = 64; continue; }
            64 => { fl_l = (crate::simd_ops::ru64(rdi.wrapping_add(0x68u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 16; continue; } bb = 65; continue; }
            65 => { bb = 67; continue; }
            66 => { rcx = rdi.wrapping_add(0x60); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 67; continue; }
            67 => { r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; bb = 68; continue; }
            68 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141721e20(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141722650; const P1: u64 = 0x141722590; const P2: u64 = 0x141721f90;
    let (mut rax, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); crate::simd_ops::wu32(rsp.wrapping_add(0x10u64), ((rdx as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = (crate::simd_ops::ru32(rcx.wrapping_add(0x8u64))) as u32 as u64; r14 = rcx; rbx = ((r8 as u32)) as u32 as u64; rdx = ((rsi.wrapping_add(0x1u64)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rbp = (crate::simd_ops::ru32(r14.wrapping_add(0x48u64))) as u32 as u64; rdi = r14.wrapping_add(0x40); rcx = rdi; rdx = ((rbp.wrapping_add(0x3u64)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rdi)) as u64; rcx = ((rbp.wrapping_add(0x1u64)) as u32) as u32 as u64; r9 = ((rbx as u32)) as u32 as u64; r15 = ((rbp as u32)) as u32 as u64; r9 = ((r9).wrapping_shl(5u32)) as u64; r9 = ((r9).wrapping_add(rax)) as u64; r15 = ((r15).wrapping_shl(5u32)) as u64; r13 = (crate::simd_ops::ru32(r9.wrapping_add(0xcu64))) as u32 as u64; r8 = (crate::simd_ops::ru32(r9.wrapping_add(0x8u64))) as u32 as u64; r8 = ((r8).wrapping_shl(4u32)) as u64; r13 = ((r13).wrapping_shl(5u32)) as u64; r13 = ((r13).wrapping_add(rax)) as u64; r12 = (crate::simd_ops::ru32(r13.wrapping_add(0xcu64))) as u32 as u64; r12 = ((r12).wrapping_shl(5u32)) as u64; r12 = ((r12).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru64(r14)) as u64; r8 = ((r8).wrapping_add(rax)) as u64; r14 = ((rsi as u32)) as u32 as u64; r14 = ((r14).wrapping_shl(4u32)) as u64; r14 = ((r14).wrapping_add(rax)) as u64; rax = ((rbp.wrapping_add(0x2u64)) as u32) as u32 as u64; crate::simd_ops::wu32(r14, ((rsi as u32)) as u32); rdx = (crate::simd_ops::ru64(rdi)) as u64; r15 = ((r15).wrapping_add(rdx)) as u64; rdi = ((rax as u32)) as u32 as u64; rsi = ((rcx as u32)) as u32 as u64; rsi = ((rsi).wrapping_shl(5u32)) as u64; rsi = ((rsi).wrapping_add(rdx)) as u64; rdi = ((rdi).wrapping_shl(5u32)) as u64; crate::simd_ops::wu32(r15, ((rbp as u32)) as u32); rdi = ((rdi).wrapping_add(rdx)) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; crate::simd_ops::wu32(rsi, ((rcx as u32)) as u32); rcx = (crate::simd_ops::ru32(rsp.wrapping_add(0x68u64))) as u32 as u64; crate::simd_ops::wu32(rdi, ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(r9)) as u32 as u64; rbx = ((rcx.wrapping_add(rcx.wrapping_mul(2))) as u64) as u64; rbx = ((rbx).wrapping_shl(4u32)) as u64; rbx = ((rbx).wrapping_add(crate::simd_ops::ru64(rbp.wrapping_add(0x20u64)))) as u64; crate::simd_ops::wu32(r8.wrapping_add(0x4u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(r8)) as u32 as u64; crate::simd_ops::wu32(r15.wrapping_add(0x8u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(r15)) as u32 as u64; crate::simd_ops::wu32(r9.wrapping_add(0xcu64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(r12)) as u32 as u64; rdx = (crate::simd_ops::ru32(r15)) as u32 as u64; crate::simd_ops::wu32(r15.wrapping_add(0x4u64), ((rcx as u32)) as u32); rcx = rbx; crate::simd_ops::wu32(r15.wrapping_add(0xcu64), ((rax as u32)) as u32); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(r13)) as u32 as u64; crate::simd_ops::wu32(r14.wrapping_add(0x4u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(r14)) as u32 as u64; crate::simd_ops::wu32(r13.wrapping_add(0x8u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(r14)) as u32 as u64; crate::simd_ops::wu32(rsi.wrapping_add(0x8u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(r14)) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x8u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdi)) as u32 as u64; crate::simd_ops::wu32(rsi.wrapping_add(0xcu64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(r13)) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0xcu64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rsi)) as u32 as u64; crate::simd_ops::wu32(r13.wrapping_add(0xcu64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(r12.wrapping_add(0x4u64))) as u32 as u64; rdx = (crate::simd_ops::ru32(rsi)) as u32 as u64; crate::simd_ops::wu32(rsi.wrapping_add(0x4u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(r12.wrapping_add(0x4u64))) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rbp.wrapping_add(0x20u64)))) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x68u64))) as u32 as u64; rcx = rbx; rdx = (crate::simd_ops::ru32(rdi)) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x4u64), ((rax as u32)) as u32); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rsi)) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; crate::simd_ops::wu32(r15.wrapping_add(0x10u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(r15)) as u32 as u64; crate::simd_ops::wu32(rsi.wrapping_add(0x10u64), ((rax as u32)) as u32); rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141721f90(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x18; const P1: u64 = 0x10; const P2: u64 = 0x20; const P3: u64 = 0x1; const P4: u64 = 0x1411fa5f0; const P5: u64 = 0x8; const P6: u64 = 0xfffffffffffffffc;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rdi = ((rcx.wrapping_add(P1)) as u64) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64; rsi = ((rdx as u32)) as u32 as u64; rbx = ((rax.wrapping_add(P3)) as u64) as u64; fl_l = rbx; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 3; continue; } bb = 1; continue; }
            1 => { rcx = ((rcx).wrapping_shr(1u32)) as u64; fl_l = rbx; fl_r = rcx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 6; continue; } bb = 2; continue; }
            2 => { r8 = rbx; bb = 5; continue; }
            3 => { fl_l = rbx; fl_r = rcx; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 6; continue; } bb = 4; continue; }
            4 => { r8 = rbx; r8 = ((r8).wrapping_shr(2u32)) as u64; fl_l = ((r8).wrapping_add(rbx)) as u64; fl_r = fl_l; r8 = ((fl_l as u64)) as u64; bb = 5; continue; }
            5 => { rdx = rdi; rcx = rdi; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(rdi)) as u64; crate::simd_ops::wu64(rdi.wrapping_add(P5), rbx); crate::simd_ops::wu32(rax.wrapping_add(rbx.wrapping_mul(4)).wrapping_add(P6), ((rsi as u32)) as u32); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141722000(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1416a7f00;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; r9 = (crate::simd_ops::ru64(rcx.wrapping_add(0x40u64))) as u64; r8 = (crate::simd_ops::ru64(rcx.wrapping_add(0x20u64))) as u64; r10 = ((rdx as u32)) as u32 as u64; r10 = ((r10).wrapping_shl(5u32)) as u64; rdx = (crate::simd_ops::ru32(r10.wrapping_add(r9).wrapping_add(0xcu64))) as u32 as u64; rdx = ((rdx).wrapping_shl(5u32)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(r9).wrapping_add(0xcu64))) as u32 as u64; rax = ((rax).wrapping_shl(5u32)) as u64; rax = (crate::simd_ops::ru32(rax.wrapping_add(r9).wrapping_add(0x4u64))) as u32 as u64; r12 = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(r9).wrapping_add(0x4u64))) as u32 as u64; rdx = r8.wrapping_add(0x4); r12 = ((r12).wrapping_shl(4u32)) as u64; r12 = ((r12).wrapping_add(r8)) as u64; r13 = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru32(r10.wrapping_add(r9).wrapping_add(0x10u64))) as u32 as u64; rax = ((rax).wrapping_shl(5u32)) as u64; r13 = ((r13).wrapping_shl(4u32)) as u64; r13 = ((r13).wrapping_add(r8)) as u64; rcx = (crate::simd_ops::ru32(rax.wrapping_add(r9).wrapping_add(0xcu64))) as u32 as u64; rcx = ((rcx).wrapping_shl(5u32)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(r9).wrapping_add(0xcu64))) as u32 as u64; rax = ((rax).wrapping_shl(5u32)) as u64; rax = (crate::simd_ops::ru32(rax.wrapping_add(r9).wrapping_add(0x4u64))) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru32(r10.wrapping_add(r9).wrapping_add(0x4u64))) as u32 as u64; r9 = (crate::simd_ops::ru32(r13.wrapping_add(0x4u64))) as u32 as u64; r10 = (crate::simd_ops::ru32(r13.wrapping_add(0x8u64))) as u32 as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rdx = ((rdx).wrapping_add(rcx)) as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rax = r8.wrapping_add(0x4); r8 = (crate::simd_ops::ru32(r12.wrapping_add(0x4u64))) as u32 as u64; rax = ((rax).wrapping_add(rcx)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), rax); rcx = ((r9 as u32)) as u32 as u64; r14 = (crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32 as u64; rbx = ((r8 as u32)) as u32 as u64; r11 = (crate::simd_ops::ru32(rdx)) as u32 as u64; rbx = (((rbx as u32)).wrapping_sub((r9 as u32))) as u32 as u64; rsi = (crate::simd_ops::ru32(rax)) as u32 as u64; r15 = ((r11 as u32)) as u32 as u64; rdi = (crate::simd_ops::ru32(rax.wrapping_add(0x4u64))) as u32 as u64; rcx = (((rcx as u32)).wrapping_sub((rsi as u32))) as u32 as u64; r15 = (((r15 as u32)).wrapping_sub((rsi as u32))) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rdx); rdx = (crate::simd_ops::ru32(r12.wrapping_add(0x8u64))) as u32 as u64; rax = ((r14 as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_sub((rdi as u32))) as u32 as u64; rbp = ((rdx as u32)) as u32 as u64; rcx = ((((rcx as u32)) as i32).wrapping_mul(((rax as u32)) as i32) as u32) as u32 as u64; rbp = (((rbp as u32)).wrapping_sub((r14 as u32))) as u32 as u64; rax = ((r10 as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_sub((rdi as u32))) as u32 as u64; r15 = ((((r15 as u32)) as i32).wrapping_mul(((rax as u32)) as i32) as u32) as u32 as u64; rax = ((r10 as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_sub((r14 as u32))) as u32 as u64; r14 = (((r14 as u32)).wrapping_sub((rdx as u32))) as u32 as u64; r15 = (((r15 as u32)).wrapping_sub((rcx as u32))) as u32 as u64; rcx = ((r8 as u32)) as u32 as u64; rcx = (((rcx as u32)).wrapping_sub((r11 as u32))) as u32 as u64; rcx = ((((rcx as u32)) as i32).wrapping_mul(((rax as u32)) as i32) as u32) as u32 as u64; rax = ((r9 as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_sub((r11 as u32))) as u32 as u64; r11 = (((r11 as u32)).wrapping_sub((r8 as u32))) as u32 as u64; rbp = ((((rbp as u32)) as i32).wrapping_mul(((rax as u32)) as i32) as u32) as u32 as u64; rax = ((rdx as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_sub((r10 as u32))) as u32 as u64; rbp = (((rbp as u32)).wrapping_sub((rcx as u32))) as u32 as u64; rcx = ((rsi as u32)) as u32 as u64; rcx = (((rcx as u32)).wrapping_sub((r9 as u32))) as u32 as u64; rsi = (((rsi as u32)).wrapping_sub((r8 as u32))) as u32 as u64; rcx = ((((rcx as u32)) as i32).wrapping_mul(((rax as u32)) as i32) as u32) as u32 as u64; rax = ((rdi as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_sub((r10 as u32))) as u32 as u64; rdi = (((rdi as u32)).wrapping_sub((rdx as u32))) as u32 as u64; rbx = ((((rbx as u32)) as i32).wrapping_mul(((rax as u32)) as i32) as u32) as u32 as u64; rdi = ((((rdi as u32)) as i32).wrapping_mul(((r11 as u32)) as i32) as u32) as u32 as u64; rbx = (((rbx as u32)).wrapping_sub((rcx as u32))) as u32 as u64; r14 = ((((r14 as u32)) as i32).wrapping_mul(((rsi as u32)) as i32) as u32) as u32 as u64; r14 = (((r14 as u32)).wrapping_sub((rdi as u32))) as u32 as u64; rax = ((rbx as u32)) as u32 as u64; rax = (0u32.wrapping_sub((rax as u32))) as u32 as u64; rbx = (((((rbx as u32)) as i32).wrapping_shr(31u32) as u32)) as u32 as u64; rax = (((((rax as u32)) as i32).wrapping_shr(31u32) as u32)) as u32 as u64; rcx = ((r14 as u32)) as u32 as u64; rcx = (0u32.wrapping_sub((rcx as u32))) as u32 as u64; r14 = (((((r14 as u32)) as i32).wrapping_shr(31u32) as u32)) as u32 as u64; rcx = (((((rcx as u32)) as i32).wrapping_shr(31u32) as u32)) as u32 as u64; r14 = (((r14 as u32)).wrapping_sub(1)) as u32 as u64; rcx = ((((rcx as u32)) & ((rax as u32)))) as u32 as u64; rbx = (((rbx as u32)).wrapping_sub(1)) as u32 as u64; rax = ((rbp as u32)) as u32 as u64; rbp = (((((rbp as u32)) as i32).wrapping_shr(31u32) as u32)) as u32 as u64; rax = (0u32.wrapping_sub((rax as u32))) as u32 as u64; rbp = (((rbp as u32)).wrapping_sub(1)) as u32 as u64; rax = (((((rax as u32)) as i32).wrapping_shr(31u32) as u32)) as u32 as u64; rcx = ((((rcx as u32)) & ((rax as u32)))) as u32 as u64; rax = ((r15 as u32)) as u32 as u64; rax = (0u32.wrapping_sub((rax as u32))) as u32 as u64; r15 = (((((r15 as u32)) as i32).wrapping_shr(31u32) as u32)) as u32 as u64; rax = (((((rax as u32)) as i32).wrapping_shr(31u32) as u32)) as u32 as u64; r15 = (((r15 as u32)).wrapping_sub(1)) as u32 as u64; rcx = ((((rcx as u32)) & ((rax as u32)))) as u32 as u64; rcx = ((((rcx as u32)) & (0x1u32))) as u32 as u64; rcx = ((((rcx as u32)) & ((r14 as u32)))) as u32 as u64; rcx = ((((rcx as u32)) & ((rbx as u32)))) as u32 as u64; rcx = ((((rcx as u32)) & ((rbp as u32)))) as u32 as u64; fl_l = ((r15 as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 3; continue; }
            2 => { rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; r9 = r12.wrapping_add(0x4); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; r8 = r13.wrapping_add(0x4); rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; r8 = r12.wrapping_add(0x4); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rdx = r13.wrapping_add(0x4); rbx = rax; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rax = (0u64.wrapping_sub(rax)) as u64; rbx = (0u64.wrapping_sub(rbx)) as u64; rax = ((((rax) as i64).wrapping_shr(63u32) as u64)) as u64; rbx = ((((rbx) as i64).wrapping_shr(63u32) as u64)) as u64; rax = ((rax).wrapping_add(1)) as u64; rbx = ((rbx).wrapping_add(1)) as u64; fl_l = rbx; fl_r = rax; rax = (rax & !0xffu64) | (((if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { 1u8 } else { 0u8 }) as u8 as u64)); bb = 3; continue; }
            3 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1417221f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x40; const P1: u64 = 0x20; const P2: u64 = 0x4; const P3: u64 = 0xc; const P4: u64 = 0x10; const P5: u64 = 0x1416a7f00;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; r10 = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; r11 = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; r9 = ((rdx as u32)) as u32 as u64; r9 = ((r9).wrapping_shl(5u32)) as u64; rbx = ((r8 as u32)) as u32 as u64; rax = (crate::simd_ops::ru32(r9.wrapping_add(r10).wrapping_add(P2))) as u32 as u64; rdx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru32(r9.wrapping_add(r10).wrapping_add(P3))) as u32 as u64; rax = ((rax).wrapping_shl(5u32)) as u64; rdx = ((rdx).wrapping_shl(4u32)) as u64; rdx = ((rdx).wrapping_add(0x4u64)) as u64; rdx = ((rdx).wrapping_add(r11)) as u64; rax = (crate::simd_ops::ru32(rax.wrapping_add(r10).wrapping_add(P2))) as u32 as u64; r8 = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru32(r9.wrapping_add(r10).wrapping_add(P4))) as u32 as u64; rax = ((rax).wrapping_shl(5u32)) as u64; r8 = ((r8).wrapping_shl(4u32)) as u64; r8 = ((r8).wrapping_add(0x4u64)) as u64; r8 = ((r8).wrapping_add(r11)) as u64; rcx = (crate::simd_ops::ru32(rax.wrapping_add(r10).wrapping_add(P3))) as u32 as u64; rcx = ((rcx).wrapping_shl(5u32)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(r10).wrapping_add(P3))) as u32 as u64; rcx = ((rbx.wrapping_add(rbx.wrapping_mul(2))) as u64) as u64; rax = ((rax).wrapping_shl(5u32)) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rax = (crate::simd_ops::ru32(rax.wrapping_add(r10).wrapping_add(P2))) as u32 as u64; r9 = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; r9 = ((r9).wrapping_shl(4u32)) as u64; r9 = ((r9).wrapping_add(0x4u64)) as u64; r9 = ((r9).wrapping_add(r11)) as u64; r11 = ((r11).wrapping_add(0x4u64)) as u64; rcx = ((rcx).wrapping_add(r11)) as u64; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; rax = (rax & !0xffu64) | (((if (((((fl_l as u64)) & ((fl_r as u64))) as i64) <= 0) { 1u8 } else { 0u8 }) as u8 as u64)); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141722290(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x8; const P2: u64 = 0xffffffffffffffff; const P3: u64 = 0x1411fa5f0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = ((rcx.wrapping_add(P0)) as u64) as u64; rax = 0; fl_l = 0; fl_r = 0; rcx = (crate::simd_ops::ru32(rdi.wrapping_add(P1))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 14; continue; } bb = 1; continue; }
            1 => { r8 = (crate::simd_ops::ru64(rdi)) as u64; bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru32(r8.wrapping_add(rax.wrapping_mul(4)))) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 5; continue; } bb = 3; continue; }
            3 => { rax = (((rax as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 2; continue; } bb = 4; continue; }
            4 => { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            5 => { r9 = ((rcx.wrapping_add(P2)) as u32) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru32(r8.wrapping_add(r9.wrapping_mul(4)))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(rax.wrapping_mul(4)), ((rcx as u32)) as u32); bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(P0))) as u64; rbx = ((r9 as u32)) as u32 as u64; fl_l = rbx; fl_r = (crate::simd_ops::ru64(rdi.wrapping_add(P1))) as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 10; continue; } bb = 8; continue; }
            8 => { rax = ((rax).wrapping_shr(1u32)) as u64; fl_l = rbx; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 13; continue; } bb = 9; continue; }
            9 => { r8 = ((rbx as u32)) as u32 as u64; bb = 12; continue; }
            10 => { fl_l = rbx; fl_r = rax; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 13; continue; } bb = 11; continue; }
            11 => { r8 = rbx; r8 = ((r8).wrapping_shr(2u32)) as u64; fl_l = ((r8).wrapping_add(rbx)) as u64; fl_r = fl_l; r8 = ((fl_l as u64)) as u64; bb = 12; continue; }
            12 => { rdx = rdi; rcx = rdi; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 13; continue; }
            13 => { crate::simd_ops::wu64(rdi.wrapping_add(P1), rbx); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; bb = 14; continue; }
            14 => { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141722320(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x143932190; const P2: u64 = 0x60; const P3: u64 = 0x3; const P4: u64 = 0x50; const P5: u64 = 0x48;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = rdx; rbx = rcx; fl_l = r8; fl_r = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 8; continue; } bb = 1; continue; }
            1 => { rdx = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rdi); fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 4; continue; } bb = 2; continue; }
            2 => { rdi = 0; fl_l = 0; fl_r = 0; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(P1)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx, rdi); crate::simd_ops::wu64(rbx.wrapping_add(P0), rdi); rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            4 => { r9 = (crate::simd_ops::ru64(rax)) as u64; rdi = ((r8.wrapping_add(P3)) as u64) as u64; rdi = (((rdi) & (0xfffffffffffffffcu64))) as u64; rcx = ((rdi.wrapping_mul(4)) as u64) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { r8 = rcx; rcx = rax; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(r9.wrapping_add(P4))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx, rax); crate::simd_ops::wu64(rbx.wrapping_add(P0), rdi); rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            6 => { rdx = rcx; crate::simd_ops::wu32(rsp.wrapping_add(0x48u64), (0x2u32) as u32); rcx = rax; r8 = rsp.wrapping_add(0x48); rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(r9.wrapping_add(P5))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx, rax); bb = 7; continue; }
            7 => { crate::simd_ops::wu64(rbx.wrapping_add(P0), rdi); rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; bb = 8; continue; }
            8 => { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1417223d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x143932190; const P2: u64 = 0x60; const P3: u64 = 0x3; const P4: u64 = 0x50; const P5: u64 = 0x48;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; r9 = rdx; rbx = rcx; fl_l = r8; fl_r = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 8; continue; } bb = 1; continue; }
            1 => { rdx = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rdi); fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 4; continue; } bb = 2; continue; }
            2 => { rdi = 0; fl_l = 0; fl_r = 0; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(P1)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx, rdi); crate::simd_ops::wu64(rbx.wrapping_add(P0), rdi); rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            4 => { r10 = (crate::simd_ops::ru64(r9)) as u64; rdi = ((r8.wrapping_add(P3)) as u64) as u64; rdi = (((rdi) & (0xfffffffffffffffcu64))) as u64; rcx = r9; rax = rdi; rax = ((rax).wrapping_shl(5u32)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { r8 = rax; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(r10.wrapping_add(P4))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx, rax); crate::simd_ops::wu64(rbx.wrapping_add(P0), rdi); rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            6 => { r8 = rsp.wrapping_add(0x48); crate::simd_ops::wu32(rsp.wrapping_add(0x48u64), (0x2u32) as u32); rdx = rax; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(r10.wrapping_add(P5))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx, rax); bb = 7; continue; }
            7 => { crate::simd_ops::wu64(rbx.wrapping_add(P0), rdi); rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; bb = 8; continue; }
            8 => { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141722480(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x143932190; const P2: u64 = 0x60; const P3: u64 = 0x3; const P4: u64 = 0x50; const P5: u64 = 0x48;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; r9 = rdx; rbx = rcx; fl_l = r8; fl_r = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 8; continue; } bb = 1; continue; }
            1 => { rdx = (crate::simd_ops::ru64(rcx)) as u64; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 4; continue; } bb = 2; continue; }
            2 => { rdi = 0; fl_l = 0; fl_r = 0; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(P1)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx, rdi); crate::simd_ops::wu64(rbx.wrapping_add(P0), rdi); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            4 => { rdi = ((r8.wrapping_add(P3)) as u64) as u64; rdi = (((rdi) & (0xfffffffffffffffcu64))) as u64; rax = ((rdi.wrapping_add(rdi.wrapping_mul(2))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; r10 = (crate::simd_ops::ru64(r9)) as u64; rcx = r9; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { r8 = rax; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(r10.wrapping_add(P4))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx, rax); crate::simd_ops::wu64(rbx.wrapping_add(P0), rdi); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            6 => { crate::simd_ops::wu32(rsp.wrapping_add(0x48u64), (0x2u32) as u32); r8 = rsp.wrapping_add(0x48); rdx = rax; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(r10.wrapping_add(P5))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx, rax); bb = 7; continue; }
            7 => { crate::simd_ops::wu64(rbx.wrapping_add(P0), rdi); bb = 8; continue; }
            8 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141722590(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x18; const P2: u64 = 0x10; const P3: u64 = 0x1417223d0; const P4: u64 = 0x14; const P5: u64 = 0x20;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rbx = rdx; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; rdi = rcx; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64; fl_l = rbx; fl_r = rsi; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 4; continue; } bb = 1; continue; }
            1 => { rax = ((rax).wrapping_shr(1u32)) as u64; fl_l = rbx; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 3; continue; } bb = 2; continue; }
            2 => { r8 = rbx; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 3; continue; }
            3 => { crate::simd_ops::wu64(rdi.wrapping_add(P0), rbx); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            4 => { fl_l = rbx; fl_r = rax; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { r8 = rbx; r8 = ((r8).wrapping_shr(2u32)) as u64; r8 = ((r8).wrapping_add(rbx)) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            6 => { crate::simd_ops::wu64(rdi.wrapping_add(P0), rbx); fl_l = rbx; fl_r = rsi; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 10; continue; } bb = 7; continue; }
            7 => { rax = rsi; rax = ((rax).wrapping_shl(5u32)) as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(rdi))) as u64; fl_l = rbx; fl_r = rsi; rbx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 10; continue; } bb = 8; continue; }
            8 => { rcx = 0; fl_l = 0; fl_r = 0; bb = 9; continue; }
            9 => { crate::simd_ops::wu64(rax, (0xffffffffffffffffu64) as u64); crate::simd_ops::wu64(rax.wrapping_add(P0), (0xffffffffffffffffu64) as u64); crate::simd_ops::wu32(rax.wrapping_add(P2), (0xffffffffu32) as u32); crate::simd_ops::wu32(rax.wrapping_add(P4), (0x1u32) as u32); crate::simd_ops::wu64(rax.wrapping_add(P1), rcx); rax = ((rax.wrapping_add(P5)) as u64) as u64; fl_l = rbx; fl_r = (0x1u64) as u64; rbx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 9; continue; } bb = 10; continue; }
            10 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141722650(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x18; const P2: u64 = 0x10; const P3: u64 = 0x1416ba780;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rbx = rdx; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; rdi = rcx; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64; fl_l = rbx; fl_r = rsi; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 4; continue; } bb = 1; continue; }
            1 => { rax = ((rax).wrapping_shr(1u32)) as u64; fl_l = rbx; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 3; continue; } bb = 2; continue; }
            2 => { r8 = rbx; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 3; continue; }
            3 => { crate::simd_ops::wu64(rdi.wrapping_add(P0), rbx); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            4 => { fl_l = rbx; fl_r = rax; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { r8 = rbx; r8 = ((r8).wrapping_shr(2u32)) as u64; r8 = ((r8).wrapping_add(rbx)) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            6 => { crate::simd_ops::wu64(rdi.wrapping_add(P0), rbx); fl_l = rbx; fl_r = rsi; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 10; continue; } bb = 7; continue; }
            7 => { rax = rsi; rax = ((rax).wrapping_shl(4u32)) as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(rdi))) as u64; fl_l = rbx; fl_r = rsi; rbx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 10; continue; } bb = 8; continue; }
            8 => { rcx = 0; fl_l = 0; fl_r = 0; bb = 9; continue; }
            9 => { crate::simd_ops::wu64(rax, (0xffffffffffffffffu64) as u64); crate::simd_ops::wu64(rax.wrapping_add(P0), rcx); rax = ((rax.wrapping_add(P2)) as u64) as u64; fl_l = rbx; fl_r = (0x1u64) as u64; rbx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 9; continue; } bb = 10; continue; }
            10 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141722700(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143932190; const P1: u64 = 0x141722480;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rdx; rbp = rcx; rdi = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; r14 = (crate::simd_ops::ru64(rcx.wrapping_add(0x18u64))) as u64; fl_l = rdx; fl_r = rdi; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 9; continue; } bb = 1; continue; }
            1 => { rsi = rdi; rdi = ((rdi.wrapping_add(rdi.wrapping_mul(2))) as u64) as u64; rdi = ((rdi).wrapping_shl(4u32)) as u64; rdi = ((rdi).wrapping_add(0xffffffffffffffe0u64)) as u64; fl_l = rsi; fl_r = rdx; rsi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 6; continue; } bb = 2; continue; }
            2 => { fl_l = ((rdi).wrapping_add(crate::simd_ops::ru64(rcx))) as u64; fl_r = fl_l; rdi = ((fl_l as u64)) as u64; bb = 3; continue; }
            3 => { rdx = (crate::simd_ops::ru64(rdi)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru64(P0)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x60u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            5 => { rdi = ((rdi).wrapping_sub(0x30u64)) as u64; fl_l = rsi; fl_r = (0x1u64) as u64; rsi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x10u64))) as u64; rax = ((rax).wrapping_shr(1u32)) as u64; fl_l = rbx; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 8; continue; } bb = 7; continue; }
            7 => { r8 = rbx; rdx = r14; rcx = rbp; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 8; continue; }
            8 => { crate::simd_ops::wu64(rbp.wrapping_add(0x8u64), rbx); bb = 15; continue; }
            9 => { fl_l = rbx; fl_r = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 11; continue; } bb = 10; continue; }
            10 => { r8 = rbx; r8 = ((r8).wrapping_shr(2u32)) as u64; r8 = ((r8).wrapping_add(rbx)) as u64; rdx = r14; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 11; continue; }
            11 => { crate::simd_ops::wu64(rbp.wrapping_add(0x8u64), rbx); fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 15; continue; } bb = 12; continue; }
            12 => { rsi = ((rdi.wrapping_add(rdi.wrapping_mul(2))) as u64) as u64; rsi = ((rsi).wrapping_shl(4u32)) as u64; rsi = ((rsi).wrapping_add(crate::simd_ops::ru64(rbp))) as u64; fl_l = rbx; fl_r = rdi; rbx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 15; continue; } bb = 13; continue; }
            13 => { rdi = rsi.wrapping_add(0x20); rbp = 0; fl_l = 0; fl_r = 0; bb = 14; continue; }
            14 => { crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rsi); crate::simd_ops::wu32(rsi, (0xffffffffu32) as u32); crate::simd_ops::wu32(rdi.wrapping_sub(0x1cu64), (0x7fffffffu32) as u32); crate::simd_ops::wu32(rdi.wrapping_sub(0x18u64), (0x7fffffffu32) as u32); crate::simd_ops::wu32(rdi.wrapping_sub(0x14u64), (0x7f7fffffu32) as u32); crate::simd_ops::wu64(rdi.wrapping_sub(0x10u64), rbp); crate::simd_ops::wu64(rdi.wrapping_sub(0x8u64), rbp); crate::simd_ops::wu64(rdi, rbp); crate::simd_ops::wu64(rdi.wrapping_add(0x8u64), rbp); crate::simd_ops::wu32(rsp.wrapping_add(0x40u64), (0x2u32) as u32); rcx = (crate::simd_ops::ru64(P0)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; r8 = rsp.wrapping_add(0x40); rdx = 0x30; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x48u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rdi.wrapping_sub(0x10u64), rax); crate::simd_ops::wu64(rdi, (0xcu64) as u64); rsi = ((rsi).wrapping_add(0x30u64)) as u64; rdi = rdi.wrapping_add(0x30); fl_l = rbx; fl_r = (0x1u64) as u64; rbx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 14; continue; } bb = 15; continue; }
            15 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141722840(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1417221f0; const P1: u64 = 0x141722320; const P2: u64 = 0x141722520; const P3: u64 = 0x141722000; const P4: u64 = 0x1417218e0; const P5: u64 = 0x1411ff9f0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; r13 = 0; fl_l = 0; fl_r = 0; r15 = ((rdx as u32)) as u32 as u64; rdi = rcx; rbp = ((r13 as u32)) as u32 as u64; r12 = ((r15.wrapping_add(r15.wrapping_mul(2))) as u64) as u64; r12 = ((r12).wrapping_shl(4u32)) as u64; r12 = ((r12).wrapping_add(crate::simd_ops::ru64(rcx.wrapping_add(0x20u64)))) as u64; fl_l = (crate::simd_ops::ru32(r12.wrapping_add(0x18u64))) as u32 as u64; fl_r = ((r13 as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 13; continue; } bb = 1; continue; }
            1 => { bb = 2; continue; }
            2 => { r8 = (crate::simd_ops::ru64(rdi.wrapping_add(0x40u64))) as u64; rax = (crate::simd_ops::ru64(r12.wrapping_add(0x10u64))) as u64; rcx = ((rbp as u32)) as u32 as u64; rdx = (crate::simd_ops::ru32(rax.wrapping_add(rcx.wrapping_mul(4)))) as u32 as u64; rdx = ((rdx).wrapping_shl(5u32)) as u64; r14 = (crate::simd_ops::ru32(rdx.wrapping_add(r8).wrapping_add(0xcu64))) as u32 as u64; rbx = ((r14 as u32)) as u32 as u64; rbx = ((rbx).wrapping_shl(5u32)) as u64; rbx = ((rbx).wrapping_add(r8)) as u64; fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x10u64))) as u32 as u64; fl_r = 0xffffffff; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 12; continue; } bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x14u64))) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 12; continue; } bb = 4; continue; }
            4 => { r8 = ((r15 as u32)) as u32 as u64; rdx = ((r14 as u32)) as u32 as u64; rcx = rdi; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 12; continue; } bb = 5; continue; }
            5 => { rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x10u64))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x14u64), ((r13 as u32)) as u32); rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x40u64))) as u64; rcx = ((rcx).wrapping_shl(5u32)) as u64; crate::simd_ops::wu32(rcx.wrapping_add(rax).wrapping_add(0x14u64), ((r13 as u32)) as u32); rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x68u64))) as u64; rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x78u64))) as u64; rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x70u64))) as u64; rbx = rax.wrapping_add(0x1); fl_l = rbx; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 8; continue; } bb = 6; continue; }
            6 => { rcx = ((rcx).wrapping_shr(1u32)) as u64; fl_l = rbx; fl_r = rcx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 11; continue; } bb = 7; continue; }
            7 => { r8 = rbx; bb = 10; continue; }
            8 => { fl_l = rbx; fl_r = rcx; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 11; continue; } bb = 9; continue; }
            9 => { r8 = rbx; r8 = ((r8).wrapping_shr(2u32)) as u64; fl_l = ((r8).wrapping_add(rbx)) as u64; fl_r = fl_l; r8 = ((fl_l as u64)) as u64; bb = 10; continue; }
            10 => { rcx = rdi.wrapping_add(0x60); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 11; continue; }
            11 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x60u64))) as u64; crate::simd_ops::wu64(rdi.wrapping_add(0x68u64), rbx); crate::simd_ops::wu32(rax.wrapping_add(rbx.wrapping_mul(4)).wrapping_sub(0x4u64), ((r14 as u32)) as u32); bb = 12; continue; }
            12 => { rbp = (((rbp as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((rbp as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(r12.wrapping_add(0x18u64))) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 2; continue; } bb = 13; continue; }
            13 => { r13 = (crate::simd_ops::ru32(rdi.wrapping_add(0x48u64))) as u32 as u64; r13 = (((r13 as u32)).wrapping_add((r13 as u32))) as u32 as u64; fl_l = (crate::simd_ops::ru64(rdi.wrapping_add(0x68u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 42; continue; } bb = 14; continue; }
            14 => { bb = 15; continue; }
            15 => { rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x68u64))) as u64; rcx = rdi.wrapping_add(0x60); rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x60u64))) as u64; rbp = (crate::simd_ops::ru32(rax.wrapping_add(rdx.wrapping_mul(4)).wrapping_sub(0x4u64))) as u32 as u64; rdx = ((rdx).wrapping_sub(1)) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rsi = ((rbp as u32)) as u32 as u64; rdx = ((rbp as u32)) as u32 as u64; rsi = ((rsi).wrapping_shl(5u32)) as u64; r13 = (((r13 as u32)).wrapping_sub(1)) as u32 as u64; rsi = ((rsi).wrapping_add(crate::simd_ops::ru64(rdi.wrapping_add(0x40u64)))) as u64; rcx = (crate::simd_ops::ru32(rsi.wrapping_add(0x10u64))) as u32 as u64; crate::simd_ops::wu32(rsi.wrapping_add(0x14u64), (0x1u32) as u32); rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x40u64))) as u64; rcx = ((rcx).wrapping_shl(5u32)) as u64; crate::simd_ops::wu32(rcx.wrapping_add(rax).wrapping_add(0x14u64), (0x1u32) as u32); rcx = rdi; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 39; continue; } bb = 16; continue; }
            16 => { r8 = ((rbp as u32)) as u32 as u64; rdx = rsp.wrapping_add(0x58); rcx = rdi; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); fl_l = ((r13 as u32)) as u32 as u64; fl_r = ((r13 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 41; continue; } bb = 17; continue; }
            17 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x40u64))) as u64; rax = (crate::simd_ops::ru32(rsi.wrapping_add(0xcu64))) as u32 as u64; rax = ((rax).wrapping_shl(5u32)) as u64; r14 = (crate::simd_ops::ru32(rax.wrapping_add(rcx).wrapping_add(0xcu64))) as u32 as u64; rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x10u64))) as u32 as u64; rax = ((rax).wrapping_shl(5u32)) as u64; r14 = ((r14).wrapping_shl(5u32)) as u64; r14 = ((r14).wrapping_add(rcx)) as u64; r15 = (crate::simd_ops::ru32(rax.wrapping_add(rcx).wrapping_add(0xcu64))) as u32 as u64; r15 = ((r15).wrapping_shl(5u32)) as u64; rax = (crate::simd_ops::ru32(r14.wrapping_add(0x14u64))) as u32 as u64; r15 = ((r15).wrapping_add(rcx)) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 28; continue; } bb = 18; continue; }
            18 => { fl_l = (crate::simd_ops::ru32(r14.wrapping_add(0x10u64))) as u32 as u64; fl_r = 0xffffffff; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 28; continue; } bb = 19; continue; }
            19 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 28; continue; } bb = 20; continue; }
            20 => { r8 = (crate::simd_ops::ru32(r12)) as u32 as u64; rcx = rdi; rdx = (crate::simd_ops::ru32(r14)) as u32 as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 28; continue; } bb = 21; continue; }
            21 => { rcx = (crate::simd_ops::ru32(r14.wrapping_add(0x10u64))) as u32 as u64; crate::simd_ops::wu32(r14.wrapping_add(0x14u64), (0x0u32) as u32); rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x40u64))) as u64; rcx = ((rcx).wrapping_shl(5u32)) as u64; crate::simd_ops::wu32(rcx.wrapping_add(rax).wrapping_add(0x14u64), (0x0u32) as u32); rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x68u64))) as u64; rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x78u64))) as u64; rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x70u64))) as u64; rsi = rax.wrapping_add(0x1); fl_l = rsi; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 24; continue; } bb = 22; continue; }
            22 => { rcx = ((rcx).wrapping_shr(1u32)) as u64; fl_l = rsi; fl_r = rcx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 27; continue; } bb = 23; continue; }
            23 => { r8 = rsi; bb = 26; continue; }
            24 => { fl_l = rsi; fl_r = rcx; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 27; continue; } bb = 25; continue; }
            25 => { r8 = rsi; r8 = ((r8).wrapping_shr(2u32)) as u64; fl_l = ((r8).wrapping_add(rsi)) as u64; fl_r = fl_l; r8 = ((fl_l as u64)) as u64; bb = 26; continue; }
            26 => { rcx = rdi.wrapping_add(0x60); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 27; continue; }
            27 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x60u64))) as u64; crate::simd_ops::wu64(rdi.wrapping_add(0x68u64), rsi); rax = (crate::simd_ops::ru32(r14)) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(rsi.wrapping_mul(4)).wrapping_sub(0x4u64), ((rax as u32)) as u32); bb = 28; continue; }
            28 => { rax = (crate::simd_ops::ru32(r15.wrapping_add(0x14u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 39; continue; } bb = 29; continue; }
            29 => { fl_l = (crate::simd_ops::ru32(r15.wrapping_add(0x10u64))) as u32 as u64; fl_r = 0xffffffff; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 39; continue; } bb = 30; continue; }
            30 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 39; continue; } bb = 31; continue; }
            31 => { r8 = (crate::simd_ops::ru32(r12)) as u32 as u64; rcx = rdi; rdx = (crate::simd_ops::ru32(r15)) as u32 as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 39; continue; } bb = 32; continue; }
            32 => { rcx = (crate::simd_ops::ru32(r15.wrapping_add(0x10u64))) as u32 as u64; crate::simd_ops::wu32(r15.wrapping_add(0x14u64), (0x0u32) as u32); rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x40u64))) as u64; rcx = ((rcx).wrapping_shl(5u32)) as u64; crate::simd_ops::wu32(rcx.wrapping_add(rax).wrapping_add(0x14u64), (0x0u32) as u32); rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x68u64))) as u64; rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x78u64))) as u64; rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x70u64))) as u64; rsi = rax.wrapping_add(0x1); fl_l = rsi; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 35; continue; } bb = 33; continue; }
            33 => { rcx = ((rcx).wrapping_shr(1u32)) as u64; fl_l = rsi; fl_r = rcx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 38; continue; } bb = 34; continue; }
            34 => { r8 = rsi; bb = 37; continue; }
            35 => { fl_l = rsi; fl_r = rcx; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 38; continue; } bb = 36; continue; }
            36 => { r8 = rsi; r8 = ((r8).wrapping_shr(2u32)) as u64; fl_l = ((r8).wrapping_add(rsi)) as u64; fl_r = fl_l; r8 = ((fl_l as u64)) as u64; bb = 37; continue; }
            37 => { rcx = rdi.wrapping_add(0x60); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 38; continue; }
            38 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x60u64))) as u64; crate::simd_ops::wu64(rdi.wrapping_add(0x68u64), rsi); rax = (crate::simd_ops::ru32(r15)) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(rsi.wrapping_mul(4)).wrapping_sub(0x4u64), ((rax as u32)) as u32); bb = 39; continue; }
            39 => { fl_l = (crate::simd_ops::ru64(rdi.wrapping_add(0x68u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 15; continue; } bb = 40; continue; }
            40 => { bb = 42; continue; }
            41 => { rcx = rdi.wrapping_add(0x60); rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); bb = 42; continue; }
            42 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141722b30(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143428720; const P1: u64 = 0x1434268d0; const P2: u64 = 0x143427080; const P3: u64 = 0x143428760;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut xmm4: u128 = 0; let mut xmm5: u128 = 0; let mut xmm6: u128 = 0; let mut xmm8: u128 = 0; let mut xmm9: u128 = 0; let mut xmm10: u128 = 0; let mut xmm11: u128 = 0; let mut xmm12: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x68u64)) as u64; rax = ((r8.wrapping_sub(0x2u64)) as u32) as u32 as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x10u64), (xmm11) as u128); crate::simd_ops::wu128(rsp, (xmm12) as u128); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 21; continue; } bb = 1; continue; }
            1 => { xmm1 = ((crate::simd_ops::ru32(r9.wrapping_add(0x4u64))) as u32) as u128; xmm4 = ((crate::simd_ops::ru32(rdx)) as u32) as u128; xmm5 = (xmm1) as u128; xmm3 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x90u64))) as u32) as u128; xmm11 = (xmm1) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x50u64), (xmm6) as u128); xmm0 = (xmm4) as u128; xmm6 = ((crate::simd_ops::ru32(rcx.wrapping_add(0x4u64))) as u32) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm6 as u32)).to_bits() as u128); crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm8) as u128); xmm8 = ((crate::simd_ops::ru32(r9)) as u32) as u128; xmm2 = (xmm8) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm9) as u128); xmm9 = ((crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm10) as u128); xmm10 = ((crate::simd_ops::ru32(rcx)) as u32) as u128; xmm12 = (xmm10) as u128; xmm5 = (xmm5 & !0xffff_ffffu128) | ((f32::from_bits(xmm5 as u32) * f32::from_bits(xmm10 as u32)).to_bits() as u128); xmm12 = (xmm12 & !0xffff_ffffu128) | ((f32::from_bits(xmm12 as u32) * f32::from_bits(xmm9 as u32)).to_bits() as u128); xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) * f32::from_bits(xmm9 as u32)).to_bits() as u128); xmm12 = (xmm12 & !0xffff_ffffu128) | ((f32::from_bits(xmm12 as u32) - f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm11 = (xmm11 & !0xffff_ffffu128) | ((f32::from_bits(xmm11 as u32) * f32::from_bits(xmm6 as u32)).to_bits() as u128); xmm0 = (xmm8) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm6 as u32)).to_bits() as u128); xmm5 = (xmm5 & !0xffff_ffffu128) | ((f32::from_bits(xmm5 as u32) - f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm0 = (xmm1) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm4 as u32)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm9 as u32)).to_bits() as u128); xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) - f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm0 = (xmm8) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm10 as u32)).to_bits() as u128); xmm8 = (xmm8 & !0xffff_ffffu128) | ((f32::from_bits(xmm8 as u32) * f32::from_bits(xmm4 as u32)).to_bits() as u128); xmm11 = (xmm11 & !0xffff_ffffu128) | ((f32::from_bits(xmm11 as u32) + f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm0 = (xmm12) as u128; xmm0 = (xmm0) & (crate::simd_ops::ru128(P0)); fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = ((xmm3 as u32)) as u32 as u64; xmm8 = (xmm8 & !0xffff_ffffu128) | ((f32::from_bits(xmm8 as u32) + f32::from_bits(xmm1 as u32)).to_bits() as u128); xmm1 = 0u128; if (f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 4; continue; } bb = 2; continue; }
            2 => { xmm4 = (xmm4 & !0xffff_ffffu128) | ((f32::from_bits(xmm4 as u32) * f32::from_bits(xmm10 as u32)).to_bits() as u128); xmm6 = (xmm6 & !0xffff_ffffu128) | ((f32::from_bits(xmm6 as u32) * f32::from_bits(xmm9 as u32)).to_bits() as u128); xmm4 = (xmm4 & !0xffff_ffffu128) | ((f32::from_bits(xmm4 as u32) + f32::from_bits(xmm6 as u32)).to_bits() as u128); fl_l = ((xmm4 as u32)) as u32 as u64; fl_r = ((xmm1 as u32)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 5; continue; }
            4 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 5; continue; }
            5 => { xmm10 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; xmm0 = (xmm2) as u128; xmm0 = (xmm0) & (crate::simd_ops::ru128(P0)); fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = ((xmm3 as u32)) as u32 as u64; xmm9 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x50u64))) as u128; if (f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 8; continue; } bb = 6; continue; }
            6 => { fl_l = ((xmm8 as u32)) as u32 as u64; fl_r = ((xmm1 as u32)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 8; continue; } bb = 7; continue; }
            7 => { rcx = (rcx & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 9; continue; }
            8 => { rcx = (rcx & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 9; continue; }
            9 => { xmm8 = (crate::simd_ops::ru128(rsp.wrapping_add(0x40u64))) as u128; xmm0 = (xmm5) as u128; xmm0 = (xmm0) & (crate::simd_ops::ru128(P0)); fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = ((xmm3 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 12; continue; } bb = 10; continue; }
            10 => { fl_l = ((xmm11 as u32)) as u32 as u64; fl_r = ((xmm1 as u32)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 12; continue; } bb = 11; continue; }
            11 => { rax = (((rax as u8)) as u32) as u32 as u64; rax = ((((rax as u32)) ^ (0x1u32))) as u32 as u64; rax = (((rax as u32)).wrapping_add(1)) as u32 as u64; xmm11 = (crate::simd_ops::ru128(rsp.wrapping_add(0x10u64))) as u128; xmm12 = (crate::simd_ops::ru128(rsp)) as u128; rsp = ((rsp).wrapping_add(0x68u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            12 => { fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 14; continue; } bb = 13; continue; }
            13 => { rax = (((rax as u8)) as u32) as u32 as u64; rax = ((((rax as u32)) ^ (0x1u32))) as u32 as u64; rax = ((rax.wrapping_mul(2).wrapping_add(0x1u64)) as u32) as u32 as u64; xmm11 = (crate::simd_ops::ru128(rsp.wrapping_add(0x10u64))) as u128; xmm12 = (crate::simd_ops::ru128(rsp)) as u128; rsp = ((rsp).wrapping_add(0x68u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            14 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 21; continue; } bb = 15; continue; }
            15 => { fl_l = ((r8 as u32)) as u32 as u64; fl_r = 0x3; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 17; continue; } bb = 16; continue; }
            16 => { xmm0 = ((crate::simd_ops::ru32(P1)) as u32) as u128; bb = 18; continue; }
            17 => { xmm0 = ((crate::simd_ops::ru32(P2)) as u32) as u128; bb = 18; continue; }
            18 => { xmm4 = (xmm0) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm5 as u32)).to_bits() as u128); xmm4 = (xmm4 & !0xffff_ffffu128) | ((f32::from_bits(xmm4 as u32) * f32::from_bits(xmm12 as u32)).to_bits() as u128); fl_l = ((xmm4 as u32)) as u32 as u64; fl_r = ((xmm1 as u32)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 22; continue; } bb = 19; continue; }
            19 => { fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = ((xmm1 as u32)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 21; continue; } bb = 20; continue; }
            20 => { xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) * f32::from_bits(xmm5 as u32)).to_bits() as u128); fl_l = ((xmm2 as u32)) as u32 as u64; fl_r = ((xmm1 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 27; continue; } bb = 21; continue; }
            21 => { rax = 0x4; xmm11 = (crate::simd_ops::ru128(rsp.wrapping_add(0x10u64))) as u128; xmm12 = (crate::simd_ops::ru128(rsp)) as u128; rsp = ((rsp).wrapping_add(0x68u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            22 => { xmm3 = (xmm3) ^ (crate::simd_ops::ru128(P3)); fl_l = ((xmm4 as u32)) as u32 as u64; fl_r = ((xmm3 as u32)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 25; continue; } bb = 23; continue; }
            23 => { fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = ((xmm3 as u32)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 21; continue; } bb = 24; continue; }
            24 => { rax = 0x1; xmm11 = (crate::simd_ops::ru128(rsp.wrapping_add(0x10u64))) as u128; xmm12 = (crate::simd_ops::ru128(rsp)) as u128; rsp = ((rsp).wrapping_add(0x68u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            25 => { fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = ((xmm1 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 27; continue; } bb = 26; continue; }
            26 => { xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) * f32::from_bits(xmm5 as u32)).to_bits() as u128); fl_l = ((xmm2 as u32)) as u32 as u64; fl_r = ((xmm1 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 21; continue; } bb = 27; continue; }
            27 => { xmm11 = (crate::simd_ops::ru128(rsp.wrapping_add(0x10u64))) as u128; rax = 0; fl_l = 0; fl_r = 0; xmm12 = (crate::simd_ops::ru128(rsp)) as u128; rsp = ((rsp).wrapping_add(0x68u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141722d30(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x4; const P2: u64 = 0x14183acc2; const P3: u64 = 0xfffffffffffffff8; const P4: u64 = 0x10; const P5: u64 = 0x28;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rbx = rcx; rdi = ((r8 as u32)) as u32 as u64; rsi = ((rdi.wrapping_add(rdi.wrapping_mul(4))) as u32) as u32 as u64; rsi = (((rsi as u32)).wrapping_shl(3u32)) as u32 as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::lk_add_u32(rcx, ((rsi as u32)) as u32); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            2 => { fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rdx, rax); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            4 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbp); rbp = (crate::simd_ops::ru32(rcx)) as u32 as u64; rbp = ((rbp).wrapping_add(rax)) as u64; crate::simd_ops::wu32(rdx, ((rdi as u32)) as u32); rax = ((rbp as u32)) as u32 as u64; r8 = ((rsi as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_sub((rdx as u32))) as u32 as u64; rcx = rbp; rax = (((rax as u32)).wrapping_sub(0x4u32)) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_add(P1), ((rax as u32)) as u32); rdx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 8; continue; } bb = 5; continue; }
            5 => { rcx = rbp.wrapping_add(0x10); rdx = rdi; rax = 0; fl_l = 0; fl_r = 0; bb = 6; continue; }
            6 => { crate::simd_ops::wu64(rcx.wrapping_add(P3), rax); crate::simd_ops::wu64(rcx, rax); crate::simd_ops::wu64(rcx.wrapping_add(P0), rax); crate::simd_ops::wu64(rcx.wrapping_add(P4), rax); rcx = ((rcx.wrapping_add(P5)) as u64) as u64; fl_l = rdx; fl_r = (0x1u64) as u64; rdx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 6; continue; } bb = 7; continue; }
            7 => { bb = 8; continue; }
            8 => { crate::simd_ops::lk_add_u32(rbx, ((rsi as u32)) as u32); rax = rbp; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141722df0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1426aa688; const P1: u64 = 0x143932190; const P2: u64 = 0x60; const P3: u64 = 0x1401de410;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    rax = P0;
    crate::simd_ops::wu64(rcx, rax);
    fl_l = ((rdx as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64;
    if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { rax = rbx; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    fl_l = ((rdx as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64;
    if !(((((fl_l as u8)) & ((fl_r as u8))) != 0)) { rcx = (crate::simd_ops::ru64(P1)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rax = rbx; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    rdx = 0x48;
    rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp);
    rax = rbx;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141722e40(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1416a5240; const P1: u64 = 0x1434266dc;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm4: u128 = 0; let mut xmm5: u128 = 0; let mut xmm6: u128 = 0; let mut xmm7: u128 = 0; let mut xmm8: u128 = 0; let mut xmm9: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 21>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r11 = rsp; crate::simd_ops::wu64(r11.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(r11.wrapping_add(0x10u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rbp = r11.wrapping_sub(0x5f); rsp = ((rsp).wrapping_sub(0xa0u64)) as u64; rdi = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; rbx = rcx; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x30u64))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru64(rax.wrapping_add(0x20u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 9; continue; } bb = 2; continue; }
            2 => { rdx = 0xffff; fl_l = (crate::simd_ops::ru16(rax.wrapping_add(0x68u64))) as u16 as u64; fl_r = ((rdx as u16)) as u16 as u64; if (((fl_l as u16)) == ((fl_r as u16))) { bb = 9; continue; } bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru16(rcx.wrapping_add(0x8u64))) as u16 as u64; fl_r = ((rdx as u16)) as u16 as u64; if (((fl_l as u16)) == ((fl_r as u16))) { bb = 9; continue; } bb = 4; continue; }
            4 => { crate::simd_ops::wu128(r11.wrapping_sub(0x18u64), (xmm6) as u128); r9 = rbp.wrapping_sub(0x9); crate::simd_ops::wu128(r11.wrapping_sub(0x28u64), (xmm7) as u128); r8 = rbp.wrapping_sub(0x19); crate::simd_ops::wu128(r11.wrapping_sub(0x38u64), (xmm8) as u128); rdx = rbp.wrapping_sub(0x29); crate::simd_ops::wu128(r11.wrapping_sub(0x48u64), (xmm9) as u128); crate::simd_ops::wu64(rbp.wrapping_sub(0x29u64), (0x0u64) as u64); crate::simd_ops::wu32(rbp.wrapping_sub(0x21u64), (0x0u32) as u32); crate::simd_ops::wu64(rbp.wrapping_sub(0x19u64), (0x0u64) as u64); crate::simd_ops::wu32(rbp.wrapping_sub(0x11u64), (0x0u32) as u32); crate::simd_ops::wu64(rbp.wrapping_sub(0x9u64), (0x0u64) as u64); crate::simd_ops::wu32(rbp.wrapping_sub(0x1u64), (0x0u32) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; xmm5 = ((crate::simd_ops::ru32(rbp.wrapping_sub(0x11u64))) as u32) as u128; xmm6 = ((crate::simd_ops::ru32(rbp.wrapping_sub(0x15u64))) as u32) as u128; xmm4 = ((crate::simd_ops::ru32(rbp.wrapping_sub(0x19u64))) as u32) as u128; fl_l = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; fl_r = (0x0u64) as u64; xmm7 = ((crate::simd_ops::ru32(rbp.wrapping_sub(0x1u64))) as u32) as u128; xmm8 = ((crate::simd_ops::ru32(rbp.wrapping_sub(0x5u64))) as u32) as u128; xmm9 = ((crate::simd_ops::ru32(rbp.wrapping_sub(0x9u64))) as u32) as u128; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 7; continue; } bb = 5; continue; }
            5 => { xmm1 = ((crate::simd_ops::ru32(P1)) as u32) as u128; xmm0 = (xmm5) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(crate::simd_ops::ru32(rbp.wrapping_sub(0x21u64)))).to_bits() as u128); xmm3 = (xmm4) as u128; xmm2 = (xmm6) as u128; xmm3 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) + f32::from_bits(crate::simd_ops::ru32(rbp.wrapping_sub(0x29u64)))).to_bits() as u128); xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) + f32::from_bits(crate::simd_ops::ru32(rbp.wrapping_sub(0x25u64)))).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(xmm7 as u32)).to_bits() as u128); xmm3 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) + f32::from_bits(xmm9 as u32)).to_bits() as u128); xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) + f32::from_bits(xmm8 as u32)).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm1 as u32)).to_bits() as u128); xmm3 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) * f32::from_bits(xmm1 as u32)).to_bits() as u128); crate::simd_ops::wu32(rbp.wrapping_add(0xfu64), ((xmm0 as u32)) as u32); rax = (crate::simd_ops::ru32(rbp.wrapping_add(0xfu64))) as u32 as u64; xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) * f32::from_bits(xmm1 as u32)).to_bits() as u128); xmm0 = (xmm3) as u128; xmm0 = crate::simd_ops::vunpcklps(xmm0, xmm2); crate::simd_ops::wu64(rdi.wrapping_add(0x8u64), ((xmm0 as u64)) as u64); crate::simd_ops::wu32(rdi.wrapping_add(0x10u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; fl_l = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 7; continue; } bb = 6; continue; }
            6 => { xmm5 = (xmm5 & !0xffff_ffffu128) | ((f32::from_bits(xmm5 as u32) - f32::from_bits(crate::simd_ops::ru32(rbp.wrapping_sub(0x21u64)))).to_bits() as u128); xmm4 = (xmm4 & !0xffff_ffffu128) | ((f32::from_bits(xmm4 as u32) - f32::from_bits(crate::simd_ops::ru32(rbp.wrapping_sub(0x29u64)))).to_bits() as u128); xmm6 = (xmm6 & !0xffff_ffffu128) | ((f32::from_bits(xmm6 as u32) - f32::from_bits(crate::simd_ops::ru32(rbp.wrapping_sub(0x25u64)))).to_bits() as u128); xmm8 = (xmm8 & !0xffff_ffffu128) | ((f32::from_bits(xmm8 as u32) - f32::from_bits(crate::simd_ops::ru32(rbp.wrapping_sub(0x25u64)))).to_bits() as u128); xmm7 = (xmm7 & !0xffff_ffffu128) | ((f32::from_bits(xmm7 as u32) - f32::from_bits(crate::simd_ops::ru32(rbp.wrapping_sub(0x21u64)))).to_bits() as u128); xmm9 = (xmm9 & !0xffff_ffffu128) | ((f32::from_bits(xmm9 as u32) - f32::from_bits(crate::simd_ops::ru32(rbp.wrapping_sub(0x29u64)))).to_bits() as u128); xmm0 = (xmm5) as u128; xmm1 = (xmm6) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm8 as u32)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm7 as u32)).to_bits() as u128); xmm5 = (xmm5 & !0xffff_ffffu128) | ((f32::from_bits(xmm5 as u32) * f32::from_bits(xmm9 as u32)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) - f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm6 = (xmm6 & !0xffff_ffffu128) | ((f32::from_bits(xmm6 as u32) * f32::from_bits(xmm9 as u32)).to_bits() as u128); xmm0 = (xmm4) as u128; xmm4 = (xmm4 & !0xffff_ffffu128) | ((f32::from_bits(xmm4 as u32) * f32::from_bits(xmm8 as u32)).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm7 as u32)).to_bits() as u128); xmm4 = (xmm4 & !0xffff_ffffu128) | ((f32::from_bits(xmm4 as u32) - f32::from_bits(xmm6 as u32)).to_bits() as u128); xmm5 = (xmm5 & !0xffff_ffffu128) | ((f32::from_bits(xmm5 as u32) - f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm0 = (xmm1) as u128; crate::simd_ops::wu32(rbp.wrapping_add(0xfu64), ((xmm4 as u32)) as u32); rax = (crate::simd_ops::ru32(rbp.wrapping_add(0xfu64))) as u32 as u64; xmm0 = crate::simd_ops::vunpcklps(xmm0, xmm5); crate::simd_ops::wu64(rdi.wrapping_add(0x14u64), ((xmm0 as u64)) as u64); crate::simd_ops::wu32(rdi.wrapping_add(0x1cu64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; bb = 7; continue; }
            7 => { fl_l = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; fl_r = (0x0u64) as u64; xmm9 = (crate::simd_ops::ru128(rsp.wrapping_add(0x60u64))) as u128; xmm8 = (crate::simd_ops::ru128(rsp.wrapping_add(0x70u64))) as u128; xmm7 = (crate::simd_ops::ru128(rsp.wrapping_add(0x80u64))) as u128; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x90u64))) as u128; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 11; continue; } bb = 8; continue; }
            8 => { crate::simd_ops::wu32(rdi, (0x1u32) as u32); bb = 11; continue; }
            9 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; fl_l = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 11; continue; } bb = 10; continue; }
            10 => { crate::simd_ops::wu32(rdi, (0x0u32) as u32); bb = 11; continue; }
            11 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x38u64))) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(0x20u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 16; continue; } bb = 12; continue; }
            12 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; fl_l = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 19; continue; } bb = 13; continue; }
            13 => { crate::simd_ops::wu32(rdi.wrapping_add(0x4u64), (0xffffffffu32) as u32); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; fl_l = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 19; continue; } bb = 14; continue; }
            14 => { crate::simd_ops::wu32(rdi.wrapping_add(0x20u64), (0x0u32) as u32); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; fl_l = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 19; continue; } bb = 15; continue; }
            15 => { crate::simd_ops::wu32(rdi.wrapping_add(0x24u64), (0x0u32) as u32); bb = 19; continue; }
            16 => { rax = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x40u64))) as u32 as u64; rdx = (crate::simd_ops::ru64(rax.wrapping_add(rcx.wrapping_mul(8)))) as u64; rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; fl_l = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 19; continue; } bb = 17; continue; }
            17 => { rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x1f8u64))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x4u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; fl_l = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 19; continue; } bb = 18; continue; }
            18 => { xmm0 = ((crate::simd_ops::ru64(rdx.wrapping_add(0x40u64))) as u64) as u128; crate::simd_ops::wu64(rdi.wrapping_add(0x20u64), ((xmm0 as u64)) as u64); bb = 19; continue; }
            19 => { r11 = rsp.wrapping_add(0xa0); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x10u64))) as u64; rdi = (crate::simd_ops::ru64(r11.wrapping_add(0x18u64))) as u64; rsp = r11; rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1417230b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141722d30; const P1: u64 = 0x1426ac118; const P2: u64 = 0x1416a2e90; const P3: u64 = 0x141722e40;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 17>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x70u64)) as u64; rbx = rcx; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 4; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x30u64))) as u64; rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; xmm0 = ((crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64) as u128; crate::simd_ops::wu64(rdx.wrapping_add(0x8u64), ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(rax.wrapping_add(0x18u64))) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_add(0x10u64), ((rax as u32)) as u32); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 4; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x30u64))) as u64; rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rax = (crate::simd_ops::ru32(rax.wrapping_add(0x40u64))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x14u64), ((rax as u32)) as u32); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x30u64))) as u64; rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rax = (crate::simd_ops::ru32(rax.wrapping_add(0x38u64))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x18u64), ((rax as u32)) as u32); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; bb = 4; continue; }
            4 => { rbp = 0; fl_l = 0; fl_r = 0; fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; fl_r = rbp; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; bb = 7; continue; }
            6 => { rdx = rbp; bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x30u64))) as u64; r8 = (crate::simd_ops::ru64(rax.wrapping_add(0x20u64))) as u64; fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x18u64))) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; rcx = (if (((fl_l as u32)) != ((fl_r as u32))) { rbp } else { rcx }) as u64; r8 = (crate::simd_ops::ru32(r8.wrapping_add(0x20u64))) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r14 = rax; rsi = ((rbp as u32)) as u32 as u64; rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x30u64))) as u64; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(0x20u64))) as u64; fl_l = (crate::simd_ops::ru32(rdx.wrapping_add(0x20u64))) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 13; continue; } bb = 8; continue; }
            8 => { r15 = P1; bb = 9; continue; }
            9 => { rdx = ((rsi as u32)) as u32 as u64; rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 11; continue; } bb = 10; continue; }
            10 => { rax = ((rdx.wrapping_add(rdx.wrapping_mul(4))) as u64) as u64; rdi = ((r14.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; bb = 12; continue; }
            11 => { rdi = rbp; bb = 12; continue; }
            12 => { r8 = (crate::simd_ops::ru64(rbx.wrapping_add(0x30u64))) as u64; rcx = ((rdx.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru64(r8.wrapping_add(0x28u64))) as u64; rdx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rbp); crate::simd_ops::wu32(rsp.wrapping_add(0x40u64), (0x2u32) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r15); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rdx); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), r8); crate::simd_ops::wu32(rsp.wrapping_add(0x60u64), ((rsi as u32)) as u32); rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rdi); rcx = rsp.wrapping_add(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rsi = (((rsi as u32)).wrapping_add(1)) as u32 as u64; rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x30u64))) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x20u64))) as u64; fl_l = ((rsi as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rcx.wrapping_add(0x20u64))) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 9; continue; } bb = 13; continue; }
            13 => { r11 = rsp.wrapping_add(0x70); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rbp = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x30u64))) as u64; rsp = r11; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141723210(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1434268d0; const P1: u64 = 0x143427034; const P2: u64 = 0x141747d10; const P3: u64 = 0x14342668c; const P4: u64 = 0x1417234a0; const P5: u64 = 0x143426780;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm4: u128 = 0; let mut xmm5: u128 = 0; let mut xmm6: u128 = 0; let mut xmm7: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; fl_l = (crate::simd_ops::ru8(r8.wrapping_add(0x16u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; rbx = r8; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm6) as u128); rsi = rdx; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm7) as u128); rdi = rcx; xmm7 = ((crate::simd_ops::ru32(P0)) as u32) as u128; xmm6 = 0u128; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 8; continue; } bb = 1; continue; }
            1 => { xmm2 = ((crate::simd_ops::ru32(r8)) as u32) as u128; xmm3 = 0u128; xmm4 = ((crate::simd_ops::ru32(r8.wrapping_add(0x4u64))) as u32) as u128; xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) - f32::from_bits(crate::simd_ops::ru32(rcx))).to_bits() as u128); xmm4 = (xmm4 & !0xffff_ffffu128) | ((f32::from_bits(xmm4 as u32) - f32::from_bits(crate::simd_ops::ru32(rcx.wrapping_add(0x4u64)))).to_bits() as u128); xmm0 = (xmm2) as u128; xmm1 = (xmm4) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm2 as u32)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm4 as u32)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) + f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm3 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32)).sqrt().to_bits() as u128); fl_l = ((xmm3 as u32)) as u32 as u64; fl_r = ((xmm6 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) == f32::from_bits((fl_r as u32)) || f32::from_bits((fl_l as u32)).is_nan() || f32::from_bits((fl_r as u32)).is_nan()) { bb = 3; continue; } bb = 2; continue; }
            2 => { xmm0 = (xmm7) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) / f32::from_bits(xmm3 as u32)).to_bits() as u128); xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) * f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm4 = (xmm4 & !0xffff_ffffu128) | ((f32::from_bits(xmm4 as u32) * f32::from_bits(xmm0 as u32)).to_bits() as u128); bb = 4; continue; }
            3 => { xmm2 = 0u128; xmm4 = 0u128; bb = 4; continue; }
            4 => { xmm5 = ((crate::simd_ops::ru32(rcx.wrapping_add(0x8u64))) as u32) as u128; xmm3 = ((crate::simd_ops::ru32(rcx.wrapping_add(0xcu64))) as u32) as u128; xmm0 = (xmm5) as u128; xmm1 = (xmm3) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm5 as u32)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm3 as u32)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) + f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32)).sqrt().to_bits() as u128); fl_l = ((xmm1 as u32)) as u32 as u64; fl_r = ((xmm6 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) == f32::from_bits((fl_r as u32)) || f32::from_bits((fl_l as u32)).is_nan() || f32::from_bits((fl_r as u32)).is_nan()) { bb = 6; continue; } bb = 5; continue; }
            5 => { xmm0 = (xmm7) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) / f32::from_bits(xmm1 as u32)).to_bits() as u128); xmm5 = (xmm5 & !0xffff_ffffu128) | ((f32::from_bits(xmm5 as u32) * f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm3 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) * f32::from_bits(xmm0 as u32)).to_bits() as u128); bb = 7; continue; }
            6 => { xmm5 = 0u128; xmm3 = 0u128; bb = 7; continue; }
            7 => { xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) * f32::from_bits(xmm5 as u32)).to_bits() as u128); xmm3 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) * f32::from_bits(xmm4 as u32)).to_bits() as u128); xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) + f32::from_bits(xmm3 as u32)).to_bits() as u128); fl_l = ((xmm2 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(P1)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 17; continue; } bb = 8; continue; }
            8 => { rdx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm2 = ((crate::simd_ops::ru32(rsi.wrapping_add(0x18u64))) as u32) as u128; xmm3 = (xmm0) as u128; xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) * f32::from_bits(xmm2 as u32)).to_bits() as u128); xmm1 = (xmm2) as u128; xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(crate::simd_ops::ru32(P3))).to_bits() as u128); fl_l = ((xmm3 as u32)) as u32 as u64; fl_r = ((xmm1 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 12; continue; } bb = 9; continue; }
            9 => { xmm0 = ((crate::simd_ops::ru32(rdi)) as u32) as u128; fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rbx)) as u32 as u64; if (f32::from_bits((fl_l as u32)) != f32::from_bits((fl_r as u32)) && !f32::from_bits((fl_l as u32)).is_nan() && !f32::from_bits((fl_r as u32)).is_nan()) { bb = 11; continue; } bb = 10; continue; }
            10 => { xmm0 = ((crate::simd_ops::ru32(rdi.wrapping_add(0x4u64))) as u32) as u128; fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rbx.wrapping_add(0x4u64))) as u32 as u64; if (f32::from_bits((fl_l as u32)) == f32::from_bits((fl_r as u32)) || f32::from_bits((fl_l as u32)).is_nan() || f32::from_bits((fl_r as u32)).is_nan()) { bb = 17; continue; } bb = 11; continue; }
            11 => { rcx = rdi.wrapping_add(0x18); rdx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = 0; fl_l = 0; fl_r = 0; bb = 18; continue; }
            12 => { fl_l = ((xmm3 as u32)) as u32 as u64; fl_r = ((xmm2 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 16; continue; } bb = 13; continue; }
            13 => { xmm0 = ((crate::simd_ops::ru32(rdi)) as u32) as u128; fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rbx)) as u32 as u64; if (f32::from_bits((fl_l as u32)) != f32::from_bits((fl_r as u32)) && !f32::from_bits((fl_l as u32)).is_nan() && !f32::from_bits((fl_r as u32)).is_nan()) { bb = 15; continue; } bb = 14; continue; }
            14 => { xmm0 = ((crate::simd_ops::ru32(rdi.wrapping_add(0x4u64))) as u32) as u128; fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rbx.wrapping_add(0x4u64))) as u32 as u64; if (f32::from_bits((fl_l as u32)) == f32::from_bits((fl_r as u32)) || f32::from_bits((fl_l as u32)).is_nan() || f32::from_bits((fl_r as u32)).is_nan()) { bb = 17; continue; } bb = 15; continue; }
            15 => { xmm0 = ((crate::simd_ops::ru32(P5)) as u32) as u128; rcx = rdi.wrapping_add(0x18); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) / f32::from_bits(xmm2 as u32)).to_bits() as u128); rdx = rbx; xmm1 = (xmm7) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm3 as u32)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) - f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((if f32::from_bits(xmm1 as u32) < f32::from_bits(xmm7 as u32) { f32::from_bits(xmm1 as u32) } else { f32::from_bits(xmm7 as u32) }).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((if f32::from_bits(xmm1 as u32) > f32::from_bits(xmm6 as u32) { f32::from_bits(xmm1 as u32) } else { f32::from_bits(xmm6 as u32) }).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(crate::simd_ops::ru32(rbx.wrapping_add(0x10u64)))).to_bits() as u128); crate::simd_ops::wu32(rbx.wrapping_add(0x10u64), ((xmm1 as u32)) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = 0; fl_l = 0; fl_r = 0; bb = 18; continue; }
            16 => { rcx = rdi.wrapping_add(0x30); rdx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 17; continue; }
            17 => { rax = 0x1; bb = 18; continue; }
            18 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; xmm7 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1417233b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x20; const P2: u64 = 0xffffffffffffffc8; const P3: u64 = 0x1434268d0; const P4: u64 = 0xffffffffffffffb8; const P5: u64 = 0xffffffffffffffa8; const P6: u64 = 0x8; const P7: u64 = 0x18; const P8: u64 = 0x141747cb0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0; let mut xmm7: u128 = 0; let mut xmm8: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 17>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = rsp; crate::simd_ops::wu64(rax.wrapping_add(P0), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x60u64)) as u64; rsi = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64; rbx = 0; fl_l = 0; fl_r = 0; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0xb0u64))) as u64; r12 = rdx; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0xb8u64))) as u64; r15 = rcx; crate::simd_ops::wu128(rax.wrapping_add(P2), (xmm6) as u128); xmm6 = ((crate::simd_ops::ru32(P3)) as u32) as u128; crate::simd_ops::wu128(rax.wrapping_add(P4), (xmm7) as u128); xmm7 = (xmm3) as u128; crate::simd_ops::wu32(rbp, ((xmm7 as u32)) as u32); crate::simd_ops::wu128(rax.wrapping_add(P5), (xmm8) as u128); xmm8 = (xmm2) as u128; crate::simd_ops::wu32(r14, (0xffffffffu32) as u32); fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 6; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rax.wrapping_add(P6), rdi); rdi = ((rbx as u32)) as u32 as u64; bb = 2; continue; }
            2 => { rdx = (crate::simd_ops::ru64(r15.wrapping_add(P7))) as u64; rax = rsp.wrapping_add(0xa8); r8 = (crate::simd_ops::ru64(r12)) as u64; rdx = ((rdx).wrapping_add(rdi)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); xmm3 = (xmm7) as u128; rcx = r15; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((xmm8 as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0xa8u64), ((xmm7 as u32)) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = ((xmm6 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { xmm1 = ((crate::simd_ops::ru32(rsp.wrapping_add(0xa8u64))) as u32) as u128; xmm6 = (xmm0) as u128; crate::simd_ops::wu32(rbp, ((xmm1 as u32)) as u32); crate::simd_ops::wu32(r14, ((rbx as u32)) as u32); bb = 4; continue; }
            4 => { rbx = (((rbx as u32)).wrapping_add(1)) as u32 as u64; rdi = ((rdi).wrapping_add(0x18u64)) as u64; fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 2; continue; } bb = 5; continue; }
            5 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; bb = 6; continue; }
            6 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x98u64))) as u64; xmm0 = (xmm6) as u128; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x50u64))) as u128; xmm7 = (crate::simd_ops::ru128(rsp.wrapping_add(0x40u64))) as u128; xmm8 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; rsp = ((rsp).wrapping_add(0x60u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1417234a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x1; const P3: u64 = 0x141723520; const P4: u64 = 0xffffffffffffffe8; const P5: u64 = 0xfffffffffffffff8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rdi = rcx; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; rsi = rdx; rbx = ((rax.wrapping_add(P2)) as u64) as u64; fl_l = rbx; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 3; continue; } bb = 1; continue; }
            1 => { rcx = ((rcx).wrapping_shr(1u32)) as u64; fl_l = rbx; fl_r = rcx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 6; continue; } bb = 2; continue; }
            2 => { r8 = rbx; bb = 5; continue; }
            3 => { fl_l = rbx; fl_r = rcx; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 6; continue; } bb = 4; continue; }
            4 => { r8 = rbx; r8 = ((r8).wrapping_shr(2u32)) as u64; fl_l = ((r8).wrapping_add(rbx)) as u64; fl_r = fl_l; r8 = ((fl_l as u64)) as u64; bb = 5; continue; }
            5 => { rdx = rdi; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rcx = ((rbx.wrapping_add(rbx.wrapping_mul(2))) as u64) as u64; crate::simd_ops::wu64(rdi.wrapping_add(P0), rbx); xmm0 = (crate::simd_ops::ru128(rsi)) as u128; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu128(rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(P4), (xmm0) as u128); xmm1 = ((crate::simd_ops::ru64(rsi.wrapping_add(P1))) as u64) as u128; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; crate::simd_ops::wu64(rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(P5), ((xmm1 as u64)) as u64); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141723520(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x143932190; const P2: u64 = 0x60; const P3: u64 = 0xf; const P4: u64 = 0x50; const P5: u64 = 0x48;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rcx; fl_l = r8; fl_r = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 8; continue; } bb = 1; continue; }
            1 => { rdx = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rdi); fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 4; continue; } bb = 2; continue; }
            2 => { rdi = 0; fl_l = 0; fl_r = 0; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(P1)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx, rdi); crate::simd_ops::wu64(rbx.wrapping_add(P0), rdi); rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            4 => { rcx = (crate::simd_ops::ru64(P1)) as u64; rdi = ((r8.wrapping_add(P3)) as u64) as u64; rdi = (((rdi) & (0xfffffffffffffff0u64))) as u64; r9 = ((rdi.wrapping_add(rdi.wrapping_mul(2))) as u64) as u64; r9 = ((r9).wrapping_shl(3u32)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rcx)) as u64; r8 = r9; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P4))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx, rax); crate::simd_ops::wu64(rbx.wrapping_add(P0), rdi); rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            6 => { crate::simd_ops::wu32(rsp.wrapping_add(0x48u64), (0x87u32) as u32); r8 = rsp.wrapping_add(0x48); rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = r9; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P5))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx, rax); bb = 7; continue; }
            7 => { crate::simd_ops::wu64(rbx.wrapping_add(P0), rdi); rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; bb = 8; continue; }
            8 => { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1417235d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x4c; const P1: u64 = 0x10; const P2: u64 = 0x1411ff9f0; const P3: u64 = 0x30;
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(P0), ((xmm2 as u32)) as u32);
    rbx = rcx;
    xmm0 = (crate::simd_ops::ru128(rdx)) as u128;
    crate::simd_ops::wu128(rcx, (xmm0) as u128);
    xmm1 = ((crate::simd_ops::ru64(rdx.wrapping_add(P1))) as u64) as u128;
    crate::simd_ops::wu64(rcx.wrapping_add(P1), ((xmm1 as u64)) as u64);
    rcx = ((rcx).wrapping_add(0x18u64)) as u64;
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rcx = ((rbx.wrapping_add(P3)) as u64) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141723610(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x14; const P2: u64 = 0x1c; const P3: u64 = 0x28;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rcx.wrapping_add(P0), rax);
    crate::simd_ops::wu64(rcx.wrapping_add(P1), rax);
    crate::simd_ops::wu64(rcx.wrapping_add(P2), rax);
    crate::simd_ops::wu32(rcx.wrapping_add(P3), ((rax as u32)) as u32);
    rax = rcx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141723630(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x14; const P3: u64 = 0x1c; const P4: u64 = 0x24; const P5: u64 = 0x28;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64;
    crate::simd_ops::wu64(rcx, r8);
    xmm0 = ((crate::simd_ops::ru64(rdx)) as u64) as u128;
    crate::simd_ops::wu64(rcx.wrapping_add(P0), ((xmm0 as u64)) as u64);
    crate::simd_ops::wu32(rcx.wrapping_add(P1), ((r9 as u32)) as u32);
    xmm0 = ((crate::simd_ops::ru64(rax)) as u64) as u128;
    crate::simd_ops::wu64(rcx.wrapping_add(P2), ((xmm0 as u64)) as u64);
    xmm1 = ((crate::simd_ops::ru64(rdx)) as u64) as u128;
    crate::simd_ops::wu64(rcx.wrapping_add(P3), ((xmm1 as u64)) as u64);
    crate::simd_ops::wu32(rcx.wrapping_add(P4), (0xffffffffu32) as u32);
    crate::simd_ops::wu32(rcx.wrapping_add(P5), (0x1u32) as u32);
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141723670(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1416e3680; const P1: u64 = 0x1416e3590; const P2: u64 = 0x1417238b0; const P3: u64 = 0x141723e50; const P4: u64 = 0x141723ab0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(0x28u64))) as u32 as u64; fl_r = 0x1; rbx = rcx; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 19; continue; } bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru64(rcx)) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 27; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x18u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 27; continue; } bb = 3; continue; }
            3 => { r8 = (crate::simd_ops::ru32(rbx.wrapping_add(0x10u64))) as u32 as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 5; continue; } bb = 4; continue; }
            4 => { crate::simd_ops::wu32(rbx.wrapping_add(0x28u64), (0x8u32) as u32); rsp = ((rsp).wrapping_add(0x40u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            5 => { r9 = rsp.wrapping_add(0x50); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rdi); rdx = rbx.wrapping_add(0x8); crate::simd_ops::wu32(rsp.wrapping_add(0x50u64), (0x0u32) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 10; continue; } bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x50u64))) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 20; continue; } bb = 7; continue; }
            7 => { r8 = (crate::simd_ops::ru32(rbx.wrapping_add(0x10u64))) as u32 as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 20; continue; } bb = 8; continue; }
            8 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; r9 = rsp.wrapping_add(0x58); r8 = (((r8 as u32)).wrapping_sub(1)) as u32 as u64; rdx = rbx.wrapping_add(0x8); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 24; continue; } bb = 9; continue; }
            9 => { fl_l = (crate::simd_ops::lk_sub_u32(rbx.wrapping_add(0x10u64), (1) as u32)) as u32 as u64; bb = 10; continue; }
            10 => { crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rbp); rbp = (crate::simd_ops::ru32(rbx.wrapping_add(0x10u64))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x24u64), ((rbp as u32)) as u32); xmm1 = ((crate::simd_ops::ru32(rbx.wrapping_add(0x14u64))) as u32) as u128; xmm0 = ((crate::simd_ops::ru32(rbx.wrapping_add(0x18u64))) as u32) as u128; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r14); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) + f32::from_bits(crate::simd_ops::ru32(rbx.wrapping_add(0x8u64)))).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(crate::simd_ops::ru32(rbx.wrapping_add(0xcu64)))).to_bits() as u128); crate::simd_ops::wu32(rbx.wrapping_add(0x1cu64), ((xmm1 as u32)) as u32); crate::simd_ops::wu32(rbx.wrapping_add(0x20u64), ((xmm0 as u32)) as u32); fl_l = ((rbp as u32)) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 26; continue; } bb = 11; continue; }
            11 => { rax = (crate::simd_ops::ru64(rbx)) as u64; fl_l = ((rbp as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rax.wrapping_add(0x18u64))) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 25; continue; } bb = 12; continue; }
            12 => { r14 = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; rcx = ((rbp.wrapping_mul(2)) as u64) as u64; rcx = ((rcx).wrapping_add(rbp)) as u64; r8 = rsp.wrapping_add(0x50); rcx = ((rcx).wrapping_shl(4u32)) as u64; rdx = rbx.wrapping_add(0x1c); rcx = ((rcx).wrapping_add(r14)) as u64; xmm0 = ((crate::simd_ops::ru32(rcx.wrapping_add(0x1cu64))) as u32) as u128; xmm1 = ((crate::simd_ops::ru32(rcx.wrapping_add(0x20u64))) as u32) as u128; crate::simd_ops::wu32(rsp.wrapping_add(0x50u64), ((xmm0 as u32)) as u32); xmm0 = ((crate::simd_ops::ru32(rcx.wrapping_add(0x4u64))) as u32) as u128; crate::simd_ops::wu32(rsp.wrapping_add(0x54u64), ((xmm1 as u32)) as u32); xmm1 = ((crate::simd_ops::ru32(rcx.wrapping_add(0x8u64))) as u32) as u128; rcx = rbx.wrapping_add(0x8); crate::simd_ops::wu32(rsp.wrapping_add(0x58u64), ((xmm0 as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x5cu64), ((xmm1 as u32)) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 14; continue; } bb = 13; continue; }
            13 => { r8 = rsp.wrapping_add(0x58); rdx = rbx.wrapping_add(0x1c); rcx = rbx.wrapping_add(0x8); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 26; continue; } bb = 14; continue; }
            14 => { rax = ((rbp.wrapping_sub(0x1u64)) as u32) as u32 as u64; rbp = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rbp = ((rbp).wrapping_shl(4u32)) as u64; r8 = rsp.wrapping_add(0x50); rbp = ((rbp).wrapping_add(r14)) as u64; rdx = rbx.wrapping_add(0x1c); rcx = rbx.wrapping_add(0x8); xmm0 = ((crate::simd_ops::ru32(rbp.wrapping_add(0x4u64))) as u32) as u128; xmm1 = ((crate::simd_ops::ru32(rbp.wrapping_add(0x8u64))) as u32) as u128; crate::simd_ops::wu32(rsp.wrapping_add(0x50u64), ((xmm0 as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x54u64), ((xmm1 as u32)) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 16; continue; } bb = 15; continue; }
            15 => { xmm0 = ((crate::simd_ops::ru32(rbp.wrapping_add(0x1cu64))) as u32) as u128; r8 = rsp.wrapping_add(0x50); xmm1 = ((crate::simd_ops::ru32(rbp.wrapping_add(0x20u64))) as u32) as u128; rdx = rbx.wrapping_add(0x1c); rcx = rbx.wrapping_add(0x8); crate::simd_ops::wu32(rsp.wrapping_add(0x50u64), ((xmm0 as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x54u64), ((xmm1 as u32)) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 25; continue; } bb = 16; continue; }
            16 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x28u64))) as u32 as u64; fl_r = 0x7; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 18; continue; } bb = 17; continue; }
            17 => { rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 18; continue; }
            18 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; bb = 19; continue; }
            19 => { rsp = ((rsp).wrapping_add(0x40u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            20 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x2u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 24; continue; } bb = 21; continue; }
            21 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; r8 = (crate::simd_ops::ru32(rbx.wrapping_add(0x10u64))) as u32 as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rcx.wrapping_add(0x18u64))) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 24; continue; } bb = 22; continue; }
            22 => { r8 = (((r8 as u32)).wrapping_add(1)) as u32 as u64; r9 = rsp.wrapping_add(0x60); rdx = rbx.wrapping_add(0x8); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 24; continue; } bb = 23; continue; }
            23 => { fl_l = (crate::simd_ops::lk_add_u32(rbx.wrapping_add(0x10u64), (1) as u32)) as u32 as u64; bb = 10; continue; }
            24 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x28u64), (0x7u32) as u32); rsp = ((rsp).wrapping_add(0x40u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            25 => { rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 18; continue; }
            26 => { rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 18; continue; }
            27 => { crate::simd_ops::wu32(rbx.wrapping_add(0x28u64), (0x9u32) as u32); rsp = ((rsp).wrapping_add(0x40u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1417238b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1416ab080;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm4: u128 = 0; let mut xmm5: u128 = 0; let mut xmm6: u128 = 0; let mut xmm7: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 17>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rbp = rsp; rsp = ((rsp).wrapping_sub(0x80u64)) as u64; r10 = (crate::simd_ops::ru32(rcx.wrapping_add(0x24u64))) as u32 as u64; rbx = rcx; r8 = (crate::simd_ops::ru64(rcx)) as u64; rax = ((r10 as u32)) as u32 as u64; rcx = 0x1; crate::simd_ops::wu128(rsp.wrapping_add(0x70u64), (xmm6) as u128); fl_l = ((r10 as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x60u64), (xmm7) as u128); xmm5 = ((crate::simd_ops::ru32(rbx.wrapping_add(0x1cu64))) as u32) as u128; xmm7 = 0u128; r9 = (crate::simd_ops::ru64(r8.wrapping_add(0x10u64))) as u64; rax = (if (((fl_l as u32)) < ((fl_r as u32))) { (rcx as u32) } else { (rax as u32) }) as u32 as u64; xmm6 = ((crate::simd_ops::ru32(rbx.wrapping_add(0x20u64))) as u32) as u128; rax = (((rax as u32)).wrapping_sub(1)) as u32 as u64; r11 = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru32(r8.wrapping_add(0x18u64))) as u32 as u64; r11 = ((r11).wrapping_shl(4u32)) as u64; r11 = ((r11).wrapping_add(r9)) as u64; crate::simd_ops::wu32(rbp.wrapping_sub(0x50u64), ((xmm5 as u32)) as u32); rax = (((rax as u32)).wrapping_sub(1)) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_sub(0x4cu64), ((xmm6 as u32)) as u32); fl_l = ((r10 as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_sub(0x48u64), (0x0u32) as u32); r10 = (if (((fl_l as u32)) >= ((fl_r as u32))) { (rax as u32) } else { (r10 as u32) }) as u32 as u64; r8 = ((r10.wrapping_add(r10.wrapping_mul(2))) as u64) as u64; r8 = ((r8).wrapping_shl(4u32)) as u64; r8 = ((r8).wrapping_add(r9)) as u64; r9 = (crate::simd_ops::ru32(r11)) as u32 as u64; rax = ((r9.wrapping_sub(0x8u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 4; continue; } bb = 1; continue; }
            1 => { xmm0 = ((crate::simd_ops::ru32(r11.wrapping_add(0x1cu64))) as u32) as u128; rcx = r8.wrapping_add(0x1c); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) - f32::from_bits(crate::simd_ops::ru32(rcx))).to_bits() as u128); xmm1 = ((crate::simd_ops::ru32(r11.wrapping_add(0x20u64))) as u32) as u128; rdx = r11.wrapping_add(0x1c); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) - f32::from_bits(crate::simd_ops::ru32(rcx.wrapping_add(0x4u64)))).to_bits() as u128); xmm4 = (xmm6) as u128; xmm4 = (xmm4 & !0xffff_ffffu128) | ((f32::from_bits(xmm4 as u32) - f32::from_bits(crate::simd_ops::ru32(rcx.wrapping_add(0x4u64)))).to_bits() as u128); xmm4 = (xmm4 & !0xffff_ffffu128) | ((f32::from_bits(xmm4 as u32) * f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm0 = (xmm5) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) - f32::from_bits(crate::simd_ops::ru32(rcx))).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm4 = (xmm4 & !0xffff_ffffu128) | ((f32::from_bits(xmm4 as u32) - f32::from_bits(xmm1 as u32)).to_bits() as u128); fl_l = ((xmm4 as u32)) as u32 as u64; fl_r = ((xmm7 as u32)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 3; continue; } bb = 2; continue; }
            2 => { xmm0 = ((crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32) as u128; rax = rbp.wrapping_sub(0x50); xmm1 = ((crate::simd_ops::ru32(rbx.wrapping_add(0xcu64))) as u32) as u128; r9 = rbp.wrapping_sub(0x40); r8 = rbp.wrapping_sub(0x34); crate::simd_ops::wu32(rbp.wrapping_sub(0x40u64), ((xmm0 as u32)) as u32); crate::simd_ops::wu32(rbp.wrapping_sub(0x3cu64), ((xmm1 as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); crate::simd_ops::wu32(rbp.wrapping_sub(0x34u64), ((xmm5 as u32)) as u32); crate::simd_ops::wu32(rbp.wrapping_sub(0x30u64), ((xmm6 as u32)) as u32); crate::simd_ops::wu32(rbp.wrapping_sub(0x38u64), (0x0u32) as u32); crate::simd_ops::wu32(rbp.wrapping_sub(0x2cu64), (0x0u32) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm0 = ((crate::simd_ops::ru32(rbp.wrapping_sub(0x50u64))) as u32) as u128; xmm1 = ((crate::simd_ops::ru32(rbp.wrapping_sub(0x4cu64))) as u32) as u128; crate::simd_ops::wu32(rbx.wrapping_add(0x1cu64), ((xmm0 as u32)) as u32); crate::simd_ops::wu32(rbx.wrapping_add(0x20u64), ((xmm1 as u32)) as u32); crate::simd_ops::wu32(rbx.wrapping_add(0x28u64), (0x6u32) as u32); bb = 7; continue; }
            3 => { rax = ((r9.wrapping_sub(0x4u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 6; continue; } bb = 4; continue; }
            4 => { xmm0 = ((crate::simd_ops::ru32(r8.wrapping_add(0x4u64))) as u32) as u128; rcx = r11.wrapping_add(0x4); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) - f32::from_bits(crate::simd_ops::ru32(rcx))).to_bits() as u128); xmm1 = ((crate::simd_ops::ru32(r8.wrapping_add(0x8u64))) as u32) as u128; rdx = r8.wrapping_add(0x4); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) - f32::from_bits(crate::simd_ops::ru32(rcx.wrapping_add(0x4u64)))).to_bits() as u128); xmm4 = (xmm6) as u128; xmm4 = (xmm4 & !0xffff_ffffu128) | ((f32::from_bits(xmm4 as u32) - f32::from_bits(crate::simd_ops::ru32(rcx.wrapping_add(0x4u64)))).to_bits() as u128); xmm4 = (xmm4 & !0xffff_ffffu128) | ((f32::from_bits(xmm4 as u32) * f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm0 = (xmm5) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) - f32::from_bits(crate::simd_ops::ru32(rcx))).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm4 = (xmm4 & !0xffff_ffffu128) | ((f32::from_bits(xmm4 as u32) - f32::from_bits(xmm1 as u32)).to_bits() as u128); fl_l = ((xmm4 as u32)) as u32 as u64; fl_r = ((xmm7 as u32)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 6; continue; } bb = 5; continue; }
            5 => { xmm0 = ((crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32) as u128; rax = rbp.wrapping_sub(0x50); xmm1 = ((crate::simd_ops::ru32(rbx.wrapping_add(0xcu64))) as u32) as u128; r9 = rbp.wrapping_sub(0x34); r8 = rbp.wrapping_sub(0x40); crate::simd_ops::wu32(rbp.wrapping_sub(0x34u64), ((xmm0 as u32)) as u32); crate::simd_ops::wu32(rbp.wrapping_sub(0x30u64), ((xmm1 as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); crate::simd_ops::wu32(rbp.wrapping_sub(0x40u64), ((xmm5 as u32)) as u32); crate::simd_ops::wu32(rbp.wrapping_sub(0x3cu64), ((xmm6 as u32)) as u32); crate::simd_ops::wu32(rbp.wrapping_sub(0x2cu64), (0x0u32) as u32); crate::simd_ops::wu32(rbp.wrapping_sub(0x38u64), (0x0u32) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm0 = ((crate::simd_ops::ru32(rbp.wrapping_sub(0x50u64))) as u32) as u128; xmm1 = ((crate::simd_ops::ru32(rbp.wrapping_sub(0x4cu64))) as u32) as u128; crate::simd_ops::wu32(rbx.wrapping_add(0x1cu64), ((xmm0 as u32)) as u32); crate::simd_ops::wu32(rbx.wrapping_add(0x20u64), ((xmm1 as u32)) as u32); crate::simd_ops::wu32(rbx.wrapping_add(0x28u64), (0x5u32) as u32); bb = 7; continue; }
            6 => { crate::simd_ops::wu32(rbx.wrapping_add(0x28u64), (0x2u32) as u32); bb = 7; continue; }
            7 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x70u64))) as u128; xmm7 = (crate::simd_ops::ru128(rsp.wrapping_add(0x60u64))) as u128; rsp = ((rsp).wrapping_add(0x80u64)) as u64; rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141723ab0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1434268d0; const P1: u64 = 0x143426ff0; const P2: u64 = 0x1416e30e0; const P3: u64 = 0x1417238b0;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm4: u128 = 0; let mut xmm5: u128 = 0; let mut xmm6: u128 = 0; let mut xmm7: u128 = 0; let mut xmm8: u128 = 0; let mut xmm9: u128 = 0; let mut xmm10: u128 = 0; let mut xmm11: u128 = 0; let mut xmm12: u128 = 0; let mut xmm13: u128 = 0; let mut xmm14: u128 = 0; let mut xmm15: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 27>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = rsp; crate::simd_ops::wu64(rax.wrapping_add(0x10u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0xd0u64)) as u64; rdx = (crate::simd_ops::ru32(rcx.wrapping_add(0x24u64))) as u32 as u64; rbx = rcx; r10 = (crate::simd_ops::ru64(rcx)) as u64; xmm2 = 0u128; xmm5 = ((crate::simd_ops::ru32(P0)) as u32) as u128; crate::simd_ops::wu128(rax.wrapping_sub(0x18u64), (xmm6) as u128); xmm6 = ((crate::simd_ops::ru32(P1)) as u32) as u128; r8 = (crate::simd_ops::ru32(r10.wrapping_add(0x18u64))) as u32 as u64; crate::simd_ops::wu128(rax.wrapping_sub(0x28u64), (xmm7) as u128); crate::simd_ops::wu128(rax.wrapping_sub(0x38u64), (xmm8) as u128); crate::simd_ops::wu128(rax.wrapping_sub(0x48u64), (xmm9) as u128); crate::simd_ops::wu128(rax.wrapping_sub(0x58u64), (xmm10) as u128); crate::simd_ops::wu128(rax.wrapping_sub(0x68u64), (xmm11) as u128); rax = ((r8.wrapping_sub(0x1u64)) as u32) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm15) as u128); crate::simd_ops::wu128(rsp.wrapping_add(0x60u64), (xmm12) as u128); rdx = (if (((fl_l as u32)) >= ((fl_r as u32))) { (rax as u32) } else { (rdx as u32) }) as u32 as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x50u64), (xmm13) as u128); crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm14) as u128); rdx = ((rdx.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64; rdx = ((rdx).wrapping_shl(4u32)) as u64; fl_l = ((rdx).wrapping_add(crate::simd_ops::ru64(r10.wrapping_add(0x10u64)))) as u64; fl_r = fl_l; rdx = ((fl_l as u64)) as u64; xmm15 = ((crate::simd_ops::ru32(rdx.wrapping_add(0x1cu64))) as u32) as u128; xmm8 = ((crate::simd_ops::ru32(rdx.wrapping_add(0x20u64))) as u32) as u128; xmm10 = ((crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32) as u128; xmm11 = ((crate::simd_ops::ru32(rdx.wrapping_add(0x8u64))) as u32) as u128; bb = 1; continue; }
            1 => { xmm9 = ((crate::simd_ops::ru32(rbx.wrapping_add(0x1cu64))) as u32) as u128; xmm4 = (xmm10) as u128; xmm1 = ((crate::simd_ops::ru32(rbx.wrapping_add(0x20u64))) as u32) as u128; xmm4 = (xmm4 & !0xffff_ffffu128) | ((f32::from_bits(xmm4 as u32) - f32::from_bits(xmm15 as u32)).to_bits() as u128); xmm0 = (xmm1) as u128; xmm3 = (xmm9) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) - f32::from_bits(xmm11 as u32)).to_bits() as u128); xmm7 = (xmm11) as u128; xmm3 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) - f32::from_bits(xmm10 as u32)).to_bits() as u128); xmm7 = (xmm7 & !0xffff_ffffu128) | ((f32::from_bits(xmm7 as u32) - f32::from_bits(xmm8 as u32)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) - f32::from_bits(xmm8 as u32)).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm4 as u32)).to_bits() as u128); xmm3 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) * f32::from_bits(xmm7 as u32)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm4 as u32)).to_bits() as u128); xmm3 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) - f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm0 = (xmm9) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) - f32::from_bits(xmm15 as u32)).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm7 as u32)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) - f32::from_bits(xmm0 as u32)).to_bits() as u128); fl_l = ((xmm1 as u32)) as u32 as u64; fl_r = ((xmm2 as u32)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 3; continue; } bb = 2; continue; }
            2 => { xmm0 = (xmm5) as u128; bb = 4; continue; }
            3 => { xmm0 = (xmm2) as u128; bb = 4; continue; }
            4 => { fl_l = ((xmm3 as u32)) as u32 as u64; fl_r = ((xmm2 as u32)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 6; continue; } bb = 5; continue; }
            5 => { xmm3 = (xmm2) as u128; bb = 7; continue; }
            6 => { xmm3 = (xmm5) as u128; bb = 7; continue; }
            7 => { fl_l = ((xmm7 as u32)) as u32 as u64; fl_r = ((xmm6 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 9; continue; } bb = 8; continue; }
            8 => { xmm1 = (xmm3) as u128; bb = 10; continue; }
            9 => { xmm1 = (xmm0) as u128; bb = 10; continue; }
            10 => { fl_l = ((xmm4 as u32)) as u32 as u64; fl_r = ((xmm2 as u32)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 13; continue; } bb = 11; continue; }
            11 => { fl_l = ((xmm4 as u32)) as u32 as u64; fl_r = ((xmm6 as u32)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 14; continue; } bb = 12; continue; }
            12 => { bb = 15; continue; }
            13 => { xmm1 = (xmm3) as u128; bb = 14; continue; }
            14 => { xmm0 = (xmm1) as u128; bb = 15; continue; }
            15 => { fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = ((xmm2 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) != f32::from_bits((fl_r as u32)) && !f32::from_bits((fl_l as u32)).is_nan() && !f32::from_bits((fl_r as u32)).is_nan()) { bb = 51; continue; } bb = 16; continue; }
            16 => { rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x24u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 48; continue; } bb = 17; continue; }
            17 => { rcx = (((rcx as u32)).wrapping_add(1)) as u32 as u64; xmm3 = (xmm9) as u128; crate::simd_ops::wu32(rbx.wrapping_add(0x24u64), ((rcx as u32)) as u32); xmm8 = ((crate::simd_ops::ru32(rbx.wrapping_add(0x20u64))) as u32) as u128; xmm14 = ((crate::simd_ops::ru32(rbx.wrapping_add(0xcu64))) as u32) as u128; xmm13 = (xmm8) as u128; xmm12 = ((crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32) as u128; xmm13 = (xmm13 & !0xffff_ffffu128) | ((f32::from_bits(xmm13 as u32) - f32::from_bits(xmm14 as u32)).to_bits() as u128); rax = (crate::simd_ops::ru32(r10.wrapping_add(0x18u64))) as u32 as u64; xmm9 = (xmm9 & !0xffff_ffffu128) | ((f32::from_bits(xmm9 as u32) - f32::from_bits(xmm12 as u32)).to_bits() as u128); rax = (((rax as u32)).wrapping_sub(1)) as u32 as u64; xmm0 = (xmm8) as u128; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; rcx = (if (((fl_l as u32)) >= ((fl_r as u32))) { (rax as u32) } else { (rcx as u32) }) as u32 as u64; rcx = ((rcx.wrapping_add(rcx.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(r10.wrapping_add(0x10u64)))) as u64; xmm1 = ((crate::simd_ops::ru32(rcx.wrapping_add(0x20u64))) as u32) as u128; xmm15 = ((crate::simd_ops::ru32(rcx.wrapping_add(0x1cu64))) as u32) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) - f32::from_bits(xmm1 as u32)).to_bits() as u128); xmm10 = ((crate::simd_ops::ru32(rcx.wrapping_add(0x4u64))) as u32) as u128; xmm3 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) - f32::from_bits(xmm15 as u32)).to_bits() as u128); xmm11 = ((crate::simd_ops::ru32(rcx.wrapping_add(0x8u64))) as u32) as u128; xmm7 = (xmm1) as u128; crate::simd_ops::wu32(rsp.wrapping_add(0xe0u64), ((xmm1 as u32)) as u32); xmm7 = (xmm7 & !0xffff_ffffu128) | ((f32::from_bits(xmm7 as u32) - f32::from_bits(xmm14 as u32)).to_bits() as u128); xmm4 = (xmm15) as u128; xmm1 = (xmm13) as u128; xmm4 = (xmm4 & !0xffff_ffffu128) | ((f32::from_bits(xmm4 as u32) - f32::from_bits(xmm12 as u32)).to_bits() as u128); xmm9 = (xmm9 & !0xffff_ffffu128) | ((f32::from_bits(xmm9 as u32) * f32::from_bits(xmm7 as u32)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm4 as u32)).to_bits() as u128); xmm3 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) * f32::from_bits(xmm7 as u32)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) - f32::from_bits(xmm9 as u32)).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm4 as u32)).to_bits() as u128); xmm3 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) - f32::from_bits(xmm0 as u32)).to_bits() as u128); fl_l = ((xmm1 as u32)) as u32 as u64; fl_r = ((xmm2 as u32)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 19; continue; } bb = 18; continue; }
            18 => { xmm0 = (xmm5) as u128; bb = 20; continue; }
            19 => { xmm0 = (xmm2) as u128; bb = 20; continue; }
            20 => { fl_l = ((xmm3 as u32)) as u32 as u64; fl_r = ((xmm2 as u32)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 22; continue; } bb = 21; continue; }
            21 => { xmm3 = (xmm2) as u128; bb = 23; continue; }
            22 => { xmm3 = (xmm5) as u128; bb = 23; continue; }
            23 => { fl_l = ((xmm7 as u32)) as u32 as u64; fl_r = ((xmm6 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 25; continue; } bb = 24; continue; }
            24 => { xmm1 = (xmm3) as u128; bb = 26; continue; }
            25 => { xmm1 = (xmm0) as u128; bb = 26; continue; }
            26 => { fl_l = ((xmm4 as u32)) as u32 as u64; fl_r = ((xmm2 as u32)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 29; continue; } bb = 27; continue; }
            27 => { fl_l = ((xmm4 as u32)) as u32 as u64; fl_r = ((xmm6 as u32)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 30; continue; } bb = 28; continue; }
            28 => { bb = 31; continue; }
            29 => { xmm1 = (xmm3) as u128; bb = 30; continue; }
            30 => { xmm0 = (xmm1) as u128; bb = 31; continue; }
            31 => { fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = ((xmm2 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) == f32::from_bits((fl_r as u32)) || f32::from_bits((fl_l as u32)).is_nan() || f32::from_bits((fl_r as u32)).is_nan()) { bb = 50; continue; } bb = 32; continue; }
            32 => { xmm1 = ((crate::simd_ops::ru32(rbx.wrapping_add(0x1cu64))) as u32) as u128; xmm8 = (xmm8 & !0xffff_ffffu128) | ((f32::from_bits(xmm8 as u32) - f32::from_bits(xmm11 as u32)).to_bits() as u128); xmm3 = (xmm1) as u128; xmm4 = (xmm10) as u128; xmm4 = (xmm4 & !0xffff_ffffu128) | ((f32::from_bits(xmm4 as u32) - f32::from_bits(xmm12 as u32)).to_bits() as u128); xmm7 = (xmm11) as u128; xmm7 = (xmm7 & !0xffff_ffffu128) | ((f32::from_bits(xmm7 as u32) - f32::from_bits(xmm14 as u32)).to_bits() as u128); xmm3 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) - f32::from_bits(xmm10 as u32)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) - f32::from_bits(xmm12 as u32)).to_bits() as u128); xmm0 = (xmm4) as u128; xmm8 = (xmm8 & !0xffff_ffffu128) | ((f32::from_bits(xmm8 as u32) * f32::from_bits(xmm4 as u32)).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm13 as u32)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm7 as u32)).to_bits() as u128); xmm3 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) * f32::from_bits(xmm7 as u32)).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) - f32::from_bits(xmm1 as u32)).to_bits() as u128); xmm3 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) - f32::from_bits(xmm8 as u32)).to_bits() as u128); fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = ((xmm2 as u32)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 34; continue; } bb = 33; continue; }
            33 => { xmm0 = (xmm5) as u128; bb = 35; continue; }
            34 => { xmm0 = (xmm2) as u128; bb = 35; continue; }
            35 => { fl_l = ((xmm3 as u32)) as u32 as u64; fl_r = ((xmm2 as u32)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 37; continue; } bb = 36; continue; }
            36 => { xmm3 = (xmm2) as u128; bb = 38; continue; }
            37 => { xmm3 = (xmm5) as u128; bb = 38; continue; }
            38 => { fl_l = ((xmm7 as u32)) as u32 as u64; fl_r = ((xmm6 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 40; continue; } bb = 39; continue; }
            39 => { xmm1 = (xmm3) as u128; bb = 41; continue; }
            40 => { xmm1 = (xmm0) as u128; bb = 41; continue; }
            41 => { fl_l = ((xmm4 as u32)) as u32 as u64; fl_r = ((xmm2 as u32)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 44; continue; } bb = 42; continue; }
            42 => { fl_l = ((xmm4 as u32)) as u32 as u64; fl_r = ((xmm6 as u32)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 45; continue; } bb = 43; continue; }
            43 => { bb = 46; continue; }
            44 => { xmm1 = (xmm3) as u128; bb = 45; continue; }
            45 => { xmm0 = (xmm1) as u128; bb = 46; continue; }
            46 => { fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = ((xmm2 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) != f32::from_bits((fl_r as u32)) && !f32::from_bits((fl_l as u32)).is_nan() && !f32::from_bits((fl_r as u32)).is_nan()) { bb = 50; continue; } bb = 47; continue; }
            47 => { xmm8 = ((crate::simd_ops::ru32(rsp.wrapping_add(0xe0u64))) as u32) as u128; bb = 1; continue; }
            48 => { xmm0 = ((crate::simd_ops::ru64(rbx.wrapping_add(0x1cu64))) as u64) as u128; rax = rsp.wrapping_add(0xe0); r9 = r10.wrapping_add(0x40); crate::simd_ops::wu64(rsp.wrapping_add(0xe0u64), ((xmm0 as u64)) as u64); rdx = rbx.wrapping_add(0x8); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); r8 = rbx.wrapping_add(0x1c); rcx = r10; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 50; continue; } bb = 49; continue; }
            49 => { xmm0 = ((crate::simd_ops::ru64(rsp.wrapping_add(0xe0u64))) as u64) as u128; crate::simd_ops::wu64(rbx.wrapping_add(0x1cu64), ((xmm0 as u64)) as u64); crate::simd_ops::wu32(rbx.wrapping_add(0x28u64), (0x4u32) as u32); bb = 55; continue; }
            50 => { rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 55; continue; }
            51 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x24u64))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 54; continue; } bb = 52; continue; }
            52 => { xmm0 = ((crate::simd_ops::ru64(rbx.wrapping_add(0x1cu64))) as u64) as u128; rax = rsp.wrapping_add(0xe0); r9 = r10.wrapping_add(0x28); crate::simd_ops::wu64(rsp.wrapping_add(0xe0u64), ((xmm0 as u64)) as u64); rdx = rbx.wrapping_add(0x8); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); r8 = rbx.wrapping_add(0x1c); rcx = r10; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 54; continue; } bb = 53; continue; }
            53 => { xmm0 = ((crate::simd_ops::ru64(rsp.wrapping_add(0xe0u64))) as u64) as u128; crate::simd_ops::wu64(rbx.wrapping_add(0x1cu64), ((xmm0 as u64)) as u64); crate::simd_ops::wu32(rbx.wrapping_add(0x28u64), (0x3u32) as u32); bb = 55; continue; }
            54 => { crate::simd_ops::wu32(rbx.wrapping_add(0x28u64), (0x2u32) as u32); bb = 55; continue; }
            55 => { xmm14 = (crate::simd_ops::ru128(rsp.wrapping_add(0x40u64))) as u128; r11 = rsp.wrapping_add(0xd0); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x18u64))) as u64; xmm6 = (crate::simd_ops::ru128(r11.wrapping_sub(0x10u64))) as u128; xmm7 = (crate::simd_ops::ru128(r11.wrapping_sub(0x20u64))) as u128; xmm8 = (crate::simd_ops::ru128(r11.wrapping_sub(0x30u64))) as u128; xmm9 = (crate::simd_ops::ru128(r11.wrapping_sub(0x40u64))) as u128; xmm10 = (crate::simd_ops::ru128(r11.wrapping_sub(0x50u64))) as u128; xmm11 = (crate::simd_ops::ru128(r11.wrapping_sub(0x60u64))) as u128; xmm13 = (crate::simd_ops::ru128(rsp.wrapping_add(0x50u64))) as u128; xmm12 = (crate::simd_ops::ru128(rsp.wrapping_add(0x60u64))) as u128; xmm15 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; rsp = r11; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141723e50(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1434268d0; const P1: u64 = 0x143426ff0; const P2: u64 = 0x1416e30e0; const P3: u64 = 0x1417238b0;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm4: u128 = 0; let mut xmm5: u128 = 0; let mut xmm6: u128 = 0; let mut xmm7: u128 = 0; let mut xmm8: u128 = 0; let mut xmm9: u128 = 0; let mut xmm10: u128 = 0; let mut xmm11: u128 = 0; let mut xmm12: u128 = 0; let mut xmm13: u128 = 0; let mut xmm14: u128 = 0; let mut xmm15: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 27>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = rsp; crate::simd_ops::wu64(rax.wrapping_add(0x10u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0xd0u64)) as u64; r10 = (crate::simd_ops::ru64(rcx)) as u64; r8 = 0x1; xmm5 = ((crate::simd_ops::ru32(P0)) as u32) as u128; rbx = rcx; crate::simd_ops::wu128(rax.wrapping_sub(0x18u64), (xmm6) as u128); xmm2 = 0u128; xmm6 = ((crate::simd_ops::ru32(P1)) as u32) as u128; crate::simd_ops::wu128(rax.wrapping_sub(0x28u64), (xmm7) as u128); crate::simd_ops::wu128(rax.wrapping_sub(0x38u64), (xmm8) as u128); crate::simd_ops::wu128(rax.wrapping_sub(0x48u64), (xmm9) as u128); crate::simd_ops::wu128(rax.wrapping_sub(0x58u64), (xmm10) as u128); crate::simd_ops::wu128(rax.wrapping_sub(0x68u64), (xmm11) as u128); rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x24u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm15) as u128); rax = (if (((fl_l as u32)) < ((fl_r as u32))) { (r8 as u32) } else { (rax as u32) }) as u32 as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x60u64), (xmm12) as u128); rax = (((rax as u32)).wrapping_sub(1)) as u32 as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x50u64), (xmm13) as u128); crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm14) as u128); rax = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; fl_l = ((rax).wrapping_add(crate::simd_ops::ru64(r10.wrapping_add(0x10u64)))) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; xmm10 = ((crate::simd_ops::ru32(rax.wrapping_add(0x1cu64))) as u32) as u128; xmm11 = ((crate::simd_ops::ru32(rax.wrapping_add(0x20u64))) as u32) as u128; xmm15 = ((crate::simd_ops::ru32(rax.wrapping_add(0x4u64))) as u32) as u128; xmm8 = ((crate::simd_ops::ru32(rax.wrapping_add(0x8u64))) as u32) as u128; bb = 1; continue; }
            1 => { xmm9 = ((crate::simd_ops::ru32(rbx.wrapping_add(0x1cu64))) as u32) as u128; xmm4 = (xmm10) as u128; xmm1 = ((crate::simd_ops::ru32(rbx.wrapping_add(0x20u64))) as u32) as u128; xmm4 = (xmm4 & !0xffff_ffffu128) | ((f32::from_bits(xmm4 as u32) - f32::from_bits(xmm15 as u32)).to_bits() as u128); xmm0 = (xmm1) as u128; xmm3 = (xmm9) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) - f32::from_bits(xmm11 as u32)).to_bits() as u128); xmm7 = (xmm11) as u128; xmm3 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) - f32::from_bits(xmm10 as u32)).to_bits() as u128); xmm7 = (xmm7 & !0xffff_ffffu128) | ((f32::from_bits(xmm7 as u32) - f32::from_bits(xmm8 as u32)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) - f32::from_bits(xmm8 as u32)).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm4 as u32)).to_bits() as u128); xmm3 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) * f32::from_bits(xmm7 as u32)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm4 as u32)).to_bits() as u128); xmm3 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) - f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm0 = (xmm9) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) - f32::from_bits(xmm15 as u32)).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm7 as u32)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) - f32::from_bits(xmm0 as u32)).to_bits() as u128); fl_l = ((xmm1 as u32)) as u32 as u64; fl_r = ((xmm2 as u32)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 3; continue; } bb = 2; continue; }
            2 => { xmm0 = (xmm5) as u128; bb = 4; continue; }
            3 => { xmm0 = (xmm2) as u128; bb = 4; continue; }
            4 => { fl_l = ((xmm3 as u32)) as u32 as u64; fl_r = ((xmm2 as u32)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 6; continue; } bb = 5; continue; }
            5 => { xmm3 = (xmm2) as u128; bb = 7; continue; }
            6 => { xmm3 = (xmm5) as u128; bb = 7; continue; }
            7 => { fl_l = ((xmm7 as u32)) as u32 as u64; fl_r = ((xmm6 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 9; continue; } bb = 8; continue; }
            8 => { xmm1 = (xmm3) as u128; bb = 10; continue; }
            9 => { xmm1 = (xmm0) as u128; bb = 10; continue; }
            10 => { fl_l = ((xmm4 as u32)) as u32 as u64; fl_r = ((xmm2 as u32)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 13; continue; } bb = 11; continue; }
            11 => { fl_l = ((xmm4 as u32)) as u32 as u64; fl_r = ((xmm6 as u32)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 14; continue; } bb = 12; continue; }
            12 => { bb = 15; continue; }
            13 => { xmm1 = (xmm3) as u128; bb = 14; continue; }
            14 => { xmm0 = (xmm1) as u128; bb = 15; continue; }
            15 => { fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = ((xmm2 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) != f32::from_bits((fl_r as u32)) && !f32::from_bits((fl_l as u32)).is_nan() && !f32::from_bits((fl_r as u32)).is_nan()) { bb = 51; continue; } bb = 16; continue; }
            16 => { rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x24u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x24u64), ((rcx as u32)) as u32); if (((fl_l as u32)) == ((fl_r as u32))) { bb = 48; continue; } bb = 17; continue; }
            17 => { xmm12 = ((crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32) as u128; rax = ((r8 as u32)) as u32 as u64; xmm8 = ((crate::simd_ops::ru32(rbx.wrapping_add(0x20u64))) as u32) as u128; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; xmm14 = ((crate::simd_ops::ru32(rbx.wrapping_add(0xcu64))) as u32) as u128; xmm3 = (xmm9) as u128; rax = (if (((fl_l as u32)) >= ((fl_r as u32))) { (rcx as u32) } else { (rax as u32) }) as u32 as u64; xmm9 = (xmm9 & !0xffff_ffffu128) | ((f32::from_bits(xmm9 as u32) - f32::from_bits(xmm12 as u32)).to_bits() as u128); rax = (((rax as u32)).wrapping_sub(1)) as u32 as u64; xmm13 = (xmm8) as u128; xmm13 = (xmm13 & !0xffff_ffffu128) | ((f32::from_bits(xmm13 as u32) - f32::from_bits(xmm14 as u32)).to_bits() as u128); xmm0 = (xmm8) as u128; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(r10.wrapping_add(0x10u64)))) as u64; xmm1 = ((crate::simd_ops::ru32(rcx.wrapping_add(0x8u64))) as u32) as u128; xmm15 = ((crate::simd_ops::ru32(rcx.wrapping_add(0x4u64))) as u32) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) - f32::from_bits(xmm1 as u32)).to_bits() as u128); xmm10 = ((crate::simd_ops::ru32(rcx.wrapping_add(0x1cu64))) as u32) as u128; xmm3 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) - f32::from_bits(xmm15 as u32)).to_bits() as u128); xmm11 = ((crate::simd_ops::ru32(rcx.wrapping_add(0x20u64))) as u32) as u128; xmm7 = (xmm1) as u128; crate::simd_ops::wu32(rsp.wrapping_add(0xe0u64), ((xmm1 as u32)) as u32); xmm7 = (xmm7 & !0xffff_ffffu128) | ((f32::from_bits(xmm7 as u32) - f32::from_bits(xmm14 as u32)).to_bits() as u128); xmm4 = (xmm15) as u128; xmm4 = (xmm4 & !0xffff_ffffu128) | ((f32::from_bits(xmm4 as u32) - f32::from_bits(xmm12 as u32)).to_bits() as u128); xmm9 = (xmm9 & !0xffff_ffffu128) | ((f32::from_bits(xmm9 as u32) * f32::from_bits(xmm7 as u32)).to_bits() as u128); xmm1 = (xmm4) as u128; xmm3 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) * f32::from_bits(xmm7 as u32)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm13 as u32)).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm4 as u32)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) - f32::from_bits(xmm9 as u32)).to_bits() as u128); xmm3 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) - f32::from_bits(xmm0 as u32)).to_bits() as u128); fl_l = ((xmm1 as u32)) as u32 as u64; fl_r = ((xmm2 as u32)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 19; continue; } bb = 18; continue; }
            18 => { xmm0 = (xmm5) as u128; bb = 20; continue; }
            19 => { xmm0 = (xmm2) as u128; bb = 20; continue; }
            20 => { fl_l = ((xmm3 as u32)) as u32 as u64; fl_r = ((xmm2 as u32)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 22; continue; } bb = 21; continue; }
            21 => { xmm3 = (xmm2) as u128; bb = 23; continue; }
            22 => { xmm3 = (xmm5) as u128; bb = 23; continue; }
            23 => { fl_l = ((xmm7 as u32)) as u32 as u64; fl_r = ((xmm6 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 25; continue; } bb = 24; continue; }
            24 => { xmm1 = (xmm3) as u128; bb = 26; continue; }
            25 => { xmm1 = (xmm0) as u128; bb = 26; continue; }
            26 => { fl_l = ((xmm4 as u32)) as u32 as u64; fl_r = ((xmm2 as u32)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 29; continue; } bb = 27; continue; }
            27 => { fl_l = ((xmm4 as u32)) as u32 as u64; fl_r = ((xmm6 as u32)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 30; continue; } bb = 28; continue; }
            28 => { bb = 31; continue; }
            29 => { xmm1 = (xmm3) as u128; bb = 30; continue; }
            30 => { xmm0 = (xmm1) as u128; bb = 31; continue; }
            31 => { fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = ((xmm2 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) == f32::from_bits((fl_r as u32)) || f32::from_bits((fl_l as u32)).is_nan() || f32::from_bits((fl_r as u32)).is_nan()) { bb = 50; continue; } bb = 32; continue; }
            32 => { xmm1 = ((crate::simd_ops::ru32(rbx.wrapping_add(0x1cu64))) as u32) as u128; xmm8 = (xmm8 & !0xffff_ffffu128) | ((f32::from_bits(xmm8 as u32) - f32::from_bits(xmm11 as u32)).to_bits() as u128); xmm3 = (xmm1) as u128; xmm4 = (xmm10) as u128; xmm4 = (xmm4 & !0xffff_ffffu128) | ((f32::from_bits(xmm4 as u32) - f32::from_bits(xmm12 as u32)).to_bits() as u128); xmm7 = (xmm11) as u128; xmm7 = (xmm7 & !0xffff_ffffu128) | ((f32::from_bits(xmm7 as u32) - f32::from_bits(xmm14 as u32)).to_bits() as u128); xmm3 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) - f32::from_bits(xmm10 as u32)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) - f32::from_bits(xmm12 as u32)).to_bits() as u128); xmm0 = (xmm4) as u128; xmm8 = (xmm8 & !0xffff_ffffu128) | ((f32::from_bits(xmm8 as u32) * f32::from_bits(xmm4 as u32)).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm13 as u32)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm7 as u32)).to_bits() as u128); xmm3 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) * f32::from_bits(xmm7 as u32)).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) - f32::from_bits(xmm1 as u32)).to_bits() as u128); xmm3 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) - f32::from_bits(xmm8 as u32)).to_bits() as u128); fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = ((xmm2 as u32)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 34; continue; } bb = 33; continue; }
            33 => { xmm0 = (xmm5) as u128; bb = 35; continue; }
            34 => { xmm0 = (xmm2) as u128; bb = 35; continue; }
            35 => { fl_l = ((xmm3 as u32)) as u32 as u64; fl_r = ((xmm2 as u32)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 37; continue; } bb = 36; continue; }
            36 => { xmm3 = (xmm2) as u128; bb = 38; continue; }
            37 => { xmm3 = (xmm5) as u128; bb = 38; continue; }
            38 => { fl_l = ((xmm7 as u32)) as u32 as u64; fl_r = ((xmm6 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 40; continue; } bb = 39; continue; }
            39 => { xmm1 = (xmm3) as u128; bb = 41; continue; }
            40 => { xmm1 = (xmm0) as u128; bb = 41; continue; }
            41 => { fl_l = ((xmm4 as u32)) as u32 as u64; fl_r = ((xmm2 as u32)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 44; continue; } bb = 42; continue; }
            42 => { fl_l = ((xmm4 as u32)) as u32 as u64; fl_r = ((xmm6 as u32)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 45; continue; } bb = 43; continue; }
            43 => { bb = 46; continue; }
            44 => { xmm1 = (xmm3) as u128; bb = 45; continue; }
            45 => { xmm0 = (xmm1) as u128; bb = 46; continue; }
            46 => { fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = ((xmm2 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) != f32::from_bits((fl_r as u32)) && !f32::from_bits((fl_l as u32)).is_nan() && !f32::from_bits((fl_r as u32)).is_nan()) { bb = 50; continue; } bb = 47; continue; }
            47 => { xmm8 = ((crate::simd_ops::ru32(rsp.wrapping_add(0xe0u64))) as u32) as u128; bb = 1; continue; }
            48 => { xmm0 = ((crate::simd_ops::ru64(rbx.wrapping_add(0x1cu64))) as u64) as u128; rax = rsp.wrapping_add(0xe0); r9 = r10.wrapping_add(0x28); crate::simd_ops::wu64(rsp.wrapping_add(0xe0u64), ((xmm0 as u64)) as u64); rdx = rbx.wrapping_add(0x8); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); r8 = rbx.wrapping_add(0x1c); rcx = r10; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 50; continue; } bb = 49; continue; }
            49 => { xmm0 = ((crate::simd_ops::ru64(rsp.wrapping_add(0xe0u64))) as u64) as u128; crate::simd_ops::wu64(rbx.wrapping_add(0x1cu64), ((xmm0 as u64)) as u64); crate::simd_ops::wu32(rbx.wrapping_add(0x28u64), (0x3u32) as u32); bb = 55; continue; }
            50 => { rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 55; continue; }
            51 => { rax = (crate::simd_ops::ru32(r10.wrapping_add(0x18u64))) as u32 as u64; fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x24u64))) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 54; continue; } bb = 52; continue; }
            52 => { xmm0 = ((crate::simd_ops::ru64(rbx.wrapping_add(0x1cu64))) as u64) as u128; rax = rsp.wrapping_add(0xe0); r9 = r10.wrapping_add(0x40); crate::simd_ops::wu64(rsp.wrapping_add(0xe0u64), ((xmm0 as u64)) as u64); rdx = rbx.wrapping_add(0x8); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); r8 = rbx.wrapping_add(0x1c); rcx = r10; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 54; continue; } bb = 53; continue; }
            53 => { xmm0 = ((crate::simd_ops::ru64(rsp.wrapping_add(0xe0u64))) as u64) as u128; crate::simd_ops::wu64(rbx.wrapping_add(0x1cu64), ((xmm0 as u64)) as u64); crate::simd_ops::wu32(rbx.wrapping_add(0x28u64), (0x4u32) as u32); bb = 55; continue; }
            54 => { crate::simd_ops::wu32(rbx.wrapping_add(0x28u64), (0x2u32) as u32); bb = 55; continue; }
            55 => { xmm14 = (crate::simd_ops::ru128(rsp.wrapping_add(0x40u64))) as u128; r11 = rsp.wrapping_add(0xd0); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x18u64))) as u64; xmm6 = (crate::simd_ops::ru128(r11.wrapping_sub(0x10u64))) as u128; xmm7 = (crate::simd_ops::ru128(r11.wrapping_sub(0x20u64))) as u128; xmm8 = (crate::simd_ops::ru128(r11.wrapping_sub(0x30u64))) as u128; xmm9 = (crate::simd_ops::ru128(r11.wrapping_sub(0x40u64))) as u128; xmm10 = (crate::simd_ops::ru128(r11.wrapping_sub(0x50u64))) as u128; xmm11 = (crate::simd_ops::ru128(r11.wrapping_sub(0x60u64))) as u128; xmm13 = (crate::simd_ops::ru128(rsp.wrapping_add(0x50u64))) as u128; xmm12 = (crate::simd_ops::ru128(rsp.wrapping_add(0x60u64))) as u128; xmm15 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; rsp = r11; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141724200(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x14; const P2: u64 = 0x18; const P3: u64 = 0x1c; const P4: u64 = 0x20; const P5: u64 = 0x28; const P6: u64 = 0x30; const P7: u64 = 0x38; const P8: u64 = 0x40; const P9: u64 = 0x48; const P10: u64 = 0x4c; const P11: u64 = 0x50;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rcx, rax);
    crate::simd_ops::wu32(rcx.wrapping_add(P0), (0x5u32) as u32);
    crate::simd_ops::wu32(rcx.wrapping_add(P1), (0x7fffffffu32) as u32);
    crate::simd_ops::wu32(rcx.wrapping_add(P2), (0x7fffffffu32) as u32);
    crate::simd_ops::wu32(rcx.wrapping_add(P3), (0x80000000u32) as u32);
    crate::simd_ops::wu64(rcx.wrapping_add(P4), (0xffffffff80000000u64) as u64);
    crate::simd_ops::wu32(rcx.wrapping_add(P5), (0xffffffffu32) as u32);
    crate::simd_ops::wu64(rcx.wrapping_add(P6), rax);
    crate::simd_ops::wu32(rcx.wrapping_add(P7), ((rax as u32)) as u32);
    crate::simd_ops::wu64(rcx.wrapping_add(P8), rax);
    crate::simd_ops::wu32(rcx.wrapping_add(P9), (0x7fffffffu32) as u32);
    crate::simd_ops::wu32(rcx.wrapping_add(P10), (0x7fffffffu32) as u32);
    crate::simd_ops::wu32(rcx.wrapping_add(P11), (0x80000000u32) as u32);
    crate::simd_ops::wu64(rcx.wrapping_add(0x54u64), (0xffffffff80000000u64) as u64);
    crate::simd_ops::wu32(rcx.wrapping_add(0x5cu64), (0xffffffffu32) as u32);
    crate::simd_ops::wu8(rcx.wrapping_add(0x60u64), ((rax as u8)) as u8);
    crate::simd_ops::wu64(rcx.wrapping_add(0x68u64), rax);
    crate::simd_ops::wu64(rcx.wrapping_add(0x70u64), rax);
    rax = rcx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141724270(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141724330; const P1: u64 = 0x70; const P2: u64 = 0x14169edb0; const P3: u64 = 0x68; const P4: u64 = 0x40;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64;
    fl_l = rcx; fl_r = rcx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); }
    rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P3))) as u64;
    fl_l = rcx; fl_r = rcx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); }
    rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P4))) as u64;
    fl_l = rcx; fl_r = rcx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); }
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1417242c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x20; const P1: u64 = 0x68; const P2: u64 = 0x8; const P3: u64 = 0x14169eda0; const P4: u64 = 0x14169edb0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rdx; rdx = rcx; crate::simd_ops::wu8(rbx.wrapping_add(P0), (0x1u8) as u8); fl_l = (crate::simd_ops::ru64(rbx)) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 7; continue; } bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; r8 = (crate::simd_ops::ru32(rdx.wrapping_add(P2))) as u32 as u64; rdx = rsp.wrapping_add(0x38); rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rdi = rax; rcx = (crate::simd_ops::ru64(rax)) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rdi)) as u64; crate::simd_ops::wu64(rbx, rax); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 7; continue; }
            7 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141724330(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x68; const P1: u64 = 0x8; const P2: u64 = 0x10; const P3: u64 = 0x38; const P4: u64 = 0x30; const P5: u64 = 0xffffffffffffffff; const P6: u64 = 0x14169edb0; const P7: u64 = 0x143932190; const P8: u64 = 0x60; const P9: u64 = 0x14; const P10: u64 = 0x18; const P11: u64 = 0x1c;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rcx; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = (crate::simd_ops::ru32(rbx.wrapping_add(P1))) as u32 as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(P3))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 9; continue; } bb = 3; continue; }
            3 => { rsi = ((rax as u32)) as u32 as u64; rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P4))) as u64; rdi = ((rax.wrapping_add(P5)) as u64) as u64; rdi = ((rdi.wrapping_add(rdi.wrapping_mul(4))) as u64) as u64; rdi = ((rdx.wrapping_add(rdi.wrapping_mul(8))) as u64) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru64(rdi)) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            6 => { rdi = ((rdi).wrapping_sub(0x28u64)) as u64; fl_l = rsi; fl_r = (0x1u64) as u64; rsi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } bb = 7; continue; }
            7 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P4))) as u64; bb = 8; continue; }
            8 => { rcx = (crate::simd_ops::ru64(P7)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P8))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 9; continue; }
            9 => { crate::simd_ops::wu32(rbx.wrapping_add(P9), (0x7fffffffu32) as u32); crate::simd_ops::wu32(rbx.wrapping_add(P10), (0x7fffffffu32) as u32); crate::simd_ops::wu32(rbx.wrapping_add(P11), (0x80000000u32) as u32); crate::simd_ops::wu64(rbx.wrapping_add(0x20u64), (0xffffffff80000000u64) as u64); crate::simd_ops::wu32(rbx.wrapping_add(0x28u64), (0xffffffffu32) as u32); rdi = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rbx.wrapping_add(0x30u64), rdi); crate::simd_ops::wu32(rbx.wrapping_add(0x38u64), ((rdi as u32)) as u32); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x40u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); bb = 11; continue; }
            11 => { crate::simd_ops::wu64(rbx.wrapping_add(0x40u64), rdi); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x70u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 12; continue; }
            12 => { rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); bb = 13; continue; }
            13 => { crate::simd_ops::wu64(rbx.wrapping_add(0x70u64), rdi); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141724400(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x4; const P1: u64 = 0x10; const P2: u64 = 0xffffffffffffffff; const P3: u64 = 0x18; const P4: u64 = 0x24; const P5: u64 = 0x14; const P6: u64 = 0x30; const P7: u64 = 0x1417242c0;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64; r11 = rdx; r9 = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64; r10 = rcx; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rdx = (if (rax as u32 as i32) < 0 { 0xffffffffu32 } else { 0u32 }) as u64; let d = ((r9 as u32)) as i32 as i64; let n = (((rdx as u32 as u64) << 32) | (rax as u32 as u64)) as i64; if d != 0 { rax = n.wrapping_div(d) as i32 as u32 as u64; rdx = n.wrapping_rem(d) as i32 as u32 as u64; } r8 = ((rax as u32)) as u32 as u64; bb = 3; continue; }
            2 => { rax = (((rax as u32)).wrapping_add(1)) as u32 as u64; rdx = (if (rax as u32 as i32) < 0 { 0xffffffffu32 } else { 0u32 }) as u64; let d = ((r9 as u32)) as i32 as i64; let n = (((rdx as u32 as u64) << 32) | (rax as u32 as u64)) as i64; if d != 0 { rax = n.wrapping_div(d) as i32 as u32 as u64; rdx = n.wrapping_rem(d) as i32 as u32 as u64; } r8 = ((rax.wrapping_add(P2)) as u32) as u32 as u64; bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru32(r11)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rdx = (if (rax as u32 as i32) < 0 { 0xffffffffu32 } else { 0u32 }) as u64; let d = ((r9 as u32)) as i32 as i64; let n = (((rdx as u32 as u64) << 32) | (rax as u32 as u64)) as i64; if d != 0 { rax = n.wrapping_div(d) as i32 as u32 as u64; rdx = n.wrapping_rem(d) as i32 as u32 as u64; } bb = 6; continue; }
            5 => { rax = (((rax as u32)).wrapping_add(1)) as u32 as u64; rdx = (if (rax as u32 as i32) < 0 { 0xffffffffu32 } else { 0u32 }) as u64; let d = ((r9 as u32)) as i32 as i64; let n = (((rdx as u32 as u64) << 32) | (rax as u32 as u64)) as i64; if d != 0 { rax = n.wrapping_div(d) as i32 as u32 as u64; rdx = n.wrapping_rem(d) as i32 as u32 as u64; } fl_l = (((rax as u32)).wrapping_sub(1)) as u32 as u64; rax = ((fl_l as u32)) as u32 as u64; bb = 6; continue; }
            6 => { r8 = (((r8 as u32)).wrapping_sub(crate::simd_ops::ru32(rcx.wrapping_add(P3)))) as u32 as u64; r8 = ((((r8 as u32)) as i32).wrapping_mul((crate::simd_ops::ru32(rcx.wrapping_add(P4))) as i32) as u32) as u32 as u64; r8 = (((r8 as u32)).wrapping_sub(crate::simd_ops::ru32(rcx.wrapping_add(P5)))) as u32 as u64; fl_l = (((r8 as u32)).wrapping_add((rax as u32))) as u32 as u64; fl_r = fl_l; r8 = ((fl_l as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(r10.wrapping_add(P6))) as u64; rcx = ((r8.wrapping_add(r8.wrapping_mul(4))) as u64) as u64; rdx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; rcx = r10; return crate::simd_ops::call_fwd(P7, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141724460(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143932190; const P1: u64 = 0x14169eda0; const P2: u64 = 0x14169edb0;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 23>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rbp = rsp; rsp = ((rsp).wrapping_sub(0x78u64)) as u64; r8 = rdx; rsi = rcx; r14 = rcx.wrapping_add(0x14); crate::simd_ops::wu64(rbp.wrapping_sub(0x50u64), r14); xmm0 = (crate::simd_ops::ru128(r14)) as u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x48u64), (xmm0) as u128); xmm1 = ((crate::simd_ops::ru64(r14.wrapping_add(0x10u64))) as u64) as u128; crate::simd_ops::wu64(rbp.wrapping_sub(0x38u64), ((xmm1 as u64)) as u64); rax = (crate::simd_ops::ru32(rdx.wrapping_add(0xcu64))) as u32 as u64; rcx = (crate::simd_ops::ru32(rcx.wrapping_add(0x10u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rdx = (if (rax as u32 as i32) < 0 { 0xffffffffu32 } else { 0u32 }) as u64; let d = ((rcx as u32)) as i32 as i64; let n = (((rdx as u32 as u64) << 32) | (rax as u32 as u64)) as i64; if d != 0 { rax = n.wrapping_div(d) as i32 as u32 as u64; rdx = n.wrapping_rem(d) as i32 as u32 as u64; } r10 = ((rax as u32)) as u32 as u64; bb = 3; continue; }
            2 => { rax = (((rax as u32)).wrapping_add(1)) as u32 as u64; rdx = (if (rax as u32 as i32) < 0 { 0xffffffffu32 } else { 0u32 }) as u64; let d = ((rcx as u32)) as i32 as i64; let n = (((rdx as u32 as u64) << 32) | (rax as u32 as u64)) as i64; if d != 0 { rax = n.wrapping_div(d) as i32 as u32 as u64; rdx = n.wrapping_rem(d) as i32 as u32 as u64; } r10 = ((rax.wrapping_sub(0x1u64)) as u32) as u32 as u64; bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru32(r8.wrapping_add(0x8u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rdx = (if (rax as u32 as i32) < 0 { 0xffffffffu32 } else { 0u32 }) as u64; let d = ((rcx as u32)) as i32 as i64; let n = (((rdx as u32 as u64) << 32) | (rax as u32 as u64)) as i64; if d != 0 { rax = n.wrapping_div(d) as i32 as u32 as u64; rdx = n.wrapping_rem(d) as i32 as u32 as u64; } r9 = ((rax as u32)) as u32 as u64; bb = 6; continue; }
            5 => { rax = (((rax as u32)).wrapping_add(1)) as u32 as u64; rdx = (if (rax as u32 as i32) < 0 { 0xffffffffu32 } else { 0u32 }) as u64; let d = ((rcx as u32)) as i32 as i64; let n = (((rdx as u32 as u64) << 32) | (rax as u32 as u64)) as i64; if d != 0 { rax = n.wrapping_div(d) as i32 as u32 as u64; rdx = n.wrapping_rem(d) as i32 as u32 as u64; } r9 = ((rax.wrapping_sub(0x1u64)) as u32) as u32 as u64; bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru32(r8.wrapping_add(0x4u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { rdx = (if (rax as u32 as i32) < 0 { 0xffffffffu32 } else { 0u32 }) as u64; let d = ((rcx as u32)) as i32 as i64; let n = (((rdx as u32 as u64) << 32) | (rax as u32 as u64)) as i64; if d != 0 { rax = n.wrapping_div(d) as i32 as u32 as u64; rdx = n.wrapping_rem(d) as i32 as u32 as u64; } r11 = ((rax as u32)) as u32 as u64; bb = 9; continue; }
            8 => { rax = (((rax as u32)).wrapping_add(1)) as u32 as u64; rdx = (if (rax as u32 as i32) < 0 { 0xffffffffu32 } else { 0u32 }) as u64; let d = ((rcx as u32)) as i32 as i64; let n = (((rdx as u32 as u64) << 32) | (rax as u32 as u64)) as i64; if d != 0 { rax = n.wrapping_div(d) as i32 as u32 as u64; rdx = n.wrapping_rem(d) as i32 as u32 as u64; } r11 = ((rax.wrapping_sub(0x1u64)) as u32) as u32 as u64; bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru32(r8)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { rdx = (if (rax as u32 as i32) < 0 { 0xffffffffu32 } else { 0u32 }) as u64; let d = ((rcx as u32)) as i32 as i64; let n = (((rdx as u32 as u64) << 32) | (rax as u32 as u64)) as i64; if d != 0 { rax = n.wrapping_div(d) as i32 as u32 as u64; rdx = n.wrapping_rem(d) as i32 as u32 as u64; } bb = 12; continue; }
            11 => { rax = (((rax as u32)).wrapping_add(1)) as u32 as u64; rdx = (if (rax as u32 as i32) < 0 { 0xffffffffu32 } else { 0u32 }) as u64; let d = ((rcx as u32)) as i32 as i64; let n = (((rdx as u32 as u64) << 32) | (rax as u32 as u64)) as i64; if d != 0 { rax = n.wrapping_div(d) as i32 as u32 as u64; rdx = n.wrapping_rem(d) as i32 as u32 as u64; } fl_l = (((rax as u32)).wrapping_sub(1)) as u32 as u64; rax = ((fl_l as u32)) as u32 as u64; bb = 12; continue; }
            12 => { rcx = (crate::simd_ops::ru32(r14)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; rcx = (if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { (rax as u32) } else { (rcx as u32) }) as u32 as u64; crate::simd_ops::wu32(r14, ((rcx as u32)) as u32); fl_l = (crate::simd_ops::ru32(r14.wrapping_add(0x8u64))) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; r9 = (if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { crate::simd_ops::ru32(r14.wrapping_add(0x8u64)) } else { (r9 as u32) }) as u32 as u64; crate::simd_ops::wu32(r14.wrapping_add(0x8u64), ((r9 as u32)) as u32); rdx = (crate::simd_ops::ru32(r14.wrapping_add(0x4u64))) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((r11 as u32)) as u32 as u64; rdx = (if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { (r11 as u32) } else { (rdx as u32) }) as u32 as u64; crate::simd_ops::wu32(r14.wrapping_add(0x4u64), ((rdx as u32)) as u32); fl_l = (crate::simd_ops::ru32(r14.wrapping_add(0xcu64))) as u32 as u64; fl_r = ((r10 as u32)) as u32 as u64; r10 = (if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { crate::simd_ops::ru32(r14.wrapping_add(0xcu64)) } else { (r10 as u32) }) as u32 as u64; crate::simd_ops::wu32(r14.wrapping_add(0xcu64), ((r10 as u32)) as u32); r9 = (((r9 as u32)).wrapping_sub((rcx as u32))) as u32 as u64; r9 = (((r9 as u32)).wrapping_add(1)) as u32 as u64; crate::simd_ops::wu32(r14.wrapping_add(0x10u64), ((r9 as u32)) as u32); r10 = (((r10 as u32)).wrapping_sub((rdx as u32))) as u32 as u64; r10 = (((r10 as u32)).wrapping_add(1)) as u32 as u64; crate::simd_ops::wu32(r14.wrapping_add(0x14u64), ((r10 as u32)) as u32); fl_l = ((rcx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rbp.wrapping_sub(0x48u64))) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 16; continue; } bb = 13; continue; }
            13 => { rax = (crate::simd_ops::ru64(rbp.wrapping_sub(0x48u64))) as u64; rax = ((rax).wrapping_shr(32u32)) as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 16; continue; } bb = 14; continue; }
            14 => { rax = (crate::simd_ops::ru32(rbp.wrapping_sub(0x40u64))) as u32 as u64; fl_l = (crate::simd_ops::ru32(rsi.wrapping_add(0x1cu64))) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 16; continue; } bb = 15; continue; }
            15 => { rax = (crate::simd_ops::ru64(rbp.wrapping_sub(0x40u64))) as u64; rax = ((rax).wrapping_shr(32u32)) as u64; fl_l = (crate::simd_ops::ru32(rsi.wrapping_add(0x20u64))) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 53; continue; } bb = 16; continue; }
            16 => { r12 = (crate::simd_ops::ru32(rsi.wrapping_add(0x28u64))) as u32 as u64; r12 = ((((r12 as u32)) as i32).wrapping_mul((crate::simd_ops::ru32(rsi.wrapping_add(0x24u64))) as i32) as u32) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x58u64), ((r12 as u32)) as u32); r15 = (crate::simd_ops::ru64(rsi.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x60u64), r15); rbx = ((r12 as u32)) as u32 as u64; rcx = (crate::simd_ops::ru64(P0)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = ((r12.wrapping_add(r12.wrapping_mul(4))) as u64) as u64; rdx = ((rdx).wrapping_shl(3u32)) as u64; r8 = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x48u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rsi.wrapping_add(0x30u64), rax); r12 = 0; fl_l = 0; fl_r = 0; rdi = 0x80000000; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 19; continue; } bb = 17; continue; }
            17 => { rcx = rax.wrapping_add(0x14); bb = 18; continue; }
            18 => { crate::simd_ops::wu64(rax, r12); crate::simd_ops::wu32(rcx.wrapping_sub(0xcu64), (0x7fffffffu32) as u32); crate::simd_ops::wu32(rcx.wrapping_sub(0x8u64), (0x7fffffffu32) as u32); crate::simd_ops::wu32(rcx.wrapping_sub(0x4u64), ((rdi as u32)) as u32); crate::simd_ops::wu64(rcx, (0xffffffff80000000u64) as u64); crate::simd_ops::wu32(rcx.wrapping_add(0x8u64), (0xffffffffu32) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0xcu64), ((r12 as u8)) as u8); rax = rax.wrapping_add(0x28); rcx = rcx.wrapping_add(0x28); fl_l = rbx; fl_r = (0x1u64) as u64; rbx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 18; continue; } bb = 19; continue; }
            19 => { crate::simd_ops::wu32(rbp.wrapping_add(0x50u64), ((rdi as u32)) as u32); r13 = ((rdi as u32)) as u32 as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x58u64), r14); fl_l = (crate::simd_ops::ru32(rbp.wrapping_sub(0x38u64))) as u32 as u64; fl_r = ((r12 as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) <= (((fl_r as u32)) as i32)) { bb = 24; continue; } bb = 20; continue; }
            20 => { fl_l = (crate::simd_ops::ru32(rbp.wrapping_sub(0x34u64))) as u32 as u64; fl_r = ((r12 as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) <= (((fl_r as u32)) as i32)) { bb = 24; continue; } bb = 21; continue; }
            21 => { rax = (crate::simd_ops::ru32(r14.wrapping_add(0x10u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 24; continue; } bb = 22; continue; }
            22 => { fl_l = (crate::simd_ops::ru32(r14.wrapping_add(0x14u64))) as u32 as u64; fl_r = ((r12 as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) <= (((fl_r as u32)) as i32)) { bb = 24; continue; } bb = 23; continue; }
            23 => { rbx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x48u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x20u64), rbx); rsi = ((r12 as u32)) as u32 as u64; rbx = ((rbx).wrapping_shr(32u32)) as u64; rbx = (((rbx as u32)).wrapping_sub(crate::simd_ops::ru32(r14.wrapping_add(0x4u64)))) as u32 as u64; rbx = ((((rbx as u32)) as i32).wrapping_mul(((rax as u32)) as i32) as u32) as u32 as u64; rbx = (((rbx as u32)).wrapping_sub(crate::simd_ops::ru32(r14))) as u32 as u64; fl_l = (((rbx as u32)).wrapping_add(crate::simd_ops::ru32(rbp.wrapping_sub(0x48u64)))) as u32 as u64; fl_r = fl_l; rbx = ((fl_l as u32)) as u32 as u64; rdi = (crate::simd_ops::ru32(rbp.wrapping_sub(0x1cu64))) as u32 as u64; r13 = (crate::simd_ops::ru32(rbp.wrapping_sub(0x20u64))) as u32 as u64; bb = 25; continue; }
            24 => { rsi = 0xffffffff; rdi = (crate::simd_ops::ru32(rbp.wrapping_add(0x50u64))) as u32 as u64; rbx = ((rsi as u32)) as u32 as u64; bb = 25; continue; }
            25 => { r14 = (crate::simd_ops::ru64(rbp.wrapping_add(0x48u64))) as u64; bb = 26; continue; }
            26 => { fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 37; continue; } bb = 27; continue; }
            27 => { fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 37; continue; } bb = 28; continue; }
            28 => { rax = ((rsi as u32)) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; r12 = ((r15.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; rax = ((rbx as u32)) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = (crate::simd_ops::ru64(r14.wrapping_add(0x30u64))) as u64; r15 = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; rcx = (crate::simd_ops::ru64(r12)) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 30; continue; } bb = 29; continue; }
            29 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 30; continue; }
            30 => { rcx = (crate::simd_ops::ru64(r15)) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 32; continue; } bb = 31; continue; }
            31 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 32; continue; }
            32 => { rax = (crate::simd_ops::ru64(r12)) as u64; crate::simd_ops::wu64(r15, rax); xmm0 = (crate::simd_ops::ru128(r12.wrapping_add(0x8u64))) as u128; crate::simd_ops::wu128(r15.wrapping_add(0x8u64), (xmm0) as u128); xmm1 = ((crate::simd_ops::ru64(r12.wrapping_add(0x18u64))) as u64) as u128; crate::simd_ops::wu64(r15.wrapping_add(0x18u64), ((xmm1 as u64)) as u64); rax = ((crate::simd_ops::ru8(r12.wrapping_add(0x20u64))) as u32) as u32 as u64; crate::simd_ops::wu8(r15.wrapping_add(0x20u64), ((rax as u8)) as u8); r15 = (crate::simd_ops::ru64(rbp.wrapping_add(0x60u64))) as u64; fl_l = ((r13 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rbp.wrapping_sub(0x40u64))) as u32 as u64; if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { bb = 34; continue; } bb = 33; continue; }
            33 => { r13 = (((r13 as u32)).wrapping_add(1)) as u32 as u64; rsi = (((rsi as u32)).wrapping_add(1)) as u32 as u64; fl_l = (((rbx as u32)).wrapping_add(1)) as u32 as u64; rbx = ((fl_l as u32)) as u32 as u64; bb = 26; continue; }
            34 => { fl_l = ((rdi as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rbp.wrapping_sub(0x3cu64))) as u32 as u64; if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { bb = 36; continue; } bb = 35; continue; }
            35 => { r13 = (crate::simd_ops::ru32(rbp.wrapping_sub(0x48u64))) as u32 as u64; rdi = (((rdi as u32)).wrapping_add(1)) as u32 as u64; rsi = (((rsi as u32)).wrapping_add(1)) as u32 as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x58u64))) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x10u64))) as u32 as u64; rax = (((rax as u32)).wrapping_sub(crate::simd_ops::ru32(rbp.wrapping_sub(0x38u64)))) as u32 as u64; rax = (((rax as u32)).wrapping_add(1)) as u32 as u64; fl_l = (((rbx as u32)).wrapping_add((rax as u32))) as u32 as u64; fl_r = fl_l; rbx = ((fl_l as u32)) as u32 as u64; bb = 26; continue; }
            36 => { rsi = 0xffffffff; rbx = ((rsi as u32)) as u32 as u64; bb = 26; continue; }
            37 => { rsi = (crate::simd_ops::ru32(r14.wrapping_add(0x38u64))) as u32 as u64; rbx = rsi.wrapping_sub(0x1); rbx = ((rbx.wrapping_add(rbx.wrapping_mul(4))) as u64) as u64; rbx = ((r15.wrapping_add(rbx.wrapping_mul(8))) as u64) as u64; fl_l = rsi; fl_r = rsi; rdi = 0x80000000; r14 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x50u64))) as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 42; continue; } bb = 38; continue; }
            38 => { bb = 39; continue; }
            39 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 41; continue; } bb = 40; continue; }
            40 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 41; continue; }
            41 => { rbx = ((rbx).wrapping_sub(0x28u64)) as u64; fl_l = rsi; fl_r = (0x1u64) as u64; rsi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 39; continue; } bb = 42; continue; }
            42 => { rcx = (crate::simd_ops::ru64(P0)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = r15; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x60u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rsi = (crate::simd_ops::ru64(rbp.wrapping_add(0x48u64))) as u64; rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x58u64))) as u32 as u64; crate::simd_ops::wu32(rsi.wrapping_add(0x38u64), ((rax as u32)) as u32); r11 = ((rdi as u32)) as u32 as u64; r10 = 0xffffffff; fl_l = (crate::simd_ops::ru32(r14.wrapping_add(0x10u64))) as u32 as u64; fl_r = 0x0; if ((((fl_l as u32)) as i32) <= (((fl_r as u32)) as i32)) { bb = 45; continue; } bb = 43; continue; }
            43 => { fl_l = (crate::simd_ops::ru32(r14.wrapping_add(0x14u64))) as u32 as u64; fl_r = 0x0; if ((((fl_l as u32)) as i32) <= (((fl_r as u32)) as i32)) { bb = 45; continue; } bb = 44; continue; }
            44 => { rax = (crate::simd_ops::ru64(r14)) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x28u64), rax); r10 = 0; fl_l = 0; fl_r = 0; r11 = (crate::simd_ops::ru32(rbp.wrapping_sub(0x24u64))) as u32 as u64; rdi = ((rax as u32)) as u32 as u64; bb = 45; continue; }
            45 => { fl_l = ((r10 as u32)) as u32 as u64; fl_r = ((r10 as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 53; continue; } bb = 46; continue; }
            46 => { rax = ((r10 as u32)) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x30u64))) as u64; rbx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = (crate::simd_ops::ru64(rbx)) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 48; continue; } bb = 47; continue; }
            47 => { rcx = (crate::simd_ops::ru32(rsi.wrapping_add(0x10u64))) as u32 as u64; r8 = ((rdi as u32)) as u32 as u64; r8 = ((((r8 as u32)) as i32).wrapping_mul(((rcx as u32)) as i32) as u32) as u32 as u64; r9 = ((r11 as u32)) as u32 as u64; r9 = ((((r9 as u32)) as i32).wrapping_mul(((rcx as u32)) as i32) as u32) as u32 as u64; rax = ((rcx.wrapping_sub(0x1u64)) as u32) as u32 as u64; rax = (((rax as u32)).wrapping_add((r8 as u32))) as u32 as u64; rdx = ((rcx.wrapping_sub(0x1u64)) as u32) as u32 as u64; rdx = (((rdx as u32)).wrapping_add((r9 as u32))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_sub(0x30u64), ((r8 as u32)) as u32); crate::simd_ops::wu32(rbp.wrapping_sub(0x2cu64), ((r9 as u32)) as u32); crate::simd_ops::wu32(rbp.wrapping_sub(0x28u64), ((rax as u32)) as u32); crate::simd_ops::wu32(rbp.wrapping_sub(0x24u64), ((rdx as u32)) as u32); rax = (((rax as u32)).wrapping_sub((r8 as u32))) as u32 as u64; rax = (((rax as u32)).wrapping_add(1)) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_sub(0x20u64), ((rax as u32)) as u32); fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; rdx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; rax = ((rdx.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_sub(0x1cu64), ((rax as u32)) as u32); xmm0 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x30u64))) as u128; crate::simd_ops::wu128(rbx.wrapping_add(0x8u64), (xmm0) as u128); xmm1 = ((crate::simd_ops::ru64(rbp.wrapping_sub(0x20u64))) as u64) as u128; crate::simd_ops::wu64(rbx.wrapping_add(0x18u64), ((xmm1 as u64)) as u64); bb = 48; continue; }
            48 => { fl_l = ((rdi as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(r14.wrapping_add(0x8u64))) as u32 as u64; if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { bb = 50; continue; } bb = 49; continue; }
            49 => { rdi = (((rdi as u32)).wrapping_add(1)) as u32 as u64; fl_l = (((r10 as u32)).wrapping_add(1)) as u32 as u64; r10 = ((fl_l as u32)) as u32 as u64; bb = 45; continue; }
            50 => { fl_l = ((r11 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(r14.wrapping_add(0xcu64))) as u32 as u64; if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { bb = 52; continue; } bb = 51; continue; }
            51 => { rdi = (crate::simd_ops::ru32(r14)) as u32 as u64; r11 = (((r11 as u32)).wrapping_add(1)) as u32 as u64; fl_l = (((r10 as u32)).wrapping_add(1)) as u32 as u64; r10 = ((fl_l as u32)) as u32 as u64; bb = 45; continue; }
            52 => { r10 = 0xffffffff; bb = 45; continue; }
            53 => { rsp = ((rsp).wrapping_add(0x78u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141724820(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141724cb0; const P1: u64 = 0x143426780; const P2: u64 = 0x143427068; const P3: u64 = 0x141714b00; const P4: u64 = 0x1416daf20; const P5: u64 = 0x143932190; const P6: u64 = 0x1416a8460; const P7: u64 = 0x1416a82a0;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0; let mut xmm7: u128 = 0; let mut xmm8: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 31>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = rsp; crate::simd_ops::wu64(rax.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rax.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rax.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rbp = rax.wrapping_sub(0x5f); rsp = ((rsp).wrapping_sub(0xd0u64)) as u64; crate::simd_ops::wu128(rax.wrapping_sub(0x38u64), (xmm6) as u128); crate::simd_ops::wu128(rax.wrapping_sub(0x48u64), (xmm7) as u128); crate::simd_ops::wu128(rax.wrapping_sub(0x58u64), (xmm8) as u128); rdi = rdx; r13 = rcx; rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; rbx = (crate::simd_ops::ru32(rax.wrapping_add(0xb0u64))) as u32 as u64; fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 54; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0xa2u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 6; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0xa3u64))) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 6; continue; } bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0xa4u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 6; continue; } bb = 4; continue; }
            4 => { rdx = ((rdx).wrapping_add(0xa8u64)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rdx = rdi.wrapping_add(0xc0); rcx = r13; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 54; continue; } bb = 6; continue; }
            6 => { r8 = (crate::simd_ops::ru32(rdi.wrapping_add(0x94u64))) as u32 as u64; r8 = (((r8 as u32)).wrapping_add(1)) as u32 as u64; rdx = (crate::simd_ops::ru32(rdi.wrapping_add(0x90u64))) as u32 as u64; rdx = (((rdx as u32)).wrapping_add(1)) as u32 as u64; rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x8cu64))) as u32 as u64; rcx = (((rcx as u32)).wrapping_sub(1)) as u32 as u64; rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x88u64))) as u32 as u64; rax = (((rax as u32)).wrapping_sub(1)) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_sub(0x79u64), ((rax as u32)) as u32); crate::simd_ops::wu32(rbp.wrapping_sub(0x75u64), ((rcx as u32)) as u32); crate::simd_ops::wu32(rbp.wrapping_sub(0x71u64), ((rdx as u32)) as u32); crate::simd_ops::wu32(rbp.wrapping_sub(0x6du64), ((r8 as u32)) as u32); rdx = (((rdx as u32)).wrapping_sub((rax as u32))) as u32 as u64; rdx = (((rdx as u32)).wrapping_add(1)) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_sub(0x69u64), ((rdx as u32)) as u32); r8 = (((r8 as u32)).wrapping_sub((rcx as u32))) as u32 as u64; r8 = (((r8 as u32)).wrapping_add(1)) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_sub(0x65u64), ((r8 as u32)) as u32); fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 54; continue; } bb = 7; continue; }
            7 => { r12 = 0; fl_l = 0; fl_r = 0; r15 = rbx; crate::simd_ops::wu64(rbp.wrapping_add(0x67u64), rbx); xmm8 = 0u128; xmm6 = ((crate::simd_ops::ru32(P1)) as u32) as u128; xmm7 = ((crate::simd_ops::ru32(P2)) as u32) as u128; bb = 8; continue; }
            8 => { rax = (crate::simd_ops::ru64(r13.wrapping_add(0x10u64))) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(0xa8u64))) as u64; rsi = (crate::simd_ops::ru64(r12.wrapping_add(rcx))) as u64; r14 = (crate::simd_ops::ru64(rsi.wrapping_add(0x70u64))) as u64; fl_l = r14; fl_r = r14; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 53; continue; } bb = 9; continue; }
            9 => { fl_l = (crate::simd_ops::ru32(rsi.wrapping_add(0x84u64))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 53; continue; } bb = 10; continue; }
            10 => { r8 = (crate::simd_ops::ru32(r14.wrapping_add(0x34u64))) as u32 as u64; r8 = (((r8 as u32)).wrapping_sub(crate::simd_ops::ru32(rdi.wrapping_add(0x88u64)))) as u32 as u64; r8 = (((((r8 as u32)) as i32).wrapping_shr(31u32) as u32)) as u32 as u64; r8 = (((r8 as u32)).wrapping_add(1)) as u32 as u64; rdx = (crate::simd_ops::ru32(rdi.wrapping_add(0x90u64))) as u32 as u64; rdx = (((rdx as u32)).wrapping_sub(crate::simd_ops::ru32(r14.wrapping_add(0x2cu64)))) as u32 as u64; rdx = (((((rdx as u32)) as i32).wrapping_shr(31u32) as u32)) as u32 as u64; rdx = (((rdx as u32)).wrapping_add(1)) as u32 as u64; rcx = (crate::simd_ops::ru32(r14.wrapping_add(0x38u64))) as u32 as u64; rcx = (((rcx as u32)).wrapping_sub(crate::simd_ops::ru32(rdi.wrapping_add(0x8cu64)))) as u32 as u64; rcx = (((((rcx as u32)) as i32).wrapping_shr(31u32) as u32)) as u32 as u64; rcx = (((rcx as u32)).wrapping_add(1)) as u32 as u64; rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x94u64))) as u32 as u64; rax = (((rax as u32)).wrapping_sub(crate::simd_ops::ru32(r14.wrapping_add(0x30u64)))) as u32 as u64; rax = (((((rax as u32)) as i32).wrapping_shr(31u32) as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_add(1)) as u32 as u64; rcx = ((((rcx as u32)) & ((rax as u32)))) as u32 as u64; rcx = ((((rcx as u32)) & ((r8 as u32)))) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 53; continue; } bb = 11; continue; }
            11 => { rbx = 0; fl_l = 0; fl_r = 0; rcx = (crate::simd_ops::ru32(rsi.wrapping_add(0x16cu64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 14; continue; } bb = 12; continue; }
            12 => { fl_l = (crate::simd_ops::ru32(rsi.wrapping_add(0x170u64))) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 14; continue; } bb = 13; continue; }
            13 => { rax = (((rcx as u8)) as u32) as u32 as u64; bb = 15; continue; }
            14 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 15; continue; }
            15 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 24; continue; } bb = 16; continue; }
            16 => { rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x158u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 18; continue; } bb = 17; continue; }
            17 => { fl_l = (crate::simd_ops::lk_add_u32(rax.wrapping_add(0x78u64), (1) as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x158u64))) as u64; rcx = (crate::simd_ops::ru32(rsi.wrapping_add(0x16cu64))) as u32 as u64; bb = 18; continue; }
            18 => { crate::simd_ops::wu64(rbp.wrapping_sub(0x61u64), rax); xmm0 = ((crate::simd_ops::ru64(rsi.wrapping_add(0x160u64))) as u64) as u128; crate::simd_ops::wu64(rbp.wrapping_sub(0x59u64), ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x168u64))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_sub(0x51u64), ((rax as u32)) as u32); crate::simd_ops::wu32(rbp.wrapping_sub(0x4du64), ((rcx as u32)) as u32); rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x170u64))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_sub(0x49u64), ((rax as u32)) as u32); xmm0 = ((crate::simd_ops::ru32(rsi.wrapping_add(0x174u64))) as u32) as u128; crate::simd_ops::wu32(rbp.wrapping_sub(0x45u64), ((xmm0 as u32)) as u32); rax = ((crate::simd_ops::ru8(rsi.wrapping_add(0x178u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rbp.wrapping_sub(0x41u64), ((rax as u8)) as u8); rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x2b0u64))) as u64; xmm1 = ((crate::simd_ops::ru32(rax.wrapping_add(0x58u64))) as u32) as u128; rcx = rbp.wrapping_sub(0x61); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r15 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x61u64))) as u64; rbx = (crate::simd_ops::ru32(rbp.wrapping_sub(0x49u64))) as u32 as u64; fl_l = (crate::simd_ops::ru32(rbp.wrapping_sub(0x4du64))) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 20; continue; } bb = 19; continue; }
            19 => { rax = (crate::simd_ops::ru32(r15.wrapping_add(0x1cu64))) as u32 as u64; rax = (((rax as u32)).wrapping_sub(1)) as u32 as u64; fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; rbx = (if (((fl_l as u32)) >= ((fl_r as u32))) { (rax as u32) } else { (rbx as u32) }) as u32 as u64; bb = 20; continue; }
            20 => { fl_l = r15; fl_r = r15; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 23; continue; } bb = 21; continue; }
            21 => { rcx = (crate::simd_ops::ru32(r15.wrapping_add(0x78u64))) as u32 as u64; rax = ((rcx.wrapping_sub(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(r15.wrapping_add(0x78u64), ((rax as u32)) as u32); fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 23; continue; } bb = 22; continue; }
            22 => { rcx = r15; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(P5)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = r15; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x60u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 23; continue; }
            23 => { r15 = (crate::simd_ops::ru64(rbp.wrapping_add(0x67u64))) as u64; bb = 24; continue; }
            24 => { fl_l = (crate::simd_ops::ru64(r14.wrapping_add(0x8u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 35; continue; } bb = 25; continue; }
            25 => { rax = (crate::simd_ops::ru32(r14.wrapping_add(0x18u64))) as u32 as u64; rax = (((rax as u32)).wrapping_sub(1)) as u32 as u64; fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 53; continue; } bb = 26; continue; }
            26 => { rcx = ((rbx as u32)) as u32 as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(r14.wrapping_add(0x50u64)))) as u64; rbx = (((rbx as u32)).wrapping_add(1)) as u32 as u64; rdx = ((rbx as u32)) as u32 as u64; rdx = ((rdx).wrapping_shl(4u32)) as u64; rdx = ((rdx).wrapping_add(crate::simd_ops::ru64(r14.wrapping_add(0x50u64)))) as u64; r8 = (crate::simd_ops::ru32(rcx)) as u32 as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rdx)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 33; continue; } bb = 27; continue; }
            27 => { rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x4u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 33; continue; } bb = 28; continue; }
            28 => { fl_l = ((r8 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rdi.wrapping_add(0x88u64))) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 25; continue; } bb = 29; continue; }
            29 => { fl_l = ((r8 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rdi.wrapping_add(0x90u64))) as u32 as u64; if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { bb = 25; continue; } bb = 30; continue; }
            30 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rdi.wrapping_add(0x8cu64))) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 25; continue; } bb = 31; continue; }
            31 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rdi.wrapping_add(0x94u64))) as u32 as u64; if ((((fl_l as u32)) as i32) <= (((fl_r as u32)) as i32)) { bb = 51; continue; } bb = 32; continue; }
            32 => { bb = 25; continue; }
            33 => { r8 = rbp.wrapping_sub(0x79); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 51; continue; } bb = 34; continue; }
            34 => { bb = 25; continue; }
            35 => { rcx = (crate::simd_ops::ru64(r13.wrapping_add(0x10u64))) as u64; rax = (((crate::simd_ops::ru32(rcx.wrapping_add(0x74u64))) as i32 as i64) as u64) as u64; rdx = (((crate::simd_ops::ru32(rbp.wrapping_sub(0x79u64))) as i32 as i64) as u64) as u64; rdx = (((rdx) as i64).wrapping_mul((rax) as i64) as u64) as u64; r8 = (((crate::simd_ops::ru32(rbp.wrapping_sub(0x75u64))) as i32 as i64) as u64) as u64; r8 = (((r8) as i64).wrapping_mul((rax) as i64) as u64) as u64; r9 = (((crate::simd_ops::ru32(rbp.wrapping_sub(0x71u64))) as i32 as i64) as u64) as u64; r9 = (((r9) as i64).wrapping_mul((rax) as i64) as u64) as u64; r10 = (((crate::simd_ops::ru32(rbp.wrapping_sub(0x6du64))) as i32 as i64) as u64) as u64; r10 = (((r10) as i64).wrapping_mul((rax) as i64) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x39u64), rdx); crate::simd_ops::wu64(rbp.wrapping_sub(0x31u64), r8); crate::simd_ops::wu64(rbp.wrapping_sub(0x29u64), r9); crate::simd_ops::wu64(rbp.wrapping_sub(0x21u64), r10); rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x2b0u64))) as u64; xmm0 = ((crate::simd_ops::ru32(rax.wrapping_add(0x3cu64))) as u32) as u128; fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = ((xmm8 as u32)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 37; continue; } bb = 36; continue; }
            36 => { xmm1 = (xmm6) as u128; bb = 38; continue; }
            37 => { xmm1 = (xmm7) as u128; bb = 38; continue; }
            38 => { xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(crate::simd_ops::ru32(rcx.wrapping_add(0x6cu64)))).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(xmm1 as u32)).to_bits() as u128); rcx = (crate::simd_ops::cvt_f32_i64((f32::from_bits(xmm0 as u32)))) as u64; rdx = ((rdx).wrapping_sub(rcx)) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x39u64), rdx); fl_l = r8; fl_r = rcx; r8 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x31u64), r8); rax = ((rcx.wrapping_add(r9)) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x29u64), rax); rax = ((rcx.wrapping_add(r10)) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x21u64), rax); bb = 39; continue; }
            39 => { rax = (crate::simd_ops::ru32(r14.wrapping_add(0x18u64))) as u32 as u64; rax = (((rax as u32)).wrapping_sub(1)) as u32 as u64; fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 53; continue; } bb = 40; continue; }
            40 => { rcx = ((rbx as u32)) as u32 as u64; r9 = ((rbx as u32)) as u32 as u64; r9 = ((r9).wrapping_shl(4u32)) as u64; r9 = ((r9).wrapping_add(crate::simd_ops::ru64(r14.wrapping_add(0x50u64)))) as u64; rbx = (((rbx as u32)).wrapping_add(1)) as u32 as u64; r10 = ((rbx as u32)) as u32 as u64; r10 = ((r10).wrapping_shl(4u32)) as u64; r10 = ((r10).wrapping_add(crate::simd_ops::ru64(r14.wrapping_add(0x50u64)))) as u64; rax = (crate::simd_ops::ru64(r14.wrapping_add(0x70u64))) as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(rax))) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 43; continue; } bb = 41; continue; }
            41 => { r8 = (crate::simd_ops::ru64(r13.wrapping_add(0x10u64))) as u64; rdx = (((crate::simd_ops::ru32(r8.wrapping_add(0x74u64))) as i32 as i64) as u64) as u64; rcx = (((crate::simd_ops::ru32(r9)) as i32 as i64) as u64) as u64; rcx = (((rcx) as i64).wrapping_mul((rdx) as i64) as u64) as u64; rax = (((crate::simd_ops::ru32(r9.wrapping_add(0x8u64))) as i32 as i64) as u64) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x9u64), rcx); rcx = (((crate::simd_ops::ru32(r9.wrapping_add(0x4u64))) as i32 as i64) as u64) as u64; rcx = (((rcx) as i64).wrapping_mul((rdx) as i64) as u64) as u64; rax = (((crate::simd_ops::ru32(r9.wrapping_add(0xcu64))) as i32 as i64) as u64) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x1u64), rcx); rdx = (((crate::simd_ops::ru32(r8.wrapping_add(0x74u64))) as i32 as i64) as u64) as u64; rcx = (((crate::simd_ops::ru32(r10)) as i32 as i64) as u64) as u64; rcx = (((rcx) as i64).wrapping_mul((rdx) as i64) as u64) as u64; rax = (((crate::simd_ops::ru32(r10.wrapping_add(0x8u64))) as i32 as i64) as u64) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x19u64), rcx); rcx = (((crate::simd_ops::ru32(r10.wrapping_add(0x4u64))) as i32 as i64) as u64) as u64; rcx = (((rcx) as i64).wrapping_mul((rdx) as i64) as u64) as u64; rax = (((crate::simd_ops::ru32(r10.wrapping_add(0xcu64))) as i32 as i64) as u64) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x11u64), rcx); r8 = rbp.wrapping_sub(0x39); rdx = rbp.wrapping_sub(0x19); rcx = rbp.wrapping_sub(0x9); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 51; continue; } bb = 42; continue; }
            42 => { bb = 39; continue; }
            43 => { rax = (crate::simd_ops::ru32(r9)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(r10)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 50; continue; } bb = 44; continue; }
            44 => { rcx = (crate::simd_ops::ru32(r9.wrapping_add(0x4u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(r10.wrapping_add(0x4u64))) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 50; continue; } bb = 45; continue; }
            45 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rdi.wrapping_add(0x88u64))) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 39; continue; } bb = 46; continue; }
            46 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rdi.wrapping_add(0x90u64))) as u32 as u64; if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { bb = 39; continue; } bb = 47; continue; }
            47 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rdi.wrapping_add(0x8cu64))) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 39; continue; } bb = 48; continue; }
            48 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rdi.wrapping_add(0x94u64))) as u32 as u64; if ((((fl_l as u32)) as i32) <= (((fl_r as u32)) as i32)) { bb = 51; continue; } bb = 49; continue; }
            49 => { bb = 39; continue; }
            50 => { r8 = rbp.wrapping_sub(0x79); rdx = r10; rcx = r9; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 39; continue; } bb = 51; continue; }
            51 => { fl_l = (crate::simd_ops::ru64(rsi.wrapping_add(0x70u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 53; continue; } bb = 52; continue; }
            52 => { crate::simd_ops::wu8(rsi.wrapping_add(0x120u64), (0x1u8) as u8); crate::simd_ops::wu32(rsi.wrapping_add(0x118u64), (0x2u32) as u32); bb = 53; continue; }
            53 => { r12 = ((r12).wrapping_add(0x8u64)) as u64; fl_l = r15; fl_r = (0x1u64) as u64; r15 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x67u64), r15); if (((fl_l as u64)) != ((fl_r as u64))) { bb = 8; continue; } bb = 54; continue; }
            54 => { r11 = rsp.wrapping_add(0xd0); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x40u64))) as u64; rdi = (crate::simd_ops::ru64(r11.wrapping_add(0x48u64))) as u64; xmm6 = (crate::simd_ops::ru128(r11.wrapping_sub(0x10u64))) as u128; xmm7 = (crate::simd_ops::ru128(r11.wrapping_sub(0x20u64))) as u128; xmm8 = (crate::simd_ops::ru128(r11.wrapping_sub(0x30u64))) as u128; rsp = r11; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141724cb0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x14169f640; const P2: u64 = 0x10; const P3: u64 = 0x1c;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = 0; fl_l = 0; fl_r = 0; r14 = rdx; fl_l = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 8; continue; } bb = 1; continue; }
            1 => { bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(r14)) as u64; rbx = 0; fl_l = 0; fl_r = 0; rdi = (crate::simd_ops::ru64(rax.wrapping_add(rsi.wrapping_mul(8)))) as u64; bb = 3; continue; }
            3 => { rdx = rsp.wrapping_add(0x48); rcx = rdi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rax)) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 4; continue; }
            4 => { fl_l = ((rbx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rcx.wrapping_add(P2))) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 7; continue; } bb = 5; continue; }
            5 => { rdx = rsp.wrapping_add(0x50); rcx = rdi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rax)) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(rbx.wrapping_mul(8)))) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(P3))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 9; continue; } bb = 6; continue; }
            6 => { fl_l = (((rbx as u32)).wrapping_add(1)) as u32 as u64; rbx = ((fl_l as u32)) as u32 as u64; bb = 3; continue; }
            7 => { rsi = (((rsi as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((rsi as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(r14.wrapping_add(P0))) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 2; continue; } bb = 8; continue; }
            8 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            9 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141724d50(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x4; const P2: u64 = 0x10;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64;
    crate::simd_ops::wu32(rcx, ((rax as u32)) as u32);
    xmm0 = ((crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32) as u128;
    xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(crate::simd_ops::ru32(rdx.wrapping_add(P0)))).to_bits() as u128);
    crate::simd_ops::wu8(rcx.wrapping_add(P2), (0x0u8) as u8);
    crate::simd_ops::wu32(rcx.wrapping_add(P1), ((xmm0 as u32)) as u32);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141724d70(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x4; const P1: u64 = 0x8; const P2: u64 = 0xc; const P3: u64 = 0x10; const P4: u64 = 0x14; const P5: u64 = 0x18; const P6: u64 = 0x1c; const P7: u64 = 0x24; const P8: u64 = 0x28; const P9: u64 = 0x2c; const P10: u64 = 0x30; const P11: u64 = 0x34;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu32(rcx, (0x1u32) as u32); rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rcx.wrapping_add(P0), (0x1u32) as u32); crate::simd_ops::wu32(rcx.wrapping_add(P1), (0x1u32) as u32); crate::simd_ops::wu32(rcx.wrapping_add(P2), (0x1u32) as u32); crate::simd_ops::wu32(rcx.wrapping_add(P3), (0x7fffffffu32) as u32); crate::simd_ops::wu32(rcx.wrapping_add(P4), (0x7fffffffu32) as u32); crate::simd_ops::wu32(rcx.wrapping_add(P5), (0x80000000u32) as u32); crate::simd_ops::wu64(rcx.wrapping_add(P6), (0xffffffff80000000u64) as u64); crate::simd_ops::wu32(rcx.wrapping_add(P7), (0xffffffffu32) as u32); crate::simd_ops::wu32(rcx.wrapping_add(P8), (0x7fffffffu32) as u32); crate::simd_ops::wu32(rcx.wrapping_add(P9), (0x7fffffffu32) as u32); crate::simd_ops::wu32(rcx.wrapping_add(P10), (0x80000000u32) as u32); crate::simd_ops::wu64(rcx.wrapping_add(P11), (0xffffffff80000000u64) as u64); crate::simd_ops::wu32(rcx.wrapping_add(0x3cu64), (0xffffffffu32) as u32); crate::simd_ops::wu32(rcx.wrapping_add(0x40u64), (0x7fffffffu32) as u32); crate::simd_ops::wu32(rcx.wrapping_add(0x44u64), (0x7fffffffu32) as u32); crate::simd_ops::wu32(rcx.wrapping_add(0x48u64), (0x80000000u32) as u32); crate::simd_ops::wu64(rcx.wrapping_add(0x4cu64), (0xffffffff80000000u64) as u64); crate::simd_ops::wu32(rcx.wrapping_add(0x54u64), (0xffffffffu32) as u32); crate::simd_ops::wu32(rcx.wrapping_add(0x58u64), (0x7fffffffu32) as u32); crate::simd_ops::wu32(rcx.wrapping_add(0x5cu64), (0x7fffffffu32) as u32); crate::simd_ops::wu32(rcx.wrapping_add(0x60u64), (0x80000000u32) as u32); crate::simd_ops::wu64(rcx.wrapping_add(0x64u64), (0xffffffff80000000u64) as u64); crate::simd_ops::wu32(rcx.wrapping_add(0x6cu64), (0xffffffffu32) as u32); crate::simd_ops::wu32(rcx.wrapping_add(0x70u64), (0x7fffffffu32) as u32); crate::simd_ops::wu32(rcx.wrapping_add(0x74u64), (0x7fffffffu32) as u32); crate::simd_ops::wu32(rcx.wrapping_add(0x78u64), (0x80000000u32) as u32); crate::simd_ops::wu64(rcx.wrapping_add(0x7cu64), (0xffffffff80000000u64) as u64); crate::simd_ops::wu32(rcx.wrapping_add(0x84u64), (0xffffffffu32) as u32); crate::simd_ops::wu32(rcx.wrapping_add(0x88u64), (0x7fffffffu32) as u32); crate::simd_ops::wu32(rcx.wrapping_add(0x8cu64), (0x7fffffffu32) as u32); crate::simd_ops::wu32(rcx.wrapping_add(0x90u64), (0x80000000u32) as u32); crate::simd_ops::wu64(rcx.wrapping_add(0x94u64), (0xffffffff80000000u64) as u64); crate::simd_ops::wu32(rcx.wrapping_add(0x9cu64), (0xffffffffu32) as u32); crate::simd_ops::wu64(rcx.wrapping_add(0xa8u64), rax); crate::simd_ops::wu64(rcx.wrapping_add(0xb0u64), rax); crate::simd_ops::wu64(rcx.wrapping_add(0xb8u64), rax); crate::simd_ops::wu64(rcx.wrapping_add(0xc0u64), rax); crate::simd_ops::wu64(rcx.wrapping_add(0xc8u64), rax); crate::simd_ops::wu64(rcx.wrapping_add(0xd0u64), rax); crate::simd_ops::wu32(rcx.wrapping_add(0xa0u64), ((rax as u32)) as u32); crate::simd_ops::wu16(rcx.wrapping_add(0xa4u64), ((rax as u16)) as u16); rax = rcx; return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141724ec0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x4; const P1: u64 = 0x8; const P2: u64 = 0xc;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    crate::simd_ops::wu32(rcx, (0x1u32) as u32);
    rax = rcx;
    crate::simd_ops::wu32(rcx.wrapping_add(P0), (0x1u32) as u32);
    crate::simd_ops::wu32(rcx.wrapping_add(P1), (0x1u32) as u32);
    crate::simd_ops::wu32(rcx.wrapping_add(P2), (0x1u32) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141724ee0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1416bb650; const P1: u64 = 0xa8;
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    rcx = ((rcx).wrapping_add(0xc0u64)) as u64;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rcx = ((rbx.wrapping_add(P1)) as u64) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(P0, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141725050(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141725850; const P1: u64 = 0x14169eda0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; xmm0 = (crate::simd_ops::ru128(rcx)) as u128; rbx = rdx; r14 = rcx; crate::simd_ops::wu128(rdx, (xmm0) as u128); r9 = (crate::simd_ops::ru32(rdx.wrapping_add(0x10u64))) as u32 as u64; fl_l = ((r9 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rcx.wrapping_add(0x10u64))) as u32 as u64; r9 = (if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { crate::simd_ops::ru32(rcx.wrapping_add(0x10u64)) } else { (r9 as u32) }) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_add(0x10u64), ((r9 as u32)) as u32); r8 = (crate::simd_ops::ru32(rcx.wrapping_add(0x18u64))) as u32 as u64; fl_l = (crate::simd_ops::ru32(rdx.wrapping_add(0x18u64))) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; r8 = (if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { crate::simd_ops::ru32(rdx.wrapping_add(0x18u64)) } else { (r8 as u32) }) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_add(0x18u64), ((r8 as u32)) as u32); rdx = (crate::simd_ops::ru32(rdx.wrapping_add(0x14u64))) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rcx.wrapping_add(0x14u64))) as u32 as u64; rdx = (if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { crate::simd_ops::ru32(rcx.wrapping_add(0x14u64)) } else { (rdx as u32) }) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x14u64), ((rdx as u32)) as u32); rcx = (crate::simd_ops::ru32(rcx.wrapping_add(0x1cu64))) as u32 as u64; fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x1cu64))) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; rcx = (if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { crate::simd_ops::ru32(rbx.wrapping_add(0x1cu64)) } else { (rcx as u32) }) as u32 as u64; r8 = (((r8 as u32)).wrapping_sub((r9 as u32))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x1cu64), ((rcx as u32)) as u32); r8 = (((r8 as u32)).wrapping_add(1)) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x20u64), ((r8 as u32)) as u32); rcx = (((rcx as u32)).wrapping_sub((rdx as u32))) as u32 as u64; rcx = (((rcx as u32)).wrapping_add(1)) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x24u64), ((rcx as u32)) as u32); r9 = (crate::simd_ops::ru32(rbx.wrapping_add(0x28u64))) as u32 as u64; fl_l = ((r9 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(r14.wrapping_add(0x28u64))) as u32 as u64; r9 = (if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { crate::simd_ops::ru32(r14.wrapping_add(0x28u64)) } else { (r9 as u32) }) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x28u64), ((r9 as u32)) as u32); r8 = (crate::simd_ops::ru32(r14.wrapping_add(0x30u64))) as u32 as u64; fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x30u64))) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; r8 = (if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { crate::simd_ops::ru32(rbx.wrapping_add(0x30u64)) } else { (r8 as u32) }) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x30u64), ((r8 as u32)) as u32); rdx = (crate::simd_ops::ru32(rbx.wrapping_add(0x2cu64))) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(r14.wrapping_add(0x2cu64))) as u32 as u64; rdx = (if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { crate::simd_ops::ru32(r14.wrapping_add(0x2cu64)) } else { (rdx as u32) }) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x2cu64), ((rdx as u32)) as u32); rcx = (crate::simd_ops::ru32(r14.wrapping_add(0x34u64))) as u32 as u64; fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x34u64))) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; rcx = (if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { crate::simd_ops::ru32(rbx.wrapping_add(0x34u64)) } else { (rcx as u32) }) as u32 as u64; r8 = (((r8 as u32)).wrapping_sub((r9 as u32))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x34u64), ((rcx as u32)) as u32); r8 = (((r8 as u32)).wrapping_add(1)) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x38u64), ((r8 as u32)) as u32); rcx = (((rcx as u32)).wrapping_sub((rdx as u32))) as u32 as u64; rcx = (((rcx as u32)).wrapping_add(1)) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x3cu64), ((rcx as u32)) as u32); r9 = (crate::simd_ops::ru32(rbx.wrapping_add(0x40u64))) as u32 as u64; fl_l = ((r9 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(r14.wrapping_add(0x40u64))) as u32 as u64; r9 = (if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { crate::simd_ops::ru32(r14.wrapping_add(0x40u64)) } else { (r9 as u32) }) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x40u64), ((r9 as u32)) as u32); r8 = (crate::simd_ops::ru32(r14.wrapping_add(0x48u64))) as u32 as u64; fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x48u64))) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; r8 = (if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { crate::simd_ops::ru32(rbx.wrapping_add(0x48u64)) } else { (r8 as u32) }) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x48u64), ((r8 as u32)) as u32); rdx = (crate::simd_ops::ru32(rbx.wrapping_add(0x44u64))) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(r14.wrapping_add(0x44u64))) as u32 as u64; rdx = (if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { crate::simd_ops::ru32(r14.wrapping_add(0x44u64)) } else { (rdx as u32) }) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x44u64), ((rdx as u32)) as u32); rcx = (crate::simd_ops::ru32(r14.wrapping_add(0x4cu64))) as u32 as u64; fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x4cu64))) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; rcx = (if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { crate::simd_ops::ru32(rbx.wrapping_add(0x4cu64)) } else { (rcx as u32) }) as u32 as u64; r8 = (((r8 as u32)).wrapping_sub((r9 as u32))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x4cu64), ((rcx as u32)) as u32); r8 = (((r8 as u32)).wrapping_add(1)) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x50u64), ((r8 as u32)) as u32); rcx = (((rcx as u32)).wrapping_sub((rdx as u32))) as u32 as u64; rcx = (((rcx as u32)).wrapping_add(1)) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x54u64), ((rcx as u32)) as u32); r9 = (crate::simd_ops::ru32(rbx.wrapping_add(0x58u64))) as u32 as u64; fl_l = ((r9 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(r14.wrapping_add(0x58u64))) as u32 as u64; r9 = (if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { crate::simd_ops::ru32(r14.wrapping_add(0x58u64)) } else { (r9 as u32) }) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x58u64), ((r9 as u32)) as u32); r8 = (crate::simd_ops::ru32(r14.wrapping_add(0x60u64))) as u32 as u64; fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x60u64))) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; r8 = (if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { crate::simd_ops::ru32(rbx.wrapping_add(0x60u64)) } else { (r8 as u32) }) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x60u64), ((r8 as u32)) as u32); rdx = (crate::simd_ops::ru32(rbx.wrapping_add(0x5cu64))) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(r14.wrapping_add(0x5cu64))) as u32 as u64; rdx = (if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { crate::simd_ops::ru32(r14.wrapping_add(0x5cu64)) } else { (rdx as u32) }) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x5cu64), ((rdx as u32)) as u32); rcx = (crate::simd_ops::ru32(r14.wrapping_add(0x64u64))) as u32 as u64; fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x64u64))) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; rcx = (if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { crate::simd_ops::ru32(rbx.wrapping_add(0x64u64)) } else { (rcx as u32) }) as u32 as u64; r8 = (((r8 as u32)).wrapping_sub((r9 as u32))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x64u64), ((rcx as u32)) as u32); r8 = (((r8 as u32)).wrapping_add(1)) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x68u64), ((r8 as u32)) as u32); rcx = (((rcx as u32)).wrapping_sub((rdx as u32))) as u32 as u64; rcx = (((rcx as u32)).wrapping_add(1)) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x6cu64), ((rcx as u32)) as u32); r9 = (crate::simd_ops::ru32(rbx.wrapping_add(0x70u64))) as u32 as u64; fl_l = ((r9 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(r14.wrapping_add(0x70u64))) as u32 as u64; r9 = (if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { crate::simd_ops::ru32(r14.wrapping_add(0x70u64)) } else { (r9 as u32) }) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x70u64), ((r9 as u32)) as u32); r8 = (crate::simd_ops::ru32(r14.wrapping_add(0x78u64))) as u32 as u64; fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x78u64))) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; r8 = (if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { crate::simd_ops::ru32(rbx.wrapping_add(0x78u64)) } else { (r8 as u32) }) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x78u64), ((r8 as u32)) as u32); rdx = (crate::simd_ops::ru32(rbx.wrapping_add(0x74u64))) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(r14.wrapping_add(0x74u64))) as u32 as u64; rdx = (if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { crate::simd_ops::ru32(r14.wrapping_add(0x74u64)) } else { (rdx as u32) }) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x74u64), ((rdx as u32)) as u32); rcx = (crate::simd_ops::ru32(r14.wrapping_add(0x7cu64))) as u32 as u64; fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x7cu64))) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; rcx = (if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { crate::simd_ops::ru32(rbx.wrapping_add(0x7cu64)) } else { (rcx as u32) }) as u32 as u64; r8 = (((r8 as u32)).wrapping_sub((r9 as u32))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x7cu64), ((rcx as u32)) as u32); r8 = (((r8 as u32)).wrapping_add(1)) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x80u64), ((r8 as u32)) as u32); rcx = (((rcx as u32)).wrapping_sub((rdx as u32))) as u32 as u64; rcx = (((rcx as u32)).wrapping_add(1)) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x84u64), ((rcx as u32)) as u32); rax = (crate::simd_ops::ru32(r14.wrapping_add(0x88u64))) as u32 as u64; r9 = (crate::simd_ops::ru32(rbx.wrapping_add(0x88u64))) as u32 as u64; fl_l = ((r9 as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; r9 = (if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { (rax as u32) } else { (r9 as u32) }) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x88u64), ((r9 as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x90u64))) as u32 as u64; r8 = (crate::simd_ops::ru32(r14.wrapping_add(0x90u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; r8 = (if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { (rax as u32) } else { (r8 as u32) }) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x90u64), ((r8 as u32)) as u32); rax = (crate::simd_ops::ru32(r14.wrapping_add(0x8cu64))) as u32 as u64; rdx = (crate::simd_ops::ru32(rbx.wrapping_add(0x8cu64))) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; rdx = (if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { (rax as u32) } else { (rdx as u32) }) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x8cu64), ((rdx as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x94u64))) as u32 as u64; rcx = (crate::simd_ops::ru32(r14.wrapping_add(0x94u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; rcx = (if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { (rax as u32) } else { (rcx as u32) }) as u32 as u64; r8 = (((r8 as u32)).wrapping_sub((r9 as u32))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x94u64), ((rcx as u32)) as u32); r8 = (((r8 as u32)).wrapping_add(1)) as u32 as u64; rcx = (((rcx as u32)).wrapping_sub((rdx as u32))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x98u64), ((r8 as u32)) as u32); rcx = (((rcx as u32)).wrapping_add(1)) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x9cu64), ((rcx as u32)) as u32); fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0xa0u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 3; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru8(r14.wrapping_add(0xa0u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 4; continue; }
            3 => { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0xa1u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; crate::simd_ops::wu8(rbx.wrapping_add(0xa0u64), ((rax as u8)) as u8); if (((fl_l as u8)) != ((fl_r as u8))) { bb = 7; continue; } bb = 5; continue; }
            5 => { fl_l = (crate::simd_ops::ru8(r14.wrapping_add(0xa1u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 8; continue; }
            7 => { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 8; continue; }
            8 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0xa2u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; crate::simd_ops::wu8(rbx.wrapping_add(0xa1u64), ((rax as u8)) as u8); if (((fl_l as u8)) != ((fl_r as u8))) { bb = 11; continue; } bb = 9; continue; }
            9 => { fl_l = (crate::simd_ops::ru8(r14.wrapping_add(0xa2u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 11; continue; } bb = 10; continue; }
            10 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 12; continue; }
            11 => { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 12; continue; }
            12 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0xa3u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; crate::simd_ops::wu8(rbx.wrapping_add(0xa2u64), ((rax as u8)) as u8); if (((fl_l as u8)) != ((fl_r as u8))) { bb = 15; continue; } bb = 13; continue; }
            13 => { fl_l = (crate::simd_ops::ru8(r14.wrapping_add(0xa3u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 15; continue; } bb = 14; continue; }
            14 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 16; continue; }
            15 => { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 16; continue; }
            16 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0xa4u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; crate::simd_ops::wu8(rbx.wrapping_add(0xa3u64), ((rax as u8)) as u8); if (((fl_l as u8)) != ((fl_r as u8))) { bb = 19; continue; } bb = 17; continue; }
            17 => { fl_l = (crate::simd_ops::ru8(r14.wrapping_add(0xa4u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 19; continue; } bb = 18; continue; }
            18 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 20; continue; }
            19 => { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 20; continue; }
            20 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0xa5u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; crate::simd_ops::wu8(rbx.wrapping_add(0xa4u64), ((rax as u8)) as u8); if (((fl_l as u8)) != ((fl_r as u8))) { bb = 23; continue; } bb = 21; continue; }
            21 => { fl_l = (crate::simd_ops::ru8(r14.wrapping_add(0xa5u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 23; continue; } bb = 22; continue; }
            22 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 24; continue; }
            23 => { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 24; continue; }
            24 => { rsi = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rdi); rbp = ((rsi as u32)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), r15); crate::simd_ops::wu8(rbx.wrapping_add(0xa5u64), ((rax as u8)) as u8); fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0xb0u64))) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 30; continue; } bb = 25; continue; }
            25 => { crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), r12); bb = 26; continue; }
            26 => { rax = (crate::simd_ops::ru64(r14.wrapping_add(0xa8u64))) as u64; rdx = rbx.wrapping_add(0xa8); r8 = (crate::simd_ops::ru64(rbx.wrapping_add(0xb0u64))) as u64; rcx = ((rbp as u32)) as u32 as u64; r8 = ((r8).wrapping_add(1)) as u64; r15 = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; rcx = rbx.wrapping_add(0xa8); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0xa8u64))) as u64; rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0xb0u64))) as u64; r12 = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; rax = (crate::simd_ops::ru64(r15)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 28; continue; } bb = 27; continue; }
            27 => { rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(r15)) as u64; bb = 28; continue; }
            28 => { rbp = (((rbp as u32)).wrapping_add(1)) as u32 as u64; crate::simd_ops::wu64(r12.wrapping_sub(0x8u64), rax); fl_l = ((rbp as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rbx.wrapping_add(0xb0u64))) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 26; continue; } bb = 29; continue; }
            29 => { r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; bb = 30; continue; }
            30 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0xc8u64))) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 35; continue; } bb = 31; continue; }
            31 => { bb = 32; continue; }
            32 => { rax = (crate::simd_ops::ru64(r14.wrapping_add(0xc0u64))) as u64; rdx = rbx.wrapping_add(0xc0); r8 = (crate::simd_ops::ru64(rbx.wrapping_add(0xc8u64))) as u64; rcx = rbx.wrapping_add(0xc0); r8 = ((r8).wrapping_add(1)) as u64; r15 = ((rax.wrapping_add(rsi.wrapping_mul(8))) as u64) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0xc0u64))) as u64; rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0xc8u64))) as u64; rbp = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; rax = (crate::simd_ops::ru64(r15)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 34; continue; } bb = 33; continue; }
            33 => { rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(r15)) as u64; bb = 34; continue; }
            34 => { rsi = (((rsi as u32)).wrapping_add(1)) as u32 as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x8u64), rax); fl_l = ((rsi as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rbx.wrapping_add(0xc8u64))) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 32; continue; } bb = 35; continue; }
            35 => { r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141725440(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x4; const P2: u64 = 0xc; const P3: u64 = 0x1;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); r10 = (crate::simd_ops::ru32(rdx)) as u32 as u64; r11 = rdx; fl_l = ((r10 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rcx)) as u32 as u64; rbx = r9; r10 = (if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { crate::simd_ops::ru32(rcx) } else { (r10 as u32) }) as u32 as u64; crate::simd_ops::wu32(rdx, ((r10 as u32)) as u32); r9 = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; fl_l = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; r9 = (if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { crate::simd_ops::ru32(rdx.wrapping_add(P0)) } else { (r9 as u32) }) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_add(P0), ((r9 as u32)) as u32); rdx = (crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64; rdx = (if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { crate::simd_ops::ru32(rcx.wrapping_add(P1)) } else { (rdx as u32) }) as u32 as u64; crate::simd_ops::wu32(r11.wrapping_add(0x4u64), ((rdx as u32)) as u32); rcx = (crate::simd_ops::ru32(rcx.wrapping_add(P2))) as u32 as u64; fl_l = (crate::simd_ops::ru32(r11.wrapping_add(0xcu64))) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; rcx = (if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { crate::simd_ops::ru32(r11.wrapping_add(0xcu64)) } else { (rcx as u32) }) as u32 as u64; r9 = (((r9 as u32)).wrapping_sub((r10 as u32))) as u32 as u64; crate::simd_ops::wu32(r11.wrapping_add(0xcu64), ((rcx as u32)) as u32); r9 = (((r9 as u32)).wrapping_add(1)) as u32 as u64; rcx = (((rcx as u32)).wrapping_sub((rdx as u32))) as u32 as u64; crate::simd_ops::wu32(r11.wrapping_add(0x10u64), ((r9 as u32)) as u32); rcx = (((rcx as u32)).wrapping_add(1)) as u32 as u64; crate::simd_ops::wu32(r11.wrapping_add(0x14u64), ((rcx as u32)) as u32); fl_l = (crate::simd_ops::ru8(r8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; rdx = 0x1; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0xffffffff; rcx = ((rax.wrapping_add(P3)) as u32) as u32 as u64; rcx = (if (((fl_l as u32)) == ((fl_r as u32))) { (rdx as u32) } else { (rcx as u32) }) as u32 as u64; crate::simd_ops::wu32(rbx, ((rcx as u32)) as u32); crate::simd_ops::wu8(r8, ((rdx as u8)) as u8); bb = 2; continue; }
            2 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x8u64))) as u64; return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1417254c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x14; const P2: u64 = 0x18; const P3: u64 = 0x1c; const P4: u64 = 0x24; const P5: u64 = 0x28; const P6: u64 = 0x2c; const P7: u64 = 0x30; const P8: u64 = 0x34; const P9: u64 = 0x3c; const P10: u64 = 0x40; const P11: u64 = 0x44; const P12: u64 = 0x141725850;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rcx; crate::simd_ops::wu32(rcx.wrapping_add(P0), (0x7fffffffu32) as u32); crate::simd_ops::wu32(rcx.wrapping_add(P1), (0x7fffffffu32) as u32); r8 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rcx.wrapping_add(P2), (0x80000000u32) as u32); crate::simd_ops::wu64(rcx.wrapping_add(P3), (0xffffffff80000000u64) as u64); crate::simd_ops::wu32(rcx.wrapping_add(P4), (0xffffffffu32) as u32); crate::simd_ops::wu32(rcx.wrapping_add(P5), (0x7fffffffu32) as u32); crate::simd_ops::wu32(rcx.wrapping_add(P6), (0x7fffffffu32) as u32); crate::simd_ops::wu32(rcx.wrapping_add(P7), (0x80000000u32) as u32); crate::simd_ops::wu64(rcx.wrapping_add(P8), (0xffffffff80000000u64) as u64); crate::simd_ops::wu32(rcx.wrapping_add(P9), (0xffffffffu32) as u32); crate::simd_ops::wu32(rcx.wrapping_add(P10), (0x7fffffffu32) as u32); crate::simd_ops::wu32(rcx.wrapping_add(P11), (0x7fffffffu32) as u32); crate::simd_ops::wu32(rcx.wrapping_add(0x48u64), (0x80000000u32) as u32); crate::simd_ops::wu64(rcx.wrapping_add(0x4cu64), (0xffffffff80000000u64) as u64); crate::simd_ops::wu32(rcx.wrapping_add(0x54u64), (0xffffffffu32) as u32); crate::simd_ops::wu32(rcx.wrapping_add(0x58u64), (0x7fffffffu32) as u32); crate::simd_ops::wu32(rcx.wrapping_add(0x5cu64), (0x7fffffffu32) as u32); crate::simd_ops::wu32(rcx.wrapping_add(0x60u64), (0x80000000u32) as u32); crate::simd_ops::wu64(rcx.wrapping_add(0x64u64), (0xffffffff80000000u64) as u64); crate::simd_ops::wu32(rcx.wrapping_add(0x6cu64), (0xffffffffu32) as u32); crate::simd_ops::wu32(rcx.wrapping_add(0x70u64), (0x7fffffffu32) as u32); crate::simd_ops::wu32(rcx.wrapping_add(0x74u64), (0x7fffffffu32) as u32); crate::simd_ops::wu32(rcx.wrapping_add(0x78u64), (0x80000000u32) as u32); crate::simd_ops::wu64(rcx.wrapping_add(0x7cu64), (0xffffffff80000000u64) as u64); crate::simd_ops::wu32(rcx.wrapping_add(0x84u64), (0xffffffffu32) as u32); crate::simd_ops::wu32(rcx.wrapping_add(0x88u64), (0x7fffffffu32) as u32); crate::simd_ops::wu32(rcx.wrapping_add(0x8cu64), (0x7fffffffu32) as u32); crate::simd_ops::wu32(rcx.wrapping_add(0x90u64), (0x80000000u32) as u32); crate::simd_ops::wu64(rcx.wrapping_add(0x94u64), (0xffffffff80000000u64) as u64); crate::simd_ops::wu32(rcx.wrapping_add(0x9cu64), (0xffffffffu32) as u32); crate::simd_ops::wu32(rcx.wrapping_add(0xa0u64), (0x0u32) as u32); crate::simd_ops::wu16(rcx.wrapping_add(0xa4u64), (0x0u16) as u16); rcx = ((rcx).wrapping_add(0xa8u64)) as u64; rdx = rcx; rax = crate::simd_ops::call_stk(P12, rax, rcx, rdx, r8, r9, rsp); rcx = rbx.wrapping_add(0xc0); r8 = 0; fl_l = 0; fl_r = 0; rdx = rcx; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P12, rax, rcx, rdx, r8, r9, rsp); }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1417255f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x40; const P1: u64 = 0x50; const P2: u64 = 0x88; const P3: u64 = 0x98; const P4: u64 = 0x58; const P5: u64 = 0x90; const P6: u64 = 0x60; const P7: u64 = 0x8c; const P8: u64 = 0x5c; const P9: u64 = 0x94; const P10: u64 = 0x64; const P11: u64 = 0x9c;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { xmm0 = (crate::simd_ops::ru128(rcx.wrapping_add(P0))) as u128; r10 = rcx; xmm1 = ((crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64) as u128; crate::simd_ops::wu128(rcx.wrapping_add(P2), (xmm0) as u128); crate::simd_ops::wu64(rcx.wrapping_add(P3), ((xmm1 as u64)) as u64); r9 = (crate::simd_ops::ru32(rcx.wrapping_add(P2))) as u32 as u64; fl_l = ((r9 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rcx.wrapping_add(P4))) as u32 as u64; r9 = (if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { crate::simd_ops::ru32(rcx.wrapping_add(P4)) } else { (r9 as u32) }) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(P2), ((r9 as u32)) as u32); rax = (crate::simd_ops::ru32(rcx.wrapping_add(P5))) as u32 as u64; r8 = (crate::simd_ops::ru32(rcx.wrapping_add(P6))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; r8 = (if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { (rax as u32) } else { (r8 as u32) }) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(P5), ((r8 as u32)) as u32); rdx = (crate::simd_ops::ru32(rcx.wrapping_add(P7))) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rcx.wrapping_add(P8))) as u32 as u64; rdx = (if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { crate::simd_ops::ru32(rcx.wrapping_add(P8)) } else { (rdx as u32) }) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(P7), ((rdx as u32)) as u32); rax = (crate::simd_ops::ru32(rcx.wrapping_add(P9))) as u32 as u64; rcx = (crate::simd_ops::ru32(rcx.wrapping_add(P10))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; rcx = (if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { (rax as u32) } else { (rcx as u32) }) as u32 as u64; r8 = (((r8 as u32)).wrapping_sub((r9 as u32))) as u32 as u64; crate::simd_ops::wu32(r10.wrapping_add(P9), ((rcx as u32)) as u32); r8 = (((r8 as u32)).wrapping_add(1)) as u32 as u64; rcx = (((rcx as u32)).wrapping_sub((rdx as u32))) as u32 as u64; crate::simd_ops::wu32(r10.wrapping_add(P3), ((r8 as u32)) as u32); rcx = (((rcx as u32)).wrapping_add(1)) as u32 as u64; crate::simd_ops::wu32(r10.wrapping_add(P11), ((rcx as u32)) as u32); r11 = (crate::simd_ops::ru32(r10.wrapping_add(0x70u64))) as u32 as u64; fl_l = ((r9 as u32)) as u32 as u64; fl_r = ((r11 as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { bb = 2; continue; } bb = 1; continue; }
            1 => { r11 = ((r9 as u32)) as u32 as u64; bb = 2; continue; }
            2 => { crate::simd_ops::wu32(r10.wrapping_add(0x88u64), ((r11 as u32)) as u32); rax = (crate::simd_ops::ru32(r10.wrapping_add(0x90u64))) as u32 as u64; r8 = (crate::simd_ops::ru32(r10.wrapping_add(0x78u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; r8 = (if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { (rax as u32) } else { (r8 as u32) }) as u32 as u64; crate::simd_ops::wu32(r10.wrapping_add(0x90u64), ((r8 as u32)) as u32); fl_l = ((rdx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(r10.wrapping_add(0x74u64))) as u32 as u64; rdx = (if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { crate::simd_ops::ru32(r10.wrapping_add(0x74u64)) } else { (rdx as u32) }) as u32 as u64; crate::simd_ops::wu32(r10.wrapping_add(0x8cu64), ((rdx as u32)) as u32); rax = (crate::simd_ops::ru32(r10.wrapping_add(0x94u64))) as u32 as u64; rcx = (crate::simd_ops::ru32(r10.wrapping_add(0x7cu64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; rcx = (if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { (rax as u32) } else { (rcx as u32) }) as u32 as u64; r8 = (((r8 as u32)).wrapping_sub((r11 as u32))) as u32 as u64; crate::simd_ops::wu32(r10.wrapping_add(0x94u64), ((rcx as u32)) as u32); r8 = (((r8 as u32)).wrapping_add(1)) as u32 as u64; rcx = (((rcx as u32)).wrapping_sub((rdx as u32))) as u32 as u64; crate::simd_ops::wu32(r10.wrapping_add(0x98u64), ((r8 as u32)) as u32); fl_l = (((rcx as u32)).wrapping_add(1)) as u32 as u64; rcx = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(r10.wrapping_add(0x9cu64), ((rcx as u32)) as u32); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141725990(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x14; const P2: u64 = 0x30; const P3: u64 = 0x38; const P4: u64 = 0x40; const P5: u64 = 0x48; const P6: u64 = 0x50; const P7: u64 = 0x58; const P8: u64 = 0x10; const P9: u64 = 0x18; const P10: u64 = 0x1c; const P11: u64 = 0x20;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    crate::simd_ops::wu64(rcx, rdx);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rcx.wrapping_add(P0), rdx);
    crate::simd_ops::wu32(rcx.wrapping_add(P1), (0x7fffffffu32) as u32);
    crate::simd_ops::wu64(rcx.wrapping_add(P2), rax);
    crate::simd_ops::wu64(rcx.wrapping_add(P3), rax);
    crate::simd_ops::wu64(rcx.wrapping_add(P4), rax);
    crate::simd_ops::wu64(rcx.wrapping_add(P5), rax);
    crate::simd_ops::wu64(rcx.wrapping_add(P6), rax);
    crate::simd_ops::wu64(rcx.wrapping_add(P7), rax);
    crate::simd_ops::wu32(rcx.wrapping_add(P8), ((rax as u32)) as u32);
    rax = rcx;
    crate::simd_ops::wu32(rcx.wrapping_add(P1), (0x7fffffffu32) as u32);
    crate::simd_ops::wu32(rcx.wrapping_add(P9), (0x7fffffffu32) as u32);
    crate::simd_ops::wu32(rcx.wrapping_add(P10), (0x80000000u32) as u32);
    crate::simd_ops::wu64(rcx.wrapping_add(P11), (0xffffffff80000000u64) as u64);
    crate::simd_ops::wu32(rcx.wrapping_add(0x28u64), (0xffffffffu32) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1417259f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1416bcf30; const P1: u64 = 0x143932190; const P2: u64 = 0x1416d9580; const P3: u64 = 0x1416bb560;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = rcx; r15 = rcx.wrapping_add(0x30); rdx = 0; fl_l = 0; fl_r = 0; rcx = r15; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rbx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rsi.wrapping_add(0x10u64), ((rbx as u32)) as u32); crate::simd_ops::wu32(rsi.wrapping_add(0x14u64), (0x7fffffffu32) as u32); crate::simd_ops::wu32(rsi.wrapping_add(0x18u64), (0x7fffffffu32) as u32); crate::simd_ops::wu32(rsi.wrapping_add(0x1cu64), (0x80000000u32) as u32); crate::simd_ops::wu64(rsi.wrapping_add(0x20u64), (0xffffffff80000000u64) as u64); crate::simd_ops::wu32(rsi.wrapping_add(0x28u64), (0xffffffffu32) as u32); rdi = ((rbx as u32)) as u32 as u64; fl_l = (crate::simd_ops::ru32(rsi.wrapping_add(0x50u64))) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 6; continue; } bb = 1; continue; }
            1 => { bb = 2; continue; }
            2 => { rax = ((rdi as u32)) as u32 as u64; rdx = ((rax.wrapping_mul(8)) as u64) as u64; rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x48u64))) as u64; rcx = (crate::simd_ops::ru64(rdx.wrapping_add(rax))) as u64; crate::simd_ops::wu64(rcx.wrapping_add(0x30u64), rbx); r14 = (crate::simd_ops::ru64(rsi.wrapping_add(0x48u64))) as u64; r14 = ((r14).wrapping_add(rdx)) as u64; rdx = (crate::simd_ops::ru64(r14)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 3; continue; }
            3 => { rax = 0xffffffff; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rdx, xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru64(P1)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x60u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            5 => { crate::simd_ops::wu64(r14, rbx); rdi = (((rdi as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((rdi as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rsi.wrapping_add(0x50u64))) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 2; continue; } bb = 6; continue; }
            6 => { rsi = ((rsi).wrapping_add(0x48u64)) as u64; rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x8u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 9; continue; } bb = 7; continue; }
            7 => { rdx = ((rax as u32)) as u32 as u64; bb = 8; continue; }
            8 => { rax = (crate::simd_ops::ru64(rsi)) as u64; rcx = (crate::simd_ops::ru64(rbx.wrapping_add(rax))) as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x10u64), (0xffffffffu32) as u32); rbx = rbx.wrapping_add(0x8); fl_l = rdx; fl_r = (0x1u64) as u64; rdx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 8; continue; } bb = 9; continue; }
            9 => { r8 = 0; fl_l = 0; fl_r = 0; rdx = rsi; rcx = rsi; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = rsi; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru64(r15)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { rcx = (crate::simd_ops::ru64(P1)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x60u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 11; continue; }
            11 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141725b10(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1416bcf30; const P1: u64 = 0x14; const P2: u64 = 0x18; const P3: u64 = 0x1c; const P4: u64 = 0x20; const P5: u64 = 0x28; const P6: u64 = 0x10; const P7: u64 = 0x50; const P8: u64 = 0x48; const P9: u64 = 0x30; const P10: u64 = 0x143932190; const P11: u64 = 0x60; const P12: u64 = 0x1416d9580;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rsi); rdx = 0; fl_l = 0; fl_r = 0; rsi = rcx; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rdi); rcx = ((rcx).wrapping_add(0x30u64)) as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rbx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rsi.wrapping_add(P1), (0x7fffffffu32) as u32); crate::simd_ops::wu32(rsi.wrapping_add(P2), (0x7fffffffu32) as u32); rdi = ((rbx as u32)) as u32 as u64; crate::simd_ops::wu32(rsi.wrapping_add(P3), (0x80000000u32) as u32); crate::simd_ops::wu64(rsi.wrapping_add(P4), (0xffffffff80000000u64) as u64); crate::simd_ops::wu32(rsi.wrapping_add(P5), (0xffffffffu32) as u32); crate::simd_ops::wu32(rsi.wrapping_add(P6), ((rbx as u32)) as u32); fl_l = (crate::simd_ops::ru32(rsi.wrapping_add(P7))) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 7; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), r14); bb = 2; continue; }
            2 => { rax = ((rdi as u32)) as u32 as u64; rdx = ((rax.wrapping_mul(8)) as u64) as u64; rax = (crate::simd_ops::ru64(rsi.wrapping_add(P8))) as u64; rcx = (crate::simd_ops::ru64(rdx.wrapping_add(rax))) as u64; crate::simd_ops::wu64(rcx.wrapping_add(P9), rbx); r14 = (crate::simd_ops::ru64(rsi.wrapping_add(P8))) as u64; r14 = ((r14).wrapping_add(rdx)) as u64; rdx = (crate::simd_ops::ru64(r14)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 3; continue; }
            3 => { rax = 0xffffffff; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rdx, xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru64(P10)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P11))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            5 => { rdi = (((rdi as u32)).wrapping_add(1)) as u32 as u64; crate::simd_ops::wu64(r14, rbx); fl_l = ((rdi as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rsi.wrapping_add(0x50u64))) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 2; continue; } bb = 6; continue; }
            6 => { r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x50u64))) as u32 as u64; rcx = rsi.wrapping_add(0x48); rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 10; continue; } bb = 8; continue; }
            8 => { r8 = ((rax as u32)) as u32 as u64; bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rbx = rbx.wrapping_add(0x8); rdx = (crate::simd_ops::ru64(rax.wrapping_add(rbx).wrapping_sub(0x8u64))) as u64; crate::simd_ops::wu32(rdx.wrapping_add(0x10u64), (0xffffffffu32) as u32); fl_l = r8; fl_r = (0x1u64) as u64; r8 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 9; continue; } bb = 10; continue; }
            10 => { r8 = 0; fl_l = 0; fl_r = 0; rdx = rcx; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P12, rax, rcx, rdx, r8, r9, rsp); }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141725c10(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x14; const P2: u64 = 0x80; const P3: u64 = 0x1417271e0; const P4: u64 = 0x50; const P5: u64 = 0x48; const P6: u64 = 0x8; const P7: u64 = 0x1417482b0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; fl_r = 0x0;
    rbx = rcx;
    if (((fl_l as u32)) == ((fl_r as u32))) { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    rdx = ((rcx.wrapping_add(P1)) as u64) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rsi);
    rcx = (crate::simd_ops::ru64(rcx)) as u64;
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64;
    rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp);
    fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(P0))) as u32 as u64; fl_r = 0x0;
    rsi = (crate::simd_ops::ru32(rbx.wrapping_add(P4))) as u32 as u64;
    if (((fl_l as u32)) <= ((fl_r as u32))) { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rdi);
    loop { r8 = (crate::simd_ops::ru64(rbx.wrapping_add(P5))) as u64; rdx = rsp.wrapping_add(0x30); rax = ((rsi as u32)) as u32 as u64; rcx = ((rbx.wrapping_add(P6)) as u64) as u64; rax = (((rax as u32)).wrapping_sub(crate::simd_ops::ru32(rbx.wrapping_add(P0)))) as u32 as u64; r8 = (crate::simd_ops::ru64(r8.wrapping_add(rax.wrapping_mul(8)))) as u64; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::lk_add_u32(rbx.wrapping_add(P0), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if !((((fl_l as u32)) != 0)) { break; } }
    rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64;
    rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141725c90(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141747f40; const P1: u64 = 0x1416d9580; const P2: u64 = 0x1416bce80; const P3: u64 = 0x141747da0; const P4: u64 = 0x143932190;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = r8; rdi = rcx; r9 = 0; fl_l = 0; fl_r = 0; r8 = (crate::simd_ops::ru64(rcx)) as u64; rcx = rsp.wrapping_add(0x30); rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); r10 = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; fl_l = r10; fl_r = r10; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rbx = 0; fl_l = 0; fl_r = 0; bb = 14; continue; }
            2 => { crate::simd_ops::wu64(r10.wrapping_add(0x50u64), rbx); rsi = (crate::simd_ops::ru32(rdi.wrapping_add(0x50u64))) as u32 as u64; r8 = (crate::simd_ops::ru64(rdi.wrapping_add(0x50u64))) as u64; r8 = ((r8).wrapping_add(1)) as u64; rdx = rdi.wrapping_add(0x48); rcx = rdi.wrapping_add(0x48); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x50u64))) as u64; rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x48u64))) as u64; rdx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::lk_add_u32(rax, (1) as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; bb = 4; continue; }
            4 => { crate::simd_ops::wu64(rdx.wrapping_sub(0x8u64), rax); rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x50u64))) as u64; rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x48u64))) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_sub(0x8u64))) as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x10u64), ((rsi as u32)) as u32); rax = (crate::simd_ops::ru64(rdi)) as u64; fl_l = (crate::simd_ops::ru32(rax.wrapping_add(0x68u64))) as u32 as u64; fl_r = 0x7fffffff; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 12; continue; } bb = 5; continue; }
            5 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rdi = ((rdi).wrapping_add(0x30u64)) as u64; rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; rbx = rax.wrapping_add(0x1); rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; fl_l = rbx; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 8; continue; } bb = 6; continue; }
            6 => { rcx = ((rcx).wrapping_shr(1u32)) as u64; fl_l = rbx; fl_r = rcx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 11; continue; } bb = 7; continue; }
            7 => { r8 = rbx; bb = 10; continue; }
            8 => { fl_l = rbx; fl_r = rcx; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 11; continue; } bb = 9; continue; }
            9 => { r8 = rbx; r8 = ((r8).wrapping_shr(2u32)) as u64; fl_l = ((r8).wrapping_add(rbx)) as u64; fl_r = fl_l; r8 = ((fl_l as u64)) as u64; bb = 10; continue; }
            10 => { rdx = rdi; rcx = rdi; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 11; continue; }
            11 => { crate::simd_ops::wu64(rdi.wrapping_add(0x8u64), rbx); rax = (crate::simd_ops::ru64(rdi)) as u64; crate::simd_ops::wu64(rax.wrapping_add(rbx.wrapping_mul(8)).wrapping_sub(0x8u64), rsi); r10 = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; bb = 13; continue; }
            12 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); r10 = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; r9 = (crate::simd_ops::ru32(rdi.wrapping_add(0x14u64))) as u32 as u64; fl_l = ((r9 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(r10.wrapping_add(0x14u64))) as u32 as u64; r9 = (if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { crate::simd_ops::ru32(r10.wrapping_add(0x14u64)) } else { (r9 as u32) }) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x14u64), ((r9 as u32)) as u32); r8 = (crate::simd_ops::ru32(r10.wrapping_add(0x1cu64))) as u32 as u64; fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x1cu64))) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; r8 = (if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { crate::simd_ops::ru32(rdi.wrapping_add(0x1cu64)) } else { (r8 as u32) }) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x1cu64), ((r8 as u32)) as u32); rdx = (crate::simd_ops::ru32(rdi.wrapping_add(0x18u64))) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(r10.wrapping_add(0x18u64))) as u32 as u64; rdx = (if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { crate::simd_ops::ru32(r10.wrapping_add(0x18u64)) } else { (rdx as u32) }) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x18u64), ((rdx as u32)) as u32); rcx = (crate::simd_ops::ru32(r10.wrapping_add(0x20u64))) as u32 as u64; fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x20u64))) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; rcx = (if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { crate::simd_ops::ru32(rdi.wrapping_add(0x20u64)) } else { (rcx as u32) }) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x20u64), ((rcx as u32)) as u32); r8 = (((r8 as u32)).wrapping_sub((r9 as u32))) as u32 as u64; r8 = (((r8 as u32)).wrapping_add(1)) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x24u64), ((r8 as u32)) as u32); rcx = (((rcx as u32)).wrapping_sub((rdx as u32))) as u32 as u64; rcx = (((rcx as u32)).wrapping_add(1)) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x28u64), ((rcx as u32)) as u32); fl_l = (crate::simd_ops::lk_add_u32(rdi.wrapping_add(0x10u64), (1) as u32)) as u32 as u64; bb = 13; continue; }
            13 => { rbx = r10; bb = 14; continue; }
            14 => { fl_l = r10; fl_r = r10; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 15; continue; }
            15 => { rcx = 0xffffffff; let xa_src: u32 = (rcx as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(r10, xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rcx = (xa_old) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 17; continue; } bb = 16; continue; }
            16 => { rcx = (crate::simd_ops::ru64(P4)) as u64; r8 = (crate::simd_ops::ru64(rcx)) as u64; rdx = r10; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(r8.wrapping_add(0x60u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 17; continue; }
            17 => { rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141725e20(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143428750; const P1: u64 = 0x141747da0; const P2: u64 = 0x1417271e0; const P3: u64 = 0x1417482b0; const P4: u64 = 0x143932190;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x48u64)) as u64; xmm0 = (crate::simd_ops::ru128(P0)) as u128; r15 = 0; fl_l = 0; fl_r = 0; rbx = rcx; crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), rsi); rsi = ((r15 as u32)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), r14); r14 = rdx; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (0xffffffffffffffffu64) as u64); fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(0x38u64))) as u32 as u64; fl_r = ((r15 as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x30u64))) as u64; rcx = ((rsi as u32)) as u32 as u64; rdi = (crate::simd_ops::ru64(rax.wrapping_add(rcx.wrapping_mul(8)))) as u64; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r9 = (crate::simd_ops::ru32(rsp.wrapping_add(0x20u64))) as u32 as u64; fl_l = ((r9 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rdi.wrapping_add(0x14u64))) as u32 as u64; r8 = (crate::simd_ops::ru32(rsp.wrapping_add(0x28u64))) as u32 as u64; r9 = (if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { crate::simd_ops::ru32(rdi.wrapping_add(0x14u64)) } else { (r9 as u32) }) as u32 as u64; rdx = (crate::simd_ops::ru32(rsp.wrapping_add(0x24u64))) as u32 as u64; rcx = (crate::simd_ops::ru32(rsp.wrapping_add(0x2cu64))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((r9 as u32)) as u32); fl_l = ((r8 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rdi.wrapping_add(0x1cu64))) as u32 as u64; r8 = (if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { crate::simd_ops::ru32(rdi.wrapping_add(0x1cu64)) } else { (r8 as u32) }) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((r8 as u32)) as u32); fl_l = ((rdx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rdi.wrapping_add(0x18u64))) as u32 as u64; rdx = (if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { crate::simd_ops::ru32(rdi.wrapping_add(0x18u64)) } else { (rdx as u32) }) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x24u64), ((rdx as u32)) as u32); fl_l = ((rcx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rdi.wrapping_add(0x20u64))) as u32 as u64; rcx = (if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { crate::simd_ops::ru32(rdi.wrapping_add(0x20u64)) } else { (rcx as u32) }) as u32 as u64; r8 = (((r8 as u32)).wrapping_sub((r9 as u32))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x2cu64), ((rcx as u32)) as u32); r8 = (((r8 as u32)).wrapping_add(1)) as u32 as u64; rcx = (((rcx as u32)).wrapping_sub((rdx as u32))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((r8 as u32)) as u32); rcx = (((rcx as u32)).wrapping_add(1)) as u32 as u64; rsi = (((rsi as u32)).wrapping_add(1)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x34u64), ((rcx as u32)) as u32); fl_l = ((rsi as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rbx.wrapping_add(0x38u64))) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 1; continue; } bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; rdx = rsp.wrapping_add(0x20); rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x80u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r9 = (crate::simd_ops::ru32(r14)) as u32 as u64; rsi = ((r15 as u32)) as u32 as u64; fl_l = ((r9 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rsp.wrapping_add(0x20u64))) as u32 as u64; r8 = (crate::simd_ops::ru32(rsp.wrapping_add(0x28u64))) as u32 as u64; r9 = (if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { crate::simd_ops::ru32(rsp.wrapping_add(0x20u64)) } else { (r9 as u32) }) as u32 as u64; rcx = (crate::simd_ops::ru32(rsp.wrapping_add(0x2cu64))) as u32 as u64; crate::simd_ops::wu32(r14, ((r9 as u32)) as u32); fl_l = (crate::simd_ops::ru32(r14.wrapping_add(0x8u64))) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; r8 = (if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { crate::simd_ops::ru32(r14.wrapping_add(0x8u64)) } else { (r8 as u32) }) as u32 as u64; crate::simd_ops::wu32(r14.wrapping_add(0x8u64), ((r8 as u32)) as u32); rdx = (crate::simd_ops::ru32(r14.wrapping_add(0x4u64))) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rsp.wrapping_add(0x24u64))) as u32 as u64; rdx = (if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { crate::simd_ops::ru32(rsp.wrapping_add(0x24u64)) } else { (rdx as u32) }) as u32 as u64; crate::simd_ops::wu32(r14.wrapping_add(0x4u64), ((rdx as u32)) as u32); fl_l = (crate::simd_ops::ru32(r14.wrapping_add(0xcu64))) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; rcx = (if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { crate::simd_ops::ru32(r14.wrapping_add(0xcu64)) } else { (rcx as u32) }) as u32 as u64; r8 = (((r8 as u32)).wrapping_sub((r9 as u32))) as u32 as u64; crate::simd_ops::wu32(r14.wrapping_add(0xcu64), ((rcx as u32)) as u32); r8 = (((r8 as u32)).wrapping_add(1)) as u32 as u64; rcx = (((rcx as u32)).wrapping_sub((rdx as u32))) as u32 as u64; crate::simd_ops::wu32(r14.wrapping_add(0x10u64), ((r8 as u32)) as u32); rcx = (((rcx as u32)).wrapping_add(1)) as u32 as u64; crate::simd_ops::wu32(r14.wrapping_add(0x14u64), ((rcx as u32)) as u32); r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x38u64))) as u32 as u64; fl_r = ((r15 as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 6; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), rbp); bb = 4; continue; }
            4 => { r8 = (crate::simd_ops::ru64(rbx.wrapping_add(0x30u64))) as u64; rdx = rsp.wrapping_add(0x60); rax = ((rsi as u32)) as u32 as u64; rcx = rbx.wrapping_add(0x8); rdi = ((rax.wrapping_mul(8)) as u64) as u64; r8 = (crate::simd_ops::ru64(rdi.wrapping_add(r8))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x30u64))) as u64; rsi = (((rsi as u32)).wrapping_add(1)) as u32 as u64; crate::simd_ops::wu64(rdi.wrapping_add(rax), r15); fl_l = ((rsi as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rbx.wrapping_add(0x38u64))) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 4; continue; } bb = 5; continue; }
            5 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; bb = 6; continue; }
            6 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x30u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { rcx = (crate::simd_ops::ru64(P4)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x60u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 8; continue; }
            8 => { crate::simd_ops::wu64(rbx.wrapping_add(0x30u64), r15); crate::simd_ops::wu64(rbx.wrapping_add(0x38u64), r15); crate::simd_ops::wu64(rbx.wrapping_add(0x40u64), r15); rsp = ((rsp).wrapping_add(0x48u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141725fc0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141748760; const P1: u64 = 0x1416bcf30; const P2: u64 = 0x143932190; const P3: u64 = 0x1416d9580; const P4: u64 = 0x14183acc2;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rdx; rsi = rcx; fl_l = (crate::simd_ops::ru64(rdx.wrapping_add(0x30u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 23; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru32(rdx.wrapping_add(0x2cu64))) as u32 as u64; fl_r = 0x7fffffff; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rcx = ((rcx).wrapping_add(0x8u64)) as u64; r8 = rdx; rdx = rsp.wrapping_add(0x68); rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); bb = 11; continue; }
            3 => { rax = 0; fl_l = 0; fl_r = 0; rdx = (crate::simd_ops::ru32(rcx.wrapping_add(0x38u64))) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 11; continue; } bb = 4; continue; }
            4 => { r8 = (crate::simd_ops::ru64(rcx.wrapping_add(0x30u64))) as u64; bb = 5; continue; }
            5 => { fl_l = (crate::simd_ops::ru64(r8.wrapping_add(rax.wrapping_mul(8)))) as u64; fl_r = rbx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 8; continue; } bb = 6; continue; }
            6 => { rax = (((rax as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 5; continue; } bb = 7; continue; }
            7 => { bb = 11; continue; }
            8 => { r9 = ((rdx.wrapping_sub(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 10; continue; } bb = 9; continue; }
            9 => { rdx = (crate::simd_ops::ru64(r8.wrapping_add(r9.wrapping_mul(8)))) as u64; crate::simd_ops::wu64(r8.wrapping_add(rax.wrapping_mul(8)), rdx); bb = 10; continue; }
            10 => { rdx = ((r9 as u32)) as u32 as u64; rcx = ((rcx).wrapping_add(0x30u64)) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 11; continue; }
            11 => { crate::simd_ops::wu64(rbx.wrapping_add(0x30u64), (0x0u64) as u64); crate::simd_ops::lk_add_u32(rbx, (1) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), rbx); r13 = (crate::simd_ops::ru32(rbx.wrapping_add(0x10u64))) as u32 as u64; rdi = (crate::simd_ops::ru32(rsi.wrapping_add(0x50u64))) as u32 as u64; rdi = (((rdi as u32)).wrapping_sub(1)) as u32 as u64; r12 = ((r13.wrapping_mul(8)) as u64) as u64; rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x48u64))) as u64; rcx = (crate::simd_ops::ru64(r12.wrapping_add(rax))) as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x10u64), (0xffffffffu32) as u32); r14 = 0xffffffff; fl_l = ((r13 as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 18; continue; } bb = 12; continue; }
            12 => { rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x48u64))) as u64; rbp = ((rcx.wrapping_add(rdi.wrapping_mul(8))) as u64) as u64; r15 = ((r12.wrapping_add(rcx)) as u64) as u64; rax = (crate::simd_ops::ru64(rbp)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 13; continue; }
            13 => { fl_l = (crate::simd_ops::lk_add_u32(rax, (1) as u32)) as u32 as u64; bb = 14; continue; }
            14 => { rdx = (crate::simd_ops::ru64(r15)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 15; continue; }
            15 => { rax = ((r14 as u32)) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rdx, xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 17; continue; } bb = 16; continue; }
            16 => { rcx = (crate::simd_ops::ru64(P2)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x60u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 17; continue; }
            17 => { rax = (crate::simd_ops::ru64(rbp)) as u64; crate::simd_ops::wu64(r15, rax); rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x48u64))) as u64; rcx = (crate::simd_ops::ru64(r12.wrapping_add(rax))) as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x10u64), ((r13 as u32)) as u32); bb = 18; continue; }
            18 => { rbp = (crate::simd_ops::ru64(rsi.wrapping_add(0x50u64))) as u64; r8 = rdi; rdx = rsi.wrapping_add(0x48); rcx = rsi.wrapping_add(0x48); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); fl_l = rdi; fl_r = rbp; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 21; continue; } bb = 19; continue; }
            19 => { rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x48u64))) as u64; rcx = ((rax.wrapping_add(rbp.wrapping_mul(8))) as u64) as u64; fl_l = rdi; fl_r = rbp; rdi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 21; continue; } bb = 20; continue; }
            20 => { rdi = ((rdi).wrapping_shl(3u32)) as u64; r8 = rdi; rdx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 21; continue; }
            21 => { let xa_src: u32 = (r14 as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx, xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; r14 = (xa_old) as u32 as u64; rax = ((r14.wrapping_sub(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 23; continue; } bb = 22; continue; }
            22 => { rcx = (crate::simd_ops::ru64(P2)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x60u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 23; continue; }
            23 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141726130(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    xmm0 = 0u128;
    rax = rcx;
    crate::simd_ops::wu128(rcx, (xmm0) as u128);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141726140(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14397d1e0; const P1: u64 = 0x143932470; const P2: u64 = 0x143932460; const P3: u64 = 0x141834a0c; const P4: u64 = 0x141834a78; const P5: u64 = 0x143243df8; const P6: u64 = 0x142646950;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru32(P0)) as u32 as u64; rcx = ((crate::exe::tls_pointer() as u64)) as u64; rdx = 0x74; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(rax.wrapping_mul(8)))) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(rcx))) as u32 as u64; fl_l = (crate::simd_ops::ru32(P1)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { bb = 4; continue; } bb = 1; continue; }
            1 => { rax = P2; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            2 => { rbx = (rbx & !0xffu64) | (((((rbx as u8)).wrapping_shl(4u32)) as u8 as u64)); rdi = ((rdi).wrapping_add(0x2u64)) as u64; rbx = (rbx & !0xffu64) | ((((((rbx as u8)) | ((rcx as u8)))) as u8 as u64)); rbp = (((rbp as u32)).wrapping_add(1)) as u32 as u64; crate::simd_ops::wu8(r14, ((rbx as u8)) as u8); r14 = ((r14).wrapping_add(1)) as u64; fl_l = ((rbp as u32)) as u32 as u64; fl_r = 0x10; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 6; continue; } bb = 3; continue; }
            3 => { rcx = P1; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rax = rsi; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            4 => { rcx = P1; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru32(P1)) as u32 as u64; fl_r = 0xffffffff; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 1; continue; } bb = 5; continue; }
            5 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx); rsi = P2; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rbp); rbp = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rdi); rdi = P5; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), r14); r14 = rsi; bb = 6; continue; }
            6 => { rax = ((rbp.wrapping_sub(0x4u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0xfffffff9; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { fl_l = ((rdi).wrapping_add(1)) as u64; rdi = ((fl_l as u64)) as u64; bb = 8; continue; }
            8 => { rbx = (((crate::simd_ops::ru8(rdi)) as i8 as i32) as u32) as u32 as u64; rax = ((rbx.wrapping_sub(0x30u64)) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x9u8) as u8 as u64; if (((fl_l as u8)) > ((fl_r as u8))) { bb = 10; continue; } bb = 9; continue; }
            9 => { fl_l = ((rbx as u8)) as u8 as u64; fl_r = (0x30u8) as u8 as u64; rbx = (rbx & !0xffu64) | ((((fl_l as u8).wrapping_sub(fl_r as u8)) as u8 as u64)); bb = 11; continue; }
            10 => { rcx = ((rbx as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp); rbx = ((rax.wrapping_sub(0x57u64)) as u32) as u32 as u64; bb = 11; continue; }
            11 => { rcx = (((crate::simd_ops::ru8(rdi.wrapping_add(0x1u64))) as i8 as i32) as u32) as u32 as u64; rax = ((rcx.wrapping_sub(0x30u64)) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x9u8) as u8 as u64; if (((fl_l as u8)) > ((fl_r as u8))) { bb = 13; continue; } bb = 12; continue; }
            12 => { fl_l = ((rcx as u8)) as u8 as u64; fl_r = (0x30u8) as u8 as u64; rcx = (rcx & !0xffu64) | ((((fl_l as u8).wrapping_sub(fl_r as u8)) as u8 as u64)); bb = 2; continue; }
            13 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp); rcx = ((rax.wrapping_sub(0x57u64)) as u32) as u32 as u64; bb = 2; continue; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141726240(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xfffffffffffffffc; const P1: u64 = 0xffffffffffffffd0; const P2: u64 = 0x142646950; const P3: u64 = 0xffffffffffffffa9; const P4: u64 = 0x1;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = rdx; r14 = rcx; rsi = 0; fl_l = 0; fl_r = 0; bb = 1; continue; }
            1 => { rax = ((rsi.wrapping_add(P0)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0xfffffff9; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { fl_l = ((rdi).wrapping_add(1)) as u64; rdi = ((fl_l as u64)) as u64; bb = 3; continue; }
            3 => { rbx = (((crate::simd_ops::ru8(rdi)) as i8 as i32) as u32) as u32 as u64; rax = ((rbx.wrapping_add(P1)) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x9u8) as u8 as u64; if (((fl_l as u8)) > ((fl_r as u8))) { bb = 5; continue; } bb = 4; continue; }
            4 => { fl_l = ((rbx as u8)) as u8 as u64; fl_r = (0x30u8) as u8 as u64; rbx = (rbx & !0xffu64) | ((((fl_l as u8).wrapping_sub(fl_r as u8)) as u8 as u64)); bb = 6; continue; }
            5 => { rcx = ((rbx as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); rbx = ((rax.wrapping_add(P3)) as u32) as u32 as u64; bb = 6; continue; }
            6 => { rcx = (((crate::simd_ops::ru8(rdi.wrapping_add(P4))) as i8 as i32) as u32) as u32 as u64; rax = ((rcx.wrapping_add(P1)) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x9u8) as u8 as u64; if (((fl_l as u8)) > ((fl_r as u8))) { bb = 8; continue; } bb = 7; continue; }
            7 => { fl_l = ((rcx as u8)) as u8 as u64; fl_r = (0x30u8) as u8 as u64; rcx = (rcx & !0xffu64) | ((((fl_l as u8).wrapping_sub(fl_r as u8)) as u8 as u64)); bb = 9; continue; }
            8 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); rcx = ((rax.wrapping_add(P3)) as u32) as u32 as u64; bb = 9; continue; }
            9 => { rbx = (rbx & !0xffu64) | (((((rbx as u8)).wrapping_shl(4u32)) as u8 as u64)); rdi = ((rdi).wrapping_add(0x2u64)) as u64; rbx = (rbx & !0xffu64) | ((((((rbx as u8)) | ((rcx as u8)))) as u8 as u64)); rsi = (((rsi as u32)).wrapping_add(1)) as u32 as u64; crate::simd_ops::wu8(r14, ((rbx as u8)) as u8); r14 = ((r14).wrapping_add(1)) as u64; fl_l = ((rsi as u32)) as u32 as u64; fl_r = 0x10; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 1; continue; } bb = 10; continue; }
            10 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1417262d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 83>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r11 = rsp; rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x280u64)) as u64; rax = r8; rbp = r8; rax = ((rax).wrapping_sub(rdx)) as u64; rdi = rdx; rbx = rcx; fl_l = rax; fl_r = (0x2u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 25; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(r11.wrapping_add(0x10u64), r12); r12 = rsp; crate::simd_ops::wu64(r11.wrapping_add(0x8u64), rsi); crate::simd_ops::wu64(r11.wrapping_add(0x18u64), r14); crate::simd_ops::wu64(r11.wrapping_add(0x20u64), r15); bb = 2; continue; }
            2 => { rax = rbp; r9 = rdi.wrapping_add(0x1); rax = ((rax).wrapping_sub(rdi)) as u64; fl_l = rax; fl_r = (0x9u64) as u64; if ((((fl_l as u64)) as i64) <= (((fl_r as u64)) as i64)) { bb = 15; continue; } bb = 3; continue; }
            3 => { r8 = (crate::simd_ops::ru64(rbx)) as u64; r10 = rbp.wrapping_sub(0x1); rdx = if (rax as i64) < 0 { 0xffffffffffffffffu64 } else { 0u64 }; rax = ((rax).wrapping_sub(rdx)) as u64; rax = ((((rax) as i64).wrapping_shr(1u32) as u64)) as u64; rcx = (crate::simd_ops::ru64(r8.wrapping_add(rdi.wrapping_mul(8)))) as u64; rax = ((rax).wrapping_add(rdi)) as u64; rdx = ((r8.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rax = (crate::simd_ops::ru64(r8.wrapping_add(rax.wrapping_mul(8)))) as u64; crate::simd_ops::wu64(r8.wrapping_add(rdi.wrapping_mul(8)), rax); crate::simd_ops::wu64(rdx, rcx); r8 = (crate::simd_ops::ru64(rbx)) as u64; rsi = (crate::simd_ops::ru64(r8.wrapping_add(r10.wrapping_mul(8)))) as u64; r11 = ((r8.wrapping_add(r10.wrapping_mul(8))) as u64) as u64; r15 = (crate::simd_ops::ru64(r8.wrapping_add(r9.wrapping_mul(8)))) as u64; r14 = ((r8.wrapping_add(r9.wrapping_mul(8))) as u64) as u64; rax = (crate::simd_ops::ru64(rsi)) as u64; rdx = (crate::simd_ops::ru64(r15)) as u64; rcx = (crate::simd_ops::ru32(rax.wrapping_add(0x20u64))) as u32 as u64; fl_l = (crate::simd_ops::ru32(rdx.wrapping_add(0x20u64))) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 5; continue; } bb = 4; continue; }
            4 => { crate::simd_ops::wu64(r11, r15); crate::simd_ops::wu64(r14, rsi); r8 = (crate::simd_ops::ru64(rbx)) as u64; bb = 5; continue; }
            5 => { r15 = (crate::simd_ops::ru64(r8.wrapping_add(rdi.wrapping_mul(8)))) as u64; r14 = ((r8.wrapping_add(rdi.wrapping_mul(8))) as u64) as u64; rsi = (crate::simd_ops::ru64(r8.wrapping_add(r9.wrapping_mul(8)))) as u64; r11 = ((r8.wrapping_add(r9.wrapping_mul(8))) as u64) as u64; rax = (crate::simd_ops::ru64(r15)) as u64; rdx = (crate::simd_ops::ru64(rsi)) as u64; rcx = (crate::simd_ops::ru32(rax.wrapping_add(0x20u64))) as u32 as u64; fl_l = (crate::simd_ops::ru32(rdx.wrapping_add(0x20u64))) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 7; continue; } bb = 6; continue; }
            6 => { crate::simd_ops::wu64(r14, rsi); crate::simd_ops::wu64(r11, r15); r8 = (crate::simd_ops::ru64(rbx)) as u64; bb = 7; continue; }
            7 => { r11 = (crate::simd_ops::ru64(r8.wrapping_add(r10.wrapping_mul(8)))) as u64; rsi = (crate::simd_ops::ru64(r8.wrapping_add(rdi.wrapping_mul(8)))) as u64; rax = (crate::simd_ops::ru64(r11)) as u64; rdx = (crate::simd_ops::ru64(rsi)) as u64; rcx = (crate::simd_ops::ru32(rax.wrapping_add(0x20u64))) as u32 as u64; fl_l = (crate::simd_ops::ru32(rdx.wrapping_add(0x20u64))) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 9; continue; } bb = 8; continue; }
            8 => { crate::simd_ops::wu64(r8.wrapping_add(r10.wrapping_mul(8)), rsi); crate::simd_ops::wu64(r8.wrapping_add(rdi.wrapping_mul(8)), r11); bb = 9; continue; }
            9 => { rdx = (crate::simd_ops::ru64(rbx)) as u64; rax = (crate::simd_ops::ru64(rdx.wrapping_add(rdi.wrapping_mul(8)))) as u64; rcx = (crate::simd_ops::ru64(rax)) as u64; r8 = (crate::simd_ops::ru32(rcx.wrapping_add(0x20u64))) as u32 as u64; bb = 10; continue; }
            10 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(r9.wrapping_mul(8)).wrapping_add(0x8u64))) as u64; r9 = ((r9).wrapping_add(1)) as u64; rcx = (crate::simd_ops::ru64(rax)) as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rcx.wrapping_add(0x20u64))) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 10; continue; } bb = 11; continue; }
            11 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(r10.wrapping_mul(8)).wrapping_sub(0x8u64))) as u64; r10 = ((r10).wrapping_sub(1)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; fl_l = (crate::simd_ops::ru32(rax.wrapping_add(0x20u64))) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 11; continue; } bb = 12; continue; }
            12 => { fl_l = r9; fl_r = r10; if ((((fl_l as u64)) as i64) > (((fl_r as u64)) as i64)) { bb = 14; continue; } bb = 13; continue; }
            13 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(r9.wrapping_mul(8)))) as u64; crate::simd_ops::wu64(rdx.wrapping_add(r9.wrapping_mul(8)), rcx); crate::simd_ops::wu64(rdx.wrapping_add(r10.wrapping_mul(8)), rax); bb = 9; continue; }
            14 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(rdi.wrapping_mul(8)))) as u64; r8 = rbp; crate::simd_ops::wu64(rdx.wrapping_add(rdi.wrapping_mul(8)), rcx); r8 = ((r8).wrapping_sub(r9)) as u64; crate::simd_ops::wu64(rdx.wrapping_add(r10.wrapping_mul(8)), rax); rcx = r10; rcx = ((rcx).wrapping_sub(rdi)) as u64; rax = r9; fl_l = rcx; fl_r = r8; rdx = r10; rax = (if ((((fl_l as u64)) as i64) <= (((fl_r as u64)) as i64)) { rdi } else { rax }) as u64; rdx = (if ((((fl_l as u64)) as i64) <= (((fl_r as u64)) as i64)) { rbp } else { rdx }) as u64; rdi = (if ((((fl_l as u64)) as i64) <= (((fl_r as u64)) as i64)) { r9 } else { rdi }) as u64; rbp = (if ((((fl_l as u64)) as i64) <= (((fl_r as u64)) as i64)) { r10 } else { rbp }) as u64; crate::simd_ops::wu64(r12, rdi); rdi = rax; crate::simd_ops::wu64(r12.wrapping_add(0x8u64), rdx); fl_l = ((r12).wrapping_add(0x10u64)) as u64; fl_r = fl_l; r12 = ((fl_l as u64)) as u64; bb = 2; continue; }
            15 => { rax = rdi; fl_l = r9; fl_r = rbp; if ((((fl_l as u64)) as i64) >= (((fl_r as u64)) as i64)) { bb = 22; continue; } bb = 16; continue; }
            16 => { bb = 17; continue; }
            17 => { r11 = (crate::simd_ops::ru64(rbx)) as u64; r10 = ((rax.wrapping_mul(8)) as u64) as u64; rcx = (crate::simd_ops::ru64(r11.wrapping_add(r10).wrapping_add(0x8u64))) as u64; r8 = (crate::simd_ops::ru64(rcx)) as u64; rcx = (crate::simd_ops::ru64(r11.wrapping_add(r10))) as u64; rdx = (crate::simd_ops::ru64(rcx)) as u64; rcx = (crate::simd_ops::ru32(r8.wrapping_add(0x20u64))) as u32 as u64; fl_l = (crate::simd_ops::ru32(rdx.wrapping_add(0x20u64))) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 21; continue; } bb = 18; continue; }
            18 => { bb = 19; continue; }
            19 => { rcx = (crate::simd_ops::ru64(r10.wrapping_add(r11))) as u64; rdx = (crate::simd_ops::ru64(r10.wrapping_add(r11).wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(r10.wrapping_add(r11).wrapping_add(0x8u64), rcx); crate::simd_ops::wu64(r10.wrapping_add(r11), rdx); fl_l = rax; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 21; continue; } bb = 20; continue; }
            20 => { r11 = (crate::simd_ops::ru64(rbx)) as u64; rax = ((rax).wrapping_sub(1)) as u64; r10 = ((rax.wrapping_mul(8)) as u64) as u64; rcx = (crate::simd_ops::ru64(r11.wrapping_add(r10).wrapping_add(0x8u64))) as u64; r8 = (crate::simd_ops::ru64(rcx)) as u64; rcx = (crate::simd_ops::ru64(r11.wrapping_add(r10))) as u64; rdx = (crate::simd_ops::ru64(rcx)) as u64; rcx = (crate::simd_ops::ru32(r8.wrapping_add(0x20u64))) as u32 as u64; fl_l = (crate::simd_ops::ru32(rdx.wrapping_add(0x20u64))) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 19; continue; } bb = 21; continue; }
            21 => { rax = r9; r9 = ((r9).wrapping_add(1)) as u64; fl_l = r9; fl_r = rbp; if ((((fl_l as u64)) as i64) < (((fl_r as u64)) as i64)) { bb = 17; continue; } bb = 22; continue; }
            22 => { rax = rsp; fl_l = r12; fl_r = rax; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 24; continue; } bb = 23; continue; }
            23 => { rdi = (crate::simd_ops::ru64(r12.wrapping_sub(0x10u64))) as u64; fl_l = r12; fl_r = (0x10u64) as u64; r12 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; rbp = (crate::simd_ops::ru64(r12.wrapping_add(0x8u64))) as u64; bb = 2; continue; }
            24 => { r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x2b0u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x2a0u64))) as u64; r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x2b8u64))) as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x2a8u64))) as u64; bb = 25; continue; }
            25 => { rsp = ((rsp).wrapping_add(0x280u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}
