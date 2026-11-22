#![allow(unused_imports, unused_variables, unused_mut, unused_assignments, non_snake_case, dead_code, clippy::all, unreachable_code, unused_parens)]

#[inline(never)]
pub unsafe extern "win64" fn fn_14152b0f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x14392e250; const P1: u64 = 0x14392e258; const P2: u64 = 0x14392e260; const P3: u64 = 0x14392e268; const P4: u64 = 0x14392e270;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = (crate::simd_ops::ru64(P0)) as u64; fl_r = (0x0u64) as u64;
    if (((fl_l as u64)) == ((fl_r as u64))) { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; return rax; }
    fl_l = (crate::simd_ops::ru64(P1)) as u64; fl_r = (0x0u64) as u64;
    if (((fl_l as u64)) == ((fl_r as u64))) { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; return rax; }
    fl_l = (crate::simd_ops::ru64(P2)) as u64; fl_r = (0x0u64) as u64;
    if (((fl_l as u64)) == ((fl_r as u64))) { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; return rax; }
    fl_l = (crate::simd_ops::ru64(P3)) as u64; fl_r = (0x0u64) as u64;
    if (((fl_l as u64)) == ((fl_r as u64))) { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; return rax; }
    fl_l = (crate::simd_ops::ru64(P4)) as u64; fl_r = (0x0u64) as u64;
    if !((((fl_l as u64)) == ((fl_r as u64)))) { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); return rax; }
    rax = (rax & !0xffu64) | (((0u8) as u8 as u64));
    fl_l = 0; fl_r = 0;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14152b130(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14392e270; const P1: u64 = 0x14392e250; const P2: u64 = 0x14392e258; const P3: u64 = 0x14392e260; const P4: u64 = 0x14392e268;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64;
    crate::simd_ops::wu64(P0, rax);
    crate::simd_ops::wu64(P1, rcx);
    crate::simd_ops::wu64(P2, rdx);
    crate::simd_ops::wu64(P3, r8);
    crate::simd_ops::wu64(P4, r9);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14152b160(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1426767d8; const P1: u64 = 0x10; const P2: u64 = 0x142676798; const P3: u64 = 0x141834d30;
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
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(P1), (0x0u64) as u64); }
    rax = P2;
    crate::simd_ops::wu64(rbx, rax);
    fl_l = ((rdi as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64;
    if !(((((fl_l as u8)) & ((fl_r as u8))) == 0)) { rdx = 0x50; rcx = rbx; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); }
    rax = rbx;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14152b1f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14183acc2;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm4: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = ((rsp).wrapping_sub(0x48u64)) as u64; rax = ((crate::simd_ops::ru16(rdx.wrapping_add(0x12u64))) as u32) as u32 as u64; r12 = rdx; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r8); rdi = rcx; fl_l = ((rax as u16)) as u16 as u64; fl_r = ((rax as u16)) as u16 as u64; if ((((fl_l as u16)) & ((fl_r as u16))) == 0) { bb = 38; continue; } bb = 1; continue; }
            1 => { r11 = ((crate::simd_ops::ru16(rdx.wrapping_add(0x10u64))) as u32) as u32 as u64; r8 = (crate::simd_ops::ru64(rdx)) as u64; rdx = ((rax as u32)) as u32 as u64; r9 = (crate::simd_ops::ru64(rdi.wrapping_add(0x30u64))) as u64; xmm3 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x24u64))) as u32) as u128; xmm2 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x20u64))) as u32) as u128; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rbx); rcx = ((r8.wrapping_add(r11.wrapping_mul(4))) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), rbp); r10 = ((r9.wrapping_add(r11.wrapping_mul(4))) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rsi); rbx = 0; fl_l = 0; fl_r = 0; fl_l = ((xmm3 as u32)) as u32 as u64; fl_r = ((xmm2 as u32)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), r14); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), r15); if (f32::from_bits((fl_l as u32)) != f32::from_bits((fl_r as u32)) && !f32::from_bits((fl_l as u32)).is_nan() && !f32::from_bits((fl_r as u32)).is_nan()) { bb = 24; continue; } bb = 2; continue; }
            2 => { xmm4 = (xmm2) as u128; rsi = ((rbx as u32)) as u32 as u64; xmm4 = crate::simd_ops::vshufps(xmm4, xmm4, 0x0u8); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 32; continue; } bb = 3; continue; }
            3 => { fl_l = rax; fl_r = (0x10u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 17; continue; } bb = 4; continue; }
            4 => { rbp = ((rax.wrapping_mul(4).wrapping_sub(0x4u64)) as u64) as u64; r15 = ((rax.wrapping_mul(4).wrapping_sub(0x4u64)) as u64) as u64; rbp = ((rbp).wrapping_add(r10)) as u64; r15 = ((r15).wrapping_add(rcx)) as u64; fl_l = r10; fl_r = r15; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { fl_l = rbp; fl_r = rcx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 17; continue; } bb = 6; continue; }
            6 => { r14 = r8.wrapping_sub(0x4); r14 = ((r14.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; fl_l = r10; fl_r = r14; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 8; continue; } bb = 7; continue; }
            7 => { fl_l = rbp; fl_r = r8; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 17; continue; } bb = 8; continue; }
            8 => { rax = r9.wrapping_sub(0x4); rax = ((rax.wrapping_add(rdx.wrapping_mul(4))) as u64) as u64; fl_l = r10; fl_r = rax; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 10; continue; } bb = 9; continue; }
            9 => { fl_l = rbp; fl_r = r9; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 17; continue; } bb = 10; continue; }
            10 => { fl_l = r9; fl_r = r15; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 12; continue; } bb = 11; continue; }
            11 => { fl_l = rax; fl_r = rcx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 17; continue; } bb = 12; continue; }
            12 => { fl_l = r9; fl_r = r14; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 14; continue; } bb = 13; continue; }
            13 => { fl_l = rax; fl_r = r8; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 17; continue; } bb = 14; continue; }
            14 => { rbp = rdx; fl_l = (((rbp) & (0xfffffffffffffff0u64))) as u64; fl_r = fl_l; rbp = ((fl_l as u64)) as u64; rax = 0x20; bb = 15; continue; }
            15 => { xmm0 = (crate::simd_ops::ru128(rax.wrapping_add(r8).wrapping_sub(0x20u64))) as u128; rsi = ((rsi).wrapping_add(0x10u64)) as u64; xmm1 = (crate::simd_ops::ru128(rax.wrapping_add(rcx).wrapping_sub(0x20u64))) as u128; rax = rax.wrapping_add(0x40); xmm0 = crate::simd_ops::vmulps(xmm0, xmm4); xmm1 = crate::simd_ops::vmulps(xmm1, xmm4); crate::simd_ops::wu128(rax.wrapping_add(r9).wrapping_sub(0x60u64), (xmm0) as u128); xmm0 = (crate::simd_ops::ru128(rax.wrapping_add(r8).wrapping_sub(0x50u64))) as u128; crate::simd_ops::wu128(rax.wrapping_add(r10).wrapping_sub(0x60u64), (xmm1) as u128); xmm0 = crate::simd_ops::vmulps(xmm0, xmm4); xmm1 = (crate::simd_ops::ru128(rax.wrapping_add(rcx).wrapping_sub(0x50u64))) as u128; xmm1 = crate::simd_ops::vmulps(xmm1, xmm4); crate::simd_ops::wu128(rax.wrapping_add(r9).wrapping_sub(0x50u64), (xmm0) as u128); xmm0 = (crate::simd_ops::ru128(rax.wrapping_add(r8).wrapping_sub(0x40u64))) as u128; crate::simd_ops::wu128(rax.wrapping_add(r10).wrapping_sub(0x50u64), (xmm1) as u128); xmm0 = crate::simd_ops::vmulps(xmm0, xmm4); xmm1 = (crate::simd_ops::ru128(rax.wrapping_add(rcx).wrapping_sub(0x40u64))) as u128; xmm1 = crate::simd_ops::vmulps(xmm1, xmm4); crate::simd_ops::wu128(rax.wrapping_add(r9).wrapping_sub(0x40u64), (xmm0) as u128); xmm0 = (crate::simd_ops::ru128(rax.wrapping_add(r8).wrapping_sub(0x30u64))) as u128; crate::simd_ops::wu128(rax.wrapping_add(r10).wrapping_sub(0x40u64), (xmm1) as u128); xmm0 = crate::simd_ops::vmulps(xmm0, xmm4); xmm1 = (crate::simd_ops::ru128(rax.wrapping_add(rcx).wrapping_sub(0x30u64))) as u128; xmm1 = crate::simd_ops::vmulps(xmm1, xmm4); crate::simd_ops::wu128(rax.wrapping_add(r9).wrapping_sub(0x30u64), (xmm0) as u128); crate::simd_ops::wu128(rax.wrapping_add(r10).wrapping_sub(0x30u64), (xmm1) as u128); fl_l = rsi; fl_r = rbp; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 15; continue; } bb = 16; continue; }
            16 => { fl_l = rsi; fl_r = rdx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 32; continue; } bb = 17; continue; }
            17 => { rax = rdx; r8 = ((r8).wrapping_sub(r9)) as u64; rax = ((rax).wrapping_sub(rsi)) as u64; rcx = ((rcx).wrapping_sub(r9)) as u64; fl_l = rax; fl_r = (0x4u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 21; continue; } bb = 18; continue; }
            18 => { r10 = rdx; rax = r9.wrapping_add(0x4); r10 = ((r10).wrapping_sub(rsi)) as u64; rax = ((rax.wrapping_add(rsi.wrapping_mul(4))) as u64) as u64; r10 = ((r10).wrapping_sub(0x4u64)) as u64; r10 = ((r10).wrapping_shr(2u32)) as u64; fl_l = ((r10).wrapping_add(1)) as u64; r10 = ((fl_l as u64)) as u64; rsi = ((rsi.wrapping_add(r10.wrapping_mul(4))) as u64) as u64; bb = 19; continue; }
            19 => { rax = rax.wrapping_add(0x10); xmm0 = (xmm2) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(crate::simd_ops::ru32(r8.wrapping_add(rax).wrapping_sub(0x14u64)))).to_bits() as u128); xmm1 = (xmm2) as u128; crate::simd_ops::wu32(rax.wrapping_sub(0x14u64), ((xmm0 as u32)) as u32); xmm0 = (xmm2) as u128; xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(crate::simd_ops::ru32(rcx.wrapping_add(rax).wrapping_sub(0x14u64)))).to_bits() as u128); crate::simd_ops::wu32(rax.wrapping_add(r11.wrapping_mul(4)).wrapping_sub(0x14u64), ((xmm1 as u32)) as u32); xmm1 = (xmm2) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(crate::simd_ops::ru32(r8.wrapping_add(rax).wrapping_sub(0x10u64)))).to_bits() as u128); crate::simd_ops::wu32(rax.wrapping_sub(0x10u64), ((xmm0 as u32)) as u32); xmm0 = (xmm2) as u128; xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(crate::simd_ops::ru32(rcx.wrapping_add(rax).wrapping_sub(0x10u64)))).to_bits() as u128); crate::simd_ops::wu32(rax.wrapping_add(r11.wrapping_mul(4)).wrapping_sub(0x10u64), ((xmm1 as u32)) as u32); xmm1 = (xmm2) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(crate::simd_ops::ru32(r8.wrapping_add(rax).wrapping_sub(0xcu64)))).to_bits() as u128); crate::simd_ops::wu32(rax.wrapping_sub(0xcu64), ((xmm0 as u32)) as u32); xmm0 = (xmm2) as u128; xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(crate::simd_ops::ru32(rcx.wrapping_add(rax).wrapping_sub(0xcu64)))).to_bits() as u128); crate::simd_ops::wu32(rax.wrapping_add(r11.wrapping_mul(4)).wrapping_sub(0xcu64), ((xmm1 as u32)) as u32); xmm1 = (xmm2) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(crate::simd_ops::ru32(r8.wrapping_add(rax).wrapping_sub(0x8u64)))).to_bits() as u128); crate::simd_ops::wu32(rax.wrapping_sub(0x8u64), ((xmm0 as u32)) as u32); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(crate::simd_ops::ru32(rcx.wrapping_add(rax).wrapping_sub(0x8u64)))).to_bits() as u128); crate::simd_ops::wu32(rax.wrapping_add(r11.wrapping_mul(4)).wrapping_sub(0x8u64), ((xmm1 as u32)) as u32); fl_l = r10; fl_r = (0x1u64) as u64; r10 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 19; continue; } bb = 20; continue; }
            20 => { fl_l = rsi; fl_r = rdx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 32; continue; } bb = 21; continue; }
            21 => { rax = ((r9.wrapping_add(rsi.wrapping_mul(4))) as u64) as u64; fl_l = rdx; fl_r = rsi; rdx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; bb = 22; continue; }
            22 => { rax = rax.wrapping_add(0x4); xmm0 = (xmm2) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(crate::simd_ops::ru32(r8.wrapping_add(rax).wrapping_sub(0x4u64)))).to_bits() as u128); xmm1 = (xmm2) as u128; crate::simd_ops::wu32(rax.wrapping_sub(0x4u64), ((xmm0 as u32)) as u32); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(crate::simd_ops::ru32(rcx.wrapping_add(rax).wrapping_sub(0x4u64)))).to_bits() as u128); crate::simd_ops::wu32(rax.wrapping_add(r11.wrapping_mul(4)).wrapping_sub(0x4u64), ((xmm1 as u32)) as u32); fl_l = rdx; fl_r = (0x1u64) as u64; rdx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 22; continue; } bb = 23; continue; }
            23 => { bb = 32; continue; }
            24 => { xmm3 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) - f32::from_bits(xmm2 as u32)).to_bits() as u128); xmm0 = 0u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((((rax) as i64) as f32).to_bits() as u128); rax = rbx; xmm3 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) / f32::from_bits(xmm0 as u32)).to_bits() as u128); fl_l = rdx; fl_r = (0x4u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 28; continue; } bb = 25; continue; }
            25 => { r11 = rdx.wrapping_sub(0x3); bb = 26; continue; }
            26 => { xmm0 = (xmm2) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(crate::simd_ops::ru32(r8.wrapping_add(rax.wrapping_mul(4))))).to_bits() as u128); crate::simd_ops::wu32(r9.wrapping_add(rax.wrapping_mul(4)), ((xmm0 as u32)) as u32); xmm0 = (xmm2) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(crate::simd_ops::ru32(rcx.wrapping_add(rax.wrapping_mul(4))))).to_bits() as u128); xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) + f32::from_bits(xmm3 as u32)).to_bits() as u128); crate::simd_ops::wu32(r10.wrapping_add(rax.wrapping_mul(4)), ((xmm0 as u32)) as u32); xmm1 = (xmm2) as u128; xmm0 = (xmm2) as u128; xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(crate::simd_ops::ru32(r8.wrapping_add(rax.wrapping_mul(4)).wrapping_add(0x4u64)))).to_bits() as u128); xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) + f32::from_bits(xmm3 as u32)).to_bits() as u128); crate::simd_ops::wu32(r9.wrapping_add(rax.wrapping_mul(4)).wrapping_add(0x4u64), ((xmm1 as u32)) as u32); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(crate::simd_ops::ru32(rcx.wrapping_add(rax.wrapping_mul(4)).wrapping_add(0x4u64)))).to_bits() as u128); xmm1 = (xmm2) as u128; crate::simd_ops::wu32(r10.wrapping_add(rax.wrapping_mul(4)).wrapping_add(0x4u64), ((xmm0 as u32)) as u32); xmm0 = (xmm2) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(crate::simd_ops::ru32(r8.wrapping_add(rax.wrapping_mul(4)).wrapping_add(0x8u64)))).to_bits() as u128); xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) + f32::from_bits(xmm3 as u32)).to_bits() as u128); crate::simd_ops::wu32(r9.wrapping_add(rax.wrapping_mul(4)).wrapping_add(0x8u64), ((xmm0 as u32)) as u32); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(crate::simd_ops::ru32(rcx.wrapping_add(rax.wrapping_mul(4)).wrapping_add(0x8u64)))).to_bits() as u128); xmm0 = (xmm2) as u128; crate::simd_ops::wu32(r10.wrapping_add(rax.wrapping_mul(4)).wrapping_add(0x8u64), ((xmm1 as u32)) as u32); xmm1 = (xmm2) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(crate::simd_ops::ru32(r8.wrapping_add(rax.wrapping_mul(4)).wrapping_add(0xcu64)))).to_bits() as u128); xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) + f32::from_bits(xmm3 as u32)).to_bits() as u128); crate::simd_ops::wu32(r9.wrapping_add(rax.wrapping_mul(4)).wrapping_add(0xcu64), ((xmm0 as u32)) as u32); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(crate::simd_ops::ru32(rcx.wrapping_add(rax.wrapping_mul(4)).wrapping_add(0xcu64)))).to_bits() as u128); crate::simd_ops::wu32(r10.wrapping_add(rax.wrapping_mul(4)).wrapping_add(0xcu64), ((xmm1 as u32)) as u32); rax = ((rax).wrapping_add(0x4u64)) as u64; fl_l = rax; fl_r = r11; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 26; continue; } bb = 27; continue; }
            27 => { bb = 31; continue; }
            28 => { fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 32; continue; } bb = 29; continue; }
            29 => { bb = 30; continue; }
            30 => { xmm0 = (xmm2) as u128; xmm1 = (xmm2) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(crate::simd_ops::ru32(r8.wrapping_add(rax.wrapping_mul(4))))).to_bits() as u128); xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) + f32::from_bits(xmm3 as u32)).to_bits() as u128); crate::simd_ops::wu32(r9.wrapping_add(rax.wrapping_mul(4)), ((xmm0 as u32)) as u32); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(crate::simd_ops::ru32(rcx.wrapping_add(rax.wrapping_mul(4))))).to_bits() as u128); crate::simd_ops::wu32(r10.wrapping_add(rax.wrapping_mul(4)), ((xmm1 as u32)) as u32); fl_l = ((rax).wrapping_add(1)) as u64; rax = ((fl_l as u64)) as u64; bb = 31; continue; }
            31 => { fl_l = rax; fl_r = rdx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 30; continue; } bb = 32; continue; }
            32 => { rax = ((crate::simd_ops::ru16(r12.wrapping_add(0x12u64))) as u32) as u32 as u64; r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu16(rdi.wrapping_add(0x42u64), ((rax as u16)) as u16); rcx = ((crate::simd_ops::ru16(rdi.wrapping_add(0x40u64))) as u32) as u32 as u64; rdx = ((crate::simd_ops::ru8(rdi.wrapping_add(0x38u64))) as u32) as u32 as u64; fl_l = ((rax as u16)) as u16 as u64; fl_r = ((rcx as u16)) as u16 as u64; rax = (rax & !0xffffu64) | (((if (((fl_l as u16)) >= ((fl_r as u16))) { (rcx as u16) } else { (rax as u16) }) as u16 as u64)); r8 = (((rax as u16)) as u32) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 37; continue; } bb = 33; continue; }
            33 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 36; continue; } bb = 34; continue; }
            34 => { rbp = ((rcx.wrapping_mul(4)) as u32) as u32 as u64; r14 = ((r8 as u32)) as u32 as u64; rsi = ((rdx as u32)) as u32 as u64; bb = 35; continue; }
            35 => { rcx = ((crate::simd_ops::ru16(rdi.wrapping_add(0x40u64))) as u32) as u32 as u64; r8 = rbp; rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x30u64))) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = (((rcx) as i64).wrapping_mul((rbx) as i64) as u64) as u64; rcx = ((rcx).wrapping_add(r14)) as u64; rcx = ((rax.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = ((rbx).wrapping_add(1)) as u64; fl_l = rsi; fl_r = (0x1u64) as u64; rsi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 35; continue; } bb = 36; continue; }
            36 => { rax = ((crate::simd_ops::ru16(rdi.wrapping_add(0x40u64))) as u32) as u32 as u64; crate::simd_ops::wu16(rdi.wrapping_add(0x42u64), ((rax as u16)) as u16); bb = 37; continue; }
            37 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; bb = 38; continue; }
            38 => { rsp = ((rsp).wrapping_add(0x48u64)) as u64; r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14152b600(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14152b0f0; const P1: u64 = 0x14152b0a0; const P2: u64 = 0x143413a30; const P3: u64 = 0x143413a20; const P4: u64 = 0x142647298; const P5: u64 = 0x142676780; const P6: u64 = 0x1426472a0; const P7: u64 = 0x1426472a8; const P8: u64 = 0x142646a20;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedStack::<1>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rbp = rsp.wrapping_sub(0x2000); rax = 0x2100; rsp = ((rsp).wrapping_sub(rax)) as u64; r14 = r8; rbx = ((rdx as u32)) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 19; continue; } bb = 1; continue; }
            1 => { rdx = rbp.wrapping_sub(0x10); rcx = ((rbx as u32)) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 19; continue; } bb = 2; continue; }
            2 => { r15 = 0; fl_l = 0; fl_r = 0; rax = rsp.wrapping_add(0x48); r9 = P2; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), r15); rdx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); rcx = P3; r8 = ((r15.wrapping_add(0x17u64)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 19; continue; } bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 19; continue; } bb = 4; continue; }
            4 => { crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), r15); r9 = rsp.wrapping_add(0x40); rax = (crate::simd_ops::ru64(rcx)) as u64; r8 = ((r15.wrapping_add(0xfu64)) as u32) as u32 as u64; rdx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsi = ((r15.wrapping_add(0x2u64)) as u32) as u32 as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 18; continue; } bb = 5; continue; }
            5 => { crate::simd_ops::wu32(rbp.wrapping_add(0x2038u64), ((r15 as u32)) as u32); rdx = rbp.wrapping_add(0x2038); rax = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x2120u64), rdi); (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = ((r15 as u32)) as u32 as u64; fl_l = (crate::simd_ops::ru32(rbp.wrapping_add(0x2038u64))) as u32 as u64; fl_r = ((r15 as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) <= (((fl_r as u32)) as i32)) { bb = 17; continue; } bb = 6; continue; }
            6 => { bb = 7; continue; }
            7 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; r8 = rsp.wrapping_add(0x30); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), r15); rdx = ((rdi as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x20u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 8; continue; }
            8 => { crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), r15); r8 = rsp.wrapping_add(0x38); rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x20u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 13; continue; } bb = 9; continue; }
            9 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; r8 = rsp.wrapping_add(0x50); rax = 0; fl_l = 0; fl_r = 0; rdx = P5; xmm0 = 0u128; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rax); crate::simd_ops::wu128(rsp.wrapping_add(0x50u64), (xmm0) as u128); rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x28u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 13; continue; }
            11 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rdx = rsp.wrapping_add(0x70); r8 = 0x40; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rsp.wrapping_add(0x50); rbx = ((rax as u32)) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P7), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 13; continue; } bb = 12; continue; }
            12 => { r8 = 0x40; rdx = rsp.wrapping_add(0x70); rcx = rbp.wrapping_sub(0xc); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P8), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 16; continue; } bb = 13; continue; }
            13 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 14; continue; }
            14 => { rdi = (((rdi as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((rdi as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rbp.wrapping_add(0x2038u64))) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 7; continue; } bb = 15; continue; }
            15 => { bb = 17; continue; }
            16 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = 0x1; crate::simd_ops::wu64(r14, rax); bb = 17; continue; }
            17 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rdx = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rdx.wrapping_add(0x10u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x2120u64))) as u64; bb = 18; continue; }
            18 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rdx = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rdx.wrapping_add(0x10u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = ((rsi as u32)) as u32 as u64; bb = 20; continue; }
            19 => { rax = 0x2; bb = 20; continue; }
            20 => { r11 = rsp.wrapping_add(0x2100); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x30u64))) as u64; rsp = r11; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14152b800(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142647278; const P1: u64 = 0x14152b0b0; const P2: u64 = 0x14152b600; const P3: u64 = 0x142645718; const P4: u64 = 0x142647268; const P5: u64 = 0x142676898; const P6: u64 = 0x1426768a8; const P7: u64 = 0x142676820; const P8: u64 = 0x14183acc2; const P9: u64 = 0x1426472d0;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 59>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = rsp; crate::simd_ops::wu64(rax.wrapping_add(0x18u64), r8); crate::simd_ops::wu64(rax.wrapping_add(0x10u64), rdx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rbp = rax.wrapping_sub(0xd8); rsp = ((rsp).wrapping_sub(0x1c8u64)) as u64; crate::simd_ops::wu64(rax.wrapping_add(0x20u64), rbx); rdi = 0x2; crate::simd_ops::wu64(rax.wrapping_sub(0x18u64), rsi); rdx = ((rdi as u32)) as u32 as u64; crate::simd_ops::wu64(rax.wrapping_sub(0x20u64), r12); rsi = rcx; crate::simd_ops::wu64(rax.wrapping_sub(0x30u64), r14); rcx = 0; fl_l = 0; fl_r = 0; r14 = r8; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r12 = 0; fl_l = 0; fl_r = 0; rbx = ((rax as u32)) as u32 as u64; fl_l = (crate::simd_ops::ru64(rsi.wrapping_add(0x10u64))) as u64; fl_r = r12; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 19; continue; } bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru32(rsi.wrapping_add(0x8u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 19; continue; } bb = 2; continue; }
            2 => { rdx = rbp.wrapping_add(0xe0); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru32(rbp.wrapping_add(0xe0u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 41; continue; } bb = 3; continue; }
            3 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rdi = ((rax.wrapping_add(0x59u64)) as u32) as u32 as u64; bb = 41; continue; }
            5 => { rdx = (crate::simd_ops::ru32(rsi.wrapping_add(0x8u64))) as u32 as u64; r8 = rsp.wrapping_add(0x40); rcx = r14; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), r12); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 41; continue; } bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x10u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 8; continue; }
            8 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; crate::simd_ops::wu64(rsi.wrapping_add(0x10u64), rcx); crate::simd_ops::wu32(rsi.wrapping_add(0x18u64), (0xffffffffu32) as u32); fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 19; continue; } bb = 9; continue; }
            9 => { crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), r12); rdx = rsp.wrapping_add(0x40); rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x28u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 19; continue; } bb = 10; continue; }
            10 => { r8 = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rax = (0xffffffffffffffffu64) as u64; rcx = rax; bb = 11; continue; }
            11 => { rcx = ((rcx).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru16(r8.wrapping_add(rcx.wrapping_mul(2)))) as u16 as u64; fl_r = ((r12 as u16)) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 11; continue; } bb = 12; continue; }
            12 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x102; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 15; continue; } bb = 13; continue; }
            13 => { rax = ((rax).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru16(r8.wrapping_add(rax.wrapping_mul(2)))) as u16 as u64; fl_r = ((r12 as u16)) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 13; continue; } bb = 14; continue; }
            14 => { bb = 16; continue; }
            15 => { rax = 0x102; bb = 16; continue; }
            16 => { crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), r12); rcx = rbp.wrapping_sub(0x70); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), r12); r9 = ((rax as u32)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0x103u32) as u32); rdx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rcx); rcx = 0xfde9; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = ((((rax as u32)) as i32 as i64) as u64) as u64; r10 = rbp.wrapping_sub(0x70); r9 = 0x811c9dc5; crate::simd_ops::wu8(rbp.wrapping_add(rcx).wrapping_sub(0x70u64), ((r12 as u8)) as u8); r8 = ((crate::simd_ops::ru8(rbp.wrapping_sub(0x70u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rbp.wrapping_add(0x93u64), ((r12 as u8)) as u8); fl_l = ((r8 as u8)) as u8 as u64; fl_r = ((r8 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 18; continue; } bb = 17; continue; }
            17 => { r9 = ((((r9 as u32)) as i32).wrapping_mul((0x1000193u32) as i32) as u32) as u32 as u64; rax = ((r8.wrapping_add(0x20u64)) as u32) as u32 as u64; rcx = (((rax as u8)) as u32) as u32 as u64; rdx = ((r8.wrapping_sub(0x41u64)) as u32) as u32 as u64; rax = (((r8 as u8)) as u32) as u32 as u64; r10 = r10.wrapping_add(0x1); r8 = ((crate::simd_ops::ru8(r10)) as u32) as u32 as u64; fl_l = ((rdx as u8)) as u8 as u64; fl_r = (0x19u8) as u8 as u64; rcx = (if (((fl_l as u8)) > ((fl_r as u8))) { (rax as u32) } else { (rcx as u32) }) as u32 as u64; rax = (((rcx as u8)) as u32) as u32 as u64; r9 = ((((r9 as u32)) ^ ((rax as u32)))) as u32 as u64; fl_l = ((r8 as u8)) as u8 as u64; fl_r = ((r8 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 17; continue; } bb = 18; continue; }
            18 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; crate::simd_ops::wu32(rsi.wrapping_add(0x18u64), ((r9 as u32)) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 19; continue; }
            19 => { rax = (crate::simd_ops::ru64(r14)) as u64; rcx = r14; crate::simd_ops::wu64(rsp.wrapping_add(0x1b0u64), r13); crate::simd_ops::wu64(rsp.wrapping_add(0x1a0u64), r15); (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rax; rdx = (crate::simd_ops::ru64(rax)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rdx.wrapping_add(0x10u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = (crate::simd_ops::ru64(r14)) as u64; rcx = r14; r13 = (((rax as u16)) as u32) as u32 as u64; crate::simd_ops::wu16(rbp.wrapping_add(0xe0u64), ((r13 as u16)) as u16); (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rax; rdx = (crate::simd_ops::ru64(rax)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rdx.wrapping_add(0x20u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x10u64))) as u64; r8 = rsp.wrapping_add(0x50); rdx = 0; fl_l = 0; fl_r = 0; r14 = ((rax as u32)) as u32 as u64; r9 = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(r9.wrapping_add(0x20u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; fl_r = r12; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 37; continue; } bb = 20; continue; }
            20 => { rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x10u64))) as u64; r15 = rsi.wrapping_add(0x20); r9 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r15); rdx = P5; rax = (crate::simd_ops::ru64(rcx)) as u64; r8 = ((r9.wrapping_add(0x17u64)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 38; continue; } bb = 21; continue; }
            21 => { rcx = (crate::simd_ops::ru64(r15)) as u64; rdx = rsp.wrapping_add(0x48); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), r12); rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x40u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 38; continue; } bb = 22; continue; }
            22 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 38; continue; } bb = 23; continue; }
            23 => { fl_l = (crate::simd_ops::ru16(rcx.wrapping_add(0x2u64))) as u16 as u64; fl_r = (0x4u16) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 37; continue; } bb = 24; continue; }
            24 => { xmm0 = (crate::simd_ops::ru128(rcx)) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x60u64), (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(rcx.wrapping_add(0x10u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x70u64), (xmm1) as u128); xmm0 = ((crate::simd_ops::ru64(rcx.wrapping_add(0x20u64))) as u64) as u128; crate::simd_ops::wu64(rbp.wrapping_sub(0x80u64), ((xmm0 as u64)) as u64); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), r12); fl_l = (crate::simd_ops::ru32(rsp.wrapping_add(0x64u64))) as u32 as u64; fl_r = ((r14 as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 26; continue; } bb = 25; continue; }
            25 => { rax = ((crate::simd_ops::ru16(rsp.wrapping_add(0x6cu64))) as u32) as u32 as u64; r12 = 0x80000000; rax = ((((rax as u32)) as i32).wrapping_mul(((r14 as u32)) as i32) as u32) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x64u64), ((r14 as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x68u64), ((rax as u32)) as u32); bb = 26; continue; }
            26 => { rcx = (crate::simd_ops::ru64(r15)) as u64; r8 = rsp.wrapping_add(0x58); rdx = rbp.wrapping_sub(0x78); rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x48u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = 0; fl_l = 0; fl_r = 0; rax = (((r13) as i64).wrapping_mul((0x989680u64) as i64) as u64) as u64; r13 = (crate::simd_ops::ru64(rbp.wrapping_add(0x100u64))) as u64; rcx = (crate::simd_ops::ru32(r13)) as u32 as u64; let d = (rcx) as u128; let n = ((rdx as u128) << 64) | (rax as u128); if d != 0 { rax = (n / d) as u64; rdx = (n % d) as u64; } rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0xf0u64))) as u64; r14 = rax; rdx = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rdx.wrapping_add(0x80u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r9 = (((rax as u16)) as u32) as u32 as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; r9 = (((r9) as i64).wrapping_mul((r14) as i64) as u64) as u64; fl_l = r9; fl_r = rax; if ((((fl_l as u64)) as i64) >= (((fl_r as u64)) as i64)) { bb = 29; continue; } bb = 27; continue; }
            27 => { bb = 28; continue; }
            28 => { r9 = ((r9).wrapping_add(r14)) as u64; fl_l = r9; fl_r = rax; if ((((fl_l as u64)) as i64) < (((fl_r as u64)) as i64)) { bb = 28; continue; } bb = 29; continue; }
            29 => { rcx = (crate::simd_ops::ru64(r15)) as u64; rdx = rsp.wrapping_add(0x60); r8 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), r8); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rdx); rdx = 0; fl_l = 0; fl_r = 0; rax = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r8); r8 = ((r12 as u32)) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 38; continue; } bb = 30; continue; }
            30 => { rcx = (crate::simd_ops::ru64(r15)) as u64; rdx = rsi.wrapping_add(0x48); rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x20u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 38; continue; } bb = 31; continue; }
            31 => { rcx = (crate::simd_ops::ru64(r15)) as u64; r8 = rsi.wrapping_add(0x28); rdx = P6; rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x70u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 38; continue; } bb = 32; continue; }
            32 => { rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0xe8u64))) as u64; r9 = P7; r15 = ((crate::simd_ops::ru16(rbp.wrapping_add(0xe0u64))) as u32) as u32 as u64; r8 = 0x10; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x185u32) as u32); rax = (crate::simd_ops::ru64(rcx)) as u64; r14 = ((r15.wrapping_mul(8)) as u64) as u64; rdx = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 34; continue; } bb = 33; continue; }
            33 => { rdi = ((rax.wrapping_add(0x34u64)) as u32) as u32 as u64; bb = 38; continue; }
            34 => { rcx = 0x3; rdx = (rdx & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 35; continue; }
            35 => { rdx = (rdx & !0xffu64) | (((((rdx as u8)).wrapping_add(1)) as u8 as u64)); rax = ((rcx.wrapping_sub(0x1u64)) as u32) as u32 as u64; fl_l = ((((rcx as u32)) & ((rax as u32)))) as u32 as u64; fl_r = fl_l; rcx = ((fl_l as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 35; continue; } bb = 36; continue; }
            36 => { crate::simd_ops::wu8(r13.wrapping_add(0x4u64), ((rdx as u8)) as u8); r8 = r14; crate::simd_ops::lk_and_u32(r13.wrapping_add(0x4u64), (0xffu32) as u32); rdx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::lk_or_u32(r13.wrapping_add(0x4u64), (0x3100u32) as u32); rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(rsi.wrapping_add(0x3cu64), (0x2bu32) as u32); r14 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsi.wrapping_add(0x30u64), rdi); crate::simd_ops::wu16(rsi.wrapping_add(0x40u64), ((r15 as u16)) as u16); crate::simd_ops::wu16(rsi.wrapping_add(0x42u64), ((r14 as u16)) as u16); rdi = ((r14.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rsi.wrapping_add(0x38u64), (0x3102u32) as u32); bb = 39; continue; }
            37 => { rdi = 0x5a; bb = 38; continue; }
            38 => { r14 = 0; fl_l = 0; fl_r = 0; bb = 39; continue; }
            39 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x1a0u64))) as u64; r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x1b0u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 41; continue; } bb = 40; continue; }
            40 => { rdx = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rdx.wrapping_add(0x10u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), r14); bb = 41; continue; }
            41 => { r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x1a8u64))) as u64; fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x1f8u64))) as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x1b8u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x1c0u64))) as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 43; continue; } bb = 42; continue; }
            42 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P9), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 43; continue; }
            43 => { rax = ((rdi as u32)) as u32 as u64; rsp = ((rsp).wrapping_add(0x1c8u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14152bc60(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14152b0f0; const P1: u64 = 0x14152b0e0; const P2: u64 = 0x14183acc2;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = ((crate::simd_ops::ru16(rcx.wrapping_add(0x42u64))) as u32) as u32 as u64; r8 = rsp.wrapping_add(0x40); rsi = ((crate::simd_ops::ru16(rcx.wrapping_add(0x40u64))) as u32) as u32 as u64; rbp = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rbp); rbx = rcx; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x28u64))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 9; continue; } bb = 1; continue; }
            1 => { rdx = ((rdi as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 12; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; fl_r = rbp; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 12; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), r14); rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; rdx = ((rbp.wrapping_add(0x1u64)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            5 => { r14 = (crate::simd_ops::ru64(rbx.wrapping_add(0x30u64))) as u64; r10 = ((rbp as u32)) as u32 as u64; rax = ((crate::simd_ops::ru16(rbx.wrapping_add(0x40u64))) as u32) as u32 as u64; r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; r8 = ((r14.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 8; continue; } bb = 6; continue; }
            6 => { r11 = r14; fl_l = r11; fl_r = r8; r11 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; bb = 7; continue; }
            7 => { rdx = ((r10.wrapping_mul(4)) as u32) as u32 as u64; r10 = (((r10 as u32)).wrapping_add(1)) as u32 as u64; crate::simd_ops::wu32(r9.wrapping_add(rdx.wrapping_mul(4)), ((rbp as u32)) as u32); rax = ((rdx.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(r9.wrapping_add(rax.wrapping_mul(4)), ((rbp as u32)) as u32); rcx = ((rdx.wrapping_add(0x2u64)) as u32) as u32 as u64; rax = (crate::simd_ops::ru32(r11.wrapping_add(r8))) as u32 as u64; rdx = ((rdx.wrapping_add(0x3u64)) as u32) as u32 as u64; crate::simd_ops::wu32(r9.wrapping_add(rcx.wrapping_mul(4)), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(r8)) as u32 as u64; r8 = r8.wrapping_add(0x4); crate::simd_ops::wu32(r9.wrapping_add(rdx.wrapping_mul(4)), ((rax as u32)) as u32); fl_l = ((r10 as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 7; continue; } bb = 8; continue; }
            8 => { r8 = ((rsi.wrapping_mul(8)) as u32) as u32 as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = r14; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x28u64))) as u64; r8 = 0; fl_l = 0; fl_r = 0; rdx = ((rdi as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x20u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; crate::simd_ops::wu16(rbx.wrapping_add(0x42u64), ((rbp as u16)) as u16); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            9 => { rdx = ((rsi as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 12; continue; } bb = 10; continue; }
            10 => { fl_l = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; fl_r = rbp; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 12; continue; } bb = 11; continue; }
            11 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x28u64))) as u64; r8 = 0x2; rdx = ((rsi as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x20u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 12; continue; }
            12 => { crate::simd_ops::wu16(rbx.wrapping_add(0x42u64), ((rbp as u16)) as u16); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14152bd80(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x20; const P1: u64 = 0x50;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp);
    rax = 0x1;
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14152bda0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x28; const P1: u64 = 0x10; const P2: u64 = 0x20; const P3: u64 = 0x58; const P4: u64 = 0x30; const P5: u64 = 0x42;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rcx; rsi = 0; fl_l = 0; fl_r = 0; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rdi = rdx; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(P0), rsi); bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(P2), rsi); bb = 4; continue; }
            4 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P4))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rcx = rdi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(P4), rsi); crate::simd_ops::wu16(rbx.wrapping_add(P5), ((rsi as u16)) as u16); bb = 6; continue; }
            6 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = 0x1; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14152be20(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142676820; const P1: u64 = 0x1426767d8; const P2: u64 = 0x14152b0f0; const P3: u64 = 0x14152b0b0; const P4: u64 = 0x14152bda0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; r14 = r9; r9 = 0x45; rsi = (((r8 as u8)) as u32) as u32 as u64; rbp = ((rdx as u32)) as u32 as u64; r8 = P0; rdi = rcx; rdx = ((r9.wrapping_add(0xbu64)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rbx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), r15); rax = P1; r15 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rbx, rax); crate::simd_ops::wu8(rbx.wrapping_add(0x38u64), ((r15 as u8)) as u8); crate::simd_ops::lk_and_u32(rbx.wrapping_add(0x38u64), (0xffu32) as u32); crate::simd_ops::wu32(rbx.wrapping_add(0x8u64), (0xffffffffu32) as u32); crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), r15); crate::simd_ops::wu32(rbx.wrapping_add(0x18u64), (0xffffffffu32) as u32); crate::simd_ops::wu64(rbx.wrapping_add(0x20u64), r15); crate::simd_ops::wu64(rbx.wrapping_add(0x28u64), r15); crate::simd_ops::wu32(rbx.wrapping_add(0x48u64), ((r15 as u32)) as u32); crate::simd_ops::wu64(rbx.wrapping_add(0x30u64), r15); crate::simd_ops::wu64(rbx.wrapping_add(0x3cu64), (0x2bu64) as u64); fl_l = ((rsi as u8)) as u8 as u64; fl_r = ((rsi as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 5; continue; } bb = 2; continue; }
            2 => { rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 5; continue; } bb = 3; continue; }
            3 => { rsi = ((rbp as u32)) as u32 as u64; rdx = rsp.wrapping_add(0x50); fl_l = ((((rsi as u32)) >> (31u32)) & 1) as u64; rsi = (((rsi as u32)) & !((1u32) << (31u32))) as u32 as u64; rcx = ((rsi as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { crate::simd_ops::wu32(rbx.wrapping_add(0x8u64), ((rsi as u32)) as u32); bb = 11; continue; }
            5 => { rax = (crate::simd_ops::ru64(r14)) as u64; rcx = r14; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = rax; rdx = (crate::simd_ops::ru64(rax)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rdx.wrapping_add(0xc0u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = rax; rsi = rax; rdx = (crate::simd_ops::ru64(rax)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 6; continue; }
            6 => { rdx = (crate::simd_ops::ru64(rsi)) as u64; rcx = rsi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); r8 = ((rbp as u32)) as u32 as u64; rdx = rsp.wrapping_add(0x20); rcx = rax; r9 = (crate::simd_ops::ru64(rax)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(r9.wrapping_add(0x20u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rsi = rax; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { rdx = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rdx.wrapping_add(0x10u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), r15); bb = 8; continue; }
            8 => { rcx = (crate::simd_ops::ru64(rsi)) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), rcx); rcx = (crate::simd_ops::ru32(rsi.wrapping_add(0x8u64))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x18u64), ((rcx as u32)) as u32); crate::simd_ops::wu64(rsi, r15); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 10; continue; }
            10 => { fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; fl_r = r15; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 14; continue; } bb = 11; continue; }
            11 => { rax = rbx; bb = 12; continue; }
            12 => { r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; bb = 13; continue; }
            13 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            14 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = rdi; rcx = rbx; r8 = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; rax = P4; fl_l = r8; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 16; continue; } bb = 15; continue; }
            15 => { rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 17; continue; }
            16 => { rax = crate::simd_ops::call_stk(r8, rax, rcx, rdx, r8, r9, rsp); bb = 17; continue; }
            17 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rdi)) as u64; rdx = rbx; rcx = rdi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rax = 0; fl_l = 0; fl_r = 0; bb = 12; continue; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14152c010(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x4;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    crate::simd_ops::wu8(rdx, (0x7u8) as u8);
    rax = 0x1;
    crate::simd_ops::wu32(rdx.wrapping_add(P0), (0x7e601u32) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14152c020(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14152cc80; const P1: u64 = 0x14152e120; const P2: u64 = 0x14152d960; const P3: u64 = 0x14152df30; const P4: u64 = 0x14152be20;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rax = (crate::simd_ops::ru64(r8)) as u64; rsi = r8; rdi = rdx; r8 = rsp.wrapping_add(0x34); rbx = rcx; rdx = rsp.wrapping_add(0x60); rcx = rsi; rbp = r9; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); r8 = rsi; rdx = rsp.wrapping_add(0x30); rcx = rsp.wrapping_add(0x60); rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 11; continue; } bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru32(rsp.wrapping_add(0x30u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 10; continue; } bb = 2; continue; }
            2 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 9; continue; } bb = 3; continue; }
            3 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 8; continue; } bb = 4; continue; }
            4 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 7; continue; } bb = 5; continue; }
            5 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 15; continue; } bb = 6; continue; }
            6 => { rdx = (crate::simd_ops::ru32(rsp.wrapping_add(0x60u64))) as u32 as u64; rcx = rdi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 14; continue; }
            7 => { rdx = (crate::simd_ops::ru32(rsp.wrapping_add(0x60u64))) as u32 as u64; r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 13; continue; }
            8 => { rdx = (crate::simd_ops::ru32(rsp.wrapping_add(0x60u64))) as u32 as u64; rcx = rdi; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 14; continue; }
            9 => { rdx = (crate::simd_ops::ru32(rsp.wrapping_add(0x60u64))) as u32 as u64; rcx = rdi; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 14; continue; }
            10 => { rax = 0x2; bb = 18; continue; }
            11 => { rdx = (crate::simd_ops::ru32(rsp.wrapping_add(0x60u64))) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 15; continue; } bb = 12; continue; }
            12 => { r8 = 0; fl_l = 0; fl_r = 0; bb = 13; continue; }
            13 => { r9 = rsi; rcx = rdi; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 14; continue; }
            14 => { crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), rax); bb = 15; continue; }
            15 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 16; continue; }
            16 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; r9 = rbp; r10 = (crate::simd_ops::ru64(rcx)) as u64; r8 = rsi; rdx = rdi; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(r10.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 18; continue; }
            17 => { rax = 0x5a; bb = 18; continue; }
            18 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14152c160(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rcx; rdi = rdx; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rsi); rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rsi = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rsi)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rsi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rdi)) as u64; rdx = rsi; rcx = rdi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 3; continue; }
            3 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(P0), (0x0u64) as u64); bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rdi)) as u64; rdx = rbx; rcx = rdi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rax = 0x1; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14152c1e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x3; const P1: u64 = 0x1426452c8;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x48u64)) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x1000u32) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (r8) as u64); crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((rdx as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x34u64), (0x0u32) as u32); r9 = ((rsp.wrapping_add(0x20u64)) as u64) as u64; rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; r8 = ((rdx.wrapping_add(P0)) as u32) as u32 as u64; rcx = (0x406d1388u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P1)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 1; continue; }
            1 => { rsp = ((rsp).wrapping_add(0x48u64)) as u64; return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14152c220(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143930da4; const P1: u64 = 0x143930dc8; const P2: u64 = 0x1426452b0; const P3: u64 = 0x143930da0; const P4: u64 = 0x14392e280; const P5: u64 = 0x1426452a8; const P6: u64 = 0x214; const P7: u64 = 0x143930da8; const P8: u64 = 0x18;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; fl_l = (crate::simd_ops::ru8(P0)) as u8 as u64; fl_r = (0x0u8) as u8 as u64; rdi = rdx; rbx = ((rcx as u32)) as u32 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 6; continue; } bb = 1; continue; }
            1 => { rcx = P1; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = (crate::simd_ops::ru32(P3)) as u32 as u64; rax = 0; fl_l = 0; fl_r = 0; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 5; continue; } bb = 2; continue; }
            2 => { r9 = P4; bb = 3; continue; }
            3 => { rcx = ((rax as u32)) as u32 as u64; r8 = (((rcx) as i64).wrapping_mul((0x228u64) as i64) as u64) as u64; fl_l = ((rbx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(r8.wrapping_add(r9))) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 7; continue; } bb = 4; continue; }
            4 => { rax = (((rax as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 3; continue; } bb = 5; continue; }
            5 => { rcx = P1; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 6; continue; }
            6 => { rax = 0x2; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            7 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 5; continue; } bb = 8; continue; }
            8 => { xmm0 = (crate::simd_ops::ru128(r8.wrapping_add(r9).wrapping_add(P6))) as u128; rcx = P1; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(P7)) as u64; rdx = rsp.wrapping_add(0x20); r9 = 0; fl_l = 0; fl_r = 0; r8 = rdi; rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P8))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 6; continue; } bb = 9; continue; }
            9 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rax = 0x1; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14152c2e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142647278; const P1: u64 = 0x142676660; const P2: u64 = 0x142676650; const P3: u64 = 0x1; const P4: u64 = 0x142647298; const P5: u64 = 0x142645728; const P6: u64 = 0x38; const P7: u64 = 0x10; const P8: u64 = 0x1426472d0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rdx = 0x2; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rbx); rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); rsi = 0; fl_l = 0; fl_r = 0; r9 = P1; rbx = ((rax as u32)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rsi); rax = rsp.wrapping_add(0x40); rdx = 0; fl_l = 0; fl_r = 0; rcx = P2; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); r8 = ((rsi.wrapping_add(P3)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 5; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rdi); rdi = (crate::simd_ops::ru64(rax)) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; r8 = 0x800; rdx = rax; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rdi.wrapping_add(P6))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) >= 0) { bb = 4; continue; } bb = 2; continue; }
            2 => { fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P7))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = ((rsi as u32)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rcx); bb = 4; continue; }
            4 => { rsi = rcx; bb = 5; continue; }
            5 => { fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P8), rax), rax, rcx, rdx, r8, r9, rsp); bb = 7; continue; }
            7 => { rax = rsi; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14152c390(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x50; const P1: u64 = 0x14; const P2: u64 = 0x143930dc8; const P3: u64 = 0x1426452b0; const P4: u64 = 0x143930da0; const P5: u64 = 0x1426452a8; const P6: u64 = 0x4; const P7: u64 = 0x14392e280; const P8: u64 = 0x10; const P9: u64 = 0x20c; const P10: u64 = 0x214; const P11: u64 = 0x28; const P12: u64 = 0x1426469d0;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; r9 = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64; rax = 0; fl_l = 0; fl_r = 0; rdi = rcx; fl_l = ((r9 as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { r10 = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64; bb = 2; continue; }
            2 => { fl_l = ((r10 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rdx.wrapping_add(rax.wrapping_mul(4)))) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = (((rax as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 2; continue; } bb = 4; continue; }
            4 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 11; continue; } bb = 5; continue; }
            5 => { rcx = P2; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = (crate::simd_ops::ru32(P4)) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x14; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rcx = P2; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = 0; fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            7 => { rcx = ((rdi.wrapping_add(P6)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx); rbx = (((rdx) as i64).wrapping_mul((0x228u64) as i64) as u64) as u64; rdx = (((rdx as u32)).wrapping_add(1)) as u32 as u64; rax = P7; crate::simd_ops::wu32(P4, ((rdx as u32)) as u32); r8 = ((rcx.wrapping_add(P8)) as u64) as u64; xmm0 = (crate::simd_ops::ru128(rcx)) as u128; rbx = ((rbx).wrapping_add(rax)) as u64; rdx = 0x811c9dc5; crate::simd_ops::wu32(rbx.wrapping_add(P9), (0x1u32) as u32); crate::simd_ops::wu128(rbx.wrapping_add(P10), (xmm0) as u128); fl_l = rcx; fl_r = r8; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 10; continue; } bb = 8; continue; }
            8 => { bb = 9; continue; }
            9 => { rax = ((crate::simd_ops::ru8(rcx)) as u32) as u32 as u64; rcx = ((rcx).wrapping_add(1)) as u64; rdx = ((((rdx as u32)) as i32).wrapping_mul((0x1000193u32) as i32) as u32) as u32 as u64; rdx = ((((rdx as u32)) ^ ((rax as u32)))) as u32 as u64; fl_l = rcx; fl_r = r8; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 9; continue; } bb = 10; continue; }
            10 => { crate::simd_ops::wu32(rbx, ((rdx as u32)) as u32); r8 = ((rdi.wrapping_add(P11)) as u64) as u64; rdx = 0x104; rcx = rbx.wrapping_add(0x4); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P12), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = P2; crate::simd_ops::wu8(rbx.wrapping_add(0x210u64), (0x0u8) as u8); crate::simd_ops::wu32(rbx.wrapping_add(0x224u64), (0x1u32) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; bb = 11; continue; }
            11 => { rax = 0x1; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14152c490(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142647278; const P1: u64 = 0x143930da0; const P2: u64 = 0x14152cd90; const P3: u64 = 0x143930da4; const P4: u64 = 0x143930da8; const P5: u64 = 0x14152c2e0; const P6: u64 = 0x14152c390; const P7: u64 = 0x14152d660; const P8: u64 = 0x143930dc8; const P9: u64 = 0x1426452b0; const P10: u64 = 0x14392e284; const P11: u64 = 0x14392e280; const P12: u64 = 0x14152d790; const P13: u64 = 0x142676a78; const P14: u64 = 0x1426469d0; const P15: u64 = 0x14152d7b0; const P16: u64 = 0x1426452a8; const P17: u64 = 0x143930df0; const P18: u64 = 0x1426472d0;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 21>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = rsp; rsp = ((rsp).wrapping_sub(0xa8u64)) as u64; crate::simd_ops::wu64(rax.wrapping_add(0x8u64), rbx); rdx = 0x2; crate::simd_ops::wu64(rax.wrapping_add(0x18u64), rsi); rsi = rcx; crate::simd_ops::wu64(rax.wrapping_sub(0x8u64), rdi); rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rax.wrapping_sub(0x10u64), r14); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r14 = 0; fl_l = 0; fl_r = 0; rcx = rsp.wrapping_add(0x30); rbx = ((rax as u32)) as u32 as u64; crate::simd_ops::wu32(P1, ((r14 as u32)) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru8(P3)) as u8 as u64; fl_r = ((r14 as u8)) as u8 as u64; rdi = (crate::simd_ops::ru64(P4)) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x80u64), ((rax as u32)) as u32); if (((fl_l as u8)) == ((fl_r as u8))) { bb = 2; continue; } bb = 1; continue; }
            1 => { fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 3; continue; }
            3 => { r10 = (crate::simd_ops::ru64(rdi)) as u64; r9 = rsp.wrapping_add(0x30); r8 = P6; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x101u32) as u32); rdx = 0x4; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(r10.wrapping_add(0x20u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rdi; fl_r = (crate::simd_ops::ru64(P4)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 5; continue; }
            5 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 12; continue; } bb = 6; continue; }
            6 => { crate::simd_ops::wu64(rsp.wrapping_add(0xb8u64), rbp); rcx = P8; crate::simd_ops::wu64(rsp.wrapping_add(0x90u64), r15); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P9), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = ((r14 as u32)) as u32 as u64; r15 = P10; rbp = P11; bb = 7; continue; }
            7 => { fl_l = (crate::simd_ops::ru32(P1)) as u32 as u64; fl_r = 0x14; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 11; continue; } bb = 8; continue; }
            8 => { rcx = ((rdi as u32)) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P12, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru32(P1)) as u32 as u64; r8 = P13; rcx = (((rax) as i64).wrapping_mul((0x228u64) as i64) as u64) as u64; rdx = 0x104; rcx = ((rcx).wrapping_add(r15)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P14), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru32(P1)) as u32 as u64; rdx = ((rdi.wrapping_add(0x30u64)) as u32) as u32 as u64; rcx = (((rax) as i64).wrapping_mul((0x228u64) as i64) as u64) as u64; xmm0 = 0u128; crate::simd_ops::wu16(rcx.wrapping_add(rbp).wrapping_add(0x2au64), ((rdx as u16)) as u16); rax = (crate::simd_ops::ru32(P1)) as u32 as u64; rcx = (((rax) as i64).wrapping_mul((0x228u64) as i64) as u64) as u64; crate::simd_ops::wu32(rcx.wrapping_add(rbp).wrapping_add(0x20cu64), (0x1u32) as u32); rax = (crate::simd_ops::ru32(P1)) as u32 as u64; rcx = (((rax) as i64).wrapping_mul((0x228u64) as i64) as u64) as u64; fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; rdx = (rdx & !0xffu64) | (((if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { 1u8 } else { 0u8 }) as u8 as u64)); crate::simd_ops::wu32(rcx.wrapping_add(rbp), ((rdi as u32)) as u32); rax = (crate::simd_ops::ru32(P1)) as u32 as u64; rcx = (((rax) as i64).wrapping_mul((0x228u64) as i64) as u64) as u64; crate::simd_ops::wu8(rcx.wrapping_add(rbp).wrapping_add(0x210u64), ((rdx as u8)) as u8); rax = (crate::simd_ops::ru32(P1)) as u32 as u64; rcx = (((rax) as i64).wrapping_mul((0x228u64) as i64) as u64) as u64; crate::simd_ops::wu128(rcx.wrapping_add(rbp).wrapping_add(0x214u64), (xmm0) as u128); rax = (crate::simd_ops::ru32(P1)) as u32 as u64; rdx = (((rax) as i64).wrapping_mul((0x228u64) as i64) as u64) as u64; crate::simd_ops::wu32(rdx.wrapping_add(rbp).wrapping_add(0x224u64), (0x2u32) as u32); fl_l = (crate::simd_ops::lk_add_u32(P1, (1) as u32)) as u32 as u64; bb = 10; continue; }
            10 => { rdi = (((rdi as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((rdi as u32)) as u32 as u64; fl_r = 0x4; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 7; continue; } bb = 11; continue; }
            11 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P15, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = P8; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P16), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0xb8u64))) as u64; bb = 12; continue; }
            12 => { rax = (crate::simd_ops::ru64(P17)) as u64; fl_l = rsi; fl_r = rsi; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0xa0u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0xc0u64))) as u64; rax = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { r14 } else { rax }) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x98u64))) as u64; fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0xb0u64))) as u64; crate::simd_ops::wu64(P17, rax); if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 14; continue; } bb = 13; continue; }
            13 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P18), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 14; continue; }
            14 => { rax = 0; fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0xa8u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14152c6b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14152b0f0; const P1: u64 = 0x14152b0c0; const P2: u64 = 0x14152b0b0; const P3: u64 = 0x142676938; const P4: u64 = 0x1405eba60; const P5: u64 = 0x142645710; const P6: u64 = 0x1426469c8; const P7: u64 = 0x142676968; const P8: u64 = 0x142676990;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 187>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), r8); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx); crate::simd_ops::wu32(rsp.wrapping_add(0x8u64), ((rcx as u32)) as u32); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rbp = rsp.wrapping_sub(0x4c0); rsp = ((rsp).wrapping_sub(0x5c0u64)) as u64; r15 = r8; rsi = rdx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = 0x2; rsp = ((rsp).wrapping_add(0x5c0u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            2 => { crate::simd_ops::wu64(rsp.wrapping_add(0x5b8u64), rbx); rbx = 0x1; crate::simd_ops::wu64(rsp.wrapping_add(0x5b0u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0x5a8u64), r12); r12 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x598u64), r14); r14 = ((r12 as u32)) as u32 as u64; rdi = ((rbx.wrapping_add(0x3u64)) as u32) as u32 as u64; bb = 3; continue; }
            3 => { r8 = 0; fl_l = 0; fl_r = 0; rdx = 0; fl_l = 0; fl_r = 0; rcx = ((rbx as u32)) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rcx = ((r14 as u32)) as u32 as u64; fl_l = (((r14 as u32)).wrapping_add(1)) as u32 as u64; r14 = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(rcx.wrapping_mul(4)).wrapping_add(0x30u64), ((rax as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(rcx.wrapping_mul(4)).wrapping_add(0x40u64), ((rbx as u32)) as u32); bb = 5; continue; }
            5 => { rbx = (((rbx as u32)).wrapping_add(1)) as u32 as u64; fl_l = rdi; fl_r = (0x1u64) as u64; rdi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 6; continue; }
            6 => { fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 30; continue; } bb = 7; continue; }
            7 => { rbx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x5a0u64), r13); rdi = ((rbx as u32)) as u32 as u64; r13 = ((rbx as u32)) as u32 as u64; fl_l = ((r14 as u32)) as u32 as u64; fl_r = ((r14 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 29; continue; } bb = 8; continue; }
            8 => { r15 = ((rbx as u32)) as u32 as u64; bb = 9; continue; }
            9 => { r12 = (crate::simd_ops::ru32(rsp.wrapping_add(r15).wrapping_add(0x30u64))) as u32 as u64; rdx = rbp.wrapping_add(0x4f8); rcx = ((r12 as u32)) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 27; continue; } bb = 10; continue; }
            10 => { rcx = (crate::simd_ops::ru32(rbp.wrapping_add(0x4f8u64))) as u32 as u64; rsi = rsp.wrapping_add(0x40); fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 16; continue; } bb = 11; continue; }
            11 => { r9 = (crate::simd_ops::ru32(rsi.wrapping_add(r15))) as u32 as u64; r8 = P3; rdx = 0x104; crate::simd_ops::wu32(rbp.wrapping_add(0x15cu64), ((rbx as u32)) as u32); rcx = rbp.wrapping_add(0x170); crate::simd_ops::wu8(rbp.wrapping_add(0x160u64), ((rax as u8)) as u8); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = rbp.wrapping_add(0x280); crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0x104u32) as u32); r9 = 0xffffffff; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); r8 = rbp.wrapping_add(0x170); rdx = 0; fl_l = 0; fl_r = 0; rcx = 0xfde9; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu16(rbp.wrapping_add(0x486u64), ((rbx as u16)) as u16); rax = rbp.wrapping_add(0x280); rbx = (0xffffffffffffffffu64) as u64; bb = 12; continue; }
            12 => { rbx = ((rbx).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru16(rax.wrapping_add(rbx.wrapping_mul(2)))) as u16 as u64; fl_r = (0x0u16) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 12; continue; } bb = 13; continue; }
            13 => { rax = 0x103; r8 = rbp.wrapping_add(0x280); rbx = ((rbx).wrapping_add(1)) as u64; rcx = rsp.wrapping_add(0x54); fl_l = rbx; fl_r = rax; rbx = (if (((fl_l as u64)) > ((fl_r as u64))) { rax } else { rbx }) as u64; rdx = ((rax.wrapping_add(0x1u64)) as u32) as u32 as u64; r9 = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = 0; fl_l = 0; fl_r = 0; rcx = 0x4; crate::simd_ops::wu16(rsp.wrapping_add(rbx.wrapping_mul(2)).wrapping_add(0x54u64), ((rax as u16)) as u16); rax = ((r12 as u32)) as u32 as u64; fl_l = ((((rax as u32)) >> (31u32)) & 1) as u64; rax = (((rax as u32)) | ((1u32) << (31u32))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x15cu64), (0x1u32) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x50u64), ((rax as u32)) as u32); rax = ((rdi as u32)) as u32 as u64; rdx = (((rax) as i64).wrapping_mul((0x214u64) as i64) as u64) as u64; crate::simd_ops::wu8(rbp.wrapping_add(0x160u64), (0x0u8) as u8); rax = rsp.wrapping_add(0x50); fl_l = ((rdx).wrapping_add(crate::simd_ops::ru64(rbp.wrapping_add(0x4e8u64)))) as u64; fl_r = fl_l; rdx = ((fl_l as u64)) as u64; bb = 14; continue; }
            14 => { rdx = rdx.wrapping_add(0x80); xmm0 = (crate::simd_ops::ru128(rax)) as u128; xmm1 = (crate::simd_ops::ru128(rax.wrapping_add(0x10u64))) as u128; rax = rax.wrapping_add(0x80); crate::simd_ops::wu128(rdx.wrapping_sub(0x80u64), (xmm0) as u128); xmm0 = (crate::simd_ops::ru128(rax.wrapping_sub(0x60u64))) as u128; crate::simd_ops::wu128(rdx.wrapping_sub(0x70u64), (xmm1) as u128); xmm1 = (crate::simd_ops::ru128(rax.wrapping_sub(0x50u64))) as u128; crate::simd_ops::wu128(rdx.wrapping_sub(0x60u64), (xmm0) as u128); xmm0 = (crate::simd_ops::ru128(rax.wrapping_sub(0x40u64))) as u128; crate::simd_ops::wu128(rdx.wrapping_sub(0x50u64), (xmm1) as u128); xmm1 = (crate::simd_ops::ru128(rax.wrapping_sub(0x30u64))) as u128; crate::simd_ops::wu128(rdx.wrapping_sub(0x40u64), (xmm0) as u128); xmm0 = (crate::simd_ops::ru128(rax.wrapping_sub(0x20u64))) as u128; crate::simd_ops::wu128(rdx.wrapping_sub(0x30u64), (xmm1) as u128); xmm1 = (crate::simd_ops::ru128(rax.wrapping_sub(0x10u64))) as u128; crate::simd_ops::wu128(rdx.wrapping_sub(0x20u64), (xmm0) as u128); crate::simd_ops::wu128(rdx.wrapping_sub(0x10u64), (xmm1) as u128); fl_l = rcx; fl_r = (0x1u64) as u64; rcx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 14; continue; } bb = 15; continue; }
            15 => { xmm0 = (crate::simd_ops::ru128(rax)) as u128; rax = (crate::simd_ops::ru32(rax.wrapping_add(0x10u64))) as u32 as u64; rdi = (((rdi as u32)).wrapping_add(1)) as u32 as u64; rcx = (crate::simd_ops::ru32(rbp.wrapping_add(0x4f8u64))) as u32 as u64; rbx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu128(rdx, (xmm0) as u128); crate::simd_ops::wu32(rdx.wrapping_add(0x10u64), ((rax as u32)) as u32); bb = 16; continue; }
            16 => { fl_l = ((rdi as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rbp.wrapping_add(0x4e0u64))) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 28; continue; } bb = 17; continue; }
            17 => { rax = ((rcx.wrapping_sub(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 27; continue; } bb = 18; continue; }
            18 => { crate::simd_ops::wu32(rbp.wrapping_add(0x15cu64), ((rbx as u32)) as u32); crate::simd_ops::wu8(rbp.wrapping_add(0x160u64), (0x0u8) as u8); fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 20; continue; } bb = 19; continue; }
            19 => { r9 = (crate::simd_ops::ru32(rsi.wrapping_add(r15))) as u32 as u64; r8 = P7; rdx = 0x104; rcx = rbp.wrapping_add(0x170); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru32(rbp.wrapping_add(0x4f8u64))) as u32 as u64; bb = 20; continue; }
            20 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 22; continue; } bb = 21; continue; }
            21 => { r9 = (crate::simd_ops::ru32(rsi.wrapping_add(r15))) as u32 as u64; r8 = P8; rdx = 0x104; rcx = rbp.wrapping_add(0x170); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 22; continue; }
            22 => { rax = rbp.wrapping_add(0x280); crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0x104u32) as u32); r9 = 0xffffffff; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); r8 = rbp.wrapping_add(0x170); rdx = 0; fl_l = 0; fl_r = 0; rcx = 0xfde9; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu16(rbp.wrapping_add(0x486u64), ((rbx as u16)) as u16); rax = rbp.wrapping_add(0x280); rbx = (0xffffffffffffffffu64) as u64; bb = 23; continue; }
            23 => { rbx = ((rbx).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru16(rax.wrapping_add(rbx.wrapping_mul(2)))) as u16 as u64; fl_r = (0x0u16) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 23; continue; } bb = 24; continue; }
            24 => { rax = 0x103; r8 = rbp.wrapping_add(0x280); rbx = ((rbx).wrapping_add(1)) as u64; rcx = rsp.wrapping_add(0x54); fl_l = rbx; fl_r = rax; rbx = (if (((fl_l as u64)) > ((fl_r as u64))) { rax } else { rbx }) as u64; rdx = ((rax.wrapping_add(0x1u64)) as u32) as u32 as u64; r9 = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = 0; fl_l = 0; fl_r = 0; rcx = 0x4; crate::simd_ops::wu16(rsp.wrapping_add(rbx.wrapping_mul(2)).wrapping_add(0x54u64), ((rax as u16)) as u16); crate::simd_ops::wu8(rbp.wrapping_add(0x160u64), ((rax as u8)) as u8); rax = ((rdi as u32)) as u32 as u64; rdx = (((rax) as i64).wrapping_mul((0x214u64) as i64) as u64) as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x15cu64), (0x1u32) as u32); rax = rsp.wrapping_add(0x50); fl_l = ((rdx).wrapping_add(crate::simd_ops::ru64(rbp.wrapping_add(0x4e8u64)))) as u64; fl_r = fl_l; rdx = ((fl_l as u64)) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x50u64), ((r12 as u32)) as u32); bb = 25; continue; }
            25 => { rdx = rdx.wrapping_add(0x80); xmm0 = (crate::simd_ops::ru128(rax)) as u128; xmm1 = (crate::simd_ops::ru128(rax.wrapping_add(0x10u64))) as u128; rax = rax.wrapping_add(0x80); crate::simd_ops::wu128(rdx.wrapping_sub(0x80u64), (xmm0) as u128); xmm0 = (crate::simd_ops::ru128(rax.wrapping_sub(0x60u64))) as u128; crate::simd_ops::wu128(rdx.wrapping_sub(0x70u64), (xmm1) as u128); xmm1 = (crate::simd_ops::ru128(rax.wrapping_sub(0x50u64))) as u128; crate::simd_ops::wu128(rdx.wrapping_sub(0x60u64), (xmm0) as u128); xmm0 = (crate::simd_ops::ru128(rax.wrapping_sub(0x40u64))) as u128; crate::simd_ops::wu128(rdx.wrapping_sub(0x50u64), (xmm1) as u128); xmm1 = (crate::simd_ops::ru128(rax.wrapping_sub(0x30u64))) as u128; crate::simd_ops::wu128(rdx.wrapping_sub(0x40u64), (xmm0) as u128); xmm0 = (crate::simd_ops::ru128(rax.wrapping_sub(0x20u64))) as u128; crate::simd_ops::wu128(rdx.wrapping_sub(0x30u64), (xmm1) as u128); xmm1 = (crate::simd_ops::ru128(rax.wrapping_sub(0x10u64))) as u128; crate::simd_ops::wu128(rdx.wrapping_sub(0x20u64), (xmm0) as u128); crate::simd_ops::wu128(rdx.wrapping_sub(0x10u64), (xmm1) as u128); fl_l = rcx; fl_r = (0x1u64) as u64; rcx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 25; continue; } bb = 26; continue; }
            26 => { xmm0 = (crate::simd_ops::ru128(rax)) as u128; rcx = (crate::simd_ops::ru32(rax.wrapping_add(0x10u64))) as u32 as u64; rdi = (((rdi as u32)).wrapping_add(1)) as u32 as u64; crate::simd_ops::wu128(rdx, (xmm0) as u128); crate::simd_ops::wu32(rdx.wrapping_add(0x10u64), ((rcx as u32)) as u32); fl_l = ((rdi as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rbp.wrapping_add(0x4e0u64))) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 28; continue; } bb = 27; continue; }
            27 => { r13 = (((r13 as u32)).wrapping_add(1)) as u32 as u64; r15 = ((r15).wrapping_add(0x4u64)) as u64; rbx = 0x0; fl_l = ((r13 as u32)) as u32 as u64; fl_r = ((r14 as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 9; continue; } bb = 28; continue; }
            28 => { r15 = (crate::simd_ops::ru64(rbp.wrapping_add(0x4f0u64))) as u64; bb = 29; continue; }
            29 => { r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x5a0u64))) as u64; bb = 38; continue; }
            30 => { rdi = ((r12 as u32)) as u32 as u64; fl_l = ((r14 as u32)) as u32 as u64; fl_r = ((r14 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 38; continue; } bb = 31; continue; }
            31 => { rbx = rsp.wrapping_add(0x30); rsi = ((r14 as u32)) as u32 as u64; bb = 32; continue; }
            32 => { rcx = (crate::simd_ops::ru32(rbx)) as u32 as u64; rdx = rbp.wrapping_add(0x4f8); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 37; continue; } bb = 33; continue; }
            33 => { rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x4f8u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 35; continue; } bb = 34; continue; }
            34 => { fl_l = (((rdi as u32)).wrapping_add((rax as u32))) as u32 as u64; fl_r = fl_l; rdi = ((fl_l as u32)) as u32 as u64; bb = 37; continue; }
            35 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 37; continue; } bb = 36; continue; }
            36 => { fl_l = (((rdi as u32)).wrapping_add(1)) as u32 as u64; rdi = ((fl_l as u32)) as u32 as u64; bb = 37; continue; }
            37 => { rbx = ((rbx).wrapping_add(0x4u64)) as u64; fl_l = rsi; fl_r = (0x1u64) as u64; rsi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 32; continue; } bb = 38; continue; }
            38 => { crate::simd_ops::wu32(r15, ((rdi as u32)) as u32); rax = 0x1; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x598u64))) as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x5a8u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x5b0u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x5b8u64))) as u64; rsp = ((rsp).wrapping_add(0x5c0u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14152cb20(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143930da4; const P1: u64 = 0x14152c490; const P2: u64 = 0x14152c6b0; const P3: u64 = 0x143930dc8; const P4: u64 = 0x1426452b0; const P5: u64 = 0x143930da0; const P6: u64 = 0x14392e280; const P7: u64 = 0x1426452a8;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; fl_l = (crate::simd_ops::ru8(P0)) as u8 as u64; fl_r = (0x0u8) as u8 as u64; rsi = rdx; rdi = rcx; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = 0x1; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru32(rdi)) as u32 as u64; r8 = rsp.wrapping_add(0x38); rdx = rsi; crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), (0x0u32) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = (crate::simd_ops::ru32(rsp.wrapping_add(0x38u64))) as u32 as u64; rcx = P3; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru32(P5)) as u32 as u64; fl_l = (((rcx as u32)).wrapping_add((rbx as u32))) as u32 as u64; fl_r = fl_l; rcx = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(rdi, ((rcx as u32)) as u32); bb = 11; continue; }
            4 => { fl_l = (crate::simd_ops::ru32(P5)) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 10; continue; } bb = 5; continue; }
            5 => { r9 = ((rbx as u32)) as u32 as u64; r10 = P6; fl_l = 0; fl_r = ((r9 as u32)) as u32 as u64; r9 = (0u32.wrapping_sub(fl_r as u32)) as u32 as u64; bb = 6; continue; }
            6 => { fl_l = ((rbx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rdi)) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 10; continue; } bb = 7; continue; }
            7 => { rcx = ((r9.wrapping_add(rbx)) as u32) as u32 as u64; rax = ((rbx as u32)) as u32 as u64; rdx = (((rcx) as i64).wrapping_mul((0x228u64) as i64) as u64) as u64; r8 = (((rax) as i64).wrapping_mul((0x214u64) as i64) as u64) as u64; rax = ((rdx.wrapping_add(r10)) as u64) as u64; rcx = 0x4; fl_l = ((r8).wrapping_add(rsi)) as u64; fl_r = fl_l; r8 = ((fl_l as u64)) as u64; bb = 8; continue; }
            8 => { r8 = r8.wrapping_add(0x80); xmm0 = (crate::simd_ops::ru128(rax)) as u128; rax = rax.wrapping_add(0x80); crate::simd_ops::wu128(r8.wrapping_sub(0x80u64), (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(rax.wrapping_sub(0x70u64))) as u128; crate::simd_ops::wu128(r8.wrapping_sub(0x70u64), (xmm1) as u128); xmm0 = (crate::simd_ops::ru128(rax.wrapping_sub(0x60u64))) as u128; crate::simd_ops::wu128(r8.wrapping_sub(0x60u64), (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(rax.wrapping_sub(0x50u64))) as u128; crate::simd_ops::wu128(r8.wrapping_sub(0x50u64), (xmm1) as u128); xmm0 = (crate::simd_ops::ru128(rax.wrapping_sub(0x40u64))) as u128; crate::simd_ops::wu128(r8.wrapping_sub(0x40u64), (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(rax.wrapping_sub(0x30u64))) as u128; crate::simd_ops::wu128(r8.wrapping_sub(0x30u64), (xmm1) as u128); xmm0 = (crate::simd_ops::ru128(rax.wrapping_sub(0x20u64))) as u128; crate::simd_ops::wu128(r8.wrapping_sub(0x20u64), (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(rax.wrapping_sub(0x10u64))) as u128; crate::simd_ops::wu128(r8.wrapping_sub(0x10u64), (xmm1) as u128); fl_l = rcx; fl_r = (0x1u64) as u64; rcx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 8; continue; } bb = 9; continue; }
            9 => { xmm0 = (crate::simd_ops::ru128(rax)) as u128; rbx = (((rbx as u32)).wrapping_add(1)) as u32 as u64; crate::simd_ops::wu128(r8, (xmm0) as u128); rcx = (crate::simd_ops::ru32(rax.wrapping_add(0x10u64))) as u32 as u64; rax = ((r9.wrapping_add(rbx)) as u32) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(0x10u64), ((rcx as u32)) as u32); fl_l = ((rax as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(P5)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 6; continue; } bb = 10; continue; }
            10 => { rdx = (crate::simd_ops::ru32(rdi)) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; rdx = (if (((fl_l as u32)) >= ((fl_r as u32))) { (rbx as u32) } else { (rdx as u32) }) as u32 as u64; crate::simd_ops::wu32(rdi, ((rdx as u32)) as u32); bb = 11; continue; }
            11 => { rcx = P3; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P7), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = 0x1; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14152cc80(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143930dc8; const P1: u64 = 0x1426452b0; const P2: u64 = 0x143930da0; const P3: u64 = 0x14392e280; const P4: u64 = 0x14152b0f0; const P5: u64 = 0x14152b0b0; const P6: u64 = 0x1426452a8; const P7: u64 = 0x143930df0;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rcx; rdi = rdx; rcx = P0; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); r9 = (crate::simd_ops::ru32(P2)) as u32 as u64; r8 = 0; fl_l = 0; fl_r = 0; fl_l = ((r9 as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { r10 = (crate::simd_ops::ru32(rbx)) as u32 as u64; rdx = P3; bb = 2; continue; }
            2 => { rax = ((r8 as u32)) as u32 as u64; rcx = (((rax) as i64).wrapping_mul((0x228u64) as i64) as u64) as u64; fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(rdx))) as u32 as u64; fl_r = ((r10 as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 8; continue; } bb = 3; continue; }
            3 => { r8 = (((r8 as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 2; continue; } bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::ru32(rbx)) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 9; continue; } bb = 5; continue; }
            5 => { rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 9; continue; } bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru32(rbx)) as u32 as u64; rdx = rsp.wrapping_add(0x48); fl_l = ((((rcx as u32)) >> (31u32)) & 1) as u64; rcx = (((rcx as u32)) & !((1u32) << (31u32))) as u32 as u64; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 9; continue; } bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; rcx = P0; rax = (((((rax as u32)) as i32).wrapping_shr(31u32) as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_add(0x4u32)) as u32 as u64; crate::simd_ops::wu32(rdi, ((rax as u32)) as u32); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp); rax = 0x1; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            8 => { rax = (crate::simd_ops::ru32(rcx.wrapping_add(rdx).wrapping_add(0x224u64))) as u32 as u64; rcx = P0; crate::simd_ops::wu32(rdi, ((rax as u32)) as u32); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp); rax = 0x1; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            9 => { fl_l = (crate::simd_ops::ru64(P7)) as u64; fl_r = (0x0u64) as u64; rcx = P0; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 11; continue; } bb = 10; continue; }
            10 => { crate::simd_ops::wu32(rbx, (0x0u32) as u32); crate::simd_ops::wu32(rdi, (0x0u32) as u32); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp); rax = 0x1; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            11 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = 0x2; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14152cd90(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14183acc2; const P1: u64 = 0x142676a08; const P2: u64 = 0x1426769f8; const P3: u64 = 0x142647298; const P4: u64 = 0x142676a18; const P5: u64 = 0x142645ce0; const P6: u64 = 0x142676a38; const P7: u64 = 0x142676a58; const P8: u64 = 0x142647270; const P9: u64 = 0x1426769b8; const P10: u64 = 0x14183ace0; const P11: u64 = 0x1426769c8; const P12: u64 = 0x14152d460; const P13: u64 = 0x1426769d8; const P14: u64 = 0x1426769e8; const P15: u64 = 0x142676a70; const P16: u64 = 0x142645cf0; const P17: u64 = 0x142645ce8;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 45>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = rsp; rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rbp = rax.wrapping_sub(0x68); rsp = ((rsp).wrapping_sub(0x158u64)) as u64; crate::simd_ops::wu64(rax.wrapping_add(0x8u64), rbx); rdx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rax.wrapping_sub(0x20u64), r12); r8 = 0xa0; crate::simd_ops::wu64(rax.wrapping_sub(0x28u64), r13); r12 = rcx; crate::simd_ops::wu64(rax.wrapping_sub(0x30u64), r14); rcx = rbp.wrapping_sub(0x70); r14 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rax.wrapping_sub(0x38u64), r15); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), r14); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), r14); rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rax = rsp.wrapping_add(0x58); crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), r14); r9 = P1; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); rdx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rbp.wrapping_add(0x80u64), ((r14 as u32)) as u32); r8 = ((r14.wrapping_add(0x1u64)) as u32) as u32 as u64; r13 = ((r14 as u32)) as u32 as u64; rcx = P2; rdi = ((r14 as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); r15 = ((r14.wrapping_add(0x14u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 38; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 38; continue; } bb = 2; continue; }
            2 => { rcx = P4; crate::simd_ops::wu64(rsp.wrapping_add(0x150u64), rsi); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), rax); rsi = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 37; continue; } bb = 3; continue; }
            3 => { rcx = P6; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), rax); rbx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 33; continue; } bb = 4; continue; }
            4 => { rcx = P7; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); r13 = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 33; continue; } bb = 5; continue; }
            5 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rax = rsp.wrapping_add(0x60); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rax); r9 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), r14); r8 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), r14); rdx = rsi; r10 = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((r14 as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r14); rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(r10.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 33; continue; } bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 33; continue; } bb = 7; continue; }
            7 => { crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), ((r14 as u32)) as u32); rdx = ((r14.wrapping_add(0xau64)) as u32) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), r14); r9 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0x3u32) as u32); r8 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x3u32) as u32); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P8), rax), rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rdx = rsp.wrapping_add(0x50); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdx); r9 = 0; fl_l = 0; fl_r = 0; r8 = 0; fl_l = 0; fl_r = 0; rdx = rbx; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x90u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 33; continue; } bb = 8; continue; }
            8 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 33; continue; } bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = rbp.wrapping_add(0x80); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdx); r9 = rbp.wrapping_sub(0x70); rdx = 0x2710; r8 = ((r15 as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x20u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 33; continue; } bb = 10; continue; }
            10 => { bb = 11; continue; }
            11 => { rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x80u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 32; continue; } bb = 12; continue; }
            12 => { rsi = ((r14 as u32)) as u32 as u64; bb = 13; continue; }
            13 => { rax = ((rsi as u32)) as u32 as u64; rbx = rbp.wrapping_sub(0x70); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(rax.wrapping_mul(8)))) as u64; rbx = ((rbx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), r14); r9 = rsp.wrapping_add(0x78); r8 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r14); rdx = r13; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x20u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rbx)) as u64; r14 = ((rax as u32)) as u32 as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 15; continue; } bb = 14; continue; }
            14 => { rdx = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rdx.wrapping_add(0x10u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx, (0x0u64) as u64); bb = 15; continue; }
            15 => { fl_l = ((r14 as u32)) as u32 as u64; fl_r = ((r14 as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 28; continue; } bb = 16; continue; }
            16 => { r14 = 0; fl_l = 0; fl_r = 0; fl_l = (crate::simd_ops::ru16(rsp.wrapping_add(0x78u64))) as u16 as u64; fl_r = (0x8u16) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 29; continue; } bb = 17; continue; }
            17 => { rbx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x80u64))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 29; continue; } bb = 18; continue; }
            18 => { rdx = P9; crate::simd_ops::wu32(rbp.wrapping_add(0x88u64), ((r14 as u32)) as u32); rcx = rbx; crate::simd_ops::wu32(rbp.wrapping_add(0x78u64), ((r14 as u32)) as u32); rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 20; continue; } bb = 19; continue; }
            19 => { r8 = rbp.wrapping_add(0x78); rcx = rax; rdx = P11; rax = crate::simd_ops::call_stk(P12, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru32(rbp.wrapping_add(0x78u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; rdx = (if (((fl_l as u32)) != ((fl_r as u32))) { (r14 as u32) } else { (rdx as u32) }) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x78u64), ((rdx as u32)) as u32); bb = 20; continue; }
            20 => { rdx = P13; rcx = rbx; rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 22; continue; } bb = 21; continue; }
            21 => { r8 = rbp.wrapping_add(0x88); rcx = rax; rdx = P14; rax = crate::simd_ops::call_stk(P12, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru32(rbp.wrapping_add(0x88u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; rcx = (if (((fl_l as u32)) != ((fl_r as u32))) { (r14 as u32) } else { (rcx as u32) }) as u32 as u64; bb = 23; continue; }
            22 => { rcx = (crate::simd_ops::ru32(rbp.wrapping_add(0x88u64))) as u32 as u64; bb = 23; continue; }
            23 => { rax = ((crate::simd_ops::ru16(rbp.wrapping_add(0x78u64))) as u32) as u32 as u64; rdx = P15; rbx = (((rcx as u16)) as u32) as u32 as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x80u64))) as u64; rbx = (((rbx as u32)).wrapping_shl(16u32)) as u32 as u64; rbx = ((((rbx as u32)) | ((rax as u32)))) as u32 as u64; rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 25; continue; } bb = 24; continue; }
            24 => { rax = ((rdi as u32)) as u32 as u64; fl_l = (((rdi as u32)).wrapping_add(1)) as u32 as u64; rdi = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(r12.wrapping_add(rax.wrapping_mul(4)), ((rbx as u32)) as u32); bb = 25; continue; }
            25 => { fl_l = ((rbx as u32)) as u32 as u64; fl_r = 0x11ff28de; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 27; continue; } bb = 26; continue; }
            26 => { crate::simd_ops::wu64(rbp.wrapping_sub(0x80u64), r14); bb = 29; continue; }
            27 => { rcx = rsp.wrapping_add(0x78); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P16), rax), rax, rcx, rdx, r8, r9, rsp); bb = 29; continue; }
            28 => { r14 = 0; fl_l = 0; fl_r = 0; bb = 29; continue; }
            29 => { fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((r15 as u32)) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 32; continue; } bb = 30; continue; }
            30 => { rsi = (((rsi as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((rsi as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rbp.wrapping_add(0x80u64))) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 13; continue; } bb = 31; continue; }
            31 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rdx = rbp.wrapping_add(0x80); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdx); r9 = rbp.wrapping_sub(0x70); r8 = ((r15 as u32)) as u32 as u64; rdx = 0x2710; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x20u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) >= 0) { bb = 11; continue; } bb = 32; continue; }
            32 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; bb = 33; continue; }
            33 => { rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P17), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = r13; fl_r = r13; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 35; continue; } bb = 34; continue; }
            34 => { rcx = r13; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P17), rax), rax, rcx, rdx, r8, r9, rsp); bb = 35; continue; }
            35 => { fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 37; continue; } bb = 36; continue; }
            36 => { rcx = rbx; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P17), rax), rax, rcx, rdx, r8, r9, rsp); bb = 37; continue; }
            37 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x150u64))) as u64; bb = 38; continue; }
            38 => { r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x140u64))) as u64; rbx = rbp.wrapping_sub(0x70); r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x148u64))) as u64; bb = 39; continue; }
            39 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 41; continue; } bb = 40; continue; }
            40 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx, r14); bb = 41; continue; }
            41 => { rbx = ((rbx).wrapping_add(0x8u64)) as u64; fl_l = r15; fl_r = (0x1u64) as u64; r15 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 39; continue; } bb = 42; continue; }
            42 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x130u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x170u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 44; continue; } bb = 43; continue; }
            43 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), r14); bb = 44; continue; }
            44 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 46; continue; } bb = 45; continue; }
            45 => { rdx = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rdx.wrapping_add(0x10u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), r14); bb = 46; continue; }
            46 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x138u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 48; continue; } bb = 47; continue; }
            47 => { rdx = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rdx.wrapping_add(0x10u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 48; continue; }
            48 => { rax = ((rdi as u32)) as u32 as u64; rsp = ((rsp).wrapping_add(0x158u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14152d190(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143930da4; const P1: u64 = 0x143930da0; const P2: u64 = 0x14152c2e0; const P3: u64 = 0x143930da8; const P4: u64 = 0x14392e280; const P5: u64 = 0x14152d1e0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    fl_l = (crate::simd_ops::ru8(P0)) as u8 as u64; fl_r = (0x0u8) as u8 as u64;
    if (((fl_l as u8)) != ((fl_r as u8))) { rax = 0x1; rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
    crate::simd_ops::wu32(P1, (0x0u32) as u32);
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    crate::simd_ops::wu64(P3, rax);
    fl_l = rax; fl_r = rax;
    if !(((((fl_l as u64)) & ((fl_r as u64))) != 0)) { rax = 0x2; rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
    rcx = P4;
    crate::simd_ops::wu8(P0, (0x1u8) as u8);
    rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp);
    rax = 0x1;
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14152d1e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x2b24; const P1: u64 = 0x2b70; const P2: u64 = 0x1426452b0; const P3: u64 = 0x2b48; const P4: u64 = 0x2b20; const P5: u64 = 0x1426452a8; const P6: u64 = 0x14152c490; const P7: u64 = 0x1426456a0; const P8: u64 = 0x142676920; const P9: u64 = 0x14152c1e0; const P10: u64 = 0x1426456b0; const P11: u64 = 0x1426455b0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(P0))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; rdi = rcx; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 7; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 7; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rbx); rcx = ((rcx).wrapping_add(0x2b48u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rsi); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); rsi = 0; fl_l = 0; fl_r = 0; rcx = ((rdi.wrapping_add(P3)) as u64) as u64; crate::simd_ops::wu32(rdi.wrapping_add(P4), ((rsi as u32)) as u32); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); rax = rsp.wrapping_add(0x40); r9 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); r8 = P6; rdx = 0x20000; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rsi as u32)) as u32); rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P7), rax), rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; crate::simd_ops::wu64(rdi.wrapping_add(P1), rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 3; continue; }
            3 => { rdx = (crate::simd_ops::ru32(rsp.wrapping_add(0x40u64))) as u32 as u64; r8 = P8; rcx = rax; rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P1))) as u64; rdx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P10), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 6; continue; } bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P1))) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P11), rax), rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            5 => { crate::simd_ops::wu64(rdi.wrapping_add(0x2b70u64), rsi); bb = 6; continue; }
            6 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; bb = 7; continue; }
            7 => { rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14152d2b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143930db8; const P1: u64 = 0x143930db0; const P2: u64 = 0x4; const P3: u64 = 0x1426460c8; const P4: u64 = 0x14152d3e0; const P5: u64 = 0x143930dc0; const P6: u64 = 0x1426460a8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x40u64)) as u64;
    fl_l = (crate::simd_ops::ru64(P0)) as u64; fl_r = (0x0u64) as u64;
    rbx = rcx;
    if (((fl_l as u64)) != ((fl_r as u64))) { rsp = ((rsp).wrapping_add(0x40u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
    fl_l = (crate::simd_ops::ru64(P1)) as u64; fl_r = (0x0u64) as u64;
    if !((((fl_l as u64)) != ((fl_r as u64)))) { rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x24u64), (0x5u64) as u64); xmm0 = 0u128; crate::simd_ops::wu32(rsp.wrapping_add(0x3cu64), ((rax as u32)) as u32); rdx = rsp.wrapping_add(0x20); crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x20u32) as u32); crate::simd_ops::wu128(rsp.wrapping_add(0x2cu64), (xmm0) as u128); r8 = ((rax.wrapping_add(P2)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = P4; rdx = 0xfffffffc; rcx = rbx; crate::simd_ops::wu64(P5, rax); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(P0, rax); crate::simd_ops::wu64(P1, rbx); }
    rsp = ((rsp).wrapping_add(0x40u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14152d330(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143930dc0; const P1: u64 = 0x1426460c0; const P2: u64 = 0x143930df0; const P3: u64 = 0x142645748; const P4: u64 = 0x143930da8; const P5: u64 = 0x10; const P6: u64 = 0x143930db0; const P7: u64 = 0x1426460b0; const P8: u64 = 0x14152d3e0; const P9: u64 = 0x143930db8; const P10: u64 = 0x1426460a8; const P11: u64 = 0x143930da4; const P12: u64 = 0x143930da0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rcx = (crate::simd_ops::ru64(P0)) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(P2)) as u64; rbx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(P0, rbx); fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rdx = 0xffffffff; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(P2, rbx); bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru64(P4)) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P5))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(P4, rbx); bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru64(P6)) as u64; rdx = 0xfffffffc; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P7), rax), rax, rcx, rdx, r8, r9, rsp); rcx = P8; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { r8 = (crate::simd_ops::ru64(P9)) as u64; rdx = 0xfffffffc; rcx = (crate::simd_ops::ru64(P6)) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P10), rax), rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(P9, rbx); crate::simd_ops::wu64(P6, rbx); bb = 6; continue; }
            6 => { crate::simd_ops::wu8(P11, ((rbx as u8)) as u8); rax = 0x1; crate::simd_ops::wu32(P12, ((rbx as u32)) as u32); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14152d3e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143930da4; const P1: u64 = 0xffffffffffff8000; const P2: u64 = 0x14392e280; const P3: u64 = 0x14152d1e0; const P4: u64 = 0x143930db8; const P5: u64 = 0x1426460b8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp);
    crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x30u64)) as u64;
    fl_l = (crate::simd_ops::ru8(P0)) as u8 as u64; fl_r = (0x0u8) as u8 as u64;
    rsi = r9;
    rdi = r8;
    rbx = ((rdx as u32)) as u32 as u64;
    rbp = rcx;
    if (((fl_l as u8)) == ((fl_r as u8))) { rcx = (crate::simd_ops::ru64(P4)) as u64; r9 = rdi; r8 = ((rbx as u32)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); rdx = rbp; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x219;
    if (((fl_l as u32)) != ((fl_r as u32))) { rcx = (crate::simd_ops::ru64(P4)) as u64; r9 = rdi; r8 = ((rbx as u32)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); rdx = rbp; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    rax = ((r8.wrapping_add(P1)) as u64) as u64;
    fl_l = rax; fl_r = (0xfffffffffffffffbu64) as u64;
    if !(((((fl_l as u64)) & ((fl_r as u64))) != 0)) { rcx = P2; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); }
    rcx = (crate::simd_ops::ru64(P4)) as u64;
    r9 = rdi;
    r8 = ((rbx as u32)) as u32 as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi);
    rdx = rbp;
    rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64;
    rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64;
    rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64;
    rsp = ((rsp).wrapping_add(0x30u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14152d460(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1405503f0; const P1: u64 = 0x142646888;
    let (mut rax, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), r8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r9);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rsi);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x30u64)) as u64;
    rbx = rdx;
    rsi = rsp.wrapping_add(0x60);
    rdi = rcx;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rsi);
    r9 = rbx;
    r8 = (0xffffffffffffffffu64) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (0x0u64) as u64);
    rdx = rdi;
    rcx = (crate::simd_ops::ru64(rax)) as u64;
    rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x30u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    rsi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14152d500(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143930e08; const P1: u64 = 0x143930e10; const P2: u64 = 0x142676b68; const P3: u64 = 0x142676b88; const P4: u64 = 0x142676ba8; const P5: u64 = 0x142645728; const P6: u64 = 0x142645358; const P7: u64 = 0x142676bc8; const P8: u64 = 0x142645758; const P9: u64 = 0x142676be0; const P10: u64 = 0x143930df8; const P11: u64 = 0x143930e00; const P12: u64 = 0x14152e390;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 15>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x50u64)) as u64; rax = (crate::simd_ops::ru64(P0)) as u64; r14 = r9; r15 = r8; r12 = rdx; r13 = rcx; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::lk_add_u32(P1, (1) as u32)) as u32 as u64; bb = 15; continue; }
            2 => { rcx = P2; rbp = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rcx); rdi = rsp.wrapping_add(0x30); rcx = P3; rbx = ((rbp as u32)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rcx); rcx = P4; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rcx); bb = 3; continue; }
            3 => { fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 14; continue; } bb = 4; continue; }
            4 => { rsi = (crate::simd_ops::ru64(rdi)) as u64; rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(P0, rax); bb = 7; continue; }
            6 => { rax = (crate::simd_ops::ru64(P0)) as u64; bb = 7; continue; }
            7 => { rbx = (((rbx as u32)).wrapping_add(1)) as u32 as u64; rdi = ((rdi).wrapping_add(0x8u64)) as u64; fl_l = ((rbx as u32)) as u32 as u64; fl_r = 0x3; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 3; continue; } bb = 8; continue; }
            8 => { fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 14; continue; } bb = 9; continue; }
            9 => { rbx = rsp.wrapping_add(0x30); bb = 10; continue; }
            10 => { fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 14; continue; } bb = 11; continue; }
            11 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp); rbp = (((rbp as u32)).wrapping_add(1)) as u32 as u64; rbx = ((rbx).wrapping_add(0x8u64)) as u64; crate::simd_ops::wu64(P0, rax); fl_l = ((rbp as u32)) as u32 as u64; fl_r = 0x3; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 10; continue; } bb = 12; continue; }
            12 => { fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 14; continue; } bb = 13; continue; }
            13 => { rax = 0x54; bb = 16; continue; }
            14 => { crate::simd_ops::lk_add_u32(P1, (1) as u32); rdx = P7; rcx = rax; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P8), rax), rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(P0)) as u64; rdx = P9; crate::simd_ops::wu64(P10, rax); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P8), rax), rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(P11, rax); bb = 15; continue; }
            15 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xa0u64))) as u64; r9 = r14; r8 = r15; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); rdx = r12; rcx = r13; crate::simd_ops::wu8(rax.wrapping_add(0x4u64), (0x2u8) as u8); crate::simd_ops::lk_and_u32(rax.wrapping_add(0x4u64), (0xffu32) as u32); rax = crate::simd_ops::call_stk(P12, rax, rcx, rdx, r8, r9, rsp); bb = 16; continue; }
            16 => { r11 = rsp.wrapping_add(0x50); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(r11.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x40u64))) as u64; rsp = r11; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14152d660(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143930e08; const P1: u64 = 0x143930e10; const P2: u64 = 0x142676b68; const P3: u64 = 0x142676b88; const P4: u64 = 0x142676ba8; const P5: u64 = 0x142645728; const P6: u64 = 0x142645358; const P7: u64 = 0x142676bc8; const P8: u64 = 0x142645758; const P9: u64 = 0x142676be0; const P10: u64 = 0x143930df8; const P11: u64 = 0x143930e00;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x48u64)) as u64; rax = (crate::simd_ops::ru64(P0)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::lk_add_u32(P1, (1) as u32); rax = 0x1; rsp = ((rsp).wrapping_add(0x48u64)) as u64; return rax; }
            2 => { crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rbx); rcx = P2; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rsi); rsi = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rcx); rbx = ((rsi as u32)) as u32 as u64; rcx = P3; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rcx); rdi = rsp.wrapping_add(0x20); rcx = P4; crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rcx); bb = 3; continue; }
            3 => { fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 14; continue; } bb = 4; continue; }
            4 => { rbp = (crate::simd_ops::ru64(rdi)) as u64; rcx = rbp; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rcx = rbp; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(P0, rax); bb = 7; continue; }
            6 => { rax = (crate::simd_ops::ru64(P0)) as u64; bb = 7; continue; }
            7 => { rbx = (((rbx as u32)).wrapping_add(1)) as u32 as u64; rdi = ((rdi).wrapping_add(0x8u64)) as u64; fl_l = ((rbx as u32)) as u32 as u64; fl_r = 0x3; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 3; continue; } bb = 8; continue; }
            8 => { fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 14; continue; } bb = 9; continue; }
            9 => { rbx = rsp.wrapping_add(0x20); bb = 10; continue; }
            10 => { fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 14; continue; } bb = 11; continue; }
            11 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp); rsi = (((rsi as u32)).wrapping_add(1)) as u32 as u64; rbx = ((rbx).wrapping_add(0x8u64)) as u64; crate::simd_ops::wu64(P0, rax); fl_l = ((rsi as u32)) as u32 as u64; fl_r = 0x3; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 10; continue; } bb = 12; continue; }
            12 => { fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 14; continue; } bb = 13; continue; }
            13 => { rax = 0x54; bb = 15; continue; }
            14 => { crate::simd_ops::lk_add_u32(P1, (1) as u32); rdx = P7; rcx = rax; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P8), rax), rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(P0)) as u64; rdx = P9; crate::simd_ops::wu64(P10, rax); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P8), rax), rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(P11, rax); rax = 0x1; bb = 15; continue; }
            15 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x48u64)) as u64; return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14152d790(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143930df8;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x48u64)) as u64;
    r8 = rsp.wrapping_add(0x20);
    rdx = 0;
    fl_l = 0; fl_r = 0;
    rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp);
    fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64;
    rax = (rax & !0xffu64) | (((if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { 1u8 } else { 0u8 }) as u8 as u64));
    rsp = ((rsp).wrapping_add(0x48u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14152d7b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143930e10; const P1: u64 = 0x143930e08; const P2: u64 = 0x142645750; const P3: u64 = 0x143930df8; const P4: u64 = 0x143930e00;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rax = (crate::simd_ops::ru32(P0)) as u32 as u64;
    fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64;
    if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
    fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1;
    rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64;
    crate::simd_ops::wu32(P0, ((rax as u32)) as u32);
    if !((((fl_l as u32)) != ((fl_r as u32)))) { rcx = (crate::simd_ops::ru64(P1)) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(P1, rax); crate::simd_ops::wu64(P3, rax); crate::simd_ops::wu64(P4, rax); }
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14152d800(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142676bf0; const P1: u64 = 0x143930e00; const P2: u64 = 0x18;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x28u64)) as u64; xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(crate::simd_ops::ru32(P0))).to_bits() as u128); xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) * f32::from_bits(crate::simd_ops::ru32(P0))).to_bits() as u128); rax = (crate::simd_ops::cvt_f32_i32((f32::from_bits(xmm1 as u32)))) as u32 as u64; crate::simd_ops::wu16(rsp.wrapping_add(0x38u64), ((rax as u16)) as u16); rax = (crate::simd_ops::cvt_f32_i32((f32::from_bits(xmm2 as u32)))) as u32 as u64; crate::simd_ops::wu16(rsp.wrapping_add(0x3au64), ((rax as u16)) as u16); rax = (crate::simd_ops::ru64(P1)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru32(rcx.wrapping_add(P2))) as u32 as u64; rdx = rsp.wrapping_add(0x38); (rax, xmm0) = crate::simd_ops::call_stk_xmm(rax, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 4; continue; } bb = 2; continue; }
            2 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0xaa; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 5; continue; } bb = 3; continue; }
            3 => { rax = 0x4d; rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
            4 => { rax = 0x1; rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
            5 => { rax = 0x2; rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14152d870(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143930e00; const P1: u64 = 0x143930e10; const P2: u64 = 0x143930e08; const P3: u64 = 0x142645750; const P4: u64 = 0x143930df8; const P5: u64 = 0x14152e450;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru64(P0)) as u64; r14 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rsp.wrapping_add(0x50u64), ((r14 as u32)) as u32); rbp = rdx; rsi = rcx; rbx = ((r14.wrapping_add(0x2u64)) as u32) as u32 as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru32(rcx.wrapping_add(0x18u64))) as u32 as u64; rdx = rsp.wrapping_add(0x50); rax = crate::simd_ops::call_stk(rax, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 4; continue; } bb = 2; continue; }
            2 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0xaa; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 5; continue; } bb = 3; continue; }
            3 => { rbx = ((rax.wrapping_sub(0x5du64)) as u32) as u32 as u64; bb = 5; continue; }
            4 => { rbx = 0x1; bb = 5; continue; }
            5 => { rdi = r14; bb = 6; continue; }
            6 => { fl_l = ((rbx as u32)) as u32 as u64; fl_r = 0x4d; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 13; continue; } bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru64(P0)) as u64; rbx = 0x2; crate::simd_ops::wu32(rsp.wrapping_add(0x58u64), ((r14 as u32)) as u32); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 8; continue; }
            8 => { rcx = (crate::simd_ops::ru32(rsi.wrapping_add(0x18u64))) as u32 as u64; rdx = rsp.wrapping_add(0x58); rax = crate::simd_ops::call_stk(rax, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 11; continue; } bb = 9; continue; }
            9 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0xaa; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 12; continue; } bb = 10; continue; }
            10 => { rbx = ((rax.wrapping_sub(0x5du64)) as u32) as u32 as u64; bb = 12; continue; }
            11 => { rbx = 0x1; bb = 12; continue; }
            12 => { rdi = ((rdi).wrapping_add(1)) as u64; fl_l = rdi; fl_r = (0x2710u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 6; continue; } bb = 13; continue; }
            13 => { rax = (crate::simd_ops::ru32(P1)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 16; continue; } bb = 14; continue; }
            14 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; crate::simd_ops::wu32(P1, ((rax as u32)) as u32); if (((fl_l as u32)) != ((fl_r as u32))) { bb = 16; continue; } bb = 15; continue; }
            15 => { rcx = (crate::simd_ops::ru64(P2)) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(P2, r14); crate::simd_ops::wu64(P4, r14); crate::simd_ops::wu64(P0, r14); bb = 16; continue; }
            16 => { rdx = rbp; rcx = rsi; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P5, rax, rcx, rdx, r8, r9, rsp); }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14152d960(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142676af0; const P1: u64 = 0xfffffffffffffff7; const P2: u64 = 0x8; const P3: u64 = 0x14152e1c0; const P4: u64 = 0x142676aa8; const P5: u64 = 0x18;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    r8 = P0;
    r9 = 0x29;
    rdi = ((rdx as u32)) as u32 as u64;
    rdx = ((r9.wrapping_add(P1)) as u32) as u32 as u64;
    rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp);
    rbx = rax;
    fl_l = rax; fl_r = rax;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rcx = rax; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rax = P4; crate::simd_ops::wu32(rbx.wrapping_add(P5), ((rdi as u32)) as u32); crate::simd_ops::wu64(rbx, rax); rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14152d9c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x18; const P1: u64 = 0x10; const P2: u64 = 0x4; const P3: u64 = 0x14152d800;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; fl_r = 0xffffffff;
    if !((((fl_l as u32)) == ((fl_r as u32)))) { rax = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; xmm2 = ((crate::simd_ops::ru32(rax.wrapping_add(P2))) as u32) as u128; xmm1 = ((crate::simd_ops::ru32(rax)) as u32) as u128; return crate::simd_ops::tail_fwd_xmm(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8, [xmm0, xmm1, xmm2, xmm3]); }
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14152da20(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x88; const P2: u64 = 0x14152dab0; const P3: u64 = 0x4; const P4: u64 = 0x14152e390;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rax = (crate::simd_ops::ru64(r8)) as u64; rbp = rcx; rcx = r8; rbx = r9; rdi = r8; rsi = rdx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = rax; r10 = (crate::simd_ops::ru64(rax)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(r10.wrapping_add(P1))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = rbp; r10 = (crate::simd_ops::ru64(rax)) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x30u64), r10); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; r9 = rbx; r8 = rdi; rdx = rsi; rax = (rax & !0xffu64) | (((if (((fl_l as u32)) == ((fl_r as u32))) { 1u8 } else { 0u8 }) as u8 as u64)); rcx = rbp; crate::simd_ops::wu8(rbp.wrapping_add(0x38u64), ((rax as u8)) as u8); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; crate::simd_ops::wu8(rax.wrapping_add(P3), (0x2u8) as u8); crate::simd_ops::lk_and_u32(rax.wrapping_add(P3), (0xffu32) as u32); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P4, rax, rcx, rdx, r8, r9, rsp); }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14152dab0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14152c220; const P1: u64 = 0x1426a7b98; const P2: u64 = 0x1426766f0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 21>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rbp = rsp.wrapping_sub(0x3f); rsp = ((rsp).wrapping_sub(0x88u64)) as u64; rbx = rcx; rdx = rbp.wrapping_add(0x6f); rcx = (crate::simd_ops::ru32(rcx.wrapping_add(0x18u64))) as u32 as u64; rsi = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rbp.wrapping_add(0x6fu64), rsi); rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x6fu64))) as u64; rdi = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 4; continue; } bb = 1; continue; }
            1 => { fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rdx = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rdx.wrapping_add(0x10u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 3; continue; }
            3 => { rax = ((rdi as u32)) as u32 as u64; rsp = ((rsp).wrapping_add(0x88u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            4 => { crate::simd_ops::wu64(rbx.wrapping_add(0x28u64), rcx); rdx = P1; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x58u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x28u64))) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) >= 0) { bb = 7; continue; } bb = 5; continue; }
            5 => { fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rax = 0x2; crate::simd_ops::wu64(rbx.wrapping_add(0x28u64), rsi); rsp = ((rsp).wrapping_add(0x88u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            7 => { rax = (crate::simd_ops::ru64(rcx)) as u64; r8 = 0x5; rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x30u64))) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x68u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x28u64))) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 5; continue; } bb = 8; continue; }
            8 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x38u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x28u64))) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 5; continue; } bb = 9; continue; }
            9 => { rax = rbp.wrapping_add(0x77); crate::simd_ops::wu32(rbp.wrapping_add(0x77u64), ((rsi as u32)) as u32); crate::simd_ops::wu64(rbp.wrapping_add(0x7u64), rax); r9 = rbx.wrapping_add(0x20); rax = rbp.wrapping_add(0x7f); crate::simd_ops::wu32(rbp.wrapping_add(0x7bu64), (0x4u32) as u32); crate::simd_ops::wu64(rbp.wrapping_add(0xfu64), rax); r8 = rbp.wrapping_sub(0x19); rax = rbp.wrapping_add(0x67); crate::simd_ops::wu64(rbp.wrapping_add(0x7fu64), rsi); crate::simd_ops::wu64(rbp.wrapping_add(0x27u64), rax); rdx = P2; crate::simd_ops::wu32(rbp.wrapping_add(0x67u64), ((rsi as u32)) as u32); crate::simd_ops::wu64(rbp.wrapping_add(0x1fu64), (0x4u64) as u64); crate::simd_ops::wu64(rbp.wrapping_add(0x2fu64), rsi); crate::simd_ops::wu32(rbp.wrapping_sub(0x19u64), (0x50u32) as u32); crate::simd_ops::wu32(rbp.wrapping_sub(0x15u64), (0x12u32) as u32); crate::simd_ops::wu32(rbp.wrapping_sub(0x11u64), (0xffffffffu32) as u32); crate::simd_ops::wu32(rbp.wrapping_sub(0xdu64), ((rsi as u32)) as u32); crate::simd_ops::wu32(rbp.wrapping_sub(0x9u64), (0x2710u32) as u32); crate::simd_ops::wu32(rbp.wrapping_sub(0x5u64), (0xffffffffu32) as u32); crate::simd_ops::wu32(rbp.wrapping_sub(0x1u64), ((rsi as u32)) as u32); crate::simd_ops::wu32(rbp.wrapping_add(0x3u64), (0x2u32) as u32); crate::simd_ops::wu64(rbp.wrapping_add(0x17u64), rsi); rax = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x90u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) >= 0) { bb = 13; continue; } bb = 10; continue; }
            10 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x28u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 11; continue; }
            11 => { rdx = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rdx.wrapping_add(0x10u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(0x28u64), rsi); bb = 12; continue; }
            12 => { rax = 0x2; rsp = ((rsp).wrapping_add(0x88u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            13 => { rax = 0x1; rsp = ((rsp).wrapping_add(0x88u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14152dc20(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x38; const P1: u64 = 0x28; const P2: u64 = 0xc8; const P3: u64 = 0x18;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x50u64)) as u64;
    fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(P0))) as u8 as u64; fl_r = (0x0u8) as u8 as u64;
    rdi = rcx;
    if !((((fl_l as u8)) != ((fl_r as u8)))) { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x50u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rbx);
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp);
    rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P1))) as u64;
    rdx = rsp.wrapping_add(0x20);
    crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x2cu32) as u32);
    rbx = (rbx & !0xffu64) | (((0u8) as u8 as u64));
    fl_l = 0; fl_r = 0;
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp);
    fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64;
    if !(((((fl_l as u32)) & ((fl_r as u32))) != 0)) { fl_l = (crate::simd_ops::ru8(rsp.wrapping_add(0x24u64))) as u8 as u64; fl_r = (0x1u8) as u8 as u64; rax = 0x1; rbx = (((rbx as u8)) as u32) as u32 as u64; rbx = (if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { (rax as u32) } else { (rbx as u32) }) as u32 as u64; }
    rax = (((rbx as u8)) as u32) as u32 as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64;
    rsp = ((rsp).wrapping_add(0x50u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14152dc90(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xffffffffffffffe8; const P1: u64 = 0xffffffffffffffd8; const P2: u64 = 0x14152dc20; const P3: u64 = 0x20; const P4: u64 = 0x143426f78; const P5: u64 = 0x30; const P6: u64 = 0x1;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0; let mut xmm7: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 21>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = rsp; rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0xa0u64)) as u64; crate::simd_ops::wu128(rax.wrapping_add(P0), (xmm6) as u128); rbx = rcx; crate::simd_ops::wu128(rax.wrapping_add(P1), (xmm7) as u128); xmm6 = (xmm2) as u128; xmm7 = (xmm1) as u128; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { r10 = (crate::simd_ops::ru64(rbx.wrapping_add(P3))) as u64; fl_l = r10; fl_r = r10; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { xmm7 = (xmm7 & !0xffff_ffffu128) | ((f32::from_bits(xmm7 as u32) * f32::from_bits(crate::simd_ops::ru32(P4))).to_bits() as u128); rdx = rsp.wrapping_add(0x30); rbx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), (0x4u64) as u64); xmm6 = (xmm6 & !0xffff_ffffu128) | ((f32::from_bits(xmm6 as u32) * f32::from_bits(crate::simd_ops::ru32(P4))).to_bits() as u128); r8 = 0x20000140; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rbx); rcx = (crate::simd_ops::cvt_f32_i32((f32::from_bits(xmm7 as u32)))) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rbx); crate::simd_ops::wu32(rsp.wrapping_add(0x48u64), ((rbx as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), rbx); crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), (0x50u32) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x34u64), (0x10u32) as u32); rax = (crate::simd_ops::cvt_f32_i32((f32::from_bits(xmm6 as u32)))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rcx as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x4cu64), (0x2u32) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rbx); xmm2 = (((rcx as u32)) as u32) as u128; xmm1 = (((rcx as u32)) as u32) as u128; rcx = r10; crate::simd_ops::wu32(rsp.wrapping_add(0x24u64), ((rax as u32)) as u32); xmm2 = crate::simd_ops::vcvtdq2pd(xmm2); xmm3 = (((rax as u32)) as u32) as u128; xmm0 = (((rax as u32)) as u32) as u128; xmm3 = crate::simd_ops::vcvtdq2pd(xmm3); xmm0 = crate::simd_ops::vcvtdq2pd(xmm0); xmm1 = crate::simd_ops::vcvtdq2pd(xmm1); xmm3 = (xmm3 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm3 as u64) * f64::from_bits(xmm0 as u64)).to_bits() as u128); xmm2 = (xmm2 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm2 as u64) * f64::from_bits(xmm1 as u64)).to_bits() as u128); xmm3 = (xmm3 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm3 as u64) + f64::from_bits(xmm2 as u64)).to_bits() as u128); xmm0 = crate::simd_ops::vsqrtpd(xmm3); rax = (crate::simd_ops::cvt_f64_i64((f64::from_bits(xmm0 as u64)))) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0xc8u64), ((rax as u32)) as u32); rax = rsp.wrapping_add(0x20); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rax); rax = rsp.wrapping_add(0xc8); crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), rax); r9 = (crate::simd_ops::ru64(r10)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(r9.wrapping_add(P5))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; rbx = (rbx & !0xffu64) | (((if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { 1u8 } else { 0u8 }) as u8 as u64)); rax = ((rbx.wrapping_add(P6)) as u32) as u32 as u64; bb = 4; continue; }
            3 => { rax = 0x2; bb = 4; continue; }
            4 => { xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x90u64))) as u128; xmm7 = (crate::simd_ops::ru128(rsp.wrapping_add(0x80u64))) as u128; rsp = ((rsp).wrapping_add(0xa0u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14152ddc0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14152dc90; const P1: u64 = 0x14152dc20; const P2: u64 = 0x14152e450;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 19>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = rsp; rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x88u64)) as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0x38u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; r14 = rdx; rbx = rcx; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 14; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rax.wrapping_add(0x10u64), rbp); xmm2 = 0u128; crate::simd_ops::wu64(rax.wrapping_sub(0x18u64), rsi); xmm1 = 0u128; crate::simd_ops::wu64(rax.wrapping_sub(0x20u64), rdi); crate::simd_ops::wu64(rax.wrapping_sub(0x28u64), r15); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r15 = 0; fl_l = 0; fl_r = 0; rcx = ((rax as u32)) as u32 as u64; rsi = ((r15 as u32)) as u32 as u64; bb = 2; continue; }
            2 => { rdi = rbx.wrapping_add(0x20); fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x4d; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 9; continue; } bb = 3; continue; }
            3 => { rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rcx = 0x2; bb = 8; continue; }
            5 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x20u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { rcx = 0x2; bb = 8; continue; }
            7 => { rax = rsp.wrapping_add(0xb0); crate::simd_ops::wu64(rsp.wrapping_add(0xb0u64), r15); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rax); rdx = rsp.wrapping_add(0x20); rax = rsp.wrapping_add(0xa0); crate::simd_ops::wu32(rsp.wrapping_add(0xa0u64), ((r15 as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rax); r8 = 0x20000140; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), r15); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), r15); crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), ((r15 as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), r15); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), (0x4u64) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), r15); crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x50u32) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x24u64), (0x10u32) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x3cu64), (0x2u32) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), r15); rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x30u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; rcx = ((r15 as u32)) as u32 as u64; rcx = (rcx & !0xffu64) | (((if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { 1u8 } else { 0u8 }) as u8 as u64)); fl_l = (((rcx as u32)).wrapping_add(1)) as u32 as u64; rcx = ((fl_l as u32)) as u32 as u64; bb = 8; continue; }
            8 => { rdx = 0x20; rsi = ((rsi).wrapping_add(1)) as u64; rax = rbx; rdi = ((rdx.wrapping_add(rbx)) as u64) as u64; fl_l = rsi; fl_r = (0x2710u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 2; continue; } bb = 9; continue; }
            9 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x28u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0xa8u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x40u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rbx.wrapping_add(0x28u64), r15); bb = 11; continue; }
            11 => { rcx = (crate::simd_ops::ru64(rdi)) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 12; continue; }
            12 => { rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rdi, r15); bb = 13; continue; }
            13 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; bb = 14; continue; }
            14 => { rdx = r14; rcx = rbx; rsp = ((rsp).wrapping_add(0x88u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14152df30(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142676c40; const P1: u64 = 0x4; const P2: u64 = 0x8; const P3: u64 = 0x14152e1c0; const P4: u64 = 0x142676bf8; const P5: u64 = 0x18; const P6: u64 = 0x20; const P7: u64 = 0x28; const P8: u64 = 0x38;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    r8 = P0;
    r9 = 0x3c;
    rdi = ((rdx as u32)) as u32 as u64;
    rdx = ((r9.wrapping_add(P1)) as u32) as u32 as u64;
    rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp);
    rbx = rax;
    fl_l = rax; fl_r = rax;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rcx = rax; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rax = P4; crate::simd_ops::wu32(rbx.wrapping_add(P5), ((rdi as u32)) as u32); crate::simd_ops::wu64(rbx, rax); rax = rbx; crate::simd_ops::wu64(rbx.wrapping_add(P6), (0x0u64) as u64); crate::simd_ops::wu64(rbx.wrapping_add(P7), (0x0u64) as u64); crate::simd_ops::wu8(rbx.wrapping_add(P8), (0x0u8) as u8); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14152dfa0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14152dc20; const P1: u64 = 0x10; const P2: u64 = 0x4; const P3: u64 = 0x14152dc90;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64;
    if !(((((fl_l as u8)) & ((fl_r as u8))) == 0)) { rax = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64; rcx = rbx; xmm2 = ((crate::simd_ops::ru32(rax.wrapping_add(P2))) as u32) as u128; xmm1 = ((crate::simd_ops::ru32(rax)) as u32) as u128; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); }
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14152dfe0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x18; const P1: u64 = 0x14152b0b0; const P2: u64 = 0x4; const P3: u64 = 0x14152e390;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp);
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x30u64)) as u64;
    rbp = rdx;
    rbx = rcx;
    rcx = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64;
    rdx = rsp.wrapping_add(0x40);
    rdi = r9;
    rsi = r8;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64;
    if !(((((fl_l as u32)) & ((fl_r as u32))) == 0)) { rax = 0x2; } else { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; r9 = rdi; r8 = rsi; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); rdx = rbp; rcx = rbx; crate::simd_ops::wu8(rax.wrapping_add(P2), (0x2u8) as u8); crate::simd_ops::lk_and_u32(rax.wrapping_add(P2), (0xffu32) as u32); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64;
    rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64;
    rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64;
    rsp = ((rsp).wrapping_add(0x30u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14152e060(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x18; const P1: u64 = 0x14152b0e0; const P2: u64 = 0x143426eec; const P3: u64 = 0x14152b0d0;
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut xmm6: u128 = 0; let mut xmm7: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x40u64)) as u64;
    crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm6) as u128);
    rbx = rcx;
    rcx = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64;
    rdx = 0x3;
    crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm7) as u128);
    xmm6 = (xmm1) as u128;
    xmm7 = (xmm2) as u128;
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    xmm6 = (xmm6 & !0xffff_ffffu128) | ((f32::from_bits(xmm6 as u32) * f32::from_bits(crate::simd_ops::ru32(P2))).to_bits() as u128);
    rdx = rsp.wrapping_add(0x58);
    rcx = (crate::simd_ops::ru32(rbx.wrapping_add(P0))) as u32 as u64;
    xmm7 = (xmm7 & !0xffff_ffffu128) | ((f32::from_bits(xmm7 as u32) * f32::from_bits(crate::simd_ops::ru32(P2))).to_bits() as u128);
    rax = (crate::simd_ops::cvt_f32_i32((f32::from_bits(xmm6 as u32)))) as u32 as u64;
    crate::simd_ops::wu8(rsp.wrapping_add(0x58u64), ((rax as u8)) as u8);
    rax = (crate::simd_ops::cvt_f32_i32((f32::from_bits(xmm7 as u32)))) as u32 as u64;
    crate::simd_ops::wu8(rsp.wrapping_add(0x59u64), ((rax as u8)) as u8);
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128;
    xmm7 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128;
    rsp = ((rsp).wrapping_add(0x40u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14152e0d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x18; const P1: u64 = 0x14152b0e0; const P2: u64 = 0x14152b0d0; const P3: u64 = 0x14152e450;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rdx;
    rdi = rcx;
    rcx = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64;
    rdx = 0x3;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rcx = (crate::simd_ops::ru32(rdi.wrapping_add(P0))) as u32 as u64;
    rdx = rsp.wrapping_add(0x30);
    crate::simd_ops::wu16(rsp.wrapping_add(0x30u64), (0x0u16) as u16);
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    rdx = rbx;
    rcx = rdi;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(P3, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14152e120(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14152b0f0; const P1: u64 = 0x142676d00; const P2: u64 = 0xffffffffffffffee; const P3: u64 = 0x8; const P4: u64 = 0x14152e1c0; const P5: u64 = 0x142676cb8; const P6: u64 = 0x18;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = ((rdx as u32)) as u32 as u64; rbx = rcx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rbx)) as u64; r8 = P1; r9 = 0x32; rcx = rbx; rdx = ((r9.wrapping_add(P2)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rbx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rcx = rax; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rax = P5; crate::simd_ops::wu32(rbx.wrapping_add(P6), ((rdi as u32)) as u32); crate::simd_ops::wu64(rbx, rax); rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            3 => { rbx = 0; fl_l = 0; fl_r = 0; rax = ((rbx as u32)) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            4 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = 0; fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14152e1a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x4; const P2: u64 = 0x14152e060;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    xmm2 = ((crate::simd_ops::ru32(rax.wrapping_add(P1))) as u32) as u128;
    xmm1 = ((crate::simd_ops::ru32(rax)) as u32) as u128;
    return crate::simd_ops::tail_fwd_xmm(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8, [xmm0, xmm1, xmm2, xmm3]);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14152e220(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143428720;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm4: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 1>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsi = (crate::simd_ops::ru32(rcx.wrapping_add(0xcu64))) as u32 as u64; rbx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rcx.wrapping_add(0x8u64), ((rbx as u32)) as u32); rdi = rdx; r9 = ((crate::simd_ops::ru16(rdx.wrapping_add(0x10u64))) as u32) as u32 as u64; r10 = rcx; rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x8u64))) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), r8); xmm2 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x14u64))) as u32) as u128; xmm4 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x10u64))) as u32) as u128; rax = (((rax as u8)) as u32) as u32 as u64; xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) - f32::from_bits(xmm4 as u32)).to_bits() as u128); fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; xmm0 = (((r9 as u32)) as u32) as u128; xmm0 = crate::simd_ops::vcvtdq2ps(xmm0); rsi = (if (((fl_l as u32)) >= ((fl_r as u32))) { (rax as u32) } else { (rsi as u32) }) as u32 as u64; xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) / f32::from_bits(xmm0 as u32)).to_bits() as u128); fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 11; continue; } bb = 1; continue; }
            1 => { xmm3 = ((crate::simd_ops::ru32(P0)) as u32) as u128; r8 = ((rbx as u32)) as u32 as u64; bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rdi)) as u64; xmm1 = (xmm4) as u128; r11 = (((r9 as u16)) as u32) as u32 as u64; rcx = ((rbx as u32)) as u32 as u64; rcx = (((rcx) as i64).wrapping_mul((r11) as i64) as u64) as u64; rdx = ((rax.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64; rcx = ((rdx.wrapping_add(r11.wrapping_mul(4))) as u64) as u64; fl_l = rdx; fl_r = rcx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 10; continue; } bb = 3; continue; }
            3 => { fl_l = r11; fl_r = (0x4u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 7; continue; } bb = 4; continue; }
            4 => { r9 = rcx.wrapping_sub(0xc); bb = 5; continue; }
            5 => { xmm0 = ((crate::simd_ops::ru32(rdx)) as u32) as u128; rax = (crate::simd_ops::ru64(r10.wrapping_add(0x10u64))) as u64; xmm0 = (xmm0) & (xmm3); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm1 as u32)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) + f32::from_bits(xmm2 as u32)).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((if f32::from_bits(xmm0 as u32) > f32::from_bits(crate::simd_ops::ru32(r8.wrapping_add(rax))) { f32::from_bits(xmm0 as u32) } else { f32::from_bits(crate::simd_ops::ru32(r8.wrapping_add(rax))) }).to_bits() as u128); crate::simd_ops::wu32(r8.wrapping_add(rax), ((xmm0 as u32)) as u32); xmm0 = ((crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32) as u128; rax = (crate::simd_ops::ru64(r10.wrapping_add(0x10u64))) as u64; xmm0 = (xmm0) & (xmm3); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm1 as u32)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) + f32::from_bits(xmm2 as u32)).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((if f32::from_bits(xmm0 as u32) > f32::from_bits(crate::simd_ops::ru32(r8.wrapping_add(rax))) { f32::from_bits(xmm0 as u32) } else { f32::from_bits(crate::simd_ops::ru32(r8.wrapping_add(rax))) }).to_bits() as u128); crate::simd_ops::wu32(r8.wrapping_add(rax), ((xmm0 as u32)) as u32); xmm0 = ((crate::simd_ops::ru32(rdx.wrapping_add(0x8u64))) as u32) as u128; rax = (crate::simd_ops::ru64(r10.wrapping_add(0x10u64))) as u64; xmm0 = (xmm0) & (xmm3); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm1 as u32)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) + f32::from_bits(xmm2 as u32)).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((if f32::from_bits(xmm0 as u32) > f32::from_bits(crate::simd_ops::ru32(r8.wrapping_add(rax))) { f32::from_bits(xmm0 as u32) } else { f32::from_bits(crate::simd_ops::ru32(r8.wrapping_add(rax))) }).to_bits() as u128); crate::simd_ops::wu32(r8.wrapping_add(rax), ((xmm0 as u32)) as u32); xmm0 = ((crate::simd_ops::ru32(rdx.wrapping_add(0xcu64))) as u32) as u128; rdx = ((rdx).wrapping_add(0x10u64)) as u64; rax = (crate::simd_ops::ru64(r10.wrapping_add(0x10u64))) as u64; xmm0 = (xmm0) & (xmm3); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm1 as u32)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) + f32::from_bits(xmm2 as u32)).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((if f32::from_bits(xmm0 as u32) > f32::from_bits(crate::simd_ops::ru32(r8.wrapping_add(rax))) { f32::from_bits(xmm0 as u32) } else { f32::from_bits(crate::simd_ops::ru32(r8.wrapping_add(rax))) }).to_bits() as u128); crate::simd_ops::wu32(r8.wrapping_add(rax), ((xmm0 as u32)) as u32); fl_l = rdx; fl_r = r9; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 5; continue; } bb = 6; continue; }
            6 => { r9 = ((crate::simd_ops::ru16(rdi.wrapping_add(0x10u64))) as u32) as u32 as u64; bb = 7; continue; }
            7 => { fl_l = rdx; fl_r = rcx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 10; continue; } bb = 8; continue; }
            8 => { xmm0 = ((crate::simd_ops::ru32(rdx)) as u32) as u128; rdx = ((rdx).wrapping_add(0x4u64)) as u64; rax = (crate::simd_ops::ru64(r10.wrapping_add(0x10u64))) as u64; xmm0 = (xmm0) & (xmm3); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm1 as u32)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) + f32::from_bits(xmm2 as u32)).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((if f32::from_bits(xmm0 as u32) > f32::from_bits(crate::simd_ops::ru32(r8.wrapping_add(rax))) { f32::from_bits(xmm0 as u32) } else { f32::from_bits(crate::simd_ops::ru32(r8.wrapping_add(rax))) }).to_bits() as u128); crate::simd_ops::wu32(r8.wrapping_add(rax), ((xmm0 as u32)) as u32); fl_l = rdx; fl_r = rcx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 8; continue; } bb = 9; continue; }
            9 => { r9 = ((crate::simd_ops::ru16(rdi.wrapping_add(0x10u64))) as u32) as u32 as u64; bb = 10; continue; }
            10 => { rbx = (((rbx as u32)).wrapping_add(1)) as u32 as u64; r8 = ((r8).wrapping_add(0x4u64)) as u64; fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 2; continue; } bb = 11; continue; }
            11 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x18u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14152e390(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x4; const P1: u64 = 0x142676dc0; const P2: u64 = 0xc; const P3: u64 = 0x8; const P4: u64 = 0x10; const P5: u64 = 0x34; const P6: u64 = 0x14183acc2;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64;
    r10 = rdx;
    rbx = rcx;
    r9 = 0x1a;
    r8 = (crate::simd_ops::ru32(rax.wrapping_add(P0))) as u32 as u64;
    rdx = (((r8 as u8)) as u32) as u32 as u64;
    r8 = P1;
    crate::simd_ops::wu32(rcx.wrapping_add(P2), ((rdx as u32)) as u32);
    rcx = r10;
    rax = (crate::simd_ops::ru64(r10)) as u64;
    rdx = ((rdx).wrapping_shl(2u32)) as u64;
    rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp);
    crate::simd_ops::wu64(rbx.wrapping_add(P4), rax);
    rcx = rax;
    fl_l = rax; fl_r = rax;
    if !(((((fl_l as u64)) & ((fl_r as u64))) != 0)) { rax = ((rcx.wrapping_add(P5)) as u32) as u32 as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    r8 = (crate::simd_ops::ru32(rbx.wrapping_add(P2))) as u32 as u64;
    rdx = 0;
    fl_l = 0; fl_r = 0;
    r8 = (((r8 as u32)).wrapping_shl(2u32)) as u32 as u64;
    rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp);
    rax = 0x1;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14152e400(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x40; const P2: u64 = 0xc; const P3: u64 = 0x10;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    crate::simd_ops::lk_add_u32(rcx.wrapping_add(P0), (1) as u32);
    rbx = rcx;
    fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; fl_r = 0x2;
    if (((fl_l as u32)) > ((fl_r as u32))) { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp);
    r8 = 0;
    fl_l = 0; fl_r = 0;
    rdx = ((r8 as u32)) as u32 as u64;
    fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(P2))) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64;
    if (((fl_l as u32)) <= ((fl_r as u32))) { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    loop { rax = (crate::simd_ops::ru64(rbx.wrapping_add(P3))) as u64; rcx = ((rdx as u32)) as u32 as u64; rdx = (((rdx as u32)).wrapping_add(1)) as u32 as u64; crate::simd_ops::wu32(rax.wrapping_add(rcx.wrapping_mul(4)), ((r8 as u32)) as u32); fl_l = ((rdx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rbx.wrapping_add(P2))) as u32 as u64; if !((((fl_l as u32)) < ((fl_r as u32)))) { break; } }
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14152e450(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    r8 = rdx;
    rbx = rcx;
    rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    fl_l = rdx; fl_r = rdx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rax = (crate::simd_ops::ru64(r8)) as u64; rcx = r8; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(P0), (0x0u64) as u64); }
    crate::simd_ops::wu64(rbx.wrapping_add(P1), (0x0u64) as u64);
    rax = 0x1;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14152e4c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142676ed0; const P1: u64 = 0x17; const P2: u64 = 0x8; const P3: u64 = 0x40; const P4: u64 = 0x142676e30; const P5: u64 = 0xc; const P6: u64 = 0x10; const P7: u64 = 0x14; const P8: u64 = 0x18; const P9: u64 = 0x1c; const P10: u64 = 0x20; const P11: u64 = 0x24;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; r8 = P0; r10 = rdx; r9 = 0x31; rbx = rcx; rcx = r10; rdx = ((r9.wrapping_add(P1)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P3))) as u64; rax = P4; crate::simd_ops::wu64(r8, rax); crate::simd_ops::wu64(r8.wrapping_add(P3), rdx); rcx = (crate::simd_ops::ru32(rbx.wrapping_add(P2))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(P2), ((rcx as u32)) as u32); rcx = (crate::simd_ops::ru32(rbx.wrapping_add(P5))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(P5), ((rcx as u32)) as u32); rcx = (crate::simd_ops::ru32(rbx.wrapping_add(P6))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(P6), ((rcx as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_add(P7))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(P7), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_add(P8))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(P8), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_add(P9))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(P9), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_add(P10))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(P10), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_add(P11))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(0x24u64), ((rax as u32)) as u32); rax = ((crate::simd_ops::ru8(rbx.wrapping_add(0x28u64))) as u32) as u32 as u64; crate::simd_ops::wu8(r8.wrapping_add(0x28u64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(rbx.wrapping_add(0x29u64))) as u32) as u32 as u64; crate::simd_ops::wu8(r8.wrapping_add(0x29u64), ((rax as u8)) as u8); rax = r8; xmm0 = (crate::simd_ops::ru128(rbx.wrapping_add(0x2au64))) as u128; crate::simd_ops::wu128(r8.wrapping_add(0x2au64), (xmm0) as u128); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            2 => { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14152e560(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142676ed0; const P1: u64 = 0x8; const P2: u64 = 0x142676e60; const P3: u64 = 0x10; const P4: u64 = 0x20; const P5: u64 = 0x18;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    r8 = P0;
    r9 = 0xc5;
    rdx = 0x38;
    rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp);
    rcx = rax;
    fl_l = rax; fl_r = rax;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rax = P2; crate::simd_ops::wu64(rcx, rax); rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rcx.wrapping_add(P3), rax); crate::simd_ops::wu64(rcx.wrapping_add(P1), rax); crate::simd_ops::wu64(rcx.wrapping_add(P4), rax); crate::simd_ops::wu64(rcx.wrapping_add(P5), rax); rax = rcx; rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14152e5b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142676ed0; const P1: u64 = 0x2a; const P2: u64 = 0x8; const P3: u64 = 0x142676e30; const P4: u64 = 0x40;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    r8 = P0;
    r9 = 0x1e;
    rbx = rcx;
    rdx = ((r9.wrapping_add(P1)) as u32) as u32 as u64;
    rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp);
    fl_l = rax; fl_r = rax;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rcx = P3; crate::simd_ops::wu64(rax.wrapping_add(P4), rbx); crate::simd_ops::wu64(rax, rcx); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14152e5f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14152e6f0;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; crate::simd_ops::wu32(rdx.wrapping_add(0xcu64), (0x2du32) as u32); rdi = rcx; r8 = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; rbx = rdx; xmm0 = (crate::simd_ops::ru128(r8.wrapping_add(0x8u64))) as u128; rcx = ((crate::simd_ops::ru8(r8.wrapping_add(0x29u64))) as u32) as u32 as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); xmm0 = (crate::simd_ops::ru128(r8.wrapping_add(0x18u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128); fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 3; continue; } bb = 2; continue; }
            2 => { r8 = rsp.wrapping_add(0x20); rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = ((crate::simd_ops::ru16(rbx.wrapping_add(0x10u64))) as u32) as u32 as u64; crate::simd_ops::wu16(rbx.wrapping_add(0x12u64), ((rax as u16)) as u16); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            3 => { rax = ((crate::simd_ops::ru16(rdx.wrapping_add(0x10u64))) as u32) as u32 as u64; crate::simd_ops::wu16(rdx.wrapping_add(0x12u64), ((rax as u16)) as u16); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            4 => { rax = ((crate::simd_ops::ru8(rdx.wrapping_add(0x8u64))) as u32) as u32 as u64; r11 = ((crate::simd_ops::ru16(rdx.wrapping_add(0x10u64))) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 10; continue; } bb = 5; continue; }
            5 => { crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rsi); r9 = (((r11 as u16)) as u32) as u32 as u64; rsi = 0; fl_l = 0; fl_r = 0; r10 = ((rax as u32)) as u32 as u64; rdx = ((rsi as u32)) as u32 as u64; fl_l = ((r8).wrapping_add(0x2au64)) as u64; fl_r = fl_l; r8 = ((fl_l as u64)) as u64; bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = (((r9 as u16)) as u32) as u32 as u64; rcx = (((rcx) as i64).wrapping_mul((rdx) as i64) as u64) as u64; rdi = ((rax.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64; rax = ((crate::simd_ops::ru16(r8)) as u32) as u32 as u64; xmm0 = ((crate::simd_ops::ru32(rsp.wrapping_add(rax.wrapping_mul(4)).wrapping_add(0x20u64))) as u32) as u128; crate::simd_ops::wu32(rsp.wrapping_add(0x50u64), ((xmm0 as u32)) as u32); fl_l = ((rsi as u16)) as u16 as u64; fl_r = ((r11 as u16)) as u16 as u64; if (((fl_l as u16)) >= ((fl_r as u16))) { bb = 8; continue; } bb = 7; continue; }
            7 => { rax = (((crate::simd_ops::ru32(rsp.wrapping_add(0x50u64))) as i32 as i64) as u64) as u64; rcx = r11; while rcx != 0 { crate::simd_ops::wu32(rdi, rax as u32); rdi = rdi.wrapping_add(4); rcx = rcx.wrapping_sub(1); } r9 = ((crate::simd_ops::ru16(rbx.wrapping_add(0x10u64))) as u32) as u32 as u64; bb = 8; continue; }
            8 => { rdx = ((rdx).wrapping_add(1)) as u64; r8 = ((r8).wrapping_add(0x2u64)) as u64; fl_l = r10; fl_r = (0x1u64) as u64; r10 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 6; continue; } bb = 9; continue; }
            9 => { rax = ((crate::simd_ops::ru16(rbx.wrapping_add(0x10u64))) as u32) as u32 as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; crate::simd_ops::wu16(rbx.wrapping_add(0x12u64), ((rax as u16)) as u16); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            10 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; crate::simd_ops::wu16(rdx.wrapping_add(0x12u64), ((r11 as u16)) as u16); rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14152e6f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14183acc2; const P1: u64 = 0x1414ea240; const P2: u64 = 0x143426d10; const P3: u64 = 0x141518450;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm4: u128 = 0; let mut xmm5: u128 = 0; let mut xmm6: u128 = 0; let mut xmm7: u128 = 0; let mut xmm8: u128 = 0; let mut xmm9: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 17>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = ((rsp).wrapping_sub(0x80u64)) as u64; rbx = ((crate::simd_ops::ru16(rdx.wrapping_add(0x10u64))) as u32) as u32 as u64; rsi = rcx; r9 = (crate::simd_ops::ru64(rdx)) as u64; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; xmm3 = ((crate::simd_ops::ru32(rcx.wrapping_add(0x18u64))) as u32) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x60u64), (xmm7) as u128); xmm7 = 0u128; fl_l = ((xmm3 as u32)) as u32 as u64; fl_r = ((xmm7 as u32)) as u32 as u64; rax = ((crate::simd_ops::ru16(rdx.wrapping_add(0x2au64))) as u32) as u32 as u64; rbp = ((r9.wrapping_add(rbx.wrapping_mul(4))) as u64) as u64; xmm4 = ((crate::simd_ops::ru32(r8.wrapping_add(rax.wrapping_mul(4)))) as u32) as u128; rax = ((crate::simd_ops::ru16(rdx.wrapping_add(0x2cu64))) as u32) as u32 as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x50u64), (xmm8) as u128); crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm9) as u128); xmm9 = ((crate::simd_ops::ru32(rcx.wrapping_add(0x1cu64))) as u32) as u128; xmm8 = ((crate::simd_ops::ru32(r8.wrapping_add(rax.wrapping_mul(4)))) as u32) as u128; crate::simd_ops::wu32(rcx.wrapping_add(0x1cu64), ((xmm8 as u32)) as u32); crate::simd_ops::wu32(rcx.wrapping_add(0x18u64), ((xmm4 as u32)) as u32); if (f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 3; continue; } bb = 1; continue; }
            1 => { fl_l = ((xmm4 as u32)) as u32 as u64; fl_r = ((xmm7 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 3; continue; } bb = 2; continue; }
            2 => { r8 = ((rbx.wrapping_mul(4)) as u32) as u32 as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = r9; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 27; continue; }
            3 => { r10 = (crate::simd_ops::ru32(rcx.wrapping_add(0x20u64))) as u32 as u64; r8 = 0; fl_l = 0; fl_r = 0; fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 14; continue; } bb = 4; continue; }
            4 => { crate::simd_ops::wu64(rsp.wrapping_add(0x90u64), rdi); bb = 5; continue; }
            5 => { rcx = (crate::simd_ops::ru32(rsi.wrapping_add(0x24u64))) as u32 as u64; rax = ((rcx as u32)) as u32 as u64; rdx = (if (rax as u32 as i32) < 0 { 0xffffffffu32 } else { 0u32 }) as u64; rax = (((rax as u32)).wrapping_sub((rdx as u32))) as u32 as u64; rax = (((((rax as u32)) as i32).wrapping_shr(1u32) as u32)) as u32 as u64; rdx = ((rax as u32)) as u32 as u64; fl_l = ((r10 as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { bb = 8; continue; } bb = 6; continue; }
            6 => { rdx = (((rdx as u32)).wrapping_sub((r10 as u32))) as u32 as u64; rcx = ((rbx as u32)) as u32 as u64; rcx = (((rcx as u32)).wrapping_sub((r8 as u32))) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; rdx = (if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { (rcx as u32) } else { (rdx as u32) }) as u32 as u64; r10 = (((r10 as u32)).wrapping_add((rdx as u32))) as u32 as u64; rax = ((rdx.wrapping_add(r8)) as u32) as u32 as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { bb = 12; continue; } bb = 7; continue; }
            7 => { rax = ((((r8 as u32)) as i32 as i64) as u64) as u64; fl_l = (((r8 as u32)).wrapping_add((rdx as u32))) as u32 as u64; fl_r = fl_l; r8 = ((fl_l as u32)) as u32 as u64; rcx = ((((rdx as u32)) as i32 as i64) as u64) as u64; rdi = ((r9.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = 0x3f800000; while rcx != 0 { crate::simd_ops::wu32(rdi, rax as u32); rdi = rdi.wrapping_add(4); rcx = rcx.wrapping_sub(1); } bb = 12; continue; }
            8 => { fl_l = ((r10 as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { bb = 11; continue; } bb = 9; continue; }
            9 => { rcx = (((rcx as u32)).wrapping_sub((r10 as u32))) as u32 as u64; rax = ((rbx as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_sub((r8 as u32))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; rcx = (if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { (rax as u32) } else { (rcx as u32) }) as u32 as u64; r10 = (((r10 as u32)).wrapping_add((rcx as u32))) as u32 as u64; rax = ((rcx.wrapping_add(r8)) as u32) as u32 as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { bb = 12; continue; } bb = 10; continue; }
            10 => { rax = ((((r8 as u32)) as i32 as i64) as u64) as u64; fl_l = (((r8 as u32)).wrapping_add((rcx as u32))) as u32 as u64; fl_r = fl_l; r8 = ((fl_l as u32)) as u32 as u64; rcx = ((((rcx as u32)) as i32 as i64) as u64) as u64; rdi = ((r9.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = (0xffffffffbf800000u64) as u64; while rcx != 0 { crate::simd_ops::wu32(rdi, rax as u32); rdi = rdi.wrapping_add(4); rcx = rcx.wrapping_sub(1); } bb = 12; continue; }
            11 => { r10 = 0; fl_l = 0; fl_r = 0; bb = 12; continue; }
            12 => { fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 5; continue; } bb = 13; continue; }
            13 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; bb = 14; continue; }
            14 => { fl_l = ((xmm4 as u32)) as u32 as u64; fl_r = ((xmm3 as u32)) as u32 as u64; crate::simd_ops::wu32(rsi.wrapping_add(0x20u64), ((r10 as u32)) as u32); if (f32::from_bits((fl_l as u32)) != f32::from_bits((fl_r as u32)) && !f32::from_bits((fl_l as u32)).is_nan() && !f32::from_bits((fl_r as u32)).is_nan()) { bb = 16; continue; } bb = 15; continue; }
            15 => { r8 = ((rbx as u32)) as u32 as u64; xmm1 = (xmm3) as u128; rcx = r9; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 27; continue; }
            16 => { rdx = ((r9.wrapping_add(rbx.wrapping_mul(4))) as u64) as u64; fl_l = ((rbx as u32)) as u32 as u64; fl_r = 0x4; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 21; continue; } bb = 17; continue; }
            17 => { xmm0 = 0u128; crate::simd_ops::wu128(rsp.wrapping_add(0x70u64), (xmm6) as u128); xmm2 = (xmm4) as u128; rax = ((rbx as u32)) as u32 as u64; xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) - f32::from_bits(xmm3 as u32)).to_bits() as u128); rax = ((((rax as u32)) & (0xfffffffcu32))) as u32 as u64; rcx = ((rax as u32)) as u32 as u64; xmm5 = (xmm3) as u128; xmm5 = crate::simd_ops::vshufps(xmm5, xmm5, 0xe1u8); rax = ((r9.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; xmm0 = (xmm0 & !0xffff_ffffu128) | ((((rcx) as i64) as f32).to_bits() as u128); xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) / f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm0 = (xmm3) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(xmm2 as u32)).to_bits() as u128); xmm6 = (xmm2) as u128; xmm6 = (xmm6 & !0xffff_ffffu128) | ((f32::from_bits(xmm6 as u32) * f32::from_bits(crate::simd_ops::ru32(P2))).to_bits() as u128); xmm1 = (xmm2) as u128; xmm6 = crate::simd_ops::vshufps(xmm6, xmm6, 0x0u8); xmm5 = (xmm5 & !0xffff_ffffu128) | (xmm0 & 0xffff_ffffu128); xmm5 = crate::simd_ops::vshufps(xmm5, xmm5, 0xc6u8); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) + f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) + f32::from_bits(xmm1 as u32)).to_bits() as u128); xmm5 = (xmm5 & !0xffff_ffffu128) | (xmm1 & 0xffff_ffffu128); xmm5 = crate::simd_ops::vshufps(xmm5, xmm5, 0x27u8); xmm5 = (xmm5 & !0xffff_ffffu128) | (xmm2 & 0xffff_ffffu128); xmm5 = crate::simd_ops::vshufps(xmm5, xmm5, 0x39u8); fl_l = r9; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 20; continue; } bb = 18; continue; }
            18 => { xmm1 = (xmm5) as u128; bb = 19; continue; }
            19 => { xmm0 = (crate::simd_ops::ru128(r9)) as u128; xmm5 = crate::simd_ops::vaddps(xmm5, xmm6); xmm0 = crate::simd_ops::vmulps(xmm0, xmm1); xmm1 = (xmm5) as u128; crate::simd_ops::wu128(r9, (xmm0) as u128); r9 = ((r9).wrapping_add(0x10u64)) as u64; fl_l = r9; fl_r = rax; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 19; continue; } bb = 20; continue; }
            20 => { xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x70u64))) as u128; bb = 21; continue; }
            21 => { fl_l = r9; fl_r = rdx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 27; continue; } bb = 22; continue; }
            22 => { xmm0 = 0u128; xmm4 = (xmm4 & !0xffff_ffffu128) | ((f32::from_bits(xmm4 as u32) - f32::from_bits(xmm3 as u32)).to_bits() as u128); rax = rdx; rax = ((rax).wrapping_sub(r9)) as u64; rax = ((rax).wrapping_add(0x3u64)) as u64; xmm0 = (xmm0 & !0xffff_ffffu128) | ((((rbx) as i64) as f32).to_bits() as u128); rax = (((rax) & (0xfffffffffffffffcu64))) as u64; xmm4 = (xmm4 & !0xffff_ffffu128) | ((f32::from_bits(xmm4 as u32) / f32::from_bits(xmm0 as u32)).to_bits() as u128); fl_l = rax; fl_r = (0x10u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 26; continue; } bb = 23; continue; }
            23 => { rax = rdx.wrapping_sub(0xc); bb = 24; continue; }
            24 => { xmm0 = (xmm3) as u128; xmm3 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) + f32::from_bits(xmm4 as u32)).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(crate::simd_ops::ru32(r9))).to_bits() as u128); crate::simd_ops::wu32(r9, ((xmm0 as u32)) as u32); xmm0 = (xmm3) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(crate::simd_ops::ru32(r9.wrapping_add(0x4u64)))).to_bits() as u128); xmm3 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) + f32::from_bits(xmm4 as u32)).to_bits() as u128); crate::simd_ops::wu32(r9.wrapping_add(0x4u64), ((xmm0 as u32)) as u32); xmm0 = (xmm3) as u128; xmm3 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) + f32::from_bits(xmm4 as u32)).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(crate::simd_ops::ru32(r9.wrapping_add(0x8u64)))).to_bits() as u128); crate::simd_ops::wu32(r9.wrapping_add(0x8u64), ((xmm0 as u32)) as u32); xmm0 = (xmm3) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(crate::simd_ops::ru32(r9.wrapping_add(0xcu64)))).to_bits() as u128); xmm3 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) + f32::from_bits(xmm4 as u32)).to_bits() as u128); crate::simd_ops::wu32(r9.wrapping_add(0xcu64), ((xmm0 as u32)) as u32); r9 = ((r9).wrapping_add(0x10u64)) as u64; fl_l = r9; fl_r = rax; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 24; continue; } bb = 25; continue; }
            25 => { fl_l = r9; fl_r = rdx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 27; continue; } bb = 26; continue; }
            26 => { xmm0 = (xmm3) as u128; xmm3 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) + f32::from_bits(xmm4 as u32)).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(crate::simd_ops::ru32(r9))).to_bits() as u128); crate::simd_ops::wu32(r9, ((xmm0 as u32)) as u32); r9 = ((r9).wrapping_add(0x4u64)) as u64; fl_l = r9; fl_r = rdx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 26; continue; } bb = 27; continue; }
            27 => { fl_l = ((xmm9 as u32)) as u32 as u64; fl_r = ((xmm7 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 30; continue; } bb = 28; continue; }
            28 => { fl_l = ((xmm8 as u32)) as u32 as u64; fl_r = ((xmm7 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 30; continue; } bb = 29; continue; }
            29 => { r8 = ((rbx.wrapping_mul(4)) as u32) as u32 as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rbp; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 31; continue; }
            30 => { rcx = rsi.wrapping_add(0x28); crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((xmm8 as u32)) as u32); xmm3 = (xmm9) as u128; r8 = ((rbx as u32)) as u32 as u64; rdx = rbp; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 31; continue; }
            31 => { xmm7 = (crate::simd_ops::ru128(rsp.wrapping_add(0x60u64))) as u128; r11 = rsp.wrapping_add(0x80); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x18u64))) as u64; rbp = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; xmm8 = (crate::simd_ops::ru128(r11.wrapping_sub(0x30u64))) as u128; xmm9 = (crate::simd_ops::ru128(r11.wrapping_sub(0x40u64))) as u128; rsp = r11; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14152e9d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142676f3c; const P1: u64 = 0x10; const P2: u64 = 0x8; const P3: u64 = 0x28; const P4: u64 = 0x4; const P5: u64 = 0x24; const P6: u64 = 0x30; const P7: u64 = 0x2c;
    let (mut rax, mut fl_l) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    xmm0 = ((crate::simd_ops::ru32(P0)) as u32) as u128;
    xmm1 = 0u128;
    crate::simd_ops::wu64(rcx.wrapping_add(P1), r8);
    r8 = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(P2), r9);
    rax = ((crate::simd_ops::ru8(r9.wrapping_add(P3))) as u32) as u32 as u64;
    crate::simd_ops::wu8(r8.wrapping_add(P4), ((rax as u8)) as u8);
    rax = 0xcccccccd;
    crate::simd_ops::lk_and_u32(r8.wrapping_add(P4), (0xffu32) as u32);
    let prod = (rax as u32 as u64).wrapping_mul((crate::simd_ops::ru32(r8)) as u32 as u64);
    rax = prod as u32 as u64; rdx = (prod >> 32) as u32 as u64; fl_l = (rdx != 0) as u64;
    fl_l = (((rdx as u32)).wrapping_shr(4u32)) as u32 as u64;
    rdx = ((fl_l as u32)) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(P5), ((rdx as u32)) as u32);
    rax = (crate::simd_ops::ru32(r8)) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(P3), (0x0u32) as u32);
    xmm1 = (xmm1 & !0xffff_ffffu128) | ((((rax) as i64) as f32).to_bits() as u128);
    rax = 0x1;
    xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) / f32::from_bits(xmm1 as u32)).to_bits() as u128);
    crate::simd_ops::wu32(rcx.wrapping_add(P6), ((xmm1 as u32)) as u32);
    crate::simd_ops::wu32(rcx.wrapping_add(P7), ((xmm0 as u32)) as u32);
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14152ea30(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x140000000; const P1: u64 = 0x1434268d0;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut j_tgt) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r9 = (rcx) as u64; fl_l = (r8) as u64 as u64; fl_r = (r8) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (0x1fu32) as u32 as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            2 => { rax = ((((rdx as u16)) as i16 as i32) as u32) as u32 as u64; r11 = (0x1u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x7fffu32) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 14; continue; } bb = 3; continue; }
            3 => { rax = (((rax as u32)).wrapping_sub(1)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x7u32) as u32 as u64; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 13; continue; } bb = 4; continue; }
            4 => { rdx = ((P0) as u64) as u64; rax = (rax as u32 as i32 as i64) as u64; rcx = (crate::simd_ops::ru32(rdx.wrapping_add(rax.wrapping_mul(4)).wrapping_add(0x152ebb0u64))) as u32 as u64; fl_l = ((rcx).wrapping_add(rdx)) as u64 as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; j_tgt = (rcx) as u64; bb = 0xffff_fffe; continue; }
            5 => { xmm1 = ((crate::simd_ops::ru32(r8)) as u32) as u128; xmm0 = 0u128; xmm1 = (xmm1 & !0xffff_ffffu128) | ((if f32::from_bits(xmm1 as u32) > f32::from_bits(xmm0 as u32) { f32::from_bits(xmm1 as u32) } else { f32::from_bits(xmm0 as u32) }).to_bits() as u128); rax = ((r11 as u32)) as u32 as u64; xmm1 = (xmm1 & !0xffff_ffffu128) | ((if f32::from_bits(xmm1 as u32) < f32::from_bits(core::ptr::read_unaligned((P1) as *const u32)) { f32::from_bits(xmm1 as u32) } else { f32::from_bits(core::ptr::read_unaligned((P1) as *const u32)) }).to_bits() as u128); crate::simd_ops::wu32(r9.wrapping_add(0x8u64), ((xmm1 as u32)) as u32); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            6 => { xmm1 = ((crate::simd_ops::ru32(r8)) as u32) as u128; xmm0 = 0u128; xmm1 = (xmm1 & !0xffff_ffffu128) | ((if f32::from_bits(xmm1 as u32) > f32::from_bits(xmm0 as u32) { f32::from_bits(xmm1 as u32) } else { f32::from_bits(xmm0 as u32) }).to_bits() as u128); rax = ((r11 as u32)) as u32 as u64; xmm1 = (xmm1 & !0xffff_ffffu128) | ((if f32::from_bits(xmm1 as u32) < f32::from_bits(core::ptr::read_unaligned((P1) as *const u32)) { f32::from_bits(xmm1 as u32) } else { f32::from_bits(core::ptr::read_unaligned((P1) as *const u32)) }).to_bits() as u128); crate::simd_ops::wu32(r9.wrapping_add(0xcu64), ((xmm1 as u32)) as u32); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            7 => { xmm1 = ((crate::simd_ops::ru32(r8)) as u32) as u128; xmm0 = 0u128; xmm1 = (xmm1 & !0xffff_ffffu128) | ((if f32::from_bits(xmm1 as u32) > f32::from_bits(xmm0 as u32) { f32::from_bits(xmm1 as u32) } else { f32::from_bits(xmm0 as u32) }).to_bits() as u128); rax = ((r11 as u32)) as u32 as u64; xmm1 = (xmm1 & !0xffff_ffffu128) | ((if f32::from_bits(xmm1 as u32) < f32::from_bits(core::ptr::read_unaligned((P1) as *const u32)) { f32::from_bits(xmm1 as u32) } else { f32::from_bits(core::ptr::read_unaligned((P1) as *const u32)) }).to_bits() as u128); crate::simd_ops::wu32(r9.wrapping_add(0x10u64), ((xmm1 as u32)) as u32); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            8 => { xmm1 = ((crate::simd_ops::ru32(r8)) as u32) as u128; xmm0 = 0u128; xmm1 = (xmm1 & !0xffff_ffffu128) | ((if f32::from_bits(xmm1 as u32) > f32::from_bits(xmm0 as u32) { f32::from_bits(xmm1 as u32) } else { f32::from_bits(xmm0 as u32) }).to_bits() as u128); rax = ((r11 as u32)) as u32 as u64; xmm1 = (xmm1 & !0xffff_ffffu128) | ((if f32::from_bits(xmm1 as u32) < f32::from_bits(core::ptr::read_unaligned((P1) as *const u32)) { f32::from_bits(xmm1 as u32) } else { f32::from_bits(core::ptr::read_unaligned((P1) as *const u32)) }).to_bits() as u128); crate::simd_ops::wu32(r9.wrapping_add(0x14u64), ((xmm1 as u32)) as u32); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            9 => { xmm1 = ((crate::simd_ops::ru32(r8)) as u32) as u128; xmm0 = 0u128; xmm1 = (xmm1 & !0xffff_ffffu128) | ((if f32::from_bits(xmm1 as u32) > f32::from_bits(xmm0 as u32) { f32::from_bits(xmm1 as u32) } else { f32::from_bits(xmm0 as u32) }).to_bits() as u128); rax = ((r11 as u32)) as u32 as u64; xmm1 = (xmm1 & !0xffff_ffffu128) | ((if f32::from_bits(xmm1 as u32) < f32::from_bits(core::ptr::read_unaligned((P1) as *const u32)) { f32::from_bits(xmm1 as u32) } else { f32::from_bits(core::ptr::read_unaligned((P1) as *const u32)) }).to_bits() as u128); crate::simd_ops::wu32(r9.wrapping_add(0x18u64), ((xmm1 as u32)) as u32); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            10 => { xmm1 = ((crate::simd_ops::ru32(r8)) as u32) as u128; xmm0 = 0u128; xmm1 = (xmm1 & !0xffff_ffffu128) | ((if f32::from_bits(xmm1 as u32) > f32::from_bits(xmm0 as u32) { f32::from_bits(xmm1 as u32) } else { f32::from_bits(xmm0 as u32) }).to_bits() as u128); rax = ((r11 as u32)) as u32 as u64; xmm1 = (xmm1 & !0xffff_ffffu128) | ((if f32::from_bits(xmm1 as u32) < f32::from_bits(core::ptr::read_unaligned((P1) as *const u32)) { f32::from_bits(xmm1 as u32) } else { f32::from_bits(core::ptr::read_unaligned((P1) as *const u32)) }).to_bits() as u128); crate::simd_ops::wu32(r9.wrapping_add(0x1cu64), ((xmm1 as u32)) as u32); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            11 => { xmm1 = ((crate::simd_ops::ru32(r8)) as u32) as u128; xmm0 = 0u128; xmm1 = (xmm1 & !0xffff_ffffu128) | ((if f32::from_bits(xmm1 as u32) > f32::from_bits(xmm0 as u32) { f32::from_bits(xmm1 as u32) } else { f32::from_bits(xmm0 as u32) }).to_bits() as u128); rax = ((r11 as u32)) as u32 as u64; xmm1 = (xmm1 & !0xffff_ffffu128) | ((if f32::from_bits(xmm1 as u32) < f32::from_bits(core::ptr::read_unaligned((P1) as *const u32)) { f32::from_bits(xmm1 as u32) } else { f32::from_bits(core::ptr::read_unaligned((P1) as *const u32)) }).to_bits() as u128); crate::simd_ops::wu32(r9.wrapping_add(0x20u64), ((xmm1 as u32)) as u32); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            12 => { xmm1 = ((crate::simd_ops::ru32(r8)) as u32) as u128; xmm0 = 0u128; xmm1 = (xmm1 & !0xffff_ffffu128) | ((if f32::from_bits(xmm1 as u32) > f32::from_bits(xmm0 as u32) { f32::from_bits(xmm1 as u32) } else { f32::from_bits(xmm0 as u32) }).to_bits() as u128); rax = ((r11 as u32)) as u32 as u64; xmm1 = (xmm1 & !0xffff_ffffu128) | ((if f32::from_bits(xmm1 as u32) < f32::from_bits(core::ptr::read_unaligned((P1) as *const u32)) { f32::from_bits(xmm1 as u32) } else { f32::from_bits(core::ptr::read_unaligned((P1) as *const u32)) }).to_bits() as u128); crate::simd_ops::wu32(r9.wrapping_add(0x24u64), ((xmm1 as u32)) as u32); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            13 => { r11 = (0x1fu32) as u32 as u64; rax = ((r11 as u32)) as u32 as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            14 => { rcx = ((crate::simd_ops::ru8(r8)) as u32) as u32 as u64; rdx = ((r8.wrapping_add(0x1u64)) as u64) as u64; crate::simd_ops::wu8(r9.wrapping_add(0x28u64), ((rcx as u8)) as u8); r10 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rax = ((crate::simd_ops::ru8(rdx)) as u32) as u32 as u64; rdx = ((rdx).wrapping_add(1)) as u64; crate::simd_ops::wu8(r9.wrapping_add(0x29u64), ((rax as u8)) as u8); fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 17; continue; } bb = 15; continue; }
            15 => { r8 = ((r9.wrapping_add(0x2au64)) as u64) as u64; bb = 16; continue; }
            16 => { rcx = ((crate::simd_ops::ru16(rdx)) as u32) as u32 as u64; rdx = ((rdx.wrapping_add(0x2u64)) as u64) as u64; crate::simd_ops::wu16(r8, ((rcx as u16)) as u16); r8 = ((r8.wrapping_add(0x2u64)) as u64) as u64; rcx = ((crate::simd_ops::ru8(r9.wrapping_add(0x28u64))) as u32) as u32 as u64; r10 = (((r10 as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((r10 as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 16; continue; } bb = 17; continue; }
            17 => { rax = ((r11 as u32)) as u32 as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            0xffff_fffe => { match j_tgt.wrapping_sub(0x14152ea30u64) { 0x8 => { bb = 1; continue; } 0xe => { bb = 2; continue; } 0x22 => { bb = 3; continue; } 0x2d => { bb = 4; continue; } 0x42 => { bb = 5; continue; } 0x60 => { bb = 6; continue; } 0x7e => { bb = 7; continue; } 0x9c => { bb = 8; continue; } 0xba => { bb = 9; continue; } 0xd8 => { bb = 10; continue; } 0xf6 => { bb = 11; continue; } 0x114 => { bb = 12; continue; } 0x132 => { bb = 13; continue; } 0x13c => { bb = 14; continue; } 0x159 => { bb = 15; continue; } 0x160 => { bb = 16; continue; } 0x17c => { bb = 17; continue; } _ => { crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(j_tgt, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::set_xmm_ret(c_x0); return c_rax; } } }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14152ebd0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x20; const P1: u64 = 0x8; const P2: u64 = 0x10; const P3: u64 = 0x18; const P4: u64 = 0x28; const P5: u64 = 0x1; const P6: u64 = 0x29; const P7: u64 = 0x2a; const P8: u64 = 0x2;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    r9 = rcx;
    fl_l = rdx; fl_r = rdx;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rax = 0x1f; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
    fl_l = ((r8 as u32)) as u32 as u64; fl_r = 0x20;
    if !((((fl_l as u32)) < ((fl_r as u32)))) { xmm0 = (crate::simd_ops::ru128(rdx)) as u128; rax = ((rdx.wrapping_add(P0)) as u64) as u64; r8 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu128(rcx.wrapping_add(P1), (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(rdx.wrapping_add(P2))) as u128; crate::simd_ops::wu128(rcx.wrapping_add(P3), (xmm1) as u128); rdx = ((crate::simd_ops::ru8(rax)) as u32) as u32 as u64; crate::simd_ops::wu8(rcx.wrapping_add(P4), ((rdx as u8)) as u8); rcx = ((crate::simd_ops::ru8(rax.wrapping_add(P5))) as u32) as u32 as u64; rax = ((rax).wrapping_add(0x2u64)) as u64; crate::simd_ops::wu8(r9.wrapping_add(P6), ((rcx as u8)) as u8); fl_l = ((rdx as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { rax = 0x1; crate::simd_ops::set_xmm_ret(xmm0); return rax; } rdx = ((r9.wrapping_add(P7)) as u64) as u64; loop { rcx = ((crate::simd_ops::ru16(rax)) as u32) as u32 as u64; rax = ((rax.wrapping_add(P8)) as u64) as u64; crate::simd_ops::wu16(rdx, ((rcx as u16)) as u16); rdx = ((rdx.wrapping_add(P8)) as u64) as u64; rcx = ((crate::simd_ops::ru8(r9.wrapping_add(P4))) as u32) as u32 as u64; r8 = (((r8 as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if !(((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32))) { break; } } rax = 0x1; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
    rax = 0x1f;
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14152ec40(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rdx = rcx; r8 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rcx.wrapping_add(0x68u64), r8); xmm0 = 0u128; crate::simd_ops::wu64(rcx.wrapping_add(0x70u64), r8); rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rcx.wrapping_add(0x90u64), rax); xmm1 = 0u128; crate::simd_ops::wu32(rcx.wrapping_add(0x98u64), ((rax as u32)) as u32); crate::simd_ops::wu64(rcx.wrapping_add(0xa8u64), rax); crate::simd_ops::wu32(rcx.wrapping_add(0xb0u64), ((rax as u32)) as u32); crate::simd_ops::wu64(rcx.wrapping_add(0xa0u64), r8); rcx = ((r8 as u32)) as u32 as u64; fl_l = (((rcx) >> (3u32)) & 1) as u64; rcx = ((rcx) | ((1u64) << (3u32))) as u64; rcx = ((rcx as u32)) as u32 as u64; fl_l = (((rcx) >> (4u32)) & 1) as u64; rcx = ((rcx) | ((1u64) << (4u32))) as u64; rcx = ((rcx as u32)) as u32 as u64; fl_l = (((rcx) >> (25u32)) & 1) as u64; rcx = ((rcx) | ((1u64) << (25u32))) as u64; rcx = ((rcx as u32)) as u32 as u64; fl_l = (((rcx) >> (26u32)) & 1) as u64; rcx = ((rcx) | ((1u64) << (26u32))) as u64; crate::simd_ops::wu32(rdx.wrapping_add(0x9cu64), ((rcx as u32)) as u32); rcx = (crate::simd_ops::ru32(rdx.wrapping_add(0xa0u64))) as u32 as u64; fl_l = (((rcx) >> (28u32)) & 1) as u64; rcx = ((rcx) | ((1u64) << (28u32))) as u64; rcx = ((rcx as u32)) as u32 as u64; fl_l = (((rcx) >> (29u32)) & 1) as u64; rcx = ((rcx) | ((1u64) << (29u32))) as u64; rcx = ((rcx as u32)) as u32 as u64; fl_l = (((rcx) >> (30u32)) & 1) as u64; rcx = ((rcx) | ((1u64) << (30u32))) as u64; rcx = ((rcx as u32)) as u32 as u64; fl_l = (((rcx) >> (31u32)) & 1) as u64; rcx = ((rcx) | ((1u64) << (31u32))) as u64; crate::simd_ops::wu32(rdx.wrapping_add(0xa0u64), ((rcx as u32)) as u32); rcx = (crate::simd_ops::ru32(rdx.wrapping_add(0xa4u64))) as u32 as u64; fl_l = (((rcx) >> (0u32)) & 1) as u64; rcx = ((rcx) | ((1u64) << (0u32))) as u64; rcx = ((rcx as u32)) as u32 as u64; fl_l = (((rcx) >> (1u32)) & 1) as u64; rcx = ((rcx) | ((1u64) << (1u32))) as u64; rcx = ((rcx as u32)) as u32 as u64; fl_l = (((rcx) >> (2u32)) & 1) as u64; rcx = ((rcx) | ((1u64) << (2u32))) as u64; rcx = ((rcx as u32)) as u32 as u64; fl_l = (((rcx) >> (3u32)) & 1) as u64; rcx = ((rcx) | ((1u64) << (3u32))) as u64; rcx = ((rcx as u32)) as u32 as u64; fl_l = (((rcx) >> (4u32)) & 1) as u64; rcx = ((rcx) | ((1u64) << (4u32))) as u64; rcx = ((rcx as u32)) as u32 as u64; fl_l = (((rcx) >> (5u32)) & 1) as u64; rcx = ((rcx) | ((1u64) << (5u32))) as u64; crate::simd_ops::wu32(rdx.wrapping_add(0xa4u64), ((rcx as u32)) as u32); crate::simd_ops::wu128(rdx, (xmm0) as u128); crate::simd_ops::wu128(rdx.wrapping_add(0x10u64), (xmm0) as u128); crate::simd_ops::wu128(rdx.wrapping_add(0x20u64), (xmm0) as u128); crate::simd_ops::wu128(rdx.wrapping_add(0x30u64), (xmm0) as u128); crate::simd_ops::wu128(rdx.wrapping_add(0x40u64), (xmm0) as u128); crate::simd_ops::wu128(rdx.wrapping_add(0x50u64), (xmm0) as u128); crate::simd_ops::wu64(rdx.wrapping_add(0x60u64), rax); crate::simd_ops::lk_and_u8(rdx.wrapping_add(0xb7u64), (0xf8u8) as u8); crate::simd_ops::wu8(rdx.wrapping_add(0xb6u64), ((rax as u8)) as u8); crate::simd_ops::wu64(rdx.wrapping_add(0xb8u64), r8); crate::simd_ops::wu64(rdx.wrapping_add(0xc0u64), r8); crate::simd_ops::wu64(rdx.wrapping_add(0xccu64), r8); crate::simd_ops::wu64(rdx.wrapping_add(0xd4u64), r8); crate::simd_ops::wu64(rdx.wrapping_add(0x220u64), r8); crate::simd_ops::wu64(rdx.wrapping_add(0x228u64), r8); crate::simd_ops::wu128(rdx, (xmm0) as u128); crate::simd_ops::wu128(rdx.wrapping_add(0x10u64), (xmm0) as u128); crate::simd_ops::wu128(rdx.wrapping_add(0x20u64), (xmm0) as u128); crate::simd_ops::wu128(rdx.wrapping_add(0x30u64), (xmm0) as u128); crate::simd_ops::wu128(rdx.wrapping_add(0x40u64), (xmm0) as u128); crate::simd_ops::wu128(rdx.wrapping_add(0x50u64), (xmm0) as u128); crate::simd_ops::wu64(rdx.wrapping_add(0x60u64), rax); crate::simd_ops::wu32(rdx.wrapping_add(0x70u64), ((r8 as u32)) as u32); crate::simd_ops::wu64(rdx.wrapping_add(0xa8u64), r8); crate::simd_ops::wu32(rdx.wrapping_add(0xb0u64), ((r8 as u32)) as u32); rcx = ((crate::simd_ops::ru8(rdx.wrapping_add(0xb6u64))) as u32) as u32 as u64; crate::simd_ops::wu64(rdx.wrapping_add(0x7cu64), r8); rcx = (rcx & !0xffu64) | ((((((rcx as u8)) & (0x31u8))) as u8 as u64)); crate::simd_ops::wu64(rdx.wrapping_add(0x84u64), r8); rcx = (rcx & !0xffu64) | ((((((rcx as u8)) & (0xcfu8))) as u8 as u64)); rcx = (rcx & !0xffu64) | ((((((rcx as u8)) & (0xfeu8))) as u8 as u64)); crate::simd_ops::wu32(rdx.wrapping_add(0x8cu64), ((r8 as u32)) as u32); fl_l = (crate::simd_ops::lk_and_u8(rdx.wrapping_add(0xb7u64), (0xf8u8) as u8)) as u8 as u64; fl_r = fl_l; crate::simd_ops::wu8(rdx.wrapping_add(0xb6u64), ((rcx as u8)) as u8); crate::simd_ops::wu16(rdx.wrapping_add(0xb4u64), ((r8 as u16)) as u16); crate::simd_ops::wu32(rdx.wrapping_add(0x78u64), (0x3f800000u32) as u32); crate::simd_ops::wu32(rdx.wrapping_add(0xc0u64), ((r8 as u32)) as u32); crate::simd_ops::wu128(rdx.wrapping_add(0xc8u64), (xmm1) as u128); crate::simd_ops::wu32(rdx.wrapping_add(0xd8u64), ((xmm0 as u32)) as u32); rax = rdx; crate::simd_ops::wu64(rdx.wrapping_add(0x230u64), r8); crate::simd_ops::wu64(rdx.wrapping_add(0x238u64), r8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14152ede0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415242e0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = ((rsp).wrapping_sub(0x28u64)) as u64; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(0xb8u64))) as u64; r12 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rbx); rbp = rcx; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r15); fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu32(rcx.wrapping_add(0xc0u64), ((r12 as u32)) as u32); rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbp.wrapping_add(0xb8u64), r12); crate::simd_ops::wu32(rbp.wrapping_add(0xc4u64), ((r12 as u32)) as u32); bb = 2; continue; }
            2 => { rbx = (crate::simd_ops::ru64(rbp.wrapping_add(0x220u64))) as u64; r15 = rbp.wrapping_add(0xe0); fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru32(r15.wrapping_add(0x148u64))) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rdi); rcx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rdi = ((rbx.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 14; continue; } bb = 4; continue; }
            4 => { crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), r14); r14 = rbx.wrapping_add(0x20); bb = 5; continue; }
            5 => { rsi = (crate::simd_ops::ru64(r14.wrapping_sub(0x8u64))) as u64; rax = (crate::simd_ops::ru32(r14)) as u32 as u64; rcx = ((rsi.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = rsi; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 10; continue; } bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru64(rsi)) as u64; rax = 0xffffffff; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rcx.wrapping_add(0x14u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 8; continue; } bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 8; continue; }
            8 => { rdx = (crate::simd_ops::ru64(r14.wrapping_sub(0x8u64))) as u64; rsi = ((rsi).wrapping_add(0x8u64)) as u64; rax = (crate::simd_ops::ru32(r14)) as u32 as u64; rcx = ((rdx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = rsi; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 6; continue; } bb = 9; continue; }
            9 => { rsi = rdx; bb = 10; continue; }
            10 => { fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 11; continue; }
            11 => { rdx = rsi; crate::simd_ops::wu32(r14, ((r12 as u32)) as u32); rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(r14.wrapping_sub(0x8u64), r12); crate::simd_ops::wu32(r14.wrapping_add(0x4u64), ((r12 as u32)) as u32); bb = 12; continue; }
            12 => { rbx = ((rbx).wrapping_add(0x28u64)) as u64; r14 = ((r14).wrapping_add(0x28u64)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 5; continue; } bb = 13; continue; }
            13 => { r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; bb = 14; continue; }
            14 => { rdx = (crate::simd_ops::ru64(r15.wrapping_add(0x140u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; crate::simd_ops::wu32(r15.wrapping_add(0x148u64), ((r12 as u32)) as u32); fl_l = r15; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 16; continue; } bb = 15; continue; }
            15 => { rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); bb = 16; continue; }
            16 => { crate::simd_ops::wu64(r15.wrapping_add(0x140u64), r12); crate::simd_ops::wu32(r15.wrapping_add(0x14cu64), ((r12 as u32)) as u32); bb = 17; continue; }
            17 => { rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x68u64))) as u64; r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 19; continue; } bb = 18; continue; }
            18 => { rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rbp.wrapping_add(0x70u64), ((r12 as u32)) as u32); rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbp.wrapping_add(0x68u64), r12); crate::simd_ops::wu32(rbp.wrapping_add(0x74u64), ((r12 as u32)) as u32); bb = 19; continue; }
            19 => { rsp = ((rsp).wrapping_add(0x28u64)) as u64; r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14152ef40(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141524330; const P1: u64 = 0x141532110; const P2: u64 = 0x14159cab0; const P3: u64 = 0x143930e50; const P4: u64 = 0x143930e48; const P5: u64 = 0x141532100; const P6: u64 = 0x143930f90; const P7: u64 = 0x14159b5e0; const P8: u64 = 0x1415320f0; const P9: u64 = 0x143930e58; const P10: u64 = 0x1415372b0; const P11: u64 = 0x14159af70; const P12: u64 = 0x1415a1730; const P13: u64 = 0x141532270; const P14: u64 = 0x1415242e0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rbp = rdx; rbx = rcx; rdx = 0x98; rcx = 0; fl_l = 0; fl_r = 0; rdi = (((r9 as u8)) as u32) as u32 as u64; r14 = r8; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 1; continue; }
            1 => { r9 = r14; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rsi); r8 = rbp; crate::simd_ops::wu8(rsp.wrapping_add(0x20u64), ((rdi as u8)) as u8); rdx = rbx; rcx = rax; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rsi = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 16; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0xb8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rdi = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 15; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rax)) as u64; rdx = 0x1; rcx = rdi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x118u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = rdi; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 14; continue; } bb = 4; continue; }
            4 => { rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(P3)) as u64; rdi = 0; fl_l = 0; fl_r = 0; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { crate::simd_ops::wu64(P3, rbx); crate::simd_ops::wu64(P4, rbx); crate::simd_ops::wu64(rbx.wrapping_add(0x78u64), rdi); bb = 7; continue; }
            6 => { crate::simd_ops::wu64(rbx.wrapping_add(0x78u64), rax); crate::simd_ops::wu64(P3, rbx); bb = 7; continue; }
            7 => { rdx = rsi; rcx = rbx; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru32(rbp.wrapping_add(0x14u64))) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 16; continue; } bb = 8; continue; }
            8 => { rcx = (crate::simd_ops::ru64(P6)) as u64; rax = rsp.wrapping_add(0x30); r8 = rbx.wrapping_add(0x18); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); r9 = r14; crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((rdi as u32)) as u32); rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru32(rsp.wrapping_add(0x30u64))) as u32 as u64; rcx = rbx; rdi = ((rax as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru32(rsp.wrapping_add(0x30u64))) as u32 as u64; fl_r = 0x200000; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 11; continue; } bb = 9; continue; }
            9 => { rdx = (crate::simd_ops::ru32(rbp.wrapping_add(0x14u64))) as u32 as u64; rcx = P9; rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 11; continue; } bb = 10; continue; }
            10 => { rdx = (crate::simd_ops::ru32(rsp.wrapping_add(0x30u64))) as u32 as u64; rcx = rbx; fl_l = ((((rdx as u32)) >> (21u32)) & 1) as u64; rdx = (((rdx as u32)) & !((1u32) << (21u32))) as u32 as u64; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); bb = 11; continue; }
            11 => { rdx = (crate::simd_ops::ru32(rbp.wrapping_add(0x14u64))) as u32 as u64; rcx = (crate::simd_ops::ru64(P6)) as u64; rax = crate::simd_ops::call_stk(P11, rax, rcx, rdx, r8, r9, rsp); rax = ((rdi as u32)) as u32 as u64; bb = 12; continue; }
            12 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; bb = 13; continue; }
            13 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            14 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rdx = 0x1; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x120u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = rdi; rax = crate::simd_ops::call_stk(P12, rax, rcx, rdx, r8, r9, rsp); bb = 15; continue; }
            15 => { rcx = rsi; rax = crate::simd_ops::call_stk(P13, rax, rcx, rdx, r8, r9, rsp); rdx = rsi; rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P14, rax, rcx, rdx, r8, r9, rsp); bb = 16; continue; }
            16 => { rax = 0x2; bb = 12; continue; }
            17 => { rax = 0x2; bb = 13; continue; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14152f0e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143930e20; const P1: u64 = 0x10; const P2: u64 = 0x143930e28; const P3: u64 = 0x143930e38; const P4: u64 = 0x143930e40; const P5: u64 = 0x143930e30; const P6: u64 = 0x143930e44; const P7: u64 = 0x1415242e0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rcx = (crate::simd_ops::ru64(P0)) as u64; rdi = 0; fl_l = 0; fl_r = 0; rax = rcx; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx); bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru8(rax.wrapping_add(P1))) as u8 as u64; fl_r = (0x2u8) as u8 as u64; rbx = (crate::simd_ops::ru64(rax)) as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 10; continue; } bb = 3; continue; }
            3 => { fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 5; continue; } bb = 4; continue; }
            4 => { crate::simd_ops::wu64(P0, rbx); bb = 6; continue; }
            5 => { crate::simd_ops::wu64(rdi, rbx); bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru64(P2)) as u64; r8 = (crate::simd_ops::ru64(P3)) as u64; fl_l = rax; fl_r = rcx; rcx = (if (((fl_l as u64)) == ((fl_r as u64))) { rdi } else { rcx }) as u64; crate::simd_ops::wu64(P2, rcx); fl_l = rax; fl_r = r8; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 9; continue; } bb = 7; continue; }
            7 => { rcx = (crate::simd_ops::ru32(P4)) as u32 as u64; rdx = ((rcx.wrapping_add(rcx.wrapping_mul(2))) as u64) as u64; rcx = ((r8.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 9; continue; } bb = 8; continue; }
            8 => { rcx = (crate::simd_ops::ru64(P5)) as u64; crate::simd_ops::wu64(rax, rcx); fl_l = (crate::simd_ops::lk_sub_u32(P6, (1) as u32)) as u32 as u64; rcx = (crate::simd_ops::ru64(P0)) as u64; crate::simd_ops::wu64(P5, rax); bb = 11; continue; }
            9 => { rdx = rax; rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::lk_sub_u32(P6, (1) as u32)) as u32 as u64; rcx = (crate::simd_ops::ru64(P0)) as u64; bb = 11; continue; }
            10 => { rdi = rax; bb = 11; continue; }
            11 => { rax = rbx; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 12; continue; }
            12 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; bb = 13; continue; }
            13 => { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14152f1b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143930e50; const P1: u64 = 0x141530d60; const P2: u64 = 0x1415370e0; const P3: u64 = 0x143930e58; const P4: u64 = 0x143930e48; const P5: u64 = 0x141537600; const P6: u64 = 0x143930e30; const P7: u64 = 0x143930e20; const P8: u64 = 0x143930e38; const P9: u64 = 0x143930e28; const P10: u64 = 0x143930e40; const P11: u64 = 0x143930e44; const P12: u64 = 0x1415242e0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = (crate::simd_ops::ru64(P0)) as u64; rdi = 0; fl_l = 0; fl_r = 0; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { bb = 2; continue; }
            2 => { rcx = rbx; rdx = rsp.wrapping_add(0x30); rbx = (crate::simd_ops::ru64(rbx.wrapping_add(0x78u64))) as u64; r8 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((rdi as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x34u64), (0x4u32) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 3; continue; }
            3 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = P3; crate::simd_ops::wu64(P0, rdi); crate::simd_ops::wu64(P4, rdi); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = (crate::simd_ops::ru64(P6)) as u64; bb = 4; continue; }
            4 => { r8 = (crate::simd_ops::ru64(P7)) as u64; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(r8)) as u64; r9 = (crate::simd_ops::ru64(P8)) as u64; crate::simd_ops::wu64(P7, rax); rax = (crate::simd_ops::ru64(P9)) as u64; fl_l = r8; fl_r = rax; rax = (if (((fl_l as u64)) == ((fl_r as u64))) { rdi } else { rax }) as u64; crate::simd_ops::wu64(P9, rax); fl_l = r8; fl_r = r9; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 8; continue; } bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru32(P10)) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = ((r9.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = r8; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 8; continue; } bb = 7; continue; }
            7 => { crate::simd_ops::wu64(r8, rdx); rdx = r8; fl_l = (crate::simd_ops::lk_sub_u32(P11, (1) as u32)) as u32 as u64; crate::simd_ops::wu64(P6, rdx); bb = 4; continue; }
            8 => { rdx = r8; rcx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P12, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = (crate::simd_ops::ru64(P6)) as u64; fl_l = (crate::simd_ops::lk_sub_u32(P11, (1) as u32)) as u32 as u64; bb = 4; continue; }
            9 => { r8 = (crate::simd_ops::ru64(P8)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 10; continue; }
            10 => { rbx = (crate::simd_ops::ru64(rdx)) as u64; fl_l = rdx; fl_r = r8; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 12; continue; } bb = 11; continue; }
            11 => { rax = (crate::simd_ops::ru32(P10)) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = ((r8.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rdx; fl_r = rax; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 13; continue; } bb = 12; continue; }
            12 => { rcx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P12, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = (crate::simd_ops::ru64(P8)) as u64; bb = 13; continue; }
            13 => { rdx = rbx; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 10; continue; } bb = 14; continue; }
            14 => { fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 16; continue; } bb = 15; continue; }
            15 => { rdx = r8; rcx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P12, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(P8, rdi); bb = 16; continue; }
            16 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; xmm0 = 0u128; crate::simd_ops::wu128(P7, (xmm0) as u128); crate::simd_ops::wu32(P10, ((rdi as u32)) as u32); crate::simd_ops::wu64(P6, rdi); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14152f320(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    crate::simd_ops::wu32(rcx, (0x3f800000u32) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14152f330(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143930e58; const P1: u64 = 0x1415373a0;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    r9 = (((r8 as u8)) as u32) as u32 as u64;
    r8 = rdx;
    rdx = ((rcx as u32)) as u32 as u64;
    rcx = P0;
    return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14152f350(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143930e18; const P1: u64 = 0x1434268d0; const P2: u64 = 0x143930e58; const P3: u64 = 0x1401de410; const P4: u64 = 0x143930e1c; const P5: u64 = 0x143930e1d; const P6: u64 = 0x141524330; const P7: u64 = 0x143930e38; const P8: u64 = 0x143930e40; const P9: u64 = 0x143930e30; const P10: u64 = 0x14152f9d0; const P11: u64 = 0x1415862a0; const P12: u64 = 0x14152f310; const P13: u64 = 0x141530000; const P14: u64 = 0x14157ebe0; const P15: u64 = 0x141534ce0; const P16: u64 = 0x14157ebd0; const P17: u64 = 0x141534fa0; const P18: u64 = 0x14157ebf0; const P19: u64 = 0x14158bd20; const P20: u64 = 0x14152f1b0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x38u64)) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { xmm0 = ((crate::simd_ops::ru32(rcx)) as u32) as u128; crate::simd_ops::wu32(P0, ((xmm0 as u32)) as u32); bb = 3; continue; }
            2 => { xmm0 = ((crate::simd_ops::ru32(P1)) as u32) as u128; crate::simd_ops::wu32(P0, ((xmm0 as u32)) as u32); bb = 3; continue; }
            3 => { rcx = P2; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = 0x60; crate::simd_ops::wu8(P4, (0x0u8) as u8); rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu8(P5, (0x0u8) as u8); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(P7, rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 4; continue; }
            4 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx); rdx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(P8, (0x4u32) as u32); crate::simd_ops::wu64(P9, rax); bb = 5; continue; }
            5 => { rcx = rax.wrapping_add(0x18); rdx = (((rdx as u32)).wrapping_add(1)) as u32 as u64; crate::simd_ops::wu64(rax, rcx); r8 = rax; fl_l = ((rdx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(P8)) as u32 as u64; rax = rcx; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 5; continue; } bb = 6; continue; }
            6 => { crate::simd_ops::wu64(r8, (0x0u64) as u64); rcx = P10; r8 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0x321u32) as u32); r9 = (r9 & !0xffu64) | (((0x8u8) as u8 as u64)); crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x0u32) as u32); rdx = ((r8.wrapping_add(0x10u64)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P11, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 9; continue; } bb = 7; continue; }
            7 => { crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0x321u32) as u32); rdx = ((rax.wrapping_add(0x7fu64)) as u32) as u32 as u64; r9 = (r9 & !0xffu64) | (((0x8u8) as u8 as u64)); crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x0u32) as u32); r8 = 0; fl_l = 0; fl_r = 0; rcx = P12; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P11, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 9; continue; } bb = 8; continue; }
            8 => { rcx = P13; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P14, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = P15; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P16, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = P17; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P18, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = ((rbx as u32)) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x38u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            9 => { rdx = 0x10; rcx = P10; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P19, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = 0x10; rcx = P12; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P19, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P20, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = ((rbx as u32)) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x38u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            10 => { rax = 0x2; rsp = ((rsp).wrapping_add(0x38u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14152f4c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x143930e1c; const P1: u64 = 0x143930e1d;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    crate::simd_ops::wu8(P0, (0x1u8) as u8);
    crate::simd_ops::wu8(P1, ((rcx as u8)) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14152f4d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143930e50; const P1: u64 = 0xb8; const P2: u64 = 0x68; const P3: u64 = 0x30; const P4: u64 = 0x2c; const P5: u64 = 0x1415309d0; const P6: u64 = 0x78;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = (crate::simd_ops::ru64(P0)) as u64; rdi = ((r9 as u32)) as u32 as u64; rbp = r8; rsi = rdx; r14 = rcx; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = r14; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 7; continue; } bb = 2; continue; }
            2 => { fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64; fl_l = rsi; fl_r = (crate::simd_ops::ru64(rcx.wrapping_add(P3))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 7; continue; } bb = 4; continue; }
            4 => { fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64; fl_l = ((rdi as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rcx.wrapping_add(P4))) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rdx = rbp; rcx = rbx; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); bb = 7; continue; }
            7 => { rbx = (crate::simd_ops::ru64(rbx.wrapping_add(P6))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 1; continue; } bb = 8; continue; }
            8 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14152f560(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143930e20; const P1: u64 = 0x10; const P2: u64 = 0x8; const P3: u64 = 0xc; const P4: u64 = 0x141589e40; const P5: u64 = 0x14152f0e0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = (crate::simd_ops::ru64(P0)) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { rax = rbx; bb = 2; continue; }
            2 => { rax = ((rax).wrapping_add(0x8u64)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rbx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(P1))) as u32 as u64; rax = ((((rax as u32)) & (0xfffffffdu32))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(P1), ((rax as u32)) as u32); fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 11; continue; } bb = 5; continue; }
            5 => { rax = ((((rax as u32)) | (0x1u32))) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rdi); crate::simd_ops::wu32(rbx.wrapping_add(P1), ((rax as u32)) as u32); r9 = (r9 & !0xffu64) | (((0x1u8) as u8 as u64)); rdi = (crate::simd_ops::ru32(rbx.wrapping_add(P2))) as u32 as u64; r8 = 0; fl_l = 0; fl_r = 0; rdx = (crate::simd_ops::ru32(rbx.wrapping_add(P3))) as u32 as u64; rcx = ((rdi as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 6; continue; }
            6 => { bb = 7; continue; }
            7 => { fl_l = (crate::simd_ops::ru32(rax.wrapping_add(P2))) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 9; continue; } bb = 8; continue; }
            8 => { fl_l = (crate::simd_ops::lk_or_u32(rax.wrapping_add(P1), (0x1u32) as u32)) as u32 as u64; fl_r = fl_l; bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru64(rax)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 7; continue; } bb = 10; continue; }
            10 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; bb = 11; continue; }
            11 => { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P5, rax, rcx, rdx, r8, r9, rsp); }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14152f5e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141598270; const P1: u64 = 0x141599bb0; const P2: u64 = 0x1415999f0; const P3: u64 = 0x14152ec40; const P4: u64 = 0x1415864b0; const P5: u64 = 0x141524330; const P6: u64 = 0x141530ec0; const P7: u64 = 0x141592360; const P8: u64 = 0x1415984f0; const P9: u64 = 0x141597670; const P10: u64 = 0x141596a90; const P11: u64 = 0x141596d80; const P12: u64 = 0x14152ede0; const P13: u64 = 0x143930f90; const P14: u64 = 0x14159b5e0; const P15: u64 = 0x141597520; const P16: u64 = 0x1401de410;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 135>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), r8); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rbp = rsp.wrapping_sub(0x300); rsp = ((rsp).wrapping_sub(0x400u64)) as u64; rsi = (crate::simd_ops::ru64(rbp.wrapping_add(0x398u64))) as u64; r8 = rsp.wrapping_add(0x5c); r12 = rdx; rbx = 0; fl_l = 0; fl_r = 0; rdx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu8(rsp.wrapping_add(0x51u64), ((rbx as u8)) as u8); rcx = r12; r14 = r9; crate::simd_ops::wu64(rsi, rbx); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r9 = rsp.wrapping_add(0x58); crate::simd_ops::wu8(rsp.wrapping_add(0x50u64), ((rax as u8)) as u8); r8 = r14; rcx = rbp.wrapping_sub(0x40); rdx = r12; rdi = (((rax as u8)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm0 = ((crate::simd_ops::ru32(rbp.wrapping_sub(0x40u64))) as u32) as u128; rdx = rsp.wrapping_add(0x54); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r13 = (crate::simd_ops::ru64(rbp.wrapping_add(0x368u64))) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x60u64), ((rax as u32)) as u32); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rdi = ((rbx.wrapping_add(0x3u64)) as u32) as u32 as u64; bb = 21; continue; }
            2 => { rcx = rbp.wrapping_add(0xb0); crate::simd_ops::wu32(rsp.wrapping_add(0x70u64), ((rbx as u32)) as u32); crate::simd_ops::wu64(rbp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rbp.wrapping_add(0x30u64), rbx); crate::simd_ops::wu64(rbp.wrapping_add(0x28u64), rbx); crate::simd_ops::wu64(rbp.wrapping_add(0x20u64), rbx); crate::simd_ops::wu32(rbp.wrapping_add(0x7cu64), ((rbx as u32)) as u32); crate::simd_ops::wu64(rbp.wrapping_add(0x90u64), rbx); crate::simd_ops::wu16(rbp.wrapping_add(0x98u64), (0xff00u16) as u16); crate::simd_ops::wu64(rbp.wrapping_add(0xa8u64), rbx); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = ((crate::simd_ops::ru8(rbp.wrapping_add(0x2f8u64))) as u32) as u32 as u64; crate::simd_ops::lk_and_u8(rbp.wrapping_add(0x2f4u64), (0xfeu8) as u8); rax = (rax & !0xffu64) | ((((((rax as u8)) & (0xf3u8))) as u8 as u64)); rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x20u64))) as u64; rax = (rax & !0xffu64) | ((((((rax as u8)) | (0x3u8))) as u8 as u64)); crate::simd_ops::wu8(rbp.wrapping_add(0x2f8u64), ((rax as u8)) as u8); rax = (crate::simd_ops::ru32(r13.wrapping_add(0x14u64))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x34u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(r13.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x28u64), rax); rax = (crate::simd_ops::ru32(r13.wrapping_add(0x10u64))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x2f0u64), ((rbx as u32)) as u32); crate::simd_ops::wu32(rbp.wrapping_add(0x38u64), ((rbx as u32)) as u32); rbx = (crate::simd_ops::ru64(r13)) as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x30u64), ((rax as u32)) as u32); crate::simd_ops::wu64(rbp.wrapping_add(0x10u64), r14); fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 4; continue; }
            4 => { fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { fl_l = (crate::simd_ops::lk_add_u32(rbx, (1) as u32)) as u32 as u64; bb = 6; continue; }
            6 => { rcx = ((crate::simd_ops::ru8(rbp.wrapping_add(0x2f8u64))) as u32) as u32 as u64; rdx = 0x3b0; rax = (crate::simd_ops::ru32(rbp.wrapping_sub(0x50u64))) as u32 as u64; rcx = (rcx & !0xffu64) | ((((((rcx as u8)) & (0xfbu8))) as u8 as u64)); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x70u64))) as u128; xmm1 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x80u64))) as u128; crate::simd_ops::wu32(rbp.wrapping_add(0x78u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x388u64))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x84u64), ((rax as u32)) as u32); rax = ((crate::simd_ops::ru8(rbp.wrapping_add(0x370u64))) as u32) as u32 as u64; rax = (rax & !0xffu64) | (((((rax as u8)).wrapping_shl(2u32)) as u8 as u64)); rcx = (rcx & !0xffu64) | ((((((rcx as u8)) | ((rax as u8)))) as u8 as u64)); crate::simd_ops::wu64(rbp.wrapping_add(0x20u64), rbx); crate::simd_ops::wu128(rbp.wrapping_add(0x38u64), (xmm0) as u128); rbx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu8(rbp.wrapping_add(0x2f8u64), ((rcx as u8)) as u8); xmm0 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x70u64))) as u128; rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu128(rbp.wrapping_add(0x48u64), (xmm1) as u128); crate::simd_ops::wu32(rbp.wrapping_add(0x7cu64), ((rbx as u32)) as u32); xmm1 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x60u64))) as u128; crate::simd_ops::wu128(rbp.wrapping_add(0x58u64), (xmm0) as u128); crate::simd_ops::wu128(rbp.wrapping_add(0x68u64), (xmm1) as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 15; continue; } bb = 7; continue; }
            7 => { rcx = (crate::simd_ops::ru32(rbp.wrapping_sub(0x30u64))) as u32 as u64; rdx = rbp; xmm0 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x40u64))) as u128; r8 = (crate::simd_ops::ru64(rbp.wrapping_add(0x340u64))) as u64; r9 = r12; crate::simd_ops::wu32(rbp.wrapping_sub(0x10u64), ((rcx as u32)) as u32); rcx = (crate::simd_ops::ru32(rbp.wrapping_add(0x380u64))) as u32 as u64; crate::simd_ops::wu128(rbp.wrapping_sub(0x20u64), (xmm0) as u128); xmm0 = ((crate::simd_ops::ru32(rbp.wrapping_add(0x390u64))) as u32) as u128; crate::simd_ops::wu32(rsp.wrapping_add(0x40u64), ((xmm0 as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), ((rcx as u32)) as u32); rcx = rbp.wrapping_sub(0x20); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rcx); rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x378u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rcx); rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x350u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rcx); rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rsi, rax); crate::simd_ops::wu8(rbp.wrapping_add(0x398u64), ((rbx as u8)) as u8); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 16; continue; } bb = 8; continue; }
            8 => { xmm1 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x58u64))) as u32) as u128; rcx = rax.wrapping_add(0x18); rax = rbp.wrapping_add(0x398); r9 = (((rdi as u8)) as u32) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); r8 = 0; fl_l = 0; fl_r = 0; rax = rbp.wrapping_add(0xb0); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); rax = rsp.wrapping_add(0x54); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 17; continue; } bb = 9; continue; }
            9 => { fl_l = (crate::simd_ops::ru8(rbp.wrapping_add(0x398u64))) as u8 as u64; fl_r = ((rbx as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 12; continue; } bb = 10; continue; }
            10 => { fl_l = (crate::simd_ops::ru32(rsp.wrapping_add(0x5cu64))) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 12; continue; } bb = 11; continue; }
            11 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 13; continue; }
            12 => { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 13; continue; }
            13 => { r9 = (crate::simd_ops::ru32(rsp.wrapping_add(0x60u64))) as u32 as u64; rdx = r14; xmm2 = ((crate::simd_ops::ru32(rbp.wrapping_sub(0x40u64))) as u32) as u128; rcx = r12; crate::simd_ops::wu8(rsp.wrapping_add(0x38u64), ((rax as u8)) as u8); rax = (crate::simd_ops::ru64(rsi)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); rax = rsp.wrapping_add(0x54); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); rax = ((crate::simd_ops::ru8(rsp.wrapping_add(0x50u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x20u64), ((rax as u8)) as u8); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; crate::simd_ops::wu8(rsp.wrapping_add(0x51u64), (0x1u8) as u8); rdi = ((rax as u32)) as u32 as u64; rbx = (rbx & !0xffu64) | (((if (((fl_l as u32)) == ((fl_r as u32))) { 1u8 } else { 0u8 }) as u8 as u64)); crate::simd_ops::wu8(rsp.wrapping_add(0x50u64), ((rbx as u8)) as u8); if (((fl_l as u32)) != ((fl_r as u32))) { bb = 17; continue; } bb = 14; continue; }
            14 => { r8 = (crate::simd_ops::ru64(rsi)) as u64; rdx = rsp.wrapping_add(0x70); rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu16(rsp.wrapping_add(0x78u64), (0xff00u16) as u16); crate::simd_ops::wu64(rbp.wrapping_sub(0x78u64), rcx); r9 = (r9 & !0xffu64) | (((0x1u8) as u8 as u64)); crate::simd_ops::wu32(rbp.wrapping_sub(0x70u64), ((rcx as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), r14); rax = (crate::simd_ops::ru32(r8.wrapping_add(0x270u64))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_sub(0x6cu64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x388u64))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_sub(0x68u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(r12)) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x60u64), r8); r8 = ((r8).wrapping_add(0x90u64)) as u64; crate::simd_ops::wu32(rbp.wrapping_sub(0x64u64), ((rcx as u32)) as u32); rcx = r12; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x138u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = (crate::simd_ops::ru64(rsi)) as u64; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P10, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x360u64))) as u64; r9 = (r9 & !0xffu64) | (((0x1u8) as u8 as u64)); r8 = 0; fl_l = 0; fl_r = 0; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P11, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = ((crate::simd_ops::ru8(rsp.wrapping_add(0x50u64))) as u32) as u32 as u64; bb = 17; continue; }
            15 => { crate::simd_ops::wu64(rsi, rbx); crate::simd_ops::wu8(rbp.wrapping_add(0x398u64), ((rbx as u8)) as u8); bb = 16; continue; }
            16 => { rdi = 0x2; bb = 17; continue; }
            17 => { rcx = rbp.wrapping_add(0xb0); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P12, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x20u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 19; continue; } bb = 18; continue; }
            18 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 19; continue; }
            19 => { fl_l = ((rdi as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 24; continue; } bb = 20; continue; }
            20 => { fl_l = ((rbx as u8)) as u8 as u64; fl_r = ((rbx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 23; continue; } bb = 21; continue; }
            21 => { rdx = (crate::simd_ops::ru32(r13.wrapping_add(0x14u64))) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 23; continue; } bb = 22; continue; }
            22 => { r8 = (crate::simd_ops::ru64(rsi)) as u64; rax = rbp.wrapping_add(0x398); rcx = (crate::simd_ops::ru64(P13)) as u64; r9 = r14; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P14, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 23; continue; }
            23 => { rax = (crate::simd_ops::ru64(rsi)) as u64; rdx = rbp.wrapping_add(0x398); r8 = ((crate::simd_ops::ru8(rsp.wrapping_add(0x51u64))) as u32) as u32 as u64; r9 = r14; rcx = r12; crate::simd_ops::wu64(rbp.wrapping_add(0x398u64), rax); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P15, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = (crate::simd_ops::ru32(rsp.wrapping_add(0x54u64))) as u32 as u64; rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsi, rcx); fl_l = ((rdi as u32)) as u32 as u64; fl_r = 0x3; r8 = (crate::simd_ops::ru64(r14.wrapping_add(0x58u64))) as u64; rax = 0x1e; rdx = (if (((fl_l as u32)) != ((fl_r as u32))) { (rax as u32) } else { (rdx as u32) }) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_sub(0x4cu64), ((rcx as u32)) as u32); rax = rsp.wrapping_add(0x70); crate::simd_ops::wu32(rsp.wrapping_add(0x70u64), ((rcx as u32)) as u32); r9 = r13; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); rcx = r12; crate::simd_ops::wu32(rsp.wrapping_add(0x54u64), ((rdx as u32)) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P16, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 24; continue; }
            24 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x448u64))) as u64; rax = ((rdi as u32)) as u32 as u64; rsp = ((rsp).wrapping_add(0x400u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14152f9d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143930e1c; const P1: u64 = 0x143930e50; const P2: u64 = 0x68; const P3: u64 = 0x143930e1d; const P4: u64 = 0x58; const P5: u64 = 0x59; const P6: u64 = 0x78; const P7: u64 = 0x20; const P8: u64 = 0xe8; const P9: u64 = 0x2c; const P10: u64 = 0x143930e58; const P11: u64 = 0x141537640; const P12: u64 = 0x1438152b4; const P13: u64 = 0x1415326a0; const P14: u64 = 0x143930f08; const P15: u64 = 0x1415a9ec0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x50u64)) as u64; fl_l = (crate::simd_ops::ru8(P0)) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 5; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(P1)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 2; continue; }
            2 => { bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; rdx = ((crate::simd_ops::ru8(P3)) as u32) as u32 as u64; crate::simd_ops::wu8(rcx.wrapping_add(P4), (0x0u8) as u8); crate::simd_ops::wu8(rcx.wrapping_add(P5), ((rdx as u8)) as u8); rax = (crate::simd_ops::ru64(rax.wrapping_add(P6))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } bb = 4; continue; }
            4 => { crate::simd_ops::wu8(P0, (0x0u8) as u8); crate::simd_ops::wu8(P3, (0x0u8) as u8); bb = 5; continue; }
            5 => { rbx = (crate::simd_ops::ru64(P1)) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 6; continue; }
            6 => { crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rdi); bb = 7; continue; }
            7 => { rcx = rbx; rbx = (crate::simd_ops::ru64(rbx.wrapping_add(P6))) as u64; fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(P7))) as u32 as u64; fl_r = 0x200000; rdi = (crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 10; continue; } bb = 8; continue; }
            8 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = rsp.wrapping_add(0x20); rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P8))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 10; continue; } bb = 9; continue; }
            9 => { rdx = (crate::simd_ops::ru32(rdi.wrapping_add(P9))) as u32 as u64; r8 = rsp.wrapping_add(0x20); rcx = P10; rax = crate::simd_ops::call_stk(P11, rax, rcx, rdx, r8, r9, rsp); bb = 10; continue; }
            10 => { rdx = ((crate::simd_ops::ru16(P12)) as u32) as u32 as u64; rcx = rdi; rax = crate::simd_ops::call_stk(P13, rax, rcx, rdx, r8, r9, rsp); fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 7; continue; } bb = 11; continue; }
            11 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; bb = 12; continue; }
            12 => { rcx = (crate::simd_ops::ru64(P14)) as u64; rsp = ((rsp).wrapping_add(0x50u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P15, rax, rcx, rdx, r8, r9, rsp); }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14152faa0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143930e50; const P1: u64 = 0x143930e48; const P2: u64 = 0x143930e58; const P3: u64 = 0x141537560; const P4: u64 = 0x143930f90; const P5: u64 = 0x14159b2e0; const P6: u64 = 0x1415a1730; const P7: u64 = 0x141532270; const P8: u64 = 0x1415242e0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; r9 = (crate::simd_ops::ru64(P0)) as u64; rsi = 0; fl_l = 0; fl_r = 0; rbx = rcx; rax = r9; rdx = ((rsi as u32)) as u32 as u64; fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 1; continue; }
            1 => { fl_l = rax; fl_r = rbx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 4; continue; } bb = 2; continue; }
            2 => { rdx = rax; rax = (crate::simd_ops::ru64(rax.wrapping_add(0x78u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 1; continue; } bb = 3; continue; }
            3 => { bb = 8; continue; }
            4 => { r8 = (crate::simd_ops::ru64(rax.wrapping_add(0x78u64))) as u64; fl_l = rax; fl_r = r9; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { crate::simd_ops::wu64(P0, r8); bb = 7; continue; }
            6 => { crate::simd_ops::wu64(rdx.wrapping_add(0x78u64), r8); bb = 7; continue; }
            7 => { rcx = (crate::simd_ops::ru64(P1)) as u64; fl_l = rax; fl_r = rcx; rcx = (if (((fl_l as u64)) == ((fl_r as u64))) { rdx } else { rcx }) as u64; crate::simd_ops::wu64(P1, rcx); bb = 8; continue; }
            8 => { rdi = (crate::simd_ops::ru64(rbx.wrapping_add(0x68u64))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 15; continue; } bb = 9; continue; }
            9 => { rdx = (crate::simd_ops::ru32(rdi.wrapping_add(0x2cu64))) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 14; continue; } bb = 10; continue; }
            10 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x20u64))) as u32 as u64; fl_r = 0x200000; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 12; continue; } bb = 11; continue; }
            11 => { rcx = P2; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru32(rdi.wrapping_add(0x2cu64))) as u32 as u64; bb = 12; continue; }
            12 => { rcx = (crate::simd_ops::ru64(P4)) as u64; r8 = rbx.wrapping_add(0x18); fl_l = rbx; fl_r = rbx; r8 = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rsi } else { r8 }) as u64; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0xb8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 13; continue; }
            13 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0xb8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rdx = 0x1; rcx = rax; r8 = (crate::simd_ops::ru64(rax)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(r8.wrapping_add(0x120u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0xb8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = rax; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); bb = 14; continue; }
            14 => { rcx = rdi; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rdx = rdi; rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); bb = 15; continue; }
            15 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14152fbb0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x18; const P2: u64 = 0x20; const P3: u64 = 0x10; const P4: u64 = 0x24; const P5: u64 = 0x1415242e0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdx = (crate::simd_ops::ru64(rcx)) as u64; rbx = rcx; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rdx)) as u64; crate::simd_ops::wu64(rcx, rax); fl_l = rdx; fl_r = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rcx.wrapping_add(P0), (0x0u64) as u64); bb = 3; continue; }
            3 => { r8 = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; fl_l = rdx; fl_r = r8; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 6; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru32(rcx.wrapping_add(P2))) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = ((r8.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rdx; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(P3))) as u64; crate::simd_ops::wu64(rdx, rax); rax = 0x1; crate::simd_ops::lk_sub_u32(rbx.wrapping_add(P4), (1) as u32); crate::simd_ops::wu64(rbx.wrapping_add(P3), rdx); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            6 => { rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::lk_sub_u32(rbx.wrapping_add(P4), (1) as u32); rax = 0x1; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            7 => { rax = 0x2; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14152fc30(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143930e20; const P1: u64 = 0x143930e50; const P2: u64 = 0x141533970; const P3: u64 = 0x1438152b4; const P4: u64 = 0x141589e40; const P5: u64 = 0x14152f0e0; const P6: u64 = 0x1415336d0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x58u64)) as u64; rdi = (crate::simd_ops::ru64(P0)) as u64; r15 = rcx; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { rax = rdi; bb = 2; continue; }
            2 => { rax = ((rax).wrapping_add(0x8u64)) as u64; fl_l = rax; fl_r = r15; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rdi = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x10u64))) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 27; continue; } bb = 5; continue; }
            5 => { crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), rbx); rbx = (crate::simd_ops::ru64(P1)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rbp); rbp = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rsi); rsi = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), r12); r12 = (crate::simd_ops::ru32(rsp.wrapping_add(0x78u64))) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), r13); r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), r14); r14 = ((rbp as u32)) as u32 as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 6; continue; }
            6 => { bb = 7; continue; }
            7 => { rax = ((crate::simd_ops::ru8(rbx.wrapping_add(0x62u64))) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 13; continue; } bb = 8; continue; }
            8 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x10u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 13; continue; } bb = 9; continue; }
            9 => { crate::simd_ops::wu8(rsp.wrapping_add(0x78u64), (0x0u8) as u8); rax = rsp.wrapping_add(0x88); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x68u64))) as u64; r9 = rsp.wrapping_add(0x80); r8 = rsp.wrapping_add(0x78); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); rdx = ((rsi as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru8(rsp.wrapping_add(0x78u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 13; continue; } bb = 10; continue; }
            10 => { fl_l = r14; fl_r = r14; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 11; continue; }
            11 => { fl_l = rax; fl_r = rbp; if ((((fl_l as u64)) as i64) >= (((fl_r as u64)) as i64)) { bb = 13; continue; } bb = 12; continue; }
            12 => { r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x88u64))) as u64; rbp = rax; r12 = (crate::simd_ops::ru32(rsp.wrapping_add(0x80u64))) as u32 as u64; r14 = rbx; bb = 13; continue; }
            13 => { rbx = (crate::simd_ops::ru64(rbx.wrapping_add(0x78u64))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 7; continue; } bb = 14; continue; }
            14 => { rax = ((crate::simd_ops::ru16(P3)) as u32) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rbp = ((rbp).wrapping_sub(rax)) as u64; fl_l = rbp; fl_r = rbp; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; if (((((fl_l as u64)) & ((fl_r as u64))) as i64) > 0) { bb = 21; continue; } bb = 15; continue; }
            15 => { rdx = (crate::simd_ops::ru32(rdi.wrapping_add(0xcu64))) as u32 as u64; r9 = (r9 & !0xffu64) | (((0x1u8) as u8 as u64)); r8 = 0; fl_l = 0; fl_r = 0; rcx = ((rsi as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x10u64))) as u32 as u64; rax = ((((rax as u32)) & (0xfffffffdu32))) as u32 as u64; fl_l = ((((rax as u32)) | (0x1u32))) as u32 as u64; fl_r = fl_l; rax = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x10u64), ((rax as u32)) as u32); bb = 16; continue; }
            16 => { fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 18; continue; } bb = 17; continue; }
            17 => { fl_l = (crate::simd_ops::lk_or_u32(rdi.wrapping_add(0x10u64), (0x1u32) as u32)) as u32 as u64; fl_r = fl_l; bb = 18; continue; }
            18 => { rdi = (crate::simd_ops::ru64(rdi)) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 16; continue; } bb = 19; continue; }
            19 => { rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); bb = 20; continue; }
            20 => { r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x58u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            21 => { rcx = (crate::simd_ops::ru64(r14.wrapping_add(0x68u64))) as u64; r9 = r13; r8 = ((r12 as u32)) as u32 as u64; rdx = r15; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 20; continue; } bb = 22; continue; }
            22 => { rdx = (crate::simd_ops::ru32(rdi.wrapping_add(0xcu64))) as u32 as u64; r9 = (r9 & !0xffu64) | (((0x1u8) as u8 as u64)); r8 = 0; fl_l = 0; fl_r = 0; rcx = ((rsi as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x10u64))) as u32 as u64; rax = ((((rax as u32)) & (0xfffffffdu32))) as u32 as u64; fl_l = ((((rax as u32)) | (0x1u32))) as u32 as u64; fl_r = fl_l; rax = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x10u64), ((rax as u32)) as u32); bb = 23; continue; }
            23 => { fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 25; continue; } bb = 24; continue; }
            24 => { fl_l = (crate::simd_ops::lk_or_u32(rdi.wrapping_add(0x10u64), (0x1u32) as u32)) as u32 as u64; fl_r = fl_l; bb = 25; continue; }
            25 => { rdi = (crate::simd_ops::ru64(rdi)) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 23; continue; } bb = 26; continue; }
            26 => { bb = 19; continue; }
            27 => { rax = ((((rax as u32)) & (0xfffffffdu32))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x10u64), ((rax as u32)) as u32); rsp = ((rsp).wrapping_add(0x58u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P5, rax, rcx, rdx, r8, r9, rsp); }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14152fde0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143930e50; const P1: u64 = 0xb8; const P2: u64 = 0x68; const P3: u64 = 0x30; const P4: u64 = 0x2c; const P5: u64 = 0x141530c20; const P6: u64 = 0x78;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = (crate::simd_ops::ru64(P0)) as u64; rbp = (((r9 as u8)) as u32) as u32 as u64; r14 = r8; rsi = rdx; r15 = rcx; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rdi); rdi = (crate::simd_ops::ru32(rsp.wrapping_add(0x60u64))) as u32 as u64; bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = r15; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 8; continue; } bb = 3; continue; }
            3 => { fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64; fl_l = rsi; fl_r = (crate::simd_ops::ru64(rcx.wrapping_add(P3))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 8; continue; } bb = 5; continue; }
            5 => { fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64; fl_l = ((rdi as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rcx.wrapping_add(P4))) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 8; continue; } bb = 7; continue; }
            7 => { r8 = (((rbp as u8)) as u32) as u32 as u64; rdx = r14; rcx = rbx; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); bb = 8; continue; }
            8 => { rbx = (crate::simd_ops::ru64(rbx.wrapping_add(P6))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 9; continue; }
            9 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; bb = 10; continue; }
            10 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14152fe80(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143930e50; const P1: u64 = 0x78; const P2: u64 = 0xb8; const P3: u64 = 0x68; const P4: u64 = 0x30; const P5: u64 = 0x2c; const P6: u64 = 0xe0; const P7: u64 = 0x143930f90; const P8: u64 = 0x14159af70;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((xmm3 as u32)) as u32); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = (crate::simd_ops::ru64(P0)) as u64; rsi = ((r8 as u32)) as u32 as u64; rbp = rdx; r14 = rcx; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), r15); r15 = ((crate::simd_ops::ru8(rsp.wrapping_add(0x60u64))) as u32) as u32 as u64; bb = 2; continue; }
            2 => { rdi = rbx; rbx = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64; rcx = rdi; rax = (crate::simd_ops::ru64(rdi)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rax; fl_r = r14; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 9; continue; } bb = 3; continue; }
            3 => { fl_l = rbp; fl_r = rbp; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(P3))) as u64; fl_l = (crate::simd_ops::ru64(rax.wrapping_add(P4))) as u64; fl_r = rbp; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 9; continue; } bb = 5; continue; }
            5 => { fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(P3))) as u64; fl_l = (crate::simd_ops::ru32(rax.wrapping_add(P5))) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 9; continue; } bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rdx = rsp.wrapping_add(0x58); r8 = (((r15 as u8)) as u32) as u32 as u64; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P6))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 9; continue; } bb = 8; continue; }
            8 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(P3))) as u64; rcx = (crate::simd_ops::ru64(P7)) as u64; rdx = (crate::simd_ops::ru32(rax.wrapping_add(P5))) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 9; continue; }
            9 => { fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 10; continue; }
            10 => { r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; bb = 11; continue; }
            11 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14152ff40(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143930e50; const P1: u64 = 0x78; const P2: u64 = 0xb8; const P3: u64 = 0x68; const P4: u64 = 0x30; const P5: u64 = 0x2c; const P6: u64 = 0xd8; const P7: u64 = 0x143930f90; const P8: u64 = 0x14159af70;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((r9 as u32)) as u32); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = (crate::simd_ops::ru64(P0)) as u64; rsi = ((r8 as u32)) as u32 as u64; rbp = rdx; r14 = rcx; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), r15); r15 = ((crate::simd_ops::ru8(rsp.wrapping_add(0x60u64))) as u32) as u32 as u64; bb = 2; continue; }
            2 => { rdi = rbx; rbx = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64; rcx = rdi; rax = (crate::simd_ops::ru64(rdi)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = r14; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 9; continue; } bb = 3; continue; }
            3 => { fl_l = rbp; fl_r = rbp; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(P3))) as u64; fl_l = (crate::simd_ops::ru64(rax.wrapping_add(P4))) as u64; fl_r = rbp; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 9; continue; } bb = 5; continue; }
            5 => { fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(P3))) as u64; fl_l = (crate::simd_ops::ru32(rax.wrapping_add(P5))) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 9; continue; } bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rdx = rsp.wrapping_add(0x58); r8 = (((r15 as u8)) as u32) as u32 as u64; rcx = rdi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P6))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 9; continue; } bb = 8; continue; }
            8 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(P3))) as u64; rcx = (crate::simd_ops::ru64(P7)) as u64; rdx = (crate::simd_ops::ru32(rax.wrapping_add(P5))) as u32 as u64; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); bb = 9; continue; }
            9 => { fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 10; continue; }
            10 => { r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; bb = 11; continue; }
            11 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141530000(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143930e50; const P1: u64 = 0x141533970; const P2: u64 = 0x1438152b4; const P3: u64 = 0x143930e30; const P4: u64 = 0x141524330; const P5: u64 = 0x143930e20; const P6: u64 = 0x143930e28; const P7: u64 = 0x143930e44; const P8: u64 = 0x1415336d0; const P9: u64 = 0x14152fbb0;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rbx = (crate::simd_ops::ru64(P0)) as u64; r12 = ((rdx as u32)) as u32 as u64; r14 = (crate::simd_ops::ru32(rsp.wrapping_add(0x80u64))) as u32 as u64; rsi = ((rcx as u32)) as u32 as u64; r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), rdi); rdi = 0; fl_l = 0; fl_r = 0; rbp = ((rdi as u32)) as u32 as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 1; continue; }
            1 => { bb = 2; continue; }
            2 => { rax = ((crate::simd_ops::ru8(rbx.wrapping_add(0x62u64))) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 8; continue; } bb = 3; continue; }
            3 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x10u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 8; continue; } bb = 4; continue; }
            4 => { crate::simd_ops::wu8(rsp.wrapping_add(0x80u64), (0x0u8) as u8); rax = rsp.wrapping_add(0x30); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x68u64))) as u64; r9 = rsp.wrapping_add(0x88); r8 = rsp.wrapping_add(0x80); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); rdx = ((rsi as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru8(rsp.wrapping_add(0x80u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 8; continue; } bb = 5; continue; }
            5 => { fl_l = rbp; fl_r = rbp; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { fl_l = rax; fl_r = rdi; if ((((fl_l as u64)) as i64) >= (((fl_r as u64)) as i64)) { bb = 8; continue; } bb = 7; continue; }
            7 => { r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rdi = rax; r14 = (crate::simd_ops::ru32(rsp.wrapping_add(0x88u64))) as u32 as u64; rbp = rbx; bb = 8; continue; }
            8 => { rbx = (crate::simd_ops::ru64(rbx.wrapping_add(0x78u64))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 9; continue; }
            9 => { rax = ((crate::simd_ops::ru16(P2)) as u32) as u32 as u64; rdi = ((rdi).wrapping_sub(rax)) as u64; fl_l = rdi; fl_r = rdi; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; if (((((fl_l as u64)) & ((fl_r as u64))) as i64) <= 0) { bb = 22; continue; } bb = 10; continue; }
            10 => { rax = (crate::simd_ops::ru64(P3)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 13; continue; } bb = 11; continue; }
            11 => { rdx = ((rax.wrapping_add(0x18u64)) as u32) as u32 as u64; rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 16; continue; } bb = 12; continue; }
            12 => { rcx = (crate::simd_ops::ru64(P3)) as u64; crate::simd_ops::wu64(rax, rcx); crate::simd_ops::wu64(P3, rax); bb = 13; continue; }
            13 => { r10 = (crate::simd_ops::ru64(P5)) as u64; rbx = rax.wrapping_add(0x8); rcx = (crate::simd_ops::ru64(P6)) as u64; fl_l = r10; fl_r = r10; rcx = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rax } else { rcx }) as u64; crate::simd_ops::wu64(P6, rcx); rdx = (crate::simd_ops::ru64(rax)) as u64; crate::simd_ops::wu64(P3, rdx); crate::simd_ops::wu64(rax, r10); crate::simd_ops::lk_add_u32(P7, (1) as u32); crate::simd_ops::wu64(P5, rax); fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 14; continue; }
            14 => { rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x68u64))) as u64; r9 = r15; r8 = ((r14 as u32)) as u32 as u64; rdx = rbx; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 23; continue; } bb = 15; continue; }
            15 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; rax = ((((rax as u32)) & (0xfffffffeu32))) as u32 as u64; crate::simd_ops::wu32(rbx, ((rsi as u32)) as u32); rax = ((((rax as u32)) | (0x2u32))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x4u64), ((r12 as u32)) as u32); crate::simd_ops::wu32(rbx.wrapping_add(0x8u64), ((rax as u32)) as u32); rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            16 => { rax = (crate::simd_ops::ru64(P5)) as u64; bb = 17; continue; }
            17 => { fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 22; continue; } bb = 18; continue; }
            18 => { bb = 19; continue; }
            19 => { fl_l = (crate::simd_ops::ru32(rax.wrapping_add(0x8u64))) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 21; continue; } bb = 20; continue; }
            20 => { fl_l = (crate::simd_ops::lk_or_u32(rax.wrapping_add(0x10u64), (0x1u32) as u32)) as u32 as u64; fl_r = fl_l; bb = 21; continue; }
            21 => { rax = (crate::simd_ops::ru64(rax)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 19; continue; } bb = 22; continue; }
            22 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x40u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            23 => { rcx = P5; rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); bb = 22; continue; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415301c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143930e50; const P1: u64 = 0x78; const P2: u64 = 0xb8; const P3: u64 = 0x68; const P4: u64 = 0x30; const P5: u64 = 0x2c; const P6: u64 = 0x141530d60;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = (crate::simd_ops::ru64(P0)) as u64; rsi = ((r9 as u32)) as u32 as u64; r14 = r8; rbp = rdx; r15 = rcx; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rdi); bb = 2; continue; }
            2 => { rdi = rbx; rbx = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64; rcx = rdi; rax = (crate::simd_ops::ru64(rdi)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = r15; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 8; continue; } bb = 3; continue; }
            3 => { fl_l = rbp; fl_r = rbp; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(P3))) as u64; fl_l = rbp; fl_r = (crate::simd_ops::ru64(rax.wrapping_add(P4))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 8; continue; } bb = 5; continue; }
            5 => { fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(P3))) as u64; fl_l = ((rsi as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rax.wrapping_add(P5))) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 8; continue; } bb = 7; continue; }
            7 => { r8 = 0xffffffff; rdx = r14; rcx = rdi; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); bb = 8; continue; }
            8 => { fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 9; continue; }
            9 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; bb = 10; continue; }
            10 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141530260(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143930e50; const P1: u64 = 0x68; const P2: u64 = 0x1415acc90; const P3: u64 = 0x78;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = (crate::simd_ops::ru64(P0)) as u64; rdi = (((rdx as u8)) as u32) as u32 as u64; rsi = rcx; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rcx = ((rcx).wrapping_add(0x60u64)) as u64; r8 = (((rdi as u8)) as u32) as u32 as u64; rdx = rsi; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 3; continue; }
            3 => { rbx = (crate::simd_ops::ru64(rbx.wrapping_add(P3))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 1; continue; } bb = 4; continue; }
            4 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415302c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14152f9d0; const P1: u64 = 0x14158bd20; const P2: u64 = 0x14152f310;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rdx = 0x10;
    rcx = P0;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rdx = 0x10;
    rcx = P2;
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return crate::simd_ops::tail_stk(P1, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415302f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415376e0; const P1: u64 = 0x58; const P2: u64 = 0x62; const P3: u64 = 0x142676f58; const P4: u64 = 0x142676fc0; const P5: u64 = 0x18; const P6: u64 = 0x142676fd0; const P7: u64 = 0x28; const P8: u64 = 0x20; const P9: u64 = 0x38; const P10: u64 = 0x40; const P11: u64 = 0x48;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rcx = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu32(rbx.wrapping_add(P1), (0x3f800000u32) as u32);
    crate::simd_ops::lk_and_u8(rbx.wrapping_add(P2), (0xe0u8) as u8);
    rax = P3;
    crate::simd_ops::wu64(rbx, rax);
    rax = P4;
    crate::simd_ops::wu64(rbx.wrapping_add(P5), rax);
    rax = P6;
    crate::simd_ops::wu64(rbx.wrapping_add(P7), rax);
    rax = rbx;
    crate::simd_ops::wu32(rbx.wrapping_add(P8), ((rcx as u32)) as u32);
    crate::simd_ops::wu64(rbx.wrapping_add(P9), rcx);
    crate::simd_ops::wu64(rbx.wrapping_add(P10), rcx);
    crate::simd_ops::wu64(rbx.wrapping_add(P11), rcx);
    crate::simd_ops::wu64(rbx.wrapping_add(0x50u64), rcx);
    crate::simd_ops::wu32(rbx.wrapping_add(0x5cu64), (0xffffffffu32) as u32);
    crate::simd_ops::wu16(rbx.wrapping_add(0x60u64), ((rcx as u16)) as u16);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141530360(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x40; const P1: u64 = 0x142676f58; const P2: u64 = 0x142676fc0; const P3: u64 = 0x18; const P4: u64 = 0x142676fd0; const P5: u64 = 0x28; const P6: u64 = 0x143930f80; const P7: u64 = 0x1415adef0; const P8: u64 = 0x48; const P9: u64 = 0x142676f40; const P10: u64 = 0x38; const P11: u64 = 0x141537700;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rax = P1;
    crate::simd_ops::wu64(rcx, rax);
    rax = P2;
    crate::simd_ops::wu64(rcx.wrapping_add(P3), rax);
    rax = P4;
    crate::simd_ops::wu64(rcx.wrapping_add(P5), rax);
    rbx = rcx;
    fl_l = rdx; fl_r = rdx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { r8 = ((rcx.wrapping_add(P5)) as u64) as u64; rcx = (crate::simd_ops::ru64(P6)) as u64; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); }
    rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P8))) as u64;
    fl_l = rdx; fl_r = rdx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rcx = (crate::simd_ops::ru64(P6)) as u64; r8 = ((rbx.wrapping_add(P5)) as u64) as u64; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); }
    rax = P9;
    crate::simd_ops::wu64(rbx.wrapping_add(P10), (0x0u64) as u64);
    rcx = rbx;
    crate::simd_ops::wu64(rbx.wrapping_add(P3), rax);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(P11, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415303f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142676f58; const P1: u64 = 0x40; const P2: u64 = 0x142676fc0; const P3: u64 = 0x18; const P4: u64 = 0x142676fd0; const P5: u64 = 0x28; const P6: u64 = 0x143930f80; const P7: u64 = 0x1415adef0; const P8: u64 = 0x48; const P9: u64 = 0x142676f40; const P10: u64 = 0x38; const P11: u64 = 0x141537700; const P12: u64 = 0x141834d30;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = P0; rsi = ((rdx as u32)) as u32 as u64; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; rbx = rcx; crate::simd_ops::wu64(rcx, rax); rax = P2; crate::simd_ops::wu64(rcx.wrapping_add(P3), rax); rax = P4; crate::simd_ops::wu64(rcx.wrapping_add(P5), rax); fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { r8 = ((rcx.wrapping_add(P5)) as u64) as u64; rcx = (crate::simd_ops::ru64(P6)) as u64; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            2 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P8))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(P6)) as u64; r8 = ((rbx.wrapping_add(P5)) as u64) as u64; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { rax = P9; crate::simd_ops::wu64(rbx.wrapping_add(P10), (0x0u64) as u64); rcx = rbx; crate::simd_ops::wu64(rbx.wrapping_add(P3), rax); rax = crate::simd_ops::call_stk(P11, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rsi as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rdx = 0x68; rcx = rbx; rax = crate::simd_ops::call_stk(P12, rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            6 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415304d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x38; const P1: u64 = 0x8; const P2: u64 = 0x50;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    fl_l = rax; fl_r = rax;
    if !(((((fl_l as u64)) & ((fl_r as u64))) != 0)) { crate::simd_ops::wu64(rcx.wrapping_add(P0), rdx); crate::simd_ops::wu64(rdx.wrapping_add(P1), rax); fl_l = (crate::simd_ops::lk_add_u32(rcx.wrapping_add(P2), (1) as u32)) as u32 as u64; return rax; }
    crate::simd_ops::wu64(rdx.wrapping_add(P1), rax);
    crate::simd_ops::wu64(rcx.wrapping_add(P0), rdx);
    fl_l = (crate::simd_ops::lk_add_u32(rcx.wrapping_add(P2), (1) as u32)) as u32 as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141530500(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x40; const P1: u64 = 0x1415ad830;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    fl_l = rcx; fl_r = rcx;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64;
    if !(((((fl_l as u8)) & ((fl_r as u8))) == 0)) { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
    rax = (rax & !0xffu64) | (((0u8) as u8 as u64));
    fl_l = 0; fl_r = 0;
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141530540(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x50; const P1: u64 = 0x38; const P2: u64 = 0x8; const P3: u64 = 0x62; const P4: u64 = 0x60; const P5: u64 = 0x1415307a0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; crate::simd_ops::lk_add_u32(rcx.wrapping_add(P0), (1) as u32); rdi = rcx; rbx = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { bb = 2; continue; }
            2 => { rcx = rbx; rbx = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(P3))) as u8 as u64; fl_r = (0x2u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rcx = rdi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P4))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            5 => { rcx = rdi; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P5, rax, rcx, rdx, r8, r9, rsp); }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415305a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x50; const P1: u64 = 0x38; const P2: u64 = 0x40; const P3: u64 = 0x10; const P4: u64 = 0x8; const P5: u64 = 0x48; const P6: u64 = 0x5c; const P7: u64 = 0x62; const P8: u64 = 0x60; const P9: u64 = 0x1415307a0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; crate::simd_ops::lk_add_u32(rcx.wrapping_add(P0), (1) as u32); rsi = ((rdx as u32)) as u32 as u64; rbx = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; rdi = rcx; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = ((rsi as u32)) as u32 as u64; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rbx.wrapping_add(P4))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 1; continue; } bb = 2; continue; }
            2 => { rbx = (crate::simd_ops::ru64(rdi.wrapping_add(P1))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 3; continue; }
            3 => { bb = 4; continue; }
            4 => { rcx = rbx; rbx = (crate::simd_ops::ru64(rbx.wrapping_add(P4))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P5))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 4; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(P6))) as u32 as u64; fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 8; continue; } bb = 6; continue; }
            6 => { fl_l = ((rsi as u32)) as u32 as u64; fl_r = 0xffffffff; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 8; continue; } bb = 7; continue; }
            7 => { crate::simd_ops::wu32(rdi.wrapping_add(P6), ((rsi as u32)) as u32); rax = ((rsi as u32)) as u32 as u64; bb = 8; continue; }
            8 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 13; continue; } bb = 9; continue; }
            9 => { rax = ((crate::simd_ops::ru8(rdi.wrapping_add(P7))) as u32) as u32 as u64; rcx = (((rax as u8)) as u32) as u32 as u64; fl_l = ((((rcx as u8)) & (0xfu8))) as u8 as u64; fl_r = fl_l; rcx = (rcx & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 13; continue; } bb = 10; continue; }
            10 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x10u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 13; continue; } bb = 11; continue; }
            11 => { fl_l = ((rcx as u8)) as u8 as u64; fl_r = (0x2u8) as u8 as u64; if (((fl_l as u8)) > ((fl_r as u8))) { bb = 14; continue; } bb = 12; continue; }
            12 => { rax = (rax & !0xffu64) | ((((((rax as u8)) & (0xf2u8))) as u8 as u64)); fl_l = ((((rax as u8)) | (0x2u8))) as u8 as u64; fl_r = fl_l; rax = (rax & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); crate::simd_ops::wu8(rdi.wrapping_add(P7), ((rax as u8)) as u8); bb = 14; continue; }
            13 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rcx = rdi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P8))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 14; continue; }
            14 => { rcx = rdi; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P9, rax, rcx, rdx, r8, r9, rsp); }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141530660(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x38; const P1: u64 = 0x18; const P2: u64 = 0x8; const P3: u64 = 0x62; const P4: u64 = 0x40; const P5: u64 = 0x143930f80; const P6: u64 = 0x1415add10;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rdi = rcx; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 1; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::lk_or_u8(rdi.wrapping_add(P3), (0x10u8) as u8); rdx = (crate::simd_ops::ru64(rdi.wrapping_add(P4))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(P5)) as u64; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415306b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x62;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = ((crate::simd_ops::ru8(rcx.wrapping_add(P0))) as u32) as u32 as u64;
    rax = (rax & !0xffu64) | ((((((rax as u8)) & (0xf1u8))) as u8 as u64));
    fl_l = ((((rax as u8)) | (0x1u8))) as u8 as u64; fl_r = fl_l;
    rax = (rax & !0xffu64) | ((((fl_l as u8)) as u8 as u64));
    crate::simd_ops::wu8(rcx.wrapping_add(P0), ((rax as u8)) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415306c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x38; const P1: u64 = 0x8; const P2: u64 = 0x30;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x30u64)) as u64;
    rbx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rdi = ((rdx as u32)) as u32 as u64;
    crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm6) as u128);
    xmm6 = (xmm2) as u128;
    fl_l = rbx; fl_r = rbx;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    loop { rcx = rbx; xmm2 = (xmm6) as u128; rbx = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64; rdx = ((rdi as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rbx; fl_r = rbx; if !(((((fl_l as u64)) & ((fl_r as u64))) != 0)) { break; } }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64;
    xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128;
    rsp = ((rsp).wrapping_add(0x30u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141530710(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x38; const P1: u64 = 0x20; const P2: u64 = 0x8; const P3: u64 = 0x62; const P4: u64 = 0x40; const P5: u64 = 0x143930f80; const P6: u64 = 0x1415adf30;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rdi = rcx; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 1; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::lk_and_u8(rdi.wrapping_add(P3), (0xefu8) as u8); rdx = (crate::simd_ops::ru64(rdi.wrapping_add(P4))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(P5)) as u64; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141530760(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x62;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = ((crate::simd_ops::ru8(rcx.wrapping_add(P0))) as u32) as u32 as u64;
    rax = (rax & !0xffu64) | ((((((rax as u8)) & (0xf4u8))) as u8 as u64));
    fl_l = ((((rax as u8)) | (0x4u8))) as u8 as u64; fl_r = fl_l;
    rax = (rax & !0xffu64) | ((((fl_l as u8)) as u8 as u64));
    crate::simd_ops::wu8(rcx.wrapping_add(P0), ((rax as u8)) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141530770(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x38; const P1: u64 = 0x8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    fl_l = rbx; fl_r = rbx;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    loop { rcx = rbx; rbx = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = rbx; fl_r = rbx; if !(((((fl_l as u64)) & ((fl_r as u64))) != 0)) { break; } }
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415307a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x50; const P1: u64 = 0x10; const P2: u64 = 0x38; const P3: u64 = 0x8; const P4: u64 = 0x1415307a0; const P5: u64 = 0x14152faa0; const P6: u64 = 0x1415242e0;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; fl_l = (crate::simd_ops::lk_add_u32(rcx.wrapping_add(P0), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; rbx = rcx; if (((fl_l as u32)) != 0) { bb = 13; continue; } bb = 1; continue; }
            1 => { rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 2; continue; }
            2 => { r10 = (crate::simd_ops::ru64(rdx.wrapping_add(P2))) as u64; r8 = 0; fl_l = 0; fl_r = 0; rax = r10; fl_l = r10; fl_r = r10; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 3; continue; }
            3 => { fl_l = rax; fl_r = rbx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 6; continue; } bb = 4; continue; }
            4 => { r8 = rax; rax = (crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } bb = 5; continue; }
            5 => { crate::simd_ops::wu64(rbx.wrapping_add(P1), rax); bb = 12; continue; }
            6 => { r9 = (crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64; fl_l = rax; fl_r = r10; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 8; continue; } bb = 7; continue; }
            7 => { crate::simd_ops::wu64(rdx.wrapping_add(P2), r9); bb = 9; continue; }
            8 => { crate::simd_ops::wu64(r8.wrapping_add(P3), r9); bb = 9; continue; }
            9 => { rcx = rdx; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 10; continue; }
            10 => { crate::simd_ops::wu64(rbx.wrapping_add(P1), (0x0u64) as u64); bb = 12; continue; }
            11 => { rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); bb = 12; continue; }
            12 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rdx = rbx; rcx = 0; fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P6, rax, rcx, rdx, r8, r9, rsp); }
            13 => { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141530830(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x38; const P1: u64 = 0x8; const P2: u64 = 0x1415307a0;
    let (mut rax, mut r11, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r11 = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; r8 = 0; fl_l = 0; fl_r = 0; r9 = rcx; rax = r11; fl_l = r11; fl_r = r11; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 1; continue; }
            1 => { fl_l = rax; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 4; continue; } bb = 2; continue; }
            2 => { r8 = rax; rax = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 1; continue; } bb = 3; continue; }
            3 => { return rax; }
            4 => { rcx = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; fl_l = rax; fl_r = r11; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { crate::simd_ops::wu64(r9.wrapping_add(P0), rcx); rcx = r9; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            6 => { crate::simd_ops::wu64(r8.wrapping_add(P1), rcx); rcx = r9; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            7 => { return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141530880(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x38; const P1: u64 = 0x28; const P2: u64 = 0x8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x30u64)) as u64;
    rbx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rdi = rdx;
    crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm6) as u128);
    xmm6 = (xmm2) as u128;
    fl_l = rbx; fl_r = rbx;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    loop { rax = (crate::simd_ops::ru64(rbx)) as u64; xmm2 = (xmm6) as u128; rdx = rdi; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64; fl_l = rbx; fl_r = rbx; if !(((((fl_l as u64)) & ((fl_r as u64))) != 0)) { break; } }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64;
    xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128;
    rsp = ((rsp).wrapping_add(0x30u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415308d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xfffffffffc000000; const P1: u64 = 0xffffffffffffffd8; const P2: u64 = 0x30; const P3: u64 = 0x28; const P4: u64 = 0x20; const P5: u64 = 0x18; const P6: u64 = 0x2c; const P7: u64 = 0x10;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = (rsi & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rbp = (((r9 as u8)) as u32) as u32 as u64; rdi = rdx; rbx = rcx; fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x1000000; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 7; continue; } bb = 1; continue; }
            1 => { fl_l = ((rdi as u32)) as u32 as u64; fl_r = 0x2000000; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 6; continue; } bb = 2; continue; }
            2 => { rax = ((rdx.wrapping_add(P0)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0xfbffffff; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 10; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; rdx = ((rcx.wrapping_add(P2)) as u64) as u64; rcx = ((rcx).wrapping_add(0xffffffffffffffd8u64)) as u64; crate::simd_ops::wu32(rdx, ((xmm2 as u32)) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rbp as u8)) as u8 as u64; fl_r = ((rbp as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 10; continue; } bb = 4; continue; }
            4 => { crate::simd_ops::wu64(rbx.wrapping_add(P4), (0x0u64) as u64); fl_l = ((rdi as u32)) as u32 as u64; fl_r = 0x4000000; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 10; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64; rcx = ((rbx.wrapping_add(P1)) as u64) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P5))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 10; continue; }
            6 => { rsi = (rsi & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; rdx = ((rcx.wrapping_add(P6)) as u64) as u64; rcx = ((rcx).wrapping_add(0xffffffffffffffd8u64)) as u64; crate::simd_ops::wu32(rdx, ((xmm2 as u32)) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rbp as u8)) as u8 as u64; fl_r = ((rbp as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 10; continue; } bb = 8; continue; }
            8 => { crate::simd_ops::wu64(rbx.wrapping_add(P5), (0x0u64) as u64); fl_l = ((rsi as u8)) as u8 as u64; fl_r = ((rsi as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64; rcx = ((rbx.wrapping_add(P1)) as u64) as u64; rdx = 0xffffffff; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P7))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 10; continue; }
            10 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415309d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x60; const P1: u64 = 0x1; const P2: u64 = 0x48; const P3: u64 = 0x4; const P4: u64 = 0x143930f80; const P5: u64 = 0x1415ada30; const P6: u64 = 0x1434268d0; const P7: u64 = 0x28; const P8: u64 = 0x1415ad8b0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 15>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x70u64)) as u64; r8 = ((crate::simd_ops::ru16(rcx.wrapping_add(P0))) as u32) as u32 as u64; rbx = rcx; rax = ((r8.wrapping_add(P1)) as u32) as u32 as u64; crate::simd_ops::wu16(rcx.wrapping_add(P0), ((rax as u16)) as u16); fl_l = ((r8 as u16)) as u16 as u64; fl_r = ((r8 as u16)) as u16 as u64; if ((((fl_l as u16)) & ((fl_r as u16))) != 0) { bb = 6; continue; } bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64; r8 = 0; fl_l = 0; fl_r = 0; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru32(rdx.wrapping_add(P3))) as u32 as u64; xmm3 = 0u128; crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((r8 as u32)) as u32); r8 = 0x4000000; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx)) as u32 as u64; rdx = rcx; rcx = (crate::simd_ops::ru64(P4)) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rax as u32)) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rsp = ((rsp).wrapping_add(0x70u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            3 => { xmm0 = ((crate::simd_ops::ru32(P6)) as u32) as u128; rax = ((rbx.wrapping_add(P7)) as u64) as u64; rcx = (crate::simd_ops::ru64(P4)) as u64; xmm1 = 0u128; fl_l = rbx; fl_r = rbx; crate::simd_ops::wu32(rsp.wrapping_add(0x50u64), ((xmm0 as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x54u64), ((xmm1 as u32)) as u32); rax = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { r8 } else { rax }) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x80u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rax); rax = (crate::simd_ops::ru32(rdx)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x58u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(P3))) as u32 as u64; rdx = rsp.wrapping_add(0x40); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), (0x4000000u64) as u64); crate::simd_ops::wu32(rsp.wrapping_add(0x5cu64), ((rax as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x60u64), (0x1cu32) as u32); crate::simd_ops::wu16(rsp.wrapping_add(0x64u64), (0x100u16) as u16); crate::simd_ops::wu8(rsp.wrapping_add(0x66u64), ((r8 as u8)) as u8); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rbx.wrapping_add(P2), rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(P7))) as u64; rcx = ((rbx.wrapping_add(P7)) as u64) as u64; xmm2 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x54u64))) as u32) as u128; r9 = (r9 & !0xffu64) | (((0x1u8) as u8 as u64)); rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 5; continue; }
            5 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; bb = 6; continue; }
            6 => { rsp = ((rsp).wrapping_add(0x70u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141530ad0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1434268d0; const P1: u64 = 0x143930f80; const P2: u64 = 0x1415ada30; const P3: u64 = 0x1415ad8b0; const P4: u64 = 0x1438152b4;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 15>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x70u64)) as u64; rax = ((crate::simd_ops::ru8(rcx.wrapping_add(0x62u64))) as u32) as u32 as u64; rdi = rdx; rax = (rax & !0xffu64) | ((((((rax as u8)) & (0xfu8))) as u8 as u64)); rbx = rcx; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if (((fl_l as u8)) > ((fl_r as u8))) { bb = 13; continue; } bb = 1; continue; }
            1 => { rdx = (crate::simd_ops::ru64(rcx.wrapping_add(0x40u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x80u64), rsi); fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x4u64))) as u32 as u64; rsi = 0; fl_l = 0; fl_r = 0; xmm3 = ((crate::simd_ops::ru32(P0)) as u32) as u128; r8 = 0x1000000; rcx = (crate::simd_ops::ru64(P1)) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((rsi as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdi)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rax as u32)) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 12; continue; }
            3 => { rcx = (crate::simd_ops::ru32(rdi)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 12; continue; } bb = 4; continue; }
            4 => { xmm1 = ((crate::simd_ops::ru32(P0)) as u32) as u128; rax = rbx.wrapping_add(0x28); rsi = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rsp.wrapping_add(0x58u64), ((rcx as u32)) as u32); rcx = (crate::simd_ops::ru64(P1)) as u64; rdx = rsp.wrapping_add(0x40); xmm0 = 0u128; crate::simd_ops::wu32(rsp.wrapping_add(0x54u64), ((xmm1 as u32)) as u32); fl_l = rbx; fl_r = rbx; crate::simd_ops::wu32(rsp.wrapping_add(0x50u64), ((xmm0 as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x88u64), r14); rax = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rsi } else { rax }) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), (0x1000000u64) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rax); r8 = 0; fl_l = 0; fl_r = 0; rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x4u64))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x5cu64), ((rax as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x60u64), (0x1cu32) as u32); crate::simd_ops::wu16(rsp.wrapping_add(0x64u64), (0x100u16) as u16); crate::simd_ops::wu8(rsp.wrapping_add(0x66u64), ((rsi as u8)) as u8); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rbx.wrapping_add(0x40u64), rax); r8 = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x28u64))) as u64; rcx = rbx.wrapping_add(0x28); xmm2 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x54u64))) as u32) as u128; r9 = (r9 & !0xffu64) | (((0x1u8) as u8 as u64)); rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 11; continue; }
            6 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 11; continue; } bb = 7; continue; }
            7 => { rcx = ((crate::simd_ops::ru16(P4)) as u32) as u32 as u64; rdx = (if (rax as u32 as i32) < 0 { 0xffffffffu32 } else { 0u32 }) as u64; let d = ((rcx as u32)) as i32 as i64; let n = (((rdx as u32 as u64) << 32) | (rax as u32 as u64)) as i64; if d != 0 { rax = n.wrapping_div(d) as i32 as u32 as u64; rdx = n.wrapping_rem(d) as i32 as u32 as u64; } fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) >= 0) { bb = 10; continue; } bb = 8; continue; }
            8 => { rcx = ((rax as u32)) as u32 as u64; rcx = (0u32.wrapping_sub((rcx as u32))) as u32 as u64; fl_l = (crate::simd_ops::ru32(r8.wrapping_add(0x18u64))) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 10; continue; } bb = 9; continue; }
            9 => { crate::simd_ops::wu32(r8.wrapping_add(0x18u64), ((rsi as u32)) as u32); bb = 11; continue; }
            10 => { fl_l = (crate::simd_ops::lk_add_u32(r8.wrapping_add(0x18u64), ((rax as u32)) as u32)) as u32 as u64; fl_r = fl_l; bb = 11; continue; }
            11 => { r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x88u64))) as u64; bb = 12; continue; }
            12 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x58u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; bb = 13; continue; }
            13 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; rsp = ((rsp).wrapping_add(0x70u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141530c20(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1434268d0; const P1: u64 = 0x143930f80; const P2: u64 = 0x1415ada30; const P3: u64 = 0x1415ad8b0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 15>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x70u64)) as u64; rbx = rcx; fl_l = ((r8 as u8)) as u8 as u64; fl_r = ((r8 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { rcx = ((crate::simd_ops::ru16(rcx.wrapping_add(0x60u64))) as u32) as u32 as u64; fl_l = ((rcx as u16)) as u16 as u64; fl_r = (0x1u16) as u16 as u64; if (((fl_l as u16)) <= ((fl_r as u16))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rcx = (rcx & !0xffffu64) | (((((rcx as u16)).wrapping_sub(1)) as u16 as u64)); crate::simd_ops::wu16(rbx.wrapping_add(0x60u64), ((rcx as u16)) as u16); rsp = ((rsp).wrapping_add(0x70u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            3 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x48u64))) as u64; r9 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x80u64), rdi); crate::simd_ops::wu16(rbx.wrapping_add(0x60u64), ((r9 as u16)) as u16); fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32 as u64; r8 = 0x8000000; xmm3 = ((crate::simd_ops::ru32(P0)) as u32) as u128; crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((r9 as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx)) as u32 as u64; rdx = rcx; rcx = (crate::simd_ops::ru64(P1)) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rax as u32)) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 9; continue; }
            5 => { r8 = (crate::simd_ops::ru32(rdx)) as u32 as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 8; continue; } bb = 6; continue; }
            6 => { xmm0 = ((crate::simd_ops::ru32(rbx.wrapping_add(0x58u64))) as u32) as u128; rax = rbx.wrapping_add(0x28); xmm1 = ((crate::simd_ops::ru32(P0)) as u32) as u128; fl_l = rbx; fl_r = rbx; rcx = (crate::simd_ops::ru64(P1)) as u64; rax = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { r9 } else { rax }) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x58u64), ((r8 as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rax); r8 = 0; fl_l = 0; fl_r = 0; rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32 as u64; rdx = rsp.wrapping_add(0x40); crate::simd_ops::wu32(rsp.wrapping_add(0x50u64), ((xmm0 as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x54u64), ((xmm1 as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), (0x8000000u64) as u64); crate::simd_ops::wu32(rsp.wrapping_add(0x5cu64), ((rax as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x60u64), (0x1cu32) as u32); crate::simd_ops::wu16(rsp.wrapping_add(0x64u64), (0x100u16) as u16); crate::simd_ops::wu8(rsp.wrapping_add(0x66u64), ((r9 as u8)) as u8); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rbx.wrapping_add(0x48u64), rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 9; continue; } bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x28u64))) as u64; rcx = rbx.wrapping_add(0x28); xmm2 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x54u64))) as u32) as u128; r9 = (r9 & !0xffu64) | (((0x1u8) as u8 as u64)); rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 9; continue; }
            8 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = rbx.wrapping_add(0x58); xmm2 = ((crate::simd_ops::ru32(P0)) as u32) as u128; rcx = rbx; crate::simd_ops::wu32(rbx.wrapping_add(0x58u64), (0x3f800000u32) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x28u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; rsp = ((rsp).wrapping_add(0x70u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x20u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141530d60(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143930f80; const P1: u64 = 0x1415adef0; const P2: u64 = 0x1415ada30; const P3: u64 = 0x1434268d0; const P4: u64 = 0x1415ad8b0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 15>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = ((rsp).wrapping_sub(0x70u64)) as u64; rax = ((crate::simd_ops::ru8(rcx.wrapping_add(0x62u64))) as u32) as u32 as u64; rsi = ((r8 as u32)) as u32 as u64; r8 = rdx; rbx = rcx; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x10u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = ((rsi as u32)) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x88u64))) as u64; rsp = ((rsp).wrapping_add(0x70u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); }
            2 => { rdx = (crate::simd_ops::ru64(rcx.wrapping_add(0x40u64))) as u64; rcx = (crate::simd_ops::ru32(r8)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x80u64), rdi); fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 3; continue; }
            3 => { rdi = 0; fl_l = 0; fl_r = 0; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 7; continue; } bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru64(P0)) as u64; r8 = rbx.wrapping_add(0x28); fl_l = rbx; fl_r = rbx; r8 = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rdi } else { r8 }) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rbx.wrapping_add(0x40u64), rdi); bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = ((rsi as u32)) as u32 as u64; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 6; continue; }
            6 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x88u64))) as u64; rsp = ((rsp).wrapping_add(0x70u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            7 => { rax = (crate::simd_ops::ru32(r8.wrapping_add(0x4u64))) as u32 as u64; xmm3 = 0u128; crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((rdi as u32)) as u32); r8 = 0x2000000; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rax as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rcx as u32)) as u32); rcx = (crate::simd_ops::ru64(P0)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 6; continue; }
            8 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 5; continue; } bb = 9; continue; }
            9 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x3u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 5; continue; } bb = 10; continue; }
            10 => { xmm0 = ((crate::simd_ops::ru32(P3)) as u32) as u128; rax = rbx.wrapping_add(0x28); rdi = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rsp.wrapping_add(0x58u64), ((rcx as u32)) as u32); rcx = (crate::simd_ops::ru64(P0)) as u64; rdx = rsp.wrapping_add(0x40); xmm1 = 0u128; crate::simd_ops::wu32(rsp.wrapping_add(0x50u64), ((xmm0 as u32)) as u32); fl_l = rbx; fl_r = rbx; crate::simd_ops::wu32(rsp.wrapping_add(0x54u64), ((xmm1 as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), (0x2000000u64) as u64); rax = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rdi } else { rax }) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x60u64), (0x1cu32) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rax); rax = (crate::simd_ops::ru32(r8.wrapping_add(0x4u64))) as u32 as u64; r8 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rsp.wrapping_add(0x5cu64), ((rax as u32)) as u32); crate::simd_ops::wu16(rsp.wrapping_add(0x64u64), (0x100u16) as u16); crate::simd_ops::wu8(rsp.wrapping_add(0x66u64), ((rdi as u8)) as u8); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rbx.wrapping_add(0x40u64), rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 6; continue; } bb = 11; continue; }
            11 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x28u64))) as u64; rcx = rbx.wrapping_add(0x28); xmm2 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x54u64))) as u32) as u128; r9 = (r9 & !0xffu64) | (((0x1u8) as u8 as u64)); rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 6; continue; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141530ec0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141593180; const P1: u64 = 0x380; const P2: u64 = 0x1415376e0; const P3: u64 = 0x3a8; const P4: u64 = 0x142676fd8; const P5: u64 = 0x305; const P6: u64 = 0x142677048; const P7: u64 = 0x10; const P8: u64 = 0x142677050; const P9: u64 = 0x18; const P10: u64 = 0x1426770e0; const P11: u64 = 0x80; const P12: u64 = 0x142677110;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x78u64))) as u32 as u64; r10 = r9; r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rdi = r8; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rax as u32)) as u32); r8 = r10; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rsi = rcx; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rdi; rcx = ((rsi.wrapping_add(P1)) as u64) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::lk_and_u32(rsi.wrapping_add(P3), (0xfffffffeu32) as u32); rax = P4; crate::simd_ops::lk_and_u8(rsi.wrapping_add(P5), (0xdfu8) as u8); xmm0 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x80u64))) as u32) as u128; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; crate::simd_ops::wu64(rsi, rax); rax = P6; crate::simd_ops::wu64(rsi.wrapping_add(P7), rax); rax = P8; crate::simd_ops::wu64(rsi.wrapping_add(P9), rax); rax = P10; crate::simd_ops::wu64(rsi.wrapping_add(P11), rax); rax = P12; crate::simd_ops::wu64(rsi.wrapping_add(0x380u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; crate::simd_ops::wu64(rsi.wrapping_add(0x398u64), rax); rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsi.wrapping_add(0x3a0u64), rax); rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x308u64))) as u32 as u64; rax = ((((rax as u32)) & (0xfffffff9u32))) as u32 as u64; crate::simd_ops::wu32(rsi.wrapping_add(0x2acu64), ((xmm0 as u32)) as u32); rax = ((((rax as u32)) | (0x1u32))) as u32 as u64; crate::simd_ops::wu32(rsi.wrapping_add(0x308u64), ((rax as u32)) as u32); rax = rsi; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141530fa8(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141530fc0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = rcx; fl_r = (0x380u64) as u64;
    rcx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64;
    return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141530fc0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142676fd8; const P1: u64 = 0x142677048; const P2: u64 = 0x10; const P3: u64 = 0x142677050; const P4: u64 = 0x18; const P5: u64 = 0x1426770e0; const P6: u64 = 0x80; const P7: u64 = 0x142677110; const P8: u64 = 0x141537700; const P9: u64 = 0x1415935e0; const P10: u64 = 0x141834d30;
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
    rax = P1;
    crate::simd_ops::wu64(rcx.wrapping_add(P2), rax);
    rax = P3;
    crate::simd_ops::wu64(rcx.wrapping_add(P4), rax);
    rax = P5;
    crate::simd_ops::wu64(rcx.wrapping_add(P6), rax);
    rcx = ((rcx).wrapping_add(0x380u64)) as u64;
    rax = P7;
    crate::simd_ops::wu64(rcx, rax);
    rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp);
    rcx = rdi;
    rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp);
    fl_l = ((rbx as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64;
    if !(((((fl_l as u8)) & ((fl_r as u8))) == 0)) { rdx = 0x3b0; rcx = rdi; rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp); }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rax = rdi;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141531040(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x305; const P1: u64 = 0x290; const P2: u64 = 0xc; const P3: u64 = 0x1438152b4; const P4: u64 = 0x328; const P5: u64 = 0x2ac; const P6: u64 = 0x143426780; const P7: u64 = 0x143427068; const P8: u64 = 0x2fc; const P9: u64 = 0x306;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r8 = ((crate::simd_ops::ru8(rcx.wrapping_add(P0))) as u32) as u32 as u64; fl_l = ((r8 as u8)) as u8 as u64; fl_r = (0x40u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; rdx = (crate::simd_ops::ru32(rax.wrapping_add(P2))) as u32 as u64; rdx = (((rdx as u32)).wrapping_shr(1u32)) as u32 as u64; fl_l = ((rdx as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 11; continue; } bb = 2; continue; }
            2 => { rax = ((crate::simd_ops::ru16(P3)) as u32) as u32 as u64; xmm0 = 0u128; r10 = (crate::simd_ops::ru32(rcx.wrapping_add(P4))) as u32 as u64; xmm1 = (((rax as u32)) as u32) as u128; xmm1 = crate::simd_ops::vcvtdq2ps(xmm1); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(crate::simd_ops::ru32(rcx.wrapping_add(P5)))).to_bits() as u128); fl_l = ((xmm1 as u32)) as u32 as u64; fl_r = ((xmm0 as u32)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { xmm0 = ((crate::simd_ops::ru32(P6)) as u32) as u128; bb = 5; continue; }
            4 => { xmm0 = ((crate::simd_ops::ru32(P7)) as u32) as u128; bb = 5; continue; }
            5 => { xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(xmm1 as u32)).to_bits() as u128); rax = ((r10 as u32)) as u32 as u64; rdx = (if (rax as u32 as i32) < 0 { 0xffffffffu32 } else { 0u32 }) as u64; r11 = (crate::simd_ops::cvt_f32_i32((f32::from_bits(xmm0 as u32)))) as u32 as u64; let d = ((r11 as u32)) as i32 as i64; let n = (((rdx as u32 as u64) << 32) | (rax as u32 as u64)) as i64; if d != 0 { rax = n.wrapping_div(d) as i32 as u32 as u64; rdx = n.wrapping_rem(d) as i32 as u32 as u64; } rax = ((r11 as u32)) as u32 as u64; r9 = ((rdx as u32)) as u32 as u64; rdx = (if (rax as u32 as i32) < 0 { 0xffffffffu32 } else { 0u32 }) as u64; rax = (((rax as u32)).wrapping_sub((rdx as u32))) as u32 as u64; rax = (((((rax as u32)) as i32).wrapping_shr(1u32) as u32)) as u32 as u64; fl_l = ((r9 as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { bb = 9; continue; } bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru32(rcx.wrapping_add(P8))) as u32 as u64; fl_l = ((r9 as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { bb = 9; continue; } bb = 7; continue; }
            7 => { fl_l = ((r9 as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 11; continue; } bb = 8; continue; }
            8 => { r10 = (((r10 as u32)).wrapping_sub((r9 as u32))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(P4), ((r10 as u32)) as u32); crate::simd_ops::wu32(rcx.wrapping_add(P8), ((rax as u32)) as u32); bb = 10; continue; }
            9 => { r11 = (((r11 as u32)).wrapping_sub((r9 as u32))) as u32 as u64; fl_l = (crate::simd_ops::lk_add_u32(rcx.wrapping_add(P8), ((r11 as u32)) as u32)) as u32 as u64; fl_r = fl_l; rax = ((r11.wrapping_add(r10)) as u32) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(P4), ((rax as u32)) as u32); bb = 10; continue; }
            10 => { crate::simd_ops::lk_and_u8(rcx.wrapping_add(P9), (0xfeu8) as u8); r8 = (r8 & !0xffu64) | ((((((r8 as u8)) & (0x3fu8))) as u8 as u64)); fl_l = ((((r8 as u8)) | (0x40u8))) as u8 as u64; fl_r = fl_l; r8 = (r8 & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); crate::simd_ops::wu8(rcx.wrapping_add(P0), ((r8 as u8)) as u8); bb = 11; continue; }
            11 => { fl_l = (crate::simd_ops::lk_or_u8(rcx.wrapping_add(P9), (0x8u8) as u8)) as u8 as u64; fl_r = fl_l; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141531120(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x378; const P1: u64 = 0x3a8;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rdx = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64;
    rax = 0xffffffff;
    fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(P1))) as u8 as u64; fl_r = (0x1u8) as u8 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(P0), ((rax as u32)) as u32);
    rax = (if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { (rdx as u32) } else { (rax as u32) }) as u32 as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141531140(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = (0x7ffffe3fffe60b9u64) as u64;
    crate::simd_ops::wu64(rdx, rax);
    rax = rdx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141531160(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141524340; const P1: u64 = 0x141524330; const P2: u64 = 0x1415242e0;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = ((rsp).wrapping_sub(0x28u64)) as u64; r13 = (crate::simd_ops::ru32(rcx.wrapping_add(0xcu64))) as u32 as u64; rbx = rcx; rax = (crate::simd_ops::ru64(rcx)) as u64; r13 = (((r13 as u32)).wrapping_add((rdx as u32))) as u32 as u64; rdi = (crate::simd_ops::ru32(rcx.wrapping_add(0x8u64))) as u32 as u64; rcx = 0; fl_l = 0; fl_r = 0; rdx = ((r13 as u32)) as u32 as u64; rdx = ((rdx).wrapping_shl(4u32)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { r8 = rdx; rdx = rax; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rsi = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 2; continue; }
            2 => { bb = 25; continue; }
            3 => { rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rsi = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x28u64)) as u64; r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            5 => { rax = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 25; continue; } bb = 6; continue; }
            6 => { fl_l = rax; fl_r = rsi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 25; continue; } bb = 7; continue; }
            7 => { rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rbp); rbp = ((rcx as u32)) as u32 as u64; fl_l = rdi; fl_r = (0x4u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 19; continue; } bb = 8; continue; }
            8 => { crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), r12); r11 = rdi.wrapping_sub(0x4); crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), r14); rdx = rsi.wrapping_add(0x18); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r15); r14 = 0x18; r15 = 0x8; r11 = ((r11).wrapping_shr(2u32)) as u64; r12 = (0xffffffffffffffe8u64) as u64; r15 = ((r15).wrapping_sub(rsi)) as u64; r14 = ((r14).wrapping_sub(rsi)) as u64; r12 = ((r12).wrapping_sub(rsi)) as u64; fl_l = ((r11).wrapping_add(1)) as u64; r11 = ((fl_l as u64)) as u64; rbp = ((r11.wrapping_mul(4)) as u64) as u64; bb = 9; continue; }
            9 => { r9 = rdx.wrapping_sub(0x18); fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { crate::simd_ops::wu32(r9, ((rcx as u32)) as u32); crate::simd_ops::wu64(rdx.wrapping_sub(0x14u64), (0x0u64) as u64); bb = 11; continue; }
            11 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; r10 = ((r12.wrapping_add(rdx)) as u64) as u64; r8 = rdx.wrapping_sub(0x8); rax = (crate::simd_ops::ru32(r10.wrapping_add(rcx))) as u32 as u64; crate::simd_ops::wu32(r9, ((rax as u32)) as u32); r9 = 0; fl_l = 0; fl_r = 0; rax = (crate::simd_ops::ru32(r10.wrapping_add(rcx).wrapping_add(0x4u64))) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_sub(0x14u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(r10.wrapping_add(rcx).wrapping_add(0x8u64))) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_sub(0x10u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(r10.wrapping_add(rcx).wrapping_add(0xcu64))) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_sub(0xcu64), ((rax as u32)) as u32); fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 12; continue; }
            12 => { crate::simd_ops::wu32(r8, ((r9 as u32)) as u32); crate::simd_ops::wu64(rdx.wrapping_sub(0x4u64), r9); bb = 13; continue; }
            13 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; rax = (crate::simd_ops::ru32(r10.wrapping_add(rcx).wrapping_add(0x10u64))) as u32 as u64; crate::simd_ops::wu32(r8, ((rax as u32)) as u32); r8 = rdx.wrapping_add(0x8); rax = (crate::simd_ops::ru32(r10.wrapping_add(rcx).wrapping_add(0x14u64))) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_sub(0x4u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(r10.wrapping_add(rcx).wrapping_add(0x18u64))) as u32 as u64; crate::simd_ops::wu32(rdx, ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(r10.wrapping_add(rcx).wrapping_add(0x1cu64))) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_add(0x4u64), ((rax as u32)) as u32); fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 15; continue; } bb = 14; continue; }
            14 => { crate::simd_ops::wu32(r8, ((r9 as u32)) as u32); crate::simd_ops::wu64(rdx.wrapping_add(0xcu64), (0x0u64) as u64); bb = 15; continue; }
            15 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; rcx = ((rcx).wrapping_add(r15)) as u64; rcx = ((rcx).wrapping_add(rdx)) as u64; rax = (crate::simd_ops::ru32(rcx)) as u32 as u64; crate::simd_ops::wu32(r8, ((rax as u32)) as u32); r8 = rdx.wrapping_add(0x18); rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x4u64))) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_add(0xcu64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x8u64))) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_add(0x10u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rcx.wrapping_add(0xcu64))) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_add(0x14u64), ((rax as u32)) as u32); fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 16; continue; }
            16 => { crate::simd_ops::wu32(r8, ((r9 as u32)) as u32); crate::simd_ops::wu64(rdx.wrapping_add(0x1cu64), (0x0u64) as u64); bb = 17; continue; }
            17 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; rcx = ((rcx).wrapping_add(r14)) as u64; rcx = ((rcx).wrapping_add(rdx)) as u64; rax = (crate::simd_ops::ru32(rcx)) as u32 as u64; crate::simd_ops::wu32(r8, ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x4u64))) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_add(0x1cu64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x8u64))) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_add(0x20u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rcx.wrapping_add(0xcu64))) as u32 as u64; rcx = 0x0; crate::simd_ops::wu32(rdx.wrapping_add(0x24u64), ((rax as u32)) as u32); rdx = ((rdx).wrapping_add(0x40u64)) as u64; fl_l = r11; fl_r = (0x1u64) as u64; r11 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 9; continue; } bb = 18; continue; }
            18 => { r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; bb = 19; continue; }
            19 => { fl_l = rbp; fl_r = rdi; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 24; continue; } bb = 20; continue; }
            20 => { rdx = rbp; r10 = (0xfffffffffffffff8u64) as u64; rdx = ((rdx).wrapping_shl(4u32)) as u64; r10 = ((r10).wrapping_sub(rsi)) as u64; rdx = ((rdx).wrapping_add(0x8u64)) as u64; rdx = ((rdx).wrapping_add(rsi)) as u64; fl_l = rdi; fl_r = rbp; rdi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; bb = 21; continue; }
            21 => { r9 = rdx.wrapping_sub(0x8); fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 23; continue; } bb = 22; continue; }
            22 => { crate::simd_ops::wu32(r9, ((rcx as u32)) as u32); crate::simd_ops::wu64(rdx.wrapping_sub(0x4u64), (0x0u64) as u64); bb = 23; continue; }
            23 => { rcx = ((r10.wrapping_add(rdx)) as u64) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rbx))) as u64; rax = (crate::simd_ops::ru32(rcx)) as u32 as u64; crate::simd_ops::wu32(r9, ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x4u64))) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_sub(0x4u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x8u64))) as u32 as u64; crate::simd_ops::wu32(rdx, ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rcx.wrapping_add(0xcu64))) as u32 as u64; rcx = 0x0; crate::simd_ops::wu32(rdx.wrapping_add(0x4u64), ((rax as u32)) as u32); rdx = ((rdx).wrapping_add(0x10u64)) as u64; fl_l = rdi; fl_r = (0x1u64) as u64; rdi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 21; continue; } bb = 24; continue; }
            24 => { rdx = (crate::simd_ops::ru64(rbx)) as u64; rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; bb = 25; continue; }
            25 => { crate::simd_ops::wu64(rbx, rsi); rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); crate::simd_ops::wu32(rbx.wrapping_add(0xcu64), ((r13 as u32)) as u32); rsp = ((rsp).wrapping_add(0x28u64)) as u64; r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415313a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141537740; const P1: u64 = 0x1415947d0;
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    rcx = ((rcx).wrapping_add(0x368u64)) as u64;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rcx = rbx;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(P1, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415313e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x390; const P1: u64 = 0x141538190; const P2: u64 = 0x306; const P3: u64 = 0x328; const P4: u64 = 0x2fc; const P5: u64 = 0x305;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    r9 = rsp.wrapping_add(0x40);
    rdx = rcx;
    r8 = rsp.wrapping_add(0x30);
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64;
    if !(((((fl_l as u8)) & ((fl_r as u8))) == 0)) { rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x30u64))) as u32 as u64; crate::simd_ops::lk_and_u8(rbx.wrapping_add(P2), (0xfeu8) as u8); crate::simd_ops::wu32(rbx.wrapping_add(P3), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x40u64))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(P4), ((rax as u32)) as u32); rax = ((crate::simd_ops::ru8(rbx.wrapping_add(P5))) as u32) as u32 as u64; rax = (rax & !0xffu64) | ((((((rax as u8)) & (0x3fu8))) as u8 as u64)); rax = (rax & !0xffu64) | ((((((rax as u8)) | (0x40u8))) as u8 as u64)); crate::simd_ops::wu8(rbx.wrapping_add(P5), ((rax as u8)) as u8); rax = 0x1; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    rax = 0x2;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141531450(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xffffffffffffff2c; const P1: u64 = 0x143426780; const P2: u64 = 0x143427068; const P3: u64 = 0xfffffffffffffff8; const P4: u64 = 0x28; const P5: u64 = 0x1415970c0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x28u64)) as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0xffffffff; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 6; continue; } bb = 1; continue; }
            1 => { xmm1 = 0u128; rax = ((rdx as u32)) as u32 as u64; xmm0 = 0u128; xmm1 = (xmm1 & !0xffff_ffffu128) | ((((rax) as i64) as f32).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) / f32::from_bits(crate::simd_ops::ru32(rcx.wrapping_add(P0)))).to_bits() as u128); fl_l = ((xmm1 as u32)) as u32 as u64; fl_r = ((xmm0 as u32)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 3; continue; } bb = 2; continue; }
            2 => { xmm0 = ((crate::simd_ops::ru32(P1)) as u32) as u128; bb = 4; continue; }
            3 => { xmm0 = ((crate::simd_ops::ru32(P2)) as u32) as u128; bb = 4; continue; }
            4 => { xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(xmm1 as u32)).to_bits() as u128); rax = (crate::simd_ops::cvt_f32_i32((f32::from_bits(xmm0 as u32)))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rcx.wrapping_add(P3))) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 6; continue; } bb = 5; continue; }
            5 => { crate::simd_ops::wu32(rcx.wrapping_add(P3), ((rax as u32)) as u32); bb = 6; continue; }
            6 => { fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(P3))) as u32 as u64; fl_r = 0xffffffff; crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), (0x0u32) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x34u64), (0x4u32) as u32); if (((fl_l as u32)) == ((fl_r as u32))) { bb = 9; continue; } bb = 7; continue; }
            7 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(P4))) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { r8 = 0; fl_l = 0; fl_r = 0; bb = 10; continue; }
            9 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 10; continue; }
            10 => { rcx = ((rcx).wrapping_add(0xfffffffffffffc80u64)) as u64; rdx = rsp.wrapping_add(0x30); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rsp = ((rsp).wrapping_add(0x28u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415314d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x18;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = ((rcx).wrapping_add(0xfffffffffffffc80u64)) as u64; fl_r = fl_l;
    rcx = ((fl_l as u64)) as u64;
    rdx = (rdx & !0xffu64) | (((0x1u8) as u8 as u64));
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    return crate::simd_ops::call_fwd(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415314e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0xfffffffffffffe90; const P1: u64 = 0xffffffffffffff2c;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x36;
    if !((((fl_l as u32)) != ((fl_r as u32)))) { fl_l = (crate::simd_ops::lk_and_u8(rcx.wrapping_add(P0), (0xfeu8) as u8)) as u8 as u64; fl_r = fl_l; crate::simd_ops::wu32(rcx.wrapping_add(P1), ((xmm2 as u32)) as u32); }
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141531500(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x20; const P1: u64 = 0xfffffffffffffc98; const P2: u64 = 0xffffffffffffff34; const P3: u64 = 0x78;
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    rcx = ((rcx).wrapping_add(0xfffffffffffffc80u64)) as u64;
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp);
    rcx = ((rbx.wrapping_add(P1)) as u64) as u64;
    crate::simd_ops::wu32(rbx.wrapping_add(P2), (0x3f800000u32) as u32);
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141531540(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141595410; const P1: u64 = 0x14183ad64; const P2: u64 = 0x142677168; const P3: u64 = 0x14268918c; const P4: u64 = 0x143930f44; const P5: u64 = 0x1415317a0; const P6: u64 = 0x142689190;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0; let mut xmm7: u128 = 0; let mut xmm8: u128 = 0; let mut xmm9: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 15>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x70u64)) as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x60u64), (xmm6) as u128); rbx = rcx; crate::simd_ops::wu128(rsp.wrapping_add(0x50u64), (xmm7) as u128); crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm8) as u128); crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm9) as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm0 = ((crate::simd_ops::ru32(rbx.wrapping_add(0x294u64))) as u32) as u128; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(crate::simd_ops::ru32(P2))).to_bits() as u128); rsi = 0; fl_l = 0; fl_r = 0; xmm7 = ((crate::simd_ops::ru32(rbx.wrapping_add(0x388u64))) as u32) as u128; xmm8 = 0u128; crate::simd_ops::wu32(rbx.wrapping_add(0x8cu64), ((xmm0 as u32)) as u32); rdi = (crate::simd_ops::ru32(P3)) as u32 as u64; fl_l = ((rdi as u32)) as u32 as u64; fl_r = 0x1a; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 17; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru32(P4)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 16; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0x124u64))) as u8 as u64; fl_r = (0x8u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 16; continue; } bb = 3; continue; }
            3 => { fl_l = ((xmm7 as u32)) as u32 as u64; fl_r = ((xmm8 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) == f32::from_bits((fl_r as u32)) || f32::from_bits((fl_l as u32)).is_nan() || f32::from_bits((fl_r as u32)).is_nan()) { bb = 17; continue; } bb = 4; continue; }
            4 => { crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rsi as u32)) as u32); rcx = rbx.wrapping_add(0xf0); crate::simd_ops::wu16(rsp.wrapping_add(0x20u64), (0x403u16) as u16); r8 = rsp.wrapping_add(0x80); crate::simd_ops::wu8(rsp.wrapping_add(0x22u64), ((rsi as u8)) as u8); rdx = rsp.wrapping_add(0x20); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x20u64))) as u32 as u64; xmm9 = 0u128; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rax as u32)) as u32); xmm6 = 0u128; crate::simd_ops::wu8(rsp.wrapping_add(0x80u64), ((rsi as u8)) as u8); crate::simd_ops::wu64(rsp.wrapping_add(0x24u64), rsi); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 5; continue; }
            5 => { fl_l = (crate::simd_ops::ru8(rsp.wrapping_add(0x80u64))) as u8 as u64; fl_r = ((rsi as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 7; continue; } bb = 6; continue; }
            6 => { crate::simd_ops::wu32(rax.wrapping_add(0xcu64), ((rsi as u32)) as u32); bb = 7; continue; }
            7 => { xmm9 = ((crate::simd_ops::ru32(rax.wrapping_add(0xcu64))) as u32) as u128; xmm6 = (xmm9) as u128; xmm6 = (xmm6 & !0xffff_ffffu128) | ((f32::from_bits(xmm6 as u32) + f32::from_bits(xmm7 as u32)).to_bits() as u128); crate::simd_ops::wu32(rax.wrapping_add(0xcu64), ((xmm6 as u32)) as u32); bb = 8; continue; }
            8 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0x130u64))) as u8 as u64; fl_r = (0x8u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { fl_l = (crate::simd_ops::lk_or_u8(rbx.wrapping_add(0x13fu64), (0x4u8) as u8)) as u8 as u64; fl_r = fl_l; bb = 17; continue; }
            10 => { xmm0 = ((crate::simd_ops::ru32(rbx.wrapping_add(rdi.wrapping_mul(4)).wrapping_add(0x88u64))) as u32) as u128; fl_l = ((xmm9 as u32)) as u32 as u64; fl_r = ((xmm0 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) != f32::from_bits((fl_r as u32)) && !f32::from_bits((fl_l as u32)).is_nan() && !f32::from_bits((fl_r as u32)).is_nan()) { bb = 13; continue; } bb = 11; continue; }
            11 => { fl_l = ((xmm6 as u32)) as u32 as u64; fl_r = ((xmm0 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 14; continue; } bb = 12; continue; }
            12 => { fl_l = (crate::simd_ops::lk_or_u8(rbx.wrapping_add(0x13fu64), (0x4u8) as u8)) as u8 as u64; fl_r = fl_l; bb = 17; continue; }
            13 => { fl_l = ((xmm6 as u32)) as u32 as u64; fl_r = ((xmm0 as u32)) as u32 as u64; bb = 14; continue; }
            14 => { if !(f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 17; continue; } bb = 15; continue; }
            15 => { crate::simd_ops::wu32(rbx.wrapping_add(rdi.wrapping_mul(4)).wrapping_add(0x88u64), ((xmm6 as u32)) as u32); bb = 17; continue; }
            16 => { xmm7 = (xmm7 & !0xffff_ffffu128) | ((f32::from_bits(xmm7 as u32) + f32::from_bits(crate::simd_ops::ru32(rbx.wrapping_add(rdi.wrapping_mul(4)).wrapping_add(0x88u64)))).to_bits() as u128); crate::simd_ops::wu32(rbx.wrapping_add(rdi.wrapping_mul(4)).wrapping_add(0x88u64), ((xmm7 as u32)) as u32); bb = 17; continue; }
            17 => { rdi = (crate::simd_ops::ru32(P6)) as u32 as u64; xmm7 = ((crate::simd_ops::ru32(rbx.wrapping_add(0x38cu64))) as u32) as u128; xmm9 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; fl_l = ((rdi as u32)) as u32 as u64; fl_r = 0x1a; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 33; continue; } bb = 18; continue; }
            18 => { fl_l = (crate::simd_ops::ru32(P4)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 32; continue; } bb = 19; continue; }
            19 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0x124u64))) as u8 as u64; fl_r = (0x10u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 32; continue; } bb = 20; continue; }
            20 => { fl_l = ((xmm7 as u32)) as u32 as u64; fl_r = ((xmm8 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) == f32::from_bits((fl_r as u32)) || f32::from_bits((fl_l as u32)).is_nan() || f32::from_bits((fl_r as u32)).is_nan()) { bb = 33; continue; } bb = 21; continue; }
            21 => { crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rsi as u32)) as u32); rcx = rbx.wrapping_add(0xf0); crate::simd_ops::wu16(rsp.wrapping_add(0x20u64), (0x404u16) as u16); r8 = rsp.wrapping_add(0x80); crate::simd_ops::wu8(rsp.wrapping_add(0x22u64), ((rsi as u8)) as u8); rdx = rsp.wrapping_add(0x20); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x20u64))) as u32 as u64; xmm6 = 0u128; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rax as u32)) as u32); crate::simd_ops::wu8(rsp.wrapping_add(0x80u64), ((rsi as u8)) as u8); crate::simd_ops::wu64(rsp.wrapping_add(0x24u64), rsi); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 25; continue; } bb = 22; continue; }
            22 => { fl_l = (crate::simd_ops::ru8(rsp.wrapping_add(0x80u64))) as u8 as u64; fl_r = ((rsi as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 24; continue; } bb = 23; continue; }
            23 => { crate::simd_ops::wu32(rax.wrapping_add(0xcu64), ((rsi as u32)) as u32); bb = 24; continue; }
            24 => { xmm8 = ((crate::simd_ops::ru32(rax.wrapping_add(0xcu64))) as u32) as u128; xmm6 = (xmm8) as u128; xmm6 = (xmm6 & !0xffff_ffffu128) | ((f32::from_bits(xmm6 as u32) + f32::from_bits(xmm7 as u32)).to_bits() as u128); crate::simd_ops::wu32(rax.wrapping_add(0xcu64), ((xmm6 as u32)) as u32); bb = 25; continue; }
            25 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0x130u64))) as u8 as u64; fl_r = (0x10u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 28; continue; } bb = 26; continue; }
            26 => { xmm0 = ((crate::simd_ops::ru32(rbx.wrapping_add(rdi.wrapping_mul(4)).wrapping_add(0x88u64))) as u32) as u128; fl_l = ((xmm8 as u32)) as u32 as u64; fl_r = ((xmm0 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) != f32::from_bits((fl_r as u32)) && !f32::from_bits((fl_l as u32)).is_nan() && !f32::from_bits((fl_r as u32)).is_nan()) { bb = 29; continue; } bb = 27; continue; }
            27 => { fl_l = ((xmm6 as u32)) as u32 as u64; fl_r = ((xmm0 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 30; continue; } bb = 28; continue; }
            28 => { fl_l = (crate::simd_ops::lk_or_u8(rbx.wrapping_add(0x13fu64), (0x4u8) as u8)) as u8 as u64; fl_r = fl_l; bb = 33; continue; }
            29 => { fl_l = ((xmm6 as u32)) as u32 as u64; fl_r = ((xmm0 as u32)) as u32 as u64; bb = 30; continue; }
            30 => { if !(f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 33; continue; } bb = 31; continue; }
            31 => { crate::simd_ops::wu32(rbx.wrapping_add(rdi.wrapping_mul(4)).wrapping_add(0x88u64), ((xmm6 as u32)) as u32); bb = 33; continue; }
            32 => { xmm7 = (xmm7 & !0xffff_ffffu128) | ((f32::from_bits(xmm7 as u32) + f32::from_bits(crate::simd_ops::ru32(rbx.wrapping_add(rdi.wrapping_mul(4)).wrapping_add(0x88u64)))).to_bits() as u128); crate::simd_ops::wu32(rbx.wrapping_add(rdi.wrapping_mul(4)).wrapping_add(0x88u64), ((xmm7 as u32)) as u32); bb = 33; continue; }
            33 => { xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x60u64))) as u128; r11 = rsp.wrapping_add(0x70); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x18u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; xmm8 = (crate::simd_ops::ru128(r11.wrapping_sub(0x30u64))) as u128; xmm7 = (crate::simd_ops::ru128(rsp.wrapping_add(0x50u64))) as u128; rsp = r11; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415317a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141531160; const P1: u64 = 0x14183acc8;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; r10 = (crate::simd_ops::ru32(rcx.wrapping_add(0x8u64))) as u32 as u64; r13 = 0; fl_l = 0; fl_r = 0; rbp = (crate::simd_ops::ru32(rdx)) as u32 as u64; r12 = r8; rsi = (crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32 as u64; r15 = rdx; rdi = (crate::simd_ops::ru32(rdx.wrapping_add(0x8u64))) as u32 as u64; rbx = rcx; r11 = ((r13 as u32)) as u32 as u64; fl_l = ((r10 as u32)) as u32 as u64; fl_r = ((r10 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 6; continue; } bb = 1; continue; }
            1 => { r14 = (crate::simd_ops::ru64(rcx)) as u64; bb = 2; continue; }
            2 => { r8 = ((r10 as u32)) as u32 as u64; r8 = (((r8 as u32)).wrapping_shr(1u32)) as u32 as u64; r8 = (((r8 as u32)).wrapping_add((r11 as u32))) as u32 as u64; rcx = ((((r8 as u32)) as i32 as i64) as u64) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(r14).wrapping_add(0x4u64))) as u32 as u64; rdx = (crate::simd_ops::ru32(rcx.wrapping_add(r14).wrapping_add(0x8u64))) as u32 as u64; fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(r14))) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 5; continue; } bb = 3; continue; }
            3 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 5; continue; } bb = 4; continue; }
            4 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 15; continue; } bb = 5; continue; }
            5 => { rcx = (rcx & !0xffu64) | (((if (((fl_l as u32)) < ((fl_r as u32))) { 1u8 } else { 0u8 }) as u8 as u64)); rax = ((r8.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; rax = (if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { (r11 as u32) } else { (rax as u32) }) as u32 as u64; r11 = ((rax as u32)) as u32 as u64; rax = ((r10.wrapping_sub(0x1u64)) as u32) as u32 as u64; rax = (if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { (r10 as u32) } else { (rax as u32) }) as u32 as u64; r10 = ((rax as u32)) as u32 as u64; fl_l = (((r10 as u32)).wrapping_shr(1u32)) as u32 as u64; r10 = ((fl_l as u32)) as u32 as u64; if (((fl_l as u32)) != 0) { bb = 2; continue; } bb = 6; continue; }
            6 => { crate::simd_ops::wu8(r12, ((r13 as u8)) as u8); rcx = (crate::simd_ops::ru64(rbx)) as u64; rsi = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_add(0xcu64))) as u32 as u64; rdi = ((((r11 as u32)) as i32 as i64) as u64) as u64; rdi = ((rdi).wrapping_shl(4u32)) as u64; fl_l = ((rcx).wrapping_add(rdi)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; if (((fl_l as u64)) == 0) { bb = 16; continue; } bb = 7; continue; }
            7 => { rdi = ((((rdi) as i64).wrapping_shr(4u32) as u64)) as u64; fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 9; continue; } bb = 8; continue; }
            8 => { rdx = ((rax as u32)) as u32 as u64; rcx = 0x1; rdx = (((rdx as u32)).wrapping_shr(1u32)) as u32 as u64; rdx = (((rdx as u32)).wrapping_add((rax as u32))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; rdx = (if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { (rcx as u32) } else { (rdx as u32) }) as u32 as u64; rcx = rbx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 23; continue; } bb = 9; continue; }
            9 => { fl_l = ((rsi as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rbx.wrapping_add(0xcu64))) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 23; continue; } bb = 10; continue; }
            10 => { rsi = (crate::simd_ops::ru64(rbx)) as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; r14 = ((rdi as u32)) as u32 as u64; r14 = ((r14).wrapping_shl(4u32)) as u64; rsi = ((rsi).wrapping_add(r14)) as u64; fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 12; continue; } bb = 11; continue; }
            11 => { rax = (((rax as u32)).wrapping_sub((rdi as u32))) as u32 as u64; rcx = rsi.wrapping_add(0x10); rax = (((rax as u32)).wrapping_shl(4u32)) as u32 as u64; rdx = rsi; r8 = ((rax as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 12; continue; }
            12 => { fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 13; continue; }
            13 => { crate::simd_ops::wu64(rsi, r13); crate::simd_ops::wu32(rsi.wrapping_add(0x8u64), ((r13 as u32)) as u32); bb = 14; continue; }
            14 => { crate::simd_ops::lk_add_u32(rbx.wrapping_add(0x8u64), (1) as u32); rdx = (crate::simd_ops::ru64(rbx)) as u64; fl_l = ((rdx).wrapping_add(r14)) as u64; fl_r = fl_l; rdx = ((fl_l as u64)) as u64; bb = 21; continue; }
            15 => { crate::simd_ops::wu8(r12, (0x1u8) as u8); rax = (crate::simd_ops::ru64(rbx)) as u64; fl_l = ((rax).wrapping_add(rcx)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; bb = 24; continue; }
            16 => { fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 18; continue; } bb = 17; continue; }
            17 => { rdx = ((rax as u32)) as u32 as u64; rcx = 0x1; rdx = (((rdx as u32)).wrapping_shr(1u32)) as u32 as u64; rdx = (((rdx as u32)).wrapping_add((rax as u32))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; rdx = (if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { (rcx as u32) } else { (rdx as u32) }) as u32 as u64; rcx = rbx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 23; continue; } bb = 18; continue; }
            18 => { fl_l = ((rsi as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rbx.wrapping_add(0xcu64))) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 23; continue; } bb = 19; continue; }
            19 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; rdx = ((rax as u32)) as u32 as u64; rdx = ((rdx).wrapping_shl(4u32)) as u64; rax = ((rax.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x8u64), ((rax as u32)) as u32); fl_l = ((rdx).wrapping_add(crate::simd_ops::ru64(rbx))) as u64; fl_r = fl_l; rdx = ((fl_l as u64)) as u64; if (((fl_l as u64)) == 0) { bb = 21; continue; } bb = 20; continue; }
            20 => { crate::simd_ops::wu64(rdx, r13); crate::simd_ops::wu32(rdx.wrapping_add(0x8u64), ((r13 as u32)) as u32); bb = 21; continue; }
            21 => { rax = rdx; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 24; continue; } bb = 22; continue; }
            22 => { rcx = (crate::simd_ops::ru32(r15)) as u32 as u64; crate::simd_ops::wu32(rdx, ((rcx as u32)) as u32); rcx = (crate::simd_ops::ru32(r15.wrapping_add(0x4u64))) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_add(0x4u64), ((rcx as u32)) as u32); rcx = (crate::simd_ops::ru32(r15.wrapping_add(0x8u64))) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_add(0x8u64), ((rcx as u32)) as u32); bb = 24; continue; }
            23 => { rax = r13; bb = 24; continue; }
            24 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141531940(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141595d10; const P1: u64 = 0x142689190; const P2: u64 = 0x143930f44; const P3: u64 = 0x1415317a0; const P4: u64 = 0x14268918c; const P5: u64 = 0x1434268d0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0; let mut xmm7: u128 = 0; let mut xmm8: u128 = 0; let mut xmm9: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 15>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = rsp; rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x70u64)) as u64; crate::simd_ops::wu128(rax.wrapping_sub(0x28u64), (xmm7) as u128); rbx = rcx; xmm7 = (xmm2) as u128; rcx = ((rdx as u32)) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 41; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rax.wrapping_add(0x8u64), rsi); crate::simd_ops::wu64(rax.wrapping_add(0x18u64), rdi); crate::simd_ops::wu128(rax.wrapping_sub(0x18u64), (xmm6) as u128); crate::simd_ops::wu128(rax.wrapping_sub(0x38u64), (xmm8) as u128); crate::simd_ops::wu128(rax.wrapping_sub(0x48u64), (xmm9) as u128); fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 24; continue; } bb = 2; continue; }
            2 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 7; continue; } bb = 3; continue; }
            3 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 5; continue; } bb = 4; continue; }
            4 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = ((((rdx as u32)) as i32 as i64) as u64) as u64; rcx = rbx; rax = ((rax).wrapping_add(rbx)) as u64; rdx = rsp.wrapping_add(0x20); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x28u64))) as u32 as u64; rax = ((((rax as u32)) & (0xffffff72u32))) as u32 as u64; rax = ((((rax as u32)) | (0x72u32))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rax as u32)) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 6; continue; }
            6 => { xmm8 = (crate::simd_ops::ru128(rsp.wrapping_add(0x40u64))) as u128; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x60u64))) as u128; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; xmm9 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; xmm7 = (crate::simd_ops::ru128(rsp.wrapping_add(0x50u64))) as u128; rsp = ((rsp).wrapping_add(0x70u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            7 => { rsi = (crate::simd_ops::ru32(P1)) as u32 as u64; xmm6 = (xmm7) as u128; xmm6 = (xmm6 & !0xffff_ffffu128) | ((f32::from_bits(xmm6 as u32) - f32::from_bits(crate::simd_ops::ru32(rbx.wrapping_add(0x3a4u64)))).to_bits() as u128); fl_l = ((rsi as u32)) as u32 as u64; fl_r = 0x1a; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 23; continue; } bb = 8; continue; }
            8 => { fl_l = (crate::simd_ops::ru32(P2)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 22; continue; } bb = 9; continue; }
            9 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0x13cu64))) as u8 as u64; fl_r = (0x10u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 22; continue; } bb = 10; continue; }
            10 => { xmm9 = 0u128; fl_l = ((xmm6 as u32)) as u32 as u64; fl_r = ((xmm9 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) == f32::from_bits((fl_r as u32)) || f32::from_bits((fl_l as u32)).is_nan() || f32::from_bits((fl_r as u32)).is_nan()) { bb = 23; continue; } bb = 11; continue; }
            11 => { rdi = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu8(rsp.wrapping_add(0x88u64), (0x0u8) as u8); crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rdi as u32)) as u32); rcx = rbx.wrapping_add(0x108); crate::simd_ops::wu16(rsp.wrapping_add(0x20u64), (0x404u16) as u16); r8 = rsp.wrapping_add(0x88); crate::simd_ops::wu8(rsp.wrapping_add(0x22u64), ((rdi as u8)) as u8); rdx = rsp.wrapping_add(0x20); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x20u64))) as u32 as u64; xmm8 = 0u128; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rax as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x24u64), rdi); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 15; continue; } bb = 12; continue; }
            12 => { fl_l = (crate::simd_ops::ru8(rsp.wrapping_add(0x88u64))) as u8 as u64; fl_r = ((rdi as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 14; continue; } bb = 13; continue; }
            13 => { crate::simd_ops::wu32(rax.wrapping_add(0xcu64), ((rdi as u32)) as u32); bb = 14; continue; }
            14 => { xmm9 = ((crate::simd_ops::ru32(rax.wrapping_add(0xcu64))) as u32) as u128; xmm8 = (xmm9) as u128; xmm8 = (xmm8 & !0xffff_ffffu128) | ((f32::from_bits(xmm8 as u32) + f32::from_bits(xmm6 as u32)).to_bits() as u128); crate::simd_ops::wu32(rax.wrapping_add(0xcu64), ((xmm8 as u32)) as u32); bb = 15; continue; }
            15 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0x148u64))) as u8 as u64; fl_r = (0x10u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 18; continue; } bb = 16; continue; }
            16 => { xmm0 = ((crate::simd_ops::ru32(rbx.wrapping_add(rsi.wrapping_mul(4)).wrapping_add(0xa0u64))) as u32) as u128; fl_l = ((xmm9 as u32)) as u32 as u64; fl_r = ((xmm0 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) != f32::from_bits((fl_r as u32)) && !f32::from_bits((fl_l as u32)).is_nan() && !f32::from_bits((fl_r as u32)).is_nan()) { bb = 19; continue; } bb = 17; continue; }
            17 => { fl_l = ((xmm8 as u32)) as u32 as u64; fl_r = ((xmm0 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 20; continue; } bb = 18; continue; }
            18 => { fl_l = (crate::simd_ops::lk_or_u8(rbx.wrapping_add(0x157u64), (0x4u8) as u8)) as u8 as u64; fl_r = fl_l; crate::simd_ops::wu32(rbx.wrapping_add(0x3a4u64), ((xmm7 as u32)) as u32); bb = 6; continue; }
            19 => { fl_l = ((xmm8 as u32)) as u32 as u64; fl_r = ((xmm0 as u32)) as u32 as u64; bb = 20; continue; }
            20 => { if !(f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 23; continue; } bb = 21; continue; }
            21 => { crate::simd_ops::wu32(rbx.wrapping_add(rsi.wrapping_mul(4)).wrapping_add(0xa0u64), ((xmm8 as u32)) as u32); crate::simd_ops::wu32(rbx.wrapping_add(0x3a4u64), ((xmm7 as u32)) as u32); bb = 6; continue; }
            22 => { xmm6 = (xmm6 & !0xffff_ffffu128) | ((f32::from_bits(xmm6 as u32) + f32::from_bits(crate::simd_ops::ru32(rbx.wrapping_add(rsi.wrapping_mul(4)).wrapping_add(0xa0u64)))).to_bits() as u128); crate::simd_ops::wu32(rbx.wrapping_add(rsi.wrapping_mul(4)).wrapping_add(0xa0u64), ((xmm6 as u32)) as u32); bb = 23; continue; }
            23 => { crate::simd_ops::wu32(rbx.wrapping_add(0x3a4u64), ((xmm7 as u32)) as u32); bb = 6; continue; }
            24 => { rsi = (crate::simd_ops::ru32(P4)) as u32 as u64; xmm6 = (xmm7) as u128; xmm6 = (xmm6 & !0xffff_ffffu128) | ((f32::from_bits(xmm6 as u32) - f32::from_bits(crate::simd_ops::ru32(rbx.wrapping_add(0x3a0u64)))).to_bits() as u128); fl_l = ((rsi as u32)) as u32 as u64; fl_r = 0x1a; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 40; continue; } bb = 25; continue; }
            25 => { fl_l = (crate::simd_ops::ru32(P2)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 39; continue; } bb = 26; continue; }
            26 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0x13cu64))) as u8 as u64; fl_r = (0x8u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 39; continue; } bb = 27; continue; }
            27 => { xmm9 = 0u128; fl_l = ((xmm6 as u32)) as u32 as u64; fl_r = ((xmm9 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) == f32::from_bits((fl_r as u32)) || f32::from_bits((fl_l as u32)).is_nan() || f32::from_bits((fl_r as u32)).is_nan()) { bb = 40; continue; } bb = 28; continue; }
            28 => { rdi = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu8(rsp.wrapping_add(0x88u64), (0x0u8) as u8); crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rdi as u32)) as u32); rcx = rbx.wrapping_add(0x108); crate::simd_ops::wu16(rsp.wrapping_add(0x20u64), (0x403u16) as u16); r8 = rsp.wrapping_add(0x88); crate::simd_ops::wu8(rsp.wrapping_add(0x22u64), ((rdi as u8)) as u8); rdx = rsp.wrapping_add(0x20); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x20u64))) as u32 as u64; xmm8 = 0u128; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rax as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x24u64), rdi); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 32; continue; } bb = 29; continue; }
            29 => { fl_l = (crate::simd_ops::ru8(rsp.wrapping_add(0x88u64))) as u8 as u64; fl_r = ((rdi as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 31; continue; } bb = 30; continue; }
            30 => { crate::simd_ops::wu32(rax.wrapping_add(0xcu64), ((rdi as u32)) as u32); bb = 31; continue; }
            31 => { xmm9 = ((crate::simd_ops::ru32(rax.wrapping_add(0xcu64))) as u32) as u128; xmm8 = (xmm9) as u128; xmm8 = (xmm8 & !0xffff_ffffu128) | ((f32::from_bits(xmm8 as u32) + f32::from_bits(xmm6 as u32)).to_bits() as u128); crate::simd_ops::wu32(rax.wrapping_add(0xcu64), ((xmm8 as u32)) as u32); bb = 32; continue; }
            32 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0x148u64))) as u8 as u64; fl_r = (0x8u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 35; continue; } bb = 33; continue; }
            33 => { xmm0 = ((crate::simd_ops::ru32(rbx.wrapping_add(rsi.wrapping_mul(4)).wrapping_add(0xa0u64))) as u32) as u128; fl_l = ((xmm9 as u32)) as u32 as u64; fl_r = ((xmm0 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) != f32::from_bits((fl_r as u32)) && !f32::from_bits((fl_l as u32)).is_nan() && !f32::from_bits((fl_r as u32)).is_nan()) { bb = 36; continue; } bb = 34; continue; }
            34 => { fl_l = ((xmm8 as u32)) as u32 as u64; fl_r = ((xmm0 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 37; continue; } bb = 35; continue; }
            35 => { fl_l = (crate::simd_ops::lk_or_u8(rbx.wrapping_add(0x157u64), (0x4u8) as u8)) as u8 as u64; fl_r = fl_l; crate::simd_ops::wu32(rbx.wrapping_add(0x3a0u64), ((xmm7 as u32)) as u32); bb = 6; continue; }
            36 => { fl_l = ((xmm8 as u32)) as u32 as u64; fl_r = ((xmm0 as u32)) as u32 as u64; bb = 37; continue; }
            37 => { if !(f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 40; continue; } bb = 38; continue; }
            38 => { crate::simd_ops::wu32(rbx.wrapping_add(rsi.wrapping_mul(4)).wrapping_add(0xa0u64), ((xmm8 as u32)) as u32); crate::simd_ops::wu32(rbx.wrapping_add(0x3a0u64), ((xmm7 as u32)) as u32); bb = 6; continue; }
            39 => { xmm6 = (xmm6 & !0xffff_ffffu128) | ((f32::from_bits(xmm6 as u32) + f32::from_bits(crate::simd_ops::ru32(rbx.wrapping_add(rsi.wrapping_mul(4)).wrapping_add(0xa0u64)))).to_bits() as u128); crate::simd_ops::wu32(rbx.wrapping_add(rsi.wrapping_mul(4)).wrapping_add(0xa0u64), ((xmm6 as u32)) as u32); bb = 40; continue; }
            40 => { crate::simd_ops::wu32(rbx.wrapping_add(0x3a0u64), ((xmm7 as u32)) as u32); bb = 6; continue; }
            41 => { rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x28u64))) as u32 as u64; rdx = rsp.wrapping_add(0x20); xmm7 = (xmm7 & !0xffff_ffffu128) | ((f32::from_bits(xmm7 as u32) + f32::from_bits(crate::simd_ops::ru32(P5))).to_bits() as u128); rax = ((((rax as u32)) & (0xffffff72u32))) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbx); rax = ((((rax as u32)) | (0x72u32))) as u32 as u64; rcx = rbx; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rax as u32)) as u32); xmm2 = (xmm7) as u128; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm7 = (crate::simd_ops::ru128(rsp.wrapping_add(0x50u64))) as u128; rsp = ((rsp).wrapping_add(0x70u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141531c50(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141591570; const P1: u64 = 0xfffffffffffffc80; const P2: u64 = 0x141595d10;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut xmm6: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x40u64)) as u64;
    rdi = rcx;
    crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm6) as u128);
    rcx = ((rcx).wrapping_add(0xfffffffffffffc98u64)) as u64;
    xmm6 = (xmm2) as u128;
    rbx = rdx;
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x28u64))) as u32 as u64;
    rcx = ((rdi.wrapping_add(P1)) as u64) as u64;
    rax = ((((rax as u32)) & (0xffffff72u32))) as u32 as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbx);
    rax = ((((rax as u32)) | (0x72u32))) as u32 as u64;
    rdx = rsp.wrapping_add(0x20);
    xmm2 = (xmm6) as u128;
    crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rax as u32)) as u32);
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64;
    xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128;
    rsp = ((rsp).wrapping_add(0x40u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141531cb0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x378; const P1: u64 = 0xffffffffffffffe8; const P2: u64 = 0x368; const P3: u64 = 0x1415384f0; const P4: u64 = 0x141537750; const P5: u64 = 0x141595ec0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    rsi = (((rdx as u8)) as u32) as u32 as u64;
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    fl_l = rcx; fl_r = rcx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { r8 = 0; fl_l = 0; fl_r = 0; rax = ((rbx.wrapping_add(P1)) as u64) as u64; fl_l = rax; fl_r = rax; rdx = ((rbx.wrapping_add(P2)) as u64) as u64; rdx = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { r8 } else { rdx }) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); }
    rcx = ((rbx.wrapping_add(P2)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp);
    rdx = (((rsi as u8)) as u32) as u32 as u64;
    rcx = rbx;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(P5, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141531d20(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14152f4c0; const P1: u64 = 0x141596600;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = ((r8 as u32)) as u32 as u64; fl_r = 0xd;
    if !((((fl_l as u32)) != ((fl_r as u32)))) { rcx = 0; fl_l = 0; fl_r = 0; return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
    return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141531d40(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x378; const P1: u64 = 0x1415971d0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = ((r8 as u8)) as u8 as u64; fl_r = ((r8 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; fl_r = 0xffffffff; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 3; continue; } bb = 2; continue; }
            2 => { r8 = 0; fl_l = 0; fl_r = 0; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            3 => { return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141531d70(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415302f0; const P1: u64 = 0x142677170; const P2: u64 = 0x142677260; const P3: u64 = 0x18; const P4: u64 = 0x142676fd0; const P5: u64 = 0x28; const P6: u64 = 0x68; const P7: u64 = 0x70;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rax = P1;
    crate::simd_ops::wu64(rbx, rax);
    rax = P2;
    crate::simd_ops::wu64(rbx.wrapping_add(P3), rax);
    rax = P4;
    crate::simd_ops::wu64(rbx.wrapping_add(P5), rax);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rbx.wrapping_add(P6), rax);
    crate::simd_ops::wu64(rbx.wrapping_add(P7), rax);
    rax = rbx;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141531dc0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142677170; const P1: u64 = 0x142677260; const P2: u64 = 0x18; const P3: u64 = 0x142676fd0; const P4: u64 = 0x28; const P5: u64 = 0x141530360;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = P0;
    crate::simd_ops::wu64(rcx, rax);
    rax = P1;
    crate::simd_ops::wu64(rcx.wrapping_add(P2), rax);
    rax = P3;
    crate::simd_ops::wu64(rcx.wrapping_add(P4), rax);
    return crate::simd_ops::call_fwd(P5, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141531e00(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142677170; const P1: u64 = 0x142677260; const P2: u64 = 0x18; const P3: u64 = 0x142676fd0; const P4: u64 = 0x28; const P5: u64 = 0x141530360; const P6: u64 = 0x141834d30;
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
    rax = P3;
    crate::simd_ops::wu64(rcx.wrapping_add(P4), rax);
    rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp);
    fl_l = ((rbx as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64;
    if !(((((fl_l as u8)) & ((fl_r as u8))) == 0)) { rdx = 0x80; rcx = rdi; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rax = rdi;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141531e60(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x70; const P1: u64 = 0x10; const P2: u64 = 0x141531e60;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64;
    fl_l = rcx; fl_r = rcx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rax = ((rax).wrapping_add(rbx)) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    rax = rbx;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141531e90(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141539540; const P1: u64 = 0x141539980; const P2: u64 = 0x1401de410;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 43>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x150u64)) as u64; rdi = rdx; r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); rdx = rcx; rcx = rsp.wrapping_add(0x20); rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru64(rsp.wrapping_add(0x128u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rdx = rdi; rcx = rsp.wrapping_add(0x20); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rbx = rax; bb = 4; continue; }
            2 => { rbx = 0; fl_l = 0; fl_r = 0; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu64(rdi, rbx); bb = 4; continue; }
            4 => { rcx = rsp.wrapping_add(0x20); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x160u64))) as u64; rsp = ((rsp).wrapping_add(0x150u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141531f00(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141539540; const P1: u64 = 0x10; const P2: u64 = 0x70; const P3: u64 = 0x141531e60; const P4: u64 = 0x68; const P5: u64 = 0x8; const P6: u64 = 0x141539650; const P7: u64 = 0x141538ad0; const P8: u64 = 0x54; const P9: u64 = 0x141532ab0; const P10: u64 = 0x1401de410;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 43>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x150u64)) as u64; r14 = rdx; rbx = rcx; rdx = rcx; r8 = 0; fl_l = 0; fl_r = 0; rcx = rsp.wrapping_add(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x128u64))) as u64; fl_l = rbp; fl_r = rbp; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x160u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x168u64), rdi); rdi = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rdi).wrapping_add(rax)) as u64; fl_r = fl_l; rdi = ((fl_l as u64)) as u64; bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(P4))) as u64; rsi = (crate::simd_ops::ru64(rax.wrapping_add(P5))) as u64; rsi = ((rsi).wrapping_sub(rdi)) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x168u64))) as u64; fl_l = (crate::simd_ops::ru64(rbp.wrapping_add(0x20u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rdx = 0; fl_l = 0; fl_r = 0; rcx = rsp.wrapping_add(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rsi = ((rsi).wrapping_sub(rax)) as u64; rcx = rsp.wrapping_add(0x128); rdx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rax; bb = 6; continue; }
            5 => { rcx = 0; fl_l = 0; fl_r = 0; bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(P4))) as u64; rdx = ((rcx as u32)) as u32 as u64; r8 = r14; rcx = rbp; xmm3 = ((crate::simd_ops::ru32(rax.wrapping_add(P8))) as u32) as u128; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x160u64))) as u64; rbx = ((rax as u32)) as u32 as u64; bb = 8; continue; }
            7 => { rbx = 0x2; bb = 8; continue; }
            8 => { rcx = rsp.wrapping_add(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P10, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r11 = rsp.wrapping_add(0x150); rax = ((rbx as u32)) as u32 as u64; rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rbp = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; rsp = r11; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141531ff0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xb0;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    r8 = rsp.wrapping_add(0x30);
    rdx = rsp.wrapping_add(0x38);
    rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp);
    rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x30u64))) as u32 as u64;
    rax = (((rax as u32)).wrapping_sub(crate::simd_ops::ru32(rsp.wrapping_add(0x38u64)))) as u32 as u64;
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141532020(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141530530; const P1: u64 = 0x10; const P2: u64 = 0x68; const P3: u64 = 0x14152ef40;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp);
    crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdi = (((r9 as u8)) as u32) as u32 as u64;
    rsi = r8;
    rbp = rdx;
    rbx = rcx;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rax = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64;
    fl_l = rax; fl_r = rax;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rax = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(P2), rax); rax = 0x1; } else { r9 = (((rdi as u8)) as u32) as u32 as u64; r8 = rbp; rdx = rsi; rcx = rbx; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141532090(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141530760; const P1: u64 = 0x10; const P2: u64 = 0x68; const P3: u64 = 0x141533280;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64; fl_r = (0x0u64) as u64;
    if (((fl_l as u64)) != ((fl_r as u64))) { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64;
    fl_l = rcx; fl_r = rcx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P3, rax, rcx, rdx, r8, r9, rsp); }
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415320c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x68; const P2: u64 = 0x141533430; const P3: u64 = 0x1415307a0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; fl_r = (0x0u64) as u64;
    rbx = rcx;
    if !((((fl_l as u64)) != ((fl_r as u64)))) { rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); }
    rcx = rbx;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(P3, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141532110(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142677270; const P1: u64 = 0x1415aca60; const P2: u64 = 0x1415864b0; const P3: u64 = 0x1415af330; const P4: u64 = 0x143930f60; const P5: u64 = 0x1415afe30;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = P0; crate::simd_ops::wu64(rcx.wrapping_add(0x10u64), rdx); crate::simd_ops::wu64(rcx, rax); rdi = rcx; rax = 0; fl_l = 0; fl_r = 0; rsi = r9; crate::simd_ops::wu64(rcx.wrapping_add(0x28u64), rax); rbx = r8; crate::simd_ops::wu64(rcx.wrapping_add(0x20u64), rax); crate::simd_ops::wu64(rcx.wrapping_add(0x18u64), rax); crate::simd_ops::wu64(rcx.wrapping_add(0x30u64), rax); crate::simd_ops::wu64(rcx.wrapping_add(0x48u64), rax); crate::simd_ops::wu32(rcx.wrapping_add(0x54u64), (0x3f800000u32) as u32); crate::simd_ops::wu64(rcx.wrapping_add(0x8u64), rax); crate::simd_ops::wu32(rcx.wrapping_add(0x50u64), ((rax as u32)) as u32); crate::simd_ops::wu16(rcx.wrapping_add(0x58u64), ((rax as u16)) as u16); rcx = ((rcx).wrapping_add(0x60u64)) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x14u64))) as u32 as u64; rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x18u64))) as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x2cu64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rdi.wrapping_add(0x20u64), rax); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x10u64))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x28u64), ((rax as u32)) as u32); rbx = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            2 => { fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::lk_add_u32(rbx, (1) as u32)) as u32 as u64; bb = 4; continue; }
            4 => { rax = ((crate::simd_ops::ru8(rsp.wrapping_add(0x50u64))) as u32) as u32 as u64; crate::simd_ops::wu64(rdi.wrapping_add(0x18u64), rbx); crate::simd_ops::wu8(rdi.wrapping_add(0x38u64), ((rax as u8)) as u8); crate::simd_ops::wu64(rdi.wrapping_add(0x30u64), rsi); rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x60u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 9; continue; } bb = 5; continue; }
            5 => { fl_l = (crate::simd_ops::ru8(rsi.wrapping_add(0x64u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 9; continue; } bb = 6; continue; }
            6 => { rax = (((rax as u32)).wrapping_add(1)) as u32 as u64; crate::simd_ops::wu32(rsi.wrapping_add(0x60u64), ((rax as u32)) as u32); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 8; continue; } bb = 7; continue; }
            7 => { fl_l = (crate::simd_ops::ru8(rsi.wrapping_add(0x64u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 10; continue; } bb = 8; continue; }
            8 => { rcx = rsi; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 10; continue; }
            9 => { fl_l = (((rax as u32)).wrapping_add(1)) as u32 as u64; rax = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(rsi.wrapping_add(0x60u64), ((rax as u32)) as u32); bb = 10; continue; }
            10 => { r8 = (crate::simd_ops::ru64(rdi.wrapping_add(0x30u64))) as u64; rdx = rdi; rcx = (crate::simd_ops::ru64(P4)) as u64; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = rdi; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141532200(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x18; const P2: u64 = 0x20; const P3: u64 = 0x50; const P4: u64 = 0x2c; const P5: u64 = 0x48; const P6: u64 = 0x3c; const P7: u64 = 0x44; const P8: u64 = 0x88; const P9: u64 = 0x80; const P10: u64 = 0x28;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    r9 = rcx;
    rcx = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(r9, rcx);
    crate::simd_ops::wu64(r9.wrapping_add(P0), rdx);
    crate::simd_ops::wu64(r9.wrapping_add(P1), rcx);
    crate::simd_ops::wu64(r9.wrapping_add(P2), r8);
    fl_l = r8; fl_r = r8;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { fl_l = (crate::simd_ops::lk_add_u32(r8.wrapping_add(P3), (1) as u32)) as u32 as u64; }
    rax = 0xfff8;
    crate::simd_ops::wu64(r9.wrapping_add(P4), rcx);
    crate::simd_ops::lk_and_u16(r9.wrapping_add(P5), ((rax as u16)) as u16);
    crate::simd_ops::wu64(r9.wrapping_add(P6), rcx);
    crate::simd_ops::wu32(r9.wrapping_add(P7), (0x4u32) as u32);
    rax = (crate::simd_ops::ru64(r9.wrapping_add(P2))) as u64;
    fl_l = rax; fl_r = rax;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { crate::simd_ops::wu64(rax.wrapping_add(P8), r9); rax = (crate::simd_ops::ru64(r9.wrapping_add(P2))) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(P9))) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(P2))) as u32 as u64; crate::simd_ops::wu32(r9.wrapping_add(P10), ((rax as u32)) as u32); rax = r9; return rax; }
    crate::simd_ops::wu32(r9.wrapping_add(P10), ((rcx as u32)) as u32);
    rax = r9;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141532270(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142677270; const P1: u64 = 0x141533b20; const P2: u64 = 0x143930f60; const P3: u64 = 0x1415b0bc0; const P4: u64 = 0x30; const P5: u64 = 0x48; const P6: u64 = 0x60; const P7: u64 = 0x64; const P8: u64 = 0x1415af340; const P9: u64 = 0x1415242e0; const P10: u64 = 0x1415aca90; const P11: u64 = 0x18; const P12: u64 = 0x1415864b0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = P0; rdi = rcx; crate::simd_ops::wu64(rcx, rax); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(P2)) as u64; rdx = rdi; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rdi.wrapping_add(P4))) as u64; crate::simd_ops::wu64(rdi.wrapping_add(P5), (0x0u64) as u64); fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru32(rbx.wrapping_add(P6))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 4; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(P7))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rdx = (rdx & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 5; continue; }
            4 => { rdx = (rdx & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 5; continue; }
            5 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(P6), ((rcx as u32)) as u32); if (((fl_l as u32)) == ((fl_r as u32))) { bb = 9; continue; } bb = 6; continue; }
            6 => { fl_l = ((rdx as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 10; continue; } bb = 7; continue; }
            7 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 10; continue; } bb = 8; continue; }
            8 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(P7))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 10; continue; } bb = 9; continue; }
            9 => { rcx = rbx; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); bb = 10; continue; }
            10 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(P6))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 12; continue; } bb = 11; continue; }
            11 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rdx = rbx; rcx = 0x4; rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); bb = 12; continue; }
            12 => { crate::simd_ops::wu64(rdi.wrapping_add(P4), (0x0u64) as u64); bb = 13; continue; }
            13 => { rcx = ((rdi.wrapping_add(P6)) as u64) as u64; rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P11))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 15; continue; } bb = 14; continue; }
            14 => { rax = crate::simd_ops::call_stk(P12, rax, rcx, rdx, r8, r9, rsp); bb = 15; continue; }
            15 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141532330(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x2c; const P1: u64 = 0x40; const P2: u64 = 0x1438152ac; const P3: u64 = 0x143426d28; const P4: u64 = 0x1434269f8; const P5: u64 = 0x142677278; const P6: u64 = 0x48; const P7: u64 = 0x44; const P8: u64 = 0x3c;
    let (mut rax, mut r10, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    r10 = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(P1), ((rdx as u32)) as u32);
    fl_l = ((r9 as u32)) as u32 as u64; fl_r = ((r10 as u32)) as u32 as u64;
    if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { fl_l = (crate::simd_ops::lk_or_u16(rcx.wrapping_add(P6), (0x2u16) as u16)) as u16 as u64; fl_r = fl_l; crate::simd_ops::wu32(rcx.wrapping_add(P7), ((r8 as u32)) as u32); crate::simd_ops::wu32(rcx.wrapping_add(P8), ((r9 as u32)) as u32); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
    rax = (crate::simd_ops::ru32(P2)) as u32 as u64;
    r9 = (((r9 as u32)).wrapping_sub((r10 as u32))) as u32 as u64;
    xmm0 = 0u128;
    xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((((rax) as i64) as f64).to_bits() as u128);
    xmm1 = (((r9 as u32)) as u32) as u128;
    xmm1 = crate::simd_ops::vcvtdq2pd(xmm1);
    xmm1 = (xmm1 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm1 as u64) * f64::from_bits(crate::simd_ops::ru64(P3))).to_bits() as u128);
    xmm1 = (xmm1 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm1 as u64) / f64::from_bits(xmm0 as u64)).to_bits() as u128);
    xmm0 = 0u128;
    fl_l = ((xmm1 as u64)) as u64; fl_r = ((xmm0 as u64)) as u64;
    if !(!(f64::from_bits(fl_l) > f64::from_bits(fl_r))) { xmm0 = ((crate::simd_ops::ru64(P4)) as u64) as u128; } else { xmm0 = ((crate::simd_ops::ru64(P5)) as u64) as u128; }
    xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm0 as u64) + f64::from_bits(xmm1 as u64)).to_bits() as u128);
    r9 = ((r10 as u32)) as u32 as u64;
    rax = (crate::simd_ops::cvt_f64_i32((f64::from_bits(xmm0 as u64)))) as u32 as u64;
    rax = (((rax as u32)).wrapping_add((rdx as u32))) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(P1), ((rax as u32)) as u32);
    fl_l = (crate::simd_ops::lk_or_u16(rcx.wrapping_add(P6), (0x2u16) as u16)) as u16 as u64; fl_r = fl_l;
    crate::simd_ops::wu32(rcx.wrapping_add(P7), ((r8 as u32)) as u32);
    crate::simd_ops::wu32(rcx.wrapping_add(P8), ((r9 as u32)) as u32);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415323a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x20; const P1: u64 = 0x48; const P2: u64 = 0x18;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; fl_r = (0x0u64) as u64;
    if (((fl_l as u64)) != ((fl_r as u64))) { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; return rax; }
    fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(P1))) as u8 as u64; fl_r = (0x2u8) as u8 as u64;
    if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; return rax; }
    fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64; fl_r = (0x0u64) as u64;
    if !((((fl_l as u64)) != ((fl_r as u64)))) { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); return rax; }
    rax = (rax & !0xffu64) | (((0u8) as u8 as u64));
    fl_l = 0; fl_r = 0;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415323c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x20; const P1: u64 = 0x62; const P2: u64 = 0x48; const P3: u64 = 0x8; const P4: u64 = 0x141533f30; const P5: u64 = 0x18; const P6: u64 = 0x10; const P7: u64 = 0x14152f560; const P8: u64 = 0x1415242e0; const P9: u64 = 0x88; const P10: u64 = 0x1415307a0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rsi = rcx; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru8(rax.wrapping_add(P1))) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 4; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(P2))) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rdx = (((rdx as u32)).wrapping_sub(crate::simd_ops::ru32(rcx.wrapping_add(P3)))) as u32 as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P4, rax, rcx, rdx, r8, r9, rsp); }
            4 => { rax = 0xfffc; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rdi); crate::simd_ops::lk_and_u16(rcx.wrapping_add(P2), ((rax as u16)) as u16); rdi = (crate::simd_ops::ru64(rcx.wrapping_add(P5))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 15; continue; } bb = 5; continue; }
            5 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx); rbx = rdi; bb = 6; continue; }
            6 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(P3))) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 8; continue; } bb = 7; continue; }
            7 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P6))) as u64; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); bb = 8; continue; }
            8 => { rdi = (crate::simd_ops::ru64(rdi.wrapping_add(P5))) as u64; rax = (crate::simd_ops::ru64(rbx.wrapping_add(P5))) as u64; fl_l = rbx; fl_r = (crate::simd_ops::ru64(rsi.wrapping_add(P5))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 10; continue; } bb = 9; continue; }
            9 => { crate::simd_ops::wu64(rsi.wrapping_add(P5), rax); bb = 11; continue; }
            10 => { crate::simd_ops::wu64(0x18u64, rax); bb = 11; continue; }
            11 => { fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 12; continue; }
            12 => { rdx = rbx; rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); bb = 13; continue; }
            13 => { rbx = rdi; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 6; continue; } bb = 14; continue; }
            14 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; bb = 15; continue; }
            15 => { rax = (crate::simd_ops::ru64(rsi.wrapping_add(P0))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 18; continue; } bb = 16; continue; }
            16 => { rdx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rax.wrapping_add(P9), rdx); rcx = (crate::simd_ops::ru64(rsi.wrapping_add(P0))) as u64; crate::simd_ops::wu64(rsi.wrapping_add(P0), rdx); fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 18; continue; } bb = 17; continue; }
            17 => { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P10, rax, rcx, rdx, r8, r9, rsp); }
            18 => { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141532490(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143930f88; const P1: u64 = 0x1426452b0; const P2: u64 = 0x1426452a8; const P3: u64 = 0x142645560; const P4: u64 = 0x1415309b0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 19>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = rsp; rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x70u64)) as u64; rbx = (crate::simd_ops::ru64(P0)) as u64; r12 = r8; crate::simd_ops::wu64(rax.wrapping_add(0x10u64), r14); r15 = rcx; r14 = ((rdx as u32)) as u32 as u64; crate::simd_ops::wu64(rax.wrapping_add(0x8u64), rsi); bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 11; continue; } bb = 2; continue; }
            2 => { r9 = ((rax.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; let cx_src: u32 = (r9 as u32); let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u32(rbx, fl_l as u32, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u32)) as u32 as u64; } if (((fl_l as u32)) != ((fl_r as u32))) { bb = 1; continue; } bb = 3; continue; }
            3 => { rsi = rbx.wrapping_add(0x8); bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x48u64))) as u32 as u64; rbp = 0; fl_l = 0; fl_r = 0; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 13; continue; } bb = 5; continue; }
            5 => { rdx = 0; fl_l = 0; fl_r = 0; rax = ((r14 as u32)) as u32 as u64; let d = ((rcx as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x40u64))) as u64; rdi = (crate::simd_ops::ru64(rax.wrapping_add(rdx.wrapping_mul(8)))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 6; continue; }
            6 => { fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x10u64))) as u32 as u64; fl_r = ((r14 as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 9; continue; } bb = 7; continue; }
            7 => { rdi = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 6; continue; } bb = 8; continue; }
            8 => { rdi = rbp; bb = 9; continue; }
            9 => { fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 10; continue; }
            10 => { fl_l = (crate::simd_ops::lk_add_u32(rdi.wrapping_add(0x14u64), (1) as u32)) as u32 as u64; bb = 14; continue; }
            11 => { rsi = rbx.wrapping_add(0x8); rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 4; continue; } bb = 12; continue; }
            12 => { rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); bb = 1; continue; }
            13 => { rdi = rbp; bb = 14; continue; }
            14 => { rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0xa8u64))) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 21; continue; } bb = 15; continue; }
            15 => { rcx = ((rax.wrapping_sub(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; let cx_src: u32 = (rcx as u32); let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u32(rbx, fl_l as u32, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u32)) as u32 as u64; } if (((fl_l as u32)) == ((fl_r as u32))) { bb = 18; continue; } bb = 16; continue; }
            16 => { rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) >= 0) { bb = 15; continue; } bb = 17; continue; }
            17 => { bb = 22; continue; }
            18 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 23; continue; } bb = 19; continue; }
            19 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x30u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 23; continue; } bb = 20; continue; }
            20 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x38u64))) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); bb = 23; continue; }
            21 => { rsi = rbx.wrapping_add(0x8); bb = 22; continue; }
            22 => { rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); bb = 23; continue; }
            23 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0xa0u64))) as u64; rbx = rdi.wrapping_sub(0x10); fl_l = rdi; fl_r = rdi; rbx = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rbp } else { rbx }) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 30; continue; } bb = 24; continue; }
            24 => { rax = (crate::simd_ops::ru64(rbx)) as u64; r9 = r15.wrapping_add(0x18); r8 = (crate::simd_ops::ru64(r15.wrapping_add(0x30u64))) as u64; rcx = rbx; rdx = (crate::simd_ops::ru64(r15.wrapping_add(0x10u64))) as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x20u64), (0x0u8) as u8); rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x278u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rdi = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 27; continue; } bb = 25; continue; }
            25 => { rcx = ((crate::simd_ops::ru16(rsp.wrapping_add(0x64u64))) as u32) as u32 as u64; r9 = rsp.wrapping_add(0x40); crate::simd_ops::wu8(rsp.wrapping_add(0x38u64), (0x0u8) as u8); rdx = rsp.wrapping_add(0x50); rax = 0xff47; crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((rbp as u32)) as u32); rcx = (rcx & !0xffffu64) | ((((((rcx as u16)) & ((rax as u16)))) as u16 as u64)); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), (0x0u64) as u64); rax = (crate::simd_ops::ru64(rdi)) as u64; rcx = (rcx & !0xffffu64) | ((((((rcx as u16)) | (0x40u16))) as u16 as u64)); crate::simd_ops::wu16(rsp.wrapping_add(0x64u64), ((rcx as u16)) as u16); r8 = 0; fl_l = 0; fl_r = 0; rcx = rsp.wrapping_add(0xb8); crate::simd_ops::wu32(rsp.wrapping_add(0x50u64), ((rbp as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rcx); rcx = rdi; crate::simd_ops::wu32(rsp.wrapping_add(0x60u64), ((rbp as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x40u64), ((rbp as u32)) as u32); crate::simd_ops::wu8(rsp.wrapping_add(0x44u64), (0x0u8) as u8); crate::simd_ops::wu32(rsp.wrapping_add(0x48u64), ((rbp as u32)) as u32); crate::simd_ops::wu8(rsp.wrapping_add(0x4cu64), (0x0u8) as u8); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r12); rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x98u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 27; continue; } bb = 26; continue; }
            26 => { rcx = rdi; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rdi = rbp; bb = 27; continue; }
            27 => { rcx = rbx.wrapping_add(0x10); rax = 0xffffffff; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rcx.wrapping_add(0x14u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 29; continue; } bb = 28; continue; }
            28 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 29; continue; }
            29 => { rax = rdi; rsp = ((rsp).wrapping_add(0x70u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            30 => { crate::simd_ops::wu32(r12, ((rbp as u32)) as u32); rax = 0; fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x70u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141532660(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x18; const P1: u64 = 0x20; const P2: u64 = 0x1415307a0; const P3: u64 = 0x1415242e0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    fl_l = rcx; fl_r = rcx;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { return rax; }
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    crate::simd_ops::wu64(rcx.wrapping_add(P0), (0x0u64) as u64);
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64;
    fl_l = rcx; fl_r = rcx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(P1), (0x0u64) as u64); }
    rdx = rbx;
    rcx = 0;
    fl_l = 0; fl_r = 0;
    rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415326a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141532920; const P1: u64 = 0x143426780; const P2: u64 = 0x141530eb0; const P3: u64 = 0x14342668c; const P4: u64 = 0x143426d10; const P5: u64 = 0x143427068; const P6: u64 = 0x1415309c0; const P7: u64 = 0x1415acbb0; const P8: u64 = 0x141530540; const P9: u64 = 0x1415307a0;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0; let mut xmm7: u128 = 0; let mut xmm9: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 19>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r11 = rsp; crate::simd_ops::wu64(r11.wrapping_add(0x18u64), rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x90u64)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; rdi = rcx; rbp = ((rdx as u32)) as u32 as u64; crate::simd_ops::lk_add_u32(rax.wrapping_add(0x50u64), (1) as u32); rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; rcx = ((crate::simd_ops::ru8(rax.wrapping_add(0x62u64))) as u32) as u32 as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = (0x3u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 18; continue; } bb = 1; continue; }
            1 => { fl_l = ((rcx as u8)) as u8 as u64; fl_r = (0x10u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 18; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu64(r11.wrapping_add(0x8u64), rbx); rcx = rdi; crate::simd_ops::wu64(r11.wrapping_add(0x10u64), rsi); crate::simd_ops::wu128(r11.wrapping_sub(0x18u64), (xmm6) as u128); crate::simd_ops::wu128(rsp.wrapping_add(0x70u64), (xmm7) as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(0x58u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; rsi = rax; xmm6 = ((crate::simd_ops::ru32(P1)) as u32) as u128; xmm7 = 0u128; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 14; continue; } bb = 3; continue; }
            3 => { fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(0x59u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x60u64), (xmm9) as u128); xmm9 = ((crate::simd_ops::ru32(rdi.wrapping_add(0x54u64))) as u32) as u128; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x30u64))) as u64; r8 = rsp.wrapping_add(0x30); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); rdx = rdi.wrapping_add(0x54); rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x2cu64))) as u32 as u64; rcx = rsi; crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), ((rax as u32)) as u32); crate::simd_ops::wu32(rdi.wrapping_add(0x54u64), (0x3f800000u32) as u32); rax = (crate::simd_ops::ru64(rsi)) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x40u64), (0x0u32) as u32); crate::simd_ops::wu8(rsp.wrapping_add(0x48u64), (0xffu8) as u8); crate::simd_ops::wu8(rsp.wrapping_add(0x50u64), (0xffu8) as u8); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), (0x0u64) as u64); (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x280u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm1 = ((crate::simd_ops::ru32(rdi.wrapping_add(0x54u64))) as u32) as u128; xmm0 = ((crate::simd_ops::ru32(P3)) as u32) as u128; fl_l = ((xmm1 as u32)) as u32 as u64; fl_r = ((xmm0 as u32)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 8; continue; } bb = 7; continue; }
            7 => { xmm0 = (xmm1) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((if f32::from_bits(xmm0 as u32) < f32::from_bits(crate::simd_ops::ru32(P4)) { f32::from_bits(xmm0 as u32) } else { f32::from_bits(crate::simd_ops::ru32(P4)) }).to_bits() as u128); bb = 8; continue; }
            8 => { xmm3 = 0u128; xmm3 = (xmm3 & !0xffff_ffffu128) | ((((rbp) as i64) as f32).to_bits() as u128); xmm1 = (xmm3) as u128; xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm0 as u32)).to_bits() as u128); fl_l = ((xmm1 as u32)) as u32 as u64; fl_r = ((xmm7 as u32)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 10; continue; } bb = 9; continue; }
            9 => { xmm0 = (xmm6) as u128; bb = 11; continue; }
            10 => { xmm0 = ((crate::simd_ops::ru32(P5)) as u32) as u128; bb = 11; continue; }
            11 => { xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(xmm1 as u32)).to_bits() as u128); xmm2 = 0u128; rcx = (crate::simd_ops::cvt_f32_i32((f32::from_bits(xmm0 as u32)))) as u32 as u64; rcx = ((rcx as u32)) as u32 as u64; xmm2 = (xmm2 & !0xffff_ffffu128) | ((((rcx) as i64) as f32).to_bits() as u128); xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) / f32::from_bits(xmm3 as u32)).to_bits() as u128); xmm0 = (xmm2) as u128; crate::simd_ops::wu32(rdi.wrapping_add(0x54u64), ((xmm2 as u32)) as u32); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) - f32::from_bits(xmm9 as u32)).to_bits() as u128); xmm9 = (crate::simd_ops::ru128(rsp.wrapping_add(0x60u64))) as u128; fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = ((xmm7 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) == f32::from_bits((fl_r as u32)) || f32::from_bits((fl_l as u32)).is_nan() || f32::from_bits((fl_r as u32)).is_nan()) { bb = 13; continue; } bb = 12; continue; }
            12 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; rdx = 0x36; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 13; continue; }
            13 => { crate::simd_ops::wu16(rdi.wrapping_add(0x58u64), (0x1u16) as u16); bb = 14; continue; }
            14 => { xmm0 = 0u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((((rbp) as i64) as f32).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(crate::simd_ops::ru32(rdi.wrapping_add(0x54u64)))).to_bits() as u128); fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = ((xmm7 as u32)) as u32 as u64; xmm7 = (crate::simd_ops::ru128(rsp.wrapping_add(0x70u64))) as u128; if (f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 16; continue; } bb = 15; continue; }
            15 => { xmm6 = ((crate::simd_ops::ru32(P5)) as u32) as u128; bb = 16; continue; }
            16 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; xmm6 = (xmm6 & !0xffff_ffffu128) | ((f32::from_bits(xmm6 as u32) + f32::from_bits(xmm0 as u32)).to_bits() as u128); rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; r9 = rsp.wrapping_add(0x20); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (0x0u64) as u64); crate::simd_ops::wu8(rsp.wrapping_add(0x28u64), (0x0u8) as u8); rbx = (crate::simd_ops::cvt_f32_i32((f32::from_bits(xmm6 as u32)))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x50u64), ((rbx as u32)) as u32); r8 = ((rbx as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x90u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm2 = ((crate::simd_ops::ru32(rdi.wrapping_add(0x54u64))) as u32) as u128; rcx = rdi.wrapping_add(0x60); rdx = ((rbx as u32)) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x50u64))) as u32 as u64; rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; crate::simd_ops::lk_add_u64(rdi.wrapping_add(0x8u64), (rax) as u64); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x80u64))) as u128; fl_l = rax; fl_r = rsi; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0xa8u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0xa0u64))) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 18; continue; } bb = 17; continue; }
            17 => { crate::simd_ops::wu8(rdi.wrapping_add(0x58u64), (0x0u8) as u8); bb = 18; continue; }
            18 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0xb0u64))) as u64; rsp = ((rsp).wrapping_add(0x90u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415328b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x20; const P1: u64 = 0x40; const P2: u64 = 0x80; const P3: u64 = 0x14153cdb0; const P4: u64 = 0x14153dc80; const P5: u64 = 0x3c;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rsi = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rdi = rcx;
    fl_l = rsi; fl_r = rsi;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64; fl_r = 0x0;
    if (((fl_l as u32)) != ((fl_r as u32))) { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    rsi = (crate::simd_ops::ru64(rsi.wrapping_add(P2))) as u64;
    rcx = rsi;
    fl_l = ((rdx as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64;
    if !(((((fl_l as u8)) & ((fl_r as u8))) == 0)) { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = rsi; rbx = ((rax as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rbx = (((rbx as u32)).wrapping_add((rax as u32))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(P5), ((rbx as u32)) as u32); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp);
    crate::simd_ops::wu32(rdi.wrapping_add(P5), ((rax as u32)) as u32);
    rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141532920(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x141539540; const P2: u64 = 0x141539980; const P3: u64 = 0x1401de410;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 43>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x150u64)) as u64;
    rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    r8 = 0;
    fl_l = 0; fl_r = 0;
    rcx = rsp.wrapping_add(0x20);
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    fl_l = (crate::simd_ops::ru64(rsp.wrapping_add(0x128u64))) as u64; fl_r = (0x0u64) as u64;
    if !((((fl_l as u64)) != ((fl_r as u64)))) { rbx = 0; fl_l = 0; fl_r = 0; } else { rdx = rsp.wrapping_add(0x160); rcx = rsp.wrapping_add(0x20); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x160u64))) as u64; fl_l = rax; fl_r = rax; rbx = (if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { rax } else { rbx }) as u64; }
    rcx = rsp.wrapping_add(0x20);
    rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp);
    rax = rbx;
    rsp = ((rsp).wrapping_add(0x150u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141532980(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141539980; const P1: u64 = 0x14153d9c0; const P2: u64 = 0x1415397b0; const P3: u64 = 0x1415242e0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 15>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = rsp; rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x68u64)) as u64; crate::simd_ops::wu64(rax.wrapping_add(0x8u64), rbx); r15 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rax.wrapping_add(0x10u64), rbp); rbx = ((r8 as u32)) as u32 as u64; crate::simd_ops::wu64(rax.wrapping_add(0x18u64), rsi); rbp = rcx; crate::simd_ops::wu64(rax.wrapping_sub(0x18u64), r14); rsi = r9; r14 = rdx; crate::simd_ops::wu64(r9, r15); rcx = r14; crate::simd_ops::wu64(rax.wrapping_sub(0x30u64), r15); rdx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rax.wrapping_sub(0x28u64), r15); rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rcx = rax; r8 = rsp.wrapping_add(0x48); rdx = ((rbx as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x50u64))) as u32 as u64; rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 11; continue; } bb = 1; continue; }
            1 => { rbx = rdx; fl_l = (crate::simd_ops::ru32(rdx)) as u32 as u64; fl_r = ((r15 as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rdi = ((r15.wrapping_add(0x2u64)) as u32) as u32 as u64; bb = 12; continue; }
            3 => { rcx = ((rdx.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rdi = 0x1; fl_l = rdx; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 12; continue; } bb = 4; continue; }
            4 => { bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x10u64))) as u64; r9 = rsp.wrapping_add(0x98); rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x8u64))) as u64; rcx = r14; r8 = (crate::simd_ops::ru32(rbx)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x98u64), ((r15 as u32)) as u32); rdx = ((rdx).wrapping_sub(crate::simd_ops::ru64(rax.wrapping_add(0x70u64)))) as u64; rax = rsp.wrapping_add(0x40); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); crate::simd_ops::wu8(rsp.wrapping_add(0x28u64), ((r15 as u8)) as u8); crate::simd_ops::wu8(rsp.wrapping_add(0x20u64), ((r15 as u8)) as u8); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 9; continue; } bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; fl_l = ((rdi as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 8; continue; } bb = 7; continue; }
            7 => { fl_l = rax; fl_r = (crate::simd_ops::ru64(rsi)) as u64; if ((((fl_l as u64)) as i64) >= (((fl_r as u64)) as i64)) { bb = 9; continue; } bb = 8; continue; }
            8 => { crate::simd_ops::wu64(rsi, rax); rdi = ((r15 as u32)) as u32 as u64; bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x50u64))) as u32 as u64; rbx = ((rbx).wrapping_add(0x4u64)) as u64; rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rax = ((rdx.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; fl_l = rbx; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 5; continue; } bb = 10; continue; }
            10 => { bb = 12; continue; }
            11 => { rdi = 0x3; bb = 12; continue; }
            12 => { r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x88u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 13; continue; }
            13 => { rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rsp.wrapping_add(0x50u64), ((r15 as u32)) as u32); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 14; continue; }
            14 => { rax = ((rdi as u32)) as u32 as u64; rsp = ((rsp).wrapping_add(0x68u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141532ab0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14153dca0; const P1: u64 = 0x143426d28; const P2: u64 = 0x142677278; const P3: u64 = 0x1434269f8; const P4: u64 = 0x1438152ac; const P5: u64 = 0x14153cdb0; const P6: u64 = 0x14153dc80; const P7: u64 = 0x141533cd0; const P8: u64 = 0x14153bb80; const P9: u64 = 0x1434268d0;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0; let mut xmm7: u128 = 0; let mut xmm8: u128 = 0; let mut xmm9: u128 = 0; let mut xmm10: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 15>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r11 = rsp; crate::simd_ops::wu64(r11.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(r11.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x70u64)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x20u64))) as u64; rbx = r8; crate::simd_ops::wu128(r11.wrapping_sub(0x58u64), (xmm10) as u128); xmm10 = (xmm3) as u128; rsi = ((rdx as u32)) as u32 as u64; rdi = rcx; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 16; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(r11.wrapping_add(0x8u64), rbp); rbp = (crate::simd_ops::ru64(rax.wrapping_add(0x80u64))) as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x60u64), (xmm6) as u128); rcx = rbp; crate::simd_ops::wu128(rsp.wrapping_add(0x50u64), (xmm7) as u128); crate::simd_ops::wu128(r11.wrapping_sub(0x38u64), (xmm8) as u128); crate::simd_ops::wu128(r11.wrapping_sub(0x48u64), (xmm9) as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm8 = ((crate::simd_ops::ru64(P1)) as u64) as u128; xmm0 = 0u128; xmm7 = ((crate::simd_ops::ru64(P2)) as u64) as u128; xmm9 = 0u128; xmm6 = ((crate::simd_ops::ru64(P3)) as u64) as u128; xmm1 = (((rax as u32)) as u32) as u128; rax = (crate::simd_ops::ru32(P4)) as u32 as u64; xmm1 = crate::simd_ops::vcvtdq2pd(xmm1); xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((((rax) as i64) as f64).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm1 as u64) * f64::from_bits(xmm8 as u64)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm1 as u64) / f64::from_bits(xmm0 as u64)).to_bits() as u128); fl_l = ((xmm1 as u64)) as u64; fl_r = ((xmm9 as u64)) as u64; if !(f64::from_bits(fl_l) > f64::from_bits(fl_r)) { bb = 3; continue; } bb = 2; continue; }
            2 => { xmm0 = (xmm6) as u128; bb = 4; continue; }
            3 => { xmm0 = (xmm7) as u128; bb = 4; continue; }
            4 => { xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm0 as u64) + f64::from_bits(xmm1 as u64)).to_bits() as u128); rcx = rbp; rax = (crate::simd_ops::cvt_f64_i32((f64::from_bits(xmm0 as u64)))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x4u64), ((rax as u32)) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm0 = 0u128; xmm1 = (((rax as u32)) as u32) as u128; rax = (crate::simd_ops::ru32(P4)) as u32 as u64; xmm1 = crate::simd_ops::vcvtdq2pd(xmm1); xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((((rax) as i64) as f64).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm1 as u64) * f64::from_bits(xmm8 as u64)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm1 as u64) / f64::from_bits(xmm0 as u64)).to_bits() as u128); fl_l = ((xmm1 as u64)) as u64; fl_r = ((xmm9 as u64)) as u64; if !(f64::from_bits(fl_l) > f64::from_bits(fl_r)) { bb = 6; continue; } bb = 5; continue; }
            5 => { xmm0 = (xmm6) as u128; bb = 7; continue; }
            6 => { xmm0 = (xmm7) as u128; bb = 7; continue; }
            7 => { xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm0 as u64) + f64::from_bits(xmm1 as u64)).to_bits() as u128); rcx = rbp; rax = (crate::simd_ops::cvt_f64_i32((f64::from_bits(xmm0 as u64)))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x8u64), ((rax as u32)) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm0 = 0u128; xmm1 = (((rax as u32)) as u32) as u128; rax = (crate::simd_ops::ru32(P4)) as u32 as u64; xmm1 = crate::simd_ops::vcvtdq2pd(xmm1); xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((((rax) as i64) as f64).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm1 as u64) * f64::from_bits(xmm8 as u64)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm1 as u64) / f64::from_bits(xmm0 as u64)).to_bits() as u128); fl_l = ((xmm1 as u64)) as u64; fl_r = ((xmm9 as u64)) as u64; if !(f64::from_bits(fl_l) > f64::from_bits(fl_r)) { bb = 9; continue; } bb = 8; continue; }
            8 => { xmm0 = (xmm6) as u128; bb = 10; continue; }
            9 => { xmm0 = (xmm7) as u128; bb = 10; continue; }
            10 => { xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm0 as u64) + f64::from_bits(xmm1 as u64)).to_bits() as u128); xmm1 = (((rsi as u32)) as u32) as u128; xmm1 = crate::simd_ops::vcvtdq2pd(xmm1); rax = (crate::simd_ops::cvt_f64_i32((f64::from_bits(xmm0 as u64)))) as u32 as u64; xmm1 = (xmm1 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm1 as u64) * f64::from_bits(xmm8 as u64)).to_bits() as u128); xmm0 = 0u128; crate::simd_ops::wu32(rbx.wrapping_add(0xcu64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(P4)) as u32 as u64; xmm8 = (crate::simd_ops::ru128(rsp.wrapping_add(0x40u64))) as u128; xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((((rax) as i64) as f64).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm1 as u64) / f64::from_bits(xmm0 as u64)).to_bits() as u128); fl_l = ((xmm1 as u64)) as u64; fl_r = ((xmm9 as u64)) as u64; xmm9 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; if (f64::from_bits(fl_l) > f64::from_bits(fl_r)) { bb = 12; continue; } bb = 11; continue; }
            11 => { xmm6 = (xmm7) as u128; bb = 12; continue; }
            12 => { xmm7 = (crate::simd_ops::ru128(rsp.wrapping_add(0x50u64))) as u128; xmm6 = (xmm6 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm6 as u64) + f64::from_bits(xmm1 as u64)).to_bits() as u128); rax = (crate::simd_ops::cvt_f64_i32((f64::from_bits(xmm6 as u64)))) as u32 as u64; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x60u64))) as u128; crate::simd_ops::wu32(rbx, ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x20u64))) as u64; rcx = (crate::simd_ops::ru32(rax.wrapping_add(0xa0u64))) as u32 as u64; rcx = (((rcx as u32)).wrapping_sub((rsi as u32))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 14; continue; } bb = 13; continue; }
            13 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 15; continue; }
            14 => { rax = 0; fl_l = 0; fl_r = 0; bb = 15; continue; }
            15 => { rcx = rbp; crate::simd_ops::wu32(rbx.wrapping_add(0x10u64), ((rax as u32)) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru32(P4)) as u32 as u64; xmm0 = 0u128; xmm2 = ((crate::simd_ops::ru32(P9)) as u32) as u128; xmm1 = 0u128; rdx = rax; xmm0 = (xmm0 & !0xffff_ffffu128) | ((((rcx) as i64) as f32).to_bits() as u128); rcx = (crate::simd_ops::ru32(rax.wrapping_add(0x8u64))) as u32 as u64; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm10 as u32)).to_bits() as u128); xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) / f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm0 = 0u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((((rcx) as i64) as f32).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm2 as u32)).to_bits() as u128); crate::simd_ops::wu32(rbx.wrapping_add(0x18u64), ((xmm0 as u32)) as u32); xmm0 = 0u128; rcx = (crate::simd_ops::ru32(rax.wrapping_add(0x4u64))) as u32 as u64; xmm1 = (xmm1 & !0xffff_ffffu128) | ((((rcx) as i64) as f32).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm2 as u32)).to_bits() as u128); crate::simd_ops::wu32(rbx.wrapping_add(0x14u64), ((xmm1 as u32)) as u32); xmm1 = 0u128; rax = (crate::simd_ops::ru32(rax.wrapping_add(0xcu64))) as u32 as u64; xmm0 = (xmm0 & !0xffff_ffffu128) | ((((rax) as i64) as f32).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm2 as u32)).to_bits() as u128); crate::simd_ops::wu32(rbx.wrapping_add(0x1cu64), ((xmm0 as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x10u64))) as u32 as u64; xmm1 = (xmm1 & !0xffff_ffffu128) | ((((rax) as i64) as f32).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm2 as u32)).to_bits() as u128); crate::simd_ops::wu32(rbx.wrapping_add(0x20u64), ((xmm1 as u32)) as u32); rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; bb = 17; continue; }
            16 => { xmm0 = 0u128; crate::simd_ops::wu64(r8.wrapping_add(0x4u64), rax); crate::simd_ops::wu64(r8.wrapping_add(0xcu64), rax); crate::simd_ops::wu32(r8, ((rax as u32)) as u32); crate::simd_ops::wu64(r8.wrapping_add(0x14u64), rax); crate::simd_ops::wu32(r8.wrapping_add(0x1cu64), ((rax as u32)) as u32); crate::simd_ops::wu32(r8.wrapping_add(0x20u64), ((xmm0 as u32)) as u32); bb = 17; continue; }
            17 => { r11 = rsp.wrapping_add(0x70); rax = 0x1; rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x18u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; xmm10 = (crate::simd_ops::ru128(r11.wrapping_sub(0x50u64))) as u128; rsp = r11; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141532d00(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x20;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rax = (crate::simd_ops::ru32(rax.wrapping_add(P1))) as u32 as u64;
    fl_l = ((((rax as u32)) & (0x7f00u32))) as u32 as u64; fl_r = fl_l;
    rax = ((fl_l as u32)) as u32 as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141532d10(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141539540; const P1: u64 = 0x141539980; const P2: u64 = 0x14153bce0; const P3: u64 = 0x141532490; const P4: u64 = 0x1415397b0; const P5: u64 = 0x141533d20; const P6: u64 = 0x1415242e0; const P7: u64 = 0x1401de410; const P8: u64 = 0x1415309b0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 59>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rbp = rsp.wrapping_sub(0xb8); rsp = ((rsp).wrapping_sub(0x1b8u64)) as u64; rdi = ((rdx as u32)) as u32 as u64; rbx = (((r8 as u8)) as u32) as u32 as u64; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; r14 = rcx; rax = ((crate::simd_ops::ru8(rdx.wrapping_add(0x62u64))) as u32) as u32 as u64; rax = (rax & !0xffu64) | ((((((rax as u8)) & (0xfu8))) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if (((fl_l as u8)) > ((fl_r as u8))) { bb = 25; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; r8 = 0; fl_l = 0; fl_r = 0; rax = ((rax).wrapping_sub(crate::simd_ops::ru64(rdx.wrapping_add(0x70u64)))) as u64; rcx = rsp.wrapping_add(0x70); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rax); rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru64(rbp.wrapping_add(0x78u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 24; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rsp.wrapping_add(0x1e8u64), rsi); rsi = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x1a8u64), r13); r13 = ((rsi as u32)) as u32 as u64; fl_l = ((rbx as u8)) as u8 as u64; fl_r = ((rbx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 5; continue; } bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(r14.wrapping_add(0x10u64))) as u64; rdx = rsp.wrapping_add(0x70); r8 = 0; fl_l = 0; fl_r = 0; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0xa0u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru64(rbp.wrapping_add(0x78u64))) as u64; fl_r = rsi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 23; continue; } bb = 4; continue; }
            4 => { r13 = ((rsi.wrapping_add(0x1u64)) as u32) as u32 as u64; bb = 5; continue; }
            5 => { crate::simd_ops::wu64(rsp.wrapping_add(0x1b0u64), r12); crate::simd_ops::wu64(rsp.wrapping_add(0x1a0u64), r15); r15 = 0x2; bb = 6; continue; }
            6 => { rdx = rsp.wrapping_add(0x50); r12 = rsi; rcx = rsp.wrapping_add(0x70); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rdx = rsp.wrapping_add(0x40); fl_l = rax; fl_r = rax; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), (0x0u64) as u64); rcx = (if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { rax } else { rcx }) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); r8 = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x48u64))) as u32 as u64; rbx = r8; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = ((r8.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = r8; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 34; continue; } bb = 7; continue; }
            7 => { fl_l = (crate::simd_ops::ru32(rbx)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 10; continue; } bb = 8; continue; }
            8 => { rbx = ((rbx).wrapping_add(0x18u64)) as u64; fl_l = rbx; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 7; continue; } bb = 9; continue; }
            9 => { bb = 34; continue; }
            10 => { rax = (crate::simd_ops::ru64(r14.wrapping_add(0x10u64))) as u64; rdx = (crate::simd_ops::ru64(r14.wrapping_add(0x8u64))) as u64; rdx = ((rdx).wrapping_sub(crate::simd_ops::ru64(rax.wrapping_add(0x70u64)))) as u64; rax = (crate::simd_ops::ru64(r14.wrapping_add(0x48u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 15; continue; } bb = 11; continue; }
            11 => { bb = 12; continue; }
            12 => { fl_l = (crate::simd_ops::ru32(rax.wrapping_add(0x10u64))) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 14; continue; } bb = 13; continue; }
            13 => { rcx = (crate::simd_ops::ru32(rax.wrapping_add(0x20u64))) as u32 as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rax.wrapping_add(0x18u64)))) as u64; fl_l = rcx; fl_r = rdx; if ((((fl_l as u64)) as i64) >= (((fl_r as u64)) as i64)) { bb = 40; continue; } bb = 14; continue; }
            14 => { rax = (crate::simd_ops::ru64(rax)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 12; continue; } bb = 15; continue; }
            15 => { rdx = (crate::simd_ops::ru32(rbx.wrapping_add(0x4u64))) as u32 as u64; r8 = rbp.wrapping_add(0xe0); rcx = r14; r12 = rbx; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rsi = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 17; continue; } bb = 16; continue; }
            16 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x4u64))) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 19; continue; } bb = 17; continue; }
            17 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0xcu64))) as u32 as u64; r9 = rbp.wrapping_add(0xf8); rdx = (((crate::simd_ops::ru32(rbp.wrapping_add(0xe0u64))) as i32 as i64) as u64) as u64; rcx = rsp.wrapping_add(0x70); rdx = ((rdx).wrapping_add(crate::simd_ops::ru64(rsp.wrapping_add(0x58u64)))) as u64; r8 = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0xf8u64), ((rax as u32)) as u32); rax = rsp.wrapping_add(0x60); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); crate::simd_ops::wu8(rsp.wrapping_add(0x28u64), (0x0u8) as u8); crate::simd_ops::wu8(rsp.wrapping_add(0x20u64), (0x0u8) as u8); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); r15 = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 26; continue; } bb = 18; continue; }
            18 => { fl_l = ((r13 as u32)) as u32 as u64; fl_r = ((r13 as u32)) as u32 as u64; rdx = rsp.wrapping_add(0x70); r9 = rbx; r8 = rsi; rcx = (rcx & !0xffu64) | (((if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { 1u8 } else { 0u8 }) as u8 as u64)); crate::simd_ops::wu8(rsp.wrapping_add(0x30u64), ((rcx as u8)) as u8); rcx = (crate::simd_ops::ru32(rbp.wrapping_add(0xe0u64))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rcx as u32)) as u32); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rcx); rcx = r14; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((r15 as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 33; continue; } bb = 19; continue; }
            19 => { rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 22; continue; } bb = 20; continue; }
            20 => { crate::simd_ops::wu32(rsp.wrapping_add(0x48u64), (0x0u32) as u32); bb = 21; continue; }
            21 => { rcx = 0x2; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); bb = 22; continue; }
            22 => { r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x1b0u64))) as u64; r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x1a0u64))) as u64; bb = 23; continue; }
            23 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x1e8u64))) as u64; r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x1a8u64))) as u64; bb = 24; continue; }
            24 => { rcx = rsp.wrapping_add(0x70); rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); bb = 25; continue; }
            25 => { rsp = ((rsp).wrapping_add(0x1b8u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            26 => { fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 28; continue; } bb = 27; continue; }
            27 => { rcx = rsi; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); bb = 28; continue; }
            28 => { fl_l = (crate::simd_ops::ru8(rbp.wrapping_add(0x98u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 32; continue; } bb = 29; continue; }
            29 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x14u64))) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 32; continue; } bb = 30; continue; }
            30 => { fl_l = ((r13 as u32)) as u32 as u64; fl_r = ((r13 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 32; continue; } bb = 31; continue; }
            31 => { rcx = (crate::simd_ops::ru64(r14.wrapping_add(0x10u64))) as u64; rdx = rsp.wrapping_add(0x70); r8 = 0; fl_l = 0; fl_r = 0; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0xa0u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); r13 = 0x1; bb = 33; continue; }
            32 => { r15 = 0x1; bb = 33; continue; }
            33 => { r8 = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsi = 0; fl_l = 0; fl_r = 0; bb = 34; continue; }
            34 => { fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 36; continue; } bb = 35; continue; }
            35 => { rdx = r8; crate::simd_ops::wu32(rsp.wrapping_add(0x48u64), ((rsi as u32)) as u32); rcx = 0x2; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rsi); crate::simd_ops::wu32(rsp.wrapping_add(0x4cu64), ((rsi as u32)) as u32); bb = 36; continue; }
            36 => { fl_l = ((r15 as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 22; continue; } bb = 37; continue; }
            37 => { fl_l = r12; fl_r = r12; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 22; continue; } bb = 38; continue; }
            38 => { fl_l = (crate::simd_ops::ru64(rbp.wrapping_add(0x78u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 6; continue; } bb = 39; continue; }
            39 => { bb = 22; continue; }
            40 => { fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 22; continue; } bb = 41; continue; }
            41 => { crate::simd_ops::wu32(rsp.wrapping_add(0x48u64), ((rsi as u32)) as u32); rdx = r8; bb = 21; continue; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141533000(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141532ab0; const P1: u64 = 0x14153db40; const P2: u64 = 0x143930f90; const P3: u64 = 0x14159ae70; const P4: u64 = 0x14153d750; const P5: u64 = 0x14153cdb0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 17>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = ((r9 as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 23; continue; } bb = 1; continue; }
            1 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x68u64)) as u64; xmm3 = ((crate::simd_ops::ru32(rsp.wrapping_add(0xb8u64))) as u32) as u128; r12 = ((r8 as u32)) as u32 as u64; r8 = rsp.wrapping_add(0x38); crate::simd_ops::wu64(rsp.wrapping_add(0x98u64), rdi); rbp = ((r9 as u32)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), r15); r14 = ((rdx as u32)) as u32 as u64; rbx = rcx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r15 = rbx.wrapping_add(0x20); rbx = (crate::simd_ops::ru32(rsp.wrapping_add(0xb0u64))) as u32 as u64; fl_l = ((((rbp as u32)) >> (13u32)) & 1) as u64; if ((fl_l as u32) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru64(r15)) as u64; r8 = rsp.wrapping_add(0x38); r9 = ((r14 as u32)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((r12 as u32)) as u32); rdx = ((rbx as u32)) as u32 as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x80u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 3; continue; }
            3 => { fl_l = ((((rbp as u32)) >> (10u32)) & 1) as u64; if ((fl_l as u32) == 0) { bb = 7; continue; } bb = 4; continue; }
            4 => { fl_l = ((r14 as u32)) as u32 as u64; fl_r = ((r14 as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) > 0) { bb = 7; continue; } bb = 5; continue; }
            5 => { rax = ((r14.wrapping_add(r12)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru64(P2)) as u64; r9 = rsp.wrapping_add(0x38); r8 = 0x400; rdx = ((rbx as u32)) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 7; continue; }
            7 => { rcx = (crate::simd_ops::ru64(r15)) as u64; rax = rsp.wrapping_add(0x34); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); r9 = rsp.wrapping_add(0xa8); rax = rsp.wrapping_add(0x30); crate::simd_ops::wu64(rsp.wrapping_add(0x90u64), rsi); r8 = ((r12 as u32)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x80u64))) as u64; rdx = ((r14 as u32)) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = 0; fl_l = 0; fl_r = 0; fl_l = ((((rbp as u32)) >> (9u32)) & 1) as u64; if ((fl_l as u32) == 0) { bb = 11; continue; } bb = 8; continue; }
            8 => { rsi = ((rdi as u32)) as u32 as u64; fl_l = (crate::simd_ops::ru32(rsp.wrapping_add(0xa8u64))) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 11; continue; } bb = 9; continue; }
            9 => { bb = 10; continue; }
            10 => { rcx = (crate::simd_ops::ru64(P2)) as u64; r9 = rsp.wrapping_add(0x38); r8 = 0x200; rdx = ((rbx as u32)) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rsi = (((rsi as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((rsi as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rsp.wrapping_add(0xa8u64))) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 10; continue; } bb = 11; continue; }
            11 => { fl_l = ((((rbp as u32)) >> (8u32)) & 1) as u64; if ((fl_l as u32) == 0) { bb = 14; continue; } bb = 12; continue; }
            12 => { rsi = ((rdi as u32)) as u32 as u64; fl_l = (crate::simd_ops::ru32(rsp.wrapping_add(0x30u64))) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 14; continue; } bb = 13; continue; }
            13 => { rcx = (crate::simd_ops::ru64(P2)) as u64; r9 = rsp.wrapping_add(0x38); r8 = 0x100; rdx = ((rbx as u32)) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rsi = (((rsi as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((rsi as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rsp.wrapping_add(0x30u64))) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 13; continue; } bb = 14; continue; }
            14 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; fl_l = ((((rbp as u32)) >> (12u32)) & 1) as u64; if ((fl_l as u32) == 0) { bb = 18; continue; } bb = 15; continue; }
            15 => { fl_l = (crate::simd_ops::ru32(rsp.wrapping_add(0x34u64))) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 18; continue; } bb = 16; continue; }
            16 => { bb = 17; continue; }
            17 => { rcx = (crate::simd_ops::ru64(P2)) as u64; r9 = rsp.wrapping_add(0x38); r8 = 0x1000; rdx = ((rbx as u32)) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = (((rdi as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((rdi as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rsp.wrapping_add(0x34u64))) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 17; continue; } bb = 18; continue; }
            18 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x98u64))) as u64; fl_l = ((((rbp as u32)) >> (11u32)) & 1) as u64; if ((fl_l as u32) == 0) { bb = 22; continue; } bb = 19; continue; }
            19 => { rcx = (crate::simd_ops::ru64(r15)) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x80u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((r14 as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 22; continue; } bb = 20; continue; }
            20 => { rcx = ((r14.wrapping_add(r12)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { bb = 22; continue; } bb = 21; continue; }
            21 => { rcx = (crate::simd_ops::ru64(P2)) as u64; r9 = rsp.wrapping_add(0x38); r8 = 0x800; rdx = ((rbx as u32)) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 22; continue; }
            22 => { r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsp = ((rsp).wrapping_add(0x68u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); bb = 23; continue; }
            23 => { return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415331e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x18; const P1: u64 = 0x8; const P2: u64 = 0x10; const P3: u64 = 0x14152f560; const P4: u64 = 0x1415242e0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rbp = 0; fl_l = 0; fl_r = 0; r14 = rcx; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx); rbx = rdi; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rsi); bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(P1))) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 8; continue; } bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rdi = (crate::simd_ops::ru64(rdi.wrapping_add(P0))) as u64; rax = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64; fl_l = rbx; fl_r = (crate::simd_ops::ru64(r14.wrapping_add(P0))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 5; continue; } bb = 4; continue; }
            4 => { crate::simd_ops::wu64(r14.wrapping_add(P0), rax); bb = 6; continue; }
            5 => { crate::simd_ops::wu64(rbp.wrapping_add(0x18u64), rax); bb = 6; continue; }
            6 => { fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 7; continue; }
            7 => { rdx = rbx; rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 9; continue; }
            8 => { rbp = rdi; rdi = (crate::simd_ops::ru64(rdi.wrapping_add(P0))) as u64; bb = 9; continue; }
            9 => { rbx = rdi; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 10; continue; }
            10 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; bb = 11; continue; }
            11 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141533290(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x48; const P1: u64 = 0x18; const P2: u64 = 0x8; const P3: u64 = 0x10; const P4: u64 = 0x14152f560; const P5: u64 = 0x1415242e0; const P6: u64 = 0x20; const P7: u64 = 0x88; const P8: u64 = 0x1415307a0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = 0xfffc; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rdi); crate::simd_ops::lk_and_u16(rcx.wrapping_add(P0), ((rax as u16)) as u16); rsi = rcx; rdi = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx); rbx = rdi; bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(P2))) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P3))) as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { rdi = (crate::simd_ops::ru64(rdi.wrapping_add(P1))) as u64; rax = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64; fl_l = rbx; fl_r = (crate::simd_ops::ru64(rsi.wrapping_add(P1))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { crate::simd_ops::wu64(rsi.wrapping_add(P1), rax); bb = 7; continue; }
            6 => { crate::simd_ops::wu64(0x18u64, rax); bb = 7; continue; }
            7 => { fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { rdx = rbx; rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); bb = 9; continue; }
            9 => { rbx = rdi; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 10; continue; }
            10 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; bb = 11; continue; }
            11 => { rax = (crate::simd_ops::ru64(rsi.wrapping_add(P6))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 12; continue; }
            12 => { rdx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rax.wrapping_add(P7), rdx); rcx = (crate::simd_ops::ru64(rsi.wrapping_add(P6))) as u64; crate::simd_ops::wu64(rsi.wrapping_add(P6), rdx); fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 13; continue; }
            13 => { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P8, rax, rcx, rdx, r8, r9, rsp); }
            14 => { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141533340(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x18; const P1: u64 = 0x8;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdi); rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rdi = 0; fl_l = 0; fl_r = 0; r10 = r8; r9 = rdx; rbx = rcx; r11 = ((rdi as u32)) as u32 as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 1; continue; }
            1 => { r8 = (crate::simd_ops::ru64(rax)) as u64; fl_l = r8; fl_r = r10; if ((((fl_l as u64)) as i64) > (((fl_r as u64)) as i64)) { bb = 5; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru32(rax.wrapping_add(P1))) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { fl_l = r8; fl_r = r10; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 5; continue; } bb = 4; continue; }
            4 => { r11 = rax; rax = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64; bb = 11; continue; }
            5 => { rdx = ((rax.wrapping_add(P0)) as u64) as u64; r8 = rax; rcx = (crate::simd_ops::ru64(rdx)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 7; continue; } bb = 6; continue; }
            6 => { crate::simd_ops::wu64(rbx.wrapping_add(P0), rcx); bb = 8; continue; }
            7 => { crate::simd_ops::wu64(r11.wrapping_add(0x18u64), rcx); bb = 8; continue; }
            8 => { rax = rcx; rcx = (crate::simd_ops::ru64(r9.wrapping_add(P1))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { crate::simd_ops::wu64(r9.wrapping_add(P1), r8); crate::simd_ops::wu64(rdx, rdi); bb = 11; continue; }
            10 => { crate::simd_ops::wu64(rdx, rcx); crate::simd_ops::wu64(r9.wrapping_add(P1), r8); bb = 11; continue; }
            11 => { fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 1; continue; } bb = 12; continue; }
            12 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x8u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x10u64))) as u64; return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415333d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x20; const P1: u64 = 0x14153a440; const P2: u64 = 0x48; const P3: u64 = 0x2c; const P4: u64 = 0x30;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    rax = 0;
    fl_l = 0; fl_r = 0;
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rsi = ((r8 as u32)) as u32 as u64;
    rdi = ((rdx as u32)) as u32 as u64;
    fl_l = rcx; fl_r = rcx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rdx = ((r8 as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); r9 = ((rdi as u32)) as u32 as u64; r9 = (((r9 as u32)).wrapping_sub((rsi as u32))) as u32 as u64; fl_l = (((rax as u32)).wrapping_add((r9 as u32))) as u32 as u64; fl_r = fl_l; rax = ((fl_l as u32)) as u32 as u64; }
    rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    rdi = (((rdi as u32)).wrapping_sub((rax as u32))) as u32 as u64;
    crate::simd_ops::lk_or_u16(rbx.wrapping_add(P2), (0x1u16) as u16);
    crate::simd_ops::wu32(rbx.wrapping_add(P3), ((rdi as u32)) as u32);
    crate::simd_ops::wu32(rbx.wrapping_add(P4), (0x0u32) as u32);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141533430(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415337d0; const P1: u64 = 0x10; const P2: u64 = 0x141539540; const P3: u64 = 0x20; const P4: u64 = 0x54; const P5: u64 = 0x2c; const P6: u64 = 0x141539a60; const P7: u64 = 0x141539650; const P8: u64 = 0x141538ad0; const P9: u64 = 0xa0; const P10: u64 = 0x1401de410;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 45>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x160u64)) as u64; rdi = ((r8 as u32)) as u32 as u64; rsi = rdx; rbx = rcx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64; rcx = rsp.wrapping_add(0x30); r8 = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru64(rsp.wrapping_add(0x138u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 6; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x170u64), rbp); rbp = (crate::simd_ops::ru32(rax.wrapping_add(P3))) as u32 as u64; fl_l = ((((rbp as u32)) & (0x7f00u32))) as u32 as u64; fl_r = fl_l; rbp = ((fl_l as u32)) as u32 as u64; bb = 2; continue; }
            2 => { xmm0 = ((crate::simd_ops::ru32(rbx.wrapping_add(P4))) as u32) as u128; rcx = rsp.wrapping_add(0x30); rax = (crate::simd_ops::ru32(rbx.wrapping_add(P5))) as u32 as u64; r9 = ((rbp as u32)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((xmm0 as u32)) as u32); r8 = ((rdi as u32)) as u32 as u64; rdx = rsi; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rax as u32)) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru8(rsp.wrapping_add(0x158u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 5; continue; } bb = 3; continue; }
            3 => { rdx = 0; fl_l = 0; fl_r = 0; rcx = rsp.wrapping_add(0x30); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rsi; rcx = rsp.wrapping_add(0x138); rdx = ((rdx).wrapping_sub(rax)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = (crate::simd_ops::ru64(rsp.wrapping_add(0x150u64))) as u64; rcx = 0; fl_l = 0; fl_r = 0; r8 = ((r8).wrapping_sub(rax)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(rsp.wrapping_add(0x150u64))) as u64; rcx = (if ((((fl_l as u64)) as i64) < (((fl_r as u64)) as i64)) { r8 } else { rcx }) as u64; fl_l = rcx; fl_r = rdi; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64; rdx = rsp.wrapping_add(0x30); r8 = 0; fl_l = 0; fl_r = 0; rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P9))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru64(rsp.wrapping_add(0x138u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 5; continue; }
            5 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x170u64))) as u64; bb = 6; continue; }
            6 => { rcx = rsp.wrapping_add(0x30); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P10, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r11 = rsp.wrapping_add(0x160); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x18u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rsp = r11; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141533550(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141530ad0; const P1: u64 = 0x141533f30; const P2: u64 = 0x141530d60; const P3: u64 = 0x14153a4e0; const P4: u64 = 0x14152f560; const P5: u64 = 0x1415242e0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rbx = rdx; r14 = ((r8 as u32)) as u32 as u64; rbx = ((rbx).wrapping_sub(crate::simd_ops::ru64(rcx.wrapping_add(0x8u64)))) as u64; rsi = rcx; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x20u64))) as u64; r14 = ((r14).wrapping_add(rbx)) as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm6) as u128); xmm6 = (xmm3) as u128; rdi = ((r8 as u32)) as u32 as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 20; continue; } bb = 1; continue; }
            1 => { rdx = ((crate::simd_ops::ru16(rsi.wrapping_add(0x48u64))) as u32) as u32 as u64; r9 = rcx; fl_l = ((rdx as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 7; continue; } bb = 2; continue; }
            2 => { rax = (((crate::simd_ops::ru32(rsi.wrapping_add(0x2cu64))) as i32 as i64) as u64) as u64; fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { bb = 12; continue; } bb = 3; continue; }
            3 => { fl_l = r14; fl_r = rax; if ((((fl_l as u64)) as i64) <= (((fl_r as u64)) as i64)) { bb = 12; continue; } bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::ru8(rsp.wrapping_add(0x60u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rdx = rsi.wrapping_add(0x30); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = 0xfffe; fl_l = (crate::simd_ops::lk_and_u16(rsi.wrapping_add(0x48u64), ((rax as u16)) as u16)) as u16 as u64; fl_r = fl_l; bb = 11; continue; }
            6 => { rdx = (rdx & !0xffffu64) | ((((((rdx as u16)) | (0x4u16))) as u16 as u64)); rax = 0xfffe; crate::simd_ops::wu16(rsi.wrapping_add(0x48u64), ((rdx as u16)) as u16); fl_l = (crate::simd_ops::lk_and_u16(rsi.wrapping_add(0x48u64), ((rax as u16)) as u16)) as u16 as u64; fl_r = fl_l; bb = 11; continue; }
            7 => { fl_l = (crate::simd_ops::ru8(rsp.wrapping_add(0x60u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 12; continue; } bb = 8; continue; }
            8 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0x62u64))) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 12; continue; } bb = 9; continue; }
            9 => { fl_l = ((rdx as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 12; continue; } bb = 10; continue; }
            10 => { rdx = ((rbx as u32)) as u32 as u64; rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 11; continue; }
            11 => { r9 = (crate::simd_ops::ru64(rsi.wrapping_add(0x20u64))) as u64; bb = 12; continue; }
            12 => { fl_l = (crate::simd_ops::ru8(rsi.wrapping_add(0x48u64))) as u8 as u64; fl_r = (0x2u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 16; continue; } bb = 13; continue; }
            13 => { r8 = (((crate::simd_ops::ru32(rsi.wrapping_add(0x3cu64))) as i32 as i64) as u64) as u64; fl_l = r14; fl_r = r8; if ((((fl_l as u64)) as i64) <= (((fl_r as u64)) as i64)) { bb = 16; continue; } bb = 14; continue; }
            14 => { fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { bb = 16; continue; } bb = 15; continue; }
            15 => { r8 = (((r8 as u32)).wrapping_sub((rbx as u32))) as u32 as u64; rdx = rsi.wrapping_add(0x40); rcx = r9; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = 0xfffd; fl_l = (crate::simd_ops::lk_and_u16(rsi.wrapping_add(0x48u64), ((rax as u16)) as u16)) as u16 as u64; fl_r = fl_l; bb = 16; continue; }
            16 => { rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x20u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 20; continue; } bb = 17; continue; }
            17 => { rax = ((crate::simd_ops::ru8(rcx.wrapping_add(0x62u64))) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x3u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 20; continue; } bb = 18; continue; }
            18 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x10u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 20; continue; } bb = 19; continue; }
            19 => { xmm3 = (xmm6) as u128; r8 = ((rdi as u32)) as u32 as u64; rdx = ((rbx as u32)) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 20; continue; }
            20 => { rbx = (crate::simd_ops::ru64(rsi.wrapping_add(0x18u64))) as u64; rbp = 0; fl_l = 0; fl_r = 0; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 30; continue; } bb = 21; continue; }
            21 => { rdi = rbx; bb = 22; continue; }
            22 => { fl_l = r14; fl_r = (crate::simd_ops::ru64(rdi)) as u64; if ((((fl_l as u64)) as i64) <= (((fl_r as u64)) as i64)) { bb = 28; continue; } bb = 23; continue; }
            23 => { fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 25; continue; } bb = 24; continue; }
            24 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 25; continue; }
            25 => { rbx = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x18u64))) as u64; fl_l = rdi; fl_r = (crate::simd_ops::ru64(rsi.wrapping_add(0x18u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 27; continue; } bb = 26; continue; }
            26 => { rdx = rdi; crate::simd_ops::wu64(rsi.wrapping_add(0x18u64), rax); rcx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 29; continue; }
            27 => { rdx = rdi; crate::simd_ops::wu64(rbp.wrapping_add(0x18u64), rax); rcx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 29; continue; }
            28 => { rbp = rbx; rbx = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; bb = 29; continue; }
            29 => { rdi = rbx; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 22; continue; } bb = 30; continue; }
            30 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; rsp = ((rsp).wrapping_add(0x30u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415336d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x141539540; const P2: u64 = 0xa0; const P3: u64 = 0x141524330; const P4: u64 = 0x8; const P5: u64 = 0x141539650; const P6: u64 = 0x141538ad0; const P7: u64 = 0x18; const P8: u64 = 0x1401de410;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 43>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x150u64)) as u64; rbp = rdx; rbx = ((r8 as u32)) as u32 as u64; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rdi = rcx; rcx = rsp.wrapping_add(0x20); r8 = 0; fl_l = 0; fl_r = 0; rsi = r9; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru64(rsp.wrapping_add(0x128u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 8; continue; } bb = 1; continue; }
            1 => { fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 4; continue; } bb = 2; continue; }
            2 => { bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P0))) as u64; rdx = rsp.wrapping_add(0x20); r8 = 0; fl_l = 0; fl_r = 0; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = (((rbx as u32)).wrapping_add(0xffffffffu32)) as u32 as u64; fl_r = fl_l; rbx = ((fl_l as u32)) as u32 as u64; if (((fl_l as u32)) != 0) { bb = 3; continue; } bb = 4; continue; }
            4 => { rdx = 0x20; rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rbx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 5; continue; }
            5 => { rdi = 0x1; rcx = rsp.wrapping_add(0x20); rdx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rax.wrapping_add(P4), ((rdi as u32)) as u32); rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rsi = ((rsi).wrapping_sub(rax)) as u64; rcx = rsp.wrapping_add(0x128); rdx = rsi; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx, rax); crate::simd_ops::wu64(rbx.wrapping_add(P0), rbp); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x128u64))) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(P7))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { crate::simd_ops::wu64(rax.wrapping_add(P7), rbx); crate::simd_ops::wu64(rbx.wrapping_add(P7), rcx); bb = 9; continue; }
            7 => { crate::simd_ops::wu64(rbx.wrapping_add(P7), rcx); crate::simd_ops::wu64(rax.wrapping_add(P7), rbx); bb = 9; continue; }
            8 => { rdi = 0x2; bb = 9; continue; }
            9 => { rcx = rsp.wrapping_add(0x20); rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); r11 = rsp.wrapping_add(0x150); rax = ((rdi as u32)) as u32 as u64; rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x10u64))) as u64; rbp = (crate::simd_ops::ru64(r11.wrapping_add(0x18u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rsp = r11; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415337d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141530ad0; const P1: u64 = 0x141530d60; const P2: u64 = 0x1415307a0; const P3: u64 = 0x1415242e0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rbx = (crate::simd_ops::ru64(rcx.wrapping_add(0x48u64))) as u64; rbp = rdx; r15 = ((r8 as u32)) as u32 as u64; r13 = rcx; r15 = ((r15).wrapping_add(rdx)) as u64; r12 = ((r8 as u32)) as u32 as u64; rdx = 0; fl_l = 0; fl_r = 0; rsi = ((rdx as u32)) as u32 as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 27; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0x80u64), r14); bb = 2; continue; }
            2 => { r9 = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rdi = rbx.wrapping_add(0x8); r14 = rbx; fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 15; continue; } bb = 3; continue; }
            3 => { rax = (((crate::simd_ops::ru32(rbx.wrapping_add(0x24u64))) as i32 as i64) as u64) as u64; rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; rcx = ((rcx).wrapping_sub(rax)) as u64; fl_l = r15; fl_r = rcx; if ((((fl_l as u64)) as i64) <= (((fl_r as u64)) as i64)) { bb = 14; continue; } bb = 4; continue; }
            4 => { fl_l = rbp; fl_r = rcx; if ((((fl_l as u64)) as i64) > (((fl_r as u64)) as i64)) { bb = 6; continue; } bb = 5; continue; }
            5 => { crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rdx as u32)) as u32); rcx = r9; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rdx as u32)) as u32); rdx = rsp.wrapping_add(0x20); rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); r9 = (crate::simd_ops::ru64(rdi)) as u64; rdx = 0; fl_l = 0; fl_r = 0; bb = 6; continue; }
            6 => { rax = (((crate::simd_ops::ru32(rbx.wrapping_add(0x28u64))) as i32 as i64) as u64) as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(rbx.wrapping_add(0x18u64)))) as u64; fl_l = rbp; fl_r = rax; if ((((fl_l as u64)) as i64) > (((fl_r as u64)) as i64)) { bb = 9; continue; } bb = 7; continue; }
            7 => { fl_l = r15; fl_r = rax; if ((((fl_l as u64)) as i64) <= (((fl_r as u64)) as i64)) { bb = 9; continue; } bb = 8; continue; }
            8 => { rax = (((rax as u32)).wrapping_sub((rbp as u32))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x70u64), ((rdx as u32)) as u32); r8 = ((rax as u32)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x74u64), (0x4u32) as u32); rdx = rsp.wrapping_add(0x70); rcx = r9; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 9; continue; }
            9 => { rcx = (crate::simd_ops::ru64(rdi)) as u64; rax = ((crate::simd_ops::ru8(rcx.wrapping_add(0x62u64))) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x3u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 14; continue; } bb = 10; continue; }
            10 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x10u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 14; continue; } bb = 11; continue; }
            11 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; r9 = rsp.wrapping_add(0x30); rax = ((rax).wrapping_add(1)) as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x38u64), (0x1u8) as u8); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); r8 = ((r12 as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = rbp; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x90u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rdi)) as u64; rax = ((crate::simd_ops::ru8(rcx.wrapping_add(0x62u64))) as u32) as u32 as u64; rax = (rax & !0xffu64) | ((((((rax as u8)) & (0xfu8))) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if (((fl_l as u8)) <= ((fl_r as u8))) { bb = 14; continue; } bb = 12; continue; }
            12 => { crate::simd_ops::wu64(rdi, (0x0u64) as u64); fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 13; continue; }
            13 => { rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 14; continue; }
            14 => { rsi = rbx; rbx = (crate::simd_ops::ru64(rbx)) as u64; bb = 25; continue; }
            15 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x20u64))) as u32 as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(rbx.wrapping_add(0x18u64)))) as u64; rcx = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rbp; fl_r = rax; if ((((fl_l as u64)) as i64) <= (((fl_r as u64)) as i64)) { bb = 24; continue; } bb = 16; continue; }
            16 => { fl_l = rbx; fl_r = (crate::simd_ops::ru64(r13.wrapping_add(0x48u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 18; continue; } bb = 17; continue; }
            17 => { crate::simd_ops::wu64(r13.wrapping_add(0x48u64), rcx); bb = 19; continue; }
            18 => { crate::simd_ops::wu64(rsi, rcx); bb = 19; continue; }
            19 => { rbx = rcx; rcx = (crate::simd_ops::ru64(rdi)) as u64; crate::simd_ops::wu64(rdi, rdx); fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 21; continue; } bb = 20; continue; }
            20 => { rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 21; continue; }
            21 => { rcx = (crate::simd_ops::ru64(rdi)) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 23; continue; } bb = 22; continue; }
            22 => { rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rdi, (0x0u64) as u64); bb = 23; continue; }
            23 => { rdx = r14; rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 25; continue; }
            24 => { rsi = rbx; rbx = rcx; bb = 25; continue; }
            25 => { rdx = 0x0; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 26; continue; }
            26 => { r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; bb = 27; continue; }
            27 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x88u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141533970(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141539540; const P1: u64 = 0x141532980; const P2: u64 = 0x141539bc0; const P3: u64 = 0x1401de410;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 47>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); crate::simd_ops::wu32(rsp.wrapping_add(0x10u64), ((rdx as u32)) as u32); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x150u64)) as u64; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; rsi = r8; r15 = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; r13 = rcx; r8 = 0; fl_l = 0; fl_r = 0; rcx = rsp.wrapping_add(0x20); r12 = r9; r15 = ((r15).wrapping_sub(crate::simd_ops::ru64(rdx.wrapping_add(0x70u64)))) as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x128u64))) as u64; rbx = 0; fl_l = 0; fl_r = 0; fl_l = rbp; fl_r = rbp; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu8(rsi, ((rbx as u8)) as u8); bb = 24; continue; }
            2 => { r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x1a0u64))) as u64; rdi = 0x3; crate::simd_ops::wu8(rsi, (0x1u8) as u8); rbp = (crate::simd_ops::ru64(rbp.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu32(r12, ((rbx as u32)) as u32); fl_l = rbp; fl_r = rbp; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru8(rbp.wrapping_add(0x62u64))) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 6; continue; } bb = 4; continue; }
            4 => { r8 = (crate::simd_ops::ru32(rsp.wrapping_add(0x188u64))) as u32 as u64; rdx = rsp.wrapping_add(0x20); r9 = r14; rcx = r13; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rdi = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 24; continue; } bb = 5; continue; }
            5 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 22; continue; } bb = 6; continue; }
            6 => { fl_l = (crate::simd_ops::ru8(rsp.wrapping_add(0x148u64))) as u8 as u64; fl_r = ((rbx as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 23; continue; } bb = 7; continue; }
            7 => { rcx = (crate::simd_ops::ru64(r13.wrapping_add(0x10u64))) as u64; rdx = rsp.wrapping_add(0x20); r8 = 0; fl_l = 0; fl_r = 0; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0xa0u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x128u64))) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x20u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 12; continue; } bb = 8; continue; }
            8 => { fl_l = ((rdi as u32)) as u32 as u64; fl_r = 0x3; rax = (rax & !0xffu64) | (((if (((fl_l as u32)) != ((fl_r as u32))) { 1u8 } else { 0u8 }) as u8 as u64)); crate::simd_ops::wu8(rsi, ((rax as u8)) as u8); fl_l = ((rdi as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 24; continue; } bb = 9; continue; }
            9 => { fl_l = rbp; fl_r = rbp; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 24; continue; } bb = 10; continue; }
            10 => { fl_l = (crate::simd_ops::ru8(rbp.wrapping_add(0x62u64))) as u8 as u64; fl_r = ((rdi as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 24; continue; } bb = 11; continue; }
            11 => { bb = 21; continue; }
            12 => { fl_l = ((rdi as u32)) as u32 as u64; fl_r = 0x3; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 14; continue; } bb = 13; continue; }
            13 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0x62u64))) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 23; continue; } bb = 14; continue; }
            14 => { r8 = (crate::simd_ops::ru32(rsp.wrapping_add(0x188u64))) as u32 as u64; rdx = rsp.wrapping_add(0x20); r9 = r14; crate::simd_ops::wu32(r12, (0x1u32) as u32); rcx = r13; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x3; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 16; continue; } bb = 15; continue; }
            15 => { fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 1; continue; } bb = 16; continue; }
            16 => { crate::simd_ops::wu8(rsi, (0x1u8) as u8); fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 22; continue; } bb = 17; continue; }
            17 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 24; continue; } bb = 18; continue; }
            18 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 20; continue; } bb = 19; continue; }
            19 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 24; continue; } bb = 20; continue; }
            20 => { fl_l = ((rdi as u32)) as u32 as u64; fl_r = 0x3; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 24; continue; } bb = 21; continue; }
            21 => { rcx = rsp.wrapping_add(0x20); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rbx = rax; crate::simd_ops::wu64(r14, rax); fl_l = rbx; fl_r = r15; rbx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; bb = 24; continue; }
            22 => { rbx = (crate::simd_ops::ru64(r14)) as u64; fl_l = rbx; fl_r = r15; rbx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; bb = 24; continue; }
            23 => { fl_l = ((rdi as u32)) as u32 as u64; fl_r = 0x3; rcx = (rcx & !0xffu64) | (((if (((fl_l as u32)) != ((fl_r as u32))) { 1u8 } else { 0u8 }) as u8 as u64)); crate::simd_ops::wu8(rsi, ((rcx as u8)) as u8); bb = 24; continue; }
            24 => { rcx = rsp.wrapping_add(0x20); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); r11 = rsp.wrapping_add(0x150); rax = rbx; rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(r11.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x48u64))) as u64; rsp = r11; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141533b20(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x48; const P1: u64 = 0x8; const P2: u64 = 0x1415307a0; const P3: u64 = 0x1415242e0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; r14 = 0; fl_l = 0; fl_r = 0; rsi = rcx; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rdi); rdx = rbx; bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rdi = rbx; fl_l = rbx; fl_r = rdx; rbx = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu64(rsi.wrapping_add(P0), rax); bb = 5; continue; }
            4 => { crate::simd_ops::wu64(r14, rax); bb = 5; continue; }
            5 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P1))) as u64; crate::simd_ops::wu64(rdi.wrapping_add(P1), r14); fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 7; continue; }
            7 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P1))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rdi.wrapping_add(P1), r14); bb = 9; continue; }
            9 => { rdx = rdi; rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru64(rsi.wrapping_add(P0))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 10; continue; }
            10 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; bb = 11; continue; }
            11 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141533bc0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14152fc30; const P1: u64 = 0x141532d10; const P2: u64 = 0x1415242e0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rsi = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; r15 = rdx; rbp = rcx; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 22; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rbx); rbx = rsi; crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), r13); r13 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), r12); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r14); bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 5; continue; } bb = 3; continue; }
            3 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 15; continue; } bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); bb = 15; continue; }
            5 => { rdi = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; r12 = (crate::simd_ops::ru32(rdi.wrapping_add(0x10u64))) as u32 as u64; r14 = ((crate::simd_ops::ru8(rdi.wrapping_add(0x2cu64))) as u32) as u32 as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x10u64))) as u64; rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x8u64))) as u64; r8 = (crate::simd_ops::ru64(rcx)) as u64; rdx = ((rdx).wrapping_sub(crate::simd_ops::ru64(rax.wrapping_add(0x70u64)))) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(r8.wrapping_add(0xc8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x48u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 8; continue; }
            8 => { bb = 9; continue; }
            9 => { fl_l = rax; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 12; continue; } bb = 10; continue; }
            10 => { rax = (crate::simd_ops::ru64(rax)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 9; continue; } bb = 11; continue; }
            11 => { bb = 13; continue; }
            12 => { crate::simd_ops::wu32(rax.wrapping_add(0x10u64), ((r13 as u32)) as u32); bb = 13; continue; }
            13 => { fl_l = ((r14 as u8)) as u8 as u64; fl_r = ((r14 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 15; continue; } bb = 14; continue; }
            14 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); rdx = ((r12 as u32)) as u32 as u64; rcx = rbp; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 15; continue; }
            15 => { rdi = (crate::simd_ops::ru64(rsi.wrapping_add(0x18u64))) as u64; fl_l = rsi; fl_r = (crate::simd_ops::ru64(r15.wrapping_add(0x8u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 17; continue; } bb = 16; continue; }
            16 => { crate::simd_ops::wu64(r15.wrapping_add(0x8u64), rdi); bb = 18; continue; }
            17 => { crate::simd_ops::wu64(0x18u64, rdi); bb = 18; continue; }
            18 => { rsi = rdi; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 20; continue; } bb = 19; continue; }
            19 => { rdx = rbx; rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 20; continue; }
            20 => { rbx = rdi; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 21; continue; }
            21 => { r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; bb = 22; continue; }
            22 => { rsp = ((rsp).wrapping_add(0x30u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141533cd0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x1438152ac; const P1: u64 = 0x143426d28; const P2: u64 = 0x1434269f8; const P3: u64 = 0x142677278;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rax = (crate::simd_ops::ru32(P0)) as u32 as u64;
    xmm0 = 0u128;
    xmm1 = (((rcx as u32)) as u32) as u128;
    xmm1 = crate::simd_ops::vcvtdq2pd(xmm1);
    xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((((rax) as i64) as f64).to_bits() as u128);
    xmm1 = (xmm1 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm1 as u64) * f64::from_bits(crate::simd_ops::ru64(P1))).to_bits() as u128);
    xmm1 = (xmm1 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm1 as u64) / f64::from_bits(xmm0 as u64)).to_bits() as u128);
    xmm0 = 0u128;
    fl_l = ((xmm1 as u64)) as u64; fl_r = ((xmm0 as u64)) as u64;
    if !(!(f64::from_bits(fl_l) > f64::from_bits(fl_r))) { xmm0 = ((crate::simd_ops::ru64(P2)) as u64) as u128; xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm0 as u64) + f64::from_bits(xmm1 as u64)).to_bits() as u128); rax = (crate::simd_ops::cvt_f64_i32((f64::from_bits(xmm0 as u64)))) as u32 as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
    xmm0 = ((crate::simd_ops::ru64(P3)) as u64) as u128;
    xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm0 as u64) + f64::from_bits(xmm1 as u64)).to_bits() as u128);
    rax = (crate::simd_ops::cvt_f64_i32((f64::from_bits(xmm0 as u64)))) as u32 as u64;
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}
