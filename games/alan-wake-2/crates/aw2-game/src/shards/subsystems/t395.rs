#![allow(unused_imports, unused_variables, unused_mut, unused_assignments, non_snake_case, dead_code, clippy::all, unreachable_code, unused_parens)]

#[inline(never)]
pub unsafe extern "win64" fn fn_142531720(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1433e0fac; const P1: u64 = 0x141470f70; const P2: u64 = 0x141483600;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rbp = rdx; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (0x2u64) as u64); rdi = rcx; rax = P0; rcx = rbp; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); rdx = rsp.wrapping_add(0x20); rsi = r8; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 1; continue; }
            1 => { rbx = (crate::simd_ops::ru64(rax)) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 2; continue; }
            2 => { r9 = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; rax = ((r9.wrapping_add(r9.wrapping_mul(2))) as u64) as u64; rax = ((rax).wrapping_shr(2u32)) as u64; fl_l = rcx; fl_r = rax; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 10; continue; } bb = 3; continue; }
            3 => { fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 4; continue; }
            4 => { r10 = (crate::simd_ops::ru64(rdi.wrapping_add(0x18u64))) as u64; fl_l = rbx; fl_r = r10; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 9; continue; } bb = 5; continue; }
            5 => { r11 = (crate::simd_ops::ru64(rdi)) as u64; r9 = ((r9).wrapping_sub(1)) as u64; rcx = rbx; rcx = ((rcx).wrapping_shr(5u32)) as u64; rcx = (((rcx) ^ (rbx))) as u64; rcx = ((rcx).wrapping_shr(4u32)) as u64; rcx = (((rcx) & (r9))) as u64; rdx = 0; fl_l = 0; fl_r = 0; bb = 6; continue; }
            6 => { rax = rcx; rax = ((rax).wrapping_add(rax)) as u64; r8 = (crate::simd_ops::ru64(r11.wrapping_add(rax.wrapping_mul(8)))) as u64; fl_l = r8; fl_r = rbx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 10; continue; } bb = 7; continue; }
            7 => { fl_l = r8; fl_r = r10; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 9; continue; } bb = 8; continue; }
            8 => { rcx = ((rcx).wrapping_add(1)) as u64; rcx = ((rcx).wrapping_add(rdx)) as u64; rdx = ((rdx).wrapping_add(1)) as u64; rcx = (((rcx) & (r9))) as u64; fl_l = rdx; fl_r = r9; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 6; continue; } bb = 9; continue; }
            9 => { rcx = rdi; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 10; continue; }
            10 => { r9 = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; rcx = rbx; r10 = (crate::simd_ops::ru64(rdi)) as u64; r9 = ((r9).wrapping_sub(1)) as u64; r11 = (crate::simd_ops::ru64(rdi.wrapping_add(0x18u64))) as u64; rcx = ((rcx).wrapping_shr(5u32)) as u64; rcx = (((rcx) ^ (rbx))) as u64; rcx = ((rcx).wrapping_shr(4u32)) as u64; rcx = (((rcx) & (r9))) as u64; rdx = 0; fl_l = 0; fl_r = 0; bb = 11; continue; }
            11 => { rax = rcx; rax = ((rax).wrapping_shl(4u32)) as u64; rax = ((rax).wrapping_add(r10)) as u64; r8 = (crate::simd_ops::ru64(rax)) as u64; fl_l = r8; fl_r = r11; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 15; continue; } bb = 12; continue; }
            12 => { fl_l = r8; fl_r = rbx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 16; continue; } bb = 13; continue; }
            13 => { rcx = ((rcx).wrapping_add(1)) as u64; rcx = ((rcx).wrapping_add(rdx)) as u64; rdx = ((rdx).wrapping_add(1)) as u64; rcx = (((rcx) & (r9))) as u64; fl_l = rdx; fl_r = r9; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 11; continue; } bb = 14; continue; }
            14 => { rax = 0; fl_l = 0; fl_r = 0; bb = 16; continue; }
            15 => { crate::simd_ops::wu64(rax, rbx); fl_l = (crate::simd_ops::lk_add_u64(rdi.wrapping_add(0x10u64), (1) as u64)) as u64; bb = 16; continue; }
            16 => { crate::simd_ops::wu32(rax.wrapping_add(0x8u64), (0x1u32) as u32); bb = 17; continue; }
            17 => { fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 40; continue; } bb = 18; continue; }
            18 => { rcx = (crate::simd_ops::ru64(rsi)) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 40; continue; } bb = 19; continue; }
            19 => { crate::simd_ops::wu32(rsp.wrapping_add(0x2cu64), (0x0u32) as u32); bb = 20; continue; }
            20 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rcx); rax = (0xffffffffffffffffu64) as u64; bb = 21; continue; }
            21 => { rax = ((rax).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(rax))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 21; continue; } bb = 22; continue; }
            22 => { rdx = rsp.wrapping_add(0x20); crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rax as u32)) as u32); rcx = rbp; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 39; continue; } bb = 23; continue; }
            23 => { rbx = (crate::simd_ops::ru64(rax)) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 39; continue; } bb = 24; continue; }
            24 => { r9 = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; rax = ((r9.wrapping_add(r9.wrapping_mul(2))) as u64) as u64; rax = ((rax).wrapping_shr(2u32)) as u64; fl_l = rcx; fl_r = rax; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 32; continue; } bb = 25; continue; }
            25 => { fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 31; continue; } bb = 26; continue; }
            26 => { r10 = (crate::simd_ops::ru64(rdi.wrapping_add(0x18u64))) as u64; fl_l = rbx; fl_r = r10; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 31; continue; } bb = 27; continue; }
            27 => { r11 = (crate::simd_ops::ru64(rdi)) as u64; r9 = ((r9).wrapping_sub(1)) as u64; rcx = rbx; rcx = ((rcx).wrapping_shr(5u32)) as u64; rcx = (((rcx) ^ (rbx))) as u64; rcx = ((rcx).wrapping_shr(4u32)) as u64; rcx = (((rcx) & (r9))) as u64; rdx = 0; fl_l = 0; fl_r = 0; bb = 28; continue; }
            28 => { rax = rcx; rax = ((rax).wrapping_add(rax)) as u64; r8 = (crate::simd_ops::ru64(r11.wrapping_add(rax.wrapping_mul(8)))) as u64; fl_l = r8; fl_r = rbx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 32; continue; } bb = 29; continue; }
            29 => { fl_l = r8; fl_r = r10; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 31; continue; } bb = 30; continue; }
            30 => { rcx = ((rcx).wrapping_add(1)) as u64; rcx = ((rcx).wrapping_add(rdx)) as u64; rdx = ((rdx).wrapping_add(1)) as u64; rcx = (((rcx) & (r9))) as u64; fl_l = rdx; fl_r = r9; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 28; continue; } bb = 31; continue; }
            31 => { rcx = rdi; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 32; continue; }
            32 => { r9 = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; rcx = rbx; r10 = (crate::simd_ops::ru64(rdi)) as u64; r9 = ((r9).wrapping_sub(1)) as u64; r11 = (crate::simd_ops::ru64(rdi.wrapping_add(0x18u64))) as u64; rcx = ((rcx).wrapping_shr(5u32)) as u64; rcx = (((rcx) ^ (rbx))) as u64; rcx = ((rcx).wrapping_shr(4u32)) as u64; rcx = (((rcx) & (r9))) as u64; rdx = 0; fl_l = 0; fl_r = 0; bb = 33; continue; }
            33 => { rax = rcx; rax = ((rax).wrapping_shl(4u32)) as u64; rax = ((rax).wrapping_add(r10)) as u64; r8 = (crate::simd_ops::ru64(rax)) as u64; fl_l = r8; fl_r = r11; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 37; continue; } bb = 34; continue; }
            34 => { fl_l = r8; fl_r = rbx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 38; continue; } bb = 35; continue; }
            35 => { rcx = ((rcx).wrapping_add(1)) as u64; rcx = ((rcx).wrapping_add(rdx)) as u64; rdx = ((rdx).wrapping_add(1)) as u64; rcx = (((rcx) & (r9))) as u64; fl_l = rdx; fl_r = r9; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 33; continue; } bb = 36; continue; }
            36 => { rax = 0; fl_l = 0; fl_r = 0; bb = 38; continue; }
            37 => { crate::simd_ops::wu64(rax, rbx); fl_l = (crate::simd_ops::lk_add_u64(rdi.wrapping_add(0x10u64), (1) as u64)) as u64; bb = 38; continue; }
            38 => { crate::simd_ops::wu32(rax.wrapping_add(0x8u64), (0x1u32) as u32); bb = 39; continue; }
            39 => { rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; rsi = ((rsi).wrapping_add(0x8u64)) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 20; continue; } bb = 40; continue; }
            40 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425319a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143424eb8;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x48u64)) as u64;
    rax = P0;
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax);
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rcx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rdx);
    rax = (crate::simd_ops::ru64(r8)) as u64;
    rdx = rsp.wrapping_add(0x20);
    rcx = r8;
    rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x48u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425319d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x143d01738; const P2: u64 = 0x143d01618; const P3: u64 = 0x38; const P4: u64 = 0x30; const P5: u64 = 0x20; const P6: u64 = 0x143d01610; const P7: u64 = 0x1433e0fd8; const P8: u64 = 0x14183ad52; const P9: u64 = 0x28;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdx = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; rdi = 0; fl_l = 0; fl_r = 0; rsi = (crate::simd_ops::ru32(P1)) as u32 as u64; rax = ((rdi as u32)) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; rax = (if (((fl_l as u32)) == ((fl_r as u32))) { rcx } else { rax }) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 9; continue; } bb = 1; continue; }
            1 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(P2)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx); rbx = ((rdi as u32)) as u32 as u64; rbx = (if (((fl_l as u32)) == ((fl_r as u32))) { rcx } else { rbx }) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(P3))) as u8 as u64; fl_r = ((rdi as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 8; continue; } bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(P4))) as u64; fl_r = (0x2u64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 8; continue; } bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P5))) as u64; rdx = ((rdi as u32)) as u32 as u64; rax = (crate::simd_ops::ru32(P6)) as u32 as u64; fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; rdx = (if (((fl_l as u32)) == ((fl_r as u32))) { rcx } else { rdx }) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 5; continue; }
            5 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P5))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 6; continue; }
            6 => { rdx = P7; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P9))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rdx = (crate::simd_ops::ru64(rcx)) as u64; fl_l = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; rdi = (if (((fl_l as u32)) == ((fl_r as u32))) { rdx } else { rdi }) as u64; rax = rdi; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            8 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = 0; fl_l = 0; fl_r = 0; bb = 9; continue; }
            9 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142531a80(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143425088; const P1: u64 = 0x1418350d0;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 23>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r11 = rsp; rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0xb0u64)) as u64; r8 = rdx; rax = P0; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rcx); xmm0 = 0u128; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128); xmm1 = 0u128; crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm1) as u128); rbx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(r11.wrapping_sub(0x60u64), rbx); crate::simd_ops::wu128(rsp.wrapping_add(0x60u64), (xmm0) as u128); crate::simd_ops::wu128(rsp.wrapping_add(0x70u64), (xmm1) as u128); crate::simd_ops::wu64(r11.wrapping_sub(0x30u64), rbx); crate::simd_ops::wu128(r11.wrapping_sub(0x28u64), (xmm0) as u128); crate::simd_ops::wu64(r11.wrapping_sub(0x18u64), rbx); rax = (crate::simd_ops::ru64(rdx)) as u64; rdx = rsp.wrapping_add(0x20); rcx = r8; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x88u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rsp.wrapping_add(0x88u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x90u64), rbx); bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rbx); bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 6; continue; }
            6 => { rsp = ((rsp).wrapping_add(0xb0u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142531b40(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x140000000; const P1: u64 = 0x14183ad9a; const P2: u64 = 0x142531f10;
    let (mut rax, mut fl_l, mut fl_r, mut j_tgt, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = (0xeu32) as u32 as u64; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 52; continue; } bb = 1; continue; }
            1 => { rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = ((((rdx as u32)) as i32 as i64) as u64) as u64; rbx = (rcx) as u64; rcx = ((P0) as u64) as u64; rdx = (crate::simd_ops::ru32(rcx.wrapping_add(rax.wrapping_mul(4)).wrapping_add(0x2531e48u64))) as u32 as u64; fl_l = ((rdx).wrapping_add(rcx)) as u64 as u64; fl_r = fl_l; rdx = ((fl_l as u64)) as u64; j_tgt = (rdx) as u64; bb = 0xffff_fffe; continue; }
            2 => { fl_l = (crate::simd_ops::ru32(r8)) as u32 as u64; fl_r = (0x3u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 51; continue; } bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru32(r9)) as u32 as u64; fl_r = (0x3u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 51; continue; } bb = 4; continue; }
            4 => { crate::simd_ops::wu32(rbx, (0x3u32) as u32); xmm0 = ((crate::simd_ops::ru64(r8.wrapping_add(0x8u64))) as u64) as u128; xmm1 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm0 as u64) + f64::from_bits(core::ptr::read_unaligned((r9.wrapping_add(0x8u64)) as *const u64))).to_bits() as u128); crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), ((xmm1 as u64)) as u64); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            5 => { fl_l = (crate::simd_ops::ru32(r8)) as u32 as u64; fl_r = (0x3u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 51; continue; } bb = 6; continue; }
            6 => { fl_l = (crate::simd_ops::ru32(r9)) as u32 as u64; fl_r = (0x3u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 51; continue; } bb = 7; continue; }
            7 => { crate::simd_ops::wu32(rbx, (0x3u32) as u32); xmm0 = ((crate::simd_ops::ru64(r8.wrapping_add(0x8u64))) as u64) as u128; xmm1 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm0 as u64) - f64::from_bits(core::ptr::read_unaligned((r9.wrapping_add(0x8u64)) as *const u64))).to_bits() as u128); crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), ((xmm1 as u64)) as u64); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            8 => { fl_l = (crate::simd_ops::ru32(r8)) as u32 as u64; fl_r = (0x3u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 51; continue; } bb = 9; continue; }
            9 => { fl_l = (crate::simd_ops::ru32(r9)) as u32 as u64; fl_r = (0x3u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 51; continue; } bb = 10; continue; }
            10 => { crate::simd_ops::wu32(rbx, (0x3u32) as u32); xmm0 = ((crate::simd_ops::ru64(r8.wrapping_add(0x8u64))) as u64) as u128; xmm1 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm0 as u64) * f64::from_bits(core::ptr::read_unaligned((r9.wrapping_add(0x8u64)) as *const u64))).to_bits() as u128); crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), ((xmm1 as u64)) as u64); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            11 => { fl_l = (crate::simd_ops::ru32(r8)) as u32 as u64; fl_r = (0x3u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 51; continue; } bb = 12; continue; }
            12 => { fl_l = (crate::simd_ops::ru32(r9)) as u32 as u64; fl_r = (0x3u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 51; continue; } bb = 13; continue; }
            13 => { crate::simd_ops::wu32(rbx, (0x3u32) as u32); xmm0 = ((crate::simd_ops::ru64(r8.wrapping_add(0x8u64))) as u64) as u128; xmm1 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm0 as u64) / f64::from_bits(core::ptr::read_unaligned((r9.wrapping_add(0x8u64)) as *const u64))).to_bits() as u128); crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), ((xmm1 as u64)) as u64); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            14 => { fl_l = (crate::simd_ops::ru32(r8)) as u32 as u64; fl_r = (0x3u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 51; continue; } bb = 15; continue; }
            15 => { fl_l = (crate::simd_ops::ru32(r9)) as u32 as u64; fl_r = (0x3u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 51; continue; } bb = 16; continue; }
            16 => { crate::simd_ops::wu32(rbx, (0x3u32) as u32); xmm3 = ((crate::simd_ops::ru64(r8.wrapping_add(0x8u64))) as u64) as u128; xmm0 = (xmm3 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm3 as u64) / f64::from_bits(core::ptr::read_unaligned((r9.wrapping_add(0x8u64)) as *const u64))).to_bits() as u128); xmm1 = 0u128; xmm1 = crate::simd_ops::vroundsd(xmm1, (xmm0 as u64), 0x1u8); xmm0 = (xmm1 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm1 as u64) * f64::from_bits(core::ptr::read_unaligned((r9.wrapping_add(0x8u64)) as *const u64))).to_bits() as u128); xmm1 = (xmm3 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm3 as u64) - f64::from_bits(xmm0 as u64)).to_bits() as u128); crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), ((xmm1 as u64)) as u64); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            17 => { fl_l = (crate::simd_ops::ru32(r8)) as u32 as u64; fl_r = (0x3u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 51; continue; } bb = 18; continue; }
            18 => { fl_l = (crate::simd_ops::ru32(r9)) as u32 as u64; fl_r = (0x3u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 51; continue; } bb = 19; continue; }
            19 => { crate::simd_ops::wu32(rbx, (0x3u32) as u32); xmm1 = ((crate::simd_ops::ru64(r9.wrapping_add(0x8u64))) as u64) as u128; xmm0 = ((crate::simd_ops::ru64(r8.wrapping_add(0x8u64))) as u64) as u128; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), ((xmm0 as u64)) as u64); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            20 => { fl_l = (crate::simd_ops::ru32(r8)) as u32 as u64; fl_r = (0x0u32) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 51; continue; } bb = 21; continue; }
            21 => { fl_l = (crate::simd_ops::ru32(r9)) as u32 as u64; fl_r = (0x0u32) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 51; continue; } bb = 22; continue; }
            22 => { rdx = (r9) as u64; crate::simd_ops::wu32(rbx, (0x2u32) as u32); rcx = (r8) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = (rax & !0xffu64) | ((((((rax as u8)) ^ (0x1u8))) as u8 as u64)); crate::simd_ops::wu8(rbx.wrapping_add(0x8u64), ((rax as u8)) as u8); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            23 => { fl_l = (crate::simd_ops::ru32(r8)) as u32 as u64; fl_r = (0x0u32) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 51; continue; } bb = 24; continue; }
            24 => { fl_l = (crate::simd_ops::ru32(r9)) as u32 as u64; fl_r = (0x0u32) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 51; continue; } bb = 25; continue; }
            25 => { rdx = (r9) as u64; crate::simd_ops::wu32(rbx, (0x2u32) as u32); rcx = (r8) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu8(rbx.wrapping_add(0x8u64), ((rax as u8)) as u8); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            26 => { fl_l = (crate::simd_ops::ru32(r8)) as u32 as u64; fl_r = (0x3u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 51; continue; } bb = 27; continue; }
            27 => { fl_l = (crate::simd_ops::ru32(r9)) as u32 as u64; fl_r = (0x3u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 51; continue; } bb = 28; continue; }
            28 => { crate::simd_ops::wu32(rbx, (0x2u32) as u32); xmm0 = ((crate::simd_ops::ru64(r9.wrapping_add(0x8u64))) as u64) as u128; fl_l = ((xmm0 as u64)) as u64; fl_r = (crate::simd_ops::ru64(r8.wrapping_add(0x8u64))) as u64; rax = (rax & !0xffu64) | (((if (f64::from_bits(fl_l) > f64::from_bits(fl_r)) { 1u8 } else { 0u8 }) as u8 as u64)); crate::simd_ops::wu8(rbx.wrapping_add(0x8u64), ((rax as u8)) as u8); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            29 => { fl_l = (crate::simd_ops::ru32(r8)) as u32 as u64; fl_r = (0x3u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 51; continue; } bb = 30; continue; }
            30 => { fl_l = (crate::simd_ops::ru32(r9)) as u32 as u64; fl_r = (0x3u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 51; continue; } bb = 31; continue; }
            31 => { crate::simd_ops::wu32(rbx, (0x2u32) as u32); xmm0 = ((crate::simd_ops::ru64(r9.wrapping_add(0x8u64))) as u64) as u128; fl_l = ((xmm0 as u64)) as u64; fl_r = (crate::simd_ops::ru64(r8.wrapping_add(0x8u64))) as u64; rax = (rax & !0xffu64) | (((if (f64::from_bits(fl_l) >= f64::from_bits(fl_r)) { 1u8 } else { 0u8 }) as u8 as u64)); crate::simd_ops::wu8(rbx.wrapping_add(0x8u64), ((rax as u8)) as u8); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            32 => { fl_l = (crate::simd_ops::ru32(r8)) as u32 as u64; fl_r = (0x3u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 51; continue; } bb = 33; continue; }
            33 => { fl_l = (crate::simd_ops::ru32(r9)) as u32 as u64; fl_r = (0x3u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 51; continue; } bb = 34; continue; }
            34 => { crate::simd_ops::wu32(rbx, (0x2u32) as u32); xmm0 = ((crate::simd_ops::ru64(r8.wrapping_add(0x8u64))) as u64) as u128; fl_l = ((xmm0 as u64)) as u64; fl_r = (crate::simd_ops::ru64(r9.wrapping_add(0x8u64))) as u64; rax = (rax & !0xffu64) | (((if (f64::from_bits(fl_l) > f64::from_bits(fl_r)) { 1u8 } else { 0u8 }) as u8 as u64)); crate::simd_ops::wu8(rbx.wrapping_add(0x8u64), ((rax as u8)) as u8); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            35 => { fl_l = (crate::simd_ops::ru32(r8)) as u32 as u64; fl_r = (0x3u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 51; continue; } bb = 36; continue; }
            36 => { fl_l = (crate::simd_ops::ru32(r9)) as u32 as u64; fl_r = (0x3u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 51; continue; } bb = 37; continue; }
            37 => { crate::simd_ops::wu32(rbx, (0x2u32) as u32); xmm0 = ((crate::simd_ops::ru64(r8.wrapping_add(0x8u64))) as u64) as u128; fl_l = ((xmm0 as u64)) as u64; fl_r = (crate::simd_ops::ru64(r9.wrapping_add(0x8u64))) as u64; rax = (rax & !0xffu64) | (((if (f64::from_bits(fl_l) >= f64::from_bits(fl_r)) { 1u8 } else { 0u8 }) as u8 as u64)); crate::simd_ops::wu8(rbx.wrapping_add(0x8u64), ((rax as u8)) as u8); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            38 => { rax = (crate::simd_ops::ru32(r8)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 51; continue; } bb = 39; continue; }
            39 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 43; continue; } bb = 40; continue; }
            40 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x2u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 42; continue; } bb = 41; continue; }
            41 => { fl_l = (crate::simd_ops::ru8(r8.wrapping_add(0x8u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 43; continue; } bb = 42; continue; }
            42 => { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; r8 = (if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { r9 } else { r8 }) as u64; xmm0 = (crate::simd_ops::ru128(r8)) as u128; crate::simd_ops::wu128(rbx, (xmm0) as u128); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            43 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; r8 = (if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { r9 } else { r8 }) as u64; xmm0 = (crate::simd_ops::ru128(r8)) as u128; crate::simd_ops::wu128(rbx, (xmm0) as u128); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            44 => { rax = (crate::simd_ops::ru32(r8)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 51; continue; } bb = 45; continue; }
            45 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 49; continue; } bb = 46; continue; }
            46 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x2u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 48; continue; } bb = 47; continue; }
            47 => { fl_l = (crate::simd_ops::ru8(r8.wrapping_add(0x8u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 49; continue; } bb = 48; continue; }
            48 => { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 50; continue; }
            49 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 50; continue; }
            50 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; r9 = (if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { r8 } else { r9 }) as u64; xmm0 = (crate::simd_ops::ru128(r9)) as u128; crate::simd_ops::wu128(rbx, (xmm0) as u128); bb = 51; continue; }
            51 => { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); bb = 52; continue; }
            52 => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            0xffff_fffe => { match j_tgt.wrapping_sub(0x142531b40u64) { 0x9 => { bb = 1; continue; } 0x27 => { bb = 2; continue; } 0x31 => { bb = 3; continue; } 0x3b => { bb = 4; continue; } 0x58 => { bb = 5; continue; } 0x62 => { bb = 6; continue; } 0x6c => { bb = 7; continue; } 0x89 => { bb = 8; continue; } 0x93 => { bb = 9; continue; } 0x9d => { bb = 10; continue; } 0xba => { bb = 11; continue; } 0xc4 => { bb = 12; continue; } 0xce => { bb = 13; continue; } 0xeb => { bb = 14; continue; } 0xf5 => { bb = 15; continue; } 0xff => { bb = 16; continue; } 0x130 => { bb = 17; continue; } 0x13a => { bb = 18; continue; } 0x144 => { bb = 19; continue; } 0x166 => { bb = 20; continue; } 0x170 => { bb = 21; continue; } 0x17a => { bb = 22; continue; } 0x196 => { bb = 23; continue; } 0x1a0 => { bb = 24; continue; } 0x1aa => { bb = 25; continue; } 0x1c4 => { bb = 26; continue; } 0x1ce => { bb = 27; continue; } 0x1d8 => { bb = 28; continue; } 0x1f6 => { bb = 29; continue; } 0x200 => { bb = 30; continue; } 0x20a => { bb = 31; continue; } 0x228 => { bb = 32; continue; } 0x232 => { bb = 33; continue; } 0x23c => { bb = 34; continue; } 0x25a => { bb = 35; continue; } 0x264 => { bb = 36; continue; } 0x26e => { bb = 37; continue; } 0x28c => { bb = 38; continue; } 0x293 => { bb = 39; continue; } 0x298 => { bb = 40; continue; } 0x29d => { bb = 41; continue; } 0x2a4 => { bb = 42; continue; } 0x2bb => { bb = 43; continue; } 0x2d2 => { bb = 44; continue; } 0x2d9 => { bb = 45; continue; } 0x2de => { bb = 46; continue; } 0x2e3 => { bb = 47; continue; } 0x2ea => { bb = 48; continue; } 0x2ee => { bb = 49; continue; } 0x2f0 => { bb = 50; continue; } 0x2ff => { bb = 51; continue; } 0x304 => { bb = 52; continue; } _ => { let s5 = crate::simd_ops::read_s5_tail(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x40) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(j_tgt, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::set_xmm_ret(c_x0); return c_rax; } } }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142531e90(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x4; const P1: u64 = 0x8; const P2: u64 = 0x143428740;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 7; continue; } bb = 1; continue; }
            1 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x1; rdx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 5; continue; } bb = 2; continue; }
            2 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 13; continue; } bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru32(r8)) as u32 as u64; fl_r = 0x4; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 13; continue; } bb = 4; continue; }
            4 => { crate::simd_ops::wu32(rcx, (0x3u32) as u32); rax = (crate::simd_ops::ru32(r8.wrapping_add(P0))) as u32 as u64; xmm0 = 0u128; xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((((rax) as i64) as f64).to_bits() as u128); crate::simd_ops::wu64(rcx.wrapping_add(P1), ((xmm0 as u64)) as u64); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            5 => { fl_l = (crate::simd_ops::ru32(r8)) as u32 as u64; fl_r = 0x3; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 13; continue; } bb = 6; continue; }
            6 => { crate::simd_ops::wu32(rcx, (0x3u32) as u32); xmm0 = ((crate::simd_ops::ru64(r8.wrapping_add(P1))) as u64) as u128; xmm0 = (xmm0) ^ (crate::simd_ops::ru128(P2)); crate::simd_ops::wu64(rcx.wrapping_add(P1), ((xmm0 as u64)) as u64); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            7 => { fl_l = (crate::simd_ops::ru32(r8)) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 13; continue; } bb = 8; continue; }
            8 => { crate::simd_ops::wu32(rcx, (0x2u32) as u32); rax = (crate::simd_ops::ru32(r8)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 12; continue; } bb = 9; continue; }
            9 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 11; continue; } bb = 10; continue; }
            10 => { fl_l = (crate::simd_ops::ru8(r8.wrapping_add(P1))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 12; continue; } bb = 11; continue; }
            11 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; crate::simd_ops::wu8(rcx.wrapping_add(P1), ((rax as u8)) as u8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            12 => { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); crate::simd_ops::wu8(rcx.wrapping_add(P1), ((rax as u8)) as u8); bb = 13; continue; }
            13 => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142531f10(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x4; const P1: u64 = 0x8; const P2: u64 = 0x14183acda;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x28u64)) as u64; r8 = (crate::simd_ops::ru32(rcx)) as u32 as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = 0x1; r8 = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 16; continue; } bb = 1; continue; }
            1 => { fl_l = ((r8 as u32)) as u32 as u64; fl_r = 0x1; r8 = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 12; continue; } bb = 2; continue; }
            2 => { fl_l = ((r8 as u32)) as u32 as u64; fl_r = 0x1; r8 = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 8; continue; } bb = 3; continue; }
            3 => { fl_l = ((r8 as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 15; continue; } bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::ru32(rdx)) as u32 as u64; fl_r = 0x4; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 15; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 15; continue; } bb = 6; continue; }
            6 => { rdx = (crate::simd_ops::ru64(rdx.wrapping_add(P1))) as u64; r8 = ((rax as u32)) as u32 as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 15; continue; } bb = 7; continue; }
            7 => { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); rsp = ((rsp).wrapping_add(0x28u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            8 => { fl_l = (crate::simd_ops::ru32(rdx)) as u32 as u64; fl_r = 0x3; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 15; continue; } bb = 9; continue; }
            9 => { xmm0 = ((crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64) as u128; fl_l = ((xmm0 as u64)) as u64; fl_r = (crate::simd_ops::ru64(rdx.wrapping_add(P1))) as u64; if (f64::from_bits(fl_l).is_nan() || f64::from_bits(fl_r).is_nan()) { bb = 15; continue; } bb = 10; continue; }
            10 => { if (f64::from_bits(fl_l) != f64::from_bits(fl_r) && !f64::from_bits(fl_l).is_nan() && !f64::from_bits(fl_r).is_nan()) { bb = 15; continue; } bb = 11; continue; }
            11 => { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); rsp = ((rsp).wrapping_add(0x28u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            12 => { fl_l = (crate::simd_ops::ru32(rdx)) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 15; continue; } bb = 13; continue; }
            13 => { rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P1))) as u32) as u32 as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(P1))) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 15; continue; } bb = 14; continue; }
            14 => { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); rsp = ((rsp).wrapping_add(0x28u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            15 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x28u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            16 => { fl_l = (crate::simd_ops::ru32(rdx)) as u32 as u64; fl_r = 0x1; rax = (rax & !0xffu64) | (((if (((fl_l as u32)) == ((fl_r as u32))) { 1u8 } else { 0u8 }) as u8 as u64)); rsp = ((rsp).wrapping_add(0x28u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142531fa0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143425258; const P1: u64 = 0x10; const P2: u64 = 0xfffffffffffffff8; const P3: u64 = 0x1426467f0; const P4: u64 = 0x141834d30;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 15>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x78u64)) as u64; rax = P0; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rcx); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rdx); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), r8); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), r9); xmm0 = 0u128; crate::simd_ops::wu128(rsp.wrapping_add(0x50u64), (xmm0) as u128); crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), (0x0u64) as u64); fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru64(r8.wrapping_add(P1))) as u64; fl_r = (0x0u64) as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x48u64), (0x1u8) as u8); if (((fl_l as u64)) == ((fl_r as u64))) { bb = 3; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu8(rsp.wrapping_add(0x48u64), (0x0u8) as u8); bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0xa0u64))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = rsp.wrapping_add(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 4; continue; }
            4 => { rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rdx = ((rdx).wrapping_sub(rcx)) as u64; rdx = (((rdx) & (0xfffffffffffffff0u64))) as u64; rax = rcx; fl_l = rdx; fl_r = (0x1000u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 7; continue; } bb = 5; continue; }
            5 => { rdx = ((rdx).wrapping_add(0x27u64)) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64; rax = ((rax).wrapping_sub(rcx)) as u64; rax = ((rax).wrapping_add(0xfffffffffffffff8u64)) as u64; fl_l = rax; fl_r = (0x1fu64) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 7; continue; } bb = 6; continue; }
            6 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, 0x14253203f, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            7 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 8; continue; }
            8 => { rsp = ((rsp).wrapping_add(0x78u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142532050(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rax = (crate::simd_ops::cvt_f64_i64((f64::from_bits(xmm0 as u64)))) as u64;
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142532060(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x14336adcc; const P2: u64 = 0x4; const P3: u64 = 0x14336adb0; const P4: u64 = 0x1433dfc78; const P5: u64 = 0x14336f5a4;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r8 = (crate::simd_ops::ru32(rdx)) as u32 as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = 0x1; r8 = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 8; continue; } bb = 1; continue; }
            1 => { fl_l = ((r8 as u32)) as u32 as u64; fl_r = 0x1; r8 = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 7; continue; } bb = 2; continue; }
            2 => { fl_l = ((r8 as u32)) as u32 as u64; fl_r = 0x1; r8 = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 6; continue; } bb = 3; continue; }
            3 => { fl_l = ((r8 as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rcx, rax); crate::simd_ops::wu64(rcx.wrapping_add(P0), rax); rax = rcx; return rax; }
            5 => { rax = P1; crate::simd_ops::wu32(rcx.wrapping_add(P2), (0x6u32) as u32); crate::simd_ops::wu32(rcx, (0x4u32) as u32); crate::simd_ops::wu64(rcx.wrapping_add(P0), rax); rax = rcx; return rax; }
            6 => { rax = P3; crate::simd_ops::wu32(rcx.wrapping_add(P2), (0x6u32) as u32); crate::simd_ops::wu32(rcx, (0x4u32) as u32); crate::simd_ops::wu64(rcx.wrapping_add(P0), rax); rax = rcx; return rax; }
            7 => { rax = P4; crate::simd_ops::wu32(rcx.wrapping_add(P2), (0x7u32) as u32); crate::simd_ops::wu32(rcx, (0x4u32) as u32); crate::simd_ops::wu64(rcx.wrapping_add(P0), rax); rax = rcx; return rax; }
            8 => { rax = P5; crate::simd_ops::wu32(rcx.wrapping_add(P2), (0x3u32) as u32); crate::simd_ops::wu32(rcx, (0x4u32) as u32); crate::simd_ops::wu64(rcx.wrapping_add(P0), rax); rax = rcx; return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142532100(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x4; const P1: u64 = 0x8;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    crate::simd_ops::wu32(rcx, (0x4u32) as u32);
    rax = (0xffffffffffffffffu64) as u64;
    loop { rax = ((rax).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(rax))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if !((((fl_l as u8)) != ((fl_r as u8)))) { break; } }
    crate::simd_ops::wu32(rcx.wrapping_add(P0), ((rax as u32)) as u32);
    rax = rcx;
    crate::simd_ops::wu64(rcx.wrapping_add(P1), rdx);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142532130(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    crate::simd_ops::wu64(rcx.wrapping_add(P0), ((xmm1 as u64)) as u64);
    crate::simd_ops::wu64(rcx, (0x3u64) as u64);
    rax = rcx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142532150(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rcx, (0x2u64) as u64);
    crate::simd_ops::wu64(rcx.wrapping_add(P0), rax);
    rax = rcx;
    crate::simd_ops::wu8(rcx.wrapping_add(P0), ((rdx as u8)) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142532190(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x140000000; const P1: u64 = 0x143428730; const P2: u64 = 0x1425476ac; const P3: u64 = 0x1425476b2; const P4: u64 = 0x14183adb2; const P5: u64 = 0x1425476b8; const P6: u64 = 0x1425476be; const P7: u64 = 0x14183ad6a; const P8: u64 = 0x143426920; const P9: u64 = 0x14183ad70; const P10: u64 = 0x1425476c4; const P11: u64 = 0x142646720; const P12: u64 = 0x14183ad5e; const P13: u64 = 0x14183ad7c; const P14: u64 = 0x143426ac0; const P15: u64 = 0x1425476ca; const P16: u64 = 0x143426b90; const P17: u64 = 0x14183ad9a; const P18: u64 = 0x1425476d6; const P19: u64 = 0x14183adb8; const P20: u64 = 0x1425476dc; const P21: u64 = 0x1425476e2; const P22: u64 = 0x14183adbe; const P23: u64 = 0x142532060; const P24: u64 = 0x143426a48; const P25: u64 = 0x143427098; const P26: u64 = 0x1426466c8;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut j_tgt, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rdx = (((rdx as u32)).wrapping_add(0xfffffffeu32)) as u32 as u64; rdi = (r8) as u64; rbx = (rcx) as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = (0x2eu32) as u32 as u64; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 209; continue; } bb = 1; continue; }
            1 => { rax = ((((rdx as u32)) as i32 as i64) as u64) as u64; rcx = ((P0) as u64) as u64; rdx = (crate::simd_ops::ru32(rcx.wrapping_add(rax.wrapping_mul(4)).wrapping_add(0x2532eacu64))) as u32 as u64; fl_l = ((rdx).wrapping_add(rcx)) as u64 as u64; fl_r = fl_l; rdx = ((fl_l as u64)) as u64; j_tgt = (rdx) as u64; bb = 0xffff_fffe; continue; }
            2 => { fl_l = (r9) as u64 as u64; fl_r = (0x1u64) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 209; continue; } bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru32(r8)) as u32 as u64; fl_r = (0x3u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 209; continue; } bb = 4; continue; }
            4 => { xmm0 = ((crate::simd_ops::ru64(r8.wrapping_add(0x8u64))) as u64) as u128; xmm0 = (xmm0) & (crate::simd_ops::ru128(P1)); crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), ((xmm0 as u64)) as u64); crate::simd_ops::wu64(rbx, (0x3u64) as u64); rax = (rbx) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            5 => { fl_l = (r9) as u64 as u64; fl_r = (0x1u64) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 209; continue; } bb = 6; continue; }
            6 => { fl_l = (crate::simd_ops::ru32(r8)) as u32 as u64; fl_r = (0x3u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 209; continue; } bb = 7; continue; }
            7 => { xmm0 = ((crate::simd_ops::ru64(r8.wrapping_add(0x8u64))) as u64) as u128; crate::simd_ops::wu64(rbx, (0x3u64) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), ((xmm0 as u64)) as u64); rax = (rbx) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            8 => { fl_l = (r9) as u64 as u64; fl_r = (0x1u64) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 209; continue; } bb = 9; continue; }
            9 => { fl_l = (crate::simd_ops::ru32(r8)) as u32 as u64; fl_r = (0x3u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 209; continue; } bb = 10; continue; }
            10 => { xmm0 = ((crate::simd_ops::ru64(r8.wrapping_add(0x8u64))) as u64) as u128; crate::simd_ops::wu64(rbx, (0x3u64) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), ((xmm0 as u64)) as u64); rax = (rbx) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            11 => { fl_l = (r9) as u64 as u64; fl_r = (0x2u64) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 209; continue; } bb = 12; continue; }
            12 => { fl_l = (crate::simd_ops::ru32(r8)) as u32 as u64; fl_r = (0x3u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 209; continue; } bb = 13; continue; }
            13 => { fl_l = (crate::simd_ops::ru32(r8.wrapping_add(0x10u64))) as u32 as u64; fl_r = (0x3u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 209; continue; } bb = 14; continue; }
            14 => { xmm1 = ((crate::simd_ops::ru64(r8.wrapping_add(0x18u64))) as u64) as u128; xmm0 = ((crate::simd_ops::ru64(r8.wrapping_add(0x8u64))) as u64) as u128; crate::simd_ops::wu64(rbx, (0x3u64) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), ((xmm0 as u64)) as u64); rax = (rbx) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            15 => { fl_l = (r9) as u64 as u64; fl_r = (0x1u64) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 209; continue; } bb = 16; continue; }
            16 => { fl_l = (crate::simd_ops::ru32(r8)) as u32 as u64; fl_r = (0x3u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 209; continue; } bb = 17; continue; }
            17 => { xmm0 = ((crate::simd_ops::ru64(r8.wrapping_add(0x8u64))) as u64) as u128; crate::simd_ops::wu64(rbx, (0x3u64) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), ((xmm0 as u64)) as u64); rax = (rbx) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            18 => { fl_l = (r9) as u64 as u64; fl_r = (0x1u64) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 209; continue; } bb = 19; continue; }
            19 => { fl_l = (crate::simd_ops::ru32(r8)) as u32 as u64; fl_r = (0x3u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 209; continue; } bb = 20; continue; }
            20 => { xmm0 = 0u128; xmm0 = crate::simd_ops::vroundsd(xmm0, crate::simd_ops::ru64(r8.wrapping_add(0x8u64)), 0x2u8); crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), ((xmm0 as u64)) as u64); crate::simd_ops::wu64(rbx, (0x3u64) as u64); rax = (rbx) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            21 => { fl_l = (r9) as u64 as u64; fl_r = (0x1u64) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 209; continue; } bb = 22; continue; }
            22 => { fl_l = (crate::simd_ops::ru32(r8)) as u32 as u64; fl_r = (0x3u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 209; continue; } bb = 23; continue; }
            23 => { xmm0 = ((crate::simd_ops::ru64(r8.wrapping_add(0x8u64))) as u64) as u128; crate::simd_ops::wu64(rbx, (0x3u64) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), ((xmm0 as u64)) as u64); rax = (rbx) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            24 => { fl_l = (r9) as u64 as u64; fl_r = (0x1u64) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 209; continue; } bb = 25; continue; }
            25 => { fl_l = (crate::simd_ops::ru32(r8)) as u32 as u64; fl_r = (0x3u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 209; continue; } bb = 26; continue; }
            26 => { xmm0 = ((crate::simd_ops::ru64(r8.wrapping_add(0x8u64))) as u64) as u128; crate::simd_ops::wu64(rbx, (0x3u64) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), ((xmm0 as u64)) as u64); rax = (rbx) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            27 => { fl_l = (r9) as u64 as u64; fl_r = (0x1u64) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 209; continue; } bb = 28; continue; }
            28 => { fl_l = (crate::simd_ops::ru32(r8)) as u32 as u64; fl_r = (0x3u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 209; continue; } bb = 29; continue; }
            29 => { xmm0 = ((crate::simd_ops::ru64(r8.wrapping_add(0x8u64))) as u64) as u128; xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm0 as u64) / f64::from_bits(core::ptr::read_unaligned((P8) as *const u64))).to_bits() as u128); crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), ((xmm0 as u64)) as u64); crate::simd_ops::wu64(rbx, (0x3u64) as u64); rax = (rbx) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            30 => { fl_l = (r9) as u64 as u64; fl_r = (0x1u64) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 209; continue; } bb = 31; continue; }
            31 => { fl_l = (crate::simd_ops::ru32(r8)) as u32 as u64; fl_r = (0x3u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 209; continue; } bb = 32; continue; }
            32 => { xmm0 = ((crate::simd_ops::ru64(r8.wrapping_add(0x8u64))) as u64) as u128; crate::simd_ops::wu64(rbx, (0x3u64) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), ((xmm0 as u64)) as u64); rax = (rbx) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            33 => { fl_l = (r9) as u64 as u64; fl_r = (0x1u64) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 209; continue; } bb = 34; continue; }
            34 => { fl_l = (crate::simd_ops::ru32(r8)) as u32 as u64; fl_r = (0x3u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 209; continue; } bb = 35; continue; }
            35 => { xmm0 = 0u128; xmm0 = crate::simd_ops::vroundsd(xmm0, crate::simd_ops::ru64(r8.wrapping_add(0x8u64)), 0x1u8); crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), ((xmm0 as u64)) as u64); crate::simd_ops::wu64(rbx, (0x3u64) as u64); rax = (rbx) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            36 => { fl_l = (r9) as u64 as u64; fl_r = (0x2u64) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 209; continue; } bb = 37; continue; }
            37 => { fl_l = (crate::simd_ops::ru32(r8)) as u32 as u64; fl_r = (0x3u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 209; continue; } bb = 38; continue; }
            38 => { fl_l = (crate::simd_ops::ru32(r8.wrapping_add(0x10u64))) as u32 as u64; fl_r = (0x3u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 209; continue; } bb = 39; continue; }
            39 => { xmm1 = ((crate::simd_ops::ru64(r8.wrapping_add(0x18u64))) as u64) as u128; xmm0 = ((crate::simd_ops::ru64(r8.wrapping_add(0x8u64))) as u64) as u128; crate::simd_ops::wu64(rbx, (0x3u64) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P10) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), ((xmm0 as u64)) as u64); rax = (rbx) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            40 => { fl_l = (r9) as u64 as u64; fl_r = (0x2u64) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 209; continue; } bb = 41; continue; }
            41 => { fl_l = (crate::simd_ops::ru32(r8)) as u32 as u64; fl_r = (0x3u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 209; continue; } bb = 42; continue; }
            42 => { fl_l = (crate::simd_ops::ru32(r8.wrapping_add(0x10u64))) as u32 as u64; fl_r = (0x3u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 209; continue; } bb = 43; continue; }
            43 => { rdx = (crate::simd_ops::cvt_f64_i32((f64::from_bits(core::ptr::read_unaligned((r8.wrapping_add(0x18u64)) as *const u64))))) as u32 as u64; xmm0 = ((crate::simd_ops::ru64(r8.wrapping_add(0x8u64))) as u64) as u128; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P11)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu64(rbx, (0x3u64) as u64); rax = (rbx) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), ((xmm0 as u64)) as u64); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            44 => { fl_l = (r9) as u64 as u64; fl_r = (0x1u64) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 209; continue; } bb = 45; continue; }
            45 => { fl_l = (crate::simd_ops::ru32(r8)) as u32 as u64; fl_r = (0x3u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 209; continue; } bb = 46; continue; }
            46 => { xmm0 = ((crate::simd_ops::ru64(r8.wrapping_add(0x8u64))) as u64) as u128; crate::simd_ops::wu64(rbx, (0x3u64) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P12) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), ((xmm0 as u64)) as u64); rax = (rbx) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            47 => { fl_l = (r9) as u64 as u64; fl_r = (0x1u64) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 50; continue; } bb = 48; continue; }
            48 => { fl_l = (crate::simd_ops::ru32(r8)) as u32 as u64; fl_r = (0x3u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 209; continue; } bb = 49; continue; }
            49 => { xmm0 = ((crate::simd_ops::ru64(r8.wrapping_add(0x8u64))) as u64) as u128; crate::simd_ops::wu64(rbx, (0x3u64) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P13) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), ((xmm0 as u64)) as u64); rax = (rbx) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            50 => { fl_l = (r9) as u64 as u64; fl_r = (0x2u64) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 209; continue; } bb = 51; continue; }
            51 => { fl_l = (crate::simd_ops::ru32(r8)) as u32 as u64; fl_r = (0x3u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 209; continue; } bb = 52; continue; }
            52 => { fl_l = (crate::simd_ops::ru32(r8.wrapping_add(0x10u64))) as u32 as u64; fl_r = (0x3u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 209; continue; } bb = 53; continue; }
            53 => { xmm1 = ((crate::simd_ops::ru64(r8.wrapping_add(0x18u64))) as u64) as u128; crate::simd_ops::wu32(rbx, (0x3u32) as u32); fl_l = ((xmm1 as u64)) as u64; fl_r = (crate::simd_ops::ru64(P14)) as u64; xmm0 = ((crate::simd_ops::ru64(r8.wrapping_add(0x8u64))) as u64) as u128; if (f64::from_bits(fl_l).is_nan() || f64::from_bits(fl_r).is_nan()) { bb = 56; continue; } bb = 54; continue; }
            54 => { if (f64::from_bits(fl_l) != f64::from_bits(fl_r) && !f64::from_bits(fl_l).is_nan() && !f64::from_bits(fl_r).is_nan()) { bb = 56; continue; } bb = 55; continue; }
            55 => { rax = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rbx.wrapping_add(0x4u64), ((rax as u32)) as u32); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P15) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), ((xmm0 as u64)) as u64); rax = (rbx) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            56 => { fl_l = ((xmm1 as u64)) as u64; fl_r = (crate::simd_ops::ru64(P16)) as u64; if (f64::from_bits(fl_l).is_nan() || f64::from_bits(fl_r).is_nan()) { bb = 59; continue; } bb = 57; continue; }
            57 => { if (f64::from_bits(fl_l) != f64::from_bits(fl_r) && !f64::from_bits(fl_l).is_nan() && !f64::from_bits(fl_r).is_nan()) { bb = 59; continue; } bb = 58; continue; }
            58 => { rax = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rbx.wrapping_add(0x4u64), ((rax as u32)) as u32); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P12) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), ((xmm0 as u64)) as u64); rax = (rbx) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            59 => { rax = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm6) as u128); crate::simd_ops::wu32(rbx.wrapping_add(0x4u64), ((rax as u32)) as u32); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P13) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; xmm6 = (xmm0) as u128; xmm0 = ((crate::simd_ops::ru64(rdi.wrapping_add(0x18u64))) as u64) as u128; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P13) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; xmm1 = (xmm6 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm6 as u64) / f64::from_bits(xmm0 as u64)).to_bits() as u128); xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; rax = (rbx) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), ((xmm1 as u64)) as u64); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            60 => { fl_l = (r9) as u64 as u64; fl_r = (0x1u64) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 209; continue; } bb = 61; continue; }
            61 => { fl_l = (crate::simd_ops::ru32(r8)) as u32 as u64; fl_r = (0x3u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 209; continue; } bb = 62; continue; }
            62 => { xmm1 = ((crate::simd_ops::ru64(r8.wrapping_add(0x8u64))) as u64) as u128; rcx = (0x1u32) as u32 as u64; fl_l = (rcx) as u64 as u64; fl_r = (r9) as u64 as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 66; continue; } bb = 63; continue; }
            63 => { fl_l = ((rdi).wrapping_add(0x10u64)) as u64 as u64; fl_r = fl_l; rdi = ((fl_l as u64)) as u64; bb = 64; continue; }
            64 => { fl_l = (crate::simd_ops::ru32(rdi)) as u32 as u64; fl_r = (0x3u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 209; continue; } bb = 65; continue; }
            65 => { xmm0 = ((crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64) as u128; rdi = ((rdi).wrapping_add(0x10u64)) as u64; rcx = ((rcx).wrapping_add(1)) as u64; xmm1 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((if f64::from_bits(xmm0 as u64) > f64::from_bits(xmm1 as u64) { f64::from_bits(xmm0 as u64) } else { f64::from_bits(xmm1 as u64) }).to_bits() as u128); fl_l = (rcx) as u64 as u64; fl_r = (r9) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 64; continue; } bb = 66; continue; }
            66 => { crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), ((xmm1 as u64)) as u64); crate::simd_ops::wu64(rbx, (0x3u64) as u64); rax = (rbx) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            67 => { fl_l = (r9) as u64 as u64; fl_r = (0x1u64) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 209; continue; } bb = 68; continue; }
            68 => { fl_l = (crate::simd_ops::ru32(r8)) as u32 as u64; fl_r = (0x3u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 209; continue; } bb = 69; continue; }
            69 => { xmm1 = ((crate::simd_ops::ru64(r8.wrapping_add(0x8u64))) as u64) as u128; rcx = (0x1u32) as u32 as u64; fl_l = (rcx) as u64 as u64; fl_r = (r9) as u64 as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 66; continue; } bb = 70; continue; }
            70 => { fl_l = ((rdi).wrapping_add(0x10u64)) as u64 as u64; fl_r = fl_l; rdi = ((fl_l as u64)) as u64; bb = 71; continue; }
            71 => { fl_l = (crate::simd_ops::ru32(rdi)) as u32 as u64; fl_r = (0x3u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 209; continue; } bb = 72; continue; }
            72 => { xmm0 = ((crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64) as u128; rdi = ((rdi).wrapping_add(0x10u64)) as u64; rcx = ((rcx).wrapping_add(1)) as u64; xmm1 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((if f64::from_bits(xmm0 as u64) < f64::from_bits(xmm1 as u64) { f64::from_bits(xmm0 as u64) } else { f64::from_bits(xmm1 as u64) }).to_bits() as u128); fl_l = (rcx) as u64 as u64; fl_r = (r9) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 71; continue; } bb = 73; continue; }
            73 => { crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), ((xmm1 as u64)) as u64); crate::simd_ops::wu64(rbx, (0x3u64) as u64); rax = (rbx) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            74 => { fl_l = (r9) as u64 as u64; fl_r = (0x2u64) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 209; continue; } bb = 75; continue; }
            75 => { fl_l = (crate::simd_ops::ru32(r8)) as u32 as u64; fl_r = (0x3u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 209; continue; } bb = 76; continue; }
            76 => { fl_l = (crate::simd_ops::ru32(r8.wrapping_add(0x10u64))) as u32 as u64; fl_r = (0x3u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 209; continue; } bb = 77; continue; }
            77 => { xmm1 = ((crate::simd_ops::ru64(r8.wrapping_add(0x18u64))) as u64) as u128; xmm0 = ((crate::simd_ops::ru64(r8.wrapping_add(0x8u64))) as u64) as u128; crate::simd_ops::wu64(rbx, (0x3u64) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P17) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), ((xmm0 as u64)) as u64); rax = (rbx) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            78 => { fl_l = (r9) as u64 as u64; fl_r = (0x1u64) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 209; continue; } bb = 79; continue; }
            79 => { fl_l = (crate::simd_ops::ru32(r8)) as u32 as u64; fl_r = (0x3u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 209; continue; } bb = 80; continue; }
            80 => { xmm0 = ((crate::simd_ops::ru64(r8.wrapping_add(0x8u64))) as u64) as u128; xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm0 as u64) * f64::from_bits(core::ptr::read_unaligned((P8) as *const u64))).to_bits() as u128); crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), ((xmm0 as u64)) as u64); crate::simd_ops::wu64(rbx, (0x3u64) as u64); rax = (rbx) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            81 => { fl_l = (r9) as u64 as u64; fl_r = (0x1u64) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 209; continue; } bb = 82; continue; }
            82 => { fl_l = (crate::simd_ops::ru32(r8)) as u32 as u64; fl_r = (0x3u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 209; continue; } bb = 83; continue; }
            83 => { xmm0 = ((crate::simd_ops::ru64(r8.wrapping_add(0x8u64))) as u64) as u128; crate::simd_ops::wu64(rbx, (0x3u64) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P18) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), ((xmm0 as u64)) as u64); rax = (rbx) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            84 => { fl_l = (r9) as u64 as u64; fl_r = (0x1u64) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 209; continue; } bb = 85; continue; }
            85 => { fl_l = (crate::simd_ops::ru32(r8)) as u32 as u64; fl_r = (0x3u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 209; continue; } bb = 86; continue; }
            86 => { xmm0 = ((crate::simd_ops::ru64(r8.wrapping_add(0x8u64))) as u64) as u128; crate::simd_ops::wu64(rbx, (0x3u64) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P19) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), ((xmm0 as u64)) as u64); rax = (rbx) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            87 => { fl_l = (r9) as u64 as u64; fl_r = (0x1u64) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 209; continue; } bb = 88; continue; }
            88 => { fl_l = (crate::simd_ops::ru32(r8)) as u32 as u64; fl_r = (0x3u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 209; continue; } bb = 89; continue; }
            89 => { xmm1 = ((crate::simd_ops::ru64(r8.wrapping_add(0x8u64))) as u64) as u128; xmm0 = 0u128; fl_l = ((xmm0 as u64)) as u64; fl_r = ((xmm1 as u64)) as u64; crate::simd_ops::wu64(rbx, (0x3u64) as u64); if (f64::from_bits(fl_l) > f64::from_bits(fl_r)) { bb = 91; continue; } bb = 90; continue; }
            90 => { xmm0 = (xmm1 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm1 as u64)).sqrt().to_bits() as u128); crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), ((xmm0 as u64)) as u64); rax = (rbx) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            91 => { xmm0 = ((crate::simd_ops::ru64(r8.wrapping_add(0x8u64))) as u64) as u128; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P20) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), ((xmm0 as u64)) as u64); rax = (rbx) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            92 => { fl_l = (r9) as u64 as u64; fl_r = (0x1u64) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 209; continue; } bb = 93; continue; }
            93 => { fl_l = (crate::simd_ops::ru32(r8)) as u32 as u64; fl_r = (0x3u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 209; continue; } bb = 94; continue; }
            94 => { xmm0 = ((crate::simd_ops::ru64(r8.wrapping_add(0x8u64))) as u64) as u128; crate::simd_ops::wu64(rbx, (0x3u64) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P21) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), ((xmm0 as u64)) as u64); rax = (rbx) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            95 => { fl_l = (r9) as u64 as u64; fl_r = (0x1u64) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 209; continue; } bb = 96; continue; }
            96 => { fl_l = (crate::simd_ops::ru32(r8)) as u32 as u64; fl_r = (0x3u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 209; continue; } bb = 97; continue; }
            97 => { xmm0 = ((crate::simd_ops::ru64(r8.wrapping_add(0x8u64))) as u64) as u128; crate::simd_ops::wu64(rbx, (0x3u64) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P22) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), ((xmm0 as u64)) as u64); rax = (rbx) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            98 => { fl_l = (r9) as u64 as u64; fl_r = (0x2u64) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 209; continue; } bb = 99; continue; }
            99 => { fl_l = (crate::simd_ops::ru32(r8)) as u32 as u64; fl_r = (0x3u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 209; continue; } bb = 100; continue; }
            100 => { fl_l = (crate::simd_ops::ru32(r8.wrapping_add(0x10u64))) as u32 as u64; fl_r = (0x3u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 209; continue; } bb = 101; continue; }
            101 => { rcx = (crate::simd_ops::cvt_f64_i32((f64::from_bits(core::ptr::read_unaligned((r8.wrapping_add(0x18u64)) as *const u64))))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = (0x20u32) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 209; continue; } bb = 102; continue; }
            102 => { rax = (crate::simd_ops::cvt_f64_i64((f64::from_bits(core::ptr::read_unaligned((r8.wrapping_add(0x8u64)) as *const u64))))) as u64; rax = (((((rax as u32)) as i32).wrapping_shr(((((rcx as u32)) as u32) & 31)) as u32)) as u32 as u64; xmm0 = 0u128; xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((((rax) as i64) as f64).to_bits() as u128); crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), ((xmm0 as u64)) as u64); crate::simd_ops::wu64(rbx, (0x3u64) as u64); rax = (rbx) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            103 => { fl_l = (r9) as u64 as u64; fl_r = (0x1u64) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 209; continue; } bb = 104; continue; }
            104 => { fl_l = (crate::simd_ops::ru32(r8)) as u32 as u64; fl_r = (0x3u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 209; continue; } bb = 105; continue; }
            105 => { rdx = (crate::simd_ops::cvt_f64_i64((f64::from_bits(core::ptr::read_unaligned((r8.wrapping_add(0x8u64)) as *const u64))))) as u64; rcx = (0x1u32) as u32 as u64; fl_l = (rcx) as u64 as u64; fl_r = (r9) as u64 as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 109; continue; } bb = 106; continue; }
            106 => { fl_l = ((rdi).wrapping_add(0x10u64)) as u64 as u64; fl_r = fl_l; rdi = ((fl_l as u64)) as u64; bb = 107; continue; }
            107 => { fl_l = (crate::simd_ops::ru32(rdi)) as u32 as u64; fl_r = (0x3u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 209; continue; } bb = 108; continue; }
            108 => { rax = (crate::simd_ops::cvt_f64_i64((f64::from_bits(core::ptr::read_unaligned((rdi.wrapping_add(0x8u64)) as *const u64))))) as u64; rdx = ((((rdx as u32)) & ((rax as u32)))) as u32 as u64; rcx = ((rcx).wrapping_add(1)) as u64; rdi = ((rdi).wrapping_add(0x10u64)) as u64; fl_l = (rcx) as u64 as u64; fl_r = (r9) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 107; continue; } bb = 109; continue; }
            109 => { rax = ((rdx as u32)) as u32 as u64; bb = 110; continue; }
            110 => { xmm0 = 0u128; xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((((rax) as i64) as f64).to_bits() as u128); bb = 111; continue; }
            111 => { crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), ((xmm0 as u64)) as u64); crate::simd_ops::wu64(rbx, (0x3u64) as u64); rax = (rbx) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            112 => { fl_l = (r9) as u64 as u64; fl_r = (0x1u64) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 209; continue; } bb = 113; continue; }
            113 => { fl_l = (crate::simd_ops::ru32(r8)) as u32 as u64; fl_r = (0x3u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 209; continue; } bb = 114; continue; }
            114 => { rax = (crate::simd_ops::cvt_f64_i64((f64::from_bits(core::ptr::read_unaligned((r8.wrapping_add(0x8u64)) as *const u64))))) as u64; rax = (!((rax as u32))) as u32 as u64; crate::simd_ops::wu64(rbx, (0x3u64) as u64); xmm0 = 0u128; xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((((rax) as i64) as f64).to_bits() as u128); crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), ((xmm0 as u64)) as u64); rax = (rbx) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            115 => { fl_l = (r9) as u64 as u64; fl_r = (0x1u64) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 209; continue; } bb = 116; continue; }
            116 => { fl_l = (crate::simd_ops::ru32(r8)) as u32 as u64; fl_r = (0x3u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 209; continue; } bb = 117; continue; }
            117 => { rdx = (crate::simd_ops::cvt_f64_i64((f64::from_bits(core::ptr::read_unaligned((r8.wrapping_add(0x8u64)) as *const u64))))) as u64; rcx = (0x1u32) as u32 as u64; fl_l = (rcx) as u64 as u64; fl_r = (r9) as u64 as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 109; continue; } bb = 118; continue; }
            118 => { fl_l = ((rdi).wrapping_add(0x10u64)) as u64 as u64; fl_r = fl_l; rdi = ((fl_l as u64)) as u64; bb = 119; continue; }
            119 => { fl_l = (crate::simd_ops::ru32(rdi)) as u32 as u64; fl_r = (0x3u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 209; continue; } bb = 120; continue; }
            120 => { rax = (crate::simd_ops::cvt_f64_i64((f64::from_bits(core::ptr::read_unaligned((rdi.wrapping_add(0x8u64)) as *const u64))))) as u64; rdx = ((((rdx as u32)) | ((rax as u32)))) as u32 as u64; rcx = ((rcx).wrapping_add(1)) as u64; rdi = ((rdi).wrapping_add(0x10u64)) as u64; fl_l = (rcx) as u64 as u64; fl_r = (r9) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 119; continue; } bb = 121; continue; }
            121 => { bb = 109; continue; }
            122 => { fl_l = (r9) as u64 as u64; fl_r = (0x1u64) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 209; continue; } bb = 123; continue; }
            123 => { fl_l = (crate::simd_ops::ru32(r8)) as u32 as u64; fl_r = (0x3u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 209; continue; } bb = 124; continue; }
            124 => { rdx = (crate::simd_ops::cvt_f64_i64((f64::from_bits(core::ptr::read_unaligned((r8.wrapping_add(0x8u64)) as *const u64))))) as u64; rcx = (0x1u32) as u32 as u64; fl_l = (rcx) as u64 as u64; fl_r = (r9) as u64 as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 109; continue; } bb = 125; continue; }
            125 => { fl_l = ((rdi).wrapping_add(0x10u64)) as u64 as u64; fl_r = fl_l; rdi = ((fl_l as u64)) as u64; bb = 126; continue; }
            126 => { fl_l = (crate::simd_ops::ru32(rdi)) as u32 as u64; fl_r = (0x3u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 209; continue; } bb = 127; continue; }
            127 => { rax = (crate::simd_ops::cvt_f64_i64((f64::from_bits(core::ptr::read_unaligned((rdi.wrapping_add(0x8u64)) as *const u64))))) as u64; rdx = ((((rdx as u32)) ^ ((rax as u32)))) as u32 as u64; rcx = ((rcx).wrapping_add(1)) as u64; rdi = ((rdi).wrapping_add(0x10u64)) as u64; fl_l = (rcx) as u64 as u64; fl_r = (r9) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 126; continue; } bb = 128; continue; }
            128 => { bb = 109; continue; }
            129 => { fl_l = (r9) as u64 as u64; fl_r = (0x1u64) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 209; continue; } bb = 130; continue; }
            130 => { fl_l = (crate::simd_ops::ru32(r8)) as u32 as u64; fl_r = (0x3u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 209; continue; } bb = 131; continue; }
            131 => { rdx = (crate::simd_ops::cvt_f64_i64((f64::from_bits(core::ptr::read_unaligned((r8.wrapping_add(0x8u64)) as *const u64))))) as u64; rcx = (0x1u32) as u32 as u64; fl_l = (rcx) as u64 as u64; fl_r = (r9) as u64 as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 135; continue; } bb = 132; continue; }
            132 => { fl_l = ((rdi).wrapping_add(0x10u64)) as u64 as u64; fl_r = fl_l; rdi = ((fl_l as u64)) as u64; bb = 133; continue; }
            133 => { fl_l = (crate::simd_ops::ru32(rdi)) as u32 as u64; fl_r = (0x3u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 209; continue; } bb = 134; continue; }
            134 => { rax = (crate::simd_ops::cvt_f64_i64((f64::from_bits(core::ptr::read_unaligned((rdi.wrapping_add(0x8u64)) as *const u64))))) as u64; rdx = ((((rdx as u32)) & ((rax as u32)))) as u32 as u64; rcx = ((rcx).wrapping_add(1)) as u64; rdi = ((rdi).wrapping_add(0x10u64)) as u64; fl_l = (rcx) as u64 as u64; fl_r = (r9) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 133; continue; } bb = 135; continue; }
            135 => { rax = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rbx, (0x2u64) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), (rax) as u64); fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; rax = (rax & !0xffu64) | (((if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { 1u8 } else { 0u8 }) as u8 as u64)); crate::simd_ops::wu8(rbx.wrapping_add(0x8u64), ((rax as u8)) as u8); rax = (rbx) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            136 => { fl_l = (r9) as u64 as u64; fl_r = (0x2u64) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 209; continue; } bb = 137; continue; }
            137 => { fl_l = (crate::simd_ops::ru32(r8)) as u32 as u64; fl_r = (0x3u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 209; continue; } bb = 138; continue; }
            138 => { fl_l = (crate::simd_ops::ru32(r8.wrapping_add(0x10u64))) as u32 as u64; fl_r = (0x3u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 209; continue; } bb = 139; continue; }
            139 => { fl_l = (r9) as u64 as u64; fl_r = (0x2u64) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 141; continue; } bb = 140; continue; }
            140 => { fl_l = (crate::simd_ops::ru32(r8.wrapping_add(0x20u64))) as u32 as u64; fl_r = (0x3u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 209; continue; } bb = 141; continue; }
            141 => { r8 = (crate::simd_ops::cvt_f64_i64((f64::from_bits(core::ptr::read_unaligned((r8.wrapping_add(0x8u64)) as *const u64))))) as u64; rdx = (crate::simd_ops::cvt_f64_i32((f64::from_bits(core::ptr::read_unaligned((rdi.wrapping_add(0x18u64)) as *const u64))))) as u32 as u64; fl_l = (r9) as u64 as u64; fl_r = (0x2u64) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 143; continue; } bb = 142; continue; }
            142 => { rcx = ((r9.wrapping_sub(0x1u64)) as u32) as u32 as u64; bb = 144; continue; }
            143 => { rcx = (crate::simd_ops::cvt_f64_i32((f64::from_bits(core::ptr::read_unaligned((rdi.wrapping_add(0x28u64)) as *const u64))))) as u32 as u64; bb = 144; continue; }
            144 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 209; continue; } bb = 145; continue; }
            145 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 209; continue; } bb = 146; continue; }
            146 => { rax = ((rcx.wrapping_add(rdx)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x20u32) as u32 as u64; if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { bb = 209; continue; } bb = 147; continue; }
            147 => { rax = ((rcx.wrapping_sub(0x1u64)) as u32) as u32 as u64; rcx = (0xfffffffeu32) as u32 as u64; rcx = (((rcx as u32)).wrapping_shl(((((rax as u32)) as u32) & 31))) as u32 as u64; rax = (((r8 as u32)).wrapping_shr(((((rdx as u32)) as u32) & 31))) as u32 as u64; fl_l = ((!((rcx as u32))) & ((rax as u32))) as u32 as u64; fl_r = fl_l; rax = ((fl_l as u32)) as u32 as u64; rax = ((rax as u32)) as u32 as u64; bb = 110; continue; }
            148 => { fl_l = (r9) as u64 as u64; fl_r = (0x2u64) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 209; continue; } bb = 149; continue; }
            149 => { fl_l = (crate::simd_ops::ru32(r8)) as u32 as u64; fl_r = (0x3u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 209; continue; } bb = 150; continue; }
            150 => { fl_l = (crate::simd_ops::ru32(r8.wrapping_add(0x10u64))) as u32 as u64; fl_r = (0x3u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 209; continue; } bb = 151; continue; }
            151 => { rdx = (crate::simd_ops::cvt_f64_i32((f64::from_bits(core::ptr::read_unaligned((rdi.wrapping_add(0x18u64)) as *const u64))))) as u32 as u64; r8 = (crate::simd_ops::cvt_f64_i64((f64::from_bits(core::ptr::read_unaligned((r8.wrapping_add(0x8u64)) as *const u64))))) as u64; rax = ((rdx as u32)) as u32 as u64; rdx = ((((rdx as u32)) & (0x1fu32))) as u32 as u64; rax = (0u32.wrapping_sub((rax as u32))) as u32 as u64; rax = ((((rax as u32)) & (0x1fu32))) as u32 as u64; rcx = (((r8 as u32)).wrapping_shr(((((rax as u32)) as u32) & 31))) as u32 as u64; rax = (((r8 as u32)).wrapping_shl(((((rdx as u32)) as u32) & 31))) as u32 as u64; fl_l = ((((rax as u32)) | ((rcx as u32)))) as u32 as u64; fl_r = fl_l; rax = ((fl_l as u32)) as u32 as u64; bb = 110; continue; }
            152 => { fl_l = (r9) as u64 as u64; fl_r = (0x2u64) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 209; continue; } bb = 153; continue; }
            153 => { fl_l = (crate::simd_ops::ru32(r8)) as u32 as u64; fl_r = (0x3u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 209; continue; } bb = 154; continue; }
            154 => { fl_l = (crate::simd_ops::ru32(r8.wrapping_add(0x10u64))) as u32 as u64; fl_r = (0x3u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 209; continue; } bb = 155; continue; }
            155 => { rcx = (crate::simd_ops::cvt_f64_i32((f64::from_bits(core::ptr::read_unaligned((r8.wrapping_add(0x18u64)) as *const u64))))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = (0x20u32) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 209; continue; } bb = 156; continue; }
            156 => { rax = (crate::simd_ops::cvt_f64_i64((f64::from_bits(core::ptr::read_unaligned((r8.wrapping_add(0x8u64)) as *const u64))))) as u64; rax = (((rax as u32)).wrapping_shl(((((rcx as u32)) as u32) & 31))) as u32 as u64; bb = 110; continue; }
            157 => { fl_l = (r9) as u64 as u64; fl_r = (0x3u64) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 209; continue; } bb = 158; continue; }
            158 => { fl_l = (crate::simd_ops::ru32(r8)) as u32 as u64; fl_r = (0x3u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 209; continue; } bb = 159; continue; }
            159 => { fl_l = (crate::simd_ops::ru32(r8.wrapping_add(0x10u64))) as u32 as u64; fl_r = (0x3u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 209; continue; } bb = 160; continue; }
            160 => { fl_l = (crate::simd_ops::ru32(r8.wrapping_add(0x20u64))) as u32 as u64; fl_r = (0x3u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 209; continue; } bb = 161; continue; }
            161 => { fl_l = (r9) as u64 as u64; fl_r = (0x3u64) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 163; continue; } bb = 162; continue; }
            162 => { fl_l = (crate::simd_ops::ru32(r8.wrapping_add(0x30u64))) as u32 as u64; fl_r = (0x3u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 209; continue; } bb = 163; continue; }
            163 => { r10 = (crate::simd_ops::cvt_f64_i64((f64::from_bits(core::ptr::read_unaligned((r8.wrapping_add(0x8u64)) as *const u64))))) as u64; r11 = (crate::simd_ops::cvt_f64_i64((f64::from_bits(core::ptr::read_unaligned((r8.wrapping_add(0x18u64)) as *const u64))))) as u64; r8 = (crate::simd_ops::cvt_f64_i32((f64::from_bits(core::ptr::read_unaligned((r8.wrapping_add(0x28u64)) as *const u64))))) as u32 as u64; fl_l = (r9) as u64 as u64; fl_r = (0x3u64) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 165; continue; } bb = 164; continue; }
            164 => { rcx = ((r9.wrapping_sub(0x2u64)) as u32) as u32 as u64; bb = 166; continue; }
            165 => { rcx = (crate::simd_ops::cvt_f64_i32((f64::from_bits(core::ptr::read_unaligned((rdi.wrapping_add(0x38u64)) as *const u64))))) as u32 as u64; bb = 166; continue; }
            166 => { fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 209; continue; } bb = 167; continue; }
            167 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 209; continue; } bb = 168; continue; }
            168 => { rax = ((rcx.wrapping_add(r8)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x20u32) as u32 as u64; if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { bb = 209; continue; } bb = 169; continue; }
            169 => { rax = ((rcx.wrapping_sub(0x1u64)) as u32) as u32 as u64; rcx = (0xfffffffeu32) as u32 as u64; rdx = (((rcx as u32)).wrapping_shl(((((rax as u32)) as u32) & 31))) as u32 as u64; rdx = (!((rdx as u32))) as u32 as u64; rax = (((rdx as u32)).wrapping_shl(((((r8 as u32)) as u32) & 31))) as u32 as u64; rdx = ((((rdx as u32)) & ((r11 as u32)))) as u32 as u64; fl_l = ((!((rax as u32))) & ((r10 as u32))) as u32 as u64; fl_r = fl_l; rcx = ((fl_l as u32)) as u32 as u64; rax = (((rdx as u32)).wrapping_shl(((((r8 as u32)) as u32) & 31))) as u32 as u64; fl_l = ((((rax as u32)) | ((rcx as u32)))) as u32 as u64; fl_r = fl_l; rax = ((fl_l as u32)) as u32 as u64; bb = 110; continue; }
            170 => { fl_l = (r9) as u64 as u64; fl_r = (0x2u64) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 209; continue; } bb = 171; continue; }
            171 => { fl_l = (crate::simd_ops::ru32(r8)) as u32 as u64; fl_r = (0x3u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 209; continue; } bb = 172; continue; }
            172 => { fl_l = (crate::simd_ops::ru32(r8.wrapping_add(0x10u64))) as u32 as u64; fl_r = (0x3u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 209; continue; } bb = 173; continue; }
            173 => { rdx = (crate::simd_ops::cvt_f64_i32((f64::from_bits(core::ptr::read_unaligned((rdi.wrapping_add(0x18u64)) as *const u64))))) as u32 as u64; r8 = (crate::simd_ops::cvt_f64_i64((f64::from_bits(core::ptr::read_unaligned((r8.wrapping_add(0x8u64)) as *const u64))))) as u64; rax = ((rdx as u32)) as u32 as u64; rdx = ((((rdx as u32)) & (0x1fu32))) as u32 as u64; rax = (0u32.wrapping_sub((rax as u32))) as u32 as u64; rax = ((((rax as u32)) & (0x1fu32))) as u32 as u64; rcx = (((r8 as u32)).wrapping_shl(((((rax as u32)) as u32) & 31))) as u32 as u64; rax = (((r8 as u32)).wrapping_shr(((((rdx as u32)) as u32) & 31))) as u32 as u64; fl_l = ((((rax as u32)) | ((rcx as u32)))) as u32 as u64; fl_r = fl_l; rax = ((fl_l as u32)) as u32 as u64; bb = 110; continue; }
            174 => { fl_l = (r9) as u64 as u64; fl_r = (0x2u64) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 209; continue; } bb = 175; continue; }
            175 => { fl_l = (crate::simd_ops::ru32(r8)) as u32 as u64; fl_r = (0x3u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 209; continue; } bb = 176; continue; }
            176 => { fl_l = (crate::simd_ops::ru32(r8.wrapping_add(0x10u64))) as u32 as u64; fl_r = (0x3u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 209; continue; } bb = 177; continue; }
            177 => { rcx = (crate::simd_ops::cvt_f64_i32((f64::from_bits(core::ptr::read_unaligned((r8.wrapping_add(0x18u64)) as *const u64))))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = (0x20u32) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 209; continue; } bb = 178; continue; }
            178 => { rax = (crate::simd_ops::cvt_f64_i64((f64::from_bits(core::ptr::read_unaligned((r8.wrapping_add(0x8u64)) as *const u64))))) as u64; rax = (((rax as u32)).wrapping_shr(((((rcx as u32)) as u32) & 31))) as u32 as u64; bb = 110; continue; }
            179 => { fl_l = (r9) as u64 as u64; fl_r = (0x1u64) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 209; continue; } bb = 180; continue; }
            180 => { fl_l = (crate::simd_ops::ru32(r8)) as u32 as u64; fl_r = (0x0u32) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 209; continue; } bb = 181; continue; }
            181 => { rdx = (rdi) as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P23) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = (rbx) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            182 => { fl_l = (r9) as u64 as u64; fl_r = (0x1u64) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 186; continue; } bb = 183; continue; }
            183 => { fl_l = (crate::simd_ops::ru32(r8)) as u32 as u64; fl_r = (0x4u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 209; continue; } bb = 184; continue; }
            184 => { fl_l = (crate::simd_ops::ru32(r8.wrapping_add(0x4u64))) as u32 as u64; fl_r = (0x0u32) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 209; continue; } bb = 185; continue; }
            185 => { rax = (crate::simd_ops::ru64(r8.wrapping_add(0x8u64))) as u64; xmm0 = 0u128; rcx = ((crate::simd_ops::ru8(rax)) as u32) as u32 as u64; xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | (((((rcx as u32)) as i32) as f64).to_bits() as u128); bb = 111; continue; }
            186 => { fl_l = (r9) as u64 as u64; fl_r = (0x2u64) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 209; continue; } bb = 187; continue; }
            187 => { fl_l = (crate::simd_ops::ru32(r8)) as u32 as u64; fl_r = (0x4u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 209; continue; } bb = 188; continue; }
            188 => { fl_l = (crate::simd_ops::ru32(r8.wrapping_add(0x10u64))) as u32 as u64; fl_r = (0x3u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 209; continue; } bb = 189; continue; }
            189 => { rcx = (crate::simd_ops::cvt_f64_i32((f64::from_bits(core::ptr::read_unaligned((r8.wrapping_add(0x18u64)) as *const u64))))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 209; continue; } bb = 190; continue; }
            190 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(r8.wrapping_add(0x4u64))) as u32 as u64; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 209; continue; } bb = 191; continue; }
            191 => { rax = (crate::simd_ops::ru64(r8.wrapping_add(0x8u64))) as u64; rcx = ((((rcx as u32)) as i32 as i64) as u64) as u64; xmm0 = 0u128; rcx = ((crate::simd_ops::ru8(rax.wrapping_add(rcx).wrapping_sub(0x1u64))) as u32) as u32 as u64; xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | (((((rcx as u32)) as i32) as f64).to_bits() as u128); bb = 111; continue; }
            192 => { fl_l = (r9) as u64 as u64; fl_r = (0x1u64) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 209; continue; } bb = 193; continue; }
            193 => { fl_l = (crate::simd_ops::ru32(r8)) as u32 as u64; fl_r = (0x4u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 209; continue; } bb = 194; continue; }
            194 => { rax = (crate::simd_ops::ru32(r8.wrapping_add(0x4u64))) as u32 as u64; bb = 110; continue; }
            195 => { fl_l = (r9) as u64 as u64; fl_r = (0x3u64) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 209; continue; } bb = 196; continue; }
            196 => { fl_l = (crate::simd_ops::ru32(r8)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 209; continue; } bb = 197; continue; }
            197 => { fl_l = (crate::simd_ops::ru32(r8.wrapping_add(0x10u64))) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 209; continue; } bb = 198; continue; }
            198 => { fl_l = (crate::simd_ops::ru32(r8.wrapping_add(0x20u64))) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 209; continue; } bb = 199; continue; }
            199 => { xmm0 = ((crate::simd_ops::ru64(r8.wrapping_add(0x18u64))) as u64) as u128; xmm1 = ((crate::simd_ops::ru64(r8.wrapping_add(0x28u64))) as u64) as u128; fl_l = ((xmm1 as u64)) as u64; fl_r = ((xmm0 as u64)) as u64; if !(f64::from_bits(fl_l) >= f64::from_bits(fl_r)) { bb = 209; continue; } bb = 200; continue; }
            200 => { xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((if f64::from_bits(xmm0 as u64) > f64::from_bits(core::ptr::read_unaligned((r8.wrapping_add(0x8u64)) as *const u64)) { f64::from_bits(xmm0 as u64) } else { f64::from_bits(core::ptr::read_unaligned((r8.wrapping_add(0x8u64)) as *const u64)) }).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffff_ffff_ffffu128) | ((if f64::from_bits(xmm1 as u64) < f64::from_bits(xmm0 as u64) { f64::from_bits(xmm1 as u64) } else { f64::from_bits(xmm0 as u64) }).to_bits() as u128); crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), ((xmm1 as u64)) as u64); crate::simd_ops::wu64(rbx, (r9) as u64); rax = (rbx) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            201 => { fl_l = (r9) as u64 as u64; fl_r = (0x1u64) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 209; continue; } bb = 202; continue; }
            202 => { fl_l = (crate::simd_ops::ru32(r8)) as u32 as u64; fl_r = (0x3u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 209; continue; } bb = 203; continue; }
            203 => { xmm0 = ((crate::simd_ops::ru64(r8.wrapping_add(0x8u64))) as u64) as u128; xmm2 = 0u128; fl_l = ((xmm0 as u64)) as u64; fl_r = ((xmm2 as u64)) as u64; if !(f64::from_bits(fl_l) > f64::from_bits(fl_r)) { bb = 205; continue; } bb = 204; continue; }
            204 => { xmm1 = ((crate::simd_ops::ru64(P24)) as u64) as u128; crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), ((xmm1 as u64)) as u64); crate::simd_ops::wu64(rbx, (0x3u64) as u64); rax = (rbx) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            205 => { xmm1 = crate::simd_ops::vcmpsd(xmm0, (xmm0 as u64), (xmm2 as u64), 0x1u8); xmm0 = ((crate::simd_ops::ru64(P25)) as u64) as u128; xmm1 = crate::simd_ops::vblendvpd(xmm2, xmm0, xmm1); crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), ((xmm1 as u64)) as u64); crate::simd_ops::wu64(rbx, (0x3u64) as u64); rax = (rbx) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            206 => { fl_l = (r9) as u64 as u64; fl_r = (0x1u64) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 209; continue; } bb = 207; continue; }
            207 => { fl_l = (crate::simd_ops::ru32(r8)) as u32 as u64; fl_r = (0x3u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 209; continue; } bb = 208; continue; }
            208 => { xmm0 = ((crate::simd_ops::ru64(r8.wrapping_add(0x8u64))) as u64) as u128; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P26)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 111; continue; }
            209 => { rax = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rbx, (rax) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), (rax) as u64); rax = (rbx) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            0xffff_fffe => { match j_tgt.wrapping_sub(0x142532190u64) { 0x1c => { bb = 1; continue; } 0x32 => { bb = 2; continue; } 0x3c => { bb = 3; continue; } 0x46 => { bb = 4; continue; } 0x6e => { bb = 5; continue; } 0x78 => { bb = 6; continue; } 0x82 => { bb = 7; continue; } 0xa7 => { bb = 8; continue; } 0xb1 => { bb = 9; continue; } 0xbb => { bb = 10; continue; } 0xe0 => { bb = 11; continue; } 0xea => { bb = 12; continue; } 0xf4 => { bb = 13; continue; } 0xff => { bb = 14; continue; } 0x12a => { bb = 15; continue; } 0x134 => { bb = 16; continue; } 0x13e => { bb = 17; continue; } 0x163 => { bb = 18; continue; } 0x16d => { bb = 19; continue; } 0x177 => { bb = 20; continue; } 0x19c => { bb = 21; continue; } 0x1a6 => { bb = 22; continue; } 0x1b0 => { bb = 23; continue; } 0x1d5 => { bb = 24; continue; } 0x1df => { bb = 25; continue; } 0x1e9 => { bb = 26; continue; } 0x20e => { bb = 27; continue; } 0x218 => { bb = 28; continue; } 0x222 => { bb = 29; continue; } 0x24a => { bb = 30; continue; } 0x254 => { bb = 31; continue; } 0x25e => { bb = 32; continue; } 0x283 => { bb = 33; continue; } 0x28d => { bb = 34; continue; } 0x297 => { bb = 35; continue; } 0x2bc => { bb = 36; continue; } 0x2c6 => { bb = 37; continue; } 0x2d0 => { bb = 38; continue; } 0x2db => { bb = 39; continue; } 0x306 => { bb = 40; continue; } 0x310 => { bb = 41; continue; } 0x31a => { bb = 42; continue; } 0x325 => { bb = 43; continue; } 0x351 => { bb = 44; continue; } 0x35b => { bb = 45; continue; } 0x365 => { bb = 46; continue; } 0x38a => { bb = 47; continue; } 0x390 => { bb = 48; continue; } 0x39a => { bb = 49; continue; } 0x3bf => { bb = 50; continue; } 0x3c9 => { bb = 51; continue; } 0x3d3 => { bb = 52; continue; } 0x3de => { bb = 53; continue; } 0x3fa => { bb = 54; continue; } 0x3fc => { bb = 55; continue; } 0x419 => { bb = 56; continue; } 0x423 => { bb = 57; continue; } 0x425 => { bb = 58; continue; } 0x442 => { bb = 59; continue; } 0x47d => { bb = 60; continue; } 0x487 => { bb = 61; continue; } 0x491 => { bb = 62; continue; } 0x4a1 => { bb = 63; continue; } 0x4a5 => { bb = 64; continue; } 0x4ae => { bb = 65; continue; } 0x4c3 => { bb = 66; continue; } 0x4dd => { bb = 67; continue; } 0x4e7 => { bb = 68; continue; } 0x4f1 => { bb = 69; continue; } 0x501 => { bb = 70; continue; } 0x505 => { bb = 71; continue; } 0x50e => { bb = 72; continue; } 0x523 => { bb = 73; continue; } 0x53d => { bb = 74; continue; } 0x547 => { bb = 75; continue; } 0x551 => { bb = 76; continue; } 0x55c => { bb = 77; continue; } 0x587 => { bb = 78; continue; } 0x591 => { bb = 79; continue; } 0x59b => { bb = 80; continue; } 0x5c3 => { bb = 81; continue; } 0x5cd => { bb = 82; continue; } 0x5d7 => { bb = 83; continue; } 0x5fc => { bb = 84; continue; } 0x606 => { bb = 85; continue; } 0x610 => { bb = 86; continue; } 0x635 => { bb = 87; continue; } 0x63f => { bb = 88; continue; } 0x649 => { bb = 89; continue; } 0x660 => { bb = 90; continue; } 0x677 => { bb = 91; continue; } 0x695 => { bb = 92; continue; } 0x69f => { bb = 93; continue; } 0x6a9 => { bb = 94; continue; } 0x6ce => { bb = 95; continue; } 0x6d8 => { bb = 96; continue; } 0x6e2 => { bb = 97; continue; } 0x707 => { bb = 98; continue; } 0x711 => { bb = 99; continue; } 0x71b => { bb = 100; continue; } 0x726 => { bb = 101; continue; } 0x735 => { bb = 102; continue; } 0x763 => { bb = 103; continue; } 0x76d => { bb = 104; continue; } 0x777 => { bb = 105; continue; } 0x787 => { bb = 106; continue; } 0x790 => { bb = 107; continue; } 0x799 => { bb = 108; continue; } 0x7ad => { bb = 109; continue; } 0x7af => { bb = 110; continue; } 0x7b8 => { bb = 111; continue; } 0x7d2 => { bb = 112; continue; } 0x7dc => { bb = 113; continue; } 0x7e6 => { bb = 114; continue; } 0x811 => { bb = 115; continue; } 0x81b => { bb = 116; continue; } 0x825 => { bb = 117; continue; } 0x839 => { bb = 118; continue; } 0x840 => { bb = 119; continue; } 0x849 => { bb = 120; continue; } 0x85d => { bb = 121; continue; } 0x862 => { bb = 122; continue; } 0x86c => { bb = 123; continue; } 0x876 => { bb = 124; continue; } 0x88a => { bb = 125; continue; } 0x890 => { bb = 126; continue; } 0x899 => { bb = 127; continue; } 0x8ad => { bb = 128; continue; } 0x8b2 => { bb = 129; continue; } 0x8bc => { bb = 130; continue; } 0x8c6 => { bb = 131; continue; } 0x8d6 => { bb = 132; continue; } 0x8e0 => { bb = 133; continue; } 0x8e9 => { bb = 134; continue; } 0x8fd => { bb = 135; continue; } 0x920 => { bb = 136; continue; } 0x92a => { bb = 137; continue; } 0x934 => { bb = 138; continue; } 0x93f => { bb = 139; continue; } 0x945 => { bb = 140; continue; } 0x950 => { bb = 141; continue; } 0x961 => { bb = 142; continue; } 0x967 => { bb = 143; continue; } 0x96c => { bb = 144; continue; } 0x974 => { bb = 145; continue; } 0x97c => { bb = 146; continue; } 0x988 => { bb = 147; continue; } 0x9a6 => { bb = 148; continue; } 0x9b0 => { bb = 149; continue; } 0x9ba => { bb = 150; continue; } 0x9c5 => { bb = 151; continue; } 0x9eb => { bb = 152; continue; } 0x9f5 => { bb = 153; continue; } 0x9ff => { bb = 154; continue; } 0xa0a => { bb = 155; continue; } 0xa19 => { bb = 156; continue; } 0xa29 => { bb = 157; continue; } 0xa33 => { bb = 158; continue; } 0xa3d => { bb = 159; continue; } 0xa48 => { bb = 160; continue; } 0xa53 => { bb = 161; continue; } 0xa59 => { bb = 162; continue; } 0xa64 => { bb = 163; continue; } 0xa7c => { bb = 164; continue; } 0xa82 => { bb = 165; continue; } 0xa87 => { bb = 166; continue; } 0xa90 => { bb = 167; continue; } 0xa98 => { bb = 168; continue; } 0xaa5 => { bb = 169; continue; } 0xacd => { bb = 170; continue; } 0xad7 => { bb = 171; continue; } 0xae1 => { bb = 172; continue; } 0xaec => { bb = 173; continue; } 0xb12 => { bb = 174; continue; } 0xb1c => { bb = 175; continue; } 0xb26 => { bb = 176; continue; } 0xb31 => { bb = 177; continue; } 0xb40 => { bb = 178; continue; } 0xb50 => { bb = 179; continue; } 0xb5a => { bb = 180; continue; } 0xb64 => { bb = 181; continue; } 0xb7d => { bb = 182; continue; } 0xb83 => { bb = 183; continue; } 0xb8d => { bb = 184; continue; } 0xb98 => { bb = 185; continue; } 0xbac => { bb = 186; continue; } 0xbb6 => { bb = 187; continue; } 0xbc0 => { bb = 188; continue; } 0xbcb => { bb = 189; continue; } 0xbd9 => { bb = 190; continue; } 0xbe3 => { bb = 191; continue; } 0xbfc => { bb = 192; continue; } 0xc06 => { bb = 193; continue; } 0xc10 => { bb = 194; continue; } 0xc19 => { bb = 195; continue; } 0xc23 => { bb = 196; continue; } 0xc2c => { bb = 197; continue; } 0xc36 => { bb = 198; continue; } 0xc40 => { bb = 199; continue; } 0xc56 => { bb = 200; continue; } 0xc76 => { bb = 201; continue; } 0xc80 => { bb = 202; continue; } 0xc86 => { bb = 203; continue; } 0xc96 => { bb = 204; continue; } 0xcb8 => { bb = 205; continue; } 0xce5 => { bb = 206; continue; } 0xceb => { bb = 207; continue; } 0xcf1 => { bb = 208; continue; } 0xd02 => { bb = 209; continue; } _ => { let s5 = crate::simd_ops::read_s5_tail(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x40) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(j_tgt, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::set_xmm_ret(c_x0); return c_rax; } } }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142532ff0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142533250; const P1: u64 = 0x1c;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdi = r8;
    rbx = rdx;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rcx = rax;
    rdx = 0x80000000;
    rax = (crate::simd_ops::ru32(rbx.wrapping_add(P1))) as u32 as u64;
    fl_l = rax; fl_r = rdx;
    if !((((fl_l as u64)) < ((fl_r as u64)))) { rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; fl_l = ((rax).wrapping_shl(16u32)) as u64; rax = ((fl_l as u64)) as u64; }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rax = ((rax).wrapping_sub(rdi)) as u64;
    rax = ((rax).wrapping_add(rcx)) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142533250(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1c;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); r10 = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64; r9 = rdx; rbx = rdx; r9 = ((r9).wrapping_sub(rcx)) as u64; r9 = ((r9).wrapping_sub(0x108u64)) as u64; rax = (0x6666666666666667u64) as u64; let prod = (rax as i64 as i128).wrapping_mul((r9) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; r11 = rcx; rdx = ((((rdx) as i64).wrapping_shr(5u32) as u64)) as u64; rax = rdx; rax = ((rax).wrapping_shr(63u32)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; fl_l = r10; fl_r = (0x8u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 5; continue; } bb = 1; continue; }
            1 => { fl_l = r10; fl_r = (0x40u64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rdx = ((rdx).wrapping_shl(16u32)) as u64; rax = ((r10.wrapping_add(r10.wrapping_mul(2))) as u64) as u64; rcx = r8; rcx = ((rcx).wrapping_sub(rdx)) as u64; rcx = ((rcx).wrapping_sub(r11)) as u64; fl_l = rcx; fl_r = rax; rcx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; bb = 7; continue; }
            3 => { fl_l = r10; fl_r = (0x200u64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rdx = ((rdx).wrapping_shl(16u32)) as u64; rcx = r8; rcx = ((rcx).wrapping_sub(rdx)) as u64; rcx = ((rcx).wrapping_sub(r11)) as u64; fl_l = rcx; fl_r = r10; rcx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; bb = 7; continue; }
            5 => { rdx = ((rdx).wrapping_shl(16u32)) as u64; rcx = r8; rcx = ((rcx).wrapping_sub(rdx)) as u64; rax = 0x80000000; rcx = ((rcx).wrapping_sub(r11)) as u64; fl_l = r10; fl_r = rax; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 7; continue; } bb = 6; continue; }
            6 => { r10 = (crate::simd_ops::ru32(rbx)) as u32 as u64; fl_l = ((r10).wrapping_shl(16u32)) as u64; r10 = ((fl_l as u64)) as u64; bb = 7; continue; }
            7 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x8u64))) as u64; rdx = 0; fl_l = 0; fl_r = 0; rax = rcx; let d = (r10) as u128; let n = ((rdx as u128) << 64) | (rax as u128); if d != 0 { rax = (n / d) as u64; rdx = (n % d) as u64; } fl_l = r8; fl_r = rdx; r8 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; rax = r8; return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142533510(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7; const P1: u64 = 0x8; const P2: u64 = 0x141484ec0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = ((rdx.wrapping_add(P0)) as u64) as u64;
    r8 = rdx;
    rax = ((rax).wrapping_shr(3u32)) as u64;
    r9 = 0;
    fl_l = 0; fl_r = 0;
    rdx = (crate::simd_ops::ru64(rcx.wrapping_add(rax.wrapping_mul(8)).wrapping_add(P1))) as u64;
    return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142533630(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rcx = ((((rcx as u32)) & (0x3u32))) as u32 as u64;
    fl_l = (((rcx) | (rdx))) as u64; fl_r = fl_l;
    rcx = ((fl_l as u64)) as u64;
    rax = rcx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425336d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x1c;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    r8 = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64;
    rax = (0x6666666666666667u64) as u64;
    rdx = ((rdx).wrapping_sub(rcx)) as u64;
    rdx = ((rdx).wrapping_sub(0x108u64)) as u64;
    let prod = (rax as i64 as i128).wrapping_mul((rdx) as i64 as i128) as u128;
    rax = prod as u64; rdx = (prod >> 64) as u64;
    rdx = ((((rdx) as i64).wrapping_shr(5u32) as u64)) as u64;
    rax = rdx;
    rax = ((rax).wrapping_shr(63u32)) as u64;
    rdx = ((rdx).wrapping_add(rax)) as u64;
    rax = 0;
    fl_l = 0; fl_r = 0;
    fl_l = r8; fl_r = (0x8u64) as u64;
    if (((fl_l as u64)) < ((fl_r as u64))) { rdx = ((rdx).wrapping_shl(16u32)) as u64; rax = ((rax).wrapping_add(rdx)) as u64; fl_l = ((rax).wrapping_add(rcx)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; return rax; }
    fl_l = r8; fl_r = (0x40u64) as u64;
    if !((((fl_l as u64)) > ((fl_r as u64)))) { rdx = ((rdx).wrapping_shl(16u32)) as u64; rax = ((r8.wrapping_add(r8.wrapping_mul(2))) as u64) as u64; rax = ((rax).wrapping_add(rdx)) as u64; fl_l = ((rax).wrapping_add(rcx)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; return rax; }
    fl_l = r8; fl_r = (0x200u64) as u64;
    rax = (if (((fl_l as u64)) <= ((fl_r as u64))) { r8 } else { rax }) as u64;
    rdx = ((rdx).wrapping_shl(16u32)) as u64;
    rax = ((rax).wrapping_add(rdx)) as u64;
    fl_l = ((rax).wrapping_add(rcx)) as u64; fl_r = fl_l;
    rax = ((fl_l as u64)) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425338c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xffffffffffffffff; const P1: u64 = 0x108; const P2: u64 = 0x4; const P3: u64 = 0x100; const P4: u64 = 0xe; const P5: u64 = 0x20; const P6: u64 = 0x18; const P7: u64 = 0x1425371a0; const P8: u64 = 0x142533200;
    let (mut rax, mut r10, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 1; continue; }
            1 => { r10 = ((rcx.wrapping_add(P0)) as u64) as u64; rax = rcx; r10 = (((r10) & (0xfffffffffe000000u64))) as u64; rax = ((rax).wrapping_sub(r10)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rdx = ((r10.wrapping_add(P1)) as u64) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(P2))) as u32 as u64; rdx = ((rdx).wrapping_sub(rax)) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(r10.wrapping_add(P3))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 6; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(P4))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 5; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(P5))) as u64; crate::simd_ops::wu64(rcx, rax); fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(P6), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; crate::simd_ops::wu64(rdx.wrapping_add(P5), rcx); if (((fl_l as u32)) != 0) { bb = 7; continue; } bb = 4; continue; }
            4 => { rcx = rdx; return crate::simd_ops::call_fwd(P7, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            5 => { r9 = rcx; r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); rcx = r10; return crate::simd_ops::call_fwd(P8, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            6 => { r9 = rcx; r8 = 0; fl_l = 0; fl_r = 0; rcx = r10; return crate::simd_ops::call_fwd(P8, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            7 => { return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142533940(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142534c70; const P1: u64 = 0x142533a80; const P2: u64 = 0x143844164; const P3: u64 = 0x141834d18; const P4: u64 = 0x143a8ae08; const P5: u64 = 0x143c4d030; const P6: u64 = 0x142538150; const P7: u64 = 0x142538010; const P8: u64 = 0x143a632e8; const P9: u64 = 0x143872bf0; const P10: u64 = 0x1425375a0; const P11: u64 = 0x143425860; const P12: u64 = 0x142538300; const P13: u64 = 0x143872e54; const P14: u64 = 0x143872e50; const P15: u64 = 0x143872e74; const P16: u64 = 0x143857bec; const P17: u64 = 0x143872e70; const P18: u64 = 0x143857be8; const P19: u64 = 0x1425345a0; const P20: u64 = 0x142533dd0; const P21: u64 = 0x14397d8f6; const P22: u64 = 0x1434254a8; const P23: u64 = 0x14384314c; const P24: u64 = 0x1438430c8; const P25: u64 = 0x142544510;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rcx = P1; crate::simd_ops::wu8(P2, (0x0u8) as u8); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rbx = 0x1; let xa_src: u64 = rbx; let xa_old: u64 = crate::simd_ops::atomic_xadd_u64(P4, xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rbx = (xa_old) as u64; rax = 0x8000; rdi = P5; fl_l = rbx; fl_r = rax; rcx = rdi; rbx = (if (((fl_l as u64)) > ((fl_r as u64))) { rax } else { rbx }) as u64; rdx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu8(rbx.wrapping_add(rdi), (0x0u8) as u8); rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu8(rbx.wrapping_add(rdi), (0xau8) as u8); rax = P7; rdi = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(P8, rax); rbx = P9; rsi = ((rdi.wrapping_add(0x1au64)) as u32) as u32 as u64; bb = 1; continue; }
            1 => { fl_l = rdi; fl_r = (0x19u64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 4; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x4u64))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rcx = rbx; rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { r8 = (crate::simd_ops::ru32(rbx)) as u32 as u64; rcx = P11; rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rax = crate::simd_ops::call_stk(P12, rax, rcx, rdx, r8, r9, rsp); rdi = ((rdi).wrapping_add(1)) as u64; rbx = ((rbx).wrapping_add(0x20u64)) as u64; fl_l = rsi; fl_r = (0x1u64) as u64; rsi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 5; continue; }
            5 => { fl_l = (crate::simd_ops::ru32(P13)) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rcx = P14; rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp); bb = 7; continue; }
            7 => { fl_l = (crate::simd_ops::ru32(P15)) as u32 as u64; fl_r = 0x0; rax = (crate::simd_ops::ru32(P14)) as u32 as u64; crate::simd_ops::wu32(P16, ((rax as u32)) as u32); if (((fl_l as u32)) != ((fl_r as u32))) { bb = 9; continue; } bb = 8; continue; }
            8 => { rcx = P17; rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp); bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru32(P17)) as u32 as u64; crate::simd_ops::wu32(P18, ((rax as u32)) as u32); rax = crate::simd_ops::call_stk(P19, rax, rcx, rdx, r8, r9, rsp); rax = crate::simd_ops::call_stk(P20, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru8(P21)) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 11; continue; } bb = 10; continue; }
            10 => { rcx = P22; rax = crate::simd_ops::call_stk(P12, rax, rcx, rdx, r8, r9, rsp); bb = 11; continue; }
            11 => { fl_l = (crate::simd_ops::ru8(P23)) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 13; continue; } bb = 12; continue; }
            12 => { rdx = 0; fl_l = 0; fl_r = 0; rcx = P24; rax = crate::simd_ops::call_stk(P25, rax, rcx, rdx, r8, r9, rsp); bb = 13; continue; }
            13 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = 0; fl_l = 0; fl_r = 0; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142533a80(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14397d8f3; const P1: u64 = 0x143a28fa5; const P2: u64 = 0x143857d08; const P3: u64 = 0x1426452f0; const P4: u64 = 0x142545b80; const P5: u64 = 0x143872eb4; const P6: u64 = 0x143872eb0; const P7: u64 = 0x1425375a0; const P8: u64 = 0x1425462a0; const P9: u64 = 0x143073d80; const P10: u64 = 0x1425457f0; const P11: u64 = 0x142534310; const P12: u64 = 0x1425338c0; const P13: u64 = 0x142534c70; const P14: u64 = 0x143c7f580; const P15: u64 = 0x143842580; const P16: u64 = 0x143c7d200; const P17: u64 = 0x142539cc0; const P18: u64 = 0x1425427b0; const P19: u64 = 0x143872c14; const P20: u64 = 0x143872c10; const P21: u64 = 0x143872c34; const P22: u64 = 0x143872c30; const P23: u64 = 0x143a095c0; const P24: u64 = 0x142540260; const P25: u64 = 0x14183acc2; const P26: u64 = 0x14253eb50; const P27: u64 = 0x1438430a0; const P28: u64 = 0x143425450; const P29: u64 = 0x142538300; const P30: u64 = 0x143844164;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 21>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0xa8u64)) as u64; fl_l = (crate::simd_ops::ru8(P0)) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 43; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru8(P1)) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 43; continue; } bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru32(P2)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xc0u64), rsi); crate::simd_ops::wu8(P1, (0x1u8) as u8); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = ((crate::exe::tls_pointer() as u64)) as u64; rdx = 0x1; rsi = 0x6348; rsi = ((rsi).wrapping_add(crate::simd_ops::ru64(rax))) as u64; rcx = (crate::simd_ops::ru64(rsi)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru32(P5)) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rcx = P6; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::ru32(P6)) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 33; continue; } bb = 5; continue; }
            5 => { rcx = (crate::simd_ops::ru64(rsi)) as u64; rdx = 0x1; crate::simd_ops::wu64(rsp.wrapping_add(0xb0u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0xb8u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0xc8u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0x98u64), r14); crate::simd_ops::wu64(rsp.wrapping_add(0x90u64), r15); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(rax)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; rcx = (crate::simd_ops::ru64(rax)) as u64; rbx = (crate::simd_ops::ru64(rcx.wrapping_add(0x18u64))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 21; continue; } bb = 6; continue; }
            6 => { rbp = P9; bb = 7; continue; }
            7 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0xbf0u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; rdi = (crate::simd_ops::ru64(rbx.wrapping_add(0xbe8u64))) as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 19; continue; } bb = 8; continue; }
            8 => { fl_l = rbx; fl_r = rbp; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 20; continue; } bb = 9; continue; }
            9 => { rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P10, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; fl_l = rcx; fl_r = rbx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 20; continue; } bb = 10; continue; }
            10 => { fl_l = rbx; fl_r = (crate::simd_ops::ru64(rsi)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 12; continue; } bb = 11; continue; }
            11 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P11, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 12; continue; }
            12 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x18u64))) as u64; fl_l = rax; fl_r = rbx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 17; continue; } bb = 13; continue; }
            13 => { bb = 14; continue; }
            14 => { fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 18; continue; } bb = 15; continue; }
            15 => { rcx = rax; rax = (crate::simd_ops::ru64(rax.wrapping_add(0xbe8u64))) as u64; fl_l = rax; fl_r = rbx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 14; continue; } bb = 16; continue; }
            16 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0xbe8u64))) as u64; crate::simd_ops::wu64(rcx.wrapping_add(0xbe8u64), rax); rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P12, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 20; continue; }
            17 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0xbe8u64))) as u64; crate::simd_ops::wu64(rcx.wrapping_add(0x18u64), rax); bb = 18; continue; }
            18 => { rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P12, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 20; continue; }
            19 => { rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P10, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 20; continue; }
            20 => { rbx = rdi; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 7; continue; } bb = 21; continue; }
            21 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P13, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rsi = (crate::simd_ops::ru64(P14)) as u64; r15 = 0; fl_l = 0; fl_r = 0; r14 = (crate::simd_ops::ru64(P15)) as u64; rbp = ((r15 as u32)) as u32 as u64; rdi = ((r15 as u32)) as u32 as u64; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 32; continue; } bb = 22; continue; }
            22 => { crate::simd_ops::wu64(rsp.wrapping_add(0xa0u64), r12); r12 = P16; bb = 23; continue; }
            23 => { rbx = (crate::simd_ops::ru64(r12.wrapping_add(rdi.wrapping_mul(8)))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 30; continue; } bb = 24; continue; }
            24 => { fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(0x20u64))) as u64; fl_r = r15; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 27; continue; } bb = 25; continue; }
            25 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x1cu64))) as u32 as u64; rax = (((rax as u32)).wrapping_add(0xfffffffdu32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 27; continue; } bb = 26; continue; }
            26 => { crate::simd_ops::wu64(r12.wrapping_add(rdi.wrapping_mul(8)), r15); r9 = rsp.wrapping_add(0x30); rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x28u64))) as u64; r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); xmm0 = (crate::simd_ops::ru128(rbx.wrapping_add(0x8u64))) as u128; xmm1 = ((crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64) as u128; rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x20u64))) as u64; rdx = ((rdx).wrapping_shl(25u32)) as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), ((xmm1 as u64)) as u64); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P17, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 28; continue; }
            27 => { rbp = rdi; bb = 28; continue; }
            28 => { xmm1 = ((crate::simd_ops::ru64(rbx.wrapping_add(0x50u64))) as u64) as u128; xmm0 = (crate::simd_ops::ru128(rbx.wrapping_add(0x40u64))) as u128; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), ((xmm1 as u64)) as u64); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rax = ((rax).wrapping_shr(32u32)) as u64; rax = (((rax as u32)).wrapping_add(0xfffffffdu32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 30; continue; } bb = 29; continue; }
            29 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x38u64))) as u64; r9 = rsp.wrapping_add(0x70); r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); rcx = rbx; crate::simd_ops::wu128(rsp.wrapping_add(0x70u64), (xmm0) as u128); crate::simd_ops::wu64(rsp.wrapping_add(0x80u64), ((xmm1 as u64)) as u64); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P17, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 30; continue; }
            30 => { rdi = ((rdi).wrapping_add(1)) as u64; fl_l = rdi; fl_r = rsi; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 23; continue; } bb = 31; continue; }
            31 => { r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0xa0u64))) as u64; bb = 32; continue; }
            32 => { rax = rsi; fl_l = rax; let cx_src: u64 = rbp; let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u64(P14, fl_l as u64, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u64)) as u64; } rdx = (rdx & !0xffu64) | (((0x1u8) as u8 as u64)); r8 = r14.wrapping_add(0x3c0); rcx = (((rdx as u8)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P18, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x98u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0xc8u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0xb8u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0xb0u64))) as u64; bb = 33; continue; }
            33 => { fl_l = (crate::simd_ops::ru32(P19)) as u32 as u64; fl_r = 0x0; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0xc0u64))) as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 35; continue; } bb = 34; continue; }
            34 => { rcx = P20; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 35; continue; }
            35 => { fl_l = (crate::simd_ops::ru32(P20)) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 39; continue; } bb = 36; continue; }
            36 => { fl_l = (crate::simd_ops::ru32(P21)) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 38; continue; } bb = 37; continue; }
            37 => { rcx = P22; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 38; continue; }
            38 => { fl_l = (crate::simd_ops::ru32(P22)) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 42; continue; } bb = 39; continue; }
            39 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r9 = (crate::simd_ops::ru64(rax)) as u64; rax = P23; r9 = ((r9).wrapping_add(0x3c0u64)) as u64; fl_l = r9; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 41; continue; } bb = 40; continue; }
            40 => { rdx = r9; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P24, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = 0; fl_l = 0; fl_r = 0; rcx = r9; r8 = 0x280; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P25, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 41; continue; }
            41 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P26, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 42; continue; }
            42 => { rdx = (crate::simd_ops::ru64(P27)) as u64; rcx = P28; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P29, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu8(P30, (0x1u8) as u8); bb = 43; continue; }
            43 => { rsp = ((rsp).wrapping_add(0xa8u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142533dd0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143a63148; const P1: u64 = 0x14397d8f3; const P2: u64 = 0x1434254c0; const P3: u64 = 0x142538300; const P4: u64 = 0x1425345a0; const P5: u64 = 0x143872f48; const P6: u64 = 0x143872f4a; const P7: u64 = 0x143a09580; const P8: u64 = 0x142645720; const P9: u64 = 0x143872f30; const P10: u64 = 0x143872f40; const P11: u64 = 0x143425fc8; const P12: u64 = 0x142645308; const P13: u64 = 0x143426040; const P14: u64 = 0x142645758; const P15: u64 = 0x143bc5fd8; const P16: u64 = 0x143426058; const P17: u64 = 0x142645750; const P18: u64 = 0x143426068; const P19: u64 = 0x143426078; const P20: u64 = 0x143bfe128; const P21: u64 = 0x143425fd8; const P22: u64 = 0x143425fe8; const P23: u64 = 0x143426008; const P24: u64 = 0x143bce008; const P25: u64 = 0x143426020; const P26: u64 = 0x143be6098; const P27: u64 = 0x1434260e0; const P28: u64 = 0x143bee0c8; const P29: u64 = 0x143bde068; const P30: u64 = 0x143872cb4; const P31: u64 = 0x143872cb0; const P32: u64 = 0x1425375a0; const P33: u64 = 0x143872cd4; const P34: u64 = 0x143872cd0; const P35: u64 = 0x143872f38; const P36: u64 = 0x142541180; const P37: u64 = 0x142534c70; const P38: u64 = 0x143425420; const P39: u64 = 0x14309f824; const P40: u64 = 0x143425438; const P41: u64 = 0x142534500; const P42: u64 = 0x143857d08; const P43: u64 = 0x142645318; const P44: u64 = 0x14253eb30; const P45: u64 = 0x143a095c0; const P46: u64 = 0x14183acc2; const P47: u64 = 0x143bbd998; const P48: u64 = 0x143bbdfa8; const P49: u64 = 0x1425412f0; const P50: u64 = 0x143872cf4; const P51: u64 = 0x143872cf0; const P52: u64 = 0x142541dd0; const P53: u64 = 0x142544430; const P54: u64 = 0x143872d14; const P55: u64 = 0x143872d10; const P56: u64 = 0x1425421a0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 15>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x78u64)) as u64; fl_l = (crate::simd_ops::ru64(P0)) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 48; continue; } bb = 1; continue; }
            1 => { rcx = 0x1; rax = 0; fl_l = 0; fl_r = 0; fl_l = rax; let cx_src: u64 = rcx; let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u64(P0, fl_l as u64, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u64)) as u64; } if (((fl_l as u64)) != ((fl_r as u64))) { bb = 48; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu8(P1, ((rcx as u8)) as u8); rcx = P2; rdx = ((crate::exe::teb_self() as u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), rbx); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu16(P5, (0x100u16) as u16); rax = 0x7; crate::simd_ops::wu8(P6, (0x1u8) as u8); let (cp_a, cp_b, cp_c, cp_d) = crate::simd_ops::cpuid_x64(rax as u32, rcx as u32); rax = cp_a; rbx = cp_b; rcx = cp_c; rdx = cp_d; rdx = (((rdx as u32)).wrapping_shr(4u32)) as u32 as u64; rcx = rsp.wrapping_add(0x40); rdx = (rdx & !0xffu64) | ((((((rdx as u8)) & (0x1u8))) as u8 as u64)); crate::simd_ops::wu8(P7, ((rdx as u8)) as u8); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P8), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x44u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu64(P9, rax); bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x68u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { crate::simd_ops::wu64(P10, rax); bb = 6; continue; }
            6 => { rcx = P11; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P12), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 7; continue; }
            7 => { rdx = P13; rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P14), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(P15, rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { rdx = P16; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P14), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(P15, rax); bb = 9; continue; }
            9 => { rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P17), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 10; continue; }
            10 => { rcx = P18; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P12), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 11; continue; }
            11 => { rdx = P19; rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P14), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rbx; crate::simd_ops::wu64(P20, rax); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P17), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 12; continue; }
            12 => { rcx = P21; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P12), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 13; continue; }
            13 => { rdx = P22; rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P14), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = P23; rcx = rbx; crate::simd_ops::wu64(P24, rax); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P14), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = P25; rcx = rbx; crate::simd_ops::wu64(P26, rax); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P14), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = P27; rcx = rbx; crate::simd_ops::wu64(P28, rax); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P14), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rbx; crate::simd_ops::wu64(P29, rax); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P17), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 14; continue; }
            14 => { fl_l = (crate::simd_ops::ru32(P30)) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 16; continue; } bb = 15; continue; }
            15 => { rcx = P31; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P32, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 16; continue; }
            16 => { fl_l = (crate::simd_ops::ru32(P31)) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 20; continue; } bb = 17; continue; }
            17 => { fl_l = (crate::simd_ops::ru32(P33)) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 19; continue; } bb = 18; continue; }
            18 => { rcx = P34; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P32, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 19; continue; }
            19 => { fl_l = (crate::simd_ops::ru32(P34)) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 21; continue; } bb = 20; continue; }
            20 => { rcx = P35; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P36, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 21; continue; }
            21 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P37, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = 0; fl_l = 0; fl_r = 0; rcx = P38; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = P39; rcx = P40; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P41, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru32(P42)) as u32 as u64; rdx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P43), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P44, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = P45; fl_l = rax; fl_r = rbx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 23; continue; } bb = 22; continue; }
            22 => { rdx = 0; fl_l = 0; fl_r = 0; r8 = 0x280; rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P46, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 23; continue; }
            23 => { rdx = 0; fl_l = 0; fl_r = 0; r8 = 0x280; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P46, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru64(P47)) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 28; continue; } bb = 24; continue; }
            24 => { xmm1 = 0u128; xmm1 = (xmm1 & !0xffff_ffff_ffff_ffffu128) | ((((crate::simd_ops::ru64(P48)) as i64) as f64).to_bits() as u128); xmm0 = 0u128; fl_l = ((xmm1 as u64)) as u64; fl_r = ((xmm0 as u64)) as u64; if (f64::from_bits(fl_l).is_nan() || f64::from_bits(fl_r).is_nan()) { bb = 27; continue; } bb = 25; continue; }
            25 => { if (f64::from_bits(fl_l) != f64::from_bits(fl_r) && !f64::from_bits(fl_l).is_nan() && !f64::from_bits(fl_r).is_nan()) { bb = 27; continue; } bb = 26; continue; }
            26 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P49, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P49, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rax; fl_r = rbx; rax = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; crate::simd_ops::wu64(P48, rax); bb = 27; continue; }
            27 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P49, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(P47, rax); bb = 28; continue; }
            28 => { rcx = (crate::simd_ops::ru64(P33)) as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 30; continue; } bb = 29; continue; }
            29 => { rcx = P34; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P32, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(P33)) as u64; bb = 30; continue; }
            30 => { rax = (crate::simd_ops::ru32(P34)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 41; continue; } bb = 31; continue; }
            31 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 33; continue; } bb = 32; continue; }
            32 => { rcx = P34; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P32, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru32(P34)) as u32 as u64; bb = 33; continue; }
            33 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) >= 0) { bb = 35; continue; } bb = 34; continue; }
            34 => { rax = 0; fl_l = 0; fl_r = 0; bb = 36; continue; }
            35 => { rcx = 0x20000; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; rax = (if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { (rcx as u32) } else { (rax as u32) }) as u32 as u64; bb = 36; continue; }
            36 => { fl_l = (crate::simd_ops::ru32(P50)) as u32 as u64; fl_r = 0x0; rbx = ((((rax as u32)) as i32 as i64) as u64) as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 38; continue; } bb = 37; continue; }
            37 => { rcx = P51; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P32, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 38; continue; }
            38 => { rdx = (crate::simd_ops::ru32(P51)) as u32 as u64; rcx = rbx; r8 = (((rbx) as i64).wrapping_mul((0x1f4u64) as i64) as u64) as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0xffffffff; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 40; continue; } bb = 39; continue; }
            39 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P52, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 41; continue; }
            40 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P53, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 41; continue; }
            41 => { rdx = (crate::simd_ops::ru64(P54)) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 43; continue; } bb = 42; continue; }
            42 => { rcx = P55; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P32, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = (crate::simd_ops::ru64(P54)) as u64; bb = 43; continue; }
            43 => { rax = (crate::simd_ops::ru32(P55)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 48; continue; } bb = 44; continue; }
            44 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 46; continue; } bb = 45; continue; }
            45 => { rcx = P55; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P32, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru32(P55)) as u32 as u64; bb = 46; continue; }
            46 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 48; continue; } bb = 47; continue; }
            47 => { rcx = ((((rax as u32)) as i32 as i64) as u64) as u64; r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); rcx = ((rcx).wrapping_shl(10u32)) as u64; rdx = (((r8 as u8)) as u32) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (0x0u64) as u64); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P56, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 48; continue; }
            48 => { rsp = ((rsp).wrapping_add(0x78u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142534160(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143a09580;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 3>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x10u64)) as u64;
    rcx = 0;
    fl_l = 0; fl_r = 0;
    rax = 0x7;
    let (cp_a, cp_b, cp_c, cp_d) = crate::simd_ops::cpuid_x64(rax as u32, rcx as u32); rax = cp_a; rbx = cp_b; rcx = cp_c; rdx = cp_d;
    rdx = (((rdx as u32)).wrapping_shr(4u32)) as u32 as u64;
    rdx = (rdx & !0xffu64) | ((((((rdx as u8)) & (0x1u8))) as u8 as u64));
    crate::simd_ops::wu8(P0, ((rdx as u8)) as u8);
    rsp = ((rsp).wrapping_add(0x10u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142534190(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142534c70; const P1: u64 = 0x142533a80; const P2: u64 = 0x143844164; const P3: u64 = 0x141834d18; const P4: u64 = 0x143a8ae08; const P5: u64 = 0x143c4d030; const P6: u64 = 0x142538150; const P7: u64 = 0x142538010; const P8: u64 = 0x143a632e8; const P9: u64 = 0x143872bf0; const P10: u64 = 0x1425375a0; const P11: u64 = 0x143425860; const P12: u64 = 0x142538300; const P13: u64 = 0x143872e54; const P14: u64 = 0x143872e50; const P15: u64 = 0x143872e74; const P16: u64 = 0x143857bec; const P17: u64 = 0x143872e70; const P18: u64 = 0x143857be8; const P19: u64 = 0x1425345a0; const P20: u64 = 0x142533dd0; const P21: u64 = 0x14397d8f6; const P22: u64 = 0x1434254a8; const P23: u64 = 0x14384314c; const P24: u64 = 0x1438430c8; const P25: u64 = 0x142544510;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rcx = P1; crate::simd_ops::wu8(P2, (0x0u8) as u8); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rbx = 0x1; let xa_src: u64 = rbx; let xa_old: u64 = crate::simd_ops::atomic_xadd_u64(P4, xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rbx = (xa_old) as u64; rax = 0x8000; rdi = P5; fl_l = rbx; fl_r = rax; rcx = rdi; rbx = (if (((fl_l as u64)) > ((fl_r as u64))) { rax } else { rbx }) as u64; rdx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu8(rbx.wrapping_add(rdi), (0x0u8) as u8); rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu8(rbx.wrapping_add(rdi), (0xau8) as u8); rax = P7; rdi = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(P8, rax); rbx = P9; rsi = ((rdi.wrapping_add(0x1au64)) as u32) as u32 as u64; bb = 1; continue; }
            1 => { fl_l = rdi; fl_r = (0x19u64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 4; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x4u64))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rcx = rbx; rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { r8 = (crate::simd_ops::ru32(rbx)) as u32 as u64; rcx = P11; rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rax = crate::simd_ops::call_stk(P12, rax, rcx, rdx, r8, r9, rsp); rdi = ((rdi).wrapping_add(1)) as u64; rbx = ((rbx).wrapping_add(0x20u64)) as u64; fl_l = rsi; fl_r = (0x1u64) as u64; rsi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 5; continue; }
            5 => { fl_l = (crate::simd_ops::ru32(P13)) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rcx = P14; rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp); bb = 7; continue; }
            7 => { fl_l = (crate::simd_ops::ru32(P15)) as u32 as u64; fl_r = 0x0; rax = (crate::simd_ops::ru32(P14)) as u32 as u64; crate::simd_ops::wu32(P16, ((rax as u32)) as u32); if (((fl_l as u32)) != ((fl_r as u32))) { bb = 9; continue; } bb = 8; continue; }
            8 => { rcx = P17; rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp); bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru32(P17)) as u32 as u64; crate::simd_ops::wu32(P18, ((rax as u32)) as u32); rax = crate::simd_ops::call_stk(P19, rax, rcx, rdx, r8, r9, rsp); rax = crate::simd_ops::call_stk(P20, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru8(P21)) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 11; continue; } bb = 10; continue; }
            10 => { rcx = P22; rax = crate::simd_ops::call_stk(P12, rax, rcx, rdx, r8, r9, rsp); bb = 11; continue; }
            11 => { fl_l = (crate::simd_ops::ru8(P23)) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 13; continue; } bb = 12; continue; }
            12 => { rdx = 0; fl_l = 0; fl_r = 0; rcx = P24; rax = crate::simd_ops::call_stk(P25, rax, rcx, rdx, r8, r9, rsp); bb = 13; continue; }
            13 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425342e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64));
    fl_l = rcx; fl_r = rcx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { crate::simd_ops::wu64(rcx, (0x0u64) as u64); }
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142534310(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143857d08; const P1: u64 = 0x142645318;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = ((crate::exe::tls_pointer() as u64)) as u64;
    r8 = (0x6348u32) as u32 as u64;
    rdx = (crate::simd_ops::ru64(rax)) as u64;
    crate::simd_ops::wu64(r8.wrapping_add(rdx), (rcx) as u64);
    rdx = (rcx) as u64;
    rcx = (crate::simd_ops::ru32(P0)) as u32 as u64;
    crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P1)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142534340(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143073d80; const P1: u64 = 0x143867148; const P2: u64 = 0x143a096e0; const P3: u64 = 0x1425408f0; const P4: u64 = 0x1438430a0; const P5: u64 = 0x143842580; const P6: u64 = 0x143857d08; const P7: u64 = 0x142645318; const P8: u64 = 0x142546510; const P9: u64 = 0x142546310; const P10: u64 = 0x143a095c0; const P11: u64 = 0x142540260; const P12: u64 = 0x14183acc2; const P13: u64 = 0x143c4cfb0; const P14: u64 = 0x142539cc0;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x50u64)) as u64; r10 = 0x6348; rdi = rcx; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = ((crate::exe::tls_pointer() as u64)) as u64; rcx = (crate::simd_ops::ru64(rax)) as u64; rdi = (crate::simd_ops::ru64(r10.wrapping_add(rcx))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 25; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), rsi); rsi = P0; fl_l = rdi; fl_r = rsi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 24; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::lk_sub_u64(P1, (1) as u64); rcx = P2; rdx = (0xffffffffffffffffu64) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r9 = ((crate::exe::teb_self() as u64)) as u64; fl_l = (crate::simd_ops::ru64(rdi.wrapping_add(0xb20u64))) as u64; fl_r = r9; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 24; continue; } bb = 4; continue; }
            4 => { crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), rbx); rbx = 0; fl_l = 0; fl_r = 0; fl_l = (crate::simd_ops::ru64(P4)) as u64; fl_r = rbx; crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), rbp); if (((fl_l as u64)) == ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = (crate::simd_ops::ru64(P4)) as u64; fl_r = rax; rax = ((rbx as u32)) as u32 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rax = 0x1; bb = 7; continue; }
            7 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; rbp = P5; rax = ((crate::exe::tls_pointer() as u64)) as u64; rdx = rsi; rdx = (if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { rbp } else { rdx }) as u64; rcx = (crate::simd_ops::ru64(rax)) as u64; crate::simd_ops::wu64(r10.wrapping_add(rcx), rdx); rcx = (crate::simd_ops::ru32(P6)) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P7), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rdi)) as u64; rdi = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; fl_l = rdi; fl_r = rsi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 23; continue; } bb = 8; continue; }
            8 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 9; continue; }
            9 => { bb = 10; continue; }
            10 => { rsi = (crate::simd_ops::ru64(rcx.wrapping_add(0xbe8u64))) as u64; fl_l = rcx; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 12; continue; } bb = 11; continue; }
            11 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 12; continue; }
            12 => { rcx = rsi; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 10; continue; } bb = 13; continue; }
            13 => { fl_l = rdi; fl_r = rbp; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 15; continue; } bb = 14; continue; }
            14 => { rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 15; continue; }
            15 => { r9 = (crate::simd_ops::ru64(rdi)) as u64; rax = P10; r9 = ((r9).wrapping_add(0x3c0u64)) as u64; fl_l = r9; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 17; continue; } bb = 16; continue; }
            16 => { rdx = r9; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P11, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = 0; fl_l = 0; fl_r = 0; rcx = r9; r8 = 0x280; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P12, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 17; continue; }
            17 => { fl_l = rdi; fl_r = rbp; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 23; continue; } bb = 18; continue; }
            18 => { r8 = P13; rdx = r8; bb = 19; continue; }
            19 => { fl_l = (crate::simd_ops::ru64(rdx)) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 21; continue; } bb = 20; continue; }
            20 => { rax = ((((rbx as u32)) as i32 as i64) as u64) as u64; rcx = ((r8.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rax = 0; fl_l = 0; fl_r = 0; fl_l = rax; let cx_src: u64 = rdi; let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u64(rcx, fl_l as u64, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u64)) as u64; } if (((fl_l as u64)) == ((fl_r as u64))) { bb = 23; continue; } bb = 21; continue; }
            21 => { rbx = (((rbx as u32)).wrapping_add(1)) as u32 as u64; rdx = ((rdx).wrapping_add(0x8u64)) as u64; fl_l = ((rbx as u32)) as u32 as u64; fl_r = 0x10; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 19; continue; } bb = 22; continue; }
            22 => { xmm0 = (crate::simd_ops::ru128(rdi.wrapping_add(0x1238u64))) as u128; xmm1 = ((crate::simd_ops::ru64(rdi.wrapping_add(0x1248u64))) as u64) as u128; r9 = rsp.wrapping_add(0x30); r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); rdx = 0x1250; rcx = rdi; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), ((xmm1 as u64)) as u64); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P14, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 23; continue; }
            23 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; bb = 24; continue; }
            24 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; bb = 25; continue; }
            25 => { rsp = ((rsp).wrapping_add(0x50u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142534500(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
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
            5 => { rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142534570(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x1438430a0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = (crate::simd_ops::ru64(P0)) as u64; fl_r = (0x0u64) as u64;
    if (((fl_l as u64)) == ((fl_r as u64))) { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); return rax; }
    rax = ((crate::exe::teb_self() as u64)) as u64;
    fl_l = (crate::simd_ops::ru64(P0)) as u64; fl_r = rax;
    if !((((fl_l as u64)) == ((fl_r as u64)))) { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; return rax; }
    rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64));
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425345a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143a28fa4; const P1: u64 = 0x142540c40; const P2: u64 = 0x1426452e8; const P3: u64 = 0x143842580; const P4: u64 = 0x143857d08; const P5: u64 = 0x142645318;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    fl_l = (crate::simd_ops::ru8(P0)) as u8 as u64; fl_r = (0x0u8) as u8 as u64;
    if !((((fl_l as u8)) != ((fl_r as u8)))) { rcx = P1; crate::simd_ops::wu8(P0, (0x1u8) as u8); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); rcx = ((crate::exe::tls_pointer() as u64)) as u64; rdx = P3; r9 = 0x6348; crate::simd_ops::wu32(P4, ((rax as u32)) as u32); r8 = (crate::simd_ops::ru64(rcx)) as u64; rcx = ((rax as u32)) as u32 as u64; crate::simd_ops::wu64(r9.wrapping_add(r8), rdx); rsp = ((rsp).wrapping_add(0x28u64)) as u64; return crate::simd_ops::tail_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); }
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142534600(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143073d80; const P1: u64 = 0x1438430a0; const P2: u64 = 0x143842580; const P3: u64 = 0x143857d08; const P4: u64 = 0x142645318; const P5: u64 = 0x142546510; const P6: u64 = 0x142546310; const P7: u64 = 0x143a095c0; const P8: u64 = 0x142540260; const P9: u64 = 0x14183acc2; const P10: u64 = 0x143c4cfb0; const P11: u64 = 0x142539cc0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x50u64)) as u64; rsi = P0; rdi = rcx; fl_l = rcx; fl_r = rsi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rsp = ((rsp).wrapping_add(0x50u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            2 => { crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), rbx); rbx = 0; fl_l = 0; fl_r = 0; fl_l = (crate::simd_ops::ru64(P1)) as u64; fl_r = rbx; crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), rbp); if (((fl_l as u64)) == ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = (crate::simd_ops::ru64(P1)) as u64; fl_r = rax; rax = ((rbx as u32)) as u32 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = 0x1; bb = 5; continue; }
            5 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; r8 = 0x6348; rax = ((crate::exe::tls_pointer() as u64)) as u64; rbp = P2; rdx = rsi; rdx = (if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { rbp } else { rdx }) as u64; rcx = (crate::simd_ops::ru64(rax)) as u64; crate::simd_ops::wu64(r8.wrapping_add(rcx), rdx); rcx = (crate::simd_ops::ru32(P3)) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rdi)) as u64; rdi = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; fl_l = rdi; fl_r = rsi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 20; continue; } bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru64(rdi)) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x18u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 7; continue; }
            7 => { rsi = (crate::simd_ops::ru64(rcx.wrapping_add(0xbe8u64))) as u64; fl_l = rcx; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 9; continue; } bb = 8; continue; }
            8 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 9; continue; }
            9 => { rcx = rsi; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 7; continue; } bb = 10; continue; }
            10 => { fl_l = rdi; fl_r = rbp; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 12; continue; } bb = 11; continue; }
            11 => { rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 12; continue; }
            12 => { r9 = (crate::simd_ops::ru64(rdi)) as u64; rax = P7; r9 = ((r9).wrapping_add(0x3c0u64)) as u64; fl_l = r9; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 14; continue; } bb = 13; continue; }
            13 => { rdx = r9; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = 0; fl_l = 0; fl_r = 0; rcx = r9; r8 = 0x280; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 14; continue; }
            14 => { fl_l = rdi; fl_r = rbp; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 20; continue; } bb = 15; continue; }
            15 => { r8 = P10; rdx = r8; bb = 16; continue; }
            16 => { fl_l = (crate::simd_ops::ru64(rdx)) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 18; continue; } bb = 17; continue; }
            17 => { rax = ((((rbx as u32)) as i32 as i64) as u64) as u64; rcx = ((r8.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rax = 0; fl_l = 0; fl_r = 0; fl_l = rax; let cx_src: u64 = rdi; let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u64(rcx, fl_l as u64, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u64)) as u64; } if (((fl_l as u64)) == ((fl_r as u64))) { bb = 20; continue; } bb = 18; continue; }
            18 => { rbx = (((rbx as u32)).wrapping_add(1)) as u32 as u64; rdx = ((rdx).wrapping_add(0x8u64)) as u64; fl_l = ((rbx as u32)) as u32 as u64; fl_r = 0x10; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 16; continue; } bb = 19; continue; }
            19 => { xmm0 = (crate::simd_ops::ru128(rdi.wrapping_add(0x1238u64))) as u128; xmm1 = ((crate::simd_ops::ru64(rdi.wrapping_add(0x1248u64))) as u64) as u128; r9 = rsp.wrapping_add(0x30); r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); rdx = 0x1250; rcx = rdi; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), ((xmm1 as u64)) as u64); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P11, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 20; continue; }
            20 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rsp = ((rsp).wrapping_add(0x50u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142534780(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143073d80; const P1: u64 = 0x1438430a0; const P2: u64 = 0x143c4cfb0; const P3: u64 = 0x14253a550; const P4: u64 = 0x143425468; const P5: u64 = 0x1425386a0; const P6: u64 = 0x143a09580; const P7: u64 = 0x14183acc2; const P8: u64 = 0x143074a00; const P9: u64 = 0x14183acbc; const P10: u64 = 0x142544510; const P11: u64 = 0x1425466e0; const P12: u64 = 0x143857d08; const P13: u64 = 0x142645318; const P14: u64 = 0x1438430b0; const P15: u64 = 0x143842580; const P16: u64 = 0x1438430c8; const P17: u64 = 0x1438430b8; const P18: u64 = 0x1438430c0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rax = ((crate::exe::tls_pointer() as u64)) as u64; r15 = P0; r12 = 0x6348; rdi = ((r12 as u32)) as u32 as u64; rdi = ((rdi).wrapping_add(crate::simd_ops::ru64(rax))) as u64; fl_l = (crate::simd_ops::ru64(rdi)) as u64; fl_r = r15; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); rsp = ((rsp).wrapping_add(0x40u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            2 => { fl_l = (crate::simd_ops::ru64(P1)) as u64; fl_r = (0x0u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), rsi); if (((fl_l as u64)) == ((fl_r as u64))) { bb = 21; continue; } bb = 3; continue; }
            3 => { rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = (crate::simd_ops::ru64(P1)) as u64; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 21; continue; } bb = 4; continue; }
            4 => { rdi = P2; rcx = 0; fl_l = 0; fl_r = 0; rdx = rdi; bb = 5; continue; }
            5 => { rbx = (crate::simd_ops::ru64(rdx)) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { rbx = 0; fl_l = 0; fl_r = 0; rax = ((((rcx as u32)) as i32 as i64) as u64) as u64; let tmp_xchg: u64 = crate::simd_ops::atomic_xchg_u64(rdi.wrapping_add(rax.wrapping_mul(8)), rbx); rbx = (tmp_xchg) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 14; continue; } bb = 7; continue; }
            7 => { rcx = (((rcx as u32)).wrapping_add(1)) as u32 as u64; rdx = ((rdx).wrapping_add(0x8u64)) as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x10; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 5; continue; } bb = 8; continue; }
            8 => { fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 14; continue; } bb = 9; continue; }
            9 => { rdx = rsp.wrapping_add(0x20); rcx = 0x1250; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 12; continue; } bb = 10; continue; }
            10 => { rdx = rsp.wrapping_add(0x20); rcx = 0x1250; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 12; continue; } bb = 11; continue; }
            11 => { r8 = 0x1250; rdx = P4; rcx = ((rax.wrapping_add(0xcu64)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 24; continue; }
            12 => { fl_l = (crate::simd_ops::ru8(rsp.wrapping_add(0x32u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; xmm1 = ((crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64) as u128; crate::simd_ops::wu128(rbx.wrapping_add(0x1238u64), (xmm0) as u128); crate::simd_ops::wu64(rbx.wrapping_add(0x1248u64), ((xmm1 as u64)) as u64); if (((fl_l as u8)) == ((fl_r as u8))) { bb = 14; continue; } bb = 13; continue; }
            13 => { rsi = ((crate::simd_ops::ru8(P6)) as u32) as u32 as u64; bb = 17; continue; }
            14 => { rsi = ((crate::simd_ops::ru8(P6)) as u32) as u32 as u64; fl_l = ((rsi as u8)) as u8 as u64; fl_r = ((rsi as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 16; continue; } bb = 15; continue; }
            15 => { rdi = rbx; rax = 0; fl_l = 0; fl_r = 0; rcx = 0x1250; while rcx != 0 { crate::simd_ops::wu8(rdi, rax as u8); rdi = rdi.wrapping_add(1); rcx = rcx.wrapping_sub(1); } bb = 17; continue; }
            16 => { rdx = 0; fl_l = 0; fl_r = 0; r8 = 0x1250; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 17; continue; }
            17 => { crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), r14); r14 = rbx.wrapping_add(0xbf8); fl_l = ((rsi as u8)) as u8 as u64; fl_r = ((rsi as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 19; continue; } bb = 18; continue; }
            18 => { rsi = P8; rdi = r14; rcx = 0x640; while rcx != 0 { crate::simd_ops::wu8(rdi, crate::simd_ops::ru8(rsi)); rdi = rdi.wrapping_add(1); rsi = rsi.wrapping_add(1); rcx = rcx.wrapping_sub(1); } rsi = r15; rdi = rbx; rcx = 0xbf8; while rcx != 0 { crate::simd_ops::wu8(rdi, crate::simd_ops::ru8(rsi)); rdi = rdi.wrapping_add(1); rsi = rsi.wrapping_add(1); rcx = rcx.wrapping_sub(1); } bb = 20; continue; }
            19 => { rcx = r14; rdx = P8; r8 = 0x640; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rbx; rdx = r15; r8 = 0xbf8; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 20; continue; }
            20 => { rax = ((crate::exe::teb_self() as u64)) as u64; rcx = rbx.wrapping_add(0xb48); rdx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rbx.wrapping_add(0xb20u64), rax); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P10, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P11, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (((rax) | (0x1u64))) as u64; rcx = rbx; crate::simd_ops::wu64(rbx.wrapping_add(0xb30u64), rax); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P11, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rbx; crate::simd_ops::wu64(rbx.wrapping_add(0xb38u64), rax); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P11, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rbx.wrapping_add(0xb40u64), rax); rcx = r14.wrapping_add(0x3c0); crate::simd_ops::wu64(rbx, r14); rax = r14.wrapping_add(0x3b0); crate::simd_ops::wu64(r14.wrapping_add(0x3a0u64), rcx); rdx = rbx; crate::simd_ops::wu64(r14.wrapping_add(0x3b8u64), rcx); crate::simd_ops::wu64(r14.wrapping_add(0x3a8u64), rax); rax = ((crate::exe::tls_pointer() as u64)) as u64; crate::simd_ops::wu64(r14.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(r14.wrapping_add(0x18u64), rbx); rcx = (crate::simd_ops::ru64(rax)) as u64; crate::simd_ops::wu64(r12.wrapping_add(rcx), rbx); rcx = (crate::simd_ops::ru32(P12)) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P13), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; bb = 24; continue; }
            21 => { fl_l = (crate::simd_ops::ru64(P14)) as u64; fl_r = (0x0u64) as u64; rsi = P15; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 23; continue; } bb = 22; continue; }
            22 => { rax = ((crate::exe::teb_self() as u64)) as u64; rcx = P16; rdx = (rdx & !0xffu64) | (((0x1u8) as u8 as u64)); crate::simd_ops::wu64(P1, rax); crate::simd_ops::wu64(P14, (0x1u64) as u64); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P10, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P11, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rsi; crate::simd_ops::wu64(P14, rax); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P11, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rsi; crate::simd_ops::wu64(P17, rax); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P11, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(P18, rax); bb = 23; continue; }
            23 => { rcx = (crate::simd_ops::ru32(P12)) as u32 as u64; rdx = rsi; crate::simd_ops::wu64(rdi, rsi); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P13), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 24; continue; }
            24 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142534a50(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143c4cfb0; const P1: u64 = 0x1238; const P2: u64 = 0x1248; const P3: u64 = 0x142539cc0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x50u64)) as u64; rdi = P0; rsi = 0x10; rbx = rdi; bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru64(rdi)) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 4; continue; } bb = 2; continue; }
            2 => { rcx = 0; fl_l = 0; fl_r = 0; let tmp_xchg: u64 = crate::simd_ops::atomic_xchg_u64(rbx, rcx); rcx = (tmp_xchg) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { xmm0 = (crate::simd_ops::ru128(rcx.wrapping_add(P1))) as u128; xmm1 = ((crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64) as u128; r9 = rsp.wrapping_add(0x30); r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); rdx = 0x1250; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), ((xmm1 as u64)) as u64); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 4; continue; }
            4 => { rbx = ((rbx).wrapping_add(0x8u64)) as u64; rdi = ((rdi).wrapping_add(0x8u64)) as u64; fl_l = rsi; fl_r = (0x1u64) as u64; rsi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 5; continue; }
            5 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rsp = ((rsp).wrapping_add(0x50u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142534ae0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143c4cfb0; const P1: u64 = 0x1238; const P2: u64 = 0x1248; const P3: u64 = 0x142539cc0;
    let (mut rax, mut r10, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x58u64)) as u64; r10 = P0; rdx = 0; fl_l = 0; fl_r = 0; r9 = r10; bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru64(r9)) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rax = ((((rdx as u32)) as i32 as i64) as u64) as u64; r8 = ((r10.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rax = 0; fl_l = 0; fl_r = 0; fl_l = rax; let cx_src: u64 = rcx; let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u64(r8, fl_l as u64, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u64)) as u64; } if (((fl_l as u64)) == ((fl_r as u64))) { bb = 5; continue; } bb = 3; continue; }
            3 => { rdx = (((rdx as u32)).wrapping_add(1)) as u32 as u64; r9 = ((r9).wrapping_add(0x8u64)) as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x10; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 1; continue; } bb = 4; continue; }
            4 => { xmm0 = (crate::simd_ops::ru128(rcx.wrapping_add(P1))) as u128; xmm1 = ((crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64) as u128; r9 = rsp.wrapping_add(0x30); r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); rdx = 0x1250; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), ((xmm1 as u64)) as u64); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 5; continue; }
            5 => { rsp = ((rsp).wrapping_add(0x58u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142534b50(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143c4cfb0; const P1: u64 = 0x14253a550; const P2: u64 = 0x143425468; const P3: u64 = 0x1425386a0; const P4: u64 = 0x143a09580; const P5: u64 = 0x14183acc2;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rdi = P0; rcx = 0; fl_l = 0; fl_r = 0; rdx = rdi; bb = 1; continue; }
            1 => { rbx = (crate::simd_ops::ru64(rdx)) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rbx = 0; fl_l = 0; fl_r = 0; rax = ((((rcx as u32)) as i32 as i64) as u64) as u64; let tmp_xchg: u64 = crate::simd_ops::atomic_xchg_u64(rdi.wrapping_add(rax.wrapping_mul(8)), rbx); rbx = (tmp_xchg) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 9; continue; } bb = 3; continue; }
            3 => { rcx = (((rcx as u32)).wrapping_add(1)) as u32 as u64; rdx = ((rdx).wrapping_add(0x8u64)) as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x10; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 1; continue; } bb = 4; continue; }
            4 => { fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 9; continue; } bb = 5; continue; }
            5 => { rdx = rsp.wrapping_add(0x20); rcx = 0x1250; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 8; continue; } bb = 6; continue; }
            6 => { rdx = rsp.wrapping_add(0x20); rcx = 0x1250; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { r8 = 0x1250; rdx = P2; rcx = ((rax.wrapping_add(0xcu64)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            8 => { fl_l = (crate::simd_ops::ru8(rsp.wrapping_add(0x32u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; xmm1 = ((crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64) as u128; crate::simd_ops::wu128(rbx.wrapping_add(0x1238u64), (xmm0) as u128); crate::simd_ops::wu64(rbx.wrapping_add(0x1248u64), ((xmm1 as u64)) as u64); if (((fl_l as u8)) != ((fl_r as u8))) { bb = 12; continue; } bb = 9; continue; }
            9 => { fl_l = (crate::simd_ops::ru8(P4)) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 11; continue; } bb = 10; continue; }
            10 => { rax = 0; fl_l = 0; fl_r = 0; rdi = rbx; rcx = 0x1250; while rcx != 0 { crate::simd_ops::wu8(rdi, rax as u8); rdi = rdi.wrapping_add(1); rcx = rcx.wrapping_sub(1); } rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            11 => { rdx = 0; fl_l = 0; fl_r = 0; r8 = 0x1250; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 12; continue; }
            12 => { rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142534c50(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142534c70; const P1: u64 = 0x143842580;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rax = P1;
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142534c70(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1438430b0; const P1: u64 = 0x1438430c8; const P2: u64 = 0x1438430a0; const P3: u64 = 0x142544510; const P4: u64 = 0x143842580; const P5: u64 = 0x1425466e0; const P6: u64 = 0x1438430b8; const P7: u64 = 0x1438430c0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    fl_l = (crate::simd_ops::ru64(P0)) as u64; fl_r = (0x0u64) as u64;
    if !((((fl_l as u64)) != ((fl_r as u64)))) { rax = ((crate::exe::teb_self() as u64)) as u64; rcx = P1; rdx = (rdx & !0xffu64) | (((0x1u8) as u8 as u64)); crate::simd_ops::wu64(P2, rax); crate::simd_ops::wu64(P0, (0x1u64) as u64); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = P4; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rcx = P4; crate::simd_ops::wu64(P0, rax); rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rcx = P4; crate::simd_ops::wu64(P6, rax); rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(P7, rax); }
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142534e50(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x143a63148;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = (crate::simd_ops::ru64(P0)) as u64; fl_r = (0x0u64) as u64;
    if !((((fl_l as u64)) == ((fl_r as u64)))) { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; return rax; }
    rcx = 0x1;
    rax = 0;
    fl_l = 0; fl_r = 0;
    fl_l = rax; let cx_src: u64 = rcx;
    let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u64(P0, fl_l as u64, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u64)) as u64; }
    rax = (rax & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64));
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142534e90(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = 0;
    fl_l = 0; fl_r = 0;
    let tmp_xchg: u64 = crate::simd_ops::atomic_xchg_u64(rcx, rax);
    rax = (tmp_xchg) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142534f00(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x143867148;
    let (mut rax, mut fl_l) = (crate::simd_ops::get_rax_in(), 0u64);
    rax = 0x1;
    let xa_src: u64 = rax; let xa_old: u64 = crate::simd_ops::atomic_xadd_u64(P0, xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64;
    rax = (xa_old) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142534f10(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7; const P1: u64 = 0x8; const P2: u64 = 0x10; const P3: u64 = 0x142537280; const P4: u64 = 0x18; const P5: u64 = 0x142534f80;
    let (mut rax, mut r10, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    r10 = rcx;
    fl_l = r8; fl_r = r8;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rax = 0; fl_l = 0; fl_r = 0; return rax; }
    fl_l = ((r8) & ((r8).wrapping_sub(1))) as u64;
    rax = ((fl_l as u64)) as u64;
    if !((((fl_l as u64)) != 0)) { fl_l = ((rdx) & ((rdx).wrapping_sub(1))) as u64; rax = ((fl_l as u64)) as u64; if (((fl_l as u64)) != 0) { return crate::simd_ops::call_fwd(P5, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); } fl_l = rdx; fl_r = r8; if (((fl_l as u64)) < ((fl_r as u64))) { return crate::simd_ops::call_fwd(P5, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); } fl_l = rdx; fl_r = (0x400u64) as u64; if !((((fl_l as u64)) > ((fl_r as u64)))) { rax = ((rdx.wrapping_add(P0)) as u64) as u64; rax = ((rax).wrapping_shr(3u32)) as u64; r8 = (crate::simd_ops::ru64(rcx.wrapping_add(rax.wrapping_mul(8)).wrapping_add(P1))) as u64; rcx = (crate::simd_ops::ru64(r8.wrapping_add(P2))) as u64; fl_l = rcx; fl_r = rcx; if !(((((fl_l as u64)) & ((fl_r as u64))) != 0)) { r9 = 0; fl_l = 0; fl_r = 0; r8 = 0; fl_l = 0; fl_r = 0; rcx = r10; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); } fl_l = (crate::simd_ops::lk_add_u32(r8.wrapping_add(P4), (1) as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::wu64(r8.wrapping_add(P2), rax); rax = rcx; return rax; } return crate::simd_ops::call_fwd(P5, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
    rax = 0;
    fl_l = 0; fl_r = 0;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142535000(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xffffffffffffffff; const P1: u64 = 0x7; const P2: u64 = 0x8; const P3: u64 = 0x10; const P4: u64 = 0x141484ec0; const P5: u64 = 0x142535070;
    let (mut rax, mut r10, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 1; continue; }
            1 => { r9 = ((r8.wrapping_add(P0)) as u64) as u64; fl_l = r8; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 9; continue; } bb = 2; continue; }
            2 => { rax = (0x7fffffffffffffffu64) as u64; fl_l = rdx; fl_r = rax; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 9; continue; } bb = 3; continue; }
            3 => { fl_l = rdx; fl_r = (0x400u64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 8; continue; } bb = 4; continue; }
            4 => { fl_l = r8; fl_r = rdx; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 8; continue; } bb = 5; continue; }
            5 => { rax = ((rdx.wrapping_add(P1)) as u64) as u64; rax = ((rax).wrapping_shr(3u32)) as u64; r10 = (crate::simd_ops::ru64(rcx.wrapping_add(rax.wrapping_mul(8)).wrapping_add(P2))) as u64; rax = (crate::simd_ops::ru64(r10.wrapping_add(P3))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 6; continue; }
            6 => { fl_l = r9; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { r8 = rdx; r9 = 0; fl_l = 0; fl_r = 0; rdx = r10; return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            8 => { return crate::simd_ops::call_fwd(P5, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            9 => { rax = 0; fl_l = 0; fl_r = 0; return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142535280(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0xffffffffffffffff; const P1: u64 = 0x4;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rdx = ((rcx.wrapping_add(P0)) as u64) as u64;
    rdx = (((rdx) & (0xfffffffffe000000u64))) as u64;
    rcx = ((rcx).wrapping_sub(rdx)) as u64;
    rcx = ((rcx).wrapping_shr(16u32)) as u64;
    rax = ((rcx.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64;
    rax = ((rax).wrapping_shl(4u32)) as u64;
    rax = ((rax).wrapping_add(0x108u64)) as u64;
    rax = ((rax).wrapping_add(rdx)) as u64;
    rcx = (crate::simd_ops::ru32(rax.wrapping_add(P1))) as u32 as u64;
    fl_l = rax; fl_r = rcx;
    rax = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142535350(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l) = (crate::simd_ops::get_rax_in(), 0u64);
    fl_l = ((rcx) & ((rcx).wrapping_sub(1))) as u64;
    rax = ((fl_l as u64)) as u64;
    rax = (rax & !0xffu64) | (((if (((fl_l as u64)) == 0) { 1u8 } else { 0u8 }) as u8 as u64));
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142535590(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14253a310; const P1: u64 = 0x142536cb0; const P2: u64 = 0x41; const P3: u64 = 0x1425364e0;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rcx; r10 = r8; rcx = rdx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); r11 = rax; fl_l = rax; fl_r = (0x1000000u64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 3; continue; } bb = 1; continue; }
            1 => { fl_l = r10; fl_r = r10; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rdx = 0; fl_l = 0; fl_r = 0; bb = 4; continue; }
            3 => { rdx = 0x1; bb = 4; continue; }
            4 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; rax = 0x80000000; rcx = r11; rcx = (if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { rax } else { rcx }) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); r9 = (((rax as u8)) as u32) as u32 as u64; r8 = r11; rcx = rbx; rax = ((r9.wrapping_add(P2)) as u64) as u64; rax = ((r9.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; r9 = r10; rdx = ((rbx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P3, rax, rcx, rdx, r8, r9, rsp); }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142535600(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143a63258; const P1: u64 = 0x8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    r9 = (((rdx as u8)) as u32) as u32 as u64;
    rbx = rcx;
    crate::simd_ops::lk_add_u64(rax, (1) as u64);
    rax = (crate::simd_ops::ru64(P0)) as u64;
    fl_l = rax; fl_r = rax;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    fl_l = (crate::simd_ops::ru8(rax.wrapping_add(P1))) as u8 as u64; fl_r = (0x0u8) as u8 as u64;
    if !((((fl_l as u8)) != ((fl_r as u8)))) { crate::simd_ops::wu8(rax.wrapping_add(P1), (0x1u8) as u8); r8 = 0; fl_l = 0; fl_r = 0; rdx = (crate::simd_ops::ru64(rcx)) as u64; rcx = (((r9 as u8)) as u32) as u32 as u64; rax = (crate::simd_ops::ru64(P0)) as u64; rdx = (crate::simd_ops::ru64(rdx)) as u64; rax = crate::simd_ops::call_stk(rax, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu8(rax.wrapping_add(P1), (0x0u8) as u8); }
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142535660(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142536cb0; const P1: u64 = 0x1434254d8; const P2: u64 = 0x1425386a0; const P3: u64 = 0x142535770;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = rcx; rcx = rdx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); r9 = (((rax as u8)) as u32) as u32 as u64; rax = ((r9.wrapping_add(r9.wrapping_mul(2))) as u64) as u64; rsi = ((rdi.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rbx = (crate::simd_ops::ru64(rsi.wrapping_add(0x410u64))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 15; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x28u64))) as u64; fl_l = rax; fl_r = (0xfffffffffffffffcu64) as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 2; continue; }
            2 => { rdx = rax; rcx = rax; rdx = (((rdx) & (0xfffffffffffffffcu64))) as u64; rcx = ((((rcx as u32)) & (0x3u32))) as u32 as u64; fl_l = rax; let cx_src: u64 = rcx; let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u64(rbx.wrapping_add(0x28u64), fl_l as u64, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u64)) as u64; } if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 3; continue; }
            3 => { fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rdx)) as u64; rcx = 0x1; r8 = ((crate::simd_ops::ru16(rbx.wrapping_add(0xau64))) as u32) as u32 as u64; r9 = rdx; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 5; continue; }
            5 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 8; continue; } bb = 6; continue; }
            6 => { r9 = rax; rcx = (((rcx as u32)).wrapping_add(1)) as u32 as u64; rax = (crate::simd_ops::ru64(rax)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 7; continue; }
            7 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 9; continue; } bb = 8; continue; }
            8 => { rdx = P1; rcx = 0xe; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 10; continue; }
            9 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(r9, rax); fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x18u64))) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x18u64), ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32); crate::simd_ops::wu64(rbx.wrapping_add(0x20u64), rdx); bb = 10; continue; }
            10 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x20u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 11; continue; }
            11 => { fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 14; continue; } bb = 12; continue; }
            12 => { fl_l = (crate::simd_ops::lk_and_u8(rbx.wrapping_add(0xfu64), (0xfeu8) as u8)) as u8 as u64; fl_r = fl_l; crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), rax); crate::simd_ops::wu64(rbx.wrapping_add(0x20u64), (0x0u64) as u64); bb = 13; continue; }
            13 => { fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 15; continue; } bb = 14; continue; }
            14 => { crate::simd_ops::lk_and_u8(rbx.wrapping_add(0xfu64), (0x1u8) as u8); rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            15 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); rdx = rsi.wrapping_add(0x410); rcx = rdi; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P3, rax, rcx, rdx, r8, r9, rsp); }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142535990(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x30; const P1: u64 = 0x1c; const P2: u64 = 0x8; const P3: u64 = 0xf; const P4: u64 = 0xc; const P5: u64 = 0x142535a00;
    let (mut rax, mut r10, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    r10 = (crate::simd_ops::ru32(rdx)) as u32 as u64;
    rax = 0x80000000;
    fl_l = r8; fl_r = rax;
    crate::simd_ops::wu64(rdx.wrapping_add(P0), rcx);
    r9 = rdx;
    rax = (if (((fl_l as u64)) < ((fl_r as u64))) { (r8 as u32) } else { (rax as u32) }) as u32 as u64;
    r10 = ((r10).wrapping_shl(16u32)) as u64;
    rcx = ((rax as u32)) as u32 as u64;
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu32(rdx.wrapping_add(P1), ((rcx as u32)) as u32);
    fl_l = rcx; fl_r = (0x8u64) as u64;
    if (((fl_l as u64)) < ((fl_r as u64))) { rcx = ((crate::simd_ops::ru8(r9.wrapping_add(P2))) as u32) as u32 as u64; r10 = ((r10).wrapping_sub(rax)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = (rcx & !0xffu64) | (((((rcx as u8)).wrapping_shr(1u32)) as u8 as u64)); rcx = (rcx & !0xffu64) | ((((((rcx as u8)) ^ (crate::simd_ops::ru8(r9.wrapping_add(P3))))) as u8 as u64)); rax = r10; let d = (r8) as u128; let n = ((rdx as u128) << 64) | (rax as u128); if d != 0 { rax = (n / d) as u64; rdx = (n % d) as u64; } rcx = (rcx & !0xffu64) | ((((((rcx as u8)) & (0x1u8))) as u8 as u64)); rdx = r9; fl_l = (crate::simd_ops::lk_xor_u8(r9.wrapping_add(P3), ((rcx as u8)) as u8)) as u8 as u64; fl_r = fl_l; crate::simd_ops::wu16(r9.wrapping_add(P4), ((rax as u16)) as u16); return crate::simd_ops::call_fwd(P5, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
    fl_l = rcx; fl_r = (0x40u64) as u64;
    if !((((fl_l as u64)) > ((fl_r as u64)))) { rax = ((rcx.wrapping_add(rcx.wrapping_mul(2))) as u64) as u64; } else { fl_l = rcx; fl_r = (0x200u64) as u64; rax = (if (((fl_l as u64)) <= ((fl_r as u64))) { rcx } else { rax }) as u64; }
    rcx = ((crate::simd_ops::ru8(r9.wrapping_add(P2))) as u32) as u32 as u64;
    r10 = ((r10).wrapping_sub(rax)) as u64;
    rdx = 0;
    fl_l = 0; fl_r = 0;
    rcx = (rcx & !0xffu64) | (((((rcx as u8)).wrapping_shr(1u32)) as u8 as u64));
    rcx = (rcx & !0xffu64) | ((((((rcx as u8)) ^ (crate::simd_ops::ru8(r9.wrapping_add(P3))))) as u8 as u64));
    rax = r10;
    let d = (r8) as u128; let n = ((rdx as u128) << 64) | (rax as u128);
    if d != 0 { rax = (n / d) as u64; rdx = (n % d) as u64; }
    rcx = (rcx & !0xffu64) | ((((((rcx as u8)) & (0x1u8))) as u8 as u64));
    rdx = r9;
    fl_l = (crate::simd_ops::lk_xor_u8(r9.wrapping_add(P3), ((rcx as u8)) as u8)) as u8 as u64; fl_r = fl_l;
    crate::simd_ops::wu16(r9.wrapping_add(P4), ((rax as u16)) as u16);
    return crate::simd_ops::call_fwd(P5, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142535b90(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143823010; const P1: u64 = 0x143428c60; const P2: u64 = 0x1425466e0;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0; let mut ymm1_hi: u128 = 0; let mut ymm2_hi: u128 = 0; let mut ymm3_hi: u128 = 0; let mut xmm4: u128 = 0; let mut ymm4_hi: u128 = 0; let mut xmm5: u128 = 0; let mut ymm5_hi: u128 = 0; let mut xmm6: u128 = 0; let mut ymm6_hi: u128 = 0; let mut xmm7: u128 = 0; let mut ymm7_hi: u128 = 0; let mut xmm8: u128 = 0; let mut ymm8_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 143>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = rsp; crate::simd_ops::wu64(rax.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rax.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rax.wrapping_add(0x10u64), rdx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x450u64)) as u64; r10 = (crate::simd_ops::ru32(rdx.wrapping_add(0x1cu64))) as u32 as u64; r11 = rdx.wrapping_sub(0x1); crate::simd_ops::wu128(rax.wrapping_sub(0x38u64), (xmm6) as u128); r14 = r9; r11 = (((r11) & (0xfffffffffe000000u64))) as u64; rdi = rdx; rdi = ((rdi).wrapping_sub(r11)) as u64; rax = (0x6666666666666667u64) as u64; rdi = ((rdi).wrapping_sub(0x108u64)) as u64; r15 = rdx; let prod = (rax as i64 as i128).wrapping_mul((rdi) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; xmm6 = (r8) as u128; ymm6_hi = 0; r9 = rdx; r12 = r8; r9 = ((((r9) as i64).wrapping_shr(5u32) as u64)) as u64; r13 = rcx; rax = r9; rax = ((rax).wrapping_shr(63u32)) as u64; r9 = ((r9).wrapping_add(rax)) as u64; rax = 0; fl_l = 0; fl_r = 0; let bc_v: u128 = crate::simd_ops::vpbroadcastq((xmm6 as u64)); let (y_lo, y_hi): (u128, u128) = (bc_v, bc_v); xmm6 = y_lo; ymm6_hi = y_hi; fl_l = r10; fl_r = (0x8u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 4; continue; } bb = 1; continue; }
            1 => { fl_l = r10; fl_r = (0x40u64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rax = ((r10.wrapping_add(r10.wrapping_mul(2))) as u64) as u64; bb = 4; continue; }
            3 => { fl_l = r10; fl_r = (0x200u64) as u64; rax = (if (((fl_l as u64)) <= ((fl_r as u64))) { r10 } else { rax }) as u64; bb = 4; continue; }
            4 => { r9 = ((r9).wrapping_shl(16u32)) as u64; rcx = 0x6; r9 = ((r9).wrapping_add(rax)) as u64; r9 = ((r9).wrapping_add(r11)) as u64; xmm5 = (r9) as u128; ymm5_hi = 0; let bc_v: u128 = crate::simd_ops::vpbroadcastq((xmm5 as u64)); let (y_lo, y_hi): (u128, u128) = (bc_v, bc_v); xmm5 = y_lo; ymm5_hi = y_hi; fl_l = r14; fl_r = (0x40u64) as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rcx = ((rcx).wrapping_sub(1)) as u64; rax = ((r14).wrapping_shr((((rcx) as u32) & 63))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 6; continue; }
            6 => { rdx = 0; fl_l = 0; fl_r = 0; rsi = 0x1; rbx = ((rsi).wrapping_shl((((rcx) as u32) & 63))) as u64; rax = r14; let d = (rbx) as u128; let n = ((rdx as u128) << 64) | (rax as u128); if d != 0 { rax = (n / d) as u64; rdx = (n % d) as u64; } rdx = 0; fl_l = 0; fl_r = 0; r11 = rax; xmm4 = (rax) as u128; ymm4_hi = 0; let bc_v: u128 = crate::simd_ops::vpbroadcastq((xmm4 as u64)); let (y_lo, y_hi): (u128, u128) = (bc_v, bc_v); xmm4 = y_lo; ymm4_hi = y_hi; fl_l = rbx; fl_r = (0x10u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 11; continue; } bb = 7; continue; }
            7 => { fl_l = (crate::simd_ops::ru32(P0)) as u32 as u64; fl_r = 0x6; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 12; continue; } bb = 8; continue; }
            8 => { rax = (((crate::simd_ops::ru16(r15.wrapping_add(0xau64))) as i16 as i32) as u32) as u32 as u64; rcx = rbx; crate::simd_ops::wu128(rsp.wrapping_add(0x430u64), (xmm7) as u128); fl_l = (((rcx) & (0xfffffffffffffff0u64))) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; let y_addr: u64 = P1; xmm7 = crate::simd_ops::ru128(y_addr); ymm7_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); xmm0 = (((rax as u32)) as u32) as u128; ymm0_hi = 0; xmm0 = crate::simd_ops::vpbroadcastw((xmm0 as u16)); ymm0_hi = 0; crate::simd_ops::wu128(rsp.wrapping_add(0x420u64), (xmm8) as u128); let pm_in: u64 = (xmm0 as u64); let (y_lo, y_hi): (u128, u128) = (crate::simd_ops::vpmovzxwq(pm_in as u128), crate::simd_ops::vpmovzxwq((pm_in >> 32) as u128)); xmm8 = y_lo; ymm8_hi = y_hi; bb = 9; continue; }
            9 => { rax = rdx.wrapping_add(0x4); let y_addr: u64 = rsp.wrapping_add(rdx.wrapping_mul(8)).wrapping_add(0x20u64); crate::simd_ops::wu128(y_addr, xmm4); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm4_hi); xmm0 = (rdx) as u128; ymm0_hi = 0; let bc_v: u128 = crate::simd_ops::vpbroadcastq((xmm0 as u64)); let (y_lo, y_hi): (u128, u128) = (bc_v, bc_v); xmm0 = y_lo; ymm0_hi = y_hi; let (y_lo, y_hi): (u128, u128) = (crate::simd_ops::vpaddq(xmm0, xmm7), crate::simd_ops::vpaddq(ymm0_hi, ymm7_hi)); xmm0 = y_lo; ymm0_hi = y_hi; let (y_lo, y_hi): (u128, u128) = (crate::simd_ops::vpmullq(xmm0, xmm4), crate::simd_ops::vpmullq(ymm0_hi, ymm4_hi)); xmm1 = y_lo; ymm1_hi = y_hi; let (y_lo, y_hi): (u128, u128) = (crate::simd_ops::vpaddq(xmm1, xmm8), crate::simd_ops::vpaddq(ymm1_hi, ymm8_hi)); xmm2 = y_lo; ymm2_hi = y_hi; let (y_lo, y_hi): (u128, u128) = (crate::simd_ops::vpmullq(xmm2, xmm6), crate::simd_ops::vpmullq(ymm2_hi, ymm6_hi)); xmm3 = y_lo; ymm3_hi = y_hi; let (y_lo, y_hi): (u128, u128) = (crate::simd_ops::vpaddq(xmm3, xmm5), crate::simd_ops::vpaddq(ymm3_hi, ymm5_hi)); xmm0 = y_lo; ymm0_hi = y_hi; let y_addr: u64 = rsp.wrapping_add(rdx.wrapping_mul(8)).wrapping_add(0x220u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); xmm0 = (rax) as u128; ymm0_hi = 0; let bc_v: u128 = crate::simd_ops::vpbroadcastq((xmm0 as u64)); let (y_lo, y_hi): (u128, u128) = (bc_v, bc_v); xmm0 = y_lo; ymm0_hi = y_hi; let (y_lo, y_hi): (u128, u128) = (crate::simd_ops::vpaddq(xmm0, xmm7), crate::simd_ops::vpaddq(ymm0_hi, ymm7_hi)); xmm0 = y_lo; ymm0_hi = y_hi; let (y_lo, y_hi): (u128, u128) = (crate::simd_ops::vpmullq(xmm0, xmm4), crate::simd_ops::vpmullq(ymm0_hi, ymm4_hi)); xmm1 = y_lo; ymm1_hi = y_hi; let (y_lo, y_hi): (u128, u128) = (crate::simd_ops::vpaddq(xmm1, xmm8), crate::simd_ops::vpaddq(ymm1_hi, ymm8_hi)); xmm2 = y_lo; ymm2_hi = y_hi; let (y_lo, y_hi): (u128, u128) = (crate::simd_ops::vpmullq(xmm2, xmm6), crate::simd_ops::vpmullq(ymm2_hi, ymm6_hi)); xmm3 = y_lo; ymm3_hi = y_hi; let (y_lo, y_hi): (u128, u128) = (crate::simd_ops::vpaddq(xmm3, xmm5), crate::simd_ops::vpaddq(ymm3_hi, ymm5_hi)); xmm0 = y_lo; ymm0_hi = y_hi; let y_addr: u64 = rsp.wrapping_add(rdx.wrapping_mul(8)).wrapping_add(0x240u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); rax = rdx.wrapping_add(0x8); let y_addr: u64 = rsp.wrapping_add(rdx.wrapping_mul(8)).wrapping_add(0x40u64); crate::simd_ops::wu128(y_addr, xmm4); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm4_hi); xmm0 = (rax) as u128; ymm0_hi = 0; let bc_v: u128 = crate::simd_ops::vpbroadcastq((xmm0 as u64)); let (y_lo, y_hi): (u128, u128) = (bc_v, bc_v); xmm0 = y_lo; ymm0_hi = y_hi; let (y_lo, y_hi): (u128, u128) = (crate::simd_ops::vpaddq(xmm0, xmm7), crate::simd_ops::vpaddq(ymm0_hi, ymm7_hi)); xmm0 = y_lo; ymm0_hi = y_hi; let (y_lo, y_hi): (u128, u128) = (crate::simd_ops::vpmullq(xmm0, xmm4), crate::simd_ops::vpmullq(ymm0_hi, ymm4_hi)); xmm1 = y_lo; ymm1_hi = y_hi; let (y_lo, y_hi): (u128, u128) = (crate::simd_ops::vpaddq(xmm1, xmm8), crate::simd_ops::vpaddq(ymm1_hi, ymm8_hi)); xmm2 = y_lo; ymm2_hi = y_hi; let (y_lo, y_hi): (u128, u128) = (crate::simd_ops::vpmullq(xmm2, xmm6), crate::simd_ops::vpmullq(ymm2_hi, ymm6_hi)); xmm3 = y_lo; ymm3_hi = y_hi; let (y_lo, y_hi): (u128, u128) = (crate::simd_ops::vpaddq(xmm3, xmm5), crate::simd_ops::vpaddq(ymm3_hi, ymm5_hi)); xmm0 = y_lo; ymm0_hi = y_hi; let y_addr: u64 = rsp.wrapping_add(rdx.wrapping_mul(8)).wrapping_add(0x260u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); rax = rdx.wrapping_add(0xc); let y_addr: u64 = rsp.wrapping_add(rdx.wrapping_mul(8)).wrapping_add(0x60u64); crate::simd_ops::wu128(y_addr, xmm4); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm4_hi); xmm0 = (rax) as u128; ymm0_hi = 0; let bc_v: u128 = crate::simd_ops::vpbroadcastq((xmm0 as u64)); let (y_lo, y_hi): (u128, u128) = (bc_v, bc_v); xmm0 = y_lo; ymm0_hi = y_hi; let (y_lo, y_hi): (u128, u128) = (crate::simd_ops::vpaddq(xmm0, xmm7), crate::simd_ops::vpaddq(ymm0_hi, ymm7_hi)); xmm0 = y_lo; ymm0_hi = y_hi; let (y_lo, y_hi): (u128, u128) = (crate::simd_ops::vpmullq(xmm0, xmm4), crate::simd_ops::vpmullq(ymm0_hi, ymm4_hi)); xmm1 = y_lo; ymm1_hi = y_hi; let (y_lo, y_hi): (u128, u128) = (crate::simd_ops::vpaddq(xmm1, xmm8), crate::simd_ops::vpaddq(ymm1_hi, ymm8_hi)); xmm2 = y_lo; ymm2_hi = y_hi; let (y_lo, y_hi): (u128, u128) = (crate::simd_ops::vpmullq(xmm2, xmm6), crate::simd_ops::vpmullq(ymm2_hi, ymm6_hi)); xmm3 = y_lo; ymm3_hi = y_hi; let (y_lo, y_hi): (u128, u128) = (crate::simd_ops::vpaddq(xmm3, xmm5), crate::simd_ops::vpaddq(ymm3_hi, ymm5_hi)); xmm0 = y_lo; ymm0_hi = y_hi; let y_addr: u64 = rsp.wrapping_add(rdx.wrapping_mul(8)).wrapping_add(0x280u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); let y_addr: u64 = rsp.wrapping_add(rdx.wrapping_mul(8)).wrapping_add(0x80u64); crate::simd_ops::wu128(y_addr, xmm4); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm4_hi); rdx = ((rdx).wrapping_add(0x10u64)) as u64; fl_l = rdx; fl_r = rcx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 9; continue; } bb = 10; continue; }
            10 => { xmm8 = (crate::simd_ops::ru128(rsp.wrapping_add(0x420u64))) as u128; ymm8_hi = 0; xmm7 = (crate::simd_ops::ru128(rsp.wrapping_add(0x430u64))) as u128; ymm7_hi = 0; bb = 11; continue; }
            11 => { fl_l = rdx; fl_r = rbx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 14; continue; } bb = 12; continue; }
            12 => { rax = ((crate::simd_ops::ru16(r15.wrapping_add(0xau64))) as u32) as u32 as u64; rdi = rsp.wrapping_add(0x20); r8 = rdx; rdi = ((rdi.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64; r8 = (((r8) as i64).wrapping_mul((r11) as i64) as u64) as u64; r10 = r11; rcx = rbx; r8 = ((r8).wrapping_add(rax)) as u64; rax = r11; r8 = (((r8) as i64).wrapping_mul((r12) as i64) as u64) as u64; r10 = (((r10) as i64).wrapping_mul((r12) as i64) as u64) as u64; r8 = ((r8).wrapping_add(r9)) as u64; fl_l = rcx; fl_r = rdx; rcx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; while rcx != 0 { crate::simd_ops::wu64(rdi, rax); rdi = rdi.wrapping_add(8); rcx = rcx.wrapping_sub(1); } bb = 13; continue; }
            13 => { crate::simd_ops::wu64(rsp.wrapping_add(rdx.wrapping_mul(8)).wrapping_add(0x220u64), r8); rdx = ((rdx).wrapping_add(1)) as u64; r8 = ((r8).wrapping_add(r10)) as u64; fl_l = rdx; fl_r = rbx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 13; continue; } bb = 14; continue; }
            14 => { rdx = 0; fl_l = 0; fl_r = 0; rax = r14; let d = (rbx) as u128; let n = ((rdx as u128) << 64) | (rax as u128); if d != 0 { rax = (n / d) as u64; rdx = (n % d) as u64; } rcx = r13; crate::simd_ops::lk_add_u64(rsp.wrapping_add(rbx.wrapping_mul(8)).wrapping_add(0x18u64), (rdx) as u64); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x440u64))) as u128; ymm6_hi = 0; rdi = rax; rdx = 0; fl_l = 0; fl_r = 0; rdi = (((rdi) | (rsi))) as u64; let d = (rbx) as u128; let n = ((rdx as u128) << 64) | (rax as u128); if d != 0 { rax = (n / d) as u64; rdx = (n % d) as u64; } rcx = rdi; r13 = (0xbf58476d1ce4e5b9u64) as u64; rcx = ((rcx).wrapping_shr(30u32)) as u64; r8 = rdx; rcx = (((rcx) ^ (rdi))) as u64; rcx = (((rcx) as i64).wrapping_mul((r13) as i64) as u64) as u64; crate::simd_ops::lk_sub_u64(rsp.wrapping_add(rdx.wrapping_mul(8)).wrapping_add(0x20u64), (1) as u64); r10 = (crate::simd_ops::ru64(rsp.wrapping_add(rdx.wrapping_mul(8)).wrapping_add(0x220u64))) as u64; rax = rcx; rax = ((rax).wrapping_shr(27u32)) as u64; rax = (((rax) ^ (rcx))) as u64; rcx = (0x94d049bb133111ebu64) as u64; rax = (((rax) as i64).wrapping_mul((rcx) as i64) as u64) as u64; rdi = rax; rdi = ((rdi).wrapping_shr(31u32)) as u64; rdi = (((rdi) ^ (rax))) as u64; fl_l = rsi; fl_r = r14; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 23; continue; } bb = 15; continue; }
            15 => { r9 = rbx.wrapping_sub(0x1); r11 = 0x11; r15 = rcx; bb = 16; continue; }
            16 => { rdx = rsi; fl_l = ((((rdx as u32)) & (0x7u32))) as u32 as u64; fl_r = fl_l; rdx = ((fl_l as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 18; continue; } bb = 17; continue; }
            17 => { fl_l = rdi; fl_r = rdi; rdi = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { r11 } else { rdi }) as u64; rax = rdi; rax = ((rax).wrapping_shr(30u32)) as u64; rax = (((rax) ^ (rdi))) as u64; rax = (((rax) as i64).wrapping_mul((r13) as i64) as u64) as u64; rcx = rax; rcx = ((rcx).wrapping_shr(27u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) as i64).wrapping_mul((r15) as i64) as u64) as u64; rdi = rcx; rdi = ((rdi).wrapping_shr(31u32)) as u64; fl_l = (((rdi) ^ (rcx))) as u64; fl_r = fl_l; rdi = ((fl_l as u64)) as u64; bb = 18; continue; }
            18 => { rax = ((rdx.wrapping_mul(8)) as u64) as u64; rax = ((rdi).wrapping_shr((((rax) as u32) & 63))) as u64; rax = (((rax) & (r9))) as u64; fl_l = (crate::simd_ops::ru64(rsp.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x20u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 21; continue; } bb = 19; continue; }
            19 => { bb = 20; continue; }
            20 => { rcx = rax.wrapping_add(0x1); rax = 0; fl_l = 0; fl_r = 0; fl_l = rcx; fl_r = rbx; rax = (if (((fl_l as u64)) != ((fl_r as u64))) { rcx } else { rax }) as u64; fl_l = (crate::simd_ops::ru64(rsp.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x20u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 20; continue; } bb = 21; continue; }
            21 => { rdx = (crate::simd_ops::ru64(rsp.wrapping_add(r8.wrapping_mul(8)).wrapping_add(0x220u64))) as u64; rsi = ((rsi).wrapping_add(1)) as u64; crate::simd_ops::lk_sub_u64(rsp.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x20u64), (1) as u64); rcx = ((rdx.wrapping_add(r12)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(r8.wrapping_mul(8)).wrapping_add(0x220u64), rcx); r8 = rax; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x220u64))) as u64; crate::simd_ops::wu64(rdx, rcx); fl_l = rsi; fl_r = r14; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 16; continue; } bb = 22; continue; }
            22 => { r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x488u64))) as u64; bb = 23; continue; }
            23 => { rax = (crate::simd_ops::ru64(r15.wrapping_add(0x10u64))) as u64; r11 = rsp.wrapping_add(0x450); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(r8.wrapping_mul(8)).wrapping_add(0x220u64))) as u64; rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x40u64))) as u64; crate::simd_ops::wu64(rcx, rax); crate::simd_ops::wu64(r15.wrapping_add(0x10u64), r10); rsp = r11; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142536150(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142536aa0; const P1: u64 = 0x143a095c0; const P2: u64 = 0x143a09840; const P3: u64 = 0x14253b2a0;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; r14 = (crate::simd_ops::ru64(rcx.wrapping_add(0x30u64))) as u64; rsi = rcx.wrapping_add(0x30); r8 = (crate::simd_ops::ru64(rcx.wrapping_add(0x40u64))) as u64; rbx = rcx; rbp = (crate::simd_ops::ru64(r14)) as u64; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x38u64))) as u64; r9 = rcx.wrapping_add(0x30); crate::simd_ops::wu64(r8.wrapping_add(0x38u64), rax); bb = 3; continue; }
            2 => { r9 = rsi; bb = 3; continue; }
            3 => { r10 = (crate::simd_ops::ru64(rcx.wrapping_add(0x38u64))) as u64; r8 = rcx.wrapping_add(0x40); r15 = rcx.wrapping_add(0x40); fl_l = r10; fl_r = r10; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x40u64))) as u64; r15 = r8; crate::simd_ops::wu64(r10.wrapping_add(0x40u64), rax); rsi = r9; bb = 5; continue; }
            5 => { fl_l = rbx; fl_r = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x40u64))) as u64; crate::simd_ops::wu64(rdx.wrapping_add(0x8u64), rax); bb = 7; continue; }
            7 => { fl_l = rbx; fl_r = (crate::simd_ops::ru64(rdx)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 9; continue; } bb = 8; continue; }
            8 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x38u64))) as u64; rcx = r14; crate::simd_ops::wu64(rdx, rax); rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); bb = 9; continue; }
            9 => { crate::simd_ops::lk_sub_u64(r14.wrapping_add(0xbd0u64), (1) as u64); rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::lk_and_u8(rbx.wrapping_add(0xeu64), (0xfeu8) as u8); crate::simd_ops::wu64(rbx.wrapping_add(0x38u64), rax); rbx = ((rbx).wrapping_sub(1)) as u64; crate::simd_ops::wu64(r15, rax); rbx = (((rbx) & (0xfffffffffe000000u64))) as u64; crate::simd_ops::wu64(rsi, rax); rax = P1; crate::simd_ops::lk_add_u64(rbx.wrapping_add(0xc0u64), (1) as u64); rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x3a0u64))) as u64; rcx = ((rcx).wrapping_add(0x100u64)) as u64; fl_l = rcx; fl_r = rax; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 15; continue; } bb = 10; continue; }
            10 => { rax = P2; fl_l = rcx; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 15; continue; } bb = 11; continue; }
            11 => { rdx = 0x1; let xa_src: u64 = rdx; let xa_old: u64 = crate::simd_ops::atomic_xadd_u64(rcx.wrapping_add(0x18u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rdx = (xa_old) as u64; fl_l = ((rdx).wrapping_add(1)) as u64; rdx = ((fl_l as u64)) as u64; bb = 12; continue; }
            12 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; fl_l = rax; fl_r = rdx; if ((((fl_l as u64)) as i64) >= (((fl_r as u64)) as i64)) { bb = 14; continue; } bb = 13; continue; }
            13 => { fl_l = rax; let cx_src: u64 = rdx; let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u64(rcx.wrapping_add(0x10u64), fl_l as u64, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u64)) as u64; } if (((fl_l as u64)) != ((fl_r as u64))) { bb = 12; continue; } bb = 14; continue; }
            14 => { fl_l = (crate::simd_ops::lk_add_u64(rcx, (1) as u64)) as u64; bb = 18; continue; }
            15 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x18u64))) as u64; rax = ((rax).wrapping_add(1)) as u64; crate::simd_ops::wu64(rcx.wrapping_add(0x18u64), rax); fl_l = rax; fl_r = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; if ((((fl_l as u64)) as i64) <= (((fl_r as u64)) as i64)) { bb = 17; continue; } bb = 16; continue; }
            16 => { crate::simd_ops::wu64(rcx.wrapping_add(0x10u64), rax); bb = 17; continue; }
            17 => { fl_l = (crate::simd_ops::lk_add_u64(rcx, (1) as u64)) as u64; bb = 18; continue; }
            18 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0xc0u64))) as u64; fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(0xd0u64))) as u64; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 20; continue; } bb = 19; continue; }
            19 => { rdx = rbp.wrapping_add(0x20); rcx = rbx; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 20; continue; }
            20 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425362a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xe; const P1: u64 = 0x30; const P2: u64 = 0x1425368c0; const P3: u64 = 0x28; const P4: u64 = 0xa; const P5: u64 = 0x1434254d8; const P6: u64 = 0x1425386a0; const P7: u64 = 0x20; const P8: u64 = 0x18; const P9: u64 = 0x10; const P10: u64 = 0xf;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(P0))) as u8 as u64; fl_r = (0x1u8) as u8 as u64; rbx = rcx; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 13; continue; } bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; r8 = rbx; rcx = ((rcx).wrapping_add(0xb00u64)) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rbx.wrapping_add(P3))) as u64; fl_l = rax; fl_r = (0xfffffffffffffffcu64) as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 2; continue; }
            2 => { rdx = rax; rcx = rax; rdx = (((rdx) & (0xfffffffffffffffcu64))) as u64; rcx = ((((rcx as u32)) & (0x3u32))) as u32 as u64; fl_l = rax; let cx_src: u64 = rcx; let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u64(rbx.wrapping_add(P3), fl_l as u64, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u64)) as u64; } if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 3; continue; }
            3 => { fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rdx)) as u64; rcx = 0x1; r8 = ((crate::simd_ops::ru16(rbx.wrapping_add(P4))) as u32) as u32 as u64; r9 = rdx; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 5; continue; }
            5 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 8; continue; } bb = 6; continue; }
            6 => { r9 = rax; rcx = (((rcx as u32)).wrapping_add(1)) as u32 as u64; rax = (crate::simd_ops::ru64(rax)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 7; continue; }
            7 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 9; continue; } bb = 8; continue; }
            8 => { rdx = P5; rcx = 0xe; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); bb = 10; continue; }
            9 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(P7))) as u64; crate::simd_ops::wu64(r9, rax); fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(P8))) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(P8), ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32); crate::simd_ops::wu64(rbx.wrapping_add(P7), rdx); bb = 10; continue; }
            10 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(P7))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 11; continue; }
            11 => { fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(P9))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 13; continue; } bb = 12; continue; }
            12 => { fl_l = (crate::simd_ops::lk_and_u8(rbx.wrapping_add(P10), (0xfeu8) as u8)) as u8 as u64; fl_r = fl_l; crate::simd_ops::wu64(rbx.wrapping_add(P9), rax); crate::simd_ops::wu64(rbx.wrapping_add(P7), (0x0u64) as u64); bb = 13; continue; }
            13 => { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142536420(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xb18; const P1: u64 = 0x142533040;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = rcx; r14 = 0; fl_l = 0; fl_r = 0; bb = 1; continue; }
            1 => { rbx = (crate::simd_ops::ru64(rdi.wrapping_add(P0))) as u64; bb = 2; continue; }
            2 => { fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 3; continue; }
            3 => { rax = rbx; fl_l = rax; let cx_src: u64 = r14; let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u64(rdi.wrapping_add(P0), fl_l as u64, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u64)) as u64; } if (((fl_l as u64)) == ((fl_r as u64))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rbx = rax; bb = 2; continue; }
            5 => { rbp = (rbp & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 6; continue; }
            6 => { rsi = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 9; continue; } bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(P0))) as u64; rbp = (rbp & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 8; continue; }
            8 => { crate::simd_ops::wu64(rbx, rax); fl_l = rax; let cx_src: u64 = rbx; let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u64(rdi.wrapping_add(P0), fl_l as u64, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u64)) as u64; } if (((fl_l as u64)) != ((fl_r as u64))) { bb = 8; continue; } bb = 9; continue; }
            9 => { rbx = rsi; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 6; continue; } bb = 10; continue; }
            10 => { fl_l = ((rbp as u8)) as u8 as u64; fl_r = ((rbp as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 12; continue; } bb = 11; continue; }
            11 => { bb = 1; continue; }
            12 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425364c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x1425364e0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    r8 = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    r9 = 0;
    fl_l = 0; fl_r = 0;
    return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142536640(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1c; const P1: u64 = 0x142536cb0; const P2: u64 = 0xe; const P3: u64 = 0x41; const P4: u64 = 0x10; const P5: u64 = 0x38; const P6: u64 = 0x40; const P7: u64 = 0x8; const P8: u64 = 0x142536aa0; const P9: u64 = 0xbd0;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rcx; rax = 0x80000000; rcx = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64; r10 = rdx; fl_l = rcx; fl_r = rax; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru32(rdx)) as u32 as u64; fl_l = ((rcx).wrapping_shl(16u32)) as u64; rcx = ((fl_l as u64)) as u64; bb = 2; continue; }
            2 => { rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); r9 = (((rax as u8)) as u32) as u32 as u64; rax = ((crate::simd_ops::ru8(r10.wrapping_add(P2))) as u32) as u32 as u64; rdx = ((r9.wrapping_add(P3)) as u64) as u64; rdx = ((r9.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64; fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(rdx.wrapping_mul(8)).wrapping_add(P4))) as u64; fl_r = (0x20010u64) as u64; rdx = ((rbx.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64; rcx = (rcx & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); rax = (rax & !0xffu64) | ((((((rax as u8)) & (0xfeu8))) as u8 as u64)); rcx = (rcx & !0xffu64) | ((((((rcx as u8)) | ((rax as u8)))) as u8 as u64)); crate::simd_ops::wu8(r10.wrapping_add(P2), ((rcx as u8)) as u8); rax = (crate::simd_ops::ru64(rdx)) as u64; crate::simd_ops::wu64(r10.wrapping_add(P5), rax); crate::simd_ops::wu64(r10.wrapping_add(P6), (0x0u64) as u64); rax = (crate::simd_ops::ru64(rdx)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu64(rax.wrapping_add(P6), r10); bb = 5; continue; }
            4 => { crate::simd_ops::wu64(rdx.wrapping_add(P7), r10); bb = 5; continue; }
            5 => { rcx = rbx; crate::simd_ops::wu64(rdx, r10); rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::lk_add_u64(rbx.wrapping_add(P9), (1) as u64); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425366d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x28; const P1: u64 = 0xa; const P2: u64 = 0x1434254d8; const P3: u64 = 0x1425386a0; const P4: u64 = 0x20; const P5: u64 = 0x18;
    let (mut rax, mut r10, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; r8 = rcx; bb = 1; continue; }
            1 => { r9 = rax; rdx = rax; r9 = (((r9) & (0xfffffffffffffffcu64))) as u64; rdx = ((((rdx as u32)) & (0x3u32))) as u32 as u64; fl_l = rax; let cx_src: u64 = rdx; let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u64(rcx.wrapping_add(P0), fl_l as u64, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u64)) as u64; } if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 2; continue; }
            2 => { fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(r9)) as u64; r10 = r9; rdx = ((crate::simd_ops::ru16(rcx.wrapping_add(P1))) as u32) as u32 as u64; rcx = 0x1; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 4; continue; }
            4 => { bb = 5; continue; }
            5 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 8; continue; } bb = 6; continue; }
            6 => { r10 = rax; rcx = (((rcx as u32)).wrapping_add(1)) as u32 as u64; rax = (crate::simd_ops::ru64(rax)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 7; continue; }
            7 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 9; continue; } bb = 8; continue; }
            8 => { rdx = P2; rcx = 0xe; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            9 => { rax = (crate::simd_ops::ru64(r8.wrapping_add(P4))) as u64; crate::simd_ops::wu64(r10, rax); fl_l = (crate::simd_ops::ru32(r8.wrapping_add(P5))) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(P5), ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32); crate::simd_ops::wu64(r8.wrapping_add(P4), r9); bb = 10; continue; }
            10 => { return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142536750(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x28;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); r11 = (((r8 as u8)) as u32) as u32 as u64; r10 = ((((rdx as u32)) as i32 as i64) as u64) as u64; bb = 1; continue; }
            1 => { r9 = 0; fl_l = 0; fl_r = 0; bb = 2; continue; }
            2 => { r8 = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rdx = r8; rax = ((r8 as u32)) as u32 as u64; rdx = (((rdx) & (0xfffffffffffffffcu64))) as u64; rax = ((((rax as u32)) & (0x3u32))) as u32 as u64; rdx = (((rdx) | (r10))) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 5; continue; } bb = 3; continue; }
            3 => { fl_l = r9; fl_r = (0x4u64) as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 1; continue; } bb = 4; continue; }
            4 => { fl_l = ((r9).wrapping_add(1)) as u64; r9 = ((fl_l as u64)) as u64; bb = 2; continue; }
            5 => { fl_l = ((r10 as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 9; continue; } bb = 6; continue; }
            6 => { fl_l = ((r11 as u8)) as u8 as u64; fl_r = ((r11 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x3; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 9; continue; } bb = 8; continue; }
            8 => { rax = r8; fl_l = rax; let cx_src: u64 = rdx; let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u64(rcx.wrapping_add(P0), fl_l as u64, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u64)) as u64; } if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 9; continue; }
            9 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x10u64))) as u64; return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425367c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    r8 = (((r8) as i64).wrapping_mul((r9) as i64) as u64) as u64;
    rax = ((rdx.wrapping_add(r8)) as u64) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425367d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142536aa0;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; r9 = (crate::simd_ops::ru64(r8)) as u64; rdi = r8; r11 = rdx; r10 = rcx; fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = 0; fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            2 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx); rbx = 0; fl_l = 0; fl_r = 0; bb = 3; continue; }
            3 => { crate::simd_ops::wu64(r9.wrapping_add(0x30u64), r10); bb = 4; continue; }
            4 => { rcx = 0; fl_l = 0; fl_r = 0; bb = 5; continue; }
            5 => { rdx = (crate::simd_ops::ru64(r9.wrapping_add(0x28u64))) as u64; r8 = rdx; rax = ((rdx as u32)) as u32 as u64; r8 = (((r8) & (0xfffffffffffffffcu64))) as u64; rax = ((((rax as u32)) & (0x3u32))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 8; continue; } bb = 6; continue; }
            6 => { fl_l = rcx; fl_r = (0x4u64) as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 4; continue; } bb = 7; continue; }
            7 => { fl_l = ((rcx).wrapping_add(1)) as u64; rcx = ((fl_l as u64)) as u64; bb = 5; continue; }
            8 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 11; continue; } bb = 9; continue; }
            9 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x3; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 11; continue; } bb = 10; continue; }
            10 => { rax = rdx; fl_l = rax; let cx_src: u64 = r8; let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u64(r9.wrapping_add(0x28u64), fl_l as u64, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u64)) as u64; } if (((fl_l as u64)) != ((fl_r as u64))) { bb = 5; continue; } bb = 11; continue; }
            11 => { r9 = (crate::simd_ops::ru64(r9.wrapping_add(0x38u64))) as u64; rbx = ((rbx).wrapping_add(1)) as u64; fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } bb = 12; continue; }
            12 => { rcx = (crate::simd_ops::ru64(r11.wrapping_add(0x8u64))) as u64; rax = (crate::simd_ops::ru64(rdi)) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 14; continue; } bb = 13; continue; }
            13 => { crate::simd_ops::wu64(r11, rax); rdx = r11; rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; rcx = r10; crate::simd_ops::wu64(r11.wrapping_add(0x8u64), rax); rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            14 => { crate::simd_ops::wu64(rcx.wrapping_add(0x38u64), rax); rax = rbx; rcx = (crate::simd_ops::ru64(r11.wrapping_add(0x8u64))) as u64; rdx = (crate::simd_ops::ru64(rdi)) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu64(rdx.wrapping_add(0x40u64), rcx); rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(r11.wrapping_add(0x8u64), rcx); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142536a00(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x40; const P1: u64 = 0x30; const P2: u64 = 0x38; const P3: u64 = 0x8; const P4: u64 = 0x142536aa0; const P5: u64 = 0xbd0; const P6: u64 = 0xe;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; r8 = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; rbx = rdx; rsi = (crate::simd_ops::ru64(rdx.wrapping_add(P1))) as u64; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(P2))) as u64; crate::simd_ops::wu64(r8.wrapping_add(P2), rax); bb = 2; continue; }
            2 => { r9 = (crate::simd_ops::ru64(rdx.wrapping_add(P2))) as u64; r8 = ((rdx.wrapping_add(P0)) as u64) as u64; r14 = ((rdx.wrapping_add(P0)) as u64) as u64; fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; r14 = r8; crate::simd_ops::wu64(r9.wrapping_add(P0), rax); bb = 4; continue; }
            4 => { fl_l = rbx; fl_r = (crate::simd_ops::ru64(rcx.wrapping_add(P3))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; crate::simd_ops::wu64(rcx.wrapping_add(P3), rax); bb = 6; continue; }
            6 => { fl_l = rbx; fl_r = (crate::simd_ops::ru64(rcx)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 8; continue; } bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(P2))) as u64; rdx = rcx; crate::simd_ops::wu64(rcx, rax); rcx = rsi; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 8; continue; }
            8 => { crate::simd_ops::lk_sub_u64(rsi.wrapping_add(P5), (1) as u64); rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::lk_and_u8(rbx.wrapping_add(P6), (0xfeu8) as u8); rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(P2), rax); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu64(r14, rax); rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142536c20(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xe; const P1: u64 = 0x1c; const P2: u64 = 0x142536cb0; const P3: u64 = 0x82;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(P0))) as u8 as u64; fl_r = (0x1u8) as u8 as u64;
    r9 = rcx;
    if !(((((fl_l as u8)) & ((fl_r as u8))) == 0)) { rax = (rax & !0xffu64) | (((0x4au8) as u8 as u64)); } else { rcx = (crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32 as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); }
    rax = (((rax as u8)) as u32) as u32 as u64;
    rax = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64;
    rax = ((rax.wrapping_add(P3)) as u64) as u64;
    rax = ((r9.wrapping_add(rax.wrapping_mul(8))) as u64) as u64;
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142536c60(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xe; const P1: u64 = 0x1c; const P2: u64 = 0x142536cb0; const P3: u64 = 0x30;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(P0))) as u8 as u64; fl_r = (0x1u8) as u8 as u64;
    r9 = rcx;
    if !(((((fl_l as u8)) & ((fl_r as u8))) == 0)) { rax = (rax & !0xffu64) | (((0x4au8) as u8 as u64)); } else { rcx = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); }
    rax = (((rax as u8)) as u32) as u32 as u64;
    rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64;
    rax = (crate::simd_ops::ru64(r9.wrapping_add(P3))) as u64;
    rax = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64;
    rax = ((rax).wrapping_add(0x410u64)) as u64;
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142536d20(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x10;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; fl_r = (0x20000u64) as u64;
    rax = (rax & !0xffu64) | (((if (((fl_l as u64)) > ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64));
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142536d40(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x10;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; fl_r = (0x20010u64) as u64;
    rax = (rax & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64));
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142536d60(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x10;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; fl_r = (0x20008u64) as u64;
    rax = (rax & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64));
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142536ee0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0xe;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = (crate::simd_ops::lk_and_u8(rcx.wrapping_add(P0), (0xfdu8) as u8)) as u8 as u64; fl_r = fl_l;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142536ef0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0xe;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    r8 = ((crate::simd_ops::ru8(rcx.wrapping_add(P0))) as u32) as u32 as u64;
    r8 = (r8 & !0xffu64) | ((((((r8 as u8)) ^ ((rdx as u8)))) as u8 as u64));
    r8 = (r8 & !0xffu64) | ((((((r8 as u8)) & (0x1u8))) as u8 as u64));
    fl_l = (crate::simd_ops::lk_xor_u8(rcx.wrapping_add(P0), ((r8 as u8)) as u8)) as u8 as u64; fl_r = fl_l;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142536f20(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142536cb0; const P1: u64 = 0x41;
    let (mut rax, mut r10) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    r10 = rcx;
    rcx = rdx;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    r9 = (((rax as u8)) as u32) as u32 as u64;
    rax = ((r9.wrapping_add(P1)) as u64) as u64;
    rax = ((r9.wrapping_add(rax.wrapping_mul(2))) as u64) as u64;
    rax = ((r10.wrapping_add(rax.wrapping_mul(8))) as u64) as u64;
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425373f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xe; const P1: u64 = 0x30; const P2: u64 = 0x1c; const P3: u64 = 0x142536cb0; const P4: u64 = 0xb00; const P5: u64 = 0x41; const P6: u64 = 0x1425368c0;
    let (mut rax, mut r11, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rax = ((crate::simd_ops::ru8(rcx.wrapping_add(P0))) as u32) as u32 as u64;
    r11 = rcx;
    fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64;
    if !(((((fl_l as u8)) & ((fl_r as u8))) == 0)) { r9 = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; rax = (rax & !0xffu64) | ((((((rax as u8)) & (0xfeu8))) as u8 as u64)); crate::simd_ops::wu8(rcx.wrapping_add(P0), ((rax as u8)) as u8); rcx = (crate::simd_ops::ru32(rcx.wrapping_add(P2))) as u32 as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::lk_or_u8(r11.wrapping_add(0xeu64), (0x1u8) as u8); rdx = ((r9.wrapping_add(P4)) as u64) as u64; rax = (((rax as u8)) as u32) as u32 as u64; r8 = r11; rcx = ((rax.wrapping_add(P5)) as u64) as u64; rcx = ((rax.wrapping_add(rcx.wrapping_mul(2))) as u64) as u64; rcx = ((r9.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; rsp = ((rsp).wrapping_add(0x28u64)) as u64; return crate::simd_ops::tail_stk(P6, rax, rcx, rdx, r8, r9, rsp); }
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142537450(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1434254d8; const P1: u64 = 0x1425386a0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = (((rdx as u8)) as u32) as u32 as u64; rbx = rcx; fl_l = ((rdx as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(0x28u64))) as u64; fl_r = (0xfffffffffffffffcu64) as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x28u64))) as u64; bb = 3; continue; }
            3 => { rdx = rax; rcx = rax; rdx = (((rdx) & (0xfffffffffffffffcu64))) as u64; rcx = ((((rcx as u32)) & (0x3u32))) as u32 as u64; fl_l = rax; let cx_src: u64 = rcx; let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u64(rbx.wrapping_add(0x28u64), fl_l as u64, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u64)) as u64; } if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 4; continue; }
            4 => { fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rdx)) as u64; rcx = 0x1; r8 = ((crate::simd_ops::ru16(rbx.wrapping_add(0xau64))) as u32) as u32 as u64; r9 = rdx; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 6; continue; }
            6 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 9; continue; } bb = 7; continue; }
            7 => { r9 = rax; rcx = (((rcx as u32)).wrapping_add(1)) as u32 as u64; rax = (crate::simd_ops::ru64(rax)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 6; continue; } bb = 8; continue; }
            8 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 10; continue; } bb = 9; continue; }
            9 => { rdx = P0; rcx = 0xe; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 11; continue; }
            10 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(r9, rax); fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x18u64))) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x18u64), ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32); crate::simd_ops::wu64(rbx.wrapping_add(0x20u64), rdx); bb = 11; continue; }
            11 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x20u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 20; continue; } bb = 12; continue; }
            12 => { fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 14; continue; } bb = 13; continue; }
            13 => { crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), rcx); bb = 19; continue; }
            14 => { fl_l = ((rdi as u8)) as u8 as u64; fl_r = ((rdi as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 20; continue; } bb = 15; continue; }
            15 => { rax = (crate::simd_ops::ru64(rcx)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 18; continue; } bb = 16; continue; }
            16 => { bb = 17; continue; }
            17 => { rcx = rax; rax = (crate::simd_ops::ru64(rax)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 17; continue; } bb = 18; continue; }
            18 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; crate::simd_ops::wu64(rcx, rax); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), rax); bb = 19; continue; }
            19 => { fl_l = (crate::simd_ops::lk_and_u8(rbx.wrapping_add(0xfu64), (0xfeu8) as u8)) as u8 as u64; fl_r = fl_l; crate::simd_ops::wu64(rbx.wrapping_add(0x20u64), (0x0u64) as u64); bb = 20; continue; }
            20 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142537540(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x28;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); r9 = 0; fl_l = 0; fl_r = 0; rbx = ((((rdx as u32)) as i32 as i64) as u64) as u64; bb = 1; continue; }
            1 => { r11 = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; r10 = r11; rax = ((r11 as u32)) as u32 as u64; r10 = (((r10) & (0xfffffffffffffffcu64))) as u64; rax = ((((rax as u32)) & (0x3u32))) as u32 as u64; r10 = (((r10) | (rbx))) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 4; continue; } bb = 2; continue; }
            2 => { fl_l = r9; fl_r = (0x4u64) as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 9; continue; } bb = 3; continue; }
            3 => { fl_l = ((r9).wrapping_add(1)) as u64; r9 = ((fl_l as u64)) as u64; bb = 1; continue; }
            4 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 8; continue; } bb = 5; continue; }
            5 => { fl_l = ((r8 as u8)) as u8 as u64; fl_r = ((r8 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x3; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 8; continue; } bb = 7; continue; }
            7 => { rax = r11; fl_l = rax; let cx_src: u64 = r10; let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u64(rcx.wrapping_add(P0), fl_l as u64, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u64)) as u64; } if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 8; continue; }
            8 => { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x10u64))) as u64; return rax; }
            9 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x10u64))) as u64; rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425375a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1434257e8; const P1: u64 = 0x142645630; const P2: u64 = 0x143425800; const P3: u64 = 0x142537a30; const P4: u64 = 0x143425850; const P5: u64 = 0x142646360; const P6: u64 = 0x1434258b8; const P7: u64 = 0x142534300; const P8: u64 = 0x142646500; const P9: u64 = 0x1434258c8;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 25>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rbp = rsp.wrapping_sub(0x57); rsp = ((rsp).wrapping_sub(0xc0u64)) as u64; rbx = rcx; rdi = P0; rcx = rdi; rax = rbp.wrapping_sub(0x49); fl_l = rcx; fl_r = rax; rcx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; rdx = rbp.wrapping_sub(0x49); r8 = 0x41; bb = 1; continue; }
            1 => { fl_l = r8; fl_r = (0x1u64) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rax = ((crate::simd_ops::ru8(rcx.wrapping_add(rdx))) as u32) as u32 as u64; r8 = ((r8).wrapping_sub(1)) as u64; crate::simd_ops::wu8(rdx, ((rax as u8)) as u8); rdx = ((rdx).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(rdx))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 1; continue; } bb = 3; continue; }
            3 => { r8 = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rcx = rbp.wrapping_sub(0x49); crate::simd_ops::wu8(rdx, (0x0u8) as u8); rdx = 0x41; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::ru8(rbp.wrapping_sub(0x49u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 7; continue; } bb = 5; continue; }
            5 => { fl_l = rdx; fl_r = (0x1u64) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rcx = ((rcx).wrapping_add(1)) as u64; rdx = ((rdx).wrapping_sub(1)) as u64; fl_l = (crate::simd_ops::ru8(rcx)) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 5; continue; } bb = 7; continue; }
            7 => { fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 8; continue; }
            8 => { fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 9; continue; }
            9 => { fl_l = (crate::simd_ops::ru8(r8)) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 13; continue; } bb = 10; continue; }
            10 => { fl_l = r8; fl_r = rcx; r8 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; bb = 11; continue; }
            11 => { fl_l = rdx; fl_r = (0x1u64) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 13; continue; } bb = 12; continue; }
            12 => { rax = ((crate::simd_ops::ru8(r8.wrapping_add(rcx))) as u32) as u32 as u64; rdx = ((rdx).wrapping_sub(1)) as u64; crate::simd_ops::wu8(rcx, ((rax as u8)) as u8); rcx = ((rcx).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(r8.wrapping_add(rcx))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 11; continue; } bb = 13; continue; }
            13 => { crate::simd_ops::wu8(rcx, (0x0u8) as u8); bb = 14; continue; }
            14 => { r8 = 0x41; crate::simd_ops::wu8(rbp.wrapping_add(0x7u64), (0x0u8) as u8); rdx = rbp.wrapping_add(0x7); rcx = rbp.wrapping_sub(0x49); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); rax = (((rax as u32)).wrapping_sub(1)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x3f; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 31; continue; } bb = 15; continue; }
            15 => { r9 = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 41; continue; } bb = 16; continue; }
            16 => { rcx = 0x41; rdx = rbp.wrapping_sub(0x49); bb = 17; continue; }
            17 => { fl_l = rcx; fl_r = (0x1u64) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 19; continue; } bb = 18; continue; }
            18 => { rax = ((crate::simd_ops::ru8(rdi)) as u32) as u32 as u64; rcx = ((rcx).wrapping_sub(1)) as u64; rdi = ((rdi).wrapping_add(1)) as u64; crate::simd_ops::wu8(rdx, ((rax as u8)) as u8); rdx = ((rdx).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(rdi)) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 17; continue; } bb = 19; continue; }
            19 => { crate::simd_ops::wu8(rdx, (0x0u8) as u8); rcx = rbp.wrapping_sub(0x49); fl_l = (crate::simd_ops::ru8(rbp.wrapping_sub(0x49u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; rdx = 0x41; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 22; continue; } bb = 20; continue; }
            20 => { fl_l = rdx; fl_r = (0x1u64) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 22; continue; } bb = 21; continue; }
            21 => { rcx = ((rcx).wrapping_add(1)) as u64; rdx = ((rdx).wrapping_sub(1)) as u64; fl_l = (crate::simd_ops::ru8(rcx)) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 20; continue; } bb = 22; continue; }
            22 => { fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 29; continue; } bb = 23; continue; }
            23 => { fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 29; continue; } bb = 24; continue; }
            24 => { fl_l = (crate::simd_ops::ru8(r9)) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 28; continue; } bb = 25; continue; }
            25 => { fl_l = r9; fl_r = rcx; r9 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; bb = 26; continue; }
            26 => { fl_l = rdx; fl_r = (0x1u64) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 28; continue; } bb = 27; continue; }
            27 => { rax = ((crate::simd_ops::ru8(r9.wrapping_add(rcx))) as u32) as u32 as u64; rdx = ((rdx).wrapping_sub(1)) as u64; crate::simd_ops::wu8(rcx, ((rax as u8)) as u8); rcx = ((rcx).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(r9.wrapping_add(rcx))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 26; continue; } bb = 28; continue; }
            28 => { crate::simd_ops::wu8(rcx, (0x0u8) as u8); bb = 29; continue; }
            29 => { r8 = 0x41; crate::simd_ops::wu8(rbp.wrapping_add(0x7u64), (0x0u8) as u8); rdx = rbp.wrapping_add(0x7); rcx = rbp.wrapping_sub(0x49); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); rax = (((rax as u32)).wrapping_sub(1)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x3f; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 41; continue; } bb = 30; continue; }
            30 => { r8 = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rcx = P2; rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 31; continue; }
            31 => { rdi = 0; fl_l = 0; fl_r = 0; r9 = ((rdi as u32)) as u32 as u64; fl_l = (crate::simd_ops::ru8(rbp.wrapping_add(0x7u64))) as u8 as u64; fl_r = ((rdi as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 34; continue; } bb = 32; continue; }
            32 => { fl_l = r9; fl_r = (0x40u64) as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 34; continue; } bb = 33; continue; }
            33 => { r9 = ((r9).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(rbp.wrapping_add(r9).wrapping_add(0x7u64))) as u8 as u64; fl_r = ((rdi as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 32; continue; } bb = 34; continue; }
            34 => { r10 = rdi; fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 37; continue; } bb = 35; continue; }
            35 => { bb = 36; continue; }
            36 => { r8 = ((crate::simd_ops::ru8(rbp.wrapping_add(r10).wrapping_add(0x7u64))) as u32) as u32 as u64; rdx = ((r8.wrapping_sub(0x61u64)) as u32) as u32 as u64; fl_l = ((rdx as u8)) as u8 as u64; fl_r = (0x19u8) as u8 as u64; rax = ((r8.wrapping_sub(0x20u64)) as u32) as u32 as u64; rcx = (((rax as u8)) as u32) as u32 as u64; rcx = (if (((fl_l as u8)) > ((fl_r as u8))) { (r8 as u32) } else { (rcx as u32) }) as u32 as u64; crate::simd_ops::wu8(rbp.wrapping_add(r10).wrapping_sub(0x49u64), ((rcx as u8)) as u8); r10 = ((r10).wrapping_add(1)) as u64; fl_l = r10; fl_r = r9; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 36; continue; } bb = 37; continue; }
            37 => { crate::simd_ops::wu8(rbp.wrapping_add(r9).wrapping_sub(0x49u64), ((rdi as u8)) as u8); fl_l = (crate::simd_ops::ru8(rbp.wrapping_sub(0x49u64))) as u8 as u64; fl_r = ((rdi as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 65; continue; } bb = 38; continue; }
            38 => { rdx = rbp.wrapping_sub(0x49); rcx = P4; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 65; continue; } bb = 39; continue; }
            39 => { rdx = rbp.wrapping_sub(0x49); rcx = P6; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 43; continue; } bb = 40; continue; }
            40 => { crate::simd_ops::wu32(rbx, ((rdi as u32)) as u32); bb = 66; continue; }
            41 => { rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 67; continue; } bb = 42; continue; }
            42 => { crate::simd_ops::wu32(rbx.wrapping_add(0x4u64), (0x1u32) as u32); bb = 67; continue; }
            43 => { rax = rbp.wrapping_sub(0x49); r8 = 0xa; rdx = rbp.wrapping_add(0x67); crate::simd_ops::wu64(rbp.wrapping_add(0x67u64), rax); rcx = rbp.wrapping_sub(0x49); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P8), rax), rax, rcx, rdx, r8, r9, rsp); r8 = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = 0x9; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 46; continue; } bb = 44; continue; }
            44 => { fl_l = ((r8 as u32)) as u32 as u64; fl_r = 0x17; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 46; continue; } bb = 45; continue; }
            45 => { rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x67u64))) as u64; bb = 59; continue; }
            46 => { rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x67u64))) as u64; rdx = ((crate::simd_ops::ru8(rcx)) as u32) as u32 as u64; fl_l = ((rdx as u8)) as u8 as u64; fl_r = (0x4bu8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 48; continue; } bb = 47; continue; }
            47 => { fl_l = ((rcx).wrapping_add(1)) as u64; rcx = ((fl_l as u64)) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x67u64), rcx); bb = 53; continue; }
            48 => { fl_l = ((rdx as u8)) as u8 as u64; fl_r = (0x4du8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 50; continue; } bb = 49; continue; }
            49 => { rax = (((rax as u32)).wrapping_shl(10u32)) as u32 as u64; fl_l = ((rcx).wrapping_add(1)) as u64; rcx = ((fl_l as u64)) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x67u64), rcx); bb = 53; continue; }
            50 => { fl_l = ((rdx as u8)) as u8 as u64; fl_r = (0x47u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 52; continue; } bb = 51; continue; }
            51 => { rax = (((rax as u32)).wrapping_shl(20u32)) as u32 as u64; fl_l = ((rcx).wrapping_add(1)) as u64; rcx = ((fl_l as u64)) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x67u64), rcx); bb = 53; continue; }
            52 => { rax = (rax as u32 as i32 as i64) as u64; rax = ((rax).wrapping_add(0x3ffu64)) as u64; fl_l = ((rax).wrapping_shr(10u32)) as u64; rax = ((fl_l as u64)) as u64; bb = 53; continue; }
            53 => { rdx = ((crate::simd_ops::ru8(rcx)) as u32) as u32 as u64; fl_l = ((rdx as u8)) as u8 as u64; fl_r = (0x49u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 56; continue; } bb = 54; continue; }
            54 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0x1u64))) as u8 as u64; fl_r = (0x42u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 59; continue; } bb = 55; continue; }
            55 => { fl_l = ((rcx).wrapping_add(0x2u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; bb = 58; continue; }
            56 => { fl_l = ((rdx as u8)) as u8 as u64; fl_r = (0x42u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 59; continue; } bb = 57; continue; }
            57 => { fl_l = ((rcx).wrapping_add(1)) as u64; rcx = ((fl_l as u64)) as u64; bb = 58; continue; }
            58 => { crate::simd_ops::wu64(rbp.wrapping_add(0x67u64), rcx); bb = 59; continue; }
            59 => { fl_l = (crate::simd_ops::ru8(rcx)) as u8 as u64; fl_r = ((rdi as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 61; continue; } bb = 60; continue; }
            60 => { crate::simd_ops::wu32(rbx, ((rax as u32)) as u32); bb = 66; continue; }
            61 => { crate::simd_ops::wu32(rbx.wrapping_add(0x4u64), (0x1u32) as u32); fl_l = ((r8 as u32)) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 64; continue; } bb = 62; continue; }
            62 => { fl_l = (crate::simd_ops::ru32(rbx)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 64; continue; } bb = 63; continue; }
            63 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rcx = P9; crate::simd_ops::wu32(rbx, (0x1u32) as u32); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu32(rbx, ((rdi as u32)) as u32); bb = 67; continue; }
            64 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rcx = P9; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 67; continue; }
            65 => { crate::simd_ops::wu32(rbx, (0x1u32) as u32); bb = 66; continue; }
            66 => { crate::simd_ops::wu32(rbx.wrapping_add(0x4u64), (0x2u32) as u32); bb = 67; continue; }
            67 => { r11 = rsp.wrapping_add(0xc0); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x18u64))) as u64; rdi = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rsp = r11; rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425378e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142645630;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    fl_l = rcx; fl_r = rcx;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
    fl_l = rdx; fl_r = rdx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { r8 = 0x41; crate::simd_ops::wu8(rdx, (0x0u8) as u8); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); rax = (((rax as u32)).wrapping_sub(1)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x3f; rax = (rax & !0xffu64) | (((if (((fl_l as u32)) <= ((fl_r as u32))) { 1u8 } else { 0u8 }) as u8 as u64)); rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
    rax = (rax & !0xffu64) | (((0u8) as u8 as u64));
    fl_l = 0; fl_r = 0;
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142537920(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = 0; fl_l = 0; fl_r = 0; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru8(rcx)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 5; continue; } bb = 2; continue; }
            2 => { bb = 3; continue; }
            3 => { fl_l = rax; fl_r = rdx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = ((rax).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(rcx))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 3; continue; } bb = 5; continue; }
            5 => { return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142537950(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r8 = 0x41; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 1; continue; }
            1 => { fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru8(rcx)) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 5; continue; } bb = 3; continue; }
            3 => { fl_l = r8; fl_r = (0x1u64) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rcx = ((rcx).wrapping_add(1)) as u64; r8 = ((r8).wrapping_sub(1)) as u64; fl_l = (crate::simd_ops::ru8(rcx)) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 3; continue; } bb = 5; continue; }
            5 => { fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 6; continue; }
            6 => { fl_l = (crate::simd_ops::ru8(rdx)) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 10; continue; } bb = 7; continue; }
            7 => { fl_l = rdx; fl_r = rcx; rdx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; bb = 8; continue; }
            8 => { fl_l = r8; fl_r = (0x1u64) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 10; continue; } bb = 9; continue; }
            9 => { rax = ((crate::simd_ops::ru8(rdx.wrapping_add(rcx))) as u32) as u32 as u64; r8 = ((r8).wrapping_sub(1)) as u64; crate::simd_ops::wu8(rcx, ((rax as u8)) as u8); rcx = ((rcx).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(rcx))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 8; continue; } bb = 10; continue; }
            10 => { crate::simd_ops::wu8(rcx, (0x0u8) as u8); bb = 11; continue; }
            11 => { return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425379b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x1;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 1; continue; }
            1 => { fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 2; continue; }
            2 => { fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 3; continue; }
            3 => { rax = ((crate::simd_ops::ru8(rdx)) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 7; continue; } bb = 4; continue; }
            4 => { fl_l = rdx; fl_r = rcx; rdx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; bb = 5; continue; }
            5 => { fl_l = r8; fl_r = (0x1u64) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 7; continue; } bb = 6; continue; }
            6 => { crate::simd_ops::wu8(rcx, ((rax as u8)) as u8); r8 = ((r8).wrapping_sub(1)) as u64; rax = ((crate::simd_ops::ru8(rdx.wrapping_add(rcx).wrapping_add(P0))) as u32) as u32 as u64; rcx = ((rcx).wrapping_add(1)) as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 5; continue; } bb = 7; continue; }
            7 => { crate::simd_ops::wu8(rcx, (0x0u8) as u8); bb = 8; continue; }
            8 => { return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142537a00(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0xffffffffffffff9f; const P1: u64 = 0xffffffffffffffe0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = ((rcx.wrapping_add(P0)) as u32) as u32 as u64;
    fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x19u8) as u8 as u64;
    if !((((fl_l as u8)) > ((fl_r as u8)))) { rax = ((rcx.wrapping_add(P1)) as u32) as u32 as u64; return rax; }
    rax = (((rcx as u8)) as u32) as u32 as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142537a30(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143872c34; const P1: u64 = 0x143872c30; const P2: u64 = 0x1425375a0; const P3: u64 = 0x143872bf4; const P4: u64 = 0x143872bf0; const P5: u64 = 0x143857be8; const P6: u64 = 0x143a8a008; const P7: u64 = 0x1434257d0; const P8: u64 = 0x142537b60;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), r8); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r9); rsp = ((rsp).wrapping_sub(0x38u64)) as u64; fl_l = (crate::simd_ops::ru32(P0)) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = P1; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru32(P1)) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 8; continue; } bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru32(P3)) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rcx = P4; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            5 => { fl_l = (crate::simd_ops::ru32(P4)) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 9; continue; } bb = 6; continue; }
            6 => { fl_l = (crate::simd_ops::ru32(P5)) as u32 as u64; fl_r = 0x0; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 8; continue; } bb = 7; continue; }
            7 => { rax = 0x1; let xa_src: u64 = rax; let xa_old: u64 = crate::simd_ops::atomic_xadd_u64(P6, xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(P5)) as u32 as u64; if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { bb = 9; continue; } bb = 8; continue; }
            8 => { r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rax = rsp.wrapping_add(0x48); r8 = P7; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); bb = 9; continue; }
            9 => { rsp = ((rsp).wrapping_add(0x38u64)) as u64; return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142537ad0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143872c34; const P1: u64 = 0x143872c30; const P2: u64 = 0x1425375a0; const P3: u64 = 0x143872bf4; const P4: u64 = 0x143872bf0; const P5: u64 = 0x143857bec; const P6: u64 = 0x143bbd1e8; const P7: u64 = 0x1434258a0; const P8: u64 = 0x142537b60;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; fl_l = (crate::simd_ops::ru32(P0)) as u32 as u64; fl_r = 0x0; rbx = rdx; rdi = rcx; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = P1; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru32(P1)) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 8; continue; } bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru32(P3)) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rcx = P4; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            5 => { fl_l = (crate::simd_ops::ru32(P4)) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 9; continue; } bb = 6; continue; }
            6 => { fl_l = (crate::simd_ops::ru32(P5)) as u32 as u64; fl_r = 0x0; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 8; continue; } bb = 7; continue; }
            7 => { rax = 0x1; let xa_src: u64 = rax; let xa_old: u64 = crate::simd_ops::atomic_xadd_u64(P6, xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(P5)) as u32 as u64; if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { bb = 9; continue; } bb = 8; continue; }
            8 => { r9 = rdi; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbx); r8 = P7; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); bb = 9; continue; }
            9 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142537b60(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1438430a0; const P1: u64 = 0x143425878; const P2: u64 = 0x1405eba60; const P3: u64 = 0x142537f50; const P4: u64 = 0x1405eb9e0; const P5: u64 = 0x142646840; const P6: u64 = 0x142537f30; const P7: u64 = 0x143a632e8; const P8: u64 = 0x142538090;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 79>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x270u64)) as u64; rdi = r9; rbx = r8; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 1; continue; }
            1 => { rax = 0; fl_l = 0; fl_r = 0; fl_l = (crate::simd_ops::ru8(r8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 5; continue; } bb = 2; continue; }
            2 => { fl_l = rax; fl_r = (0x21u64) as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = ((rax).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(r8))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 2; continue; } bb = 4; continue; }
            4 => { fl_l = rax; fl_r = (0x20u64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 14; continue; } bb = 5; continue; }
            5 => { fl_l = (crate::simd_ops::ru64(P0)) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 8; continue; } bb = 6; continue; }
            6 => { rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = (crate::simd_ops::ru64(P0)) as u64; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 8; continue; } bb = 7; continue; }
            7 => { rax = 0; fl_l = 0; fl_r = 0; bb = 9; continue; }
            8 => { rax = 0x1; bb = 9; continue; }
            9 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 14; continue; } bb = 10; continue; }
            10 => { rax = ((crate::exe::teb_self() as u64)) as u64; r8 = P1; r9 = rbx; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); rdx = 0x40; rcx = rsp.wrapping_add(0x30); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 21; continue; } bb = 11; continue; }
            11 => { rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 21; continue; } bb = 12; continue; }
            12 => { rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); r9 = rdi; rdx = rsp.wrapping_add(0x70); r8 = 0x1ff; rcx = (crate::simd_ops::ru64(rax)) as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x2a0u64))) as u64; rcx = (((rcx) | (0x2u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (0x0u64) as u64); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 21; continue; } bb = 13; continue; }
            13 => { rbx = (crate::simd_ops::ru64(P7)) as u64; rax = P8; fl_l = rbx; fl_r = rbx; rcx = rsp.wrapping_add(0x30); rbx = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rax } else { rbx }) as u64; rdx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(rbx, rax, rcx, rdx, r8, r9, rsp); rdx = 0; fl_l = 0; fl_r = 0; rcx = rsp.wrapping_add(0x70); rax = crate::simd_ops::call_stk(rbx, rax, rcx, rdx, r8, r9, rsp); bb = 20; continue; }
            14 => { fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 21; continue; } bb = 15; continue; }
            15 => { rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 21; continue; } bb = 16; continue; }
            16 => { rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); r9 = rdi; rdx = rsp.wrapping_add(0x70); r8 = 0x1ff; rcx = (crate::simd_ops::ru64(rax)) as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x2a0u64))) as u64; rcx = (((rcx) | (0x2u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (0x0u64) as u64); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 21; continue; } bb = 17; continue; }
            17 => { rdi = (crate::simd_ops::ru64(P7)) as u64; rax = P8; fl_l = rdi; fl_r = rdi; rdi = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rax } else { rdi }) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 19; continue; } bb = 18; continue; }
            18 => { rdx = 0; fl_l = 0; fl_r = 0; rcx = rbx; rax = crate::simd_ops::call_stk(rdi, rax, rcx, rdx, r8, r9, rsp); bb = 19; continue; }
            19 => { rdx = 0; fl_l = 0; fl_r = 0; rcx = rsp.wrapping_add(0x70); rax = crate::simd_ops::call_stk(rdi, rax, rcx, rdx, r8, r9, rsp); bb = 20; continue; }
            20 => { rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); bb = 21; continue; }
            21 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x280u64))) as u64; rsp = ((rsp).wrapping_add(0x270u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142537d10(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142537f50; const P1: u64 = 0x1405eb9e0; const P2: u64 = 0x142646840; const P3: u64 = 0x142537f30; const P4: u64 = 0x142646870; const P5: u64 = 0x143a632e8; const P6: u64 = 0x142538090;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 75>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), r8); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r9); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x238u64)) as u64; rdi = r8; rbp = rsp.wrapping_add(0x278); rsi = rdx; rbx = rcx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 8; continue; } bb = 2; continue; }
            2 => { rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rbp); rdx = rsp.wrapping_add(0x30); r9 = rdi; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (0x0u64) as u64); r8 = 0x1ff; rcx = (crate::simd_ops::ru64(rax)) as u64; rcx = (((rcx) | (0x2u64))) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 3; continue; }
            3 => { rcx = 0x1; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = rbx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 6; continue; } bb = 4; continue; }
            4 => { rcx = 0x2; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = rbx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rdx = rsi; rcx = rsp.wrapping_add(0x30); rax = crate::simd_ops::call_stk(rbx, rax, rcx, rdx, r8, r9, rsp); rsp = ((rsp).wrapping_add(0x238u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            6 => { rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru64(P5)) as u64; rcx = P6; fl_l = rax; fl_r = rax; rax = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rcx } else { rax }) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rsp.wrapping_add(0x30); rax = crate::simd_ops::call_stk(rax, rax, rcx, rdx, r8, r9, rsp); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 8; continue; }
            8 => { rsp = ((rsp).wrapping_add(0x238u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); bb = 9; continue; }
            9 => { return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142537e00(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142537f50; const P1: u64 = 0x1405eb9e0; const P2: u64 = 0x142646840; const P3: u64 = 0x142537f30; const P4: u64 = 0x142646870; const P5: u64 = 0x143a632e8; const P6: u64 = 0x142538090;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 71>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x230u64)) as u64; rbp = r9; rdi = r8; rsi = rdx; rbx = rcx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 12; continue; } bb = 2; continue; }
            2 => { rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); r9 = rbp; rdx = rsp.wrapping_add(0x30); r8 = 0x1ff; rcx = (crate::simd_ops::ru64(rax)) as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x260u64))) as u64; rcx = (((rcx) | (0x2u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (0x0u64) as u64); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 3; continue; }
            3 => { rcx = 0x1; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = rbx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 8; continue; } bb = 4; continue; }
            4 => { rcx = 0x2; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = rbx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 8; continue; } bb = 5; continue; }
            5 => { fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { rdx = rsi; rcx = rdi; rax = crate::simd_ops::call_stk(rbx, rax, rcx, rdx, r8, r9, rsp); bb = 7; continue; }
            7 => { rdx = rsi; rcx = rsp.wrapping_add(0x30); rax = crate::simd_ops::call_stk(rbx, rax, rcx, rdx, r8, r9, rsp); bb = 12; continue; }
            8 => { rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 12; continue; } bb = 9; continue; }
            9 => { rbx = (crate::simd_ops::ru64(P5)) as u64; rax = P6; fl_l = rbx; fl_r = rbx; rbx = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rax } else { rbx }) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { rdx = 0; fl_l = 0; fl_r = 0; rcx = rdi; rax = crate::simd_ops::call_stk(rbx, rax, rcx, rdx, r8, r9, rsp); bb = 11; continue; }
            11 => { rdx = 0; fl_l = 0; fl_r = 0; rcx = rsp.wrapping_add(0x30); rax = crate::simd_ops::call_stk(rbx, rax, rcx, rdx, r8, r9, rsp); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 12; continue; }
            12 => { r11 = rsp.wrapping_add(0x230); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x10u64))) as u64; rbp = (crate::simd_ops::ru64(r11.wrapping_add(0x18u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rsp = r11; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); bb = 13; continue; }
            13 => { return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142537f30(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = ((crate::exe::tls_pointer() as u64)) as u64;
    rdx = 0x6310;
    rcx = (crate::simd_ops::ru64(rax)) as u64;
    crate::simd_ops::wu8(rdx.wrapping_add(rcx), (0x0u8) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142537f50(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = ((crate::exe::tls_pointer() as u64)) as u64;
    rdx = 0x6310;
    rcx = (crate::simd_ops::ru64(rax)) as u64;
    fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(rcx))) as u8 as u64; fl_r = (0x0u8) as u8 as u64;
    if !((((fl_l as u8)) == ((fl_r as u8)))) { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; return rax; }
    crate::simd_ops::wu8(rdx.wrapping_add(rcx), (0x1u8) as u8);
    rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64));
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142537f80(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143a8ae08; const P1: u64 = 0x143c4d030; const P2: u64 = 0x142538150; const P3: u64 = 0x142538010; const P4: u64 = 0x143a632e8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = 0x1;
    let xa_src: u64 = rbx; let xa_old: u64 = crate::simd_ops::atomic_xadd_u64(P0, xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64;
    rbx = (xa_old) as u64;
    rax = 0x8000;
    rdi = P1;
    fl_l = rbx; fl_r = rax;
    rcx = rdi;
    rbx = (if (((fl_l as u64)) > ((fl_r as u64))) { rax } else { rbx }) as u64;
    rdx = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu8(rbx.wrapping_add(rdi), (0x0u8) as u8);
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    crate::simd_ops::wu8(rbx.wrapping_add(rdi), (0xau8) as u8);
    rax = P3;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    crate::simd_ops::wu64(P4, rax);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142537fe0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x143a632e8; const P1: u64 = 0x142538090;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = rcx; fl_r = rcx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { crate::simd_ops::wu64(rcx, (0x0u64) as u64); }
    rax = (crate::simd_ops::ru64(P0)) as u64;
    rcx = P1;
    fl_l = rax; fl_r = rax;
    rax = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rcx } else { rax }) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142538010(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142538150; const P1: u64 = 0x142538090;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rdx;
    rdi = rcx;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rdx = rbx;
    rcx = rdi;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(P1, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142538040(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143a8ae08; const P1: u64 = 0x143c4d030; const P2: u64 = 0x142541c70;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rax = 0x1;
    let xa_src: u64 = rax; let xa_old: u64 = crate::simd_ops::atomic_xadd_u64(P0, xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64;
    rax = (xa_old) as u64;
    rcx = 0x8000;
    fl_l = rax; fl_r = rcx;
    rax = (if (((fl_l as u64)) > ((fl_r as u64))) { rcx } else { rax }) as u64;
    rcx = P1;
    rbx = ((rax.wrapping_add(rcx)) as u64) as u64;
    crate::simd_ops::wu8(rbx, (0x0u8) as u8);
    fl_l = (crate::simd_ops::ru8(P1)) as u8 as u64; fl_r = (0x0u8) as u8 as u64;
    if !((((fl_l as u8)) == ((fl_r as u8)))) { rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); }
    crate::simd_ops::wu8(rbx, (0xau8) as u8);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142538090(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143a8ae08; const P1: u64 = 0x143c4d030; const P2: u64 = 0x143a09580; const P3: u64 = 0x14183acbc;
    let (mut rax, mut fl_l, mut fl_r, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x28u64)) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru64(P0)) as u64; fl_r = (0x8000u64) as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 10; continue; } bb = 2; continue; }
            2 => { rax = 0; fl_l = 0; fl_r = 0; fl_l = (crate::simd_ops::ru8(rcx)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = ((rax).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(rax))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 3; continue; } bb = 4; continue; }
            4 => { fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 5; continue; }
            5 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rdi = rax; let xa_src: u64 = rdi; let xa_old: u64 = crate::simd_ops::atomic_xadd_u64(P0, xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rdi = (xa_old) as u64; fl_l = rdi; fl_r = (0x8000u64) as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 9; continue; } bb = 6; continue; }
            6 => { rdx = ((rax.wrapping_add(rdi)) as u64) as u64; r8 = 0x7fff; r8 = ((r8).wrapping_sub(rdi)) as u64; fl_l = rdx; fl_r = (0x8000u64) as u64; r8 = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { r8 }) as u64; rax = P1; rax = ((rax).wrapping_add(rdi)) as u64; fl_l = (crate::simd_ops::ru8(P2)) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 8; continue; } bb = 7; continue; }
            7 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rsi); rdi = rax; rsi = rcx; rcx = r8; while rcx != 0 { crate::simd_ops::wu8(rdi, crate::simd_ops::ru8(rsi)); rdi = rdi.wrapping_add(1); rsi = rsi.wrapping_add(1); rcx = rcx.wrapping_sub(1); } rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
            8 => { rdx = rcx; rcx = rax; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rsp = ((rsp).wrapping_add(0x28u64)) as u64; return crate::simd_ops::tail_stk(P3, rax, rcx, rdx, r8, r9, rsp); }
            9 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; bb = 10; continue; }
            10 => { rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142538150(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142534300; const P1: u64 = 0x1438673c8; const P2: u64 = 0x1426457c8; const P3: u64 = 0x1426457b0; const P4: u64 = 0x143a28fa6; const P5: u64 = 0x142645330; const P6: u64 = 0x1426457d0; const P7: u64 = 0x142646870; const P8: u64 = 0x142646848;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 19; continue; } bb = 1; continue; }
            1 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x50u64)) as u64; fl_l = (crate::simd_ops::ru8(rcx)) as u8 as u64; fl_r = (0x0u8) as u8 as u64; rbx = rcx; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 18; continue; } bb = 2; continue; }
            2 => { rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 18; continue; } bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(P1)) as u64; fl_l = rcx; fl_r = (0xffffffffffffffffu64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 8; continue; } bb = 4; continue; }
            4 => { rcx = 0xfffffff4; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(P1, rax); rcx = rax; fl_l = rax; fl_r = (0xffffffffffffffffu64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 7; continue; } bb = 5; continue; }
            5 => { rdx = rsp.wrapping_add(0x30); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(P1)) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { rdx = (rdx & !0xffu64) | (((0x1u8) as u8 as u64)); crate::simd_ops::wu8(P4, ((rdx as u8)) as u8); bb = 9; continue; }
            7 => { rdx = (rdx & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; crate::simd_ops::wu8(P4, ((rdx as u8)) as u8); bb = 9; continue; }
            8 => { rdx = ((crate::simd_ops::ru8(P4)) as u32) as u32 as u64; bb = 9; continue; }
            9 => { r10 = 0; fl_l = 0; fl_r = 0; r8 = ((r10 as u32)) as u32 as u64; fl_l = (crate::simd_ops::ru8(rbx)) as u8 as u64; fl_r = ((r8 as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 18; continue; } bb = 10; continue; }
            10 => { bb = 11; continue; }
            11 => { r8 = ((r8).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(r8))) as u8 as u64; fl_r = ((r10 as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 11; continue; } bb = 12; continue; }
            12 => { rax = r8.wrapping_sub(0x1); r9 = 0xfffffffd; fl_l = rax; fl_r = r9; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 18; continue; } bb = 13; continue; }
            13 => { crate::simd_ops::wu32(rsp.wrapping_add(0x60u64), ((r10 as u32)) as u32); fl_l = ((rdx as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 15; continue; } bb = 14; continue; }
            14 => { r9 = rsp.wrapping_add(0x60); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r10); rdx = rbx; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); rsp = ((rsp).wrapping_add(0x50u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            15 => { fl_l = rcx; fl_r = (0xffffffffffffffffu64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 17; continue; } bb = 16; continue; }
            16 => { r9 = rsp.wrapping_add(0x60); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r10); rdx = rbx; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp); rsp = ((rsp).wrapping_add(0x50u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            17 => { rcx = 0x2; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P7), rax), rax, rcx, rdx, r8, r9, rsp); rdx = rax; rcx = rbx; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P8), rax), rax, rcx, rdx, r8, r9, rsp); bb = 18; continue; }
            18 => { rsp = ((rsp).wrapping_add(0x50u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); bb = 19; continue; }
            19 => { return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142538260(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143872ed4; const P1: u64 = 0x143872ed0; const P2: u64 = 0x1425375a0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    fl_l = (crate::simd_ops::ru32(P0)) as u32 as u64; fl_r = 0x0;
    if !((((fl_l as u32)) != ((fl_r as u32)))) { rcx = P1; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); }
    rax = (crate::simd_ops::ru64(P1)) as u64;
    fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64;
    if !((((((fl_l as u32)) & ((fl_r as u32))) as i32) >= 0)) { rax = 0; fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
    rax = (rax as u32 as i32 as i64) as u64;
    rax = ((rax).wrapping_shl(10u32)) as u64;
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142538300(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143872c34; const P1: u64 = 0x143872c30; const P2: u64 = 0x1425375a0; const P3: u64 = 0x142537f50; const P4: u64 = 0x1405eb9e0; const P5: u64 = 0x142646840; const P6: u64 = 0x142537f30; const P7: u64 = 0x143a632e8; const P8: u64 = 0x142538090; const P9: u64 = 0x143425890;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 73>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), r8); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r9); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x240u64)) as u64; fl_l = (crate::simd_ops::ru32(P0)) as u32 as u64; fl_r = 0x0; rbx = rcx; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = P1; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x250u64))) as u64; bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru32(P1)) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 8; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu64(rsp.wrapping_add(0x238u64), rdi); rdi = rsp.wrapping_add(0x258); fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 4; continue; }
            4 => { rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 7; continue; } bb = 5; continue; }
            5 => { rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rdi); rdx = rsp.wrapping_add(0x30); r9 = rbx; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (0x0u64) as u64); r8 = 0x1ff; rcx = (crate::simd_ops::ru64(rax)) as u64; rcx = (((rcx) | (0x2u64))) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { rbx = (crate::simd_ops::ru64(P7)) as u64; rax = P8; fl_l = rbx; fl_r = rbx; rcx = P9; rbx = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rax } else { rbx }) as u64; rdx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(rbx, rax, rcx, rdx, r8, r9, rsp); rdx = 0; fl_l = 0; fl_r = 0; rcx = rsp.wrapping_add(0x30); rax = crate::simd_ops::call_stk(rbx, rax, rcx, rdx, r8, r9, rsp); rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); bb = 7; continue; }
            7 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x238u64))) as u64; bb = 8; continue; }
            8 => { rsp = ((rsp).wrapping_add(0x240u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425383f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142646870; const P1: u64 = 0x142537f50; const P2: u64 = 0x143a632e8; const P3: u64 = 0x142538090; const P4: u64 = 0x142537f30;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbp = r9; rdi = r8; rsi = rdx; rbx = rcx; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 1; continue; }
            1 => { rcx = 0x1; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = rbx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 6; continue; } bb = 2; continue; }
            2 => { rcx = 0x2; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = rbx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 6; continue; } bb = 3; continue; }
            3 => { fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rdx = rsi; rcx = rdi; rax = crate::simd_ops::call_stk(rbx, rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            5 => { rdx = rsi; rcx = rbp; rax = crate::simd_ops::call_stk(rbx, rax, rcx, rdx, r8, r9, rsp); bb = 10; continue; }
            6 => { rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 10; continue; } bb = 7; continue; }
            7 => { rbx = (crate::simd_ops::ru64(P2)) as u64; rax = P3; fl_l = rbx; fl_r = rbx; rbx = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rax } else { rbx }) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { rdx = 0; fl_l = 0; fl_r = 0; rcx = rdi; rax = crate::simd_ops::call_stk(rbx, rax, rcx, rdx, r8, r9, rsp); bb = 9; continue; }
            9 => { rdx = 0; fl_l = 0; fl_r = 0; rcx = rbp; rax = crate::simd_ops::call_stk(rbx, rax, rcx, rdx, r8, r9, rsp); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 10; continue; }
            10 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425384a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143a8ae08; const P1: u64 = 0x143c4d030; const P2: u64 = 0x142538150; const P3: u64 = 0x142538010; const P4: u64 = 0x143a632e8; const P5: u64 = 0x143872bf0; const P6: u64 = 0x1a; const P7: u64 = 0x4; const P8: u64 = 0x1425375a0; const P9: u64 = 0x143425860; const P10: u64 = 0x10; const P11: u64 = 0x142538300; const P12: u64 = 0x143872e54; const P13: u64 = 0x143872e50; const P14: u64 = 0x143872e74; const P15: u64 = 0x143857bec; const P16: u64 = 0x143872e70; const P17: u64 = 0x143857be8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = 0x1; let xa_src: u64 = rbx; let xa_old: u64 = crate::simd_ops::atomic_xadd_u64(P0, xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rbx = (xa_old) as u64; rax = 0x8000; rdi = P1; fl_l = rbx; fl_r = rax; rcx = rdi; rbx = (if (((fl_l as u64)) > ((fl_r as u64))) { rax } else { rbx }) as u64; rdx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu8(rbx.wrapping_add(rdi), (0x0u8) as u8); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu8(rbx.wrapping_add(rdi), (0xau8) as u8); rax = P3; rdi = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(P4, rax); rbx = P5; rsi = ((rdi.wrapping_add(P6)) as u32) as u32 as u64; bb = 1; continue; }
            1 => { fl_l = rdi; fl_r = (0x19u64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 4; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(P7))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rcx = rbx; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { r8 = (crate::simd_ops::ru32(rbx)) as u32 as u64; rcx = P9; rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P10))) as u64; rax = crate::simd_ops::call_stk(P11, rax, rcx, rdx, r8, r9, rsp); rdi = ((rdi).wrapping_add(1)) as u64; rbx = ((rbx).wrapping_add(0x20u64)) as u64; fl_l = rsi; fl_r = (0x1u64) as u64; rsi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 5; continue; }
            5 => { fl_l = (crate::simd_ops::ru32(P12)) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rcx = P13; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); bb = 7; continue; }
            7 => { fl_l = (crate::simd_ops::ru32(P14)) as u32 as u64; fl_r = 0x0; rax = (crate::simd_ops::ru32(P13)) as u32 as u64; crate::simd_ops::wu32(P15, ((rax as u32)) as u32); if (((fl_l as u32)) != ((fl_r as u32))) { bb = 9; continue; } bb = 8; continue; }
            8 => { rcx = P16; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru32(P16)) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; crate::simd_ops::wu32(P17, ((rax as u32)) as u32); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142538590(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143872bf0; const P1: u64 = 0x4; const P2: u64 = 0x1425375a0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x19;
    if !((((fl_l as u32)) > ((fl_r as u32)))) { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbx); rax = P0; rbx = ((((rcx as u32)) as i32 as i64) as u64) as u64; rbx = ((rbx).wrapping_shl(5u32)) as u64; rbx = ((rbx).wrapping_add(rax)) as u64; fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(P1))) as u32 as u64; fl_r = 0x0; if !((((fl_l as u32)) != ((fl_r as u32)))) { rcx = rbx; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); } rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
    rax = 0;
    fl_l = 0; fl_r = 0;
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425385e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143872bf0; const P1: u64 = 0x4; const P2: u64 = 0x1425375a0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = ((r8 as u32)) as u32 as u64; rdi = ((rdx as u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x19; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 4; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx); rax = P0; rbx = ((((rcx as u32)) as i32 as i64) as u64) as u64; rbx = ((rbx).wrapping_shl(5u32)) as u64; rbx = ((rbx).wrapping_add(rax)) as u64; fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(P1))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rcx = rbx; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; bb = 5; continue; }
            4 => { rax = 0; fl_l = 0; fl_r = 0; bb = 5; continue; }
            5 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { bb = 7; continue; } bb = 6; continue; }
            6 => { rax = ((rdi as u32)) as u32 as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            7 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; rax = (if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { (rsi as u32) } else { (rax as u32) }) as u32 as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142538650(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143872bf0; const P1: u64 = 0x4; const P2: u64 = 0x1425375a0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x19;
    if !((((fl_l as u32)) > ((fl_r as u32)))) { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbx); rax = P0; rbx = ((((rcx as u32)) as i32 as i64) as u64) as u64; rbx = ((rbx).wrapping_shl(5u32)) as u64; rbx = ((rbx).wrapping_add(rax)) as u64; fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(P1))) as u32 as u64; fl_r = 0x0; if !((((fl_l as u32)) != ((fl_r as u32)))) { rcx = rbx; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); } rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rax = (rax & !0xffu64) | (((if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { 1u8 } else { 0u8 }) as u8 as u64)); rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
    rax = 0;
    fl_l = 0; fl_r = 0;
    fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64;
    rax = (rax & !0xffu64) | (((if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { 1u8 } else { 0u8 }) as u8 as u64));
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142538760(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = 0;
    fl_l = 0; fl_r = 0;
    fl_l = rcx; fl_r = rcx;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { return rax; }
    fl_l = (crate::simd_ops::ru8(rcx)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64;
    if (((fl_l as u8)) == ((fl_r as u8))) { return rax; }
    loop { rax = ((rax).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(rax))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if !((((fl_l as u8)) != ((fl_r as u8)))) { break; } }
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142538780(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143d01c60; const P1: u64 = 0x142538840; const P2: u64 = 0x143bce008; const P3: u64 = 0x143be6098; const P4: u64 = 0x143bde068; const P5: u64 = 0x142645300;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = (crate::simd_ops::ru64(P0)) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rbx = rax; bb = 2; continue; }
            2 => { fl_l = rbx; fl_r = (0x1u64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = 0; fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            4 => { rax = (crate::simd_ops::ru64(P2)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rdi); rdi = 0; fl_l = 0; fl_r = 0; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 5; continue; }
            5 => { fl_l = (crate::simd_ops::ru64(P3)) as u64; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 8; continue; } bb = 6; continue; }
            6 => { rcx = rsp.wrapping_add(0x40); rax = crate::simd_ops::call_stk(rax, rax, rcx, rdx, r8, r9, rsp); rdx = rsp.wrapping_add(0x38); crate::simd_ops::wu16(rsp.wrapping_add(0x38u64), ((rdi as u16)) as u16); rcx = rsp.wrapping_add(0x40); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 11; continue; } bb = 7; continue; }
            7 => { rdi = ((crate::simd_ops::ru16(rsp.wrapping_add(0x38u64))) as u32) as u32 as u64; bb = 11; continue; }
            8 => { fl_l = (crate::simd_ops::ru64(P4)) as u64; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 11; continue; } bb = 9; continue; }
            9 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); rdx = rsp.wrapping_add(0x30); crate::simd_ops::wu8(rsp.wrapping_add(0x30u64), ((rdi as u8)) as u8); rcx = (((rax as u8)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { rdi = ((crate::simd_ops::ru8(rsp.wrapping_add(0x30u64))) as u32) as u32 as u64; bb = 11; continue; }
            11 => { rax = (((rdi as u16)) as u32) as u32 as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; fl_l = rax; fl_r = rbx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 13; continue; } bb = 12; continue; }
            12 => { rdx = 0; fl_l = 0; fl_r = 0; let d = (rbx) as u128; let n = ((rdx as u128) << 64) | (rax as u128); if d != 0 { rax = (n / d) as u64; rdx = (n % d) as u64; } rax = rdx; bb = 13; continue; }
            13 => { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142538840(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143d01c60; const P1: u64 = 0x143872df4; const P2: u64 = 0x143872df0; const P3: u64 = 0x1425375a0; const P4: u64 = 0x142645340; const P5: u64 = 0x143bee0c8; const P6: u64 = 0x142645310; const P7: u64 = 0x143425c98; const P8: u64 = 0x142538300;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x38u64)) as u64; rax = (crate::simd_ops::ru64(P0)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 18; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru32(P1)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx); if (((fl_l as u32)) != ((fl_r as u32))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rcx = P2; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 3; continue; }
            3 => { rax = (((crate::simd_ops::ru32(P2)) as i32 as i64) as u64) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rbx = rax; bb = 17; continue; }
            5 => { rcx = rsp.wrapping_add(0x40); crate::simd_ops::wu32(rsp.wrapping_add(0x40u64), (0x0u32) as u32); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x40u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 16; continue; } bb = 6; continue; }
            6 => { bb = 7; continue; }
            7 => { r8 = (crate::simd_ops::ru64(P5)) as u64; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 8; continue; }
            8 => { rdx = rsp.wrapping_add(0x20); rcx = (((rax as u16)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(r8, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 13; continue; } bb = 9; continue; }
            9 => { fl_l = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; fl_r = (0x0u64) as u64; bb = 12; continue; }
            10 => { rdx = rsp.wrapping_add(0x48); rcx = (((rax as u8)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 13; continue; } bb = 11; continue; }
            11 => { fl_l = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; fl_r = (0x0u64) as u64; bb = 12; continue; }
            12 => { if (((fl_l as u64)) != ((fl_r as u64))) { bb = 15; continue; } bb = 13; continue; }
            13 => { rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x40u64))) as u32 as u64; fl_l = (((rax as u32)).wrapping_add(0xffffffffu32)) as u32 as u64; fl_r = fl_l; rax = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x40u64), ((rax as u32)) as u32); if (((fl_l as u32)) != 0) { bb = 7; continue; } bb = 14; continue; }
            14 => { bb = 16; continue; }
            15 => { rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x40u64))) as u32 as u64; bb = 16; continue; }
            16 => { rbx = ((rax as u32)) as u32 as u64; fl_l = ((rbx).wrapping_add(1)) as u64; rbx = ((fl_l as u64)) as u64; bb = 17; continue; }
            17 => { rdx = rbx; crate::simd_ops::wu64(P0, rbx); rcx = P7; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; bb = 18; continue; }
            18 => { rsp = ((rsp).wrapping_add(0x38u64)) as u64; return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142538920(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142645738; const P1: u64 = 0x142645708; const P2: u64 = 0x1426455e0; const P3: u64 = 0x143425900; const P4: u64 = 0x142537a30; const P5: u64 = 0x143a09620; const P6: u64 = 0x1425408f0; const P7: u64 = 0x143a09600;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = ((rsp).wrapping_sub(0x60u64)) as u64; rsi = rdx; rbx = rcx; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 16; continue; } bb = 2; continue; }
            2 => { fl_l = rdx; fl_r = (0x40000000u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 16; continue; } bb = 3; continue; }
            3 => { fl_l = ((rsi).wrapping_shr(30u32)) as u64; rsi = ((fl_l as u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), rdi); bb = 4; continue; }
            4 => { fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 5; continue; }
            5 => { rdx = 0; fl_l = 0; fl_r = 0; r8 = 0x8000; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 13; continue; } bb = 6; continue; }
            6 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1e7; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 11; continue; } bb = 7; continue; }
            7 => { xmm0 = 0u128; ymm0_hi = 0; let y_addr: u64 = rsp.wrapping_add(0x30u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); crate::simd_ops::wu128(rsp.wrapping_add(0x50u64), (xmm0) as u128); r8 = 0x30; rdx = rsp.wrapping_add(0x30); rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; fl_l = rcx; fl_r = rbx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 12; continue; } bb = 8; continue; }
            8 => { rax = rbx; rax = ((rax).wrapping_sub(rcx)) as u64; fl_l = rax; fl_r = (0x2000000u64) as u64; if ((((fl_l as u64)) as i64) >= (((fl_r as u64)) as i64)) { bb = 12; continue; } bb = 9; continue; }
            9 => { rdx = 0; fl_l = 0; fl_r = 0; r8 = 0x8000; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 13; continue; } bb = 10; continue; }
            10 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = ((rax as u32)) as u32 as u64; bb = 11; continue; }
            11 => { fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 13; continue; } bb = 12; continue; }
            12 => { r9 = 0x40000000; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbx); r8 = ((rdi as u32)) as u32 as u64; rcx = P3; rdx = ((rdi as u32)) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 13; continue; }
            13 => { rdx = (0xffffffffc0000000u64) as u64; rcx = P5; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = (0xffffffffc0000000u64) as u64; rcx = P7; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 14; continue; }
            14 => { rbx = ((rbx).wrapping_add(0x40000000u64)) as u64; fl_l = rsi; fl_r = (0x1u64) as u64; rsi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } bb = 15; continue; }
            15 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; bb = 16; continue; }
            16 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rsp = ((rsp).wrapping_add(0x60u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); bb = 17; continue; }
            17 => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142538a60(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143c55040; const P1: u64 = 0x1425466e0; const P2: u64 = 0x143bbdfa8; const P3: u64 = 0x1425412f0; const P4: u64 = 0x142541180; const P5: u64 = 0x143bfe128; const P6: u64 = 0x143844165; const P7: u64 = 0x1426456c8; const P8: u64 = 0x1434261f0; const P9: u64 = 0x142537a30; const P10: u64 = 0x143bc5fd8; const P11: u64 = 0x142645730; const P12: u64 = 0x142645708; const P13: u64 = 0x143a09638; const P14: u64 = 0x143a09630; const P15: u64 = 0x143a09620; const P16: u64 = 0x143a09618; const P17: u64 = 0x143a09610; const P18: u64 = 0x143a09600; const P19: u64 = 0x143425a30; const P20: u64 = 0x143425c50; const P21: u64 = 0x143425c10; const P22: u64 = 0x142539e50;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 27>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r9); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rbp = rsp.wrapping_sub(0x17); rsp = ((rsp).wrapping_sub(0xa0u64)) as u64; r14 = (crate::simd_ops::ru64(rbp.wrapping_add(0x77u64))) as u64; r13 = rcx; rcx = 0; fl_l = 0; fl_r = 0; rax = r9; crate::simd_ops::wu64(rbp.wrapping_sub(0x29u64), rcx); r12 = r8; rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x7fu64))) as u64; r15 = ((rdx as u32)) as u32 as u64; xmm0 = 0u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x39u64), (xmm0) as u128); crate::simd_ops::wu128(rcx, (xmm0) as u128); xmm0 = ((crate::simd_ops::ru64(rbp.wrapping_sub(0x29u64))) as u64) as u128; crate::simd_ops::wu64(rcx.wrapping_add(0x10u64), ((xmm0 as u64)) as u64); rcx = 0; fl_l = 0; fl_r = 0; fl_l = r14; fl_r = r14; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(r14, rcx); bb = 2; continue; }
            2 => { fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu64(r9, rcx); bb = 4; continue; }
            4 => { rbx = (crate::simd_ops::ru64(P0)) as u64; rsi = r13; fl_l = ((rsi).wrapping_shl(30u32)) as u64; rsi = ((fl_l as u64)) as u64; r14 = 0x6348; bb = 5; continue; }
            5 => { rdi = rbx; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { rax = ((crate::exe::tls_pointer() as u64)) as u64; rcx = (crate::simd_ops::ru64(rax)) as u64; rcx = (crate::simd_ops::ru64(r14.wrapping_add(rcx))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = rax; rdi = ((rdi).wrapping_shr(17u32)) as u64; rdi = ((((rdi as u32)) & (0xfffu32))) as u32 as u64; rdi = ((rdi).wrapping_add(0x8000u64)) as u64; fl_l = ((rdi).wrapping_shl(30u32)) as u64; rdi = ((fl_l as u64)) as u64; bb = 7; continue; }
            7 => { rcx = ((rdi.wrapping_add(rsi)) as u64) as u64; rax = rbx; fl_l = rax; let cx_src: u64 = rcx; let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u64(P0, fl_l as u64, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u64)) as u64; } rbx = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 5; continue; } bb = 8; continue; }
            8 => { r14 = (crate::simd_ops::ru64(rbp.wrapping_add(0x77u64))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { rax = 0; fl_l = 0; fl_r = 0; bb = 55; continue; }
            10 => { xmm1 = 0u128; xmm1 = (xmm1 & !0xffff_ffff_ffff_ffffu128) | ((((crate::simd_ops::ru64(P2)) as i64) as f64).to_bits() as u128); xmm0 = 0u128; fl_l = ((xmm1 as u64)) as u64; fl_r = ((xmm0 as u64)) as u64; if (f64::from_bits(fl_l).is_nan() || f64::from_bits(fl_r).is_nan()) { bb = 13; continue; } bb = 11; continue; }
            11 => { if (f64::from_bits(fl_l) != f64::from_bits(fl_r) && !f64::from_bits(fl_l).is_nan() && !f64::from_bits(fl_r).is_nan()) { bb = 13; continue; } bb = 12; continue; }
            12 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rax; fl_r = rbx; rax = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; crate::simd_ops::wu64(P2, rax); bb = 13; continue; }
            13 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rbp.wrapping_sub(0x41u64), rax); rsi = ((rbx as u32)) as u32 as u64; fl_l = r13; fl_r = r13; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 48; continue; } bb = 14; continue; }
            14 => { r14 = rdi; bb = 15; continue; }
            15 => { rcx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru64(P5)) as u64; fl_r = (0x0u64) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x21u64), rbx); crate::simd_ops::wu64(rbp.wrapping_sub(0x19u64), rbx); crate::simd_ops::wu64(rbp.wrapping_sub(0x11u64), rbx); crate::simd_ops::wu64(rbp.wrapping_sub(0x9u64), rbx); crate::simd_ops::wu64(rbp.wrapping_sub(0x1u64), rbx); crate::simd_ops::wu64(rbp.wrapping_add(0x7u64), rbx); if (((fl_l as u64)) == ((fl_r as u64))) { bb = 22; continue; } bb = 16; continue; }
            16 => { fl_l = (crate::simd_ops::ru8(P6)) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 22; continue; } bb = 17; continue; }
            17 => { crate::simd_ops::wu8(rbp.wrapping_sub(0x21u64), (0x5u8) as u8); crate::simd_ops::wu64(rbp.wrapping_sub(0x19u64), (0x10u64) as u64); fl_l = ((r15 as u32)) as u32 as u64; fl_r = ((r15 as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 19; continue; } bb = 18; continue; }
            18 => { crate::simd_ops::wu8(rbp.wrapping_sub(0x11u64), (0x2u8) as u8); crate::simd_ops::wu32(rbp.wrapping_sub(0x9u64), ((r15 as u32)) as u32); bb = 19; continue; }
            19 => { rbx = ((r15 as u32)) as u32 as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x49u64), (0x40000000u64) as u64); rbx = (((((rbx as u32)) as i32).wrapping_shr(31u32) as u32)) as u32 as u64; rbx = (((rbx as u32)).wrapping_add(0x2u32)) as u32 as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x57u64), r14); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P7), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((rbx as u32)) as u32); r8 = rbp.wrapping_sub(0x49); rcx = rax; rdx = rbp.wrapping_add(0x57); rax = rbp.wrapping_sub(0x21); r9 = 0x20003000; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x4u32) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 21; continue; } bb = 20; continue; }
            20 => { rbx = (crate::simd_ops::ru64(rbp.wrapping_add(0x57u64))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 28; continue; } bb = 21; continue; }
            21 => { rdx = ((rax as u32)) as u32 as u64; crate::simd_ops::wu8(P6, (0x0u8) as u8); rcx = P8; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 22; continue; }
            22 => { fl_l = (crate::simd_ops::ru64(P10)) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 25; continue; } bb = 23; continue; }
            23 => { fl_l = ((r15 as u32)) as u32 as u64; fl_r = ((r15 as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 25; continue; } bb = 24; continue; }
            24 => { crate::simd_ops::wu8(rbp.wrapping_sub(0x21u64), (0x2u8) as u8); crate::simd_ops::wu32(rbp.wrapping_sub(0x19u64), ((r15 as u32)) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P7), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), (0x1u32) as u32); r9 = 0x20003000; rcx = rax; r8 = 0x40000000; rax = rbp.wrapping_sub(0x21); rdx = r14; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x4u32) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P10), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 26; continue; }
            25 => { rdx = 0x40000000; r9 = 0x4; r8 = 0x20003000; rcx = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P11), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 26; continue; }
            26 => { rbx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 28; continue; } bb = 27; continue; }
            27 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P12), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 42; continue; } bb = 28; continue; }
            28 => { fl_l = rbx; fl_r = r14; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 44; continue; } bb = 29; continue; }
            29 => { rsi = ((rsi).wrapping_add(1)) as u64; rcx = 0x40000000; r14 = ((r14).wrapping_add(0x40000000u64)) as u64; let xa_src: u64 = rcx; let xa_old: u64 = crate::simd_ops::atomic_xadd_u64(P13, xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rcx = (xa_old) as u64; fl_l = ((rcx).wrapping_add(0x40000000u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; bb = 30; continue; }
            30 => { rax = (crate::simd_ops::ru64(P14)) as u64; fl_l = rax; fl_r = rcx; if ((((fl_l as u64)) as i64) >= (((fl_r as u64)) as i64)) { bb = 32; continue; } bb = 31; continue; }
            31 => { fl_l = rax; let cx_src: u64 = rcx; let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u64(P14, fl_l as u64, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u64)) as u64; } if (((fl_l as u64)) != ((fl_r as u64))) { bb = 30; continue; } bb = 32; continue; }
            32 => { crate::simd_ops::lk_add_u64(P15, (0x40000000u64) as u64); rcx = 0x40000000; let xa_src: u64 = rcx; let xa_old: u64 = crate::simd_ops::atomic_xadd_u64(P16, xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rcx = (xa_old) as u64; fl_l = ((rcx).wrapping_add(0x40000000u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; bb = 33; continue; }
            33 => { rax = (crate::simd_ops::ru64(P17)) as u64; fl_l = rax; fl_r = rcx; if ((((fl_l as u64)) as i64) >= (((fl_r as u64)) as i64)) { bb = 35; continue; } bb = 34; continue; }
            34 => { fl_l = rax; let cx_src: u64 = rcx; let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u64(P17, fl_l as u64, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u64)) as u64; } if (((fl_l as u64)) != ((fl_r as u64))) { bb = 33; continue; } bb = 35; continue; }
            35 => { crate::simd_ops::lk_add_u64(P18, (0x40000000u64) as u64); fl_l = r12; fl_r = r12; if (((((fl_l as u64)) & ((fl_r as u64))) as i64) <= 0) { bb = 40; continue; } bb = 36; continue; }
            36 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = rax; r8 = ((r8).wrapping_sub(crate::simd_ops::ru64(P2))) as u64; r8 = ((r8).wrapping_sub(crate::simd_ops::ru64(rbp.wrapping_sub(0x41u64)))) as u64; fl_l = rsi; fl_r = (0x1u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 39; continue; } bb = 37; continue; }
            37 => { rcx = rsi.wrapping_add(0x1); rdx = 0; fl_l = 0; fl_r = 0; rax = r8; let d = (rcx) as u128; let n = ((rdx as u128) << 64) | (rax as u128); if d != 0 { rax = (n / d) as u64; rdx = (n % d) as u64; } rcx = ((r12.wrapping_add(r12)) as u64) as u64; rax = (((rax) as i64).wrapping_mul((r13) as i64) as u64) as u64; fl_l = rax; fl_r = rcx; if ((((fl_l as u64)) as i64) <= (((fl_r as u64)) as i64)) { bb = 39; continue; } bb = 38; continue; }
            38 => { r8 = r12.wrapping_add(0x1); bb = 39; continue; }
            39 => { fl_l = r8; fl_r = r12; if ((((fl_l as u64)) as i64) > (((fl_r as u64)) as i64)) { bb = 43; continue; } bb = 40; continue; }
            40 => { rbx = 0; fl_l = 0; fl_r = 0; fl_l = rsi; fl_r = r13; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 47; continue; } bb = 41; continue; }
            41 => { bb = 15; continue; }
            42 => { r9 = r14; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (0x40000000u64) as u64); r8 = ((rax as u32)) as u32 as u64; rcx = P19; rdx = ((rax as u32)) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 46; continue; }
            43 => { rdx = rsi; rcx = P20; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 46; continue; }
            44 => { fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 46; continue; } bb = 45; continue; }
            45 => { r8 = r14; rcx = P21; rdx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); rdx = 0x40000000; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P22, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 46; continue; }
            46 => { rbx = 0; fl_l = 0; fl_r = 0; bb = 47; continue; }
            47 => { r14 = (crate::simd_ops::ru64(rbp.wrapping_add(0x77u64))) as u64; bb = 48; continue; }
            48 => { rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x6fu64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 50; continue; } bb = 49; continue; }
            49 => { crate::simd_ops::wu64(rax, rsi); bb = 50; continue; }
            50 => { fl_l = r14; fl_r = r14; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 52; continue; } bb = 51; continue; }
            51 => { rax = rsi; fl_l = ((rax).wrapping_shl(30u32)) as u64; rax = ((fl_l as u64)) as u64; crate::simd_ops::wu64(r14, rax); bb = 52; continue; }
            52 => { fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 54; continue; } bb = 53; continue; }
            53 => { rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rbp.wrapping_sub(0x29u64), rax); fl_l = rsi; fl_r = rsi; rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x7fu64))) as u64; crate::simd_ops::wu32(rbp.wrapping_sub(0x25u64), (0x3u32) as u32); xmm0 = ((crate::simd_ops::ru64(rbp.wrapping_sub(0x29u64))) as u64) as u128; crate::simd_ops::wu64(rbp.wrapping_sub(0x29u64), ((xmm0 as u64)) as u64); crate::simd_ops::wu16(rbp.wrapping_sub(0x28u64), (0x101u16) as u16); crate::simd_ops::wu8(rbp.wrapping_sub(0x29u64), (0x1u8) as u8); xmm0 = ((crate::simd_ops::ru64(rbp.wrapping_sub(0x29u64))) as u64) as u128; xmm1 = 0u128; crate::simd_ops::wu128(rax, (xmm1) as u128); crate::simd_ops::wu64(rax.wrapping_add(0x10u64), ((xmm0 as u64)) as u64); crate::simd_ops::wu32(rax.wrapping_add(0x14u64), (0x4u32) as u32); bb = 54; continue; }
            54 => { rdi = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rbx } else { rdi }) as u64; rax = rdi; bb = 55; continue; }
            55 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0xe8u64))) as u64; rsp = ((rsp).wrapping_add(0xa0u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142538e90(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143c55040; const P1: u64 = 0x1425466e0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = rdx; rdi = rcx; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rdx, (0x0u64) as u64); bb = 2; continue; }
            2 => { rbx = (crate::simd_ops::ru64(P0)) as u64; fl_l = ((rdi).wrapping_shl(30u32)) as u64; rdi = ((fl_l as u64)) as u64; rbp = 0x6348; bb = 3; continue; }
            3 => { rcx = rbx; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = ((crate::exe::tls_pointer() as u64)) as u64; rcx = (crate::simd_ops::ru64(rax)) as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_add(rcx))) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = rax; rcx = ((rcx).wrapping_shr(17u32)) as u64; rcx = ((((rcx as u32)) & (0xfffu32))) as u32 as u64; rcx = ((rcx).wrapping_add(0x8000u64)) as u64; fl_l = ((rcx).wrapping_shl(30u32)) as u64; rcx = ((fl_l as u64)) as u64; bb = 5; continue; }
            5 => { rdx = ((rcx.wrapping_add(rdi)) as u64) as u64; rax = rbx; fl_l = rax; let cx_src: u64 = rdx; let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u64(P0, fl_l as u64, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u64)) as u64; } rbx = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 6; continue; }
            6 => { rax = rcx; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { crate::simd_ops::wu64(rsi, rdi); bb = 8; continue; }
            8 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142538f50(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142538f60;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64));
    return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142538f60(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143872f30; const P1: u64 = 0x142645338; const P2: u64 = 0x142645708; const P3: u64 = 0x1434259c0; const P4: u64 = 0x1434259c8; const P5: u64 = 0x1434259e0; const P6: u64 = 0x142537a30;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rbp = (((r8 as u8)) as u32) as u32 as u64; r11 = rdx; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 1; continue; }
            1 => { fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 2; continue; }
            2 => { r9 = (crate::simd_ops::ru64(P0)) as u64; r10 = r9.wrapping_sub(0x1); rax = ((r10.wrapping_add(rcx)) as u64) as u64; fl_l = r10; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { fl_l = ((!(r10)) & (rax)) as u64; fl_r = fl_l; rsi = ((fl_l as u64)) as u64; rax = ((rdx.wrapping_add(rcx)) as u64) as u64; fl_l = ((!(r10)) & (rax)) as u64; fl_r = fl_l; rdi = ((fl_l as u64)) as u64; bb = 5; continue; }
            4 => { rdx = 0; fl_l = 0; fl_r = 0; let d = (r9) as u128; let n = ((rdx as u128) << 64) | (rax as u128); if d != 0 { rax = (n / d) as u64; rdx = (n % d) as u64; } rdx = 0; fl_l = 0; fl_r = 0; rsi = rax; rax = ((r11.wrapping_add(rcx)) as u64) as u64; let d = (r9) as u128; let n = ((rdx as u128) << 64) | (rax as u128); if d != 0 { rax = (n / d) as u64; rdx = (n % d) as u64; } rsi = (((rsi) as i64).wrapping_mul((r9) as i64) as u64) as u64; rdi = rax; rdi = (((rdi) as i64).wrapping_mul((r9) as i64) as u64) as u64; bb = 5; continue; }
            5 => { rdi = ((rdi).wrapping_sub(rsi)) as u64; fl_l = rdi; fl_r = rdi; if (((((fl_l as u64)) & ((fl_r as u64))) as i64) <= 0) { bb = 10; continue; } bb = 6; continue; }
            6 => { rax = 0x1; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rbx); rbx = 0; fl_l = 0; fl_r = 0; r9 = rsp.wrapping_add(0x48); r8 = 0x4; crate::simd_ops::wu32(rsp.wrapping_add(0x48u64), ((rbx as u32)) as u32); fl_l = ((rbp as u8)) as u8 as u64; fl_r = ((rbp as u8)) as u8 as u64; rdx = rdi; rcx = rsi; r8 = (if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { (rax as u32) } else { (r8 as u32) }) as u32 as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 9; continue; } bb = 7; continue; }
            7 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); rbx = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { fl_l = ((rbp as u8)) as u8 as u64; fl_r = ((rbp as u8)) as u8 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rdi); rax = P3; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); rdx = P4; r9 = ((rbx as u32)) as u32 as u64; rdx = (if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { rax } else { rdx }) as u64; rcx = P5; r8 = ((rbx as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); bb = 9; continue; }
            9 => { fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rax = (rax & !0xffu64) | (((if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { 1u8 } else { 0u8 }) as u8 as u64)); rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            10 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142539080(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1425390a0;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    r9 = r8;
    r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64));
    return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425390a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143872dd4; const P1: u64 = 0x143872dd0; const P2: u64 = 0x1425375a0; const P3: u64 = 0x220; const P4: u64 = 0x143a095c0; const P5: u64 = 0x143a09840; const P6: u64 = 0x8; const P7: u64 = 0xa0; const P8: u64 = 0x1425408f0; const P9: u64 = 0x143872c94; const P10: u64 = 0x143872c90; const P11: u64 = 0x142534300; const P12: u64 = 0x142539350; const P13: u64 = 0x1425391a0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; fl_l = (crate::simd_ops::ru32(P0)) as u32 as u64; fl_r = 0x0; rdi = r9; rbp = (((r8 as u8)) as u32) as u32 as u64; rbx = rdx; rsi = rcx; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = P1; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru32(P1)) as u32 as u64; fl_r = 0x0; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 14; continue; } bb = 3; continue; }
            3 => { r9 = ((rdi.wrapping_add(P3)) as u64) as u64; rax = P4; fl_l = r9; fl_r = rax; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 6; continue; } bb = 4; continue; }
            4 => { rax = P5; fl_l = r9; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { crate::simd_ops::lk_add_u64(r9.wrapping_add(P6), (1) as u64); fl_l = (crate::simd_ops::lk_add_u64(r9, (1) as u64)) as u64; bb = 7; continue; }
            6 => { crate::simd_ops::lk_add_u64(r9.wrapping_add(P6), (1) as u64); fl_l = (crate::simd_ops::lk_add_u64(r9, (1) as u64)) as u64; bb = 7; continue; }
            7 => { rcx = ((rdi.wrapping_add(P7)) as u64) as u64; rdx = rbx; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru32(P9)) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 9; continue; } bb = 8; continue; }
            8 => { rcx = P10; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 9; continue; }
            9 => { fl_l = (crate::simd_ops::ru32(P10)) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 12; continue; } bb = 10; continue; }
            10 => { rax = crate::simd_ops::call_stk(P11, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 12; continue; } bb = 11; continue; }
            11 => { r8 = rsp.wrapping_add(0x20); crate::simd_ops::wu8(rsp.wrapping_add(0x20u64), (0x1u8) as u8); rdx = rbx; rcx = rsi; rax = crate::simd_ops::call_stk(P12, rax, rcx, rdx, r8, r9, rsp); rax = ((crate::simd_ops::ru8(rsp.wrapping_add(0x20u64))) as u32) as u32 as u64; bb = 15; continue; }
            12 => { fl_l = ((rbp as u8)) as u8 as u64; fl_r = ((rbp as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 14; continue; } bb = 13; continue; }
            13 => { r8 = rdi; rdx = rbx; rcx = rsi; rax = crate::simd_ops::call_stk(P13, rax, rcx, rdx, r8, r9, rsp); bb = 14; continue; }
            14 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 15; continue; }
            15 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425391a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143872f30; const P1: u64 = 0x143a095c0; const P2: u64 = 0x143a09840; const P3: u64 = 0x143427870; const P4: u64 = 0x142645730; const P5: u64 = 0x142645708; const P6: u64 = 0x143425bc0; const P7: u64 = 0x142537a30;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; r11 = rdx; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 26; continue; } bb = 1; continue; }
            1 => { fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 26; continue; } bb = 2; continue; }
            2 => { r9 = (crate::simd_ops::ru64(P0)) as u64; r10 = r9.wrapping_sub(0x1); rax = ((r10.wrapping_add(rcx)) as u64) as u64; fl_l = r10; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { fl_l = ((!(r10)) & (rax)) as u64; fl_r = fl_l; rsi = ((fl_l as u64)) as u64; rax = ((rdx.wrapping_add(rcx)) as u64) as u64; fl_l = ((!(r10)) & (rax)) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; bb = 5; continue; }
            4 => { rdx = 0; fl_l = 0; fl_r = 0; let d = (r9) as u128; let n = ((rdx as u128) << 64) | (rax as u128); if d != 0 { rax = (n / d) as u64; rdx = (n % d) as u64; } rdx = 0; fl_l = 0; fl_r = 0; rsi = rax; rax = ((r11.wrapping_add(rcx)) as u64) as u64; let d = (r9) as u128; let n = ((rdx as u128) << 64) | (rax as u128); if d != 0 { rax = (n / d) as u64; rdx = (n % d) as u64; } rsi = (((rsi) as i64).wrapping_mul((r9) as i64) as u64) as u64; rbx = rax; rbx = (((rbx) as i64).wrapping_mul((r9) as i64) as u64) as u64; bb = 5; continue; }
            5 => { rbx = ((rbx).wrapping_sub(rsi)) as u64; fl_l = rbx; fl_r = rbx; if (((((fl_l as u64)) & ((fl_r as u64))) as i64) <= 0) { bb = 26; continue; } bb = 6; continue; }
            6 => { rcx = r8.wrapping_add(0x80); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rdi); r10 = P1; r9 = P2; fl_l = rcx; fl_r = r10; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 14; continue; } bb = 7; continue; }
            7 => { fl_l = rcx; fl_r = r9; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 14; continue; } bb = 8; continue; }
            8 => { rdx = rbx; let xa_src: u64 = rdx; let xa_old: u64 = crate::simd_ops::atomic_xadd_u64(rcx.wrapping_add(0x18u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rdx = (xa_old) as u64; fl_l = ((rdx).wrapping_add(rbx)) as u64; fl_r = fl_l; rdx = ((fl_l as u64)) as u64; bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; fl_l = rax; fl_r = rdx; if ((((fl_l as u64)) as i64) >= (((fl_r as u64)) as i64)) { bb = 11; continue; } bb = 10; continue; }
            10 => { fl_l = rax; let cx_src: u64 = rdx; let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u64(rcx.wrapping_add(0x10u64), fl_l as u64, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u64)) as u64; } if (((fl_l as u64)) != ((fl_r as u64))) { bb = 9; continue; } bb = 11; continue; }
            11 => { rax = rbx; fl_l = rbx; fl_r = rbx; if (((((fl_l as u64)) & ((fl_r as u64))) as i64) > 0) { bb = 13; continue; } bb = 12; continue; }
            12 => { rcx = ((rcx).wrapping_add(0x8u64)) as u64; fl_l = 0; fl_r = rax; rax = (0u64.wrapping_sub(fl_r as u64)) as u64; bb = 13; continue; }
            13 => { fl_l = (crate::simd_ops::lk_add_u64(rcx, (rax) as u64)) as u64; fl_r = fl_l; bb = 17; continue; }
            14 => { rdx = (crate::simd_ops::ru64(rcx.wrapping_add(0x18u64))) as u64; rdx = ((rdx).wrapping_add(rbx)) as u64; crate::simd_ops::wu64(rcx.wrapping_add(0x18u64), rdx); fl_l = rdx; fl_r = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; if ((((fl_l as u64)) as i64) <= (((fl_r as u64)) as i64)) { bb = 16; continue; } bb = 15; continue; }
            15 => { crate::simd_ops::wu64(rcx.wrapping_add(0x10u64), rdx); bb = 16; continue; }
            16 => { fl_l = (crate::simd_ops::lk_add_u64(rcx, (rbx) as u64)) as u64; fl_r = fl_l; bb = 17; continue; }
            17 => { rax = r8.wrapping_add(0x210); fl_l = rax; fl_r = r10; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 20; continue; } bb = 18; continue; }
            18 => { fl_l = rax; fl_r = r9; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 20; continue; } bb = 19; continue; }
            19 => { crate::simd_ops::lk_add_u64(rax.wrapping_add(0x8u64), (1) as u64); fl_l = (crate::simd_ops::lk_add_u64(rax, (1) as u64)) as u64; bb = 21; continue; }
            20 => { xmm1 = (crate::simd_ops::ru128(P3)) as u128; xmm1 = crate::simd_ops::vpaddq(xmm1, crate::simd_ops::ru128(rax)); crate::simd_ops::wu128(rax, (xmm1) as u128); bb = 21; continue; }
            21 => { r9 = 0x4; r8 = 0x80000; rdx = rbx; rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 23; continue; } bb = 22; continue; }
            22 => { rdi = 0; fl_l = 0; fl_r = 0; fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rax = (rax & !0xffu64) | (((if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { 1u8 } else { 0u8 }) as u8 as u64)); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            23 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 25; continue; } bb = 24; continue; }
            24 => { r9 = rsi; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbx); r8 = ((rax as u32)) as u32 as u64; rcx = P6; rdx = ((rax as u32)) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 25; continue; }
            25 => { fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rax = (rax & !0xffu64) | (((if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { 1u8 } else { 0u8 }) as u8 as u64)); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            26 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); rsp = ((rsp).wrapping_add(0x30u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142539330(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142539350;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), r8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    r8 = rsp.wrapping_add(0x40);
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142539350(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143a09620; const P1: u64 = 0x1425408f0; const P2: u64 = 0x143872f30; const P3: u64 = 0x142645738; const P4: u64 = 0x142645708; const P5: u64 = 0x143425b70; const P6: u64 = 0x142537a30;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; r10 = rdx; r9 = rcx; rdx = (0u64.wrapping_sub(rdx)) as u64; rcx = P0; rbx = r8; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); fl_l = r10; fl_r = r10; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 1; continue; }
            1 => { fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru64(P2)) as u64; rdx = rcx.wrapping_sub(0x1); rax = ((rdx.wrapping_add(r9)) as u64) as u64; fl_l = rdx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { fl_l = ((!(rdx)) & (rax)) as u64; fl_r = fl_l; rsi = ((fl_l as u64)) as u64; rax = ((r10.wrapping_add(r9)) as u64) as u64; fl_l = ((!(rdx)) & (rax)) as u64; fl_r = fl_l; rdi = ((fl_l as u64)) as u64; bb = 5; continue; }
            4 => { rdx = 0; fl_l = 0; fl_r = 0; let d = (rcx) as u128; let n = ((rdx as u128) << 64) | (rax as u128); if d != 0 { rax = (n / d) as u64; rdx = (n % d) as u64; } rdx = 0; fl_l = 0; fl_r = 0; rsi = rax; rax = ((r10.wrapping_add(r9)) as u64) as u64; let d = (rcx) as u128; let n = ((rdx as u128) << 64) | (rax as u128); if d != 0 { rax = (n / d) as u64; rdx = (n % d) as u64; } rsi = (((rsi) as i64).wrapping_mul((rcx) as i64) as u64) as u64; rdi = rax; rdi = (((rdi) as i64).wrapping_mul((rcx) as i64) as u64) as u64; bb = 5; continue; }
            5 => { rdi = ((rdi).wrapping_sub(rsi)) as u64; fl_l = rdi; fl_r = rdi; if (((((fl_l as u64)) & ((fl_r as u64))) as i64) <= 0) { bb = 11; continue; } bb = 6; continue; }
            6 => { r8 = 0x4000; crate::simd_ops::wu8(rbx, (0x1u8) as u8); rdx = rdi; rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu8(rbx, (0x1u8) as u8); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { rbx = 0; fl_l = 0; fl_r = 0; fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; rax = (rax & !0xffu64) | (((if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { 1u8 } else { 0u8 }) as u8 as u64)); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            8 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); rbx = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { r9 = rsi; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); r8 = ((rax as u32)) as u32 as u64; rcx = P5; rdx = ((rax as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); bb = 10; continue; }
            10 => { fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; rax = (rax & !0xffu64) | (((if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { 1u8 } else { 0u8 }) as u8 as u64)); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            11 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142539460(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143a09620; const P1: u64 = 0x1425408f0; const P2: u64 = 0x143a097c8; const P3: u64 = 0x143a097c0; const P4: u64 = 0x143872f30; const P5: u64 = 0x142645730; const P6: u64 = 0x142645708; const P7: u64 = 0x143425b20; const P8: u64 = 0x142537a30;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x38u64)) as u64; r9 = rdx; r10 = rcx; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu8(r8, (0x0u8) as u8); bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rbx); rcx = P0; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rdi); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::lk_add_u64(P2, (1) as u64); crate::simd_ops::lk_add_u64(P3, (1) as u64); fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 3; continue; }
            3 => { fl_l = r10; fl_r = r10; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru64(P4)) as u64; r8 = rcx.wrapping_sub(0x1); fl_l = r8; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = ((r9.wrapping_add(r8)) as u64) as u64; rax = ((rax).wrapping_add(r10)) as u64; fl_l = ((!(r8)) & (r10)) as u64; fl_r = fl_l; rdi = ((fl_l as u64)) as u64; fl_l = ((!(r8)) & (rax)) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; bb = 7; continue; }
            6 => { rdx = 0; fl_l = 0; fl_r = 0; rax = r10; let d = (rcx) as u128; let n = ((rdx as u128) << 64) | (rax as u128); if d != 0 { rax = (n / d) as u64; rdx = (n % d) as u64; } rdx = 0; fl_l = 0; fl_r = 0; rdi = rax; rax = ((r8.wrapping_add(r9)) as u64) as u64; rax = ((rax).wrapping_add(r10)) as u64; let d = (rcx) as u128; let n = ((rdx as u128) << 64) | (rax as u128); if d != 0 { rax = (n / d) as u64; rdx = (n % d) as u64; } rdi = (((rdi) as i64).wrapping_mul((rcx) as i64) as u64) as u64; rbx = rax; rbx = (((rbx) as i64).wrapping_mul((rcx) as i64) as u64) as u64; bb = 7; continue; }
            7 => { rbx = ((rbx).wrapping_sub(rdi)) as u64; fl_l = rbx; fl_r = rbx; if (((((fl_l as u64)) & ((fl_r as u64))) as i64) <= 0) { bb = 11; continue; } bb = 8; continue; }
            8 => { r9 = 0x4; r8 = 0x1000; rdx = rbx; rcx = rdi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 11; continue; } bb = 9; continue; }
            9 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { r9 = rdi; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbx); r8 = ((rax as u32)) as u32 as u64; rcx = P7; rdx = ((rax as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x38u64)) as u64; return rax; }
            11 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x38u64)) as u64; return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142539560(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x143872f30; const P1: u64 = 0xffffffffffffffff;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r11 = rdx; r9 = rcx; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(r8, (0x0u64) as u64); bb = 2; continue; }
            2 => { fl_l = r11; fl_r = r11; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 3; continue; }
            3 => { fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 4; continue; }
            4 => { r10 = (crate::simd_ops::ru64(P0)) as u64; rdx = ((r10.wrapping_add(P1)) as u64) as u64; rax = ((rdx.wrapping_add(rcx)) as u64) as u64; fl_l = rdx; fl_r = r10; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { fl_l = ((!(rdx)) & (rax)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; rax = ((r9.wrapping_add(r11)) as u64) as u64; fl_l = ((!(rdx)) & (rax)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; bb = 7; continue; }
            6 => { rdx = 0; fl_l = 0; fl_r = 0; let d = (r10) as u128; let n = ((rdx as u128) << 64) | (rax as u128); if d != 0 { rax = (n / d) as u64; rdx = (n % d) as u64; } rdx = 0; fl_l = 0; fl_r = 0; rcx = rax; rax = ((r9.wrapping_add(r11)) as u64) as u64; let d = (r10) as u128; let n = ((rdx as u128) << 64) | (rax as u128); if d != 0 { rax = (n / d) as u64; rdx = (n % d) as u64; } rcx = (((rcx) as i64).wrapping_mul((r10) as i64) as u64) as u64; rax = (((rax) as i64).wrapping_mul((r10) as i64) as u64) as u64; bb = 7; continue; }
            7 => { rax = ((rax).wrapping_sub(rcx)) as u64; fl_l = rax; fl_r = rax; if (((((fl_l as u64)) & ((fl_r as u64))) as i64) <= 0) { bb = 11; continue; } bb = 8; continue; }
            8 => { fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { crate::simd_ops::wu64(r8, rax); bb = 10; continue; }
            10 => { rax = rcx; return rax; }
            11 => { rax = 0; fl_l = 0; fl_r = 0; return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425395e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x143872f30; const P1: u64 = 0xffffffffffffffff;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r11 = r8; r10 = rdx; fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(r9, (0x0u64) as u64); bb = 2; continue; }
            2 => { fl_l = r11; fl_r = r11; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 19; continue; } bb = 3; continue; }
            3 => { fl_l = r10; fl_r = r10; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 19; continue; } bb = 4; continue; }
            4 => { r8 = (crate::simd_ops::ru64(P0)) as u64; rax = ((r8.wrapping_add(P1)) as u64) as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 8; continue; } bb = 5; continue; }
            5 => { rcx = ((rax.wrapping_add(rdx)) as u64) as u64; fl_l = rax; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { fl_l = ((!(rax)) & (rcx)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; rax = ((r10.wrapping_add(r11)) as u64) as u64; rdx = ((r8.wrapping_add(P1)) as u64) as u64; bb = 12; continue; }
            7 => { rdx = 0; fl_l = 0; fl_r = 0; rax = rcx; let d = (r8) as u128; let n = ((rdx as u128) << 64) | (rax as u128); if d != 0 { rax = (n / d) as u64; rdx = (n % d) as u64; } rdx = ((r8.wrapping_add(P1)) as u64) as u64; rcx = rax; rax = ((r10.wrapping_add(r11)) as u64) as u64; rcx = (((rcx) as i64).wrapping_mul((r8) as i64) as u64) as u64; bb = 12; continue; }
            8 => { fl_l = rax; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { fl_l = ((!(rax)) & (r10)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; bb = 11; continue; }
            10 => { rdx = 0; fl_l = 0; fl_r = 0; rax = r10; let d = (r8) as u128; let n = ((rdx as u128) << 64) | (rax as u128); if d != 0 { rax = (n / d) as u64; rdx = (n % d) as u64; } rcx = rax; rcx = (((rcx) as i64).wrapping_mul((r8) as i64) as u64) as u64; bb = 11; continue; }
            11 => { rax = ((r10.wrapping_add(r11)) as u64) as u64; rdx = ((r8.wrapping_add(P1)) as u64) as u64; fl_l = ((rax).wrapping_add(rdx)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; bb = 12; continue; }
            12 => { fl_l = rdx; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 14; continue; } bb = 13; continue; }
            13 => { fl_l = ((!(rdx)) & (rax)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; bb = 15; continue; }
            14 => { rdx = 0; fl_l = 0; fl_r = 0; let d = (r8) as u128; let n = ((rdx as u128) << 64) | (rax as u128); if d != 0 { rax = (n / d) as u64; rdx = (n % d) as u64; } rax = (((rax) as i64).wrapping_mul((r8) as i64) as u64) as u64; bb = 15; continue; }
            15 => { rax = ((rax).wrapping_sub(rcx)) as u64; fl_l = rax; fl_r = rax; if (((((fl_l as u64)) & ((fl_r as u64))) as i64) <= 0) { bb = 19; continue; } bb = 16; continue; }
            16 => { fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 18; continue; } bb = 17; continue; }
            17 => { crate::simd_ops::wu64(r9, rax); bb = 18; continue; }
            18 => { rax = rcx; return rax; }
            19 => { rax = 0; fl_l = 0; fl_r = 0; return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425396a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x1425397a0; const P2: u64 = 0xffffffffffffffff; const P3: u64 = 0x143872f30; const P4: u64 = 0x142539350;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = ((rsp).wrapping_sub(0x50u64)) as u64; xmm0 = 0u128; rax = 0; fl_l = 0; fl_r = 0; rsi = (((r9 as u8)) as u32) as u32 as u64; r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0x88u64))) as u64; r10 = rdx; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rax); crate::simd_ops::wu128(r9, (xmm0) as u128); xmm0 = ((crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64) as u128; crate::simd_ops::wu64(r9.wrapping_add(P0), ((xmm0 as u64)) as u64); fl_l = r8; fl_r = (0x2000000u64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 11; continue; } bb = 1; continue; }
            1 => { fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r9); r8 = (((rsi as u8)) as u32) as u32 as u64; r9 = ((crate::simd_ops::ru8(rsp.wrapping_add(0x80u64))) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rsp = ((rsp).wrapping_add(0x50u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            3 => { rax = ((rdx.wrapping_add(P2)) as u64) as u64; rbx = ((rax.wrapping_add(r8)) as u64) as u64; fl_l = r10; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { fl_l = ((!(rax)) & (rbx)) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; bb = 6; continue; }
            5 => { rdx = 0; fl_l = 0; fl_r = 0; rax = rbx; let d = (r10) as u128; let n = ((rdx as u128) << 64) | (rax as u128); if d != 0 { rax = (n / d) as u64; rdx = (n % d) as u64; } fl_l = rbx; fl_r = rdx; rbx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; bb = 6; continue; }
            6 => { rbx = ((rbx).wrapping_sub(r8)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r9); r9 = ((crate::simd_ops::ru8(rsp.wrapping_add(0x80u64))) as u32) as u32 as u64; rcx = ((rcx).wrapping_add(rbx)) as u64; r8 = (((rsi as u8)) as u32) as u32 as u64; rdx = r10; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 7; continue; }
            7 => { crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rdi); rdi = ((rax.wrapping_add(rbx)) as u64) as u64; fl_l = ((rsi as u8)) as u8 as u64; fl_r = ((rsi as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 10; continue; } bb = 8; continue; }
            8 => { fl_l = rbx; fl_r = (crate::simd_ops::ru64(P3)) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 10; continue; } bb = 9; continue; }
            9 => { r8 = rsp.wrapping_add(0x88); rdx = rbx; rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 10; continue; }
            10 => { rax = rdi; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rsp = ((rsp).wrapping_add(0x50u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            11 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rax = 0; fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x50u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425397a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14253a310; const P1: u64 = 0x143872f30; const P2: u64 = 0x1425398e0;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x60u64)) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rax); rsi = (((r8 as u8)) as u32) as u32 as u64; r10 = rdx; xmm0 = 0u128; crate::simd_ops::wu128(rdi, (xmm0) as u128); xmm0 = ((crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64) as u128; crate::simd_ops::wu64(rdi.wrapping_add(0x10u64), ((xmm0 as u64)) as u64); fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), rbx); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(P1)) as u64; r11 = rax; rdx = rcx.wrapping_sub(0x1); rax = ((rdx.wrapping_add(r10)) as u64) as u64; fl_l = rdx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { fl_l = ((!(rdx)) & (rax)) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; bb = 4; continue; }
            3 => { rdx = 0; fl_l = 0; fl_r = 0; let d = (rcx) as u128; let n = ((rdx as u128) << 64) | (rax as u128); if d != 0 { rax = (n / d) as u64; rdx = (n % d) as u64; } rbx = rax; rbx = (((rbx) as i64).wrapping_mul((rcx) as i64) as u64) as u64; bb = 4; continue; }
            4 => { rax = rsp.wrapping_add(0x98); crate::simd_ops::wu8(rsp.wrapping_add(0x70u64), (0x0u8) as u8); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); r8 = (((rsi as u8)) as u32) as u32 as u64; rax = rsp.wrapping_add(0x90); crate::simd_ops::wu8(rsp.wrapping_add(0x90u64), (0x0u8) as u8); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); rdx = rbx; rax = rsp.wrapping_add(0x70); crate::simd_ops::wu64(rsp.wrapping_add(0x98u64), (0x0u64) as u64); rcx = r11; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rax); rax = ((crate::simd_ops::ru8(rsp.wrapping_add(0x90u64))) as u32) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x54u64), (0x3u32) as u32); xmm0 = ((crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64) as u128; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), ((xmm0 as u64)) as u64); crate::simd_ops::wu8(rsp.wrapping_add(0x52u64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(rsp.wrapping_add(0x70u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x50u64), ((rax as u8)) as u8); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x98u64))) as u64; xmm1 = 0u128; crate::simd_ops::wu128(rdi, (xmm1) as u128); crate::simd_ops::wu8(rsp.wrapping_add(0x51u64), ((rsi as u8)) as u8); xmm0 = ((crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64) as u128; crate::simd_ops::wu64(rdi.wrapping_add(0x10u64), ((xmm0 as u64)) as u64); crate::simd_ops::wu64(rdi, rax); crate::simd_ops::wu64(rdi.wrapping_add(0x8u64), rbx); bb = 6; continue; }
            6 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rax = rcx; bb = 7; continue; }
            7 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; rsp = ((rsp).wrapping_add(0x60u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425398e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143872f30; const P1: u64 = 0x142539b00; const P2: u64 = 0x143425a90; const P3: u64 = 0x142537a30; const P4: u64 = 0x142539e50; const P5: u64 = 0x143872f49; const P6: u64 = 0x142539460;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; r10 = 0; fl_l = 0; fl_r = 0; rbp = (((r8 as u8)) as u32) as u32 as u64; fl_l = ((r8 as u8)) as u8 as u64; fl_r = ((r8 as u8)) as u8 as u64; rax = (((r9 as u8)) as u32) as u32 as u64; r8 = (crate::simd_ops::ru64(P0)) as u64; rdi = rdx; r10 = (if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { (rax as u32) } else { (r10 as u32) }) as u32 as u64; fl_l = rdx; fl_r = r8; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 23; continue; } bb = 1; continue; }
            1 => { r14 = rdx.wrapping_sub(0x1); fl_l = rdx; fl_r = r14; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 23; continue; } bb = 2; continue; }
            2 => { rax = r8.wrapping_sub(0x1); rcx = ((rcx).wrapping_add(rax)) as u64; fl_l = rax; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { fl_l = ((!(rax)) & (rcx)) as u64; fl_r = fl_l; rsi = ((fl_l as u64)) as u64; bb = 5; continue; }
            4 => { rdx = 0; fl_l = 0; fl_r = 0; rax = rcx; let d = (r8) as u128; let n = ((rdx as u128) << 64) | (rax as u128); if d != 0 { rax = (n / d) as u64; rdx = (n % d) as u64; } rsi = rax; rsi = (((rsi) as i64).wrapping_mul((r8) as i64) as u64) as u64; bb = 5; continue; }
            5 => { r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x88u64))) as u64; r9 = (((r10 as u8)) as u32) as u32 as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; r8 = (((rbp as u8)) as u32) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), r15); rdx = rdi; rcx = rsi; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r12); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rbx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 23; continue; } bb = 6; continue; }
            6 => { rdx = 0; fl_l = 0; fl_r = 0; let d = (rdi) as u128; let n = ((rdx as u128) << 64) | (rax as u128); if d != 0 { rax = (n / d) as u64; rdx = (n % d) as u64; } fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; rax = rbx; crate::simd_ops::wu64(rcx, rbx); bb = 24; continue; }
            8 => { r9 = rdi; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rbp as u32)) as u32); r8 = rbx; rcx = P2; rdx = rsi; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); r8 = (((rbp as u8)) as u32) as u32 as u64; rdx = rsi; rcx = rbx; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rax = rdi; rax = (!(rax)) as u64; fl_l = rsi; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 23; continue; } bb = 9; continue; }
            9 => { r9 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), r15); rdi = ((rdi).wrapping_add(rsi)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r12); fl_l = (crate::simd_ops::ru8(P5)) as u8 as u64; fl_r = ((r9 as u8)) as u8 as u64; rcx = rdi; rdx = ((r9.wrapping_add(0x1u64)) as u32) as u32 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 13; continue; } bb = 10; continue; }
            10 => { r8 = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 23; continue; } bb = 11; continue; }
            11 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; crate::simd_ops::wu64(rcx, rax); rax = ((rax).wrapping_add(r14)) as u64; fl_l = ((!(r14)) & (rax)) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; fl_l = ((rbp as u8)) as u8 as u64; fl_r = ((rbp as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 22; continue; } bb = 12; continue; }
            12 => { r8 = 0; fl_l = 0; fl_r = 0; rdx = rsi; rcx = rbx; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rax = rbx; bb = 24; continue; }
            13 => { r8 = (((rbp as u8)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); r9 = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 23; continue; } bb = 14; continue; }
            14 => { r8 = (crate::simd_ops::ru64(P0)) as u64; rcx = ((r14.wrapping_add(rax)) as u64) as u64; fl_l = ((!(r14)) & (rcx)) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; rcx = rbx; rcx = ((rcx).wrapping_sub(rax)) as u64; rax = r8.wrapping_sub(0x1); r10 = ((rax.wrapping_add(rsi)) as u64) as u64; fl_l = rax; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 16; continue; } bb = 15; continue; }
            15 => { fl_l = ((!(rax)) & (r10)) as u64; fl_r = fl_l; rsi = ((fl_l as u64)) as u64; bb = 17; continue; }
            16 => { rdx = 0; fl_l = 0; fl_r = 0; rax = r10; let d = (r8) as u128; let n = ((rdx as u128) << 64) | (rax as u128); if d != 0 { rax = (n / d) as u64; rdx = (n % d) as u64; } rsi = rax; rsi = (((rsi) as i64).wrapping_mul((r8) as i64) as u64) as u64; bb = 17; continue; }
            17 => { rdi = ((rdi).wrapping_sub(rsi)) as u64; rdi = ((rdi).wrapping_sub(rcx)) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 19; continue; } bb = 18; continue; }
            18 => { rdx = rcx; r8 = (((rbp as u8)) as u32) as u32 as u64; rcx = r9; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 19; continue; }
            19 => { fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 21; continue; } bb = 20; continue; }
            20 => { rcx = ((rsi.wrapping_add(rbx)) as u64) as u64; r8 = (((rbp as u8)) as u32) as u32 as u64; rdx = rdi; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 21; continue; }
            21 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; crate::simd_ops::wu64(rax, rbx); bb = 22; continue; }
            22 => { rax = rbx; bb = 24; continue; }
            23 => { rax = 0; fl_l = 0; fl_r = 0; bb = 24; continue; }
            24 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142539b00(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143872f38; const P1: u64 = 0x143872cb4; const P2: u64 = 0x143872cb0; const P3: u64 = 0x1425375a0; const P4: u64 = 0x143c0e188; const P5: u64 = 0x142541030; const P6: u64 = 0x142645708; const P7: u64 = 0x143425950; const P8: u64 = 0x142537a30; const P9: u64 = 0x143a09600; const P10: u64 = 0x1425408f0; const P11: u64 = 0x143a09620;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x48u64)) as u64; r14 = (((r8 as u8)) as u32) as u32 as u64; rbx = rcx; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = 0; fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x48u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            2 => { rcx = (crate::simd_ops::ru64(P0)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rbp); rbp = 0x3000; crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), rsi); rsi = 0x1; crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), r12); r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), r15); r15 = 0; fl_l = 0; fl_r = 0; fl_l = ((r8 as u8)) as u8 as u64; fl_r = ((r8 as u8)) as u8 as u64; rax = (((r9 as u8)) as u32) as u32 as u64; r15 = (if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { (rax as u32) } else { (r15 as u32) }) as u32 as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x88u64))) as u64; fl_l = rdx; fl_r = rdx; rsi = (if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { rdx } else { rsi }) as u64; fl_l = ((r8 as u8)) as u8 as u64; fl_r = ((r8 as u8)) as u8 as u64; crate::simd_ops::wu8(rax, (0x1u8) as u8); rax = 0x2000; rbp = (if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { (rax as u32) } else { (rbp as u32) }) as u32 as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru32(P1)) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rcx = P2; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(P0)) as u64; bb = 5; continue; }
            5 => { fl_l = (crate::simd_ops::ru32(P2)) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 17; continue; } bb = 6; continue; }
            6 => { rdx = 0; fl_l = 0; fl_r = 0; rax = rbx; let d = (rcx) as u128; let n = ((rdx as u128) << 64) | (rax as u128); if d != 0 { rax = (n / d) as u64; rdx = (n % d) as u64; } fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 9; continue; } bb = 7; continue; }
            7 => { rax = rsi; let d = (rcx) as u128; let n = ((rdx as u128) << 64) | (rax as u128); if d != 0 { rax = (n / d) as u64; rdx = (n % d) as u64; } fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 10; continue; }
            9 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 10; continue; }
            10 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 17; continue; } bb = 11; continue; }
            11 => { fl_l = ((r15 as u8)) as u8 as u64; fl_r = ((r15 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 17; continue; } bb = 12; continue; }
            12 => { fl_l = ((((rbp as u32)) >> (12u32)) & 1) as u64; if ((fl_l as u32) == 0) { bb = 17; continue; } bb = 13; continue; }
            13 => { rax = (crate::simd_ops::ru64(P4)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 15; continue; } bb = 14; continue; }
            14 => { rcx = rax.wrapping_sub(0x1); fl_l = rax; let cx_src: u64 = rcx; let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u64(P4, fl_l as u64, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u64)) as u64; } bb = 17; continue; }
            15 => { r9 = ((rbp as u32)) as u32 as u64; crate::simd_ops::wu8(r12, (0x1u8) as u8); fl_l = ((((r9 as u32)) >> (29u32)) & 1) as u64; r9 = (((r9 as u32)) | ((1u32) << (29u32))) as u32 as u64; r8 = rsi; rdx = rbx; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rdi = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 18; continue; } bb = 16; continue; }
            16 => { crate::simd_ops::wu64(P4, (0xau64) as u64); bb = 17; continue; }
            17 => { r9 = ((rbp as u32)) as u32 as u64; crate::simd_ops::wu8(r12, (0x0u8) as u8); r8 = rsi; rdx = rbx; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rdi = rax; bb = 18; continue; }
            18 => { r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 21; continue; } bb = 19; continue; }
            19 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 23; continue; } bb = 20; continue; }
            20 => { rcx = (((r15 as u8)) as u32) as u32 as u64; r9 = rbx; crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((rcx as u32)) as u32); r8 = ((rax as u32)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((r14 as u32)) as u32); rcx = P7; rdx = ((rax as u32)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); bb = 23; continue; }
            21 => { rdx = rbx; rcx = P9; rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp); fl_l = ((r14 as u8)) as u8 as u64; fl_r = ((r14 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 23; continue; } bb = 22; continue; }
            22 => { rdx = rbx; rcx = P11; rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp); bb = 23; continue; }
            23 => { r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rax = rdi; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rsp = ((rsp).wrapping_add(0x48u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142539cc0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14253a310; const P1: u64 = 0x142645738; const P2: u64 = 0x142645708; const P3: u64 = 0x1426455e0; const P4: u64 = 0x143425900; const P5: u64 = 0x142537a30; const P6: u64 = 0x143a09620; const P7: u64 = 0x1425408f0; const P8: u64 = 0x143a09600; const P9: u64 = 0x142539e50;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x60u64)) as u64; r11 = (crate::simd_ops::ru32(r9.wrapping_add(0x14u64))) as u32 as u64; rdi = (((r8 as u8)) as u32) as u32 as u64; r10 = rcx; rax = ((r11.wrapping_sub(0x3u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 22; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), rbx); rcx = rdx; crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), rsi); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rsi = rax; rbx = r10; rax = (crate::simd_ops::ru64(r9)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { r10 = ((r10).wrapping_sub(rax)) as u64; rbx = rax; fl_l = ((rsi).wrapping_add(r10)) as u64; fl_r = fl_l; rsi = ((fl_l as u64)) as u64; bb = 3; continue; }
            3 => { fl_l = ((r11 as u32)) as u32 as u64; fl_r = 0x4; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 20; continue; } bb = 4; continue; }
            4 => { fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 21; continue; } bb = 5; continue; }
            5 => { fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 21; continue; } bb = 6; continue; }
            6 => { fl_l = rsi; fl_r = (0x40000000u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 21; continue; } bb = 7; continue; }
            7 => { fl_l = ((rsi).wrapping_shr(30u32)) as u64; rsi = ((fl_l as u64)) as u64; bb = 8; continue; }
            8 => { fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 18; continue; } bb = 9; continue; }
            9 => { rdx = 0; fl_l = 0; fl_r = 0; r8 = 0x8000; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 17; continue; } bb = 10; continue; }
            10 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1e7; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 15; continue; } bb = 11; continue; }
            11 => { xmm0 = 0u128; ymm0_hi = 0; let y_addr: u64 = rsp.wrapping_add(0x30u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); crate::simd_ops::wu128(rsp.wrapping_add(0x50u64), (xmm0) as u128); r8 = 0x30; rdx = rsp.wrapping_add(0x30); rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; fl_l = rcx; fl_r = rbx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 16; continue; } bb = 12; continue; }
            12 => { rax = rbx; rax = ((rax).wrapping_sub(rcx)) as u64; fl_l = rax; fl_r = (0x2000000u64) as u64; if ((((fl_l as u64)) as i64) >= (((fl_r as u64)) as i64)) { bb = 16; continue; } bb = 13; continue; }
            13 => { rdx = 0; fl_l = 0; fl_r = 0; r8 = 0x8000; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 17; continue; } bb = 14; continue; }
            14 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = ((rax as u32)) as u32 as u64; bb = 15; continue; }
            15 => { fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 17; continue; } bb = 16; continue; }
            16 => { r9 = 0x40000000; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbx); r8 = ((rdi as u32)) as u32 as u64; rcx = P4; rdx = ((rdi as u32)) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 17; continue; }
            17 => { rdx = (0xffffffffc0000000u64) as u64; rcx = P6; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = (0xffffffffc0000000u64) as u64; rcx = P8; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 18; continue; }
            18 => { rbx = ((rbx).wrapping_add(0x40000000u64)) as u64; fl_l = rsi; fl_r = (0x1u64) as u64; rsi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 8; continue; } bb = 19; continue; }
            19 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rsp = ((rsp).wrapping_add(0x60u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            20 => { r8 = (((rdi as u8)) as u32) as u32 as u64; rdx = rsi; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 21; continue; }
            21 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; bb = 22; continue; }
            22 => { rsp = ((rsp).wrapping_add(0x60u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142539e50(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142645738; const P1: u64 = 0x142645708; const P2: u64 = 0x1426455e0; const P3: u64 = 0x143425900; const P4: u64 = 0x142537a30; const P5: u64 = 0x143a09620; const P6: u64 = 0x1425408f0; const P7: u64 = 0x143a09600;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 15; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = ((rsp).wrapping_sub(0x60u64)) as u64; rbp = (((r8 as u8)) as u32) as u32 as u64; rbx = rdx; rsi = rcx; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 2; continue; }
            2 => { rdx = 0; fl_l = 0; fl_r = 0; r8 = 0x8000; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 11; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), rdi); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1e7; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 8; continue; } bb = 4; continue; }
            4 => { xmm0 = 0u128; ymm0_hi = 0; let y_addr: u64 = rsp.wrapping_add(0x30u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); crate::simd_ops::wu128(rsp.wrapping_add(0x50u64), (xmm0) as u128); r8 = 0x30; rdx = rsp.wrapping_add(0x30); rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; fl_l = rcx; fl_r = rsi; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 9; continue; } bb = 5; continue; }
            5 => { rax = rsi; rax = ((rax).wrapping_sub(rcx)) as u64; fl_l = rax; fl_r = (0x2000000u64) as u64; if ((((fl_l as u64)) as i64) >= (((fl_r as u64)) as i64)) { bb = 9; continue; } bb = 6; continue; }
            6 => { rdx = 0; fl_l = 0; fl_r = 0; r8 = 0x8000; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 10; continue; } bb = 7; continue; }
            7 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = ((rax as u32)) as u32 as u64; bb = 8; continue; }
            8 => { fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { r9 = rbx; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); r8 = ((rdi as u32)) as u32 as u64; rcx = P3; rdx = ((rdi as u32)) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 10; continue; }
            10 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; bb = 11; continue; }
            11 => { rbx = (0u64.wrapping_sub(rbx)) as u64; fl_l = ((rbp as u8)) as u8 as u64; fl_r = ((rbp as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 13; continue; } bb = 12; continue; }
            12 => { rdx = rbx; rcx = P5; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 13; continue; }
            13 => { rdx = rbx; rcx = P7; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 14; continue; }
            14 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; rsp = ((rsp).wrapping_add(0x60u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); bb = 15; continue; }
            15 => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142539f60(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xfffffffffffffffe; const P1: u64 = 0x1ffffff; const P2: u64 = 0x143c55080; const P3: u64 = 0xffffffffffffffff; const P4: u64 = 0x1425466e0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = ((rcx.wrapping_add(P0)) as u64) as u64; rsi = rcx; fl_l = rax; fl_r = (0x1fffffeu64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 6; continue; } bb = 1; continue; }
            1 => { rdi = ((rdx.wrapping_add(P1)) as u64) as u64; rdi = (((rdi) & (0xfffffffffe000000u64))) as u64; fl_l = rdi; fl_r = (0x40000000u64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 6; continue; } bb = 2; continue; }
            2 => { rbx = rdi; let xa_src: u64 = rbx; let xa_old: u64 = crate::simd_ops::atomic_xadd_u64(P2, xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rbx = (xa_old) as u64; rax = ((rbx.wrapping_add(P3)) as u64) as u64; rcx = (0x1dffffffffffu64) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = ((crate::exe::tls_pointer() as u64)) as u64; rdx = 0x6348; rcx = (crate::simd_ops::ru64(rax)) as u64; rcx = (crate::simd_ops::ru64(rdx.wrapping_add(rcx))) as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rcx = rax; rdx = (0x3ffffffffffu64) as u64; rcx = ((rcx).wrapping_shr(17u32)) as u64; rax = ((rdi.wrapping_add(rbx)) as u64) as u64; rcx = ((rcx).wrapping_shl(25u32)) as u64; rcx = (((rcx) & (rdx))) as u64; rdx = (0x20000000000u64) as u64; rcx = ((rcx).wrapping_add(rdx)) as u64; fl_l = rax; let cx_src: u64 = rcx; let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u64(P2, fl_l as u64, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u64)) as u64; } rbx = rdi; let xa_src: u64 = rbx; let xa_old: u64 = crate::simd_ops::atomic_xadd_u64(P2, xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rbx = (xa_old) as u64; bb = 4; continue; }
            4 => { rdx = 0; fl_l = 0; fl_r = 0; rax = rbx; let d = (rsi) as u128; let n = ((rdx as u128) << 64) | (rax as u128); if d != 0 { rax = (n / d) as u64; rdx = (n % d) as u64; } fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            6 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = 0; fl_l = 0; fl_r = 0; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14253a0b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143872f38; const P1: u64 = 0x143872cb4; const P2: u64 = 0x143872cb0; const P3: u64 = 0x1425375a0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; r8 = (crate::simd_ops::ru64(P0)) as u64; rbx = rdx; rdi = rcx; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru32(P1)) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rcx = P2; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); r8 = (crate::simd_ops::ru64(P0)) as u64; bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru32(P2)) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 7; continue; } bb = 4; continue; }
            4 => { rdx = 0; fl_l = 0; fl_r = 0; rax = rdi; let d = (r8) as u128; let n = ((rdx as u128) << 64) | (rax as u128); if d != 0 { rax = (n / d) as u64; rdx = (n % d) as u64; } fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 7; continue; } bb = 5; continue; }
            5 => { rax = rbx; let d = (r8) as u128; let n = ((rdx as u128) << 64) | (rax as u128); if d != 0 { rax = (n / d) as u64; rdx = (n % d) as u64; } fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            7 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14253a130(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x143872f38; const P1: u64 = 0x143872f30;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rcx = (crate::simd_ops::ru64(P0)) as u64;
    rax = (crate::simd_ops::ru64(P1)) as u64;
    fl_l = rcx; fl_r = rcx;
    rax = (if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { rcx } else { rax }) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14253a1b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x11; const P2: u64 = 0x12;
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
    crate::simd_ops::wu32(rsp.wrapping_add(0x14u64), (0x3u32) as u32);
    xmm0 = ((crate::simd_ops::ru64(rsp.wrapping_add(0x10u64))) as u64) as u128;
    crate::simd_ops::wu64(rcx.wrapping_add(P0), ((xmm0 as u64)) as u64);
    crate::simd_ops::wu8(rcx.wrapping_add(P1), ((rdx as u8)) as u8);
    crate::simd_ops::wu8(rcx.wrapping_add(P2), ((r8 as u8)) as u8);
    crate::simd_ops::wu8(rcx.wrapping_add(P0), ((r9 as u8)) as u8);
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14253a2f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x143c55080;
    let (mut rax, mut fl_l) = (crate::simd_ops::get_rax_in(), 0u64);
    let xa_src: u64 = rdx; let xa_old: u64 = crate::simd_ops::atomic_xadd_u64(P0, xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64;
    rdx = (xa_old) as u64;
    rax = rdx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14253a550(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14253a310; const P1: u64 = 0x143872f38; const P2: u64 = 0x143872cb4; const P3: u64 = 0x143872cb0; const P4: u64 = 0x1425375a0; const P5: u64 = 0x142541030; const P6: u64 = 0x142645708; const P7: u64 = 0x143425950; const P8: u64 = 0x142537a30; const P9: u64 = 0x143a09600; const P10: u64 = 0x1425408f0; const P11: u64 = 0x143a09620;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = ((rsp).wrapping_sub(0x60u64)) as u64; xmm0 = 0u128; rax = 0; fl_l = 0; fl_r = 0; rsi = rdx; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rax); crate::simd_ops::wu128(rdx, (xmm0) as u128); xmm0 = ((crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64) as u128; crate::simd_ops::wu64(rdx.wrapping_add(0x10u64), ((xmm0 as u64)) as u64); fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x80u64), rdi); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rbx = 0; fl_l = 0; fl_r = 0; bb = 11; continue; }
            3 => { fl_l = (crate::simd_ops::ru64(P1)) as u64; fl_r = (0x0u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), rbp); if (((fl_l as u64)) == ((fl_r as u64))) { bb = 6; continue; } bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::ru32(P2)) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rcx = P3; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 6; continue; }
            6 => { r9 = 0x3000; r8 = 0x1; rdx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = rax; rbp = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 9; continue; } bb = 7; continue; }
            7 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 10; continue; } bb = 8; continue; }
            8 => { crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((rbx as u32)) as u32); rcx = P7; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0x1u32) as u32); r9 = rdi; r8 = ((rax as u32)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (0x1u64) as u64); rdx = ((rax as u32)) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 10; continue; }
            9 => { rdx = rdi; rcx = P9; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P10, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rdi; rcx = P11; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P10, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rax); crate::simd_ops::wu32(rsp.wrapping_add(0x54u64), (0x3u32) as u32); xmm0 = ((crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64) as u128; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), ((xmm0 as u64)) as u64); crate::simd_ops::wu16(rsp.wrapping_add(0x51u64), (0x101u16) as u16); crate::simd_ops::wu8(rsp.wrapping_add(0x50u64), ((rax as u8)) as u8); xmm0 = ((crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64) as u128; xmm1 = 0u128; crate::simd_ops::wu128(rsi, (xmm1) as u128); crate::simd_ops::wu64(rsi.wrapping_add(0x10u64), ((xmm0 as u64)) as u64); bb = 10; continue; }
            10 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; bb = 11; continue; }
            11 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; bb = 12; continue; }
            12 => { rsp = ((rsp).wrapping_add(0x60u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14253a6a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x142539cc0;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = ((rsp).wrapping_sub(0x58u64)) as u64;
    xmm0 = (crate::simd_ops::ru128(r8)) as u128;
    xmm1 = ((crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64) as u128;
    r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64));
    r9 = rsp.wrapping_add(0x30);
    crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128);
    crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), ((xmm1 as u64)) as u64);
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rsp = ((rsp).wrapping_add(0x58u64)) as u64;
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14253a6f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x143c55090;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (0x280000000000u64) as u64;
    fl_l = rcx; fl_r = rax;
    if (((fl_l as u64)) >= ((fl_r as u64))) { return rax; }
    rcx = ((rcx).wrapping_shr(25u32)) as u64;
    r8 = rcx;
    rcx = ((rcx).wrapping_shr(6u32)) as u64;
    r8 = ((((r8 as u32)) & (0x3fu32))) as u32 as u64;
    fl_l = rcx; fl_r = (0x5000u64) as u64;
    if (((fl_l as u64)) == ((fl_r as u64))) { return rax; }
    rax = P0;
    rdx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64;
    rax = (crate::simd_ops::ru64(rax.wrapping_add(rcx.wrapping_mul(8)))) as u64;
    loop { rcx = rax; fl_l = (((rcx) >> (((r8) as u32 & 63))) & 1) as u64; rcx = ((rcx) & !((1u64) << (((r8) as u32 & 63)))) as u64; fl_l = rax; let cx_src: u64 = rcx; let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u64(rdx, fl_l as u64, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u64)) as u64; } if !((((fl_l as u64)) != ((fl_r as u64)))) { break; } }
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14253a750(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x143c55090;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (0x280000000000u64) as u64;
    fl_l = rcx; fl_r = rax;
    if (((fl_l as u64)) >= ((fl_r as u64))) { return rax; }
    rcx = ((rcx).wrapping_shr(25u32)) as u64;
    r8 = rcx;
    rcx = ((rcx).wrapping_shr(6u32)) as u64;
    r8 = ((((r8 as u32)) & (0x3fu32))) as u32 as u64;
    fl_l = rcx; fl_r = (0x5000u64) as u64;
    if (((fl_l as u64)) == ((fl_r as u64))) { return rax; }
    rax = P0;
    rdx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64;
    rax = (crate::simd_ops::ru64(rax.wrapping_add(rcx.wrapping_mul(8)))) as u64;
    loop { rcx = rax; fl_l = (((rcx) >> (((r8) as u32 & 63))) & 1) as u64; rcx = ((rcx) | ((1u64) << (((r8) as u32 & 63)))) as u64; fl_l = rax; let cx_src: u64 = rcx; let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u64(rdx, fl_l as u64, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u64)) as u64; } if !((((fl_l as u64)) != ((fl_r as u64)))) { break; } }
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14253a7b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (0x280000000000u64) as u64;
    fl_l = rcx; fl_r = rax;
    if !((((fl_l as u64)) < ((fl_r as u64)))) { crate::simd_ops::wu64(rdx, (0x0u64) as u64); rax = 0x5000; return rax; }
    rcx = ((rcx).wrapping_shr(25u32)) as u64;
    rax = rcx;
    rcx = ((rcx).wrapping_shr(6u32)) as u64;
    fl_l = ((((rax as u32)) & (0x3fu32))) as u32 as u64; fl_r = fl_l;
    rax = ((fl_l as u32)) as u32 as u64;
    crate::simd_ops::wu64(rdx, rax);
    rax = rcx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14253a840(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14253b9c0; const P1: u64 = 0x143a095c0; const P2: u64 = 0x1425391a0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rax = rsp.wrapping_add(0x60); rbx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rbx); rsi = rdx; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); r9 = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 1; continue; }
            1 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 2; continue; }
            2 => { r8 = (crate::simd_ops::ru32(rdi.wrapping_add(0x1cu64))) as u32 as u64; rcx = rdi; rcx = ((rcx).wrapping_sub(rax)) as u64; rax = (0x6666666666666667u64) as u64; rcx = ((rcx).wrapping_sub(0x108u64)) as u64; let prod = (rax as i64 as i128).wrapping_mul((rcx) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rcx = rdx; rcx = ((((rcx) as i64).wrapping_shr(5u32) as u64)) as u64; rax = rcx; rax = ((rax).wrapping_shr(63u32)) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rdi)) as u32 as u64; rax = ((rax).wrapping_shl(16u32)) as u64; fl_l = r8; fl_r = (0x8u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 6; continue; } bb = 3; continue; }
            3 => { fl_l = r8; fl_r = (0x40u64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rbx = ((r8.wrapping_add(r8.wrapping_mul(2))) as u64) as u64; bb = 6; continue; }
            5 => { fl_l = r8; fl_r = (0x200u64) as u64; rbx = (if (((fl_l as u64)) <= ((fl_r as u64))) { r8 } else { rbx }) as u64; bb = 6; continue; }
            6 => { rcx = ((rcx).wrapping_shl(16u32)) as u64; rax = ((rax).wrapping_sub(rbx)) as u64; rcx = ((rcx).wrapping_add(rbx)) as u64; rdx = 0x80000000; rcx = ((rcx).wrapping_add(r9)) as u64; fl_l = rax; fl_r = rdx; rax = (if (((fl_l as u64)) > ((fl_r as u64))) { (rdx as u32) } else { (rax as u32) }) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x1cu64), ((rax as u32)) as u32); fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 7; continue; }
            7 => { fl_l = (crate::simd_ops::ru8(r9.wrapping_add(0x18u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 12; continue; } bb = 8; continue; }
            8 => { rax = rsi.wrapping_sub(0x1); r9 = ((rax.wrapping_add(rcx)) as u64) as u64; fl_l = rsi; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { fl_l = ((!(rax)) & (r9)) as u64; fl_r = fl_l; r9 = ((fl_l as u64)) as u64; bb = 11; continue; }
            10 => { rdx = 0; fl_l = 0; fl_r = 0; rax = r9; let d = (rsi) as u128; let n = ((rdx as u128) << 64) | (rax as u128); if d != 0 { rax = (n / d) as u64; rdx = (n % d) as u64; } fl_l = r9; fl_r = rdx; r9 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; bb = 11; continue; }
            11 => { rcx = ((rcx).wrapping_add(0x8u64)) as u64; r8 = P1; r9 = ((r9).wrapping_sub(rcx)) as u64; rdx = r9; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 12; continue; }
            12 => { rax = rdi; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            13 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rax = 0; fl_l = 0; fl_r = 0; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14253aa70(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14253abc0; const P1: u64 = 0xb28; const P2: u64 = 0x14253b9c0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x30u64)) as u64;
    rbx = r9;
    rdi = rcx;
    r9 = rsp.wrapping_add(0x60);
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbx);
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    fl_l = (crate::simd_ops::ru8(rsp.wrapping_add(0x60u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64;
    if !((((fl_l as u8)) == ((fl_r as u8)))) { rax = 0; fl_l = 0; fl_r = 0; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    fl_l = rax; fl_r = rax;
    if !(((((fl_l as u64)) & ((fl_r as u64))) != 0)) { r8 = (crate::simd_ops::ru32(rdi.wrapping_add(P1))) as u32 as u64; r9 = rbx; rdx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64;
    rsp = ((rsp).wrapping_add(0x30u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14253aae0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14253b550; const P1: u64 = 0x14253b430; const P2: u64 = 0x14253b0b0; const P3: u64 = 0xd0; const P4: u64 = 0x14253add0; const P5: u64 = 0x380; const P6: u64 = 0x14253c770; const P7: u64 = 0x143c7d0c0; const P8: u64 = 0xb0; const P9: u64 = 0x143c7d140;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; fl_l = ((rdx as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; rdi = 0x400; rax = 0x4000; rsi = r8; rdi = (if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { (rax as u32) } else { (rdi as u32) }) as u32 as u64; rbp = (((rdx as u8)) as u32) as u32 as u64; r14 = rcx; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            2 => { rdi = (((rdi as u32)).wrapping_sub(1)) as u32 as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rbx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 3; continue; }
            3 => { r9 = rsi; r8 = 0; fl_l = 0; fl_r = 0; rdx = 0; fl_l = 0; fl_r = 0; rcx = rax; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(P3))) as u64; fl_r = (0x0u64) as u64; rcx = rbx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 5; continue; } bb = 4; continue; }
            4 => { r9 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); r8 = 0; fl_l = 0; fl_r = 0; rdx = r14; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 8; continue; }
            5 => { r8 = (crate::simd_ops::ru64(rsi.wrapping_add(P5))) as u64; rdx = (((rbp as u8)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(P7)) as u64; bb = 6; continue; }
            6 => { crate::simd_ops::wu64(rbx.wrapping_add(P8), rax); fl_l = rax; let cx_src: u64 = rbx; let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u64(P7, fl_l as u64, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u64)) as u64; } if (((fl_l as u64)) != ((fl_r as u64))) { bb = 6; continue; } bb = 7; continue; }
            7 => { fl_l = (crate::simd_ops::lk_add_u64(P9, (1) as u64)) as u64; bb = 8; continue; }
            8 => { fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) > 0) { bb = 2; continue; } bb = 9; continue; }
            9 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14253abc0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143872e94; const P1: u64 = 0x143872e90; const P2: u64 = 0x1425375a0; const P3: u64 = 0x14253b430; const P4: u64 = 0x14253b0b0; const P5: u64 = 0x14253c770; const P6: u64 = 0x143c7d0c0; const P7: u64 = 0x143c7d140; const P8: u64 = 0x14253add0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 15>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x50u64)) as u64; crate::simd_ops::wu8(r9, (0x0u8) as u8); r13 = r9; fl_l = (crate::simd_ops::ru32(P0)) as u32 as u64; fl_r = 0x0; r15 = r8; r12 = rdx; r14 = rcx; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = P1; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 2; continue; }
            2 => { rsi = (crate::simd_ops::ru32(P1)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x80u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x88u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x90u64), rdi); fl_l = ((rsi as u32)) as u32 as u64; fl_r = 0x8; if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { bb = 4; continue; } bb = 3; continue; }
            3 => { rsi = 0x8; bb = 5; continue; }
            4 => { rax = 0x400; fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; rsi = (if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { (rax as u32) } else { (rsi as u32) }) as u32 as u64; fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 24; continue; } bb = 5; continue; }
            5 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0xa0u64))) as u64; bb = 6; continue; }
            6 => { rsi = (((rsi as u32)).wrapping_sub(1)) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 24; continue; } bb = 7; continue; }
            7 => { crate::simd_ops::lk_add_u64(rax.wrapping_add(0xc8u64), (1) as u64); xmm0 = ((crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64) as u128; xmm1 = (crate::simd_ops::ru128(rax)) as u128; rcx = (crate::simd_ops::ru32(r14.wrapping_add(0xb28u64))) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rax = ((rax).wrapping_shr(32u32)) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x6; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 13; continue; } bb = 8; continue; }
            8 => { rax = (((xmm1 >> 64) as u64)) as u64; rax = ((rax).wrapping_shr(32u32)) as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 12; continue; } bb = 10; continue; }
            10 => { rax = (((xmm1 >> 64) as u32) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 12; continue; } bb = 11; continue; }
            11 => { rdi = (rdi & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 14; continue; }
            12 => { rdi = (rdi & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 14; continue; }
            13 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; rdi = (rdi & !0xffu64) | (((if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { 1u8 } else { 0u8 }) as u8 as u64)); bb = 14; continue; }
            14 => { r9 = rbp; r8 = r15; rdx = r12; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(0xd0u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 22; continue; } bb = 15; continue; }
            15 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 17; continue; } bb = 16; continue; }
            16 => { fl_l = ((rdi as u8)) as u8 as u64; fl_r = ((rdi as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 26; continue; } bb = 17; continue; }
            17 => { fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(0xc8u64))) as u64; fl_r = (0x3u64) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 19; continue; } bb = 18; continue; }
            18 => { fl_l = ((rdi as u8)) as u8 as u64; fl_r = ((rdi as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 22; continue; } bb = 19; continue; }
            19 => { r8 = (crate::simd_ops::ru64(rbp.wrapping_add(0x380u64))) as u64; rdx = (rdx & !0xffu64) | (((0x1u8) as u8 as u64)); rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(P6)) as u64; bb = 20; continue; }
            20 => { crate::simd_ops::wu64(rbx.wrapping_add(0xb0u64), rax); fl_l = rax; let cx_src: u64 = rbx; let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u64(P6, fl_l as u64, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u64)) as u64; } if (((fl_l as u64)) != ((fl_r as u64))) { bb = 20; continue; } bb = 21; continue; }
            21 => { fl_l = (crate::simd_ops::lk_add_u64(P7, (1) as u64)) as u64; bb = 23; continue; }
            22 => { r9 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbp); r8 = 0; fl_l = 0; fl_r = 0; rdx = r14; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 23; continue; }
            23 => { fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) > 0) { bb = 6; continue; } bb = 24; continue; }
            24 => { rax = 0; fl_l = 0; fl_r = 0; bb = 25; continue; }
            25 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x88u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; rsp = ((rsp).wrapping_add(0x50u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            26 => { r9 = r13; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbp); r8 = r15; rdx = r14; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 25; continue; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14253ad70(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14253b430; const P1: u64 = 0x14253add0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x30u64)) as u64;
    rbx = rdx;
    rdi = rcx;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    fl_l = rax; fl_r = rax;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    loop { r9 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbx); r8 = 0; fl_l = 0; fl_r = 0; rdx = rdi; rcx = rax; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if !(((((fl_l as u64)) & ((fl_r as u64))) != 0)) { break; } }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64;
    rsp = ((rsp).wrapping_add(0x30u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14253add0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14253d8f0; const P1: u64 = 0x1425408f0; const P2: u64 = 0x1434254d8; const P3: u64 = 0x1425386a0; const P4: u64 = 0x14253b6b0; const P5: u64 = 0x142536cb0; const P6: u64 = 0x142536aa0; const P7: u64 = 0x14253c400; const P8: u64 = 0x14253b840;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; r14 = r9; r12 = r8; rbp = rdx; rdi = rcx; fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu8(r9, (0x0u8) as u8); bb = 2; continue; }
            2 => { rax = ((crate::exe::teb_self() as u64)) as u64; r13 = 0; fl_l = 0; fl_r = 0; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; crate::simd_ops::wu64(rcx.wrapping_add(0x100u64), rax); rdx = rsi; crate::simd_ops::wu64(rcx.wrapping_add(0xc8u64), r13); rcx = (crate::simd_ops::ru32(rcx.wrapping_add(0xe0u64))) as u32 as u64; rcx = (((rcx as u32)).wrapping_shl(16u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x380u64))) as u64; rdx = r13.wrapping_sub(0x1); rcx = ((rcx).wrapping_add(0xe0u64)) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rdi.wrapping_add(0xf8u64))) as u64; r9 = rdi.wrapping_add(0x108); r15 = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = (crate::simd_ops::ru32(r9)) as u32 as u64; r15 = ((r15).wrapping_shl(4u32)) as u64; r15 = ((r15).wrapping_add(0x108u64)) as u64; r15 = ((r15).wrapping_add(rdi)) as u64; rbx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rbx = ((rbx).wrapping_shl(4u32)) as u64; rbx = ((rbx).wrapping_add(r9)) as u64; fl_l = rbx; fl_r = r15; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 42; continue; } bb = 3; continue; }
            3 => { r9 = 0x80000000; bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x1cu64))) as u32 as u64; fl_r = ((r13 as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 39; continue; } bb = 5; continue; }
            5 => { rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x380u64))) as u64; rdx = (0xffffffffffffffffu64) as u64; rcx = ((rcx).wrapping_add(0x100u64)) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::lk_sub_u64(rdi.wrapping_add(0xc0u64), (1) as u64)) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x30u64), rbp); bb = 6; continue; }
            6 => { rcx = r13; bb = 7; continue; }
            7 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x28u64))) as u64; r8 = rdx; rax = ((rdx as u32)) as u32 as u64; r8 = (((r8) & (0xfffffffffffffffcu64))) as u64; rax = ((((rax as u32)) & (0x3u32))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 10; continue; } bb = 8; continue; }
            8 => { fl_l = rcx; fl_r = (0x4u64) as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 6; continue; } bb = 9; continue; }
            9 => { fl_l = ((rcx).wrapping_add(1)) as u64; rcx = ((fl_l as u64)) as u64; bb = 7; continue; }
            10 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 12; continue; } bb = 11; continue; }
            11 => { rax = rdx; fl_l = rax; let cx_src: u64 = r8; let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u64(rbx.wrapping_add(0x28u64), fl_l as u64, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u64)) as u64; } if (((fl_l as u64)) != ((fl_r as u64))) { bb = 7; continue; } bb = 12; continue; }
            12 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x28u64))) as u64; fl_l = rax; fl_r = (0xfffffffffffffffcu64) as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 24; continue; } bb = 13; continue; }
            13 => { bb = 14; continue; }
            14 => { rdx = rax; rcx = rax; rdx = (((rdx) & (0xfffffffffffffffcu64))) as u64; rcx = ((((rcx as u32)) & (0x3u32))) as u32 as u64; fl_l = rax; let cx_src: u64 = rcx; let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u64(rbx.wrapping_add(0x28u64), fl_l as u64, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u64)) as u64; } if (((fl_l as u64)) != ((fl_r as u64))) { bb = 14; continue; } bb = 15; continue; }
            15 => { fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 24; continue; } bb = 16; continue; }
            16 => { rax = (crate::simd_ops::ru64(rdx)) as u64; rcx = 0x1; r9 = ((crate::simd_ops::ru16(rbx.wrapping_add(0xau64))) as u32) as u32 as u64; r8 = rdx; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 20; continue; } bb = 17; continue; }
            17 => { bb = 18; continue; }
            18 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 21; continue; } bb = 19; continue; }
            19 => { r8 = rax; rcx = (((rcx as u32)).wrapping_add(1)) as u32 as u64; rax = (crate::simd_ops::ru64(rax)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 18; continue; } bb = 20; continue; }
            20 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 22; continue; } bb = 21; continue; }
            21 => { rdx = P2; rcx = 0xe; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 23; continue; }
            22 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(r8, rax); fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x18u64))) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x18u64), ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32); crate::simd_ops::wu64(rbx.wrapping_add(0x20u64), rdx); bb = 23; continue; }
            23 => { r9 = 0x80000000; bb = 24; continue; }
            24 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x20u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 27; continue; } bb = 25; continue; }
            25 => { fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; fl_r = r13; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 27; continue; } bb = 26; continue; }
            26 => { fl_l = (crate::simd_ops::lk_and_u8(rbx.wrapping_add(0xfu64), (0xfeu8) as u8)) as u8 as u64; fl_r = fl_l; crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), rax); crate::simd_ops::wu64(rbx.wrapping_add(0x20u64), r13); bb = 27; continue; }
            27 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x18u64))) as u32 as u64; fl_r = ((r13 as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 29; continue; } bb = 28; continue; }
            28 => { rdx = rsi; rcx = rbx; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 40; continue; }
            29 => { rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x1cu64))) as u32 as u64; fl_l = rcx; fl_r = r9; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 31; continue; } bb = 30; continue; }
            30 => { rcx = (crate::simd_ops::ru32(rbx)) as u32 as u64; fl_l = ((rcx).wrapping_shl(16u32)) as u64; rcx = ((fl_l as u64)) as u64; bb = 31; continue; }
            31 => { rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); r9 = (((rax as u8)) as u32) as u32 as u64; rdx = rbp.wrapping_add(0x410); rax = ((r9.wrapping_add(r9.wrapping_mul(2))) as u64) as u64; fl_l = (crate::simd_ops::ru64(rdx.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x10u64))) as u64; fl_r = (0x20010u64) as u64; rdx = ((rdx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rax = ((crate::simd_ops::ru8(rbx.wrapping_add(0xeu64))) as u32) as u32 as u64; rcx = (rcx & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); rax = (rax & !0xffu64) | ((((((rax as u8)) & (0xfeu8))) as u8 as u64)); rcx = (rcx & !0xffu64) | ((((((rcx as u8)) | ((rax as u8)))) as u8 as u64)); crate::simd_ops::wu8(rbx.wrapping_add(0xeu64), ((rcx as u8)) as u8); rax = (crate::simd_ops::ru64(rdx)) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x38u64), rax); crate::simd_ops::wu64(rbx.wrapping_add(0x40u64), r13); rax = (crate::simd_ops::ru64(rdx)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 33; continue; } bb = 32; continue; }
            32 => { crate::simd_ops::wu64(rax.wrapping_add(0x40u64), rbx); bb = 34; continue; }
            33 => { crate::simd_ops::wu64(rdx.wrapping_add(0x8u64), rbx); bb = 34; continue; }
            34 => { rcx = rbp; crate::simd_ops::wu64(rdx, rbx); rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::lk_add_u64(rbp.wrapping_add(0xbd0u64), (1) as u64); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x1cu64))) as u32 as u64; fl_l = r12; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 41; continue; } bb = 35; continue; }
            35 => { rax = ((crate::simd_ops::ru16(rbx.wrapping_add(0xcu64))) as u32) as u32 as u64; fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x18u64))) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 37; continue; } bb = 36; continue; }
            36 => { fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(0x28u64))) as u64; fl_r = (0xfffffffffffffffcu64) as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 41; continue; } bb = 37; continue; }
            37 => { fl_l = r14; fl_r = r14; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 41; continue; } bb = 38; continue; }
            38 => { crate::simd_ops::wu8(r14, (0x1u8) as u8); bb = 41; continue; }
            39 => { rdx = rsi; rcx = rbx; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); bb = 40; continue; }
            40 => { rbx = rax; bb = 41; continue; }
            41 => { rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; r9 = 0x80000000; rcx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rbx = ((rbx).wrapping_add(rcx)) as u64; fl_l = rbx; fl_r = r15; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 4; continue; } bb = 42; continue; }
            42 => { fl_l = (crate::simd_ops::ru64(rdi.wrapping_add(0xd0u64))) as u64; fl_r = r13; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 44; continue; } bb = 43; continue; }
            43 => { r8 = rsi; rcx = rdi; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); rax = 0; fl_l = 0; fl_r = 0; bb = 45; continue; }
            44 => { rax = rdi; bb = 45; continue; }
            45 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14253b0b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1434254d8; const P1: u64 = 0x1425386a0; const P2: u64 = 0x1425408f0; const P3: u64 = 0x14253b6b0;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(0xf8u64))) as u64; r11 = rcx.wrapping_add(0x108); r14 = rcx.wrapping_add(0x108); rdi = (rdi & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; r13 = r9; r15 = r8; rbp = rdx; rsi = rcx; r10 = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = (crate::simd_ops::ru32(r11)) as u32 as u64; r10 = ((r10).wrapping_shl(4u32)) as u64; r14 = ((r14).wrapping_add(r10)) as u64; rbx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rbx = ((rbx).wrapping_shl(4u32)) as u64; rbx = ((rbx).wrapping_add(r11)) as u64; fl_l = rbx; fl_r = r14; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 29; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), r12); r12 = 0; fl_l = 0; fl_r = 0; bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x1cu64))) as u32 as u64; fl_r = ((r12 as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 25; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x28u64))) as u64; fl_l = rax; fl_r = (0xfffffffffffffffcu64) as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 4; continue; }
            4 => { bb = 5; continue; }
            5 => { rdx = rax; rcx = rax; rdx = (((rdx) & (0xfffffffffffffffcu64))) as u64; rcx = ((((rcx as u32)) & (0x3u32))) as u32 as u64; fl_l = rax; let cx_src: u64 = rcx; let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u64(rbx.wrapping_add(0x28u64), fl_l as u64, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u64)) as u64; } if (((fl_l as u64)) != ((fl_r as u64))) { bb = 5; continue; } bb = 6; continue; }
            6 => { fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru64(rdx)) as u64; rcx = 0x1; r9 = ((crate::simd_ops::ru16(rbx.wrapping_add(0xau64))) as u32) as u32 as u64; r8 = rdx; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 8; continue; }
            8 => { bb = 9; continue; }
            9 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 12; continue; } bb = 10; continue; }
            10 => { r8 = rax; rcx = (((rcx as u32)).wrapping_add(1)) as u32 as u64; rax = (crate::simd_ops::ru64(rax)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 9; continue; } bb = 11; continue; }
            11 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 13; continue; } bb = 12; continue; }
            12 => { rdx = P0; rcx = 0xe; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 14; continue; }
            13 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(r8, rax); fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x18u64))) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x18u64), ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32); crate::simd_ops::wu64(rbx.wrapping_add(0x20u64), rdx); bb = 14; continue; }
            14 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x20u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 15; continue; }
            15 => { fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; fl_r = r12; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 17; continue; } bb = 16; continue; }
            16 => { fl_l = (crate::simd_ops::lk_and_u8(rbx.wrapping_add(0xfu64), (0xfeu8) as u8)) as u8 as u64; fl_r = fl_l; crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), rax); crate::simd_ops::wu64(rbx.wrapping_add(0x20u64), r12); bb = 17; continue; }
            17 => { rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x18u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 20; continue; } bb = 18; continue; }
            18 => { rcx = (crate::simd_ops::ru64(r13.wrapping_add(0x380u64))) as u64; rdx = (0xffffffffffffffffu64) as u64; rcx = ((rcx).wrapping_add(0x100u64)) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::lk_sub_u64(rsi.wrapping_add(0xc0u64), (1) as u64); rdx = r13; rcx = rbx; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rbx = rax; rax = (crate::simd_ops::ru32(rax)) as u32 as u64; fl_l = rax; fl_r = rbp; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 24; continue; } bb = 19; continue; }
            19 => { rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; rdi = (rdi & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 27; continue; }
            20 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x1cu64))) as u32 as u64; fl_l = rax; fl_r = r15; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 24; continue; } bb = 21; continue; }
            21 => { rax = ((crate::simd_ops::ru16(rbx.wrapping_add(0xcu64))) as u32) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 23; continue; } bb = 22; continue; }
            22 => { fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(0x28u64))) as u64; fl_r = (0xfffffffffffffffcu64) as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 24; continue; } bb = 23; continue; }
            23 => { rdi = (rdi & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 24; continue; }
            24 => { rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; bb = 27; continue; }
            25 => { rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; fl_l = rax; fl_r = rbp; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 27; continue; } bb = 26; continue; }
            26 => { rdi = (rdi & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 27; continue; }
            27 => { rcx = ((rax as u32)) as u32 as u64; rdx = ((rcx.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64; rdx = ((rdx).wrapping_shl(4u32)) as u64; rbx = ((rbx).wrapping_add(rdx)) as u64; fl_l = rbx; fl_r = r14; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 2; continue; } bb = 28; continue; }
            28 => { r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; bb = 29; continue; }
            29 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rax = (((rdi as u8)) as u32) as u32 as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14253b260(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0xf8; const P1: u64 = 0x108;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    r9 = ((rcx.wrapping_add(P1)) as u64) as u64;
    rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64;
    rax = ((rax).wrapping_shl(4u32)) as u64;
    rax = ((rax).wrapping_add(0x108u64)) as u64;
    rax = ((rax).wrapping_add(rcx)) as u64;
    crate::simd_ops::wu64(rdx, rax);
    rax = (crate::simd_ops::ru32(r9)) as u32 as u64;
    rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64;
    rax = ((rax).wrapping_shl(4u32)) as u64;
    fl_l = ((rax).wrapping_add(r9)) as u64; fl_r = fl_l;
    rax = ((fl_l as u64)) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14253b2a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14253c530; const P1: u64 = 0x143872d74; const P2: u64 = 0x143872d70; const P3: u64 = 0x1425375a0; const P4: u64 = 0x14253c770; const P5: u64 = 0x143a095c0; const P6: u64 = 0x143a09840; const P7: u64 = 0x14253d8f0; const P8: u64 = 0x143c7d180; const P9: u64 = 0x143c7d1c0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(0xf8u64))) as u64; rsi = rcx.wrapping_add(0x108); rbx = rcx.wrapping_add(0x108); r14 = 0; fl_l = 0; fl_r = 0; rbp = rdx; rdi = rcx; r8 = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; r8 = ((r8).wrapping_shl(4u32)) as u64; rsi = ((rsi).wrapping_add(r8)) as u64; fl_l = rbx; fl_r = rsi; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 4; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x1cu64))) as u32 as u64; fl_r = ((r14 as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rdx = rbp; rcx = rbx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu32(rbx.wrapping_add(0x1cu64), ((r14 as u32)) as u32); bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rbx = ((rbx).wrapping_add(rcx)) as u64; fl_l = rbx; fl_r = rsi; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 1; continue; } bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::ru32(P1)) as u32 as u64; fl_r = ((r14 as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rcx = P2; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            6 => { fl_l = (crate::simd_ops::ru32(P2)) as u32 as u64; fl_r = ((r14 as u32)) as u32 as u64; rcx = rdi; r8 = (crate::simd_ops::ru64(rbp.wrapping_add(0x380u64))) as u64; rdx = (rdx & !0xffu64) | (((if (((fl_l as u32)) != ((fl_r as u32))) { 1u8 } else { 0u8 }) as u8 as u64)); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x380u64))) as u64; rax = P5; rcx = ((rcx).wrapping_add(0xe0u64)) as u64; fl_l = rcx; fl_r = rax; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 12; continue; } bb = 7; continue; }
            7 => { rax = P6; fl_l = rcx; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 12; continue; } bb = 8; continue; }
            8 => { rdx = 0x1; let xa_src: u64 = rdx; let xa_old: u64 = crate::simd_ops::atomic_xadd_u64(rcx.wrapping_add(0x18u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rdx = (xa_old) as u64; fl_l = ((rdx).wrapping_add(1)) as u64; rdx = ((fl_l as u64)) as u64; bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; fl_l = rax; fl_r = rdx; if ((((fl_l as u64)) as i64) >= (((fl_r as u64)) as i64)) { bb = 11; continue; } bb = 10; continue; }
            10 => { fl_l = rax; let cx_src: u64 = rdx; let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u64(rcx.wrapping_add(0x10u64), fl_l as u64, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u64)) as u64; } if (((fl_l as u64)) != ((fl_r as u64))) { bb = 9; continue; } bb = 11; continue; }
            11 => { fl_l = (crate::simd_ops::lk_add_u64(rcx, (1) as u64)) as u64; bb = 15; continue; }
            12 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x18u64))) as u64; rax = ((rax).wrapping_add(1)) as u64; crate::simd_ops::wu64(rcx.wrapping_add(0x18u64), rax); fl_l = rax; fl_r = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; if ((((fl_l as u64)) as i64) <= (((fl_r as u64)) as i64)) { bb = 14; continue; } bb = 13; continue; }
            13 => { crate::simd_ops::wu64(rcx.wrapping_add(0x10u64), rax); bb = 14; continue; }
            14 => { fl_l = (crate::simd_ops::lk_add_u64(rcx, (1) as u64)) as u64; bb = 15; continue; }
            15 => { rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0xe0u64))) as u32 as u64; rdx = rbp; rcx = (0u32.wrapping_sub((rcx as u32))) as u32 as u64; rcx = (((rcx as u32)).wrapping_shl(16u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rdi.wrapping_add(0x100u64), r14); crate::simd_ops::wu64(rdi.wrapping_add(0xb0u64), r14); crate::simd_ops::wu64(rdi.wrapping_add(0xc8u64), (0x1u64) as u64); rax = (crate::simd_ops::ru64(P8)) as u64; bb = 16; continue; }
            16 => { rcx = rax; rcx = (((rcx) & (0xfffffffffe000000u64))) as u64; crate::simd_ops::wu64(rdi.wrapping_add(0xb0u64), rcx); rcx = rax.wrapping_add(0x1); rcx = ((((rcx as u32)) & (0x1ffffffu32))) as u32 as u64; rcx = (((rcx) | (rdi))) as u64; fl_l = rax; let cx_src: u64 = rcx; let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u64(P8, fl_l as u64, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u64)) as u64; } if (((fl_l as u64)) != ((fl_r as u64))) { bb = 16; continue; } bb = 17; continue; }
            17 => { crate::simd_ops::lk_add_u64(P9, (1) as u64); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14253b430(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143c7d180; const P1: u64 = 0x14253b550; const P2: u64 = 0x143c7d100; const P3: u64 = 0x1; const P4: u64 = 0xb0; const P5: u64 = 0x143c7d1c0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x28u64)) as u64; fl_l = (crate::simd_ops::ru64(P0)) as u64; fl_r = (0xfffffffffe000000u64) as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rax = 0; fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
            3 => { fl_l = (crate::simd_ops::lk_add_u64(P2, (1) as u64)) as u64; rax = (crate::simd_ops::ru64(P0)) as u64; bb = 4; continue; }
            4 => { rcx = rax; fl_l = (((rcx) & (0xfffffffffe000000u64))) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 5; continue; }
            5 => { rdx = ((rax.wrapping_add(P3)) as u64) as u64; rdx = ((((rdx as u32)) & (0x1ffffffu32))) as u32 as u64; rdx = (((rdx) | (crate::simd_ops::ru64(rcx.wrapping_add(P4))))) as u64; fl_l = rax; let cx_src: u64 = rdx; let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u64(P0, fl_l as u64, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u64)) as u64; } if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } bb = 6; continue; }
            6 => { crate::simd_ops::lk_sub_u64(P2, (1) as u64); rdx = P5; crate::simd_ops::wu64(rcx.wrapping_add(P4), (0x0u64) as u64); rax = (0xffffffffffffffffu64) as u64; crate::simd_ops::lk_add_u64(rdx, (rax) as u64); rax = rcx; rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
            7 => { rdx = P2; rax = (0xffffffffffffffffu64) as u64; crate::simd_ops::lk_add_u64(rdx, (rax) as u64); rax = rcx; rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14253b4e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x143c7d100;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru64(P0)) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 3; continue; } bb = 2; continue; }
            2 => { bb = 1; continue; }
            3 => { return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14253b500(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x143c7d180; const P1: u64 = 0xb0; const P2: u64 = 0x1; const P3: u64 = 0x143c7d1c0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru64(P0)) as u64;
    loop { rdx = rax; rdx = (((rdx) & (0xfffffffffe000000u64))) as u64; crate::simd_ops::wu64(rcx.wrapping_add(P1), rdx); rdx = ((rax.wrapping_add(P2)) as u64) as u64; rdx = ((((rdx as u32)) & (0x1ffffffu32))) as u32 as u64; rdx = (((rdx) | (rcx))) as u64; fl_l = rax; let cx_src: u64 = rdx; let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u64(P0, fl_l as u64, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u64)) as u64; } if !((((fl_l as u64)) != ((fl_r as u64)))) { break; } }
    fl_l = (crate::simd_ops::lk_add_u64(P3, (1) as u64)) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14253b550(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x143c7d0c0; const P1: u64 = 0x143c7d180; const P2: u64 = 0x143c7d140; const P3: u64 = 0x1; const P4: u64 = 0xb0; const P5: u64 = 0x143c7d1c0;
    let (mut rax, mut r10, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = (crate::simd_ops::ru64(P0)) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 11; continue; } bb = 1; continue; }
            1 => { rdx = 0; fl_l = 0; fl_r = 0; let tmp_xchg: u64 = crate::simd_ops::atomic_xchg_u64(P0, rdx); rdx = (tmp_xchg) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 2; continue; }
            2 => { r8 = (crate::simd_ops::ru64(P1)) as u64; fl_l = r8; fl_r = (0xfffffffffe000000u64) as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 3; continue; }
            3 => { r9 = (crate::simd_ops::ru64(P2)) as u64; rcx = ((r8.wrapping_add(P3)) as u64) as u64; rcx = ((((rcx as u32)) & (0x1ffffffu32))) as u32 as u64; rax = r8; rcx = (((rcx) | (rdx))) as u64; fl_l = rax; let cx_src: u64 = rcx; let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u64(P1, fl_l as u64, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u64)) as u64; } fl_l = rax; fl_r = r8; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 10; continue; } bb = 4; continue; }
            4 => { r8 = (crate::simd_ops::ru64(P1)) as u64; bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(P4))) as u64; r10 = rdx; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 6; continue; }
            6 => { bb = 7; continue; }
            7 => { r10 = rax; rax = (crate::simd_ops::ru64(rax.wrapping_add(P4))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 7; continue; } bb = 8; continue; }
            8 => { bb = 9; continue; }
            9 => { r9 = (crate::simd_ops::ru64(P2)) as u64; rcx = r8; rcx = (((rcx) & (0xfffffffffe000000u64))) as u64; rax = r8; crate::simd_ops::wu64(r10.wrapping_add(P4), rcx); rcx = ((r8.wrapping_add(P3)) as u64) as u64; rcx = ((((rcx as u32)) & (0x1ffffffu32))) as u32 as u64; rcx = (((rcx) | (rdx))) as u64; fl_l = rax; let cx_src: u64 = rcx; let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u64(P1, fl_l as u64, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u64)) as u64; } r8 = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 9; continue; } bb = 10; continue; }
            10 => { crate::simd_ops::lk_add_u64(P5, (r9) as u64); r9 = (0u64.wrapping_sub(r9)) as u64; fl_l = (crate::simd_ops::lk_add_u64(P2, (r9) as u64)) as u64; fl_r = fl_l; rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); return rax; }
            11 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14253b630(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x143c7d0c0; const P1: u64 = 0xb0; const P2: u64 = 0x143c7d140;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru64(P0)) as u64;
    loop { crate::simd_ops::wu64(rcx.wrapping_add(P1), rax); fl_l = rax; let cx_src: u64 = rcx; let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u64(P0, fl_l as u64, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u64)) as u64; } if !((((fl_l as u64)) != ((fl_r as u64)))) { break; } }
    fl_l = (crate::simd_ops::lk_add_u64(P2, (1) as u64)) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14253b670(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x1;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = ((rdx.wrapping_add(P0)) as u64) as u64;
    rax = ((((rax as u32)) & (0x1ffffffu32))) as u32 as u64;
    fl_l = (((rax) | (rcx))) as u64; fl_r = fl_l;
    rax = ((fl_l as u64)) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14253b690(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = (((rcx) & (0xfffffffffe000000u64))) as u64; fl_r = fl_l;
    rcx = ((fl_l as u64)) as u64;
    rax = rcx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14253b840(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14253c530; const P1: u64 = 0x1425408f0; const P2: u64 = 0x143c55090; const P3: u64 = 0x14253d8f0; const P4: u64 = 0x14253ddf0; const P5: u64 = 0x143c7d100; const P6: u64 = 0x142543fc0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x50u64)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(0xf8u64))) as u64; rsi = rcx.wrapping_add(0x108); rbx = rcx.wrapping_add(0x108); rbp = r8; rdi = rcx; rdx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rdx = ((rdx).wrapping_shl(4u32)) as u64; rsi = ((rsi).wrapping_add(rdx)) as u64; fl_l = rbx; fl_r = rsi; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 6; continue; } bb = 1; continue; }
            1 => { bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x1cu64))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 5; continue; } bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0xf0u64))) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rdx = rbp; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rbx = ((rbx).wrapping_add(rcx)) as u64; fl_l = rbx; fl_r = rsi; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 2; continue; } bb = 6; continue; }
            6 => { rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0xe8u64))) as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x380u64))) as u64; rdx = (0u64.wrapping_sub(rdx)) as u64; rdx = ((rdx).wrapping_shl(16u32)) as u64; rcx = ((rcx).wrapping_add(0xc0u64)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (0x280000000000u64) as u64; crate::simd_ops::wu64(rdi.wrapping_add(0x100u64), (0x0u64) as u64); fl_l = rdi; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 10; continue; } bb = 7; continue; }
            7 => { rax = rdi; rax = ((rax).wrapping_shr(25u32)) as u64; r8 = rax; rax = ((rax).wrapping_shr(6u32)) as u64; r8 = ((((r8 as u32)) & (0x3fu32))) as u32 as u64; fl_l = rax; fl_r = (0x5000u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 10; continue; } bb = 8; continue; }
            8 => { rcx = P2; rdx = ((rcx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(rax.wrapping_mul(8)))) as u64; bb = 9; continue; }
            9 => { rcx = rax; fl_l = (((rcx) >> (((r8) as u32 & 63))) & 1) as u64; rcx = ((rcx) & !((1u64) << (((r8) as u32 & 63)))) as u64; fl_l = rax; let cx_src: u64 = rcx; let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u64(rdx, fl_l as u64, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u64)) as u64; } if (((fl_l as u64)) != ((fl_r as u64))) { bb = 9; continue; } bb = 10; continue; }
            10 => { rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0xe0u64))) as u32 as u64; rdx = rbp; rcx = (0u32.wrapping_sub((rcx as u32))) as u32 as u64; rcx = (((rcx as u32)).wrapping_shl(16u32)) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0xe0u64))) as u64; rcx = rdi.wrapping_add(0x70); rdx = ((rdx).wrapping_shl(16u32)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = rax; bb = 11; continue; }
            11 => { fl_l = (crate::simd_ops::ru64(P5)) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 13; continue; } bb = 12; continue; }
            12 => { bb = 11; continue; }
            13 => { rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0xe0u64))) as u64; r9 = rsp.wrapping_add(0x30); xmm0 = (crate::simd_ops::ru128(rdi)) as u128; xmm1 = ((crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64) as u128; rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x380u64))) as u64; rcx = rdi; rdx = ((rdx).wrapping_shl(16u32)) as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), ((xmm1 as u64)) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rsp = ((rsp).wrapping_add(0x50u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14253b9c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143872f30; const P1: u64 = 0x143867148; const P2: u64 = 0x143872db4; const P3: u64 = 0x143872db0; const P4: u64 = 0x1425375a0; const P5: u64 = 0x143872c54; const P6: u64 = 0x143872c50; const P7: u64 = 0x14253bc40; const P8: u64 = 0x143a09580; const P9: u64 = 0x14183acc2; const P10: u64 = 0x1438430b0; const P11: u64 = 0x1425408f0; const P12: u64 = 0x14253c220; const P13: u64 = 0x14253c600;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 15>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x60u64)) as u64; rsi = rcx; r14 = r9; rcx = (crate::simd_ops::ru64(P0)) as u64; rbx = ((r8 as u32)) as u32 as u64; rbp = rdx; rax = rcx.wrapping_sub(0x1); rdi = rax.wrapping_add(0xa158); fl_l = rax; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { fl_l = ((!(rax)) & (rdi)) as u64; fl_r = fl_l; rdi = ((fl_l as u64)) as u64; bb = 3; continue; }
            2 => { rdx = 0; fl_l = 0; fl_r = 0; rax = rdi; let d = (rcx) as u128; let n = ((rdx as u128) << 64) | (rax as u128); if d != 0 { rax = (n / d) as u64; rdx = (n % d) as u64; } fl_l = rdi; fl_r = rdx; rdi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; bb = 3; continue; }
            3 => { crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rdi); rdi = ((rdi).wrapping_add(0xffffu64)) as u64; rdi = (((rdi) & (0xffffffffffff0000u64))) as u64; rax = rdi; rax = ((rax).wrapping_shr(16u32)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xa0u64), rax); fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = 0x2000000; bb = 6; continue; }
            5 => { rax = rsi.wrapping_add(0xffff); rax = ((rax).wrapping_add(rdi)) as u64; fl_l = (((rax) & (0xffffffffffff0000u64))) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; bb = 6; continue; }
            6 => { rax = ((rax).wrapping_shr(16u32)) as u64; fl_l = (crate::simd_ops::ru64(P1)) as u64; fl_r = (0x1u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x80u64), rax); if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 11; continue; } bb = 7; continue; }
            7 => { fl_l = (crate::simd_ops::ru32(P2)) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 9; continue; } bb = 8; continue; }
            8 => { rcx = P3; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 9; continue; }
            9 => { rax = (((crate::simd_ops::ru32(P3)) as i32 as i64) as u64) as u64; fl_l = (crate::simd_ops::ru64(r14.wrapping_add(0x360u64))) as u64; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 11; continue; } bb = 10; continue; }
            10 => { rdi = (rdi & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 14; continue; }
            11 => { rdi = (rdi & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; fl_l = (crate::simd_ops::ru32(P5)) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 13; continue; } bb = 12; continue; }
            12 => { rcx = P6; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 13; continue; }
            13 => { fl_l = (crate::simd_ops::ru32(P6)) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 16; continue; } bb = 14; continue; }
            14 => { fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 16; continue; } bb = 15; continue; }
            15 => { rcx = (rcx & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 17; continue; }
            16 => { rcx = (rcx & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 17; continue; }
            17 => { crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), r14); rax = rsp.wrapping_add(0xa0); crate::simd_ops::wu8(rsp.wrapping_add(0x38u64), ((rcx as u8)) as u8); r9 = ((rbx as u32)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); r8 = (((rdi as u8)) as u32) as u32 as u64; rax = rsp.wrapping_add(0x50); rdx = rbp; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); rcx = rsi; rax = rsp.wrapping_add(0x80); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rbx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 26; continue; } bb = 18; continue; }
            18 => { fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x12u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 22; continue; } bb = 19; continue; }
            19 => { r8 = rbp.wrapping_add(0x2); r8 = ((r8.wrapping_add(r8.wrapping_mul(4))) as u64) as u64; r8 = ((r8).wrapping_shl(4u32)) as u64; rcx = rax.wrapping_add(0xb8); fl_l = (crate::simd_ops::ru8(P8)) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 21; continue; } bb = 20; continue; }
            20 => { rdi = rcx; rax = 0; fl_l = 0; fl_r = 0; rcx = r8; while rcx != 0 { crate::simd_ops::wu8(rdi, rax as u8); rdi = rdi.wrapping_add(1); rcx = rcx.wrapping_sub(1); } bb = 22; continue; }
            21 => { rdx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); bb = 22; continue; }
            22 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xa0u64))) as u64; rcx = 0x200; crate::simd_ops::wu64(rbx.wrapping_add(0xe8u64), rax); crate::simd_ops::wu64(rbx.wrapping_add(0xe0u64), rbp); rax = ((crate::exe::teb_self() as u64)) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0xa0u64))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x100u64), rax); rdx = rdi; rax = rbx; rax = (((rax) ^ (crate::simd_ops::ru64(P10)))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0xd8u64), rax); fl_l = rbp; fl_r = rcx; rax = rbp; rax = (if (((fl_l as u64)) > ((fl_r as u64))) { rcx } else { rax }) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0xf8u64), rax); rax = 0; fl_l = 0; fl_r = 0; fl_l = rsi; fl_r = rsi; rax = (rax & !0xffu64) | (((if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { 1u8 } else { 0u8 }) as u8 as u64)); rdx = ((rdx).wrapping_shl(16u32)) as u64; crate::simd_ops::wu32(rbx.wrapping_add(0xf0u64), ((rax as u32)) as u32); rcx = (crate::simd_ops::ru64(r14.wrapping_add(0x380u64))) as u64; rcx = ((rcx).wrapping_add(0xc0u64)) as u64; rax = crate::simd_ops::call_stk(P11, rax, rcx, rdx, r8, r9, rsp); r8 = rdi; rdx = 0; fl_l = 0; fl_r = 0; rcx = rbx; rax = crate::simd_ops::call_stk(P12, rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 26; continue; } bb = 23; continue; }
            23 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0xf0u64))) as u32 as u64; fl_r = 0x0; rdx = rdi; crate::simd_ops::wu64(rbx.wrapping_add(0xd0u64), (0x0u64) as u64); rcx = rbx; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 25; continue; } bb = 24; continue; }
            24 => { r8 = (crate::simd_ops::ru64(rbx.wrapping_add(0xf8u64))) as u64; r9 = 0; fl_l = 0; fl_r = 0; r8 = ((r8).wrapping_sub(rdi)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r14); rax = crate::simd_ops::call_stk(P13, rax, rcx, rdx, r8, r9, rsp); rax = rbx; bb = 27; continue; }
            25 => { rbp = ((rbp).wrapping_sub(rdi)) as u64; r8 = rbp; rax = crate::simd_ops::call_stk(P12, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0xa8u64))) as u64; crate::simd_ops::wu64(rcx, rax); rax = rbx; bb = 27; continue; }
            26 => { rax = 0; fl_l = 0; fl_r = 0; bb = 27; continue; }
            27 => { r11 = rsp.wrapping_add(0x60); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; rbp = (crate::simd_ops::ru64(r11.wrapping_add(0x30u64))) as u64; rsp = r11; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14253c1b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xffffffffffffffff; const P1: u64 = 0x14253c600;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x30u64)) as u64;
    rbx = r8;
    rdi = rdx;
    r8 = (crate::simd_ops::ru32(rdx)) as u32 as u64;
    fl_l = r8; fl_r = rbx;
    if !((((fl_l as u64)) <= ((fl_r as u64)))) { rax = ((rdx.wrapping_add(P0)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r9); rax = (((rax) & (0xfffffffffe000000u64))) as u64; r10 = rdx; r10 = ((r10).wrapping_sub(rax)) as u64; r8 = ((r8).wrapping_sub(rbx)) as u64; r10 = ((r10).wrapping_sub(0x108u64)) as u64; rax = (0x6666666666666667u64) as u64; let prod = (rax as i64 as i128).wrapping_mul((r10) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; r9 = 0; fl_l = 0; fl_r = 0; rdx = ((((rdx) as i64).wrapping_shr(5u32) as u64)) as u64; rax = rdx; rax = ((rax).wrapping_shr(63u32)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; rdx = ((rdx).wrapping_add(rbx)) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu32(rdi, ((rbx as u32)) as u32); }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64;
    rsp = ((rsp).wrapping_add(0x30u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14253c530(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0xfffffffffffffffe; const P1: u64 = 0x40; const P2: u64 = 0x38; const P3: u64 = 0x8; const P4: u64 = 0x1c;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r10 = (crate::simd_ops::ru32(rcx)) as u32 as u64; r11 = rdx; r9 = rcx; r8 = ((r10 as u32)) as u32 as u64; fl_l = r10; fl_r = (0x1u64) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 4; continue; } bb = 1; continue; }
            1 => { fl_l = r8; fl_r = (0x1u64) as u64; r8 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 13; continue; } bb = 2; continue; }
            2 => { rdx = 0x40; bb = 3; continue; }
            3 => { rax = ((rdx.wrapping_add(P0)) as u64) as u64; r8 = ((r8).wrapping_shr((((rax) as u32) & 63))) as u64; r8 = ((((r8 as u32)) & (0x3u32))) as u32 as u64; rax = ((rdx.wrapping_mul(4)) as u64) as u64; r8 = (((r8) | (rax))) as u64; fl_l = r8; fl_r = (0x4u64) as u64; r8 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; bb = 4; continue; }
            4 => { rdx = (crate::simd_ops::ru64(r9.wrapping_add(P1))) as u64; rax = ((r8.wrapping_add(r8.wrapping_mul(2))) as u64) as u64; rcx = ((r11.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; r10 = ((r9.wrapping_add(P1)) as u64) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(r9.wrapping_add(P2))) as u64; crate::simd_ops::wu64(rdx.wrapping_add(P2), rax); bb = 6; continue; }
            6 => { rdx = ((r9.wrapping_add(P1)) as u64) as u64; fl_l = r9; fl_r = (crate::simd_ops::ru64(rcx)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 8; continue; } bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru64(r9.wrapping_add(P2))) as u64; rdx = r10; crate::simd_ops::wu64(rcx, rax); bb = 8; continue; }
            8 => { r10 = (crate::simd_ops::ru64(r9.wrapping_add(P2))) as u64; fl_l = r10; fl_r = r10; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru64(r9.wrapping_add(P1))) as u64; crate::simd_ops::wu64(r10.wrapping_add(P1), rax); bb = 10; continue; }
            10 => { fl_l = r9; fl_r = (crate::simd_ops::ru64(rcx.wrapping_add(P3))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 12; continue; } bb = 11; continue; }
            11 => { rax = (crate::simd_ops::ru64(rdx)) as u64; crate::simd_ops::wu64(rcx.wrapping_add(P3), rax); bb = 12; continue; }
            12 => { rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(r9.wrapping_add(P4), (0x1u32) as u32); crate::simd_ops::wu64(rdx, rax); crate::simd_ops::wu64(r9.wrapping_add(P2), rax); return rax; }
            13 => { let bit_in: u64 = r8; if bit_in != 0 { rcx = (((63u32.wrapping_sub(bit_in.leading_zeros())) as u64)) as u64; } fl_l = bit_in as u64; rdx = 0x3f; rax = ((rdx as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_sub((rcx as u32))) as u32 as u64; rcx = ((rax as u32)) as u32 as u64; rdx = ((rdx).wrapping_sub(rcx)) as u64; fl_l = rdx; fl_r = (0x2u64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 3; continue; } bb = 14; continue; }
            14 => { r8 = r10; bb = 4; continue; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14253cca0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143425cb0; const P1: u64 = 0x142537a30; const P2: u64 = 0x1425390a0; const P3: u64 = 0x14253ddf0; const P4: u64 = 0x143a09620; const P5: u64 = 0x1425408f0;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 31>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = rsp; crate::simd_ops::wu64(rax.wrapping_add(0x20u64), r9); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rbp = rax.wrapping_sub(0x5f); rsp = ((rsp).wrapping_sub(0xe8u64)) as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0x19u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; r11 = rdx; rsi = rcx; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 34; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rax.wrapping_add(0x10u64), rbx); rbx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rax.wrapping_sub(0x18u64), rdi); r10 = ((rbx as u32)) as u32 as u64; crate::simd_ops::wu64(rax.wrapping_sub(0x20u64), r12); rdi = ((rbx as u32)) as u32 as u64; crate::simd_ops::wu64(rax.wrapping_sub(0x28u64), r13); r12 = ((rbx as u32)) as u32 as u64; crate::simd_ops::wu64(rax.wrapping_sub(0x30u64), r14); r13 = ((rbx as u32)) as u32 as u64; crate::simd_ops::wu64(rax.wrapping_sub(0x38u64), r15); r14 = ((rbx as u32)) as u32 as u64; rax = r8.wrapping_sub(0x1); crate::simd_ops::wu64(rbp.wrapping_sub(0x39u64), rbx); crate::simd_ops::wu64(rbp.wrapping_sub(0x31u64), rbx); r15 = ((rbx as u32)) as u32 as u64; crate::simd_ops::wu64(rbp.wrapping_add(0xfu64), rbx); crate::simd_ops::wu64(rbp.wrapping_sub(0x29u64), rbx); crate::simd_ops::wu64(rbp.wrapping_add(0x17u64), rbx); crate::simd_ops::wu64(rbp.wrapping_sub(0x21u64), rbx); crate::simd_ops::wu64(rbp.wrapping_sub(0x49u64), rbx); crate::simd_ops::wu64(rbp.wrapping_sub(0x19u64), rbx); crate::simd_ops::wu64(rbp.wrapping_add(0x67u64), rbx); crate::simd_ops::wu64(rbp.wrapping_sub(0x11u64), rbx); crate::simd_ops::wu64(rbp.wrapping_sub(0x59u64), rbx); crate::simd_ops::wu64(rbp.wrapping_sub(0x9u64), rbx); crate::simd_ops::wu64(rbp.wrapping_sub(0x51u64), rbx); crate::simd_ops::wu64(rbp.wrapping_sub(0x1u64), rbx); fl_l = rax; fl_r = (0x1ffffffu64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 21; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(0xf0u64))) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 21; continue; } bb = 3; continue; }
            3 => { rdx = (crate::simd_ops::ru64(rcx.wrapping_add(0xe8u64))) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0xe0u64))) as u64; rcx = ((rcx).wrapping_shl(16u32)) as u64; rdx = ((rdx).wrapping_shl(16u32)) as u64; rax = ((rcx.wrapping_add(rsi)) as u64) as u64; fl_l = r11; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 21; continue; } bb = 4; continue; }
            4 => { rax = r11; rax = ((rax).wrapping_sub(rsi)) as u64; r9 = rax.wrapping_add(0xffff); r10 = ((rax.wrapping_add(r8)) as u64) as u64; r9 = (((r9) & (0xffffffffffff0000u64))) as u64; r10 = (((r10) & (0xffffffffffff0000u64))) as u64; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { fl_l = r9; fl_r = rdx; r9 = (if (((fl_l as u64)) < ((fl_r as u64))) { rdx } else { r9 }) as u64; bb = 6; continue; }
            6 => { fl_l = r10; fl_r = rcx; rcx = (if (((fl_l as u64)) <= ((fl_r as u64))) { r10 } else { rcx }) as u64; r10 = ((r9.wrapping_add(rsi)) as u64) as u64; r12 = rcx; crate::simd_ops::wu64(rbp.wrapping_add(0x7u64), r10); r12 = ((r12).wrapping_sub(r9)) as u64; fl_l = rcx; fl_r = r9; r12 = (if (((fl_l as u64)) <= ((fl_r as u64))) { rbx } else { r12 }) as u64; fl_l = r12; fl_r = r12; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 20; continue; } bb = 7; continue; }
            7 => { r14 = r9; rdi = r12; r14 = ((r14).wrapping_shr(16u32)) as u64; rdi = ((rdi).wrapping_shr(16u32)) as u64; rax = ((rdi.wrapping_add(r14)) as u64) as u64; fl_l = rax; fl_r = (0x200u64) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 9; continue; } bb = 8; continue; }
            8 => { crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), r12); rdx = r14; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), r8); r8 = rdi; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), r11); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rcx); rcx = P0; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); r10 = (crate::simd_ops::ru64(rbp.wrapping_add(0x7u64))) as u64; bb = 9; continue; }
            9 => { fl_l = rdi; fl_r = (0x200u64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 11; continue; } bb = 10; continue; }
            10 => { r15 = (0xffffffffffffffffu64) as u64; rcx = r15; crate::simd_ops::wu64(rbp.wrapping_sub(0x39u64), r15); crate::simd_ops::wu64(rbp.wrapping_sub(0x49u64), rcx); r13 = r15; crate::simd_ops::wu64(rbp.wrapping_sub(0x19u64), rcx); rdi = r15; crate::simd_ops::wu64(rbp.wrapping_add(0x67u64), rcx); r14 = r15; crate::simd_ops::wu64(rbp.wrapping_sub(0x11u64), rcx); crate::simd_ops::wu64(rbp.wrapping_sub(0x59u64), rcx); crate::simd_ops::wu64(rbp.wrapping_sub(0x9u64), rcx); crate::simd_ops::wu64(rbp.wrapping_sub(0x51u64), rcx); crate::simd_ops::wu64(rbp.wrapping_sub(0x1u64), rcx); crate::simd_ops::wu64(rbp.wrapping_sub(0x31u64), r15); crate::simd_ops::wu64(rbp.wrapping_sub(0x29u64), r15); crate::simd_ops::wu64(rbp.wrapping_sub(0x21u64), r15); bb = 20; continue; }
            11 => { fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 14; continue; } bb = 12; continue; }
            12 => { rdi = (crate::simd_ops::ru64(rbp.wrapping_add(0xfu64))) as u64; r14 = (crate::simd_ops::ru64(rbp.wrapping_add(0x17u64))) as u64; bb = 20; continue; }
            13 => { bb = 14; continue; }
            14 => { rax = r14; rdx = rbp.wrapping_sub(0x39); rax = ((rax).wrapping_shr(6u32)) as u64; r15 = (0xffffffffffffffffu64) as u64; fl_l = ((((r14 as u32)) & (0x3fu32))) as u32 as u64; fl_r = fl_l; r14 = ((fl_l as u32)) as u32 as u64; rdx = ((rdx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; r8 = ((r15.wrapping_add(0x2u64)) as u32) as u32 as u64; bb = 15; continue; }
            15 => { rax = 0x40; rcx = rdi; rax = ((rax).wrapping_sub(r14)) as u64; fl_l = rdi; fl_r = rax; rcx = (if (((fl_l as u64)) > ((fl_r as u64))) { rax } else { rcx }) as u64; fl_l = rcx; fl_r = (0x40u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 17; continue; } bb = 16; continue; }
            16 => { rax = r15; bb = 18; continue; }
            17 => { rax = ((r8).wrapping_shl((((rcx) as u32) & 63))) as u64; fl_l = ((rax).wrapping_sub(1)) as u64; rax = ((fl_l as u64)) as u64; rax = ((rax).wrapping_shl((((r14) as u32) & 63))) as u64; bb = 18; continue; }
            18 => { crate::simd_ops::wu64(rdx, rax); r14 = rbx; rdx = ((rdx).wrapping_add(0x8u64)) as u64; fl_l = rdi; fl_r = rcx; rdi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 15; continue; } bb = 19; continue; }
            19 => { rax = (crate::simd_ops::ru64(rbp.wrapping_sub(0x1u64))) as u64; r14 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x21u64))) as u64; rdi = (crate::simd_ops::ru64(rbp.wrapping_sub(0x29u64))) as u64; r13 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x31u64))) as u64; r15 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x39u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x51u64), rax); rax = (crate::simd_ops::ru64(rbp.wrapping_sub(0x9u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x59u64), rax); rax = (crate::simd_ops::ru64(rbp.wrapping_sub(0x11u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x67u64), rax); rax = (crate::simd_ops::ru64(rbp.wrapping_sub(0x19u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x49u64), rax); bb = 20; continue; }
            20 => { r9 = (crate::simd_ops::ru64(rbp.wrapping_add(0x7fu64))) as u64; bb = 21; continue; }
            21 => { rax = rbx; bb = 22; continue; }
            22 => { fl_l = (crate::simd_ops::ru64(rbp.wrapping_add(rax.wrapping_mul(8)).wrapping_sub(0x39u64))) as u64; fl_r = rbx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 25; continue; } bb = 23; continue; }
            23 => { rax = ((rax).wrapping_add(1)) as u64; fl_l = rax; fl_r = (0x8u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 22; continue; } bb = 24; continue; }
            24 => { bb = 33; continue; }
            25 => { fl_l = r12; fl_r = r12; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 33; continue; } bb = 26; continue; }
            26 => { bb = 27; continue; }
            27 => { rax = (crate::simd_ops::ru64(rbp.wrapping_add(rbx.wrapping_mul(8)).wrapping_sub(0x39u64))) as u64; fl_l = (crate::simd_ops::ru64(rsi.wrapping_add(rbx.wrapping_mul(8)).wrapping_add(0x70u64))) as u64; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 30; continue; } bb = 28; continue; }
            28 => { rbx = ((rbx).wrapping_add(1)) as u64; fl_l = rbx; fl_r = (0x8u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 27; continue; } bb = 29; continue; }
            29 => { bb = 32; continue; }
            30 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); rdx = r12; rcx = r10; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 32; continue; } bb = 31; continue; }
            31 => { rax = r15; rcx = rbp.wrapping_sub(0x39); rax = (((rax) & (crate::simd_ops::ru64(rsi.wrapping_add(0x70u64))))) as u64; rdx = 0x2000000; crate::simd_ops::wu64(rbp.wrapping_sub(0x39u64), rax); rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x78u64))) as u64; rax = (((rax) & (r13))) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x31u64), rax); rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x80u64))) as u64; rax = (((rax) & (rdi))) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x29u64), rax); rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x88u64))) as u64; rax = (((rax) & (r14))) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x21u64), rax); rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x90u64))) as u64; rax = (((rax) & (crate::simd_ops::ru64(rbp.wrapping_sub(0x49u64))))) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x19u64), rax); rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x98u64))) as u64; rax = (((rax) & (crate::simd_ops::ru64(rbp.wrapping_add(0x67u64))))) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x11u64), rax); rax = (crate::simd_ops::ru64(rsi.wrapping_add(0xa0u64))) as u64; rax = (((rax) & (crate::simd_ops::ru64(rbp.wrapping_sub(0x59u64))))) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x9u64), rax); rax = (crate::simd_ops::ru64(rsi.wrapping_add(0xa8u64))) as u64; rax = (((rax) & (crate::simd_ops::ru64(rbp.wrapping_sub(0x51u64))))) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x1u64), rax); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); r12 = ((r12).wrapping_sub(rax)) as u64; rcx = P4; rdx = r12; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rax = r15; rax = (!(rax)) as u64; crate::simd_ops::lk_and_u64(rsi.wrapping_add(0x70u64), (rax) as u64); rax = r13; rax = (!(rax)) as u64; crate::simd_ops::lk_and_u64(rsi.wrapping_add(0x78u64), (rax) as u64); rax = rdi; rax = (!(rax)) as u64; crate::simd_ops::lk_and_u64(rsi.wrapping_add(0x80u64), (rax) as u64); rax = r14; rax = (!(rax)) as u64; crate::simd_ops::lk_and_u64(rsi.wrapping_add(0x88u64), (rax) as u64); rax = (crate::simd_ops::ru64(rbp.wrapping_sub(0x49u64))) as u64; rax = (!(rax)) as u64; crate::simd_ops::lk_and_u64(rsi.wrapping_add(0x90u64), (rax) as u64); rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x67u64))) as u64; rax = (!(rax)) as u64; crate::simd_ops::lk_and_u64(rsi.wrapping_add(0x98u64), (rax) as u64); rax = (crate::simd_ops::ru64(rbp.wrapping_sub(0x59u64))) as u64; rax = (!(rax)) as u64; crate::simd_ops::lk_and_u64(rsi.wrapping_add(0xa0u64), (rax) as u64); rax = (crate::simd_ops::ru64(rbp.wrapping_sub(0x51u64))) as u64; rax = (!(rax)) as u64; fl_l = (crate::simd_ops::lk_and_u64(rsi.wrapping_add(0xa8u64), (rax) as u64)) as u64; fl_r = fl_l; bb = 32; continue; }
            32 => { rax = (crate::simd_ops::ru64(rbp.wrapping_sub(0x49u64))) as u64; r15 = (!(r15)) as u64; crate::simd_ops::lk_and_u64(rsi.wrapping_add(0x30u64), (r15) as u64); rax = (!(rax)) as u64; crate::simd_ops::lk_and_u64(rsi.wrapping_add(0x50u64), (rax) as u64); r13 = (!(r13)) as u64; rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x67u64))) as u64; rdi = (!(rdi)) as u64; crate::simd_ops::lk_and_u64(rsi.wrapping_add(0x38u64), (r13) as u64); rax = (!(rax)) as u64; crate::simd_ops::lk_and_u64(rsi.wrapping_add(0x58u64), (rax) as u64); r14 = (!(r14)) as u64; rax = (crate::simd_ops::ru64(rbp.wrapping_sub(0x59u64))) as u64; crate::simd_ops::lk_and_u64(rsi.wrapping_add(0x40u64), (rdi) as u64); rax = (!(rax)) as u64; crate::simd_ops::lk_and_u64(rsi.wrapping_add(0x60u64), (rax) as u64); rax = (crate::simd_ops::ru64(rbp.wrapping_sub(0x51u64))) as u64; crate::simd_ops::lk_and_u64(rsi.wrapping_add(0x48u64), (r14) as u64); rax = (!(rax)) as u64; fl_l = (crate::simd_ops::lk_and_u64(rsi.wrapping_add(0x68u64), (rax) as u64)) as u64; fl_r = fl_l; bb = 33; continue; }
            33 => { r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0xc8u64))) as u64; r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0xd0u64))) as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0xd8u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0xe0u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x108u64))) as u64; r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0xc0u64))) as u64; bb = 34; continue; }
            34 => { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); rsp = ((rsp).wrapping_add(0xe8u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14253d0c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x70; const P1: u64 = 0x30; const P2: u64 = 0x14253d110;
    let (mut rax, mut r10, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r9 = 0; fl_l = 0; fl_r = 0; rax = ((rcx.wrapping_add(P0)) as u64) as u64; r10 = ((r9 as u32)) as u32 as u64; bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru64(rax)) as u64; fl_r = (0xffffffffffffffffu64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 7; continue; } bb = 2; continue; }
            2 => { r10 = ((r10).wrapping_add(1)) as u64; rax = ((rax).wrapping_add(0x8u64)) as u64; fl_l = r10; fl_r = (0x8u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 1; continue; } bb = 3; continue; }
            3 => { rax = ((rcx.wrapping_add(P1)) as u64) as u64; bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::ru64(rax)) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 7; continue; } bb = 5; continue; }
            5 => { r9 = ((r9).wrapping_add(1)) as u64; rax = ((rax).wrapping_add(0x8u64)) as u64; fl_l = r9; fl_r = (0x8u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 4; continue; } bb = 6; continue; }
            6 => { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); return rax; }
            7 => { return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14253d680(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143425cb0; const P1: u64 = 0x142537a30; const P2: u64 = 0x14253dfa0;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; rdi = 0; fl_l = 0; fl_r = 0; r11 = r9; rbp = r8; r9 = (((rdx as u8)) as u32) as u32 as u64; crate::simd_ops::wu64(rbx, rdi); rax = r11.wrapping_sub(0x1); crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), rdi); crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), rdi); crate::simd_ops::wu64(rbx.wrapping_add(0x18u64), rdi); crate::simd_ops::wu64(rbx.wrapping_add(0x20u64), rdi); crate::simd_ops::wu64(rbx.wrapping_add(0x28u64), rdi); crate::simd_ops::wu64(rbx.wrapping_add(0x30u64), rdi); crate::simd_ops::wu64(rbx.wrapping_add(0x38u64), rdi); fl_l = rax; fl_r = (0x1ffffffu64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 12; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(0xf0u64))) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 12; continue; } bb = 2; continue; }
            2 => { rdx = (crate::simd_ops::ru64(rcx.wrapping_add(0xe0u64))) as u64; r8 = (crate::simd_ops::ru64(rcx.wrapping_add(0xe8u64))) as u64; rdx = ((rdx).wrapping_shl(16u32)) as u64; r8 = ((r8).wrapping_shl(16u32)) as u64; rax = ((rdx.wrapping_add(rcx)) as u64) as u64; fl_l = rbp; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 12; continue; } bb = 3; continue; }
            3 => { rax = rbp; rax = ((rax).wrapping_sub(rcx)) as u64; r10 = ((r11.wrapping_add(rax)) as u64) as u64; fl_l = ((r9 as u8)) as u8 as u64; fl_r = ((r9 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { r9 = rax.wrapping_add(0xffff); bb = 6; continue; }
            5 => { r9 = rax; fl_l = ((r10).wrapping_add(0xffffu64)) as u64; fl_r = fl_l; r10 = ((fl_l as u64)) as u64; bb = 6; continue; }
            6 => { r10 = (((r10) & (0xffffffffffff0000u64))) as u64; r9 = (((r9) & (0xffffffffffff0000u64))) as u64; fl_l = rax; fl_r = r8; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 8; continue; } bb = 7; continue; }
            7 => { fl_l = r9; fl_r = r8; r9 = (if (((fl_l as u64)) < ((fl_r as u64))) { r8 } else { r9 }) as u64; bb = 8; continue; }
            8 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; fl_l = r10; fl_r = rdx; rdx = (if (((fl_l as u64)) <= ((fl_r as u64))) { r10 } else { rdx }) as u64; rcx = ((rcx).wrapping_add(r9)) as u64; crate::simd_ops::wu64(rax, rcx); rcx = rdx; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rcx = ((rcx).wrapping_sub(r9)) as u64; fl_l = rdx; fl_r = r9; rcx = (if (((fl_l as u64)) <= ((fl_r as u64))) { rdi } else { rcx }) as u64; crate::simd_ops::wu64(rax, rcx); fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 9; continue; }
            9 => { crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rsi); rdi = r9; rdi = ((rdi).wrapping_shr(16u32)) as u64; rsi = rcx; rsi = ((rsi).wrapping_shr(16u32)) as u64; rax = ((rsi.wrapping_add(rdi)) as u64) as u64; fl_l = rax; fl_r = (0x200u64) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 11; continue; } bb = 10; continue; }
            10 => { crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rcx); r8 = rsi; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), r11); rcx = P0; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdx); rdx = rdi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 11; continue; }
            11 => { r8 = rbx; rdx = rsi; rcx = rdi; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; bb = 12; continue; }
            12 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14253d7f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x100; const P1: u64 = 0x143c55090; const P2: u64 = 0xe0; const P3: u64 = 0x14253d8f0; const P4: u64 = 0x70; const P5: u64 = 0x14253ddf0; const P6: u64 = 0x143c7d100; const P7: u64 = 0x10; const P8: u64 = 0x380; const P9: u64 = 0x142543fc0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x50u64)) as u64; rax = (0x280000000000u64) as u64; crate::simd_ops::wu64(rcx.wrapping_add(P0), (0x0u64) as u64); rdi = rdx; rbx = rcx; fl_l = rcx; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 4; continue; } bb = 1; continue; }
            1 => { rax = rcx; rax = ((rax).wrapping_shr(25u32)) as u64; r8 = rax; rax = ((rax).wrapping_shr(6u32)) as u64; r8 = ((((r8 as u32)) & (0x3fu32))) as u32 as u64; fl_l = rax; fl_r = (0x5000u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 4; continue; } bb = 2; continue; }
            2 => { rcx = P1; rdx = ((rcx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(rax.wrapping_mul(8)))) as u64; bb = 3; continue; }
            3 => { rcx = rax; fl_l = (((rcx) >> (((r8) as u32 & 63))) & 1) as u64; rcx = ((rcx) & !((1u64) << (((r8) as u32 & 63)))) as u64; fl_l = rax; let cx_src: u64 = rcx; let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u64(rdx, fl_l as u64, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u64)) as u64; } if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru32(rbx.wrapping_add(P2))) as u32 as u64; rdx = rdi; rcx = (0u32.wrapping_sub((rcx as u32))) as u32 as u64; rcx = (((rcx as u32)).wrapping_shl(16u32)) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64; rcx = ((rbx.wrapping_add(P4)) as u64) as u64; rdx = ((rdx).wrapping_shl(16u32)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = rax; bb = 5; continue; }
            5 => { fl_l = (crate::simd_ops::ru64(P6)) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 7; continue; } bb = 6; continue; }
            6 => { bb = 5; continue; }
            7 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64; r9 = rsp.wrapping_add(0x30); xmm0 = (crate::simd_ops::ru128(rbx)) as u128; xmm1 = ((crate::simd_ops::ru64(rbx.wrapping_add(P7))) as u64) as u128; rax = (crate::simd_ops::ru64(rdi.wrapping_add(P8))) as u64; rcx = rbx; rdx = ((rdx).wrapping_shl(16u32)) as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), ((xmm1 as u64)) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsp = ((rsp).wrapping_add(0x50u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14253d8f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x380; const P1: u64 = 0x143a095c0; const P2: u64 = 0x143a09840; const P3: u64 = 0x18; const P4: u64 = 0x10; const P5: u64 = 0x1425408f0; const P6: u64 = 0x360; const P7: u64 = 0x368; const P8: u64 = 0x370; const P9: u64 = 0x378;
    let (mut rax, mut r10, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x28u64)) as u64; r10 = ((((rcx as u32)) as i32 as i64) as u64) as u64; r9 = rdx; rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_l = ((r10 as u32)) as u32 as u64; fl_r = ((r10 as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 10; continue; } bb = 1; continue; }
            1 => { rax = P1; fl_l = rcx; fl_r = rax; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 7; continue; } bb = 2; continue; }
            2 => { rax = P2; fl_l = rcx; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 7; continue; } bb = 3; continue; }
            3 => { rdx = 0x1; let xa_src: u64 = rdx; let xa_old: u64 = crate::simd_ops::atomic_xadd_u64(rcx.wrapping_add(P3), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rdx = (xa_old) as u64; fl_l = ((rdx).wrapping_add(1)) as u64; rdx = ((fl_l as u64)) as u64; bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(P4))) as u64; fl_l = rax; fl_r = rdx; if ((((fl_l as u64)) as i64) >= (((fl_r as u64)) as i64)) { bb = 6; continue; } bb = 5; continue; }
            5 => { fl_l = rax; let cx_src: u64 = rdx; let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u64(rcx.wrapping_add(P4), fl_l as u64, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u64)) as u64; } if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } bb = 6; continue; }
            6 => { fl_l = (crate::simd_ops::lk_add_u64(rcx, (1) as u64)) as u64; bb = 11; continue; }
            7 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(P3))) as u64; rax = ((rax).wrapping_add(1)) as u64; crate::simd_ops::wu64(rcx.wrapping_add(P3), rax); fl_l = rax; fl_r = (crate::simd_ops::ru64(rcx.wrapping_add(P4))) as u64; if ((((fl_l as u64)) as i64) <= (((fl_r as u64)) as i64)) { bb = 9; continue; } bb = 8; continue; }
            8 => { crate::simd_ops::wu64(rcx.wrapping_add(P4), rax); bb = 9; continue; }
            9 => { fl_l = (crate::simd_ops::lk_add_u64(rcx, (1) as u64)) as u64; bb = 11; continue; }
            10 => { rdx = (0xffffffffffffffffu64) as u64; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); bb = 11; continue; }
            11 => { rdx = (crate::simd_ops::ru64(r9.wrapping_add(P6))) as u64; rcx = r10; rcx = ((((rcx) as i64).wrapping_shr(63u32) as u64)) as u64; rdx = ((rdx).wrapping_add(1)) as u64; rcx = (((rcx) & (0xfffffffffffffffeu64))) as u64; rdx = ((rdx).wrapping_add(rcx)) as u64; crate::simd_ops::wu64(r9.wrapping_add(P6), rdx); fl_l = rdx; fl_r = (crate::simd_ops::ru64(r9.wrapping_add(P7))) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 13; continue; } bb = 12; continue; }
            12 => { crate::simd_ops::wu64(r9.wrapping_add(P7), rdx); bb = 13; continue; }
            13 => { rcx = (crate::simd_ops::ru64(r9.wrapping_add(P8))) as u64; rcx = ((rcx).wrapping_add(r10)) as u64; crate::simd_ops::wu64(r9.wrapping_add(P8), rcx); fl_l = rcx; fl_r = (crate::simd_ops::ru64(r9.wrapping_add(P9))) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 15; continue; } bb = 14; continue; }
            14 => { crate::simd_ops::wu64(r9.wrapping_add(P9), rcx); bb = 15; continue; }
            15 => { rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14253d9d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x143872f30; const P1: u64 = 0xa158; const P2: u64 = 0xffff;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    r11 = (crate::simd_ops::ru64(P0)) as u64;
    r10 = rdx;
    rax = r11.wrapping_sub(0x1);
    r9 = ((rax.wrapping_add(P1)) as u64) as u64;
    fl_l = rax; fl_r = r11;
    if !(((((fl_l as u64)) & ((fl_r as u64))) != 0)) { fl_l = ((!(rax)) & (r9)) as u64; fl_r = fl_l; r9 = ((fl_l as u64)) as u64; } else { rdx = 0; fl_l = 0; fl_r = 0; rax = r9; let d = (r11) as u128; let n = ((rdx as u128) << 64) | (rax as u128); if d != 0 { rax = (n / d) as u64; rdx = (n % d) as u64; } fl_l = r9; fl_r = rdx; r9 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; }
    fl_l = r10; fl_r = r10;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { crate::simd_ops::wu64(r10, r9); }
    r9 = ((r9).wrapping_add(0xffffu64)) as u64;
    r9 = (((r9) & (0xffffffffffff0000u64))) as u64;
    fl_l = r8; fl_r = r8;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rax = r9; fl_l = ((rax).wrapping_shr(16u32)) as u64; rax = ((fl_l as u64)) as u64; crate::simd_ops::wu64(r8, rax); }
    fl_l = rcx; fl_r = rcx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) != 0)) { rax = 0x200; return rax; }
    rax = ((rcx.wrapping_add(P2)) as u64) as u64;
    rax = ((rax).wrapping_add(r9)) as u64;
    fl_l = ((rax).wrapping_shr(16u32)) as u64;
    rax = ((fl_l as u64)) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14253da50(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); r10 = rdx; r11 = rdx; r10 = ((r10).wrapping_sub(rcx)) as u64; rbx = rcx; r10 = ((r10).wrapping_sub(0x108u64)) as u64; rax = (0x6666666666666667u64) as u64; let prod = (rax as i64 as i128).wrapping_mul((r10) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rcx = (crate::simd_ops::ru32(r11)) as u32 as u64; rdx = ((((rdx) as i64).wrapping_shr(5u32) as u64)) as u64; rax = rdx; rcx = ((rcx).wrapping_shl(16u32)) as u64; rax = ((rax).wrapping_shr(63u32)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; rax = 0; fl_l = 0; fl_r = 0; fl_l = r8; fl_r = (0x8u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 4; continue; } bb = 1; continue; }
            1 => { fl_l = r8; fl_r = (0x40u64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rax = ((r8.wrapping_add(r8.wrapping_mul(2))) as u64) as u64; bb = 4; continue; }
            3 => { fl_l = r8; fl_r = (0x200u64) as u64; rax = (if (((fl_l as u64)) <= ((fl_r as u64))) { r8 } else { rax }) as u64; bb = 4; continue; }
            4 => { fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { fl_l = rcx; fl_r = rax; rcx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; crate::simd_ops::wu64(r9, rcx); bb = 6; continue; }
            6 => { rdx = ((rdx).wrapping_shl(16u32)) as u64; rax = ((rax).wrapping_add(rdx)) as u64; fl_l = ((rax).wrapping_add(rbx)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x8u64))) as u64; return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14253daf0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x1c;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; fl_r = 0x0;
    rax = (rax & !0xffu64) | (((if (((fl_l as u32)) > ((fl_r as u32))) { 1u8 } else { 0u8 }) as u8 as u64));
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14253db00(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x40; const P1: u64 = 0x38; const P2: u64 = 0x8; const P3: u64 = 0x1c;
    let (mut rax, mut r11, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    r9 = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    r11 = ((rdx.wrapping_add(P0)) as u64) as u64;
    fl_l = r9; fl_r = r9;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rax = (crate::simd_ops::ru64(rdx.wrapping_add(P1))) as u64; crate::simd_ops::wu64(r9.wrapping_add(P1), rax); }
    r9 = ((rdx.wrapping_add(P0)) as u64) as u64;
    fl_l = rdx; fl_r = (crate::simd_ops::ru64(rcx)) as u64;
    if !((((fl_l as u64)) != ((fl_r as u64)))) { rax = (crate::simd_ops::ru64(rdx.wrapping_add(P1))) as u64; r9 = r11; crate::simd_ops::wu64(rcx, rax); }
    r11 = (crate::simd_ops::ru64(rdx.wrapping_add(P1))) as u64;
    fl_l = r11; fl_r = r11;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rax = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; crate::simd_ops::wu64(r11.wrapping_add(0x40u64), rax); }
    fl_l = rdx; fl_r = (crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64;
    if !((((fl_l as u64)) != ((fl_r as u64)))) { rax = (crate::simd_ops::ru64(r9)) as u64; crate::simd_ops::wu64(rcx.wrapping_add(P2), rax); }
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu32(rdx.wrapping_add(P3), (0x1u32) as u32);
    crate::simd_ops::wu64(r9, rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P1), rax);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14253db60(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0xffffffffffffffff; const P1: u64 = 0x40; const P2: u64 = 0xfffffffffffffffe;
    let (mut rax, mut r10, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r10 = rdx; r8 = rcx; fl_l = rcx; fl_r = (0x1u64) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 5; continue; } bb = 1; continue; }
            1 => { r9 = ((rcx.wrapping_add(P0)) as u64) as u64; fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rdx = ((r9.wrapping_add(P1)) as u32) as u32 as u64; bb = 4; continue; }
            3 => { let bit_in: u64 = r9; if bit_in != 0 { rcx = (((63u32.wrapping_sub(bit_in.leading_zeros())) as u64)) as u64; } fl_l = bit_in as u64; rdx = 0x3f; rax = ((rdx as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_sub((rcx as u32))) as u32 as u64; rcx = ((rax as u32)) as u32 as u64; rdx = ((rdx).wrapping_sub(rcx)) as u64; fl_l = rdx; fl_r = (0x2u64) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = ((rdx.wrapping_add(P2)) as u64) as u64; r8 = ((r9).wrapping_shr((((rax) as u32) & 63))) as u64; r8 = ((((r8 as u32)) & (0x3u32))) as u32 as u64; rax = ((rdx.wrapping_mul(4)) as u64) as u64; r8 = (((r8) | (rax))) as u64; fl_l = r8; fl_r = (0x4u64) as u64; r8 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; bb = 5; continue; }
            5 => { rax = ((r8.wrapping_add(r8.wrapping_mul(2))) as u64) as u64; rax = ((r10.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14253dbc0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x40; const P1: u64 = 0x38; const P2: u64 = 0x1c; const P3: u64 = 0x8;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    r8 = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rdx.wrapping_add(P0), r8);
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    crate::simd_ops::wu64(rdx.wrapping_add(P1), rax);
    crate::simd_ops::wu64(rcx, rdx);
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P1))) as u64;
    fl_l = rax; fl_r = rax;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { crate::simd_ops::wu64(rax.wrapping_add(P0), rdx); crate::simd_ops::wu32(rdx.wrapping_add(P2), ((r8 as u32)) as u32); return rax; }
    crate::simd_ops::wu64(rcx.wrapping_add(P3), rdx);
    crate::simd_ops::wu32(rdx.wrapping_add(P2), ((r8 as u32)) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14253dc00(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0xffffffffffffffff;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = ((rcx.wrapping_add(P0)) as u64) as u64;
    rax = (((rax) & (0xfffffffffe000000u64))) as u64;
    rcx = ((rcx).wrapping_sub(rax)) as u64;
    rax = (0x6666666666666667u64) as u64;
    rcx = ((rcx).wrapping_sub(0x108u64)) as u64;
    let prod = (rax as i64 as i128).wrapping_mul((rcx) as i64 as i128) as u128;
    rax = prod as u64; rdx = (prod >> 64) as u64;
    rdx = ((((rdx) as i64).wrapping_shr(5u32) as u64)) as u64;
    rax = rdx;
    rax = ((rax).wrapping_shr(63u32)) as u64;
    fl_l = ((rax).wrapping_add(rdx)) as u64; fl_r = fl_l;
    rax = ((fl_l as u64)) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14253dc40(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0xffffffffffffffff; const P1: u64 = 0x40; const P2: u64 = 0xfffffffffffffffe;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = rcx; fl_l = rcx; fl_r = (0x1u64) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 5; continue; } bb = 1; continue; }
            1 => { r9 = ((rcx.wrapping_add(P0)) as u64) as u64; fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { r8 = ((r9.wrapping_add(P1)) as u32) as u32 as u64; bb = 4; continue; }
            3 => { let bit_in: u64 = r9; if bit_in != 0 { rdx = (((63u32.wrapping_sub(bit_in.leading_zeros())) as u64)) as u64; } fl_l = bit_in as u64; r8 = 0x3f; rcx = ((r8 as u32)) as u32 as u64; rcx = (((rcx as u32)).wrapping_sub((rdx as u32))) as u32 as u64; rdx = ((rcx as u32)) as u32 as u64; r8 = ((r8).wrapping_sub(rdx)) as u64; fl_l = r8; fl_r = (0x2u64) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = ((r8.wrapping_add(P2)) as u64) as u64; rax = ((r9).wrapping_shr((((rax) as u32) & 63))) as u64; rax = ((((rax as u32)) & (0x3u32))) as u32 as u64; rcx = ((r8.wrapping_mul(4)) as u64) as u64; rax = (((rax) | (rcx))) as u64; fl_l = rax; fl_r = (0x4u64) as u64; rax = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; bb = 5; continue; }
            5 => { return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14253dca0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x40; const P1: u64 = 0xfffffffffffffffe;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r8 = rcx; rax = rcx; fl_l = rcx; fl_r = (0x1u64) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 5; continue; } bb = 1; continue; }
            1 => { fl_l = r8; fl_r = (0x1u64) as u64; r8 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 2; continue; }
            2 => { r9 = ((r8.wrapping_add(P0)) as u32) as u32 as u64; bb = 4; continue; }
            3 => { let bit_in: u64 = r8; if bit_in != 0 { rdx = (((63u32.wrapping_sub(bit_in.leading_zeros())) as u64)) as u64; } fl_l = bit_in as u64; r9 = 0x3f; rcx = ((r9 as u32)) as u32 as u64; rcx = (((rcx as u32)).wrapping_sub((rdx as u32))) as u32 as u64; rdx = ((rcx as u32)) as u32 as u64; r9 = ((r9).wrapping_sub(rdx)) as u64; fl_l = r9; fl_r = (0x2u64) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = ((r9.wrapping_add(P1)) as u64) as u64; rax = ((r8).wrapping_shr((((rax) as u32) & 63))) as u64; rax = ((((rax as u32)) & (0x3u32))) as u32 as u64; rcx = ((r9.wrapping_mul(4)) as u64) as u64; rax = (((rax) | (rcx))) as u64; fl_l = rax; fl_r = (0x4u64) as u64; rax = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; bb = 5; continue; }
            5 => { return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14253dd00(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0xffffffffffffffff;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    r8 = ((rcx.wrapping_add(P0)) as u64) as u64;
    rax = (0x6666666666666667u64) as u64;
    r8 = (((r8) & (0xfffffffffe000000u64))) as u64;
    rcx = ((rcx).wrapping_sub(r8)) as u64;
    rcx = ((rcx).wrapping_sub(0x108u64)) as u64;
    let prod = (rax as i64 as i128).wrapping_mul((rcx) as i64 as i128) as u128;
    rax = prod as u64; rdx = (prod >> 64) as u64;
    rdx = ((((rdx) as i64).wrapping_shr(5u32) as u64)) as u64;
    rax = rdx;
    rax = ((rax).wrapping_shr(63u32)) as u64;
    rax = ((rax).wrapping_add(rdx)) as u64;
    rax = ((rax).wrapping_shl(16u32)) as u64;
    fl_l = ((rax).wrapping_add(r8)) as u64; fl_r = fl_l;
    rax = ((fl_l as u64)) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14253dd40(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0xf8;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64;
    rax = ((rax).wrapping_shl(4u32)) as u64;
    rax = ((rax).wrapping_add(0x108u64)) as u64;
    fl_l = ((rax).wrapping_add(rcx)) as u64; fl_r = fl_l;
    rax = ((fl_l as u64)) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14253dd60(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r10 = (crate::simd_ops::ru64(rdx)) as u64; r11 = rcx; r9 = r10; r10 = ((((r10 as u32)) & (0x3fu32))) as u32 as u64; r9 = ((r9).wrapping_shr(6u32)) as u64; fl_l = r9; fl_r = (0x8u64) as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 4; continue; } bb = 1; continue; }
            1 => { r8 = 0; fl_l = 0; fl_r = 0; bb = 2; continue; }
            2 => { rax = ((crate::simd_ops::ru64(rcx.wrapping_add(r9.wrapping_mul(8)))).wrapping_shr((((r10) as u32) & 63))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 3; continue; }
            3 => { r9 = ((r9).wrapping_add(1)) as u64; r10 = r8; fl_l = r9; fl_r = (0x8u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 2; continue; } bb = 4; continue; }
            4 => { crate::simd_ops::wu64(rdx, (0x200u64) as u64); rax = 0; fl_l = 0; fl_r = 0; return rax; }
            5 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { rax = ((rax).wrapping_shr(1u32)) as u64; r10 = ((r10).wrapping_add(1)) as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 6; continue; } bb = 7; continue; }
            7 => { fl_l = r9; fl_r = (0x8u64) as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 4; continue; } bb = 8; continue; }
            8 => { rcx = r9; rcx = ((rcx).wrapping_shl(6u32)) as u64; fl_l = ((r10).wrapping_add(rcx)) as u64; fl_r = fl_l; r10 = ((fl_l as u64)) as u64; crate::simd_ops::wu64(rdx, r10); bb = 9; continue; }
            9 => { r8 = ((r8).wrapping_add(1)) as u64; rax = ((rax).wrapping_shr(1u32)) as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 9; continue; } bb = 10; continue; }
            10 => { rcx = ((r8.wrapping_add(r10)) as u32) as u32 as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = (0x3fu8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 13; continue; } bb = 11; continue; }
            11 => { r9 = ((r9).wrapping_add(1)) as u64; fl_l = r9; fl_r = (0x8u64) as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 14; continue; } bb = 12; continue; }
            12 => { rax = (crate::simd_ops::ru64(r11.wrapping_add(r9.wrapping_mul(8)))) as u64; bb = 13; continue; }
            13 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 9; continue; } bb = 14; continue; }
            14 => { rax = r8; return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14253ddf0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r9 = (crate::simd_ops::ru64(rcx)) as u64; rax = 0; fl_l = 0; fl_r = 0; r8 = r9; r11 = rdx; r8 = (!(r8)) as u64; r10 = rcx; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = ((r8.wrapping_add(0x40u64)) as u32) as u32 as u64; bb = 4; continue; }
            2 => { fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { fl_l = ((r9 as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; rcx = rax.wrapping_add(0x1); rcx = (if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { rax } else { rcx }) as u64; fl_l = ((r9).wrapping_shr(1u32)) as u64; r9 = ((fl_l as u64)) as u64; rax = rcx; if (((fl_l as u64)) != 0) { bb = 3; continue; } bb = 4; continue; }
            4 => { rdx = (crate::simd_ops::ru64(r10.wrapping_add(0x8u64))) as u64; rcx = rdx; rcx = (!(rcx)) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { fl_l = ((rax).wrapping_add(0x40u64)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; bb = 9; continue; }
            6 => { fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 7; continue; }
            7 => { bb = 8; continue; }
            8 => { fl_l = ((rdx as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; rcx = rax.wrapping_add(0x1); rcx = (if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { rax } else { rcx }) as u64; fl_l = ((rdx).wrapping_shr(1u32)) as u64; rdx = ((fl_l as u64)) as u64; rax = rcx; if (((fl_l as u64)) != 0) { bb = 8; continue; } bb = 9; continue; }
            9 => { rdx = (crate::simd_ops::ru64(r10.wrapping_add(0x10u64))) as u64; rcx = rdx; rcx = (!(rcx)) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { fl_l = ((rax).wrapping_add(0x40u64)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; bb = 14; continue; }
            11 => { fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 12; continue; }
            12 => { bb = 13; continue; }
            13 => { fl_l = ((rdx as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; rcx = rax.wrapping_add(0x1); rcx = (if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { rax } else { rcx }) as u64; fl_l = ((rdx).wrapping_shr(1u32)) as u64; rdx = ((fl_l as u64)) as u64; rax = rcx; if (((fl_l as u64)) != 0) { bb = 13; continue; } bb = 14; continue; }
            14 => { rdx = (crate::simd_ops::ru64(r10.wrapping_add(0x18u64))) as u64; rcx = rdx; rcx = (!(rcx)) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 16; continue; } bb = 15; continue; }
            15 => { fl_l = ((rax).wrapping_add(0x40u64)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; bb = 19; continue; }
            16 => { fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 19; continue; } bb = 17; continue; }
            17 => { bb = 18; continue; }
            18 => { fl_l = ((rdx as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; rcx = rax.wrapping_add(0x1); rcx = (if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { rax } else { rcx }) as u64; fl_l = ((rdx).wrapping_shr(1u32)) as u64; rdx = ((fl_l as u64)) as u64; rax = rcx; if (((fl_l as u64)) != 0) { bb = 18; continue; } bb = 19; continue; }
            19 => { rdx = (crate::simd_ops::ru64(r10.wrapping_add(0x20u64))) as u64; rcx = rdx; rcx = (!(rcx)) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 21; continue; } bb = 20; continue; }
            20 => { fl_l = ((rax).wrapping_add(0x40u64)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; bb = 24; continue; }
            21 => { fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 24; continue; } bb = 22; continue; }
            22 => { bb = 23; continue; }
            23 => { fl_l = ((rdx as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; rcx = rax.wrapping_add(0x1); rcx = (if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { rax } else { rcx }) as u64; fl_l = ((rdx).wrapping_shr(1u32)) as u64; rdx = ((fl_l as u64)) as u64; rax = rcx; if (((fl_l as u64)) != 0) { bb = 23; continue; } bb = 24; continue; }
            24 => { rdx = (crate::simd_ops::ru64(r10.wrapping_add(0x28u64))) as u64; rcx = rdx; rcx = (!(rcx)) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 26; continue; } bb = 25; continue; }
            25 => { fl_l = ((rax).wrapping_add(0x40u64)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; bb = 29; continue; }
            26 => { fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 29; continue; } bb = 27; continue; }
            27 => { bb = 28; continue; }
            28 => { fl_l = ((rdx as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; rcx = rax.wrapping_add(0x1); rcx = (if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { rax } else { rcx }) as u64; fl_l = ((rdx).wrapping_shr(1u32)) as u64; rdx = ((fl_l as u64)) as u64; rax = rcx; if (((fl_l as u64)) != 0) { bb = 28; continue; } bb = 29; continue; }
            29 => { rdx = (crate::simd_ops::ru64(r10.wrapping_add(0x30u64))) as u64; rcx = rdx; rcx = (!(rcx)) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 31; continue; } bb = 30; continue; }
            30 => { fl_l = ((rax).wrapping_add(0x40u64)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; bb = 34; continue; }
            31 => { fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 34; continue; } bb = 32; continue; }
            32 => { bb = 33; continue; }
            33 => { fl_l = ((rdx as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; rcx = rax.wrapping_add(0x1); rcx = (if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { rax } else { rcx }) as u64; fl_l = ((rdx).wrapping_shr(1u32)) as u64; rdx = ((fl_l as u64)) as u64; rax = rcx; if (((fl_l as u64)) != 0) { bb = 33; continue; } bb = 34; continue; }
            34 => { rdx = (crate::simd_ops::ru64(r10.wrapping_add(0x38u64))) as u64; rcx = rdx; rcx = (!(rcx)) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 36; continue; } bb = 35; continue; }
            35 => { rax = ((rax).wrapping_add(0x40u64)) as u64; r11 = ((r11).wrapping_shr(9u32)) as u64; rax = (((rax) as i64).wrapping_mul((r11) as i64) as u64) as u64; return rax; }
            36 => { fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 39; continue; } bb = 37; continue; }
            37 => { bb = 38; continue; }
            38 => { fl_l = ((rdx as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; rcx = rax.wrapping_add(0x1); rcx = (if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { rax } else { rcx }) as u64; fl_l = ((rdx).wrapping_shr(1u32)) as u64; rdx = ((fl_l as u64)) as u64; rax = rcx; if (((fl_l as u64)) != 0) { bb = 38; continue; } bb = 39; continue; }
            39 => { r11 = ((r11).wrapping_shr(9u32)) as u64; rax = (((rax) as i64).wrapping_mul((r11) as i64) as u64) as u64; return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14253dfa0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x18; const P3: u64 = 0x20; const P4: u64 = 0x28; const P5: u64 = 0x30; const P6: u64 = 0x38; const P7: u64 = 0x1;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r9 = rcx; fl_l = rdx; fl_r = (0x200u64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(r8, (0xffffffffffffffffu64) as u64); crate::simd_ops::wu64(r8.wrapping_add(P0), (0xffffffffffffffffu64) as u64); crate::simd_ops::wu64(r8.wrapping_add(P1), (0xffffffffffffffffu64) as u64); crate::simd_ops::wu64(r8.wrapping_add(P2), (0xffffffffffffffffu64) as u64); crate::simd_ops::wu64(r8.wrapping_add(P3), (0xffffffffffffffffu64) as u64); crate::simd_ops::wu64(r8.wrapping_add(P4), (0xffffffffffffffffu64) as u64); crate::simd_ops::wu64(r8.wrapping_add(P5), (0xffffffffffffffffu64) as u64); crate::simd_ops::wu64(r8.wrapping_add(P6), (0xffffffffffffffffu64) as u64); return rax; }
            2 => { rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(r8, rcx); crate::simd_ops::wu64(r8.wrapping_add(P0), rcx); crate::simd_ops::wu64(r8.wrapping_add(P1), rcx); crate::simd_ops::wu64(r8.wrapping_add(P2), rcx); crate::simd_ops::wu64(r8.wrapping_add(P3), rcx); crate::simd_ops::wu64(r8.wrapping_add(P4), rcx); crate::simd_ops::wu64(r8.wrapping_add(P5), rcx); crate::simd_ops::wu64(r8.wrapping_add(P6), rcx); fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 3; continue; }
            3 => { rax = r9; r11 = ((rcx.wrapping_add(P7)) as u32) as u32 as u64; rax = ((rax).wrapping_shr(6u32)) as u64; fl_l = ((((r9 as u32)) & (0x3fu32))) as u32 as u64; fl_r = fl_l; r9 = ((fl_l as u32)) as u32 as u64; r10 = ((r8.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; bb = 4; continue; }
            4 => { rax = 0x40; r8 = rdx; rax = ((rax).wrapping_sub(r9)) as u64; fl_l = rdx; fl_r = rax; r8 = (if (((fl_l as u64)) > ((fl_r as u64))) { rax } else { r8 }) as u64; fl_l = r8; fl_r = (0x40u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = (0xffffffffffffffffu64) as u64; bb = 7; continue; }
            6 => { rax = ((r11).wrapping_shl((((r8) as u32) & 63))) as u64; fl_l = ((rax).wrapping_sub(1)) as u64; rax = ((fl_l as u64)) as u64; rax = ((rax).wrapping_shl((((r9) as u32) & 63))) as u64; bb = 7; continue; }
            7 => { crate::simd_ops::wu64(r10, rax); r9 = rcx; r10 = ((r10).wrapping_add(0x8u64)) as u64; fl_l = rdx; fl_r = r8; rdx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } bb = 8; continue; }
            8 => { return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14253e080(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x20;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut ymm1_hi: u128 = 0;
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    fl_l = rdx; fl_r = rcx;
    rdx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64;
    rax = 0x2;
    loop { let y_addr: u64 = rcx; xmm1 = crate::simd_ops::ru128(y_addr); ymm1_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let (y_lo, y_hi): (u128, u128) = ((xmm1) | (crate::simd_ops::ru128(rdx.wrapping_add(rcx))), (ymm1_hi) | (crate::simd_ops::ru128((rdx.wrapping_add(rcx)).wrapping_add(16)))); xmm1 = y_lo; ymm1_hi = y_hi; let y_addr: u64 = rcx; crate::simd_ops::wu128(y_addr, xmm1); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm1_hi); rcx = ((rcx.wrapping_add(P0)) as u64) as u64; fl_l = rax; fl_r = (0x1u64) as u64; rax = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if !((((fl_l as u64)) != ((fl_r as u64)))) { break; } }
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14253e0c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x20;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut ymm0_hi: u128 = 0; let mut ymm2_hi: u128 = 0;
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    fl_l = rdx; fl_r = rcx;
    rdx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64;
    rax = 0x2;
    loop { let y_addr: u64 = rdx.wrapping_add(rcx); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let (y_lo, y_hi): (u128, u128) = ((!(xmm0)) & (crate::simd_ops::ru128(rcx)), (!(ymm0_hi)) & (crate::simd_ops::ru128((rcx).wrapping_add(16)))); xmm2 = y_lo; ymm2_hi = y_hi; let y_addr: u64 = rcx; crate::simd_ops::wu128(y_addr, xmm2); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm2_hi); rcx = ((rcx.wrapping_add(P0)) as u64) as u64; fl_l = rax; fl_r = (0x1u64) as u64; rax = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if !((((fl_l as u64)) != ((fl_r as u64)))) { break; } }
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14253e100(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x20;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut ymm0_hi: u128 = 0; let mut ymm1_hi: u128 = 0;
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rdx = ((rdx).wrapping_sub(rcx)) as u64;
    rax = 0x2;
    fl_l = r8; fl_r = rcx;
    r8 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64;
    loop { let y_addr: u64 = rdx.wrapping_add(rcx); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let (y_lo, y_hi): (u128, u128) = ((xmm0) & (crate::simd_ops::ru128(rcx)), (ymm0_hi) & (crate::simd_ops::ru128((rcx).wrapping_add(16)))); xmm1 = y_lo; ymm1_hi = y_hi; let y_addr: u64 = r8.wrapping_add(rcx); crate::simd_ops::wu128(y_addr, xmm1); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm1_hi); rcx = ((rcx.wrapping_add(P0)) as u64) as u64; fl_l = rax; fl_r = (0x1u64) as u64; rax = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if !((((fl_l as u64)) != ((fl_r as u64)))) { break; } }
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}
